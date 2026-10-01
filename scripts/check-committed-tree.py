#!/usr/bin/env python3
"""Run canonical checks in an owned export of the exact clean Git tree."""

from __future__ import annotations

import hashlib
import argparse
import json
import os
from pathlib import Path, PurePosixPath
import stat
import shutil
import subprocess
import sys
import tempfile
from typing import NamedTuple

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent.parent


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
            "--no-lazy-fetch",
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


def route(root: Path, candidate: str, base: str | None):
    sys.path.insert(0, str(ROOT / "scripts/documentation-route"))
    from documentation_route import RouteError, classify
    policy = root / "scripts/documentation-policy.json"
    try:
        return classify(root, candidate_commit=candidate, base_commit=base,
                        policy_path="scripts/documentation-policy.json" if policy.exists() else None).to_dict()
    except RouteError as error:
        raise CheckError(str(error)) from error


def verify(root: Path, temporary_parent: Path, base: str | None = None) -> None:
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
    verdict = route(root, expected[0], base)
    print("Verification route: " + json.dumps(verdict, sort_keys=True), flush=True)
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
                "EMUELLA_FUZZ_TARGET_DIR": str(build / "fuzz"),
            }
        )
        print(f"Disposable source: {source}\nDisposable build:  {build}", flush=True)
        result = subprocess.run(
            ["sh", "scripts/check-documentation.sh" if verdict["route"] == "docs" else "scripts/check.sh"],
            cwd=source, env=env, check=False
        )
        if result.returncode:
            raise CheckError(
                f"canonical checks failed (exit {result.returncode}) for commit {commit}, tree {tree}"
            )
        check_files(source, entries, algorithm, exact=True)
        check_checkout(root, expected, entries, algorithm)
    print(f"Verification passed: commit {commit}, tree {tree}", flush=True)


def event_base(root: Path, event_name: str, event: dict, github_sha: str | None = None) -> str | None:
    """Bind recognised Actions topology; unsupported/missing history stays full."""
    candidate = identity(root)[0]
    if github_sha and github_sha != candidate:
        raise CheckError("event checkout identity differs from GITHUB_SHA")
    if event_name == "pull_request":
        pr = event.get("pull_request")
        if not isinstance(pr, dict):
            return None
        head = pr.get("head", {}).get("sha") if isinstance(pr.get("head"), dict) else None
        base = pr.get("base", {}).get("sha") if isinstance(pr.get("base"), dict) else None
        if not all(isinstance(value, str) and len(value) in {40, 64}
                   and all(c in "0123456789abcdef" for c in value) for value in (head, base)):
            return None
        if candidate != head:
            parents = git(root, "show", "-s", "--format=%P", candidate).decode().split()
            if parents != [base, head]:
                raise CheckError("PR checkout is neither event head nor its recognised synthetic merge")
        return base
    if event_name == "push":
        after, before = event.get("after"), event.get("before")
        if not isinstance(after, str) or after != candidate:
            raise CheckError("push event after differs from actual checkout")
        if not isinstance(before, str) or not before or set(before) == {"0"}:
            return None
        return before
    return None


def ci_location(root: Path) -> Path:
    workspace = Path(os.environ.get("GITHUB_WORKSPACE", "")).absolute()
    if root.name != "checkout" or workspace != root.parent:
        raise CheckError("hosted verification requires the fixed checkout/sibling layout")
    for path in (workspace, *workspace.parents):
        if path.is_symlink():
            raise CheckError("hosted scratch parent contains a symbolic link")
    return workspace / "ci-verification"


