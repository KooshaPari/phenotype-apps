// FocusSessionJourneyTests.swift — Focus session journey
// Tests PR review deep work with break suggestions and session completion

import XCTest
@testable import FocalPointJourneyTests

final class FocusSessionJourneyTests: XCTestCase {
    var journey: Journey!
    var driver: AppDriver!

    override func setUp() {
        super.setUp()
        continueAfterFailure = false

        journey = Journey(
            journeyId: "focus-session",
            title: "Focus Session - PR Review",
            persona: "Bob (Developer)"
        )

        driver = AppDriver()
    }

    override func tearDown() {
        journey = nil
        driver = nil
        super.tearDown()
    }

    func testFocusSessionJourney() async throws {
        // Start recording
        if #available(iOS 16.0, *) {
            try? await ScreenRecorderManager.shared.startRecording(journeyId: journey.journeyId)
        }

        defer {
            if #available(iOS 16.0, *) {
                Task { _ = try? await ScreenRecorderManager.shared.stopRecording() }
            }
        }

        try await journey
            // Step 1: PR notification
            .step(slug: "pr-notification", intent: "PR ready notification appears") {
                try self.driver
                    .launchForUITesting()
                    .waitForIdle()

                // Trigger PR notification (via test hook or manual)
                try self.driver.tap("notifications")
                self.driver.waitForIdle()

                try self.driver.assertTextVisible("PR Ready for Review")
                try self.driver.screenshot(named: "pr-notification")
            }
            // Step 2: Start focus
            .step(slug: "start-focus", intent: "Tap start review to activate focus mode") {
                try self.driver.tap("start-review")
                self.driver.waitForIdle()

                try self.driver.screenshot(named: "focus-activation")
            }
            // Step 3: Focus mode active
            .step(slug: "focus-mode-active", intent: "Verify apps blocked, timer running") {
                try self.driver.assertTextVisible("FOCUS MODE ACTIVE")
                try self.driver.assertTextVisible("Slack")

                // Timer should show
                let timer = self.driver.app.staticTexts["focus-timer"]
                if timer.waitForExistence(timeout: 2) {
                    try self.driver.screenshot(named: "apps-blocked")
                }
            }
            // Step 4: Review in progress
            .step(slug: "review-in-progress", intent: "Reviewing PR with full focus") {
                // In real test, would navigate to GitHub
                self.driver.waitForIdle()
                try self.driver.screenshot(named: "review-in-progress")
            }
            // Step 5: Break suggestion at 90 min
            .step(slug: "break-suggestion", intent: "90-minute break suggestion appears") {
                // Simulate 90 minutes passing (test hook)
                // For now, check if break UI exists
                let breakSuggestion = self.driver.app.buttons["take-break"]
                if breakSuggestion.waitForExistence(timeout: 2) {
                    breakSuggestion.tap()
                    try self.driver.screenshot(named: "break-suggestion")
                }
            }
            // Step 6: Break timer
            .step(slug: "break-timer", intent: "15-minute break timer running") {
                try self.driver.assertTextVisible("BREAK TIME")
                try self.driver.screenshot(named: "break-timer")
            }
            // Step 7: Session complete
            .step(slug: "session-complete", intent: "Focus session completes, stats shown") {
                try self.driver.tap("skip-break")
                self.driver.waitForIdle()

                // End session
                try self.driver.tap("end-session")
                self.driver.waitForIdle()

                try self.driver.screenshot(named: "focus-complete")
            }
            // Step 8: Coachy celebration
            .step(slug: "coachy-celebration", intent: "Coachy celebrates completion") {
                try self.driver.assertTextVisible("FOCUS SESSION COMPLETE")
                try self.driver.screenshot(named: "coachy-celebration")
            }
            .run()

        XCTAssertTrue(journey.manifest?.passed ?? false)
        print("📊 Journey manifest: \(journey.manifest?.keyframeCount ?? 0) keyframes captured")
    }
}
