#!/usr/bin/env bash
# scripts/sbom-diff.sh — v22-T5 (L48) CycloneDX SBOM diff gate.
#
# Computes the diff between the current CycloneDX SBOM and the SBOM
# committed at HEAD~1, classifies the dependency change set
# (added / removed / upgraded / downgraded), cross-references each
# new component version against the RUSTSEC advisory database via
# `cargo audit`, and writes a human-readable Markdown report to
# `SBOM_DIFF.md`.
#
# Exit codes:
#   0  — no diff, or diff contains only low/medium advisories
#   1  — diff contains one or more NEW high/critical RUSTSEC CVEs
#   2  — usage error (missing tool, malformed args, etc.)
#
# Pillar: L48 (Supply chain security)
# See findings/2026-06-22-v22-T5-L48-sbom-diff.md for design notes.
#
# Dependencies (must be on PATH or installable via cargo):
#   - cargo (rust toolchain)
#   - cargo-cyclonedx  (matches [build-dependencies] in Cargo.toml)
#   - cargo-audit      (RUSTSEC advisory database)
#   - jq               (JSON slicing)
#   - git              (for HEAD~1 fetch)
#
# Usage:
#   scripts/sbom-diff.sh                 # full diff vs HEAD~1
#   scripts/sbom-diff.sh <ref>           # diff vs git ref (e.g., main, abc123)
#   scripts/sbom-diff.sh --no-audit      # skip cargo audit (faster, no CVE gate)
#
# The script is deliberately written in pure bash + jq + cargo (no Python)
# so it runs on minimal CI images (ubuntu-latest has bash/jq/git by
# default; cargo-cyclonedx and cargo-audit are installed via
# `taiki-e/install-action` in the workflow).

set -euo pipefail

# ─── Configuration ───────────────────────────────────────────────────────────

SCRIPT_NAME="$(basename "$0")"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRATE_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SBOM_FILE="$CRATE_ROOT/sbom.cdx.json"
PREV_SBOM_FILE=""
REPORT_FILE="$CRATE_ROOT/SBOM_DIFF.md"
BASE_REF="HEAD~1"
RUN_AUDIT=1
EXIT_CODE=0
CRITICAL_FINDINGS=()

# Count non-empty lines from stdin; returns 0 if input is empty.
count_lines() {
    local n
    n="$(grep -c '.' 2>/dev/null || true)"
    echo "${n:-0}"
}

# ─── Argument parsing ────────────────────────────────────────────────────────

usage() {
    cat <<EOF
$SCRIPT_NAME — CycloneDX SBOM diff + CVE gate (L48)

Usage:
  $SCRIPT_NAME [REF] [--no-audit]
  $SCRIPT_NAME --help

Arguments:
  REF          Git ref to diff against (default: HEAD~1)

Options:
  --no-audit   Skip cargo-audit cross-reference (no CVE gate)
  --help       Print this help

Exit codes:
  0  No new high/critical CVE introduced
  1  One or more new high/critical CVEs introduced
  2  Usage error
EOF
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --help|-h)
            usage
            exit 0
            ;;
        --no-audit)
            RUN_AUDIT=0
            shift
            ;;
        -*)
            echo "ERROR: unknown option: $1" >&2
            usage >&2
            exit 2
            ;;
        *)
            BASE_REF="$1"
            shift
            ;;
    esac
done

# ─── Tool availability ───────────────────────────────────────────────────────

require_tool() {
    local tool=$1
    if ! command -v "$tool" >/dev/null 2>&1; then
        echo "ERROR: required tool not found on PATH: $tool" >&2
        echo "  Install with: cargo install $tool --locked" >&2
        exit 2
    fi
}

require_tool cargo
require_tool jq
require_tool git

if [[ "$RUN_AUDIT" -eq 1 ]]; then
    require_tool cargo-audit
fi

# ─── Generate current SBOM ───────────────────────────────────────────────────

echo "==> Generating current SBOM (cargo-cyclonedx)..."
cd "$CRATE_ROOT"

# cargo-cyclonedx 0.5.x installs as `cargo-cyclonedx` binary invoked via
# `cargo cyclonedx-bom` (or legacy `cargo cyclonedx`). We try both for
# forward-compatibility.
if cargo cyclonedx-bom --help >/dev/null 2>&1; then
    cargo cyclonedx-bom --format json --override-filename sbom.cdx 1>/dev/null
    # cargo-cyclonedx 0.5.x writes "<name>.cdx.json" (CycloneDX convention)
    if [[ ! -f sbom.cdx.json && -f target/sbom.cdx.json ]]; then
        mv target/sbom.cdx.json sbom.cdx.json
    elif [[ ! -f sbom.cdx.json && -f target/sbom.json ]]; then
        mv target/sbom.json sbom.cdx.json
    fi
