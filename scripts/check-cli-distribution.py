#!/usr/bin/env python3
"""Inspect a prebuilt CLI archive and record its release evidence."""

from __future__ import annotations

import argparse
import hashlib
import stat
import sys
import tarfile
from pathlib import Path, PurePosixPath

sys.dont_write_bytecode = True

from public_tree_policy import content_policy_errors, sha256_bytes  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
CLI_PROJECT = ROOT / "crates/emuella-j2k-cli"


def arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--archive", required=True, type=Path)
    parser.add_argument("--evidence-dir", required=True, type=Path)
    parser.add_argument("--expected-binary", required=True, type=Path)
    return parser.parse_args()


def expected_files() -> dict[str, bytes]:
    result = {
        "LICENSE-APACHE-2.0": (ROOT / "LICENSE").read_bytes(),
        "LICENSE-OPENJPH-BSD-2-CLAUSE": (
            ROOT / "LICENSES/OpenJPH-BSD-2-Clause.txt"
        ).read_bytes(),
        "NOTICE": (ROOT / "NOTICE").read_bytes(),
        "THIRD_PARTY.md": (ROOT / "THIRD_PARTY.md").read_bytes(),
        "THIRD_PARTY_DEPENDENCIES.md": (
            CLI_PROJECT / "THIRD_PARTY_DEPENDENCIES.md"
        ).read_bytes(),
    }
    license_root = CLI_PROJECT / "THIRD_PARTY_LICENSES"
    if license_root.is_dir():
        result.update(
            {
                path.relative_to(CLI_PROJECT).as_posix(): path.read_bytes()
                for path in sorted(license_root.rglob("*"))
                if path.is_file()
            }
        )
    return result


def safe_archive_path(name: str) -> bool:
    path = PurePosixPath(name)
    return (
        bool(name)
        and not path.is_absolute()
        and ".." not in path.parts
        and "\\" not in name
    )


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def main() -> int:
    options = arguments()
    archive_path = options.archive.resolve()
    evidence_dir = options.evidence_dir.resolve()
    evidence_dir.mkdir(parents=True, exist_ok=True)
    errors: list[str] = []
    expected = expected_files()
    expected_binary_path = options.expected_binary.resolve()
    expected_binary_hash = None
    if not expected_binary_path.is_file():
        errors.append(f"expected CLI build output is missing: {expected_binary_path}")
    else:
        expected_binary_hash = sha256(expected_binary_path)

    try:
        with tarfile.open(archive_path, mode="r:gz") as archive:
            members = archive.getmembers()
            names = [member.name for member in members]
            (evidence_dir / "members.txt").write_text(
                "".join(f"{name}\n" for name in sorted(names)), encoding="utf-8"
            )
            if len(names) != len(set(names)):
                errors.append("CLI archive contains duplicate member names")
            for member in members:
                if not safe_archive_path(member.name):
                    errors.append(f"unsafe CLI archive path: {member.name!r}")
                if member.issym() or member.islnk():
                    errors.append(f"CLI archive contains a link: {member.name}")
                elif not member.isfile() and not member.isdir():
                    errors.append(
                        f"CLI archive contains unsupported member: {member.name}"
                    )

            roots = {PurePosixPath(name).parts[0] for name in names if name}
            if len(roots) != 1:
                errors.append(f"CLI archive has multiple roots: {sorted(roots)!r}")
            else:
                root = PurePosixPath(next(iter(roots)))
                expected_members = {(root / name).as_posix() for name in expected}
                binary_name = (root / "emuella-j2k").as_posix()
                binary_exceptions = (
                    {PurePosixPath("emuella-j2k"): expected_binary_hash}
                    if expected_binary_hash is not None
                    else {}
                )
                actual_files = {member.name for member in members if member.isfile()}
                if actual_files != expected_members | {binary_name}:
                    errors.append(
                        f"CLI archive files {sorted(actual_files)!r} do not match "
                        f"{sorted(expected_members | {binary_name})!r}"
                    )
                for name, expected_content in expected.items():
                    member_name = (root / name).as_posix()
                    if member_name not in actual_files:
                        continue
                    extracted = archive.extractfile(member_name)
                    if extracted is None or extracted.read() != expected_content:
                        errors.append(f"CLI legal file content differs: {name}")
                for member in members:
                    if not member.isfile() or not safe_archive_path(member.name):
                        continue
                    member_path = PurePosixPath(member.name)
                    try:
                        relative = member_path.relative_to(root)
                    except ValueError:
                        continue
                    extracted = archive.extractfile(member)
                    if extracted is None:
                        errors.append(f"cannot read CLI archive member: {member.name}")
                        continue
                    content = extracted.read()
                    errors.extend(
                        f"CLI archive: {error}"
                        for error in content_policy_errors(
                            relative,
                            content,
                            hash_exceptions=binary_exceptions,
                        )
                    )
                    if member.name == binary_name:
                        (evidence_dir / "binary-SHA256SUMS").write_text(
                            f"{expected_binary_hash}  "
                            f"build/{expected_binary_path.name}\n"
                            f"{sha256_bytes(content)}  archive/emuella-j2k\n",
                            encoding="utf-8",
                        )
                binary_members = [
                    member for member in members if member.name == binary_name
                ]
                if len(binary_members) != 1 or not binary_members[0].isfile():
                    errors.append("CLI archive does not contain one regular executable")
                elif not binary_members[0].mode & stat.S_IXUSR:
                    errors.append("CLI archive binary is not executable")
    except (OSError, tarfile.TarError) as error:
        errors.append(f"cannot inspect CLI archive: {error}")

    (evidence_dir / "SHA256SUMS").write_text(
        f"{sha256(archive_path)}  dist/{archive_path.name}\n", encoding="utf-8"
    )
    if errors:
        print("CLI distribution check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print(f"CLI distribution check passed: {archive_path.name}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
