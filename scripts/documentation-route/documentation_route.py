"""Conservative Git scope observations; callers own verification and event semantics."""

from __future__ import annotations

import argparse
from dataclasses import asdict, dataclass
import hashlib
import json
import os
from pathlib import Path
import re
import stat
import subprocess

SCHEMA_VERSION = 1
OBJECT_ID = re.compile(r"(?:[0-9a-f]{40}|[0-9a-f]{64})\Z")
PROSE_SUFFIXES = {".md", ".rst", ".txt", ".adoc"}


class RouteError(ValueError):
    """Invalid candidate identity, current configuration or helper integrity."""


class ObservationError(Exception):
    """Git state cannot be observed safely; full verification is required."""


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def safe_path(value: object) -> str:
    if (
        not isinstance(value, str)
        or not value
        or "\0" in value
        or "\\" in value
        or any(part in {"", ".", "..", ".git"} for part in value.split("/"))
    ):
        raise RouteError("expected an exact safe relative path")
    try:
        value.encode("utf-8")
    except UnicodeError as error:
        raise RouteError("path must use UTF-8") from error
    return value


def parse_policy(data: bytes) -> dict:
    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise RouteError("duplicate policy field")
            result[key] = value
        return result

    try:
        policy = json.loads(data, object_pairs_hook=unique)
    except (ValueError, UnicodeError) as error:
        raise RouteError("malformed policy JSON") from error
    if (
        not isinstance(policy, dict)
        or set(policy) != {"schema_version", "eligible_paths"}
        or type(policy["schema_version"]) is not int
        or policy["schema_version"] != SCHEMA_VERSION
        or not isinstance(policy["eligible_paths"], list)
    ):
        raise RouteError("unsupported policy schema")
    paths = [safe_path(path) for path in policy["eligible_paths"]]
    if len(set(paths)) != len(paths):
        raise RouteError("duplicate eligible path")
    if any(Path(path).suffix.lower() not in PROSE_SUFFIXES for path in paths):
        raise RouteError("eligible paths must be exact prose filenames")
    return policy


class Git:
    def __init__(self, root: Path):
        self.root = root
        self.env = {
            key: value
            for key, value in os.environ.items()
            if not key.startswith("GIT_")
        }
        self.env.update(
            GIT_NO_REPLACE_OBJECTS="1", GIT_OPTIONAL_LOCKS="0", GIT_NO_LAZY_FETCH="1"
        )

    def run(self, *args: str) -> bytes:
        try:
            result = subprocess.run(
                [
                    "git",
                    "--no-lazy-fetch",
                    "--no-optional-locks",
                    "-C",
                    str(self.root),
                    "-c",
                    "core.fsmonitor=false",
                    "-c",
                    "core.untrackedCache=false",
                    "-c",
                    "core.fileMode=true",
                    *args,
                ],
                env=self.env,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                timeout=20,
                check=False,
            )
        except (OSError, subprocess.TimeoutExpired) as error:
            raise ObservationError("Git observation unavailable") from error
        if result.returncode:
            raise ObservationError(f"Git observation failed: {args[0]}")
        return result.stdout

    def identity(self, value: str, kind: str) -> str:
        result = self.run(
            "rev-parse", "--verify", "--end-of-options", value + "^{" + kind + "}"
        )
        resolved = result.decode("ascii").strip()
        if not OBJECT_ID.fullmatch(resolved):
            raise ObservationError("invalid resolved Git identity")
        return resolved

    def blob(self, commit: str, path: str) -> bytes:
        entry = self.run("ls-tree", "-z", commit, "--", ":(literal)" + path)
        fields = entry.split(b"\0")
        if len(fields) != 2 or fields[-1] or b"\t" not in fields[0]:
            raise ObservationError("policy blob unavailable")
        header, raw_path = fields[0].split(b"\t", 1)
        parts = header.split()
        if (
            len(parts) != 3
            or parts[:2] != [b"100644", b"blob"]
            or raw_path != path.encode()
        ):
            raise ObservationError("policy is not a regular blob")
        return self.run("cat-file", "blob", parts[2].decode("ascii"))


@dataclass(frozen=True)
class Verdict:
    schema_version: int
    route: str
    reasons: list[str]
    candidate_commit: str
    candidate_tree: str
    base_commit: str | None
    base_tree: str | None
    helper_version: str
    helper_sha256: str
    policy: dict
    changes: list[dict]

    def to_dict(self) -> dict:
        return asdict(self)


def nul_fields(data: bytes) -> list[bytes]:
    if not data:
        return []
    parts = data.split(b"\0")
    if parts.pop() != b"":
        raise ObservationError("incomplete NUL observation")
    return parts


