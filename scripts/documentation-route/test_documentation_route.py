"""Portable isolated regressions; requires only Python's standard library and Git."""

import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

import documentation_route as route


class RepositoryCase(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name) / "repo"
        self.root.mkdir()
        self.git("init", "-q")
        self.git("config", "user.email", "test@example.invalid")
        self.git("config", "user.name", "Documentation route tests")
        self.write("README.md", "Prose\n")
        self.write("docs/guide.md", "Guide\n")
        self.write("src/lib.rs", "// Comment\npub fn example() {}\n")
        self.write(".github/workflows/check.yml", "name: Check\n")
        self.write(
            "policy.json",
            json.dumps(
                {
                    "schema_version": 1,
                    "eligible_paths": [
                        "README.md",
                        "docs/guide.md",
                        "docs/new.md",
                        "docs/tab\tnewline\n ü.md",
                    ],
                }
            )
            + "\n",
        )
        self.base = self.commit()

    def git(self, *args, input=None, check=True):
        return subprocess.run(
            ["git", "-C", str(self.root), *args],
            input=input,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=check,
        ).stdout.decode()

    def write(self, path, text="Changed\n"):
        file = self.root / path
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text(text)
        return file

    def commit(self):
        self.git("add", ".")
        self.git("commit", "-qm", "Test snapshot")
        return self.git("rev-parse", "HEAD").strip()

    def classify(self, **kwargs):
        values = dict(
            candidate_commit=self.git("rev-parse", "HEAD").strip(),
            base_commit=self.base,
            policy_path="policy.json",
        )
        values.update(kwargs)
        return route.classify(self.root, **values)

    def reset(self):
        self.git("reset", "--hard", "-q", self.base)
        self.git("clean", "-fdq")


