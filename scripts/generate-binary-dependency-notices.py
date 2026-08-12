#!/usr/bin/env python3
"""Generate locked Cargo dependency inventories for prebuilt binary distributions."""

from __future__ import annotations

import argparse
import hashlib
import json
import shutil
import subprocess
import sys
from collections import deque
from pathlib import Path, PurePosixPath

import tomllib

ROOT = Path(__file__).resolve().parent.parent
LOCK_PATH = ROOT / "Cargo.lock"
NOTICE_PREFIXES = ("COPYING", "COPYRIGHT", "LICENSE", "NOTICE", "UNLICENSE")
ROLE_PRIORITY = {"dev-only": 0, "build-only": 1, "runtime": 2}
TARGETS = {
    "python": (
        "emuella-j2k-python",
        ROOT / "crates/emuella-j2k-python",
    ),
    "cli": (
        "emuella-j2k-cli",
        ROOT / "crates/emuella-j2k-cli",
    ),
}


def arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--write", action="store_true")
    return parser.parse_args()


def cargo_metadata() -> dict[str, object]:
    completed = subprocess.run(
        ["cargo", "metadata", "--locked", "--format-version", "1"],
        cwd=ROOT,
        check=True,
        stdout=subprocess.PIPE,
    )
    return json.loads(completed.stdout)


def lock_packages() -> dict[tuple[str, str, str], dict[str, object]]:
    lock = tomllib.loads(LOCK_PATH.read_text(encoding="utf-8"))
    return {
        (
            package["name"],
            package["version"],
            package.get("source", ""),
        ): package
        for package in lock["package"]
    }


def is_proc_macro(package: dict[str, object]) -> bool:
    return any("proc-macro" in target["kind"] for target in package["targets"])


def dependency_roles(
    root_id: str,
    nodes: dict[str, dict[str, object]],
    packages: dict[str, dict[str, object]],
) -> dict[str, str]:
    reached: dict[str, set[str]] = {root_id: {"runtime"}}
    pending = deque([(root_id, "runtime")])
    visited: set[tuple[str, str]] = set()
    while pending:
        package_id, parent_role = pending.popleft()
        if (package_id, parent_role) in visited:
            continue
        visited.add((package_id, parent_role))
        for dependency in nodes[package_id]["deps"]:
            dependency_id = dependency["pkg"]
            proc_macro = is_proc_macro(packages[dependency_id])
            for dependency_kind in dependency["dep_kinds"]:
                kind = dependency_kind["kind"]
                # Cargo metadata includes dev edges for every workspace member.
                # Only the selected distribution root's dev dependencies belong
                # to this graph; transitive packages never enable their dev deps.
                if kind == "dev" and package_id != root_id:
                    continue
                if parent_role == "runtime":
                    if kind == "dev":
                        role = "dev-only"
                    elif kind == "build" or proc_macro:
                        role = "build-only"
                    else:
                        role = "runtime"
                elif parent_role == "build-only":
                    role = "build-only"
                else:
                    role = "dev-only"
                roles = reached.setdefault(dependency_id, set())
                if role not in roles:
                    roles.add(role)
                    pending.append((dependency_id, role))
    return {
        package_id: max(roles, key=ROLE_PRIORITY.__getitem__)
        for package_id, roles in reached.items()
    }


def normalized_spdx(value: str | None, package: str) -> str:
    if not value:
        raise ValueError(f"dependency has no declared licence expression: {package}")
    return value.replace("MIT/Apache-2.0", "MIT OR Apache-2.0")


def notice_files(package: dict[str, object]) -> list[Path]:
    package_root = Path(package["manifest_path"]).parent
    result: set[Path] = set()
    declared = package.get("license_file")
    if declared:
        declared_path = Path(declared)
        if not declared_path.is_absolute():
            declared_path = package_root / declared_path
        result.add(declared_path)
    for candidate in package_root.glob("*"):
        if candidate.is_file() and candidate.name.upper().startswith(NOTICE_PREFIXES):
            result.add(candidate)
    for directory in package_root.iterdir():
        if not directory.is_dir():
            continue
        for candidate in directory.glob("*"):
            if candidate.is_file() and candidate.name.upper().startswith(
                NOTICE_PREFIXES
            ):
                result.add(candidate)
    return sorted(result, key=lambda path: path.relative_to(package_root).as_posix())


def markdown(value: str) -> str:
    return value.replace("|", "\\|").replace("\n", " ")


def package_directory(package: dict[str, object]) -> str:
    return f"{package['name']}-{package['version']}"


