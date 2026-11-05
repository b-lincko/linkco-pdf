//! A failed MCP call says *what kind* of failure it was, and the server names its schema version
//! (REC-B02). A client branches on `structuredContent.error.kind`, not on the words.
#![cfg(feature = "mcp")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

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

/// Every reply, `initialize`'s first.
fn session(flags: &[&str], requests: &[Value]) -> Vec<Value> {
    let out = run(flags, requests);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).unwrap().lines().map(|l| serde_json::from_str(l).unwrap()).collect()
}

fn call(flags: &[&str], name: &str, arguments: Value) -> Value {
    let replies = session(flags, &[json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": name, "arguments": arguments } })]);
    replies[1]["result"].clone()
}

#[test]
fn the_server_names_its_schema_version_in_both_places() {
    let replies = session(&[], &[json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {} })]);
    let version = pdfcraft_automation::TOOL_SCHEMA_VERSION;
    assert_eq!(replies[0]["result"]["serverInfo"]["toolSchemaVersion"], version, "{}", replies[0]);
    assert_eq!(replies[1]["result"]["_meta"]["schemaVersion"], version, "{}", replies[1]);
}

#[test]
fn a_failed_call_carries_its_kind_not_only_its_message() {
    // `usage`: the arguments do not match what the tool takes.
    let result = call(&[], "page_render", json!({ "doc": "not a number" }));
    assert_eq!(result["isError"], true, "{result}");
    assert_eq!(result["structuredContent"]["error"]["kind"], "usage", "{result}");
    let message = result["structuredContent"]["error"]["message"].as_str().expect("a message");
    assert!(!message.is_empty(), "{result}");
    // The text block is still there for a client that only reads text, and it is the same message:
    // the kind is the machine-readable half, not a second, drifting description.
    assert_eq!(result["content"][0]["type"], "text");
    assert_eq!(result["content"][0]["text"], message, "{result}");

    // `failed`: the tool ran and could not finish.
    let result = call(&[], "doc_open", json!({ "path": "/nonexistent/in.pdf" }));
    assert_eq!(result["structuredContent"]["error"]["kind"], "failed", "{result}");
}

#[test]
fn a_refused_tool_says_refused_and_an_unknown_one_says_unsupported() {
    // The profile refuses it: `refused`, with the message naming the profile.
    let result = call(&["--profile", "read-only"], "doc_save", json!({ "doc": 1 }));
    assert_eq!(result["isError"], true, "{result}");
    assert_eq!(result["structuredContent"]["error"]["kind"], "refused", "{result}");
    assert!(result["content"][0]["text"].as_str().unwrap().contains("read-only profile"), "{result}");

    // A name that is not a tool at all: a protocol error, because the request itself is wrong.
    let replies = session(&[], &[json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "no_such_tool" } })]);
    assert_eq!(replies[1]["error"]["code"], -32602, "{}", replies[1]);
}

#[test]
fn the_compact_meta_tools_carry_the_kind_too() {
    // `tool_call` around a tool that does not exist: `unsupported`.
    let result = call(&["--compact"], "tool_call", json!({ "name": "no_such_tool", "arguments": {} }));
    assert_eq!(result["structuredContent"]["error"]["kind"], "unsupported", "{result}");

    // `tool_search` for a missing argument: `usage`.
    let result = call(&["--compact"], "tool_search", json!({ "query": 7 }));
    assert_eq!(result["structuredContent"]["error"]["kind"], "usage", "{result}");

    // `tool_search` naming a tool the profile refuses: `refused`, not "unknown".
    let result = call(&["--compact", "--profile", "read-only"], "tool_search", json!({ "name": "doc_save" }));
    assert_eq!(result["structuredContent"]["error"]["kind"], "refused", "{result}");
}