class ClassifierTests(RepositoryCase):
    def test_empty_default_and_absent_base_are_full(self):
        self.assertEqual(self.classify(policy_path=None).route, "full")
        self.assertEqual(self.classify(base_commit=None).route, "full")

    def test_docs_each_independent_layer_and_provenance(self):
        self.write("README.md")
        self.commit()
        self.write("docs/guide.md")
        self.git("add", "docs/guide.md")
        self.write("README.md", "Unstaged\n")
        self.write("docs/new.md")
        result = self.classify()
        self.assertEqual(result.route, "docs")
        self.assertEqual(
            {c["layer"] for c in result.changes},
            {"committed", "staged", "unstaged", "untracked"},
        )
        value = result.to_dict()
        self.assertEqual(value["schema_version"], 1)
        self.assertEqual(value["base_commit"], self.base)
        self.assertEqual(
            value["candidate_tree"], self.git("rev-parse", "HEAD^{tree}").strip()
        )
        self.assertEqual(value["policy"]["sha256"], value["policy"]["base_sha256"])
        self.assertEqual(
            value["helper_sha256"], route.sha256(Path(route.__file__).read_bytes())
        )
        json.dumps(value)

    def test_every_rust_edit_including_comment_is_full(self):
        for layer in ("unstaged", "staged", "committed"):
            with self.subTest(layer=layer):
                self.write(
                    "src/lib.rs", "// Changed inline comment\npub fn example() {}\n"
                )
                self.write("README.md")
                if layer != "unstaged":
                    self.git("add", ".")
                if layer == "committed":
                    self.commit()
                self.assertEqual(self.classify().route, "full")
                self.reset()

    def test_noneligible_untracked_and_ignored_files(self):
        self.write("ignored.txt")
        self.git("config", "core.excludesFile", str(self.root.parent / "ignore"))
        (self.root.parent / "ignore").write_text("ignored.txt\n")
        self.assertEqual(self.classify().route, "docs")
        self.write("new.rs")
        self.assertEqual(self.classify().route, "full")

    def test_staged_code_reversal_cannot_hide_scope(self):
        self.write("src/lib.rs")
        self.git("add", "src/lib.rs")
        self.write("src/lib.rs", "// Comment\npub fn example() {}\n")
        result = self.classify()
        self.assertEqual(result.route, "full")
        self.assertEqual({c["layer"] for c in result.changes}, {"staged", "unstaged"})

    def test_transitions_inspect_both_sides_without_rename_inference(self):
        for source, target, expected in (
            ("README.md", "docs/new.md", "docs"),
            ("README.md", "new.rs", "full"),
            ("src/lib.rs", "docs/new.md", "full"),
        ):
            with self.subTest(source=source, target=target):
                self.git("mv", source, target)
                result = self.classify()
                self.assertEqual(result.route, expected)
                self.assertEqual({c["status"] for c in result.changes}, {"A", "D"})
                self.reset()
        self.git("rm", "README.md")
        self.assertEqual(self.classify().route, "docs")
        self.git("rm", "src/lib.rs")
        self.assertEqual(self.classify().route, "full")

    def test_mode_changes_and_symlinks_select_full(self):
        for kind in (
            "executable",
            "symlink",
            "untracked_executable",
            "untracked_symlink",
        ):
            with self.subTest(kind=kind):
                path = self.root / (
                    "docs/new.md" if kind.startswith("untracked") else "README.md"
                )
                if "symlink" in kind:
                    path.unlink(missing_ok=True)
                    path.symlink_to(self.root / "src/lib.rs")
                else:
                    path.write_text("Prose\n")
                    path.chmod(0o755)
                self.git("config", "core.fileMode", "false")
                self.assertEqual(self.classify().route, "full")
                self.reset()

    def test_hidden_flags_alone_select_full(self):
        for flag in ("assume-unchanged", "skip-worktree"):
            for path in ("README.md", "src/lib.rs"):
                with self.subTest(flag=flag, path=path):
                    self.git("update-index", "--" + flag, path)
                    self.assertEqual(self.classify().route, "full")
                    self.write(path)
                    self.assertEqual(self.classify().route, "full")
                    self.git("update-index", "--no-" + flag, path)
                    self.reset()

    def test_sparse_config_even_when_materialised_selects_full(self):
        self.git("config", "core.sparseCheckout", "true")
        self.assertEqual(self.classify().route, "full")
        self.git("config", "core.sparseCheckout", "false")
        self.assertEqual(self.classify().route, "docs")
        self.git("sparse-checkout", "set", "--no-cone", "/README.md", "/policy.json")
        self.assertEqual(self.classify().route, "full")

    def test_unmerged_index_selects_full(self):
        blob = self.git("rev-parse", "HEAD:README.md").strip()
        self.git("update-index", "--force-remove", "README.md")
        self.git(
            "update-index",
            "--index-info",
            input=(
                f"100644 {blob} 1\tREADME.md\n100644 {blob} 2\tREADME.md\n"
            ).encode(),
        )
        self.assertEqual(self.classify().route, "full")

    def test_policy_candidate_broadening_and_dirty_changes_select_full(self):
        policy = json.loads((self.root / "policy.json").read_text())
        policy["eligible_paths"].append("extra.md")
        self.write("policy.json", json.dumps(policy))
        self.write("extra.md")
        self.assertEqual(self.classify().route, "full")
        self.git("add", ".")
        self.assertEqual(self.classify().route, "full")
        self.commit()
        result = self.classify()
        self.assertEqual(result.route, "full")
        self.assertNotEqual(result.policy["sha256"], result.policy["base_sha256"])

    def test_missing_base_policy_selects_full(self):
        self.git("rm", "policy.json")
        absent = self.commit()
        self.git("checkout", self.base, "--", "policy.json")
        self.commit()
        self.assertEqual(self.classify(base_commit=absent).route, "full")

    def test_invalid_current_policy_fails_in_all_layers(self):
        for invalid in (
            "{",
            '{"schema_version":true,"eligible_paths":[]}',
            '{"schema_version":1,"eligible_paths":["../README.md"]}',
            '{"schema_version":1,"eligible_paths":["src/lib.rs"]}',
            '{"schema_version":1,"eligible_paths":[],"force_docs":true}',
            '{"schema_version":1,"schema_version":1,"eligible_paths":[]}',
        ):
            for layer in ("unstaged", "staged", "committed"):
                with self.subTest(invalid=invalid, layer=layer):
                    self.write("policy.json", invalid)
                    if layer != "unstaged":
                        self.git("add", ".")
                    if layer == "committed":
                        self.commit()
                    with self.assertRaises(route.RouteError):
                        self.classify()
                    self.reset()

    def test_invalid_candidate_is_fatal_and_base_is_conservative(self):
        for candidate in (None, "", True, "HEAD", "--help", "0" * 40):
            with self.subTest(candidate=candidate):
                with self.assertRaises(route.RouteError):
                    self.classify(candidate_commit=candidate)
        self.write("README.md")
        self.commit()
        with self.assertRaises(route.RouteError):
            self.classify(candidate_commit=self.base)
        for base in (None, "", True, "HEAD", "--help", "0" * 40):
            self.assertEqual(self.classify(base_commit=base).route, "full")

    def test_malformed_staged_policy_is_fatal_even_with_reversal_and_unknown_scope(
        self,
    ):
        original = (self.root / "policy.json").read_text()
        self.write("policy.json", "malformed")
        self.git("add", "policy.json")
        self.write("policy.json", original)
        with open(os.fsencode(self.root) + b"/unsupported-\xff.md", "wb") as stream:
            stream.write(b"Prose\n")
        with self.assertRaises(route.RouteError):
            self.classify()

    def test_unrelated_history_selects_full(self):
        self.git("checkout", "--orphan", "other")
        self.write("README.md")
        unrelated = self.commit()
        self.git("checkout", "-q", self.base)
        self.assertEqual(self.classify(base_commit=unrelated).route, "full")

    def test_shallow_history_selects_full(self):
        self.write("README.md")
        self.commit()
        clone = self.root.parent / "clone"
        subprocess.run(
            ["git", "clone", "-q", "--depth=2", self.root.as_uri(), str(clone)],
            check=True,
        )
        candidate = self.git("rev-parse", "HEAD").strip()
        self.assertEqual(
            route.classify(
                clone,
                candidate_commit=candidate,
                base_commit=self.base,
                policy_path="policy.json",
            ).route,
            "full",
        )

    def test_odd_valid_filename_is_preserved_and_bad_encoding_is_full(self):
        odd = "docs/tab\tnewline\n ü.md"
        self.write(odd)
        result = self.classify()
        self.assertEqual(result.route, "docs")
        self.assertEqual(result.changes[0]["new_path"], odd)
        self.commit()
        self.assertEqual(self.classify().route, "docs")
        bad = os.fsencode(self.root) + b"/invalid-\xff.md"
        with open(bad, "wb") as stream:
            stream.write(b"Prose\n")
        self.assertEqual(self.classify().route, "full")

    def test_git_redirect_environment_and_replace_objects_are_ignored(self):
        with patch.dict(
            os.environ,
            {
                "GIT_DIR": "/missing",
                "GIT_WORK_TREE": "/missing",
                "GIT_INDEX_FILE": "/missing",
                "GIT_CONFIG_COUNT": "1",
                "GIT_CONFIG_KEY_0": "core.sparseCheckout",
                "GIT_CONFIG_VALUE_0": "true",
            },
        ):
            self.assertEqual(
                route.classify(
                    self.root,
                    candidate_commit=self.base,
                    base_commit=self.base,
                    policy_path="policy.json",
                ).route,
                "docs",
            )
        self.write("src/lib.rs")
        candidate = self.commit()
        self.git("replace", self.base, candidate)
        self.assertEqual(self.classify().route, "full")

    def test_helper_integrity_and_runtime_folder_guard(self):
        folder = self.root / "tools/documentation-route"
        folder.mkdir(parents=True)
        for name in ("documentation_route.py", "VERSION"):
            shutil.copyfile(Path(route.__file__).parent / name, folder / name)
        self.write("tools/documentation-route/README.md")
        policy = json.loads((self.root / "policy.json").read_text())
        policy["eligible_paths"].append("tools/documentation-route/README.md")
        self.write("policy.json", json.dumps(policy))
        self.base = self.commit()
        spec = importlib.util.spec_from_file_location(
            "nested_route", folder / "documentation_route.py"
        )
        module = importlib.util.module_from_spec(spec)
        sys.modules[spec.name] = module
        self.addCleanup(sys.modules.pop, spec.name)
        spec.loader.exec_module(module)
        self.write("tools/documentation-route/README.md", "New helper docs\n")
        self.assertEqual(
            module.classify(
                self.root,
                candidate_commit=self.base,
                base_commit=self.base,
                policy_path="policy.json",
            ).route,
            "full",
        )
        self.write(
            "tools/documentation-route/documentation_route.py", "Edited helper\n"
        )
        with self.assertRaises(module.RouteError):
            module.classify(
                self.root,
                candidate_commit=self.base,
                base_commit=self.base,
                policy_path="policy.json",
            )

    def test_incomplete_observation_selects_full(self):
        original = route.Git.run

        def incomplete(git, *args):
            return b"not NUL terminated" if args[0] == "diff" else original(git, *args)

        with patch.object(route.Git, "run", incomplete):
            self.assertEqual(self.classify().route, "full")


if __name__ == "__main__":
    unittest.main()
