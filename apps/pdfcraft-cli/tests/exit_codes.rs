//! The exit codes and the machine-readable failure format a script depends on (REC-B02).
//!
//! These are the promises `apps/pdfcraft-cli/src/exit.rs` documents: a script must be able to tell
//! a bad command line from a damaged file, a missing tool from a failure, without reading prose.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::PathBuf;
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_pdfcraft-cli");

/// A scratch folder of this test's own, removed when it goes away.
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!("pdfcraft-exit-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn run(args: &[&str]) -> Output {
    Command::new(BIN).args(args).output().unwrap()
}

/// A real one-page PDF, so "damaged" means damaged and not "empty file".
fn pdf(scratch: &Scratch) -> String {
    let path = scratch.0.join("in.pdf");
    let bytes = pdfcraft_engine::Session::new().create_from_text("Exit codes", "A page of text").unwrap();
    std::fs::write(&path, bytes.as_slice()).unwrap();
    path.to_string_lossy().into_owned()
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

#[test]
fn a_command_line_that_cannot_run_is_usage() {
    // Exit 2 covers everything about the invocation itself: nothing was read or written, so a
    // script may correct the call and try again.
    let scratch = Scratch::new("usage");
    let file = pdf(&scratch);
    let image = scratch.0.join("page.png").to_string_lossy().into_owned();
    let cases: [&[&str]; 6] = [
        &["frobnicate"],
        &["info"],
        &["info", "--bogus"],
        &["render", file.as_str(), "--page", "0", "--out", image.as_str()],
        &["render", file.as_str(), "--page", "1", "--dpi", "coarse", "--out", image.as_str()],
        &["run"],
    ];
    for args in cases {
        let out = run(args);
        assert_eq!(out.status.code(), Some(2), "{args:?}: {}", stderr(&out));
        assert!(out.stdout.is_empty(), "{args:?} wrote to stdout: {:?}", out.stdout);
        assert!(!scratch.0.join("page.png").exists(), "{args:?} wrote a file");
    }
}

#[test]
fn a_file_that_is_not_a_document_is_malformed() {
    let scratch = Scratch::new("malformed");
    let path = scratch.0.join("broken.pdf");
    std::fs::write(&path, b"this is not a PDF at all").unwrap();
    let broken = path.to_string_lossy().into_owned();
    let out = run(&["info", &broken]);
    assert_eq!(out.status.code(), Some(4), "{}", stderr(&out));
}

#[test]
fn a_tool_this_build_does_not_have_is_unsupported() {
    // Asking for a tool that does not exist is not a failed operation: nothing ran.
    let out = run(&["run", "no_such_tool"]);
    assert_eq!(out.status.code(), Some(3), "{}", stderr(&out));
    assert!(stderr(&out).contains("no_such_tool"), "{}", stderr(&out));
}

#[test]
fn a_script_that_stops_after_some_work_is_partial() {
    // Step 1 opened a document and step 2 asked for a tool that does not exist; the open stands,
    // so the outcome is partial (6) rather than failed (1) or usage (2).
    let scratch = Scratch::new("partial");
    let file = pdf(&scratch);
    let script = scratch.0.join("steps.json");
    let steps = serde_json::json!([
        { "tool": "doc_open", "args": { "path": file } },
        { "tool": "no_such_tool", "args": {} },
    ]);
    std::fs::write(&script, serde_json::to_vec(&steps).unwrap()).unwrap();
    let script = script.to_string_lossy().into_owned();
    let out = run(&["run", "--script", &script]);
    assert_eq!(out.status.code(), Some(6), "{}", stderr(&out));
    let message = stderr(&out);
    assert!(message.contains("step 2 (no_such_tool)"), "{message}");
}

#[test]
fn the_json_format_reports_one_parseable_line() {
    for args in [&["--error-format=json", "info"][..], &["info", "--error-format", "json"]] {
        let out = run(args);
        assert_eq!(out.status.code(), Some(2), "{args:?}: {}", stderr(&out));
        let line = stderr(&out);
        assert_eq!(line.trim_end().lines().count(), 1, "not one line: {line:?}");
        let value: serde_json::Value = serde_json::from_str(line.trim_end()).expect("the failure is one JSON line");
        let error = &value["error"];
        assert_eq!(error["kind"], "usage", "{line}");
        assert_eq!(error["exit_code"], 2, "{line}");
        assert!(error["message"].as_str().unwrap_or_default().contains("missing file"), "{line}");
    }
}

#[test]
fn a_name_with_a_quote_survives_the_json_format() {
    // The file's name reaches the message; it must not be able to break the JSON around it.
    let scratch = Scratch::new("quote");
    let missing = scratch.0.join("say \"hi\".pdf").to_string_lossy().into_owned();
    let out = run(&["--error-format=json", "info", &missing]);
    let line = stderr(&out);
    let value: serde_json::Value = serde_json::from_str(line.trim_end()).expect("still one JSON line");
    let message = value["error"]["message"].as_str().unwrap_or_default();
    assert!(message.contains("say \"hi\".pdf"), "{message}");
    // Nothing was written: the read failed, which is not a usage error.
    assert_eq!(out.status.code(), Some(1), "{line}");
    assert_eq!(value["error"]["kind"], "failed", "{line}");
}

#[test]
fn text_reporting_reports_the_prose_by_default() {
    // The default stays the line it always was, so existing wrappers keep working.
    let out = run(&["info"]);
    assert_eq!(out.status.code(), Some(2));
    assert_eq!(stderr(&out), "pdfcraft-cli: info: missing file\n");
}
