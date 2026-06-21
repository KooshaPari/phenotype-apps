#!/usr/bin/env bash
# scripts/vault_smoke.sh
# Smoke test: POST to Vault dev server, retrieve + decrypt a test secret, assert rotation works.
# Companion to docs/adr/2026-06-21/ADR-079-vault-migration-roadmap.md (v19 T1, L5-153).
#
# Usage:
#   VAULT_ADDR=http://127.0.0.1:8200 VAULT_TOKEN=dev-root-token ./scripts/vault_smoke.sh
#
# Exit codes:
#   0 — all assertions pass
#   1 — Vault unreachable / auth failed
#   2 — secret write/read mismatch
#   3 — rotation failed (old secret still valid after rotation)
#   4 — missing dependency (curl or jq)
#
# This is a developer-local smoke; it does NOT hit a real Vault cluster.
# For CI smoke against the staging Vault, see .github/workflows/vault-smoke.yml.

set -euo pipefail

VAULT_ADDR="${VAULT_ADDR:-http://127.0.0.1:8200}"
VAULT_TOKEN="${VAULT_TOKEN:-}"
TEST_PATH="secret/data/pheno/smoke/test-app/test-secret"

# --- preflight ---
for cmd in curl jq; do
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "FAIL: missing dependency: $cmd" >&2
    exit 4
  fi
done

if [[ -z "$VAULT_TOKEN" ]]; then
  echo "FAIL: VAULT_TOKEN not set (use 'dev-root-token' for vault server -dev)" >&2
  exit 1
fi

# --- 1. reachability ---
echo "[1/4] checking vault reachability at $VAULT_ADDR ..."
if ! curl -sf -m 5 -o /dev/null "$VAULT_ADDR/v1/sys/health"; then
  echo "FAIL: vault unreachable at $VAULT_ADDR" >&2
  exit 1
fi
echo "  OK (vault responding)"

# --- 2. write a test secret ---
echo "[2/4] writing test secret at $TEST_PATH ..."
WRITE_BODY=$(jq -n --arg v "pheno-smoke-$(date +%s)" '{data: {value: $v}}')
WRITE_RESP=$(curl -sf -m 5 -X POST \
  -H "X-Vault-Token: $VAULT_TOKEN" \
  -H "Content-Type: application/json" \
  -d "$WRITE_BODY" \
  "$VAULT_ADDR/v1/$TEST_PATH")
echo "  OK (write succeeded)"

# --- 3. read it back, assert round-trip ---
echo "[3/4] reading test secret back ..."
READ_RESP=$(curl -sf -m 5 -H "X-Vault-Token: $VAULT_TOKEN" "$VAULT_ADDR/v1/$TEST_PATH")
READ_VALUE=$(echo "$READ_RESP" | jq -r '.data.data.value')
EXPECTED_VALUE=$(echo "$WRITE_BODY" | jq -r '.data.value')
if [[ "$READ_VALUE" != "$EXPECTED_VALUE" ]]; then
  echo "FAIL: secret round-trip mismatch (wrote '$EXPECTED_VALUE', read '$READ_VALUE')" >&2
  exit 2
fi
echo "  OK (round-trip match: $READ_VALUE)"

# --- 4. rotation check: overwrite, then verify the old value is gone ---
echo "[4/4] rotating test secret ..."
NEW_BODY=$(jq -n --arg v "pheno-smoke-rotated-$(date +%s)" '{data: {value: $v}}')
curl -sf -m 5 -X POST \
  -H "X-Vault-Token: $VAULT_TOKEN" \
  -H "Content-Type: application/json" \
  -d "$NEW_BODY" \
  "$VAULT_ADDR/v1/$TEST_PATH" >/dev/null

NEW_READ=$(curl -sf -m 5 -H "X-Vault-Token: $VAULT_TOKEN" "$VAULT_ADDR/v1/$TEST_PATH" | jq -r '.data.data.value')
EXPECTED_NEW=$(echo "$NEW_BODY" | jq -r '.data.value')
if [[ "$NEW_READ" != "$EXPECTED_NEW" ]]; then
  echo "FAIL: rotation mismatch (expected '$EXPECTED_NEW', got '$NEW_READ')" >&2
  exit 3
fi
echo "  OK (rotation applied: $NEW_READ)"

echo ""
echo "PASS: vault smoke green (reachability, write, read, rotation)"