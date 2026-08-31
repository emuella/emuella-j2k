#!/usr/bin/env python3
"""Run canonical checks in an owned export of the exact clean Git tree."""

from __future__ import annotations

import hashlib
import os
from pathlib import Path, PurePosixPath
import stat
import subprocess
import sys
import tempfile
from typing import NamedTuple

sys.dont_write_bytecode = True


class CheckError(Exception):
    pass


class Entry(NamedTuple):
    mode: str
    oid: str


def environment() -> dict[str, str]:
    # Do not let a caller's index, worktree, object directory or config redirect
    # Git to a different repository. Never impersonate hosted CI.
    return {
        key: value for key, value in os.environ.items() if not key.startswith("GIT_")
    }


def git(root: Path, *arguments: str) -> bytes:
    result = subprocess.run(
        [
            "git",
            "--no-replace-objects",
            "--no-optional-locks",
            "-c",
            "core.fsmonitor=false",
            "-c",
            "core.untrackedCache=false",
            "-C",
            str(root),
            *arguments,
        ],
        env=environment(),
        capture_output=True,
        check=False,
    )
    if result.returncode:
        raise CheckError(f"Git {arguments[0]} failed (exit {result.returncode})")
    return result.stdout


def identity(root: Path) -> tuple[str, str]:
    commit = git(root, "rev-parse", "--verify", "HEAD^{commit}").decode().strip()
    tree = git(root, "rev-parse", "--verify", f"{commit}^{{tree}}").decode().strip()
    return commit, tree


def tree_entries(root: Path, tree: str) -> dict[str, Entry]:
    entries = {}
    for record in git(root, "ls-tree", "-r", "-t", "--full-tree", "-z", tree).split(
        b"\0"
    ):
        if not record:
            continue
        metadata, raw_path = record.split(b"\t", 1)
        mode, kind, oid = metadata.decode("ascii").split()
        path = raw_path.decode("utf-8")
        parts = path.split("/")
        if (
            any(part in ("", ".", "..") or part.lower() == ".git" for part in parts)
            or "\\" in path
            or PurePosixPath(path).is_absolute()
            or path in entries
        ):
            raise CheckError(f"unsafe or duplicate Git path: {path!r}")
        if (mode, kind) not in {
            ("040000", "tree"),
            ("100644", "blob"),
            ("100755", "blob"),
        }:
            raise CheckError(f"unsupported Git mode/type for {path!r}: {mode} {kind}")
        parent = str(PurePosixPath(path).parent)
        if parent != "." and entries.get(parent, Entry("", "")).mode != "040000":
            raise CheckError(f"missing Git parent directory: {path!r}")
        entries[path] = Entry(mode, oid)
    if (
        "scripts/check.sh" not in entries
        or entries["scripts/check.sh"].mode == "040000"
    ):
        raise CheckError("committed tree has no scripts/check.sh")
    return entries


def blob_identity(content: bytes, algorithm: str) -> str:
    return hashlib.new(
        algorithm, b"blob " + str(len(content)).encode("ascii") + b"\0" + content
    ).hexdigest()


def check_files(
    root: Path, entries: dict[str, Entry], algorithm: str, *, exact: bool
) -> None:
    for name, entry in entries.items():
        path = root / name
        mode = path.lstat().st_mode
        if entry.mode == "040000":
            if not stat.S_ISDIR(mode):
                raise CheckError(f"source directory differs from Git tree: {name!r}")
        elif (
            not stat.S_ISREG(mode)
            or bool(mode & 0o111) != (entry.mode == "100755")
            or blob_identity(path.read_bytes(), algorithm) != entry.oid
        ):
            raise CheckError(f"source file differs from Git tree: {name!r}")
    if exact:
        actual = set()
        for directory, names, files in os.walk(root, followlinks=False):
            for name in names + files:
                actual.add((Path(directory) / name).relative_to(root).as_posix())
        if actual != entries.keys():
            raise CheckError("export paths differ from complete Git tree")


