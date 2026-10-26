//! Autosave and crash recovery.
//!
//! While documents have unsaved changes, their working file is written every
//! `AUTOSAVE_SECS` to a recovery folder in the user's data directory. Each entry is
//! `<key>.pdf` plus `<key>.json` (name, original path, time). Saving, discarding or closing a
//! document removes its entry, and a clean quit leaves the folder empty. If entries are found at
//! startup, the previous session ended unexpectedly and the app offers to recover them.
//!
//! Encrypted documents are stored encrypted (the working file is), so recovery never writes
//! plaintext of a protected document to disk. The web build has no recovery store yet.
//! A snapshot is acknowledged only after both store writes succeed; failed revisions remain
//! pending for retry without another edit. The PDF and metadata are still separate replacements.
//! Recovery keys are bounded portable identifiers, never paths. Discovered JSON must name its
//! own filename key. This does not authenticate metadata or sandbox symbolic links/parent races.

use std::path::{Path, PathBuf};

use crate::PdfCraftApp;

/// How often unsaved changes are written to the recovery folder.
pub const AUTOSAVE_SECS: f64 = 60.0;

/// What a recovery entry records next to the PDF bytes.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RecoveryMeta {
    /// A portable filename identifier, not a path (at most 128 UTF-8 bytes).
    pub key: String,
    pub name: String,
    /// Where the document was saved before (Save writes back there after recovery).
    pub path: Option<String>,
    /// Seconds since the Unix epoch when the snapshot was taken.
    pub saved_at: u64,
    /// The bytes are encrypted; recovering asks for the password.
    pub encrypted: bool,
}

/// The recovery folder.
#[derive(Clone, Debug)]
pub struct RecoveryStore {
    dir: PathBuf,
}

