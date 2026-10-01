#!/usr/bin/env python3
"""Regressions for the deliberately bounded documentation contract."""

import sys
import tempfile
from pathlib import Path
import unittest

sys.dont_write_bytecode = True
import documentation_checks as docs


class DocumentationTests(unittest.TestCase):
    def test_actual_surfaces_have_explicit_example_consumers(self):
        examples = docs.inspect_documents(docs.ROOT)
        self.assertEqual(examples["README.md"][0][0], "rust,no_run")
        self.assertEqual(examples["docs/getting-started.md"][0][0], "rust")

    def test_missing_extra_ignored_or_compile_only_guide_examples_fail(self):
        original = (docs.ROOT / "docs/getting-started.md").read_text()
        for text in (original.replace("```rust", "```rust,ignore"),
                     original.replace("```rust", "```rust,no_run"),
                     original + "\n```rust\nassert!(true);\n```\n",
                     original[:original.index("```rust")] + "\nEnd.\n"):
            with self.subTest(text=text[:30]), self.assertRaises(docs.DocumentationError):
                _, examples = docs.prose_and_examples(text, strict=True)
                docs.select_examples("docs/getting-started.md", examples)

    def test_local_links_anchors_and_same_repo_urls_use_current_source(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "README.md").write_text("# Get started\n## Get started\n")
            for target in ("README.md#get-started", "README.md#get-started-1", docs.SAME_REPO + "README.md#get-started"):
                docs.check_target(root, "README.md", target)
            for target in ("README.md#missing", "missing.md", "../outside.md", docs.SAME_REPO + "missing.md"):
                with self.subTest(target=target), self.assertRaises(docs.DocumentationError):
                    docs.check_target(root, "README.md", target)

    def test_reference_definitions_are_checked(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "README.md").write_text("# Guide\n")
            docs.check_references(root, "README.md", "[guide][ref]\n[ref]: README.md#guide")
            for text in ("[guide][missing]", "[ref]: missing.md", "[bad](README.md#missing)", "[guide](README.md \"title\")", "![image](image.png)"):
                with self.subTest(text=text), self.assertRaises(docs.DocumentationError):
                    docs.check_references(root, "README.md", text)

    def test_unsupported_fences_and_html_fail(self):
        for text in ("```rust,ignore\n1\n```", "~~~rust\n1\n~~~", "    indented code", "<a href='x'>link</a>", "```rust\n1"):
            with self.subTest(text=text), self.assertRaises(docs.DocumentationError):
                docs.prose_and_examples(text, strict=True)

    def test_each_required_doctest_consumer_has_coverage(self):
        output = "".join(f"Doc-tests {name.replace('-', '_')}\n"
                         f"test result: ok. {1 if name in docs.PACKAGES[:2] else 0} passed; 0 failed; 0 ignored\n"
                         for name in docs.PACKAGES)
        docs.check_doctest_coverage(output)
        for broken in (output.replace("1 passed", "0 passed", 1),
                       output.replace("0 ignored", "1 ignored", 1),
                       output.replace("Doc-tests emuella_j2k\n", "Doc-tests unrelated\n")):
            with self.assertRaises(docs.DocumentationError):
                docs.check_doctest_coverage(broken)


if __name__ == "__main__":
    unittest.main()
