#!/usr/bin/env python3
"""Regression probes for the C ABI unsafe inventory gate."""
import importlib.util
import json
from pathlib import Path
import unittest

ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location("audit", ROOT / "scripts/audit-c-api-unsafe.py")
audit = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(audit)


class UnsafeInventoryTests(unittest.TestCase):
    def setUp(self):
        self.source = (ROOT / "crates/emuella-j2k-capi/src/lib.rs").read_text()
        self.expected = json.loads((ROOT / "scripts/c-api-unsafe-inventory.json").read_text())

    def test_reviewed_inventory(self):
        audit.audit(self.source, self.expected)

    def test_new_operation_before_and_after_decode_test_hook(self):
        for anchor in ("unsafe { pointer.read() }", "unsafe { checked_copy(destination, &error.message) }"):
            with self.subTest(anchor=anchor):
                self.assertIn(anchor, self.source)
                changed = self.source.replace(anchor, anchor + "; unsafe { pointer.read_volatile() }", 1)
                with self.assertRaises(ValueError):
                    audit.audit(changed, self.expected)

    def test_addition_inside_existing_delegation_block(self):
        anchor = "unsafe { checked_read(source, \"source\") }"
        self.assertIn(anchor, self.source)
        changed = self.source.replace(anchor, 'unsafe { pointer.read_volatile(); checked_read(source, "source") }', 1)
        with self.assertRaises(ValueError):
            audit.audit(changed, self.expected)

    def test_nested_block_requires_review(self):
        changed = self.source.replace("unsafe { pointer.read() }", "unsafe { { pointer.read() } }", 1)
        with self.assertRaises(ValueError):
            audit.audit(changed, self.expected)


if __name__ == "__main__":
    unittest.main()
