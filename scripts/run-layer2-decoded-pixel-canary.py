#!/usr/bin/env python3
"""Run one pinned decoded-pixel conformance comparison."""

from __future__ import annotations

import argparse
import dataclasses
import importlib.util
import math
import os
from pathlib import Path, PurePosixPath
import subprocess
import sys
from typing import Any


sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent.parent
COMMON_SCRIPT = Path(__file__).with_name("run-layer2-conformance-inspection.py")
COMMON_SPEC = importlib.util.spec_from_file_location("layer2_inspection", COMMON_SCRIPT)
assert COMMON_SPEC is not None and COMMON_SPEC.loader is not None
common = importlib.util.module_from_spec(COMMON_SPEC)
sys.modules[COMMON_SPEC.name] = common
COMMON_SPEC.loader.exec_module(common)

DEFAULT_CASE = "annex-c/class0-profile0/p0-01"
EXPECTED_RESULT_KEYS = {
    "component",
    "width",
    "height",
    "samples",
    "peak_error",
    "mean_squared_error",
    "peak_error_limit",
    "mean_squared_error_limit",
    "passed",
}


class RunnerError(Exception):
    """A decoded-pixel contract or execution failure."""


@dataclasses.dataclass(frozen=True)
class ComparisonCase:
    id: str
    input: str
    reference: str
    component: int
    output_window: bool
    resolution_reduction: int
    output_origin_x: int
    output_origin_y: int
    width: int
    height: int
    bits_per_sample: int
    signed: bool
    peak_error_limit: int
    mean_squared_error_limit: float


@dataclasses.dataclass(frozen=True)
class ComparisonSelection:
    id: str
    minimum_passing: int
    alternatives: tuple[ComparisonCase, ...]
    is_choice_group: bool


@dataclasses.dataclass(frozen=True)
class ComparisonResult:
    component: int
    width: int
    height: int
    samples: int
    peak_error: int
    mean_squared_error: float
    peak_error_limit: int
    mean_squared_error_limit: float
    passed: bool


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Verify and run the pinned Layer 2 decoded-pixel canary."
    )
    parser.add_argument(
        "--testdata",
        type=Path,
        default=os.environ.get("EMUELLA_TESTDATA_ROOT"),
        help="emuella-testdata checkout (or set EMUELLA_TESTDATA_ROOT)",
    )
    parser.add_argument("--pack-root", type=Path, help="explicit materialised pack root")
    parser.add_argument(
        "--codec",
        type=Path,
        default=ROOT / "target/debug/emuella-j2k",
        help="emuella-j2k executable built from this checkout",
    )
    parser.add_argument(
        "--unbound-codec",
        action="store_true",
        help="allow an arbitrary executable as diagnostic, non-qualification evidence",
    )
    parser.add_argument("--case", default=DEFAULT_CASE)
    parser.add_argument(
        "--lock", type=Path, default=ROOT / "testdata.lock.toml", help=argparse.SUPPRESS
    )
    parser.add_argument(
        "--timeout-seconds", type=common.positive_finite_seconds, default=10.0
    )
    arguments = parser.parse_args()
    if arguments.testdata is None:
        parser.error("--testdata or EMUELLA_TESTDATA_ROOT is required")
    return arguments


def non_negative_int(value: object, label: str, maximum: int | None = None) -> int:
    if (
        not isinstance(value, int)
        or isinstance(value, bool)
        or value < 0
        or (maximum is not None and value > maximum)
    ):
        raise RunnerError(f"{label} is outside its supported integer range")
    return value


def positive_int(value: object, label: str, maximum: int | None = None) -> int:
    parsed = non_negative_int(value, label, maximum)
    if parsed == 0:
        raise RunnerError(f"{label} must be positive")
    return parsed


def non_negative_finite(value: object, label: str) -> float:
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise RunnerError(f"{label} is not numeric")
    parsed = float(value)
    if not math.isfinite(parsed) or parsed < 0:
        raise RunnerError(f"{label} must be finite and non-negative")
    return parsed


def inventory_paths(inventory: dict[str, Any]) -> set[str]:
    assets = inventory.get("assets")
    if not isinstance(assets, list):
        raise RunnerError("locked inventory lacks assets")
    paths: set[str] = set()
    for asset in assets:
        if not isinstance(asset, dict):
            raise RunnerError("locked inventory contains a non-table asset")
        path = common.safe_relative_path(asset.get("path"), "inventory asset path")
        if path in paths:
            raise RunnerError(f"locked inventory repeats asset path {path}")
        paths.add(path)
    return paths


