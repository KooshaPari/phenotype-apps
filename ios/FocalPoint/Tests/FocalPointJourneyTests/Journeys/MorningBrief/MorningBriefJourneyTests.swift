// MorningBriefJourneyTests.swift — Morning brief ritual journey
// Tests Bob's daily morning ritual with GitHub activity and focus opportunities

import XCTest
@testable import FocalPointJourneyTests

final class MorningBriefJourneyTests: XCTestCase {
    var journey: Journey!
    var driver: AppDriver!

    override func setUp() {
        super.setUp()
        continueAfterFailure = false

        journey = Journey(
            journeyId: "morning-brief",
            title: "Morning Brief Ritual",
            persona: "Bob (Developer)"
        )

        driver = AppDriver()
    }

    override func tearDown() {
        journey = nil
        driver = nil
        super.tearDown()
    }

    func testMorningBriefJourney() async throws {
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
            // Step 1: Open morning brief
            .step(slug: "open-morning-brief", intent: "Navigate to morning brief") {
                try self.driver
                    .launchForUITesting()
                    .waitForIdle()

                try self.driver.goToRituals()
                try self.driver.tap("morning-brief")
                self.driver.waitForIdle()
            }
            // Step 2: View sleep stats
            .step(slug: "view-sleep-stats", intent: "See sleep duration from Apple Health") {
                // Sleep stats should be visible
                let sleepStat = self.driver.app.staticTexts["sleep-stat"]
                if sleepStat.waitForExistence(timeout: 3) {
                    try self.driver.screenshot(named: "sleep-stats")
                }
            }
            // Step 3: View today's schedule
            .step(slug: "view-today-schedule", intent: "See calendar events and deadlines") {
                try self.driver.assertTextVisible("Today")
                try self.driver.screenshot(named: "today-schedule")
            }
            // Step 4: View GitHub activity
            .step(slug: "view-github-activity", intent: "See PRs, issues, and milestones") {
                try self.driver.assertTextVisible("GitHub")
                try self.driver.screenshot(named: "github-activity")
            }
            // Step 5: View focus opportunities
            .step(slug: "view-focus-opportunities", intent: "See suggested focus blocks") {
                try self.driver.assertTextVisible("Focus opportunities")
                try self.driver.screenshot(named: "focus-opportunities")
            }
            // Step 6: View streak
            .step(slug: "view-streak", intent: "See current streak count") {
                let streak = self.driver.app.staticTexts["streak"]
                if streak.waitForExistence(timeout: 2) {
                    try self.driver.screenshot(named: "streak")
                }
            }
            // Step 7: Start first focus
            .step(slug: "start-first-focus", intent: "Start first focus block of the day") {
                try self.driver.tap("focus.start.1")
                self.driver.waitForIdle()

                try self.driver.assertTextVisible("Focus Mode")
                try self.driver.screenshot(named: "focus-started")
            }
            .run()

        XCTAssertTrue(journey.manifest?.passed ?? false)
        print("📊 Journey manifest: \(journey.manifest?.keyframeCount ?? 0) keyframes captured")
    }
}
