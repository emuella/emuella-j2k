#!/usr/bin/env python3
"""Regression tests for the fail-closed public-file policy."""

from __future__ import annotations

import sys
import unittest
from pathlib import PurePosixPath

sys.dont_write_bytecode = True

from public_tree_policy import (  # noqa: E402
    contains_public_openjph_identifier,
    content_policy_errors,
    exception_configuration_errors,
    sha256_bytes,
)


class PublicTreePolicyTests(unittest.TestCase):
    def test_public_openjph_identifier_is_rejected(self) -> None:
        self.assertTrue(
            contains_public_openjph_identifier("pub openjph_diagnostic: Option<u32>,")
        )
        self.assertTrue(
            contains_public_openjph_identifier(
                'pub async unsafe extern "C" fn openjph_comparator() {}'
            )
        )

    def test_private_provenance_identifier_is_allowed(self) -> None:
        self.assertFalse(
            contains_public_openjph_identifier(
                "pub(super) fn openjph_reversible_transfer() {}"
            )
        )

    def test_approved_utf8_source_is_allowed(self) -> None:
        self.assertEqual(
            content_policy_errors(PurePosixPath("src/lib.rs"), b"pub fn ok() {}\n"),
            [],
        )

    def test_reviewed_legal_basename_is_allowed(self) -> None:
        self.assertEqual(
            content_policy_errors(PurePosixPath("LICENSE-APACHE-2.0"), b"text\n"),
            [],
        )

    def test_unknown_text_suffix_is_rejected(self) -> None:
        errors = content_policy_errors(PurePosixPath("unreviewed.xyz"), b"text\n")
        self.assertIn("unreviewed file type .xyz", errors[0])

    def test_elf_suffix_is_rejected_even_when_content_looks_textual(self) -> None:
        errors = content_policy_errors(PurePosixPath("unreviewed.so"), b"text\n")
        self.assertIn("forbidden executable file", errors[0])

    def test_binary_hidden_behind_text_suffix_is_rejected(self) -> None:
        errors = content_policy_errors(
            PurePosixPath("unreviewed.md"), b"\x7fELF\x02\x01\x01\x00"
        )
        self.assertIn("binary content in approved text file", errors[0])
        self.assertIn("ELF", errors[0])

    def test_non_utf8_text_is_rejected(self) -> None:
        errors = content_policy_errors(PurePosixPath("unreviewed.txt"), b"\xff")
        self.assertIn("non-UTF-8", errors[0])

    def test_exact_hash_exception_is_allowed(self) -> None:
        path = PurePosixPath("reviewed/payload.so")
        content = b"\x7fELF reviewed payload"
        exceptions = {path: sha256_bytes(content)}
        self.assertEqual(
            content_policy_errors(path, content, hash_exceptions=exceptions), []
        )

    def test_hash_exception_fails_after_payload_changes(self) -> None:
        path = PurePosixPath("reviewed/payload.so")
        exceptions = {path: sha256_bytes(b"expected")}
        errors = content_policy_errors(path, b"changed", hash_exceptions=exceptions)
        self.assertIn("hash-pinned exception differs", errors[0])

    def test_stale_exception_is_rejected(self) -> None:
        path = PurePosixPath("missing/payload.so")
        errors = exception_configuration_errors(set(), {path: "0" * 64})
        self.assertIn("stale hash-pinned exception", errors[0])


if __name__ == "__main__":
    unittest.main()
