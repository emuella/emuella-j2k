#!/usr/bin/env python3
"""Self-contained behavioural tests for exact committed-tree verification."""

from __future__ import annotations

import contextlib
import importlib.util
import io
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location(
    "committed_tree_check", ROOT / "scripts/check-committed-tree.py"
)
assert SPEC is not None and SPEC.loader is not None
runner = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = runner
SPEC.loader.exec_module(runner)

PROBE = """#!/bin/sh
set -eu
test ! -e .git
test ! -e .local
test ! -e target
test -z "${GIT_DIR-}"
test -z "${GIT_WORK_TREE-}"
test "$PWD" != "$CARGO_TARGET_DIR"
test "$PYTHONDONTWRITEBYTECODE" = 1
printf '%s\\n' "$PWD" "$CARGO_TARGET_DIR" "$TMPDIR" > "$CHECK_TEST_REPORT"
touch "$CARGO_TARGET_DIR/build-output" "$TMPDIR/check-temp"
"""


class CommittedTreeTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory(prefix="committed-tree-test-")
        self.addCleanup(self.temporary.cleanup)
        self.parent = Path(self.temporary.name)
        self.repo = self.parent / "repository"
        self.scratch = self.parent / "scratch"
        self.scratch.mkdir()
        self.report = self.parent / "report.txt"
        patch = mock.patch.dict(os.environ, {"CHECK_TEST_REPORT": str(self.report)})
        patch.start()
        self.addCleanup(patch.stop)
        self.initialise(self.repo)

    def git(self, *arguments: str, root: Path | None = None) -> bytes:
        return subprocess.run(
            [
                "git",
                "-c",
                "user.name=Verification Test",
                "-c",
                "user.email=verification@example.invalid",
                "-c",
                "commit.gpgsign=false",
                "-c",
                "core.hooksPath=/dev/null",
                "-C",
                str(root or self.repo),
                *arguments,
            ],
            env=runner.environment(),
            capture_output=True,
            check=True,
        ).stdout

    def initialise(self, root: Path, algorithm: str = "sha1") -> None:
        root.mkdir()
        self.git("init", f"--object-format={algorithm}", root=root)
        (root / "scripts").mkdir()
        (root / "scripts/check.sh").write_text(PROBE)
        (root / "scripts/check.sh").chmod(0o755)
        (root / "source.txt").write_text("committed source\n")
        (root / ".gitignore").write_text("/.local/\n/target/\n")
        self.commit(root)

    def commit(self, root: Path | None = None) -> None:
        self.git("add", "--all", root=root)
        self.git(
            "commit",
            "--quiet",
            "-m",
            "Create synthetic verification fixture",
            root=root,
        )

    def verify(self, root: Path | None = None) -> str:
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            runner.verify(root or self.repo, self.scratch)
        self.assertEqual(list(self.scratch.iterdir()), [])
        return output.getvalue()

    def test_primary_identity_build_separation_and_cleanup(self) -> None:
        sentinel = self.scratch / "unrelated.txt"
        sentinel.write_text("preserve")
        expected = runner.identity(self.repo)
        with contextlib.redirect_stdout(io.StringIO()) as output:
            runner.verify(self.repo, self.scratch)
        self.assertIn(
            f"Verification passed: commit {expected[0]}, tree {expected[1]}",
            output.getvalue(),
        )
        source, build, temporary = map(Path, self.report.read_text().splitlines())
        self.assertEqual(
            {source.parent, build.parent, temporary.parent}, {source.parent}
        )
        self.assertEqual(
            {source.name, build.name, temporary.name}, {"source", "build", "tmp"}
        )
        self.assertFalse(source.parent.exists())
        self.assertEqual(list(self.scratch.iterdir()), [sentinel])
        self.assertEqual(sentinel.read_text(), "preserve")

    def test_linked_checkout_and_inherited_git_environment(self) -> None:
        linked = self.parent / "linked"
        self.git("worktree", "add", "--detach", str(linked), "HEAD")
        self.assertTrue((linked / ".git").is_file())
        with mock.patch.dict(
            os.environ,
            {
                "GIT_DIR": str(self.parent / "wrong-git"),
                "GIT_WORK_TREE": str(self.parent / "wrong-tree"),
                "GIT_INDEX_FILE": str(self.parent / "wrong-index"),
                "GIT_CONFIG_COUNT": "1",
                "GIT_CONFIG_KEY_0": "core.bare",
                "GIT_CONFIG_VALUE_0": "true",
            },
        ):
            self.assertIn("Verification passed:", self.verify(linked))

    def test_sha256_object_identity(self) -> None:
        repository = self.parent / "sha256"
        self.initialise(repository, "sha256")
        self.assertEqual(len(runner.identity(repository)[0]), 64)
        self.assertIn("Verification passed:", self.verify(repository))

    def test_ignored_private_overlays_and_build_cache_are_not_exported(self) -> None:
        for name in (".local", "target"):
            (self.repo / name).mkdir()
            (self.repo / name / "private.txt").write_text("not public source")
        self.verify()
        self.assertTrue((self.repo / ".local/private.txt").is_file())
        self.assertTrue((self.repo / "target/private.txt").is_file())

    def test_relative_cache_home_stays_outside_export(self) -> None:
        cache = self.repo / ".local/cargo"
        cache.mkdir(parents=True)
        with (self.repo / "scripts/check.sh").open("a") as script:
            script.write('touch "$CARGO_HOME/cache-probe"\n')
        self.commit()
        with mock.patch.dict(os.environ, {"CARGO_HOME": ".local/cargo"}):
            self.verify()
        self.assertTrue((cache / "cache-probe").is_file())

    def test_staged_unstaged_and_untracked_changes_are_refused(self) -> None:
        for change in ("unstaged", "staged", "untracked"):
            with self.subTest(change=change):
                repository = self.parent / change
                self.initialise(repository)
                path = repository / (
                    "new.txt" if change == "untracked" else "source.txt"
                )
                path.write_text("new source\n")
                if change == "staged":
                    self.git("add", "source.txt", root=repository)
                with self.assertRaisesRegex(runner.CheckError, "commit first"):
                    self.verify(repository)
        self.assertFalse(self.report.exists())

    def test_stat_cache_flags_do_not_hide_changed_working_bytes(self) -> None:
        for flag in ("--assume-unchanged", "--skip-worktree"):
            with self.subTest(flag=flag):
                self.git("update-index", flag, "source.txt")
                (self.repo / "source.txt").write_text("hidden change\n")
                with self.assertRaisesRegex(runner.CheckError, "differs from Git tree"):
                    self.verify()
                (self.repo / "source.txt").write_text("committed source\n")
                self.git(
                    "update-index",
                    "--no-assume-unchanged",
                    "--no-skip-worktree",
                    "source.txt",
                )

    def test_archive_attributes_cannot_omit_or_transform_source(self) -> None:
        (self.repo / ".gitattributes").write_text(
            "source.txt export-ignore\nstamp.txt export-subst\n"
        )
        (self.repo / "stamp.txt").write_text("$Format:%H$\n")
        with (self.repo / "scripts/check.sh").open("a") as script:
            script.write(
                "test -f source.txt\ntest \"$(cat stamp.txt)\" = '$Format:%H$'\n"
            )
        self.commit()
        self.verify()

    def test_unsupported_symlink_and_submodule_modes_are_refused(self) -> None:
        for mode in ("120000", "160000"):
            with self.subTest(mode=mode):
                repository = self.parent / mode
                self.initialise(repository)
                if mode == "120000":
                    (repository / "link").symlink_to("source.txt")
                    self.commit(repository)
                else:
                    commit = runner.identity(repository)[0]
                    self.git(
                        "update-index",
                        "--add",
                        "--cacheinfo",
                        f"160000,{commit},module",
                        root=repository,
                    )
                    self.git(
                        "commit",
                        "--quiet",
                        "-m",
                        "Add synthetic gitlink",
                        root=repository,
                    )
                with self.assertRaisesRegex(runner.CheckError, "unsupported Git mode"):
                    self.verify(repository)

    def test_unsafe_tree_paths_are_refused_before_writing(self) -> None:
        for path in (
            "../escape",
            "/absolute",
            "a/../escape",
            ".git/config",
            "a\\escape",
            "a//b",
        ):
            with (
                self.subTest(path=path),
                mock.patch.object(
                    runner,
                    "git",
                    return_value=f"100644 blob {'1' * 40}\t{path}\0".encode(),
                ),
            ):
                with self.assertRaisesRegex(runner.CheckError, "unsafe"):
                    runner.tree_entries(self.repo, "tree")

    def test_export_byte_and_path_changes_fail_closed(self) -> None:
        for mutation in ("changed", "missing", "extra"):
            with self.subTest(mutation=mutation):
                original = runner.export_tree

                def altered_export(root, destination, entries, algorithm):
                    original(root, destination, entries, algorithm)
                    if mutation == "changed":
                        (destination / "source.txt").write_text("altered")
                    elif mutation == "missing":
                        (destination / "source.txt").unlink()
                    else:
                        (destination / "extra.txt").write_text("extra")

                with mock.patch.object(
                    runner, "export_tree", side_effect=altered_export
                ):
                    with self.assertRaises((runner.CheckError, OSError)):
                        self.verify()
                self.assertEqual(list(self.scratch.iterdir()), [])

    def test_build_process_source_mutation_is_refused(self) -> None:
        with (self.repo / "scripts/check.sh").open("a") as script:
            script.write("printf 'changed' > source.txt\n")
        self.commit()
        with self.assertRaisesRegex(runner.CheckError, "differs from Git tree"):
            self.verify()
        self.assertEqual(list(self.scratch.iterdir()), [])

    def test_original_source_mutation_during_export_is_refused(self) -> None:
        original = runner.export_tree

        def mutate(root, destination, entries, algorithm):
            original(root, destination, entries, algorithm)
            (root / "source.txt").write_text("changed during export")

        with mock.patch.object(runner, "export_tree", side_effect=mutate):
            with self.assertRaisesRegex(runner.CheckError, "commit first"):
                self.verify()
        self.assertFalse(self.report.exists())
        self.assertEqual(list(self.scratch.iterdir()), [])

    def test_original_head_mutation_during_checks_is_refused(self) -> None:
        original = runner.subprocess.run

        def mutate(command, **kwargs):
            result = original(command, **kwargs)
            if command == ["sh", "scripts/check.sh"]:
                self.git(
                    "commit", "--quiet", "--allow-empty", "-m", "Advance fixture HEAD"
                )
            return result

        with mock.patch.object(runner.subprocess, "run", side_effect=mutate):
            with self.assertRaisesRegex(runner.CheckError, "HEAD changed"):
                self.verify()
        self.assertEqual(list(self.scratch.iterdir()), [])

    def test_original_untracked_source_during_checks_is_refused(self) -> None:
        original = runner.subprocess.run

        def mutate(command, **kwargs):
            result = original(command, **kwargs)
            if command == ["sh", "scripts/check.sh"]:
                (self.repo / "new-source.txt").write_text("new source")
            return result

        with mock.patch.object(runner.subprocess, "run", side_effect=mutate):
            with self.assertRaisesRegex(runner.CheckError, "commit first"):
                self.verify()
        self.assertEqual(list(self.scratch.iterdir()), [])

    def test_failed_checks_and_failed_export_clean_only_owned_child(self) -> None:
        with (self.repo / "scripts/check.sh").open("a") as script:
            script.write("exit 23\n")
        self.commit()
        with self.assertRaisesRegex(runner.CheckError, "exit 23"):
            self.verify()
        self.assertEqual(list(self.scratch.iterdir()), [])
        with mock.patch.object(
            runner, "export_tree", side_effect=OSError("synthetic export failure")
        ):
            with self.assertRaisesRegex(OSError, "synthetic export failure"):
                self.verify()
        self.assertEqual(list(self.scratch.iterdir()), [])

    def test_git_failure_does_not_report_success(self) -> None:
        failed = subprocess.CompletedProcess([], 9, b"", b"synthetic failure")
        with mock.patch.object(runner.subprocess, "run", return_value=failed):
            with self.assertRaisesRegex(runner.CheckError, "Git rev-parse failed"):
                self.verify()
        self.assertFalse(self.report.exists())

    def test_temporary_parent_inside_checkout_is_refused(self) -> None:
        with self.assertRaisesRegex(runner.CheckError, "outside the checkout"):
            runner.verify(self.repo, self.repo)

    def test_missing_or_misplaced_git_marker_is_not_a_checkout(self) -> None:
        source = self.repo / "nested"
        source.mkdir()
        with self.assertRaisesRegex(runner.CheckError, "expected a primary or linked"):
            runner.verify(source, self.scratch)
        (source / ".git").mkdir()
        with self.assertRaisesRegex(runner.CheckError, "actual Git checkout root"):
            runner.verify(source, self.scratch)

    def test_canonical_entrypoint_and_ci_retain_focused_parallel_gate(self) -> None:
        command = "cargo test -p emuella-j2k-test-support --features emuella-j2k-core/parallel --test native_planes --test jp2_presentation"
        for path in ("scripts/check.sh", ".github/workflows/ci.yml"):
            text = (ROOT / path).read_text()
            self.assertIn(command, text)
            self.assertIn("sh scripts/check-lossy-ht-public-matrix.sh", text)
            self.assertIn("python3 scripts/test-check-committed-tree.py", text)

    def test_real_shell_dispatches_checkout_but_not_parent_repository(self) -> None:
        # Exercise the actual shell entry point with no-op check commands. The
        # synthetic repository never invokes Cargo or any real policy gate.
        tools = self.parent / "tools"
        tools.mkdir()
        python = tools / "python3"
        python.write_text(
            f'#!/bin/sh\nif [ "$1" = scripts/check-committed-tree.py ]; then\n'
            f"  exec '{sys.executable}' \"$@\"\nfi\nexit 0\n"
        )
        python.chmod(0o755)
        cargo = tools / "cargo"
        cargo.write_text(
            "#!/bin/sh\n"
            "listed=false\n"
            "ignored=false\n"
            'for argument in "$@"; do\n'
            '  [ "$argument" = --list ] && listed=true\n'
            '  [ "$argument" = --ignored ] && ignored=true\n'
            "done\n"
            'if [ "$listed" = true ]; then\n'
            "  if [ \"$ignored\" = false ]; then\n"
            "    echo 'ht_lossy_public_tests::lossy_ht_public_smoke: test'\n"
            "  fi\n"
            "  echo 'ht_lossy_public_tests::lossy_ht_public_complete_matrix: test'\n"
            "fi\n"
            "exit 0\n"
        )
        cargo.chmod(0o755)
        for name in (
            "check.sh",
            "check-committed-tree.py",
            "check-lossy-ht-public-matrix.sh",
        ):
            (self.repo / "scripts" / name).write_bytes(
                (ROOT / "scripts" / name).read_bytes()
            )
        self.commit()
        env = runner.environment()
        env.update(
            {
                "PATH": str(tools) + os.pathsep + os.environ["PATH"],
                "EMUELLA_CHECK_TMPDIR": str(self.scratch),
            }
        )
        result = subprocess.run(
            ["sh", "scripts/check.sh"],
            cwd=self.repo,
            env=env,
            capture_output=True,
            text=True,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn("Verification passed:", result.stdout)
        self.assertEqual(list(self.scratch.iterdir()), [])
        (self.repo / "source.txt").write_text("uncommitted source")
        result = subprocess.run(
            ["sh", "scripts/check.sh"],
            cwd=self.repo,
            env=env,
            capture_output=True,
            text=True,
        )
        self.assertEqual(result.returncode, 1)
        self.assertIn("Verification failed:", result.stderr)
        self.assertIn("commit first", result.stderr)
        self.assertNotIn("Verification passed:", result.stdout)
        nested = self.repo / "unpacked"
        (nested / "scripts").mkdir(parents=True)
        (nested / "scripts/check.sh").write_bytes(
            (ROOT / "scripts/check.sh").read_bytes()
        )
        (nested / "scripts/check-lossy-ht-public-matrix.sh").write_bytes(
            (ROOT / "scripts/check-lossy-ht-public-matrix.sh").read_bytes()
        )
        result = subprocess.run(
            ["sh", "scripts/check.sh"],
            cwd=nested,
            env=env,
            capture_output=True,
            text=True,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertNotIn("Verification passed:", result.stdout)


if __name__ == "__main__":
    unittest.main()
