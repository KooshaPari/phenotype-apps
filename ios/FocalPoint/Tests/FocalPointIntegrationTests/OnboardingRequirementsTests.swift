import XCTest
@testable import FocalPointCore

@MainActor
final class OnboardingRequirementsTests: XCTestCase {
    override func setUp() {
        super.setUp()
        OnboardingResumeState.resetTracking()
    }

    override func tearDown() {
        OnboardingResumeState.resetTracking()
        super.tearDown()
    }

    func testDefaultFirstRunStartsAtCoachyWelcome() {
        let coordinator = OnboardingCoordinator()
        XCTAssertEqual(coordinator.step, .welcome)
        XCTAssertTrue(coordinator.canAdvance)
    }

    func testLegacyConsentStartRequiresPrivacyAndTerms() {
        let coordinator = OnboardingCoordinator(initialStep: .consent)
        XCTAssertEqual(coordinator.step, .consent)
        XCTAssertFalse(coordinator.canAdvance)

        coordinator.privacyAccepted = true
        XCTAssertFalse(coordinator.canAdvance)

        coordinator.termsAccepted = true
        XCTAssertTrue(coordinator.canAdvance)
    }

    func testGoalSelectionHasMinimumAndMaximum() {
        let coordinator = OnboardingCoordinator()
        coordinator.jump(to: .goals)
        XCTAssertFalse(coordinator.canAdvance)

        coordinator.toggleGoal(.school)
        XCTAssertTrue(coordinator.canAdvance)

        coordinator.toggleGoal(.work)
        coordinator.toggleGoal(.sleep)
        coordinator.toggleGoal(.exercise)

        XCTAssertEqual(coordinator.goals.count, 3)
        XCTAssertFalse(coordinator.goals.contains(.exercise))
    }

    func testResumeStartsCleanAndMovesToNextVisibleStep() {
        XCTAssertEqual(OnboardingResumeState.getCurrentStepIndex(), -1)
        XCTAssertEqual(OnboardingResumeState.resumeStep(), .welcome)
        XCTAssertFalse(OnboardingResumeState.hasPartialProgress())
        XCTAssertEqual(OnboardingResumeState.getProgressLabel(), "Ready to start")

        OnboardingResumeState.completeStep(OnboardingCoordinator.Step.welcome.rawValue)
        XCTAssertTrue(OnboardingResumeState.hasPartialProgress())
        XCTAssertEqual(OnboardingResumeState.getProgressLabel(), "2/6 steps complete")
        XCTAssertEqual(OnboardingResumeState.resumeStep(), .goals)
    }

    func testResumeSummaryOnlyShowsCompletedRequirements() {
        OnboardingResumeState.completeStep(OnboardingCoordinator.Step.welcome.rawValue)
        XCTAssertEqual(
            OnboardingResumeState.completedSummaryItems().map(\.title),
            ["Coachy intro viewed"]
        )

        OnboardingResumeState.completeStep(OnboardingCoordinator.Step.goals.rawValue)
        OnboardingResumeState.completeStep(OnboardingCoordinator.Step.permissions.rawValue)
        XCTAssertEqual(
            OnboardingResumeState.completedSummaryItems().map(\.title),
            [
                "Coachy intro viewed",
                "Focus goals selected",
                "Calendar step reviewed",
                "Focus template chosen",
                "Permissions reviewed",
            ]
        )
    }

    func testDoneStepIsTerminalNotPartialProgress() {
        OnboardingResumeState.completeStep(OnboardingCoordinator.Step.done.rawValue)
        XCTAssertFalse(OnboardingResumeState.hasPartialProgress())
        XCTAssertEqual(OnboardingResumeState.resumeStep(), .done)
    }

    func testCompleteAndSeedFallsBackToDeepWorkBaseline() throws {
        let dir = FileManager.default.temporaryDirectory
            .appendingPathComponent("onboarding-seed-\(UUID().uuidString)", isDirectory: true)
        try FileManager.default.createDirectory(at: dir, withIntermediateDirectories: true)
        defer { try? FileManager.default.removeItem(at: dir) }

        let core = try FocalPointCore(storagePath: dir.appendingPathComponent("core.db").path)
        let coordinator = OnboardingCoordinator()

        let installed = try coordinator.completeAndSeed(into: core)
        XCTAssertEqual(installed, 1)
        XCTAssertTrue(try core.rules().listEnabled().contains { $0.name == "Deep work — no social" })
    }
}
