#!/usr/bin/env python3
"""Verify release archives after `cargo package --workspace --locked`."""

from __future__ import annotations

import json
import os
import sys
import tarfile
from pathlib import Path, PurePosixPath

import tomllib

sys.dont_write_bytecode = True

from public_tree_policy import content_policy_errors  # noqa: E402
from package_legal_policy import (  # noqa: E402
    PACKAGE_POLICY,
    PackageLegalPolicy,
    legal_content_errors,
)

ROOT = Path(__file__).resolve().parent.parent
PACKAGE_DIRECTORY = ROOT / "target/package"


def workspace_package_names() -> set[str]:
    manifest = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    names: set[str] = set()
    for relative in manifest["workspace"]["members"]:
        member = tomllib.loads(
            (ROOT / relative / "Cargo.toml").read_text(encoding="utf-8")
        )
        names.add(member["package"]["name"])
    return names


def package_version() -> str:
    manifest = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    return manifest["workspace"]["package"]["version"]


def archive_text(archive: tarfile.TarFile, member_name: str) -> str:
    member = archive.getmember(member_name)
    extracted = archive.extractfile(member)
    if extracted is None:
        raise ValueError(f"archive member is not a regular file: {member_name}")
    return extracted.read().decode("utf-8")


def dependency_path_errors(manifest: dict[str, object]) -> list[str]:
    errors: list[str] = []

    def visit(node: object, location: str) -> None:
        if not isinstance(node, dict):
            return
        for key, value in node.items():
            child_location = f"{location}.{key}" if location else str(key)
            if key in {
                "dependencies",
                "dev-dependencies",
                "build-dependencies",
            } and isinstance(value, dict):
                for dependency, specification in value.items():
                    if isinstance(specification, dict) and "path" in specification:
                        errors.append(
                            f"{child_location}.{dependency} retains a local path"
                        )
            visit(value, child_location)

    visit(manifest, "")
    return errors


