/*!
Desktop Recording Engine for Professional Video Capture
Provides high-quality screen recording during automation
*/

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::process::Stdio;
use tokio::process::Command as AsyncCommand;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingConfig {
    pub output_file: String,
    pub quality: RecordingQuality,
    pub framerate: u32,
    pub resolution: Option<(u32, u32)>,
    pub audio_enabled: bool,
    pub duration_limit: Option<u64>, // seconds
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordingQuality {
    Low,
    Medium,
    High,
    Ultra,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveRecording {
    pub recording_id: String,
    pub config: RecordingConfig,
    pub start_time: f64,
    pub status: RecordingStatus,
    pub output_path: String,
    pub process_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordingStatus {
    Starting,
    Recording,
    Stopping,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingResult {
    pub success: bool,
    pub output_file: String,
    pub duration: f64,
    pub file_size: u64,
    pub metadata: HashMap<String, String>,
}

/// Desktop Recording Engine for automation video capture
pub struct RecordingEngine {
    pub active_recordings: RwLock<HashMap<String, ActiveRecording>>,
    pub default_config: RecordingConfig,
}

impl RecordingEngine {
    pub fn new() -> Result<Self> {
        let default_config = RecordingConfig {
            output_file: "automation_recording.mp4".to_string(),
            quality: RecordingQuality::High,
            framerate: 30,
            resolution: None, // Use screen resolution
            audio_enabled: false,
            duration_limit: Some(300), // 5 minutes max
        };

        Ok(Self {
            active_recordings: RwLock::new(HashMap::new()),
            default_config,
        })
    }

    /// Start recording desktop with configuration
    pub async fn start_recording(&self, recording_id: &str, output_file: &str) -> Result<()> {
        self.start_recording_with_config(
            recording_id,
            &RecordingConfig {
                output_file: output_file.to_string(),
                ..self.default_config.clone()
            },
        )
        .await
    }

    /// Start recording with custom configuration
    pub async fn start_recording_with_config(
        &self,
        recording_id: &str,
        config: &RecordingConfig,
    ) -> Result<()> {
        info!("📹 Starting desktop recording: {}", recording_id);

        // Check if recording already exists
        {
            let recordings = self.active_recordings.read().await;
            if recordings.contains_key(recording_id) {
                return Err(anyhow::anyhow!("Recording already exists: {}", recording_id));
            }
        }

        // Create recording entry
        let recording = ActiveRecording {
            recording_id: recording_id.to_string(),
            config: config.clone(),
            start_time: chrono::Utc::now().timestamp() as f64,
            status: RecordingStatus::Starting,
            output_path: config.output_file.clone(),
            process_id: None,
        };

        // Insert into active recordings
        {
            let mut recordings = self.active_recordings.write().await;
            recordings.insert(recording_id.to_string(), recording);
        }

        // Start the actual recording process
        match self.start_ffmpeg_recording(recording_id, config).await {
            Ok(process_id) => {
                let mut recordings = self.active_recordings.write().await;
                if let Some(recording) = recordings.get_mut(recording_id) {
                    recording.status = RecordingStatus::Recording;
                    recording.process_id = Some(process_id);
                }
                info!("✅ Recording started successfully: {}", recording_id);
            }
            Err(e) => {
                let mut recordings = self.active_recordings.write().await;
                if let Some(recording) = recordings.get_mut(recording_id) {
                    recording.status = RecordingStatus::Failed(e.to_string());
                }
                error!("❌ Failed to start recording: {}", e);
                return Err(e);
            }
        }

        Ok(())
    }

    /// Stop recording
    pub async fn stop_recording(&self, recording_id: &str) -> Result<RecordingResult> {
        info!("🛑 Stopping recording: {}", recording_id);

        let (process_id, output_path, start_time) = {
            let mut recordings = self.active_recordings.write().await;
            if let Some(recording) = recordings.get_mut(recording_id) {
                recording.status = RecordingStatus::Stopping;
                (
                    recording.process_id,
                    recording.output_path.clone(),
                    recording.start_time,
                )
            } else {
                return Err(anyhow::anyhow!("Recording not found: {}", recording_id));
            }
        };

        // Stop the recording process
        if let Some(pid) = process_id {
            self.stop_ffmpeg_recording(pid).await?;
        }

        // Wait for file to be finalized
        sleep(Duration::from_secs(2)).await;

        // Get recording results
        let result = self.get_recording_result(&output_path, start_time).await?;

        // Update recording status
        {
            let mut recordings = self.active_recordings.write().await;
            if let Some(recording) = recordings.get_mut(recording_id) {
                recording.status = if result.success {
                    RecordingStatus::Completed
                } else {
                    RecordingStatus::Failed("Recording file not found".to_string())
                };
            }
        }

        info!("✅ Recording stopped: {} ({}s)", recording_id, result.duration);
        Ok(result)
    }

    /// Get recording status
    pub async fn get_recording_status(&self, recording_id: &str) -> Result<RecordingStatus> {
        let recordings = self.active_recordings.read().await;
        if let Some(recording) = recordings.get(recording_id) {
            Ok(recording.status.clone())
        } else {
            Err(anyhow::anyhow!("Recording not found: {}", recording_id))
        }
    }

    /// List active recordings
    pub async fn list_active_recordings(&self) -> Vec<String> {
        let recordings = self.active_recordings.read().await;
        recordings.keys().cloned().collect()
    }

    /// Start FFmpeg recording process
    async fn start_ffmpeg_recording(&self, _recording_id: &str, config: &RecordingConfig) -> Result<u32> {
        // Ensure output directory exists
        if let Some(parent) = Path::new(&config.output_file).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Build FFmpeg command
        let mut cmd = AsyncCommand::new("ffmpeg");
        
        // Input configuration
        cmd.arg("-f").arg("x11grab");
        
        // Frame rate
        cmd.arg("-r").arg(config.framerate.to_string());
        
        // Screen resolution and position
        if let Some((width, height)) = config.resolution {
            cmd.arg("-s").arg(format!("{}x{}", width, height));
        }
        
        // Display input
        cmd.arg("-i").arg(":0.0");
        
        // Video codec and quality settings
        match config.quality {
            RecordingQuality::Low => {
                cmd.arg("-c:v").arg("libx264");
                cmd.arg("-preset").arg("ultrafast");
                cmd.arg("-crf").arg("28");
            }
            RecordingQuality::Medium => {
                cmd.arg("-c:v").arg("libx264");
                cmd.arg("-preset").arg("fast");
                cmd.arg("-crf").arg("23");
            }
            RecordingQuality::High => {
                cmd.arg("-c:v").arg("libx264");
                cmd.arg("-preset").arg("medium");
                cmd.arg("-crf").arg("18");
            }
            RecordingQuality::Ultra => {
                cmd.arg("-c:v").arg("libx264");
                cmd.arg("-preset").arg("slow");
                cmd.arg("-crf").arg("15");
            }
        }
        
        // Audio configuration
        if config.audio_enabled {
            cmd.arg("-f").arg("pulse");
            cmd.arg("-i").arg("default");
            cmd.arg("-c:a").arg("aac");
            cmd.arg("-b:a").arg("128k");
        } else {
            cmd.arg("-an"); // No audio
        }
        
        // Duration limit
        if let Some(duration) = config.duration_limit {
            cmd.arg("-t").arg(duration.to_string());
        }
        
        // Output options
        cmd.arg("-pix_fmt").arg("yuv420p");
        cmd.arg("-movflags").arg("+faststart");
        
        // Overwrite output file
        cmd.arg("-y");
        
        // Output file
        cmd.arg(&config.output_file);
        
        // Set process options
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        
        info!("🎬 Starting FFmpeg: {:?}", cmd);
        
        // Start the process
        let mut child = cmd.spawn()?;
        
        // Get process ID
        let pid = child.id().ok_or_else(|| anyhow::anyhow!("Failed to get process ID"))?;
        
        // Detach the process (don't wait for it to finish)
        tokio::spawn(async move {
            let _ = child.wait().await;
        });
        
        info!("📹 FFmpeg process started with PID: {}", pid);
        Ok(pid)
    }

    /// Stop FFmpeg recording process
    async fn stop_ffmpeg_recording(&self, process_id: u32) -> Result<()> {
        info!("🛑 Stopping FFmpeg process: {}", process_id);
        
        // Send SIGTERM to gracefully stop FFmpeg
        let output = AsyncCommand::new("kill")
            .args(&["-TERM", &process_id.to_string()])
            .output()
            .await?;
        
        if !output.status.success() {
            warn!("Failed to send SIGTERM to process {}", process_id);
            
            // Force kill if graceful stop fails
            let output = AsyncCommand::new("kill")
                .args(&["-KILL", &process_id.to_string()])
                .output()
                .await?;
            
            if !output.status.success() {
                return Err(anyhow::anyhow!("Failed to stop recording process {}", process_id));
            }
        }
        
        info!("✅ FFmpeg process stopped: {}", process_id);
        Ok(())
    }

    /// Get recording result information
    async fn get_recording_result(&self, output_path: &str, start_time: f64) -> Result<RecordingResult> {
        let end_time = chrono::Utc::now().timestamp() as f64;
        let duration = end_time - start_time;
        
        // Check if file exists and get size
        let (success, file_size) = match tokio::fs::metadata(output_path).await {
            Ok(metadata) => (true, metadata.len()),
            Err(_) => (false, 0),
        };
        
        let mut metadata = HashMap::new();
        metadata.insert("start_time".to_string(), start_time.to_string());
        metadata.insert("end_time".to_string(), end_time.to_string());
        metadata.insert("recording_engine".to_string(), "ffmpeg".to_string());
        
        // Get video information if file exists
        if success {
            if let Ok(video_info) = self.get_video_info(output_path).await {
                metadata.extend(video_info);
            }
        }
        
        Ok(RecordingResult {
            success,
            output_file: output_path.to_string(),
            duration,
            file_size,
            metadata,
        })
    }

    /// Get video file information using ffprobe
    async fn get_video_info(&self, video_path: &str) -> Result<HashMap<String, String>> {
        let output = AsyncCommand::new("ffprobe")
            .args(&[
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                "-show_streams",
                video_path,
            ])
            .output()
            .await?;
        
        if output.status.success() {
            let json_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(info) = serde_json::from_str::<serde_json::Value>(&json_str) {
                let mut video_info = HashMap::new();
                
                // Extract format information
                if let Some(format) = info.get("format") {
                    if let Some(duration) = format.get("duration").and_then(|d| d.as_str()) {
                        video_info.insert("video_duration".to_string(), duration.to_string());
                    }
                    if let Some(size) = format.get("size").and_then(|s| s.as_str()) {
                        video_info.insert("file_size_bytes".to_string(), size.to_string());
                    }
                    if let Some(bitrate) = format.get("bit_rate").and_then(|b| b.as_str()) {
                        video_info.insert("bitrate".to_string(), bitrate.to_string());
                    }
                }
                
                // Extract video stream information
                if let Some(streams) = info.get("streams").and_then(|s| s.as_array()) {
                    for stream in streams {
                        if let Some(codec_type) = stream.get("codec_type").and_then(|c| c.as_str()) {
                            if codec_type == "video" {
                                if let Some(width) = stream.get("width").and_then(|w| w.as_i64()) {
                                    video_info.insert("width".to_string(), width.to_string());
                                }
                                if let Some(height) = stream.get("height").and_then(|h| h.as_i64()) {
                                    video_info.insert("height".to_string(), height.to_string());
                                }
                                if let Some(fps) = stream.get("r_frame_rate").and_then(|f| f.as_str()) {
                                    video_info.insert("framerate".to_string(), fps.to_string());
                                }
                                if let Some(codec) = stream.get("codec_name").and_then(|c| c.as_str()) {
                                    video_info.insert("video_codec".to_string(), codec.to_string());
                                }
                            }
                        }
                    }
                }
                
                return Ok(video_info);
            }
        }
        
        Ok(HashMap::new())
    }

    /// Convert video to different format/quality
    pub async fn convert_video(&self, input_path: &str, output_path: &str, quality: RecordingQuality) -> Result<()> {
        info!("🔄 Converting video: {} -> {}", input_path, output_path);
        
        let mut cmd = AsyncCommand::new("ffmpeg");
        cmd.arg("-i").arg(input_path);
        
        // Quality settings
        match quality {
            RecordingQuality::Low => {
                cmd.arg("-c:v").arg("libx264");
                cmd.arg("-crf").arg("28");
                cmd.arg("-preset").arg("fast");
            }
            RecordingQuality::Medium => {
                cmd.arg("-c:v").arg("libx264");
                cmd.arg("-crf").arg("23");
                cmd.arg("-preset").arg("medium");
            }
            RecordingQuality::High => {
                cmd.arg("-c:v").arg("libx264");
                cmd.arg("-crf").arg("18");
                cmd.arg("-preset").arg("medium");
            }
            RecordingQuality::Ultra => {
                cmd.arg("-c:v").arg("libx264");
                cmd.arg("-crf").arg("15");
                cmd.arg("-preset").arg("slow");
            }
        }
        
        cmd.arg("-y"); // Overwrite output
        cmd.arg(output_path);
        
        let output = cmd.output().await?;
        
        if output.status.success() {
            info!("✅ Video conversion completed: {}", output_path);
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Video conversion failed: {}", error))
        }
    }

    /// Create GIF from video
    pub async fn create_gif(&self, video_path: &str, gif_path: &str, start_time: Option<f64>, duration: Option<f64>) -> Result<()> {
        info!("🎞️ Creating GIF: {} -> {}", video_path, gif_path);
        
        let mut cmd = AsyncCommand::new("ffmpeg");
        cmd.arg("-i").arg(video_path);
        
        // Start time
        if let Some(start) = start_time {
            cmd.arg("-ss").arg(start.to_string());
        }
        
        // Duration
        if let Some(dur) = duration {
            cmd.arg("-t").arg(dur.to_string());
        }
        
        // GIF settings
        cmd.arg("-vf")
           .arg("fps=10,scale=800:-1:flags=lanczos,palettegen");
        
        let palette_path = "/tmp/palette.png";
        cmd.arg(palette_path);
        cmd.arg("-y");
        
        // Generate palette
        let output = cmd.output().await?;
        if !output.status.success() {
            return Err(anyhow::anyhow!("Failed to generate palette"));
        }
        
        // Create GIF with palette
        let mut cmd = AsyncCommand::new("ffmpeg");
        cmd.arg("-i").arg(video_path);
        cmd.arg("-i").arg(palette_path);
        
        if let Some(start) = start_time {
            cmd.arg("-ss").arg(start.to_string());
        }
        if let Some(dur) = duration {
            cmd.arg("-t").arg(dur.to_string());
        }
        
        cmd.arg("-lavfi")
           .arg("fps=10,scale=800:-1:flags=lanczos[x];[x][1:v]paletteuse");
        cmd.arg(gif_path);
        cmd.arg("-y");
        
        let output = cmd.output().await?;
        
        // Clean up palette
        let _ = tokio::fs::remove_file(palette_path).await;
        
        if output.status.success() {
            info!("✅ GIF created: {}", gif_path);
            Ok(())
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("GIF creation failed: {}", error))
        }
    }

    /// Extract frames from video
    pub async fn extract_frames(&self, video_path: &str, output_dir: &str, frame_rate: Option<f64>) -> Result<Vec<String>> {
        info!("📸 Extracting frames from: {}", video_path);
        
        // Create output directory
        tokio::fs::create_dir_all(output_dir).await?;
        
        let mut cmd = AsyncCommand::new("ffmpeg");
        cmd.arg("-i").arg(video_path);
        
        // Frame rate
        if let Some(fps) = frame_rate {
            cmd.arg("-vf").arg(format!("fps={}", fps));
        }
        
        let output_pattern = format!("{}/frame_%04d.png", output_dir);
        cmd.arg(&output_pattern);
        cmd.arg("-y");
        
        let output = cmd.output().await?;
        
        if output.status.success() {
            // List generated frames
            let mut frames = Vec::new();
            let mut entries = tokio::fs::read_dir(output_dir).await?;
            
            while let Some(entry) = entries.next_entry().await? {
                if let Some(filename) = entry.file_name().to_str() {
                    if filename.starts_with("frame_") && filename.ends_with(".png") {
                        frames.push(entry.path().to_string_lossy().to_string());
                    }
                }
            }
            
            frames.sort();
            info!("✅ Extracted {} frames", frames.len());
            Ok(frames)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Frame extraction failed: {}", error))
        }
    }
}

impl Default for RecordingEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create recording engine")
    }
}