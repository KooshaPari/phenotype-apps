#!/usr/bin/env bash
# extract-keyframes.sh — Extract keyframes from journey recording using ffmpeg
# Usage: ./scripts/extract-keyframes.sh <journey-id>

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JOURNEY_ID="${1:-}"

if [ -z "$JOURNEY_ID" ]; then
    echo "Usage: $0 <journey-id>"
    echo "Example: $0 student-canvas"
    exit 1
fi

# Check ffmpeg
if ! command -v ffmpeg &> /dev/null; then
    echo "Error: ffmpeg not found. Install with: brew install ffmpeg"
    exit 1
fi

PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
RECORDINGS_DIR="${PROJECT_ROOT}/Tests/FocalPointJourneyTests/recordings"
JOURNEY_DIR="${RECORDINGS_DIR}/${JOURNEY_ID}"
RECORDING="${JOURNEY_DIR}/recording.mp4"
KEYFRAME_DIR="${JOURNEY_DIR}/keyframes"
MANIFEST="${JOURNEY_DIR}/manifest.json"

# Create keyframe directory
mkdir -p "$KEYFRAME_DIR"

echo "Extracting keyframes from ${RECORDING}..."

# Extract I-frames (true keyframes for better quality)
ffmpeg -i "$RECORDING" \
    -vf "select='eq(pict_type,I)'" \
    -vsync vfr \
    -q:v 2 \
    "${KEYFRAME_DIR}/keyframe-%03d.png" 2>&1 | grep -E "(frame=|Output)" || true

# If we got fewer than 3 keyframes, fallback to steady sampling
KEYFRAME_COUNT=$(ls "${KEYFRAME_DIR}"/keyframe-*.png 2>/dev/null | wc -l | tr -d ' ')
echo "Extracted ${KEYFRAME_COUNT} I-frame keyframes"

if [ "${KEYFRAME_COUNT}" -lt 3 ]; then
    echo "Found only ${KEYFRAME_COUNT} I-frames; extracting steady sample at 1 fps..."
    rm -f "${KEYFRAME_DIR}"/keyframe-*.png
    ffmpeg -i "$RECORDING" \
        -vf "fps=1" \
        -q:v 2 \
        "${KEYFRAME_DIR}/keyframe-%03d.png" 2>&1 | grep -E "(frame=|Output)" || true

    FINAL_COUNT=$(ls "${KEYFRAME_DIR}"/keyframe-*.png 2>/dev/null | wc -l | tr -d ' ')
    echo "Extracted ${FINAL_COUNT} steady-sample keyframes"
fi

# Create optimized GIF preview
echo "Creating optimized GIF preview..."
GIF_OUTPUT="${JOURNEY_DIR}/preview.gif"
ffmpeg -i "$RECORDING" \
    -filter_complex "fps=10,scale=585:-1:flags=lanczos[s];[s]split[a][b];[a]palettegen=max_colors=256:stats_mode=diff[pal];[b][pal]paletteuse=dither=bayer:bayer_scale=5:diff_mode=rectangle" \
    -loop 0 \
    "$GIF_OUTPUT" 2>&1 | grep -E "(frame=|Output)" || true

if [ -f "$GIF_OUTPUT" ]; then
    echo "Created GIF preview: preview.gif"
fi

# Update manifest with keyframe info
if [ -f "$MANIFEST" ]; then
    KEYFRAMES_JSON="["
    first=true
    for kf in "${KEYFRAME_DIR}"/keyframe-*.png; do
        if [ -f "$kf" ]; then
            fname=$(basename "$kf")
            if [ "$first" = true ]; then
                first=false
            else
                KEYFRAMES_JSON+=","
            fi
            KEYFRAMES_JSON+="\"keyframes/${fname}\""
        fi
    done
    KEYFRAMES_JSON+="]"

    # Use jq if available, otherwise just note the keyframes
    if command -v jq &> /dev/null; then
        jq --argjson keyframes "$KEYFRAMES_JSON" \
           '.keyframes = $keyframes | .keyframe_count = ($keyframes | length)' \
           "$MANIFEST" > "${MANIFEST}.tmp" && mv "${MANIFEST}.tmp" "$MANIFEST"
        echo "Updated manifest with keyframe references"
    else
        echo "Note: jq not available, manifest not updated with keyframe paths"
    fi
fi

echo ""
echo "Keyframe extraction complete!"
echo "  Recording: ${RECORDING}"
echo "  Keyframes: ${KEYFRAME_DIR}/"
echo "  GIF preview: ${GIF_OUTPUT}"
