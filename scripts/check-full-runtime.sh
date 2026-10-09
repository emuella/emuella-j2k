#!/bin/sh
set -eu
fuzz_target=${EMUELLA_FUZZ_TARGET_DIR:-${CARGO_TARGET_DIR:-crates/emuella-j2k-codestream/fuzz/target}}
python3 scripts/test-layer2-conformance-inspection.py
python3 scripts/test-layer2-decoded-pixel-canary.py
python3 scripts/test-layer2-derived-set.py
python3 scripts/test-layer2-rendered-pixel.py
cargo fmt --all --check
cargo fmt \
  --manifest-path crates/emuella-j2k-codestream/fuzz/Cargo.toml \
  --all \
  -- \
  --check
cargo check --workspace --all-targets
cargo check -p emuella-j2k-codestream --features parallel
python3 scripts/run-nextest.py workspace --workspace
cargo test --workspace --doc
env -u CARGO_TARGET_DIR -u CARGO_BUILD_BUILD_DIR sh scripts/check-c-api.sh
sh scripts/check-lossy-ht-public-matrix.sh
python3 scripts/run-nextest.py native-parallel -p emuella-j2k-test-support --features emuella-j2k-core/parallel --test native_planes --test jp2_presentation --test native_eight_components
python3 scripts/run-nextest.py lossless-parallel-release --release -p emuella-j2k-test-support --features parallel --test lossless_parallel --test lossless_bypass
python3 scripts/run-nextest.py codestream-parallel -p emuella-j2k-codestream --features parallel scalable_lossless::parallel
cargo test -p emuella-j2k-codestream --features parallel --doc scalable_lossless::parallel
python3 scripts/run-nextest.py codestream-forward53 -p emuella-j2k-codestream --features parallel scalable_lossless::forward53_tests
cargo test -p emuella-j2k-codestream --features parallel --doc scalable_lossless::forward53_tests
python3 scripts/run-nextest.py transform-analysis53 -p emuella-j2k-transform --features parallel,classic-execution-diagnostics analysis53
cargo test -p emuella-j2k-transform --features parallel,classic-execution-diagnostics --doc analysis53
python3 scripts/run-nextest.py forward53-panels --release -p emuella-j2k-test-support --features parallel,classic-execution-diagnostics --test forward53_panels
python3 scripts/run-nextest.py ordinary-dispatch --release -p emuella-j2k-test-support --features classic-execution-diagnostics --test forward53_panels ordinary_dispatch
python3 scripts/run-nextest.py codestream-diagnostics -p emuella-j2k-codestream --features parallel,classic-execution-diagnostics scalable_lossless::diagnostics
cargo test -p emuella-j2k-codestream --features parallel,classic-execution-diagnostics --doc scalable_lossless::diagnostics
python3 scripts/run-nextest.py example-lossless-parallel -p emuella-j2k-test-support --example lossless_parallel
python3 scripts/run-nextest.py example-bypass-batch -p emuella-j2k-test-support --example lossless_bypass_batch
cargo clippy --workspace --all-targets -- -D warnings
env CARGO_TARGET_DIR="$fuzz_target" CARGO_BUILD_BUILD_DIR="$fuzz_target" cargo clippy \
  --manifest-path crates/emuella-j2k-codestream/fuzz/Cargo.toml \
  --all-targets \
  --locked \
  -- \
  -D warnings
cargo check \
  -p emuella-j2k \
  -p emuella-j2k-core \
  -p emuella-j2k-codestream \
  -p emuella-j2k-container \
  -p emuella-j2k-ht \
  -p emuella-j2k-tier1 \
  -p emuella-j2k-transform \
  --no-default-features
env CARGO_TARGET_DIR="$fuzz_target" CARGO_BUILD_BUILD_DIR="$fuzz_target" cargo check \
  --manifest-path crates/emuella-j2k-codestream/fuzz/Cargo.toml \
  --all-targets \
  --locked