def raw_changes(data: bytes, layer: str) -> list[dict]:
    fields = nul_fields(data)
    if len(fields) % 2:
        raise ObservationError("incomplete raw diff")
    changes = []
    for header, raw_path in zip(fields[::2], fields[1::2]):
        parts = header.decode("ascii").split()
        if (
            len(parts) != 5
            or not re.fullmatch(r":[0-7]{6}", parts[0])
            or not re.fullmatch(r"[0-7]{6}", parts[1])
            or any(not re.fullmatch(r"[0-9a-f]+", value) for value in parts[2:4])
            or parts[4] not in {"A", "M", "D", "T", "U"}
        ):
            raise ObservationError("unsupported raw diff record")
        path = observed_path(raw_path)
        changes.append(
            dict(
                layer=layer,
                status=parts[4],
                old_path=None if parts[0] == ":000000" else path,
                new_path=None if parts[1] == "000000" else path,
                old_mode=parts[0][1:],
                new_mode=parts[1],
            )
        )
    return changes


def observed_path(raw: bytes) -> str:
    try:
        return safe_path(raw.decode("utf-8"))
    except RouteError as error:
        raise ObservationError("unsupported observed filename") from error


def classify(root, *, candidate_commit, base_commit=None, policy_path=None) -> Verdict:
    """Bind actual HEAD, observe each layer, and choose docs only with unchanged policy.

    Only full object IDs are accepted. RouteError is fatal; observation uncertainty
    is a successful full verdict. The result does not authorise reduced checks.
    """
    if not isinstance(candidate_commit, str) or not OBJECT_ID.fullmatch(
        candidate_commit
    ):
        raise RouteError("candidate must be a full hexadecimal commit identity")
    root = Path(root).resolve()
    git = Git(root)
    try:
        if (
            Path(
                os.fsdecode(git.run("rev-parse", "--show-toplevel")).rstrip("\n")
            ).resolve()
            != root
        ):
            raise RouteError("root must be the actual repository root")
        head = git.identity("HEAD", "commit")
        if (
            head != candidate_commit
            or git.identity(candidate_commit, "commit") != candidate_commit
        ):
            raise RouteError("candidate must equal actual HEAD")
        tree = git.identity(candidate_commit, "tree")
    except (ObservationError, UnicodeError) as error:
        raise RouteError("candidate identity cannot be established") from error

    helper = Path(__file__).resolve()
    try:
        version = (helper.parent / "VERSION").read_text(encoding="ascii").strip()
        if not re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", version):
            raise RouteError("invalid helper version")
        helper_digest = sha256(helper.read_bytes())
    except (OSError, UnicodeError) as error:
        raise RouteError("helper integrity unavailable") from error
    if helper.is_relative_to(root):
        try:
            for file in (helper, helper.parent / "VERSION"):
                if (
                    git.blob(candidate_commit, file.relative_to(root).as_posix())
                    != file.read_bytes()
                ):
                    raise RouteError("helper bytes differ from candidate")
        except (ObservationError, OSError) as error:
            raise RouteError("candidate helper integrity unavailable") from error
    reasons, changes = [], []
    policy_data = None
    policy = {"schema_version": SCHEMA_VERSION, "eligible_paths": []}
    if policy_path is not None:
        policy_path = safe_path(policy_path)
        try:
            policy_data = git.blob(candidate_commit, policy_path)
        except ObservationError as error:
            raise RouteError("candidate policy blob unavailable") from error
        policy = parse_policy(policy_data)
        current = root / policy_path
        try:
            if current.is_symlink() or not stat.S_ISREG(current.lstat().st_mode):
                reasons.append("policy working path is not a regular file")
            else:
                current_data = current.read_bytes()
                parse_policy(current_data)
                if current_data != policy_data:
                    reasons.append("policy working bytes differ from candidate")
        except FileNotFoundError:
            reasons.append("policy working file unavailable")
        except OSError as error:
            raise RouteError("current policy cannot be read") from error

    base, base_tree, base_digest = None, None, None
    if not isinstance(base_commit, str) or not OBJECT_ID.fullmatch(base_commit):
        reasons.append("comparison requires a full local base commit identity")
    else:
        try:
            base = git.identity(base_commit, "commit")
            if base != base_commit:
                raise ObservationError("comparison base is not a commit")
            base_tree = git.identity(base, "tree")
            if git.run("rev-parse", "--is-shallow-repository").strip() != b"false":
                raise ObservationError("shallow history")
            git.run("merge-base", "--is-ancestor", base, candidate_commit)
            if policy_data is not None:
                base_data = git.blob(base, policy_path)
                base_digest = sha256(base_data)
                if base_data != policy_data:
                    reasons.append("policy differs from comparison base")
        except ObservationError as error:
            reasons.append(f"comparison unavailable: {error}")

    try:
        if policy_path is not None:
            index_entries = nul_fields(
                git.run("ls-files", "--stage", "-z", "--", ":(literal)" + policy_path)
            )
            if len(index_entries) != 1:
                reasons.append("policy index entry unavailable")
            else:
                index_header, index_path = index_entries[0].split(b"\t", 1)
                mode, blob, stage = index_header.split()
                if (
                    mode != b"100644"
                    or stage != b"0"
                    or index_path != policy_path.encode()
                ):
                    reasons.append("policy index entry unsupported")
                else:
                    index_data = git.run("cat-file", "blob", blob.decode("ascii"))
                    parse_policy(index_data)
                    if index_data != policy_data:
                        reasons.append("policy index bytes differ from candidate")
    except RouteError:
        raise
    except (ObservationError, OSError, UnicodeError, ValueError) as error:
        reasons.append(f"policy index unavailable: {error}")

    try:
        flags = nul_fields(git.run("ls-files", "-v", "-z"))
        if any(not item.startswith(b"H ") for item in flags):
            reasons.append("hidden index flags or unmerged entries")
        if git.run("ls-files", "--unmerged", "-z"):
            reasons.append("unmerged index")
        # --get may return 1 for absent config, so inspect the complete local view.
        config = git.run("config", "--null", "--list")
        if any(
            item.startswith((b"core.sparsecheckout\n", b"core.sparsecheckoutcone\n"))
            and item.split(b"\n", 1)[-1] not in {b"false", b"0", b"no", b"off"}
            for item in nul_fields(config)
        ):
            reasons.append("sparse checkout configuration")
        if base is not None and base_tree is not None:
            changes.extend(
                raw_changes(
                    git.run(
                        "diff",
                        "--raw",
                        "-z",
                        "--no-renames",
                        "--no-ext-diff",
                        "--no-textconv",
                        "--ignore-submodules=none",
                        base,
                        candidate_commit,
                        "--",
                    ),
                    "committed",
                )
            )
        for layer, args in (
            ("staged", ("--cached", candidate_commit)),
            ("unstaged", ()),
        ):
            changes.extend(
                raw_changes(
                    git.run(
                        "diff",
                        "--raw",
                        "-z",
                        "--no-renames",
                        "--no-ext-diff",
                        "--no-textconv",
                        "--ignore-submodules=none",
                        *args,
                        "--",
                    ),
                    layer,
                )
            )
        for raw_path in nul_fields(
            git.run("ls-files", "--others", "--exclude-standard", "-z")
        ):
            path = observed_path(raw_path)
            mode = (root / path).lstat().st_mode
            mode_string = (
                "100644" if stat.S_ISREG(mode) and not mode & 0o111 else "unsupported"
            )
            changes.append(
                dict(
                    layer="untracked",
                    status="A",
                    old_path=None,
                    new_path=path,
                    old_mode="000000",
                    new_mode=mode_string,
                )
            )
    except RouteError:
        raise
    except (ObservationError, OSError, UnicodeError, ValueError) as error:
        reasons.append(f"scope unavailable: {error}")

    protected_prefix = None
    if helper.is_relative_to(root):
        relative_helper = helper.parent.relative_to(root).as_posix()
        protected_prefix = "" if relative_helper == "." else relative_helper + "/"
    for change in changes:
        paths = [
            path
            for path in (change["old_path"], change["new_path"])
            if path is not None
        ]
        if change["status"] not in {"A", "M", "D"} or any(
            change[key] not in {"000000", "100644"} for key in ("old_mode", "new_mode")
        ):
            reasons.append(f"unsupported modes/state in {change['layer']}: {paths!r}")
        for path in paths:
            if (
                path not in policy["eligible_paths"]
                or path == policy_path
                or path.lower().endswith(".rs")
                or path.startswith(".github/workflows/")
                or (protected_prefix is not None and path.startswith(protected_prefix))
            ):
                reasons.append(f"ineligible {change['layer']} path: {path!r}")
    if not policy["eligible_paths"]:
        reasons.append("empty eligibility policy")
    try:
        if git.identity("HEAD", "commit") != candidate_commit:
            raise RouteError("candidate changed during observation")
    except ObservationError as error:
        raise RouteError("candidate recheck unavailable") from error
    reasons = list(dict.fromkeys(reasons))
    return Verdict(
        SCHEMA_VERSION,
        "full" if reasons else "docs",
        reasons or ["all observed scope is eligible prose"],
        candidate_commit,
        tree,
        base,
        base_tree,
        version,
        helper_digest,
        dict(
            schema_version=SCHEMA_VERSION,
            path=policy_path,
            sha256=sha256(policy_data) if policy_data is not None else None,
            base_sha256=base_digest,
            eligible_paths=policy["eligible_paths"],
        ),
        changes,
    )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--candidate", required=True)
    parser.add_argument("--base")
    parser.add_argument("--policy")
    args = parser.parse_args()
    try:
        result = classify(
            args.root,
            candidate_commit=args.candidate,
            base_commit=args.base,
            policy_path=args.policy,
        )
    except RouteError as error:
        print(json.dumps({"error": str(error)}))
        return 2
    print(json.dumps(result.to_dict(), ensure_ascii=True, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
