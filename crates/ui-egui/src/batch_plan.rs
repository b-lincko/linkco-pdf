//! A batch of files, said before it runs (REC-B03).
//!
//! Two things in the interface take a list of files and a folder and write a result for each one:
//! Action Wizard ▸ Start, and Scan & OCR ▸ Recognize text in multiple files. Both used to begin the
//! moment the folder was chosen, so the first time a reader learned which files would be written —
//! and which of them would replace something that was already there — was after it had happened.
//!
//! This module is the half of REC-B03 the automation crate could not give the interface: the plan,
//! and the question. [`plan`] works out every destination without writing anything, using the same
//! naming rule the writer then uses — the writers take their destinations from the plan they were
//! approved with, so the answer cannot describe a batch other than the one that runs. [`body`] is
//! the dialog that shows it, and [`crate::PdfCraftApp::run_pending_batch`] runs it only if the
//! batch is still the one that was shown.
//!
//! What the two writers do differently, and why the plan asks which one it is:
//!
//! * an action run never replaces a file — a result whose name is taken is written beside it as
//!   `<stem> (2).pdf`, and that is what the plan says it will do;
//! * a recognition run writes each result under the input's own name, so a name that is taken is
//!   replaced, and a replaced file cannot be brought back.
//!
//! On the web build each result is a browser download rather than a file in a folder: nothing can
//! be replaced there, so there is nothing to approve, and that path is unchanged.

use std::path::{Path, PathBuf};

use pdfcraft_engine::actions::Action;
use pdfcraft_engine::ocr::OcrSettings;

/// Which of the two writers a plan is for. They name their output differently, so a plan and the
/// run it approves have to ask the same one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// Action Wizard: `<stem> (n).pdf` when the name is taken; never replaces.
    Action,
    /// Recognize text in multiple files: under the input's own name; replaces what is there.
    Recognize,
}

/// One file of a batch: what goes in, where its result goes, and whether that replaces a file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Target {
    pub input: String,
    pub output: PathBuf,
    /// A file is already at `output` and the run would replace it.
    pub replaces: bool,
}

/// Everything a batch would write, worked out without touching the disk.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plan {
    pub folder: PathBuf,
    pub kind: Kind,
    pub targets: Vec<Target>,
}

impl Plan {
    /// How many files would be written.
    pub fn total(&self) -> usize {
        self.targets.len()
    }

    /// How many of them replace a file that is already there.
    pub fn replacing(&self) -> usize {
        self.targets.iter().filter(|t| t.replaces).count()
    }

    /// How many are written under a name other than the input's own, because the input's name is
    /// taken and this writer steps past it rather than replacing it.
    pub fn renamed(&self) -> usize {
        self.targets.iter().filter(|t| !t.replaces && file_name(&t.output) != file_name_of(&t.input)).count()
    }
}

/// A batch waiting for its reader to approve it: the plan they were shown, and the work it stands
/// for. The files are already in memory (they were read to be planned), so holding them here while
/// the question is on screen costs nothing that was not already spent.
#[derive(Clone, Debug)]
pub struct Pending {
    pub plan: Plan,
    pub work: Work,
}

/// One file of a batch being written: where its result goes — the destination the approved plan
/// named, or nothing on the web build, where the result is a download — and the file with the name
/// it was given.
pub(crate) type RunItem = (Option<PathBuf>, (String, Vec<u8>));

/// What to run once the plan is approved.
#[derive(Clone, Debug)]
pub enum Work {
    Action { action: Action, files: Vec<(String, Vec<u8>)> },
    Recognize { files: Vec<(String, Vec<u8>)>, settings: OcrSettings },
}

impl Pending {
    /// The batch as it is now — the same computation as the one that was shown, so the two can be
    /// compared. A difference means something changed under the question: a file appeared at a
    /// name the plan called free, or one that was there is gone.
    pub fn replanned(&self) -> Plan {
        plan(&self.plan.folder, self.plan.kind, &self.inputs())
    }

    /// The inputs, named as the plan names them.
    pub fn inputs(&self) -> Vec<String> {
        match &self.work {
            Work::Action { files, .. } | Work::Recognize { files, .. } => files.iter().map(|(n, _)| n.clone()).collect(),
        }
    }

    /// The dialog's heading: what this batch is, in the reader's language.
    pub fn title(&self) -> String {
        let n = self.plan.total().to_string();
        match &self.work {
            Work::Action { action, .. } => crate::i18n::fmt(tl!("Run {name} on {n} files?"), &[("name", &action.name), ("n", &n)]),
            Work::Recognize { .. } => crate::i18n::fmt(tl!("Recognize text in {n} files?"), &[("n", &n)]),
        }
    }
}

