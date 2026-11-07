#!/usr/bin/env python3
"""Generate/check the machine-readable capability manifest (REC-R12).

python3 docs/implementation/build_capabilities.py [--check] [--root ROOT]
Standard library only. The manifest is generated; never edit it by hand.

`docs/capabilities.toml` is the one file a support desk, a partner or a script can read to
learn what this build does and does not do yet:

* `[interface]` — every tool group the interface draws (`crates/engine/src/catalog.rs`), with
  each tool's availability (ready / planned + milestone / provider) and the group's counts;
* `[automation]` — the command-line/MCP tool table's schema version
  (`crates/automation/src/schema.rs`) and its size and safety split
  (`crates/automation/src/tools.rs`);
* `[[network_place]]` — every place the code can move bytes off the computer
  (`docs/network-egress.toml`).

Like `check_disclosure.py`, the generator parses the real sources and raises when the parse
stops matching them, so a checker that silently reads zero entries fails loudly instead of
passing forever. `--check` (run in CI) fails when the committed manifest has drifted from the
sources it is generated from.
"""
from __future__ import annotations

import argparse
from pathlib import Path
import re
import sys
import tomllib

import check_disclosure as disclosure

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent

CATALOG = "crates/engine/src/catalog.rs"
TOOLS = "crates/automation/src/tools.rs"
SCHEMA = "crates/automation/src/schema.rs"
EGRESS = "docs/network-egress.toml"
MANIFEST = "docs/capabilities.toml"

MANIFEST_VERSION = 1

# A tool entry is `t("name"` on one line, or `t(` with the name on the next line when the
# description is long enough that the call is wrapped. Matching only the first form silently
# dropped every wrapped tool from the published count.
_TOOL = re.compile(r'^        t\(\s*(?:\n\s*)?"([a-z0-9_]+)"', re.M)
_SCHEMA_VERSION = re.compile(r'pub const TOOL_SCHEMA_VERSION: &str = "([^"]+)"')
_TOOLS_FN = "pub fn tools() -> Vec<ToolDef>"


def esc(text: str) -> str:
    """Escape a TOML basic string (labels and notes are short; control chars are not expected)."""
    return text.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n").replace("\t", "\\t")


def read(root: Path, name: str) -> str:
    path = root / name
    if not path.is_file():
        raise LookupError(f"{name}: missing, so the manifest cannot be built")
    return path.read_text(encoding="utf-8")


def interface(root: Path) -> list[dict]:
    """The tool groups, in catalogue order, via the same parser the disclosure check uses."""
    return disclosure.catalogue(root)


def automation(root: Path) -> dict:
    """The tool table's version and size, parsed from the real files."""
    tools_text = read(root, TOOLS)
    marker = tools_text.find(_TOOLS_FN)
    if marker < 0:
        raise LookupError(f"{TOOLS}: {_TOOLS_FN!r} is gone; the parse is out of date")
    body = tools_text[marker:]
    names = _TOOL.findall(body)
    if len(names) < 100:
        raise LookupError(f"{TOOLS}: read {len(names)} tools; the parse is out of date")
    if len(set(names)) != len(names):
        raise LookupError(f"{TOOLS}: a tool name appears twice; the parse is out of date")
    schema_text = read(root, SCHEMA)
    match = _SCHEMA_VERSION.search(schema_text)
    if not match:
        raise LookupError(f"{SCHEMA}: TOOL_SCHEMA_VERSION is gone or written differently")
    version = match.group(1)
    major, _, minor = version.partition(".")
    if not (major.isdigit() and minor.isdigit()):
        raise LookupError(f"{SCHEMA}: {version!r} is not a major.minor version")
    # `.ro()` and `.destructive()` only appear inside the tool chains after `pub fn tools()`.
    read_only = body.count(".ro()")
    destructive = body.count(".destructive()")
    return {
        "schema_version": version,
        "tool_count": len(names),
        "read_only_count": read_only,
        "destructive_count": destructive,
        "tools": names,
    }


