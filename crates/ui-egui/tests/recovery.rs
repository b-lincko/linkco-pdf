//! Autosave and crash recovery: a session is "crashed" by dropping the app without a clean
//! quit, and a new session must offer, recover or discard what was autosaved.

use egui::accesskit::Role;
use egui::{Key, Modifiers};
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_engine::Edit;
use pdfcraft_ui_egui::{PdfCraftApp, RecoveryStore};

fn fixture(n: usize) -> Vec<u8> {
    let mut objs: Vec<String> = vec!["<< /Type /Catalog /Pages 2 0 R >>".into()];
    let kids: Vec<String> = (0..n).map(|i| format!("{} 0 R", 4 + 2 * i)).collect();
    objs.push(format!("<< /Type /Pages /Kids [{}] /Count {n} /MediaBox [0 0 200 300] >>", kids.join(" ")));
    objs.push("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".into());
    for i in 0..n {
        objs.push(format!("<< /Type /Page /Parent 2 0 R /Contents {} 0 R /Resources << /Font << /F1 3 0 R >> >> >>", 5 + 2 * i));
        let body = format!("BT /F1 24 Tf 20 150 Td (Confidential {}) Tj ET", i + 1);
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

fn store(tag: &str) -> RecoveryStore {
    let dir = std::env::temp_dir().join(format!("pdfcraft-recovery-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    RecoveryStore::new(dir)
}

fn files_in(s: &RecoveryStore) -> usize {
    std::fs::read_dir(s.dir()).map(|d| d.count()).unwrap_or(0)
}

/// A session that edits a document, autosaves, and then "crashes" (is dropped).
fn crashed_session(s: &RecoveryStore, bytes: Vec<u8>, password: Option<&str>, path: Option<&str>) {
    let mut app = PdfCraftApp::new();
    app.set_option("language", "en").unwrap();
    app.enable_recovery(s.clone());
    match password {
        Some(pw) => {
            app.open_bytes("work.pdf", path.map(str::to_string), bytes).unwrap();
            app.submit_password(Some(pw.to_string()));
        }
        None => app.open_bytes("work.pdf", path.map(str::to_string), bytes).unwrap(),
    }
    assert!(app.apply_edit(Edit::DeletePages { pages: vec![0] }));
    app.autosave_now();
    // Dropped without a clean quit: the recovery entry stays.
}

fn harness(s: RecoveryStore) -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        app.enable_recovery(s);
        app
    });
    h.run_steps(4);
    h
}

#[test]
fn a_crashed_session_is_offered_and_recovered_unsaved() {
    let s = store("recover");
    crashed_session(&s, fixture(3), None, Some("/docs/work.pdf"));
    assert_eq!(s.list().len(), 1);
    let mut h = harness(s.clone());
    h.get_by_label("Recover unsaved documents?");
    h.get_by_label("work.pdf");
    h.get_by_label("Recover").click();
    h.run_steps(4);
    let app = h.state();
    assert_eq!(app.views.len(), 1);
    let doc = app.session.get(app.views[0].id).unwrap();
    assert!(doc.dirty, "recovered changes are unsaved");
    assert_eq!(doc.path.as_deref(), Some("/docs/work.pdf"), "Save goes back to the original file");
    assert_eq!(doc.info.pages.len(), 2, "the edit survived the crash");
    assert_eq!(s.list().len(), 1, "the entry stays until the document is saved or closed");
}

#[test]
fn discarding_removes_the_autosaves() {
    let s = store("discard");
    crashed_session(&s, fixture(2), None, None);
    let mut h = harness(s.clone());
    h.get_by_label("Discard").click();
    h.run_steps(3);
    assert!(h.state().views.is_empty());
    assert_eq!(files_in(&s), 0);
}

#[test]
fn saving_or_closing_clears_the_entry_and_nothing_is_written_for_clean_documents() {
    let s = store("save");
    let mut app = PdfCraftApp::new();
    app.set_option("language", "en").unwrap();
    app.enable_recovery(s.clone());
    app.open_bytes("a.pdf", None, fixture(2)).unwrap();
    app.autosave_now();
    assert_eq!(files_in(&s), 0, "no changes, no autosave");
    app.apply_edit(Edit::RotatePages { pages: vec![0], degrees: 90 });
    app.autosave_now();
    assert_eq!(s.list().len(), 1);
    let out = std::env::temp_dir().join(format!("pdfcraft-recovery-saved-{}.pdf", std::process::id()));
    app.save_override = Some(out.to_string_lossy().into_owned());
    assert!(app.save_active(pdfcraft_ui_egui::SaveTarget::InPlace));
    assert_eq!(files_in(&s), 0, "saved: nothing to recover");
    // Edit again, autosave, then close and discard.
    app.apply_edit(Edit::RotatePages { pages: vec![0], degrees: 90 });
    app.autosave_now();
    assert_eq!(s.list().len(), 1);
    app.close_tab(0);
    assert_eq!(files_in(&s), 0, "closed: nothing to recover");
    let _ = std::fs::remove_file(out);
}

/// A real filesystem refusal must not consume a revision. Retry without another edit, then
/// recover its complete PDF in a fresh app (not a process-kill or power-loss simulation).
#[test]
fn failed_autosaves_retry_without_an_edit_and_recover_in_a_fresh_app() {
    for protected in [false, true] {
        for prior in [false, true] {
            let s = store(&format!("retry-{protected}-{prior}"));
            let original = s.dir().with_extension("original.pdf");
            let bytes = if protected { encrypted(fixture(3), "pw") } else { fixture(3) };
            std::fs::write(&original, &bytes).unwrap();
            let path = original.to_string_lossy().into_owned();
            let mut app = PdfCraftApp::new();
            app.enable_recovery(s.clone());
            app.open_bytes("work.pdf", Some(path.clone()), bytes.clone()).unwrap();
            if protected {
                app.submit_password(Some("pw".into()));
            }
            assert!(app.apply_edit(Edit::RotatePages { pages: vec![0], degrees: 90 }));
            let backup = s.dir().with_extension("backup");
            if prior {
                app.autosave_now();
                assert_eq!(s.list().len(), 1);
                std::fs::rename(s.dir(), &backup).unwrap();
                assert!(app.apply_edit(Edit::DeletePages { pages: vec![2] }));
            }
            let id = app.views[0].id;
            let expected = app.session.get(id).unwrap().bytes.clone();
            // A file in place of the store directory fails even as root and on Windows.
            std::fs::write(s.dir(), b"BLOCKED STORE").unwrap();
            app.autosave_now();
            app.autosave_now();
            assert_eq!(std::fs::read(s.dir()).unwrap(), b"BLOCKED STORE");
            assert_eq!(app.session.get(id).unwrap().bytes, expected);
            assert!(app.session.get(id).unwrap().dirty);
            assert_eq!(app.session.autosave_snapshots().len(), 1, "failed writes stay pending");
            std::fs::remove_file(s.dir()).unwrap();
            if prior {
                std::fs::rename(&backup, s.dir()).unwrap();
                assert_ne!(s.read(&s.list()[0].key).unwrap().as_slice(), expected.as_slice(), "last stored revision survived");
            }
            app.autosave_now(); // No edit between failure and retry.
            assert!(app.session.autosave_snapshots().is_empty(), "success acknowledges the revision");
            let entries = s.list();
            assert_eq!(entries.len(), 1);
            assert_eq!(s.read(&entries[0].key).unwrap().as_slice(), expected.as_slice());
            assert_eq!(std::fs::read(&original).unwrap(), bytes, "autosave never changes the original");
            drop(app); // Deliberately no clean shutdown.
            let mut reopened = PdfCraftApp::new();
            reopened.enable_recovery(s.clone());
            assert_eq!(reopened.recoverable.len(), 1);
            reopened.recover(&[entries[0].key.clone()]);
            if protected {
                assert!(reopened.password_prompt.is_some());
                reopened.submit_password(Some("pw".into()));
            }
            assert_eq!(reopened.views.len(), 1);
            let recovered = reopened.session.get(reopened.views[0].id).unwrap();
            assert!(recovered.dirty);
            assert_eq!(recovered.path.as_deref(), Some(path.as_str()));
            assert_eq!(recovered.info.pages.len(), if prior { 2 } else { 3 });
            assert_eq!(recovered.info.pages[0].rotation, 90);
            assert_eq!(recovered.bytes, expected);
            s.clear();
            std::fs::remove_dir_all(s.dir()).unwrap();
            std::fs::remove_file(original).unwrap();
        }
    }
}

/// Metadata is the second write: persisting PDF bytes alone must not acknowledge an autosave.
#[test]
fn a_failed_metadata_write_keeps_the_autosave_pending() {
    for protected in [false, true] {
        let s = store(&format!("metadata-retry-{protected}"));
        let mut app = PdfCraftApp::new();
        app.enable_recovery(s.clone());
        let bytes = if protected { encrypted(fixture(3), "pw") } else { fixture(3) };
        app.open_bytes("work.pdf", None, bytes).unwrap();
        if protected {
            app.submit_password(Some("pw".into()));
        }
        assert!(app.apply_edit(Edit::RotatePages { pages: vec![0], degrees: 90 }));
        app.autosave_now();
        let entry = s.list().remove(0);
        let metadata = s.dir().join(format!("{}.json", entry.key));
        std::fs::remove_file(&metadata).unwrap();
        std::fs::create_dir(&metadata).unwrap(); // Portable rename denial after PDF persistence.
        assert!(app.apply_edit(Edit::DeletePages { pages: vec![2] }));
        let id = app.views[0].id;
        let expected = app.session.get(id).unwrap().bytes.clone();
        app.autosave_now();
        assert!(metadata.is_dir());
        let persisted = s.read(&entry.key).unwrap();
        assert_eq!(persisted.as_slice(), expected.as_slice(), "PDF write completed before metadata failed");
        if protected {
            assert!(!String::from_utf8_lossy(&persisted).contains("Confidential"));
        }
        assert_eq!(app.session.autosave_snapshots().len(), 1, "acknowledge only the complete store write");
        std::fs::remove_dir(metadata).unwrap();
        app.autosave_now();
        assert_eq!(s.list().len(), 1, "metadata is retried without another edit");
        assert!(app.session.autosave_snapshots().is_empty());
        assert!(app.session.get(id).unwrap().dirty);
        s.clear();
        std::fs::remove_dir_all(s.dir()).unwrap();
    }
}

#[test]
fn quitting_cleanly_leaves_nothing_behind() {
    let s = store("quit");
    let mut h = Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_eframe({
        let s = s.clone();
        move |_cc| {
            let mut app = PdfCraftApp::new();
            app.set_option("language", "en").unwrap();
            app.enable_recovery(s);
            app.open_bytes("q.pdf", None, fixture(2)).unwrap();
            app
        }
    });
    h.run_steps(3);
    h.state_mut().apply_edit(Edit::RotatePages { pages: vec![0], degrees: 90 });
    h.state_mut().autosave_now();
    assert_eq!(s.list().len(), 1);
    // Quit → "Don't save" → the app closes and the autosave is removed.
    h.state_mut().close_request = Some(pdfcraft_ui_egui::CloseRequest::Quit);
    h.run_steps(2);
    h.get_by_label("Don't save").click();
    h.run_steps(4);
    assert!(h.state().views.is_empty());
    h.state_mut().shutdown_recovery();
    assert_eq!(files_in(&s), 0);
}

/// `bytes` protected with an open password (AES-256).
fn encrypted(bytes: Vec<u8>, password: &str) -> Vec<u8> {
    let mut doc = pdfcraft_cos::Document::open(std::sync::Arc::new(bytes)).unwrap();
    doc.set_encryption(&pdfcraft_cos::NewEncryption {
        algorithm: pdfcraft_cos::Algorithm::Aes256,
        user_password: password,
        owner_password: "owner",
        permissions: -1,
        encrypt_metadata: true,
        seed: [2; 32],
    })
    .unwrap();
    pdfcraft_cos::write_full(&doc, &Default::default()).unwrap()
}

#[test]
fn encrypted_documents_are_autosaved_encrypted_and_recovered_with_the_password() {
    let bytes = encrypted(fixture(2), "pw");
    let s = store("encrypted");
    crashed_session(&s, bytes, Some("pw"), None);
    let meta = s.list().remove(0);
    assert!(meta.encrypted);
    let saved = s.read(&meta.key).unwrap();
    assert!(!String::from_utf8_lossy(&saved).contains("Confidential"), "no plaintext on disk");
    let mut h = harness(s.clone());
    h.get_by_label_contains("password-protected");
    h.get_by_label("Recover").click();
    h.run_steps(3);
    let field = h.get_by_role(Role::PasswordInput);
    field.focus();
    field.type_text("pw");
    h.run_steps(2);
    h.key_press(Key::Enter);
    h.run_steps(4);
    let app = h.state();
    assert_eq!(app.views.len(), 1);
    let d = app.session.get(app.views[0].id).unwrap();
    assert!(d.dirty);
    assert_eq!(d.info.pages.len(), 1);
    let _ = Modifiers::NONE;
}

/// #813: cancelling the password prompt of a recovered snapshot must not hand its path and
/// recovery entry to the next encrypted document that is unlocked (Save would then overwrite
/// the snapshot's original file and delete the snapshot).
#[test]
fn a_cancelled_recovery_password_does_not_attach_to_the_next_unlocked_document() {
    let s = store("cancelled");
    crashed_session(&s, encrypted(fixture(2), "pw-a"), Some("pw-a"), Some("/docs/A.pdf"));
    assert_eq!(s.list().len(), 1);
    let mut h = harness(s.clone());
    h.get_by_label("Recover").click();
    h.run_steps(3);
    assert!(h.state().password_prompt.is_some(), "the snapshot asks for its password");
    h.get_by_label("Cancel").click();
    h.run_steps(3);
    assert!(h.state().password_prompt.is_none());
    assert!(h.state().views.is_empty());

    let app = h.state_mut();
    app.open_bytes("B.pdf", Some("/docs/B.pdf".into()), encrypted(fixture(1), "pw-b")).unwrap();
    app.submit_password(Some("pw-b".into()));
    h.run_steps(3);
    let app = h.state();
    assert_eq!(app.views.len(), 1);
    let b = app.session.get(app.views[0].id).unwrap();
    assert_eq!(b.path.as_deref(), Some("/docs/B.pdf"), "B keeps its own path");
    assert!(!b.dirty, "B is clean: it is not the recovered document");
    assert_eq!(s.list().len(), 1, "A's snapshot is still in the store");
    assert_eq!(app.recoverable.len(), 1, "and on offer again");

    // Closing B (clean, so no prompt) must not remove A's snapshot either.
    h.state_mut().execute("file.close");
    h.run_steps(3);
    assert!(h.state().views.is_empty());
    assert_eq!(s.list().len(), 1, "A's snapshot survives closing B");
}

#[test]
fn incomplete_entries_are_ignored_and_cleaned_up() {
    let s = store("partial");
    std::fs::create_dir_all(s.dir()).unwrap();
    std::fs::write(s.dir().join("orphan.pdf"), b"%PDF-1.7").unwrap();
    std::fs::write(s.dir().join("broken.json"), b"{ not json").unwrap();
    assert!(s.list().is_empty());
    assert_eq!(files_in(&s), 0);
}

/// Through the control channel, the reply to a click comes after the click's effects: state read
/// right after it already reflects them (found driving the live app).
#[test]
fn control_click_effects_are_visible_when_the_reply_arrives() {
    let s = store("control");
    crashed_session(&s, fixture(2), None, None);
    let slot: std::sync::Arc<std::sync::Mutex<Option<pdfcraft_ui_egui::control::ControlClient>>> = Default::default();
    let (slot2, s2) = (slot.clone(), s.clone());
    let mut h = Harness::builder().with_size(egui::vec2(1200.0, 800.0)).build_eframe(move |cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        *slot2.lock().unwrap() = Some(app.attach_control(&cc.egui_ctx));
        app.enable_recovery(s2);
        app
    });
    h.run_steps(4);
    let c = slot.lock().unwrap().take().unwrap();
    let call = |h: &mut Harness<'static, PdfCraftApp>, m: &str, p: serde_json::Value| {
        let rx = c.send(m, p);
        for _ in 0..30 {
            h.step();
            if let Ok(r) = rx.try_recv() {
                return r.unwrap();
            }
        }
        panic!("no reply to {m}");
    };
    assert_eq!(call(&mut h, "ui.state", serde_json::json!({}))["dialog"], "Recovery");
    call(&mut h, "ui.click", serde_json::json!({ "label": "Discard" }));
    assert_eq!(files_in(&s), 0, "discarded by the time the click is answered");
    assert_eq!(call(&mut h, "ui.state", serde_json::json!({}))["dialog"], serde_json::Value::Null);
}

