//! Checking whether a form is filled in, through the same tools the CLI and MCP use (REC-F01):
//! the app draws a required field with a red border, but nothing could say which ones were still
//! empty — so a caller had no way to guide someone through a form or refuse an incomplete one.

use pdfcraft_automation::{Automation, Content};
use pdfcraft_engine::{Edit, FieldValue, NewField, Session};
use serde_json::{Value, json};

fn tool(a: &mut Automation, name: &str, args: Value) -> Value {
    match a.call(name, &args).unwrap().remove(0) {
        Content::Json(v) => v,
        other => panic!("{name}: expected JSON: {other:?}"),
    }
}

fn auto() -> Automation {
    Automation::new().with_clock(|| 1_700_000_000)
}

fn workdir(test: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("pdfcraft-form-done-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A one-page form with a required text field, a required check box and an optional text field,
/// written to `dir/form.pdf` and returned as a path.
fn seeded(dir: &std::path::Path) -> String {
    let mut s = Session::new();
    let bytes = s.create_blank(300.0, 400.0, 1).unwrap();
    let id = s.open("form.pdf", None, bytes, None).unwrap();
    let text = NewField::Text { multiline: false };
    for (i, name) in ["given", "surname", "notes"].iter().enumerate() {
        let rect = [30.0, 320.0 - 30.0 * i as f64, 210.0, 344.0 - 30.0 * i as f64];
        s.apply(id, Edit::AddField { page: 0, rect, kind: text.clone(), name: Some((*name).into()) }).unwrap();
    }
    s.apply(
        id,
        Edit::AddField { page: 0, rect: [30.0, 230.0, 44.0, 244.0], kind: NewField::CheckBox, name: Some("agree".into()) },
    )
    .unwrap();
    // Required: two text fields and the check box; `notes` stays optional.
    s.apply(
        id,
        Edit::SetFieldProps {
            name: "surname".into(),
            props: Box::new(pdfcraft_engine::FieldProps { required: Some(true), tooltip: Some("Your family name".into()), ..Default::default() }),
        },
    )
    .unwrap();
    s.apply(
        id,
        Edit::SetFieldProps { name: "given".into(), props: Box::new(pdfcraft_engine::FieldProps { required: Some(true), ..Default::default() }) },
    )
    .unwrap();
    s.apply(
        id,
        Edit::SetFieldProps { name: "agree".into(), props: Box::new(pdfcraft_engine::FieldProps { required: Some(true), ..Default::default() }) },
    )
    .unwrap();
    s.apply(id, Edit::SetFieldValue { name: "given".into(), value: FieldValue::Text("Ada".into()) }).unwrap();
    let path = dir.join("form.pdf");
    std::fs::write(&path, &*s.save_bytes(id).unwrap()).unwrap();
    path.to_string_lossy().into_owned()
}

#[test]
fn an_empty_form_names_what_is_left_in_the_order_to_fill_it() {
    let dir = workdir("guide");
    let path = seeded(&dir);
    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path }))["doc"].as_u64().unwrap();
    let out = tool(&mut a, "form_completion", json!({ "doc": doc }));

    assert_eq!(out["complete"], json!(false), "{out}");
    assert_eq!(out["remaining"], json!(2), "{out}");
    assert_eq!(out["required"], json!(3), "{out}");
    assert_eq!(out["fields"], json!(4), "every field counts, required or not: {out}");
    // `given` is answered, `notes` is not required, so what is left is `surname` then `agree`.
    let missing = out["missing"].as_array().unwrap();
    assert_eq!(missing.iter().map(|m| m["name"].as_str().unwrap()).collect::<Vec<_>>(), ["surname", "agree"], "{out}");
    assert_eq!(out["next"], "surname", "the guide points at the first one: {out}");
    assert_eq!(missing[0]["step"], json!(1), "{out}");
    assert_eq!(missing[0]["page"], json!(1), "1-based: {out}");
    assert_eq!(missing[0]["tooltip"], "Your family name", "{out}");
    assert_eq!(missing[1]["step"], json!(2), "{out}");
}

/// Filling in what is asked for makes the form complete, and the answer says so.
#[test]
fn a_completed_form_says_it_is_complete() {
    let dir = workdir("done");
    let path = seeded(&dir);
    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path }))["doc"].as_u64().unwrap();
    tool(&mut a, "form_fill", json!({ "doc": doc, "values": { "surname": "Lovelace", "agree": true } }));
    let out = tool(&mut a, "form_completion", json!({ "doc": doc }));
    assert_eq!(out["complete"], json!(true), "{out}");
    assert_eq!(out["remaining"], json!(0), "{out}");
    assert!(out["next"].is_null(), "nothing left to point at: {out}");
    assert_eq!(out["missing"].as_array().unwrap().len(), 0, "{out}");
}

/// Whitespace is not an answer, and checking changes nothing — no undo step, no dirty flag.
#[test]
fn checking_reads_the_live_state_and_changes_nothing() {
    let dir = workdir("readonly");
    let path = seeded(&dir);
    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path }))["doc"].as_u64().unwrap();
    let before = tool(&mut a, "form_completion", json!({ "doc": doc }));
    assert_eq!(before["remaining"], json!(2), "{before}");

    // An unsaved entry counts straight away: the check reads the editor, not the file.
    tool(&mut a, "form_fill", json!({ "doc": doc, "values": { "surname": "Lovelace" } }));
    let after = tool(&mut a, "form_completion", json!({ "doc": doc }));
    assert_eq!(after["remaining"], json!(1), "the unsaved entry counts: {after}");
    assert_eq!(after["next"], "agree", "{after}");

    // The check itself made no step: the only undo entry is the fill that came before it.
    let listed = tool(&mut a, "doc_info", json!({ "doc": doc }));
    // A single value is its own edit, so the label is the one that edit carries.
    assert_eq!(listed["document"]["undo"], "Fill in surname", "the check made no step of its own: {listed}");
}

/// A document with no form asks for nothing, so it is complete rather than an error.
#[test]
fn a_document_without_a_form_is_complete() {
    let dir = workdir("plain");
    let s = Session::new();
    let bytes = s.create_blank(300.0, 400.0, 1).unwrap();
    let path = dir.join("plain.pdf");
    std::fs::write(&path, &*bytes).unwrap();
    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path.to_string_lossy() }))["doc"].as_u64().unwrap();
    let out = tool(&mut a, "form_completion", json!({ "doc": doc }));
    assert_eq!(out["complete"], json!(true), "{out}");
    assert_eq!(out["fields"], json!(0), "{out}");
}

#[test]
fn the_tool_is_in_the_catalogue_read_only_and_says_what_it_does() {
    let def = pdfcraft_automation::tools().into_iter().find(|t| t.name == "form_completion").expect("in the catalogue");
    assert!(def.read_only, "checking a form changes nothing");
    assert!(!def.destructive, "and destroys nothing");
    assert_eq!(def.command, Some("form.fields"), "it belongs to the Form fields command");
    assert!(def.description.contains("tab order"), "{}", def.description);
}
