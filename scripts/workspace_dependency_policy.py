"""Constrain cargo-deny's path wildcard exception to one unpublished fixture."""

from __future__ import annotations

from pathlib import Path
import tomllib

DEPENDENCY_TABLES = {"dependencies", "dev-dependencies", "build-dependencies"}
FIXTURE_MANIFEST = "crates/emuella-j2k-capi/Cargo.toml"
FIXTURE_TABLE = "dev-dependencies"
FIXTURE_NAME = "emuella-j2k-test-support"
FIXTURE_SPEC = {"path": "../emuella-j2k-test-support"}


def dependency_tables(node: dict, prefix: str = ""):
    for key, value in node.items():
        location = f"{prefix}.{key}" if prefix else key
        if key in DEPENDENCY_TABLES and isinstance(value, dict):
            yield location, value
        elif key in {"workspace", "target"} and isinstance(value, dict):
            # Target expressions are opaque table names, not Cargo expressions.
            children = value.items() if key == "target" else [(None, value)]
            for child, contents in children:
                child_prefix = f"{location}.{child}" if child else location
                yield from dependency_tables(contents, child_prefix)


def dependency_policy_errors(root: Path) -> list[str]:
    """Audit explicit manifests; Cargo remains responsible for registry versions."""
    root = root.resolve()
    cargo = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
    policy = tomllib.loads((root / "deny.toml").read_text(encoding="utf-8"))
    errors: list[str] = []
    if policy.get("bans", {}).get("wildcards") != "deny":
        errors.append("registry wildcard denial must remain enabled")
    if policy.get("bans", {}).get("allow-wildcard-paths") is not True:
        errors.append("the audited fixture path exception must be enabled")
    if any(policy.get("sources", {}).get(key) != "deny" for key in
           ("unknown-registry", "unknown-git")):
        errors.append("unknown registry and Git source denial must remain enabled")
    workspace = cargo["workspace"]
    manifests = {root / "Cargo.toml": cargo}
    members: dict[Path, tuple[str, str]] = {}
    for member in workspace["members"]:
        path = (root / member / "Cargo.toml").resolve()
        manifest = tomllib.loads(path.read_text(encoding="utf-8"))
        manifests[path] = manifest
        package = manifest["package"]
        version = package["version"]
        if version == {"workspace": True}:
            version = workspace["package"]["version"]
        members[path.parent] = (package["name"], version)
    support = root / "crates" / FIXTURE_NAME / "Cargo.toml"
    if support not in manifests or manifests[support]["package"].get("publish") is not False:
        errors.append("fixture support must remain an unpublished workspace member")
    workspace_dependencies = workspace.get("dependencies", {})
    fixture_count = 0
    for path, manifest in manifests.items():
        relative = path.relative_to(root).as_posix()
        if manifest.get("patch") or manifest.get("replace"):
            errors.append(f"{relative}: unreviewed dependency replacement route")
        for table, dependencies in dependency_tables(manifest):
            for name, spec in dependencies.items():
                if not isinstance(spec, dict):
                    continue
                label = f"{relative}:{table}.{name}"
                if "git" in spec:
                    errors.append(f"{label}: unreviewed Git dependency route")
                if spec.get("workspace"):
                    if name not in workspace_dependencies or any(
                        key in spec for key in ("path", "git", "version", "package")
                    ):
                        errors.append(f"{label}: invalid workspace inheritance")
                    # The defining workspace entry is audited independently.
                    continue
                if "path" not in spec:
                    continue
                target = (path.parent / spec["path"]).resolve()
                expected = members.get(target)
                if expected is None:
                    errors.append(f"{label}: path is outside registered workspace members")
                    continue
                if (relative, table, name) == (FIXTURE_MANIFEST, FIXTURE_TABLE, FIXTURE_NAME):
                    fixture_count += 1
                    if spec != FIXTURE_SPEC or expected[0] != FIXTURE_NAME:
                        errors.append(f"{label}: fixture exception must retain its exact declaration")
                elif spec.get("version") != expected[1] or spec.get("package", name) != expected[0]:
                    errors.append(f"{label}: local dependency must name its member and explicit version")
    if fixture_count != 1:
        errors.append("exactly one reviewed C API fixture exception is required")
    return errors
