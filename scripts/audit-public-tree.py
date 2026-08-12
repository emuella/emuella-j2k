#!/usr/bin/env python3
"""Fail closed when excluded private-release material enters the public tree."""

from __future__ import annotations

import os
import re
import sys
from pathlib import Path, PurePosixPath

import tomllib

sys.dont_write_bytecode = True

from public_tree_policy import (  # noqa: E402
    contains_public_openjph_identifier,
    content_policy_errors,
    exception_configuration_errors,
)

ROOT = Path(__file__).resolve().parent.parent

FORBIDDEN_COMPONENTS = {
    ".git",
    ".local",
    ".smoogle",
    ".pytest_cache",
    ".ruff_cache",
    "benchmark-output",
    "target",
    "third_party",
}
FORBIDDEN_PATH_TERMS = (
    "kakadu",
    "hayro-jpeg2000",
    "openjpeg-data",
    "exec-plans",
    "prompts",
    "standards",
)
# A binary or otherwise unapproved public-tree file may be admitted only by an
# exact repository-relative path and a reviewed SHA-256. Keep this mapping empty
# unless a source-tree payload has been explicitly approved.
PUBLIC_TREE_HASH_EXCEPTIONS: dict[PurePosixPath, str] = {}
ABSOLUTE_PRIVATE_PATH = re.compile(r"/(?:home|nvme|opt)/(?:[^\s`\"']+/)+")
OLD_PROJECT_IDENTIFIER = re.compile(
    r"(?<!emuella-)\bj2k(?:[-_.\s]?rs)\b", re.IGNORECASE
)
OLD_TESTDATA_ENVIRONMENT = re.compile(r"\bEMU_TESTDATA_(?:ROOT|CACHE)\b")
STALE_FIXTURE_SURFACE = re.compile(
    r"\b(?:FixtureBacked|fixture_backed|fixture_transfer_applied|FixtureReplay|"
    r"decode_htj2k_openjph_fixture|decode_htj2k_openjph_reference|"
    r"is_htj2k_openjph_reference_fixture|OPENJPEG_DATA_)\w*\b"
)
EMPTY_BYTE_ARRAY_CONSTANT = re.compile(
    r"\bconst\s+[A-Z][A-Z0-9_]*\s*:\s*&\[u8\]\s*=\s*&\[\s*\]\s*;"
)
REPOSITORY_URL = "https://github.com/emuella/emuella-j2k"
HOSTED_CI = os.environ.get("GITHUB_ACTIONS") == "true"


def files() -> tuple[list[Path], list[Path], list[Path]]:
    result: list[Path] = []
    forbidden_directories: list[Path] = []
    symbolic_link_directories: list[Path] = []
    for directory, names, filenames in os.walk(ROOT):
        relative_directory = Path(directory).relative_to(ROOT)
        names[:] = sorted(names)
        for name in names:
            relative = relative_directory / name
            if name in FORBIDDEN_COMPONENTS and relative != Path(".git"):
                forbidden_directories.append(relative)
            if (ROOT / relative).is_symlink():
                symbolic_link_directories.append(relative)
        names[:] = [
            name
            for name in names
            if name not in FORBIDDEN_COMPONENTS
            and not (ROOT / relative_directory / name).is_symlink()
        ]
        for filename in sorted(filenames):
            result.append(relative_directory / filename)
    return result, forbidden_directories, symbolic_link_directories


