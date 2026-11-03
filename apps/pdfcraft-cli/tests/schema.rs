//! `pdfcraft-cli schema`: the versioned tool table, and the compatibility question (REC-B02).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::process::{Command, Output};

use serde_json::Value;

const BIN: &str = env!("CARGO_BIN_EXE_pdfcraft-cli");

fn run(args: &[&str]) -> Output {
    Command::new(BIN).args(args).output().unwrap()
}

/// The parsed stdout of a command that worked, and a check that it worked.
fn document(args: &[&str]) -> Value {
    let out = run(args);
    assert!(out.status.success(), "{args:?}: {}", String::from_utf8_lossy(&out.stderr));
    serde_json::from_slice(&out.stdout).unwrap()
}

#[test]
fn the_schema_document_carries_its_version_and_every_tool() {
    let doc = document(&["schema"]);
    assert_eq!(doc["schema_version"], pdfcraft_automation::TOOL_SCHEMA_VERSION, "{doc}");
    let listed = doc["tools"].as_array().unwrap();
    assert_eq!(listed.len(), pdfcraft_automation::tools().len());
    assert_eq!(doc["tool_count"].as_u64().unwrap() as usize, listed.len());
    assert!(listed.iter().any(|t| t["name"] == "doc_open" && t["input_schema"]["type"] == "object"));

    // `tools` and `schema` describe the same table: one list of names, in the same order.
    let tools = document(&["tools"]).as_array().unwrap().clone();
    let names: Vec<&str> = listed.iter().map(|t| t["name"].as_str().unwrap()).collect();
    let tool_names: Vec<&str> = tools.iter().map(|t| t["name"].as_str().unwrap()).collect();
    assert_eq!(names, tool_names);
}

#[test]
fn the_same_major_version_is_compatible_and_says_so() {
    let version = pdfcraft_automation::TOOL_SCHEMA_VERSION;
    let (major, _) = pdfcraft_automation::schema::parse_version(version).unwrap();
    for asked in [version.to_string(), format!("{major}.0"), format!("{major}.99")] {
        let doc = document(&["schema", "--schema-version", &asked]);
        assert_eq!(doc["asked"]["schema_version"], asked, "{doc}");
        assert_eq!(doc["asked"]["compatible"], true, "{doc}");
        assert_eq!(doc["schema_version"], version);
    }
}

#[test]
fn a_different_major_version_is_unsupported_and_still_reports_the_table() {
    let version = pdfcraft_automation::TOOL_SCHEMA_VERSION;
    let (major, _) = pdfcraft_automation::schema::parse_version(version).unwrap();
    let asked = format!("{}.0", major + 1);
    let out = run(&["schema", "--schema-version", &asked, "--error-format=json"]);
    assert_eq!(out.status.code(), Some(3), "{}", String::from_utf8_lossy(&out.stderr));
    let stderr = String::from_utf8_lossy(&out.stderr);
    let error: Value = serde_json::from_str(stderr.lines().next().unwrap()).unwrap();
    assert_eq!(error["error"]["kind"], "unsupported");
    assert_eq!(error["error"]["exit_code"], 3);
    assert!(error["error"]["message"].as_str().unwrap().contains(&asked), "{stderr}");
    // The caller still gets the document it asked about, so it can see what this build does speak.
    let doc: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(doc["asked"]["compatible"], false);
    assert_eq!(doc["schema_version"], version);
}

#[test]
fn a_value_that_is_not_a_version_is_a_usage_error() {
    for value in ["1", "one", "1.2.3", ""] {
        let out = run(&["schema", "--schema-version", value, "--error-format=json"]);
        assert_eq!(out.status.code(), Some(2), "{value:?}: {}", String::from_utf8_lossy(&out.stderr));
        assert!(out.stdout.is_empty(), "{value:?}: a refused version prints no document");
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(stderr.contains("usage"), "{stderr}");
    }
}

#[test]
fn the_command_says_what_it_does_not_understand() {
    let out = run(&["schema", "--frobnicate"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("unknown option --frobnicate"));
    let out = run(&["schema", "extra"]);
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("unexpected argument"));
    // The help names the command, so it can be found without the source.
    let help = String::from_utf8_lossy(&run(&["--help"]).stdout).into_owned();
    assert!(help.contains("pdfcraft-cli schema"), "{help}");
    assert!(help.contains("--schema-version"), "{help}");
}

#[test]
fn run_pins_the_schema_version_before_anything_runs() {
    let version = pdfcraft_automation::TOOL_SCHEMA_VERSION;
    let (major, _) = pdfcraft_automation::schema::parse_version(version).unwrap();
    // Same major (any minor): the call runs normally.
    for asked in [version.to_string(), format!("{major}.0"), format!("{major}.999")] {
        let out = run(&["run", "policy_status", "--schema-version", &asked]);
        assert!(out.status.success(), "{asked}: {}", String::from_utf8_lossy(&out.stderr));
        let doc: Value = serde_json::from_slice(&out.stdout).unwrap();
        assert!(doc.get("profile").is_some(), "the tool ran: {doc}");
    }
    // A different major: exit 3 (unsupported) before the tool runs. The proof that nothing ran:
    // this call would fail with the tool's own error (not 3) if the tool were reached at all.
    let asked = format!("{}.0", major + 1);
    let out = run(&["--error-format=json", "run", "doc_open", "path=/nonexistent/no-such-file.pdf", "--schema-version", &asked]);
    assert_eq!(out.status.code(), Some(3), "{}", String::from_utf8_lossy(&out.stderr));
    assert!(out.stdout.is_empty(), "a refused call prints no result");
    let error: Value = serde_json::from_str(String::from_utf8_lossy(&out.stderr).lines().next().unwrap()).unwrap();
    assert_eq!(error["error"]["kind"], "unsupported");
    assert_eq!(error["error"]["exit_code"], 3);
    let message = error["error"]["message"].as_str().unwrap();
    assert!(message.contains(&asked) && message.contains(version), "both versions named: {message}");
    // Not a version at all: exit 2 (usage), and again nothing runs.
    for value in ["banana", "1", "1.2.3"] {
        let out = run(&["run", "policy_status", "--schema-version", value]);
        assert_eq!(out.status.code(), Some(2), "{value}: {}", String::from_utf8_lossy(&out.stderr));
        assert!(out.stdout.is_empty(), "{value}: a refused call prints no result");
        assert!(String::from_utf8_lossy(&out.stderr).contains("not a version"), "{value}");
    }
    // The pin applies to a whole `--script` too, checked before the first step.
    let scratch = std::env::temp_dir().join(format!("pdfcraft-schema-pin-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    std::fs::create_dir_all(&scratch).unwrap();
    let script = scratch.join("steps.json");
    let steps = serde_json::json!([{ "tool": "doc_open", "args": { "path": "/nonexistent/no-such-file.pdf" } }]);
    std::fs::write(&script, serde_json::to_vec(&steps).unwrap()).unwrap();
    let out = run(&["run", "--script", script.to_str().unwrap(), "--schema-version", &asked]);
    assert_eq!(out.status.code(), Some(3), "{}", String::from_utf8_lossy(&out.stderr));
    let _ = std::fs::remove_dir_all(&scratch);
}

#[test]
fn the_help_names_the_run_pin() {
    let help = String::from_utf8_lossy(&run(&["--help"]).stdout).into_owned();
    assert!(help.contains("pdfcraft-cli run"), "{help}");
    assert!(help.contains("--schema-version"), "{help}");
}
