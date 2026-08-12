#!/usr/bin/env python3
"""Run the pinned Layer 2 JPEG 2000 inspection smoke suite."""

from __future__ import annotations

import argparse
import dataclasses
import hashlib
import math
import os
from pathlib import Path, PurePosixPath
import subprocess
import sys
import tomllib
from typing import Any, Iterable


ROOT = Path(__file__).resolve().parent.parent
FORMAT_ORDER = {"j2k": 0, "htj2k": 1, "jp2": 2, "jph": 3}
ALLOWED_EXTENSIONS = {".j2k", ".htj2k", ".jp2", ".jph"}
ALLOWED_FORMATS = set(FORMAT_ORDER)
ALLOWED_EXPECTATIONS = {"accept", "reject"}


class RunnerError(Exception):
    """A preflight, integrity, contract, or invocation failure."""


def positive_finite_seconds(value: str) -> float:
    try:
        seconds = float(value)
    except ValueError as error:
        raise argparse.ArgumentTypeError("must be a number") from error
    if not math.isfinite(seconds) or seconds <= 0:
        raise argparse.ArgumentTypeError("must be finite and greater than zero")
    return seconds


@dataclasses.dataclass(frozen=True)
class Lock:
    catalogue_commit: str
    suite_id: str
    suite_revision: int
    pack_id: str
    pack_version: str
    archive_sha256: str
    tree_sha256: str


@dataclasses.dataclass(frozen=True)
class Candidate:
    path: str
    format: str
    cohort: str
    expected: str
    diagnostic_contains: str | None


@dataclasses.dataclass(frozen=True)
class Result:
    candidate: Candidate
    actual: str
    diagnostic: str | None = None

    @property
    def anomaly(self) -> str | None:
        if self.actual == "timeout":
            return "timeout"
        if self.actual == "crash":
            return "crash"
        if self.candidate.expected == "accept" and self.actual == "reject":
            return "unexpected rejection"
        if self.candidate.expected == "reject" and self.actual == "accept":
            return "unexpected acceptance"
        if (
            self.actual == "reject"
            and self.candidate.expected == "reject"
            and self.candidate.diagnostic_contains not in (self.diagnostic or "")
        ):
            return "unexpected diagnostic"
        return None


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Verify and inspect the pinned Layer 2 JPEG 2000 conformance suite."
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
        "--lock",
        type=Path,
        default=ROOT / "testdata.lock.toml",
        help=argparse.SUPPRESS,
    )
    parser.add_argument("--timeout-seconds", type=positive_finite_seconds, default=10.0)
    arguments = parser.parse_args()
    if arguments.testdata is None:
        parser.error("--testdata or EMUELLA_TESTDATA_ROOT is required")
    return arguments


def load_toml(path: Path) -> dict[str, Any]:
    try:
        with path.open("rb") as source:
            return tomllib.load(source)
    except (OSError, tomllib.TOMLDecodeError) as error:
        raise RunnerError(f"cannot load {path}: {error}") from error


def require_keys(record: dict[str, Any], label: str, keys: set[str]) -> None:
    missing = sorted(keys - record.keys())
    if missing:
        raise RunnerError(f"{label} lacks required field(s): {', '.join(missing)}")


def require_sha256(value: object, label: str) -> str:
    if not isinstance(value, str) or len(value) != 64 or any(
        character not in "0123456789abcdef" for character in value
    ):
        raise RunnerError(f"{label} is not a lowercase SHA-256 digest")
    return value


def load_lock(path: Path) -> Lock:
    data = load_toml(path)
    require_keys(data, "lock", {"schema_version", "catalogue_commit", "suite", "pack"})
    if data["schema_version"] != 1:
        raise RunnerError(f"unsupported lock schema version {data['schema_version']!r}")
    suite = data["suite"]
    pack = data["pack"]
    require_keys(suite, "lock suite", {"id", "revision"})
    require_keys(
        pack,
        "lock pack",
        {"id", "version", "archive_sha256", "tree_sha256"},
    )
    commit = data["catalogue_commit"]
    if not isinstance(commit, str) or len(commit) != 40 or any(
        character not in "0123456789abcdef" for character in commit
    ):
        raise RunnerError("catalogue_commit is not a full lowercase Git commit")
    return Lock(
        catalogue_commit=commit,
        suite_id=suite["id"],
        suite_revision=suite["revision"],
        pack_id=pack["id"],
        pack_version=pack["version"],
        archive_sha256=require_sha256(pack["archive_sha256"], "locked archive digest"),
        tree_sha256=require_sha256(pack["tree_sha256"], "locked tree digest"),
    )


