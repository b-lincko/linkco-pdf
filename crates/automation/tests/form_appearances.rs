//! Regenerating field appearances through the same tools the CLI and MCP use (REC-F01): a form
//! whose fields carry a value but no appearance shows nothing, and there was no way to fix it.

use std::sync::Arc;

use pdfcraft_automation::{Automation, Content};
use pdfcraft_cos::{Document, write_full};
use pdfcraft_engine::{Edit, FieldValue, NewField, Session, form_fields};
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
    let dir = std::env::temp_dir().join(format!("pdfcraft-form-ap-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A one-page form with two filled text fields, written to `dir/form.pdf`.
fn seeded(dir: &std::path::Path) -> String {
    let mut s = Session::new();
    let bytes = s.create_blank(300.0, 400.0, 1).unwrap();
    let id = s.open("form.pdf", None, bytes, None).unwrap();
    for (i, name) in ["alpha", "beta"].iter().enumerate() {
        s.apply(
            id,
            Edit::AddField {
                page: 0,
                rect: [30.0, 300.0 - 30.0 * i as f64, 210.0, 324.0 - 30.0 * i as f64],
                kind: NewField::Text { multiline: false },
                name: Some((*name).into()),
            },
        )
        .unwrap();
    }
    let edit = Edit::Batch {
        label: "seed".into(),
        edits: vec![
            Edit::SetFieldValue { name: "alpha".into(), value: FieldValue::Text("Ada Lovelace".into()) },
            Edit::SetFieldValue { name: "beta".into(), value: FieldValue::Text("Grace Hopper".into()) },
        ],
    };
    s.apply(id, edit).unwrap();
    let path = dir.join("form.pdf");
    std::fs::write(&path, &*s.save_bytes(id).unwrap()).unwrap();
    path.to_string_lossy().into_owned()
}

/// Remove every widget's `/AP`, as a producer that writes `/V` only leaves the file: the values
/// are in the form, and nothing on the page shows them.
fn strip_appearances(path: &str) {
    let mut cos = Document::open(Arc::new(std::fs::read(path).unwrap())).unwrap();
    let all = form_fields(&cos);
    assert_eq!(all.len(), 2);
    for f in &all {
        for w in &f.widgets {
            cos.update_dict(w.obj, |d| {
                d.remove(b"AP");
            })
            .unwrap();
        }
    }
    std::fs::write(path, write_full(&cos, &Default::default()).unwrap()).unwrap();
}

#[test]
fn a_field_value_without_an_appearance_is_made_visible() {
    let dir = workdir("visible");
    let path = seeded(&dir);
    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path }))["doc"].as_u64().unwrap();
    // The seeded form already draws its values, so a regenerate finds them all current.
    let first = tool(&mut a, "form_regenerate_appearances", json!({ "doc": doc }));
    assert_eq!(first["drawn"], 0, "the seeded appearances are already right: {first}");

    strip_appearances(&path);
    let doc = tool(&mut a, "doc_open", json!({ "path": path }))["doc"].as_u64().unwrap();
    let out = tool(&mut a, "form_regenerate_appearances", json!({ "doc": doc }));
    assert_eq!(out["drawn"], 2, "both fields needed an appearance: {out}");
    assert_eq!(out["missing"].as_array().unwrap().len(), 2, "{out}");
    assert!(out["stale"].as_array().unwrap().is_empty(), "{out}");
    assert!(out["unsupported"].as_array().unwrap().is_empty(), "{out}");
    // Nothing is left to do: what was drawn matches what the fields say.
    let again = tool(&mut a, "form_regenerate_appearances", json!({ "doc": doc }));
    assert_eq!(again["drawn"], 0, "a second run draws nothing: {again}");
}

#[test]
fn regenerating_can_be_limited_and_an_unknown_field_is_an_error() {
    let dir = workdir("limited");
    let path = seeded(&dir);
    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path }))["doc"].as_u64().unwrap();
    let out = tool(&mut a, "form_regenerate_appearances", json!({ "doc": doc, "fields": ["alpha"] }));
    assert_eq!(out["drawn"], 0, "{out}");
    // A name that is not a field is refused.
    let err = a.call("form_regenerate_appearances", &json!({ "doc": doc, "fields": ["nope"] })).unwrap_err();
    assert!(err.to_string().contains("nope"), "{err}");
}

#[test]
fn a_document_without_a_form_says_so() {
    let dir = workdir("none");
    let s = Session::new();
    let bytes = s.create_blank(300.0, 400.0, 1).unwrap();
    let path = dir.join("plain.pdf");
    std::fs::write(&path, &*bytes).unwrap();
    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path.to_string_lossy() }))["doc"].as_u64().unwrap();
    let err = a.call("form_regenerate_appearances", &json!({ "doc": doc })).unwrap_err();
    assert!(err.to_string().to_lowercase().contains("form"), "{err}");
}

#[test]
fn the_tool_is_in_the_catalogue_and_says_what_it_does() {
    let def = pdfcraft_automation::tools().into_iter().find(|t| t.name == "form_regenerate_appearances").expect("in the catalogue");
    assert!(!def.destructive, "redrawing an appearance destroys nothing");
    assert!(def.description.contains("missing"), "{}", def.description);
}
