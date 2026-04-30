// AppDriver.swift — Accessibility API wrapper for FocalPoint navigation
// Provides fluent API for interacting with app elements during journeys

import XCTest
import Foundation

// MARK: - App Driver

/// Wrapper around XCUIApplication providing fluent navigation API
public class AppDriver: @unchecked Sendable {
    public let app: XCUIApplication
    private var screenshotCounter: Int = 0

    public init(app: XCUIApplication? = nil) {
        self.app = app ?? XCUIApplication()
    }

    // MARK: - Launch

    /// Launch the app with optional arguments
    public func launch(
        arguments: [String] = [],
        environment: [String: String] = [:],
        cleanState: Bool = false
    ) -> Self {
        app.launchArguments = arguments
        if cleanState {
            app.launchArguments.append("--reset-onboarding")
        }
        app.launchEnvironment = environment
        app.launch()
        return self
    }

    /// Launch in UI testing mode
    public func launchForUITesting(
        resetState: Bool = false,
        skipWake: Bool = true,
        forceOnboarding: Bool = false
    ) -> Self {
        var args = ["--ui-testing"]
        if resetState { args.append("--reset-onboarding") }
        if skipWake { args.append("--skip-wake") }
        if forceOnboarding { args.append("--force-onboarding-v2") }
        return launch(arguments: args)
    }

    // MARK: - Navigation

    /// Navigate to settings
    public func goToSettings() -> Self {
        app.buttons["settings"].tap()
        return self
    }

    /// Navigate to connectors
    public func goToConnectors() -> Self {
        app.buttons["nav.connectors"].tap()
        return self
    }

    /// Navigate to rules
    public func goToRules() -> Self {
        app.buttons["nav.rules"].tap()
        return self
    }

    /// Navigate to rituals
    public func goToRituals() -> Self {
        app.buttons["nav.rituals"].tap()
        return self
    }

    // MARK: - Connectors

    /// Connect a specific connector (e.g., "Canvas", "GitHub", "Todoist")
    public func connectConnector(named name: String) throws {
        // Try tapping the connector button or row
        let connectorButton = app.buttons["connector.\(name.lowercased())"]
            .or(app.buttons[name])
            .or(app.staticTexts[name])

        if connectorButton.waitForExistence(timeout: 5) {
            connectorButton.tap()
        } else {
            throw AppDriverError.elementNotFound("Connector: \(name)")
        }
    }

    /// Complete OAuth flow (generic)
    public func completeOAuthFlow(provider: String) throws {
        // Wait for web view or auth dialog
        let webView = app.webViews.firstMatch
        XCTAssertTrue(webView.waitForExistence(timeout: 10), "OAuth web view should appear")

        // Fill in credentials (test credentials)
        let emailField = webView.textFields.firstMatch
        if emailField.waitForExistence(timeout: 5) {
            emailField.tap()
            emailField.typeText("test@example.com")
        }

        // Tap authorize button
        let authorizeButton = webView.buttons["Authorize"]
            .or(webView.buttons["Allow"])
            .or(app.buttons["Authorize"])
        XCTAssertTrue(authorizeButton.waitForExistence(timeout: 10))
        authorizeButton.tap()
    }

    // MARK: - Rules

    /// Create a new rule
    public func createRule(name: String, configure: (RuleBuilder) -> Void) throws -> Self {
        // Tap add rule button
        let addButton = app.buttons["rule.add"]
            .or(app.buttons["Add Rule"])
            .or(app.buttons["+"])
        XCTAssertTrue(addButton.waitForExistence(timeout: 5))
        addButton.tap()

        // Enter rule name
        let nameField = app.textFields["rule.name"]
            .or(app.textFields.firstMatch)
        if nameField.waitForExistence(timeout: 5) {
            nameField.tap()
            nameField.clearText()
            nameField.typeText(name)
        }

        // Configure rule
        let builder = RuleBuilder(app: app)
        configure(builder)

        return self
    }

    /// Trigger an event manually (for testing)
    public func triggerEvent(named eventName: String) throws {
        // This would require app support or private API
        // For now, use launch arguments
        app.launchArguments.append("--trigger-event=\(eventName)")
        app.terminate()
        app.launch()
    }

    // MARK: - Rituals

    /// Start morning brief
    public func startMorningBrief() throws {
        let startButton = app.buttons["morning-brief.start"]
            .or(app.buttons["Start"])
        XCTAssertTrue(startButton.waitForExistence(timeout: 5), "Morning brief start button should appear")
        startButton.tap()
    }

    /// Complete evening shutdown
    public func completeEveningShutdown() throws {
        let doneButton = app.buttons["evening.shutdown.done"]
            .or(app.buttons["Done"])
            .or(app.buttons["Finish"])
        XCTAssertTrue(doneButton.waitForExistence(timeout: 5))
        doneButton.tap()
    }