/// Store-discovered JSON cannot redirect recovery or cleanup to a different filename/key.
#[test]
fn forged_recovery_keys_cannot_redirect_recovery_or_discard_outside_the_store() {
    for protected in [false, true] {
        for discard in [false, true] {
            let s = store(&format!("key-binding-{protected}-{discard}"));
            let bytes = if protected { encrypted(fixture(3), "pw") } else { fixture(3) };
            crashed_session(&s, bytes, protected.then_some("pw"), Some("/docs/valid.pdf"));
            let valid = s.list().remove(0);
            let outside_pdf = s.dir().with_extension("outside.pdf");
            let outside_json = outside_pdf.with_extension("json");
            std::fs::write(&outside_pdf, b"OUTSIDE PDF").unwrap();
            std::fs::write(&outside_json, b"OUTSIDE METADATA").unwrap();
            let mut escape = valid.clone();
            escape.key = format!("../{}", outside_pdf.file_stem().unwrap().to_string_lossy());
            escape.path = Some(outside_pdf.to_string_lossy().into_owned());
            std::fs::write(s.dir().join("escape.json"), serde_json::to_vec(&escape).unwrap()).unwrap();
            std::fs::write(s.dir().join("alias.json"), serde_json::to_vec(&valid).unwrap()).unwrap();
            let mut app = PdfCraftApp::new();
            app.enable_recovery(s.clone());
            assert_eq!(app.recoverable.as_slice(), std::slice::from_ref(&valid), "only the filename-bound legitimate entry is offered");
            assert!(!s.dir().join("escape.json").exists());
            assert!(!s.dir().join("alias.json").exists());
            if discard {
                // Include an invalid direct request as well as the offered entry.
                app.discard_recovered(&[escape.key.clone(), valid.key.clone()]);
                assert!(s.list().is_empty());
            } else {
                app.recover(std::slice::from_ref(&valid.key));
                if protected {
                    app.submit_password(Some("pw".into()));
                }
                assert_eq!(app.views.len(), 1);
                let doc = app.session.get(app.views[0].id).unwrap();
                assert!(doc.dirty);
                assert_eq!(doc.info.pages.len(), 2, "the legitimate edited PDF still recovers");
                assert_eq!(doc.path.as_deref(), Some("/docs/valid.pdf"));
            }
            assert_eq!(std::fs::read(&outside_pdf).unwrap(), b"OUTSIDE PDF");
            assert_eq!(std::fs::read(&outside_json).unwrap(), b"OUTSIDE METADATA");
            s.clear();
            std::fs::remove_dir_all(s.dir()).unwrap();
            std::fs::remove_file(outside_pdf).unwrap();
            std::fs::remove_file(outside_json).unwrap();
        }
    }
}

