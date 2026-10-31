//! `pdfcraft-cli mcp --profile …` and `--approve …`: the flags reach the session, narrow what it
//! lists and runs, and misuse is a usage error rather than a quietly wider server (REC-B09).
#![cfg(feature = "mcp")]

use std::io::Write as _;
use std::process::{Command, Output, Stdio};

use serde_json::{Value, json};

fn run(flags: &[&str], requests: &[Value]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_pdfcraft-cli"))
        .arg("mcp")
        .args(flags)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = String::from("{\"jsonrpc\":\"2.0\",\"id\":0,\"method\":\"initialize\",\"params\":{}}\n");
    for r in requests {
        input.push_str(&r.to_string());
        input.push('\n');
    }
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    child.wait_with_output().unwrap()
}

/// The replies to `requests`, skipping the answer to `initialize`.
fn session(flags: &[&str], requests: &[Value]) -> Vec<Value> {
    let out = run(flags, requests);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).unwrap().lines().skip(1).map(|l| serde_json::from_str(l).unwrap()).collect()
}

fn request(id: u64, method: &str, params: Value) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params })
}

fn listed_tools(flags: &[&str]) -> Vec<String> {
    let reply = session(flags, &[request(1, "tools/list", json!({}))]).remove(0);
    reply["result"]["tools"].as_array().unwrap().iter().map(|t| t["name"].as_str().unwrap().to_string()).collect()
}

fn read_only_tools() -> Vec<String> {
    let mut names: Vec<String> = pdfcraft_automation::tools().into_iter().filter(|t| t.read_only).map(|t| t.name.to_string()).collect();
    names.sort();
    names
}

#[test]
fn the_read_only_profile_lists_and_runs_only_tools_that_change_nothing() {
    let mut listed = listed_tools(&["--profile", "read-only"]);
    listed.sort();
    assert_eq!(listed, read_only_tools(), "a read-only server must not advertise a tool it will refuse");

    let reply = session(&["--profile", "read-only"], &[request(1, "tools/call", json!({ "name": "doc_save", "arguments": { "doc": 1 } }))]);
    let result = &reply[0]["result"];
    assert_eq!(result["isError"], true, "{result}");
    let text = result["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("read-only profile"), "{text}");
    // The same session says so about itself when asked, in a tool it may run.
    let reply = session(&["--profile", "read-only"], &[request(1, "tools/call", json!({ "name": "policy_status", "arguments": {} }))]);
    let status = reply[0]["result"]["content"][0]["text"].as_str().unwrap();
    assert!(status.contains("\"read-only\""), "{status}");
}

#[test]
fn the_project_profile_needs_a_root_and_approves_destructive_tools_by_name() {
    // Without a folder there is nothing for "project" to mean, so it is refused as a usage error.
    let out = run(&["--profile", "project"], &[]);
    assert_eq!(out.status.code(), Some(2), "{}", String::from_utf8_lossy(&out.stderr));
    assert!(String::from_utf8_lossy(&out.stderr).contains("needs a root"));

    let dir = std::env::temp_dir().join(format!("pdfcraft-cli-profile-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let root = dir.to_string_lossy().into_owned();

    let listed = listed_tools(&["--root", &root, "--profile", "project", "--approve", "doc_save"]);
    assert!(listed.iter().any(|n| n == "doc_save"), "the approved tool is listed");
    assert!(!listed.iter().any(|n| n == "action_run"), "an unapproved destructive tool is not");
    let reply = session(
        &["--root", &root, "--profile", "project", "--approve", "doc_save"],
        &[
            request(1, "tools/call", json!({ "name": "policy_status", "arguments": {} })),
            request(2, "tools/call", json!({ "name": "action_run", "arguments": { "paths": ["a.pdf"], "folder": "out" } })),
        ],
    );
    let status: Value = serde_json::from_str(reply[0]["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(status["profile"], "project");
    assert_eq!(status["approved"], json!(["doc_save"]));
    let refused = reply[1]["result"]["content"][0]["text"].as_str().unwrap();
    assert!(refused.contains("--approve action_run"), "{refused}");

    // `--approve` takes several names, and checks each one.
    let listed = listed_tools(&["--root", &root, "--profile", "project", "--approve", "doc_save,image_save"]);
    assert!(listed.iter().any(|n| n == "image_save"));
    for (flag, expected) in [("no_such_tool", "no tool"), ("doc_info", "not destructive")] {
        let out = run(&["--root", &root, "--profile", "project", "--approve", flag], &[]);
        assert_eq!(out.status.code(), Some(2), "{flag}: {}", String::from_utf8_lossy(&out.stderr));
        assert!(String::from_utf8_lossy(&out.stderr).contains(expected), "{flag}: {}", String::from_utf8_lossy(&out.stderr));
    }
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn an_unknown_profile_is_a_usage_error() {
    let out = run(&["--profile", "everything"], &[]);
    assert_eq!(out.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("read-only, project, destructive"), "{stderr}");
}