def load_comparison_case(
    suite: dict[str, Any],
    inventory: dict[str, Any],
    lock: common.Lock,
    case_id: str,
) -> ComparisonCase:
    selection = load_comparison_selection(suite, inventory, lock, case_id)
    if selection.is_choice_group or len(selection.alternatives) != 1:
        raise RunnerError(f"decoded-pixel selection {case_id} is not a scalar case")
    return selection.alternatives[0]


def load_comparison_selection(
    suite: dict[str, Any],
    inventory: dict[str, Any],
    lock: common.Lock,
    selection_id: str,
) -> ComparisonSelection:
    plan = suite.get("decoded_pixel_comparison")
    if not isinstance(plan, dict) or plan.get("pack_id") != lock.pack_id:
        raise RunnerError("suite lacks a decoded-pixel plan for the locked pack")
    if not isinstance(plan.get("standard"), str) or not plan["standard"]:
        raise RunnerError("decoded-pixel plan lacks its standards authority")
    clauses = plan.get("clauses")
    if (
        not isinstance(clauses, list)
        or not clauses
        or any(not isinstance(clause, str) or not clause for clause in clauses)
    ):
        raise RunnerError("decoded-pixel plan lacks clause authority")
    retrieval = plan.get("retrieval_commit")
    if (
        not isinstance(retrieval, str)
        or len(retrieval) != 40
        or any(character not in "0123456789abcdef" for character in retrieval)
    ):
        raise RunnerError("decoded-pixel plan has an invalid retrieval revision")
    cases = plan.get("cases", [])
    groups = plan.get("choice_groups", [])
    if not isinstance(cases, list) or not isinstance(groups, list) or not (cases or groups):
        raise RunnerError("decoded-pixel plan has no cases or choice groups")
    case_matches = [
        case for case in cases if isinstance(case, dict) and case.get("id") == selection_id
    ]
    group_matches = [
        group for group in groups if isinstance(group, dict) and group.get("id") == selection_id
    ]
    if len(case_matches) + len(group_matches) != 1:
        raise RunnerError(
            f"expected one decoded-pixel case or choice group {selection_id}, "
            f"found {len(case_matches) + len(group_matches)}"
        )
    paths = inventory_paths(inventory)
    if case_matches:
        record = case_matches[0]
        input_path = common.safe_relative_path(record.get("input"), "comparison input")
        comparison = comparison_from_record(
            selection_id, input_path, record, paths, scalar=True
        )
        return ComparisonSelection(selection_id, 1, (comparison,), False)

    group = group_matches[0]
    input_path = common.safe_relative_path(group.get("input"), "comparison input")
    if PurePosixPath(input_path).suffix != ".j2k" or input_path not in paths:
        raise RunnerError("choice-group input is not an inventory-backed .j2k path")
    alternatives = group.get("alternatives")
    if not isinstance(alternatives, list) or not alternatives:
        raise RunnerError("decoded-pixel choice group has no alternatives")
    minimum = positive_int(
        group.get("minimum_passing_alternatives"),
        "minimum passing alternatives",
        len(alternatives),
    )
    parsed: list[ComparisonCase] = []
    alternative_ids: set[str] = set()
    reference_paths: set[str] = set()
    for record in alternatives:
        if not isinstance(record, dict):
            raise RunnerError("decoded-pixel choice group contains a non-table alternative")
        alternative_id = record.get("id")
        if not isinstance(alternative_id, str) or not alternative_id:
            raise RunnerError("decoded-pixel alternative lacks an ID")
        if alternative_id in alternative_ids:
            raise RunnerError("decoded-pixel choice group repeats an alternative ID")
        alternative_ids.add(alternative_id)
        comparison = comparison_from_record(
            alternative_id, input_path, record, paths, scalar=False
        )
        if comparison.reference in reference_paths:
            raise RunnerError("decoded-pixel choice group repeats a reference")
        reference_paths.add(comparison.reference)
        parsed.append(comparison)
    return ComparisonSelection(selection_id, minimum, tuple(parsed), True)


