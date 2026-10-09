#!/usr/bin/env python3
"""Self-contained runner failure, report lifetime and matrix selector tests."""

import copy
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import tomllib
import unittest
from unittest import mock

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent.parent


def load(name, file):
    spec = importlib.util.spec_from_file_location(name, ROOT / "scripts" / file)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


runner = load("test_nextest_runner", "run-nextest.py")
matrix = load("test_nextest_matrix", "check-lossy-ht-public-matrix.py")


class RunnerTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nextest-runner-test-")
        self.addCleanup(self.temporary.cleanup)
        self.parent = Path(self.temporary.name)
        self.source = self.parent / "source"
        (self.source / ".config").mkdir(parents=True)
        self.config = (ROOT / ".config/nextest.toml").read_bytes()
        (self.source / ".config/nextest.toml").write_bytes(self.config)
        self.reports = self.parent / "reports"
        patch = mock.patch.dict(os.environ, {"EMUELLA_NEXTEST_REPORT_DIR": str(self.reports)})
        patch.start()
        self.addCleanup(patch.stop)

    def fake(self, exit_code=0, report=True, version="0.9.146"):
        def cargo(argv, **kwargs):
            if argv == ["cargo", "nextest", "--version"]:
                return subprocess.CompletedProcess(argv, 0, f"cargo-nextest {version}\n", "")
            self.assertEqual(argv[:3], ["cargo", "nextest", "run"])
            config = tomllib.loads(Path(argv[argv.index("--config-file") + 1]).read_text())
            profile = argv[argv.index("--profile") + 1]
            self.assertEqual(config["store"]["dir"], str(self.reports))
            self.assertEqual(config["profile"][profile]["inherits"], "ci")
            self.assertEqual(config["profile"][profile]["junit"]["report-name"], profile)
            self.assertEqual(config["profile"]["default"]["slow-timeout"], "5s")
            self.assertEqual(config["profile"]["default"]["retries"], 0)
            self.assertFalse(config["profile"]["ci"]["fail-fast"])
            self.assertEqual(config["profile"]["ci"]["junit"]["report-skipped"], "all")
            if report:
                (self.reports / profile / "junit.xml").write_text(f"fresh report; exit {exit_code}")
            return subprocess.CompletedProcess(argv, exit_code)
        return cargo

    def stale(self):
        path = self.reports / "workspace/junit.xml"
        path.parent.mkdir(parents=True)
        path.write_text("stale report")
        return path

    def test_distinct_reports_survive_without_mutating_source(self):
        with mock.patch.object(runner.subprocess, "run", side_effect=self.fake()):
            for profile in ("workspace", "native-parallel"):
                self.assertEqual(runner.run(profile, ["--workspace"], self.source), 0)
        for profile in ("workspace", "native-parallel"):
            self.assertTrue((self.reports / profile / "junit.xml").is_file())
        self.assertEqual(list(self.reports.glob("*.toml")), [])
        self.assertEqual((self.source / ".config/nextest.toml").read_bytes(), self.config)
        self.assertEqual(sorted(str(p.relative_to(self.source)) for p in self.source.rglob("*")),
                         [".config", ".config/nextest.toml"])

    def test_build_failure_propagates_and_removes_stale_report(self):
        stale = self.stale()
        with mock.patch.object(runner.subprocess, "run", side_effect=self.fake(7, report=False)):
            self.assertEqual(runner.run("workspace", ["--workspace"], self.source), 7)
        self.assertFalse(stale.exists())

    def test_test_failure_keeps_fresh_failure_report(self):
        stale = self.stale()
        with mock.patch.object(runner.subprocess, "run", side_effect=self.fake(100)):
            self.assertEqual(runner.run("workspace", ["--workspace"], self.source), 100)
        self.assertEqual(stale.read_text(), "fresh report; exit 100")

    def test_old_or_missing_runner_fails_before_execution_and_removes_stale(self):
        stale = self.stale()
        with mock.patch.object(runner.subprocess, "run", side_effect=self.fake(version="0.9.145")):
            with self.assertRaisesRegex(ValueError, ">= 0.9.146"):
                runner.run("workspace", [], self.source)
        self.assertFalse(stale.exists())
        with mock.patch.object(runner.subprocess, "run", side_effect=FileNotFoundError("cargo")):
            with self.assertRaises(FileNotFoundError):
                runner.run("workspace", [], self.source)

    def test_success_without_report_fails(self):
        with mock.patch.object(runner.subprocess, "run", side_effect=self.fake(report=False)):
            with self.assertRaisesRegex(ValueError, "did not produce"):
                runner.run("workspace", [], self.source)

    def test_output_in_exported_source_and_unknown_configuration_fail(self):
        with mock.patch.dict(os.environ, {"EMUELLA_NEXTEST_REPORT_DIR": str(self.source / "target/nextest")}):
            with self.assertRaisesRegex(ValueError, "outside exported source"):
                runner.run("workspace", [], self.source)
        with self.assertRaisesRegex(ValueError, "unknown test configuration"):
            runner.run("misspelt", [], self.source)

    def test_external_store_cannot_redirect_profile_into_source(self):
        self.reports.mkdir()
        (self.reports / "workspace").symlink_to(self.source, target_is_directory=True)
        with self.assertRaisesRegex(ValueError, "resolves inside exported source"):
            runner.run("workspace", [], self.source)

    def test_hosted_installer_and_upload_keep_pins_and_report_lifetime(self):
        for workflow in ("ci.yml", "release-dry-run.yml"):
            text = (ROOT / ".github/workflows" / workflow).read_text()
            self.assertIn("taiki-e/install-action@f7e5d7c961414b23f5b25b2da9294395d08513ad", text)
            self.assertIn("tool: cargo-nextest@0.9.146", text)
            self.assertIn("checksum: true", text)
            self.assertIn("fallback: none", text)
            self.assertIn("runs-on: ubuntu-latest", text)
        text = (ROOT / ".github/workflows/ci.yml").read_text()
        self.assertLess(text.index("--ci-finish"), text.index("Preserve Nextest reports"))
        self.assertIn("path: nextest-reports/*/junit.xml", text)


