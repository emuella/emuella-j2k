#!/usr/bin/env python3
"""Qualify a pinned catalogue-derived decoded-pixel set."""

from __future__ import annotations

import argparse
import dataclasses
import importlib.util
import json
import os
from pathlib import Path, PurePosixPath
import sys
from typing import Any


sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent.parent
CANARY_SCRIPT = Path(__file__).with_name("run-layer2-decoded-pixel-canary.py")
CANARY_SPEC = importlib.util.spec_from_file_location("layer2_canary", CANARY_SCRIPT)
assert CANARY_SPEC is not None and CANARY_SPEC.loader is not None
canary = importlib.util.module_from_spec(CANARY_SPEC)
sys.modules[CANARY_SPEC.name] = canary
CANARY_SPEC.loader.exec_module(canary)
common = canary.common

SUPPORTED_DERIVED_SET = "DS0"
SELECTION_RULE = "greatest-b-magb-not-exceeding-m-magb"
CODING_MODES = frozenset({"HTONLY", "HTMIX"})
EXPECTED_NORMALISATION = {
    "order_dependent": [
        "resolution-reduction",
        "recover-first-codestream-component",
        "round-to-nearest-integer",
        "clip-to-declared-sample-range",
        "reference-bit-depth-arithmetic-shift",
        "reference-grid-subsampling",
        "upper-left-reference-crop",
    ],
    "order_independent": [
        "planar-component-deinterleave",
        "big-endian-byte-order",
        "sign-extend-to-byte-boundary",
    ],
}


class RunnerError(Exception):
    """A capability, derived-set contract, or execution failure."""


@dataclasses.dataclass(frozen=True)
class Capability:
    derived_set_id: str
    profile: int
    compliance_class: int
    m_magb: int
    supported_coding_modes: frozenset[str]
    unsupported_coding_modes: frozenset[str]


@dataclasses.dataclass(frozen=True)
class DerivedCase:
    id: str
    coding_mode: str
    input: str | None
    b_magb: int | None
    comparison: canary.ComparisonSelection | None


@dataclasses.dataclass(frozen=True)
class CaseOutcome:
    case: DerivedCase
    outcome: str
    diagnostic: str
    aggregates: tuple[tuple[str, canary.ComparisonResult], ...] = ()


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Verify and run the pinned Layer 2 decoded-pixel derived set."
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
        "--report-all",
        action="store_true",
        help="report deliberately unsupported coding modes without executing them",
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


def exact_keys(record: dict[str, Any], expected: set[str], label: str) -> None:
    if set(record) != expected:
        missing = sorted(expected - set(record))
        extra = sorted(set(record) - expected)
        raise RunnerError(f"{label} fields disagree: missing={missing!r} extra={extra!r}")


def bounded_int(value: object, label: str, maximum: int) -> int:
    if not isinstance(value, int) or isinstance(value, bool) or not 0 <= value <= maximum:
        raise RunnerError(f"{label} is outside its supported integer range")
    return value


def non_empty_unique_modes(value: object, label: str) -> frozenset[str]:
    if (
        not isinstance(value, list)
        or not value
        or any(not isinstance(mode, str) or mode not in CODING_MODES for mode in value)
        or len(value) != len(set(value))
    ):
        raise RunnerError(f"{label} is empty, duplicated, or unsupported")
    return frozenset(value)


def load_capability(path: Path) -> tuple[common.Lock, Capability]:
    lock = common.load_lock(path)
    data = common.load_toml(path)
    claim = data.get("decoded_pixel_qualification")
    if not isinstance(claim, dict):
        raise RunnerError("lock lacks the decoded-pixel qualification claim")
    expected = {
        "derived_set_id",
        "profile",
        "compliance_class",
        "m_magb",
        "supported_coding_modes",
        "unsupported_coding_modes",
    }
    exact_keys(claim, expected, "decoded-pixel qualification claim")
    supported = non_empty_unique_modes(
        claim["supported_coding_modes"], "supported coding-mode claim"
    )
    unsupported = non_empty_unique_modes(
        claim["unsupported_coding_modes"], "unsupported coding-mode claim"
    )
    if supported & unsupported or supported | unsupported != CODING_MODES:
        raise RunnerError("supported and unsupported coding-mode claims must partition DS0")
    capability = Capability(
        derived_set_id=claim["derived_set_id"],
        profile=bounded_int(claim["profile"], "profile claim", 255),
        compliance_class=bounded_int(
            claim["compliance_class"], "compliance-class claim", 255
        ),
        m_magb=bounded_int(claim["m_magb"], "M_MAGB claim", 255),
        supported_coding_modes=supported,
        unsupported_coding_modes=unsupported,
    )
    if capability.derived_set_id != SUPPORTED_DERIVED_SET:
        raise RunnerError(f"unsupported derived-set claim {capability.derived_set_id!r}")
    if capability.profile != 0 or capability.compliance_class != 0:
        raise RunnerError("DS0 qualification is restricted to Profile 0 and Class 0")
    return lock, capability


