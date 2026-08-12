#!/usr/bin/env python3
"""Regression tests for the fail-closed public-file policy."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path, PurePosixPath

sys.dont_write_bytecode = True

from public_tree_policy import (  # noqa: E402
    contains_public_openjph_identifier,
    content_policy_errors,
    exception_configuration_errors,
    OPENJPH_COMMIT,
    OPENJPH_DERIVED_RUST_PATHS,
    openjph_provenance_errors,
    sha256_bytes,
)

ROOT = Path(__file__).resolve().parent.parent


def provenance_inputs() -> tuple[dict[PurePosixPath, str], str]:
    sources = {
        PurePosixPath(path.relative_to(ROOT).as_posix()): path.read_text(
            encoding="utf-8"
        )
        for path in sorted((ROOT / "crates").rglob("*.rs"))
    }
    third_party = (ROOT / "THIRD_PARTY.md").read_text(encoding="utf-8")
    return sources, third_party


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

    def test_repository_matches_closed_openjph_allowlist(self) -> None:
        sources, third_party = provenance_inputs()
        self.assertEqual(openjph_provenance_errors(sources, third_party), [])

    def test_missing_or_unapproved_openjph_source_is_rejected(self) -> None:
        sources, third_party = provenance_inputs()
        missing = sorted(OPENJPH_DERIVED_RUST_PATHS)[0]
        sources.pop(missing)
        sources[PurePosixPath("crates/example/src/copied.rs")] = (
            "// SPDX-License-Identifier: BSD-2-Clause\n"
        )
        errors = openjph_provenance_errors(sources, third_party)
        self.assertIn(f"approved OpenJPH-derived file is absent: {missing}", errors)
        self.assertIn(
            "unapproved BSD-derived Rust source: crates/example/src/copied.rs", errors
        )

    def test_unapproved_bsd_spdx_variants_are_rejected_anywhere_in_source(self) -> None:
        variants = (
            "pub fn before() {}\n// SPDX-License-Identifier: BSD-2-Clause\n",
            "// SPDX-License-Identifier: Apache-2.0 AND BSD-2-Clause\n",
            "//  SPDX-License-Identifier:   BSD-2-Clause  \n",
            "/* SPDX-License-Identifier: BSD-2-Clause */\n",
            "//! SPDX-License-Identifier: BSD-2-Clause\n",
        )
        for number, source in enumerate(variants):
            with self.subTest(source=source):
                sources, third_party = provenance_inputs()
                path = PurePosixPath(f"crates/example/src/copied-{number}.rs")
                sources[path] = source
                errors = openjph_provenance_errors(sources, third_party)
                self.assertIn(f"unapproved BSD-derived Rust source: {path}", errors)

    def test_displaced_header_and_missing_preamble_fields_are_rejected(self) -> None:
        sources, third_party = provenance_inputs()
        displaced, incomplete = sorted(OPENJPH_DERIVED_RUST_PATHS)[:2]
        sources[displaced] = "\n" + sources[displaced]
        sources[incomplete] = sources[incomplete].replace(OPENJPH_COMMIT, "0" * 40)
        errors = openjph_provenance_errors(sources, third_party)
        self.assertIn(
            f"approved OpenJPH-derived file must have exactly one exact first-line "
            f"BSD-2-Clause header: {displaced}",
            errors,
        )
        self.assertIn(
            f"{incomplete} header must name exactly one pinned OpenJPH revision "
            f"{OPENJPH_COMMIT}",
            errors,
        )

    def test_root_openjph_pin_must_be_unique_and_exact(self) -> None:
        sources, third_party = provenance_inputs()
        record = f"Pinned upstream commit: `{OPENJPH_COMMIT}`"
        mutations = (
            third_party.replace(f"{record}\n", ""),
            third_party.replace(record, f"Pinned upstream commit: `{'0' * 40}`"),
            third_party.replace(record, f"{record}\n{record}"),
            third_party.replace(
                record, f"{record}\nPinned upstream commit: `{'0' * 40}`"
            ),
        )
        expected = (
            "THIRD_PARTY.md must contain exactly one pinned OpenJPH commit record "
            f"for {OPENJPH_COMMIT}"
        )
        for mutation in mutations:
            with self.subTest(mutation=mutation):
                self.assertIn(expected, openjph_provenance_errors(sources, mutation))

    def test_source_openjph_pin_must_be_unique_and_exact(self) -> None:
        sources, third_party = provenance_inputs()
        path = sorted(OPENJPH_DERIVED_RUST_PATHS)[0]
        sources[path] = sources[path].replace(
            OPENJPH_COMMIT,
            f"{OPENJPH_COMMIT}\n// Commit: {'0' * 40}",
            1,
        )
        self.assertIn(
            f"{path} header must name exactly one pinned OpenJPH revision "
            f"{OPENJPH_COMMIT}",
            openjph_provenance_errors(sources, third_party),
        )

    def test_approved_source_with_an_extra_spdx_declaration_is_rejected(self) -> None:
        sources, third_party = provenance_inputs()
        path = sorted(OPENJPH_DERIVED_RUST_PATHS)[0]
        sources[path] += "\n// SPDX-License-Identifier: Apache-2.0\n"
        errors = openjph_provenance_errors(sources, third_party)
        self.assertIn(
            f"approved OpenJPH-derived file must have exactly one exact first-line "
            f"BSD-2-Clause header: {path}",
            errors,
        )

    def test_every_required_openjph_preamble_field_is_enforced(self) -> None:
        replacements = {
            "OpenJPH": "OpenJPH attribution",
            "https://github.com/aous72/OpenJPH": "OpenJPH source URL",
            "Copyright (c)": "upstream copyright notice",
            "Modified for Emuella:": "Emuella modification summary",
            "THIRD_PARTY.md": "third-party provenance reference",
        }
        path = sorted(OPENJPH_DERIVED_RUST_PATHS)[0]
        for needle, label in replacements.items():
            with self.subTest(label=label):
                sources, third_party = provenance_inputs()
                sources[path] = sources[path].replace(needle, "removed")
                errors = openjph_provenance_errors(sources, third_party)
                self.assertIn(f"{path} header omits {label}", errors)

    def test_openjph_table_extra_duplicate_and_malformed_rows_are_rejected(
        self,
    ) -> None:
        sources, third_party = provenance_inputs()
        existing = sorted(OPENJPH_DERIVED_RUST_PATHS)[0]
        rows = (
            "| `crates/example/src/copied.rs` | copied | none |\n"
            f"| `{existing}` | duplicate | duplicate |\n"
            "| [linked](crates/example/src/linked.rs) | linked | invalid |\n"
            "| `../escape.rs` | traversal | invalid |\n"
            "| `crates/emuella-j2k-ht/./src/block_encoder.rs` | dot | invalid |\n"
            "| `crates/emuella-j2k-ht//src/block_encoder.rs` | slash | invalid |\n"
        )
        third_party = third_party.replace(
            "\n\nThis table is the closed", "\n" + rows + "\nThis table is the closed"
        )
        errors = openjph_provenance_errors(sources, third_party)
        self.assertIn(
            "THIRD_PARTY.md names unapproved OpenJPH-derived file: "
            "crates/example/src/copied.rs",
            errors,
        )
        self.assertIn("THIRD_PARTY.md repeats an OpenJPH-derived Emuella path", errors)
        self.assertIn("THIRD_PARTY.md has a malformed OpenJPH Emuella path", errors)
        self.assertIn(
            "THIRD_PARTY.md has an unsafe OpenJPH Emuella path: ../escape.rs", errors
        )
        self.assertIn(
            "THIRD_PARTY.md has an unsafe OpenJPH Emuella path: "
            "crates/emuella-j2k-ht/./src/block_encoder.rs",
            errors,
        )
        self.assertIn(
            "THIRD_PARTY.md has an unsafe OpenJPH Emuella path: "
            "crates/emuella-j2k-ht//src/block_encoder.rs",
            errors,
        )

    def test_duplicate_openjph_table_heading_is_rejected(self) -> None:
        sources, third_party = provenance_inputs()
        errors = openjph_provenance_errors(
            sources, third_party + "\n### Emuella files\n"
        )
        self.assertIn(
            "THIRD_PARTY.md must contain exactly one OpenJPH Emuella-file table",
            errors,
        )


if __name__ == "__main__":
    unittest.main()
