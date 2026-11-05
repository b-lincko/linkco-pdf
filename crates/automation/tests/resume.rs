//! `--resume` for the batch writers (B044, REC-B04): a batch that stopped part-way — a crash, a
//! killed process, a time limit — can be run again, and the files it already wrote are reported
//! as resumed and left alone instead of being done again (an action run would otherwise write
//! them under a stepped-aside `name (2).pdf`).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};

use pdfcraft_automation::{Automation, Content};
use serde_json::{Value, json};

fn workdir(test: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pdfcraft-resume-{test}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn ok(a: &mut Automation, tool: &str, args: Value) -> Value {
    match a.call(tool, &args) {
        Ok(mut c) => match c.remove(0) {
            Content::Json(v) => v,
            other => panic!("{tool}: expected JSON, got {other:?}"),
        },
        Err(e) => panic!("{tool} {args}: {e}"),
    }
}

fn auto(dir: &Path) -> Automation {
    Automation::new().with_root(dir).unwrap().with_clock(|| 1_700_000_000)
}

/// Write a small PDF called `name` into `dir`.
fn seed(dir: &Path, name: &str) {
    let mut a = auto(dir);
    let doc = ok(&mut a, "doc_create", json!({ "from": "text", "text": "Page one" }))["doc"].as_u64().unwrap();
    ok(&mut a, "doc_save", json!({ "doc": doc, "path": name }));
}

fn steps() -> Value {
    json!({ "steps": [{ "step": "set_title", "arg": "Resumed" }], "paths": ["a.pdf", "b.pdf", "c.pdf"], "folder": "out" })
}

#[test]
fn a_resumed_run_skips_the_files_an_earlier_run_already_wrote() {
    let dir = workdir("skip");
    for name in ["a.pdf", "b.pdf", "c.pdf"] {
        seed(&dir, name);
    }
    // An earlier run wrote b's result (e.g. it was killed after the first file).
    std::fs::create_dir_all(dir.join("out")).unwrap();
    std::fs::write(dir.join("out/b.pdf"), b"already there from the earlier run").unwrap();

    let mut a = auto(&dir);
    let mut args = steps();
    args["resume"] = json!(true);
    let run = ok(&mut a, "action_run", args);
    let files = run["files"].as_array().unwrap();
    assert_eq!(files.len(), 3, "{run}");
    let by_path = |p: &str| files.iter().find(|f| f["path"] == p).unwrap().clone();
    assert!(by_path("a.pdf")["output"].is_string(), "a was done now: {run}");
    assert!(by_path("c.pdf")["output"].is_string(), "c was done now: {run}");
    let b = by_path("b.pdf");
    assert!(b["resumed"].is_string(), "b is reported as resumed: {run}");
    assert!(b["output"].is_null() && b["error"].is_null(), "a resumed file is neither written nor an error: {run}");
    // The earlier run's output is untouched, and a and c were written under their own names.
    assert_eq!(std::fs::read(dir.join("out/b.pdf")).unwrap(), b"already there from the earlier run");
    assert!(dir.join("out/a.pdf").exists() && dir.join("out/c.pdf").exists(), "the rest of the batch was done");
    // No stepped-aside name was invented for b.
    assert!(!dir.join("out/b (2).pdf").exists(), "a resumed file is not redone under another name");

    // Without resume the same call does the file again, under the next free name (the own name is
    // taken on disk, and a plain run never replaces what is there).
    let mut b2 = auto(&dir);
    let run = ok(&mut b2, "action_run", steps());
    let files = run["files"].as_array().unwrap();
    let b = files.iter().find(|f| f["path"] == "b.pdf").unwrap();
    assert!(b["output"].as_str().unwrap().ends_with("b (2).pdf"), "without resume b is redone under a stepped-aside name: {run}");
    assert!(dir.join("out/b (2).pdf").exists());
    assert_eq!(std::fs::read(dir.join("out/b.pdf")).unwrap(), b"already there from the earlier run", "a plain run still replaces nothing");
}

#[test]
fn a_plan_made_with_resume_marks_the_finished_files_and_carries_the_flag() {
    let dir = workdir("plan");
    for name in ["a.pdf", "b.pdf"] {
        seed(&dir, name);
    }
    std::fs::create_dir_all(dir.join("out")).unwrap();
    std::fs::write(dir.join("out/a.pdf"), b"done earlier").unwrap();

    let mut a = auto(&dir);
    let plan = ok(
        &mut a,
        "batch_plan",
        json!({ "operation": "action", "steps": [{ "step": "set_title", "arg": "Resumed" }], "paths": ["a.pdf", "b.pdf"], "folder": "out", "resume": true }),
    );
    let targets = plan["targets"].as_array().unwrap();
    let a_target = targets.iter().find(|t| t["path"] == "a.pdf").unwrap();
    let b_target = targets.iter().find(|t| t["path"] == "b.pdf").unwrap();
    assert_eq!(a_target["resumed"], true, "a is marked resumed: {plan}");
    assert!(a_target["resumed_note"].is_string(), "{plan}");
    assert!(b_target["resumed"].is_null(), "b is not resumed: {plan}");
    // The approve call carries the flag, so following the plan resumes.
    assert_eq!(plan["approve"]["args"]["resume"], true, "{plan}");
    assert_eq!(plan["approve"]["tool"], "action_run", "{plan}");

    // A plan without resume marks nothing and does not carry the flag.
    let mut b = auto(&dir);
    let plan = ok(
        &mut b,
        "batch_plan",
        json!({ "operation": "action", "steps": [{ "step": "set_title", "arg": "Resumed" }], "paths": ["a.pdf", "b.pdf"], "folder": "out" }),
    );
    assert!(plan["targets"].as_array().unwrap().iter().all(|t| t["resumed"].is_null()), "{plan}");
    assert!(plan["approve"]["args"]["resume"].is_null(), "{plan}");

    // The approval still binds the batch: a token from the resume plan runs the resume call.
    let mut c = auto(&dir);
    let token = plan_token(&mut c);
    let args = json!({ "steps": [{ "step": "set_title", "arg": "Resumed" }], "paths": ["a.pdf", "b.pdf"], "folder": "out", "resume": true, "approve_token": token });
    let run = ok(&mut c, "action_run", args);
    let files = run["files"].as_array().unwrap();
    let a = files.iter().find(|f| f["path"] == "a.pdf").unwrap();
    assert!(a["resumed"].is_string(), "the approved resume run skipped the finished file: {run}");
    assert_eq!(std::fs::read(dir.join("out/a.pdf")).unwrap(), b"done earlier");
}

/// The token of a resume plan, made through batch_plan.
fn plan_token(a: &mut Automation) -> String {
    let plan = ok(
        a,
        "batch_plan",
        json!({ "operation": "action", "steps": [{ "step": "set_title", "arg": "Resumed" }], "paths": ["a.pdf", "b.pdf"], "folder": "out", "resume": true }),
    );
    plan["approval"]["token"].as_str().unwrap().to_string()
}
