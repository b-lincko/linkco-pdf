//! Folders, exclusions and duplicates in a batch (B030, REC-B04).
//!
//! Two tools write a file for every path they are given and overwrite what they write. Until this
//! component they took a list of files and nothing else: a folder could not be named, and the same
//! file could be given twice — by two spellings, by two overlapping folders, or through a link — and
//! be processed twice. The claims that follow are the ones a person running a batch cares about:
//!
//! * a folder means the files in it, in a stable order, with the ones asked for left out;
//! * a file is in the batch once, whatever route found it;
//! * what a run would leave out is reported, not silently dropped;
//! * the plan and the run cover the same files, and the run's `approve` block is the plan;
//! * nothing outside the session's root is reached, and a link cannot make a loop.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};

use pdfcraft_automation::{Automation, Content, Limits, ToolError};
use serde_json::{Value, json};

/// A PDF with `n` 200×300 pt pages reading "Page 1", "Page 2", …
fn fixture(n: usize) -> Vec<u8> {
    let mut objs: Vec<Vec<u8>> = vec![b"<< /Type /Catalog /Pages 2 0 R >>".to_vec()];
    let kids: Vec<String> = (0..n).map(|i| format!("{} 0 R", 4 + 2 * i)).collect();
    objs.push(format!("<< /Type /Pages /Kids [{}] /Count {n} /MediaBox [0 0 200 300] >>", kids.join(" ")).into_bytes());
    objs.push(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec());
    for i in 0..n {
        objs.push(format!("<< /Type /Page /Parent 2 0 R /Contents {} 0 R /Resources << /Font << /F1 3 0 R >> >> >>", 5 + 2 * i).into_bytes());
        let body = format!("BT /F1 24 Tf 20 150 Td (Page {}) Tj ET", i + 1);
        objs.push(format!("<< /Length {} >>\nstream\n{body}\nendstream", body.len()).into_bytes());
    }
    let mut out = b"%PDF-1.7\n".to_vec();
    let mut offsets = Vec::new();
    for (i, o) in objs.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", i + 1).as_bytes());
        out.extend_from_slice(o);
        out.extend_from_slice(b"\nendobj\n");
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", objs.len() + 1).as_bytes());
    for o in offsets {
        out.extend_from_slice(format!("{o:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", objs.len() + 1).as_bytes());
    out
}

/// A fresh root with the tree a batch test needs:
///
/// ```text
/// in/a.pdf            1 page
/// in/draft.draft.pdf  1 page
/// in/notes.txt        not a PDF, and not the walker's business
/// in/.hidden/d.pdf    1 page, behind a dot
/// in/sub/b.pdf        2 pages
/// in/sub/deep/c.pdf   3 pages
/// ```
fn tree(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("pdfcraft-batch-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    for sub in ["in/sub/deep", "in/.hidden"] {
        std::fs::create_dir_all(dir.join(sub)).unwrap();
    }
    std::fs::write(dir.join("in/a.pdf"), fixture(1)).unwrap();
    std::fs::write(dir.join("in/draft.draft.pdf"), fixture(1)).unwrap();
    std::fs::write(dir.join("in/notes.txt"), b"not a pdf").unwrap();
    std::fs::write(dir.join("in/.hidden/d.pdf"), fixture(1)).unwrap();
    std::fs::write(dir.join("in/sub/b.pdf"), fixture(2)).unwrap();
    std::fs::write(dir.join("in/sub/deep/c.pdf"), fixture(3)).unwrap();
    dir
}

fn auto(dir: &Path) -> Automation {
    Automation::new().with_root(dir).unwrap().with_clock(|| 1_700_000_000)
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

/// The files a call listed, as strings.
fn files(v: &Value) -> Vec<String> {
    v["files"].as_array().unwrap().iter().map(|f| f.as_str().unwrap().to_string()).collect()
}

/// Why a path was left out, or `None` when it was not mentioned at all.
fn why(v: &Value, path: &str) -> Option<String> {
    v["skipped"].as_array().unwrap().iter().find(|s| s["path"] == path).map(|s| s["why"].as_str().unwrap_or_default().to_string())
}

/// Everything under `dir`, sorted: what a call that writes nothing must leave unchanged.
fn listing(dir: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).unwrap() {
            let e = e.unwrap();
            let p = e.path();
            if p.is_dir() {
                stack.push(p.clone());
            }
            out.push(p.strip_prefix(dir).unwrap().to_string_lossy().replace('\\', "/"));
        }
    }
    out.sort();
    out
}

