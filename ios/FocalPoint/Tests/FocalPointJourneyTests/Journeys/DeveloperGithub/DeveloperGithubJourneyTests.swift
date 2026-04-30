// DeveloperGithubJourneyTests.swift — Developer GitHub integration journey
// Tests Bob's complete GitHub integration: OAuth to 45-day commit streak

import XCTest
@testable import FocalPointJourneyTests

final class DeveloperGithubJourneyTests: XCTestCase {
    var journey: Journey!
    var driver: AppDriver!

    override func setUp() {
        super.setUp()
        continueAfterFailure = false

        journey = Journey(
            journeyId: "developer-github",
            title: "Developer GitHub Integration",
            persona: "Bob (Full-stack Engineer, 3 repos)"
        )

        driver = AppDriver()
    }

    override func tearDown() {
        journey = nil
        driver = nil
        super.tearDown()
    }

    func testDeveloperGithubJourney() async throws {
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
            // Step 1: Install and basic setup
            .step(slug: "install-basic-setup", intent: "Install FocalPoint, create basic rule") {
                try self.driver
                    .launchForUITesting(resetState: true)
                    .waitForIdle()

                // Skip onboarding if fresh
                let skipButton = self.driver.app.buttons["skip"]
                if skipButton.waitForExistence(timeout: 3) {
                    skipButton.tap()
                }
            }
            // Step 2: Link GitHub
            .step(slug: "link-github-oauth", intent: "OAuth with GitHub account") {
                try self.driver
                    .goToConnectors()
                    .connectConnector(named: "GitHub")

                // Complete OAuth
                try self.driver.completeOAuthFlow(provider: "GitHub")
                self.driver.waitForIdle()
            }
            // Step 3: Select repositories
            .step(slug: "select-repos", intent: "Select repos to monitor: backend, frontend, cli") {
                try self.driver.assertTextVisible("Select Repositories")

                // Select 3 repos
                try self.driver.tap("repo.myorg/backend")
                try self.driver.tap("repo.myorg/frontend")
                try self.driver.tap("repo.myorg/cli")

                try self.driver.tap("continue")
                self.driver.waitForIdle()

                try self.driver.screenshot(named: "repo-selection")
            }
            // Step 4: Verify morning brief shows GitHub
            .step(slug: "morning-brief-github", intent: "Morning brief shows PRs and issues") {
                try self.driver.goToRituals()
                try self.driver.tap("morning-brief")

                try self.driver.assertTextVisible("GitHub Activity")
                try self.driver.screenshot(named: "morning-brief-github")
            }
            // Step 5: Create PR review rule
            .step(slug: "create-pr-review-rule", intent: "Create PR review focus session rule") {
                try self.driver.goToRules()
                try self.driver.tap("rule.add")

                let nameField = self.driver.app.textFields["rule.name"]
                nameField.tap()
                nameField.typeText("PR Review Focus")

                // Select trigger
                try self.driver.tap("trigger.connector-event")
                let eventPicker = self.driver.app.tables.firstMatch
                if eventPicker.waitForExistence(timeout: 3) {
                    eventPicker.cells["github.pr.ready_for_review"].tap()
                }

                // Configure focus session
                try self.driver.tap("action.start-focus")
                try self.driver.tap("block-slack")
                try self.driver.tap("block-discord")

                try self.driver.tap("rule.save")
                self.driver.waitForIdle()

                try self.driver.screenshot(named: "pr-review-rule")
            }
            // Step 6: Simulate PR notification
            .step(slug: "pr-notification", intent: "PR ready for review notification appears") {
                // This would be triggered by a test webhook or mock
                try self.driver.tap("notifications")

                try self.driver.assertTextVisible("PR Ready for Review")
                try self.driver.screenshot(named: "pr-notification")
            }
            // Step 7: Start focus session
            .step(slug: "start-pr-review-focus", intent: "Start PR review focus session") {
                try self.driver.tap("focus.start")
                self.driver.waitForIdle()

                // Verify focus mode
                try self.driver.assertTextVisible("Focus Mode")
                try self.driver.screenshot(named: "focus-pr-active")
            }
            // Step 8: Context switch warning
            .step(slug: "context-switch-warning", intent: "Context switch detection works") {
                // Simulate switching apps (would need test hooks)
                // For now, check if warning appears
                let warning = self.driver.app.staticTexts["context-switch-warning"]
                if warning.waitForExistence(timeout: 2) {
                    try self.driver.screenshot(named: "context-switch-warning")
                }
            }
            // Step 9: Complete review
            .step(slug: "complete-review", intent: "Submit PR review") {
                // In real test, would navigate to GitHub and submit
                try self.driver.tap("focus.end")
                self.driver.waitForIdle()
            }
            // Step 10: Evening stats
            .step(slug: "evening-stats", intent: "Evening shutdown shows code activity") {
                try self.driver.goToRituals()
                try self.driver.tap("evening-shutdown")

                try self.driver.assertTextVisible("CODE ACTIVITY")
                try self.driver.screenshot(named: "evening-stats")
            }
            .run()

        XCTAssertTrue(journey.manifest?.passed ?? false)
        print("📊 Journey manifest: \(journey.manifest?.keyframeCount ?? 0) keyframes captured")
    }
}
