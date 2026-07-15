// Advanced FFmpeg Video Recording and Processing Pipeline
// High-quality recording with real-time optimization and GIF conversion

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs;
use tokio::process::Command as AsyncCommand;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingConfig {
    pub output_directory: String,
    pub video_codec: String,
    pub audio_codec: String,
    pub video_quality: VideoQuality,
    pub frame_rate: u32,
    pub resolution: Resolution,
    pub audio_enabled: bool,
    pub hardware_acceleration: bool,
    pub real_time_optimization: bool,
    pub auto_gif_generation: bool,
    pub gif_optimization_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoQuality {
    Low,      // Fast encoding, larger files
    Medium,   // Balanced
    High,     // Slow encoding, smaller files
    Lossless, // No compression
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

impl Default for RecordingConfig {
    fn default() -> Self {
        Self {
            output_directory: "/tmp/kvirtualstage_recordings".to_string(),
            video_codec: "libx264".to_string(),
            audio_codec: "aac".to_string(),
            video_quality: VideoQuality::High,
            frame_rate: 30,
            resolution: Resolution { width: 1920, height: 1080 },
            audio_enabled: true,
            hardware_acceleration: true,
            real_time_optimization: true,
            auto_gif_generation: true,
            gif_optimization_level: 2,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct RecordingSession {
    pub session_id: String,
    #[serde(skip)]
    pub start_time: Instant,
    #[serde(skip)]
    pub end_time: Option<Instant>,
    pub output_path: PathBuf,
    pub config: RecordingConfig,
    pub status: RecordingStatus,
    pub metrics: RecordingMetrics,
}

impl Default for RecordingSession {
    fn default() -> Self {
        Self {
            session_id: String::new(),
            start_time: std::time::Instant::now(),
            end_time: None,
            output_path: std::path::PathBuf::new(),
            config: RecordingConfig::default(),
            status: RecordingStatus::Preparing,
            metrics: RecordingMetrics::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordingStatus {
    Preparing,
    Recording,
    Stopping,
    Processing,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingMetrics {
    pub duration: Duration,
    pub file_size_bytes: u64,
    pub average_fps: f64,
    pub peak_cpu_usage: f64,
    pub frame_drops: u32,
    pub audio_samples: u64,
}

impl Default for RecordingMetrics {
    fn default() -> Self {
        Self {
            duration: Duration::ZERO,
            file_size_bytes: 0,
            average_fps: 0.0,
            peak_cpu_usage: 0.0,
            frame_drops: 0,
            audio_samples: 0,
        }
    }
}

pub struct FFmpegPipeline {
    config: RecordingConfig,
    active_sessions: Arc<RwLock<HashMap<String, RecordingSession>>>,
    ffmpeg_path: String,
    capabilities: FFmpegCapabilities,
}

#[derive(Debug, Clone)]
struct FFmpegCapabilities {
    hardware_encoders: Vec<String>,
    supported_codecs: Vec<String>,
    filters_available: Vec<String>,
    version: String,
}

impl FFmpegPipeline {
    pub async fn new(config: RecordingConfig) -> Result<Self> {
        info!("Initializing FFmpeg Pipeline");
        
        // Verify FFmpeg installation
        let ffmpeg_path = Self::find_ffmpeg_binary().await?;
        let capabilities = Self::detect_ffmpeg_capabilities(&ffmpeg_path).await?;
        
        // Create output directory
        fs::create_dir_all(&config.output_directory).await?;
        
        info!("FFmpeg Pipeline initialized with version: {}", capabilities.version);
        
        Ok(Self {
            config,
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            ffmpeg_path,
            capabilities,
        })
    }
    
    async fn find_ffmpeg_binary() -> Result<String> {
        // Try common FFmpeg locations
        let potential_paths = [
            "ffmpeg",
            "/usr/bin/ffmpeg",
            "/usr/local/bin/ffmpeg",
            "/opt/homebrew/bin/ffmpeg",
            "/snap/bin/ffmpeg",
        ];
        
        for path in &potential_paths {
            if let Ok(output) = Command::new(path).args(["-version"]).output() {
                if output.status.success() {
                    return Ok(path.to_string());
                }
            }
        }
        
        Err(anyhow!("FFmpeg not found. Please install FFmpeg and ensure it's in your PATH"))
    }
    
    async fn detect_ffmpeg_capabilities(ffmpeg_path: &str) -> Result<FFmpegCapabilities> {
        let mut capabilities = FFmpegCapabilities {
            hardware_encoders: Vec::new(),
            supported_codecs: Vec::new(),
            filters_available: Vec::new(),
            version: String::new(),
        };
        
        // Get version
        if let Ok(output) = Command::new(ffmpeg_path).args(["-version"]).output() {
            let version_text = String::from_utf8_lossy(&output.stdout);
            if let Some(line) = version_text.lines().next() {
                capabilities.version = line.to_string();
            }
        }
        
        // Get available encoders
        if let Ok(output) = Command::new(ffmpeg_path).args(["-encoders"]).output() {
            let encoders_text = String::from_utf8_lossy(&output.stdout);
            for line in encoders_text.lines() {
                if line.contains("h264") || line.contains("hevc") || line.contains("vp9") {
                    if let Some(encoder) = line.split_whitespace().nth(1) {
                        capabilities.supported_codecs.push(encoder.to_string());
                    }
                }
                // Check for hardware encoders
                if line.contains("nvenc") || line.contains("qsv") || line.contains("videotoolbox") {
                    if let Some(encoder) = line.split_whitespace().nth(1) {
                        capabilities.hardware_encoders.push(encoder.to_string());
                    }
                }
            }
        }
        
        // Get available filters
        if let Ok(output) = Command::new(ffmpeg_path).args(["-filters"]).output() {
            let filters_text = String::from_utf8_lossy(&output.stdout);
            for line in filters_text.lines() {
                if line.contains("scale") || line.contains("fps") || line.contains("palettegen") {
                    if let Some(filter) = line.split_whitespace().nth(1) {
                        capabilities.filters_available.push(filter.to_string());
                    }
                }
            }
        }
        
        Ok(capabilities)
    }
    
    pub async fn start_recording(&self, session_id: String, display: Option<&str>) -> Result<RecordingSession> {
        info!("Starting recording session: {}", session_id);
        
        let display_env = display.unwrap_or(":0");
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let output_filename = format!("{}_{}.mp4", session_id, timestamp);
        let output_path = PathBuf::from(&self.config.output_directory).join(output_filename);
        
        // Build FFmpeg command
        let mut ffmpeg_args = Vec::new();

        // Input configuration
        let frame_rate_str = self.config.frame_rate.to_string();
        let resolution_str = format!("{}x{}", self.config.resolution.width, self.config.resolution.height);
        let display_str = format!("{}+0,0", display_env);

        ffmpeg_args.extend([
            "-f", "x11grab",
            "-r", &frame_rate_str,
            "-s", &resolution_str,
            "-i", &display_str,
        ]);
        
        // Audio input (if enabled)
        if self.config.audio_enabled {
            ffmpeg_args.extend([
                "-f", "pulse",
                "-i", "default",
            ]);
        }
        
        // Video encoding configuration
        let video_codec = self.select_optimal_video_codec();
        ffmpeg_args.extend(["-c:v", &video_codec]);
        
        // Quality settings
        self.add_quality_settings(&mut ffmpeg_args);
        
        // Hardware acceleration (if available)
        if self.config.hardware_acceleration && !self.capabilities.hardware_encoders.is_empty() {
            self.add_hardware_acceleration(&mut ffmpeg_args);
        }
        
        // Real-time optimization
        if self.config.real_time_optimization {
            ffmpeg_args.extend([
                "-preset", "ultrafast",
                "-tune", "zerolatency",
                "-threads", "0",
            ]);
        }
        
        // Audio encoding (if enabled)
        if self.config.audio_enabled {
            ffmpeg_args.extend([
                "-c:a", &self.config.audio_codec,
                "-b:a", "128k",
            ]);
        }
        
        // Output configuration
        ffmpeg_args.extend([
            "-y", // Overwrite output files
            output_path.to_str().unwrap(),
        ]);
        
        info!("FFmpeg command: {} {}", self.ffmpeg_path, ffmpeg_args.join(" "));
        
        // Start FFmpeg process
        let mut child = AsyncCommand::new(&self.ffmpeg_path)
            .args(&ffmpeg_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("DISPLAY", display_env)
            .spawn()?;
        
        let session = RecordingSession {
            session_id: session_id.clone(),
            start_time: Instant::now(),
            end_time: None,
            output_path,
            config: self.config.clone(),
            status: RecordingStatus::Recording,
            metrics: RecordingMetrics::default(),
        };
        
        // Store session
        {
            let mut sessions = self.active_sessions.write().await;
            sessions.insert(session_id.clone(), session.clone());
        }
        
        // Monitor recording in background
        let sessions_ref = Arc::clone(&self.active_sessions);
        let session_id_clone = session_id.clone();
        
        tokio::spawn(async move {
            let result = child.wait().await;
            
            let mut sessions = sessions_ref.write().await;
            if let Some(session) = sessions.get_mut(&session_id_clone) {
                match result {
                    Ok(exit_status) => {
                        if exit_status.success() {
                            session.status = RecordingStatus::Completed;
                            session.end_time = Some(Instant::now());
                            info!("Recording completed successfully: {}", session_id_clone);
                        } else {
                            session.status = RecordingStatus::Failed("FFmpeg process failed".to_string());
                            error!("Recording failed: {}", session_id_clone);
                        }
                    }
                    Err(e) => {
                        session.status = RecordingStatus::Failed(format!("Process error: {}", e));
                        error!("Recording process error: {}", e);
                    }
                }
            }
        });
        
        Ok(session)
    }
    
    pub async fn stop_recording(&self, session_id: &str) -> Result<RecordingSession> {
        info!("Stopping recording session: {}", session_id);
        
        let mut sessions = self.active_sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            if matches!(session.status, RecordingStatus::Recording) {
                session.status = RecordingStatus::Stopping;
                
                // Send quit command to FFmpeg (graceful shutdown)
                // In a real implementation, we'd send 'q' to stdin of the FFmpeg process
                
                // For now, we'll mark as processing
                session.status = RecordingStatus::Processing;
                session.end_time = Some(Instant::now());
                
                // Calculate final metrics
                session.metrics.duration = session.end_time.unwrap() - session.start_time;
                
                // Get file size
                if let Ok(metadata) = fs::metadata(&session.output_path).await {
                    session.metrics.file_size_bytes = metadata.len();
                }
                
                let final_session = session.clone();
                
                // Auto-generate GIF if enabled
                if self.config.auto_gif_generation {
                    let output_path = session.output_path.clone();
                    let gif_config = self.config.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::generate_gif_from_video(&output_path, &gif_config).await {
                            warn!("Failed to generate GIF: {}", e);
                        }
                    });
                }
                
                session.status = RecordingStatus::Completed;
                return Ok(final_session);
            }
        }
        
        Err(anyhow!("Recording session not found or not active: {}", session_id))
    }
    
    pub async fn get_session_status(&self, session_id: &str) -> Option<RecordingSession> {
        let sessions = self.active_sessions.read().await;
        sessions.get(session_id).cloned()
    }
    
    pub async fn list_active_sessions(&self) -> Vec<RecordingSession> {
        let sessions = self.active_sessions.read().await;
        sessions.values().cloned().collect()
    }
    
    async fn generate_gif_from_video(video_path: &Path, config: &RecordingConfig) -> Result<()> {
        info!("Generating GIF from video: {:?}", video_path);
        
        let gif_path = video_path.with_extension("gif");
        
        // Two-pass GIF generation for better quality
        // Pass 1: Generate palette
        let palette_path = video_path.with_extension("png");
        
        let palette_args = [
            "-i", video_path.to_str().unwrap(),
            "-vf", &format!("fps={},scale={}:{}:flags=lanczos,palettegen=max_colors=256", 
                           config.frame_rate / 2, // Reduce FPS for GIF
                           config.resolution.width / 2, // Reduce resolution for GIF
                           config.resolution.height / 2),
            "-y",
            palette_path.to_str().unwrap(),
        ];
        
        let output = Command::new("ffmpeg")
            .args(&palette_args)
            .output()?;
            
        if !output.status.success() {
            return Err(anyhow!("Failed to generate palette for GIF"));
        }
        
        // Pass 2: Generate GIF using palette
        let gif_args = [
            "-i", video_path.to_str().unwrap(),
            "-i", palette_path.to_str().unwrap(),
            "-lavfi", &format!("fps={},scale={}:{}:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer",
                              config.frame_rate / 2,
                              config.resolution.width / 2,
                              config.resolution.height / 2),
            "-y",
            gif_path.to_str().unwrap(),
        ];
        
        let output = Command::new("ffmpeg")
            .args(&gif_args)
            .output()?;
            
        // Clean up palette file
        let _ = fs::remove_file(&palette_path).await;
        
        if output.status.success() {
            info!("GIF generated successfully: {:?}", gif_path);
            Ok(())
        } else {
            Err(anyhow!("Failed to generate GIF"))
        }
    }
    
    pub async fn convert_to_gif(&self, video_path: &str, output_path: Option<&str>) -> Result<String> {
        let input_path = Path::new(video_path);
        let output_gif = if let Some(path) = output_path {
            PathBuf::from(path)
        } else {
            input_path.with_extension("gif")
        };
        
        Self::generate_gif_from_video(input_path, &self.config).await?;
        
        Ok(output_gif.to_string_lossy().to_string())
    }
    
    pub async fn optimize_video(&self, video_path: &str, optimization_level: u32) -> Result<String> {
        info!("Optimizing video: {} (level {})", video_path, optimization_level);
        
        let input_path = Path::new(video_path);
        let optimized_path = input_path.with_file_name(
            format!("{}_optimized.{}", 
                   input_path.file_stem().unwrap().to_str().unwrap(),
                   input_path.extension().unwrap().to_str().unwrap())
        );
        
        let mut args = vec![
            "-i", video_path,
            "-c:v", "libx264",
        ];
        
        // Optimization level settings
        match optimization_level {
            1 => {
                args.extend(["-preset", "fast", "-crf", "28"]);
            }
            2 => {
                args.extend(["-preset", "medium", "-crf", "25"]);
            }
            3 => {
                args.extend(["-preset", "slow", "-crf", "22"]);
            }
            _ => {
                args.extend(["-preset", "medium", "-crf", "25"]);
            }
        }
        
        args.extend([
            "-movflags", "+faststart", // Optimize for web streaming
            "-y",
            optimized_path.to_str().unwrap(),
        ]);
        
        let output = Command::new(&self.ffmpeg_path)
            .args(&args)
            .output()?;
            
        if output.status.success() {
            info!("Video optimization completed: {:?}", optimized_path);
            Ok(optimized_path.to_string_lossy().to_string())
        } else {
            Err(anyhow!("Video optimization failed"))
        }
    }
    
    pub async fn extract_frames(&self, video_path: &str, output_dir: &str, frame_rate: Option<f64>) -> Result<Vec<String>> {
        info!("Extracting frames from video: {}", video_path);
        
        fs::create_dir_all(output_dir).await?;
        
        let fps = frame_rate.unwrap_or(1.0);
        let output_pattern = format!("{}/frame_%04d.png", output_dir);
        
        let args = [
            "-i", video_path,
            "-vf", &format!("fps={}", fps),
            "-y",
            &output_pattern,
        ];
        
        let output = Command::new(&self.ffmpeg_path)
            .args(&args)
            .output()?;
            
        if !output.status.success() {
            return Err(anyhow!("Frame extraction failed"));
        }
        
        // List generated frames
        let mut frames = Vec::new();
        let mut entries = fs::read_dir(output_dir).await?;
        
        while let Some(entry) = entries.next_entry().await? {
            if let Some(name) = entry.file_name().to_str() {
                if name.starts_with("frame_") && name.ends_with(".png") {
                    frames.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
        
        frames.sort();
        info!("Extracted {} frames", frames.len());
        
        Ok(frames)
    }
    
    fn select_optimal_video_codec(&self) -> String {
        // Prefer hardware-accelerated encoders if available
        if self.config.hardware_acceleration {
            for encoder in &self.capabilities.hardware_encoders {
                if encoder.contains("h264") {
                    return encoder.clone();
                }
            }
        }
        
        // Fallback to software encoder
        self.config.video_codec.clone()
    }
    
    fn add_quality_settings(&self, args: &mut Vec<&str>) {
        match self.config.video_quality {
            VideoQuality::Low => {
                args.extend(["-crf", "30", "-preset", "ultrafast"]);
            }
            VideoQuality::Medium => {
                args.extend(["-crf", "25", "-preset", "medium"]);
            }
            VideoQuality::High => {
                args.extend(["-crf", "18", "-preset", "slow"]);
            }
            VideoQuality::Lossless => {
                args.extend(["-crf", "0", "-preset", "ultrafast"]);
            }
        }
    }
    
    fn add_hardware_acceleration(&self, args: &mut Vec<&str>) {
        // Add hardware acceleration flags based on available encoders
        if self.capabilities.hardware_encoders.iter().any(|e| e.contains("nvenc")) {
            args.extend(["-hwaccel", "cuda"]);
        } else if self.capabilities.hardware_encoders.iter().any(|e| e.contains("qsv")) {
            args.extend(["-hwaccel", "qsv"]);
        } else if self.capabilities.hardware_encoders.iter().any(|e| e.contains("videotoolbox")) {
            args.extend(["-hwaccel", "videotoolbox"]);
        }
    }
    
    pub fn get_capabilities(&self) -> &FFmpegCapabilities {
        &self.capabilities
    }
    
    pub async fn get_recording_stats(&self) -> HashMap<String, u64> {
        let sessions = self.active_sessions.read().await;
        let mut stats = HashMap::new();
        
        stats.insert("total_sessions".to_string(), sessions.len() as u64);
        
        let active_count = sessions.values()
            .filter(|s| matches!(s.status, RecordingStatus::Recording))
            .count();
        stats.insert("active_sessions".to_string(), active_count as u64);
        
        let completed_count = sessions.values()
            .filter(|s| matches!(s.status, RecordingStatus::Completed))
            .count();
        stats.insert("completed_sessions".to_string(), completed_count as u64);
        
        let total_size: u64 = sessions.values()
            .map(|s| s.metrics.file_size_bytes)
            .sum();
        stats.insert("total_size_bytes".to_string(), total_size);
        
        stats
    }
    
    pub async fn cleanup_old_recordings(&self, max_age_hours: u64) -> Result<usize> {
        info!("Cleaning up recordings older than {} hours", max_age_hours);
        
        let cutoff_time = Instant::now() - Duration::from_secs(max_age_hours * 3600);
        let mut cleaned_count = 0;
        
        let mut sessions = self.active_sessions.write().await;
        let mut to_remove = Vec::new();
        
        for (session_id, session) in sessions.iter() {
            if session.start_time < cutoff_time && 
               matches!(session.status, RecordingStatus::Completed | RecordingStatus::Failed(_)) {
                // Remove file
                if let Err(e) = fs::remove_file(&session.output_path).await {
                    warn!("Failed to remove old recording file: {}", e);
                } else {
                    cleaned_count += 1;
                }
                
                to_remove.push(session_id.clone());
            }
        }
        
        for session_id in to_remove {
            sessions.remove(&session_id);
        }
        
        info!("Cleaned up {} old recordings", cleaned_count);
        Ok(cleaned_count)
    }
}