def run_git(root: Path, *arguments: str) -> str:
    try:
        completed = subprocess.run(
            ["git", "-C", str(root), *arguments],
            check=True,
            capture_output=True,
            text=True,
        )
    except (OSError, subprocess.CalledProcessError) as error:
        detail = getattr(error, "stderr", "") or str(error)
        raise RunnerError(f"cannot inspect catalogue Git state: {detail.strip()}") from error
    return completed.stdout.strip()


def verify_catalogue_checkout(root: Path, lock: Lock) -> None:
    head = run_git(root, "rev-parse", "HEAD^{commit}")
    if head != lock.catalogue_commit:
        raise RunnerError(
            f"catalogue commit mismatch: expected {lock.catalogue_commit}, found {head}"
        )
    dirty = run_git(root, "status", "--porcelain", "--untracked-files=no")
    if dirty:
        raise RunnerError("catalogue checkout has tracked modifications")


def find_record(root: Path, directory: str, identity: str) -> tuple[Path, dict[str, Any]]:
    matches: list[tuple[Path, dict[str, Any]]] = []
    for path in sorted((root / directory).rglob("*.toml")):
        record = load_toml(path)
        if record.get("id") == identity:
            matches.append((path, record))
    if len(matches) != 1:
        raise RunnerError(
            f"expected one {directory} record for {identity}, found {len(matches)}"
        )
    return matches[0]


def safe_relative_path(value: object, label: str) -> str:
    if not isinstance(value, str) or not value:
        raise RunnerError(f"{label} is empty or not text")
    path = PurePosixPath(value)
    if path.is_absolute() or ".." in path.parts or "." in path.parts or "" in path.parts:
        raise RunnerError(f"{label} is not a safe relative path: {value!r}")
    return value


def selected_pack(suite: dict[str, Any], lock: Lock) -> dict[str, Any]:
    selected = [pack for pack in suite.get("packs", []) if pack.get("id") == lock.pack_id]
    if len(selected) != 1:
        raise RunnerError(f"suite must select {lock.pack_id} exactly once")
    if selected[0].get("version") != lock.pack_version or selected[0].get("required") is not True:
        raise RunnerError("suite pack version or required state disagrees with the lock")
    return selected[0]


def load_candidates(
    suite: dict[str, Any], inventory: dict[str, Any], lock: Lock
) -> list[Candidate]:
    inspection = suite.get("inspection")
    if not isinstance(inspection, dict) or inspection.get("pack_id") != lock.pack_id:
        raise RunnerError("suite lacks the locked pack inspection plan")
    extensions = inspection.get("extensions")
    if (
        not isinstance(extensions, list)
        or not extensions
        or len(set(extensions)) != len(extensions)
        or any(extension not in ALLOWED_EXTENSIONS for extension in extensions)
    ):
        raise RunnerError("inspection extensions are empty, duplicated, or unsupported")
    default_expected = inspection.get("expected")
    default_diagnostic = inspection.get("diagnostic_contains")
    validate_expectation(default_expected, default_diagnostic, "inspection default")

    classifications = inspection.get("classifications")
    if not isinstance(classifications, list) or not classifications:
        raise RunnerError("inspection classification set is empty")
    rule_matches = [0] * len(classifications)
    assets = inventory.get("assets")
    if not isinstance(assets, list):
        raise RunnerError("inventory assets are absent")
    selected_assets = sorted(
        (
            asset
            for asset in assets
            if PurePosixPath(safe_relative_path(asset.get("path"), "inventory asset path"))
            .suffix.lower()
            in extensions
        ),
        key=lambda asset: asset["path"],
    )
    if not selected_assets:
        raise RunnerError("inspection candidate selection is empty")

    overrides: dict[str, dict[str, Any]] = {}
    for override in inspection.get("overrides", []):
        path = safe_relative_path(override.get("path"), "inspection override path")
        if path in overrides:
            raise RunnerError(f"duplicate inspection override: {path}")
        validate_expectation(
            override.get("expected"),
            override.get("diagnostic_contains"),
            f"inspection override {path}",
        )
        overrides[path] = override

    candidates: list[Candidate] = []
    for asset in selected_assets:
        path = asset["path"]
        matching: list[tuple[int, dict[str, Any]]] = []
        for index, rule in enumerate(classifications):
            exact = rule.get("path")
            prefix = rule.get("path_prefix")
            if (exact is None) == (prefix is None):
                raise RunnerError(
                    "inspection classification must set exactly one of path or path_prefix"
                )
            selector = safe_relative_path(exact if exact is not None else prefix, "selector")
            if path == selector if exact is not None else path.startswith(selector):
                matching.append((index, rule))
        if len(matching) != 1:
            raise RunnerError(
                f"unclassified or multiply classified input {path}: {len(matching)} matches"
            )
        index, rule = matching[0]
        rule_matches[index] += 1
        input_format = rule.get("format")
        cohort = rule.get("cohort")
        if input_format not in ALLOWED_FORMATS or not isinstance(cohort, str) or not cohort:
            raise RunnerError(f"invalid format or cohort classification for {path}")
        override = overrides.pop(path, None)
        candidates.append(
            Candidate(
                path=path,
                format=input_format,
                cohort=cohort,
                expected=(override or {}).get("expected", default_expected),
                diagnostic_contains=(override or {}).get(
                    "diagnostic_contains", default_diagnostic
                ),
            )
        )
    if any(matches == 0 for matches in rule_matches):
        raise RunnerError("inspection plan contains a classification matching no candidates")
    if overrides:
        raise RunnerError(
            "inspection overrides name unselected inputs: " + ", ".join(sorted(overrides))
        )
    return candidates