def require_text(value: object, label: str) -> str:
    if not isinstance(value, str) or not value:
        raise RunnerError(f"{label} is empty or not text")
    return value


def parse_limit(record: object, identity_key: str, label: str) -> tuple[object, int, float]:
    if not isinstance(record, dict):
        raise RunnerError(f"{label} is not a table")
    exact_keys(
        record,
        {identity_key, "peak_error_limit", "mean_squared_error_limit"},
        label,
    )
    identity = record[identity_key]
    if identity_key == "component":
        identity = bounded_int(identity, f"{label} component", 65_535)
    else:
        identity = require_text(identity, f"{label} alternative ID")
    peak = canary.non_negative_int(record["peak_error_limit"], f"{label} peak limit")
    mean = canary.non_negative_finite(
        record["mean_squared_error_limit"], f"{label} mean-squared limit"
    )
    return identity, peak, mean


def apply_variant_limits(
    selection: canary.ComparisonSelection, variant: dict[str, Any], input_path: str
) -> canary.ComparisonSelection:
    has_components = "component_limits" in variant
    has_alternatives = "alternative_limits" in variant
    if has_components == has_alternatives:
        raise RunnerError("derived-set variant must declare exactly one limit form")
    if selection.is_choice_group != has_alternatives:
        raise RunnerError("derived-set limit form disagrees with its reference contract")
    key = "alternative_limits" if has_alternatives else "component_limits"
    identity_key = "alternative_id" if has_alternatives else "component"
    limits = variant[key]
    if not isinstance(limits, list) or not limits:
        raise RunnerError(f"derived-set variant {key} is empty")
    parsed = [parse_limit(record, identity_key, f"derived-set {key}") for record in limits]
    identities = [identity for identity, _, _ in parsed]
    if len(identities) != len(set(identities)):
        raise RunnerError(f"derived-set variant repeats a {identity_key} limit")
    expected_identities = [
        case.id if has_alternatives else case.component for case in selection.alternatives
    ]
    if set(identities) != set(expected_identities):
        raise RunnerError("derived-set limits do not exactly cover the reference contract")
    by_identity = {identity: (peak, mean) for identity, peak, mean in parsed}
    alternatives = tuple(
        dataclasses.replace(
            case,
            input=input_path,
            peak_error_limit=by_identity[
                case.id if has_alternatives else case.component
            ][0],
            mean_squared_error_limit=by_identity[
                case.id if has_alternatives else case.component
            ][1],
        )
        for case in selection.alternatives
    )
    return dataclasses.replace(selection, alternatives=alternatives)


