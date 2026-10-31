//! A folder becomes a list of files once (REC-B04).
//!
//! Two automation tools take a list of paths and write a result for each one: `action_run` and
//! `ocr_recognize_files`. Until this module existed, every path in that list had to be a file, and
//! the same file could appear twice in it — reached through two spellings, two overlapping folders,
//! or a link — and be processed twice by a tool that overwrites what it writes. Whole folders, with
//! the subfolders a person means by "this folder", could not be given at all.
//!
//! So there is one expansion, and everything that walks a batch uses it: the batch tools, the plan
//! that says what a batch would do, and `batch_expand`, which reports the same list without
//! writing anything. The rules are the same three times over, because they are the same code:
//!
//! * **A path may be a file or a folder.** A folder is walked, `depth` levels deep (default
//!   [`MAX_DEPTH`]; 1 takes its own files and no subfolder's).
//! * **Exclusions are by name or by relative path**, with `*` for any run of characters and `?` for
//!   one. `.`-prefixed names are left out unless `include_hidden` asks for them, and the output
//!   folder is left out when there is one — a run must not pick up what an earlier run wrote.
//! * **A link to a folder is never followed**, so a link cannot make a loop; a link to a file is
//!   taken unless `follow_links` says not to. A folder already walked is not walked again, which is
//!   what a junction on Windows would otherwise do.
//! * **A file appears once.** Two spellings of the same path, two folders that overlap, a link to a
//!   file already in the list: one entry, and the second is reported as skipped with the file it
//!   duplicates. That is the promise REC-B04 makes ("duplicate events cannot cause duplicate
//!   destructive processing"), kept at the only place it can be: before the destructive tool sees
//!   the list.
//! * **Order is the caller's.** Inputs are taken in the order given; inside a folder, entries are
//!   sorted by name, so the same tree always produces the same list.
//!
//! Everything still goes through [`Automation::resolve`]: a path outside the root refuses the whole
//! call (as `ocr_recognize_files` already did), a file inside it that is missing is a per-file
//! error, and a link that points outside the root is refused rather than followed.
//!
//! What this is not: a watcher. Nothing here runs in the background or reacts to a file changing on
//! disk; a job is a list, made once, when it starts. A watcher needs a thread and a queue in
//! whatever process is watching, which is the app's job (it has one) rather than this crate's.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use crate::limits::{MAX_DEPTH, MAX_EXCLUDES, MAX_FILES};
use crate::{Args, Automation, Result, ToolError};

/// One entry of a batch, in the order the caller gave it.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Item {
    /// A file the run will process, named the way the caller named it (plus any folder it was
    /// found in).
    File(String),
    /// A path that could not be resolved — missing, outside the root, unreadable. The run reports
    /// it as a per-file error, exactly as it did when the list was taken verbatim.
    Broken(String, String),
}

/// What a list of paths expands to, and what was left out on the way.
pub(crate) struct Expanded {
    pub items: Vec<Item>,
    /// One object per path that was left out on purpose: `{ path, why }`.
    pub skipped: Vec<Value>,
    /// How many folders were walked (the ones the caller named, and their subfolders).
    pub folders: usize,
}

impl Expanded {
    /// The files a run will process, in order.
    pub fn files(&self) -> Vec<&str> {
        self.items
            .iter()
            .filter_map(|i| match i {
                Item::File(p) => Some(p.as_str()),
                Item::Broken(..) => None,
            })
            .collect()
    }

    /// Every entry, files and broken paths together: what the batch counts against its limit.
    pub fn count(&self) -> usize {
        self.items.len()
    }

    /// The paths to pass to the run that would carry this list out: each file under the name it was
    /// found by, and each broken path, in order — so the run reports the same list, and the same
    /// per-file errors, as the plan described.
    pub fn approve_paths(&self) -> Vec<String> {
        self.items
            .iter()
            .map(|i| match i {
                Item::File(p) | Item::Broken(p, _) => p.clone(),
            })
            .collect()
    }

    fn skip(&mut self, path: &str, why: impl Into<String>) {
        self.skipped.push(json!({ "path": path, "why": why.into() }));
    }

    /// Everything that will not be processed, broken paths included, in the order it was met.
    fn not_processed(&self) -> Vec<Value> {
        let mut out: Vec<Value> = self
            .items
            .iter()
            .filter_map(|i| match i {
                Item::Broken(p, why) => Some(json!({ "path": p, "why": why })),
                Item::File(_) => None,
            })
            .collect();
        out.extend(self.skipped.iter().cloned());
        out
    }
}

