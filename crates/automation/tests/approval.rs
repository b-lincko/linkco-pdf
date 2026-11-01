//! An approval names the batch it approves (B033, REC-B03).
//!
//! `batch_plan` has said what a destructive batch would write since B025, and handed back the call
//! that would carry it out. It could not stop that call from doing something else: the batch is a
//! list made at one moment — a file added to a walked folder, a different folder argument, another
//! action's steps — and the run would carry out whatever it found when it started.
//!
//! These tests hold the approval to its one promise: the run that is given a token carries out the
//! batch that token names, or refuses and writes nothing. And to its one limit: a call without a
//! token is the call it always was.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};

use pdfcraft_automation::{Automation, Content, ToolError};
use serde_json::{Value, json};

/// A PDF with `n` 200×300 pt pages reading "Page 1", "Page 2", …
fn fixture(n: usize) -> Vec<u8> {
    let mut objs: Vec<Vec<u8>> = vec![b"<< /Type /Catalog /Pages 2 0 R >>".to_vec()];
    let kids: Vec<String> = (0..n).map(|i| format!("{} 0 R", 4 + 2 * i)).collect();
    objs.push(format!("<< /Type /Pages /Kids [{}] /Count {n} /MediaBox [0 0 200 300] >>", kids.join(" ")).into_bytes());
    objs.push(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec());
    for i in 0..n {
        objs.push(format!("<< /Type /Page /Parent 2 0 R /Contents {} 0 R /Resources << /Font << /F1 3 0 R >> >> >>", 5 + 2 * i).into_bytes());
        let body = format!("BT /F1 24 Tf 20 150 Td (Page {}) Tj ET", i + 1);
        objs.push(format!("<< /Length {} >>\nstream\n{body}\nendstream", body.len()).into_bytes());
    }
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (i, o) in objs.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", i + 1).as_bytes());
        out.extend_from_slice(o);
        out.extend_from_slice(b"\nendobj\n");
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objs.len() + 1).as_bytes());
    for o in offsets {
        out.extend_from_slice(format!("{o:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objs.len() + 1).as_bytes());
    out
}

/// A fresh root: `in/a.pdf`, `in/b.pdf` and `in/sub/c.pdf`, and no output folder.
fn tree(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pdfcraft-approval-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("in/sub")).unwrap();
    std::fs::write(dir.join("in/a.pdf"), fixture(1)).unwrap();
    std::fs::write(dir.join("in/b.pdf"), fixture(2)).unwrap();
    std::fs::write(dir.join("in/sub/c.pdf"), fixture(1)).unwrap();
    dir
}

fn auto(dir: &Path) -> Automation {
    Automation::new().with_root(dir).unwrap().with_clock(|| 1_700_000_000)
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

/// Everything under `dir`, sorted: what a refused call must leave exactly as it was.
fn listing(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).unwrap() {
            let e = e.unwrap();
            let p = e.path();
            if p.is_dir() {
                stack.push(p.clone());
            }
            out.push(p.strip_prefix(dir).unwrap().to_string_lossy().replace('\\', "/"));
        }
    }
    out.sort();
    out
}

/// The steps a plan of this test uses.
fn title() -> Value {
    json!([{ "step": "set_title", "arg": "Approved" }])
}

/// A plan of the tree's `in` folder into `out`, and the token it carries.
fn plan(a: &mut Automation, folder: &str) -> (Value, String) {
    let plan = ok(&mut a, "batch_plan", json!({ "operation": "action", "steps": title(), "paths": ["in"], "folder": folder }));
    let token = plan["approval"]["token"].as_str().unwrap_or_else(|| panic!("no token in {plan}")).to_string();
    (plan, token)
}

/// The call a plan says would carry it out, with the plan's token added.
fn approve(plan: &Value, token: &str) -> Value {
    let mut args = plan["approve"]["args"].clone();
    args["approve_token"] = json!(token);
    args
}

