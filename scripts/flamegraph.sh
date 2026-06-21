#!/usr/bin/env bash
# scripts/flamegraph.sh — v20 T2 (L44 flamegraph-driven perf deep-dives)
#
# Wraps `cargo flamegraph` so CI can run it deterministically.
# Produces an SVG at target/flamegraph.svg (per cargo-flamegraph default).
#
# Usage:
#   scripts/flamegraph.sh --bin <name> [cargo-flamegraph-flags...]
#   scripts/flamegraph.sh --example <name> [cargo-flamegraph-flags...]
#   scripts/flamegraph.sh --bench <name> [cargo-flamegraph-flags...]
#   scripts/flamegraph.sh --test <name> [cargo-flamegraph-flags...]
#
# The first non-flag arg is the target; everything after is forwarded to
# `cargo flamegraph`. Common forwarded flags:
#   --features <list>   Enable cargo features
#   --no-default-features
#   --release           Profile (default: release)
#   --root             Run as root (CI/Linux perf)
#   --output <path>    Override SVG output path
#   -- <args>          Args passed to the binary
#
# Environment:
#   FLAMEGRAPH_OUTPUT  Default output path (overrides target/flamegraph.svg)
#   FLAMEGRAPH_BIN     Override cargo-flamegraph binary path
#   CARGO_TARGET_DIR   Standard cargo target dir (default: target)
#   CARGO              Override cargo invocation
#
# Authority: ADR-040 (coverage gates per tier) + v20 71-pillar cycle 10 plan §3 Track T2.

set -euo pipefail

usage() {
    cat <<'USAGE' >&2
usage:
  scripts/flamegraph.sh --bin <name>      [cargo-flamegraph-flags...]
  scripts/flamegraph.sh --example <name>  [cargo-flamegraph-flags...]
  scripts/flamegraph.sh --bench <name>    [cargo-flamegraph-flags...]
  scripts/flamegraph.sh --test <name>     [cargo-flamegraph-flags...]
  scripts/flamegraph.sh --help
USAGE
    exit 64
}

if [ $# -lt 1 ]; then usage; fi

# --- Parse target kind ---
case "$1" in
    --bin|--example|--bench|--test)
        target_kind="${1#--}"
        shift
        ;;
    --help|-h)
        usage
        ;;
    *)
        echo "flamegraph.sh: first arg must be --bin | --example | --bench | --test (got: $1)" >&2
        usage
        ;;
esac

if [ $# -lt 1 ]; then
    echo "flamegraph.sh: missing <name> after --${target_kind}" >&2
    usage
fi
target_name="$1"
shift

# --- Tool detection ---
CARGO_BIN="${CARGO:-cargo}"
if [ "${1:-}" != "--" ]; then
    if ! command -v cargo-flamegraph >/dev/null 2>&1; then
        echo "flamegraph.sh: cargo-flamegraph not found on PATH" >&2
        echo "  install with: cargo install flamegraph" >&2
        exit 127
    fi
    FLAMEGRAPH_BIN="${FLAMEGRAPH_BIN:-cargo-flamegraph}"
fi

# --- Output path ---
OUTPUT="${FLAMEGRAPH_OUTPUT:-}"
if [ -z "$OUTPUT" ]; then
    target_dir="${CARGO_TARGET_DIR:-target}"
    OUTPUT="${target_dir}/flamegraph.svg"
fi
mkdir -p "$(dirname "$OUTPUT")"

# --- Platform notes ---
# Linux: uses perf (kernel.perf_event_paranoid must be <=1; pass --root if running as root in CI).
# macOS: uses DTrace; requires sudo and SIP-disabled (developer-signed); CI uses Linux.
uname_s="$(uname -s)"
case "$uname_s" in
    Linux)   platform="linux" ;;
    Darwin)  platform="macos" ;;
    *)       platform="other" ;;
esac

if [ "$platform" = "linux" ]; then
    if [ "$(id -u)" -eq 0 ] && ! echo "$*" | grep -q -- "--root"; then
        echo "flamegraph.sh: running as root on Linux; forwarding --root to cargo-flamegraph" >&2
        set -- --root "$@"
    fi
fi

echo "flamegraph.sh: target=${target_kind}=${target_name} platform=${platform} output=${OUTPUT}" >&2

# --- Compose command ---
# We invoke cargo-flamegraph directly; it wraps cargo. We pass --output last to be sure
# it wins over any forwarded --output from the caller.
set -- --output "${OUTPUT}" "$@"

# shellcheck disable=SC2086
exec "$FLAMEGRAPH_BIN" "--${target_kind}" "${target_name}" "$@"