#[test]
fn failed_recovery_stays_visible_and_can_be_retried_after_read_or_parse_repair() {
    for read_failure in [false, true] {
        let s = store(&format!("recover-retry-{read_failure}"));
        crashed_session(&s, fixture(3), None, Some("/docs/retry.pdf"));
        let meta = s.list().remove(0);
        let saved = s.read(&meta.key).unwrap();
        let pdf = s.dir().join(format!("{}.pdf", meta.key));
        let mut h = harness(s.clone());
        // Damage the bytes after startup, as an interrupted replacement or media error would.
        if read_failure {
            std::fs::remove_file(&pdf).unwrap();
            std::fs::create_dir(&pdf).unwrap();
        } else {
            std::fs::write(&pdf, b"not a PDF").unwrap();
        }
        h.get_by_label("Recover").click();
        h.run_steps(4);
        assert!(h.state().views.is_empty());
        assert_eq!(h.state().recoverable, std::slice::from_ref(&meta));
        h.get_by_label("Recover unsaved documents?");
        assert!(s.dir().join(format!("{}.json", meta.key)).is_file());
        if read_failure {
            std::fs::remove_dir(&pdf).unwrap();
        }
        std::fs::write(&pdf, saved).unwrap();
        h.get_by_label("Recover").click();
        h.run_steps(4);
        assert!(h.state().recoverable.is_empty());
        let doc = h.state().session.get(h.state().views[0].id).unwrap();
        assert!(doc.dirty);
        assert_eq!(doc.path.as_deref(), Some("/docs/retry.pdf"));
        assert_eq!(doc.info.pages.len(), 2);
        s.clear();
        std::fs::remove_dir_all(s.dir()).unwrap();
    }
}

