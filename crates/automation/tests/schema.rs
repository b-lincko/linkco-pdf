//! The versioned tool schema and the stable failure kinds (REC-B02): what the document says about
//! the table, and that a caller can act on a failure without reading its message.

use std::collections::HashSet;

use pdfcraft_automation::{TOOL_SCHEMA_VERSION, ToolError, schema, tools};
use serde_json::{Value, json};

#[test]
fn the_document_describes_the_table_it_is_built_from() {
    let document = schema::schema_document();
    let listed = document["tools"].as_array().unwrap();
    let all = tools();
    assert_eq!(listed.len(), all.len(), "the document must list every tool, and nothing else");
    assert_eq!(document["tool_count"].as_u64().unwrap() as usize, all.len());
    for (entry, def) in listed.iter().zip(all.iter()) {
        assert_eq!(entry["name"], def.name);
        assert_eq!(entry["read_only"], def.read_only);
        assert_eq!(entry["destructive"], def.destructive);
        // The schema in the document is the schema the tool actually takes: one copy, not two.
        assert_eq!(entry["input_schema"], def.input_schema, "{}", def.name);
    }
}

#[test]
fn every_tool_is_callable_by_a_name_that_is_unique_and_has_a_schema() {
    let mut seen = HashSet::new();
    for def in tools() {
        assert!(seen.insert(def.name), "two tools are called {}", def.name);
        assert!(!def.name.is_empty() && def.name.chars().all(|c| c.is_ascii_lowercase() || c == '_'), "{}", def.name);
        let s = &def.input_schema;
        assert_eq!(s["type"], "object", "{}: the arguments are an object", def.name);
        let properties = s["properties"].as_object().unwrap_or_else(|| panic!("{}: no properties", def.name));
        for required in s.get("required").and_then(Value::as_array).into_iter().flatten() {
            let key = required.as_str().unwrap();
            assert!(properties.contains_key(key), "{}: requires {key:?} but does not describe it", def.name);
        }
        for (key, spec) in properties {
            assert!(spec.is_object(), "{}: {key} has no schema", def.name);
        }
    }
}

#[test]
fn the_version_is_well_formed_and_still_the_first_major() {
    // A major bump is a breaking change: it must be a decision someone made here, not a typo.
    let (major, minor) = schema::parse_version(TOOL_SCHEMA_VERSION).expect("TOOL_SCHEMA_VERSION is major.minor");
    assert_eq!(major, 1, "bumping the major version breaks every caller; record why in progress.toml");
    assert!(minor < 100, "the minor version looks like a mistake: {TOOL_SCHEMA_VERSION}");
}

#[test]
fn every_failure_kind_is_the_one_documented() {
    // The vocabulary the command line uses for its exit codes, so one script can act on either.
    let documented = [
        (ToolError::UnknownTool("nope".into()), "unsupported"),
        (ToolError::InvalidArgs("page 0 is out of range".into()), "usage"),
        (ToolError::Refused("the read-only profile refuses doc_save".into()), "refused"),
        (ToolError::Failed("in.pdf: not a PDF".into()), "failed"),
    ];
    for (error, kind) in documented {
        assert_eq!(error.kind(), kind, "{error}");
        let json = error.json();
        assert_eq!(json["kind"], kind);
        // The message a person reads is the same one the display form gives.
        assert_eq!(json["message"], error.to_string());
    }
    // No two kinds share a name, so a client's match arms cannot be ambiguous.
    let kinds: HashSet<&str> = documented.iter().map(|(e, _)| e.kind()).collect();
    assert_eq!(kinds.len(), documented.len());
}

#[test]
fn the_schema_document_is_json_a_client_can_read() {
    // Round-trip through text, the way a client receives it.
    let text = serde_json::to_string(&schema::schema_document()).unwrap();
    let read: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(read["schema_version"], json!(TOOL_SCHEMA_VERSION));
    assert!(read["tools"].as_array().unwrap().iter().any(|t| t["name"] == "doc_open"));
    assert_eq!(schema::parse_version(TOOL_SCHEMA_VERSION).is_some(), true);
}
