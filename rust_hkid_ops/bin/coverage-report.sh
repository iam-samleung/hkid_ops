#!/bin/bash
set -euo pipefail

# CONFIGURE YOUR APP NAME HERE (must match your Cargo.toml [package] name)
app_name="hkid_ops"

# Tool checks
for tool in rustup cargo llvm-profdata llvm-cov; do
    if ! command -v "$tool" &>/dev/null; then
        echo "Error: $tool is not installed; aborting."
        exit 1
    fi
done

# Enter project root directory (where Cargo.toml is)
base_dir="$(cd "$(dirname "$0")" && pwd)"
cd "${base_dir}/.."

export CARGO_INCREMENTAL=0

# Prepare fresh coverage directory
rm -rf coverage
mkdir -p coverage

# Set up coverage environment
export RUSTFLAGS="-Cinstrument-coverage -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"
export LLVM_PROFILE_FILE="coverage/${app_name}-%p-%m.profraw"

# Print debug info
echo "==== Project root: $(pwd)"
echo "==== Using app_name: $app_name"
echo "==== RUSTFLAGS=$RUSTFLAGS"

echo "==== Cleaning and compiling $app_name ===="
cargo clean
cargo test -p "$app_name" --tests

echo "==== Listing profraw files ===="
ls -lh coverage/*.profraw || { echo "No profraw files found!"; exit 1; }

echo "==== Merging profraw files into coverage.profdata ===="
llvm-profdata merge -sparse coverage/*.profraw -o coverage/coverage.profdata

echo "==== Looking for test binaries in target/debug/deps/ ===="
ls -lh target/debug/deps/ | grep "$app_name" || echo "No test binary named $app_name in target/debug/deps/"

# Find the correct test binary (the one matching app_name, executable, not *.d)
test_bin=$(find ./target/debug/deps/ -maxdepth 1 -type f -perm -111 -name "${app_name}-*" ! -name "*.d" | head -n 1)
if [ ! -f "$test_bin" ]; then
  echo "Test binary not found! Searched for ${app_name}-* in target/debug/deps/"
  exit 1
fi
echo "Using test binary: $test_bin"

# Get absolute project root path for filtering
project_dir=$(pwd | sed 's:/$::')
echo "==== Project dir for coverage: $project_dir"
echo "==== Absolute path to test_bin: $(realpath "$test_bin")"

# Debug: Show all .rs files and their absolute paths
echo "==== Rust source files in project: ===="
find . -name '*.rs' -exec realpath {} \;

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

# Check if the report is likely to be empty (common problem: over-filtering)
total_covered_lines=$(awk -F '[,:]' '/^DA:/ {s+=$3} END {print s}' coverage/lcov.info)
if [[ "$total_covered_lines" == "0" ]]; then
    echo "WARNING: Coverage is ZERO! This usually means your source files were filtered out, or the test binary is not instrumented."
    echo "  1. Try removing --ignore-filename-regex filters except for '/rustc/' and '.cargo/registry'."
    echo "  2. Make sure you are using the correct test binary."
    echo "  3. Make sure your tests are actually running and hitting your code."
    echo "  4. Check that your source files' absolute paths match your filtering rules."
    echo "  5. Check that your code is not inlined away (use -Copt-level=0)."
fi

# Optionally open the report in the browser
if [[ "${1:-}" == "--open" ]]; then
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