def main() -> int:
    errors: list[str] = []
    paths, forbidden_directories, symbolic_link_directories = files()
    errors.extend(
        f"forbidden path component: {relative}" for relative in forbidden_directories
    )
    errors.extend(
        f"public tree contains a symbolic-link directory: {relative}"
        for relative in symbolic_link_directories
    )
    policy_paths = {PurePosixPath(relative.as_posix()) for relative in paths}
    errors.extend(
        exception_configuration_errors(policy_paths, PUBLIC_TREE_HASH_EXCEPTIONS)
    )
    for relative in paths:
        absolute = ROOT / relative
        policy_path = PurePosixPath(relative.as_posix())
        lowered = relative.as_posix().lower()
        if FORBIDDEN_COMPONENTS.intersection(relative.parts):
            errors.append(f"forbidden path component: {relative}")
        if any(term in lowered for term in FORBIDDEN_PATH_TERMS):
            errors.append(f"forbidden path term: {relative}")
        if absolute.is_symlink():
            errors.append(f"public tree contains a symbolic link: {relative}")
            continue
        if not absolute.is_file():
            errors.append(f"public tree contains a non-regular file: {relative}")
            continue
        try:
            content = absolute.read_bytes()
        except OSError as error:
            errors.append(f"cannot read public file {relative}: {error}")
            continue
        policy_errors = content_policy_errors(
            policy_path,
            content,
            hash_exceptions=PUBLIC_TREE_HASH_EXCEPTIONS,
        )
        errors.extend(policy_errors)
        if policy_errors or policy_path in PUBLIC_TREE_HASH_EXCEPTIONS:
            continue
        text = content.decode("utf-8")
        if ABSOLUTE_PRIVATE_PATH.search(text):
            errors.append(f"absolute private path in {relative}")
        if relative != Path("scripts/audit-public-tree.py"):
            if OLD_PROJECT_IDENTIFIER.search(text):
                errors.append(f"old project identifier in {relative}")
            if OLD_TESTDATA_ENVIRONMENT.search(text):
                errors.append(f"old testdata environment variable in {relative}")
            if relative.suffix == ".rs" and STALE_FIXTURE_SURFACE.search(text):
                errors.append(f"stale external-fixture API or state in {relative}")
            if relative.suffix == ".rs" and EMPTY_BYTE_ARRAY_CONSTANT.search(text):
                errors.append(f"empty byte-array fixture placeholder in {relative}")
            if relative.suffix == ".rs" and contains_public_openjph_identifier(text):
                errors.append(f"external-codec name in public Rust API: {relative}")

    cargo = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    workspace = cargo.get("workspace", {})
    workspace_package = workspace.get("package", {})
    if workspace_package.get("repository") != REPOSITORY_URL:
        errors.append("workspace repository URL is missing or incorrect")
    if workspace_package.get("homepage") != REPOSITORY_URL:
        errors.append("workspace homepage URL is missing or incorrect")
    if "crates/emuella-j2k" not in workspace.get("members", []):
        errors.append("primary emuella-j2k facade is not a workspace member")
    if "crates/emuella-j2k-codestream/fuzz" not in workspace.get("exclude", []):
        errors.append("nested fuzz workspace is not explicitly excluded from the root")

    top_level_manifests = set((ROOT / "crates").glob("*/Cargo.toml"))
    all_crate_manifests = set((ROOT / "crates").glob("**/Cargo.toml"))
    fuzz_manifest_path = ROOT / "crates/emuella-j2k-codestream/fuzz/Cargo.toml"
    nested_manifests = all_crate_manifests - top_level_manifests
    if nested_manifests != {fuzz_manifest_path}:
        unexpected = sorted(path.relative_to(ROOT) for path in nested_manifests)
        errors.append(f"unreviewed nested Cargo manifests: {unexpected}")

    for manifest_path in sorted(top_level_manifests):
        manifest = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
        package = manifest.get("package", {})
        expected_name = manifest_path.parent.name
        if package.get("name") != expected_name:
            errors.append(
                f"Cargo package/path name mismatch: {manifest_path.relative_to(ROOT)}"
            )
        if package.get("repository") != {"workspace": True}:
            errors.append(f"Cargo package does not inherit repository: {expected_name}")
        if package.get("homepage") != {"workspace": True}:
            errors.append(f"Cargo package does not inherit homepage: {expected_name}")

    fuzz = tomllib.loads(fuzz_manifest_path.read_text(encoding="utf-8"))
    fuzz_package = fuzz.get("package", {})
    if fuzz_package.get("name") != "emuella-j2k-codestream-fuzz":
        errors.append("fuzz package name is missing or incorrect")
    if fuzz_package.get("license") != "Apache-2.0":
        errors.append("fuzz package does not declare its Apache-2.0 source licence")
    if fuzz_package.get("repository") != REPOSITORY_URL:
        errors.append("fuzz package repository URL is missing or incorrect")
    if fuzz_package.get("homepage") != REPOSITORY_URL:
        errors.append("fuzz package homepage URL is missing or incorrect")
    if fuzz_package.get("publish") is not False:
        errors.append("fuzz package must remain unpublished")
    if not (fuzz_manifest_path.parent / "LICENSE-APACHE-2.0").is_file():
        errors.append("fuzz package does not include its declared Apache-2.0 licence")
    if fuzz.get("workspace") != {}:
        errors.append("fuzz package is not an explicit nested workspace")
    fuzz_dependencies = fuzz.get("dependencies", {})
    expected_fuzz_paths = {
        "emuella-j2k-codestream": "..",
        "emuella-j2k-container": "../../emuella-j2k-container",
        "emuella-j2k-ht": "../../emuella-j2k-ht",
    }
    for dependency, expected_path in expected_fuzz_paths.items():
        specification = fuzz_dependencies.get(dependency, {})
        if specification.get("path") != expected_path:
            errors.append(f"fuzz dependency path is missing or incorrect: {dependency}")
        if specification.get("version") != workspace_package.get("version"):
            errors.append(
                f"fuzz dependency version is missing or incorrect: {dependency}"
            )
    if fuzz_dependencies.get("libfuzzer-sys") != "0.4":
        errors.append("fuzz package libfuzzer-sys dependency is missing or unreviewed")

    fuzz_lock_path = fuzz_manifest_path.parent / "Cargo.lock"
    fuzz_gitignore = (fuzz_manifest_path.parent / ".gitignore").read_text(
        encoding="utf-8"
    )
    if "Cargo.lock" in fuzz_gitignore.splitlines():
        errors.append("fuzz workspace lockfile is ignored")
    if not fuzz_lock_path.is_file():
        errors.append("fuzz workspace has no independent Cargo.lock")
    else:
        fuzz_lock = tomllib.loads(fuzz_lock_path.read_text(encoding="utf-8"))
        locked_packages = {
            package.get("name"): package.get("version")
            for package in fuzz_lock.get("package", [])
        }
        if locked_packages.get("libfuzzer-sys") is None:
            errors.append("fuzz lockfile does not contain libfuzzer-sys")
    fuzz_deny_path = fuzz_manifest_path.parent / "deny.toml"
    if not fuzz_deny_path.is_file():
        errors.append("fuzz workspace has no dedicated cargo-deny policy")
    else:
        fuzz_deny = tomllib.loads(fuzz_deny_path.read_text(encoding="utf-8"))
        if "NCSA" not in fuzz_deny.get("licenses", {}).get("allow", []):
            errors.append(
                "fuzz cargo-deny policy does not acknowledge libFuzzer's NCSA licence"
            )

    facade = tomllib.loads(
        (ROOT / "crates/emuella-j2k/Cargo.toml").read_text(encoding="utf-8")
    )
    if facade.get("package", {}).get("name") != "emuella-j2k":
        errors.append("primary facade package is not named emuella-j2k")
    if facade.get("package", {}).get("documentation") != "https://docs.rs/emuella-j2k":
        errors.append("primary facade documentation URL is missing or incorrect")

    cli = tomllib.loads(
        (ROOT / "crates/emuella-j2k-cli/Cargo.toml").read_text(encoding="utf-8")
    )
    if [target.get("name") for target in cli.get("bin", [])] != ["emuella-j2k"]:
        errors.append("CLI executable is not named emuella-j2k")

    python = tomllib.loads(
        (ROOT / "crates/emuella-j2k-python/pyproject.toml").read_text(encoding="utf-8")
    )
    if python.get("project", {}).get("name") != "emuella-j2k":
        errors.append("Python distribution is not named emuella-j2k")
    if python.get("build-system", {}).get("requires") != ["maturin>=1.14.1,<2"]:
        errors.append("Python distribution does not pin the reviewed Maturin range")
    if python.get("project", {}).get("license") != "Apache-2.0 AND BSD-2-Clause":
        errors.append("Python distribution does not declare its combined SPDX licence")
    expected_python_license_files = [
        "LICENSE-APACHE-2.0",
        "LICENSE-OPENJPH-BSD-2-CLAUSE",
        "NOTICE",
        "THIRD_PARTY.md",
        "THIRD_PARTY_DEPENDENCIES.md",
        "THIRD_PARTY_LICENSES/**/*",
    ]
    if python.get("project", {}).get("license-files") != expected_python_license_files:
        errors.append(
            "Python distribution licence-file inventory is missing or reordered"
        )
    python_root = ROOT / "crates/emuella-j2k-python"
    legacy_python_openjpeg_license = python_root / "LICENSE-OPENJPEG-BSD-2-CLAUSE"
    if legacy_python_openjpeg_license.exists():
        errors.append("Python distribution retains the obsolete OpenJPEG licence file")
    for license_file in expected_python_license_files[:5]:
        if not (python_root / license_file).is_file():
            errors.append(
                f"Python distribution licence file is missing: {license_file}"
            )
    python_dependency_licenses = python_root / "THIRD_PARTY_LICENSES"
    if not python_dependency_licenses.is_dir() or not any(
        path.is_file() for path in python_dependency_licenses.rglob("*")
    ):
        errors.append("Python runtime dependency licence inventory is empty")
    for legal_file in ("NOTICE", "THIRD_PARTY.md"):
        legal_text = (python_root / legal_file).read_text(encoding="utf-8")
        if "OpenJPEG" in legal_text:
            errors.append(
                f"Python distribution retains obsolete OpenJPEG provenance: {legal_file}"
            )
    if python.get("tool", {}).get("maturin", {}).get("module-name") != "emuella_j2k":
        errors.append("Python import module is not named emuella_j2k")

    if "EMUELLA_J2K_HT_BACKEND" not in (
        ROOT / "crates/emuella-j2k-accel/src/lib.rs"
    ).read_text(encoding="utf-8"):
        errors.append("codec environment-variable namespace is missing")

    ci_workflow = (ROOT / ".github/workflows/ci.yml").read_text(encoding="utf-8")
    if not ci_workflow.startswith("name: emuella-j2k CI\n"):
        errors.append("CI workflow display name is not branded")
    if "python3 scripts/test-public-tree-policy.py" not in ci_workflow:
        errors.append("hosted CI does not test the fail-closed public-file policy")
    if "python3 scripts/test-package-legal-policy.py" not in ci_workflow:
        errors.append("hosted CI does not test canonical package legal files")
    if "uses: EmbarkStudios/cargo-deny-action@v2" not in ci_workflow:
        errors.append("hosted CI does not enforce cargo-deny policy")
    if "python3 scripts/generate-binary-dependency-notices.py --check" not in (
        ci_workflow
    ):
        errors.append("hosted CI does not verify binary dependency notices")
    if (
        "manifest-path: ./crates/emuella-j2k-codestream/fuzz/Cargo.toml"
        not in ci_workflow
    ):
        errors.append("hosted CI does not audit the fuzz dependency graph")
    if "--config crates/emuella-j2k-codestream/fuzz/deny.toml" not in ci_workflow:
        errors.append("hosted CI does not apply the fuzz-specific dependency policy")
    if (
        "cargo check --manifest-path crates/emuella-j2k-codestream/fuzz/Cargo.toml"
        not in ci_workflow
    ):
        errors.append("hosted CI does not compile the locked fuzz workspace")
    if (
        "cargo clippy --manifest-path crates/emuella-j2k-codestream/fuzz/Cargo.toml"
        not in ci_workflow
    ):
        errors.append("hosted CI does not lint the locked fuzz workspace")

    release_workflow = (ROOT / ".github/workflows/release-dry-run.yml").read_text(
        encoding="utf-8"
    )
    if not release_workflow.startswith("name: emuella-j2k release dry run\n"):
        errors.append("release workflow display name is not branded")
    if "python3 scripts/test-public-tree-policy.py" not in release_workflow:
        errors.append("release workflow does not test the public-file policy")
    if "python3 scripts/test-package-legal-policy.py" not in release_workflow:
        errors.append("release workflow does not test canonical package legal files")
    if "cargo package --workspace --locked" not in release_workflow:
        errors.append("release workflow does not perform locked workspace packaging")
    if "python3 scripts/check-package-contents.py" not in release_workflow:
        errors.append("release workflow does not inspect packaged crate contents")
    if 'python3 -m pip install --disable-pip-version-check "maturin==1.14.1"' not in (
        release_workflow
    ):
        errors.append("release workflow does not install the reviewed Maturin version")
    if "sh scripts/check-python-distributions.sh" not in release_workflow:
        errors.append("release workflow does not verify Python distributions")
    if "sh scripts/package-cli-binary.sh" not in release_workflow:
        errors.append("release workflow does not verify the prebuilt CLI distribution")
    if "python3 scripts/generate-binary-dependency-notices.py --check" not in (
        release_workflow
    ):
        errors.append("release workflow does not verify binary dependency notices")
    if "uses: actions/upload-artifact@v7" not in release_workflow:
        errors.append("release workflow does not preserve release evidence")
    if "--allow-dirty" in release_workflow or "--no-verify" in release_workflow:
        errors.append("release workflow weakens Cargo package verification")

    local_check = (ROOT / "scripts/check.sh").read_text(encoding="utf-8")
    if "python3 scripts/test-public-tree-policy.py" not in local_check:
        errors.append("local checks do not test the fail-closed public-file policy")
    if "python3 scripts/test-package-legal-policy.py" not in local_check:
        errors.append("local checks do not test canonical package legal files")
    if (
        "--manifest-path crates/emuella-j2k-codestream/fuzz/Cargo.toml"
        not in local_check
    ):
        errors.append("local checks do not audit the fuzz dependency graph")
    if "--config crates/emuella-j2k-codestream/fuzz/deny.toml" not in local_check:
        errors.append("local checks do not apply the fuzz-specific dependency policy")
    if "python3 scripts/generate-binary-dependency-notices.py --check" not in (
        local_check
    ):
        errors.append("local checks do not verify binary dependency notices")

    for archive_auditor in (
        "scripts/check-package-contents.py",
        "scripts/check-python-distributions.py",
        "scripts/check-cli-distribution.py",
    ):
        auditor_text = (ROOT / archive_auditor).read_text(encoding="utf-8")
        if "content_policy_errors" not in auditor_text:
            errors.append(
                f"completed-archive auditor does not apply public-file policy: "
                f"{archive_auditor}"
            )

    if (ROOT / ".git").exists() and not HOSTED_CI:
        errors.append("staging tree contains .git")
    if errors:
        print("public-tree audit failed:", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print(f"public-tree audit passed: {len(paths)} files")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