def check_checkout(
    root: Path, expected: tuple[str, str], entries: dict[str, Entry], algorithm: str
) -> None:
    if identity(root) != expected:
        raise CheckError("checkout HEAD changed during verification")
    if git(
        root,
        "status",
        "--porcelain=v1",
        "--untracked-files=all",
        "--ignore-submodules=none",
    ):
        raise CheckError(
            "checkout has staged, unstaged or non-ignored untracked changes; commit first"
        )
    # Git's stat cache, assume-unchanged and skip-worktree bits are not proof
    # that the working bytes equal the committed source.
    check_files(root, entries, algorithm, exact=False)
    if identity(root) != expected:
        raise CheckError("checkout HEAD changed during verification")


def export_tree(
    root: Path, destination: Path, entries: dict[str, Entry], algorithm: str
) -> None:
    # Read objects directly: export-ignore/export-subst and checkout filters
    # must not omit or transform committed source as git archive can.
    for name, entry in entries.items():
        path = destination / name
        if entry.mode == "040000":
            path.mkdir()
        else:
            content = git(root, "cat-file", "blob", entry.oid)
            if blob_identity(content, algorithm) != entry.oid:
                raise CheckError(f"Git blob identity mismatch: {name!r}")
            with path.open("xb") as output:
                output.write(content)
            path.chmod(0o755 if entry.mode == "100755" else 0o644)
    check_files(destination, entries, algorithm, exact=True)


def verify(root: Path, temporary_parent: Path) -> None:
    root = root.resolve()
    if not (root / ".git").exists():
        raise CheckError("expected a primary or linked Git checkout")
    if (
        Path(os.fsdecode(git(root, "rev-parse", "--show-toplevel")).strip()).resolve()
        != root
    ):
        raise CheckError("verification must start at the actual Git checkout root")
    temporary_parent = temporary_parent.resolve(strict=True)
    if temporary_parent == root or root in temporary_parent.parents:
        raise CheckError("temporary parent must be outside the checkout")
    expected = identity(root)
    algorithm = git(root, "rev-parse", "--show-object-format").decode().strip()
    if algorithm not in {"sha1", "sha256"}:
        raise CheckError(f"unsupported Git object format: {algorithm}")
    entries = tree_entries(root, expected[1])
    check_checkout(root, expected, entries, algorithm)
    commit, tree = expected
    print(f"Checking commit {commit}\nChecking tree   {tree}", flush=True)
    # Only this newly allocated child is removed, never the supplied parent.
    with tempfile.TemporaryDirectory(
        prefix="emuella-check-", dir=temporary_parent
    ) as owned:
        scratch = Path(owned)
        source, build, temporary = (
            scratch / name for name in ("source", "build", "tmp")
        )
        for directory in (source, build, temporary):
            directory.mkdir()
        export_tree(root, source, entries, algorithm)
        check_checkout(root, expected, entries, algorithm)
        check_files(source, entries, algorithm, exact=True)
        env = environment()
        # Resolve relative cache homes before changing to the export. Otherwise
        # a caller's CARGO_HOME=.local/cargo would create untracked source there.
        for name in ("CARGO_HOME", "RUSTUP_HOME", "XDG_CACHE_HOME"):
            if name in env:
                env[name] = str((root / Path(env[name]).expanduser()).resolve())
        env.update(
            {
                "CARGO_TARGET_DIR": str(build),
                "CARGO_BUILD_BUILD_DIR": str(build),
                "TMPDIR": str(temporary),
                "TMP": str(temporary),
                "TEMP": str(temporary),
                "PYTHONDONTWRITEBYTECODE": "1",
            }
        )
        print(f"Disposable source: {source}\nDisposable build:  {build}", flush=True)
        result = subprocess.run(
            ["sh", "scripts/check.sh"], cwd=source, env=env, check=False
        )
        if result.returncode:
            raise CheckError(
                f"canonical checks failed (exit {result.returncode}) for commit {commit}, tree {tree}"
            )
        check_files(source, entries, algorithm, exact=True)
        check_checkout(root, expected, entries, algorithm)
    print(f"Verification passed: commit {commit}, tree {tree}", flush=True)


def main() -> int:
    try:
        verify(
            Path(__file__).resolve().parent.parent,
            Path(os.environ.get("EMUELLA_CHECK_TMPDIR", tempfile.gettempdir())),
        )
    except (CheckError, OSError, ValueError) as error:
        print(f"Verification failed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