impl RecoveryStore {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    /// `Recovery` in the portable data folder in portable mode ([`crate::portable`]), otherwise
    /// the platform's per-user data folder: `~/Library/Application Support/Linkco PDF Editor/Recovery`
    /// (macOS), `%LOCALAPPDATA%\Linkco PDF Editor\Recovery` (Windows), or
    /// `$XDG_DATA_HOME/linkco-pdf-editor/recovery` / `~/.local/share/linkco-pdf-editor/recovery` (others).
    pub fn default_dir() -> Option<PathBuf> {
        if let Some(dir) = crate::portable::data_dir() {
            return Some(dir.join("Recovery"));
        }
        let env = |k: &str| std::env::var_os(k).filter(|v| !v.is_empty()).map(PathBuf::from);
        if cfg!(target_os = "macos") {
            env("HOME").map(|h| h.join("Library/Application Support/Linkco PDF Editor/Recovery"))
        } else if cfg!(windows) {
            env("LOCALAPPDATA").map(|d| d.join("Linkco PDF Editor").join("Recovery"))
        } else {
            env("XDG_DATA_HOME").or_else(|| env("HOME").map(|h| h.join(".local/share"))).map(|d| d.join("linkco-pdf-editor/recovery"))
        }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Derive both entry paths only after checking the entire key. Generated keys already obey
    /// this portable policy; normal Unicode identifiers remain valid. This is lexical validation,
    /// not protection against symbolic links or replacement of the store directory itself.
    fn entry_paths(&self, key: &str) -> std::io::Result<(PathBuf, PathBuf)> {
        let invalid = key.is_empty()
            || key.len() > 128
            || key.starts_with(' ')
            || key.ends_with(['.', ' '])
            || key.chars().any(|c| c.is_control() || matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|'));
        let invalid_key = || {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "a recovery key must be a portable filename identifier of 1–128 bytes, not a path or device name",
            )
        };
        // Bound the allocation below as well as the eventual filenames.
        if invalid {
            return Err(invalid_key());
        }
        // Windows treats these names as devices even with an extension; reject them on every
        // platform so a recovery key has the same meaning when the store is copied elsewhere.
        let device = key.split('.').next().unwrap_or_default().trim_end_matches(' ').to_ascii_uppercase();
        let reserved = matches!(device.as_str(), "CON" | "PRN" | "AUX" | "NUL" | "CLOCK$" | "CONIN$" | "CONOUT$")
            || ["COM", "LPT"].iter().any(|prefix| {
                device.strip_prefix(prefix).is_some_and(|n| matches!(n, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"))
            });
        if reserved {
            return Err(invalid_key());
        }
        Ok((self.dir.join(format!("{key}.pdf")), self.dir.join(format!("{key}.json"))))
    }

    /// Write (or replace) an entry. The PDF is written before its metadata, both atomically, so
    /// a listed entry always has complete bytes.
    pub fn write(&self, meta: &RecoveryMeta, bytes: &[u8]) -> std::io::Result<()> {
        let (pdf, metadata) = self.entry_paths(&meta.key)?;
        std::fs::create_dir_all(&self.dir)?;
        crate::editing::write_atomically(&pdf.to_string_lossy(), bytes)?;
        let json = serde_json::to_vec_pretty(meta).map_err(std::io::Error::other)?;
        crate::editing::write_atomically(&metadata.to_string_lossy(), &json)
    }

    /// Remove a pair; invalid keys are ignored without filesystem access.
    pub fn remove(&self, key: &str) {
        let Ok((pdf, metadata)) = self.entry_paths(key) else { return };
        let _ = std::fs::remove_file(metadata);
        let _ = std::fs::remove_file(pdf);
    }

    /// Complete entries, newest first. Metadata keys must match their filenames. Invalid JSON
    /// entries and incomplete leftovers (bytes without metadata) are removed inside the store.
    pub fn list(&self) -> Vec<RecoveryMeta> {
        let Ok(dir) = std::fs::read_dir(&self.dir) else { return Vec::new() };
        let mut out = Vec::new();
        for entry in dir.flatten() {
            let path = entry.path();
            match path.extension().and_then(|e| e.to_str()) {
                Some("json") => {
                    let meta = std::fs::read(&path).ok().and_then(|b| serde_json::from_slice::<RecoveryMeta>(&b).ok());
                    // Do not trust a JSON key to identify a different entry, even inside the
                    // store. Check the filename binding before probing its paired PDF.
                    match meta.filter(|m| {
                        self.entry_paths(&m.key).is_ok_and(|(pdf, json)| {
                            // Case/Unicode-normalizing filesystems may return a different native
                            // spelling of the same filename. Different regular files still fail.
                            // Canonicalization follows links: this is not a symlink sandbox.
                            let same_json =
                                json == path || matches!((std::fs::canonicalize(&json), std::fs::canonicalize(&path)), (Ok(a), Ok(b)) if a == b);
                            same_json && pdf.is_file()
                        })
                    }) {
                        Some(m) => out.push(m),
                        _ => {
                            let _ = std::fs::remove_file(&path);
                        }
                    }
                }
                Some("pdf") if !path.with_extension("json").is_file() => {
                    let _ = std::fs::remove_file(&path);
                }
                _ => {}
            }
        }
        out.sort_by_key(|m| std::cmp::Reverse(m.saved_at));
        out
    }

    pub fn read(&self, key: &str) -> std::io::Result<Vec<u8>> {
        let (pdf, _) = self.entry_paths(key)?;
        std::fs::read(pdf)
    }

    /// Remove every entry (clean quit, or "Discard all").
    pub fn clear(&self) {
        for m in self.list() {
            self.remove(&m.key);
        }
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

impl PdfCraftApp {
    /// Turn on autosave into `store`, and look for documents a previous session left behind.
    pub fn enable_recovery(&mut self, store: RecoveryStore) {
        self.recoverable = store.list();
        if !self.recoverable.is_empty() {
            self.dialog = Some(crate::Dialog::Recovery);
        }
        self.recovery = Some(store);
    }

    /// Write snapshots of documents changed since the last autosave (called periodically).
    pub fn autosave_now(&mut self) {
        let Some(store) = self.recovery.clone() else { return };
        for snap in self.session.autosave_snapshots() {
            let key = self.recovery_keys.entry(snap.doc).or_insert_with(|| format!("{}-{}-{}", now_secs(), std::process::id(), snap.doc.0)).clone();
            let meta = RecoveryMeta { key, name: snap.name.clone(), path: snap.path.clone(), saved_at: now_secs(), encrypted: snap.encrypted };
            match store.write(&meta, &snap.bytes) {
                Ok(()) => {
                    self.session.acknowledge_autosave(&snap);
                }
                Err(e) => log::warn!("autosave failed: {e}"),
            }
        }
    }

    /// Drop a document's recovery entry (it was saved, discarded or closed).
    pub(crate) fn forget_recovery(&mut self, doc: pdfcraft_engine::DocId) {
        if let (Some(store), Some(key)) = (&self.recovery, self.recovery_keys.remove(&doc)) {
            store.remove(&key);
        }
    }

    /// Autosave when the interval has passed (called every frame).
    pub(crate) fn autosave_tick(&mut self, now: f64) {
        if self.recovery.is_none() {
            return;
        }
        if now - self.last_autosave >= AUTOSAVE_SECS {
            self.last_autosave = now;
            self.autosave_now();
        }
    }

    /// Reopen recovered documents (`keys`), as unsaved changes at their original paths.
    pub fn recover(&mut self, keys: &[String]) {
        let Some(store) = self.recovery.clone() else { return };
        for key in keys {
            // One prompt owns its bytes and recovery association until answered. Leave all
            // later entries on offer instead of replacing a prompt with the next document.
            if self.password_prompt.is_some() {
                break;
            }
            let Some(meta) = self.recoverable.iter().find(|m| &m.key == key).cloned() else { continue };
            let bytes = match store.read(key) {
                Ok(b) => b,
                Err(e) => {
                    self.notify_fmt("Couldn't recover {name}: {e}", &[("name", &meta.name), ("e", &e.to_string())]);
                    continue;
                }
            };
            match self.open_bytes(&meta.name, None, bytes) {
                Ok(()) => {
                    if let Some(prompt) = self.password_prompt.as_mut() {
                        prompt.recovered = Some(meta.clone());
                    } else {
                        self.finish_recovery(&meta);
                    }
                    // Only a successful open (or its pending password prompt) consumes the
                    // offer. Read/open failures and deferred entries remain retryable.
                    self.recoverable.retain(|m| &m.key != key);
                }
                Err(e) => self.notify_fmt("Couldn't recover {name}: {e}", &[("name", &meta.name), ("e", &e.to_string())]),
            }
        }
    }

    pub(crate) fn show_pending_recovery(&mut self) {
        if !self.recoverable.is_empty() && self.password_prompt.is_none() && self.dialog.is_none() {
            self.dialog = Some(crate::Dialog::Recovery);
        }
    }

    pub(crate) fn return_recovery(&mut self, meta: RecoveryMeta) {
        if !self.recoverable.iter().any(|m| m.key == meta.key) {
            self.recoverable.push(meta);
        }
        self.show_pending_recovery();
    }

    /// The most recently opened tab came from `meta`: keep its recovery entry and path.
    pub(crate) fn finish_recovery(&mut self, meta: &RecoveryMeta) {
        let Some(id) = self.views.last().map(|v| v.id) else { return };
        self.session.mark_recovered(id, meta.path.clone());
        self.recovery_keys.insert(id, meta.key.clone());
    }

    /// Delete recovery entries the user chose not to recover.
    pub fn discard_recovered(&mut self, keys: &[String]) {
        if let Some(store) = &self.recovery {
            for k in keys {
                store.remove(k);
            }
        }
        self.recoverable.retain(|m| !keys.contains(&m.key));
    }

    /// Clean shutdown: documents were saved or discarded, so nothing needs recovering.
    pub fn shutdown_recovery(&mut self) {
        if self.first_dirty().is_none()
            && let Some(store) = &self.recovery
        {
            for key in self.recovery_keys.values() {
                store.remove(key);
            }
            self.recovery_keys.clear();
        }
    }
}

#[cfg(test)]
mod store_tests {
    use super::*;

    fn fixture_store(tag: &str) -> (PathBuf, RecoveryStore) {
        let root = std::env::temp_dir().join(format!("pdfcraft-recovery-keys-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        let store = RecoveryStore::new(root.join("store"));
        (root, store)
    }

    fn metadata(key: &str) -> RecoveryMeta {
        RecoveryMeta { key: key.into(), name: "work.pdf".into(), path: Some("/docs/work.pdf".into()), saved_at: 7, encrypted: false }
    }

    #[test]
    fn invalid_recovery_keys_are_rejected_before_writing_or_creating_the_store() {
        let (root, store) = fixture_store("invalid-write");
        std::fs::write(root.join("victim.pdf"), b"ORIGINAL PDF").unwrap();
        std::fs::write(root.join("victim.json"), b"ORIGINAL METADATA").unwrap();
        let absolute = root.join("victim").to_string_lossy().into_owned();
        let long = "a".repeat(129);
        let long_unicode = "é".repeat(65);
        for key in [
            "../victim",
            absolute.as_str(),
            "",
            ".",
            "..",
            "a/b",
            "a\\b",
            "a:b",
            "a\0b",
            "a\nb",
            "a*b",
            "a?b",
            "a<b",
            "a>b",
            "a|b",
            "a\"b",
            "trailing.",
            "trailing ",
            " CON",
            "CON .txt",
            "CON",
            "con",
            "CON.txt",
            "AUX",
            "PRN",
            "NUL",
            "COM1",
            "COM9",
            "LPT1",
            "LPT9",
            "COM¹",
            "LPT²",
            "CONIN$",
            "CONOUT$",
            long.as_str(),
            long_unicode.as_str(),
        ] {
            let error = store.write(&metadata(key), b"REPLACEMENT").unwrap_err();
            assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput, "{key:?}");
            assert!(!store.dir().exists(), "validation must precede directory creation: {key:?}");
            assert_eq!(std::fs::read(root.join("victim.pdf")).unwrap(), b"ORIGINAL PDF");
            assert_eq!(std::fs::read(root.join("victim.json")).unwrap(), b"ORIGINAL METADATA");
        }
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn reads_reject_escape_keys_before_probing_a_target() {
        let (root, store) = fixture_store("invalid-read");
        std::fs::create_dir(store.dir()).unwrap();
        std::fs::write(root.join("victim.pdf"), b"OUTSIDE").unwrap();
        let absolute = root.join("victim").to_string_lossy().into_owned();
        for key in ["../victim", absolute.as_str(), "missing/child", "missing:stream", "NUL", ""] {
            assert_eq!(store.read(key).unwrap_err().kind(), std::io::ErrorKind::InvalidInput, "{key:?}");
        }
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn removal_ignores_escape_keys_and_preserves_outside_files() {
        let (root, store) = fixture_store("invalid-remove");
        std::fs::create_dir(store.dir()).unwrap();
        let absolute = root.join("victim").to_string_lossy().into_owned();
        for key in ["../victim", absolute.as_str()] {
            std::fs::write(root.join("victim.pdf"), b"OUTSIDE PDF").unwrap();
            std::fs::write(root.join("victim.json"), b"OUTSIDE JSON").unwrap();
            store.remove(key);
            assert_eq!(std::fs::read(root.join("victim.pdf")).unwrap(), b"OUTSIDE PDF");
            assert_eq!(std::fs::read(root.join("victim.json")).unwrap(), b"OUTSIDE JSON");
        }
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn listing_and_clearing_reject_escape_keys_and_mismatched_metadata() {
        let (root, store) = fixture_store("poisoned-list");
        store.write(&metadata("valid"), b"VALID SNAPSHOT").unwrap();
        std::fs::write(root.join("victim.pdf"), b"OUTSIDE PDF").unwrap();
        std::fs::write(root.join("victim.json"), b"OUTSIDE JSON").unwrap();
        for (file, key) in [("escape.json", "../victim"), ("alias.json", "valid")] {
            std::fs::write(store.dir().join(file), serde_json::to_vec(&metadata(key)).unwrap()).unwrap();
        }
        // clear used to follow the forged traversal key and delete the outside pair.
        store.clear();
        assert_eq!(std::fs::read(root.join("victim.pdf")).unwrap(), b"OUTSIDE PDF");
        assert_eq!(std::fs::read(root.join("victim.json")).unwrap(), b"OUTSIDE JSON");
        assert!(!store.dir().join("valid.pdf").exists(), "valid entries are still cleared");
        assert!(store.list().is_empty());
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn a_metadata_alias_cannot_impersonate_a_valid_entry() {
        let (root, store) = fixture_store("alias");
        store.write(&metadata("valid"), b"VALID SNAPSHOT").unwrap();
        std::fs::write(store.dir().join("alias.json"), serde_json::to_vec(&metadata("valid")).unwrap()).unwrap();
        assert_eq!(store.list(), [metadata("valid")]);
        assert!(!store.dir().join("alias.json").exists());
        assert_eq!(store.read("valid").unwrap(), b"VALID SNAPSHOT");
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn ordinary_and_generated_recovery_keys_round_trip_without_changing_original_paths() {
        let (root, store) = fixture_store("valid");
        let limit = "a".repeat(128);
        let unicode_limit = "é".repeat(64);
        for key in ["1800000000-12345-9", "a_b-c", ".hidden", "report.v2", "clé-鍵", "COM10", "conductor", limit.as_str(), unicode_limit.as_str()] {
            let meta = metadata(key);
            store.write(&meta, b"COMPLETE SNAPSHOT").unwrap();
            assert_eq!(store.list(), [meta]);
            assert_eq!(store.read(key).unwrap(), b"COMPLETE SNAPSHOT");
            store.remove(key);
            assert!(store.list().is_empty());
        }
        std::fs::remove_dir_all(root).unwrap();
    }
}
