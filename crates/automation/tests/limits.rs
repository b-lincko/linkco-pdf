//! Bounded work (REC-B07): the input, output and batch quotas, the worker count and the time
//! limit are set where a session starts, refused before the work begins, and audited.

use std::path::{Path, PathBuf};
use std::time::Duration;

use pdfcraft_automation::{Automation, Content, Limits, ToolError};
use serde_json::{Value, json};

fn workdir(test: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pdfcraft-limits-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn ok(a: &mut Automation, tool: &str, args: Value) -> Value {
    match a.call(tool, &args) {
        Ok(mut c) => match c.remove(0) {
            Content::Json(v) => v,
            other => panic!("{tool}: expected JSON, got {other:?}"),
        },
        Err(e) => panic!("{tool} {args}: {e}"),
    }
}

/// A session with no limits of its own, confined to `dir`.
fn auto(dir: &Path) -> Automation {
    Automation::new().with_root(dir).unwrap().with_clock(|| 1_700_000_000)
}

/// Write a small PDF called `name` into `dir` and return its path.
fn seed(dir: &Path, name: &str) -> PathBuf {
    let mut a = auto(dir);
    let doc = ok(&mut a, "doc_create", json!({ "from": "text", "text": "Page one" }))["doc"].as_u64().unwrap();
    ok(&mut a, "doc_save", json!({ "doc": doc, "path": name }));
    dir.join(name)
}

fn refused(a: &mut Automation, tool: &str, args: Value) -> String {
    match a.call(tool, &args) {
        Err(ToolError::Refused(message)) => message,
        Ok(_) => panic!("{tool} was allowed but should have been refused: {args}"),
        Err(e) => panic!("{tool} failed for another reason: {e}"),
    }
}

#[test]
fn an_input_over_the_limit_is_refused_before_it_is_read() {
    let dir = workdir("input");
    seed(&dir, "a.pdf");
    let size = std::fs::metadata(dir.join("a.pdf")).unwrap().len();
    assert!(size > 64);

    let mut a = auto(&dir).with_limits(Limits { max_input_bytes: 64, ..Default::default() });
    let message = refused(&mut a, "doc_open", json!({ "path": "a.pdf" }));
    assert!(message.contains("input limit") && message.contains("--max-input-mib"), "{message}");
    assert!(message.contains("nothing was read"), "{message}");
    // Nothing was opened, and the audit records the refusal rather than hiding it.
    assert_eq!(ok(&mut a, "doc_list", json!({}))["documents"].as_array().unwrap().len(), 0);
    let status = ok(&mut a, "limits_status", json!({}));
    assert_eq!(status["audit"]["inputs_refused"].as_u64(), Some(1), "{status}");
    assert_eq!(status["limits"]["max_input_bytes"].as_u64(), Some(64));

    // Under a limit that fits, the same file opens.
    let mut b = auto(&dir).with_limits(Limits { max_input_bytes: 1 << 20, ..Default::default() });
    assert_eq!(ok(&mut b, "doc_open", json!({ "path": "a.pdf" }))["pages"], 1);
    assert_eq!(ok(&mut b, "limits_status", json!({}))["audit"]["inputs_refused"].as_u64(), Some(0));
}

#[test]
fn an_output_over_the_limit_leaves_no_file() {
    let dir = workdir("output");
    let mut a = auto(&dir).with_limits(Limits { max_output_bytes: 16, ..Default::default() });
    let doc = ok(&mut a, "doc_create", json!({ "from": "text", "text": "Hello" }))["doc"].as_u64().unwrap();
    let message = refused(&mut a, "doc_save", json!({ "doc": doc, "path": "out.pdf" }));
    assert!(message.contains("output limit") && message.contains("--max-output-mib"), "{message}");
    assert!(message.contains("nothing was written"), "{message}");
    assert!(!dir.join("out.pdf").exists(), "a refused save must leave no file");
    let status = ok(&mut a, "limits_status", json!({}));
    assert_eq!(status["audit"]["outputs_refused"].as_u64(), Some(1), "{status}");
    assert_eq!(status["audit"]["bytes_written"].as_u64(), Some(0));

    // A generous limit writes, and the audit counts the bytes it wrote.
    let mut b = auto(&dir).with_limits(Limits { max_output_bytes: 1 << 20, ..Default::default() });
    let doc = ok(&mut b, "doc_create", json!({ "from": "text", "text": "Hello" }))["doc"].as_u64().unwrap();
    let saved = ok(&mut b, "doc_save", json!({ "doc": doc, "path": "written.pdf" }));
    let written = std::fs::metadata(dir.join("written.pdf")).unwrap().len();
    assert_eq!(saved["bytes"].as_u64(), Some(written));
    assert_eq!(ok(&mut b, "limits_status", json!({}))["audit"]["bytes_written"].as_u64(), Some(written));
}

#[test]
fn a_batch_over_the_limit_is_refused_before_anything_is_read() {
    let dir = workdir("batch");
    for name in ["a.pdf", "b.pdf", "c.pdf"] {
        seed(&dir, name);
    }
    let mut a = auto(&dir).with_limits(Limits { max_batch_items: 2, ..Default::default() });
    let message = refused(&mut a, "action_run", json!({ "action": "Add Page Numbers", "paths": ["a.pdf", "b.pdf", "c.pdf"], "folder": "out" }));
    assert!(message.contains("batch limit") && message.contains("--max-batch"), "{message}");
    // The plan refuses the same batch rather than promising a run that cannot happen.
    let used = ok(&mut a, "limits_status", json!({}))["audit"]["batches_refused"].as_u64();
    assert_eq!(used, Some(1));
    let message = refused(
        &mut a,
        "batch_plan",
        json!({ "operation": "action", "action": "Add Page Numbers", "paths": ["a.pdf", "b.pdf", "c.pdf"], "folder": "out" }),
    );
    assert!(message.contains("batch limit"), "{message}");
    assert!(!dir.join("out").exists(), "a refused batch created its folder");

    // Under the limit the same batch runs.
    let mut b = auto(&dir).with_limits(Limits { max_batch_items: 3, ..Default::default() });
    let run = ok(&mut b, "action_run", json!({ "action": "Add Page Numbers", "paths": ["a.pdf", "b.pdf", "c.pdf"], "folder": "out" }));
    assert_eq!(run["files"].as_array().unwrap().len(), 3);
    assert_eq!(run["timed_out"], false);
}

#[test]
fn a_time_limit_stops_a_batch_between_items_and_keeps_what_it_finished() {
    let dir = workdir("time");
    for name in ["a.pdf", "b.pdf", "c.pdf"] {
        seed(&dir, name);
    }
    // One nanosecond is over before the second item can start, so exactly one file is written and
    // the rest are reported as skipped rather than silently dropped.
    let mut a = auto(&dir).with_limits(Limits { item_timeout: Duration::from_nanos(1), ..Default::default() });
    let run = ok(
        &mut a,
        "action_run",
        json!({ "steps": [{ "step": "set_title", "arg": "Rushed" }], "paths": ["a.pdf", "b.pdf", "c.pdf"], "folder": "out" }),
    );
    let files = run["files"].as_array().unwrap();
    assert!(files[0]["output"].is_string(), "{}", files[0]);
    assert!(files[1]["skipped"].as_str().unwrap().contains("time limit"), "{}", files[1]);
    assert!(files[2]["skipped"].is_string(), "{}", files[2]);
    assert_eq!(run["timed_out"], true);
    assert_eq!(std::fs::read_dir(dir.join("out")).unwrap().count(), 1, "only the item in hand is written");
    let audit = ok(&mut a, "limits_status", json!({}))["audit"].clone();
    assert!(audit["batches_stopped_by_time"].as_u64().unwrap() >= 1, "{audit}");

    // Without a time limit the same batch finishes every file.
    let mut b = auto(&dir);
    let run = ok(
        &mut b,
        "action_run",
        json!({ "steps": [{ "step": "set_title", "arg": "Rushed" }], "paths": ["a.pdf", "b.pdf", "c.pdf"], "folder": "done" }),
    );
    assert_eq!(run["timed_out"], false);
    assert_eq!(run["files"].as_array().unwrap().iter().filter(|f| f["output"].is_string()).count(), 3);
}

#[test]
fn the_worker_count_is_clamped_and_reported() {
    let dir = workdir("workers");
    seed(&dir, "a.pdf");
    let a = auto(&dir).with_limits(Limits { workers: 0, ..Default::default() });
    // Zero workers is a mistake, not a limit: it means the one thread the work can always use.
    assert_eq!(a.resources().limits().workers, 1);
    assert!(Limits::default().workers >= 1 && Limits::default().workers <= 8);

    let mut a = auto(&dir).with_limits(Limits { workers: 2, ..Default::default() });
    let status = ok(&mut a, "limits_status", json!({}));
    assert_eq!(status["limits"]["workers"].as_u64(), Some(2));
    // The limit does not stop ordinary extraction from working through every page.
    let doc = ok(&mut a, "doc_open", json!({ "path": "a.pdf" }))["doc"].as_u64().unwrap();
    assert_eq!(ok(&mut a, "text_extract", json!({ "doc": doc }))["pages"].as_array().unwrap().len(), 1);
}

#[test]
fn a_session_without_limits_reports_zeroes_and_still_counts_what_it_did() {
    let dir = workdir("default");
    seed(&dir, "a.pdf");
    let mut a = auto(&dir);
    let status = ok(&mut a, "limits_status", json!({}));
    assert_eq!(status["limits"]["max_input_bytes"].as_u64(), Some(0), "{status}");
    assert_eq!(status["limits"]["max_output_bytes"].as_u64(), Some(0));
    assert_eq!(status["limits"]["max_batch_items"].as_u64(), Some(0));
    assert_eq!(status["limits"]["item_timeout_ms"].as_u64(), Some(0));
    let opened = ok(&mut a, "doc_open", json!({ "path": "a.pdf" }));
    assert_eq!(opened["pages"], 1);
    let audit = &ok(&mut a, "limits_status", json!({}))["audit"];
    assert!(audit["checks"].as_u64().unwrap() >= 1, "{audit}");
    assert_eq!(audit["inputs_refused"].as_u64(), Some(0));
}
