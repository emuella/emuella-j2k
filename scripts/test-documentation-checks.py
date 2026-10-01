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
        self.assertEqual(examples["docs/caller-owned-output.md"][0][0], "rust")
        self.assertEqual(examples["docs/error-handling.md"][0][0], "rust")

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
        docs.prose_and_examples("Use `Vec<u8>` and `Result<T>`.", strict=True)

    def test_each_selected_surface_rejects_missing_extra_or_ignored_examples(self):
        for name, kinds in docs.SURFACES.items():
            original = (docs.ROOT / name).read_text()
            mutations = [original + "\n```rust\nassert!(true);\n```\n"]
            if kinds:
                mutations.extend([original.replace("```" + kinds[0], "```rust,ignore", 1),
                                  original[:original.index("```" + kinds[0])] + "\nEnd.\n"])
            for text in mutations:
                with self.subTest(surface=name), self.assertRaises(docs.DocumentationError):
                    _, examples = docs.prose_and_examples(text, strict=True)
                    docs.select_examples(name, examples)

    def test_all_blocks_are_wired_to_distinct_consumers(self):
        with tempfile.TemporaryDirectory() as directory:
            harness = Path(directory)
            (harness / "src").mkdir()
            documents = {"selected.md": [("rust", "assert!(true);"),
                                         ("rust", "assert!(false);")],
                         "external.md": [("rust,no_run", "let _ = std::fs::read(\"external\");")]}
            self.assertEqual(docs.write_example_consumers(harness, documents), 3)
            source = (harness / "src/lib.rs").read_text()
            for index in range(3):
                self.assertIn(f'include_str!("example{index}.md")', source)
            self.assertIn("assert!(false);", (harness / "src/example1.md").read_text())

    def test_markdown_results_reject_missing_failed_ignored_zero_or_unaccounted_tests(self):
        valid = "test result: ok. 4 passed; 0 failed; 0 ignored"
        docs.check_example_coverage(valid, 4)
        for broken in ("", valid.replace("4 passed", "0 passed"),
                       valid.replace("4 passed", "3 passed"),
                       valid.replace("4 passed", "5 passed"),
                       valid.replace("0 failed", "1 failed"),
                       valid.replace("0 ignored", "1 ignored"), valid + "\n" + valid):
            with self.subTest(output=broken), self.assertRaises(docs.DocumentationError):
                docs.check_example_coverage(broken, 4)

    def test_each_required_doctest_consumer_has_coverage(self):
        output = "".join(f"Doc-tests {name.replace('-', '_')}\n"
                         f"test result: ok. {1 if name in docs.PACKAGES[:2] else 0} passed; 0 failed; 0 ignored\n"
                         for name in docs.PACKAGES)
        docs.check_doctest_coverage(output)
        def colour(text):
            return text.replace("Doc-tests ", "\x1b[1;32mDoc-tests\x1b[0m ").replace(
                "\ntest result:", "\x1b[0m\n\x1b[1mtest result:")

        docs.check_doctest_coverage(colour(output))
        for broken in (output.replace("1 passed", "0 passed", 1),
                       output.replace("0 failed", "1 failed", 1),
                       output.replace("0 ignored", "1 ignored", 1),
                       output.replace("Doc-tests emuella_j2k\n", "Doc-tests unrelated\n")):
            for coloured in (False, True):
                with self.subTest(colour=coloured), self.assertRaises(docs.DocumentationError):
                    docs.check_doctest_coverage(colour(broken) if coloured else broken)


if __name__ == "__main__":
    unittest.main()
