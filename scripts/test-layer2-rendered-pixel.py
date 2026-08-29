#!/usr/bin/env python3
"""Self-contained tests for the rendered-pixel Layer 2 runner."""

from __future__ import annotations

import argparse
import contextlib
import importlib.util
import io
from pathlib import Path
import sys
import tempfile
import textwrap
import unittest
from unittest import mock


sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name("run-layer2-rendered-pixel.py")
SPEC = importlib.util.spec_from_file_location("rendered_pixel_runner", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
runner = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = runner
SPEC.loader.exec_module(runner)


def rendered_record() -> dict[str, object]:
    return {
        "id": "annex-g/project-authored",
        "input": "files/input.jp2",
        "reference": "files/reference.tif",
        "width": 3,
        "height": 5,
        "components": 3,
        "bits_per_sample": 8,
        "rendered_colour_space": "sRGB",
        "reference_layout": "tiff-rgb-u8-contiguous",
        "peak_error_limit": 4,
    }


def rendered_case() -> runner.RenderedCase:
    return runner.RenderedCase(
        id="annex-g/project-authored",
        input="files/input.jp2",
        reference="files/reference.tif",
        width=3,
        height=5,
        components=3,
        peak_error_limit=4,
    )


class ContractTests(unittest.TestCase):
    def setUp(self) -> None:
        self.lock = runner.common.Lock(
            "0" * 40, "suite", 1, "pack", "1", "1" * 64, "2" * 64
        )
        self.suite = {
            "rendered_pixel_comparison": {
                "pack_id": "pack",
                "standard": "project-authored authority fixture",
                "clauses": ["fixture"],
                "retrieval_commit": "3" * 40,
                "cases": [rendered_record()],
            }
        }
        self.inventory = {
            "assets": [
                {"path": "files/input.jp2"},
                {"path": "files/reference.tif"},
            ]
        }

    def test_loads_only_the_exact_inventory_backed_contract(self) -> None:
        self.assertEqual(
            runner.load_rendered_cases(self.suite, self.inventory, self.lock),
            (rendered_case(),),
        )

    def test_rejects_schema_metadata_and_inventory_drift(self) -> None:
        cases = self.suite["rendered_pixel_comparison"]["cases"]
        record = cases[0]
        mutations = [
            ("unexpected", True, "schema shape"),
            ("components", 4, "RGB-u8-contiguous"),
            ("bits_per_sample", 16, "RGB-u8-contiguous"),
            ("rendered_colour_space", "unknown", "RGB-u8-contiguous"),
            ("reference_layout", "raw", "RGB-u8-contiguous"),
            ("peak_error_limit", float("inf"), "integer range"),
            ("width", 0, "integer range"),
        ]
        for key, value, diagnostic in mutations:
            with self.subTest(key=key):
                original = record.get(key)
                record[key] = value
                with self.assertRaisesRegex(runner.RunnerError, diagnostic):
                    runner.load_rendered_cases(self.suite, self.inventory, self.lock)
                if original is None:
                    del record[key]
                else:
                    record[key] = original

        record["reference"] = "files/missing.tif"
        with self.assertRaisesRegex(runner.RunnerError, "absent from the locked inventory"):
            runner.load_rendered_cases(self.suite, self.inventory, self.lock)
        record["reference"] = "../escape.tif"
        with self.assertRaisesRegex(runner.common.RunnerError, "safe relative path"):
            runner.load_rendered_cases(self.suite, self.inventory, self.lock)

    def test_rejects_duplicate_case_input_reference_and_inventory_paths(self) -> None:
        plan = self.suite["rendered_pixel_comparison"]
        plan["cases"].append(dict(rendered_record(), id="annex-g/second"))
        with self.assertRaisesRegex(runner.RunnerError, "repeats an input"):
            runner.load_rendered_cases(self.suite, self.inventory, self.lock)
        plan["cases"].pop()
        self.inventory["assets"].append({"path": "files/input.jp2"})
        with self.assertRaisesRegex(runner.RunnerError, "repeats an asset"):
            runner.load_rendered_cases(self.suite, self.inventory, self.lock)

    def test_rejects_case_ids_that_are_not_report_safe_catalogue_tokens(self) -> None:
        record = self.suite["rendered_pixel_comparison"]["cases"][0]
        for case_id in [
            "",
            "Uppercase",
            " leading",
            "trailing ",
            "annex-g/injected\nsummary",
            "annex-g/control\x1b",
            "annex-g/non-ascii-é",
            "../escape",
            "annex-g/../escape",
            "/absolute",
            "trailing/",
        ]:
            with self.subTest(case_id=repr(case_id)):
                record["id"] = case_id
                with self.assertRaisesRegex(runner.RunnerError, "report-safe catalogue ID"):
                    runner.load_rendered_cases(self.suite, self.inventory, self.lock)


class WorkerTests(unittest.TestCase):
    def test_accepts_only_the_deterministic_aggregate_record(self) -> None:
        output = (
            "components=3 width=3 height=5 samples=45 peak=4 limit=4 passed=true"
        )
        self.assertTrue(runner.parse_worker_output(output, rendered_case()).passed)
        for malformed in [
            output + " payload=forbidden",
            output + " peak=4",
            output.replace("samples=45", "samples=44"),
            output.replace("passed=true", "passed=false"),
            output.replace("peak=4", "peak=-1"),
            output.replace("peak=4", "peak=256").replace("passed=true", "passed=false"),
        ]:
            with self.assertRaises(runner.RunnerError):
                runner.parse_worker_output(malformed, rendered_case())

    def test_invokes_only_the_dedicated_worker_with_a_finite_timeout(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            input_path = root / "input.jp2"
            reference_path = root / "reference.tif"
            input_path.write_bytes(b"project-authored input")
            reference_path.write_bytes(b"project-authored reference")
            codec = root / "fake-codec"
            codec.write_text(
                textwrap.dedent(
                    """\
                    #!/usr/bin/env python3
                    import sys
                    import time

                    assert sys.argv[1] == "compare-rendered-tiff-rgb"
                    assert sys.argv[2].endswith("input.jp2")
                    assert sys.argv[3].endswith("reference.tif")
                    if "--timeout-probe" in sys.argv:
                        time.sleep(2)
                    print("components=3 width=3 height=5 samples=45 peak=4 limit=4 passed=true")
                    """
                ),
                encoding="utf-8",
            )
            codec.chmod(0o755)
            result = runner.run_case(
                codec, input_path, reference_path, rendered_case(), 0.5
            )
            self.assertTrue(result.passed)

            sleeper = root / "sleep-codec"
            sleeper.write_text(
                "#!/usr/bin/env python3\nimport time\ntime.sleep(2)\n",
                encoding="utf-8",
            )
            sleeper.chmod(0o755)
            with self.assertRaisesRegex(runner.RunnerError, "finite timeout"):
                runner.run_case(
                    sleeper, input_path, reference_path, rendered_case(), 0.05
                )

    def test_scrubs_asset_paths_from_worker_diagnostics(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            input_path = root / "secret-input.jp2"
            reference_path = root / "secret-reference.tif"
            codec = root / "fake-codec"
            codec.write_text(
                "#!/bin/sh\nprintf '%s\\n' \"cannot open $2 or $3\" >&2\nexit 2\n",
                encoding="utf-8",
            )
            codec.chmod(0o755)
            with self.assertRaises(runner.RunnerError) as caught:
                runner.run_case(
                    codec, input_path, reference_path, rendered_case(), 0.5
                )
            diagnostic = str(caught.exception)
            self.assertNotIn(str(input_path), diagnostic)
            self.assertNotIn(str(reference_path), diagnostic)
            self.assertIn("<asset>", diagnostic)

    def test_preserves_an_out_of_limit_aggregate_as_a_failed_result(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            codec = root / "fake-codec"
            codec.write_text(
                "#!/bin/sh\n"
                "printf '%s\\n' 'components=3 width=3 height=5 samples=45 peak=5 limit=4 passed=false'\n"
                "printf '%s\\n' 'rendered samples exceed the comparison limit' >&2\n"
                "exit 2\n",
                encoding="utf-8",
            )
            codec.chmod(0o755)
            result = runner.run_case(
                codec,
                root / "input.jp2",
                root / "reference.tif",
                rendered_case(),
                0.5,
            )
            self.assertFalse(result.passed)
            self.assertEqual(result.peak, 5)


class ResolutionAndExecutionTests(unittest.TestCase):
    def test_resolution_rejects_symlinks_and_escape(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary).resolve()
            (root / "files").mkdir()
            outside = root.parent / f"{root.name}-outside"
            outside.write_bytes(b"outside")
            try:
                (root / "files/link.jp2").symlink_to(outside)
                with self.assertRaisesRegex(runner.RunnerError, "inside the verified pack"):
                    runner.resolve_inventory_path(root, "files/link.jp2", "rendered input")
            finally:
                outside.unlink()

    def test_execute_uses_preflight_cleanliness_integrity_and_snapshot_checks(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            testdata = root / "testdata"
            pack_root = root / "pack"
            (pack_root / "files").mkdir(parents=True)
            input_path = pack_root / "files/input.jp2"
            reference_path = pack_root / "files/reference.tif"
            input_path.write_bytes(b"project-authored input")
            reference_path.write_bytes(b"project-authored reference")
            codec = root / "fake-codec"
            codec.write_text(
                "#!/bin/sh\n"
                "printf '%s\\n' 'components=3 width=3 height=5 samples=45 peak=4 limit=4 passed=true'\n",
                encoding="utf-8",
            )
            codec.chmod(0o755)
            lock = runner.common.Lock(
                "0" * 40, "suite", 1, "pack", "1", "1" * 64, "2" * 64
            )
            suite = {
                "rendered_pixel_comparison": {
                    "pack_id": "pack",
                    "standard": "project-authored authority fixture",
                    "clauses": ["fixture"],
                    "retrieval_commit": "3" * 40,
                    "cases": [rendered_record()],
                }
            }
            inventory = {
                "assets": [
                    {"path": "files/input.jp2"},
                    {"path": "files/reference.tif"},
                ]
            }
            identity = runner.common.CodecIdentity(codec, "4" * 64, None, None)
            snapshot = runner.common.CodecSnapshot(codec, "4" * 64)
            arguments = argparse.Namespace(
                lock=root / "lock.toml",
                testdata=testdata,
                pack_root=pack_root,
                codec=codec,
                unbound_codec=True,
                timeout_seconds=1.0,
            )
            events: list[str] = []

            @contextlib.contextmanager
            def snapshot_context(_identity):
                events.append("snapshot")
                yield snapshot

            with (
                mock.patch.object(runner.common, "load_lock", return_value=lock),
                mock.patch.object(
                    runner.common,
                    "verify_catalogue_checkout",
                    side_effect=lambda *_: events.append("catalogue-clean"),
                ) as verify_catalogue,
                mock.patch.object(
                    runner.common,
                    "validate_contract",
                    return_value=(suite, {}, inventory, [], pack_root),
                ),
                mock.patch.object(
                    runner.common,
                    "verify_materialisation",
                    side_effect=lambda *_: (events.append("materialisation") or (2, 48)),
                ) as verify_materialisation,
                mock.patch.object(
                    runner.common, "inspect_codec_identity", return_value=identity
                ) as inspect_identity,
                mock.patch.object(runner.common, "snapshot_codec", snapshot_context),
                mock.patch.object(runner.common, "print_codec_identity"),
                mock.patch.object(
                    runner.common,
                    "verify_codec_identity_unchanged",
                    side_effect=lambda *_: events.append("codec-unchanged"),
                ) as verify_unchanged,
            ):
                output = io.StringIO()
                with contextlib.redirect_stdout(output):
                    self.assertEqual(runner.execute(arguments), 0)

            self.assertEqual(
                events,
                ["catalogue-clean", "materialisation", "snapshot", "codec-unchanged"],
            )
            verify_catalogue.assert_called_once()
            verify_materialisation.assert_called_once()
            inspect_identity.assert_called_once_with(codec, True)
            verify_unchanged.assert_called_once_with(identity)
            report = output.getvalue()
            self.assertIn("pack=pack@1 case=annex-g/project-authored", report)
            self.assertIn("summary cases=1 passed=1 failed=0", report)
            self.assertNotIn("files/input.jp2", report)
            self.assertNotIn("files/reference.tif", report)

    def test_integrity_failure_prevents_codec_identity_or_invocation(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            arguments = argparse.Namespace(
                lock=root / "lock.toml",
                testdata=root / "testdata",
                pack_root=root / "pack",
                codec=root / "codec",
                unbound_codec=True,
                timeout_seconds=1.0,
            )
            lock = runner.common.Lock(
                "0" * 40, "suite", 1, "pack", "1", "1" * 64, "2" * 64
            )
            suite = {
                "rendered_pixel_comparison": {
                    "pack_id": "pack",
                    "standard": "fixture",
                    "clauses": ["fixture"],
                    "retrieval_commit": "3" * 40,
                    "cases": [rendered_record()],
                }
            }
            inventory = {
                "assets": [
                    {"path": "files/input.jp2"},
                    {"path": "files/reference.tif"},
                ]
            }
            with (
                mock.patch.object(runner.common, "load_lock", return_value=lock),
                mock.patch.object(runner.common, "verify_catalogue_checkout"),
                mock.patch.object(
                    runner.common,
                    "validate_contract",
                    return_value=(suite, {}, inventory, [], root / "pack"),
                ),
                mock.patch.object(
                    runner.common,
                    "verify_materialisation",
                    side_effect=runner.common.RunnerError("tree SHA-256 mismatch"),
                ),
                mock.patch.object(runner.common, "inspect_codec_identity") as inspect,
                self.assertRaisesRegex(runner.common.RunnerError, "tree SHA-256 mismatch"),
            ):
                runner.execute(arguments)
            inspect.assert_not_called()


if __name__ == "__main__":
    unittest.main()
