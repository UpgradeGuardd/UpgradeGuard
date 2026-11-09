#!/usr/bin/env bash
# Verifies the workspace builds and tests pass.
# Usage: ./scripts/verify.sh
set -euo pipefail

echo "==> Building Rust workspace"
cargo build --workspace

echo "==> Running Rust tests"
cargo test --workspace

echo "==> Done"
