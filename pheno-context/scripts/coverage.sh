#!/usr/bin/env bash
# scripts/coverage.sh — cargo-llvm-cov coverage gate for pheno-context
# Per ADR-040: lib tier = 80% lines / 75% branches / 80% fns.
# Mirrors pheno-otel/scripts/coverage.sh (the reference impl).
#
# Usage:
#   ./scripts/coverage.sh           # HTML report at coverage/index.html
#   ./scripts/coverage.sh --summary # text summary only
#   ./scripts/coverage.sh --fail    # exit non-zero if under threshold
#
# Requires: cargo-llvm-cov (cargo install cargo-llvm-cov)

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"

# Sanity check: cargo-llvm-cov installed
if ! command -v cargo-llvm-cov >/dev/null 2>&1; then
    echo "ERROR: cargo-llvm-cov not found." >&2
    echo "Install with: cargo install cargo-llvm-cov" >&2
    exit 127
fi

MODE="${1:-html}"

case "$MODE" in
    --summary|summary)
        cargo llvm-cov --summary-only --all-features
        ;;
    --fail)
        cargo llvm-cov --summary-only --all-features --fail-under-lines 80
        ;;
    html|"")
        cargo llvm-cov --all-features --html --output-dir coverage
        echo "HTML report: coverage/index.html"
        ;;
    *)
        echo "Unknown mode: $MODE" >&2
        echo "Usage: $0 [html|--summary|--fail]" >&2
        exit 64
        ;;
esac
