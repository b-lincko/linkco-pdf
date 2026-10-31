//! `batch_plan`: what a batch write would write, said before it writes it (REC-B03).
//!
//! Two automation tools take a list of files and a folder and write a result for each one:
//! `action_run` (an action's steps) and `ocr_recognize_files` (a searchable copy). Both are
//! marked `destructive`, and for good reason — the reader only finds out what they do by running
//! them, on real files, in a real folder.
//!
//! `batch_plan` answers the same call the writer would take and writes nothing: for each input it
//! names the file the run would write, whether a file is already there (and would be replaced),
//! how many pages the input has and what the operation does that cannot be undone. It also hands
//! back the exact call that would carry the plan out (`approve`), so approving is mechanical
//! rather than a re-typing of the arguments.
//!
//! The naming rule is not re-implemented here: [`crate::batch_input_name`] and
//! [`crate::action_output`] are the functions the writers themselves call, so a plan cannot name a
//! file the run would not write. A plan that lied about the destination would be worse than no
//! plan at all, because it would be believed.

use std::path::Path;
use std::sync::Arc;

use pdfcraft_engine::actions::{Action, Step, builtin};
use serde_json::{Value, json};

use crate::{Args, Automation, Result, ToolError, action_output, batch_input_name, child};

/// The most of an input `batch_plan` reads to describe it: 32 MiB, the cap the interface uses to
/// describe the other version of a saved document. A file over it is still planned — its size,
/// its destination and what would be replaced — with its page count left unknown rather than
/// loading it here.
pub(crate) const PLAN_READ_LIMIT: u64 = 32 << 20;

impl Automation {
    /// `batch_plan`: plan a folder batch without writing anything.
    pub(crate) fn batch_plan(&mut self, a: &Args) -> Result<Value> {
        let operation = a.str("operation")?;
        let paths = a.strs("paths")?;
        if paths.is_empty() {
            return Err(ToolError::InvalidArgs("paths must not be empty: there is nothing to plan".into()));
        }
        // The folder is resolved as a write target (the plan says where the run would put files),
        // but it is never created: planning leaves the filesystem exactly as it found it.
        let folder = self.resolve(a.str("folder")?, true)?;
        // A path that leaves the root refuses the plan: `ocr_recognize_files` refuses the whole call
        // for one, and a plan that described a batch that cannot be run would be no better than one
        // that described it wrongly.
        for p in &paths {
            self.resolve(p, true)?;
        }
        match operation {
            "action" => self.plan_action(a, &paths, &folder),
            "ocr" => self.plan_ocr(a, &paths, &folder),
            other => Err(ToolError::InvalidArgs(format!("unknown operation {other:?} (action, ocr)"))),
        }
    }

    fn plan_action(&mut self, a: &Args, paths: &[&str], folder: &Path) -> Result<Value> {
        let action = action_from_args(a)?;
        let steps: Vec<Value> = action.steps.iter().map(|s| json!({ "step": s.id(), "label": s.label(), "arg": s.arg() })).collect();
        let mut targets = Vec::new();
        // The names already promised in this plan: the run writes them in order, so a second input
        // with the same file name gets the next free `(n)`, exactly as it would after the first.
        let mut promised: Vec<std::path::PathBuf> = Vec::new();
        for &p in paths {
            let name = batch_input_name(p);
            let output = action_output(folder, &name, &promised);
            promised.push(output.clone());
            // The steps go on the target: this file would have them run over it.
            let mut t = self.planned_file(p, &output);
            t["steps"] = Value::Array(steps.clone());
            targets.push(t);
        }
        let mut args = json!({ "paths": paths, "folder": a.str("folder")? });
        match a.opt_str("action")? {
            Some(name) => args["action"] = json!(name),
            None => args["steps"] = a.get("steps").cloned().unwrap_or(Value::Null),
        }
        Ok(plan_json("action", "action_run", folder, targets, args))
    }

    fn plan_ocr(&mut self, a: &Args, paths: &[&str], folder: &Path) -> Result<Value> {
        let settings = self.ocr_settings(a)?;
        let mut targets = Vec::new();
        // `ocr_recognize_files` writes each result under the input's own name, so a name that is
        // already in the folder — or that an earlier input of the same batch writes — is replaced.
        let mut promised: Vec<std::path::PathBuf> = Vec::new();
        for &p in paths {
            let name = batch_input_name(p);
            let output = child(folder, &name);
            promised.push(output.clone());
            let mut t = self.planned_file(p, &output);
            if promised.iter().filter(|x| **x == output).count() > 1 {
                t["replaces"] = json!(true);
                t["replaces_note"] = json!("an earlier input in this batch writes the same file name, so this one's result replaces it");
            }
            t["changes"] = json!([
                format!("reads every page at {} dpi in language {}", settings.dpi.round(), settings.language),
                "adds an invisible text layer over each word; the page image is not changed",
                if settings.skip_text_pages {
                    "leaves pages that already contain text alone"
                } else {
                    "recognises every page, including the ones that already have text"
                },
            ]);
            targets.push(t);
        }
        let mut args = json!({ "paths": paths, "folder": a.str("folder")? });
        for (key, value) in
            [("dpi", a.get("dpi").cloned()), ("language", a.get("language").cloned()), ("skip_text_pages", a.get("skip_text_pages").cloned())]
        {
            if let Some(v) = value {
                args[key] = v;
            }
        }
        Ok(plan_json("ocr", "ocr_recognize_files", folder, targets, args))
    }

