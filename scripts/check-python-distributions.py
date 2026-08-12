#!/usr/bin/env python3
"""Inspect Python release archives and record reproducible release evidence."""

from __future__ import annotations

import argparse
import hashlib
import stat
import sys
import tarfile
import zipfile
from email import policy
from email.parser import BytesParser
from pathlib import Path, PurePosixPath

sys.dont_write_bytecode = True

from public_tree_policy import content_policy_errors, sha256_bytes  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
PYTHON_PROJECT = ROOT / "crates/emuella-j2k-python"
EXPECTED_LICENSE_EXPRESSION = "Apache-2.0 AND BSD-2-Clause"
EXPECTED_TOP_LEVEL_LEGAL_FILES = (
    "LICENSE-APACHE-2.0",
    "LICENSE-OPENJPH-BSD-2-CLAUSE",
    "NOTICE",
    "THIRD_PARTY.md",
    "THIRD_PARTY_DEPENDENCIES.md",
)
COMPILED_EXTENSION_SUFFIXES = frozenset({".dll", ".dylib", ".pyd", ".so"})


def compiled_extension_member(value: str) -> PurePosixPath:
    path = PurePosixPath(value)
    if (
        path.parent != PurePosixPath("emuella_j2k")
        or not path.name.startswith("emuella_j2k.")
        or path.suffix.lower() not in COMPILED_EXTENSION_SUFFIXES
    ):
        raise argparse.ArgumentTypeError(
            "compiled extension member must be the interpreter-derived "
            "emuella_j2k/emuella_j2k.<ABI> path"
        )
    return path


def arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--dist-dir", required=True, type=Path)
    parser.add_argument("--evidence-dir", required=True, type=Path)
    parser.add_argument("--wheel-binary", required=True, type=Path)
    parser.add_argument(
        "--wheel-binary-member", required=True, type=compiled_extension_member
    )
    parser.add_argument("--rebuilt-wheel-dir", type=Path)
    parser.add_argument("--rebuilt-wheel-binary", type=Path)
    return parser.parse_args()


def one_artifact(paths: list[Path], description: str, errors: list[str]) -> Path | None:
    if len(paths) != 1:
        errors.append(f"expected exactly one {description}, found {len(paths)}")
        return None
    return paths[0]


def safe_archive_path(name: str) -> bool:
    path = PurePosixPath(name)
    return (
        bool(name)
        and not path.is_absolute()
        and ".." not in path.parts
        and "\\" not in name
    )


def source_legal_files() -> dict[str, bytes]:
    result = {
        name: (PYTHON_PROJECT / name).read_bytes()
        for name in EXPECTED_TOP_LEVEL_LEGAL_FILES
    }
    license_root = PYTHON_PROJECT / "THIRD_PARTY_LICENSES"
    result.update(
        {
            path.relative_to(PYTHON_PROJECT).as_posix(): path.read_bytes()
            for path in sorted(license_root.rglob("*"))
            if path.is_file()
        }
    )
    return result


def check_metadata(
    raw: bytes, label: str, expected_legal_files: set[str], errors: list[str]
) -> None:
    metadata = BytesParser(policy=policy.default).parsebytes(raw)
    expression = metadata.get("License-Expression")
    if expression != EXPECTED_LICENSE_EXPRESSION:
        errors.append(
            f"{label}: License-Expression {expression!r} does not match "
            f"{EXPECTED_LICENSE_EXPRESSION!r}"
        )
    license_files = metadata.get_all("License-File", [])
    if len(license_files) != len(set(license_files)):
        errors.append(f"{label}: duplicate License-File metadata entries")
    if set(license_files) != expected_legal_files:
        errors.append(
            f"{label}: License-File entries {license_files!r} do not match "
            f"{sorted(expected_legal_files)!r}"
        )


