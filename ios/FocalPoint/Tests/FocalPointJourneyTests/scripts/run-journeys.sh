#!/usr/bin/env bash
# run-journeys.sh — Run all FocalPoint journey tests and capture recordings
# Usage: ./scripts/run-journeys.sh [--journey JOURNEY_ID] [--skip-recordings]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
XCODEPROJ="${PROJECT_ROOT}/FocalPoint.xcodeproj"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default: run all journeys
JOURNEY_FILTER="${1:-}"
SKIP_RECORDINGS=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --journey)
            JOURNEY_FILTER="$2"
            shift 2
            ;;
        --skip-recordings)
            SKIP_RECORDINGS=true
            shift
            ;;
        --help)
            echo "Usage: $0 [--journey JOURNEY_ID] [--skip-recordings]"
            echo ""
            echo "Options:"
            echo "  --journey JOURNEY_ID    Run specific journey only"
            echo "  --skip-recordings       Skip screen recording (faster test run)"
            echo "  --help                  Show this help"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  FocalPoint Journey Test Runner${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo ""

# Check prerequisites
check_prerequisites() {
    echo -e "${YELLOW}Checking prerequisites...${NC}"

    # Check XcodeGen
    if ! command -v xcodegen &> /dev/null; then
        echo -e "${RED}✗ XcodeGen not found. Install with: brew install xcodegen${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ XcodeGen found${NC}"

    # Check xcrun simctl
    if ! command -v simctl &> /dev/null; then
        echo -e "${RED}✗ simctl not found. Is Xcode Command Line Tools installed?${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ simctl found${NC}"

    # Check ffmpeg (optional but recommended)
    if command -v ffmpeg &> /dev/null; then
        echo -e "${GREEN}✓ ffmpeg found (for keyframe extraction)${NC}"
    else
        echo -e "${YELLOW}⚠ ffmpeg not found. Install with: brew install ffmpeg${NC}"
        echo "   Keyframe extraction will be skipped."
    fi

    # Check accessibility permission
    if ! tccutil status AppleEvent &> /dev/null; then
        echo -e "${YELLOW}⚠ Accessibility permission may be required for UI testing${NC}"
    else
        echo -e "${GREEN}✓ Accessibility permission status checked${NC}"
    fi
}

# Boot simulator
boot_simulator() {
    echo -e "${YELLOW}Booting iOS Simulator...${NC}"

    # Find available iPhone simulator
    DEVICE=$(xcrun simctl list devices available | grep -E "iPhone (1[4-6]|SE)" | head -1 | sed 's/.*(\([^)]*\)).*/\1/')

    if [ -z "$DEVICE" ]; then
        DEVICE="iPhone 16 Pro"
    fi

    echo -e "Using device: ${BLUE}${DEVICE}${NC}"

    # Boot if not already running
    xcrun simctl boot "$DEVICE" 2>/dev/null || true

    echo -e "${GREEN}✓ Simulator ready${NC}"
}

# Run single journey test
run_journey() {
    local journey_id="$1"
    local journey_name="${journey_id//-/ }"  # Convert foo-bar to "foo bar"

    echo ""
    echo -e "${BLUE}────────────────────────────────────────────────────────────${NC}"
    echo -e "${BLUE}  Journey: ${journey_id}${NC}"
    echo -e "${BLUE}────────────────────────────────────────────────────────────${NC}"

    # Create output directory
    local output_dir="${PROJECT_ROOT}/Tests/FocalPointJourneyTests/recordings/${journey_id}"
    mkdir -p "$output_dir"

    # Run the test with xcodebuild
    # Note: In real usage, this would invoke xcodebuild test with the specific journey
    local test_cmd="xcodebuild test \
        -project '${XCODEPROJ}' \
        -scheme FocalPoint \
        -destination 'platform=iOS Simulator,name=${DEVICE}' \
        -only-testing:FocalPointJourneyTests/${journey_id} \
        -test-iterations 1 \
        2>&1 | tee '${output_dir}/test-output.log'"

    if eval "$test_cmd"; then
        echo -e "${GREEN}✓ Journey '${journey_id}' completed${NC}"

        # Extract keyframes if recording exists
        if [ -f "${output_dir}/recording.mp4" ] && command -v ffmpeg &> /dev/null; then
            echo -e "  Extracting keyframes..."
            "${SCRIPT_DIR}/extract-keyframes.sh" "$journey_id"
        fi
    else
        echo -e "${RED}✗ Journey '${journey_id}' failed${NC}"
        return 1
    fi
}

# Main execution
main() {
    check_prerequisites
    boot_simulator

    echo ""
    echo -e "${YELLOW}Available journeys:${NC}"

    local journeys=(
        "student-canvas"
        "developer-github"
        "morning-brief"
        "evening-shutdown"
        "focus-session"
    )

    for j in "${journeys[@]}"; do
        echo "  • ${j}"
    done

    echo ""

    if [ -n "$JOURNEY_FILTER" ]; then
        # Run specific journey
        run_journey "$JOURNEY_FILTER"
    else
        # Run all journeys
        local failed=0
        for j in "${journeys[@]}"; do
            if ! run_journey "$j"; then
                ((failed++))
            fi
        done

        if [ $failed -eq 0 ]; then
            echo ""
            echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
            echo -e "${GREEN}  All journeys completed successfully!${NC}"
            echo -e "${GREEN}═══════════════════════════════════════════════════════════════${NC}"
        else
            echo ""
            echo -e "${RED}═══════════════════════════════════════════════════════════════${NC}"
            echo -e "${RED}  $failed journey(ies) failed${NC}"
            echo -e "${RED}═══════════════════════════════════════════════════════════════${NC}"
            exit 1
        fi
    fi

    # Generate manifests
    echo ""
    echo -e "${YELLOW}Generating journey manifests...${NC}"
    "${SCRIPT_DIR}/generate-manifests.sh"

    echo ""
    echo -e "${GREEN}✓ Recording complete!${NC}"
    echo "Output: ${PROJECT_ROOT}/Tests/FocalPointJourneyTests/recordings/"
}

main "$@"
