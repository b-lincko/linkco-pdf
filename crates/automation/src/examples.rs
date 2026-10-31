//! One worked example per tool that changes a document (REC-B01).
//!
//! Every tool whose schema a caller has to guess at now carries a call that is known to work: it is
//! inserted into the tool's schema as `examples` (see [`crate::schema::tool_json`]), so an agent or
//! a person reading `pdfcraft-cli schema` can copy it and get a real edit. The examples are not
//! decoration: `crates/automation/tests/roundtrip.rs` runs each one against a three-page document,
//! saves, reopens the file in a fresh session and checks the change is still there. An example that
//! stopped working fails that test.
//!
//! **Every tool that changes a document is either here or in [`why_not`] with a reason.** A test
//! holds the table to that: a new tool cannot be added without someone deciding whether it can be
//! shown by example, and a read-only tool needs neither (it changes nothing, so there is nothing to
//! save and reopen).
//!
//! The examples share one fixture, so they read alike: a three-page PDF opened as document `1`,
//! whose pages read "Page 1", "Page 2" and "Page 3". Anything that needs a picture, a second
//! document, a signature or the OCR models is not shown this way — [`why_not`] says which, and
//! where that tool is proved instead.

use serde_json::{Value, json};

/// A call that works, for a tool that changes a document — or `None` when this tool has no example
/// (see [`why_not`]).
pub fn example(tool: &str) -> Option<Value> {
    Some(match tool {
        // Pages.
        "page_rotate" => json!({ "doc": 1, "pages": [1], "degrees": 90 }),
        "page_delete" => json!({ "doc": 1, "pages": [2] }),
        "page_move" => json!({ "doc": 1, "pages": [3], "to": 1 }),
        "page_reverse" => json!({ "doc": 1 }),
        "page_duplicate" => json!({ "doc": 1, "pages": [1], "at": 2 }),
        "page_insert_blank" => json!({ "doc": 1, "at": 2 }),
        // `margins` is [left, bottom, right, top] in points from the media box.
        "page_set_box" => json!({ "doc": 1, "pages": [1], "margins": [10, 10, 10, 10] }),
        // Page numbers are labels, not drawing: the pages keep their content.
        "page_number" => json!({ "doc": 1, "from": 1, "to": 3, "style": "lower-roman" }),
        // Document properties and bookmarks.
        "doc_set_info" => json!({ "doc": 1, "key": "Title", "value": "Round trip" }),
        "bookmark_add" => json!({ "doc": 1, "title": "Start", "page": 2 }),
        // Marks that are page content.
        "page_add_text" => json!({ "doc": 1, "page": 1, "at": [20, 20], "text": "Reviewed" }),
        "doc_watermark" => json!({ "doc": 1, "text": "DRAFT" }),
        "doc_background" => json!({ "doc": 1, "color": "#eeeeee" }),
        "doc_header_footer" => json!({ "doc": 1, "footer_center": "Page <<1>> of <<n>>" }),
        // Annotations a reader makes.
        "comment_add" => json!({ "doc": 1, "page": 1, "type": "note", "at": [40, 40], "contents": "Checked" }),
        "link_add" => json!({ "doc": 1, "page": 1, "rect": [10, 10, 100, 40], "url": "https://example.com" }),
        // The rect is clear of the text, so the pages still read as they did (see roundtrip.rs).
        "redact_mark" => json!({ "doc": 1, "page": 1, "rect": [10, 200, 190, 280] }),
        // Forms and text.
        "form_add_field" => json!({ "doc": 1, "page": 1, "type": "text", "rect": [20, 20, 180, 44], "name": "email" }),
        "text_edit" => json!({ "doc": 1, "page": 1, "line": 1, "text": "Edited line" }),
        // Document-level settings and scripts.
        "accessibility_fix" => json!({ "doc": 1, "rule": "title", "value": "Accessible" }),
        "js_set_document_script" => json!({ "doc": 1, "name": "hello", "script": "app.alert('hi');" }),
        "doc_initial_view" => json!({ "doc": 1, "layout": "two_up" }),
        // Measurement needs a scale before it is meaningful; the scale is the example.
        "measure_scale" => json!({ "doc": 1, "page": 1, "rect": [0, 0, 72, 72], "units_per_point": 0.1, "unit": "cm" }),
        // Standards, security and cleaning.
        "pdfa_convert" => json!({ "doc": 1, "level": "2b" }),
        "doc_protect" => json!({ "doc": 1, "open_password": "secret" }),
        "doc_remove_hidden" => json!({ "doc": 1 }),
        _ => return None,
    })
}

