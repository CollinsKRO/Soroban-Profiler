#!/usr/bin/env bash
# Builds and installs soroban-cost-cli from the local checkout.
set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> Building workspace"
cargo build --workspace --release

echo "==> Installing soroban-cost-cli"
cargo install --path cli --force

echo "==> Done. Run: soroban-cost-cli report --manifest-path <your-contract>"
