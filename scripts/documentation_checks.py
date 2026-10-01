#!/usr/bin/env python3
"""Bounded checks for authored documentation; no committed-tree pass claim."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import tomllib
from urllib.parse import unquote, urlsplit

sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parent.parent
# These are explicit verification consumers, not reduced-route eligibility.
# New contract guides remain full-only under documentation-policy.json.
SURFACES = {"README.md": ["rust,no_run"], "docs/README.md": [],
            "docs/getting-started.md": ["rust"], "docs/rust-api.md": [],
            "docs/caller-owned-output.md": ["rust"], "docs/error-handling.md": ["rust"]}
PACKAGES = ("emuella-j2k", "emuella-j2k-core", "emuella-j2k-codestream",
            "emuella-j2k-container")
SAME_REPO = "https://github.com/emuella/emuella-j2k/blob/main/"


class DocumentationError(Exception):
    pass


def prose_and_examples(text: str, *, strict: bool) -> tuple[str, list[tuple[str, str]]]:
    prose, examples, code = [], [], []
    fence = None
    for line in text.splitlines():
        match = re.fullmatch(r"(`{3,}|~{3,})(.*)", line)
        if match:
            marker, info = match.groups()
            if fence is None:
                if strict and (marker != "```" or info not in {"rust", "rust,no_run", "sh", "toml"}):
                    raise DocumentationError(f"unsupported documentation fence: {line}")
                fence = (marker, info)
                code = []
            elif marker == fence[0] and not info:
                if fence[1].startswith("rust"):
                    examples.append((fence[1], "\n".join(code)))
                fence = None
            else:
                code.append(line)
        elif fence is not None:
            code.append(line)
        else:
            plain = re.sub(r"`[^`\n]*`", "", line)
            if strict and (line.startswith("    ") or line.lstrip().startswith(("```", "~~~")) or "<" in plain or ">" in plain):
                raise DocumentationError("unsupported indented fence/code or HTML in eligible prose")
            prose.append(line)
    if fence is not None:
        raise DocumentationError("unterminated documentation fence")
    return "\n".join(prose), examples


def anchors(text: str) -> set[str]:
    prose, _ = prose_and_examples(text, strict=False)
    result, counts = set(), {}
    for line in prose.splitlines():
        heading = re.match(r"^#{1,6} +(.+?)(?: +#+)?$", line)
        if heading:
            title = heading.group(1).lower().replace("`", "")
            slug = "".join(c for c in title if c.isalnum() or c in " _-").replace(" ", "-")
            count = counts.get(slug, 0)
            counts[slug] = count + 1
            result.add(slug if count == 0 else f"{slug}-{count}")
    return result


def check_target(root: Path, source: str, target: str) -> None:
    if target.startswith(SAME_REPO):
        target = "/" + target[len(SAME_REPO):]
    parsed = urlsplit(target)
    if parsed.scheme or parsed.netloc:
        if parsed.scheme not in {"https", "http", "mailto"}:
            raise DocumentationError(f"unsupported reference scheme: {target}")
        return  # Remote availability is deliberately outside deterministic checks.
    if parsed.query:
        raise DocumentationError(f"unsupported local reference query: {target}")
    name = unquote(parsed.path)
    path = (root / name.lstrip("/") if name.startswith("/") else
            root / Path(source).parent / name if name else root / source).resolve()
    if path != root and root not in path.parents:
        raise DocumentationError(f"reference escapes source tree: {target}")
    if not path.is_file() or path.is_symlink():
        raise DocumentationError(f"missing/non-regular local reference: {source}: {target}")
    if parsed.fragment:
        if path.suffix != ".md" or unquote(parsed.fragment) not in anchors(path.read_text(encoding="utf-8")):
            raise DocumentationError(f"missing local anchor: {source}: {target}")


def check_references(root: Path, source: str, prose: str) -> None:
    # Inspected surfaces use only simple inline and explicit reference links.
    # Reject extra syntax rather than accepting an unverified extension.
    prose = re.sub(r"`[^`\n]*`", "", prose)
    if "![" in prose or re.search(r"\[\[|\]\[\]", prose):
        raise DocumentationError("unsupported image/wiki/shortcut reference syntax")
    definitions = {}
    def definition(match: re.Match[str]) -> str:
        label, target = match.groups()
        key = label.casefold()
        if key in definitions:
            raise DocumentationError(f"duplicate reference definition: {label}")
        definitions[key] = target
        check_target(root, source, target)
        return ""
    prose = re.sub(r"^\[([^]\n]+)\]: +(\S+) *$", definition, prose, flags=re.MULTILINE)
    def inline(match: re.Match[str]) -> str:
        check_target(root, source, match.group(2))
        return match.group(1)
    prose = re.sub(r"\[([^]\n]+)\]\(([^()\s]+)\)", inline, prose)
    def reference(match: re.Match[str]) -> str:
        label = match.group(2).casefold()
        if label not in definitions:
            raise DocumentationError(f"missing reference definition: {label}")
        return match.group(1)
    prose = re.sub(r"\[([^]\n]+)\]\[([^]\n]+)\]", reference, prose)
    if re.search(r"\[[^]\n]*\]", prose):
        raise DocumentationError(f"unsupported or malformed reference in {source}")


def select_examples(name: str, examples: list[tuple[str, str]]) -> None:
    expected = SURFACES[name]
    if [kind for kind, _ in examples] != expected:
        raise DocumentationError(f"{name}: expected Rust blocks {expected}; got {[kind for kind, _ in examples]}")


def inspect_documents(root: Path) -> dict[str, list[tuple[str, str]]]:
    result = {}
    for name in SURFACES:
        prose, examples = prose_and_examples((root / name).read_text(encoding="utf-8"), strict=True)
        select_examples(name, examples)
        check_references(root, name, prose)
        result[name] = examples
    return result


def command(arguments: list[str], root: Path, env: dict[str, str]) -> str:
    print("Documentation command: " + " ".join(arguments), flush=True)
    result = subprocess.run(arguments, cwd=root, env=env, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    print(result.stdout, end="", flush=True)
    if result.returncode:
        raise DocumentationError(f"documentation command failed (exit {result.returncode})")
    return result.stdout


def check_doctest_coverage(output: str) -> None:
    # Hosted Cargo output may retain ANSI colour even when redirected. Coverage
    # depends on rustdoc's result text, not its terminal presentation.
    output = re.sub(r"\x1b\[[0-?]*[ -/]*[@-~]", "", output)
    counts = {}
    for name, section in re.findall(r"Doc-tests (\w+)\n(.*?)(?=Doc-tests |\Z)", output, re.DOTALL):
        result = re.search(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored", section)
        if result is None or name in counts:
            raise DocumentationError("missing or duplicate selected doctest result")
        counts[name] = tuple(map(int, result.groups()))
    if (set(counts) != {name.replace("-", "_") for name in PACKAGES}
            or any(failed or ignored for _, failed, ignored in counts.values())
            or any(counts.get(name, (0,))[0] < 1 for name in ("emuella_j2k", "emuella_j2k_core"))):
        raise DocumentationError("selected doctests require nonzero facade and defining-crate coverage without ignored tests")


def write_example_consumers(harness: Path, documents: dict[str, list[tuple[str, str]]]) -> int:
    """Bind every explicitly selected block; never silently consume only the first."""
    source = []
    for name, examples in documents.items():
        for block, (kind, code) in enumerate(examples, 1):
            index = len(source)
            markdown = f"```{kind}\n{code}\n```\n"
            (harness / "src" / f"example{index}.md").write_text(markdown, encoding="utf-8")
            source.append(f'#[doc = include_str!("example{index}.md")]\npub struct Example{index};\n')
            print(f"Example consumer: {name} block {block}: {'compile only (external input)' if kind == 'rust,no_run' else 'execute'}", flush=True)
    (harness / "src/lib.rs").write_text("".join(source), encoding="utf-8")
    return len(source)


def check_example_coverage(output: str, expected: int) -> None:
    output = re.sub(r"\x1b\[[0-?]*[ -/]*[@-~]", "", output)
    results = re.findall(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored", output)
    if expected < 1 or results != [(str(expected), "0", "0")]:
        raise DocumentationError(f"expected all {expected} explicit Markdown example consumers to pass without ignored or unaccounted tests")


def package_inventory(root: Path, env: dict[str, str]) -> None:
    # Apply the same legal/source member invariant to every actual inherited
    # README consumer, without claiming .crate qualification. Other members use
    # their own README or no README and retain the all-member source legal audit.
    from package_legal_policy import PACKAGE_POLICY, legal_content_errors
    workspace = tomllib.loads((root / "Cargo.toml").read_text(encoding="utf-8"))
    for member in workspace["workspace"]["members"]:
        manifest = tomllib.loads((root / member / "Cargo.toml").read_text(encoding="utf-8"))
        name = manifest["package"]["name"]
        if manifest["package"].get("readme") != {"workspace": True}:
            continue
        output = command(["cargo", "package", "--list", "--locked", "-p", name], root, env)
        files = {line for line in output.splitlines() if not line.startswith(("warning:", " "))}
        expected = set(PACKAGE_POLICY[name].legal_file_sha256) | {"Cargo.toml", "README.md", "src/lib.rs"}
        if name == "emuella-j2k-cli":
            expected.remove("src/lib.rs")
            expected.add("src/main.rs")
        if not expected <= files:
            raise DocumentationError(f"package source inventory omits {name}: {sorted(expected - files)}")
        legal = {name_: (root / member / name_).read_bytes() for name_ in PACKAGE_POLICY[name].legal_file_sha256}
        errors = legal_content_errors(name, legal)
        if errors:
            raise DocumentationError(f"package legal source {name}: {errors}")
    print("Package source inventories passed; actual release archives are not produced or qualified.")


def run(root: Path, *, inventory: bool) -> None:
    documents = inspect_documents(root)
    print(f"Local references passed: {', '.join(SURFACES)}", flush=True)
    env = os.environ.copy()
    env["CARGO_TERM_COLOR"] = "never"
    env["RUSTDOCFLAGS"] = "-D rustdoc::broken_intra_doc_links"
    selected = [argument for name in PACKAGES for argument in ("-p", name)]
    command(["cargo", "doc", "--lib", "--no-deps", "--locked", "--target", "host-tuple", *selected], root, env)
    output = command(["cargo", "test", "--doc", "--locked", "--target", "host-tuple", *selected], root, env)
    check_doctest_coverage(output)
    # Rustdoc does not collect defining-crate examples through a facade re-export.
    # Separate generated harnesses bind each inspected Markdown source explicitly.
    with tempfile.TemporaryDirectory(prefix="emuella-doc-examples-") as temporary:
        harness = Path(temporary)
        (harness / "src").mkdir()
        (harness / "Cargo.toml").write_text(
            '[package]\nname = "emuella-documentation-examples"\nversion = "0.0.0"\nedition = "2024"\n'
            '[workspace]\n[dependencies]\nemuella-j2k = { path = ' + json.dumps(str(root / "crates/emuella-j2k")) + ' }\n', encoding="utf-8")
        expected = write_example_consumers(harness, documents)
        command(["cargo", "generate-lockfile", "--offline", "--manifest-path", str(harness / "Cargo.toml")], root, env)
        output = command(["cargo", "test", "--doc", "--locked", "--offline", "--target", "host-tuple", "--manifest-path", str(harness / "Cargo.toml")], root, env)
        check_example_coverage(output, expected)
    if inventory:
        package_inventory(root, env)
    print("Documentation authoring checks passed; no committed-tree/delivery pass is claimed.", flush=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--references-only", action="store_true")
    parser.add_argument("--package-inventory", action="store_true", help="canonical export only: inspect every inherited README consumer")
    args = parser.parse_args()
    try:
        if args.references_only:
            inspect_documents(ROOT)
            print("Documentation references and example selection passed.")
        else:
            run(ROOT, inventory=args.package_inventory)
    except (DocumentationError, OSError, ValueError) as error:
        print(f"Documentation checks failed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
