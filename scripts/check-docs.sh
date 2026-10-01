#!/bin/sh
# Focused authoring feedback accepts dirty documentation/Rust source. It does
# not establish committed-tree identity or replace the canonical delivery gate.
set -eu
repository_root=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repository_root"
exec python3 scripts/documentation_checks.py
