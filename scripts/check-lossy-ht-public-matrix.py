#!/usr/bin/env python3
"""Guard exact test identity and run the complete optimised matrix once."""

from __future__ import annotations

import json
from pathlib import Path
import subprocess
import sys

from importlib.util import module_from_spec, spec_from_file_location

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parent.parent
SMOKE = "ht_lossy_public_tests::lossy_ht_public_smoke"
COMPLETE = "ht_lossy_public_tests::lossy_ht_public_complete_matrix"
BUILD = ["--release", "-p", "emuella-j2k-core", "--lib"]


def guard(inventory: dict) -> None:
    suites = inventory["rust-suites"]
    if not isinstance(suites, dict):
        raise ValueError("expected Nextest rust-suites object")
    for name, ignored in ((SMOKE, False), (COMPLETE, True)):
        matches = [(suite, suite["testcases"][name]) for suite in suites.values()
                   if name in suite["testcases"]]
        if len(matches) != 1:
            raise ValueError(f"lossy HT public qualification test is missing or ambiguous: {name}")
        suite, test = matches[0]
        if (suite["package-name"] != "emuella-j2k-core" or suite["kind"] != "lib"
                or test["kind"] != "test" or test["ignored"] is not ignored):
            raise ValueError(f"lossy HT public smoke/complete test classification is incorrect: {name}")


def main() -> int:
    try:
        spec = spec_from_file_location("nextest_runner", ROOT / "scripts/run-nextest.py")
        runner = module_from_spec(spec)
        spec.loader.exec_module(runner)
        runner.prepare("lossy-ht-matrix")
        listing = subprocess.run(["cargo", "nextest", "list", *BUILD, "--message-format=json"],
                                 cwd=ROOT, check=True, capture_output=True, text=True)
        # Keep compiler diagnostics observable even though the JSON is parsed.
        print(listing.stderr, end="", file=sys.stderr)
        guard(json.loads(listing.stdout))
        return runner.run("lossy-ht-matrix", [*BUILD, "-E", f"test(={COMPLETE})",
                          "--run-ignored", "only", "--no-tests", "fail", "--success-output", "immediate"])
    except (OSError, ValueError, KeyError, TypeError, subprocess.CalledProcessError) as error:
        if isinstance(error, subprocess.CalledProcessError):
            print(error.stderr or "", end="", file=sys.stderr)
        print(f"Lossy HT public matrix failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main())