#[test]
fn a_folder_is_walked_in_name_order_and_says_what_it_left_out() {
    let dir = tree("walk");
    let mut a = auto(&dir);
    let expanded = ok(&mut a, "batch_expand", json!({ "paths": ["in"], "exclude": ["*.draft.pdf"] }));

    // Subfolders are walked, entries come in name order, and the dot-file and the excluded file
    // are not in the list.
    assert_eq!(files(&expanded), ["in/a.pdf", "in/notes.txt", "in/sub/b.pdf", "in/sub/deep/c.pdf"], "{expanded}");
    assert_eq!(expanded["folders"], 3, "in, in/sub and in/sub/deep were walked: {expanded}");
    assert!(why(&expanded, "in/draft.draft.pdf").unwrap_or_default().contains("*.draft.pdf"), "{expanded}");
    assert!(why(&expanded, "in/.hidden").unwrap_or_default().contains("hidden"), "{expanded}");

    // Asking for the hidden name takes it, and nothing else changes.
    let with_hidden = ok(&mut a, "batch_expand", json!({ "paths": ["in"], "exclude": ["*.draft.pdf"], "include_hidden": true }));
    assert!(files(&with_hidden).contains(&"in/.hidden/d.pdf".to_string()), "{with_hidden}");
}

#[test]
fn the_same_file_reached_twice_is_in_the_batch_once() {
    let dir = tree("dupes");
    let mut a = auto(&dir);
    // A folder, a file inside it, and the same file again spelled with a `..`: one entry.
    let expanded = ok(&mut a, "batch_expand", json!({ "paths": ["in", "in/a.pdf", "in/sub/../a.pdf"], "exclude": ["*.draft.pdf"] }));
    let listed = files(&expanded);
    assert_eq!(listed.iter().filter(|p| p.ends_with("a.pdf")).count(), 1, "{expanded}");
    assert_eq!(listed, ["in/a.pdf", "in/notes.txt", "in/sub/b.pdf", "in/sub/deep/c.pdf"], "the file keeps the place the folder gave it: {expanded}");
    // The second spelling is reported rather than silently dropped.
    let skipped: Vec<&str> = expanded["skipped"].as_array().unwrap().iter().filter_map(|s| s["path"].as_str()).collect();
    assert_eq!(skipped.iter().filter(|p| p.ends_with("a.pdf")).count(), 1, "{expanded}");
    assert!(why(&expanded, "in/sub/../a.pdf").unwrap_or_default().contains("already in this batch"), "{expanded}");
}

#[test]
fn depth_stops_where_it_is_told_to() {
    let dir = tree("depth");
    let mut a = auto(&dir);
    // 1 means the folder's own files: no subfolder is walked at all.
    let shallow = ok(&mut a, "batch_expand", json!({ "paths": ["in"], "depth": 1, "exclude": ["*.draft.pdf"] }));
    assert_eq!(files(&shallow), ["in/a.pdf", "in/notes.txt"], "{shallow}");
    assert!(why(&shallow, "in/sub").unwrap_or_default().contains("deeper"), "{shallow}");
    // 2 takes one level of subfolder.
    let one_down = ok(&mut a, "batch_expand", json!({ "paths": ["in"], "depth": 2, "exclude": ["*.draft.pdf"] }));
    assert_eq!(files(&one_down), ["in/a.pdf", "in/notes.txt", "in/sub/b.pdf"], "{one_down}");
    // A depth of 0 is not "nothing": it is a mistake, and it is refused as one.
    assert!(matches!(a.call("batch_expand", &json!({ "paths": ["in"], "depth": 0 })), Err(ToolError::InvalidArgs(_))));
}

