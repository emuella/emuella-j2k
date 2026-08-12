#!/bin/sh
set -eu

repository_root=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repository_root"

evidence_root=${1:-target/release-evidence/cli}
dist_directory="$evidence_root/dist"
mkdir -p "$dist_directory"

cargo build --release --locked -p emuella-j2k-cli \
  >"$evidence_root/build.log" 2>&1 || {
    status=$?
    cat "$evidence_root/build.log" >&2
    exit "$status"
  }
cat "$evidence_root/build.log"

version=$(python3 -c 'import tomllib; print(tomllib.load(open("Cargo.toml", "rb"))["workspace"]["package"]["version"])')
target=$(rustc -vV | sed -n 's/^host: //p')
bundle_name="emuella-j2k-$version-$target"
staging_root=$(mktemp -d "$repository_root/target/cli-distribution.XXXXXX")
bundle_root="$staging_root/$bundle_name"
mkdir -p "$bundle_root"

cp target/release/emuella-j2k "$bundle_root/emuella-j2k"
cp LICENSE "$bundle_root/LICENSE-APACHE-2.0"
cp LICENSES/OpenJPH-BSD-2-Clause.txt \
  "$bundle_root/LICENSE-OPENJPH-BSD-2-CLAUSE"
cp NOTICE "$bundle_root/NOTICE"
cp THIRD_PARTY.md "$bundle_root/THIRD_PARTY.md"
cp crates/emuella-j2k-cli/THIRD_PARTY_DEPENDENCIES.md \
  "$bundle_root/THIRD_PARTY_DEPENDENCIES.md"
if [ -d crates/emuella-j2k-cli/THIRD_PARTY_LICENSES ]; then
  cp -R crates/emuella-j2k-cli/THIRD_PARTY_LICENSES \
    "$bundle_root/THIRD_PARTY_LICENSES"
fi

source_date_epoch=$(git log -1 --format=%ct)
uncompressed_archive="$staging_root/$bundle_name.tar"
archive="$dist_directory/$bundle_name.tar.gz"
tar \
  --sort=name \
  --mtime="@$source_date_epoch" \
  --owner=0 \
  --group=0 \
  --numeric-owner \
  -cf "$uncompressed_archive" \
  -C "$staging_root" \
  "$bundle_name"
gzip -n -c "$uncompressed_archive" >"$archive"

python3 scripts/check-cli-distribution.py \
  --archive "$archive" \
  --expected-binary target/release/emuella-j2k \
  --evidence-dir "$evidence_root"