class MatrixTests(unittest.TestCase):
    def setUp(self):
        self.suite = {"package-name": "emuella-j2k-core", "kind": "lib", "testcases": {
            matrix.SMOKE: {"kind": "test", "ignored": False},
            matrix.COMPLETE: {"kind": "test", "ignored": True},
            "ht_lossy_public_tests::lossy_ht_export_public_matrix": {"kind": "test", "ignored": True}}}
        self.inventory = {"rust-suites": {"emuella-j2k-core": self.suite}}

    def test_exact_single_smoke_and_ignored_complete_are_accepted(self):
        matrix.guard(self.inventory)

    def test_missing_duplicate_renamed_or_misclassified_tests_fail(self):
        for name in (matrix.SMOKE, matrix.COMPLETE):
            with self.subTest(name=name, mutation="missing"):
                bad = copy.deepcopy(self.inventory)
                del bad["rust-suites"]["emuella-j2k-core"]["testcases"][name]
                with self.assertRaisesRegex(ValueError, "missing or ambiguous"):
                    matrix.guard(bad)
            with self.subTest(name=name, mutation="ignored"):
                bad = copy.deepcopy(self.inventory)
                test = bad["rust-suites"]["emuella-j2k-core"]["testcases"][name]
                test["ignored"] = not test["ignored"]
                with self.assertRaisesRegex(ValueError, "classification"):
                    matrix.guard(bad)
        bad = copy.deepcopy(self.inventory)
        bad["rust-suites"]["duplicate"] = copy.deepcopy(self.suite)
        with self.assertRaisesRegex(ValueError, "missing or ambiguous"):
            matrix.guard(bad)
        bad = copy.deepcopy(self.inventory)
        bad["rust-suites"]["emuella-j2k-core"]["package-name"] = "other-package"
        with self.assertRaisesRegex(ValueError, "classification"):
            matrix.guard(bad)

    def test_entrypoint_selects_only_complete_once_in_release(self):
        fake_runner = mock.Mock()
        fake_runner.run.return_value = 17
        spec = mock.Mock()
        listing = subprocess.CompletedProcess([], 0, json.dumps(self.inventory), "")
        with mock.patch.object(matrix, "spec_from_file_location", return_value=spec), \
             mock.patch.object(matrix, "module_from_spec", return_value=fake_runner), \
             mock.patch.object(matrix.subprocess, "run", return_value=listing) as command:
            self.assertEqual(matrix.main(), 17)
        command.assert_called_once()
        fake_runner.prepare.assert_called_once_with("lossy-ht-matrix")
        fake_runner.run.assert_called_once_with("lossy-ht-matrix", [*matrix.BUILD, "-E", f"test(={matrix.COMPLETE})",
                             "--run-ignored", "only", "--no-tests", "fail", "--success-output", "immediate"])


if __name__ == "__main__":
    unittest.main()
