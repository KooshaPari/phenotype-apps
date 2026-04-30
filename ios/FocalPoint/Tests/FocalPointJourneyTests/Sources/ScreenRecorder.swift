// ScreenRecorder.swift — ScreenCaptureKit wrapper for FocalPoint journey recording
// Enables MP4 video recording of UI tests with FFmpeg keyframe extraction

import Foundation
import ScreenCaptureKit
import AVFoundation
import XCTest

#if canImport(UIKit)
import UIKit
#endif

// MARK: - Screen Recorder

/// Records the screen during journey execution using ScreenCaptureKit
@available(macOS 13.0, iOS 16.0, *)
public class ScreenRecorder: NSObject, @unchecked Sendable {
    private var stream: SCStream?
    private var assetWriter: AVAssetWriter?
    private var videoInput: AVAssetWriterInput?
    private var isRecording = false
    private var outputURL: URL?
    private var startTime: CMTime?

    public var recordingDirectory: URL

    public override init() {
        self.recordingDirectory = FileManager.default.temporaryDirectory
            .appendingPathComponent("FocalPointJourneys")
            .appendingPathComponent("recordings")

        super.init()

        // Ensure recording directory exists
        try? FileManager.default.createDirectory(
            at: recordingDirectory,
            withIntermediateDirectories: true
        )
    }

    // MARK: - Start Recording

    public func startRecording(journeyId: String) async throws {
        guard !isRecording else {
            throw ScreenRecorderError.alreadyRecording
        }

        let filename = "\(journeyId).mp4"
        outputURL = recordingDirectory.appendingPathComponent(filename)

        guard let url = outputURL else {
            throw ScreenRecorderError.noOutputURL
        }

        // Remove existing file
        try? FileManager.default.removeItem(at: url)

        // Create asset writer
        assetWriter = try AVAssetWriter(outputURL: url, fileType: .mp4)

        // Video settings
        let videoSettings: [String: Any] = [
            AVVideoCodecKey: AVVideoCodecType.h264,
            AVVideoWidthKey: 1170,  // iPhone 14 Pro width
            AVVideoHeightKey: 2532, // iPhone 14 Pro height
            AVVideoCompressionPropertiesKey: [
                AVVideoAverageBitRateKey: 10_000_000,
                AVVideoProfileLevelKey: AVVideoProfileLevelH264HighAutoLevel
            ]
        ]

        videoInput = AVAssetWriterInput(mediaType: .video, outputSettings: videoSettings)
        videoInput?.expectsMediaDataInRealTime = true

        if let input = videoInput {
            assetWriter?.add(input)
        }

        assetWriter?.startWriting()

        isRecording = true
        startTime = nil

        print("🎬 Started recording: \(filename)")

        // Start screen capture
        try await startScreenCapture()
    }

    // MARK: - Stop Recording

    public func stopRecording() async throws -> URL? {
        guard isRecording else {
            return nil
        }

        isRecording = false

        // Stop screen capture
        stream?.stopCapture()

        // Finish writing
        videoInput?.markAsFinished()
        await assetWriter?.finishWriting()

        if assetWriter?.status == .completed {
            print("🎬 Stopped recording: \(outputURL?.lastPathComponent ?? "unknown")")
            return outputURL
        } else if let error = assetWriter?.error {
            throw ScreenRecorderError.recordingFailed(error.localizedDescription)
        }

        return nil
    }

    // MARK: - Screenshot (Quick Capture)

    public func captureScreenshot() async throws -> URL {
        let screenshot = XCUIScreen.main.screenshot()
        let filename = "quick-capture-\(Date().timeIntervalSince1970).png"
        let url = recordingDirectory.appendingPathComponent(filename)
        try screenshot.pngRepresentation.write(to: url)
        return url
    }

    // MARK: - Private: Screen Capture

