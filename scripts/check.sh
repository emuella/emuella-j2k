#!/bin/sh
set -eu

repository_root=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repository_root"

# Only a checkout's own Git marker triggers export. Do not discover a parent
# repository when these checks run in an unpacked source tree.
if [ -e .git ] || [ -L .git ]; then
  exec python3 scripts/check-committed-tree.py "$@"
fi

# An unpacked source tree has no binding and always takes the complete route.
sh scripts/check-source.sh
python3 scripts/documentation_checks.py --package-inventory
sh scripts/check-full-runtime.sh
cargo deny check
cargo deny \
  --manifest-path crates/emuella-j2k-codestream/fuzz/Cargo.toml \
  --config crates/emuella-j2k-codestream/fuzz/deny.toml \
  --locked \
  check