/// How a list is expanded: the arguments the batch tools share with `batch_expand`.
pub(crate) struct Wanted {
    pub exclude: Vec<String>,
    pub include_hidden: bool,
    pub depth: usize,
    pub follow_links: bool,
    /// The folder a run writes into, left out of the walk when the caller has one.
    pub output: Option<PathBuf>,
}

impl Wanted {
    /// Read the shared arguments. The path list itself is not here: the caller reads that, because
    /// only it knows whether the list is required.
    pub(crate) fn from_args(a: &Args) -> Result<Self> {
        let exclude = match a.get("exclude") {
            None => Vec::new(),
            Some(v) => {
                let arr = v.as_array().ok_or_else(|| ToolError::InvalidArgs("exclude must be an array of patterns".into()))?;
                arr.iter()
                    .map(|p| p.as_str().map(str::to_owned).ok_or_else(|| ToolError::InvalidArgs("exclude must be an array of patterns".into())))
                    .collect::<Result<Vec<_>>>()?
            }
        };
        // An empty pattern matches nothing at all, so it could only be a mistake: say so here
        // rather than let a caller believe a file was excluded.
        if exclude.iter().any(|p| p.is_empty()) {
            return Err(ToolError::InvalidArgs("exclude must not hold an empty pattern".into()));
        }
        if exclude.len() > MAX_EXCLUDES {
            return Err(ToolError::InvalidArgs(format!("exclude must hold at most {MAX_EXCLUDES} patterns")));
        }
        let depth = match a.opt_int("depth")? {
            None => MAX_DEPTH,
            Some(d) if d >= 1 => usize::try_from(d).unwrap_or(MAX_DEPTH).min(MAX_DEPTH),
            Some(_) => return Err(ToolError::InvalidArgs("depth must be at least 1 (1 takes the folder's own files)".into())),
        };
        Ok(Self {
            exclude,
            include_hidden: a.opt_bool("include_hidden")?.unwrap_or(false),
            depth,
            follow_links: a.opt_bool("follow_links")?.unwrap_or(true),
            output: None,
        })
    }
}

impl Automation {
    /// `batch_expand`: the files a batch would run over, and the ones it would leave out.
    pub(crate) fn batch_expand(&mut self, a: &Args) -> Result<Value> {
        let paths = a.strs("paths")?;
        if paths.is_empty() {
            return Err(ToolError::InvalidArgs("paths must not be empty: there is nothing to expand".into()));
        }
        let mut wanted = Wanted::from_args(a)?;
        // A folder to leave out, when the caller is asking about a run that writes into one.
        if let Some(folder) = a.opt_str("folder")? {
            wanted.output = Some(self.resolve(folder, true)?);
        }
        let expanded = self.expand(&paths, &wanted)?;
        let files: Vec<Value> = expanded.files().iter().map(|p| json!(p)).collect();
        Ok(json!({
            "count": files.len(),
            "files": files,
            "folders": expanded.folders,
            "skipped": expanded.not_processed(),
        }))
    }

    /// Expand `paths` into the files a batch will process, in order, each at most once.
    ///
    /// `paths` must already be known to be non-empty: an empty batch is the caller's error to
    /// report, in its own words.
    pub(crate) fn expand(&self, paths: &[&str], wanted: &Wanted) -> Result<Expanded> {
        let mut out = Expanded { items: Vec::new(), skipped: Vec::new(), folders: 0 };
        // The files already in the list, by canonical path, with the spelling that put them there.
        let mut seen: HashMap<PathBuf, String> = HashMap::new();
        // The folders already walked, so a junction or a link cannot make a loop.
        let mut walked: HashSet<PathBuf> = HashSet::new();
        for &given in paths {
            // A path that leaves the root refuses the whole call, as it does for the tools: a list
            // built from it would describe work that cannot be done.
            self.resolve(given, true)?;
            let start = match self.resolve(given, false) {
                Ok(p) => p,
                Err(e) => {
                    out.items.push(Item::Broken(given.to_string(), e.to_string()));
                    continue;
                }
            };
            match std::fs::metadata(&start) {
                Ok(m) if m.is_dir() => {
                    out.folders += 1;
                    walked.insert(start.clone());
                    self.walk(&start, given, "", 0, wanted, &mut out, &mut seen, &mut walked)?;
                }
                Ok(_) => {
                    // A single file, given by name: the rules for it are the rules for a file
                    // found in a folder, so a duplicate is caught the same way.
                    let name = Path::new(given).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
                    self.consider(given, &name, given, &start, wanted, &mut out, &mut seen)?;
                }
                Err(e) => out.items.push(Item::Broken(given.to_string(), format!("{given}: {e}"))),
            }
        }
        Ok(out)
    }