def network_places(root: Path) -> list[dict]:
    """Every egress entry: file, kind and what it does (the restraint text stays in the source)."""
    data = tomllib.loads(read(root, EGRESS))
    places = data.get("place", [])
    if len(places) < 3:
        raise LookupError(f"{EGRESS}: read {len(places)} places; the parse is out of date")
    out = []
    for place in places:
        for key in ("file", "kind", "what"):
            if not place.get(key):
                raise LookupError(f"{EGRESS}: a place without {key!r}")
        out.append({"file": place["file"], "kind": place["kind"], "what": place["what"]})
    return out


def build(root: Path = ROOT) -> str:
    groups = interface(root)
    auto = automation(root)
    places = network_places(root)
    tool_count = sum(len(group["items"]) for group in groups)
    ready_tools = sum(1 for group in groups for item in group["items"] if item["kind"] == "ready")
    lines = [
        "# Machine-readable capability manifest (REC-R12): what this build does and does not do yet.",
        "#",
        "# Generated by docs/implementation/build_capabilities.py from the sources named below —",
        "# never edit this file by hand. `python3 docs/implementation/build_capabilities.py --check`",
        "# fails when it drifts from them, and CI runs that check (see test_capabilities.py).",
        "#",
        "# availability is one of: ready (works in this build), planned (ships in `milestone`),",
        "# provider (`note`: needs a provider the user configures).",
        f"manifest_version = {MANIFEST_VERSION}",
        "generated_by = \"docs/implementation/build_capabilities.py\"",
        f"sources = [\"{CATALOG}\", \"{TOOLS}\", \"{SCHEMA}\", \"{EGRESS}\"]",
        "",
        "[interface]",
        f"group_count = {len(groups)}",
        f"tool_count = {tool_count}",
        f"ready_tool_count = {ready_tools}",
        "",
    ]
    for group in groups:
        ready = sum(1 for item in group["items"] if item["kind"] == "ready")
        lines += [
            "[[group]]",
            f'id = "{esc(group["id"])}"',
            f'label = "{esc(group["label"])}"',
            f'availability = "{group["kind"]}"',
            f'milestone = "{esc(group["note"]) if group["kind"] == "planned" else ""}"',
            f'note = "{esc(group["note"]) if group["kind"] == "provider" else ""}"',
            f"ready_tools = {ready}",
            f"tools = {len(group['items'])}",
            "",
        ]
        for item in group["items"]:
            lines += [
                "[[group.tool]]",
                f'label = "{esc(item["label"])}"',
                f'command = "{esc(item["command"])}"',
                f'availability = "{item["kind"]}"',
                f'milestone = "{esc(item["note"]) if item["kind"] == "planned" else ""}"',
                "",
            ]
    lines += [
        "[automation]",
        "# The command-line/MCP tool table: its schema version (docs/api-stability.md promises what",
        "# a version number means) and its size. read_only tools change nothing; destructive tools",
        "# can overwrite a file or remove content and need an approval under the project profile.",
        f'schema_version = "{auto["schema_version"]}"',
        f"tool_count = {auto['tool_count']}",
        f"read_only_count = {auto['read_only_count']}",
        f"destructive_count = {auto['destructive_count']}",
        'tools = [' + ", ".join(f'"{name}"' for name in auto["tools"]) + "]",
        "",
    ]
    for place in places:
        lines += [
            "[[network_place]]",
            f'file = "{esc(place["file"])}"',
            f'kind = "{esc(place["kind"])}"',
            f'what = "{esc(place["what"])}"',
            "",
        ]
    return "\n".join(lines).rstrip() + "\n"


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail when the committed manifest has drifted")
    parser.add_argument("--root", default=str(ROOT), help="repository root (for tests)")
    args = parser.parse_args(argv)
    root = Path(args.root).resolve()
    try:
        content = build(root)
    except LookupError as error:
        print(f"capabilities: {error}", file=sys.stderr)
        return 2
    path = root / MANIFEST
    if args.check:
        if not path.is_file() or path.read_text(encoding="utf-8") != content:
            print(f"Stale {MANIFEST}; run python3 docs/implementation/build_capabilities.py", file=sys.stderr)
            return 1
        print(f"capabilities: {MANIFEST} matches {CATALOG}, {TOOLS}, {SCHEMA} and {EGRESS}")
        return 0
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    groups = interface(root)
    print(
        f"Wrote {MANIFEST}: {len(groups)} interface groups, "
        f"{automation(root)['tool_count']} automation tools, {len(network_places(root))} network places"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