#[test]
fn repeated_recovery_keys_do_not_open_duplicate_tabs() {
    let s = store("duplicate-recover");
    crashed_session(&s, fixture(2), None, None);
    let key = s.list().remove(0).key;
    let mut app = PdfCraftApp::new();
    app.enable_recovery(s.clone());
    app.recover(&[key.clone(), key]);
    assert_eq!(app.views.len(), 1);
    assert!(app.recoverable.is_empty());
    s.clear();
    std::fs::remove_dir_all(s.dir()).unwrap();
}

#[test]
fn multiple_encrypted_recoveries_keep_their_own_prompts_and_reoffer_deferred_entries() {
    let s = store("serial-passwords");
    let mut initial = PdfCraftApp::new();
    initial.enable_recovery(s.clone());
    for (name, path, password) in [("A.pdf", "/docs/A.pdf", "pw-a"), ("B.pdf", "/docs/B.pdf", "pw-b")] {
        initial.open_bytes(name, Some(path.into()), encrypted(fixture(3), password)).unwrap();
        initial.submit_password(Some(password.into()));
        assert!(initial.apply_edit(Edit::DeletePages { pages: vec![0] }));
    }
    initial.autosave_now();
    drop(initial);
    let mut h = harness(s.clone());
    let entries = h.state().recoverable.clone();
    assert_eq!(entries.len(), 2);
    let password = |name: &str| if name == "A.pdf" { "pw-a" } else { "pw-b" };
    h.get_by_label("Recover").click();
    h.run_steps(3);
    assert!(h.state().password_prompt.is_some());
    assert_eq!(h.state().recoverable, &entries[1..]);
    h.state_mut().submit_password(Some("wrong".into()));
    h.run_steps(3);
    assert!(h.state().views.is_empty());
    assert_eq!(h.state().recoverable, &entries[1..]);
    h.state_mut().submit_password(Some(password(&entries[0].name).into()));
    h.run_steps(3);
    assert_eq!(h.state().views.len(), 1);
    h.get_by_label("Recover unsaved documents?");
    h.get_by_label("Recover").click();
    h.run_steps(3);
    h.state_mut().submit_password(None);
    h.run_steps(3);
    assert_eq!(h.state().recoverable, &entries[1..], "cancel returns only that pending snapshot");
    h.get_by_label("Recover").click();
    h.run_steps(3);
    h.state_mut().submit_password(Some(password(&entries[1].name).into()));
    h.run_steps(3);
    assert_eq!(h.state().views.len(), 2);
    for view in &h.state().views {
        let doc = h.state().session.get(view.id).unwrap();
        assert!(doc.dirty);
        assert_eq!(doc.info.pages.len(), 2);
        assert_eq!(doc.path.as_deref(), Some(if doc.name == "A.pdf" { "/docs/A.pdf" } else { "/docs/B.pdf" }));
    }
    assert!(h.state().recoverable.is_empty());
    assert_eq!(s.list().len(), 2, "the entries persist until the user saves or discards");
    s.clear();
    std::fs::remove_dir_all(s.dir()).unwrap();
}