/// The name a batch writes an input under: its own file name, or `document.pdf` for a path with
/// none. The writers and the plan both take it from here.
fn file_name_of(path: &str) -> String {
    Path::new(path).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| "document.pdf".into())
}

/// The name of a planned output, for comparing it with the input's.
fn file_name(path: &Path) -> String {
    path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default()
}

/// `dir` and the input's own name.
fn child(dir: &Path, name: &str) -> PathBuf {
    dir.join(file_name_of(name))
}

/// `<stem> (n).<ext>` — or `<stem> (n)` for a name with no extension — for the first `n` whose
/// name is neither on disk nor already promised by an earlier item of this batch.
fn step_aside(dir: &Path, name: &str, taken: &[PathBuf]) -> PathBuf {
    let first = child(dir, name);
    if !first.exists() && !taken.contains(&first) {
        return first;
    }
    let source = Path::new(name);
    let file = file_name_of(name);
    let stem = Path::new(&file).file_stem().map(|s| s.to_string_lossy().into_owned()).unwrap_or_else(|| "document".into());
    let extension = source.extension().map(|e| e.to_string_lossy().into_owned());
    (2..10_000u32)
        .map(|n| match &extension {
            Some(ext) => dir.join(format!("{stem} ({n}).{ext}")),
            None => dir.join(format!("{stem} ({n})")),
        })
        .find(|p| !p.exists() && !taken.contains(p))
        .unwrap_or(first)
}

/// What a batch of these inputs would write into `folder`, in the order it would write them.
///
/// Naming is the writers' own rule, not a copy of it: an action run steps past a taken name (on
/// disk, or promised earlier in this batch), and a recognition run takes the input's own name and
/// says whether a file is already there. Nothing is created, opened for writing or removed here.
pub fn plan(folder: &Path, kind: Kind, names: &[String]) -> Plan {
    let mut promised: Vec<PathBuf> = Vec::new();
    let mut targets = Vec::with_capacity(names.len());
    for name in names {
        let (output, replaces) = match kind {
            Kind::Action => (step_aside(folder, name, &promised), false),
            Kind::Recognize => {
                let output = child(folder, name);
                let replaces = std::fs::metadata(&output).is_ok();
                (output, replaces)
            }
        };
        promised.push(output.clone());
        targets.push(Target { input: name.clone(), output, replaces });
    }
    Plan { folder: folder.to_path_buf(), kind, targets }
}

impl crate::PdfCraftApp {
    /// Ask the reader before a batch writes anything: show `pending`, replacing any question that
    /// is already on screen (a second batch cannot be approved through the first one's dialog).
    pub(crate) fn ask_batch(&mut self, pending: Pending) {
        self.pending_batch = Some(pending);
        self.dialog = Some(crate::Dialog::BatchApprove);
    }

    /// Run the batch that is waiting — but only if the batch it would write is still the one that
    /// was shown. This is the token from the automation side (`approval`), with a person instead
    /// of a digest: the plan is worked out again at the moment of the answer, and a file that has
    /// appeared at a name the plan called free (or gone from one it said would be replaced) makes
    /// this a different batch. Then nothing runs and the question stays open with the plan as it
    /// is now, so the reader can see what changed rather than be told it changed.
    ///
    /// A dismissed dialog drops its batch in [`crate::dialogs`]: nothing runs.
    pub fn run_pending_batch(&mut self) {
        let Some(pending) = self.pending_batch.take() else { return };
        let now = pending.replanned();
        if now != pending.plan {
            self.notify_tr("The folder changed since this was shown; check what this batch would write now");
            self.pending_batch = Some(Pending { plan: now, work: pending.work });
            return;
        }
        match pending.work {
            Work::Action { action, files } => self.start_action(action, files, Some(now)),
            Work::Recognize { files, settings } => self.start_recognize_files(files, settings, Some(now)),
        }
    }
}

