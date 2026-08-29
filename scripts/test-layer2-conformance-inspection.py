#!/usr/bin/env python3
"""Self-contained tests for the optional Layer 2 inspection runner."""

from __future__ import annotations

import argparse
import contextlib
import dataclasses
import hashlib
import importlib.util
import io
from pathlib import Path
import subprocess
import sys
import tempfile
import textwrap
import unittest
from unittest import mock


sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name("run-layer2-conformance-inspection.py")
SPEC = importlib.util.spec_from_file_location("layer2_runner", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
runner = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = runner
SPEC.loader.exec_module(runner)


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def make_candidate(path: str, expected: str = "accept", diagnostic: str | None = None):
    return runner.Candidate(path, "j2k", "annex-c/test", expected, diagnostic)


class ContractTests(unittest.TestCase):
    def setUp(self) -> None:
        self.lock = runner.Lock("0" * 40, "suite", 1, "pack", "1", "1" * 64, "2" * 64)
        self.suite = {
            "inspection": {
                "pack_id": "pack",
                "extensions": [".j2k", ".htj2k", ".jp2", ".jph"],
                "expected": "accept",
                "classifications": [
                    {
                        "path_prefix": "files/positive/",
                        "format": "j2k",
                        "cohort": "annex-c/test",
                    }
                ],
            }
        }

    def test_candidates_are_sorted_and_classified(self) -> None:
        inventory = {
            "assets": [
                {"path": "files/positive/b.j2k"},
                {"path": "files/ignored.txt"},
                {"path": "files/positive/a.j2k"},
            ]
        }
        candidates = runner.load_candidates(self.suite, inventory, self.lock)
        self.assertEqual(
            [candidate.path for candidate in candidates],
            [
                "files/positive/a.j2k",
                "files/positive/b.j2k",
            ],
        )

    def test_empty_selection_is_rejected(self) -> None:
        with self.assertRaisesRegex(runner.RunnerError, "selection is empty"):
            runner.load_candidates(
                self.suite, {"assets": [{"path": "files/ignored.txt"}]}, self.lock
            )

    def test_unclassified_input_is_rejected(self) -> None:
        with self.assertRaisesRegex(runner.RunnerError, "unclassified"):
            runner.load_candidates(
                self.suite,
                {"assets": [{"path": "files/other/input.j2k"}]},
                self.lock,
            )

    def test_rejection_requires_a_diagnostic(self) -> None:
        self.suite["inspection"]["expected"] = "reject"
        with self.assertRaisesRegex(runner.RunnerError, "lacks a diagnostic"):
            runner.load_candidates(
                self.suite,
                {"assets": [{"path": "files/positive/input.j2k"}]},
                self.lock,
            )

    def test_timeout_must_be_finite_and_positive(self) -> None:
        for invalid in ("nan", "inf", "-inf", "0", "-1"):
            with self.subTest(invalid=invalid):
                with self.assertRaises(argparse.ArgumentTypeError):
                    runner.positive_finite_seconds(invalid)
        self.assertEqual(runner.positive_finite_seconds("0.25"), 0.25)

    def test_malformed_lock_has_controlled_failures(self) -> None:
        cases = {
            "suite = 1\n": "lock suite is not a TOML table",
            "[suite]\nid = 1\nrevision = 1\n": "lock suite id is empty or not text",
            '[suite]\nid = "suite"\nrevision = 0\n': "lock suite revision is not a positive integer",
        }
        for suite_text, expected in cases.items():
            with (
                self.subTest(expected=expected),
                tempfile.TemporaryDirectory() as temporary,
            ):
                lock = Path(temporary) / "lock.toml"
                lock.write_text(
                    "schema_version = 1\n"
                    f'catalogue_commit = "{"0" * 40}"\n'
                    f"{suite_text}"
                    "[pack]\n"
                    'id = "pack"\nversion = "1"\n'
                    f'archive_sha256 = "{"1" * 64}"\n'
                    f'tree_sha256 = "{"2" * 64}"\n',
                    encoding="utf-8",
                )
                with self.assertRaisesRegex(runner.RunnerError, expected):
                    runner.load_lock(lock)


class ProcessIsolationTests(unittest.TestCase):
    def test_outcomes_are_isolated_and_compared(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            codec = root / "fake-codec"
            codec.write_text(
                textwrap.dedent(
                    """\
                    #!/usr/bin/env python3
                    import os
                    from pathlib import Path
                    import signal
                    import sys
                    import time

                    name = Path(sys.argv[-1]).name
                    if name == "reject.j2k":
                        print("class=invalid-marker", file=sys.stderr)
                        raise SystemExit(2)
                    if name == "crash.j2k":
                        os.kill(os.getpid(), signal.SIGKILL)
                    if name == "timeout.j2k":
                        time.sleep(2)
                    raise SystemExit(0)
                    """
                ),
                encoding="utf-8",
            )
            codec.chmod(0o755)
            candidates = [
                make_candidate("accept-before.j2k"),
                make_candidate("crash.j2k"),
                make_candidate("reject.j2k", "reject", "class=invalid-marker"),
                make_candidate("timeout.j2k"),
                make_candidate("accept-after.j2k"),
            ]
            results = runner.run_candidates(codec, root, candidates, 0.5)
            self.assertEqual(
                [result.actual for result in results],
                ["accept", "crash", "reject", "timeout", "accept"],
            )
            self.assertEqual(
                [result.anomaly for result in results],
                [None, "crash", None, "timeout", None],
            )

    def test_unexpected_acceptance_rejection_and_diagnostic(self) -> None:
        results = [
            runner.Result(make_candidate("positive.j2k"), "reject", "bad"),
            runner.Result(make_candidate("negative.j2k", "reject", "wanted"), "accept"),
            runner.Result(
                make_candidate("diagnostic.j2k", "reject", "wanted"), "reject", "other"
            ),
        ]
        self.assertEqual(
            [result.anomaly for result in results],
            ["unexpected rejection", "unexpected acceptance", "unexpected diagnostic"],
        )

    def test_summary_is_ordered_by_format_then_cohort(self) -> None:
        results = [
            runner.Result(
                runner.Candidate("d.jph", "jph", "annex-g", "accept", None), "accept"
            ),
            runner.Result(
                runner.Candidate("b.j2k", "j2k", "zeta", "accept", None), "accept"
            ),
            runner.Result(
                runner.Candidate("c.jp2", "jp2", "annex-g", "accept", None), "timeout"
            ),
            runner.Result(
                runner.Candidate("a.j2k", "j2k", "alpha", "accept", None), "reject"
            ),
            runner.Result(
                runner.Candidate("h.j2k", "htj2k", "annex-c", "accept", None), "crash"
            ),
        ]
        output = io.StringIO()
        with contextlib.redirect_stdout(output):
            runner.print_summary(results)
        rows = output.getvalue().splitlines()[1:]
        self.assertEqual(
            [(row.split()[0], row.split()[1]) for row in rows],
            [
                ("j2k", "alpha"),
                ("j2k", "zeta"),
                ("htj2k", "annex-c"),
                ("jp2", "annex-g"),
                ("jph", "annex-g"),
            ],
        )


class CodecIdentityTests(unittest.TestCase):
    def test_bound_and_unbound_codec_identity(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            checkout = root / "checkout"
            codec = checkout / "target/debug/emuella-j2k"
            codec.parent.mkdir(parents=True)
            original = b"#!/bin/sh\nexit 0\n"
            codec.write_bytes(original)
            codec.chmod(0o755)
            subprocess.run(["git", "init", "-q", str(checkout)], check=True)
            subprocess.run(
                ["git", "-C", str(checkout), "config", "user.name", "Test"], check=True
            )
            subprocess.run(
                [
                    "git",
                    "-C",
                    str(checkout),
                    "config",
                    "user.email",
                    "test@example.invalid",
                ],
                check=True,
            )
            subprocess.run(["git", "-C", str(checkout), "add", "-f", "."], check=True)
            subprocess.run(
                ["git", "-C", str(checkout), "commit", "-qm", "fixture"], check=True
            )

            with (
                mock.patch.dict(runner.os.environ, {}, clear=True),
                mock.patch.object(runner, "ROOT", checkout),
            ):
                identity = runner.inspect_codec_identity(codec, False)
                self.assertEqual(identity.sha256, digest(original))
                self.assertIsNotNone(identity.commit)
                runner.verify_codec_identity_unchanged(identity)

            outside = root / "outside-codec"
            outside.write_bytes(original)
            outside.chmod(0o755)
            with (
                mock.patch.dict(runner.os.environ, {}, clear=True),
                mock.patch.object(runner, "ROOT", checkout),
                self.assertRaisesRegex(runner.RunnerError, "canonical executable"),
            ):
                runner.inspect_codec_identity(outside, False)
            unbound = runner.inspect_codec_identity(outside, True)
            self.assertIsNone(unbound.commit)

            codec.write_text("#!/bin/sh\nexit 1\n", encoding="utf-8")
            with (
                mock.patch.dict(runner.os.environ, {}, clear=True),
                mock.patch.object(runner, "ROOT", checkout),
                self.assertRaisesRegex(runner.RunnerError, "tracked modifications"),
            ):
                runner.inspect_codec_identity(codec, False)
            with self.assertRaisesRegex(runner.RunnerError, "changed during"):
                runner.verify_codec_identity_unchanged(identity)

    def test_snapshot_is_the_executable_used_if_source_changes_and_is_restored(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "source-codec"
            marker = root / "marker"
            original = (
                "#!/usr/bin/env python3\n"
                "from pathlib import Path\n"
                f"Path({str(marker)!r}).write_text('snapshot', encoding='utf-8')\n"
            ).encode()
            replacement = b"#!/bin/sh\nexit 9\n"
            source.write_bytes(original)
            source.chmod(0o755)
            identity = runner.inspect_codec_identity(source, True)
            candidate = make_candidate("input.j2k")

            with runner.snapshot_codec(identity) as snapshot:
                source.write_bytes(replacement)
                source.write_bytes(original)
                source.chmod(0o755)
                results = runner.run_candidates(snapshot.path, root, [candidate], 1.0)

            runner.verify_codec_identity_unchanged(identity)
            self.assertEqual([result.actual for result in results], ["accept"])
            self.assertEqual(marker.read_text(encoding="utf-8"), "snapshot")
            self.assertEqual(snapshot.sha256, digest(original))

    def test_snapshot_removal_is_detected(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / "self-deleting-codec"
            source.write_text('#!/bin/sh\nrm -- "$0"\nexit 0\n', encoding="utf-8")
            source.chmod(0o755)
            identity = runner.inspect_codec_identity(source, True)

            with self.assertRaisesRegex(runner.RunnerError, "snapshot disappeared"):
                with runner.snapshot_codec(identity) as snapshot:
                    results = runner.run_candidates(
                        snapshot.path, root, [make_candidate("input.j2k")], 1.0
                    )
                    self.assertEqual([result.actual for result in results], ["accept"])


class IntegrityOrderTests(unittest.TestCase):
    def test_integrity_mismatch_fails_before_codec_invocation(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            testdata = root / "testdata"
            (testdata / "suites").mkdir(parents=True)
            (testdata / "manifests").mkdir()
            (testdata / "inventories").mkdir()
            pack_root = testdata / "artifacts/pack/1"
            (pack_root / "files").mkdir(parents=True)
            archive_bytes = b"project-authored archive placeholder"
            input_bytes = b"project-authored codestream placeholder"
            (pack_root / "archive.zip").write_bytes(archive_bytes)
            (pack_root / "files/input.j2k").write_bytes(input_bytes)
            archive_digest = digest(archive_bytes)
            input_digest = digest(input_bytes)
            records = [
                ("archive.zip", len(archive_bytes), archive_digest),
                ("files/input.j2k", len(input_bytes), input_digest),
            ]
            tree_digest = runner.tree_sha256(records)
            (testdata / "suites/suite.toml").write_text(
                textwrap.dedent(
                    """\
                    schema_version = 1
                    id = "suite"
                    revision = 1
                    layer = 2
                    gating = true
                    missing_policy = "fail"

                    [[packs]]
                    id = "pack"
                    version = "1"
                    required = true

                    [inspection]
                    pack_id = "pack"
                    extensions = [".j2k", ".htj2k", ".jp2", ".jph"]
                    expected = "accept"

                    [[inspection.classifications]]
                    path_prefix = "files/"
                    format = "j2k"
                    cohort = "annex-c/test"
                    """
                ),
                encoding="utf-8",
            )
            (testdata / "manifests/pack.toml").write_text(
                textwrap.dedent(
                    f"""\
                    schema_version = 1
                    id = "pack"
                    version = "1"
                    review_state = "locked"
                    asset_inventory = "inventories/pack.toml"

                    [source]
                    archive_filename = "archive.zip"
                    archive_sha256 = "{archive_digest}"

                    [materialization]
                    directory = "artifacts/pack/1"
                    expected_tree_sha256 = "{tree_digest}"
                    """
                ),
                encoding="utf-8",
            )
            (testdata / "inventories/pack.toml").write_text(
                textwrap.dedent(
                    f"""\
                    schema_version = 1
                    pack_id = "pack"
                    pack_version = "1"

                    [[assets]]
                    path = "archive.zip"
                    bytes = {len(archive_bytes)}
                    sha256 = "{archive_digest}"

                    [[assets]]
                    path = "files/input.j2k"
                    bytes = {len(input_bytes)}
                    sha256 = "{input_digest}"
                    """
                ),
                encoding="utf-8",
            )
            subprocess.run(["git", "init", "-q", str(testdata)], check=True)
            subprocess.run(
                ["git", "-C", str(testdata), "config", "user.name", "Test"], check=True
            )
            subprocess.run(
                [
                    "git",
                    "-C",
                    str(testdata),
                    "config",
                    "user.email",
                    "test@example.invalid",
                ],
                check=True,
            )
            subprocess.run(
                [
                    "git",
                    "-C",
                    str(testdata),
                    "add",
                    "suites",
                    "manifests",
                    "inventories",
                ],
                check=True,
            )
            subprocess.run(
                ["git", "-C", str(testdata), "commit", "-qm", "fixture"], check=True
            )
            commit = subprocess.run(
                ["git", "-C", str(testdata), "rev-parse", "HEAD"],
                check=True,
                capture_output=True,
                text=True,
            ).stdout.strip()
            lock = root / "lock.toml"
            lock.write_text(
                textwrap.dedent(
                    f"""\
                    schema_version = 1
                    catalogue_commit = "{commit}"

                    [suite]
                    id = "suite"
                    revision = 1

                    [pack]
                    id = "pack"
                    version = "1"
                    archive_sha256 = "{archive_digest}"
                    tree_sha256 = "{tree_digest}"
                    """
                ),
                encoding="utf-8",
            )
            sentinel = root / "codec-ran"
            codec = root / "fake-codec"
            codec.write_text(
                f"#!/bin/sh\ntouch '{sentinel}'\nexit 0\n", encoding="utf-8"
            )
            codec.chmod(0o755)
            (pack_root / "files/input.j2k").write_bytes(b"integrity mismatch")
            arguments = argparse.Namespace(
                lock=lock,
                testdata=testdata,
                pack_root=None,
                codec=codec,
                timeout_seconds=1.0,
                unbound_codec=True,
            )
            with self.assertRaisesRegex(runner.RunnerError, "size mismatch"):
                runner.execute(arguments)
            self.assertFalse(sentinel.exists())

            (pack_root / "files/input.j2k").write_bytes(input_bytes)
            output = io.StringIO()
            with contextlib.redirect_stdout(output):
                self.assertEqual(runner.execute(arguments), 0)
            report = output.getvalue()
            self.assertTrue(sentinel.exists())
            self.assertIn(f"verified catalogue {commit}", report)
            self.assertIn(f"SHA-256 {digest(codec.read_bytes())}", report)
            self.assertIn("source revision unbound (diagnostic evidence only)", report)
            self.assertIn("j2k", report)

            wrong_lock = dataclasses.replace(
                runner.load_lock(lock), catalogue_commit="f" * 40
            )
            with self.assertRaisesRegex(
                runner.RunnerError, "catalogue commit mismatch"
            ):
                runner.verify_catalogue_checkout(testdata, wrong_lock)
            (testdata / "suites/suite.toml").write_text(
                "tracked edit\n", encoding="utf-8"
            )
            with self.assertRaisesRegex(runner.RunnerError, "tracked modifications"):
                runner.verify_catalogue_checkout(testdata, runner.load_lock(lock))


if __name__ == "__main__":
    unittest.main()
