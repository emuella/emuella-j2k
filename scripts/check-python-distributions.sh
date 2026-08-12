#!/bin/sh
set -eu

repository_root=$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repository_root"

find_extension_binary() {
  extension_directory=$1
  extension_match=
  extension_count=0
  for extension_candidate in \
    "$extension_directory/libemuella_j2k.so" \
    "$extension_directory/libemuella_j2k.dylib" \
    "$extension_directory/emuella_j2k.dll"; do
    if [ -f "$extension_candidate" ]; then
      extension_match=$extension_candidate
      extension_count=$((extension_count + 1))
    fi
  done
  if [ "$extension_count" -ne 1 ]; then
    echo "expected one compiled extension build output in $extension_directory" >&2
    exit 1
  fi
  printf '%s\n' "$extension_match"
}

evidence_root=${1:-target/release-evidence/python}
mkdir -p "$evidence_root"
evidence_root=$(CDPATH='' cd -- "$evidence_root" && pwd)
dist_directory="$evidence_root/dist"
rebuilt_directory="$evidence_root/rebuilt"
mkdir -p "$dist_directory" "$rebuilt_directory"

{
  python3 --version
  python3 -m pip --version
  maturin --version
  rustc --version
  cargo --version
} >"$evidence_root/tool-versions.txt" 2>&1
cat "$evidence_root/tool-versions.txt"

{
  maturin build \
    --release \
    --locked \
    --manifest-path crates/emuella-j2k-python/Cargo.toml \
    --out "$dist_directory"
  maturin sdist \
    --manifest-path crates/emuella-j2k-python/Cargo.toml \
    --out "$dist_directory"
} >"$evidence_root/build.log" 2>&1 || {
    status=$?
    cat "$evidence_root/build.log" >&2
    exit "$status"
  }
cat "$evidence_root/build.log"

wheel_binary=$(find_extension_binary target/release)
wheel_binary_member=$(python3 -c \
  'import sysconfig; print("emuella_j2k/emuella_j2k" + sysconfig.get_config_var("EXT_SUFFIX"))')

python3 scripts/check-python-distributions.py \
  --dist-dir "$dist_directory" \
  --evidence-dir "$evidence_root" \
  --wheel-binary "$wheel_binary" \
  --wheel-binary-member "$wheel_binary_member"

set -- "$dist_directory"/*.whl
if [ "$#" -ne 1 ] || [ ! -f "$1" ]; then
  echo "expected exactly one wheel in $dist_directory" >&2
  exit 1
fi
wheel=$1

set -- "$dist_directory"/*.tar.gz
if [ "$#" -ne 1 ] || [ ! -f "$1" ]; then
  echo "expected exactly one source distribution in $dist_directory" >&2
  exit 1
fi
sdist=$1

install_environment=$(mktemp -d "$repository_root/target/python-wheel-venv.XXXXXX")
python3 -m venv "$install_environment"
"$install_environment/bin/python" -m pip install \
  --disable-pip-version-check \
  --no-deps \
  --no-index \
  "$wheel" \
  >"$evidence_root/install-wheel.log" 2>&1 || {
    status=$?
    cat "$evidence_root/install-wheel.log" >&2
    exit "$status"
  }
cat "$evidence_root/install-wheel.log"
"$install_environment/bin/python" -c \
  'import importlib.metadata; import emuella_j2k; expected = importlib.metadata.version("emuella-j2k"); assert emuella_j2k.version() == expected; print(emuella_j2k.__file__); print(expected)' \
  >"$evidence_root/import-wheel.log" 2>&1 || {
    status=$?
    cat "$evidence_root/import-wheel.log" >&2
    exit "$status"
  }
cat "$evidence_root/import-wheel.log"

sdist_target_directory=$(mktemp -d \
  "$repository_root/target/python-sdist-target.XXXXXX")
CARGO_TARGET_DIR="$sdist_target_directory" \
  "$install_environment/bin/python" -m pip wheel \
  --disable-pip-version-check \
  --no-deps \
  --wheel-dir "$rebuilt_directory" \
  "$sdist" \
  >"$evidence_root/rebuild-from-sdist.log" 2>&1 || {
    status=$?
    cat "$evidence_root/rebuild-from-sdist.log" >&2
    exit "$status"
  }
cat "$evidence_root/rebuild-from-sdist.log"

rebuilt_wheel_binary=$(find_extension_binary "$sdist_target_directory/release")

python3 scripts/check-python-distributions.py \
  --dist-dir "$dist_directory" \
  --wheel-binary "$wheel_binary" \
  --wheel-binary-member "$wheel_binary_member" \
  --rebuilt-wheel-dir "$rebuilt_directory" \
  --rebuilt-wheel-binary "$rebuilt_wheel_binary" \
  --evidence-dir "$evidence_root"
