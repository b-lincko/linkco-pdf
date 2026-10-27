//! Notice a document's file being renamed, moved or deleted while it is open.
//!
//! Saving already refuses to replace a file another program changed ([`crate::conflict`]). That
//! check only runs when the user saves, so a file that was moved or deleted in the meantime stayed
//! invisible until then, and a document could sit for hours pointing at a path that no longer
//! holds its file. This module polls the paths of open documents every few seconds and says so
//! once each time the situation changes; it never writes, moves or reopens anything.
//!
//! A poll is cheap when nothing happened: one `metadata` call, compared with the length, the
//! modification time and (where the platform has one) the file's identity. Only when one of those
//! differs is the file read to compare its contents, so an unchanged file is never re-read and a
//! program that rewrote identical bytes is not reported as a change.
//!
//! Bounded on purpose: when the path no longer holds the recorded file, one folder is scanned for
//! its identity, never the disk, so a file moved to another folder is reported as gone rather than
//! chased. Identity needs Unix metadata; elsewhere a rename is reported as a missing file.
//! Symbolic links are followed by `metadata`, as everywhere else in the save path, and this is a
//! poll between two filesystem operations, not a lock: the file can change again after the poll.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::conflict::{Stamp, changed};

/// How often the open documents' files are checked.
pub const DISK_CHECK_SECS: f64 = 3.0;

/// How often a file whose size and time still match is also compared by content. A rewrite that
/// keeps the length inside the same timestamp tick as the last one is invisible to the cheap
/// check, so the contents are confirmed at this slower cadence as well.
pub const DISK_CONFIRM_SECS: f64 = 60.0;

/// Where a file was seen, so the same file can be recognised under another name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Identity {
    device: u64,
    inode: u64,
}

impl Identity {
    /// The identity of `path`, if the platform can report one for a regular file.
    pub fn of_path(path: &Path) -> Option<Self> {
        let meta = std::fs::metadata(path).ok()?;
        Self::of_metadata(&meta)
    }

    /// Identity from metadata already in hand.
    #[cfg(unix)]
    pub fn of_metadata(meta: &std::fs::Metadata) -> Option<Self> {
        use std::os::unix::fs::MetadataExt as _;
        meta.is_file().then(|| Self { device: meta.dev(), inode: meta.ino() })
    }

    /// Other platforms keep no identity here: their file ids need platform calls this app does
    /// not otherwise use, so a rename there is reported as a missing file instead.
    #[cfg(not(unix))]
    pub fn of_metadata(_meta: &std::fs::Metadata) -> Option<Self> {
        None
    }

    /// The path of the file with this identity inside `dir`.
    fn find_in(self, dir: &Path) -> Option<PathBuf> {
        for entry in std::fs::read_dir(dir).ok()?.flatten() {
            let path = entry.path();
            // `metadata` follows a link: a link that leads to the remembered file counts as it.
            if Self::of_path(&path) == Some(self) {
                return Some(path);
            }
        }
        None
    }
}

/// What the app remembers about a document's file: what it wrote or read, and where it was.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Watch {
    stamp: Stamp,
    identity: Option<Identity>,
    modified: Option<SystemTime>,
}

impl Watch {
    /// Record a file the app just read: `bytes` are its contents, so no second read is needed.
    pub fn of_bytes(path: &Path, bytes: &[u8]) -> Self {
        let meta = std::fs::metadata(path).ok();
        Self {
            stamp: Stamp::of_bytes(bytes),
            identity: meta.as_ref().and_then(Identity::of_metadata),
            modified: meta.and_then(|m| m.modified().ok()),
        }
    }

    /// Record a file by reading it (after the app wrote it, it must confirm what landed).
    pub fn of_path(path: &Path) -> Option<Self> {
        let meta = std::fs::metadata(path).ok()?;
        let stamp = Stamp::of_path(path)?;
        Some(Self { stamp, identity: Identity::of_metadata(&meta), modified: meta.modified().ok() })
    }

    /// The stamped length, for the cheap "looks untouched" test.
    pub fn len(self) -> u64 {
        self.stamp.len()
    }

    /// The content stamp, for the save-time check that refuses a silent overwrite.
    pub fn stamp(self) -> Stamp {
        self.stamp
    }

