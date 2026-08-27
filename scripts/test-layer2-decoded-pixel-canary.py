#!/usr/bin/env python3
"""Self-contained tests for the decoded-pixel Layer 2 runner."""

from __future__ import annotations

import importlib.util
from pathlib import Path
import sys
import tempfile
import textwrap
import unittest


sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name("run-layer2-decoded-pixel-canary.py")
SPEC = importlib.util.spec_from_file_location("decoded_pixel_runner", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
runner = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = runner
SPEC.loader.exec_module(runner)


def comparison_case() -> runner.ComparisonCase:
    return runner.ComparisonCase(
        id=runner.DEFAULT_CASE,
        input="files/input.j2k",
        reference="files/reference.pgx",
        component=0,
        output_window=False,
        resolution_reduction=0,
        output_origin_x=0,
        output_origin_y=0,
        width=2,
        height=2,
        bits_per_sample=8,
        signed=False,
        peak_error_limit=0,
        mean_squared_error_limit=0.0,
    )


class ContractTests(unittest.TestCase):
    def setUp(self) -> None:
        self.lock = runner.common.Lock(
            "0" * 40, "suite", 1, "pack", "1", "1" * 64, "2" * 64
        )
        self.inventory = {
            "assets": [
                {"path": "files/input.j2k"},
                {"path": "files/reference.pgx"},
                {"path": "files/reduced-reference.pgx"},
            ]
        }
        self.suite = {
            "decoded_pixel_comparison": {
                "pack_id": "pack",
                "standard": "project-authored authority fixture",
                "clauses": ["fixture"],
                "retrieval_commit": "3" * 40,
                "cases": [
                    {
                        "id": runner.DEFAULT_CASE,
                        "input": "files/input.j2k",
                        "reference": "files/reference.pgx",
                        "component": 0,
                        "resolution_reduction": 0,
                        "width": 2,
                        "height": 2,
                        "bits_per_sample": 8,
                        "signed": False,
                        "peak_error_limit": 0,
                        "mean_squared_error_limit": 0.0,
                    }
                ],
                "choice_groups": [
                    {
                        "id": "project-authored/choice",
                        "input": "files/input.j2k",
                        "minimum_passing_alternatives": 1,
                        "alternatives": [
                            {
                                "id": "window",
                                "reference": "files/reference.pgx",
                                "component": 0,
                                "resolution_reduction": 0,
                                "output_origin_x": 1,
                                "output_origin_y": 2,
                                "width": 2,
                                "height": 2,
                                "bits_per_sample": 8,
                                "signed": False,
                                "peak_error_limit": 0,
                                "mean_squared_error_limit": 0.0,
                            },
                            {
                                "id": "reduced",
                                "reference": "files/reduced-reference.pgx",
                                "component": 0,
                                "resolution_reduction": 1,
                                "output_origin_x": 0,
                                "output_origin_y": 0,
                                "width": 2,
                                "height": 2,
                                "bits_per_sample": 8,
                                "signed": False,
                                "peak_error_limit": 0,
                                "mean_squared_error_limit": 0.0,
                            },
                        ],
                    }
                ],
            }
        }

    def test_loads_explicit_inventory_backed_case(self) -> None:
        case = runner.load_comparison_case(
            self.suite, self.inventory, self.lock, runner.DEFAULT_CASE
        )
        self.assertEqual(case, comparison_case())

    def test_loads_three_level_reduced_scalar_case(self) -> None:
        record = self.suite["decoded_pixel_comparison"]["cases"][0]
        record["reference"] = "files/reduced-reference.pgx"
        record["resolution_reduction"] = 3
        case = runner.load_comparison_case(
            self.suite, self.inventory, self.lock, runner.DEFAULT_CASE
        )
        self.assertFalse(case.output_window)
        self.assertEqual(case.resolution_reduction, 3)

        record["resolution_reduction"] = 4
        with self.assertRaisesRegex(runner.RunnerError, "supported integer range"):
            runner.load_comparison_case(
                self.suite, self.inventory, self.lock, runner.DEFAULT_CASE
            )

    def test_loads_explicit_scalar_output_window(self) -> None:
        record = self.suite["decoded_pixel_comparison"]["cases"][0]
        record["output_window"] = True
        record["output_origin_x"] = 7
        record["output_origin_y"] = 9
        case = runner.load_comparison_case(
            self.suite, self.inventory, self.lock, runner.DEFAULT_CASE
        )
        self.assertTrue(case.output_window)
        self.assertEqual((case.output_origin_x, case.output_origin_y), (7, 9))

    def test_rejects_scalar_origin_without_output_window(self) -> None:
        record = self.suite["decoded_pixel_comparison"]["cases"][0]
        record["output_origin_x"] = 1
        with self.assertRaisesRegex(
            runner.RunnerError, "full-component comparison cannot select an output origin"
        ):
            runner.load_comparison_case(
                self.suite, self.inventory, self.lock, runner.DEFAULT_CASE
            )

        record["output_origin_x"] = 0
        record["output_window"] = "true"
        with self.assertRaisesRegex(runner.RunnerError, "flag is not Boolean"):
            runner.load_comparison_case(
                self.suite, self.inventory, self.lock, runner.DEFAULT_CASE
            )

    def test_rejects_missing_or_unsafe_reference(self) -> None:
        self.suite["decoded_pixel_comparison"]["cases"][0]["reference"] = (
            "../escape.pgx"
        )
        with self.assertRaisesRegex(runner.common.RunnerError, "safe relative path"):
            runner.load_comparison_case(
                self.suite, self.inventory, self.lock, runner.DEFAULT_CASE
            )
        self.suite["decoded_pixel_comparison"]["cases"][0]["reference"] = (
            "files/missing.pgx"
        )
        with self.assertRaisesRegex(
            runner.RunnerError, "absent from the locked inventory"
        ):
            runner.load_comparison_case(
                self.suite, self.inventory, self.lock, runner.DEFAULT_CASE
            )

    def test_rejects_non_finite_limit(self) -> None:
        self.suite["decoded_pixel_comparison"]["cases"][0][
            "mean_squared_error_limit"
        ] = float("inf")
        with self.assertRaisesRegex(runner.RunnerError, "finite and non-negative"):
            runner.load_comparison_case(
                self.suite, self.inventory, self.lock, runner.DEFAULT_CASE
            )

    def test_loads_explicit_choice_group_semantics(self) -> None:
        selection = runner.load_comparison_selection(
            self.suite, self.inventory, self.lock, "project-authored/choice"
        )
        self.assertTrue(selection.is_choice_group)
        self.assertEqual(selection.minimum_passing, 1)
        self.assertEqual(
            [
                (
                    alternative.id,
                    alternative.output_window,
                    alternative.resolution_reduction,
                    alternative.output_origin_x,
                    alternative.output_origin_y,
                )
                for alternative in selection.alternatives
            ],
            [("window", True, 0, 1, 2), ("reduced", True, 1, 0, 0)],
        )

    def test_rejects_unbounded_choice_group(self) -> None:
        group = self.suite["decoded_pixel_comparison"]["choice_groups"][0]
        group["minimum_passing_alternatives"] = 3
        with self.assertRaisesRegex(runner.RunnerError, "supported integer range"):
            runner.load_comparison_selection(
                self.suite, self.inventory, self.lock, "project-authored/choice"
            )
        group["minimum_passing_alternatives"] = 1
        group["alternatives"][0]["resolution_reduction"] = 2
        with self.assertRaisesRegex(runner.RunnerError, "supported integer range"):
            runner.load_comparison_selection(
                self.suite, self.inventory, self.lock, "project-authored/choice"
            )


class WorkerTests(unittest.TestCase):
    def test_accepts_only_factual_aggregate_record(self) -> None:
        result = runner.parse_worker_output(
            "component=0 width=2 height=2 samples=4 peak_error=0 "
            "mean_squared_error=0 peak_error_limit=0 "
            "mean_squared_error_limit=0 passed=true",
            comparison_case(),
        )
        self.assertTrue(result.passed)
        with self.assertRaisesRegex(runner.RunnerError, "unexpected aggregate"):
            runner.parse_worker_output(
                "component=0 width=2 height=2 samples=4 peak_error=0 "
                "mean_squared_error=0 peak_error_limit=0 "
                "mean_squared_error_limit=0 passed=true payload=forbidden",
                comparison_case(),
            )

    def test_worker_isolated_with_finite_timeout(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            codec = root / "fake-codec"
            codec.write_text(
                textwrap.dedent(
                    """\
                    #!/usr/bin/env python3
                    import sys
                    import time

                    if "timeout.j2k" in sys.argv[2]:
                        time.sleep(2)
                    print("component=0 width=2 height=2 samples=4 peak_error=0 "
                          "mean_squared_error=0 peak_error_limit=0 "
                          "mean_squared_error_limit=0 passed=true")
                    """
                ),
                encoding="utf-8",
            )
            codec.chmod(0o755)
            case = comparison_case()
            result = runner.run_case(codec, root, case, 0.5)
            self.assertTrue(result.passed)
            timed = runner.dataclasses.replace(case, input="files/timeout.j2k")
            with self.assertRaisesRegex(runner.RunnerError, "finite timeout"):
                runner.run_case(codec, root, timed, 0.1)


if __name__ == "__main__":
    unittest.main()
