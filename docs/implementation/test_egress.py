#!/usr/bin/env python3
"""The network-egress inventory stays true (REC-S09, REC-S03).

The first test is the gate CI runs: the real checkout must have every networking path listed.
The rest build small trees to prove that each way of drifting is caught, because a check that
silently passes is worse than no check.
"""

from __future__ import annotations

import pathlib
import tempfile
import unittest

import check_egress as egress

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent.parent

# A place that is listed correctly, for fixtures to vary one thing at a time.
LISTED = """
[[place]]
file = "crates/alpha/src/net.rs"
kind = "loopback"
tokens = ["TcpListener"]
what = "A listener."
restraint = "Binds 127.0.0.1 only."

[[dependency]]
crate = "ureq"
where = "apps/beta/Cargo.toml"
what = "An HTTPS client."
reason = "Only when the user asks."
"""

LOOPBACK_RS = """
pub fn serve() {
    let _ = std::net::TcpListener::bind(("127.0.0.1", 0));
}
"""


class Fixture:
    """A throwaway repository root with the files a test needs."""

    def __init__(self) -> None:
        self._dir = tempfile.TemporaryDirectory()
        self.root = pathlib.Path(self._dir.name)

    def write(self, path: str, text: str) -> pathlib.Path:
        target = self.root / path
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text(text, encoding="utf-8")
        return target

    def standard(self, inventory: str = LISTED) -> "Fixture":
        self.write("crates/alpha/src/net.rs", LOOPBACK_RS)
        self.write("apps/beta/src/main.rs", "fn main() {}\n")
        self.write("apps/beta/Cargo.toml", '[package]\nname = "beta"\n\n[dependencies]\nureq = "3"\n')
        self.write("apps/pdfcraft/src/main.rs", 'fn main() {\n    app.update_source = Some(std::sync::Arc::new(updates::latest_release));\n}\n')
        self.write(egress.INVENTORY, inventory)
        return self

    def problems(self) -> list[str]:
        return egress.problems(self.root)

    def __del__(self) -> None:
        # Before the TemporaryDirectory's own finaliser, which would otherwise warn on GC.
        self._dir.cleanup()


class RealCheckout(unittest.TestCase):
    def test_every_networking_path_in_this_checkout_is_inventoried(self):
        problems = egress.problems(ROOT)
        self.assertEqual(problems, [], "\n".join(problems))

    def test_the_inventory_names_real_files(self):
        inventory = egress.load_inventory(ROOT)
        self.assertTrue(inventory.get("place"), "the inventory lists no places")
        for place in inventory["place"]:
            self.assertTrue((ROOT / place["file"]).is_file(), place["file"])
            self.assertIn(place["kind"], egress.KINDS)

    def test_the_summary_is_a_table_row_per_place(self):
        rows = egress.summary(ROOT).splitlines()
        self.assertEqual(len(rows), len(egress.load_inventory(ROOT)["place"]))
        for row in rows:
            self.assertTrue(row.startswith("| `") and row.endswith("|"), row)