def comparison_from_record(
    comparison_id: str,
    input_path: str,
    record: dict[str, Any],
    paths: set[str],
    *,
    scalar: bool,
) -> ComparisonCase:
    if scalar:
        input_path = common.safe_relative_path(record.get("input"), "comparison input")
    reference_path = common.safe_relative_path(
        record.get("reference"), "comparison reference"
    )
    if input_path == reference_path:
        raise RunnerError("comparison input and reference paths must differ")
    if PurePosixPath(input_path).suffix != ".j2k" or PurePosixPath(reference_path).suffix != ".pgx":
        raise RunnerError("comparison requires a .j2k input and .pgx reference")
    if input_path not in paths or reference_path not in paths:
        raise RunnerError("comparison input or reference is absent from the locked inventory")
    reduction = non_negative_int(record.get("resolution_reduction"), "resolution reduction", 1)
    if scalar and reduction != 0:
        raise RunnerError("scalar decoded-pixel cases require full-resolution output")
    output_origin_x = (
        0
        if scalar
        else non_negative_int(record.get("output_origin_x"), "output origin x", 0xFFFF_FFFF)
    )
    output_origin_y = (
        0
        if scalar
        else non_negative_int(record.get("output_origin_y"), "output origin y", 0xFFFF_FFFF)
    )
    signed = record.get("signed")
    if not isinstance(signed, bool):
        raise RunnerError("comparison signedness is not Boolean")
    comparison = ComparisonCase(
        id=comparison_id,
        input=input_path,
        reference=reference_path,
        component=non_negative_int(record.get("component"), "component", 65_535),
        output_window=not scalar,
        resolution_reduction=reduction,
        output_origin_x=output_origin_x,
        output_origin_y=output_origin_y,
        width=positive_int(record.get("width"), "width", 0xFFFF_FFFF),
        height=positive_int(record.get("height"), "height", 0xFFFF_FFFF),
        bits_per_sample=positive_int(record.get("bits_per_sample"), "precision", 32),
        signed=signed,
        peak_error_limit=non_negative_int(
            record.get("peak_error_limit"), "peak-error limit"
        ),
        mean_squared_error_limit=non_negative_finite(
            record.get("mean_squared_error_limit"), "mean-squared-error limit"
        ),
    )
    scale = 1 << comparison.resolution_reduction
    for value, label in (
        (comparison.output_origin_x * scale, "scaled output origin x"),
        (comparison.output_origin_y * scale, "scaled output origin y"),
        (comparison.width * scale, "scaled output width"),
        (comparison.height * scale, "scaled output height"),
    ):
        if value > 0xFFFF_FFFF:
            raise RunnerError(f"{label} exceeds the codec coordinate range")
    return comparison


def parse_worker_output(output: str, case: ComparisonCase) -> ComparisonResult:
    fields: dict[str, str] = {}
    for token in output.strip().split():
        key, separator, value = token.partition("=")
        if not separator or not key or not value or key in fields:
            raise RunnerError("codec worker returned a malformed aggregate record")
        fields[key] = value
    if set(fields) != EXPECTED_RESULT_KEYS:
        raise RunnerError("codec worker returned an unexpected aggregate record")
    try:
        result = ComparisonResult(
            component=int(fields["component"]),
            width=int(fields["width"]),
            height=int(fields["height"]),
            samples=int(fields["samples"]),
            peak_error=int(fields["peak_error"]),
            mean_squared_error=float(fields["mean_squared_error"]),
            peak_error_limit=int(fields["peak_error_limit"]),
            mean_squared_error_limit=float(fields["mean_squared_error_limit"]),
            passed={"true": True, "false": False}[fields["passed"]],
        )
    except (ValueError, KeyError) as error:
        raise RunnerError("codec worker returned invalid aggregate values") from error
    expected_samples = case.width * case.height
    if (
        result.component != case.component
        or result.width != case.width
        or result.height != case.height
        or result.samples != expected_samples
        or result.peak_error < 0
        or not math.isfinite(result.mean_squared_error)
        or result.mean_squared_error < 0
        or result.peak_error_limit != case.peak_error_limit
        or result.mean_squared_error_limit != case.mean_squared_error_limit
    ):
        raise RunnerError("codec worker aggregates disagree with the comparison contract")
    expected_pass = (
        result.peak_error <= result.peak_error_limit
        and result.mean_squared_error <= result.mean_squared_error_limit
    )
    if result.passed != expected_pass:
        raise RunnerError("codec worker pass state disagrees with its aggregates")
    return result