def expected_outputs(
    target_name: str,
    root_package_name: str,
    output_root: Path,
    metadata: dict[str, object],
    locks: dict[tuple[str, str, str], dict[str, object]],
) -> dict[PurePosixPath, bytes]:
    packages = {package["id"]: package for package in metadata["packages"]}
    nodes = {node["id"]: node for node in metadata["resolve"]["nodes"]}
    matching_roots = [
        package["id"]
        for package in metadata["packages"]
        if package["name"] == root_package_name and package["source"] is None
    ]
    if len(matching_roots) != 1:
        raise ValueError(
            f"expected one workspace package named {root_package_name}, "
            f"found {len(matching_roots)}"
        )
    roles = dependency_roles(matching_roots[0], nodes, packages)
    dependencies = [
        packages[package_id]
        for package_id in roles
        if package_id != matching_roots[0]
        and packages[package_id]["source"] is not None
    ]
    dependencies.sort(key=lambda package: (package["name"], package["version"]))

    outputs: dict[PurePosixPath, bytes] = {}
    rows: list[str] = []
    runtime_count = 0
    build_count = 0
    dev_count = 0
    for package in dependencies:
        role = roles[package["id"]]
        if role == "runtime":
            runtime_count += 1
        elif role == "build-only":
            build_count += 1
        else:
            dev_count += 1
        source = package["source"]
        lock_key = (package["name"], package["version"], source)
        if lock_key not in locks:
            raise ValueError(f"dependency is absent from Cargo.lock: {lock_key!r}")
        checksum = locks[lock_key].get("checksum", "not supplied by source")
        spdx = normalized_spdx(package.get("license"), package["name"])
        bundled: list[str] = []
        if role == "runtime":
            package_root = Path(package["manifest_path"]).parent
            notices = notice_files(package)
            if not notices:
                raise ValueError(
                    f"runtime dependency has no discoverable licence/notice text: "
                    f"{package['name']} {package['version']}"
                )
            destination_root = PurePosixPath(
                "THIRD_PARTY_LICENSES"
            ) / package_directory(package)
            for source_path in notices:
                relative = PurePosixPath(
                    source_path.relative_to(package_root).as_posix()
                )
                destination = destination_root / relative
                outputs[destination] = source_path.read_bytes()
                bundled.append(destination.as_posix())
        notice_description = (
            "<br>".join(f"`{path}`" for path in bundled)
            if bundled
            else "Not bundled; package is not linked into the runtime binary"
        )
        rows.append(
            "| "
            + " | ".join(
                (
                    f"`{markdown(package['name'])}`",
                    f"`{markdown(package['version'])}`",
                    role,
                    f"`{markdown(source)}`",
                    f"`{markdown(checksum)}`",
                    f"`{markdown(spdx)}`",
                    notice_description,
                )
            )
            + " |\n"
        )

    lock_digest = hashlib.sha256(LOCK_PATH.read_bytes()).hexdigest()
    inventory = f"""# Binary dependency notice inventory

<!-- Generated by scripts/generate-binary-dependency-notices.py; do not edit. -->

Distribution target: `{target_name}` (`{root_package_name}`)

Locked graph: repository `Cargo.lock` SHA-256 `{lock_digest}`.

This inventory is derived from `cargo metadata --locked --format-version 1`
without a platform filter, so `runtime` is the conservative union of normal,
non-proc-macro dependencies that may be linked for any resolved target.
Target-conditional runtime notices can therefore be present even when a
particular binary does not contain that crate. `build-only` includes build
dependencies and procedural macros; `dev-only` dependencies are not part of a
release binary. Workspace-authored Emuella crates are excluded because their
licensing and derived-source provenance are documented by the distribution's
primary `LICENSE`, `NOTICE`, and `THIRD_PARTY.md` files.

Summary: {runtime_count} external runtime package(s), {build_count} external
build-only package(s), and {dev_count} external dev-only package(s).

| Package | Version | Binary role | Locked source | Package checksum | SPDX licence expression | Bundled licence/notice text |
| --- | --- | --- | --- | --- | --- | --- |
{"".join(rows) if rows else "| _None_ | — | — | — | — | — | No external Cargo dependencies |\n"}"""
    outputs[PurePosixPath("THIRD_PARTY_DEPENDENCIES.md")] = inventory.encode()
    return outputs


def actual_outputs(output_root: Path) -> dict[PurePosixPath, bytes]:
    paths: list[Path] = []
    inventory = output_root / "THIRD_PARTY_DEPENDENCIES.md"
    if inventory.is_file():
        paths.append(inventory)
    licenses = output_root / "THIRD_PARTY_LICENSES"
    if licenses.is_dir():
        paths.extend(path for path in licenses.rglob("*") if path.is_file())
    return {
        PurePosixPath(path.relative_to(output_root).as_posix()): path.read_bytes()
        for path in paths
    }


def check_outputs(output_root: Path, expected: dict[PurePosixPath, bytes]) -> list[str]:
    actual = actual_outputs(output_root)
    errors: list[str] = []
    for missing in sorted(set(expected) - set(actual)):
        errors.append(f"missing generated notice file: {output_root / missing}")
    for stale in sorted(set(actual) - set(expected)):
        errors.append(f"stale generated notice file: {output_root / stale}")
    for path in sorted(set(expected) & set(actual)):
        if expected[path] != actual[path]:
            errors.append(f"outdated generated notice file: {output_root / path}")
    return errors


def write_outputs(output_root: Path, expected: dict[PurePosixPath, bytes]) -> None:
    inventory = output_root / "THIRD_PARTY_DEPENDENCIES.md"
    licenses = output_root / "THIRD_PARTY_LICENSES"
    if inventory.exists():
        inventory.unlink()
    if licenses.exists():
        shutil.rmtree(licenses)
    for relative, content in expected.items():
        destination = output_root / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(content)


def main() -> int:
    options = arguments()
    metadata = cargo_metadata()
    locks = lock_packages()
    generated = {
        name: expected_outputs(name, package, output, metadata, locks)
        for name, (package, output) in TARGETS.items()
    }
    if options.write:
        for name, (_, output_root) in TARGETS.items():
            write_outputs(output_root, generated[name])
        print(f"generated binary dependency notices for {len(TARGETS)} targets")
        return 0

    errors: list[str] = []
    for name, (_, output_root) in TARGETS.items():
        errors.extend(check_outputs(output_root, generated[name]))
    if errors:
        print("binary dependency notice check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print(f"binary dependency notice check passed: {len(TARGETS)} targets")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
