"""Standard-library consistency tests for the live implementation queue."""
import collections
import copy
import unittest
from unittest.mock import patch

import build_backlog as backlog


class BacklogTests(unittest.TestCase):
    def test_every_recommendation_gap_and_scope_review_is_retained(self):
        rows, progress = backlog.build()
        kinds = collections.Counter(r["kind"] for r in rows)
        self.assertEqual(kinds, {"recommendation": 222, "baseline gap": 384, "scope review": 23, "component": len(progress["components"])})
        self.assertEqual(len({r["id"] for r in rows}), len(rows))
        source = backlog.read_csv("baseline-features.csv")
        self.assertEqual({r["id"] for r in rows if r["kind"] == "baseline gap"},
                         {"GAP-" + r["feature_id"] for r in source if r["repo_status"] in {"partial", "planned"}})
        self.assertEqual({r["id"] for r in rows if r["kind"] == "scope review"},
                         {"SCOPE-" + r["feature_id"] for r in source if r["repo_status"] == "na"})
        self.assertTrue(all(r["stage"] in backlog.STAGES for r in rows))

    def test_current_stage_and_confirmation_gate(self):
        rows, progress = backlog.build()
        self.assertEqual(progress["workflow"]["order"], ["basic", "advanced", "future"])
        self.assertTrue(progress["workflow"]["confirmation_required_after_each_component"])
        self.assertFalse(progress["workflow"]["auto_advance"])
        active = progress["workflow"]["active_component"]
        self.assertEqual(progress["components"][active]["stage"], progress["workflow"]["active_stage"])
        self.assertTrue(all(r["status"] for r in rows))

    def test_optional_service_and_mobile_safety_stay_with_their_surface(self):
        rows, _ = backlog.build()
        stages = {r["id"]: r["stage"] for r in rows}
        self.assertEqual(stages["REC-H11"], "advanced")
        self.assertEqual(stages["REC-H04"], "basic")
        self.assertEqual(stages["REC-B06"], "basic")  # Existing CLI resume/checkpoint work.
        for number in range(1, 11):
            self.assertEqual(stages[f"REC-M{number:02d}"], "future")

    def test_cannot_silently_bypass_confirmation(self):
        _, original = backlog.build()
        changed = copy.deepcopy(original)
        changed["workflow"]["auto_advance"] = True
        with patch.object(backlog.tomllib, "loads", return_value=changed):
            with self.assertRaisesRegex(ValueError, "confirmation gate"):
                backlog.build()

    def test_cannot_advance_past_unfinished_basics(self):
        _, original = backlog.build()
        changed = copy.deepcopy(original)
        changed["workflow"]["active_stage"] = "advanced"
        with patch.object(backlog.tomllib, "loads", return_value=changed):
            with self.assertRaisesRegex(ValueError, "earlier stages"):
                backlog.build()

    def test_cannot_mark_component_complete_without_ci(self):
        _, original = backlog.build()
        changed = copy.deepcopy(original)
        changed["components"]["B001"]["status"] = "complete"
        changed["components"]["B001"]["ci"] = "pending"
        with patch.object(backlog.tomllib, "loads", return_value=changed):
            with self.assertRaisesRegex(ValueError, "passing CI"):
                backlog.build()

    def test_generated_files_are_current_and_csv_does_not_execute_formulas(self):
        rows, progress = backlog.build()
        self.assertEqual((backlog.HERE / "TODO.md").read_text(), backlog.markdown(rows, progress))
        self.assertEqual((backlog.HERE / "TODO.csv").read_text(), backlog.csv_text(rows))
        text = backlog.csv_text([{"id": "=2+2", "title": "@untrusted"}])
        self.assertIn("'=2+2", text)
        self.assertIn("'@untrusted", text)

    def test_documentation_links_resolve(self):
        import re
        for path in backlog.HERE.glob("*.md"):
            for target in re.findall(r"\]\(([^)]+)\)", path.read_text()):
                if target.startswith(("https://", "http://", "#")):
                    continue
                self.assertTrue((path.parent / target.split("#")[0]).exists(), (path, target))


if __name__ == "__main__":
    unittest.main()