def run_case(
    codec: Path,
    pack_root: Path,
    case: ComparisonCase,
    timeout_seconds: float,
) -> ComparisonResult:
    command = [
        str(codec),
        "compare-pgx",
        str(pack_root / case.input),
        str(pack_root / case.reference),
        "--component",
        str(case.component),
        "--output-window" if case.output_window else "--full-component",
        "--resolution-reduction",
        str(case.resolution_reduction),
        "--output-origin-x",
        str(case.output_origin_x),
        "--output-origin-y",
        str(case.output_origin_y),
        "--width",
        str(case.width),
        "--height",
        str(case.height),
        "--bits-per-sample",
        str(case.bits_per_sample),
        "--signed" if case.signed else "--unsigned",
        "--peak-error-limit",
        str(case.peak_error_limit),
        "--mean-squared-error-limit",
        repr(case.mean_squared_error_limit),
    ]
    try:
        completed = subprocess.run(
            command, capture_output=True, text=True, timeout=timeout_seconds
        )
    except subprocess.TimeoutExpired as error:
        raise RunnerError("codec worker exceeded the finite timeout") from error
    except OSError as error:
        raise RunnerError(f"cannot invoke codec worker: {error}") from error
    if not completed.stdout.strip():
        outcome = (
            f"signal {-completed.returncode}"
            if completed.returncode < 0
            else f"exit {completed.returncode}"
        )
        raise RunnerError(f"codec worker produced no aggregate record ({outcome})")
    result = parse_worker_output(completed.stdout, case)
    if completed.returncode == 0 and result.passed:
        return result
    if completed.returncode == 2 and not result.passed:
        return result
    raise RunnerError("codec worker exit state disagrees with its aggregate record")


def execute(arguments: argparse.Namespace) -> int:
    lock = common.load_lock(arguments.lock.resolve())
    testdata_root = arguments.testdata.resolve()
    common.verify_catalogue_checkout(testdata_root, lock)
    suite, pack, inventory, _, default_root = common.validate_contract(testdata_root, lock)
    selection = load_comparison_selection(suite, inventory, lock, arguments.case)
    pack_root = (arguments.pack_root or default_root).resolve()
    asset_count, byte_count = common.verify_materialisation(pack_root, pack, inventory, lock)
    codec_identity = common.inspect_codec_identity(arguments.codec, arguments.unbound_codec)
    print(
        f"verified catalogue {lock.catalogue_commit}; suite {lock.suite_id} "
        f"r{lock.suite_revision}; pack {lock.pack_id}@{lock.pack_version}"
    )
    print(
        f"verified archive SHA-256 {lock.archive_sha256}; {asset_count} assets "
        f"({byte_count} bytes); tree SHA-256 {lock.tree_sha256}"
    )
    with common.snapshot_codec(codec_identity) as codec_snapshot:
        common.print_codec_identity(codec_identity, codec_snapshot)
        attempted: list[tuple[ComparisonCase, ComparisonResult]] = []
        failures: list[str] = []
        passing = 0
        for case in selection.alternatives:
            try:
                result = run_case(
                    codec_snapshot.path, pack_root, case, arguments.timeout_seconds
                )
            except RunnerError as error:
                if not selection.is_choice_group:
                    raise
                failures.append(f"{case.id}: {error}")
                continue
            attempted.append((case, result))
            if result.passed:
                passing += 1
            if passing >= selection.minimum_passing:
                break
    common.verify_codec_identity_unchanged(codec_identity)
    for case, result in attempted:
        alternative = f" alternative={case.id}" if selection.is_choice_group else ""
        print(
            f"case={selection.id}{alternative} component={result.component} "
            f"width={result.width} height={result.height} samples={result.samples} "
            f"peak_error={result.peak_error} "
            f"mean_squared_error={result.mean_squared_error:.17g} "
            f"peak_error_limit={result.peak_error_limit} "
            f"mean_squared_error_limit={result.mean_squared_error_limit:.17g} "
            f"passed={str(result.passed).lower()}"
        )
    if passing >= selection.minimum_passing:
        return 0
    if not attempted and failures:
        raise RunnerError(
            "no decoded-pixel choice alternative could be executed: "
            + "; ".join(failures)
        )
    return 1


def main() -> int:
    try:
        return execute(parse_args())
    except (RunnerError, common.RunnerError) as error:
        print(f"layer2-decoded-pixel-canary: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
