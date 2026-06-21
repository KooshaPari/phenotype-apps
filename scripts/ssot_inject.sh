#!/usr/bin/env bash
# scripts/ssot_inject.sh — minimal stub for the ssot-inject CI gate.
# Per ADR-022, validate-ssot.sh is the authoritative SSOT validator;
# this shim delegates to it. Real injection is staged for a follow-up cycle.
# Usage: scripts/ssot_inject.sh [--dry-run]

set -euo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT"

if [[ "${1:-}" == "--dry-run" ]]; then
    echo "[ssot-inject] dry-run: no changes"
    exit 0
fi

if [[ -x ./scripts/validate-ssot.sh ]]; then
    ./scripts/validate-ssot.sh
else
    echo "[ssot-inject] noop — see scripts/validate-ssot.sh"
fi
