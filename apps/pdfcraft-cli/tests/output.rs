//! Every file the command line writes is written whole, or not at all (B032, REC-R05).
//!
//! `edit --out` and the commands beside it used to write with plain `std::fs::write`, which
//! truncates the file it is replacing before the new bytes are on disk. A script running
//! `pdfcraft-cli` unattended had no protection against a full volume, a denied permission or a
//! killed process turning a document into an empty file — the very failure the app's own save
//! path was hardened against. These tests hold the command line to the same promise:
//!
//! * a written file is the new bytes, whole, and nothing else is left in the folder;
//! * a write that cannot happen leaves the file that was there untouched.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BIN: &str = env!("CARGO_BIN_EXE_pdfcraft-cli");

/// A scratch folder of this test's own, removed when it goes away.
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Self {
        let path = std::env::temp_dir().join(format!("pdfcraft-cli-output-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn file(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        // A read-only folder (see the failure test) has to be writable again to be removed.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            let _ = std::fs::set_permissions(&self.0, std::fs::Permissions::from_mode(0o755));
        }
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// A one-page 200×300 pt PDF.
fn fixture() -> Vec<u8> {
    let objs: Vec<Vec<u8>> = vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 /MediaBox [0 0 200 300] >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R >>".to_vec(),
    ];
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

fn edit(input: &Path, output: &Path) -> Output {
    Command::new(BIN).args(["edit", input.to_str().unwrap(), "--out", output.to_str().unwrap(), "--title", "Written"]).output().unwrap()
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

/// The names in the folder, sorted: what a write is allowed to leave behind.
fn names(dir: &Path) -> Vec<String> {
    let mut out: Vec<String> = std::fs::read_dir(dir).unwrap().map(|e| e.unwrap().file_name().to_string_lossy().into_owned()).collect();
    out.sort();
    out
}

#[test]
fn an_overwritten_output_is_the_new_file_and_nothing_else_is_left() {
    let scratch = Scratch::new("replaces");
    let input = scratch.file("in.pdf");
    std::fs::write(&input, fixture()).unwrap();
    let output = scratch.file("out.pdf");
    // A file already at the output name, older and longer than what is about to be written: this is
    // the file `std::fs::write` used to truncate before it knew whether the write would succeed.
    let old = vec![b'o'; 4096];
    std::fs::write(&output, &old).unwrap();

    let out = edit(&input, &output);
    assert!(out.status.success(), "{}", stderr(&out));
    let written = std::fs::read(&output).unwrap();
    assert_ne!(written, old, "the old contents were replaced");
    assert!(written.starts_with(b"%PDF-"), "the output is a PDF");
    assert!(written.ends_with(b"%%EOF\n"), "the whole file was written");
    assert_eq!(names(&scratch.0), ["in.pdf", "out.pdf"], "no staging file is left behind");
}

#[test]
fn a_write_that_cannot_happen_leaves_the_file_that_was_there() {
    let scratch = Scratch::new("denied");
    let input = scratch.file("in.pdf");
    std::fs::write(&input, fixture()).unwrap();
    let output = scratch.file("out.pdf");
    let old = b"the file that was already there".to_vec();
    std::fs::write(&output, &old).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        std::fs::set_permissions(&scratch.0, std::fs::Permissions::from_mode(0o555)).unwrap();
        // Root ignores the mode, so there is nothing to demonstrate there.
        if std::fs::write(scratch.file("probe"), b"x").is_ok() {
            let _ = std::fs::remove_file(scratch.file("probe"));
            let _ = std::fs::set_permissions(&scratch.0, std::fs::Permissions::from_mode(0o755));
            return;
        }
        let out = edit(&input, &output);
        std::fs::set_permissions(&scratch.0, std::fs::Permissions::from_mode(0o755)).unwrap();
        assert!(!out.status.success(), "{}", stderr(&out));
        assert!(stderr(&out).contains("out.pdf"), "the file is named: {}", stderr(&out));
        assert_eq!(std::fs::read(&output).unwrap(), old, "the file that was there is untouched");
        // The folder holds the input and that untouched file: no partial write, no staging file.
        assert_eq!(names(&scratch.0), ["in.pdf", "out.pdf"]);
    }
    #[cfg(not(unix))]
    {
        // Windows cannot make a folder unwritable in the way this test needs without an ACL dance;
        // the same guarantee is covered there by the staging writer's own tests and by the
        // replacement test above.
        let _ = (input, output, old);
    }
}