    /// What `path` holds now, compared with this memory of it. `deep` also confirms the contents
    /// when the size and modification time still match, which catches a same-length rewrite
    /// inside one timestamp tick at the cost of reading the file.
    ///
    /// The contents decide, exactly as the save-time check does: a replacement that happens to
    /// hold the same bytes is not a loss, and a different file under the same name is reported as
    /// a change rather than as a move. Identity is only consulted when the path no longer holds a
    /// regular file at all, to find where the remembered one went.
    pub fn probe(self, path: &Path, deep: bool) -> Alert {
        if let Ok(meta) = std::fs::metadata(path)
            && meta.is_file()
        {
            // Size and modification time are only trusted for the cheap check: several filesystems
            // and clocks report whole seconds, so a quick rewrite of the same length can leave
            // both unchanged. The save-time check always compares contents, and the periodic deep
            // pass closes the gap here.
            let untouched = !deep && meta.len() == self.len() && self.modified.is_some() && meta.modified().ok() == self.modified;
            return if untouched || !changed(self.stamp, Stamp::of_path(path)) { Alert::Fine } else { Alert::Changed };
        }
        // The path does not hold a regular file. If the remembered file is still in the same
        // folder it was renamed or moved within it; otherwise it was deleted or moved out.
        let parent = path.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new("."));
        match self.identity.and_then(|id| id.find_in(parent)) {
            Some(found) => Alert::Renamed(found),
            None => Alert::Gone,
        }
    }
}

/// What a poll found for one document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Alert {
    /// The file is where the app left it and holds the bytes it wrote.
    Fine,
    /// The file holds different bytes: another program wrote to it.
    Changed,
    /// The remembered file is no longer at its path but is here instead.
    Renamed(PathBuf),
    /// No file with the remembered identity is at the path or anywhere in its folder.
    Gone,
}

#[cfg(not(target_arch = "wasm32"))]
impl crate::PdfCraftApp {
    /// Check the files of the open documents, at most every [`DISK_CHECK_SECS`].
    ///
    /// A change is reported once for each new situation: another program writing the file, the
    /// file being moved or renamed inside its folder, and the file being gone. Nothing is
    /// reported while a dialog or prompt is open — the user is answering something else, and the
    /// notice would arrive on top of it — and the same state is not repeated, so an idle window
    /// stays quiet. Every check is a read-only look at the filesystem.
    pub(crate) fn disk_tick(&mut self, now: f64) {
        if now - self.last_disk_check < DISK_CHECK_SECS {
            return;
        }
        self.last_disk_check = now;
        if self.modal_open() || self.dialog.is_some() || self.password_prompt.is_some() {
            return;
        }
        let mut reports = Vec::new();
        for view in &self.views {
            let id = view.id;
            let Some(watch) = self.disk_watch.get(&id).copied() else { continue };
            let Some(path) = self.session.get(id).and_then(|d| d.path.clone()) else { continue };
            let last = self.disk_verified.get(&id).copied().unwrap_or(f64::NEG_INFINITY);
            let deep = now - last >= DISK_CONFIRM_SECS;
            if deep {
                self.disk_verified.insert(id, now);
            }
            let alert = watch.probe(std::path::Path::new(&path), deep);
            if self.disk_alerts.get(&id) == Some(&alert) {
                continue;
            }
            let name = self.session.get(id).map(|d| d.name.clone()).unwrap_or_default();
            reports.push((id, alert, name, path));
        }
        for (id, alert, name, path) in reports {
            match &alert {
                Alert::Fine => {
                    self.disk_alerts.remove(&id);
                }
                Alert::Changed => {
                    self.disk_alerts.insert(id, alert);
                    // The same words the save-time prompt uses: both describe one situation.
                    self.notify_fmt("“{name}” changed on disk while you were working on it.", &[("name", &name)]);
                }
                Alert::Renamed(to) => {
                    let to = to.display().to_string();
                    self.disk_alerts.insert(id, alert);
                    self.notify_fmt("“{name}” was moved or renamed to “{found}”.", &[("name", &name), ("found", &to)]);
                }
                Alert::Gone => {
                    self.disk_alerts.insert(id, alert);
                    self.notify_fmt(
                        "“{name}” is no longer at “{path}”; it was moved to another folder or deleted.",
                        &[("name", &name), ("path", &path)],
                    );
                }
            }
        }
    }

    /// Check the open documents' files now, whatever the interval says, and return what each one
    /// is like (tests and automation; the frame loop uses [`Self::disk_tick`]).
    pub fn check_disk_now(&mut self) -> Vec<(pdfcraft_engine::DocId, Alert)> {
        self.last_disk_check = f64::NEG_INFINITY;
        self.disk_verified.clear();
        self.disk_tick(0.0);
        let mut states: Vec<_> = self.disk_alerts.iter().map(|(id, alert)| (*id, alert.clone())).collect();
        states.sort_by_key(|(id, _)| id.0);
        states
    }
}

