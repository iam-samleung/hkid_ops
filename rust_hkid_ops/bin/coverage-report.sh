#!/bin/bash
set -e

# Tool checks
type rustup >/dev/null 2>&1 || { echo >&2 "rustup is not installed; aborting."; exit 1; }
type cargo >/dev/null 2>&1 || { echo >&2 "cargo is not installed; aborting."; exit 1; }
type llvm-profdata >/dev/null 2>&1 || { echo >&2 "llvm-profdata is not installed; aborting."; exit 1; }
type llvm-cov >/dev/null 2>&1 || { echo >&2 "llvm-cov is not installed; aborting."; exit 1; }

# Enter project root directory
base_dir="$(cd "$(dirname "$0")" && pwd)"
cd "${base_dir}/.."

app_name=hkid_ops
export CARGO_INCREMENTAL="0"

# Prepare fresh coverage directory
rm -rf coverage 2>/dev/null || true
mkdir coverage

# Set up coverage environment
export RUSTFLAGS="-Cinstrument-coverage -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"
export LLVM_PROFILE_FILE="coverage/${app_name}-%p-%m.profraw"

echo "==== Compiling $app_name ===="
cargo clean
cargo build -p "$app_name"

echo "==== Running tests for $app_name ===="
cargo test -p "$app_name" --tests

echo "==== Listing profraw files ===="
ls -lh coverage/*.profraw || { echo "No profraw files found!"; exit 1; }

echo "==== Merging profraw files into coverage.profdata ===="
llvm-profdata merge -sparse coverage/*.profraw -o coverage/coverage.profdata

echo "==== Looking for test binaries in target/debug/deps/ ===="
ls -lh target/debug/deps/ | grep "$app_name"

# Find the correct test binary (usually the newest, not a .d file)
test_bin=$(find ./target/debug/deps/ -type f -perm -111 -name "${app_name}-*" ! -name "*.d" | head -n 1)
if [ ! -f "$test_bin" ]; then
  echo "Test binary not found!"
  exit 1
fi
echo "Using test binary: $test_bin"

# Get absolute project root path for filtering
project_dir=$(pwd | sed 's:/$::')

echo "==== Generating HTML coverage report ===="
llvm-cov show \
  "$test_bin" \
  --instr-profile=coverage/coverage.profdata \
  --ignore-filename-regex='/rustc/' \
  --ignore-filename-regex='/.cargo/registry' \
  --ignore-filename-regex='^/private/tmp/' \
  --ignore-filename-regex="^(?!${project_dir}).*" \
  --format=html \
  --output-dir=coverage

echo "==== Generating LCOV coverage report ===="
llvm-cov export \
  "$test_bin" \
  --instr-profile=coverage/coverage.profdata \
  --ignore-filename-regex='/rustc/' \
  --ignore-filename-regex='/.cargo/registry' \
  --ignore-filename-regex='^/private/tmp/' \
  --ignore-filename-regex="^(?!${project_dir}).*" \
  --format=lcov > coverage/lcov.info

# Clean up temporary profraw files
rm -f coverage/*.profraw

echo "==== Coverage HTML report is at: coverage/index.html ===="
echo "==== LCOV report is at: coverage/lcov.info ===="

# Optionally open the report in the browser
if [ "$1" == "--open" ]; then
  index="file://$(pwd)/coverage/index.html"
  if command -v xdg-open &>/dev/null; then
    xdg-open "$index"
  elif command -v open &>/dev/null; then
    open "$index"
  else
    echo >&2 "neither xdg-open nor open are installed"
    exit 1
  fi
fi