def ci_binding(root: Path) -> tuple[dict, dict[str, Entry], str]:
    expected = identity(root)
    algorithm = git(root, "rev-parse", "--show-object-format").decode().strip()
    if algorithm not in {"sha1", "sha256"}:
        raise CheckError("unsupported Git object format")
    entries = tree_entries(root, expected[1])
    check_checkout(root, expected, entries, algorithm)
    event_path = Path(os.environ["GITHUB_EVENT_PATH"])
    event_bytes = event_path.read_bytes()
    event = json.loads(event_bytes)
    if not isinstance(event, dict):
        raise CheckError("Actions event must be an object")
    event_name = os.environ.get("GITHUB_EVENT_NAME", "")
    if event_name == "push" and event.get("ref") != os.environ.get("GITHUB_REF"):
        raise CheckError("push event ref differs from Actions ref")
    base = event_base(root, event_name, event, os.environ.get("GITHUB_SHA"))
    return {"candidate": expected[0], "tree": expected[1], "algorithm": algorithm,
            "event_name": event_name, "event_sha256": hashlib.sha256(event_bytes).hexdigest(),
            "event_ref": event.get("ref"), "github_ref": os.environ.get("GITHUB_REF"),
            "verdict": route(root, expected[0], base)}, entries, algorithm


def ci_load(root: Path, expected_hash: str) -> tuple[Path, dict, dict[str, Entry], str]:
    scratch = ci_location(root)
    if scratch.is_symlink() or not scratch.is_dir():
        raise CheckError("owned hosted scratch is missing or a symbolic link")
    content = (scratch / "binding.json").read_bytes()
    if hashlib.sha256(content).hexdigest() != expected_hash:
        raise CheckError("prepared binding integrity differs from original step output")
    prepared = json.loads(content)
    if prepared.get("owner") != [scratch.stat().st_dev, scratch.stat().st_ino]:
        raise CheckError("hosted scratch ownership changed")
    current, entries, algorithm = ci_binding(root)
    if current != prepared.get("binding"):
        raise CheckError("candidate/tree/base/policy/helper/event changed after preparation")
    source = scratch / "source"
    if source.is_symlink() or not source.is_dir():
        raise CheckError("prepared source is missing or a symbolic link")
    check_files(source, entries, algorithm, exact=True)
    return scratch, current, entries, algorithm


def ci_environment(root: Path, scratch: Path) -> dict[str, str]:
    env = environment()
    for name in ("CARGO_HOME", "RUSTUP_HOME", "XDG_CACHE_HOME"):
        if name in env:
            env[name] = str((root / Path(env[name]).expanduser()).resolve())
    env.update({"CARGO_TARGET_DIR": str(root / "target"),
                "CARGO_BUILD_BUILD_DIR": str(root / "target"),
                "EMUELLA_FUZZ_TARGET_DIR": str(root / "crates/emuella-j2k-codestream/fuzz/target"),
                "TMPDIR": str(scratch / "tmp"), "TMP": str(scratch / "tmp"),
                "TEMP": str(scratch / "tmp"), "PYTHONDONTWRITEBYTECODE": "1"})
    return env


def ci_prepare(root: Path) -> None:
    scratch = ci_location(root)
    if scratch.exists() or scratch.is_symlink():
        raise CheckError("fixed hosted scratch already exists; no existing path is removed")
    binding, entries, algorithm = ci_binding(root)
    scratch.mkdir(mode=0o700)
    try:
        for name in ("source", "tmp"):
            (scratch / name).mkdir()
        export_tree(root, scratch / "source", entries, algorithm)
        check_checkout(root, (binding["candidate"], binding["tree"]), entries, algorithm)
        prepared = {"binding": binding, "owner": [scratch.stat().st_dev, scratch.stat().st_ino]}
        content = (json.dumps(prepared, sort_keys=True) + "\n").encode()
        (scratch / "binding.json").write_bytes(content)
        digest = hashlib.sha256(content).hexdigest()
        result = subprocess.run(["sh", "scripts/check-source.sh"], cwd=scratch / "source",
                                env=ci_environment(root, scratch), check=False)
        if result.returncode:
            raise CheckError(f"source audit failed before cache restore (exit {result.returncode})")
        ci_load(root, digest)
        with Path(os.environ["GITHUB_OUTPUT"]).open("a") as output:
            output.write(f"route={binding['verdict']['route']}\nbinding_sha256={digest}\n")
        print("Prepared verification route: " + json.dumps(binding, sort_keys=True), flush=True)
    except BaseException:
        shutil.rmtree(scratch)
        raise


