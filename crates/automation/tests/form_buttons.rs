//! Pressing a push button through the same tools the CLI and MCP use (REC-F01): the app has always
//! run a button's action when it is clicked, but there was no way to ask for that click without a
//! window — so a script that filled a form could not use the form's own Reset button.

use pdfcraft_automation::{Automation, Content};
use pdfcraft_engine::{Edit, FieldAction, FieldTrigger, FieldValue, NewField, Session};
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
    let dir = std::env::temp_dir().join(format!("pdfcraft-form-btn-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A one-page form with a text field holding a value that differs from its default, and a button
/// labelled `Clear` that the caller gives an action to. Returns the path to the written form.
fn seeded(dir: &std::path::Path, action: Option<FieldAction>) -> String {
    let mut s = Session::new();
    let bytes = s.create_blank(300.0, 400.0, 1).unwrap();
    let id = s.open("form.pdf", None, bytes, None).unwrap();
    let text = NewField::Text { multiline: false };
    let button = NewField::Button { caption: "Clear".into() };
    s.apply(id, Edit::AddField { page: 0, rect: [30.0, 300.0, 210.0, 324.0], kind: text, name: Some("alpha".into()) }).unwrap();
    s.apply(id, Edit::AddField { page: 0, rect: [30.0, 260.0, 210.0, 284.0], kind: button, name: Some("clear".into()) }).unwrap();
    s.apply(id, Edit::SetFieldValue { name: "alpha".into(), value: FieldValue::Text("Ada Lovelace".into()) }).unwrap();
    if let Some(action) = action {
        s.apply(
            id,
            Edit::SetFieldProps {
                name: "clear".into(),
                props: Box::new(pdfcraft_engine::FieldProps {
                    actions: Some(vec![(FieldTrigger::MouseUp, action)]),
                    ..Default::default()
                }),
            },
        )
        .unwrap();
    }
    let path = dir.join("form.pdf");
    std::fs::write(&path, &*s.save_bytes(id).unwrap()).unwrap();
    path.to_string_lossy().into_owned()
}

#[test]
fn a_reset_button_clears_the_form_and_is_one_undo_step() {
    let dir = workdir("reset");
    let path = seeded(&dir, Some(FieldAction::Reset(Vec::new())));
    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path }))["doc"].as_u64().unwrap();

    let out = tool(&mut a, "form_run_button", json!({ "doc": doc, "field": "clear" }));
    assert_eq!(out["did"], "reset", "{out}");
    assert_eq!(out["changed"], json!(true), "{out}");
    assert_eq!(out["field"], "clear", "{out}");
    let fields = out["fields"].as_array().unwrap();
    assert_eq!(fields, &[json!("alpha")], "the field that held a value is the one that moved: {out}");

    assert_eq!(out["document"]["undo"], json!(true), "the reset is one undo step: {out}");
    // The value really went back to its default.
    let listed = tool(&mut a, "form_fields", json!({ "doc": doc }));
    let alpha = listed.as_array().unwrap().iter().find(|f| f["name"] == "alpha").expect("alpha");
    assert!(alpha["value"].is_null() || alpha["value"] == "", "the field is back to its default: {alpha}");
}

#[test]
fn a_button_whose_action_needs_a_viewer_reports_it_and_changes_nothing() {
    let dir = workdir("viewer");
    let path = seeded(&dir, Some(FieldAction::Uri("https://example.com/report".into())));
    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path }))["doc"].as_u64().unwrap();

    let out = tool(&mut a, "form_run_button", json!({ "doc": doc, "field": "clear" }));
    assert_eq!(out["did"], "url", "{out}");
    assert_eq!(out["url"], "https://example.com/report", "{out}");
    assert_eq!(out["changed"], json!(false), "opening a URL changes the document nowhere: {out}");
    // No undo step was made for an action that changed nothing.
    assert_eq!(out["document"]["undo"], json!(false), "nothing was pushed onto the undo stack: {out}");
    assert_eq!(out["document"]["dirty"], json!(false), "nothing is unsaved: {out}");
}

