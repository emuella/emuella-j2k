#!/bin/sh
set -eu

repository_root=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
crate="$repository_root/crates/emuella-j2k-capi"

cleanup_target=false
if [ -z "${CARGO_TARGET_DIR:-}" ]; then
  CARGO_TARGET_DIR=$(mktemp -d "${TMPDIR:-/tmp}/emuella-j2k-capi.XXXXXX")
  cleanup_target=true
fi
export CARGO_TARGET_DIR
if [ "$cleanup_target" = true ]; then
  trap 'rm -rf -- "$CARGO_TARGET_DIR"' EXIT HUP INT TERM
fi

cargo build --manifest-path "$crate/Cargo.toml" --release --locked

set -- $(find "$CARGO_TARGET_DIR/release/build" -path '*/out/emuella_j2k.h' -print)
test "$#" -eq 1
generated_header=$1
cmp "$crate/include/emuella_j2k.h" "$generated_header"

actual_symbols="$CARGO_TARGET_DIR/emuella-j2k-capi-symbols.txt"
nm -D --defined-only "$CARGO_TARGET_DIR/release/libemuella_j2k_capi.so" \
  | awk '{print $3}' \
  | LC_ALL=C sort > "$actual_symbols"
cmp "$crate/exported-symbols.txt" "$actual_symbols"

native_output="$CARGO_TARGET_DIR/emuella-j2k-capi-native"
mkdir -p "$native_output"
for language in c cc; do
  case "$language" in
    c) compiler=${CC:-cc}; standard=-std=c11 ;;
    cc) compiler=${CXX:-c++}; standard=-std=c++17 ;;
  esac
  source="$crate/tests/native/consumer.$language"
  shared="$native_output/consumer-$language-shared"
  static="$native_output/consumer-$language-static"
  "$compiler" "$standard" -Wall -Wextra -Werror -I"$crate/include" "$source" \
    -L"$CARGO_TARGET_DIR/release" -lemuella_j2k_capi \
    -Wl,-rpath,"$CARGO_TARGET_DIR/release" -o "$shared"
  "$shared"
  "$compiler" "$standard" -Wall -Wextra -Werror -I"$crate/include" "$source" \
    "$CARGO_TARGET_DIR/release/libemuella_j2k_capi.a" -ldl -lpthread -lm -o "$static"
  "$static"
done
