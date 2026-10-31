#!/usr/bin/env python3
"""The README, the tool catalogue, the commands and the Home cards stay in step (REC-R12).

The first test is the gate CI runs: this checkout's documents must agree with each other. The
rest change one thing at a time in a throwaway copy of the real files, because a check that
silently passes is worse than no check.

The fixture copies the real `catalog.rs`, `commands.rs`, `home.rs`, parity inventory and README
rather than inventing small ones: an invented catalogue would let the parser pass while failing on
the catalogue the product actually ships.
"""

from __future__ import annotations

import pathlib
import shutil
import tempfile
import unittest

import check_disclosure as disclosure

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent.parent

FILES = (disclosure.CATALOG, disclosure.COMMANDS, disclosure.HOME, disclosure.PARITY, disclosure.README)


class Fixture:
    """A throwaway repository root holding a copy of the documents the check reads."""

    def __init__(self) -> None:
        self._dir = tempfile.TemporaryDirectory()
        self.root = pathlib.Path(self._dir.name)
        for name in FILES:
            target = self.root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)

    def edit(self, name: str, old: str, new: str) -> "Fixture":
        """Replace the one occurrence of `old` with `new` in `name`."""
        path = self.root / name
        text = path.read_text(encoding="utf-8")
        self.assertCount(text, old, 1, f"{name}: fixture text {old!r} is not there exactly once")
        path.write_text(text.replace(old, new), encoding="utf-8")
        return self

    def append(self, name: str, text: str) -> "Fixture":
        path = self.root / name
        path.write_text(path.read_text(encoding="utf-8") + text, encoding="utf-8")
        return self

    def problems(self) -> list[str]:
        try:
            return disclosure.problems(self.root)
        except LookupError as error:
            return [f"lookup: {error}"]

    @staticmethod
    def assertCount(text: str, needle: str, expected: int, message: str) -> None:
        found = text.count(needle)
        if found != expected:
            raise AssertionError(f"{message} (found {found})")

    def __del__(self) -> None:
        # Before the TemporaryDirectory's own finaliser, which would otherwise warn on GC.
        self._dir.cleanup()


def found(problems: list[str], *needles: str) -> bool:
    """Whether one message contains all of `needles`."""
    return any(all(needle in problem for needle in needles) for problem in problems)


class RealCheckout(unittest.TestCase):
    def test_the_documents_agree(self):
        problems = disclosure.problems(ROOT)
        self.assertEqual(problems, [], "\n".join(problems))

    def test_the_home_cards_are_tool_groups_in_the_documented_order(self):
        declared, cards = disclosure.recommended(ROOT)
        by_id = {group["id"]: group for group in disclosure.catalogue(ROOT)}
        self.assertEqual(declared, len(cards))
        self.assertEqual(len(set(cards)), len(cards))
        for card in cards:
            self.assertIn(card, by_id)
        # Redaction works, so the dashboard that recommends the core tools must offer it.
        self.assertIn("redact", cards)

    def test_the_summary_reports_every_group(self):
        rows = disclosure.summary(ROOT).splitlines()
        groups = disclosure.catalogue(ROOT)
        self.assertEqual(len(rows), len(groups))
        for row, group in zip(rows, groups):
            self.assertTrue(row.startswith(group["id"]), row)

    def test_check_and_summary_succeed_on_this_checkout(self):
        self.assertEqual(disclosure.main(["--check", "--root", str(ROOT)]), 0)
        self.assertEqual(disclosure.main(["--summary", "--root", str(ROOT)]), 0)


class HomeCards(unittest.TestCase):
    def test_a_card_the_catalogue_does_not_know_fails(self):
        fixture = Fixture().edit(disclosure.HOME, '"redact"]', '"redaction"]')
        self.assertTrue(found(fixture.problems(), "is not a tool group"), fixture.problems())

    def test_a_declared_length_that_lies_fails(self):
        fixture = Fixture().edit(disclosure.HOME, "[&str; 9]", "[&str; 8]")
        self.assertTrue(found(fixture.problems(), "declares 8 cards but lists 9"), fixture.problems())

    def test_a_card_missing_from_the_readme_fails(self):
        fixture = Fixture().edit(disclosure.HOME, '"protect", "redact"]', '"protect"]').edit(
            disclosure.HOME, "[&str; 9]", "[&str; 8]"
        )
        self.assertTrue(found(fixture.problems(), "must agree"), fixture.problems())

    def test_a_renamed_recommended_array_fails_loudly(self):
        fixture = Fixture().edit(disclosure.HOME, "RECOMMENDED: [&str; 9]", "CARDS: [&str; 9]")
        self.assertTrue(found(fixture.problems(), "lookup:"), fixture.problems())