    /// Start focus session
    public func startFocusSession(duration: Int? = nil) throws {
        let startButton = app.buttons["focus.start"]
            .or(app.buttons["Start Focus"])
        XCTAssertTrue(startButton.waitForExistence(timeout: 5))
        startButton.tap()

        // Set duration if specified
        if let mins = duration {
            let durationPicker = app.datePickers.firstMatch
            if durationPicker.waitForExistence(timeout: 3) {
                durationPicker.pickerWheels.element(boundBy: 0).adjust(toPickerWheelValue: "\(mins)")
            }
        }
    }

    // MARK: - Assertions

    /// Assert a button or element exists
    public func assertExists(_ identifier: String, timeout: TimeInterval = 5) throws {
        let element = app.buttons[identifier]
            .or(app.staticTexts[identifier])
            .or(app.images[identifier])

        if !element.waitForExistence(timeout: timeout) {
            throw AppDriverError.assertionFailed("\(identifier) should exist")
        }
    }

    /// Assert text is visible
    public func assertTextVisible(_ text: String, timeout: TimeInterval = 5) throws {
        if !app.staticTexts[text].waitForExistence(timeout: timeout) {
            throw AppDriverError.assertionFailed("Text '\(text)' should be visible")
        }
    }

    // MARK: - Screenshot

    /// Take a screenshot with automatic naming
    public func screenshot(named name: String? = nil) throws {
        let filename = name ?? "screenshot-\(screenshotCounter)"
        screenshotCounter += 1

        let screenshot = app.screenshot()
        let fileManager = FileManager.default
        let outputDir = fileManager.temporaryDirectory.appendingPathComponent("FocalPointJourneys/screenshots")
        try? fileManager.createDirectory(at: outputDir, withIntermediateDirectories: true)

        let fileURL = outputDir.appendingPathComponent("\(filename).png")
        try screenshot.pngRepresentation.write(to: fileURL)
    }

    // MARK: - Helpers

    /// Wait for app to settle
    public func waitForIdle(timeout: TimeInterval = 2) {
        Thread.sleep(forTimeInterval: timeout)
    }

    /// Tap button by identifier
    public func tap(_ identifier: String) throws {
        let button = app.buttons[identifier]
        if !button.waitForExistence(timeout: 5) {
            throw AppDriverError.elementNotFound("Button: \(identifier)")
        }
        button.tap()
    }

    /// Type text into field
    public func type(_ text: String, into identifier: String) throws {
        let field = app.textFields[identifier]
            .or(appSecureTextFields[identifier])
            .or(app.textViews[identifier])

        if !field.waitForExistence(timeout: 5) {
            throw AppDriverError.elementNotFound("Text field: \(identifier)")
        }

        field.tap()
        field.clearText()
        field.typeText(text)
    }
}

// MARK: - Rule Builder

public class RuleBuilder: @unchecked Sendable {
    private let app: XCUIApplication

    init(app: XCUIApplication) {
        self.app = app
    }

    /// Add a trigger
    public func trigger(type: String, config: [String: Any] = [:]) {
        let addTriggerButton = app.buttons["rule.trigger.add"]
        if addTriggerButton.waitForExistence(timeout: 3) {
            addTriggerButton.tap()
        }
        // Select trigger type
        if let typeElement = app.buttons["trigger.\(type)"]
            .or(app.buttons[type]) {
            typeElement.tap()
        }
    }

    /// Add an action
    public func action(type: String, config: [String: Any] = [:]) {
        let addActionButton = app.buttons["rule.action.add"]
        if addActionButton.waitForExistence(timeout: 3) {
            addActionButton.tap()
        }
        // Select action type
        if let typeElement = app.buttons["action.\(type)"]
            .or(app.buttons[type]) {
            typeElement.tap()
        }
    }

    /// Save the rule
    public func save() throws {
        let saveButton = app.buttons["rule.save"]
            .or(app.buttons["Save"])
        if !saveButton.waitForExistence(timeout: 5) {
            throw AppDriverError.elementNotFound("Save button")
        }
        saveButton.tap()
    }
}

// MARK: - Errors

public enum AppDriverError: Error, LocalizedError {
    case elementNotFound(String)
    case assertionFailed(String)
    case navigationFailed(String)

    public var errorDescription: String? {
        switch self {
        case .elementNotFound(let element):
            return "Element not found: \(element)"
        case .assertionFailed(let message):
            return "Assertion failed: \(message)"
        case .navigationFailed(let action):
            return "Navigation failed: \(action)"
        }
    }
}

// MARK: - XCUIElement Extension

extension XCUIElement {
    @discardableResult
    func or(_ other: XCUIElement) -> XCUIElement {
        if self.exists { return self }
        return other
    }

    func clearText() {
        // Select all and delete
        let selectAll = XCUIKeyboardKeyCommand.selectAll()
        typeKey(.delete, modifiers: [])
        tap()
        // Try clear
        if let value = self.value as? String, !value.isEmpty {
            for _ in 0..<value.count {
                typeKey(.delete, modifiers: [])
            }
        }
    }
}
