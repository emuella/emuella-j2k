#!/usr/bin/env python3
"""Run the pinned full-frame rendered-pixel comparisons."""

from __future__ import annotations

import argparse
import dataclasses
import importlib.util
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

PLAN_KEYS = {"pack_id", "standard", "clauses", "retrieval_commit", "cases"}
CASE_KEYS = {
    "id",
    "input",
    "reference",
    "width",
    "height",
    "components",
    "bits_per_sample",
    "rendered_colour_space",
    "reference_layout",
    "peak_error_limit",
}
EXPECTED_RESULT_KEYS = {
    "components",
    "width",
    "height",
    "samples",
    "peak",
    "limit",
    "passed",
}
MAX_COMPARISON_SAMPLES = 100_000_000


class RunnerError(Exception):
    """A rendered-pixel contract or execution failure."""


@dataclasses.dataclass(frozen=True)
class RenderedCase:
    id: str
    input: str
    reference: str
    width: int
    height: int
    components: int
    peak_error_limit: int


@dataclasses.dataclass(frozen=True)
class RenderedResult:
    components: int
    width: int
    height: int
    samples: int
    peak: int
    limit: int
    passed: bool


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Verify and run the pinned Layer 2 rendered-pixel comparisons."
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
        default=common.canonical_codec_path(),
        help="emuella-j2k executable built from this checkout",
    )
    parser.add_argument(
        "--unbound-codec",
        action="store_true",
        help="allow an arbitrary executable as diagnostic, non-qualification evidence",
    )
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


def positive_int(value: object, label: str, maximum: int) -> int:
    if (
        not isinstance(value, int)
        or isinstance(value, bool)
        or value <= 0
        or value > maximum
    ):
        raise RunnerError(f"{label} is outside its supported integer range")
    return value


def non_negative_int(value: object, label: str, maximum: int) -> int:
    if (
        not isinstance(value, int)
        or isinstance(value, bool)
        or value < 0
        or value > maximum
    ):
        raise RunnerError(f"{label} is outside its supported integer range")
    return value


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
            raise RunnerError("locked inventory repeats an asset path")
        paths.add(path)
    return paths


def load_rendered_cases(
    suite: dict[str, Any], inventory: dict[str, Any], lock: common.Lock
) -> tuple[RenderedCase, ...]:
    plan = suite.get("rendered_pixel_comparison")
    if not isinstance(plan, dict) or set(plan) != PLAN_KEYS:
        raise RunnerError("suite lacks the exact rendered-pixel plan shape")
    if plan.get("pack_id") != lock.pack_id:
        raise RunnerError("rendered-pixel plan does not name the locked pack")
    if not isinstance(plan.get("standard"), str) or not plan["standard"]:
        raise RunnerError("rendered-pixel plan lacks its standards authority")
    clauses = plan.get("clauses")
    if (
        not isinstance(clauses, list)
        or not clauses
        or len(set(clauses)) != len(clauses)
        or any(not isinstance(clause, str) or not clause for clause in clauses)
    ):
        raise RunnerError("rendered-pixel plan lacks unique clause authority")
    retrieval = plan.get("retrieval_commit")
    if (
        not isinstance(retrieval, str)
        or len(retrieval) != 40
        or any(character not in "0123456789abcdef" for character in retrieval)
    ):
        raise RunnerError("rendered-pixel plan has an invalid retrieval revision")
    records = plan.get("cases")
    if not isinstance(records, list) or not records:
        raise RunnerError("rendered-pixel plan has no cases")
    paths = inventory_paths(inventory)
    cases: list[RenderedCase] = []
    case_ids: set[str] = set()
    input_paths: set[str] = set()
    reference_paths: set[str] = set()
    for record in records:
        if not isinstance(record, dict) or set(record) != CASE_KEYS:
            raise RunnerError("rendered-pixel case has an unexpected schema shape")
        case_id = record.get("id")
        if not isinstance(case_id, str) or not case_id or case_id in case_ids:
            raise RunnerError("rendered-pixel case ID is empty or repeated")
        case_ids.add(case_id)
        input_path = common.safe_relative_path(record.get("input"), "rendered input")
        reference_path = common.safe_relative_path(
            record.get("reference"), "rendered reference"
        )
        if input_path == reference_path:
            raise RunnerError("rendered input and reference paths must differ")
        if (
            PurePosixPath(input_path).suffix != ".jp2"
            or PurePosixPath(reference_path).suffix != ".tif"
        ):
            raise RunnerError("rendered comparison requires a .jp2 input and .tif reference")
        if input_path not in paths or reference_path not in paths:
            raise RunnerError("rendered input or reference is absent from the locked inventory")
        if input_path in input_paths or reference_path in reference_paths:
            raise RunnerError("rendered plan repeats an input or reference path")
        input_paths.add(input_path)
        reference_paths.add(reference_path)
        if (
            record.get("components") != 3
            or record.get("bits_per_sample") != 8
            or record.get("rendered_colour_space") != "sRGB"
            or record.get("reference_layout") != "tiff-rgb-u8-contiguous"
        ):
            raise RunnerError("rendered case is outside the RGB-u8-contiguous boundary")
        width = positive_int(record.get("width"), "rendered width", 0xFFFF_FFFF)
        height = positive_int(record.get("height"), "rendered height", 0xFFFF_FFFF)
        components = positive_int(record.get("components"), "rendered components", 3)
        sample_count = width * height * components
        if sample_count > MAX_COMPARISON_SAMPLES:
            raise RunnerError("rendered sample count exceeds the worker bound")
        cases.append(
            RenderedCase(
                id=case_id,
                input=input_path,
                reference=reference_path,
                width=width,
                height=height,
                components=components,
                peak_error_limit=non_negative_int(
                    record.get("peak_error_limit"), "rendered peak-error limit", 255
                ),
            )
        )
    return tuple(cases)


