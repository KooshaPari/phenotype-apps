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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingSession {
    pub session_id: String,
    pub start_time: Instant,
    pub end_time: Option<Instant>,
    pub output_path: PathBuf,
    pub config: RecordingConfig,
    pub status: RecordingStatus,
    pub metrics: RecordingMetrics,
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
        ffmpeg_args.extend([
            "-f", "x11grab",
            "-r", &self.config.frame_rate.to_string(),
            "-s", &format!("{}x{}", self.config.resolution.width, self.config.resolution.height),
            "-i", &format!("{}+0,0", display_env),
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
        
        let gif_path = video_path.with_extension("gif");\n        \n        // Two-pass GIF generation for better quality\n        // Pass 1: Generate palette\n        let palette_path = video_path.with_extension(\"png\");\n        \n        let palette_args = [\n            \"-i\", video_path.to_str().unwrap(),\n            \"-vf\", &format!(\"fps={},scale={}:{}:flags=lanczos,palettegen=max_colors=256\", \n                           config.frame_rate / 2, // Reduce FPS for GIF\n                           config.resolution.width / 2, // Reduce resolution for GIF\n                           config.resolution.height / 2),\n            \"-y\",\n            palette_path.to_str().unwrap(),\n        ];\n        \n        let output = Command::new(\"ffmpeg\")\n            .args(&palette_args)\n            .output()?;\n            \n        if !output.status.success() {\n            return Err(anyhow!(\"Failed to generate palette for GIF\"));\n        }\n        \n        // Pass 2: Generate GIF using palette\n        let gif_args = [\n            \"-i\", video_path.to_str().unwrap(),\n            \"-i\", palette_path.to_str().unwrap(),\n            \"-lavfi\", &format!(\"fps={},scale={}:{}:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer\",\n                              config.frame_rate / 2,\n                              config.resolution.width / 2,\n                              config.resolution.height / 2),\n            \"-y\",\n            gif_path.to_str().unwrap(),\n        ];\n        \n        let output = Command::new(\"ffmpeg\")\n            .args(&gif_args)\n            .output()?;\n            \n        // Clean up palette file\n        let _ = fs::remove_file(&palette_path).await;\n        \n        if output.status.success() {\n            info!(\"GIF generated successfully: {:?}\", gif_path);\n            Ok(())\n        } else {\n            Err(anyhow!(\"Failed to generate GIF\"))\n        }\n    }\n    \n    pub async fn convert_to_gif(&self, video_path: &str, output_path: Option<&str>) -> Result<String> {\n        let input_path = Path::new(video_path);\n        let output_gif = if let Some(path) = output_path {\n            PathBuf::from(path)\n        } else {\n            input_path.with_extension(\"gif\")\n        };\n        \n        Self::generate_gif_from_video(input_path, &self.config).await?;\n        \n        Ok(output_gif.to_string_lossy().to_string())\n    }\n    \n    pub async fn optimize_video(&self, video_path: &str, optimization_level: u32) -> Result<String> {\n        info!(\"Optimizing video: {} (level {})\", video_path, optimization_level);\n        \n        let input_path = Path::new(video_path);\n        let optimized_path = input_path.with_file_name(\n            format!(\"{}_optimized.{}\", \n                   input_path.file_stem().unwrap().to_str().unwrap(),\n                   input_path.extension().unwrap().to_str().unwrap())\n        );\n        \n        let mut args = vec![\n            \"-i\", video_path,\n            \"-c:v\", \"libx264\",\n        ];\n        \n        // Optimization level settings\n        match optimization_level {\n            1 => {\n                args.extend([\"-preset\", \"fast\", \"-crf\", \"28\"]);\n            }\n            2 => {\n                args.extend([\"-preset\", \"medium\", \"-crf\", \"25\"]);\n            }\n            3 => {\n                args.extend([\"-preset\", \"slow\", \"-crf\", \"22\"]);\n            }\n            _ => {\n                args.extend([\"-preset\", \"medium\", \"-crf\", \"25\"]);\n            }\n        }\n        \n        args.extend([\n            \"-movflags\", \"+faststart\", // Optimize for web streaming\n            \"-y\",\n            optimized_path.to_str().unwrap(),\n        ]);\n        \n        let output = Command::new(&self.ffmpeg_path)\n            .args(&args)\n            .output()?;\n            \n        if output.status.success() {\n            info!(\"Video optimization completed: {:?}\", optimized_path);\n            Ok(optimized_path.to_string_lossy().to_string())\n        } else {\n            Err(anyhow!(\"Video optimization failed\"))\n        }\n    }\n    \n    pub async fn extract_frames(&self, video_path: &str, output_dir: &str, frame_rate: Option<f64>) -> Result<Vec<String>> {\n        info!(\"Extracting frames from video: {}\", video_path);\n        \n        fs::create_dir_all(output_dir).await?;\n        \n        let fps = frame_rate.unwrap_or(1.0);\n        let output_pattern = format!(\"{}/frame_%04d.png\", output_dir);\n        \n        let args = [\n            \"-i\", video_path,\n            \"-vf\", &format!(\"fps={}\", fps),\n            \"-y\",\n            &output_pattern,\n        ];\n        \n        let output = Command::new(&self.ffmpeg_path)\n            .args(&args)\n            .output()?;\n            \n        if !output.status.success() {\n            return Err(anyhow!(\"Frame extraction failed\"));\n        }\n        \n        // List generated frames\n        let mut frames = Vec::new();\n        let mut entries = fs::read_dir(output_dir).await?;\n        \n        while let Some(entry) = entries.next_entry().await? {\n            if let Some(name) = entry.file_name().to_str() {\n                if name.starts_with(\"frame_\") && name.ends_with(\".png\") {\n                    frames.push(entry.path().to_string_lossy().to_string());\n                }\n            }\n        }\n        \n        frames.sort();\n        info!(\"Extracted {} frames\", frames.len());\n        \n        Ok(frames)\n    }\n    \n    fn select_optimal_video_codec(&self) -> String {\n        // Prefer hardware-accelerated encoders if available\n        if self.config.hardware_acceleration {\n            for encoder in &self.capabilities.hardware_encoders {\n                if encoder.contains(\"h264\") {\n                    return encoder.clone();\n                }\n            }\n        }\n        \n        // Fallback to software encoder\n        self.config.video_codec.clone()\n    }\n    \n    fn add_quality_settings(&self, args: &mut Vec<&str>) {\n        match self.config.video_quality {\n            VideoQuality::Low => {\n                args.extend([\"-crf\", \"30\", \"-preset\", \"ultrafast\"]);\n            }\n            VideoQuality::Medium => {\n                args.extend([\"-crf\", \"25\", \"-preset\", \"medium\"]);\n            }\n            VideoQuality::High => {\n                args.extend([\"-crf\", \"18\", \"-preset\", \"slow\"]);\n            }\n            VideoQuality::Lossless => {\n                args.extend([\"-crf\", \"0\", \"-preset\", \"ultrafast\"]);\n            }\n        }\n    }\n    \n    fn add_hardware_acceleration(&self, args: &mut Vec<&str>) {\n        // Add hardware acceleration flags based on available encoders\n        if self.capabilities.hardware_encoders.iter().any(|e| e.contains(\"nvenc\")) {\n            args.extend([\"-hwaccel\", \"cuda\"]);\n        } else if self.capabilities.hardware_encoders.iter().any(|e| e.contains(\"qsv\")) {\n            args.extend([\"-hwaccel\", \"qsv\"]);\n        } else if self.capabilities.hardware_encoders.iter().any(|e| e.contains(\"videotoolbox\")) {\n            args.extend([\"-hwaccel\", \"videotoolbox\"]);\n        }\n    }\n    \n    pub fn get_capabilities(&self) -> &FFmpegCapabilities {\n        &self.capabilities\n    }\n    \n    pub async fn get_recording_stats(&self) -> HashMap<String, u64> {\n        let sessions = self.active_sessions.read().await;\n        let mut stats = HashMap::new();\n        \n        stats.insert(\"total_sessions\".to_string(), sessions.len() as u64);\n        \n        let active_count = sessions.values()\n            .filter(|s| matches!(s.status, RecordingStatus::Recording))\n            .count();\n        stats.insert(\"active_sessions\".to_string(), active_count as u64);\n        \n        let completed_count = sessions.values()\n            .filter(|s| matches!(s.status, RecordingStatus::Completed))\n            .count();\n        stats.insert(\"completed_sessions\".to_string(), completed_count as u64);\n        \n        let total_size: u64 = sessions.values()\n            .map(|s| s.metrics.file_size_bytes)\n            .sum();\n        stats.insert(\"total_size_bytes\".to_string(), total_size);\n        \n        stats\n    }\n    \n    pub async fn cleanup_old_recordings(&self, max_age_hours: u64) -> Result<usize> {\n        info!(\"Cleaning up recordings older than {} hours\", max_age_hours);\n        \n        let cutoff_time = Instant::now() - Duration::from_secs(max_age_hours * 3600);\n        let mut cleaned_count = 0;\n        \n        let mut sessions = self.active_sessions.write().await;\n        let mut to_remove = Vec::new();\n        \n        for (session_id, session) in sessions.iter() {\n            if session.start_time < cutoff_time && \n               matches!(session.status, RecordingStatus::Completed | RecordingStatus::Failed(_)) {\n                // Remove file\n                if let Err(e) = fs::remove_file(&session.output_path).await {\n                    warn!(\"Failed to remove old recording file: {}\", e);\n                } else {\n                    cleaned_count += 1;\n                }\n                \n                to_remove.push(session_id.clone());\n            }\n        }\n        \n        for session_id in to_remove {\n            sessions.remove(&session_id);\n        }\n        \n        info!(\"Cleaned up {} old recordings\", cleaned_count);\n        Ok(cleaned_count)\n    }\n}"