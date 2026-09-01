#!/bin/sh
set -eu

smoke_test=ht_lossy_public_tests::lossy_ht_public_smoke
complete_test=ht_lossy_public_tests::lossy_ht_public_complete_matrix

all_tests=$(cargo test --release -p emuella-j2k-core --lib -- --list)
ignored_tests=$(cargo test --release -p emuella-j2k-core --lib -- --ignored --list)

count_test() {
  expected="$1: test"
  tests=$2
  printf '%s\n' "$tests" | awk -v expected="$expected" '
    $0 == expected { count += 1 }
    END { print count + 0 }
  '
}

if [ "$(count_test "$smoke_test" "$all_tests")" -ne 1 ] ||
  [ "$(count_test "$complete_test" "$all_tests")" -ne 1 ]; then
  echo "lossy HT public qualification tests are missing or ambiguous" >&2
  exit 1
fi
if [ "$(count_test "$smoke_test" "$ignored_tests")" -ne 0 ] ||
  [ "$(count_test "$complete_test" "$ignored_tests")" -ne 1 ]; then
  echo "lossy HT public smoke/complete test classification is incorrect" >&2
  exit 1
fi

cargo test --release -p emuella-j2k-core --lib "$complete_test" -- --ignored --exact --nocapture