def resolve_inventory_path(pack_root: Path, path: str, label: str) -> Path:
    try:
        resolved = (pack_root / path).resolve(strict=True)
        resolved.relative_to(pack_root)
    except (OSError, ValueError) as error:
        raise RunnerError(f"{label} does not resolve inside the verified pack") from error
    if not resolved.is_file() or resolved.is_symlink():
        raise RunnerError(f"{label} is not a verified regular file")
    return resolved


def parse_worker_output(output: str, case: RenderedCase) -> RenderedResult:
    fields: dict[str, str] = {}
    for token in output.strip().split():
        key, separator, value = token.partition("=")
        if not separator or not key or not value or key in fields:
            raise RunnerError("codec worker returned a malformed aggregate record")
        fields[key] = value
    if set(fields) != EXPECTED_RESULT_KEYS:
        raise RunnerError("codec worker returned an unexpected aggregate record")
    try:
        result = RenderedResult(
            components=int(fields["components"]),
            width=int(fields["width"]),
            height=int(fields["height"]),
            samples=int(fields["samples"]),
            peak=int(fields["peak"]),
            limit=int(fields["limit"]),
            passed={"true": True, "false": False}[fields["passed"]],
        )
    except (ValueError, KeyError) as error:
        raise RunnerError("codec worker returned invalid aggregate values") from error
    if (
        result.components != case.components
        or result.width != case.width
        or result.height != case.height
        or result.samples != case.width * case.height * case.components
        or result.peak < 0
        or result.peak > 255
        or result.limit != case.peak_error_limit
    ):
        raise RunnerError("codec worker aggregates disagree with the rendered contract")
    if result.passed != (result.peak <= result.limit):
        raise RunnerError("codec worker pass state disagrees with its aggregates")
    return result


def run_case(
    codec: Path,
    input_path: Path,
    reference_path: Path,
    case: RenderedCase,
    timeout_seconds: float,
) -> RenderedResult:
    command = [
        str(codec),
        "compare-rendered-tiff-rgb",
        str(input_path),
        str(reference_path),
        "--width",
        str(case.width),
        "--height",
        str(case.height),
        "--components",
        str(case.components),
        "--peak-error-limit",
        str(case.peak_error_limit),
    ]
    try:
        completed = subprocess.run(
            command, capture_output=True, text=True, timeout=timeout_seconds
        )
    except subprocess.TimeoutExpired as error:
        raise RunnerError("codec worker exceeded the finite timeout") from error
    except OSError as error:
        raise RunnerError("cannot invoke codec worker") from error
    if not completed.stdout.strip():
        outcome = (
            f"signal {-completed.returncode}"
            if completed.returncode < 0
            else f"exit {completed.returncode}"
        )
        diagnostic = " ".join(completed.stderr.strip().split())
        for protected_path in (input_path, reference_path):
            diagnostic = diagnostic.replace(str(protected_path), "<asset>")
        detail = f": {diagnostic}" if diagnostic else ""
        raise RunnerError(f"codec worker produced no aggregate record ({outcome}){detail}")
    result = parse_worker_output(completed.stdout, case)
    if completed.returncode == 0 and result.passed and not completed.stderr.strip():
        return result
    if completed.returncode == 2 and not result.passed:
        return result
    raise RunnerError("codec worker exit state disagrees with its aggregate record")


def execute(arguments: argparse.Namespace) -> int:
    lock = common.load_lock(arguments.lock.resolve())
    testdata_root = arguments.testdata.resolve()
    common.verify_catalogue_checkout(testdata_root, lock)
    suite, pack, inventory, _, default_root = common.validate_contract(testdata_root, lock)
    cases = load_rendered_cases(suite, inventory, lock)
    pack_root = (arguments.pack_root or default_root).resolve()
    asset_count, byte_count = common.verify_materialisation(pack_root, pack, inventory, lock)
    resolved_cases = [
        (
            case,
            resolve_inventory_path(pack_root, case.input, "rendered input"),
            resolve_inventory_path(pack_root, case.reference, "rendered reference"),
        )
        for case in cases
    ]
    codec_identity = common.inspect_codec_identity(arguments.codec, arguments.unbound_codec)
    print(
        f"verified catalogue {lock.catalogue_commit}; suite {lock.suite_id} "
        f"r{lock.suite_revision}; pack {lock.pack_id}@{lock.pack_version}"
    )
    print(
        f"verified archive SHA-256 {lock.archive_sha256}; {asset_count} assets "
        f"({byte_count} bytes); tree SHA-256 {lock.tree_sha256}"
    )
    results: list[tuple[RenderedCase, RenderedResult]] = []
    with common.snapshot_codec(codec_identity) as codec_snapshot:
        common.print_codec_identity(codec_identity, codec_snapshot)
        for case, input_path, reference_path in resolved_cases:
            results.append(
                (
                    case,
                    run_case(
                        codec_snapshot.path,
                        input_path,
                        reference_path,
                        case,
                        arguments.timeout_seconds,
                    ),
                )
            )
    common.verify_codec_identity_unchanged(codec_identity)
    passed = 0
    for case, result in results:
        passed += result.passed
        print(
            f"pack={lock.pack_id}@{lock.pack_version} case={case.id} "
            f"components={result.components} width={result.width} height={result.height} "
            f"samples={result.samples} peak={result.peak} limit={result.limit} "
            f"passed={str(result.passed).lower()}"
        )
    failed = len(results) - passed
    print(f"summary cases={len(results)} passed={passed} failed={failed}")
    return 1 if failed else 0


def main() -> int:
    try:
        return execute(parse_args())
    except (RunnerError, common.RunnerError) as error:
        print(f"layer2-rendered-pixel: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
