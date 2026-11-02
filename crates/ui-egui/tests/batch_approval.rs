//! A batch of files is shown before it runs (B035, REC-B03).
//!
//! Action Wizard and Recognize Text in Files used to begin the moment a folder was chosen: the
//! first time a reader learned which files would be written — and which of them would replace
//! something already there — was after it had happened. Now the batch is planned, the plan is
//! shown, and nothing runs until the question is answered. And the answer is checked: the plan is
//! worked out again at the moment it is accepted, so a folder that changed under the question
//! produces a different plan, and a different plan is not the one that was approved.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_engine::Session;
use pdfcraft_ui_egui::PdfCraftApp;

/// A one-page PDF.
fn one_page() -> Vec<u8> {
    Session::new().create_from_text("t", "Prepare this").unwrap().to_vec()
}

/// A fresh root with `in/a.pdf` and an empty `out`.
fn tree(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("pdfcraft-approve-ui-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("in")).unwrap();
    std::fs::create_dir_all(dir.join("out")).unwrap();
    std::fs::write(dir.join("in/a.pdf"), one_page()).unwrap();
    dir
}

fn written(dir: &std::path::Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir.join("out")).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
    names.sort();
    names
}

/// The app, with the Action Wizard's files and results folder pinned.
fn harness(dir: &std::path::Path) -> Harness<'static, PdfCraftApp> {
    let d = dir.to_path_buf();
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.run_inline = true;
        app.export_dir_override = Some(d.join("out").to_string_lossy().into_owned());
        app.action_files_override = Some(vec![d.join("in/a.pdf").to_string_lossy().into_owned()]);
        app
    });
    h.run_steps(2);
    h
}

#[test]
fn the_batch_is_shown_and_writes_nothing_until_it_is_approved() {
    let dir = tree("shown");
    let mut h = harness(&dir);
    assert!(h.state_mut().execute("actions.distribution"));
    h.run_steps(2);
    h.get_by_label("Start").click();
    h.run_steps(2);

    // The question: what it would run, and how many files that writes.
    h.get_by_label("Run Prepare for Distribution on 1 files?");
    h.get_by_label("Writes 1 file(s).");
    assert!(written(&dir).is_empty(), "planning wrote nothing");
    assert!(h.state().action_run.is_none(), "and nothing is running yet");

    h.get_by_label("Run").click();
    h.run_steps(3);
    assert_eq!(h.state().toast.clone().unwrap().0, "Prepare for Distribution: 1 file done");
    assert_eq!(written(&dir), vec!["a.pdf".to_string()], "the file written is the file that was shown");
    assert!(h.state().pending_batch.is_none(), "the question is answered");
}

#[test]
fn a_question_that_is_cancelled_writes_nothing_and_leaves_the_slot_free() {
    let dir = tree("cancelled");
    let mut h = harness(&dir);
    h.state_mut().execute("actions.distribution");
    h.run_steps(2);
    h.get_by_label("Start").click();
    h.run_steps(2);
    h.get_by_label("Cancel").click();
    h.run_steps(2);

    assert!(h.state().pending_batch.is_none(), "the question is gone");
    assert!(h.state().dialog.is_none(), "and so is the dialog");
    assert!(h.state().action_run.is_none(), "nothing started");
    assert!(written(&dir).is_empty(), "nothing was written");

    // The slot a cancelled batch would otherwise hold is free: the wizard runs again.
    h.state_mut().execute("actions.distribution");
    h.run_steps(2);
    h.get_by_label("Start").click();
    h.run_steps(2);
    h.get_by_label("Run").click();
    h.run_steps(3);
    assert_eq!(written(&dir), vec!["a.pdf".to_string()]);
}

#[test]
fn a_folder_that_changed_under_the_question_is_not_the_batch_that_runs() {
    let dir = tree("stale");
    let mut h = harness(&dir);
    h.state_mut().execute("actions.distribution");
    h.run_steps(2);
    h.get_by_label("Start").click();
    h.run_steps(2);
    // The plan names `out/a.pdf`: it was free when the question was drawn.
    let shown = h.state().pending_batch.clone().unwrap().plan;
    assert_eq!(shown.targets[0].output, dir.join("out/a.pdf"), "the plan names the free name");

    // Someone else writes that file while the question is on screen.
    std::fs::write(dir.join("out/a.pdf"), b"not this document").unwrap();
    h.get_by_label("Run").click();
    h.run_steps(2);

    // The answer is refused, and the question stays up with the batch as it now is: `a (2).pdf`.
    assert_eq!(
        h.state().toast.as_ref().map(|t| t.0.as_str()),
        Some("The folder changed since this was shown; check what this batch would write now")
    );
    let now = h.state().pending_batch.clone().expect("the question is still up");
    assert_eq!(now.plan.targets[0].output, dir.join("out/a (2).pdf"), "the batch as it is now");
    assert!(h.state().action_run.is_none(), "a refused approval starts nothing");
    assert_eq!(std::fs::read(dir.join("out/a.pdf")).unwrap(), b"not this document", "the file that appeared is untouched");
    assert_eq!(written(&dir), vec!["a.pdf".to_string()]);

    // Answering the question that is actually on screen now runs it.
    h.get_by_label("Run").click();
    h.run_steps(3);
    assert_eq!(written(&dir), vec!["a (2).pdf".to_string(), "a.pdf".to_string()]);
    assert_eq!(std::fs::read(dir.join("out/a.pdf")).unwrap(), b"not this document");
}

#[test]
fn a_recognition_batch_names_the_file_it_would_replace_before_it_does() {
    // The plan needs no OCR models — only the refusal to write without an answer does — so this
    // holds on a machine (and a CI runner) without them.
    let dir = tree("replaces");
    std::fs::write(dir.join("out/a.pdf"), b"an older copy").unwrap();
    let mut app = PdfCraftApp::new();
    app.set_option("language", "en").unwrap();
    app.run_inline = true;
    app.export_dir_override = Some(dir.join("out").to_string_lossy().into_owned());
    app.use_files(pdfcraft_ui_egui::FilePurpose::Ocr, vec![("a.pdf".into(), one_page())]);
    let pending = app.pending_batch.clone().expect("the batch is planned before it runs");
    assert_eq!(pending.plan.replacing(), 1, "the copy already there is named as replaced");
    assert_eq!(pending.plan.targets[0].output, dir.join("a.pdf"));
    assert_eq!(std::fs::read(dir.join("out/a.pdf")).unwrap(), b"an older copy", "planning replaced nothing");
}
