//! Staging files for atomic saves: a new, unpredictably named file beside the target, written and
//! then renamed over it. Creation and flushing are shared; writing, closing, renaming and cleaning
//! up stay with the caller.

use std::path::{Path, PathBuf};

/// Flush the staging file to the disk before it replaces the original. On macOS `sync_all` asks
/// for a full flush (`F_FULLFSYNC`), which SMB and FUSE volumes, such as a mounted Cryptomator
/// vault, refuse as unsupported: there a plain `fsync` is the strongest flush the volume offers.
/// Any other failure still fails the save.
pub fn sync_staged(f: &std::fs::File, sync_all: impl FnOnce(&std::fs::File) -> std::io::Result<()>) -> std::io::Result<()> {
    #[cfg(target_os = "macos")]
    {
        use rustix::io::Errno;
        match sync_all(f) {
            Err(e) if matches!(Errno::from_io_error(&e), Some(Errno::NOTSUP | Errno::NOTTY)) => Ok(rustix::fs::fsync(f)?),
            other => other,
        }
    }
    #[cfg(not(target_os = "macos"))]
    sync_all(f)
}

/// How many staging names [`create_staging`] tries. A random 64-bit name is only taken if someone
/// put a file there on purpose, so running out means refusing, not trying harder.
pub const STAGING_ATTEMPTS: usize = 16;

/// Where the suffix goes in a staging name. Each caller keeps the form it has always used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StagingName {
    /// `.{name}.{suffix}.pdfcraft-tmp`, used by the automation tools.
    SuffixThenTag,
    /// `.{name}.pdfcraft-{suffix}.tmp`, used by the desktop app's save.
    TagThenSuffix,
}

/// Create a new, empty staging file in `dir` for the file `name`, one name per suffix. It is
/// opened with `create_new`, which fails if anything already has the name (a file, a hard link,
/// a symbolic link even when dangling, a folder), on Windows as everywhere else; such a name is
/// skipped, never opened, so a file planted at the staging path can't receive or redirect the
/// write. Pass [`staging_suffixes`]: fixed suffixes are for tests, and a predictable name can be
/// taken in advance to refuse every save. `name` must be a single native filename, not a path
/// or (on Windows) an alternate data stream. Invalid names return `InvalidInput` before any
/// suffix is consumed or filesystem access is attempted, including before truncating long names.
pub fn create_staging(
    dir: &Path,
    name: &str,
    form: StagingName,
    suffixes: impl IntoIterator<Item = u64>,
) -> std::io::Result<(PathBuf, std::fs::File)> {
    // Check the whole input before truncating it or trying a suffix. A caller must pass a
    // filename, never a path: the leading dot in the format does not neutralize `../` segments
    // if a matching directory exists. Use native path rules so legal Unix backslashes remain
    // usable, but reject alternate data streams on Windows as well as paths.
    let invalid = name.contains('\0') || Path::new(name).file_name() != Some(std::ffi::OsStr::new(name));
    #[cfg(windows)]
    let invalid = invalid || name.contains(':');
    if invalid {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "a temporary file name must be a filename, not a path"));
    }
    // At most 128 bytes of the target's name, cut between characters, so the staging name fits
    // the 255-byte (Linux, macOS) and 255-unit (Windows) limits however long that name is.
    let mut stem = String::new();
    for c in name.chars() {
        if stem.len() + c.len_utf8() > 128 {
            break;
        }
        stem.push(c);
    }
    for suffix in suffixes.into_iter().take(STAGING_ATTEMPTS) {
        let tmp = dir.join(match form {
            StagingName::SuffixThenTag => format!(".{stem}.{suffix:016x}.pdfcraft-tmp"),
            StagingName::TagThenSuffix => format!(".{stem}.pdfcraft-{suffix:016x}.tmp"),
        });
        match std::fs::OpenOptions::new().write(true).create_new(true).open(&tmp) {
            Ok(file) => return Ok((tmp, file)),
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
            // Windows reports a folder at the name as "access denied"; it is taken all the same.
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied && tmp.symlink_metadata().is_ok() => {}
            Err(e) => return Err(e),
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "every temporary file name tried is taken"))
}

