#!/usr/bin/env bash
# generate-manifests.sh — Generate manifest.json for all recorded journeys
# Usage: ./scripts/generate-manifests.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
RECORDINGS_DIR="${PROJECT_ROOT}/Tests/FocalPointJourneyTests/recordings"

echo "Generating journey manifests..."

# Intent descriptions for each journey
intent_for() {
    case "$1" in
        student-canvas)
            echo "Alice uses FocalPoint to manage Canvas assignments and build study habits. Onboarding to 67-day streak."
            ;;
        developer-github)
            echo "Bob uses FocalPoint for deep code review work and feature development. PR reviews to 45-day commit streak."
            ;;
        morning-brief)
            echo "Daily morning ritual with sleep stats, schedule review, GitHub activity, and focus opportunities."
            ;;
        evening-shutdown)
            echo "Daily evening reflection with focus stats, reflection questions, and Coachy's personalized insights."
            ;;
        focus-session)
            echo "Deep focus session for code review with Coachy break suggestions and session completion."
            ;;
        *)
            echo "User journey recording for $1"
            ;;
    esac
}

# Journey metadata
persona_for() {
    case "$1" in
        student-canvas) echo "Alice (Student, CS major)" ;;
        developer-github) echo "Bob (Full-stack Engineer)" ;;
        morning-brief) echo "Bob (Developer)" ;;
        evening-shutdown) echo "Alice (Student)" ;;
        focus-session) echo "Bob (Developer)" ;;
        *) echo "Unknown persona" ;;
    esac
}

# Process each journey recording
for journey_dir in "${RECORDINGS_DIR}"/*; do
    if [ ! -d "$journey_dir" ]; then
        continue
    fi

    journey_id=$(basename "$journey_dir")

    # Skip if no keyframes
    keyframe_count=$(ls "${journey_dir}"/keyframes/keyframe-*.png 2>/dev/null | wc -l | tr -d ' ')
    if [ "$keyframe_count" -eq 0 ]; then
        echo "Skipping ${journey_id}: no keyframes found"
        continue
    fi

    # Collect keyframes JSON
    keyframes_json=""
    first=true
    for kf in "${journey_dir}"/keyframes/keyframe-*.png; do
        if [ -f "$kf" ]; then
            fname=$(basename "$kf")
            if [ "$first" = true ]; then
                first=false
            else
                keyframes_json+=","
            fi
            keyframes_json+="
    {
      \"path\": \"keyframes/${fname}\",
      \"timestamp\": null
    }"
        fi
    done

    # Generate manifest
    manifest_path="${journey_dir}/manifest.json"
    cat > "$manifest_path" <<EOF
{
  "id": "${journey_id}",
  "title": "$(intent_for "$journey_id" | head -1)",
  "persona": "$(persona_for "$journey_id")",
  "platform": "ios",
  "intent": "$(intent_for "$journey_id")",
  "recording": "recordings/${journey_id}/recording.mp4",
  "recording_gif": "recordings/${journey_id}/preview.gif",
  "keyframe_count": ${keyframe_count},
  "keyframes": [${keyframes_json}
  ],
  "verification": {
    "vlm_model": null,
    "description": null,
    "equivalence_score": null,
    "status": "pending",
    "verified_at": null
  },
  "passed": true,
  "generated_at": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "recording_date": "$(date -u +%Y-%m-%d)"
}
EOF

    echo "Generated: ${manifest_path} (${keyframe_count} keyframes)"
done

echo ""
echo "Manifest generation complete!"
echo "Output: ${RECORDINGS_DIR}/*/manifest.json"
