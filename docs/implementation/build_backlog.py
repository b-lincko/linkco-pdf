#!/usr/bin/env python3
"""Generate/check the live TODO without changing the dated research snapshot.

python3 docs/implementation/build_backlog.py [--check]
Standard library only. Status and component evidence live in progress.toml.
"""
from __future__ import annotations

import argparse
import collections
import csv
import io
from pathlib import Path
import tomllib

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
RESEARCH = ROOT / "docs/research/pdf-product-strategy-2026-10-25"
STAGES = ("basic", "advanced", "future")
# These are new-service prerequisites, not blockers on the account-free local editor.
SERVICE_GATES = {"H06", "H07", "H08", "H09", "H10", "H11", "H12", "H13", "H14", "H15", "H17", "H18", "H19", "H20", "D07", "T05", "B08"}


def read_csv(name):
    with (RESEARCH / name).open(encoding="utf-8", newline="") as f:
        return list(csv.DictReader(f))


def default_stage(priority):
    return "basic" if priority in {"P0", "P1"} else "advanced" if priority == "P2" else "future"


def recommendation_stage(row):
    rid = row["recommendation_id"]
    if rid.startswith("M") or row["priority"] == "P3":
        return "future"
    if rid in SERVICE_GATES:
        return "advanced"
    return default_stage(row["priority"])


def build():
    progress = tomllib.loads((HERE / "progress.toml").read_text())
    recommendations = read_csv("recommended-roadmap.csv")
    baseline = read_csv("baseline-features.csv")
    gaps = [r for r in baseline if r["repo_status"] in {"partial", "planned"}]
    scope = [r for r in baseline if r["repo_status"] == "na"]
    if (len(recommendations), len(baseline), len(gaps), len(scope)) != (222, 833, 384, 23):
        raise ValueError("Research scope changed: review explicitly instead of silently dropping TODOs")
    rows = []
    linked = collections.defaultdict(list)
    for r in recommendations:
        rid = r["recommendation_id"]
        stage = recommendation_stage(r)
        for feature in r["baseline_ids"].split():
            if feature != "'-":  # Spreadsheet-safe encoding of the source's unmapped marker.
                linked[feature].append(("REC-" + rid, stage))
        rows.append({"id": "REC-" + rid, "kind": "recommendation", "stage": stage,
                     "priority": r["priority"], "title": r["function"], "status": "pending",
                     "baseline_status": r["baseline_status"], "related_tasks": r["baseline_ids"],
                     "acceptance": r["done_when"], "notes": "Dependencies: " + r["dependencies"]})
    for r in gaps + scope:
        is_scope = r["repo_status"] == "na"
        matches = linked[r["feature_id"]]
        stage = min((s for _, s in matches), key=STAGES.index) if matches else default_stage(r["repo_tier"])
        rows.append({"id": ("SCOPE-" if is_scope else "GAP-") + r["feature_id"],
                     "kind": "scope review" if is_scope else "baseline gap", "stage": "future" if is_scope else stage,
                     "priority": r["repo_tier"], "title": r["title"], "status": "scope_review" if is_scope else "pending",
                     "baseline_status": r["repo_status"], "related_tasks": "; ".join(i for i, _ in matches),
                     "acceptance": "Audit current behavior, scope a component, add API/tool coverage where applicable and prove its specific workflow with tests.",
                     "notes": r["notes"]})
    for cid, c in progress["components"].items():
        if c["status"] == "complete" and (c.get("ci") != "passed" or not c.get("ci_run_urls")):
            raise ValueError(f"{cid}: cannot be complete without recorded passing CI")
        rows.append({"id": cid, "kind": "component", "stage": c["stage"], "priority": "P0",
                     "title": c["title"], "status": c["status"], "baseline_status": "bounded implementation component",
                     "related_tasks": "; ".join(c["parent_tasks"]), "acceptance": c["acceptance"],
                     "notes": c.get("notes", c.get("audit", ""))})
    by_id = {r["id"]: r for r in rows}
    if len(by_id) != len(rows):
        raise ValueError("Duplicate TODO IDs")
    for tid, override in progress.get("task_status", {}).items():
        if tid not in by_id:
            raise ValueError(f"Unknown task override: {tid}")
        if override["status"] in {"complete", "covered"} and not override.get("evidence"):
            raise ValueError(f"{tid}: completion needs task-specific evidence")
        by_id[tid]["status"] = override["status"]
        by_id[tid]["notes"] += " " + override.get("notes", "")
    for c in progress["components"].values():
        if not set(c["parent_tasks"]) <= by_id.keys():
            raise ValueError("Unknown component parent")
    workflow = progress["workflow"]
    batch = workflow.get("batch_authorized_stage")
    batch_authorized = (
        batch == "basic"
        and workflow["active_stage"] == "basic"
        and bool(workflow.get("batch_authorization", "").strip())
        and bool(workflow.get("batch_authorization_date", "").strip())
        and workflow.get("confirmation_required_before_stage_change") is True
    )
    if (workflow["order"] != list(STAGES) or workflow["auto_advance"]
            or (not workflow["confirmation_required_after_each_component"] and not batch_authorized)
            or (batch is not None and not batch_authorized)):
        raise ValueError("Do not change the user's stage order or confirmation gate without stage-scoped batch authorization")
    if workflow["active_stage"] not in STAGES or workflow["active_component"] not in progress["components"]:
        raise ValueError("Invalid active stage/component")
    stage_index = STAGES.index(workflow["active_stage"])
    if any(STAGES.index(r["stage"]) < stage_index and r["status"] not in {"complete", "covered"} for r in rows):
        raise ValueError("Cannot advance while earlier stages have unfinished records")
    if progress["components"][workflow["active_component"]]["stage"] != workflow["active_stage"]:
        raise ValueError("Active component must belong to the active stage")
    return rows, progress


