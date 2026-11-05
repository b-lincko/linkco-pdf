//! Every document-changing tool carries an example, and the example survives a save and a reopen
//! (REC-B01).
//!
//! The claim this file holds is the one a caller cares about: a tool's schema shows a call that
//! works, that call is valid against the schema it sits in, it changes the document, and the change
//! is still there when the saved file is opened again by a session that has never seen it. That is
//! what "the feature works" means for anything that edits a document — an edit that only exists in
//! memory is not a feature, and an example that does not compile against its own schema is worse
//! than none.
//!
//! Three tests: the table's completeness ([`every_document_changing_tool_is_exempled_or_explained`]),
//! the examples' validity ([`every_example_matches_the_schema_it_sits_in`]), and the round trip
//! itself ([`every_example_survives_a_save_and_a_reopen`]).

use std::path::{Path, PathBuf};

use pdfcraft_automation::{Automation, Content, examples, tools};
use serde_json::Value;

/// A PDF with `n` 200×300 pt pages reading "Page 1", "Page 2", … — the fixture every example is
/// written against (document 1, three pages).
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

fn workdir(test: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pdfcraft-roundtrip-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
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

fn page_text(a: &mut Automation, doc: u64) -> Vec<String> {
    ok(a, "text_extract", serde_json::json!({ "doc": doc }))["pages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["text"].as_str().unwrap_or_default().trim().to_string())
        .collect()
}

/// The tool table, by name.
fn table() -> Vec<pdfcraft_automation::ToolDef> {
    tools()
}

// ---- the tables' completeness ------------------------------------------------------------------

#[test]
fn every_document_changing_tool_is_exempled_or_explained() {
    let mut wrong = Vec::new();
    for def in table() {
        let has_example = examples::example(def.name).is_some();
        let reason = examples::why_not(def.name);
        match (def.read_only, has_example, reason) {
            // A read-only tool changes nothing, so there is nothing to save and reopen.
            (true, false, None) => {}
            (true, _, _) => wrong.push(format!("{}: read-only and still in the example tables", def.name)),
            // A writer must have exactly one of: an example, or a stated reason.
            (false, true, None) | (false, false, Some(_)) => {}
            (false, true, Some(_)) => wrong.push(format!("{}: has an example and also a reason not to", def.name)),
            (false, false, None) => {
                wrong.push(format!("{}: changes a document but has neither an example nor a reason — add it to examples.rs", def.name))
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

// ---- the examples are valid calls ---------------------------------------------------------------

/// A small JSON Schema check, enough for the subset the table uses: unknown keys, missing required
/// keys, types, `minimum` and `enum`. It is deliberately not a general validator — it is the part
/// of the schema an example could plausibly get wrong.
fn check_against_schema(tool: &str, args: &Value) -> Result<(), String> {
    let def = table().into_iter().find(|t| t.name == tool).ok_or_else(|| format!("{tool}: not a tool"))?;
    let schema = &def.input_schema;
    let obj = args.as_object().ok_or_else(|| format!("{tool}: the example is not an object"))?;
    let properties = schema["properties"].as_object().ok_or_else(|| format!("{tool}: no properties"))?;
    for (key, value) in obj {
        let spec = properties.get(key).ok_or_else(|| format!("{tool}: the example passes {key:?}, which the schema does not have"))?;
        let kind = spec["type"].as_str().unwrap_or_default();
        let matches = match kind {
            "integer" => value.as_i64().is_some(),
            "number" => value.as_f64().is_some(),
            "string" => value.as_str().is_some(),
            "boolean" => value.as_bool().is_some(),
            "array" => value.is_array(),
            "object" => value.is_object(),
            // A schema without a type accepts anything; nothing in the table is like that, but a
            // future one should not fail here for that reason alone.
            _ => true,
        };
        if !matches {
            return Err(format!("{tool}: {key} is {value}, but the schema says {kind}"));
        }
        if let Some(min) = spec["minimum"].as_i64()
            && let Some(n) = value.as_i64()
            && n < min
        {
            return Err(format!("{tool}: {key} is {n}, under the schema's minimum of {min}"));
        }
        if let Some(allowed) = spec["enum"].as_array()
            && let Some(s) = value.as_str()
            && !allowed.iter().any(|a| a.as_str() == Some(s))
        {
            return Err(format!("{tool}: {key} is {s:?}, which is not one of the schema's values"));
        }
    }
    for required in schema["required"].as_array().into_iter().flatten() {
        let key = required.as_str().unwrap_or_default();
        if !obj.contains_key(key) {
            return Err(format!("{tool}: the example does not pass {key:?}, which the schema requires"));
        }
    }
    Ok(())
}

#[test]
fn every_example_matches_the_schema_it_sits_in() {
    let mut wrong = Vec::new();
    let mut checked = 0;
    for def in table() {
        let Some(example) = examples::example(def.name) else { continue };
        checked += 1;
        if let Err(e) = check_against_schema(def.name, &example) {
            wrong.push(e);
        }
        // The example is what the schema hands a caller, so the two must be the same object.
        let published = pdfcraft_automation::schema::schema_with_example(&def);
        assert_eq!(published["examples"][0], example, "{}: the published example is not the table's", def.name);
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    assert!(checked >= 20, "only {checked} examples: the table has lost most of them");
}

// ---- the round trip ----------------------------------------------------------------------------

/// What a tool's change should look like in the file once it has been saved and reopened. The
/// default is the invariant every edit must keep: the document still opens, still has its three
/// pages, and still reads the same. A tool with a visible effect checks that effect.
fn check(tool: &str, a: &mut Automation, doc: u64) -> Result<(), String> {
    let pages = page_text(a, doc);
    let expect = |wanted: &[&str]| -> Result<(), String> {
        let wanted: Vec<String> = wanted.iter().map(|s| (*s).to_string()).collect();
        if pages == wanted { Ok(()) } else { Err(format!("{tool}: pages read {pages:?}, expected {wanted:?}")) }
    };
    match tool {
        "page_rotate" => {
            let rotation = ok(a, "doc_info", serde_json::json!({ "doc": doc }))["pages"][0]["rotation"].clone();
            if rotation == 90 { Ok(()) } else { Err(format!("{tool}: page 1 rotation is {rotation}, expected 90")) }
        }
        "page_delete" => expect(&["Page 1", "Page 3"]),
        "page_move" => expect(&["Page 3", "Page 1", "Page 2"]),
        "page_reverse" => expect(&["Page 3", "Page 2", "Page 1"]),
        "page_duplicate" => expect(&["Page 1", "Page 1", "Page 2", "Page 3"]),
        "page_insert_blank" => expect(&["Page 1", "", "Page 2", "Page 3"]),
        "doc_set_info" => {
            let title = ok(a, "doc_info", serde_json::json!({ "doc": doc }))["title"].clone();
            if title == "Round trip" { Ok(()) } else { Err(format!("{tool}: title is {title}")) }
        }
        "page_number" => {
            let labels: Vec<Value> = ok(a, "doc_info", serde_json::json!({ "doc": doc }))["pages"].as_array().cloned().unwrap_or_default();
            let labels: Vec<String> = labels.iter().map(|p| p["label"].as_str().unwrap_or_default().to_string()).collect();
            if labels == ["i", "ii", "iii"] { Ok(()) } else { Err(format!("{tool}: labels are {labels:?}")) }
        }
        "bookmark_add" => {
            let list = ok(a, "bookmark_list", serde_json::json!({ "doc": doc }));
            let first = &list["bookmarks"][0];
            if first["title"] == "Start" && first["page"] == 2 { Ok(()) } else { Err(format!("{tool}: {list}")) }
        }
        "comment_add" => {
            let list = ok(a, "comment_list", serde_json::json!({ "doc": doc }));
            let found = list["comments"].as_array().is_some_and(|c| c.iter().any(|c| c["contents"] == "Checked"));
            if found { Ok(()) } else { Err(format!("{tool}: {list}")) }
        }
        "link_add" => {
            let list = ok(a, "link_list", serde_json::json!({ "doc": doc }));
            let found = list["links"].as_array().is_some_and(|l| l.iter().any(|l| l["url"] == "https://example.com"));
            if found { Ok(()) } else { Err(format!("{tool}: {list}")) }
        }
        "form_add_field" => {
            let list = ok(a, "form_fields", serde_json::json!({ "doc": doc }));
            let found = list["fields"].as_array().is_some_and(|f| f.iter().any(|f| f["name"] == "email"));
            if found { Ok(()) } else { Err(format!("{tool}: {list}")) }
        }
        "text_edit" => expect(&["Edited line", "Page 2", "Page 3"]),
        "page_add_text" => {
            if pages.first().is_some_and(|p| p.contains("Reviewed") && p.contains("Page 1")) {
                Ok(())
            } else {
                Err(format!("{tool}: {pages:?}"))
            }
        }
        "redact_mark" => {
            // The mark was saved with the file: applying it now works, and the box was drawn
            // clear of the text, so the page still reads as it did.
            ok(a, "redact_apply", serde_json::json!({ "doc": doc }));
            expect(&["Page 1", "Page 2", "Page 3"])
        }
        "js_set_document_script" => {
            let scripts = ok(a, "js_document_scripts", serde_json::json!({ "doc": doc }));
            let found = scripts["scripts"].as_array().is_some_and(|s| s.iter().any(|s| s["name"] == "hello"));
            if found { Ok(()) } else { Err(format!("{tool}: {scripts}")) }
        }
        // Everything else: the change is a mark, a label, a box, a colour, a conversion or a script,
        // so the claim is that the document still opens with its three pages and its own text
        // intact. (`contains`, not equality: a watermark or a header is drawn as page content and
        // shows up in the text just as a reader would see it.)
        _ => {
            if pages.len() != 3 {
                return Err(format!("{tool}: {pages:?} — expected three pages"));
            }
            for (i, page) in pages.iter().enumerate() {
                if !page.contains(&format!("Page {}", i + 1)) {
                    return Err(format!("{tool}: page {} reads {page:?}, which has lost its text", i + 1));
                }
            }
            Ok(())
        }
    }
}

#[test]
fn every_example_survives_a_save_and_a_reopen() {
    let tools_with_examples: Vec<&'static str> = table().iter().filter(|t| examples::example(t.name).is_some()).map(|t| t.name).collect();
    assert!(tools_with_examples.len() >= 20, "only {} tools carry an example", tools_with_examples.len());
    let mut failures = Vec::new();
    for tool in tools_with_examples {
        let dir = workdir(tool);
        std::fs::write(dir.join("in.pdf"), fixture(3)).unwrap();
        let args = examples::example(tool).unwrap();
        let mut a = auto(&dir);
        let doc = ok(&mut a, "doc_open", serde_json::json!({ "path": "in.pdf" }))["doc"].as_u64().unwrap();
        assert_eq!(doc, 1, "{tool}: the example is written for document 1");
        if let Err(e) = a.call(tool, &args) {
            failures.push(format!("{tool} {args}: {e}"));
            continue;
        }
        ok(&mut a, "doc_save", serde_json::json!({ "doc": doc, "path": "out.pdf" }));
        drop(a);

        // A session that has never seen this document: it only has the file.
        let mut b = auto(&dir);
        // An encrypted document needs its password; that is the point of protecting it.
        let mut open = serde_json::json!({ "path": "out.pdf" });
        if tool == "doc_protect" {
            open["password"] = serde_json::json!("secret");
        }
        let reopened = match b.call("doc_open", &open) {
            Ok(mut c) => match c.remove(0) {
                Content::Json(v) => v["doc"].as_u64().unwrap(),
                other => panic!("{tool}: {other:?}"),
            },
            Err(e) => {
                failures.push(format!("{tool}: the saved file does not open again: {e}"));
                continue;
            }
        };
        if tool == "doc_protect" {
            // It is encrypted, so the file must refuse to open without the password.
            if b.call("doc_open", &serde_json::json!({ "path": "out.pdf" })).is_ok() {
                failures.push(format!("{tool}: the saved file opened without its password"));
            }
            continue;
        }
        if let Err(e) = check(tool, &mut b, reopened) {
            failures.push(e);
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
