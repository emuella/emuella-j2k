#!/usr/bin/env python3
"""Self-contained tests for the decoded-pixel derived-set runner."""

from __future__ import annotations

import copy
import hashlib
import importlib.util
import io
from pathlib import Path
import sys
import tempfile
import textwrap
import unittest
from unittest import mock


sys.dont_write_bytecode = True
SCRIPT = Path(__file__).with_name("run-layer2-derived-set.py")
SPEC = importlib.util.spec_from_file_location("derived_set_runner", SCRIPT)
assert SPEC is not None and SPEC.loader is not None
runner = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = runner
SPEC.loader.exec_module(runner)


def capability(m_magb: int = 18) -> runner.Capability:
    return runner.Capability(
        "DS0", 0, 0, m_magb, frozenset({"HTONLY"}), frozenset({"HTMIX"})
    )


def fixture_contract() -> tuple[runner.common.Lock, dict, dict]:
    lock = runner.common.Lock(
        "0" * 40, "suite", 19, "pack", "1", "1" * 64, "2" * 64
    )
    inventory = {
        "assets": [
            {"path": path}
            for path in (
                "files/reference-input.j2k",
                "files/reference.pgx",
                "files/reduced.pgx",
                "files/htonly-b11.j2k",
                "files/htonly-b15.j2k",
                "files/choice-b12.j2k",
                "files/htmix-b8.j2k",
            )
        ]
    }
    plan = {
        "pack_id": "pack",
        "standard": "project-authored authority fixture",
        "clauses": ["fixture"],
        "retrieval_commit": "3" * 40,
        "output_normalisation": copy.deepcopy(runner.EXPECTED_NORMALISATION),
        "cases": [
            {
                "id": "reference/scalar",
                "input": "files/reference-input.j2k",
                "reference": "files/reference.pgx",
                "component": 0,
                "resolution_reduction": 0,
                "width": 2,
                "height": 2,
                "bits_per_sample": 8,
                "signed": False,
                "peak_error_limit": 99,
                "mean_squared_error_limit": 99.0,
            }
        ],
        "choice_groups": [
            {
                "id": "reference/choice",
                "input": "files/reference-input.j2k",
                "minimum_passing_alternatives": 1,
                "alternatives": [
                    {
                        "id": "full",
                        "reference": "files/reference.pgx",
                        "component": 0,
                        "resolution_reduction": 0,
                        "output_origin_x": 0,
                        "output_origin_y": 0,
                        "width": 2,
                        "height": 2,
                        "bits_per_sample": 8,
                        "signed": False,
                        "peak_error_limit": 99,
                        "mean_squared_error_limit": 99.0,
                    },
                    {
                        "id": "reduced",
                        "reference": "files/reduced.pgx",
                        "component": 0,
                        "resolution_reduction": 1,
                        "output_origin_x": 0,
                        "output_origin_y": 0,
                        "width": 2,
                        "height": 2,
                        "bits_per_sample": 8,
                        "signed": False,
                        "peak_error_limit": 99,
                        "mean_squared_error_limit": 99.0,
                    },
                ],
            }
        ],
        "derived_sets": [
            {
                "id": "DS0",
                "profile": 0,
                "compliance_class": 0,
                "selection": runner.SELECTION_RULE,
                "cases": [
                    {
                        "id": "derived/htonly/scalar",
                        "coding_mode": "HTONLY",
                        "reference_case_id": "reference/scalar",
                        "variants": [
                            {
                                "input": "files/htonly-b11.j2k",
                                "b_magb": 11,
                                "component_limits": [
                                    {
                                        "component": 0,
                                        "peak_error_limit": 7,
                                        "mean_squared_error_limit": 1.5,
                                    }
                                ],
                            },
                            {
                                "input": "files/htonly-b15.j2k",
                                "b_magb": 15,
                                "component_limits": [
                                    {
                                        "component": 0,
                                        "peak_error_limit": 2,
                                        "mean_squared_error_limit": 0.25,
                                    }
                                ],
                            },
                        ],
                    },
                    {
                        "id": "derived/htonly/choice",
                        "coding_mode": "HTONLY",
                        "reference_case_id": "reference/choice",
                        "variants": [
                            {
                                "input": "files/choice-b12.j2k",
                                "b_magb": 12,
                                "alternative_limits": [
                                    {
                                        "alternative_id": "full",
                                        "peak_error_limit": 4,
                                        "mean_squared_error_limit": 0.5,
                                    },
                                    {
                                        "alternative_id": "reduced",
                                        "peak_error_limit": 5,
                                        "mean_squared_error_limit": 0.75,
                                    },
                                ],
                            }
                        ],
                    },
                    {
                        "id": "derived/htmix/scalar",
                        "coding_mode": "HTMIX",
                        "reference_case_id": "reference/scalar",
                        "variants": [
                            {
                                "input": "files/htmix-b8.j2k",
                                "b_magb": 8,
                                "component_limits": [
                                    {
                                        "component": 0,
                                        "peak_error_limit": 0,
                                        "mean_squared_error_limit": 0.0,
                                    }
                                ],
                            }
                        ],
                    },
                ],
            }
        ],
    }
    return lock, {"decoded_pixel_comparison": plan}, inventory


