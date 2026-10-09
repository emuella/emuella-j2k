#!/usr/bin/env python3
"""Run one named configuration with reports outside the disposable source."""

from __future__ import annotations

import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import tomllib

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent.parent
MINIMUM = (0, 9, 146)


def run(profile: str, arguments: list[str], root: Path = ROOT) -> int:
    configuration = (root / ".config/nextest.toml").read_text()
    profiles = tomllib.loads(configuration)["profile"]
    if profile not in profiles or profile in {"default", "ci"}:
        raise ValueError(f"unknown test configuration: {profile}")
    reports = Path(os.environ.get("EMUELLA_NEXTEST_REPORT_DIR", root / "target/nextest")).resolve()
    # A Git checkout may retain reports in its ignored target. An exported
    # source tree must never gain runner output, even with CARGO_TARGET_DIR set.
    if reports == root or root in reports.parents:
        target = root / "target"
        if not (root / ".git").exists() or target not in reports.parents:
            raise ValueError("Nextest reports must stay outside exported source")
    report = reports / profile / "junit.xml"
    report.parent.mkdir(parents=True, exist_ok=True)
    report.unlink(missing_ok=True)
    version = subprocess.run(["cargo", "nextest", "--version"], capture_output=True, text=True, check=False)
    match = re.match(r"cargo-nextest (\d+)\.(\d+)\.(\d+)(?:\s|$)", version.stdout)
    if version.returncode or match is None or tuple(map(int, match.groups())) < MINIMUM:
        raise ValueError("cargo-nextest >= 0.9.146 is required; install the pinned pre-built release")
    # Nextest 0.9.146 resolves store.dir against the workspace, independently of
    # CARGO_TARGET_DIR. Override it explicitly without editing audited source.
    with tempfile.NamedTemporaryFile(mode="w", suffix=".toml", dir=reports) as config:
        config.write(configuration + "\n[store]\ndir = " + json.dumps(str(reports)) + "\n")
        config.write(f"\n[profile.{profile}.junit]\nreport-name = {json.dumps(profile)}\n")
        config.flush()
        print(f"Nextest configuration: {profile}; report: {report}", flush=True)
        result = subprocess.run(["cargo", "nextest", "run", "--config-file", config.name,
                                 "--profile", profile, *arguments], cwd=root, check=False)
    if result.returncode == 0 and not report.is_file():
        raise ValueError(f"successful Nextest run did not produce {report}")
    return result.returncode


def main() -> int:
    try:
        if len(sys.argv) < 2:
            raise ValueError("usage: run-nextest.py CONFIGURATION [NEXTEST ARGUMENTS...]")
        return run(sys.argv[1], sys.argv[2:])
    except (OSError, ValueError) as error:
        print(f"Nextest runner failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
