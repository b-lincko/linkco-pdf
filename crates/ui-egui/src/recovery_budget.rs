//! Keep the recovery folder from growing without limit.
//!
//! Every unsaved document with autosave on leaves a snapshot and its metadata in the recovery
//! folder, and a snapshot is replaced rather than removed while the document stays unsaved. The
//! folder is only cleared when the user saves, discards or quits cleanly, so a crash followed by
//! weeks of not opening the app, or a long session with many large documents, can leave snapshots
//! that nothing will ever look at again. This module decides which of them to delete once the
//! folder passes a fixed budget.
//!
//! The rule is deliberately dull: stay within [`Limits::MAX_ENTRIES`] entries and
//! [`Limits::MAX_BYTES`] bytes in total, deleting the least recently saved first. The entry that
//! was just written is never deleted, even if it alone is larger than the budget — a snapshot the
//! app is holding as the user's only copy of unsaved work is worse to lose than to keep over
//! budget, and the next save or clean quit removes it anyway.
//!
//! Ordering is by the time recorded in the metadata, ties broken by key, so the same folder always
//! produces the same decision and a test can pin it. Nothing here touches the filesystem.

/// One entry in the recovery folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    /// The entry's portable identifier (its filename without the extension).
    pub key: String,
    /// The snapshot's size in bytes (the metadata beside it is small and not counted).
    pub bytes: u64,
    /// Seconds since the Unix epoch, as recorded when the snapshot was taken.
    pub saved_at: u64,
}

/// What the recovery folder is allowed to hold.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub max_entries: usize,
    pub max_bytes: u64,
}

impl Limits {
    /// 200 snapshots or 512 MiB, whichever comes first. Office documents are far below that;
    /// scanned ones are not, and the byte budget is what actually binds there.
    pub const DEFAULT: Self = Self { max_entries: 200, max_bytes: 512 * 1024 * 1024 };

    /// Whether `entries` may stay as they are.
    pub fn satisfied_by(self, entries: &[Entry]) -> bool {
        entries.len() <= self.max_entries && total_bytes(entries) <= self.max_bytes
    }
}

impl Default for Limits {
    fn default() -> Self {
        Self::DEFAULT
    }
}

fn total_bytes(entries: &[Entry]) -> u64 {
    entries.iter().fold(0u64, |sum, e| sum.saturating_add(e.bytes))
}