    /// Walk one folder. `level` is how far below the folder the caller named it is, and `rel` is
    /// the path below that folder, for the patterns and for naming what is found.
    ///
    /// The state the walk carries — what it has found, what it has skipped, what it has already
    /// seen — is threaded through by hand rather than gathered into a struct: the walk is one
    /// function, and a struct would only move the same arguments into a field.
    #[allow(clippy::too_many_arguments)]
    fn walk(
        &self,
        dir: &Path,
        given: &str,
        rel: &str,
        level: usize,
        wanted: &Wanted,
        out: &mut Expanded,
        seen: &mut HashMap<PathBuf, String>,
        walked: &mut HashSet<PathBuf>,
    ) -> Result<()> {
        let listing = match std::fs::read_dir(dir) {
            Ok(l) => l,
            Err(e) => {
                out.skip(&emit(given, rel), format!("it could not be listed: {e}"));
                return Ok(());
            }
        };
        let mut entries: Vec<(String, PathBuf, std::fs::FileType)> = Vec::new();
        for entry in listing {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    out.skip(&emit(given, rel), format!("an entry could not be read: {e}"));
                    continue;
                }
            };
            let name = entry.file_name().to_string_lossy().into_owned();
            match entry.file_type() {
                Ok(kind) => entries.push((name, entry.path(), kind)),
                Err(e) => out.skip(&emit(given, &join(rel, &name)), format!("its type could not be read: {e}")),
            }
        }
        // Sorted by name, so the same tree always gives the same list.
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, path, kind) in entries {
            let here = join(rel, &name);
            if !wanted.include_hidden && name.starts_with('.') {
                out.skip(&emit(given, &here), "a hidden name: it starts with a dot (pass include_hidden: true to take it)");
                continue;
            }
            if kind.is_symlink() {
                // A link is looked at, never walked into: that is what makes a loop impossible.
                match std::fs::metadata(&path) {
                    Ok(m) if m.is_dir() => {
                        out.skip(&emit(given, &here), "a link to a folder: links to folders are not followed, so a link cannot make a loop");
                        continue;
                    }
                    Ok(_) if !wanted.follow_links => {
                        out.skip(&emit(given, &here), "a link to a file, and follow_links is false");
                        continue;
                    }
                    Ok(_) => {
                        self.consider(&emit(given, &here), &name, &here, &path, wanted, out, seen)?;
                        continue;
                    }
                    Err(e) => {
                        out.skip(&emit(given, &here), format!("a link that leads nowhere: {e}"));
                        continue;
                    }
                }
            }
            if kind.is_dir() {
                // `depth: 1` takes the folder's own files: its subfolders are already one level
                // deeper than that.
                if level + 1 >= wanted.depth {
                    out.skip(&emit(given, &here), format!("a folder deeper than {} level(s) below {given}", wanted.depth));
                    continue;
                }
                // A junction on Windows is a folder, not a link: walking one could leave the tree
                // or come back round, so a folder outside the root, or one already walked, is not
                // walked again.
                let real = match path.canonicalize() {
                    Ok(p) => p,
                    Err(e) => {
                        out.skip(&emit(given, &here), format!("it could not be resolved: {e}"));
                        continue;
                    }
                };
                if let Some(root) = &self.root {
                    if !real.starts_with(root) {
                        let shown = emit(given, &here);
                        out.skip(&shown, format!("{shown} is outside the allowed directory {}", root.display()));
                        continue;
                    }
                }
                if !walked.insert(real) {
                    out.skip(&emit(given, &here), "a folder already walked in this batch (a link or junction to it)");
                    continue;
                }
                out.folders += 1;
                self.walk(&path, given, &here, level + 1, wanted, out, seen, walked)?;
                continue;
            }
            if !kind.is_file() {
                out.skip(&emit(given, &here), "not a regular file");
                continue;
            }
            self.consider(&emit(given, &here), &name, &here, &path, wanted, out, seen)?;
        }
        Ok(())
    }

    /// Decide what one file is: excluded, inside the output folder, already in the list, or a file
    /// to process. Takes the same threaded state as `walk` for the same reason.
    #[allow(clippy::too_many_arguments)]
    fn consider(
        &self,
        shown: &str,
        name: &str,
        rel: &str,
        path: &Path,
        wanted: &Wanted,
        out: &mut Expanded,
        seen: &mut HashMap<PathBuf, String>,
    ) -> Result<()> {
        for pattern in &wanted.exclude {
            if wildcard(pattern, name) || wildcard(pattern, rel) {
                out.skip(shown, format!("excluded by the pattern {pattern:?}"));
                return Ok(());
            }
        }
        // The file as the tools would see it: this is also the check that a link does not lead out
        // of the root, and it is the run's own rule rather than a second copy of it.
        let resolved = match self.resolve(shown, false) {
            Ok(p) => p,
            Err(e) => {
                out.items.push(Item::Broken(shown.to_string(), e.to_string()));
                return Ok(());
            }
        };
        // One key per file, whatever spelling found it: the resolved path, made absolute where the
        // session is not confined so two spellings of one file still meet.
        let key = resolved.canonicalize().or_else(|_| path.canonicalize()).unwrap_or(resolved);
        if let Some(output) = &wanted.output {
            if key.starts_with(output) {
                out.skip(shown, format!("inside the output folder {}, so a run would process what it wrote", output.display()));
                return Ok(());
            }
        }
        if let Some(first) = seen.get(&key) {
            out.skip(shown, format!("already in this batch: it is the same file as {first}"));
            return Ok(());
        }
        seen.insert(key, shown.to_string());
        out.items.push(Item::File(shown.to_string()));
        if out.items.len() > MAX_FILES {
            return Err(ToolError::Refused(format!(
                "refused: this folder holds more than {MAX_FILES} files; narrow it with exclude or depth, or give the files by name"
            )));
        }
        Ok(())
    }
}

