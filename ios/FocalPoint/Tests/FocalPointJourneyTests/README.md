# FocalPoint Journey Tests

End-to-end journey recording and verification system for FocalPoint iOS app.

## Overview

This module provides a complete system for recording, verifying, and documenting user journeys through the FocalPoint app. It enables:

- **Automated UI Journey Recording** via XCUITest + ScreenCaptureKit
- **Keyframe Extraction** for documentation
- **VLM Verification** for traceability
- **Journey Manifest Generation** for docs-site integration

## Directory Structure

```
FocalPointJourneyTests/
├── Harnesses/
│   ├── FocalPointJourneyTestCase.swift  # Base test case with recording
│   └── ScreenCaptureManager.swift       # ScreenCaptureKit wrapper
├── Journeys/
│   ├── StudentCanvas/                   # Student Canvas integration journey
│   ├── DeveloperGithub/                # GitHub PR review journey
│   ├── MorningBrief/                    # Morning ritual journey
│   ├── EveningShutdown/                # Evening reflection journey
│   └── FocusSession/                   # Deep focus session journey
├── Sources/
│   ├── Journey.swift                   # Journey DSL and metadata
│   ├── AppDriver.swift                 # App navigation helpers
│   └── ScreenRecorder.swift           # Recording utilities
├── scripts/
│   ├── run-journeys.sh                 # Run all/specific journeys
│   ├── extract-keyframes.sh           # Extract keyframes from recordings
│   └── generate-manifests.sh           # Generate manifest.json files
├── recordings/                         # Generated recordings & keyframes
│   ├── student-canvas/
│   ├── developer-github/
│   └── ...
└── manifests/                         # Manifest templates
```

## Quick Start

### Prerequisites

1. iOS Simulator (iPhone 14 Pro recommended)
2. Xcode 15+
3. ScreenCaptureKit entitlement (included in test target)

### Run All Journeys

```bash
cd /path/to/FocalPointJourneyTests
./scripts/run-journeys.sh all
```

### Run Specific Journey

```bash
./scripts/run-journeys.sh student-canvas
```

### Extract Keyframes

```bash
./scripts/extract-keyframes.sh
```

### Generate Manifests

```bash
./scripts/generate-manifests.sh
```

## Journey Definitions

### Student Canvas Journey

**Persona**: Alice (Student, CS major)
**Intent**: "Alice uses FocalPoint to manage Canvas assignments and build study habits"
**Duration**: ~5 minutes
**Keyframes**: 10

Steps:
1. App launch → Onboarding intro
2. Create account
3. Connect Canvas LMS
4. OAuth flow
5. Course selection
6. Morning brief empty state
7. Canvas assignments synced
8. Rule editor for due date reminders
9. Focus mode active
10. Streak celebration (Day 1)

### Developer GitHub Journey

**Persona**: Bob (Full-stack Engineer)
**Intent**: "Bob uses FocalPoint for deep code review work and feature development"
**Duration**: ~4 minutes
**Keyframes**: 10

Steps:
1. Morning brief with GitHub notifications
2. GitHub OAuth connection
3. Repository selection
4. PR review rule creation
5. Context switch warning
6. Single repo focus mode
7. PR reviewed
8. Context switch context
9. Evening stats view
10. Milestone celebration (45-day streak)

### Morning Brief Journey

**Persona**: Bob (Developer)
**Intent**: "Daily morning ritual with sleep stats, schedule review, GitHub activity, and focus opportunities"
**Duration**: ~3 minutes
**Keyframes**: 5

Steps:
1. Morning trigger (7:00 AM notification)
2. Sleep stats view
3. Schedule overview
4. GitHub activity digest
5. Focus opportunities preview

### Evening Shutdown Journey

**Persona**: Alice (Student)
**Intent**: "Daily evening reflection with focus stats, reflection questions, and Coachy's personalized insights"
**Duration**: ~3 minutes
**Keyframes**: 6

Steps:
1. Evening trigger (10:00 PM notification)
2. Focus stats summary
3. Reflection questions
4. Coachy insights
5. Export options
6. Sleep trigger

### Focus Session Journey

**Persona**: Bob (Developer)
**Intent**: "Deep focus session for code review with Coachy break suggestions and session completion"
**Duration**: ~4 minutes
**Keyframes**: 7

Steps:
1. PR notification
2. Focus activation
3. Apps blocked
4. Review in progress
5. Break suggestion
6. Break taken
7. Session complete with celebration

## Recording System

### ScreenCaptureKit Integration

The `ScreenCaptureManager` class provides:
- Screen recording with audio
- Configurable resolution (1080p default)
- Frame-accurate keyframe extraction
- Automatic storage to recordings/

### Journey DSL

```swift
import XCTest

class StudentCanvasJourneyTests: XCTestCase {
    func testFullJourney() throws {
        let journey = Journey(
            id: "student-canvas",
            persona: "Alice",
            intent: "Student Canvas onboarding"
        )

        // Start recording
        try journey.beginRecording()

        // Define steps
        try journey.step("app-launch") {
            app.launch()
        }

        // Verify UI element
        try journey.verify("onboarding-title") {
            app.staticTexts["Get Started"].waitForExistence(timeout: 5)
        }

        // End recording
        try journey.endRecording()
    }
}
```

## VLM Verification

After recording, keyframes can be verified using Vision Language Models:

```bash
# Verify all keyframes against journey intent
./scripts/verify-journeys.sh --journey student-canvas --vlm claude
```

## Docs Integration

Recordings sync to docs-site via:

```bash
./scripts/sync-to-docs.sh
```

This copies:
- `recordings/*/manifest.json` → `docs-site/recordings/*/`
- `recordings/*/keyframes/*.png` → `docs-site/recordings/*/keyframes/`

## Troubleshooting

### Recording Not Starting

1. Check ScreenRecording permission in Simulator
2. Verify ScreenCaptureKit entitlement in Info.plist
3. Ensure no other app is using screen capture

### Keyframes Not Extracting

1. Verify `ffmpeg` is installed: `brew install ffmpeg`
2. Check recording exists: `ls recordings/*/recording.mp4`
3. Run with verbose: `./scripts/extract-keyframes.sh --verbose`

### Tests Failing on CI

1. Ensure `SNAPSHOT_FAILED=1` doesn't block runs
2. Use `--no-record` flag for verification-only runs
3. Set `FOCALPOINT_TEST_MODE=1` for CI environment

## License

Internal — FocalPoint Project