def markdown(rows, progress):
    continuation = ("continue the user-authorized basic batch; ask before changing stage"
                    if not progress["workflow"]["confirmation_required_after_each_component"] else "ask the user to continue")
    lines = ["# Live implementation TODO", "", "> Generated from the full research inventory and `progress.toml`; do not edit statuses here.", "",
             "[Workflow and completion rules](README.md) · [Filterable CSV](TODO.csv) · [Original research](../research/pdf-product-strategy-2026-10-25/README.md)", "",
             f"Order: **basic → advanced → future**. Implement → audit → fix → re-audit → test → push → verify CI → {continuation}.", "",
             "Scope: **222 recommendations + 384 existing gaps + 23 scope reviews**, plus bounded components. These overlap; they are not 629 distinct new functions. Existing shipped features are not marked missing. Complete a gap only when its own criteria are demonstrated, not just because a related component landed.", "",
             "P0 is a safety priority, not a universal scheduling stage. Gates for a new collaboration service stay attached to that advanced service; native mobile work is future. Never launch such a surface before its safety gates pass.", "",
             f"**Active component:** {progress['workflow']['active_component']} · **Active stage:** {progress['workflow']['active_stage']}", ""]
    for stage in STAGES:
        sub = [r for r in rows if r["stage"] == stage]
        counts = collections.Counter(r["status"] for r in sub)
        lines += [f"## {stage.title()} ({len(sub)} records)", "", ", ".join(f"{s}: {n}" for s, n in sorted(counts.items())), "",
                  "| ID | TODO | Priority | Status | Related tasks |", "|---|---|---|---|---|"]
        for r in sorted(sub, key=lambda x: (0 if x["kind"] == "component" else 1, x["priority"], x["id"])):
            mark = "[x]" if r["status"] in {"complete", "covered"} else "[ ]"
            values = [r["id"], mark + " " + r["title"], r["priority"], r["status"], r["related_tasks"]]
            lines.append("| " + " | ".join(str(v).replace("|", "\\|").replace("\n", " ") for v in values) + " |")
        lines.append("")
    lines += ["## Component verification records", ""]
    for cid, c in progress["components"].items():
        lines += [f"### {cid}: {c['title']}", "", f"- Status: **{c['status']}**; parent(s): {', '.join(c['parent_tasks'])}.", f"- Acceptance: {c['acceptance']}"]
        for key in ("audit", "verification_limit", "ci", "notes"):
            if c.get(key):
                lines.append(f"- {key.replace('_', ' ').title()}: {c[key]}")
        lines.extend(f"- Local evidence: {e}." for e in c.get("local_evidence", []))
        lines.extend(f"- CI run: {url}" for url in c.get("ci_run_urls", []))
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def csv_text(rows):
    out = io.StringIO(newline="")
    writer = csv.DictWriter(out, fieldnames=list(rows[0]), lineterminator="\n")
    writer.writeheader()
    for row in rows:
        # Textual TODO fields must not execute as formulas in a spreadsheet.
        writer.writerow({k: "'" + v if v.startswith(("=", "+", "-", "@")) else v for k, v in row.items()})
    return out.getvalue()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    rows, progress = build()
    outputs = {"TODO.md": markdown(rows, progress), "TODO.csv": csv_text(rows)}
    for name, content in outputs.items():
        path = HERE / name
        if args.check:
            if not path.is_file() or path.read_text() != content:
                raise SystemExit(f"Stale {name}; run python3 docs/implementation/build_backlog.py")
        else:
            path.write_text(content)
    print(f"Validated {len(rows)} TODO records:", dict(collections.Counter(r["stage"] for r in rows)))


if __name__ == "__main__":
    main()