/// One path below a folder, with `/` between the parts whatever the platform uses, so a pattern and
/// a plan read the same on every system.
fn join(rel: &str, name: &str) -> String {
    if rel.is_empty() { name.to_string() } else { format!("{rel}/{name}") }
}

/// A path as the caller should see it: the folder it named, then the parts below it.
fn emit(given: &str, rel: &str) -> String {
    if rel.is_empty() {
        return given.to_string();
    }
    let base = given.trim_end_matches(['/', '\\']);
    if base.is_empty() || base == "." { rel.to_string() } else { format!("{base}/{rel}") }
}

/// Does `text` match `pattern`, with `*` for any run of characters (including none) and `?` for one?
/// Case is ignored where the filesystem ignores it, so `*.PDF` finds `a.pdf` on Windows.
fn wildcard(pattern: &str, text: &str) -> bool {
    let fold = |s: &str| -> Vec<char> { if cfg!(windows) { s.to_lowercase().chars().collect() } else { s.chars().collect() } };
    let (p, t) = (fold(pattern), fold(text));
    let (mut pi, mut ti) = (0usize, 0usize);
    // The last `*` seen, and how much of the text it is currently standing in for.
    let mut star: Option<usize> = None;
    let mut resume = 0usize;
    while ti < t.len() {
        if pi < p.len() && (p[pi] == '?' || p[pi] == t[ti]) {
            pi += 1;
            ti += 1;
        } else if pi < p.len() && p[pi] == '*' {
            star = Some(pi);
            resume = ti;
            pi += 1;
        } else if let Some(s) = star {
            // Let the star swallow one more character and try again.
            pi = s + 1;
            resume += 1;
            ti = resume;
        } else {
            return false;
        }
    }
    while pi < p.len() && p[pi] == '*' {
        pi += 1;
    }
    pi == p.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_pattern_matches_names_and_paths() {
        // The shapes a person writes for a batch: an extension, a stem, a folder.
        assert!(wildcard("*.pdf", "report.pdf"));
        assert_eq!(wildcard("*.pdf", "report.PDF"), cfg!(windows), "case follows the filesystem");
        assert!(wildcard("draft*", "draft-2.pdf"));
        assert!(wildcard("*draft*", "a-draft-b.pdf"));
        assert!(wildcard("report?.pdf", "report1.pdf"));
        assert!(!wildcard("report?.pdf", "report12.pdf"));
        assert!(!wildcard("*.pdf", "report.png"));
        assert!(wildcard("**", "anything at all"));
        assert!(wildcard("*", ""));
        // A path below the folder, which is what `rel` is for: everything under one subfolder.
        assert!(wildcard("scans/*", "scans/a.pdf"));
        assert!(wildcard("scans/*", "scans/nested/a.pdf"), "a star spans the separator, deliberately");
        // A pattern with no wildcard is a name, matched exactly.
        assert!(wildcard("a.pdf", "a.pdf"));
        assert!(!wildcard("a.pdf", "b.pdf"));
    }

    #[test]
    fn a_path_below_a_folder_reads_the_same_on_every_platform() {
        assert_eq!(join("", "a.pdf"), "a.pdf");
        assert_eq!(join("sub", "a.pdf"), "sub/a.pdf");
        assert_eq!(emit("in", ""), "in");
        assert_eq!(emit("in", "sub/a.pdf"), "in/sub/a.pdf");
        assert_eq!(emit("in/", "sub/a.pdf"), "in/sub/a.pdf");
        assert_eq!(emit(".", "a.pdf"), "a.pdf");
    }
}