#[test]
fn the_folder_a_run_writes_into_is_not_part_of_its_input() {
    let dir = tree("output");
    std::fs::create_dir_all(dir.join("in/out")).unwrap();
    std::fs::write(dir.join("in/out/earlier.pdf"), fixture(1)).unwrap();
    let mut a = auto(&dir);

    // Without a folder to write into, the earlier output is just another file in the tree.
    let plain = ok(&mut a, "batch_expand", json!({ "paths": ["in"], "exclude": ["*.draft.pdf"] }));
    assert!(files(&plain).contains(&"in/out/earlier.pdf".to_string()), "{plain}");

    // With one, it is left out: a second run must not pick up what the first wrote.
    let guarded = ok(&mut a, "batch_expand", json!({ "paths": ["in"], "exclude": ["*.draft.pdf"], "folder": "in/out" }));
    assert!(!files(&guarded).contains(&"in/out/earlier.pdf".to_string()), "{guarded}");
    assert!(why(&guarded, "in/out/earlier.pdf").unwrap_or_default().contains("output folder"), "{guarded}");
}

#[test]
fn expanding_writes_nothing() {
    let dir = tree("read-only");
    let before = listing(&dir);
    let mut a = auto(&dir);
    ok(&mut a, "batch_expand", json!({ "paths": ["in"], "folder": "out" }));
    assert_eq!(listing(&dir), before, "the plan of a call must leave the filesystem as it was");
}

#[test]
fn a_plan_and_a_run_cover_the_same_files_and_approving_writes_exactly_them() {
    let dir = tree("plan-and-run");
    let mut a = auto(&dir);
    let plan = ok(
        &mut a,
        "batch_plan",
        json!({
            "operation": "action",
            "steps": [{ "step": "set_title", "arg": "Planned" }],
            "paths": ["in"],
            "exclude": ["*.draft.pdf", "*.txt"],
            "folder": "out"
        }),
    );
    let targets: Vec<String> = plan["targets"].as_array().unwrap().iter().map(|t| t["path"].as_str().unwrap().to_string()).collect();
    assert_eq!(targets, ["in/a.pdf", "in/sub/b.pdf", "in/sub/deep/c.pdf"], "{plan}");
    // The plan says what it left out, so the list it describes is the whole truth.
    assert!(why(&plan, "in/draft.draft.pdf").unwrap_or_default().contains("*.draft.pdf"), "{plan}");
    assert!(why(&plan, "in/notes.txt").unwrap_or_default().contains("*.txt"), "{plan}");

    // Approving is not a re-typing: the arguments are the files the plan was made from, by name.
    let approve_paths: Vec<&str> = plan["approve"]["args"]["paths"].as_array().unwrap().iter().map(|p| p.as_str().unwrap()).collect();
    assert_eq!(approve_paths, targets, "{plan}");

    let run = ok(&mut a, "action_run", plan["approve"]["args"].clone());
    let ran: Vec<String> = run["files"].as_array().unwrap().iter().map(|f| f["path"].as_str().unwrap().to_string()).collect();
    assert_eq!(ran, targets, "the run covers exactly the planned files: {run}");
    assert_eq!(run["skipped"].as_array().unwrap().len(), 0, "an approved plan has nothing left to leave out: {run}");
    // And the outputs are where the plan said they would be.
    let outputs: Vec<String> = plan["targets"].as_array().unwrap().iter().map(|t| t["output"].as_str().unwrap().to_string()).collect();
    for output in outputs {
        assert!(Path::new(&output).exists(), "{output} was in the plan and the run did not write it");
    }
}

#[test]
fn a_run_processes_a_file_once_even_when_the_list_names_it_twice() {
    let dir = tree("run-twice");
    let mut a = auto(&dir);
    let run = ok(
        &mut a,
        "action_run",
        json!({
            "steps": [{ "step": "set_title", "arg": "Once" }],
            "paths": ["in/a.pdf", "in", "in/sub/../a.pdf"],
            "exclude": ["*.draft.pdf", "*.txt"],
            "folder": "out"
        }),
    );
    let ran: Vec<String> = run["files"].as_array().unwrap().iter().map(|f| f["path"].as_str().unwrap().to_string()).collect();
    assert_eq!(ran, ["in/a.pdf", "in/sub/b.pdf", "in/sub/deep/c.pdf"], "{run}");
    // One output for one input: the duplicate did not take a second name.
    let out = listing(&dir.join("out"));
    assert_eq!(out, ["a.pdf", "b.pdf", "c.pdf"], "{out:?}");
    // The skipped list says why the second spelling was not processed.
    assert!(why(&run, "in/sub/../a.pdf").unwrap_or_default().contains("already in this batch"), "{run}");
}