/// The dialog: what this batch would write, and the two answers.
///
/// `(run, cancel)` — the caller runs the batch or drops it. The dialog is only the question; the
/// decision about whether the answer still matches the batch is in
/// [`crate::PdfCraftApp::run_pending_batch`].
pub(crate) fn body(ui: &mut egui::Ui, app: &crate::PdfCraftApp) -> (bool, bool) {
    use crate::theme;
    use crate::widgets;
    let (mut run, mut cancel) = (false, false);
    let Some(pending) = app.pending_batch.as_ref() else { return (false, true) };
    let plan = &pending.plan;
    ui.label(egui::RichText::new(pending.title()).font(theme::semibold(18.0)));
    ui.add_space(8.0);
    let folder = plan.folder.display().to_string();
    ui.label(crate::i18n::fmt(tl!("Results go to {folder}"), &[("folder", &folder)]));
    ui.add_space(6.0);
    // What is written, and the part that cannot be taken back.
    ui.label(crate::i18n::fmt(tl!("Writes {n} file(s)."), &[("n", &plan.total().to_string())]));
    match plan.kind {
        Kind::Recognize if plan.replacing() > 0 => {
            ui.label(
                egui::RichText::new(crate::i18n::fmt(
                    tl!("Replaces {n} file(s) that are already there. A replaced file cannot be brought back."),
                    &[("n", &plan.replacing().to_string())],
                ))
                .color(theme::Tokens::get(ui.ctx()).text_muted),
            );
        }
        Kind::Action if plan.renamed() > 0 => {
            ui.label(crate::i18n::fmt(
                tl!("{n} file(s) are written beside one that is already there, which is left as it is."),
                &[("n", &plan.renamed().to_string())],
            ));
        }
        _ => {
            ui.label(egui::RichText::new(tl!("Nothing that is already in the folder is replaced.")).color(theme::Tokens::get(ui.ctx()).text_muted));
        }
    }
    ui.add_space(8.0);
    // The names themselves, so the answer is checkable rather than trusted: the first few, and a
    // count for the rest, because a hundred lines is not a dialog.
    let shown = 6.min(plan.targets.len());
    for target in plan.targets.iter().take(shown) {
        let name = file_name(&target.output);
        ui.label(egui::RichText::new(format!("• {name}")).color(theme::Tokens::get(ui.ctx()).text_muted));
    }
    if plan.targets.len() > shown {
        ui.label(
            egui::RichText::new(crate::i18n::fmt(tl!("and {n} more"), &[("n", &(plan.targets.len() - shown).to_string())]))
                .color(theme::Tokens::get(ui.ctx()).text_faint),
        );
    }
    ui.add_space(12.0);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        if widgets::pill_button(ui, tl!("Run"), true).clicked() {
            run = true;
        }
        if widgets::pill_button(ui, tl!("Cancel"), false).clicked() {
            cancel = true;
        }
    });
    (run, cancel)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("pdfcraft-batch-plan-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn an_action_run_steps_past_a_name_that_is_taken_and_never_replaces_one() {
        let dir = dir("step-aside");
        std::fs::write(dir.join("report.pdf"), b"already here").unwrap();
        let plan = plan(&dir, Kind::Action, &["report.pdf".into(), "report.pdf".into(), "other.pdf".into()]);
        assert_eq!(plan.total(), 3);
        assert_eq!(plan.replacing(), 0, "an action run replaces nothing");
        assert_eq!(file_name(&plan.targets[0].output), "report (2).pdf");
        assert_eq!(file_name(&plan.targets[1].output), "report (3).pdf", "an earlier item of the batch counts as taken");
        assert_eq!(file_name(&plan.targets[2].output), "other.pdf", "a free name is used as it is");
        assert_eq!(plan.renamed(), 2);
        assert_eq!(std::fs::read(dir.join("report.pdf")).unwrap(), b"already here", "planning wrote nothing");
    }

    #[test]
    fn a_recognition_run_names_the_file_already_there_as_replaced() {
        let dir = dir("replaces");
        std::fs::write(dir.join("one.pdf"), b"old").unwrap();
        let plan = plan(&dir, Kind::Recognize, &["one.pdf".into(), "two.pdf".into()]);
        assert_eq!(plan.replacing(), 1);
        assert!(plan.targets[0].replaces);
        assert_eq!(file_name(&plan.targets[0].output), "one.pdf", "recognition keeps the input's own name");
        assert!(!plan.targets[1].replaces);
        assert_eq!(plan.renamed(), 0);
        assert_eq!(std::fs::read(dir.join("one.pdf")).unwrap(), b"old");
    }

    #[test]
    fn a_name_with_no_extension_or_no_file_name_still_gets_somewhere_to_go() {
        let dir = dir("odd-names");
        std::fs::write(dir.join("notes"), b"taken").unwrap();
        let stepped = plan(&dir, Kind::Action, &["notes".into(), "".into()]);
        assert_eq!(file_name(&stepped.targets[0].output), "notes (2)", "no extension: the counter is the whole difference");
        assert_eq!(file_name(&stepped.targets[1].output), "document.pdf", "an empty name still names a file");
        // A path with separators is reduced to its file name, so nothing is written outside the folder.
        let recognize = plan(&dir, Kind::Recognize, &["../escape.pdf".into()]);
        assert_eq!(recognize.targets[0].output, dir.join("escape.pdf"));
    }
}
