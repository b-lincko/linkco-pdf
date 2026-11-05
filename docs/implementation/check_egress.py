#!/usr/bin/env python3
"""Check that every networking path in the product is accounted for (REC-S09, REC-S03).

`docs/network-egress.toml` lists every place the editor or the command line can move bytes
off the computer, or hand an address to another program, together with what limits it. This
script keeps that list honest:

* a source file that gains network code and is not listed fails the check;
* a listed file that no longer has the code its entry names fails the check;
* a crate that gains an HTTP client without an entry under `[[dependency]]` fails the check;
* the update check must be reachable only through the menu command (`[[place]]` says so, and
  the wiring rule below enforces it).

Run it directly (`python3 docs/implementation/check_egress.py --check`) or through the unit
tests in `test_egress.py`, which CI already runs.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import sys
import tomllib

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent.parent

INVENTORY = "docs/network-egress.toml"

# What counts as a way off the machine, or into another program.
TOKENS = (
    "ureq::",
    "reqwest::",
    "hyper::",
    "TcpStream",
    "TcpListener",
    "UdpSocket",
    "webbrowser",
    "egui::OpenUrl",
    "OpenUrl::",
    "ShellExecute",
    "std::net::",
)

# Crates that speak HTTP (or hand a URL to a program that will).
HTTP_CLIENTS = frozenset(
    {"ureq", "reqwest", "hyper", "hyper-util", "curl", "isahc", "surf", "attohttpc", "minreq"}
)

# The kinds `[[place]]` may use. Each says what the user has to do for it to run.
KINDS = {
    "browser": "an address handed to the browser, which the user asked for",
    "loopback": "this computer only (127.0.0.1), not off the machine",
    "update-check": "an HTTPS request made only when the user asks for one",
    "no-socket": "no connection at all: the file only mentions a network type",
}

# Where the product's own code lives; tests are part of it, so they are scanned too.
SOURCE_GLOBS = ("crates/*/src", "apps/*/src")

_WHOLE_LINE_COMMENT = re.compile(r"^\s*//")


def code_only(text: str) -> str:
    """`text` with whole-line `//` comments removed.

    Comments talk *about* the network ("the webbrowser crate would open…"), which is not a
    path off the machine. Only whole-line comments are removed: a trailing comment is left in
    place, so a missed path costs a false alarm rather than a silent gap.
    """
    return "\n".join(line for line in text.splitlines() if not _WHOLE_LINE_COMMENT.match(line))


def scanned_files(root: pathlib.Path) -> list[pathlib.Path]:
    files: list[pathlib.Path] = []
    for pattern in SOURCE_GLOBS:
        for folder in sorted(root.glob(pattern)):
            files.extend(sorted(folder.rglob("*.rs")))
    return files


def network_code(root: pathlib.Path) -> dict[str, set[str]]:
    """Every source file with network code, and which of [`TOKENS`] it contains."""
    found: dict[str, set[str]] = {}
    for path in scanned_files(root):
        text = code_only(path.read_text(encoding="utf-8", errors="replace"))
        hits = {token for token in TOKENS if token in text}
        if hits:
            found[path.relative_to(root).as_posix()] = hits
    return found


def cargo_dependencies(root: pathlib.Path) -> dict[str, list[str]]:
    """HTTP-client dependencies per product crate: `crate` → Cargo.toml paths that list it."""
    found: dict[str, list[str]] = {}
    for manifest in sorted(root.glob("crates/*/Cargo.toml")) + sorted(root.glob("apps/*/Cargo.toml")):
        data = tomllib.loads(manifest.read_text(encoding="utf-8"))
        for table, name in dependency_tables(data):
            if name in HTTP_CLIENTS:
                found.setdefault(name, []).append(f"{manifest.relative_to(root).as_posix()} [{table}]")
    return found


def dependency_tables(data: dict, prefix: str = "") -> list[tuple[str, str]]:
    """Every `(table, crate)` under a `dependencies` table, target-specific ones included.

    `dev-dependencies` and `build-dependencies` are left out: a test client or a build script
    is not something the shipped editor does.
    """
    pairs: list[tuple[str, str]] = []
    for key, value in data.items():
        if isinstance(value, dict):
            where = f"{prefix}{key}"
            if where.endswith("dependencies") and where.split(".")[-1] == "dependencies":
                pairs.extend((where, name) for name in value)
            else:
                pairs.extend(dependency_tables(value, f"{where}."))
    return pairs


def load_inventory(root: pathlib.Path) -> dict:
    return tomllib.loads((root / INVENTORY).read_text(encoding="utf-8"))


def problems(root: pathlib.Path = ROOT) -> list[str]:
    """Everything wrong with the inventory, as sentences a person can act on."""
    inventory = load_inventory(root)
    found = network_code(root)
    places = inventory.get("place", [])
    out: list[str] = []

    listed: dict[str, dict] = {}
    for place in places:
        file = place.get("file", "")
        if not file:
            out.append("[[place]] without a file")
            continue
        if file in listed:
            out.append(f"{file}: listed twice")
            continue
        listed[file] = place
        kind = place.get("kind")
        if kind not in KINDS:
            out.append(f"{file}: kind {kind!r} is not one of {', '.join(sorted(KINDS))}")
        for field in ("what", "restraint"):
            if not str(place.get(field, "")).strip():
                out.append(f"{file}: {field} must say something")
        for token in place.get("tokens", []):
            if token not in found.get(file, set()):
                out.append(f"{file}: no longer contains {token!r} that its entry claims")

    for file in sorted(found):
        if file not in listed:
            tokens = ", ".join(sorted(found[file]))
            out.append(f"{file}: uses {tokens} but is not listed in {INVENTORY}")
    for file in sorted(listed):
        if file not in found:
            out.append(f"{file}: listed in {INVENTORY} but has no network code any more")

    dependencies = cargo_dependencies(root)
    declared: dict[str, dict] = {}
    for entry in inventory.get("dependency", []):
        name = entry.get("crate", "")
        if not name:
            out.append("[[dependency]] without a crate")
            continue
        if name in declared:
            out.append(f"[[dependency]] {name}: listed twice")
        declared[name] = entry
        for field in ("what", "reason"):
            if not str(entry.get(field, "")).strip():
                out.append(f"[[dependency]] {name}: {field} must say something")
        if name not in dependencies:
            out.append(f"[[dependency]] {name}: no product crate depends on it any more")
    for name in sorted(dependencies):
        if name not in declared:
            out.append(f"[[dependency]] {name}: used by {', '.join(dependencies[name])} but not listed in {INVENTORY}")

    out.extend(update_check_problems(root))
    return out


def update_check_problems(root: pathlib.Path) -> list[str]:
    """The update check may only be reachable through the command the user runs.

    `apps/pdfcraft/src/main.rs` installs the source the dialog calls. It must mention the
    network function exactly once, on the line that installs it: a second mention would mean
    something calls it at startup, which is the one thing the app promises not to do.
    """
    out: list[str] = []
    main = root / "apps/pdfcraft/src/main.rs"
    if not main.is_file():
        return [f"{main.relative_to(root).as_posix()}: missing, so the update wiring cannot be checked"]
    lines = [line for line in code_only(main.read_text(encoding="utf-8")).splitlines() if "updates::latest_release" in line]
    if len(lines) != 1:
        out.append(
            "apps/pdfcraft/src/main.rs: `updates::latest_release` must appear exactly once "
            f"(found {len(lines)}); the update check is only for the menu command"
        )
    elif "update_source" not in lines[0]:
        out.append(
            "apps/pdfcraft/src/main.rs: `updates::latest_release` is not installed as the "
            "update source, so something else calls it"
        )
    return out


def summary(root: pathlib.Path = ROOT) -> str:
    """One line per listed place, for the document's generated table and for people."""
    inventory = load_inventory(root)
    rows = []
    for place in inventory.get("place", []):
        rows.append(f"| `{place.get('file')}` | {place.get('kind')} | {place.get('what')} |")
    return "\n".join(rows)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="fail when the inventory is out of date")
    parser.add_argument("--root", default=str(ROOT), help="repository root (default: this checkout)")
    parser.add_argument("--summary", action="store_true", help="print the inventory as a table")
    args = parser.parse_args(argv)
    root = pathlib.Path(args.root).resolve()
    if args.summary:
        print(summary(root))
        return 0
    found = problems(root)
    if found:
        for problem in found:
            print(f"network egress: {problem}", file=sys.stderr)
        return 1
    places = len(load_inventory(root).get("place", []))
    print(f"Network egress: {places} places accounted for in {INVENTORY}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