/// Unpredictable staging-name suffixes, so a name can't be planted in advance. Each is the
/// standard library's keyed hash of a counter (`RandomState::hash_one`, currently SipHash-1-3),
/// keyed from the operating system's random source. That is all a staging name needs, but it is
/// not a CSPRNG: don't use these where a value must stay secret or unguessable after it has been
/// seen, such as a token or a key.
pub fn staging_suffixes() -> impl Iterator<Item = u64> {
    use std::hash::BuildHasher;
    let state = std::hash::RandomState::new();
    (0u64..).map(move |i| state.hash_one(i))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A fresh, empty folder for one test.
    fn test_dir(test: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("pdfcraft-platform-staging-{}-{test}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn file_name(p: &Path) -> String {
        p.file_name().unwrap().to_string_lossy().into_owned()
    }

    #[test]
    fn a_staged_file_is_flushed_with_the_supplied_operation() {
        use std::io::Write;
        let dir = test_dir("flush");
        let (path, mut file) = create_staging(&dir, "out.pdf", StagingName::SuffixThenTag, [1]).unwrap();
        file.write_all(b"STAGED").unwrap();
        let called = std::cell::Cell::new(false);
        sync_staged(&file, |file| {
            assert!(!called.replace(true), "flush exactly once");
            assert_eq!(file.metadata().unwrap().len(), 6);
            file.sync_all()
        })
        .unwrap();
        assert!(called.get());
        drop(file);
        assert_eq!(std::fs::read(path).unwrap(), b"STAGED");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_failed_staged_flush_is_not_ignored() {
        let dir = test_dir("flush-failure");
        let (_, file) = create_staging(&dir, "out.pdf", StagingName::SuffixThenTag, [1]).unwrap();
        let error = sync_staged(&file, |_| Err(std::io::Error::other("disk gone"))).unwrap_err();
        assert_eq!(error.to_string(), "disk gone");
        drop(file);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_staging_name_cannot_traverse_out_of_its_directory() {
        let root = test_dir("traversal");
        let dir = root.join("selected");
        // The leading dot added by the staging format used to turn this into a traversable
        // directory: selected/.escape/../../outside.pdf.<suffix>.pdfcraft-tmp.
        std::fs::create_dir_all(dir.join(".escape")).unwrap();
        let original = root.join("original.pdf");
        std::fs::write(&original, b"ORIGINAL").unwrap();
        for form in [StagingName::SuffixThenTag, StagingName::TagThenSuffix] {
            let error = create_staging(&dir, "escape/../../outside.pdf", form, [1]).unwrap_err();
            assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
        }
        assert_eq!(std::fs::read(&original).unwrap(), b"ORIGINAL");
        assert_eq!(std::fs::read_dir(&root).unwrap().count(), 2, "nothing staged beside the selected directory");
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1, "nothing staged inside it either");
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn invalid_names_are_rejected_before_using_a_suffix_or_touching_the_filesystem() {
        let dir = test_dir("invalid");
        let names = ["", ".", "..", "../out.pdf", "nested/out.pdf", "out.pdf/", "/out.pdf", "nested/../out.pdf", "out.pdf/.", "out\0.pdf"];
        for name in names {
            for form in [StagingName::SuffixThenTag, StagingName::TagThenSuffix] {
                let suffixes = std::iter::from_fn(|| panic!("an invalid name consumed a suffix"));
                let error = create_staging(&dir.join("missing"), name, form, suffixes).unwrap_err();
                assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput, "{name:?}");
            }
        }
        // Validation must precede truncation: an invalid path must not become a valid prefix.
        let long_path = format!("{}/out.pdf", "x".repeat(200));
        let error = create_staging(&dir, &long_path, StagingName::TagThenSuffix, [1]).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn windows_paths_and_alternate_streams_are_not_staging_names() {
        let dir = test_dir("windows-invalid");
        for name in [r"..\out.pdf", r"nested\out.pdf", r"C:out.pdf", r"C:\out.pdf", r"\\server\share\out.pdf", "out.pdf:payload"] {
            for form in [StagingName::SuffixThenTag, StagingName::TagThenSuffix] {
                let error = create_staging(&dir, name, form, [1]).unwrap_err();
                assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput, "{name:?}");
            }
        }
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn ordinary_native_filenames_still_stage_beside_the_target() {
        use std::io::Write;
        let dir = test_dir("valid");
        for name in ["report.pdf", "revision..v2.pdf", ".private.pdf", "..not-parent.pdf", "tender 2026.pdf", "مناقصة.pdf"] {
            for form in [StagingName::SuffixThenTag, StagingName::TagThenSuffix] {
                let (path, mut file) = create_staging(&dir, name, form, [1]).unwrap();
                assert_eq!(path.parent(), Some(dir.as_path()));
                file.write_all(b"STAGED").unwrap();
                drop(file);
                assert_eq!(std::fs::read(&path).unwrap(), b"STAGED");
            }
        }
        #[cfg(unix)]
        {
            // A backslash is a legal filename character on Unix, not a path separator.
            let (path, file) = create_staging(&dir, r"a\b.pdf", StagingName::TagThenSuffix, [1]).unwrap();
            drop(file);
            assert_eq!(path.parent(), Some(dir.as_path()));
        }
        std::fs::remove_dir_all(dir).unwrap();
    }

    /// The callers' tests plant files at these exact names; if a name changed, those tests would
    /// plant where nothing writes and pass without testing anything.
    #[test]
    fn each_form_keeps_its_exact_name() {
        let dir = test_dir("forms");
        let (tools, _) = create_staging(&dir, "out.pdf", StagingName::SuffixThenTag, [0xab]).unwrap();
        assert_eq!(file_name(&tools), ".out.pdf.00000000000000ab.pdfcraft-tmp");
        let (desktop, _) = create_staging(&dir, "out.pdf", StagingName::TagThenSuffix, [0xab]).unwrap();
        assert_eq!(file_name(&desktop), ".out.pdf.pdfcraft-00000000000000ab.tmp");
        assert_eq!(std::fs::read(&tools).unwrap(), b"", "a new, empty file");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn a_long_name_is_cut_to_128_bytes_between_characters() {
        let dir = test_dir("long");
        // One byte, then four-byte characters: the 128-byte limit falls inside the 32nd one.
        let name = format!("a{}.pdf", "\u{1F600}".repeat(40));
        let (tmp, _) = create_staging(&dir, &name, StagingName::SuffixThenTag, [1]).unwrap();
        assert_eq!(file_name(&tmp), format!(".a{}.0000000000000001.pdfcraft-tmp", "\u{1F600}".repeat(31)));
        // One-byte characters: exactly 128 of them are kept.
        let (tmp, _) = create_staging(&dir, &"x".repeat(200), StagingName::SuffixThenTag, [2]).unwrap();
        assert_eq!(file_name(&tmp), format!(".{}.0000000000000002.pdfcraft-tmp", "x".repeat(128)));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn running_out_of_suffixes_refuses_and_creates_nothing() {
        let dir = test_dir("empty");
        let e = create_staging(&dir, "out.pdf", StagingName::TagThenSuffix, []).unwrap_err();
        assert_eq!(e.kind(), std::io::ErrorKind::AlreadyExists);
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 0);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn only_the_first_sixteen_names_are_tried() {
        let dir = test_dir("cap");
        let name = |s: u64| dir.join(format!(".out.pdf.{s:016x}.pdfcraft-tmp"));
        for s in 1..=STAGING_ATTEMPTS as u64 {
            std::fs::write(name(s), "PLANTED").unwrap();
        }
        assert!(create_staging(&dir, "out.pdf", StagingName::SuffixThenTag, 1..).is_err());
        assert!(!name(STAGING_ATTEMPTS as u64 + 1).exists(), "a 17th name was tried");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