def load_derived_cases(
    suite: dict[str, Any], inventory: dict[str, Any], lock: common.Lock, capability: Capability
) -> list[DerivedCase]:
    plan = suite.get("decoded_pixel_comparison")
    if not isinstance(plan, dict) or plan.get("pack_id") != lock.pack_id:
        raise RunnerError("suite lacks a decoded-pixel plan for the locked pack")
    normalisation = plan.get("output_normalisation")
    if (
        not isinstance(normalisation, dict)
        or set(normalisation) != set(EXPECTED_NORMALISATION)
        or normalisation.get("order_dependent")
        != EXPECTED_NORMALISATION["order_dependent"]
        or not isinstance(normalisation.get("order_independent"), list)
        or len(normalisation["order_independent"])
        != len(set(normalisation["order_independent"]))
        or set(normalisation["order_independent"])
        != set(EXPECTED_NORMALISATION["order_independent"])
    ):
        raise RunnerError("decoded-pixel output normalisation contract is absent or altered")
    sets = plan.get("derived_sets")
    if not isinstance(sets, list):
        raise RunnerError("decoded-pixel plan lacks derived sets")
    matches = [
        record
        for record in sets
        if isinstance(record, dict)
        and record.get("id") == capability.derived_set_id
        and record.get("profile") == capability.profile
        and record.get("compliance_class") == capability.compliance_class
    ]
    if len(matches) != 1:
        raise RunnerError("expected exactly one derived set matching the codec claim")
    derived = matches[0]
    exact_keys(
        derived,
        {"id", "profile", "compliance_class", "selection", "cases"},
        "derived set",
    )
    if derived["selection"] != SELECTION_RULE:
        raise RunnerError("derived-set selection rule is unsupported")
    records = derived["cases"]
    if not isinstance(records, list) or not records:
        raise RunnerError("derived set has no cases")
    paths = canary.inventory_paths(inventory)
    parsed: list[DerivedCase] = []
    case_ids: set[str] = set()
    observed_modes: set[str] = set()
    for record in records:
        if not isinstance(record, dict):
            raise RunnerError("derived set contains a non-table case")
        exact_keys(
            record, {"id", "coding_mode", "reference_case_id", "variants"},
            "derived-set case"
        )
        case_id = require_text(record["id"], "derived-set case ID")
        if case_id in case_ids:
            raise RunnerError("derived set repeats a case ID")
        case_ids.add(case_id)
        mode = require_text(record["coding_mode"], "derived-set coding mode")
        if mode not in CODING_MODES:
            raise RunnerError(f"derived-set coding mode is unsupported: {mode}")
        observed_modes.add(mode)
        reference_id = require_text(record["reference_case_id"], "reference case ID")
        reference = canary.load_comparison_selection(suite, inventory, lock, reference_id)
        variants = record["variants"]
        if not isinstance(variants, list) or not variants:
            raise RunnerError(f"derived-set case {case_id} has no variants")
        candidates: list[tuple[int, str, dict[str, Any]]] = []
        seen_b_magb: set[int] = set()
        seen_inputs: set[str] = set()
        for variant in variants:
            if not isinstance(variant, dict):
                raise RunnerError("derived-set case contains a non-table variant")
            allowed = {"input", "b_magb", "component_limits", "alternative_limits"}
            if not {"input", "b_magb"} <= set(variant) or set(variant) - allowed:
                raise RunnerError("derived-set variant fields disagree with the contract")
            input_path = common.safe_relative_path(variant["input"], "derived input")
            if PurePosixPath(input_path).suffix != ".j2k" or input_path not in paths:
                raise RunnerError("derived input is not an inventory-backed .j2k path")
            b_magb = bounded_int(variant["b_magb"], "B_MAGB", 255)
            if b_magb in seen_b_magb or input_path in seen_inputs:
                raise RunnerError("derived-set case repeats B_MAGB or input identity")
            seen_b_magb.add(b_magb)
            seen_inputs.add(input_path)
            # Validate every alternative threshold, including unselected variants.
            apply_variant_limits(reference, variant, input_path)
            if b_magb <= capability.m_magb:
                candidates.append((b_magb, input_path, variant))
        if not candidates:
            parsed.append(DerivedCase(case_id, mode, None, None, None))
            continue
        b_magb, input_path, variant = max(candidates, key=lambda candidate: candidate[0])
        comparison = apply_variant_limits(reference, variant, input_path)
        parsed.append(DerivedCase(case_id, mode, input_path, b_magb, comparison))
    if observed_modes != CODING_MODES:
        raise RunnerError("derived set does not contain the complete claimed coding-mode domain")
    return parsed


def selected_cases(
    cases: list[DerivedCase], capability: Capability, report_all: bool
) -> list[DerivedCase]:
    allowed = CODING_MODES if report_all else capability.supported_coding_modes
    selected = [case for case in cases if case.coding_mode in allowed]
    if not selected:
        raise RunnerError("derived-set mode filtering selected no cases")
    return selected


