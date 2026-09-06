#!/usr/bin/env python3
"""Check the reviewed inventory of production C ABI unsafe block bodies."""
import collections
import json
from pathlib import Path
import re
import sys


def inventory(source):
    # The first cfg(test) is inside production decode, so use the test module.
    production, separator, _ = source.partition("#[cfg(test)]\nmod tests {")
    if not separator:
        raise ValueError("C ABI test-module boundary changed")
    # The boundary deliberately uses simple, non-nested unsafe expressions.
    # Fail closed on a new block shape; this is an inventory, not a Rust parser.
    blocks = re.findall(r"\bunsafe\s*\{([^{}]*)\}", production)
    if len(blocks) != len(re.findall(r"\bunsafe\s*\{", production)):
        raise ValueError("unrecognised C ABI unsafe block shape")
    return collections.Counter(" ".join(block.split()) for block in blocks)


def audit(source, expected):
    actual = inventory(source)
    if actual != collections.Counter(expected):
        raise ValueError("C ABI unsafe block inventory changed; review and update the inventory")


def main():
    root = Path(__file__).resolve().parent.parent
    source = (root / "crates/emuella-j2k-capi/src/lib.rs").read_text()
    expected = json.loads((root / "scripts/c-api-unsafe-inventory.json").read_text())
    audit(source, expected)


if __name__ == "__main__":
    try:
        main()
    except ValueError as error:
        sys.exit(str(error))
