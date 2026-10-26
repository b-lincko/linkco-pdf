//! Saving must not silently destroy a file another program replaced while it was open.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_engine::Edit;
use pdfcraft_ui_egui::{PdfCraftApp, SaveTarget};

/// A two-page PDF, so an edit can delete one page without emptying the document.
fn fixture(n: usize) -> Vec<u8> {
    let mut objs: Vec<String> = vec!["<< /Type /Catalog /Pages 2 0 R >>".into()];
    let kids: Vec<String> = (0..n).map(|i| format!("{} 0 R", 4 + 2 * i)).collect();
    objs.push(format!("<< /Type /Pages /Kids [{}] /Count {n} /MediaBox [0 0 200 300] >>", kids.join(" ")));
    objs.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".into());
    for i in 0..n {
        objs.push(format!("<< /Type /Page /Parent 2 0 R /Contents {} 0 R /Resources << /Font << /F1 3 0 R >> >> >>", 5 + 2 * i));
        let body = format!("BT /F1 24 Tf 20 150 Td (Page {}) Tj ET", i + 1);
        objs.push(format!("<< /Length {} >>\nstream\n{body}\nendstream", body.len()));
    }
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (i, o) in objs.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n{o}\nendobj\n", i + 1).as_bytes());
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objs.len() + 1).as_bytes());
    for o in offsets {
        out.extend_from_slice(format!("{o:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objs.len() + 1).as_bytes());
    out
}

fn store(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("pdfcraft-conflict-app-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn harness() -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_eframe(|_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app
    });
    h.run_steps(4);
    h
}

/// Open `path` in the harness and make one edit, so saving has something to write.
fn open_and_edit(h: &mut Harness<'static, PdfCraftApp>, path: &std::path::Path) {
    h.state_mut().open_path(&path.to_string_lossy());
    h.run_steps(4);
    assert_eq!(h.state().views.len(), 1, "the file opened");
    assert!(h.state_mut().apply_edit(Edit::DeletePages { pages: vec![0] }), "the edit applied");
    h.run_steps(2);
}

#[test]
fn saving_an_unchanged_file_writes_in_place_without_a_prompt() {
    let dir = store("clean");
    let path = dir.join("work.pdf");
    std::fs::write(&path, fixture(2)).unwrap();
    let mut h = harness();
    open_and_edit(&mut h, &path);
    assert!(h.state_mut().save_active(SaveTarget::InPlace));
    h.run_steps(2);
    assert!(h.state().save_conflict.is_none());
    assert_ne!(std::fs::read(&path).unwrap(), fixture(2), "the edit was written");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn an_external_change_blocks_the_save_until_the_user_overwrites() {
    let dir = store("conflict");
    let path = dir.join("work.pdf");
    std::fs::write(&path, fixture(2)).unwrap();
    let mut h = harness();
    open_and_edit(&mut h, &path);
    // Another program replaces the file while the document is open.
    let mut other = fixture(2);
    other.extend_from_slice(b"\n% written by another program\n");
    std::fs::write(&path, &other).unwrap();

    assert!(!h.state_mut().save_active(SaveTarget::InPlace), "the save is held back");
    h.run_steps(2);
    assert!(h.state().save_conflict.is_some());
    assert_eq!(std::fs::read(&path).unwrap(), other, "the other version is still on disk");
    h.get_by_label_contains("changed on disk");

    h.get_by_label("Overwrite").click();
    h.run_steps(3);
    assert!(h.state().save_conflict.is_none());
    let written = std::fs::read(&path).unwrap();
    assert_ne!(written, other, "the document replaced the other version");
    assert!(written.starts_with(b"%PDF-"), "a PDF was written");
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn cancelling_keeps_the_other_version_and_the_next_save_asks_again() {
    let dir = store("cancel");
    let path = dir.join("work.pdf");
    std::fs::write(&path, fixture(2)).unwrap();
    let mut h = harness();
    open_and_edit(&mut h, &path);
    let mut other = fixture(2);
    other.extend_from_slice(b"\n% another editor\n");
    std::fs::write(&path, &other).unwrap();

    assert!(!h.state_mut().save_active(SaveTarget::InPlace));
    h.run_steps(2);
    assert!(h.state().save_conflict.is_some());
    h.get_by_label("Cancel").click();
    h.run_steps(2);
    assert!(h.state().save_conflict.is_none(), "the prompt closed");
    assert_eq!(std::fs::read(&path).unwrap(), other, "nothing was written");

    // The remembered stamp still describes the loaded version, so the next save asks again.
    assert!(!h.state_mut().save_active(SaveTarget::InPlace));
    h.run_steps(2);
    assert!(h.state().save_conflict.is_some(), "the conflict is raised again");
    assert_eq!(std::fs::read(&path).unwrap(), other);
    std::fs::remove_dir_all(dir).unwrap();
}