class Drift(unittest.TestCase):
    def test_a_new_networking_file_must_be_listed(self):
        fixture = Fixture().standard()
        fixture.write("crates/alpha/src/upload.rs", 'fn send() { let _ = reqwest::get("https://example.org"); }\n')
        found = fixture.problems()
        self.assertTrue(any("upload.rs" in p and "not listed" in p for p in found), found)

    def test_a_place_that_lost_its_code_fails(self):
        fixture = Fixture().standard()
        fixture.write("crates/alpha/src/net.rs", "pub fn serve() {}\n")
        self.assertTrue(any("no network code any more" in p for p in fixture.problems()), fixture.problems())

    def test_a_place_that_lost_one_of_its_tokens_fails(self):
        inventory = LISTED.replace('tokens = ["TcpListener"]', 'tokens = ["TcpListener", "UdpSocket"]')
        fixture = Fixture().standard(inventory)
        self.assertTrue(any("UdpSocket" in p for p in fixture.problems()), fixture.problems())

    def test_a_new_http_client_must_be_explained(self):
        fixture = Fixture().standard()
        fixture.write("crates/alpha/Cargo.toml", '[package]\nname = "alpha"\n\n[dependencies]\nreqwest = "0.12"\n')
        self.assertTrue(any("reqwest" in p for p in fixture.problems()), fixture.problems())

    def test_an_http_client_that_is_gone_fails(self):
        inventory = LISTED.replace('crate = "ureq"', 'crate = "minreq"')
        fixture = Fixture().standard(inventory)
        self.assertTrue(any("minreq" in p for p in fixture.problems()), fixture.problems())

    def test_a_startup_update_check_fails(self):
        fixture = Fixture().standard()
        fixture.write(
            "apps/pdfcraft/src/main.rs",
            "fn main() {\n    app.update_source = Some(std::sync::Arc::new(updates::latest_release));\n    let _ = updates::latest_release();\n}\n",
        )
        self.assertTrue(any("exactly once" in p for p in fixture.problems()), fixture.problems())

    def test_an_update_check_not_installed_as_the_source_fails(self):
        fixture = Fixture().standard()
        fixture.write("apps/pdfcraft/src/main.rs", "fn main() {\n    let _ = updates::latest_release;\n}\n")
        self.assertTrue(any("not installed as the update source" in p for p in fixture.problems()), fixture.problems())

    def test_a_dev_dependency_is_not_a_shipped_client(self):
        fixture = Fixture().standard()
        fixture.write("crates/alpha/Cargo.toml", '[package]\nname = "alpha"\n\n[dev-dependencies]\nreqwest = "0.12"\n')
        self.assertEqual(fixture.problems(), [])


class Entries(unittest.TestCase):
    def test_an_unknown_kind_fails(self):
        inventory = LISTED.replace('kind = "loopback"', 'kind = "sometimes"')
        fixture = Fixture().standard(inventory)
        self.assertTrue(any("sometimes" in p for p in fixture.problems()), fixture.problems())

    def test_an_entry_has_to_say_what_limits_it(self):
        inventory = LISTED.replace('restraint = "Binds 127.0.0.1 only."', 'restraint = "  "')
        fixture = Fixture().standard(inventory)
        self.assertTrue(any("restraint must say something" in p for p in fixture.problems()), fixture.problems())

    def test_a_place_listed_twice_fails(self):
        inventory = LISTED + LISTED.split("[[dependency]]")[0]
        fixture = Fixture().standard(inventory)
        self.assertTrue(any("listed twice" in p for p in fixture.problems()), fixture.problems())

    def test_a_dependency_entry_has_to_say_why(self):
        inventory = LISTED.replace('reason = "Only when the user asks."', 'reason = ""')
        fixture = Fixture().standard(inventory)
        self.assertTrue(any("reason must say something" in p for p in fixture.problems()), fixture.problems())


class Comments(unittest.TestCase):
    def test_a_comment_about_the_network_is_not_a_path_off_the_machine(self):
        fixture = Fixture().standard()
        fixture.write(
            "crates/alpha/src/docs_only.rs",
            "// The desktop opens addresses through the webbrowser crate.\n/// Uses no ureq:: here.\n//! Nor reqwest::.\n",
        )
        self.assertEqual(fixture.problems(), [], fixture.problems())

    def test_a_trailing_comment_still_counts(self):
        # Deliberate: a false alarm costs a line in the inventory, a missed path costs privacy.
        fixture = Fixture().standard()
        fixture.write("crates/alpha/src/trailing.rs", "pub fn f() {} // uses ureq:: elsewhere\n")
        self.assertTrue(any("trailing.rs" in p for p in fixture.problems()), fixture.problems())


class Command(unittest.TestCase):
    def test_check_succeeds_on_this_checkout(self):
        self.assertEqual(egress.main(["--check", "--root", str(ROOT)]), 0)

    def test_summary_prints_every_place(self):
        self.assertEqual(egress.main(["--summary", "--root", str(ROOT)]), 0)


if __name__ == "__main__":
    unittest.main()