#[test]
fn a_folder_with_more_files_than_the_batch_limit_is_refused_before_anything_is_written() {
    let dir = tree("limit");
    let before = listing(&dir);
    let mut a = auto(&dir).with_limits(Limits { max_batch_items: 2, ..Default::default() });
    let refused = a.call(
        "action_run",
        &json!({
            "steps": [{ "step": "set_title", "arg": "Too many" }],
            "paths": ["in"],
            "exclude": ["*.draft.pdf"],
            "folder": "out"
        }),
    );
    assert!(matches!(refused, Err(ToolError::Refused(_))), "{refused:?}");
    assert_eq!(listing(&dir), before, "a refused batch reads and writes nothing");
    // The same limit is what a plan promises, so the plan is refused too.
    let planned = a.call("batch_plan", &json!({ "operation": "action", "steps": [{ "step": "set_title", "arg": "x" }], "paths": ["in"], "exclude": ["*.draft.pdf"], "folder": "out" }));
    assert!(matches!(planned, Err(ToolError::Refused(_))), "{planned:?}");
}

#[test]
fn a_file_that_is_not_there_is_still_a_per_file_error_in_order() {
    let dir = tree("missing");
    let mut a = auto(&dir);
    let run = ok(
        &mut a,
        "action_run",
        json!({ "steps": [{ "step": "set_title", "arg": "x" }], "paths": ["in/a.pdf", "in/missing.pdf", "in/sub/b.pdf"], "folder": "out" }),
    );
    let list = run["files"].as_array().unwrap();
    assert_eq!(list.len(), 3, "{run}");
    assert!(list[1]["error"].as_str().is_some_and(|e| e.contains("missing.pdf")), "{run}");
    assert_eq!(list[0]["path"], "in/a.pdf");
    assert_eq!(list[2]["path"], "in/sub/b.pdf");
}

/// A link to a folder is not followed. It is the one thing that could make a walk never end, so it
/// is refused by rule rather than by a counter.
#[cfg(unix)]
#[test]
fn a_link_to_a_folder_cannot_make_a_loop() {
    let dir = tree("loop");
    std::os::unix::fs::symlink(dir.join("in"), dir.join("in/sub/loop")).unwrap();
    let mut a = auto(&dir);
    let expanded = ok(&mut a, "batch_expand", json!({ "paths": ["in"], "exclude": ["*.draft.pdf"] }));
    assert_eq!(files(&expanded), ["in/a.pdf", "in/notes.txt", "in/sub/b.pdf", "in/sub/deep/c.pdf"], "{expanded}");
    assert!(why(&expanded, "in/sub/loop").unwrap_or_default().contains("link to a folder"), "{expanded}");
}

/// A link that leads out of the root is not followed either: the confinement rules come first.
#[cfg(unix)]
#[test]
fn a_link_out_of_the_root_is_not_walked_or_listed() {
    let dir = tree("escape");
    let outside = std::env::temp_dir().join(format!("pdfcraft-batch-outside-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&outside);
    std::fs::create_dir_all(&outside).unwrap();
    std::fs::write(outside.join("secret.pdf"), fixture(1)).unwrap();
    std::os::unix::fs::symlink(&outside, dir.join("in/out-link")).unwrap();
    std::os::unix::fs::symlink(outside.join("secret.pdf"), dir.join("in/secret.pdf")).unwrap();

    let mut a = auto(&dir);
    let expanded = ok(&mut a, "batch_expand", json!({ "paths": ["in"], "exclude": ["*.draft.pdf"] }));
    let listed = files(&expanded);
    assert!(!listed.iter().any(|p| p.contains("secret")), "{expanded}");
    assert!(why(&expanded, "in/secret.pdf").unwrap_or_default().contains("outside the allowed directory"), "{expanded}");
    let _ = std::fs::remove_dir_all(&outside);
}
