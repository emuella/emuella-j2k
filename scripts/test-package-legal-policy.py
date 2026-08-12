#!/usr/bin/env python3
"""Regression tests for canonical Cargo package legal-file contents."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.dont_write_bytecode = True

from package_legal_policy import (  # noqa: E402
    OPENJPH_COMMIT,
    PACKAGE_POLICY,
    legal_content_errors,
)

ROOT = Path(__file__).resolve().parent.parent


def source_legal_files(package_name: str) -> dict[str, bytes]:
    package_root = ROOT / "crates" / package_name
    return {
        name: (package_root / name).read_bytes()
        for name in PACKAGE_POLICY[package_name].legal_file_sha256
    }


def replace(files: dict[str, bytes], name: str, old: bytes, new: bytes) -> None:
    files[name] = files[name].replace(old, new)


class PackageLegalPolicyTests(unittest.TestCase):
    def test_checked_in_legal_files_match_canonical_policy(self) -> None:
        for package_name in PACKAGE_POLICY:
            with self.subTest(package=package_name):
                self.assertEqual(
                    legal_content_errors(
                        package_name, source_legal_files(package_name)
                    ),
                    [],
                )

    def test_truncated_apache_text_is_rejected(self) -> None:
        files = source_legal_files("emuella-j2k")
        files["LICENSE-APACHE-2.0"] = files["LICENSE-APACHE-2.0"][:-32]
        errors = legal_content_errors("emuella-j2k", files)
        self.assertTrue(any("legal file hash differs" in error for error in errors))

    def test_changed_openjph_commit_is_rejected(self) -> None:
        files = source_legal_files("emuella-j2k-ht")
        replace(
            files,
            "THIRD_PARTY.md",
            OPENJPH_COMMIT.encode(),
            b"0" * len(OPENJPH_COMMIT),
        )
        errors = legal_content_errors("emuella-j2k-ht", files)
        self.assertTrue(any("legal file hash differs" in error for error in errors))
        self.assertIn("THIRD_PARTY.md omits the pinned OpenJPH commit", errors)

    def test_missing_upstream_path_is_rejected(self) -> None:
        files = source_legal_files("emuella-j2k-codestream")
        replace(
            files,
            "THIRD_PARTY.md",
            b"src/core/codestream/ojph_codestream_gen.cpp",
            b"src/core/codestream/removed.cpp",
        )
        errors = legal_content_errors("emuella-j2k-codestream", files)
        self.assertTrue(any("omits upstream source path" in error for error in errors))

    def test_missing_package_specific_holder_is_rejected(self) -> None:
        files = source_legal_files("emuella-j2k-accel")
        replace(
            files,
            "LICENSE-BSD-2-CLAUSE",
            b"Intel Corporation",
            b"Removed Corporation",
        )
        errors = legal_content_errors("emuella-j2k-accel", files)
        self.assertTrue(any("omits copyright holder" in error for error in errors))

    def test_stale_openjpeg_attribution_is_rejected(self) -> None:
        files = source_legal_files("emuella-j2k-ht")
        files["NOTICE"] += b"\nOpenJPEG\n"
        errors = legal_content_errors("emuella-j2k-ht", files)
        self.assertIn("stale openjpeg attribution in NOTICE", errors)

    def test_stale_hayro_attribution_is_rejected(self) -> None:
        files = source_legal_files("emuella-j2k-ht")
        files["THIRD_PARTY.md"] += b"\nHayro JPEG 2000\n"
        errors = legal_content_errors("emuella-j2k-ht", files)
        self.assertIn("stale hayro attribution in THIRD_PARTY.md", errors)


if __name__ == "__main__":
    unittest.main()
