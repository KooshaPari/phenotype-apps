// EveningShutdownJourneyTests.swift — Evening shutdown ritual journey
// Tests Alice's evening reflection with stats review and Coachy's insight

import XCTest
@testable import FocalPointJourneyTests

final class EveningShutdownJourneyTests: XCTestCase {
    var journey: Journey!
    var driver: AppDriver!

    override func setUp() {
        super.setUp()
        continueAfterFailure = false

        journey = Journey(
            journeyId: "evening-shutdown",
            title: "Evening Shutdown Ritual",
            persona: "Alice (Student)"
        )

        driver = AppDriver()
    }

    override func tearDown() {
        journey = nil
        driver = nil
        super.tearDown()
    }

    func testEveningShutdownJourney() async throws {
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
            // Step 1: Evening notification
            .step(slug: "evening-notification", intent: "7 PM notification triggers shutdown ritual") {
                try self.driver
                    .launchForUITesting()
                    .waitForIdle()

                // Notification should appear (or tap to open)
                let notification = self.driver.app.buttons["evening.shutdown.start"]
                if notification.waitForExistence(timeout: 5) {
                    notification.tap()
                } else {
                    // Navigate manually
                    try self.driver.goToRituals()
                    try self.driver.tap("evening-shutdown")
                }
                self.driver.waitForIdle()
            }
            // Step 2: View stats
            .step(slug: "view-stats", intent: "Review today's focus stats") {
                try self.driver.assertTextVisible("TODAY'S SCORE")
                try self.driver.screenshot(named: "evening-stats-view")
            }
            // Step 3: View connector progress
            .step(slug: "view-connector-progress", intent: "See GitHub and Canvas activity") {
                try self.driver.assertTextVisible("GitHub")
                try self.driver.assertTextVisible("Canvas")
                try self.driver.screenshot(named: "connector-progress")
            }
            // Step 4: Reflection questions
            .step(slug: "reflection-questions", intent: "Answer reflection questions") {
                // Question 1: What blocked focus?
                try self.driver.tap("blocker-option-1")
                self.driver.waitForIdle()

                // Question 2: Tomorrow's priority
                let priorityField = self.driver.app.textFields["tomorrow-priority"]
                if priorityField.waitForExistence(timeout: 3) {
                    priorityField.tap()
                    priorityField.typeText("Start CS Midterm project")
                }

                try self.driver.screenshot(named: "reflection-questions")
            }
            // Step 5: Coachy's insight
            .step(slug: "coachy-insight", intent: "See personalized Coachy feedback") {
                try self.driver.assertTextVisible("COACHY INSIGHTS")
                try self.driver.screenshot(named: "coachy-insight")
            }
            // Step 6: Export options
            .step(slug: "export-options", intent: "See export/share options") {
                try self.driver.assertTextVisible("Export")
                try self.driver.screenshot(named: "export-options")
            }
            // Step 7: Complete shutdown
            .step(slug: "complete-shutdown", intent: "Finish evening ritual") {
                try self.driver.tap("done")
                self.driver.waitForIdle()
            }
            .run()

        XCTAssertTrue(journey.manifest?.passed ?? false)
        print("📊 Journey manifest: \(journey.manifest?.keyframeCount ?? 0) keyframes captured")
    }
}