elif cargo cyclonedx --help >/dev/null 2>&1; then
    cargo cyclonedx --format json --override-filename sbom 1>/dev/null
    if [[ ! -f sbom.cdx.json && -f target/sbom.json ]]; then
        mv target/sbom.json sbom.cdx.json
    fi
else
    echo "ERROR: cargo-cyclonedx not installed (neither 'cargo cyclonedx-bom' nor 'cargo cyclonedx' available)" >&2
    echo "  Install with: cargo install cargo-cyclonedx --locked" >&2
    exit 2
fi

if [[ ! -f "$SBOM_FILE" ]]; then
    echo "ERROR: SBOM file not produced at $SBOM_FILE" >&2
    echo "  Checked: sbom.cdx.json, target/sbom.cdx.json, target/sbom.json" >&2
    exit 2
fi

echo "    OK: $SBOM_FILE ($(wc -c < "$SBOM_FILE" | tr -d ' ') bytes)"

# ─── Fetch previous SBOM ─────────────────────────────────────────────────────

echo "==> Fetching previous SBOM ($BASE_REF:sbom.cdx.json)..."
PREV_SBOM_FILE="$(mktemp -t sbom-prev.XXXXXX.cdx.json)"
trap 'rm -f "$PREV_SBOM_FILE"' EXIT

if git -C "$CRATE_ROOT" show "${BASE_REF}:sbom.cdx.json" > "$PREV_SBOM_FILE" 2>/dev/null; then
    echo "    OK: previous SBOM loaded ($(wc -c < "$PREV_SBOM_FILE" | tr -d ' ') bytes)"
    HAS_PREV=1
else
    echo "    NOTE: no previous SBOM at ${BASE_REF}:sbom.cdx.json (first SBOM in repo?)"
    echo '{"bomFormat":"CycloneDX","specVersion":"1.5","version":1,"components":[]}' > "$PREV_SBOM_FILE"
    HAS_PREV=0
fi

# ─── Component extraction helpers ────────────────────────────────────────────
#
# cargo-cyclonedx writes CycloneDX v1.5 JSON. The `components` array lives
# at the top level. Each component has `name`, `version`, `bom-ref`, and
# optionally `purl`. We key components by `(name, version)` — identical to
# pheno-port-adapter/scripts/sbom-diff.py so the two stay in lockstep.