#[cfg(target_arch = "wasm32")]
impl crate::PdfCraftApp {
    /// The web build has no path-based files to watch ([`crate::disk_watch`] is desktop-only).
    /// The bookkeeping fields are still touched here, so the shared struct needs no desktop-only
    /// `cfg` on a dozen field initializers.
    pub(crate) fn disk_tick(&mut self, now: f64) {
        self.last_disk_check = now.max(self.last_disk_check);
        self.disk_verified.clear();
        self.disk_alerts.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(tag: &str) -> std::io::Result<PathBuf> {
        let dir = std::env::temp_dir().join(format!("pdfcraft-disk-watch-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    #[test]
    fn an_untouched_file_is_fine_and_is_not_read_again() {
        let dir = dir("fine").unwrap();
        let path = dir.join("a.pdf");
        std::fs::write(&path, b"%PDF-1.7 revision one").unwrap();
        let watch = Watch::of_bytes(&path, b"%PDF-1.7 revision one");
        assert_eq!(watch.probe(&path, false), Alert::Fine);
        assert_eq!(watch.probe(&path, true), Alert::Fine);
        // Repeated polls keep saying nothing happened, cheap or deep.
        assert_eq!(watch.probe(&path, false), Alert::Fine);
    }

    #[test]
    fn another_program_writing_the_file_is_reported_as_changed() {
        let dir = dir("changed").unwrap();
        let path = dir.join("a.pdf");
        std::fs::write(&path, b"%PDF-1.7 revision one").unwrap();
        let watch = Watch::of_bytes(&path, b"%PDF-1.7 revision one");
        // Different length is caught from the metadata alone.
        std::fs::write(&path, b"%PDF-1.7 revision one and more").unwrap();
        assert_eq!(watch.probe(&path, false), Alert::Changed);
        // Same length, different bytes: only the content tells them apart, so the deep pass
        // catches it even when the rewrite lands in the same timestamp tick.
        std::fs::write(&path, b"%PDF-1.7 revision two").unwrap();
        assert_eq!(watch.probe(&path, true), Alert::Changed);
    }

    #[test]
    fn rewriting_identical_bytes_is_not_a_change() {
        let dir = dir("identical").unwrap();
        let path = dir.join("a.pdf");
        std::fs::write(&path, b"%PDF-1.7 same").unwrap();
        let watch = Watch::of_bytes(&path, b"%PDF-1.7 same");
        // Rewritten but with the same bytes: nothing was lost, and the deep pass agrees.
        std::fs::write(&path, b"%PDF-1.7 same").unwrap();
        assert_eq!(watch.probe(&path, true), Alert::Fine);
    }

    #[test]
    fn a_deleted_file_is_reported_as_gone() {
        let dir = dir("deleted").unwrap();
        let path = dir.join("a.pdf");
        std::fs::write(&path, b"%PDF-1.7 one").unwrap();
        let watch = Watch::of_bytes(&path, b"%PDF-1.7 one");
        std::fs::remove_file(&path).unwrap();
        assert_eq!(watch.probe(&path, false), Alert::Gone);
    }

    #[cfg(unix)]
    #[test]
    fn a_renamed_file_is_found_by_its_identity() {
        let dir = dir("renamed").unwrap();
        let path = dir.join("a.pdf");
        let moved = dir.join("b.pdf");
        std::fs::write(&path, b"%PDF-1.7 one").unwrap();
        let watch = Watch::of_bytes(&path, b"%PDF-1.7 one");
        std::fs::rename(&path, &moved).unwrap();
        assert_eq!(watch.probe(&path, false), Alert::Renamed(moved.clone()));
        // The remembered file is still the one being watched from its new name.
        assert_eq!(watch.probe(&moved, false), Alert::Fine);
    }

    #[cfg(unix)]
    #[test]
    fn a_new_file_taking_the_name_is_reported_by_its_contents() {
        let dir = dir("replaced").unwrap();
        let path = dir.join("a.pdf");
        std::fs::write(&path, b"%PDF-1.7 one").unwrap();
        let watch = Watch::of_bytes(&path, b"%PDF-1.7 one");
        // The app writes a new file and swaps it in: the bytes differ, so the save is a conflict.
        std::fs::remove_file(&path).unwrap();
        std::fs::write(&path, b"%PDF-1.7 other program").unwrap();
        assert_eq!(watch.probe(&path, false), Alert::Changed);
        // A replacement that happens to hold the same bytes is not a loss.
        std::fs::remove_file(&path).unwrap();
        std::fs::write(&path, b"%PDF-1.7 one").unwrap();
        assert_eq!(watch.probe(&path, true), Alert::Fine);
    }
}
