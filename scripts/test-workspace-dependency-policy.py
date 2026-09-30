#!/usr/bin/env python3
"""Regression probes for the sole unpublished workspace fixture exception."""

from __future__ import annotations

from pathlib import Path
import shutil
import sys
import tempfile
import tomllib
import unittest

sys.dont_write_bytecode = True
from workspace_dependency_policy import dependency_policy_errors  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
CAPI = "crates/emuella-j2k-capi/Cargo.toml"
SUPPORT = "crates/emuella-j2k-test-support/Cargo.toml"


class DependencyPolicyTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        members = tomllib.loads((ROOT / "Cargo.toml").read_text())["workspace"]["members"]
        for relative in ["Cargo.toml", "deny.toml"] + [f"{member}/Cargo.toml" for member in members]:
            target = self.root / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / relative, target)

    def replace(self, relative: str, before: str, after: str) -> None:
        path = self.root / relative
        text = path.read_text()
        self.assertIn(before, text)
        path.write_text(text.replace(before, after, 1))

    def reject(self) -> None:
        self.assertTrue(dependency_policy_errors(self.root))

    def test_real_manifests_pass(self) -> None:
        self.assertEqual(dependency_policy_errors(ROOT), [])

    def test_other_unversioned_dev_dependency_is_rejected(self) -> None:
        self.replace(CAPI, 'static_assertions = "1.1.0"',
                     'emuella-j2k-core = { path = "../emuella-j2k-core" }')
        self.reject()

    def test_private_normal_and_build_dependencies_are_rejected(self) -> None:
        for table in ("dependencies", "build-dependencies"):
            with self.subTest(table=table):
                path = self.root / SUPPORT
                original = path.read_text()
                declaration = 'emuella-jpip = { path = "../emuella-jpip" }\n'
                if f'[{table}]' in original:
                    changed = original.replace(f'[{table}]\n', f'[{table}]\n' + declaration)
                else:
                    changed = original + f'\n[{table}]\n' + declaration
                path.write_text(changed)
                self.reject()
                path.write_text(original)

    def test_target_specific_dependency_is_rejected(self) -> None:
        path = self.root / CAPI
        path.write_text(path.read_text() + '\n[target.\'cfg(unix)\'.dev-dependencies]\nemuella-jpip = { path = "../emuella-jpip" }\n')
        self.reject()

    def test_inherited_unversioned_path_is_rejected(self) -> None:
        self.replace("Cargo.toml", 'version = "0.1.0", path = "crates/emuella-j2k"',
                     'path = "crates/emuella-j2k"')
        self.reject()

    def test_local_wildcard_version_is_rejected(self) -> None:
        self.replace("Cargo.toml", 'version = "0.1.0", path = "crates/emuella-j2k"',
                     'version = "*", path = "crates/emuella-j2k"')
        self.reject()

    def test_changed_exception_path_and_alias_are_rejected(self) -> None:
        path = self.root / CAPI
        original = path.read_text()
        self.replace(CAPI, 'path = "../emuella-j2k-test-support"',
                     'path = "../emuella-j2k-core"')
        self.reject()
        path.write_text(original)
        self.replace(CAPI, 'emuella-j2k-test-support =', 'fixture-alias =')
        self.reject()

    def test_exception_in_another_dependency_table_is_rejected(self) -> None:
        declaration = 'emuella-j2k-test-support = { path = "../emuella-j2k-test-support" }'
        self.replace(CAPI, declaration, '')
        self.replace(CAPI, '[dependencies]\n', '[dependencies]\n' + declaration + '\n')
        self.reject()

    def test_support_publication_is_rejected(self) -> None:
        self.replace(SUPPORT, 'publish = false', 'publish = true')
        self.reject()

    def test_git_route_is_rejected(self) -> None:
        self.replace(CAPI, 'static_assertions = "1.1.0"',
                     'probe = { git = "https://example.invalid/probe" }')
        self.reject()

    def test_dependency_replacement_route_is_rejected(self) -> None:
        path = self.root / "Cargo.toml"
        path.write_text(path.read_text() + '\n[patch.crates-io]\nprobe = { path = "outside" }\n')
        self.reject()

    def test_wildcard_and_source_policy_weakening_is_rejected(self) -> None:
        for setting in ('wildcards', 'unknown-registry', 'unknown-git'):
            with self.subTest(setting=setting):
                path = self.root / "deny.toml"
                original = path.read_text()
                self.replace("deny.toml", f'{setting} = "deny"', f'{setting} = "allow"')
                self.reject()
                path.write_text(original)


if __name__ == "__main__":
    unittest.main()