class ReadmeCards(unittest.TestCase):
    def test_a_command_that_does_not_exist_fails(self):
        fixture = Fixture().edit(disclosure.README, "`optimize.reduce`", "`file.reduce_size`")
        self.assertTrue(found(fixture.problems(), "file.reduce_size", "is not a registered command"), fixture.problems())

    def test_a_command_from_another_group_fails(self):
        fixture = Fixture().edit(
            disclosure.README, "Reduce the file size, or use the advanced optimiser | `optimize.reduce`",
            "Reduce the file size, or use the advanced optimiser | `page.organize`",
        )
        self.assertTrue(found(fixture.problems(), "page.organize is not a tool in the compress group"), fixture.problems())

    def test_a_label_the_catalogue_does_not_use_fails(self):
        fixture = Fixture().edit(disclosure.README, "| **Organize pages** |", "| **Organise pages** |")
        self.assertTrue(found(fixture.problems(), "but crates/engine/src/catalog.rs calls it 'Organize pages'"), fixture.problems())

    def test_a_row_out_of_order_fails(self):
        fixture = Fixture().edit(
            disclosure.README,
            "| **Combine files** | `combine` |",
            "| **Compress a PDF** | `compress` |",
        )
        self.assertTrue(found(fixture.problems(), "the README and the card order must agree"), fixture.problems())

    def test_a_tool_group_missing_from_the_readme_fails(self):
        text = "| **Redact a PDF** | `redact` | Mark content and areas, redact whole pages or lists of words and phrases, and sanitise the document | `redact.mark` |\n"
        fixture = Fixture().edit(disclosure.README, text, "")
        self.assertTrue(found(fixture.problems(), "the README and the card order must agree"), fixture.problems())

    def test_the_open_card_still_has_to_name_a_command_that_runs(self):
        fixture = Fixture().edit(disclosure.README, "| `file.open` |", "| `file.open_now` |")
        self.assertTrue(found(fixture.problems(), "file.open_now", "is not a registered command"), fixture.problems())


class NotFinishedTable(unittest.TestCase):
    def test_a_count_that_does_not_match_the_catalogue_fails(self):
        fixture = Fixture().edit(disclosure.README, "| 13 of 14 tools |", "| 12 of 14 tools |")
        self.assertTrue(found(fixture.problems(), "says '12 of 14 tools'"), fixture.problems())

    def test_a_milestone_that_does_not_match_the_catalogue_fails(self):
        fixture = Fixture().edit(disclosure.README, "| 13 of 14 tools | M7 |", "| 13 of 14 tools | M9 |")
        self.assertTrue(found(fixture.problems(), "completes in 'M9'"), fixture.problems())

    def test_a_missing_group_fails(self):
        text = "| **AI assistant** (`ai`) | 0 of 3 tools | needs a provider you configure |\n"
        fixture = Fixture().edit(disclosure.README, text, "")
        self.assertTrue(found(fixture.problems(), "does not list ai"), fixture.problems())

    def test_a_finished_group_in_the_table_fails(self):
        text = "| **AI assistant** (`ai`) | 0 of 3 tools | needs a provider you configure |\n"
        fixture = Fixture().edit(
            disclosure.README, text, text + "| **Redact a PDF** (`redact`) | 5 of 5 tools | M9 |\n"
        )
        self.assertTrue(found(fixture.problems(), "redact", "is finished, so it does not belong"), fixture.problems())

    def test_a_group_that_finished_but_kept_its_milestone_fails(self):
        # Every tool in `edit` Ready while the group still says it completes in M7.
        fixture = Fixture().edit(
            disclosure.CATALOG,
            'item("Attach file", "paperclip", "edit.attach", Planned("M12"))',
            'item("Attach file", "paperclip", "edit.attach", Ready)',
        )
        self.assertTrue(found(fixture.problems(), "every tool in it is Ready"), fixture.problems())


class ShippedFeatures(unittest.TestCase):
    def test_a_planned_tool_recorded_as_shipped_fails(self):
        fixture = Fixture().append(
            disclosure.PARITY,
            '\n[[feature]]\nid = "test.shipped-while-planned"\ntitle = "Test"\narea = "E"\ntier = "P1"\n'
            'milestone = "M12"\nstatus = "shipped"\ncommands = ["edit.attach"]\ntools = []\nevidence = []\nnotes = ""\n',
        )
        self.assertTrue(
            found(fixture.problems(), "records test.shipped-while-planned as shipped"), fixture.problems()
        )


class MissingFiles(unittest.TestCase):
    def test_a_missing_readme_is_reported_not_ignored(self):
        fixture = Fixture()
        (fixture.root / disclosure.README).unlink()
        self.assertTrue(found(fixture.problems(), "lookup: README.md: missing"), fixture.problems())

    def test_a_rewritten_catalogue_that_does_not_parse_is_reported(self):
        fixture = Fixture().edit(disclosure.CATALOG, 'id: "redact"', 'identifier: "redact"')
        self.assertTrue(found(fixture.problems(), "lookup: crates/engine/src/catalog.rs"), fixture.problems())


if __name__ == "__main__":
    unittest.main()
