#!/usr/bin/env python3
"""Leak, drift and refresh tests for the facade public API snapshot check.

Each case builds a three-crate fixture workspace (facade, core and a lower
layer) with the repository's pinned toolchain, so the check sees real rustdoc
JSON rather than hand-written approximations.
"""

from __future__ import annotations

import contextlib
import importlib.util
import io
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent.parent


def load(name: str, file: str):
    spec = importlib.util.spec_from_file_location(name, ROOT / "scripts" / file)
    module = importlib.util.module_from_spec(spec)
    # Dataclasses resolve annotations through the registered module.
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


check = load("check_facade_public_api", "check-facade-public-api.py")

LOWER = """
pub struct Exported;
pub struct Internal;
pub trait Source {
    fn len(&self) -> u64;
}
pub struct InternalSource;
impl Source for InternalSource {
    fn len(&self) -> u64 {
        0
    }
}
"""

CORE = """
pub use fixture_lower::{Exported, Source};
pub struct Internal;
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct Options {
    pub limit: u32,
}
impl Options {
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }
}
pub fn run(options: &Options) -> u32 {
    options.limit
}
"""

FACADE = "pub use fixture_core::{Exported, Options, Source, run};\n"


class FacadePublicApiTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.temporary = tempfile.TemporaryDirectory(prefix="facade-public-api-test-")
        cls.parent = Path(cls.temporary.name)
        cls.workspace = cls.parent / "workspace"
        cls.target = cls.parent / "target"
        (cls.workspace / "crates").mkdir(parents=True)
        shutil.copyfile(ROOT / "rust-toolchain.toml", cls.workspace / "rust-toolchain.toml")
        (cls.workspace / "Cargo.toml").write_text(
            '[workspace]\nmembers = ["crates/*"]\nresolver = "3"\n', encoding="utf-8"
        )
        for name, dependency in (
            ("fixture-lower", None),
            ("fixture-core", "fixture-lower"),
            ("fixture-facade", "fixture-core"),
        ):
            crate = cls.workspace / "crates" / name
            (crate / "src").mkdir(parents=True)
            dependencies = (
                f'{dependency} = {{ path = "../{dependency}" }}\n' if dependency else ""
            )
            (crate / "Cargo.toml").write_text(
                f'[package]\nname = "{name}"\nversion = "0.1.0"\nedition = "2024"\n'
                f"publish = false\n\n[dependencies]\n{dependencies}",
                encoding="utf-8",
            )
        cls.write_sources(LOWER, CORE, FACADE)
        subprocess.run(
            ["cargo", "generate-lockfile", "--offline"],
            cwd=cls.workspace,
            check=True,
            capture_output=True,
        )

    @classmethod
    def tearDownClass(cls) -> None:
        cls.temporary.cleanup()

    @classmethod
    def write_sources(cls, lower: str, core: str, facade: str) -> None:
        for name, text in (
            ("fixture-lower", lower),
            ("fixture-core", core),
            ("fixture-facade", facade),
        ):
            (cls.workspace / "crates" / name / "src/lib.rs").write_text(text, encoding="utf-8")

    def setUp(self) -> None:
        self.snapshot = self.parent / "snapshot.txt"
        self.write_sources(LOWER, CORE, FACADE)
        status, _, error = self.run_check("--update")
        self.assertEqual(status, 0, error)

    def run_check(self, *extra: str) -> tuple[int, str, str]:
        stdout, stderr = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr):
            status = check.main(
                [
                    "--manifest-path", str(self.workspace / "Cargo.toml"),
                    "--package", "fixture-facade",
                    "--snapshot", str(self.snapshot),
                    "--target-dir", str(self.target),
                    *extra,
                ]
            )
        return status, stdout.getvalue(), stderr.getvalue()

    def assert_leak(self, *, lower: str = LOWER, core: str = CORE, facade: str = FACADE,
                    reaches: str) -> None:
        self.write_sources(lower, core, facade)
        for extra in ((), ("--update",)):
            status, _, error = self.run_check(*extra)
            self.assertEqual(status, 1, error)
            self.assertIn("reaches items it does not export", error)
            self.assertIn(f"reaches `{reaches}`", error)

    def test_reviewed_snapshot_passes_and_records_members(self) -> None:
        status, output, error = self.run_check()
        self.assertEqual(status, 0, error)
        self.assertIn("matches", output)
        snapshot = self.snapshot.read_text(encoding="utf-8")
        self.assertIn("#[non_exhaustive] pub struct fixture_facade::Options\n", snapshot)
        self.assertIn("pub fixture_facade::Options::limit: u32\n", snapshot)
        self.assertIn(
            "pub fn fixture_facade::Options::with_limit(self, limit: u32) -> Self\n", snapshot
        )
        self.assertIn("impl core::marker::Send for fixture_facade::Options\n", snapshot)
        self.assertIn("impl core::clone::Clone for fixture_facade::Options\n", snapshot)
        self.assertNotIn("InternalSource", snapshot)

    def test_unreviewed_surface_change_fails_until_refreshed(self) -> None:
        core = CORE + "impl Options {\n    pub fn limit(&self) -> u32 {\n        self.limit\n    }\n}\n"
        self.write_sources(LOWER, core, FACADE)
        status, _, error = self.run_check()
        self.assertEqual(status, 1)
        self.assertIn("+pub fn fixture_facade::Options::limit(&self) -> u32", error)
        self.assertIn("--update", error)
        status, _, error = self.run_check("--update")
        self.assertEqual(status, 0, error)
        status, _, error = self.run_check()
        self.assertEqual(status, 0, error)

    def test_public_field_of_lower_type_fails(self) -> None:
        core = CORE.replace(
            "    pub limit: u32,\n", "    pub limit: u32,\n    pub internal: Option<fixture_lower::Internal>,\n"
        ).replace("#[derive(Clone, Debug, Default)]\n", "")
        self.assert_leak(core=core, reaches="fixture_lower::Internal")

    def test_inherent_method_returning_lower_type_fails(self) -> None:
        core = CORE + (
            "impl Options {\n    pub fn internal(&self) -> fixture_lower::Internal {\n"
            "        fixture_lower::Internal\n    }\n}\n"
        )
        self.assert_leak(core=core, reaches="fixture_lower::Internal")

    def test_hidden_method_returning_lower_type_fails(self) -> None:
        core = CORE + (
            "impl Options {\n    #[doc(hidden)]\n    pub fn internal(&self) -> fixture_lower::Internal {\n"
            "        fixture_lower::Internal\n    }\n}\n"
        )
        self.assert_leak(core=core, reaches="fixture_lower::Internal")

    def test_method_returning_unexported_core_type_fails(self) -> None:
        core = CORE + "impl Options {\n    pub fn internal(&self) -> Internal {\n        Internal\n    }\n}\n"
        self.assert_leak(core=core, reaches="fixture_core::Internal")

    def test_trait_implementation_naming_lower_type_fails(self) -> None:
        core = CORE + (
            "impl From<fixture_lower::Internal> for Options {\n"
            "    fn from(_: fixture_lower::Internal) -> Self {\n        Self::default()\n    }\n}\n"
        )
        self.assert_leak(core=core, reaches="fixture_lower::Internal")

    def test_associated_type_naming_lower_type_fails(self) -> None:
        core = CORE + (
            "impl core::ops::Deref for Options {\n    type Target = fixture_lower::Internal;\n"
            "    fn deref(&self) -> &Self::Target {\n        &fixture_lower::Internal\n    }\n}\n"
        )
        self.assert_leak(core=core, reaches="fixture_lower::Internal")

    def test_exported_trait_method_naming_lower_type_fails(self) -> None:
        lower = LOWER.replace(
            "    fn len(&self) -> u64;\n",
            "    fn len(&self) -> u64;\n    fn internal(&self) -> Option<Internal> {\n        None\n    }\n",
        )
        self.assert_leak(lower=lower, reaches="fixture_lower::Internal")

    def test_exported_trait_implemented_by_unexported_type_is_not_surface(self) -> None:
        status, _, error = self.run_check()
        self.assertEqual(status, 0, error)

    def test_glob_and_module_re_exports_fail(self) -> None:
        for facade, message in (
            ("pub use fixture_core::*;\n", "glob re-export"),
            (FACADE + "pub use fixture_core as core_crate;\n", "re-exports module"),
            (FACADE + "pub mod extra {}\n", "exposes module"),
        ):
            with self.subTest(facade=facade):
                self.write_sources(LOWER, CORE, facade)
                status, _, error = self.run_check()
                self.assertEqual(status, 1, error)
                self.assertIn(message, error)

    def test_unsupported_rustdoc_format_fails_closed(self) -> None:
        directory = self.parent / "format"
        directory.mkdir(exist_ok=True)
        (directory / "fixture.json").write_text('{"format_version": 1}', encoding="utf-8")
        with self.assertRaisesRegex(check.CheckError, "supports"):
            check.load(directory, ["fixture"])


if __name__ == "__main__":
    unittest.main()