    /// One input as the run would see it: where its result goes, whether that file is already
    /// there, and what the input file itself is.
    fn planned_file(&self, input: &str, output: &Path) -> Value {
        let replaces = std::fs::metadata(output).is_ok();
        let mut target = json!({ "path": input, "output": output.to_string_lossy(), "replaces": replaces });
        if replaces {
            target["replaces_note"] = json!("a file is already there and would be replaced");
        }
        match self.resolve(input, false) {
            Err(e) => target["error"] = json!(e.to_string()),
            Ok(resolved) => match describe(&resolved) {
                Ok(Value::Object(facts)) => {
                    if let Some(obj) = target.as_object_mut() {
                        for (k, v) in facts {
                            obj.insert(k, v);
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => target["error"] = json!(e),
            },
        }
        target
    }
}

/// The action `action_run` would run, read from the same arguments by the same code, so the plan
/// lists the steps that would actually run.
pub(crate) fn action_from_args(a: &Args) -> Result<Action> {
    if let Some(name) = a.opt_str("action")? {
        return builtin()
            .into_iter()
            .find(|x| x.name.eq_ignore_ascii_case(name))
            .ok_or_else(|| ToolError::InvalidArgs(format!("no built-in action {name:?}")));
    }
    let items = a.get("steps").and_then(Value::as_array).ok_or_else(|| ToolError::InvalidArgs("give action or steps".into()))?;
    let steps = items
        .iter()
        .map(|it| {
            let id = it["step"].as_str().unwrap_or("");
            Step::from_id(id, it["arg"].as_str().unwrap_or("")).ok_or_else(|| ToolError::InvalidArgs(format!("unknown step {id:?}")))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Action { name: "Custom".into(), description: String::new(), steps, builtin: false })
}

/// The facts a plan reports about one input file: its size, its pages, its title and whether it is
/// encrypted — read with the parser, in a session of its own that is dropped at once, so nothing
/// here can touch a document an agent already has open.
fn describe(path: &Path) -> std::result::Result<Value, String> {
    let meta = std::fs::metadata(path).map_err(|e| e.to_string())?;
    let mut facts = json!({ "bytes": meta.len() });
    if meta.len() > PLAN_READ_LIMIT {
        facts["pages"] = Value::Null;
        facts["note"] =
            json!(format!("not opened here: {} bytes is over the {PLAN_READ_LIMIT} byte limit, so its pages are not counted", meta.len()));
        return Ok(facts);
    }
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let mut session = pdfcraft_engine::Session::new();
    let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| "document.pdf".into());
    match session.open(name, None, Arc::new(bytes), None) {
        Ok(id) => match session.get(id) {
            Some(d) => {
                facts["pages"] = json!(d.info.pages.len());
                facts["title"] = json!(d.info.title);
                facts["encrypted"] = json!(d.info.encrypted);
                if !d.info.warnings.is_empty() {
                    facts["repair_notes"] = json!(d.info.warnings.len());
                }
            }
            None => facts["note"] = json!("the parser opened it and then lost it"),
        },
        Err(e) => {
            facts["pages"] = Value::Null;
            facts["note"] = json!(format!("not a PDF this build can open without a password ({e}); the run would fail on it"));
        }
    }
    Ok(facts)
}

/// The plan itself: what would be written, what would be replaced, what cannot be undone, and the
/// call that would do it.
fn plan_json(operation: &str, tool: &str, folder: &Path, targets: Vec<Value>, approve_args: Value) -> Value {
    let replaced: Vec<Value> =
        targets.iter().filter(|t| t.get("replaces").and_then(Value::as_bool).unwrap_or(false)).map(|t| t["output"].clone()).collect();
    let writable = targets.iter().filter(|t| t.get("error").is_none()).count();
    let mut irreversible = vec![json!(format!("writes {writable} file(s) into the folder; the inputs in paths are not changed"))];
    if !replaced.is_empty() {
        irreversible.push(json!(format!(
            "replaces {} file(s) already in the folder: {}",
            replaced.len(),
            replaced.iter().filter_map(Value::as_str).collect::<Vec<_>>().join(", ")
        )));
        irreversible.push(json!("a replaced file cannot be brought back: nothing here keeps a copy of it"));
    }
    if operation == "ocr" {
        irreversible.push(json!("the text layer is added to the copy; skip_text_pages decides whether a page that already has text is read again"));
    }
    json!({
        "operation": operation,
        "dry_run": true,
        "writes": 0,
        "folder": folder.to_string_lossy(),
        "folder_exists": folder.exists(),
        "targets": targets,
        "replaces": replaced,
        "irreversible": irreversible,
        // The call that would carry this out, unchanged, so approving is not a re-typing.
        "approve": { "tool": tool, "args": approve_args },
    })
}