    private func startScreenCapture() async throws {
        // Get available content
        let content = try await SCShareableContent.excludingDesktopWindows(
            false,
            onScreenWindowsOnly: true
        )

        // Find FocalPoint window
        let focalPointApp = content.applications.first { app in
            app.bundleIdentifier.contains("FocalPoint")
        }

        guard let app = focalPointApp else {
            throw ScreenRecorderError.appNotFound
        }

        let window = app.windows.first

        // Create content filter
        let filter: SCContentFilter
        if let win = window {
            filter = SCContentFilter(display: content.displays.first!, includingWindows: [win])
        } else {
            filter = SCContentFilter(display: content.displays.first!, includingWindows: [])
        }

        // Configure stream
        let config = SCStreamConfiguration()
        config.width = 1170
        config.height = 2532
        config.minimumFrameInterval = CMTime(value: 1, timescale: 30) // 30 fps
        config.pixelFormat = kCVPixelFormatType_32BGRA
        config.showsCursor = false

        // Create stream
        stream = SCStream(filter: filter, configuration: config, delegate: self)

        // Start capture
        try await stream?.startCapture()
    }

    // MARK: - Generate GIF Preview

    public func generateGifPreview(from mp4URL: URL) async throws -> URL {
        let filename = mp4URL.deletingPathExtension().lastPathComponent + ".gif"
        let gifURL = recordingDirectory.appendingPathComponent(filename)

        // Use ffmpeg to convert MP4 to GIF
        let process = Process()
        process.executableURL = URL(fileURLWithPath: "/opt/homebrew/bin/ffmpeg")

        // Optimized GIF settings for docs
        let args = [
            "-i", mp4URL.path,
            "-vf", "fps=10,scale=585:-1:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=256:stats_mode=full[p];[s1][p]paletteuse=dither=bayer:bayer_scale=3",
            "-loop", "0",
            gifURL.path
        ]
        process.arguments = args

        try process.run()
        process.waitUntilExit()

        if process.terminationStatus == 0 {
            print("🎬 Generated GIF preview: \(filename)")
            return gifURL
        } else {
            throw ScreenRecorderError.gifGenerationFailed
        }
    }
}

// MARK: - SCStreamDelegate

@available(macOS 13.0, iOS 16.0, *)
extension ScreenRecorder: SCStreamDelegate {
    public func stream(
        _ stream: SCStream,
        didOutputSampleBuffer sampleBuffer: CMSampleBuffer,
        of type: SCStreamOutputType
    ) {
        guard type == .screen,
              isRecording,
              let pixelBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else {
            return
        }

        // Track start time
        if startTime == nil {
            startTime = CMSampleBufferGetPresentationTimeStamp(sampleBuffer)
            assetWriter?.startSession(atSourceTime: startTime!)
        }

        // Write frame
        if videoInput?.isReadyForMoreMediaData == true {
            videoInput?.append(sampleBuffer)
        }
    }
}

// MARK: - Errors

public enum ScreenRecorderError: Error, LocalizedError {
    case alreadyRecording
    case noOutputURL
    case appNotFound
    case recordingFailed(String)
    case gifGenerationFailed

    public var errorDescription: String? {
        switch self {
        case .alreadyRecording:
            return "Recording already in progress"
        case .noOutputURL:
            return "No output URL set"
        case .appNotFound:
            return "FocalPoint app not found in screen shareable content"
        case .recordingFailed(let message):
            return "Recording failed: \(message)"
        case .gifGenerationFailed:
            return "Failed to generate GIF preview"
        }
    }
}

// MARK: - ScreenRecorderManager

/// Manages screen recording for a journey run
@available(macOS 13.0, iOS 16.0, *)
public class ScreenRecorderManager: @unchecked Sendable {
    public static let shared = ScreenRecorderManager()

    private var recorder: ScreenRecorder?

    private init() {}

    public func startRecording(journeyId: String) async throws {
        recorder = ScreenRecorder()
        try await recorder?.startRecording(journeyId: journeyId)
    }

    public func stopRecording() async throws -> URL? {
        let url = try await recorder?.stopRecording()

        // Generate GIF preview
        if let mp4URL = url {
            _ = try? await recorder?.generateGifPreview(from: mp4URL)
        }

        return url
    }

    public func getRecordingDirectory() -> URL? {
        return recorder?.recordingDirectory
    }
}