/// The keys to delete so the folder fits `limits`, least recently saved first.
///
/// `keep` is the entry just written (its key); it is never returned. The newest entry is kept as
/// well, and the folder is never emptied: a snapshot is only deleted when something newer is
/// being kept, so a recovery folder can never lose its last copy of unsaved work to housekeeping,
/// even if that copy alone is over budget. An empty result means nothing needs deleting. The
/// result is ordered oldest first, so a caller that deletes as it goes removes the least valuable
/// snapshot before the more recent ones.
pub fn to_evict(entries: &[Entry], limits: Limits, keep: Option<&str>) -> Vec<String> {
    if limits.satisfied_by(entries) {
        return Vec::new();
    }
    let mut oldest: Vec<&Entry> = entries.iter().filter(|e| Some(e.key.as_str()) != keep).collect();
    oldest.sort_by(|a, b| a.saved_at.cmp(&b.saved_at).then_with(|| a.key.cmp(&b.key)));
    // The newest entry is the last resort: never delete every snapshot in the folder.
    let newest = oldest.last().map(|e| e.key.clone());
    let mut count = entries.len();
    let mut bytes = total_bytes(entries);
    let mut evicted = Vec::new();
    for entry in oldest {
        if count <= limits.max_entries && bytes <= limits.max_bytes {
            break;
        }
        count = count.saturating_sub(1);
        bytes = bytes.saturating_sub(entry.bytes);
        evicted.push(entry.key.clone());
    }
    // Never leave the folder empty: if every entry was cut, the newest one stays, whatever the
    // budget says. (`keep`, when it names an entry that exists, already prevents this.)
    if evicted.len() == entries.len()
        && let Some(newest) = newest
    {
        evicted.retain(|key| *key != newest);
    }
    evicted
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(key: &str, bytes: u64, saved_at: u64) -> Entry {
        Entry { key: key.into(), bytes, saved_at }
    }

    fn limits(max_entries: usize, max_bytes: u64) -> Limits {
        Limits { max_entries, max_bytes }
    }

    #[test]
    fn a_folder_within_budget_keeps_everything() {
        let entries = vec![entry("a", 10, 1), entry("b", 10, 2)];
        assert!(to_evict(&entries, limits(2, 20), None).is_empty());
        assert!(to_evict(&entries, Limits::DEFAULT, None).is_empty());
        assert!(to_evict(&[], Limits::DEFAULT, None).is_empty());
    }

    #[test]
    fn too_many_entries_evicts_the_oldest_first() {
        let entries = vec![entry("old", 1, 10), entry("middle", 1, 20), entry("new", 1, 30)];
        assert_eq!(to_evict(&entries, limits(2, 100), None), vec!["old"]);
        assert_eq!(to_evict(&entries, limits(1, 100), None), vec!["old", "middle"]);
        // Even a zero budget leaves the newest snapshot: housekeeping never empties the folder.
        assert_eq!(to_evict(&entries, limits(0, 100), None), vec!["old", "middle"]);
    }

    #[test]
    fn too_many_bytes_evicts_until_it_fits() {
        let entries = vec![entry("a", 40, 1), entry("b", 40, 2), entry("c", 40, 3)];
        assert_eq!(to_evict(&entries, limits(10, 100), None), vec!["a"]);
        assert_eq!(to_evict(&entries, limits(10, 80), None), vec!["a"]);
        assert_eq!(to_evict(&entries, limits(10, 79), None), vec!["a", "b"]);
        // The newest snapshot stays even when it alone exceeds the budget.
        assert_eq!(to_evict(&entries, limits(10, 39), None), vec!["a", "b"]);
    }

    #[test]
    fn the_entry_just_written_is_never_evicted() {
        // One huge snapshot on its own is kept: it may be the user's only copy of unsaved work,
        // and the folder is never emptied by housekeeping.
        let entries = vec![entry("huge", 1_000, 1)];
        assert!(to_evict(&entries, limits(10, 100), None).is_empty());
        assert!(to_evict(&entries, limits(0, 0), None).is_empty());
        // Nor is it evicted to make room for others, even though it is the oldest.
        let entries = vec![entry("huge", 1_000, 1), entry("small", 10, 2)];
        assert_eq!(to_evict(&entries, limits(10, 100), Some("huge")), vec!["small"]);
        // Everything else can go before it does, and then it stays.
        let entries = vec![entry("huge", 1_000, 1), entry("a", 10, 2), entry("b", 10, 3)];
        assert_eq!(to_evict(&entries, limits(10, 100), Some("huge")), vec!["a", "b"]);
    }

    #[test]
    fn the_same_folder_always_gives_the_same_decision() {
        // Equal timestamps are broken by key, so the decision does not depend on listing order.
        let entries = vec![entry("b", 10, 5), entry("a", 10, 5), entry("c", 10, 5)];
        assert_eq!(to_evict(&entries, limits(1, 100), None), vec!["a", "b"]);
        let mut shuffled = entries.clone();
        shuffled.reverse();
        assert_eq!(to_evict(&shuffled, limits(1, 100), None), vec!["a", "b"]);
    }

    #[test]
    fn sizes_that_would_overflow_do_not_wrap() {
        let entries = vec![entry("a", u64::MAX, 1), entry("b", u64::MAX, 2)];
        // The sum saturates, so the folder counts as over budget instead of looking empty; the
        // newest snapshot is still the one that stays.
        assert_eq!(to_evict(&entries, limits(10, 100), None), vec!["a"]);
    }

    #[test]
    fn the_folder_is_never_emptied() {
        let entries = vec![entry("a", 500, 1), entry("b", 500, 2), entry("c", 500, 3)];
        // Even a zero budget leaves the newest snapshot behind, and never more than that is cut.
        assert_eq!(to_evict(&entries, limits(0, 0), None), vec!["a", "b"]);
        assert_eq!(to_evict(&entries, limits(0, 0), Some("c")), vec!["a", "b"]);
    }
}
