#!/usr/bin/env python3
"""Synthetic source-bound local/PR/push and hosted lifecycle regressions."""

import contextlib
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import subprocess
import shutil
import sys
import tempfile
import unittest
from unittest import mock

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location("routing_committed_tree", ROOT / "scripts/check-committed-tree.py")
runner = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = runner
SPEC.loader.exec_module(runner)


class RoutingTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="verification-routing-")
        self.addCleanup(self.temporary.cleanup)
        self.parent = Path(self.temporary.name)
        self.root = self.parent / "checkout"
        self.root.mkdir()
        self.git("init", "-q")
        (self.root / "scripts").mkdir()
        (self.root / "docs").mkdir()
        (self.root / "docs/guide.md").write_text("# Guide\n")
        (self.root / "source.rs").write_text("// Runtime source\n")
        (self.root / ".gitignore").write_text("**/target/\n")
        (self.root / "scripts/documentation-policy.json").write_text(json.dumps({"schema_version": 1, "eligible_paths": ["docs/guide.md"]}))
        for name, text in {"check-source.sh": "#!/bin/sh\nexit 0\n", "check.sh": "#!/bin/sh\necho full > \"$TEST_REPORT\"\n",
                           "check-documentation.sh": "#!/bin/sh\necho docs > \"$TEST_REPORT\"\n",
                           "check-full-runtime.sh": "#!/bin/sh\necho full >> \"$TEST_REPORT\"\n",
                           "documentation_checks.py": "import os\nfrom pathlib import Path\nPath(os.environ['TEST_REPORT']).write_text('docs\\n')\n"}.items():
            (self.root / "scripts" / name).write_text(text)
        self.commit()
        self.base = runner.identity(self.root)[0]
        (self.root / "docs/guide.md").write_text("# Guide\n\nUseful guide change.\n")
        self.commit()
        self.head = runner.identity(self.root)[0]
        self.event = self.parent / "event.json"
        self.output = self.parent / "output"
        self.report = self.parent / "report"
        patch = mock.patch.dict(os.environ, {"GITHUB_WORKSPACE": str(self.parent), "GITHUB_EVENT_PATH": str(self.event),
                                            "GITHUB_EVENT_NAME": "push", "GITHUB_SHA": self.head,
                                            "GITHUB_REF": "refs/heads/main", "GITHUB_OUTPUT": str(self.output),
                                            "TEST_REPORT": str(self.report)})
        patch.start()
        self.addCleanup(patch.stop)
        self.push_event()

    def git(self, *arguments):
        return subprocess.check_output(["git", "-c", "user.name=Routing Test", "-c", "user.email=routing@example.invalid",
                                        "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", "-C", str(self.root), *arguments],
                                       env=runner.environment(), stderr=subprocess.DEVNULL)

    def commit(self):
        self.git("add", "-A")
        self.git("commit", "-qm", "Update synthetic fixture")

    def push_event(self):
        self.event.write_text(json.dumps({"before": self.base, "after": self.head, "ref": "refs/heads/main"}))

    def prepare(self):
        with contextlib.redirect_stdout(io.StringIO()):
            runner.ci_prepare(self.root)
        return dict(line.split("=", 1) for line in self.output.read_text().splitlines())["binding_sha256"]

    def outcomes(self, full=False):
        return {name: {"outcome": outcome} for name, outcome in
                (("prepare", "success"), ("selected", "success"), ("deny", "success" if full else "skipped"),
                 ("deny_fuzz", "success" if full else "skipped"))}

    def test_local_pr_and_push_independently_select_same_docs(self):
        pr = {"pull_request": {"base": {"sha": self.base}, "head": {"sha": self.head}}}
        for base in (self.base, runner.event_base(self.root, "pull_request", pr, self.head),
                     runner.event_base(self.root, "push", json.loads(self.event.read_text()), self.head)):
            self.assertEqual(runner.route(self.root, self.head, base)["route"], "docs")
        scratch = self.parent / "local"
        scratch.mkdir()
        with contextlib.redirect_stdout(io.StringIO()):
            runner.verify(self.root, scratch, self.base)
        self.assertEqual(self.report.read_text().strip(), "docs")
        self.assertEqual(list(scratch.iterdir()), [])

    def test_unknown_missing_history_full_and_identity_mismatch_fails(self):
        self.assertEqual(runner.route(self.root, self.head, None)["route"], "full")
        self.assertEqual(runner.route(self.root, self.head, "f" * 40)["route"], "full")
        self.assertIsNone(runner.event_base(self.root, "workflow_dispatch", {}))
        self.assertIsNone(runner.event_base(self.root, "pull_request", {}))
        with self.assertRaises(runner.CheckError):
            runner.event_base(self.root, "push", {"after": self.base, "before": self.base})
        with self.assertRaises(runner.CheckError):
            runner.event_base(self.root, "push", json.loads(self.event.read_text()), self.base)

    def test_recognised_pr_synthetic_merge_and_wrong_topology(self):
        pr = {"pull_request": {"base": {"sha": self.base}, "head": {"sha": self.head}}}
        self.git("checkout", "-q", "-b", "synthetic", self.base)
        self.git("merge", "--no-ff", "--no-edit", self.head)
        merged = runner.identity(self.root)[0]
        self.assertEqual(runner.event_base(self.root, "pull_request", pr, merged), self.base)
        pr["pull_request"]["base"]["sha"] = self.head
        with self.assertRaises(runner.CheckError):
            runner.event_base(self.root, "pull_request", pr, merged)

    def test_hosted_docs_completion_cleanup_preserves_cache_and_siblings(self):
        target = self.root / "target"
        target.mkdir()
        (target / "cache-sentinel").write_text("preserve")
        sibling = self.parent / "unrelated"
        sibling.mkdir()
        digest = self.prepare()
        runner.ci_run(self.root, digest)
        runner.ci_finish(self.root, digest, self.outcomes())
        self.assertFalse((self.parent / "ci-verification").exists())
        self.assertTrue((target / "cache-sentinel").is_file())
        self.assertTrue(sibling.is_dir())

    def test_full_runtime_and_dependencies_required(self):
        (self.root / "source.rs").write_text("// Every Rust edit stays full\n")
        self.commit()
        self.head = runner.identity(self.root)[0]
        os.environ["GITHUB_SHA"] = self.head
        self.push_event()
        digest = self.prepare()
        runner.ci_run(self.root, digest)
        self.assertEqual(self.report.read_text().splitlines(), ["docs", "full"])
        runner.ci_finish(self.root, digest, self.outcomes(full=True))

    def test_missing_skipped_failed_selected_and_dependency_outcomes_fail(self):
        for full in (False, True):
            for name in (["prepare", "selected"] + (["deny", "deny_fuzz"] if full else [])):
                for state in (None, "skipped", "failure", "cancelled"):
                    outcomes = self.outcomes(full)
                    if state is None:
                        del outcomes[name]
                    else:
                        outcomes[name]["outcome"] = state
                    with self.subTest(full=full, name=name, state=state), self.assertRaises(runner.CheckError):
                        runner.check_selected_results("full" if full else "docs", outcomes)

    def test_missing_completion_fails_and_cleans_owned_export(self):
        digest = self.prepare()
        with self.assertRaises(OSError):
            runner.ci_finish(self.root, digest, self.outcomes())
        self.assertFalse((self.parent / "ci-verification").exists())

    def test_changed_export_original_head_event_and_binding_fail(self):
        for mutation in ("export", "original", "head", "event", "binding"):
            with self.subTest(mutation=mutation):
                digest = self.prepare()
                if mutation == "export":
                    (self.parent / "ci-verification/source/docs/guide.md").write_text("changed")
                elif mutation == "original":
                    (self.root / "docs/guide.md").write_text("changed")
                elif mutation == "head":
                    self.git("commit", "--allow-empty", "-qm", "Advance candidate")
                elif mutation == "event":
                    self.event.write_text("{}")
                else:
                    (self.parent / "ci-verification/binding.json").write_text("{}")
                with self.assertRaises((runner.CheckError, OSError)):
                    runner.ci_run(self.root, digest)
                # A tampered ownership record is intentionally not trusted for
                # cleanup; remove this test-owned fixture explicitly.
                import shutil
                shutil.rmtree(self.parent / "ci-verification")
                self.git("reset", "--hard", "-q", self.head)
                self.push_event()

    def test_preexisting_or_symlink_scratch_is_never_deleted(self):
        scratch = self.parent / "ci-verification"
        scratch.mkdir()
        sentinel = scratch / "preserve"
        sentinel.write_text("safe")
        with self.assertRaises(runner.CheckError):
            self.prepare()
        self.assertEqual(sentinel.read_text(), "safe")
        sentinel.unlink()
        scratch.rmdir()
        scratch.symlink_to(self.parent / "absent", target_is_directory=True)
        with self.assertRaises(runner.CheckError):
            self.prepare()
        self.assertTrue(scratch.is_symlink())

    def test_canonical_dirty_refusal_and_invalid_candidate_fail(self):
        (self.root / "source.rs").write_text("// dirty documentation comment\n")
        with self.assertRaises(runner.CheckError):
            runner.verify(self.root, self.parent, self.base)
        with self.assertRaises(runner.CheckError):
            runner.route(self.root, self.base, self.base)

    def test_workflow_source_cache_full_inventory_and_capi_boundaries(self):
        workflow = (ROOT / ".github/workflows/ci.yml").read_text()
        full = (ROOT / "scripts/check-full-runtime.sh").read_text()
        self.assertLess(workflow.index("--ci-prepare"), workflow.index("Restore Rust build cache"))
        self.assertIn("steps.prepare.outputs.route == 'full'", workflow)
        self.assertIn("github.event_name == 'push' && github.ref == 'refs/heads/main'", workflow)
        self.assertIn("cache-on-failure: false", workflow)
        self.assertIn("if: always()", workflow)
        self.assertIn("checkout/crates/emuella-j2k-codestream/fuzz -> target", workflow)
        self.assertEqual(workflow.count("rust-version: 1.99.0"), 2)
        self.assertIn("env -u CARGO_TARGET_DIR -u CARGO_BUILD_BUILD_DIR sh scripts/check-c-api.sh", full)
        self.assertIn("python3 scripts/test-layer2-rendered-pixel.py", full)
        self.assertIn("cargo test -p emuella-j2k-test-support --example lossless_bypass_batch", full)

    def test_failed_selected_command_cannot_complete(self):
        (self.root / "scripts/documentation_checks.py").write_text("raise SystemExit(7)\n")
        self.commit()
        self.head = runner.identity(self.root)[0]
        os.environ["GITHUB_SHA"] = self.head
        self.push_event()
        digest = self.prepare()
        with self.assertRaisesRegex(runner.CheckError, "exit 7"):
            runner.ci_run(self.root, digest)
        self.assertFalse((self.parent / "ci-verification/completed.json").exists())
        with self.assertRaises(runner.CheckError):
            runner.ci_finish(self.root, digest, self.outcomes(full=True) | {"selected": {"outcome": "failure"}})
        self.assertFalse((self.parent / "ci-verification").exists())

    def test_reports_survive_hosted_export_cleanup(self):
        (self.root / "scripts/documentation_checks.py").write_text(
            "import os\nfrom pathlib import Path\n"
            "reports = Path(os.environ['EMUELLA_NEXTEST_REPORT_DIR'])\n"
            "for profile in ('workspace', 'native-parallel'):\n"
            "    report = reports / profile / 'junit.xml'\n"
            "    report.parent.mkdir(parents=True, exist_ok=True)\n"
            "    report.write_text('synthetic report')\n")
        self.commit()
        self.head = runner.identity(self.root)[0]
        os.environ["GITHUB_SHA"] = self.head
        self.push_event()
        digest = self.prepare()
        runner.ci_run(self.root, digest)
        runner.ci_finish(self.root, digest, self.outcomes(full=True))
        self.assertFalse((self.parent / "ci-verification").exists())
        for profile in ("workspace", "native-parallel"):
            self.assertTrue((self.parent / "nextest-reports" / profile / "junit.xml").is_file())

    def test_unbound_archive_ignores_caller_route_assertion(self):
        archive = self.root / "archive"
        (archive / "scripts").mkdir(parents=True)
        for name in ("check-source.sh", "check-full-runtime.sh", "documentation_checks.py"):
            (archive / "scripts" / name).write_bytes((self.root / "scripts" / name).read_bytes())
        (archive / "scripts/check.sh").write_bytes((ROOT / "scripts/check.sh").read_bytes())
        tools = self.parent / "tools"
        tools.mkdir()
        (tools / "cargo").write_text("#!/bin/sh\nexit 0\n")
        (tools / "cargo").chmod(0o755)
        env = os.environ | {"PATH": str(tools) + os.pathsep + os.environ["PATH"], "EMUELLA_ROUTE": "docs"}
        result = subprocess.run(["sh", "scripts/check.sh"], cwd=archive, env=env, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.report.read_text().splitlines(), ["docs", "full"])

    def test_managed_helper_hashes_and_exact_version_exception(self):
        spec = importlib.util.spec_from_file_location("routing_public_audit", ROOT / "scripts/audit-public-tree.py")
        audit = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(audit)
        from public_tree_policy import content_policy_errors
        relative = "scripts/documentation-route/VERSION"
        from pathlib import PurePosixPath
        self.assertEqual(content_policy_errors(PurePosixPath(relative), b"1.0.0\n",
                         hash_exceptions=audit.PUBLIC_TREE_HASH_EXCEPTIONS), [])
        for bad in (b"invalid\n", b"2.0.0\n"):
            self.assertTrue(content_policy_errors(PurePosixPath(relative), bad,
                            hash_exceptions=audit.PUBLIC_TREE_HASH_EXCEPTIONS))
        self.assertTrue(content_policy_errors(PurePosixPath("other/VERSION"), b"1.0.0\n"))
        fixture = self.parent / "payload-fixture"
        helper = fixture / "scripts/documentation-route"
        shutil.copytree(ROOT / "scripts/documentation-route", helper)
        self.assertEqual(audit.documentation_payload_errors(fixture), [])
        (helper / "VERSION").write_text("invalid\n")
        self.assertTrue(audit.documentation_payload_errors(fixture))
        (helper / "VERSION").write_text("1.0.0\n")
        (helper / "LICENSE").write_text("truncated\n")
        self.assertTrue(audit.documentation_payload_errors(fixture))


if __name__ == "__main__":
    unittest.main()
