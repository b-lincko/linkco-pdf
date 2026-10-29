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
use std::time::SystemTime;

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

/// The largest file whose pages the app will read to describe the other version, and whose bytes
/// it will read to compare against it. A version bigger than this is still reported as changed,
/// but only by its size and time: parsing a huge file to answer a prompt would freeze the window,
/// and the prompt is meant to be answered in a second.
pub const READ_LIMIT: u64 = 32 * 1024 * 1024;

/// What the file on disk holds, so the prompt can say what a save would replace.
///
/// Only facts the reader can act on: how many pages the other version has and when it was
/// written. Nothing here is required for the save to be safe — the stamp decides that — so a
/// version that cannot be read (encrypted, damaged, or larger than [`READ_LIMIT`]) still gets a
/// prompt, with less said about it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiskVersion {
    bytes: u64,
    modified: Option<SystemTime>,
    pages: Option<usize>,
}

impl DiskVersion {
    /// Read the facts about `path`. `None` when there is no regular file to describe.
    pub fn of_path(path: &Path) -> Option<Self> {
        let meta = std::fs::metadata(path).ok().filter(|m| m.is_file())?;
        let bytes = meta.len();
        let modified = meta.modified().ok();
        // Pages need the file parsed, which is only worth it while it is small enough to read.
        let pages = read_capped(path, READ_LIMIT)
            .and_then(|data| pdfcraft_render::inspect(std::sync::Arc::new(data), None).ok())
            .map(|info| info.pages.len());
        Some(Self { bytes, modified, pages })
    }

    /// The other version's size in bytes.
    pub fn bytes(self) -> u64 {
        self.bytes
    }

    /// When the other version was last written, if the filesystem reports it.
    pub fn modified(self) -> Option<SystemTime> {
        self.modified
    }

    /// How many pages it has; `None` when it could not be read.
    pub fn pages(self) -> Option<usize> {
        self.pages
    }

    /// Whether the version is too large to open beside this one for a comparison.
    pub fn too_large_to_compare(self) -> bool {
        self.bytes > READ_LIMIT
    }

    /// The file's bytes, for opening it as a document to compare against. `None` when it cannot
    /// be read or is larger than [`READ_LIMIT`]; the file may also have changed since
    /// [`Self::of_path`], which the caller should expect.
    pub fn read(self, path: &Path) -> Option<Vec<u8>> {
        read_capped(path, READ_LIMIT)
    }
}

/// Read at most `cap` bytes of `path`; `None` if the file is longer than that or cannot be read.
/// The read stops at `cap + 1`, so a huge file is never loaded to find out that it is huge.
fn read_capped(path: &Path, cap: u64) -> Option<Vec<u8>> {
    let file = std::fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.take(cap.saturating_add(1)).read_to_end(&mut bytes).ok()?;
    (bytes.len() as u64 <= cap).then_some(bytes)
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

    /// A two-page PDF, so a page count can be read back from disk.
    fn two_page_pdf() -> Vec<u8> {
        let objs: Vec<String> = vec![
            "<< /Type /Catalog /Pages 2 0 R >>".into(),
            "<< /Type /Pages /Kids [3 0 R 4 0 R] /Count 2 /MediaBox [0 0 200 300] >>".into(),
            "<< /Type /Page /Parent 2 0 R >>".into(),
            "<< /Type /Page /Parent 2 0 R >>".into(),
        ];
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

    /// The prompt says what a save would replace, so the facts are read from the file itself.
    #[test]
    fn the_version_on_disk_is_described_by_its_pages_and_size() {
        let dir = store("facts");
        let path = dir.join("other.pdf");
        let pdf = two_page_pdf();
        std::fs::write(&path, &pdf).unwrap();
        let facts = DiskVersion::of_path(&path).expect("a regular file");
        assert_eq!(facts.pages(), Some(2), "the other version's pages are counted");
        assert_eq!(facts.bytes(), pdf.len() as u64);
        assert!(facts.modified().is_some(), "the filesystem reports when it was written");
        assert!(!facts.too_large_to_compare());
        assert_eq!(facts.read(&path), Some(pdf), "its bytes can be opened for a comparison");
        std::fs::remove_dir_all(dir).unwrap();
    }

    /// A version that cannot be parsed is still reported: the prompt says less about it, and the
    /// save is still guarded.
    #[test]
    fn a_version_that_cannot_be_parsed_is_still_described_by_what_the_filesystem_knows() {
        let dir = store("unparseable");
        let path = dir.join("other.pdf");
        std::fs::write(&path, b"not a PDF at all").unwrap();
        let facts = DiskVersion::of_path(&path).expect("a regular file");
        assert_eq!(facts.pages(), None, "no page count is claimed for a file that cannot be read");
        assert_eq!(facts.bytes(), 14);
        assert!(facts.modified().is_some());
        // A directory or a missing path describes nothing at all.
        assert_eq!(DiskVersion::of_path(&dir), None);
        assert_eq!(DiskVersion::of_path(&dir.join("gone.pdf")), None);
        std::fs::remove_dir_all(dir).unwrap();
    }

    /// Reading stops at the cap, so a file too large to describe never has to be loaded to find
    /// out that it is too large.
    #[test]
    fn reading_a_file_stops_at_the_cap_rather_than_loading_it() {
        let dir = store("capped");
        let path = dir.join("big.pdf");
        std::fs::write(&path, vec![b'z'; 4096]).unwrap();
        assert_eq!(read_capped(&path, 4096).map(|b| b.len()), Some(4096), "exactly at the cap");
        assert_eq!(read_capped(&path, 4095), None, "one byte over is refused");
        assert_eq!(read_capped(&dir.join("gone.pdf"), 4096), None);
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
