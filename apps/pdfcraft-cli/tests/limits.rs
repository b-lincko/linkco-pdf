//! `pdfcraft-cli run|mcp --max-workers/--max-input-mib/--max-output-mib/--max-batch/--time-limit-ms`:
//! the flags reach the session, and a bad value is a usage error (REC-B07).
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

#[test]
fn the_limits_flags_reach_the_session() {
    let out = run(
        &["--max-workers", "2", "--max-input-mib", "1", "--max-output-mib", "3", "--max-batch", "5", "--time-limit-ms", "250"],
        &[json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "limits_status", "arguments": {} } })],
    );
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let reply: Value = serde_json::from_str(String::from_utf8(out.stdout).unwrap().lines().nth(1).unwrap()).unwrap();
    let status: Value = serde_json::from_str(reply["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(status["limits"]["workers"].as_u64(), Some(2), "{status}");
    assert_eq!(status["limits"]["max_input_bytes"].as_u64(), Some(1 << 20));
    assert_eq!(status["limits"]["max_output_bytes"].as_u64(), Some(3 << 20));
    assert_eq!(status["limits"]["max_batch_items"].as_u64(), Some(5));
    assert_eq!(status["limits"]["item_timeout_ms"].as_u64(), Some(250));
}

#[test]
fn a_bad_limit_is_a_usage_error() {
    for (flag, value) in [("--max-workers", "0"), ("--max-batch", "many"), ("--max-input-mib", "-1")] {
        let out = run(&[flag, value], &[]);
        assert_eq!(out.status.code(), Some(2), "{flag} {value}: {}", String::from_utf8_lossy(&out.stderr));
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(stderr.contains(flag), "{stderr}");
    }
}

#[test]
fn a_session_without_the_flags_reports_no_limits() {
    let out = run(&[], &[json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "limits_status", "arguments": {} } })]);
    assert!(out.status.success());
    let reply: Value = serde_json::from_str(String::from_utf8(out.stdout).unwrap().lines().nth(1).unwrap()).unwrap();
    let status: Value = serde_json::from_str(reply["result"]["content"][0]["text"].as_str().unwrap()).unwrap();
    assert_eq!(status["limits"]["max_input_bytes"].as_u64(), Some(0), "{status}");
    let workers = status["limits"]["workers"].as_u64().unwrap();
    assert!((1..=8).contains(&workers), "{status}");
}
