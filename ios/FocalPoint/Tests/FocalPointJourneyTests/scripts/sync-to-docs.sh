#!/usr/bin/env bash
# sync-to-docs.sh — Sync recordings to docs-site
# Usage: ./scripts/sync-to-docs.sh [--dry-run]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
RECORDINGS_DIR="${PROJECT_ROOT}/Tests/FocalPointJourneyTests/recordings"
DOCS_ROOT="${PROJECT_ROOT}/docs-site"
DRY_RUN=false

if [ "${1:-}" = "--dry-run" ]; then
    DRY_RUN=true
fi

echo "Syncing journey recordings to docs-site..."
[ "$DRY_RUN" = true ] && echo "[DRY RUN MODE]"

# Ensure recordings directory exists
if [ ! -d "$RECORDINGS_DIR" ]; then
    echo "No recordings directory found at ${RECORDINGS_DIR}"
    echo "Run journeys first: ./scripts/run-journeys.sh all"
    exit 1
fi

# Ensure docs-site recordings directory exists
mkdir -p "${DOCS_ROOT}/recordings"

# Process each journey
for journey_dir in "${RECORDINGS_DIR}"/*; do
    if [ ! -d "$journey_dir" ]; then
        continue
    fi

    journey_id=$(basename "$journey_dir")
    target_dir="${DOCS_ROOT}/recordings/${journey_id}"

    echo ""
    echo "Syncing ${journey_id}..."

    # Copy manifest
    if [ -f "${journey_dir}/manifest.json" ]; then
        if [ "$DRY_RUN" = false ]; then
            mkdir -p "$target_dir"
            cp "${journey_dir}/manifest.json" "${target_dir}/"
        fi
        echo "  ✓ manifest.json"
    fi

    # Copy recording video
    if [ -f "${journey_dir}/recording.mp4" ]; then
        if [ "$DRY_RUN" = false ]; then
            cp "${journey_dir}/recording.mp4" "${target_dir}/"
        fi
        echo "  ✓ recording.mp4"
    fi

    # Copy preview GIF
    if [ -f "${journey_dir}/preview.gif" ]; then
        if [ "$DRY_RUN" = false ]; then
            cp "${journey_dir}/preview.gif" "${target_dir}/"
        fi
        echo "  ✓ preview.gif"
    fi

    # Copy keyframes
    keyframes_src="${journey_dir}/keyframes"
    if [ -d "$keyframes_src" ] && [ "$(ls -A "$keyframes_src" 2>/dev/null)" ]; then
        if [ "$DRY_RUN" = false ]; then
            mkdir -p "${target_dir}/keyframes"
            cp "${keyframes_src}"/*.png "${target_dir}/keyframes/" 2>/dev/null || true
        fi
        kf_count=$(ls "$keyframes_src"/*.png 2>/dev/null | wc -l | tr -d ' ')
        echo "  ✓ ${kf_count} keyframes"
    fi

    # Copy VLM verification report
    if [ -f "${journey_dir}/vlm-report.json" ]; then
        if [ "$DRY_RUN" = false ]; then
            cp "${journey_dir}/vlm-report.json" "${target_dir}/"
        fi
        echo "  ✓ vlm-report.json"
    fi
done

echo ""
echo "Sync complete!"
echo "Docs recordings: ${DOCS_ROOT}/recordings/"

if [ "$DRY_RUN" = false ]; then
    echo ""
    echo "Next steps:"
    echo "  1. Review synced files at: ${DOCS_ROOT}/recordings/"
    echo "  2. Update journey docs with JourneyViewer: docs-site/journeys/*.md"
    echo "  3. Commit changes: git add recordings/ && git commit -m 'chore: sync journey recordings'"
fi