#[test]
fn the_run_a_plan_approves_is_the_batch_it_describes() {
    let dir = tree("approves");
    let mut a = auto(&dir);
    let (plan, token) = plan(&mut a, "out");
    assert_eq!(plan["approval"]["covers"], 3, "{plan}");
    assert_eq!(token.len(), 64, "a token is a SHA-256 digest in hex: {token}");
    assert!(token.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)), "{token}");

    let run = ok(&mut a, "action_run", approve(&plan, &token));
    assert_eq!(run["approved"], true, "the token was checked and matched: {run}");
    // The files are exactly the ones the plan named, and they exist.
    let outputs: Vec<&str> = plan["targets"].as_array().unwrap().iter().map(|t| t["output"].as_str().unwrap()).collect();
    for output in &outputs {
        assert!(Path::new(output).exists(), "the plan named {output} and the run did not write it");
    }
    let ran: Vec<&str> = run["files"].as_array().unwrap().iter().map(|f| f["output"].as_str().unwrap()).collect();
    assert_eq!(ran, outputs, "the run wrote what it approved: {run}");
}

#[test]
fn a_call_without_a_token_is_the_call_it_always_was() {
    let dir = tree("no-token");
    let mut a = auto(&dir);
    let run = ok(&mut a, "action_run", json!({ "steps": title(), "paths": ["in"], "folder": "out" }));
    assert_eq!(run["approved"], false, "nothing was approved, and the answer says so: {run}");
    assert_eq!(std::fs::read_dir(dir.join("out")).unwrap().count(), 3);
}

#[test]
fn a_token_names_one_batch_and_nothing_else() {
    // Each case is the same plan's token offered for a different batch: another folder, another
    // action, another list. None of them may run, and none may leave a file behind.
    let dir = tree("drift");
    let mut a = auto(&dir);
    let (plan, token) = plan(&mut a, "out");
    let before = listing(&dir);

    let mismatch = |a: &mut Automation, args: Value, why: &str| match a.call("action_run", &args) {
        Err(ToolError::Refused(message)) => {
            assert!(message.contains("different batch"), "{why}: {message}");
            assert!(message.contains("batch_plan"), "the refusal says what to do next: {message}");
        }
        other => panic!("{why}: expected a refusal, got {other:?}"),
    };

    // Another folder.
    let mut other_folder = approve(&plan, &token);
    other_folder["folder"] = json!("out2");
    mismatch(&mut a, other_folder, "another folder");
    assert!(!dir.join("out2").exists(), "a refused run does not even create its folder");

    // Another action's steps.
    let mut other_steps = approve(&plan, &token);
    other_steps["steps"] = json!([{ "step": "set_author", "arg": "Someone Else" }]);
    mismatch(&mut a, other_steps, "another action");

    // Another list.
    let mut other_list = approve(&plan, &token);
    other_list["paths"] = json!(["in/a.pdf", "in/b.pdf"]);
    mismatch(&mut a, other_list, "another list");

    // The same token offered to the other writer, for the same files and folder.
    let crossed = json!({ "paths": ["in"], "folder": "out", "approve_token": token });
    match a.call("ocr_recognize_files", &crossed) {
        Err(ToolError::Refused(message)) => assert!(message.contains("different batch"), "{message}"),
        other => panic!("a token from another tool: expected a refusal, got {other:?}"),
    }

    assert_eq!(listing(&dir), before, "every refusal left the tree exactly as it was");
}

#[test]
fn a_batch_that_grew_after_it_was_approved_is_not_the_one_that_runs() {
    let dir = tree("grew");
    let mut a = auto(&dir);
    let (plan, token) = plan(&mut a, "out");
    // A file arrives in the folder the plan walked: the run would now write four files, and the
    // fourth is one nobody approved.
    std::fs::write(dir.join("in/sub/new.pdf"), fixture(1)).unwrap();
    let before = listing(&dir);
    match a.call("action_run", &approve(&plan, &token)) {
        Err(ToolError::Refused(message)) => {
            assert!(message.contains("4 file(s)"), "the refusal says what this call would do: {message}");
        }
        other => panic!("expected a refusal, got {other:?}"),
    }
    assert_eq!(listing(&dir), before, "nothing was written, not even for the files that were approved");
    assert!(!dir.join("out").exists());

    // Planning again gives a token that matches the batch as it is now, and that run goes ahead.
    let (again, token) = plan(&mut a, "out");
    assert_ne!(token, plan["approval"]["token"].as_str().unwrap(), "the batch changed, so the token did");
    let run = ok(&mut a, "action_run", approve(&again, &token));
    assert_eq!(run["approved"], true);
    assert_eq!(std::fs::read_dir(dir.join("out")).unwrap().count(), 4);
}