/// Why a tool that changes a document has no example, and where it is proved instead. `None` for a
/// tool that has an example, and for a read-only tool (which needs neither).
pub fn why_not(tool: &str) -> Option<&'static str> {
    const FILES: &str = "writes files rather than the open document; its own test opens what it wrote";
    const SECOND: &str = "needs a second document or a picture file, which the shared fixture does not have";
    const SIGNING: &str = "needs a certificate or a signing identity, which a test cannot have";
    const OCR: &str = "needs the OCR models, which are downloaded rather than bundled";
    const SESSION: &str = "acts on the session or the view, not on the document, so there is nothing to save";
    const SETUP: &str = "needs a field, comment or link that the shared fixture does not have; covered in its own test";
    const CREATES: &str = "creates a document instead of changing one; covered in its own test";
    const PRINTS: &str = "sends the document to a printer, a preview or a rasteriser rather than changing it";
    const NEEDS_A_MARK: &str = "needs a redaction mark to remove, which one call cannot have made first";
    Some(match tool {
        // Writes a file (the document is read, or a copy is written).
        "doc_save"
        | "page_extract"
        | "doc_combine"
        | "doc_split"
        | "doc_create_multiple"
        | "doc_export_data"
        | "doc_import_data"
        | "form_merge_data"
        | "doc_export_images"
        | "doc_export_office"
        | "doc_export_all_images"
        | "doc_export_text"
        | "doc_compare_report"
        | "accessibility_report"
        | "measure_export"
        | "image_save"
        | "action_run"
        | "ocr_recognize_files"
        | "doc_reduce"
        | "doc_optimize" => FILES,
        // A second document, a picture, or another file it reads.
        "page_insert_file" | "page_replace" | "stamp_custom" | "form_set_image" | "page_add_image" | "image_edit" => SECOND,
        "sign_id_create" | "sign_document" | "sign_trust" => SIGNING,
        "ocr_recognize" => OCR,
        // The session, the view or the undo stack.
        "doc_close"
        | "edit_undo"
        | "edit_redo"
        | "command_run"
        | "command_batch"
        | "fill_sign_date_format"
        | "fill_sign_add"
        | "js_enabled"
        | "js_run"
        | "comments_hide" => SESSION,
        // Needs something in the document the shared fixture does not build.
        "bookmark_rename"
        | "bookmark_delete"
        | "bookmark_move"
        | "bookmark_set_page"
        | "bookmark_from_structure"
        | "form_fill"
        | "form_reset"
        | "form_set_props"
        | "form_delete_field"
        | "form_tab_order"
        | "form_set_actions"
        | "form_set_script"
        | "form_detect_fields"
        | "comment_reply"
        | "comment_set_status"
        | "comment_mark"
        | "comment_lock"
        | "comments_summarize"
        | "comment_edit"
        | "comment_delete"
        | "link_edit"
        | "link_delete"
        | "links_from_urls"
        | "links_remove"
        | "content_update"
        | "content_delete"
        | "object_move"
        | "redact_apply"
        | "doc_unprotect"
        | "doc_remove_marks"
        | "doc_flatten"
        | "accessibility_set_alt"
        | "doc_compare_mark"
        | "measure_distance"
        | "measure_perimeter"
        | "measure_area" => SETUP,
        "doc_print" => PRINTS,
        "redact_clear" => NEEDS_A_MARK,
        "doc_create" | "doc_revisions" | "doc_open_revision" => CREATES,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tool_has_an_example_or_a_reason_never_both() {
        // (tool, has an example, has a reason). A read-only tool and a name that is not a tool at
        // all have neither; nothing may have both.
        for (name, wants_example, wants_reason) in [
            ("page_rotate", true, false),
            ("doc_save", false, true),
            ("ocr_recognize", false, true),
            ("doc_open", false, false),
            ("not_a_tool", false, false),
        ] {
            assert_eq!(example(name).is_some(), wants_example, "{name}: example");
            assert_eq!(why_not(name).is_some(), wants_reason, "{name}: reason");
        }
    }

    #[test]
    fn the_examples_are_objects_with_a_document() {
        // Every example is a complete call on document 1: an object, and nothing outside the tool's
        // own arguments.
        for tool in ["page_rotate", "doc_set_info", "comment_add", "form_add_field", "doc_protect"] {
            let e = example(tool);
            assert!(e.is_some(), "{tool} has no example");
            let e = e.unwrap_or(Value::Null);
            assert!(e.is_object(), "{tool}: {e}");
            assert_eq!(e["doc"], 1, "{tool}: {e}");
        }
    }
}