def validate_expectation(expected: object, diagnostic: object, label: str) -> None:
    if expected not in ALLOWED_EXPECTATIONS:
        raise RunnerError(f"{label} has invalid expectation {expected!r}")
    if expected == "reject" and (not isinstance(diagnostic, str) or not diagnostic):
        raise RunnerError(f"{label} rejection lacks a diagnostic")
    if expected == "accept" and diagnostic is not None:
        raise RunnerError(f"{label} acceptance must not name a diagnostic")


def validate_contract(
    testdata_root: Path, lock: Lock
) -> tuple[dict[str, Any], dict[str, Any], dict[str, Any], list[Candidate], Path]:
    _, suite = find_record(testdata_root, "suites", lock.suite_id)
    if (
        suite.get("revision") != lock.suite_revision
        or suite.get("layer") != 2
        or suite.get("gating") is not True
        or suite.get("missing_policy") != "fail"
    ):
        raise RunnerError("suite revision or Layer 2 gating identity disagrees with the lock")
    selected_pack(suite, lock)

    _, pack = find_record(testdata_root, "manifests", lock.pack_id)
    if pack.get("version") != lock.pack_version or pack.get("review_state") != "locked":
        raise RunnerError("pack version or locked review state disagrees with the lock")
    source = pack.get("source", {})
    materialisation = pack.get("materialization", {})
    if source.get("archive_sha256") != lock.archive_sha256:
        raise RunnerError("manifest archive digest disagrees with the codec lock")
    if materialisation.get("expected_tree_sha256") != lock.tree_sha256:
        raise RunnerError("manifest tree digest disagrees with the codec lock")
    inventory_path = safe_relative_path(pack.get("asset_inventory"), "asset inventory")
    inventory = load_toml(testdata_root / inventory_path)
    if (
        inventory.get("pack_id") != lock.pack_id
        or inventory.get("pack_version") != lock.pack_version
    ):
        raise RunnerError("inventory identity disagrees with the codec lock")
    candidates = load_candidates(suite, inventory, lock)
    default_root = testdata_root / safe_relative_path(
        materialisation.get("directory"), "materialisation directory"
    )
    return suite, pack, inventory, candidates, default_root


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    try:
        with path.open("rb") as source:
            while chunk := source.read(1024 * 1024):
                digest.update(chunk)
    except OSError as error:
        raise RunnerError(f"cannot hash {path}: {error}") from error
    return digest.hexdigest()


def materialised_records(root: Path) -> list[tuple[str, int, str]]:
    records: list[tuple[str, int, str]] = []
    try:
        for directory, names, files in os.walk(root, followlinks=False):
            directory_path = Path(directory)
            for name in names:
                path = directory_path / name
                if path.is_symlink():
                    raise RunnerError(f"materialised tree contains symlink: {path}")
            for name in files:
                path = directory_path / name
                if path.is_symlink() or not path.is_file():
                    raise RunnerError(f"materialised tree contains non-regular file: {path}")
                relative = path.relative_to(root).as_posix()
                records.append((relative, path.stat().st_size, sha256_file(path)))
    except OSError as error:
        raise RunnerError(f"cannot walk materialised tree {root}: {error}") from error
    return sorted(records)


def tree_sha256(records: Iterable[tuple[str, int, str]]) -> str:
    digest = hashlib.sha256()
    for path, size, file_digest in records:
        digest.update(f"{file_digest}\t{size}\t{path}\n".encode())
    return digest.hexdigest()


