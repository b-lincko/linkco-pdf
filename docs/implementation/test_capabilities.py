#!/usr/bin/env python3
"""The capability manifest stays in step with the sources it is generated from (REC-R12).

The first test is the gate CI runs: the committed `docs/capabilities.toml` must be exactly
what the generator produces from this checkout's real sources. The rest change one thing at a
time in a throwaway copy of those sources, because a check that silently passes is worse than
no check.

The fixture copies the real `catalog.rs`, `tools.rs`, `schema.rs`, `network-egress.toml` and
the committed manifest rather than inventing small ones: an invented catalogue would let the
generator pass while failing on the sources the product actually ships.
"""

from __future__ import annotations

import pathlib
import shutil
import tempfile
import unittest

import build_capabilities as caps

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent.parent

FILES = (caps.CATALOG, caps.TOOLS, caps.SCHEMA, caps.EGRESS, caps.MANIFEST)


class Fixture:
    """A throwaway repository root holding a copy of the files the generator reads."""

    def __init__(self) -> None:
        self._dir = tempfile.TemporaryDirectory()
        self.root = pathlib.Path(self._dir.name)
        for name in FILES:
            target = self.root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        self.committed = (self.root / caps.MANIFEST).read_text(encoding="utf-8")

    def __enter__(self) -> "Fixture":
        return self

    def __exit__(self, *exc: object) -> None:
        self._dir.cleanup()

    def edit(self, name: str, old: str, new: str) -> "Fixture":
        """Replace the one occurrence of `old` with `new` in `name`."""
        path = self.root / name
        text = path.read_text(encoding="utf-8")
        found = text.count(old)
        if found != 1:
            raise AssertionError(f"{name}: fixture text {old!r} occurs {found} times, not once")
        path.write_text(text.replace(old, new), encoding="utf-8")
        return self

    def build(self) -> str:
        try:
            return caps.build(self.root)
        except LookupError as error:
            return f"lookup: {error}"

    def drifted(self) -> bool:
        """Would `--check` fail on this copy?"""
        return self.build() != self.committed


class GateTest(unittest.TestCase):
    def test_committed_manifest_matches_the_real_sources(self) -> None:
        self.assertEqual(caps.build(ROOT), (ROOT / caps.MANIFEST).read_text(encoding="utf-8"))

    def test_check_passes_on_this_checkout(self) -> None:
        self.assertEqual(caps.main(["--check", "--root", str(ROOT)]), 0)

    def test_manifest_is_valid_toml_with_the_expected_sections(self) -> None:
        import tomllib

        data = tomllib.loads((ROOT / caps.MANIFEST).read_text(encoding="utf-8"))
        self.assertEqual(data["manifest_version"], caps.MANIFEST_VERSION)
        self.assertIn("interface", data)
        self.assertIn("automation", data)
        self.assertGreaterEqual(len(data["group"]), 20)
        self.assertGreaterEqual(len(data["network_place"]), 3)
        self.assertEqual(len(data["automation"]["tools"]), data["automation"]["tool_count"])
        self.assertEqual(
            sum(len(group["tool"]) for group in data["group"]),
            data["interface"]["tool_count"],
        )


class DriftTest(unittest.TestCase):
    def test_a_catalogue_item_changing_availability_is_caught(self) -> None:
        with Fixture() as fix:
            fix.edit(caps.CATALOG, 'item("Spreadsheet (.xlsx)", "grid-3x3", "export.xlsx", Planned("M10"))',
                     'item("Spreadsheet (.xlsx)", "grid-3x3", "export.xlsx", Ready)')
            out = fix.build()
            self.assertNotEqual(out, fix.committed)
            self.assertIn('command = "export.xlsx"\navailability = "ready"', out)

    def test_a_new_automation_tool_is_caught(self) -> None:
        with Fixture() as fix:
            fix.edit(caps.TOOLS, '        t("doc_open", "Open a PDF"',
                     '        t("zzz_probe", "Probe", "A tool the fixture added.")\n            .ro().with(schema(json!({}), &[])),\n        t("doc_open", "Open a PDF"')
            out = fix.build()
            self.assertNotEqual(out, fix.committed)
            self.assertIn('"zzz_probe"', out)
            self.assertIn("tool_count = 128", out)

    def test_a_schema_version_change_is_caught(self) -> None:
        with Fixture() as fix:
            fix.edit(caps.SCHEMA, 'pub const TOOL_SCHEMA_VERSION: &str = "1.4";',
                     'pub const TOOL_SCHEMA_VERSION: &str = "2.0";')
            out = fix.build()
            self.assertNotEqual(out, fix.committed)
            self.assertIn('schema_version = "2.0"', out)

    def test_a_removed_network_place_is_caught(self) -> None:
        with Fixture() as fix:
            text = (fix.root / caps.EGRESS).read_text(encoding="utf-8")
            start = text.index("[[place]]")
            end = text.index("[[place]]", start + 1)
            (fix.root / caps.EGRESS).write_text(text[:start] + text[end:], encoding="utf-8")
            out = fix.build()
            self.assertNotEqual(out, fix.committed)
            self.assertNotIn("apps/pdfcraft/src/updates.rs", out)

    def test_a_stale_parse_fails_loudly_instead_of_passing(self) -> None:
        with Fixture() as fix:
            text = (fix.root / caps.CATALOG).read_text(encoding="utf-8")
            (fix.root / caps.CATALOG).write_text(text.replace("    ToolGroup {", "    SomethingElse {"), encoding="utf-8")
            self.assertTrue(fix.build().startswith("lookup:"), "a gutted catalogue must not produce a manifest")

    def test_check_fails_on_a_drifted_copy_and_passes_on_the_real_root(self) -> None:
        with Fixture() as fix:
            fix.edit(caps.SCHEMA, 'pub const TOOL_SCHEMA_VERSION: &str = "1.4";',
                     'pub const TOOL_SCHEMA_VERSION: &str = "9.9";')
            self.assertEqual(caps.main(["--check", "--root", str(fix.root)]), 1)
        self.assertEqual(caps.main(["--check", "--root", str(ROOT)]), 0)


if __name__ == "__main__":
    unittest.main()