#[test]
fn recovery_does_not_dismiss_an_unrelated_password_prompt() {
    let s = store("unrelated-prompt");
    crashed_session(&s, fixture(2), None, None);
    let keys: Vec<_> = s.list().into_iter().map(|m| m.key).collect();
    let mut app = PdfCraftApp::new();
    app.enable_recovery(s.clone());
    app.open_bytes("unrelated.pdf", None, encrypted(fixture(2), "pw")).unwrap();
    app.recover(&keys);
    assert!(app.password_prompt.is_some());
    assert!(app.views.is_empty());
    assert_eq!(app.recoverable.len(), 1);
    app.submit_password(Some("pw".into()));
    assert_eq!(app.session.get(app.views[0].id).unwrap().name, "unrelated.pdf");
    assert_eq!(app.recoverable.len(), 1);
    s.clear();
    std::fs::remove_dir_all(s.dir()).unwrap();
}

/// The store owns a folder of unsaved work, so the folder is for this user alone and every
/// snapshot in it is readable only by them (Unix modes; other platforms inherit a per-user
/// profile). An older folder that an earlier version created looser is tightened when the app
/// opens the store, and the snapshots themselves are written private from their first byte.
#[cfg(unix)]
#[test]
fn the_recovery_folder_and_its_snapshots_are_private_to_the_user() {
    use std::os::unix::fs::PermissionsExt as _;
    fn mode(path: &std::path::Path) -> u32 {
        std::fs::symlink_metadata(path).unwrap().permissions().mode() & 0o777
    }
    let s = store("private");
    crashed_session(&s, fixture(2), None, None);
    assert_eq!(mode(s.dir()), 0o700, "the folder is created private");
    let entries: Vec<_> = std::fs::read_dir(s.dir()).unwrap().map(|e| e.unwrap().path()).collect();
    assert_eq!(entries.len(), 2, "one snapshot and one metadata file");
    for entry in &entries {
        assert_eq!(mode(entry), 0o600, "{entry:?} is readable only by its owner");
    }
    // A folder left looser by an earlier version is tightened when the store is opened, without
    // losing what it holds.
    std::fs::set_permissions(s.dir(), std::fs::Permissions::from_mode(0o755)).unwrap();
    let found = s.list();
    assert_eq!(found.len(), 1);
    assert_eq!(mode(s.dir()), 0o700);
    assert_eq!(s.list().len(), 1);
    s.clear();
    std::fs::remove_dir_all(s.dir()).unwrap();
}

