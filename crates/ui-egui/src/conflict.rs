//! Detect that a document's file changed on disk since it was opened or last saved.
//!
//! Saving writes the whole file (or an incremental revision) over the old bytes. If another
//! program replaced the file meanwhile, saving would silently destroy that work, so the store
//! remembers a content stamp of the bytes the app loaded and compares it before overwriting.
//! The stamp covers length and a 64-bit content hash: a rewrite that leaves identical bytes is
//! not a conflict. A file that disappeared is not one either — writing recreates it.
//!
//! This is a best-effort check between two filesystem operations, not a lock or a transaction:
//! another program can still write between the check and the rename. Authentication, hostile
//! parents and symbolic links remain out of scope, as elsewhere in the save path.

use std::io::Read;
use std::path::Path;

/// What the app last saw in a file: its length and a content hash.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Stamp {
    len: u64,
    hash: u64,
}

impl Stamp {
    /// A stamp for bytes already in memory (as loaded when the file was opened).
    pub fn of_bytes(bytes: &[u8]) -> Self {
        Self { len: bytes.len() as u64, hash: hash(bytes) }
    }

    /// The length the stamp covers.
    pub fn len(self) -> u64 {
        self.len
    }

    /// Read `path` and stamp it. `None` when it cannot be read (missing, a directory, a
    /// permission error): an unreadable file is not evidence of a conflict.
    pub fn of_path(path: &Path) -> Option<Self> {
        let file = std::fs::File::open(path).ok()?;
        Self::of_reader(file)
    }

    fn of_reader(mut reader: impl Read) -> Option<Self> {
        let mut buffer = [0u8; 64 * 1024];
        let (mut len, mut hash) = (0u64, FNV_OFFSET);
        loop {
            let read = match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(_) => return None,
            };
            len += read as u64;
            hash = fnv(hash, &buffer[..read]);
        }
        Some(Self { len, hash })
    }
}

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

fn fnv(mut hash: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn hash(bytes: &[u8]) -> u64 {
    fnv(FNV_OFFSET, bytes)
}

/// Whether the file on disk (`now`) differs from what the app last saw (`then`).
pub fn changed(then: Stamp, now: Option<Stamp>) -> bool {
    now.is_some_and(|now| now != then)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store(tag: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("pdfcraft-conflict-{}-{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn a_stamp_taken_from_memory_matches_the_file_it_came_from() {
        let dir = store("round-trip");
        let path = dir.join("a.pdf");
        std::fs::write(&path, b"%PDF-1.7 first revision").unwrap();
        let then = Stamp::of_bytes(b"%PDF-1.7 first revision");
        assert_eq!(Stamp::of_path(&path), Some(then));
        assert!(!changed(then, Stamp::of_path(&path)));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn different_bytes_of_the_same_length_are_a_change() {
        let dir = store("same-length");
        let path = dir.join("a.pdf");
        let then = Stamp::of_bytes(b"%PDF-1.7 A");
        std::fs::write(&path, b"%PDF-1.7 B").unwrap();
        assert!(changed(then, Stamp::of_path(&path)));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_missing_or_unreadable_file_is_not_a_conflict_and_large_files_are_streamed() {
        let dir = store("missing");
        let stamp = Stamp::of_bytes(b"whatever");
        assert!(!changed(stamp, Stamp::of_path(&dir.join("gone.pdf"))));
        assert!(!changed(stamp, Stamp::of_path(&dir)));
        // Larger than the read buffer, so the chunk loop has to carry the hash across reads.
        let path = dir.join("big.pdf");
        let big = vec![0x5au8; 300 * 1024];
        std::fs::write(&path, &big).unwrap();
        assert_eq!(Stamp::of_path(&path), Some(Stamp::of_bytes(&big)));
        std::fs::remove_dir_all(dir).unwrap();
    }
}
