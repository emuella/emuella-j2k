#!/bin/sh
set -eu

repository_root=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repository_root"

python3 scripts/test-public-tree-policy.py
python3 scripts/test-package-legal-policy.py
python3 scripts/test-layer2-conformance-inspection.py
python3 scripts/test-layer2-decoded-pixel-canary.py
python3 scripts/audit-public-tree.py
python3 scripts/generate-binary-dependency-notices.py --check
cargo fmt --all --check
cargo fmt \
  --manifest-path crates/emuella-j2k-codestream/fuzz/Cargo.toml \
  --all \
  -- \
  --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo clippy \
  --manifest-path crates/emuella-j2k-codestream/fuzz/Cargo.toml \
  --all-targets \
  --locked \
  -- \
  -D warnings
cargo deny check
cargo deny \
  --manifest-path crates/emuella-j2k-codestream/fuzz/Cargo.toml \
  --config crates/emuella-j2k-codestream/fuzz/deny.toml \
  --locked \
  check
cargo check \
  -p emuella-j2k \
  -p emuella-j2k-core \
  -p emuella-j2k-codestream \
  -p emuella-j2k-container \
  -p emuella-j2k-ht \
  -p emuella-j2k-tier1 \
  -p emuella-j2k-transform \
  --no-default-features
cargo check \
  --manifest-path crates/emuella-j2k-codestream/fuzz/Cargo.toml \
  --all-targets \
  --locked