def check_archive(
    package_name: str,
    version: str,
    package_policy: PackageLegalPolicy,
) -> list[str]:
    errors: list[str] = []
    archive_path = PACKAGE_DIRECTORY / f"{package_name}-{version}.crate"
    if not archive_path.is_file():
        return [f"missing package archive: {archive_path.relative_to(ROOT)}"]

    prefix = PurePosixPath(f"{package_name}-{version}")
    with tarfile.open(archive_path, mode="r:gz") as archive:
        members = archive.getmembers()
        names = [member.name for member in members]
        if len(names) != len(set(names)):
            errors.append(f"{archive_path.name}: archive contains duplicate members")
        relative_members: set[PurePosixPath] = set()
        for member in members:
            member_path = PurePosixPath(member.name)
            if member_path.is_absolute() or ".." in member_path.parts:
                errors.append(f"{archive_path.name}: unsafe archive path {member.name}")
                continue
            try:
                relative = member_path.relative_to(prefix)
            except ValueError:
                errors.append(
                    f"{archive_path.name}: member escapes package root: {member.name}"
                )
                continue
            if member.issym() or member.islnk():
                errors.append(
                    f"{archive_path.name}: archive contains a link: {member.name}"
                )
            elif not member.isfile() and not member.isdir():
                errors.append(
                    f"{archive_path.name}: unsupported archive member: {member.name}"
                )
            elif member.isfile():
                extracted = archive.extractfile(member)
                if extracted is None:
                    errors.append(
                        f"{archive_path.name}: cannot read archive member: {member.name}"
                    )
                else:
                    errors.extend(
                        f"{archive_path.name}: {error}"
                        for error in content_policy_errors(relative, extracted.read())
                    )
            relative_members.add(relative)

        root_legal_file_names = frozenset(
            relative.name
            for relative in relative_members
            if len(relative.parts) == 1
            and (
                relative.name.startswith("LICENSE")
                or relative.name in {"NOTICE", "THIRD_PARTY.md"}
            )
        )
        legal_files: dict[str, bytes] = {}
        for name in root_legal_file_names:
            member_name = (prefix / name).as_posix()
            extracted = archive.extractfile(member_name)
            if extracted is None:
                errors.append(
                    f"{archive_path.name}: legal member is not a regular file: {name}"
                )
            else:
                legal_files[name] = extracted.read()
        errors.extend(
            f"{archive_path.name}: {error}"
            for error in legal_content_errors(package_name, legal_files)
        )

        if (
            package_name == "emuella-j2k-python"
            and PurePosixPath("pyproject.toml") not in relative_members
        ):
            errors.append(f"{archive_path.name}: Python package omits pyproject.toml")

        normalized_name = (prefix / "Cargo.toml").as_posix()
        try:
            normalized = tomllib.loads(archive_text(archive, normalized_name))
        except (
            KeyError,
            UnicodeDecodeError,
            tomllib.TOMLDecodeError,
            ValueError,
        ) as error:
            errors.append(
                f"{archive_path.name}: cannot read normalized Cargo.toml: {error}"
            )
        else:
            packaged_license = normalized.get("package", {}).get("license")
            if packaged_license != package_policy.license_expression:
                errors.append(
                    f"{archive_path.name}: packaged licence {packaged_license!r} does not "
                    f"match {package_policy.license_expression!r}"
                )
            errors.extend(
                f"{archive_path.name}: {error}"
                for error in dependency_path_errors(normalized)
            )

        vcs_member = prefix / ".cargo_vcs_info.json"
        if (ROOT / ".git").exists():
            if vcs_member not in {prefix / relative for relative in relative_members}:
                errors.append(f"{archive_path.name}: missing .cargo_vcs_info.json")
            else:
                try:
                    vcs = json.loads(archive_text(archive, vcs_member.as_posix()))
                except (
                    KeyError,
                    UnicodeDecodeError,
                    json.JSONDecodeError,
                    ValueError,
                ) as error:
                    errors.append(f"{archive_path.name}: invalid VCS metadata: {error}")
                else:
                    git = vcs.get("git", {})
                    # Cargo omits `dirty` for a clean worktree and writes it only
                    # when packaging was allowed to proceed from a dirty tree.
                    # Accept both the omitted field and an explicit false value.
                    if git.get("dirty", False) is not False:
                        errors.append(
                            f"{archive_path.name}: package records a dirty worktree"
                        )
                    expected_sha = os.environ.get("GITHUB_SHA")
                    if expected_sha and git.get("sha1") != expected_sha:
                        errors.append(
                            f"{archive_path.name}: packaged commit {git.get('sha1')!r} "
                            f"does not match GITHUB_SHA"
                        )
    return errors


def main() -> int:
    errors: list[str] = []
    workspace_names = workspace_package_names()
    policy_names = set(PACKAGE_POLICY)
    if workspace_names != policy_names:
        missing = workspace_names - policy_names
        stale = policy_names - workspace_names
        if missing:
            errors.append(f"package policy is missing: {', '.join(sorted(missing))}")
        if stale:
            errors.append(
                f"package policy has stale entries: {', '.join(sorted(stale))}"
            )

    version = package_version()
    expected_archives = {
        PACKAGE_DIRECTORY / f"{package_name}-{version}.crate"
        for package_name in policy_names
    }
    actual_archives = set(PACKAGE_DIRECTORY.glob(f"*-{version}.crate"))
    unexpected_archives = actual_archives - expected_archives
    if unexpected_archives:
        errors.append(
            "unexpected package archives: "
            + ", ".join(sorted(archive.name for archive in unexpected_archives))
        )

    for package_name, package_policy in sorted(PACKAGE_POLICY.items()):
        errors.extend(check_archive(package_name, version, package_policy))

    if errors:
        print("package-content check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print(f"package-content check passed: {len(PACKAGE_POLICY)} archives")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