def verify_materialisation(
    root: Path, pack: dict[str, Any], inventory: dict[str, Any], lock: Lock
) -> tuple[int, int]:
    if not root.is_dir():
        raise RunnerError(f"materialised pack root is absent: {root}")
    expected: dict[str, tuple[int, str]] = {}
    for asset in inventory.get("assets", []):
        path = safe_relative_path(asset.get("path"), "inventory asset path")
        if path in expected:
            raise RunnerError(f"inventory repeats asset path {path}")
        expected[path] = (
            asset.get("bytes"),
            require_sha256(asset.get("sha256"), f"inventory digest for {path}"),
        )
    actual = materialised_records(root)
    actual_paths = {path for path, _, _ in actual}
    if actual_paths != set(expected):
        missing = sorted(set(expected) - actual_paths)
        extra = sorted(actual_paths - set(expected))
        raise RunnerError(
            f"materialised path mismatch: missing={missing[:3]!r} extra={extra[:3]!r}"
        )
    for path, size, digest in actual:
        expected_size, expected_digest = expected[path]
        if size != expected_size:
            raise RunnerError(
                f"size mismatch for {path}: expected {expected_size}, found {size}"
            )
        if digest != expected_digest:
            raise RunnerError(
                f"SHA-256 mismatch for {path}: expected {expected_digest}, found {digest}"
            )
    archive = safe_relative_path(pack.get("source", {}).get("archive_filename"), "archive")
    if archive not in expected or expected[archive][1] != lock.archive_sha256:
        raise RunnerError("archive identity or digest disagrees with the lock")
    actual_tree = tree_sha256(actual)
    if actual_tree != lock.tree_sha256:
        raise RunnerError(
            f"tree SHA-256 mismatch: expected {lock.tree_sha256}, found {actual_tree}"
        )
    return len(actual), sum(size for _, size, _ in actual)


def run_candidates(
    codec: Path, pack_root: Path, candidates: list[Candidate], timeout_seconds: float
) -> list[Result]:
    if not codec.is_file() or not os.access(codec, os.X_OK):
        raise RunnerError(f"codec executable is absent or not executable: {codec}")
    results: list[Result] = []
    for candidate in candidates:
        try:
            completed = subprocess.run(
                [str(codec), "inspect", str(pack_root / candidate.path)],
                capture_output=True,
                text=True,
                timeout=timeout_seconds,
            )
        except subprocess.TimeoutExpired:
            results.append(Result(candidate, "timeout"))
            continue
        except OSError as error:
            raise RunnerError(f"cannot invoke codec for {candidate.path}: {error}") from error
        diagnostic = completed.stderr.strip().replace(str(pack_root) + os.sep, "") or None
        if completed.returncode == 0:
            results.append(Result(candidate, "accept", diagnostic))
        elif completed.returncode == 2:
            results.append(Result(candidate, "reject", diagnostic))
        else:
            detail = diagnostic or (
                f"signal {-completed.returncode}"
                if completed.returncode < 0
                else f"exit {completed.returncode}"
            )
            results.append(Result(candidate, "crash", detail))
    return results


def print_summary(results: list[Result]) -> None:
    grouped: dict[tuple[str, str], list[Result]] = {}
    for result in results:
        grouped.setdefault((result.candidate.format, result.candidate.cohort), []).append(result)
    print("format  cohort                         selected accepted rejected timeout crash unexpected")
    for (input_format, cohort), group in sorted(
        grouped.items(), key=lambda item: (FORMAT_ORDER[item[0][0]], item[0][1])
    ):
        counts = {status: sum(result.actual == status for result in group) for status in ("accept", "reject", "timeout", "crash")}
        unexpected = sum(result.anomaly is not None for result in group)
        print(
            f"{input_format:<7} {cohort:<30} {len(group):>8} {counts['accept']:>8} "
            f"{counts['reject']:>8} {counts['timeout']:>7} {counts['crash']:>5} {unexpected:>10}"
        )


def execute(arguments: argparse.Namespace) -> int:
    lock = load_lock(arguments.lock.resolve())
    testdata_root = arguments.testdata.resolve()
    verify_catalogue_checkout(testdata_root, lock)
    _, pack, inventory, candidates, default_root = validate_contract(testdata_root, lock)
    pack_root = (arguments.pack_root or default_root).resolve()
    asset_count, byte_count = verify_materialisation(pack_root, pack, inventory, lock)
    print(
        f"verified catalogue {lock.catalogue_commit}; suite {lock.suite_id} r{lock.suite_revision}; "
        f"pack {lock.pack_id}@{lock.pack_version}"
    )
    print(
        f"verified archive SHA-256 {lock.archive_sha256}; {asset_count} assets "
        f"({byte_count} bytes); tree SHA-256 {lock.tree_sha256}"
    )
    results = run_candidates(arguments.codec.resolve(), pack_root, candidates, arguments.timeout_seconds)
    anomalies = [result for result in results if result.anomaly is not None]
    for result in anomalies:
        detail = f": {result.diagnostic}" if result.diagnostic else ""
        print(
            f"anomaly {result.anomaly}: {result.candidate.path} "
            f"(expected {result.candidate.expected}, found {result.actual}){detail}",
            file=sys.stderr,
        )
    print_summary(results)
    return 1 if anomalies else 0


def main() -> int:
    try:
        return execute(parse_args())
    except RunnerError as error:
        print(f"layer2-conformance-inspection: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
