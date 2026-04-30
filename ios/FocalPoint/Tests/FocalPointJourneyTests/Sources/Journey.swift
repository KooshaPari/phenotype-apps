// Journey.swift — Journey DSL for FocalPoint UI Testing
// Enables declarative journey definitions with intent labels, screenshots, and manifests

import XCTest
import Foundation

// MARK: - Journey Step

/// Represents a single step in a user journey
public struct JourneyStep: Codable, Identifiable {
    public let id: UUID
    public let index: Int
    public let slug: String
    public let intent: String
    public let action: () async throws -> Void
    public var screenshotPath: String?
    public var passed: Bool = false
    public var error: String?

    public init(
        index: Int,
        slug: String,
        intent: String,
        action: @escaping () async throws -> Void
    ) {
        self.id = UUID()
        self.index = index
        self.slug = slug
        self.intent = intent
        self.action = action
        self.screenshotPath = nil
    }

    enum CodingKeys: String, CodingKey {
        case id, index, slug, intent, screenshotPath, passed, error
    }
}

// MARK: - Journey

/// Represents a complete user journey with steps, metadata, and recording
public class Journey: XCTestCase, @unchecked Sendable {
    public let journeyId: String
    public let title: String
    public let persona: String
    public let platform: String

    private var steps: [JourneyStep] = []
    private var currentStepIndex: Int = 0

    public var outputDirectory: URL
    public var manifest: JourneyManifest?

    // MARK: - Initialization

    public init(
        journeyId: String,
        title: String,
        persona: String,
        platform: String = "ios"
    ) {
        self.journeyId = journeyId
        self.title = title
        self.persona = persona
        self.platform = platform

        let outputPath = FileManager.default.temporaryDirectory
            .appendingPathComponent("FocalPointJourneys")
            .appendingPathComponent(journeyId)
        self.outputDirectory = outputPath

        super.init()

        // Create output directory
        try? FileManager.default.createDirectory(
            at: outputPath,
            withIntermediateDirectories: true
        )
    }

    // MARK: - Journey DSL

    /// Add a step to the journey
    public func step(
        slug: String,
        intent: String,
        action: @escaping () async throws -> Void
    ) -> Self {
        let step = JourneyStep(
            index: steps.count,
            slug: slug,
            intent: intent,
            action: action
        )
        steps.append(step)
        return self
    }

    /// Add a step that includes a screenshot
    public func stepWithScreenshot(
        slug: String,
        intent: String,
        screenshotName: String? = nil,
        action: @escaping () async throws -> Void
    ) -> Self {
        let name = screenshotName ?? "\(journeyId)-\(slug)"
        return step(slug: slug, intent: intent) {
            try await self.captureScreenshot(named: name)
            try await action()
        }
    }

    // MARK: - Execution

    /// Run all steps in the journey
    public func run() async throws {
        print("🧪 Starting journey: \(journeyId)")
        print("   Persona: \(persona)")
        print("   Steps: \(steps.count)")

        for (index, var step) in steps.enumerated() {
            currentStepIndex = index
            print("   Step \(index + 1)/\(steps.count): \(step.slug)")

            do {
                try await step.action()
                step.passed = true
                print("      ✓ \(step.intent)")
            } catch {
                step.error = error.localizedDescription
                step.passed = false
                print("      ✗ Error: \(error.localizedDescription)")
                throw JourneyError.stepFailed(step: step, underlying: error)
            }

            steps[index] = step
        }

        // Generate manifest
        manifest = try await generateManifest()

        print("   ✅ Journey complete: \(journeyId)")
    }

    // MARK: - Screenshots

    /// Capture a screenshot of the current screen
    public func captureScreenshot(named name: String) async throws {
        let screenshot = XCUIScreen.main.screenshot()
        let filename = "\(name).png"
        let fileURL = outputDirectory.appendingPathComponent(filename)

        try screenshot.pngRepresentation.write(to: fileURL)

        if currentStepIndex < steps.count {
            steps[currentStepIndex].screenshotPath = filename
        }

        print("      📸 Captured: \(filename)")
    }

    /// Capture screenshot with delay for animations
    public func captureScreenshotDelayed(
        named name: String,
        delay: TimeInterval = 0.5
    ) async throws {
        try await Task.sleep(nanoseconds: UInt64(delay * 1_000_000_000))
        try await captureScreenshot(named: name)
    }

    // MARK: - Manifest Generation

    public struct JourneyManifest: Codable {
        public let id: String
        public let title: String
        public let persona: String
        public let platform: String
        public let intent: String
        public let steps: [StepManifest]
        public let recording: String?
        public let recordingGif: String?
        public let keyframeCount: Int
        public let passed: Bool
        public let generatedAt: String

        public struct StepManifest: Codable {
            public let index: Int
            public let slug: String
            public let intent: String
            public let screenshotPath: String?
        }
    }

    private func generateManifest() async throws -> JourneyManifest {
        let stepManifests = steps.map { step in
            JourneyManifest.StepManifest(
                index: step.index,
                slug: step.slug,
                intent: step.intent,
                screenshotPath: step.screenshotPath
            )
        }

        let manifest = JourneyManifest(
            id: journeyId,
            title: title,
            persona: persona,
            platform: platform,
            intent: "User journey for \(persona): \(steps.map { $0.intent }.joined(separator: " → "))",
            steps: stepManifests,
            recording: "recordings/\(journeyId).mp4",
            recordingGif: "recordings/\(journeyId).gif",
            keyframeCount: steps.filter { $0.screenshotPath != nil }.count,
            passed: steps.allSatisfy { $0.passed },
            generatedAt: ISO8601DateFormatter().string(from: Date())
        )

        // Write manifest to disk
        let encoder = JSONEncoder()
        encoder.outputFormatting = [.prettyPrinted, .sortedKeys]
        let data = try encoder.encode(manifest)
        let manifestURL = outputDirectory.appendingPathComponent("manifest.json")
        try data.write(to: manifestURL)

        print("      📄 Generated manifest: manifest.json")

        return manifest
    }
}

// MARK: - Journey Error

public enum JourneyError: Error, LocalizedError {
    case stepFailed(step: JourneyStep, underlying: Error)
    case recordingFailed(String)
    case manifestGenerationFailed(Error)

    public var errorDescription: String? {
        switch self {
        case .stepFailed(let step, let error):
            return "Step '\(step.slug)' failed: \(error.localizedDescription)"
        case .recordingFailed(let message):
            return "Recording failed: \(message)"
        case .manifestGenerationFailed(let error):
            return "Manifest generation failed: \(error.localizedDescription)"
        }
    }
}

// MARK: - Journey Builder

/// Fluent builder for creating journeys
public func journey(
    id: String,
    title: String,
    persona: String,
    platform: String = "ios",
    configure: (Journey) -> Void
) -> Journey {
    let j = Journey(journeyId: id, title: title, persona: persona, platform: platform)
    configure(j)
    return j
}