#[test]
fn an_output_that_appeared_after_the_plan_makes_the_approval_stale() {
    // The planned name is taken between planning and running, so the run would write somewhere the
    // plan did not name. That is exactly the case an approval has to catch.
    let dir = tree("in-the-way");
    let mut a = auto(&dir);
    std::fs::create_dir_all(dir.join("out")).unwrap();
    let (plan, token) = plan(&mut a, "out");
    let planned_output = plan["targets"][0]["output"].as_str().unwrap().to_string();
    assert_eq!(plan["targets"][0]["replaces"], false, "the plan says the name is free: {plan}");
    std::fs::write(&planned_output, b"something else").unwrap();

    match a.call("action_run", &approve(&plan, &token)) {
        Err(ToolError::Refused(message)) => assert!(message.contains("different batch"), "{message}"),
        other => panic!("expected a refusal, got {other:?}"),
    }
    assert_eq!(std::fs::read(planned_output.as_str()).unwrap(), b"something else", "the file that appeared is untouched");
}

#[test]
fn a_token_travels_between_spellings_but_not_between_batches() {
    // A token is a digest of what a batch *is*, not of how it was typed: `out` and `./out` are one
    // folder, and approving one approves the other.
    let dir = tree("spelling");
    let mut a = auto(&dir);
    let (_plan, token) = plan(&mut a, "out");
    let run = ok(&mut a, "action_run", json!({ "steps": title(), "paths": ["in"], "folder": "./out", "approve_token": token }));
    assert_eq!(run["approved"], true, "{run}");
}

#[test]
fn a_token_that_is_not_one_is_a_usage_error() {
    let dir = tree("shape");
    let mut a = auto(&dir);
    let (shouting, short, long) = ("Z".repeat(64), "a".repeat(63), "a".repeat(65));
    for bad in ["", "abc", shouting.as_str(), short.as_str(), long.as_str()] {
        let error = a.call("action_run", &json!({ "steps": title(), "paths": ["in"], "folder": "out", "approve_token": bad })).unwrap_err();
        assert_eq!(error.kind(), "usage", "{bad:?}: {error}");
        assert!(error.to_string().contains("approve_token"), "{error}");
    }
    assert!(!dir.join("out").exists(), "a rejected token wrote nothing");
}

#[test]
fn a_text_recognition_batch_is_approved_the_same_way() {
    // The refusal is checked before the models are looked for, so this holds on a machine without
    // the OCR models — which is most of them, and every CI runner.
    let dir = tree("ocr");
    let mut a = auto(&dir);
    let plan = ok(&mut a, "batch_plan", json!({ "operation": "ocr", "paths": ["in"], "folder": "out", "language": "en" }));
    let token = plan["approval"]["token"].as_str().unwrap().to_string();
    assert_eq!(plan["approval"]["covers"], 3, "{plan}");
    let before = listing(&dir);

    // Another language is another batch, even though this build only reads one.
    let mut other = plan["approve"]["args"].clone();
    other["approve_token"] = json!(token);
    other["dpi"] = json!(200);
    match a.call("ocr_recognize_files", &other) {
        Err(ToolError::Refused(message)) => assert!(message.contains("different batch"), "{message}"),
        other => panic!("expected a refusal, got {other:?}"),
    }
    assert_eq!(listing(&dir), before);
    assert!(!dir.join("out").exists());
}
