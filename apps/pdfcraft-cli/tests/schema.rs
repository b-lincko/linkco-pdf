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

/// One step of a `--script` may pin the revision it was written against (REC-B02): the pin is
/// answered before that step runs, so a mismatch never runs the step it was written for.
#[test]
fn a_script_step_may_pin_its_own_schema_version() {
    let version = pdfcraft_automation::TOOL_SCHEMA_VERSION;
    let (major, _) = pdfcraft_automation::schema::parse_version(version).unwrap();
    let other = format!("{}.0", major + 1);
    let scratch = std::env::temp_dir().join(format!("pdfcraft-step-pin-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&scratch);
    std::fs::create_dir_all(&scratch).unwrap();
    let write_script = |steps: serde_json::Value| {
        let script = scratch.join("steps.json");
        std::fs::write(&script, serde_json::to_vec(&steps).unwrap()).unwrap();
        script.to_str().unwrap().to_string()
    };
    let create = serde_json::json!({ "tool": "doc_create", "args": { "from": "text", "text": "pinned" } });

    // A matching pin on a step runs that step (its result is on stdout); a mismatching pin on a
    // later step refuses before it, so the run is partial (exit 6) and step 1's result stands.
    let mut first = create.clone();
    first["schema_version"] = serde_json::json!(version);
    let mut second = serde_json::json!({ "tool": "doc_open", "args": { "path": "/nonexistent/no-such-file.pdf" } });
    second["schema_version"] = serde_json::json!(other);
    let script = write_script(serde_json::json!([first, second]));
    let out = run(&["--error-format=json", "run", "--script", &script]);
    assert_eq!(out.status.code(), Some(6), "{}", String::from_utf8_lossy(&out.stderr));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("\"doc\""), "the step before the refusal ran and printed its result: {stdout}");
    let error: Value = serde_json::from_str(String::from_utf8_lossy(&out.stderr).lines().next().unwrap()).unwrap();
    assert_eq!(error["error"]["kind"], "partial");
    let message = error["error"]["message"].as_str().unwrap();
    assert!(message.contains("step 2") && message.contains(&other), "{error}");

    // A mismatching pin on the first step: exit 3, nothing ran (no result on stdout).
    let mut refused = create.clone();
    refused["schema_version"] = serde_json::json!(other);
    let script = write_script(serde_json::json!([refused]));
    let out = run(&["--error-format=json", "run", "--script", &script]);
    assert_eq!(out.status.code(), Some(3), "{}", String::from_utf8_lossy(&out.stderr));
    assert!(out.stdout.is_empty(), "a refused first step printed no result");
    let error: Value = serde_json::from_str(String::from_utf8_lossy(&out.stderr).lines().next().unwrap()).unwrap();
    assert_eq!(error["error"]["kind"], "unsupported");
    assert_eq!(error["error"]["exit_code"], 3);

    // Not a version (or not a string): exit 2, nothing ran.
    for pin in [serde_json::json!("banana"), serde_json::json!(3)] {
        let mut step = create.clone();
        step["schema_version"] = pin.clone();
        let script = write_script(serde_json::json!([step]));
        let out = run(&["run", "--script", &script]);
        assert_eq!(out.status.code(), Some(2), "{pin}: {}", String::from_utf8_lossy(&out.stderr));
        assert!(out.stdout.is_empty(), "{pin}: a refused step printed no result");
    }

    // A step without a pin is unaffected, and a matching pin on any minor runs.
    let mut plain = create.clone();
    let mut pinned = create.clone();
    pinned["schema_version"] = serde_json::json!(format!("{major}.999"));
    let script = write_script(serde_json::json!([plain, pinned]));
    let out = run(&["run", "--script", &script]);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    assert_eq!(String::from_utf8_lossy(&out.stdout).matches("\"doc\"").count(), 2, "both steps ran");
    let _ = std::fs::remove_dir_all(&scratch);
}

#[test]
fn the_help_names_the_run_pin() {
    let help = String::from_utf8_lossy(&run(&["--help"]).stdout).into_owned();
    assert!(help.contains("pdfcraft-cli run"), "{help}");
    assert!(help.contains("--schema-version"), "{help}");
    assert!(help.contains("schema_version"), "the per-step pin is named too: {help}");
}
