"""Explicit local, reviewed-revision adoption of this directory's allowlisted files."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re
import stat
import tempfile

from documentation_route import Git, OBJECT_ID, ObservationError, RouteError, sha256

PAYLOAD_ROOT = "public-safe/documentation-route"
FILES = (
    "documentation_route.py",
    "adopt.py",
    "VERSION",
    "LICENSE",
    "README.md",
    "test_documentation_route.py",
    "test_adopt.py",
)
MANIFEST = ".documentation-route.json"


def reject_symlinks(path: Path) -> None:
    for ancestor in (path, *path.parents):
        if ancestor.is_symlink():
            raise RouteError(f"symlinked destination: {ancestor}")


def current_bytes(path: Path, *, managed: bool = True) -> bytes | None:
    reject_symlinks(path)
    if not path.exists():
        return None
    if not path.is_file():
        raise RouteError(f"non-regular managed destination: {path}")
    if managed and stat.S_IMODE(path.stat().st_mode) != 0o644:
        raise RouteError(f"managed destination mode drift: {path}")
    return path.read_bytes()


def strict_json(data: bytes) -> dict:
    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise RouteError("duplicate manifest/receipt field")
            result[key] = value
        return result

    try:
        result = json.loads(data, object_pairs_hook=unique)
    except (UnicodeError, ValueError) as error:
        raise RouteError("malformed manifest/receipt") from error
    if not isinstance(result, dict):
        raise RouteError("manifest/receipt must be an object")
    return result


def load_manifest(data: bytes | None) -> dict:
    if data is None:
        return {}
    result = strict_json(data)
    if (
        set(result) != {"schema_version", "source_commit", "version", "files"}
        or type(result["schema_version"]) is not int
        or result["schema_version"] != 1
        or not isinstance(result["source_commit"], str)
        or not OBJECT_ID.fullmatch(result["source_commit"])
        or not isinstance(result["version"], str)
        or not re.fullmatch(r"[0-9]+\.[0-9]+\.[0-9]+", result["version"])
        or not isinstance(result["files"], dict)
        or set(result["files"]) != set(FILES)
        or any(
            not isinstance(value, str) or not re.fullmatch(r"[0-9a-f]{64}", value)
            for value in result["files"].values()
        )
    ):
        raise RouteError("invalid managed manifest")
    return result


def prepare(
    source: Path, revision: str, version: str, target: Path
) -> tuple[dict, dict]:
    if not isinstance(revision, str) or not OBJECT_ID.fullmatch(revision):
        raise RouteError("source revision must be a full reviewed commit identity")
    if not isinstance(version, str) or not re.fullmatch(
        r"[0-9]+\.[0-9]+\.[0-9]+", version
    ):
        raise RouteError("expected an explicit payload version")
    source = source.resolve()
    target = target.absolute()
    if ".." in target.parts:
        raise RouteError("unsafe destination path")
    reject_symlinks(target)
    if target.exists() and not target.is_dir():
        raise RouteError("destination must be a directory")
    git = Git(source)
    try:
        if (
            Path(
                os.fsdecode(git.run("rev-parse", "--show-toplevel")).rstrip("\n")
            ).resolve()
            != source
        ):
            raise RouteError("source must be a repository root")
        if git.identity(revision, "commit") != revision:
            raise RouteError("source identity is not a commit")
        payload = {
            name: git.blob(revision, PAYLOAD_ROOT + "/" + name) for name in FILES
        }
    except ObservationError as error:
        raise RouteError("reviewed source payload unavailable locally") from error
    if payload["VERSION"] != (version + "\n").encode("ascii"):
        raise RouteError("reviewed source payload version mismatch")
    manifest_bytes = current_bytes(target / MANIFEST)
    old = load_manifest(manifest_bytes)
    if old:
        try:
            prior_payload = {
                name: git.blob(old["source_commit"], PAYLOAD_ROOT + "/" + name)
                for name in FILES
            }
        except ObservationError as error:
            raise RouteError("prior manifest source unavailable locally") from error
        expected_old = dict(
            schema_version=1,
            source_commit=old["source_commit"],
            version=prior_payload["VERSION"].decode("ascii").strip(),
            files={name: sha256(data) for name, data in prior_payload.items()},
        )
        if (
            old != expected_old
            or manifest_bytes
            != (json.dumps(old, indent=2, sort_keys=True) + "\n").encode()
        ):
            raise RouteError("managed manifest differs from immutable source record")
    hashes = {name: sha256(data) for name, data in payload.items()}
    plan = dict(
        schema_version=1,
        source_commit=revision,
        version=version,
        target=str(target),
        files=hashes,
        prior_manifest_sha256=sha256(manifest_bytes)
        if manifest_bytes is not None
        else None,
        observed={},
        actions={},
    )
    for name in FILES:
        current = current_bytes(target / name)
        digest = sha256(current) if current is not None else None
        plan["observed"][name] = digest
        if digest is None:
            if name in old.get("files", {}):
                raise RouteError(f"local managed deletion: {name}")
            plan["actions"][name] = "create"
        elif digest == hashes[name]:
            plan["actions"][name] = "unchanged"
        elif digest == old.get("files", {}).get(name):
            plan["actions"][name] = "update"
        else:
            raise RouteError(f"local drift or unknown conflict: {name}")
    return plan, payload


def atomic_write(path: Path, data: bytes) -> None:
    descriptor, temporary = tempfile.mkstemp(
        prefix="." + path.name + ".", dir=path.parent
    )
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        os.chmod(temporary, 0o644)
        os.replace(temporary, path)
    finally:
        Path(temporary).unlink(missing_ok=True)


def adopt(
    source: Path, revision: str, version: str, target: Path, *, receipt=None
) -> dict:
    """Without a receipt, return a write-free plan. Apply only an identical plan."""
    plan, payload = prepare(source, revision, version, target)
    if receipt is None:
        return plan
    if receipt != plan:
        raise RouteError(
            "dry-run receipt differs; inspect a new dry-run before adoption"
        )
    destination = Path(plan["target"])
    destination.mkdir(parents=True, exist_ok=True)
    # All conflicts and every managed path are checked before the first write.
    for name in FILES:
        if plan["actions"][name] != "unchanged":
            atomic_write(destination / name, payload[name])
    manifest = {
        key: plan[key]
        for key in ("schema_version", "source_commit", "version", "files")
    }
    atomic_write(
        destination / MANIFEST,
        (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode(),
    )
    return plan


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", type=Path, required=True)
    parser.add_argument(
        "--revision", required=True, help="full locally available reviewed commit"
    )
    parser.add_argument("--version", required=True)
    parser.add_argument("--target", type=Path, required=True)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--dry-run", action="store_true")
    mode.add_argument("--apply", type=Path, metavar="RECEIPT_JSON")
    args = parser.parse_args()
    try:
        receipt = (
            strict_json(current_bytes(args.apply, managed=False))
            if args.apply
            else None
        )
        plan = adopt(
            args.source, args.revision, args.version, args.target, receipt=receipt
        )
    except (RouteError, OSError) as error:
        parser.exit(2, f"adoption failed: {error}\n")
    print(json.dumps(plan, indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
