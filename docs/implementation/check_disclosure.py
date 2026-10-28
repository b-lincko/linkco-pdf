#!/usr/bin/env python3
"""Check that the interface, the README and the tool catalogue say the same thing (REC-R12).

Four files describe what Linkco PDF Editor can do, for four different readers:

* `crates/engine/src/catalog.rs` — the tool catalogue the interface draws (group, item, and the
  milestone an unfinished tool completes in);
* `crates/engine/src/commands.rs` — the commands that actually run;
* `crates/ui-egui/src/home.rs` — the Home cards, in the order the dashboard shows them;
* `README.md` — the standing statement of what the build does (the *Recommended Tools* table) and
  of what it does not do yet (the *What is not finished yet* table).

This script keeps those four honest, and keeps them honest against `parity/acrobat-features.toml`,
which records, per feature, whether it shipped:

* the Home cards must be catalogue groups, and the declared array length must match;
* the README card table must list those cards, in that order, under the catalogue's own labels,
  with each command it names present in the registry;
* the README's not-finished table must list exactly the groups the catalogue does not call Ready,
  with the catalogue's own count of working tools and its own milestone;
* a tool the interface calls planned must not be recorded as shipped in the parity inventory — a
  working tool hidden behind a milestone note is the same lie as a broken one announced as ready,
  just in the other direction.

Commands are also expected to keep the catalogue honest in the same direction (a registered
command must be marked Ready); `crates/engine/src/commands.rs` holds both directions in Rust, next
to the registry it tests. Release notes are per-release history and are not checked here.

Run it directly (`python3 docs/implementation/check_disclosure.py --check`) or through the unit
tests in `test_disclosure.py`, which CI already runs.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys
import tomllib

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent.parent

CATALOG = "crates/engine/src/catalog.rs"
COMMANDS = "crates/engine/src/commands.rs"
HOME = "crates/ui-egui/src/home.rs"
PARITY = "parity/acrobat-features.toml"
README = "README.md"

CARDS_HEADING = "## Recommended Tools"
PARTIAL_HEADING = "## What is not finished yet"

READY = "Ready"
PROVIDER_NOTE = "needs a provider you configure"

# The command registry is written as `c("id", …)` and `ct("id", …)`; nothing else in the file
# starts with a bare `c(`.
_COMMAND = re.compile(r"\bc[t]?\(\"([a-z0-9_.]+)\"")
_GROUP_ID = re.compile(r'^\s*id: "([^"]+)"', re.M)
_GROUP_LABEL = re.compile(r'^\s*label: "([^"]+)"', re.M)
_GROUP_AVAILABILITY = re.compile(r'^\s*availability: (Ready|Planned\("([^"]+)"\)|Provider),', re.M)
_ITEM = re.compile(r'item\("([^"]*)", "[^"]*", "([^"]+)", (Ready|Planned\("([^"]*)"\)|Provider)\)')
_RECOMMENDED = re.compile(r"RECOMMENDED: \[&str; (\d+)\] = \[(.*?)\];", re.S)

_CELL_SEPARATOR = re.compile(r"^:?-{2,}:?$")


def read(root: pathlib.Path, name: str) -> str:
    path = root / name
    if not path.is_file():
        raise LookupError(f"{name}: missing, so the disclosure cannot be checked")
    return path.read_text(encoding="utf-8")


def availability_of(kind: str, milestone: str | None) -> tuple[str, str]:
    """`(kind, note)` for a group or item: `ready`, `planned` + milestone, or `provider`."""
    if kind == READY:
        return READY.lower(), ""
    if kind == "Provider":
        return "provider", PROVIDER_NOTE
    return "planned", milestone or ""


def catalogue(root: pathlib.Path) -> list[dict]:
    """Every tool group, in file order, with its items."""
    text = read(root, CATALOG)
    groups: list[dict] = []
    for block in text.split("    ToolGroup {")[1:]:
        id_ = _GROUP_ID.search(block)
        label = _GROUP_LABEL.search(block)
        availability = _GROUP_AVAILABILITY.search(block)
        if not (id_ and label and availability):
            raise LookupError(f"{CATALOG}: a ToolGroup without an id, label or availability")
        kind, note = availability_of(availability.group(1), availability.group(2))
        items = []
        for item in _ITEM.finditer(block):
            item_kind, item_note = availability_of(item.group(3), item.group(4))
            items.append({"label": item.group(1), "command": item.group(2), "kind": item_kind, "note": item_note})
        groups.append(
            {"id": id_.group(1), "label": label.group(1), "kind": kind, "note": note, "items": items}
        )
    if len(groups) < 20:
        raise LookupError(f"{CATALOG}: read {len(groups)} tool groups; the parse is out of date")
    if sum(len(g["items"]) for g in groups) != text.count('item("'):
        raise LookupError(f"{CATALOG}: some items did not parse; the parse is out of date")
    return groups


def registered(root: pathlib.Path) -> set[str]:
    """Every command id in the registry."""
    text = read(root, COMMANDS)
    found = set(_COMMAND.findall(text))
    if len(found) < 100:
        raise LookupError(f"{COMMANDS}: read {len(found)} commands; the parse is out of date")
    return found


def recommended(root: pathlib.Path) -> tuple[int, list[str]]:
    """The Home cards: the declared length and the ids, in order."""
    match = _RECOMMENDED.search(read(root, HOME))
    if not match:
        raise LookupError(f"{HOME}: RECOMMENDED is gone or written differently; update this check")
    return int(match.group(1)), re.findall(r'"([^"]+)"', match.group(2))


def shipped_features(root: pathlib.Path) -> dict[str, list[str]]:
    """Command id → the parity feature ids that record it as shipped."""
    data = tomllib.loads(read(root, PARITY))
    found: dict[str, list[str]] = {}
    for feature in data.get("feature", []):
        if feature.get("status") != "shipped":
            continue
        for command in feature.get("commands") or []:
            found.setdefault(command, []).append(feature.get("id", "?"))
    return found


def section(text: str, heading: str) -> list[str]:
    """The lines of `heading`'s section, up to the next `## ` heading."""
    lines = text.splitlines()
    try:
        start = next(i for i, line in enumerate(lines) if line.strip() == heading)
    except StopIteration:
        raise LookupError(f"{README}: no {heading!r} section") from None
    out = []
    for line in lines[start + 1 :]:
        if line.startswith("## "):
            break
        out.append(line)
    return out


