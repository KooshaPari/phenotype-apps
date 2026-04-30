import XCTest

final class FocalPointOnboardingUITests: XCTestCase {
    override func setUpWithError() throws {
        continueAfterFailure = false
    }

    func testOnboardingCanScrollSelectGoalAndFinish() throws {
        let app = XCUIApplication()
        app.launchArguments = [
            "--ui-testing",
            "--reset-onboarding",
            "--skip-wake",
            "--force-onboarding-v2",
        ]
        app.launch()

        let next = app.buttons["onboarding.next"]
        XCTAssertTrue(next.waitForExistence(timeout: 10), "Next button should be visible on welcome")
        XCTAssertTrue(app.staticTexts["Focus Coach"].waitForExistence(timeout: 5))
        next.tap()

        let sleepGoal = app.buttons["goal-card-sleep"]
        XCTAssertTrue(sleepGoal.waitForExistence(timeout: 5), "Goal card should be reachable without clipping")
        sleepGoal.tap()
        XCTAssertTrue(sleepGoal.isSelected, "Goal card should expose selected state")
        next.tap()

        XCTAssertTrue(app.staticTexts["Connect Your Calendar"].waitForExistence(timeout: 5))
        next.tap()

        XCTAssertTrue(app.staticTexts["Focus Template"].waitForExistence(timeout: 5))
        next.tap()

        XCTAssertTrue(app.staticTexts["Grant Permissions"].waitForExistence(timeout: 5))
        next.tap()

        let finish = app.buttons["onboarding.finish.inline"]
        XCTAssertTrue(finish.waitForExistence(timeout: 5), "Finish should be visible on final page")
        finish.tap()

        XCTAssertTrue(
            rootExists(app, timeout: 5) || rootExistsAfterRelaunch(app),
            "Finishing onboarding should land in the main app"
        )
    }

    private func rootExists(_ app: XCUIApplication, timeout: TimeInterval) -> Bool {
        app.otherElements["root-tab-view"].waitForExistence(timeout: timeout)
            || app.staticTexts["root-tab-view"].waitForExistence(timeout: 1)
    }

    private func rootExistsAfterRelaunch(_ app: XCUIApplication) -> Bool {
        app.terminate()
        app.launchArguments = [
            "--ui-testing",
            "--skip-wake",
            "--force-onboarding-v2",
        ]
        app.launch()
        return rootExists(app, timeout: 10)
    }
}