def ci_run(root: Path, expected_hash: str) -> None:
    scratch, binding, entries, algorithm = ci_load(root, expected_hash)
    env = ci_environment(root, scratch)
    commands = [["python3", "scripts/documentation_checks.py", "--package-inventory"]]
    if binding["verdict"]["route"] == "full":
        commands.append(["sh", "scripts/check-full-runtime.sh"])
    for command in commands:
        result = subprocess.run(command, cwd=scratch / "source", env=env, check=False)
        if result.returncode:
            raise CheckError(f"selected {binding['verdict']['route']} group failed (exit {result.returncode})")
    ci_load(root, expected_hash)
    (scratch / "completed.json").write_text(json.dumps({"binding_sha256": expected_hash,
                                                       "route": binding["verdict"]["route"]}))
    print(f"Selected {binding['verdict']['route']} group completed", flush=True)


def check_selected_results(route_name: str, outcomes: dict) -> None:
    required = ["prepare", "selected"] + (["deny", "deny_fuzz"] if route_name == "full" else [])
    for name in required:
        if not isinstance(outcomes.get(name), dict) or outcomes[name].get("outcome") != "success":
            raise CheckError(f"required selected step is missing/skipped/failed: {name}")
    if route_name == "docs" and any(outcomes.get(name, {}).get("outcome") != "skipped" for name in ("deny", "deny_fuzz")):
        raise CheckError("documentation route has inconsistent dependency step outcomes")


def ci_finish(root: Path, expected_hash: str, outcomes: dict) -> None:
    scratch = ci_location(root)
    owned = False
    try:
        # Establish ownership before integrity validation so ordinary failed
        # commands/source changes still remove only the allocated export child.
        if not scratch.is_symlink() and scratch.is_dir():
            content = (scratch / "binding.json").read_bytes()
            if hashlib.sha256(content).hexdigest() == expected_hash:
                prepared = json.loads(content)
                owned = prepared.get("owner") == [scratch.stat().st_dev, scratch.stat().st_ino]
        scratch, binding, _, _ = ci_load(root, expected_hash)
        check_selected_results(binding["verdict"]["route"], outcomes)
        complete = json.loads((scratch / "completed.json").read_text())
        if complete != {"binding_sha256": expected_hash, "route": binding["verdict"]["route"]}:
            raise CheckError("selected group completion is absent or invalid")
        print(f"Hosted verification passed: {binding['verdict']['route']}; commit {binding['candidate']}, tree {binding['tree']}", flush=True)
    finally:
        if owned:
            shutil.rmtree(scratch)
            print("Owned export/temp removed; action/runner cache targets preserved.", flush=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--base", help="explicit local comparison commit; absent/unusable history selects full")
    parser.add_argument("--ci-prepare", action="store_true")
    parser.add_argument("--ci-run", action="store_true")
    parser.add_argument("--ci-finish", action="store_true")
    parser.add_argument("--binding-sha256")
    args = parser.parse_args()
    try:
        if sum((args.ci_prepare, args.ci_run, args.ci_finish)) > 1 or (args.base and any((args.ci_prepare, args.ci_run, args.ci_finish))):
            raise CheckError("choose exactly one verification entry context")
        if args.ci_prepare:
            ci_prepare(ROOT)
        elif args.ci_run or args.ci_finish:
            if not args.binding_sha256:
                raise CheckError("original prepared binding hash is required")
            if args.ci_run:
                ci_run(ROOT, args.binding_sha256)
            else:
                ci_finish(ROOT, args.binding_sha256, json.loads(os.environ.get("EMUELLA_STEP_OUTCOMES", "{}")))
        else:
            verify(ROOT, Path(os.environ.get("EMUELLA_CHECK_TMPDIR", tempfile.gettempdir())), args.base)
    except (CheckError, OSError, ValueError, KeyError) as error:
        print(f"Verification failed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