class CapabilityLockTests(unittest.TestCase):
    def write_lock(self, root: Path, claim: str) -> Path:
        path = root / "testdata.lock.toml"
        path.write_text(
            textwrap.dedent(
                f"""\
                schema_version = 1
                catalogue_commit = "{'0' * 40}"

                [suite]
                id = "suite"
                revision = 19

                [pack]
                id = "pack"
                version = "1"
                archive_sha256 = "{'1' * 64}"
                tree_sha256 = "{'2' * 64}"

                {claim}
                """
            ),
            encoding="utf-8",
        )
        return path

    def test_loads_exact_caller_owned_capability(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            path = self.write_lock(
                Path(temporary),
                """[decoded_pixel_qualification]
                derived_set_id = "DS0"
                profile = 0
                compliance_class = 0
                m_magb = 18
                supported_coding_modes = ["HTONLY"]
                unsupported_coding_modes = ["HTMIX"]""",
            )
            lock, claim = runner.load_capability(path)
            self.assertEqual(lock.suite_revision, 19)
            self.assertEqual(claim, capability())

    def test_claim_must_be_complete_disjoint_and_bounded(self) -> None:
        invalid_claims = (
            "[decoded_pixel_qualification]\n",
            """[decoded_pixel_qualification]
            derived_set_id = "DS0"
            profile = 0
            compliance_class = 0
            m_magb = 256
            supported_coding_modes = ["HTONLY"]
            unsupported_coding_modes = ["HTMIX"]""",
            """[decoded_pixel_qualification]
            derived_set_id = "DS0"
            profile = 0
            compliance_class = 0
            m_magb = 18
            supported_coding_modes = ["HTONLY", "HTMIX"]
            unsupported_coding_modes = ["HTMIX"]""",
        )
        for claim in invalid_claims:
            with self.subTest(claim=claim), tempfile.TemporaryDirectory() as temporary:
                path = self.write_lock(Path(temporary), claim)
                with self.assertRaises(runner.RunnerError):
                    runner.load_capability(path)


class DerivedContractTests(unittest.TestCase):
    def test_selects_greatest_threshold_and_replaces_final_limits(self) -> None:
        lock, suite, inventory = fixture_contract()
        lower = runner.load_derived_cases(suite, inventory, lock, capability(12))
        higher = runner.load_derived_cases(suite, inventory, lock, capability(18))
        self.assertEqual((lower[0].b_magb, lower[0].input), (11, "files/htonly-b11.j2k"))
        self.assertEqual((higher[0].b_magb, higher[0].input), (15, "files/htonly-b15.j2k"))
        selected = higher[0].comparison.alternatives[0]
        self.assertEqual((selected.peak_error_limit, selected.mean_squared_error_limit), (2, 0.25))
        choice = higher[1].comparison.alternatives
        self.assertEqual(
            [(case.id, case.peak_error_limit, case.mean_squared_error_limit) for case in choice],
            [("full", 4, 0.5), ("reduced", 5, 0.75)],
        )

    def test_no_threshold_is_not_applicable(self) -> None:
        lock, suite, inventory = fixture_contract()
        cases = runner.load_derived_cases(suite, inventory, lock, capability(7))
        self.assertTrue(all(case.comparison is None for case in cases))

    def test_default_filters_modes_and_report_all_labels_unsupported(self) -> None:
        lock, suite, inventory = fixture_contract()
        cases = runner.load_derived_cases(suite, inventory, lock, capability())
        self.assertEqual(
            [case.coding_mode for case in runner.selected_cases(cases, capability(), False)],
            ["HTONLY", "HTONLY"],
        )
        all_cases = runner.selected_cases(cases, capability(), True)
        outcome = runner.run_derived_case(
            Path("/must/not/execute"), Path("/fixture"), all_cases[-1], capability(), 0.1
        )
        self.assertEqual((outcome.outcome, outcome.diagnostic),
                         ("not-applicable", "deliberately unsupported coding mode"))

    def test_malformed_contracts_fail_closed(self) -> None:
        mutations = []
        missing_normalisation = lambda plan: plan["decoded_pixel_comparison"].pop("output_normalisation")
        mutations.append((missing_normalisation, "normalisation"))
        mutations.append((
            lambda plan: plan["decoded_pixel_comparison"]["derived_sets"][0]["cases"][0]["variants"][1].__setitem__("b_magb", 11),
            "repeats B_MAGB",
        ))
        mutations.append((
            lambda plan: plan["decoded_pixel_comparison"]["derived_sets"][0]["cases"][1]["variants"][0]["alternative_limits"].pop(),
            "exactly cover",
        ))
        mutations.append((
            lambda plan: plan["decoded_pixel_comparison"]["derived_sets"][0]["cases"][0].__setitem__("reference_case_id", "missing"),
            "expected one decoded-pixel",
        ))
        for mutate, expected in mutations:
            with self.subTest(expected=expected):
                lock, suite, inventory = fixture_contract()
                mutate(suite)
                with self.assertRaisesRegex((runner.RunnerError, runner.canary.RunnerError), expected):
                    runner.load_derived_cases(suite, inventory, lock, capability())


class WorkerAndAggregationTests(unittest.TestCase):
    def make_codec(self, root: Path) -> Path:
        codec = root / "fake-codec"
        codec.write_text(
            textwrap.dedent(
                """\
                #!/usr/bin/env python3
                from pathlib import Path
                import sys
                import time

                input_name = Path(sys.argv[2]).name
                if input_name == "timeout.j2k":
                    time.sleep(2)
                if input_name == "unsupported.j2k":
                    print("emuella-j2k: decode failed: project-authored unsupported shape", file=sys.stderr)
                    raise SystemExit(2)
                peak = 3 if input_name == "reject.j2k" else 0
                passed = peak == 0
                print(f"component=0 width=2 height=2 samples=4 peak_error={peak} "
                      "mean_squared_error=0 peak_error_limit=0 "
                      f"mean_squared_error_limit=0 passed={str(passed).lower()}")
                raise SystemExit(0 if passed else 2)
                """
            ),
            encoding="utf-8",
        )
        codec.chmod(0o755)
        return codec

    def derived_case(self, input_name: str) -> runner.DerivedCase:
        comparison = runner.canary.ComparisonCase(
            "scalar", input_name, "reference.pgx", 0, False, 0, 0, 0,
            2, 2, 8, False, 0, 0.0
        )
        selection = runner.canary.ComparisonSelection("scalar", 1, (comparison,), False)
        return runner.DerivedCase("derived/case", "HTONLY", input_name, 11, selection)

    def test_worker_pass_rejection_failure_and_timeout_are_fail_closed(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            codec = self.make_codec(root)
            qualified = runner.run_derived_case(codec, root, self.derived_case("pass.j2k"), capability(), 0.5)
            rejected = runner.run_derived_case(codec, root, self.derived_case("reject.j2k"), capability(), 0.5)
            unsupported = runner.run_derived_case(codec, root, self.derived_case("unsupported.j2k"), capability(), 0.5)
            timed = runner.run_derived_case(codec, root, self.derived_case("timeout.j2k"), capability(), 0.1)
            self.assertEqual(qualified.outcome, "qualified")
            self.assertEqual(rejected.outcome, "rejected")
            self.assertEqual(rejected.aggregates[0][1].peak_error, 3)
            self.assertIn("project-authored unsupported shape", unsupported.diagnostic)
            self.assertEqual(timed.outcome, "rejected")
            self.assertIn("finite timeout", timed.diagnostic)

    def test_output_has_deterministic_identity_diagnostic_and_aggregates(self) -> None:
        case = self.derived_case("pass.j2k")
        result = runner.canary.ComparisonResult(0, 2, 2, 4, 0, 0.0, 0, 0.0, True)
        outcome = runner.CaseOutcome(case, "qualified", "in limit", (("scalar", result),))
        output = io.StringIO()
        with mock.patch("sys.stdout", output):
            runner.print_outcome(outcome)
        report = output.getvalue()
        for field in ("case=derived/case", "mode=HTONLY", "input=pass.j2k", "B_MAGB=11", "outcome=qualified", "aggregate_errors="):
            self.assertIn(field, report)
        self.assertNotIn("pixels", report)


class IntegrityTests(unittest.TestCase):
    def test_inventory_and_tree_mismatch_fail_before_execution(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            archive = b"project-authored archive"
            sample = b"project-authored fixture"
            (root / "files").mkdir()
            (root / "archive.zip").write_bytes(archive)
            (root / "files/input.j2k").write_bytes(sample)
            digest = lambda data: hashlib.sha256(data).hexdigest()
            inventory = {
                "assets": [
                    {"path": "archive.zip", "bytes": len(archive), "sha256": digest(archive)},
                    {"path": "files/input.j2k", "bytes": len(sample), "sha256": digest(sample)},
                ]
            }
            records = [
                ("archive.zip", len(archive), digest(archive)),
                ("files/input.j2k", len(sample), digest(sample)),
            ]
            lock = runner.common.Lock(
                "0" * 40, "suite", 1, "pack", "1", digest(archive),
                runner.common.tree_sha256(records)
            )
            pack = {"source": {"archive_filename": "archive.zip"}}
            self.assertEqual(runner.common.verify_materialisation(root, pack, inventory, lock),
                             (2, len(archive) + len(sample)))
            (root / "files/input.j2k").write_bytes(b"changed")
            with self.assertRaisesRegex(runner.common.RunnerError, "size mismatch"):
                runner.common.verify_materialisation(root, pack, inventory, lock)


if __name__ == "__main__":
    unittest.main()