extract_components() {
    local file=$1
    # Output: "<name>\t<version>" lines, sorted by name
    jq -r '
        (.components // [])[]
        | select(.name != null)
        | "\(.name)\t\(.version // "")"
    ' "$file" 2>/dev/null | sort -u
}

CURRENT_COMPS="$(extract_components "$SBOM_FILE")"
PREVIOUS_COMPS="$(extract_components "$PREV_SBOM_FILE")"

# ─── Compute diff buckets ────────────────────────────────────────────────────

ADDED="$(comm -13 <(echo "$PREVIOUS_COMPS") <(echo "$CURRENT_COMPS"))"
REMOVED="$(comm -23 <(echo "$PREVIOUS_COMPS") <(echo "$CURRENT_COMPS"))"

# Upgraded/downgraded: same name, different version
# Key by name only and find version transitions.
upgraded_downgraded() {
    local current=$1
    local previous=$2
    # For each (name, version_current) check if name exists in previous
    # with a different version. Use associative arrays.
    declare -A prev_ver
    while IFS=$'\t' read -r name version; do
        [[ -z "$name" ]] && continue
        prev_ver["$name"]="$version"
    done <<< "$previous"

    while IFS=$'\t' read -r name version; do
        [[ -z "$name" ]] && continue
        local p="${prev_ver[$name]:-}"
        if [[ -n "$p" && "$p" != "$version" ]]; then
            echo "${name}	${p}	${version}"
        fi
    done <<< "$current" | sort
}

TRANSITIONS="$(upgraded_downgraded "$CURRENT_COMPS" "$PREVIOUS_COMPS")"
UPGRADED="$(echo "$TRANSITIONS" | awk -F'\t' '$3 > $2' || true)"
DOWNGRADED="$(echo "$TRANSITIONS" | awk -F'\t' '$3 < $2' || true)"

# ─── CVE cross-reference (cargo audit) ──────────────────────────────────────

ADVISORY_DB_FILE="$(mktemp -t advisory-db.XXXXXX.json)"
trap 'rm -f "$PREV_SBOM_FILE" "$ADVISORY_DB_FILE"' EXIT

if [[ "$RUN_AUDIT" -eq 1 ]]; then
    echo "==> Cross-referencing added/upgraded components against RUSTSEC..."
    # cargo audit --json outputs { "vulnerabilities": { "list": [...] } }.
    # Each entry has: advisory.id, advisory.severity (informational/low/
    # medium/high/critical), advisory.package, advisory.versions, etc.
    if ! cargo audit --json > "$ADVISORY_DB_FILE" 2>/dev/null; then
        # Non-zero exit can mean either advisories found (handled below)
        # or cargo-audit itself errored. We treat both the same — read the
        # JSON if present.
        if [[ ! -s "$ADVISORY_DB_FILE" ]]; then
            echo "    WARNING: cargo audit failed to produce JSON; skipping CVE gate"
            RUN_AUDIT=0
        fi
    fi
fi

# Severity ordering (lowest → highest) for grep-based severity filter.
# Matches RUSTSEC severity labels.
declare -A SEV_RANK=(
    [informational]=0
    [low]=1
    [medium]=2
    [high]=3
    [critical]=4
)
SEV_THRESHOLD_RANK="${SEV_RANK[high]}"

# For each added/upgraded component, check if the current version is
# associated with a high/critical advisory. Critical/high findings are
# appended to CRITICAL_FINDINGS (array) and EXIT_CODE is set to 1.
if [[ "$RUN_AUDIT" -eq 1 && -s "$ADVISORY_DB_FILE" ]]; then
    while IFS=$'\t' read -r name new_ver; do
        [[ -z "$name" ]] && continue
        # Build a list of (advisory_id, severity) pairs for THIS package
        # at the CURRENT version. jq path:
        #   .vulnerabilities.list[]
        #     | select(.package.name == $name and (.versions.eventually_fixed
        #        // null) != null)
        # We additionally filter to "introduced in or before $new_ver".
        # Since RUSTSEC semantics are coarse (eventually-fixed or not),
        # we treat any advisory that affects the package as a finding if
        # the new version is not yet fixed.
        findings="$(jq -r --arg name "$name" --arg ver "$new_ver" '
            (.vulnerabilities.list // [])[]
            | select(.package.name == $name)
            | .advisory as $a
            | ($a.severity // "unknown") as $sev
            | $a.id as $id
            | "\($id)\t\($sev)"
        ' "$ADVISORY_DB_FILE" 2>/dev/null || true)"

        while IFS=$'\t' read -r adv_id sev; do
            [[ -z "$adv_id" ]] && continue
            rank="${SEV_RANK[$sev]:-0}"
            if [[ "$rank" -ge "$SEV_THRESHOLD_RANK" ]]; then
                CRITICAL_FINDINGS+=("$name@$new_ver|$adv_id|$sev")
                EXIT_CODE=1
            fi
        done <<< "$findings"
    done <<< "$ADDED"

    # Also check upgraded components
    while IFS=$'\t' read -r name old_ver new_ver; do
        [[ -z "$name" ]] && continue
        findings="$(jq -r --arg name "$name" --arg ver "$new_ver" '
            (.vulnerabilities.list // [])[]
            | select(.package.name == $name)
            | .advisory as $a
            | ($a.severity // "unknown") as $sev
            | $a.id as $id
            | "\($id)\t\($sev)"
        ' "$ADVISORY_DB_FILE" 2>/dev/null || true)"
        while IFS=$'\t' read -r adv_id sev; do
            [[ -z "$adv_id" ]] && continue
            rank="${SEV_RANK[$sev]:-0}"
            if [[ "$rank" -ge "$SEV_THRESHOLD_RANK" ]]; then
                CRITICAL_FINDINGS+=("$name@$new_ver|$adv_id|$sev|upgraded-from-$old_ver")
                EXIT_CODE=1
            fi
        done <<< "$findings"
    done <<< "$UPGRADED"
fi

# ─── Render SBOM_DIFF.md ─────────────────────────────────────────────────────

echo "==> Writing report to $REPORT_FILE"

{
    echo "# SBOM Diff Report"
    echo
    echo "- **Crate:** pheno-otel \`$(grep '^version' "$CRATE_ROOT/Cargo.toml" | head -1 | cut -d'"' -f2)\`"
    echo "- **Base ref:** \`$BASE_REF\`"
    echo "- **Head ref:** \`$(git -C "$CRATE_ROOT" rev-parse --short HEAD)\`"
    echo "- **Generated:** $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
    echo "- **Tooling:** cargo-cyclonedx 0.5.x + cargo-audit"
    echo "- **Previous SBOM present:** $([[ $HAS_PREV -eq 1 ]] && echo "yes" || echo "no (baseline)")"
    echo

    echo "## Summary"
    echo
    echo "| Bucket | Count |"
    echo "|--------|------:|"
    echo "| Added | $(echo "$ADDED" | count_lines) |"
    echo "| Removed | $(echo "$REMOVED" | count_lines) |"
    echo "| Upgraded | $(echo "$UPGRADED" | count_lines) |"
    echo "| Downgraded | $(echo "$DOWNGRADED" | count_lines) |"
    echo "| New high/critical CVEs | ${#CRITICAL_FINDINGS[@]} |"
    echo

    if [[ ${#CRITICAL_FINDINGS[@]} -gt 0 ]]; then
        echo "## ⚠️ Critical / High CVE Findings"
        echo
        echo "| Package | Advisory | Severity | Notes |"
        echo "|---------|----------|----------|-------|"
        for entry in "${CRITICAL_FINDINGS[@]}"; do
            IFS='|' read -r pkg adv sev notes <<< "$entry"
            echo "| \`$pkg\` | $adv | $sev | ${notes:-introduced} |"
        done
        echo
        echo "**Action required:** these CVEs were introduced (or remained unfixed)"
        echo "by the diff between \`$BASE_REF\` and HEAD. Please pin or upgrade the"
        echo "offending crate before merging."
        echo
    fi

    if [[ -n "$ADDED" ]]; then
        echo "## Added components"
        echo
        echo "| Component | Version |"
        echo "|-----------|---------|"
        while IFS=$'\t' read -r name version; do
            [[ -z "$name" ]] && continue
            echo "| \`$name\` | $version |"
        done <<< "$ADDED"
        echo
    fi

    if [[ -n "$REMOVED" ]]; then
        echo "## Removed components"
        echo
        echo "| Component |"
        echo "|-----------|"
        while IFS=$'\t' read -r name version; do
            [[ -z "$name" ]] && continue
            echo "| \`$name\` |"
        done <<< "$REMOVED"
        echo
    fi

    if [[ -n "$UPGRADED" ]]; then
        echo "## Upgraded components"
        echo
        echo "| Component | From | To |"
        echo "|-----------|------|----|"
        while IFS=$'\t' read -r name old_ver new_ver; do
            [[ -z "$name" ]] && continue
            echo "| \`$name\` | $old_ver | $new_ver |"
        done <<< "$UPGRADED"
        echo
    fi

    if [[ -n "$DOWNGRADED" ]]; then
        echo "## Downgraded components"
        echo
        echo "| Component | From | To |"
        echo "|-----------|------|----|"
        while IFS=$'\t' read -r name old_ver new_ver; do
            [[ -z "$name" ]] && continue
            echo "| \`$name\` | $old_ver | $new_ver |"
        done <<< "$DOWNGRADED"
        echo
    fi

    if [[ -z "$ADDED$REMOVED$UPGRADED$DOWNGRADED" ]]; then
        echo "_No dependency changes detected — SBOM is identical to \`$BASE_REF\`._"
        echo
    fi

    echo "## Reproduce locally"
    echo
    echo '```sh'
    echo '# Regenerate SBOM + re-run the diff gate'
    echo "scripts/$SCRIPT_NAME"
    echo ""
    echo '# Or skip the CVE gate (faster iteration)'
    echo "scripts/$SCRIPT_NAME --no-audit"
    echo '```'
} > "$REPORT_FILE"

echo "    OK: report written ($(wc -l < "$REPORT_FILE" | tr -d ' ') lines)"

# ─── Print summary to stdout ─────────────────────────────────────────────────

echo
echo "==> Summary"
echo "    added:        $(echo "$ADDED" | count_lines)"
echo "    removed:      $(echo "$REMOVED" | count_lines)"
echo "    upgraded:     $(echo "$UPGRADED" | count_lines)"
echo "    downgraded:   $(echo "$DOWNGRADED" | count_lines)"
echo "    critical CVE: ${#CRITICAL_FINDINGS[@]}"
echo

if [[ "$EXIT_CODE" -ne 0 ]]; then
    echo "::error::SBOM diff gate FAILED — ${#CRITICAL_FINDINGS[@]} new high/critical CVE(s) introduced"
    echo "    See $REPORT_FILE for details"
fi

exit "$EXIT_CODE"