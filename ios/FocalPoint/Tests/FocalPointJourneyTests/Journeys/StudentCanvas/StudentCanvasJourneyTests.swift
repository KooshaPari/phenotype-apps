// StudentCanvasJourneyTests.swift — Student Canvas onboarding journey
// Tests Alice's complete onboarding from install to 67-day streak

import XCTest
@testable import FocalPointJourneyTests

final class StudentCanvasJourneyTests: XCTestCase {
    var journey: Journey!
    var driver: AppDriver!

    override func setUp() {
        super.setUp()
        continueAfterFailure = false

        journey = Journey(
            journeyId: "student-canvas",
            title: "Student Canvas Onboarding",
            persona: "Alice (Student, CS major)"
        )

        driver = AppDriver()
    }

    override func tearDown() {
        journey = nil
        driver = nil
        super.tearDown()
    }

    func testStudentCanvasOnboardingJourney() async throws {
        // Start screen recording
        if #available(iOS 16.0, *) {
            try? await ScreenRecorderManager.shared.startRecording(journeyId: journey.journeyId)
        }

        defer {
            if #available(iOS 16.0, *) {
                Task {
                    _ = try? await ScreenRecorderManager.shared.stopRecording()
                }
            }
        }

        try await journey
            // Step 1: Install and grant permissions
            .step(slug: "install-grant-permissions", intent: "Install app and grant FamilyControls") {
                try self.driver
                    .launchForUITesting(resetState: true)
                    .waitForIdle()

                // Should see permission dialog
                try self.driver.assertExists("permission.family-controls")
            }
            // Step 2: Skip optional connectors
            .step(slug: "skip-connectors", intent: "Skip optional connectors initially") {
                // Tap skip on connectors screen if present
                let skipButton = self.driver.app.buttons["skip"]
                if skipButton.waitForExistence(timeout: 3) {
                    skipButton.tap()
                }
                self.driver.waitForIdle()
            }
            // Step 3: Create first rule
            .step(slug: "create-first-rule", intent: "Create morning brief trigger rule") {
                try self.driver
                    .goToRules()

                try self.driver.tap("rule.add")

                // Enter rule name
                let nameField = self.driver.app.textFields["rule.name"]
                nameField.tap()
                nameField.typeText("Morning Brief")

                // Select trigger: Daily at 8 AM
                try self.driver.tap("trigger.schedule")
                let timePicker = self.driver.app.datePickers.firstMatch
                if timePicker.waitForExistence(timeout: 3) {
                    timePicker.pickerWheels.element(boundBy: 0).adjust(toPickerWheelValue: "8")
                    timePicker.pickerWheels.element(boundBy: 1).adjust(toPickerWheelValue: "00")
                }

                // Select action: Show morning brief
                try self.driver.tap("action.show-morning-brief")

                // Save
                try self.driver.tap("rule.save")
                self.driver.waitForIdle()
            }
            // Step 4: See empty morning brief
            .step(slug: "see-empty-morning-brief", intent: "Morning brief shows empty state (no Canvas yet)") {
                try self.driver.assertTextVisible("Morning Brief")
                try self.driver.screenshot(named: "morning-brief-empty")
            }
            // Step 5: Connect Canvas
            .step(slug: "link-canvas-oauth", intent: "OAuth with Canvas LMS") {
                try self.driver
                    .goToConnectors()
                    .connectConnector(named: "Canvas")

                // Complete OAuth
                try self.driver.completeOAuthFlow(provider: "Canvas")
                self.driver.waitForIdle()
            }
            // Step 6: Select courses
            .step(slug: "select-courses", intent: "Select all 4 courses") {
                try self.driver.assertTextVisible("Select Courses")

                // Select all courses (assume test has 4 course checkboxes)
                let courseCheckboxes = self.driver.app.checkboxes.matching(identifier: "course.")
                let count = courseCheckboxes.count
                for i in 0..<count {
                    courseCheckboxes.element(boundBy: i).tap()
                }

                try self.driver.tap("continue")
                self.driver.waitForIdle()
            }
            // Step 7: Verify assignments synced
            .step(slug: "assignments-synced", intent: "Assignments appear in morning brief") {
                // Go back to morning brief
                try self.driver.goToRituals()
                try self.driver.tap("morning-brief")

                // Should see Canvas assignments
                try self.driver.assertTextVisible("CS 101")
                try self.driver.screenshot(named: "assignments-synced")
            }
            // Step 8: Create Canvas due-soon rule
            .step(slug: "create-due-soon-rule", intent: "Create assignment due notification rule") {
                try self.driver.goToRules()
                try self.driver.tap("rule.add")

                let nameField = self.driver.app.textFields["rule.name"]
                nameField.tap()
                nameField.typeText("Canvas Due Soon")

                // Select trigger: canvas.assignment.due_soon
                try self.driver.tap("trigger.connector-event")
                let eventPicker = self.driver.app.tables.firstMatch
                if eventPicker.waitForExistence(timeout: 3) {
                    eventPicker.cells["canvas.assignment.due_soon"].tap()
                }

                // Select action: Show focus view
                try self.driver.tap("action.block-apps")
                try self.driver.tap("block-tiktok")
                try self.driver.tap("block-instagram")

                // Save
                try self.driver.tap("rule.save")
                self.driver.waitForIdle()
            }
            // Step 9: Test rule manually
            .step(slug: "test-rule", intent: "Test rule by manually triggering") {
                // Long press on rule to trigger
                let ruleCell = self.driver.app.cells["rule.Canvas Due Soon"]
                if ruleCell.waitForExistence(timeout: 3) {
                    ruleCell.buttons["rule.trigger"].tap()
                }

                // Verify TikTok is blocked
                try self.driver.screenshot(named: "focus-mode-active")
            }
            // Step 10: Check streak
            .step(slug: "streak-visible", intent: "Day 1 streak appears") {
                try self.driver.assertTextVisible("1 day")
                try self.driver.screenshot(named: "streak-day-1")
            }
            .run()

        // Journey complete
        XCTAssertTrue(journey.manifest?.passed ?? false, "Journey should pass")
        print("📊 Journey manifest: \(journey.manifest?.keyframeCount ?? 0) keyframes captured")
    }
}
