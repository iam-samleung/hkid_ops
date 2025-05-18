#!/bin/bash
set -e

cargo test --tests
$(dirname "$0")/clippy-pedantic.sh
cargo doc --no-deps
cargo deadlinks
cargo bench --no-run --profile dev