def write_evidence(path: Path, content: bytes | str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    if isinstance(content, bytes):
        path.write_bytes(content)
    else:
        path.write_text(content, encoding="utf-8")


def member_manifest(names: list[str]) -> str:
    return "".join(f"{name}\n" for name in sorted(names))


def check_wheel(
    wheel_path: Path,
    expected_binary_path: Path,
    expected_binary_member: PurePosixPath,
    label: str,
    evidence_dir: Path,
    legal_sources: dict[str, bytes],
    errors: list[str],
) -> None:
    try:
        with zipfile.ZipFile(wheel_path) as archive:
            infos = archive.infolist()
            names = [info.filename for info in infos]
            write_evidence(
                evidence_dir / f"{label}-members.txt", member_manifest(names)
            )
            if len(names) != len(set(names)):
                errors.append(f"{label}: archive contains duplicate member names")
            compiled_extensions = [
                info
                for info in infos
                if not info.is_dir()
                and PurePosixPath(info.filename).suffix.lower()
                in COMPILED_EXTENSION_SUFFIXES
            ]
            binary_exceptions: dict[PurePosixPath, str] = {}
            compiled_paths = {
                PurePosixPath(info.filename) for info in compiled_extensions
            }
            if compiled_paths != {expected_binary_member}:
                errors.append(
                    f"{label}: compiled extension members "
                    f"{sorted(str(path) for path in compiled_paths)!r} do not match "
                    f"the reviewed path {str(expected_binary_member)!r}"
                )
            else:
                compiled_path = expected_binary_member
                if not expected_binary_path.is_file():
                    errors.append(
                        f"{label}: expected build output is missing: "
                        f"{expected_binary_path}"
                    )
                else:
                    expected_binary_hash = sha256(expected_binary_path)
                    binary_exceptions[compiled_path] = expected_binary_hash
                    write_evidence(
                        evidence_dir / f"{label}-binary-SHA256SUMS",
                        f"{expected_binary_hash}  build/{expected_binary_path.name}\n"
                        f"{sha256_bytes(archive.read(compiled_extensions[0]))}  "
                        f"archive/{compiled_path}\n",
                    )
            for info in infos:
                if not safe_archive_path(info.filename):
                    errors.append(f"{label}: unsafe archive path {info.filename!r}")
                    continue
                mode = info.external_attr >> 16
                if stat.S_ISLNK(mode):
                    errors.append(f"{label}: archive contains a link: {info.filename}")
                    continue
                file_type = stat.S_IFMT(mode)
                if file_type not in {0, stat.S_IFDIR, stat.S_IFREG}:
                    errors.append(
                        f"{label}: archive contains unsupported member: {info.filename}"
                    )
                    continue
                if info.is_dir():
                    continue
                policy_path = PurePosixPath(info.filename)
                errors.extend(
                    f"{label}: {error}"
                    for error in content_policy_errors(
                        policy_path,
                        archive.read(info),
                        hash_exceptions=binary_exceptions,
                    )
                )

            metadata_names = [
                name for name in names if name.endswith(".dist-info/METADATA")
            ]
            if len(metadata_names) != 1:
                errors.append(
                    f"{label}: expected one .dist-info/METADATA, "
                    f"found {len(metadata_names)}"
                )
                return
            metadata_name = metadata_names[0]
            metadata = archive.read(metadata_name)
            write_evidence(evidence_dir / f"{label}-METADATA.txt", metadata)
            check_metadata(metadata, label, set(legal_sources), errors)

            license_root = PurePosixPath(metadata_name).parent / "licenses"
            expected_members = {
                (license_root / name).as_posix() for name in legal_sources
            }
            actual_members = {
                name
                for name in names
                if PurePosixPath(name).is_relative_to(license_root)
                and PurePosixPath(name) != license_root
            }
            if actual_members != expected_members:
                errors.append(
                    f"{label}: legal members {sorted(actual_members)!r} do not match "
                    f"{sorted(expected_members)!r}"
                )
            for name, expected_content in legal_sources.items():
                member = (license_root / name).as_posix()
                if member in names and archive.read(member) != expected_content:
                    errors.append(f"{label}: legal file content differs: {name}")
    except (OSError, zipfile.BadZipFile) as error:
        errors.append(f"{label}: cannot inspect {wheel_path}: {error}")


def check_sdist(
    sdist_path: Path,
    evidence_dir: Path,
    legal_sources: dict[str, bytes],
    errors: list[str],
) -> None:
    label = "sdist"
    try:
        with tarfile.open(sdist_path, mode="r:gz") as archive:
            members = archive.getmembers()
            names = [member.name for member in members]
            write_evidence(evidence_dir / "sdist-members.txt", member_manifest(names))
            if len(names) != len(set(names)):
                errors.append("sdist: archive contains duplicate member names")
            for member in members:
                if not safe_archive_path(member.name):
                    errors.append(f"sdist: unsafe archive path {member.name!r}")
                if member.issym() or member.islnk():
                    errors.append(f"sdist: archive contains a link: {member.name}")

            roots = {PurePosixPath(name).parts[0] for name in names if name}
            if len(roots) != 1:
                errors.append(
                    f"sdist: expected one archive root, found {sorted(roots)!r}"
                )
                return
            root = PurePosixPath(next(iter(roots)))
            for member in members:
                if not safe_archive_path(member.name):
                    continue
                if member.issym() or member.islnk():
                    continue
                if not member.isfile() and not member.isdir():
                    errors.append(f"sdist: unsupported archive member: {member.name}")
                    continue
                if not member.isfile():
                    continue
                member_path = PurePosixPath(member.name)
                try:
                    relative = member_path.relative_to(root)
                except ValueError:
                    errors.append(f"sdist: member escapes archive root: {member.name}")
                    continue
                extracted = archive.extractfile(member)
                if extracted is None:
                    errors.append(f"sdist: cannot read archive member: {member.name}")
                    continue
                errors.extend(
                    f"sdist: {error}"
                    for error in content_policy_errors(relative, extracted.read())
                )
            metadata_name = (root / "PKG-INFO").as_posix()
            if metadata_name not in names:
                errors.append("sdist: PKG-INFO is missing from the archive root")
                return
            extracted_metadata = archive.extractfile(metadata_name)
            if extracted_metadata is None:
                errors.append("sdist: PKG-INFO is not a regular file")
                return
            metadata = extracted_metadata.read()
            write_evidence(evidence_dir / "sdist-PKG-INFO.txt", metadata)
            check_metadata(metadata, label, set(legal_sources), errors)

            expected_members = {(root / name).as_posix() for name in legal_sources}
            actual_members = {
                name
                for name in names
                if (
                    PurePosixPath(name).parent == root
                    and PurePosixPath(name).name in set(EXPECTED_TOP_LEVEL_LEGAL_FILES)
                )
                or (
                    len(PurePosixPath(name).parts) > 1
                    and PurePosixPath(name).parts[0] == root.name
                    and PurePosixPath(name).parts[1] == "THIRD_PARTY_LICENSES"
                )
            }
            if actual_members != expected_members:
                errors.append(
                    f"sdist: legal members {sorted(actual_members)!r} do not match "
                    f"{sorted(expected_members)!r}"
                )
            for name, expected_content in legal_sources.items():
                member_name = (root / name).as_posix()
                if member_name not in names:
                    continue
                extracted = archive.extractfile(member_name)
                if extracted is None:
                    errors.append(f"sdist: legal member is not a regular file: {name}")
                elif extracted.read() != expected_content:
                    errors.append(f"sdist: legal file content differs: {name}")
    except (OSError, tarfile.TarError) as error:
        errors.append(f"sdist: cannot inspect {sdist_path}: {error}")


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def write_hashes(paths: list[Path], evidence_dir: Path) -> None:
    lines = []
    for path in sorted(paths):
        try:
            displayed = path.resolve().relative_to(evidence_dir.resolve()).as_posix()
        except ValueError:
            displayed = path.name
        lines.append(f"{sha256(path)}  {displayed}\n")
    write_evidence(evidence_dir / "SHA256SUMS", "".join(lines))


def main() -> int:
    options = arguments()
    dist_dir = options.dist_dir.resolve()
    evidence_dir = options.evidence_dir.resolve()
    evidence_dir.mkdir(parents=True, exist_ok=True)

    if (options.rebuilt_wheel_dir is None) != (options.rebuilt_wheel_binary is None):
        print(
            "Python distribution check failed:\n"
            "- --rebuilt-wheel-dir and --rebuilt-wheel-binary must be used together",
            file=sys.stderr,
        )
        return 1

    errors: list[str] = []
    wheel = one_artifact(sorted(dist_dir.glob("*.whl")), "wheel", errors)
    sdist = one_artifact(
        sorted(dist_dir.glob("*.tar.gz")), "source distribution", errors
    )
    rebuilt_wheel = None
    if options.rebuilt_wheel_dir is not None:
        rebuilt_wheel = one_artifact(
            sorted(options.rebuilt_wheel_dir.resolve().glob("*.whl")),
            "wheel rebuilt from the sdist",
            errors,
        )

    legal_sources = source_legal_files()
    artifacts: list[Path] = []
    if wheel is not None:
        artifacts.append(wheel)
        check_wheel(
            wheel,
            options.wheel_binary.resolve(),
            options.wheel_binary_member,
            "wheel",
            evidence_dir,
            legal_sources,
            errors,
        )
    if sdist is not None:
        artifacts.append(sdist)
        check_sdist(sdist, evidence_dir, legal_sources, errors)
    if rebuilt_wheel is not None:
        artifacts.append(rebuilt_wheel)
        check_wheel(
            rebuilt_wheel,
            options.rebuilt_wheel_binary.resolve(),
            options.wheel_binary_member,
            "rebuilt-wheel",
            evidence_dir,
            legal_sources,
            errors,
        )
    write_hashes(artifacts, evidence_dir)

    if errors:
        print("Python distribution check failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    suffix = " including the sdist rebuild" if rebuilt_wheel is not None else ""
    print(f"Python distribution check passed{suffix}: {len(artifacts)} artifacts")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
