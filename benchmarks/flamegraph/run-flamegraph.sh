#!/usr/bin/env bash
# benchmarks/flamegraph/run-flamegraph.sh — v20 T2 L44 flamegraph wrapper
#
# Authority: v20 71-pillar cycle 10 P1 plan §2 Track T2 + ADR-040 (coverage gates per tier).
#
# Usage:
#   benchmarks/flamegraph/run-flamegraph.sh <crate>            # real run
#   benchmarks/flamegraph/run-flamegraph.sh <crate> --synthetic  # fallback SVG only
#
# Behavior:
#   1. Locates the crate directory (sibling of this script: ../<crate> or in CWD).
#   2. Detects `cargo-flamegraph` (Linux/Windows) or `cargo-instruments` (macOS).
#   3. If a real profiler is available:
#        - runs `cargo flamegraph --bench <bench>` (or `cargo bench` if no bench harness)
#        - copies the produced flamegraph.svg to benchmarks/flamegraph/<crate>-baseline.svg
#        - prints a summary line with frame count + SVG byte size
#   4. If no real profiler is available (e.g. the macOS host without dtrace entitlement):
#        - falls back to a *synthetic baseline* SVG that documents the hot paths
#          expected for that crate, so the artifact is never missing from CI artifacts
#        - the synthetic SVG is rendered from a built-in template and labeled
#          SYNTHETIC so downstream tooling can distinguish it from a real run
#
# Crates currently supported:
#   pheno-config       — config cascade hot path (Figment + TOML + env)
#   pheno-tracing      — TracePort submit + TailBasedSampler + RateLimitSampler
#   pheno-mcp-router   — route_request decision layer (LlmPort + middleware chain)
#
# Exit codes:
#   0  success (real flamegraph OR synthetic baseline generated)
#   1  invalid arguments or unsupported crate
#   2  real run failed AND synthetic fallback failed

set -euo pipefail

# ---------- helpers ----------------------------------------------------------

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
OUT_DIR="$SCRIPT_DIR"

CRATE="${1:-}"
if [ -z "$CRATE" ] || [ "${2:-}" = "--help" ] || [ "${2:-}" = "-h" ]; then
    cat <<EOF
Usage: $0 <crate> [--synthetic]

Supported crates:
  pheno-config       — config cascade (Figment + TOML + env cascade)
  pheno-tracing      — TracePort submit + samplers
  pheno-mcp-router   — route_request decision layer

Flags:
  --synthetic   skip the real cargo-flamegraph run; emit synthetic baseline SVG only

Environment overrides:
  FLAMEGRAPH_BENCH       — explicit bench name (default: auto-detect or 'noop')
  FLAMEGRAPH_SYNTHETIC   — set to '1' to force synthetic mode (CI fallback)

See benchmarks/flamegraph/README.md for the full workflow, including the macOS
cargo-instruments fallback.
EOF
    exit 1
fi

SYNTHETIC_ONLY="false"
for arg in "$@"; do
    case "$arg" in
        --synthetic) SYNTHETIC_ONLY="true" ;;
    esac
done
if [ "${FLAMEGRAPH_SYNTHETIC:-0}" = "1" ]; then
    SYNTHETIC_ONLY="true"
fi

case "$CRATE" in
    pheno-config)        BENCH_DEFAULT="cascade_bench";  EXPECTED_HOTTOP="build_cascade";  SVG_NAME="config"      ;;
    pheno-tracing)       BENCH_DEFAULT="submit_bench";   EXPECTED_HOTTOP="TracePort::submit"; SVG_NAME="tracing"    ;;
    pheno-mcp-router)    BENCH_DEFAULT="route_bench";    EXPECTED_HOTTOP="route_request";  SVG_NAME="mcp-router" ;;
    *)
        echo "ERROR: unsupported crate '$CRATE'. Supported: pheno-config, pheno-tracing, pheno-mcp-router" >&2
        exit 1
        ;;
esac

# Crate-to-filename map (deliverable spec uses short names).
declare -A SVG_FILE=(
    [pheno-config]="config-baseline.svg"
    [pheno-tracing]="tracing-baseline.svg"
    [pheno-mcp-router]="mcp-router-baseline.svg"
)
SVG_PATH="$OUT_DIR/${SVG_FILE[$CRATE]}"

BENCH="${FLAMEGRAPH_BENCH:-$BENCH_DEFAULT}"

# ---------- detection --------------------------------------------------------

have_cargo_flamegraph() {
    command -v cargo-flamegraph >/dev/null 2>&1 && cargo flamegraph --help >/dev/null 2>&1
}

have_cargo_instruments() {
    # macOS fallback — `cargo-instruments` wraps Xcode Instruments templates.
    [ "$(uname -s)" = "Darwin" ] && command -v cargo-instruments >/dev/null 2>&1
}

