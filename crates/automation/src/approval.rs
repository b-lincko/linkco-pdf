//! An approval names the batch it approves (REC-B03).
//!
//! `batch_plan` says what a destructive batch would do and writes nothing; this module makes the
//! next step mean something. A plan carries a token — a digest of the list it was made from, the
//! folder it would write into and the settings it would use — and a writer that is given that token
//! refuses to run unless the batch it is about to carry out digests to the same value.
//!
//! That closes the gap between "I looked at the plan" and "the run did what I looked at". Without
//! it, approving is a promise about the moment the plan was made: a file could be added to the
//! folder, an earlier output could be deleted, the folder argument could be mistyped, and the run
//! would happily do something the person never saw. With it, the three cases that matter are
//! refused before a byte is written:
//!
//! * a different folder (the token is bound to the folder's path),
//! * a different batch (a file added to a walked folder, a file removed, a different list of paths,
//!   a different setting such as the action's steps or the OCR language),
//! * a file that is now in the way — an output that exists now and did not when the plan was made,
//!   which turns a write into a replacement.
//!
//! The token is a digest of the *list*, not of the arguments that made it, which is what lets the
//! two be asked for separately: the call `batch_plan` hands back names the files it planned (so it
//! is exact, and a file that arrives in the folder afterwards is not in the approved batch), while
//! a call that names the folder again asks for the batch as it is now — and a folder that grew is
//! then refused, because the batch it would carry out is not the one the token names.
//!
//! What the token is not: a secret, and not a permission. It is not signed and anyone holding this
//! crate could compute one; it cannot grant a call the session's permission profile refuses, and it
//! does not decide *whether* a batch may run — the profile and the `--approve` list do that. Its
//! only job is to say **is this the batch that was approved**, which is a question about drift, not
//! about trust. Nor is it a human: the interface still has to show a plan and ask a person before
//! it runs one, and this token is what such a prompt would hand to the run afterwards.

use std::path::{Path, PathBuf};

use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::folder::{Expanded, Item};
use crate::{Result, ToolError, action_output, batch_input_name, child};

/// The length of a token: a SHA-256 digest in lowercase hexadecimal.
pub(crate) const TOKEN_LEN: usize = 64;

/// Which writer a plan is for. The two pick output names differently — an action run steps past a
/// name that is taken, a text-recognition run writes under the input's own name — so the plan and
/// the run have to ask the same one.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Kind {
    Action,
    Ocr,
}

impl Kind {
    /// The tool a plan of this kind is carried out by: the name the token is bound to.
    fn tool(self) -> &'static str {
        match self {
            Self::Action => "action_run",
            Self::Ocr => "ocr_recognize_files",
        }
    }
}

/// One entry of the list a run would carry out, worked out without writing anything.
pub(crate) struct Planned {
    /// The input, named as the caller named it.
    pub path: String,
    /// Where its result would go.
    pub output: PathBuf,
    /// Whether a file is already there, so the run would replace it.
    pub replaces: bool,
    /// Why the input cannot be read, for an entry that is in the list but would fail.
    pub error: Option<String>,
}

/// The outputs a batch would write, in order, computed without touching the disk.
///
/// This is the one place the naming rules run for a whole batch, so the plan, the token and the
/// writers cannot disagree about a single name: `action_output` steps past a name that is taken on
/// disk or already promised in this batch, and `child` is the plain name a recognition run writes.
pub(crate) fn planned(expanded: &Expanded, folder: &Path, kind: Kind) -> Vec<Planned> {
    let mut promised: Vec<PathBuf> = Vec::new();
    let mut out = Vec::new();
    for item in &expanded.items {
        let (path, error) = match item {
            Item::File(p) => (p.clone(), None),
            Item::Broken(p, why) => (p.clone(), Some(why.clone())),
        };
        let name = batch_input_name(&path);
        let output = match kind {
            Kind::Action => action_output(folder, &name, &promised),
            Kind::Ocr => child(folder, &name),
        };
        promised.push(output.clone());
        let replaces = std::fs::metadata(&output).is_ok();
        out.push(Planned { path, output, replaces, error });
    }
    out
}

/// The token for one batch: its writer, the folder, the settings that change what is written, and
/// every input with the file its result would go to.
///
/// `settings` is the caller's own words for what the batch does — an action's steps, a recognition
/// run's language and resolution — so an approval of one action cannot carry out another.
pub(crate) fn token(kind: Kind, folder: &Path, settings: &[String], targets: &[Planned]) -> String {
    let mut h = Sha256::new();
    // Each field is length-prefixed, so two different batches cannot be made to digest the same by
    // moving a separator into a file name.
    let mut field = |bytes: &[u8]| {
        h.update((bytes.len() as u64).to_le_bytes());
        h.update(bytes);
    };
    field(kind.tool().as_bytes());
    field(folder.to_string_lossy().as_bytes());
    field(&(settings.len() as u64).to_le_bytes());
    for s in settings {
        field(s.as_bytes());
    }
    for t in targets {
        field(t.path.as_bytes());
        field(t.output.to_string_lossy().as_bytes());
        field(if t.replaces { b"replaces" } else { b"new" });
        field(t.error.as_deref().unwrap_or_default().as_bytes());
    }
    let digest = h.finalize();
    let mut out = String::with_capacity(TOKEN_LEN);
    for b in digest.iter() {
        out.push_str(&format!("{b:02x}"));
    }
    out
}

/// Whether `given` is shaped like a token: lowercase hexadecimal of exactly [`TOKEN_LEN`].
fn well_formed(given: &str) -> bool {
    given.len() == TOKEN_LEN && given.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// Check a caller's approval against the batch that is about to run.
///
/// `None` is not an approval and not a refusal: the caller did not ask for this check, and the
/// call behaves as it always did. A token that does not name this batch is `Refused` — nothing has
/// been read or written at that point — with a message that says what this call would do, so the
/// caller can compare it with the plan they approved and plan again.
pub(crate) fn check(given: Option<&str>, kind: Kind, folder: &Path, settings: &[String], targets: &[Planned]) -> Result<bool> {
    let Some(given) = given else { return Ok(false) };
    if !well_formed(given) {
        return Err(ToolError::InvalidArgs(format!(
            "approve_token must be the {TOKEN_LEN}-character token a batch_plan returned, not {} characters",
            given.len()
        )));
    }
    if given == token(kind, folder, settings, targets) {
        return Ok(true);
    }
    let replacing = targets.iter().filter(|t| t.replaces).count();
    let broken = targets.iter().filter(|t| t.error.is_some()).count();
    let what = if replacing > 0 { format!("{}, replacing {replacing}", targets.len()) } else { targets.len().to_string() };
    Err(ToolError::Refused(format!(
        "this approval is for a different batch than the one this call would run: this call would write {what} file(s) into {} (and {broken} input(s) would fail). Plan it again with batch_plan and approve the batch it describes.",
        folder.display()
    )))
}

/// One attachment to a plan's result: the token, what it covers, and what it does not promise.
pub(crate) fn describe(tok: &str, targets: &[Planned]) -> Value {
    serde_json::json!({
        "token": tok,
        "covers": targets.len(),
        "argument": "approve_token",
        // Said here, where a caller reads it, because both halves are easy to get wrong: the token
        // binds this plan to a run, and the person is still in charge of the decision.
        "meaning": "Pass this as approve_token to the call in `approve`. The run refuses if the batch it would carry out is not the one this token names (a file added or removed, another folder, other settings, an output that is now in the way). The token is not a secret and not a permission: it does not let the call do anything the session's profile refuses, and it does not decide whether the batch should run — that is the person reading this plan.",
    })
}