/// A recovery folder that a link replaced is refused rather than followed: recovery must not
/// write snapshots into a folder the app does not own.
#[cfg(unix)]
#[test]
fn a_linked_recovery_folder_is_refused_instead_of_written_through() {
    let root = store("linked-root");
    let elsewhere = std::env::temp_dir().join(format!("pdfcraft-recovery-elsewhere-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&elsewhere);
    std::fs::create_dir_all(&elsewhere).unwrap();
    std::fs::create_dir_all(root.dir()).unwrap();
    let link = root.dir().join("link");
    std::os::unix::fs::symlink(&elsewhere, &link).unwrap();
    let store_through_link = RecoveryStore::new(&link);
    let meta = pdfcraft_ui_egui::RecoveryMeta { key: "link-test".into(), name: "draft.pdf".into(), path: None, saved_at: 1, encrypted: false };
    let error = store_through_link.write(&meta, &fixture(2)).unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput, "{error}");
    assert_eq!(std::fs::read_dir(&elsewhere).unwrap().count(), 0, "nothing was written through the link");
    std::fs::remove_dir_all(&elsewhere).unwrap();
    std::fs::remove_dir_all(root.dir()).unwrap();
}

/// The recovery folder cannot grow without limit: past its budget the least recently saved
/// snapshots are deleted with their metadata, the snapshot just written always stays, and the
/// folder is never emptied.
#[test]
fn the_recovery_folder_stays_within_its_budget() {
    let dir = std::env::temp_dir().join(format!("pdfcraft-recovery-budget-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    // Two entries of 1,000 bytes and at most 2,500 bytes in total: the third save must push the
    // oldest out, and the fourth (larger than the whole budget) must push the rest out.
    let s = RecoveryStore::new(&dir).with_limits(pdfcraft_ui_egui::RecoveryLimits { max_entries: 2, max_bytes: 2_500 });
    let meta = |key: &str, saved_at: u64| pdfcraft_ui_egui::RecoveryMeta {
        key: key.into(),
        name: format!("{key}.pdf"),
        path: None,
        saved_at,
        encrypted: false,
    };
    let snapshot = vec![0x41u8; 1_000];
    s.write(&meta("one", 1), &snapshot).unwrap();
    s.write(&meta("two", 2), &snapshot).unwrap();
    let keys: Vec<_> = s.list().into_iter().map(|m| m.key).collect();
    assert_eq!(keys.len(), 2, "both fit: {keys:?}");

    s.write(&meta("three", 3), &snapshot).unwrap();
    let keys: Vec<_> = s.list().into_iter().map(|m| m.key).collect();
    assert_eq!(keys.len(), 2, "the budget is kept: {keys:?}");
    assert!(keys.contains(&"three".to_string()), "the snapshot just written stays: {keys:?}");
    assert!(!keys.contains(&"one".to_string()), "the oldest went first: {keys:?}");
    // The evicted entry left no half behind.
    assert!(!dir.join("one.pdf").exists());
    assert!(!dir.join("one.json").exists());

    // A single snapshot larger than the whole budget is still kept: it may be the only copy.
    let huge = vec![0x42u8; 4_000];
    s.write(&meta("huge", 4), &huge).unwrap();
    let keys: Vec<_> = s.list().into_iter().map(|m| m.key).collect();
    assert_eq!(keys, vec!["huge".to_string()], "the newest stays even over budget: {keys:?}");
    std::fs::remove_dir_all(&dir).unwrap();
}