def run_derived_case(
    codec: Path, pack_root: Path, case: DerivedCase, capability: Capability,
    timeout_seconds: float
) -> CaseOutcome:
    if case.coding_mode in capability.unsupported_coding_modes:
        return CaseOutcome(case, "not-applicable", "deliberately unsupported coding mode")
    if case.comparison is None:
        return CaseOutcome(
            case, "not-applicable", "no B_MAGB variant does not exceed the M_MAGB claim"
        )
    aggregates: list[tuple[str, canary.ComparisonResult]] = []
    failures: list[str] = []
    passing = 0
    for comparison in case.comparison.alternatives:
        try:
            result = canary.run_case(codec, pack_root, comparison, timeout_seconds)
        except canary.RunnerError as error:
            failures.append(f"{comparison.id}: {error}")
            continue
        aggregates.append((comparison.id, result))
        passing += result.passed
        if passing >= case.comparison.minimum_passing:
            return CaseOutcome(case, "qualified", "in-limit decoded-pixel comparison", tuple(aggregates))
    diagnostic_parts = failures[:]
    if aggregates:
        diagnostic_parts.append("decoded samples exceed the comparison limits")
    diagnostic = "; ".join(diagnostic_parts) or "comparison contract was not satisfied"
    return CaseOutcome(case, "rejected", diagnostic, tuple(aggregates))


def print_outcome(result: CaseOutcome) -> None:
    aggregates = [
        {
            "alternative": alternative,
            "component": aggregate.component,
            "peak_error": aggregate.peak_error,
            "mean_squared_error": aggregate.mean_squared_error,
            "peak_error_limit": aggregate.peak_error_limit,
            "mean_squared_error_limit": aggregate.mean_squared_error_limit,
            "passed": aggregate.passed,
        }
        for alternative, aggregate in result.aggregates
    ]
    print(
        f"case={result.case.id} mode={result.case.coding_mode} "
        f"input={result.case.input or '-'} B_MAGB={result.case.b_magb if result.case.b_magb is not None else '-'} "
        f"outcome={result.outcome} diagnostic={json.dumps(result.diagnostic, ensure_ascii=True)} "
        f"aggregate_errors={json.dumps(aggregates, sort_keys=True, separators=(',', ':'))}"
    )


def execute(arguments: argparse.Namespace) -> int:
    lock, capability = load_capability(arguments.lock.resolve())
    testdata_root = arguments.testdata.resolve()
    common.verify_catalogue_checkout(testdata_root, lock)
    suite, pack, inventory, _, default_root = common.validate_contract(testdata_root, lock)
    cases = load_derived_cases(suite, inventory, lock, capability)
    cases = selected_cases(cases, capability, arguments.report_all)
    pack_root = (arguments.pack_root or default_root).resolve()
    asset_count, byte_count = common.verify_materialisation(pack_root, pack, inventory, lock)
    codec_identity = common.inspect_codec_identity(arguments.codec, False)
    print(
        f"verified catalogue {lock.catalogue_commit}; suite {lock.suite_id} r{lock.suite_revision}; "
        f"pack {lock.pack_id}@{lock.pack_version}"
    )
    print(
        f"verified archive SHA-256 {lock.archive_sha256}; {asset_count} assets "
        f"({byte_count} bytes); tree SHA-256 {lock.tree_sha256}"
    )
    print(
        f"codec claim derived_set={capability.derived_set_id} profile={capability.profile} "
        f"class={capability.compliance_class} M_MAGB={capability.m_magb} "
        f"supported_modes={','.join(sorted(capability.supported_coding_modes))} "
        f"unsupported_modes={','.join(sorted(capability.unsupported_coding_modes))}"
    )
    with common.snapshot_codec(codec_identity) as codec_snapshot:
        common.print_codec_identity(codec_identity, codec_snapshot)
        outcomes = [
            run_derived_case(
                codec_snapshot.path, pack_root, case, capability, arguments.timeout_seconds
            )
            for case in cases
        ]
    common.verify_codec_identity_unchanged(codec_identity)
    for outcome in outcomes:
        print_outcome(outcome)
    qualified = sum(outcome.outcome == "qualified" for outcome in outcomes)
    rejected = sum(outcome.outcome == "rejected" for outcome in outcomes)
    not_applicable = sum(outcome.outcome == "not-applicable" for outcome in outcomes)
    print(
        f"aggregate qualified={qualified} rejected={rejected} "
        f"not-applicable={not_applicable} selected={len(outcomes)}"
    )
    return 1 if rejected else 0


def main() -> int:
    try:
        return execute(parse_args())
    except (RunnerError, canary.RunnerError, common.RunnerError) as error:
        print(f"layer2-derived-set: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