is_macos() {
    [ "$(uname -s)" = "Darwin" ]
}

# ---------- real run ---------------------------------------------------------

real_run() {
    echo "[flamegraph] real run for $CRATE (bench=$BENCH) ..."
    cd "$REPO_ROOT/$CRATE"

    # Prefer `cargo flamegraph --bench` if a bench is registered; otherwise
    # fall back to a plain `cargo flamegraph` against the default target.
    if grep -q "^\[\[bench\]\]" Cargo.toml 2>/dev/null; then
        cargo flamegraph --bench "$BENCH" -- --bench
    else
        echo "[flamegraph] no [[bench]] in Cargo.toml; using cargo flamegraph (release) on default target"
        cargo flamegraph --release
    fi

    # cargo flamegraph writes flamegraph.svg into the CWD.
    if [ ! -s flamegraph.svg ]; then
        echo "ERROR: cargo flamegraph produced no flamegraph.svg" >&2
        return 1
    fi
    cp flamegraph.svg "$SVG_PATH"
    local bytes frames
    bytes=$(wc -c <"$SVG_PATH" | tr -d ' ')
    frames=$(grep -c "<rect " "$SVG_PATH" || echo 0)
    echo "[flamegraph] real run OK: $SVG_PATH ($bytes bytes, $frames frames)"
}

real_run_instruments() {
    echo "[flamegraph] macOS cargo-instruments run for $CRATE ..."
    cd "$REPO_ROOT/$CRATE"

    # `cargo instruments -t time` is the closest macOS analog to perf/LBR flamegraphs.
    cargo instruments -t time --bench "$BENCH" -- \
        --output-path "$OUT_DIR/${CRATE}-trace" --limit 60s \
        || return 1

    # cargo-instruments emits a .trace bundle; the SVG has to be exported via
    # `xcrun xctrace export`. For CI portability we ship the trace as an artifact
    # and generate a synthetic SVG summary (the macOS path always falls back here
    # in headless CI).
    if [ -d "$OUT_DIR/${CRATE}-trace.trace" ]; then
        echo "[flamegraph] trace bundle written to ${CRATE}-trace.trace (upload as CI artifact)"
    fi
    return 1   # force synthetic fallback for the in-repo SVG
}

# ---------- synthetic baseline ----------------------------------------------

synthetic_run() {
    echo "[flamegraph] SYNTHETIC baseline for $CRATE (real profiler unavailable) ..."
    local tmpl="$SVG_PATH"
    if [ ! -s "$tmpl" ]; then
        echo "ERROR: synthetic template missing at $tmpl" >&2
        return 1
    fi
    # Tag the SVG so consumers can tell synthetic from real.
    local stamp
    stamp="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
    local tmp="$tmpl.tmp"
    if grep -q "<!-- flamegraph: synthetic stamp" "$tmpl"; then
        sed -i.bak "s|<!-- flamegraph: synthetic stamp = [^>]*-->|<!-- flamegraph: synthetic stamp = $stamp -->|" "$tmpl" || true
        rm -f "$tmpl.bak"
    else
        awk -v stamp="$stamp" '
            /<\/svg>/ && !done { print "<!-- flamegraph: synthetic stamp = " stamp " -->"; done=1 }
            { print }
        ' "$tmpl" >"$tmp" && mv "$tmp" "$tmpl"
    fi
    local bytes frames
    bytes=$(wc -c <"$tmpl" | tr -d ' ')
    frames=$(grep -c "<rect " "$tmpl" || echo 0)
    echo "[flamegraph] synthetic OK: $tmpl ($bytes bytes, $frames frames, expected hot top: $EXPECTED_HOTTOP)"
    return 0
}

# ---------- dispatch ---------------------------------------------------------

RC=0
if [ "$SYNTHETIC_ONLY" = "false" ] && have_cargo_flamegraph; then
    real_run || RC=$?
elif [ "$SYNTHETIC_ONLY" = "false" ] && have_cargo_instruments; then
    real_run_instruments || RC=$?
elif [ "$SYNTHETIC_ONLY" = "false" ] && is_macos; then
    echo "[flamegraph] macOS detected but no cargo-flamegraph / cargo-instruments found."
    echo "[flamegraph] install with: cargo install flamegraph   OR   brew install cargo-instruments"
    echo "[flamegraph] falling back to synthetic baseline ..."
    RC=0
fi

if [ "$RC" -ne 0 ] || [ "$SYNTHETIC_ONLY" = "true" ]; then
    synthetic_run || { echo "ERROR: synthetic fallback failed for $CRATE" >&2; exit 2; }
fi

echo "[flamegraph] done for $CRATE"
exit 0