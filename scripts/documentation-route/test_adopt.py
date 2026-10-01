"""Portable regressions for reviewed local payload adoption and updates."""

import json
from pathlib import Path
import shutil
import unittest

import adopt
from documentation_route import RouteError
from test_documentation_route import RepositoryCase


class AdoptionTests(RepositoryCase):
    def setUp(self):
        super().setUp()
        self.payload = self.root / adopt.PAYLOAD_ROOT
        self.payload.mkdir(parents=True)
        for name in adopt.FILES:
            shutil.copyfile(Path(adopt.__file__).parent / name, self.payload / name)
        self.revision = self.commit()
        self.target = self.root.parent / "consumer/tools/documentation-route"

    def plan(self):
        return adopt.adopt(self.root, self.revision, "1.0.0", self.target)

    def install(self):
        plan = self.plan()
        return adopt.adopt(self.root, self.revision, "1.0.0", self.target, receipt=plan)

    def test_dry_run_then_adopt_is_explicit_and_unchanged_update_succeeds(self):
        plan = self.plan()
        self.assertFalse(self.target.exists())
        self.assertEqual(set(plan["actions"].values()), {"create"})
        self.install()
        self.assertEqual(set(self.plan()["actions"].values()), {"unchanged"})
        self.install()
        manifest = json.loads((self.target / adopt.MANIFEST).read_text())
        self.assertEqual(manifest["source_commit"], self.revision)
        self.assertEqual(set(manifest["files"]), set(adopt.FILES))

    def test_update_prior_managed_hash_and_preserve_authored_product_files(self):
        self.install()
        authored = self.target / "policy.json"
        authored.write_text("Authored product policy\n")
        (self.payload / "README.md").write_text("New helper documentation\n")
        self.revision = self.commit()
        plan = self.plan()
        self.assertEqual(plan["actions"]["README.md"], "update")
        self.install()
        self.assertEqual(authored.read_text(), "Authored product policy\n")
        self.assertEqual(
            (self.target / "README.md").read_text(), "New helper documentation\n"
        )

    def test_local_drift_and_unknown_conflict_stop_before_writes(self):
        for installed in (False, True):
            with self.subTest(installed=installed):
                shutil.rmtree(self.target, ignore_errors=True)
                if installed:
                    self.install()
                self.target.mkdir(parents=True, exist_ok=True)
                (self.target / "README.md").write_text("Local edit\n")
                before = {p.name: p.read_bytes() for p in self.target.iterdir()}
                with self.assertRaises(RouteError):
                    self.plan()
                self.assertEqual(
                    before, {p.name: p.read_bytes() for p in self.target.iterdir()}
                )

    def test_receipt_rejects_intervening_file_or_manifest_changes(self):
        plan = self.plan()
        self.install()
        with self.assertRaises(RouteError):
            adopt.adopt(self.root, self.revision, "1.0.0", self.target, receipt=plan)
        plan = self.plan()
        (self.target / adopt.MANIFEST).write_text("{}")
        with self.assertRaises(RouteError):
            adopt.adopt(self.root, self.revision, "1.0.0", self.target, receipt=plan)

    def test_managed_deletion_mode_and_valid_manifest_drift_stop(self):
        self.install()
        path = self.target / "README.md"
        original = path.read_bytes()
        path.unlink()
        with self.assertRaises(RouteError):
            self.plan()
        path.write_bytes(original)
        path.chmod(0o755)
        with self.assertRaises(RouteError):
            self.plan()
        path.chmod(0o644)
        manifest_path = self.target / adopt.MANIFEST
        manifest = json.loads(manifest_path.read_text())
        manifest["files"]["README.md"] = "a" * 64
        manifest_path.write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
        with self.assertRaises(RouteError):
            self.plan()

    def test_strict_manifest_rejects_unknown_keys_unsafe_files_and_primitive_types(
        self,
    ):
        self.install()
        manifest_path = self.target / adopt.MANIFEST
        original = json.loads(manifest_path.read_text())
        for update in (
            {"schema_version": True},
            {"source_commit": "HEAD"},
            {"version": 1},
            {"files": {"../escape": "a" * 64}},
            {"extra": "unknown"},
        ):
            manifest = {**original, **update}
            manifest_path.write_text(json.dumps(manifest))
            with self.assertRaises(RouteError):
                self.plan()

    def test_symlinked_target_parent_file_or_manifest_is_rejected(self):
        outside = self.root.parent / "outside"
        outside.mkdir()
        parent = self.target.parent
        parent.parent.mkdir(parents=True)
        parent.symlink_to(outside, target_is_directory=True)
        with self.assertRaises(RouteError):
            self.plan()
        parent.unlink()
        self.install()
        for name in ("README.md", adopt.MANIFEST):
            path = self.target / name
            content = path.read_bytes()
            path.unlink()
            path.symlink_to(outside / "missing")
            with self.assertRaises(RouteError):
                self.plan()
            path.unlink()
            path.write_bytes(content)
        self.assertEqual(list(outside.iterdir()), [])

    def test_source_is_immutable_versioned_and_regular(self):
        (self.payload / "README.md").write_text("Uncommitted source\n")
        plan = self.install()
        self.assertNotEqual(
            (self.target / "README.md").read_text(), "Uncommitted source\n"
        )
        self.assertEqual(plan["source_commit"], self.revision)
        for revision, version in (
            ("HEAD", "1.0.0"),
            ("--help", "1.0.0"),
            ("0" * 40, "1.0.0"),
            (self.revision, "2.0.0"),
        ):
            with self.assertRaises(RouteError):
                adopt.adopt(self.root, revision, version, self.target)
        (self.payload / "README.md").unlink()
        (self.payload / "README.md").symlink_to("VERSION")
        self.revision = self.commit()
        with self.assertRaises(RouteError):
            self.plan()


if __name__ == "__main__":
    unittest.main()
