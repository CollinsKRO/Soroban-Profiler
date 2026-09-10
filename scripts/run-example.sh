#!/usr/bin/env bash
# Runs the profiler against the bundled example contract and opens the HTML report.
set -euo pipefail

cd "$(dirname "$0")/.."

OUT="${1:-report.html}"

cargo run -p cli -- report --manifest-path examples/token-example --html "$OUT"

echo "==> Report written to $OUT"

if command -v open >/dev/null 2>&1; then
  open "$OUT"
elif command -v xdg-open >/dev/null 2>&1; then
  xdg-open "$OUT"
else
  echo "==> Open $OUT manually to view the report."
fi