def table_rows(lines: list[str]) -> list[list[str]]:
    """The body rows of the first Markdown table in `lines`, cells stripped."""
    rows = []
    for line in lines:
        stripped = line.strip()
        if not stripped.startswith("|"):
            continue
        cells = [cell.strip() for cell in stripped.strip("|").split("|")]
        if all(_CELL_SEPARATOR.match(cell) for cell in cells):
            continue
        rows.append(cells)
    return rows[1:] if rows else []


def bold(text: str) -> str | None:
    match = re.fullmatch(r"\*\*(.+?)\*\*", text.strip())
    return match.group(1) if match else None


def qualified(text: str) -> tuple[str | None, str | None]:
    """`**Label** (`id`)` → (label, id); a malformed cell gives (None, None)."""
    match = re.fullmatch(r"\*\*(.+?)\*\* \(`([a-z0-9_]+)`\)", text.strip())
    return (match.group(1), match.group(2)) if match else (None, None)


def ticked(text: str) -> str | None:
    """The single `code` span in a cell, or None when it holds something else."""
    match = re.fullmatch(r"`([^`]+)`", text.strip())
    return match.group(1) if match else None


def problems(root: pathlib.Path = ROOT) -> list[str]:
    """Everything that disagrees, as sentences a person can act on."""
    groups = catalogue(root)
    by_id = {group["id"]: group for group in groups}
    commands = registered(root)
    declared, cards = recommended(root)
    text = read(root, README)
    out: list[str] = []

    if declared != len(cards):
        out.append(f"{HOME}: RECOMMENDED declares {declared} cards but lists {len(cards)}")
    if len(set(cards)) != len(cards):
        out.append(f"{HOME}: RECOMMENDED lists the same group twice")
    unknown = [card for card in cards if card not in by_id]
    for card in unknown:
        out.append(f"{HOME}: RECOMMENDED names {card!r}, which is not a tool group in {CATALOG}")

    # --- the README's card table, against the Home cards and the catalogue -------------------
    card_rows = table_rows(section(text, CARDS_HEADING))
    listed: list[str] = []
    for row in card_rows:
        if len(row) != 4:
            out.append(f"{README} {CARDS_HEADING}: a row does not have four cells: {row}")
            continue
        label = bold(row[0])
        group = ticked(row[1])
        command = ticked(row[3])
        if group is None:
            # The Open card: not a tool group, so it only has to name a command that runs.
            if command is None or command not in commands:
                out.append(f"{README} {CARDS_HEADING}: {row[0]} names {row[3]!r}, which is not a registered command")
            continue
        listed.append(group)
        known = by_id.get(group)
        if known is None:
            out.append(f"{README} {CARDS_HEADING}: {group!r} is not a tool group in {CATALOG}")
            continue
        if label != known["label"]:
            out.append(
                f"{README} {CARDS_HEADING}: {group} is called {row[0]!r}, but {CATALOG} calls it {known['label']!r}"
            )
        if command is None or command not in commands:
            out.append(f"{README} {CARDS_HEADING}: {group} names {row[3]!r}, which is not a registered command")
        elif not any(item["command"] == command for item in known["items"]):
            out.append(f"{README} {CARDS_HEADING}: {command} is not a tool in the {group} group")
    if listed != cards:
        out.append(
            f"{README} {CARDS_HEADING}: lists {listed} but the Home dashboard shows {cards}; "
            "the README and the card order must agree"
        )

    # --- the README's not-finished table, against the catalogue ------------------------------
    unfinished = [group for group in groups if group["kind"] != "ready"]
    expected = [group["id"] for group in unfinished]
    partial_rows = table_rows(section(text, PARTIAL_HEADING))
    seen: list[str] = []
    for row in partial_rows:
        if len(row) != 3:
            out.append(f"{README} {PARTIAL_HEADING}: a row does not have three cells: {row}")
            continue
        label, group_id = qualified(row[0])
        if group_id is None:
            out.append(f"{README} {PARTIAL_HEADING}: {row[0]!r} is not `**Name** (`group-id`)`")
            continue
        seen.append(group_id)
        group = by_id.get(group_id)
        if group is None:
            out.append(f"{README} {PARTIAL_HEADING}: {group_id!r} is not a tool group in {CATALOG}")
            continue
        if group["kind"] == "ready":
            out.append(f"{README} {PARTIAL_HEADING}: {group_id} is finished, so it does not belong in this table")
            continue
        if label != group["label"]:
            out.append(f"{README} {PARTIAL_HEADING}: {group_id} is called {label!r}, but {CATALOG} calls it {group['label']!r}")
        ready = sum(1 for item in group["items"] if item["kind"] == "ready")
        working = f"{ready} of {len(group['items'])} tools"
        if row[1] != working:
            out.append(f"{README} {PARTIAL_HEADING}: {group_id} says {row[1]!r}, but {CATALOG} has {working}")
        if row[2] != group["note"]:
            out.append(f"{README} {PARTIAL_HEADING}: {group_id} completes in {row[2]!r}, but {CATALOG} says {group['note']!r}")
    missing = [group_id for group_id in expected if group_id not in seen]
    extra = [group_id for group_id in seen if group_id not in expected]
    if missing:
        out.append(f"{README} {PARTIAL_HEADING}: does not list {', '.join(missing)}")
    if extra:
        out.append(f"{README} {PARTIAL_HEADING}: lists {', '.join(extra)}, which {CATALOG} calls finished")
    if seen and [group_id for group_id in expected if group_id in seen] != seen:
        out.append(f"{README} {PARTIAL_HEADING}: the order differs from {CATALOG}; list the groups in catalogue order")

    # --- one group, one story ---------------------------------------------------------------
    for group in groups:
        ready = sum(1 for item in group["items"] if item["kind"] == "ready")
        if group["kind"] != "ready" and ready == len(group["items"]):
            out.append(
                f"{CATALOG}: {group['id']} says it completes in {group['note'] or 'a milestone'}, "
                "but every tool in it is Ready"
            )

    # --- a planned tool must not be a shipped feature ---------------------------------------
    shipped = shipped_features(root)
    for group in groups:
        for item in group["items"]:
            if item["kind"] == "ready":
                continue
            for feature in shipped.get(item["command"], []):
                out.append(
                    f"{CATALOG}: {item['label']} ({item['command']}) is marked {item['kind']} "
                    f"but {PARITY} records {feature} as shipped"
                )
    return out


def summary(root: pathlib.Path = ROOT) -> str:
    """One line per tool group, for people reading the check's output."""
    rows = []
    for group in catalogue(root):
        ready = sum(1 for item in group["items"] if item["kind"] == "ready")
        note = f" ({group['note']})" if group["note"] else ""
        rows.append(f"{group['id']:<18} {group['kind'] + note:<44} {ready} of {len(group['items'])} tools")
    return "\n".join(rows)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail when the documents disagree")
    parser.add_argument("--summary", action="store_true", help="print every tool group and its readiness")
    parser.add_argument("--root", default=str(ROOT), help="repository root (for tests)")
    args = parser.parse_args(argv)
    root = pathlib.Path(args.root).resolve()
    if args.summary:
        print(summary(root))
        return 0
    try:
        found = problems(root)
    except LookupError as error:
        print(f"disclosure: {error}", file=sys.stderr)
        return 2
    for problem in found:
        print(f"disclosure: {problem}", file=sys.stderr)
    if found:
        return 1
    if args.check:
        print("disclosure: README, tool catalogue, commands and Home cards agree")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