#[test]
fn a_submit_action_is_never_sent() {
    let dir = workdir("submit");
    let path = seeded(&dir, Some(FieldAction::Submit("https://example.com/form".into())));
    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path }))["doc"].as_u64().unwrap();
    let out = tool(&mut a, "form_run_button", json!({ "doc": doc, "field": "clear" }));
    assert_eq!(out["did"], "submit", "{out}");
    assert_eq!(out["url"], "https://example.com/form", "{out}");
    assert_eq!(out["sent"], json!(false), "the tool says plainly that nothing was sent: {out}");
    assert_eq!(out["changed"], json!(false), "{out}");
}

#[test]
fn a_field_that_is_not_a_push_button_is_refused() {
    let dir = workdir("refused");
    let path = seeded(&dir, Some(FieldAction::Reset(Vec::new())));
    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path }))["doc"].as_u64().unwrap();
    let err = a.call("form_run_button", &json!({ "doc": doc, "field": "alpha" })).unwrap_err();
    assert!(err.to_string().contains("not a push button"), "{err}");
    let err = a.call("form_run_button", &json!({ "doc": doc, "field": "nope" })).unwrap_err();
    assert!(err.to_string().contains("nope"), "{err}");
}

/// A button the form author left without an action is pressable; it simply does nothing.
#[test]
fn a_button_with_no_action_does_nothing() {
    let dir = workdir("inert");
    let path = seeded(&dir, None);
    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path }))["doc"].as_u64().unwrap();
    let out = tool(&mut a, "form_run_button", json!({ "doc": doc, "field": "clear" }));
    assert_eq!(out["did"], "nothing", "{out}");
    assert_eq!(out["changed"], json!(false), "{out}");
}

/// The whole point: a script that fills a form can then press its own Reset button, and the answer
/// says what that click did.
#[test]
fn a_filled_form_can_be_cleared_by_its_own_button() {
    let dir = workdir("workflow");
    let path = seeded(&dir, Some(FieldAction::Reset(Vec::new())));
    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path }))["doc"].as_u64().unwrap();
    // The field starts with a value that is not its default.
    let before = tool(&mut a, "form_fields", json!({ "doc": doc }));
    let alpha = before.as_array().unwrap().iter().find(|f| f["name"] == "alpha").expect("alpha");
    assert_eq!(alpha["value"], "Ada Lovelace", "{alpha}");

    tool(&mut a, "form_run_button", json!({ "doc": doc, "field": "clear" }));

    let after = tool(&mut a, "form_fields", json!({ "doc": doc }));
    let alpha = after.as_array().unwrap().iter().find(|f| f["name"] == "alpha").expect("alpha");
    assert!(alpha["value"].is_null() || alpha["value"] == "", "the button cleared it: {alpha}");
    // What the button did survives the round trip through bytes.
    let saved = tool(&mut a, "doc_save", json!({ "doc": doc, "path": dir.join("out.pdf").to_string_lossy() }));
    assert!(saved["bytes"].as_u64().unwrap() > 0, "{saved}");
}

/// An image field is a push button the app itself creates, with a JavaScript action that asks for
/// a picture. Pressing it reports the request and changes nothing — the app never invents one.
#[test]
fn an_image_field_reports_the_image_request() {
    let dir = workdir("image");
    let mut s = Session::new();
    let bytes = s.create_blank(300.0, 400.0, 1).unwrap();
    let id = s.open("image.pdf", None, bytes, None).unwrap();
    let rect = [30.0, 300.0, 210.0, 324.0];
    s.apply(id, Edit::AddField { page: 0, rect, kind: NewField::Image, name: Some("photo".into()) }).unwrap();
    let path = dir.join("image.pdf");
    std::fs::write(&path, &*s.save_bytes(id).unwrap()).unwrap();

    let mut a = auto();
    let doc = tool(&mut a, "doc_open", json!({ "path": path.to_string_lossy() }))["doc"].as_u64().unwrap();
    let out = tool(&mut a, "form_run_button", json!({ "doc": doc, "field": "photo" }));
    assert_eq!(out["did"], "import_icon", "{out}");
    assert_eq!(out["changed"], json!(false), "{out}");
    assert_eq!(out["document"]["undo"], json!(false), "no step was made: {out}");
}

#[test]
fn the_tool_is_in_the_catalogue_and_says_what_it_does() {
    let def = pdfcraft_automation::tools().into_iter().find(|t| t.name == "form_run_button").expect("in the catalogue");
    assert!(!def.destructive, "pressing a button destroys nothing by itself");
    assert!(def.description.contains("undo step"), "{}", def.description);
    assert!(def.description.contains("never"), "{}", def.description);
}
