// Text-to-Speech Audio System with Virtual Device Support
// Professional-grade TTS with multiple engines, effects, and virtual audio routing

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::fs;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TTSConfig {
    pub engine: TTSEngine,
    pub voice: String,
    pub language: String,
    pub speech_rate: f64,
    pub pitch: f64,
    pub volume: f64,
    pub output_format: AudioFormat,
    pub quality: AudioQuality,
    pub effects: Vec<AudioEffect>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TTSEngine {
    Espeak,
    Festival,
    Flite,
    System,
    AWS,
    Azure,
    Google,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioFormat {
    WAV,
    MP3,
    OGG,
    FLAC,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioQuality {
    Low,    // 16kHz, mono
    Medium, // 44.1kHz, stereo
    High,   // 48kHz, stereo
    Studio, // 96kHz, stereo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioEffect {
    Reverb { room_size: f64, dampening: f64 },
    Echo { delay_ms: u32, decay: f64 },
    Normalize { target_db: f64 },
    Compress { ratio: f64, threshold_db: f64 },
    EQ { low_gain: f64, mid_gain: f64, high_gain: f64 },
}

impl Default for TTSConfig {
    fn default() -> Self {
        Self {
            engine: TTSEngine::Espeak,
            voice: "en".to_string(),
            language: "en-US".to_string(),
            speech_rate: 1.0,
            pitch: 1.0,
            volume: 0.8,
            output_format: AudioFormat::WAV,
            quality: AudioQuality::Medium,
            effects: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualAudioConfig {
    pub device_name: String,
    pub sample_rate: u32,
    pub channels: u32,
    pub buffer_size: u32,
    pub auto_routing: bool,
    pub monitor_enabled: bool,
}

impl Default for VirtualAudioConfig {
    fn default() -> Self {
        Self {
            device_name: "KVirtualStage_Audio".to_string(),
            sample_rate: 44100,
            channels: 2,
            buffer_size: 1024,
            auto_routing: true,
            monitor_enabled: false,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AudioSession {
    pub session_id: String,
    #[serde(skip)]
    pub start_time: Instant,
    #[serde(skip)]
    pub end_time: Option<Instant>,
    pub output_files: Vec<PathBuf>,
    pub config: TTSConfig,
    pub status: AudioStatus,
    pub metrics: AudioMetrics,
}

impl Default for AudioSession {
    fn default() -> Self {
        Self {
            session_id: String::new(),
            start_time: std::time::Instant::now(),
            end_time: None,
            output_files: Vec::new(),
            config: TTSConfig::default(),
            status: AudioStatus::Preparing,
            metrics: AudioMetrics::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioStatus {
    Preparing,
    Synthesizing,
    Processing,
    Playing,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioMetrics {
    pub total_duration: Duration,
    pub words_per_minute: f64,
    pub file_size_bytes: u64,
    pub peak_amplitude: f64,
    pub average_amplitude: f64,
    pub processing_time: Duration,
}

impl Default for AudioMetrics {
    fn default() -> Self {
        Self {
            total_duration: Duration::ZERO,
            words_per_minute: 0.0,
            file_size_bytes: 0,
            peak_amplitude: 0.0,
            average_amplitude: 0.0,
            processing_time: Duration::ZERO,
        }
    }
}

pub struct TTSAudioSystem {
    tts_config: TTSConfig,
    virtual_audio_config: VirtualAudioConfig,
    active_sessions: Arc<RwLock<HashMap<String, AudioSession>>>,
    virtual_device_active: bool,
    available_voices: Vec<VoiceInfo>,
    audio_engine_capabilities: AudioEngineCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceInfo {
    pub name: String,
    pub language: String,
    pub gender: String,
    pub quality: String,
    pub engine: TTSEngine,
}

#[derive(Debug, Clone)]
struct AudioEngineCapabilities {
    espeak_available: bool,
    festival_available: bool,
    flite_available: bool,
    pulseaudio_available: bool,
    alsa_available: bool,
    sox_available: bool,
    ffmpeg_available: bool,
}

impl TTSAudioSystem {
    pub async fn new(tts_config: TTSConfig, virtual_audio_config: VirtualAudioConfig) -> Result<Self> {
        info!("Initializing TTS Audio System");
        
        // Detect available audio engines and capabilities
        let capabilities = Self::detect_audio_capabilities().await?;
        let available_voices = Self::discover_available_voices(&capabilities).await?;
        
        info!("Found {} available voices", available_voices.len());
        
        Ok(Self {
            tts_config,
            virtual_audio_config,
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            virtual_device_active: false,
            available_voices,
            audio_engine_capabilities: capabilities,
        })
    }
    
    async fn detect_audio_capabilities() -> Result<AudioEngineCapabilities> {
        info!("Detecting audio system capabilities");
        
        let mut capabilities = AudioEngineCapabilities {
            espeak_available: false,
            festival_available: false,
            flite_available: false,
            pulseaudio_available: false,
            alsa_available: false,
            sox_available: false,
            ffmpeg_available: false,
        };
        
        // Test for espeak
        if let Ok(output) = Command::new("espeak").args(["--version"]).output() {
            capabilities.espeak_available = output.status.success();
        }
        
        // Test for festival
        if let Ok(output) = Command::new("festival").args(["--version"]).output() {
            capabilities.festival_available = output.status.success();
        }
        
        // Test for flite
        if let Ok(output) = Command::new("flite").args(["-version"]).output() {
            capabilities.flite_available = output.status.success();
        }
        
        // Test for PulseAudio
        if let Ok(output) = Command::new("pulseaudio").args(["--version"]).output() {
            capabilities.pulseaudio_available = output.status.success();
        }
        
        // Test for ALSA
        if let Ok(output) = Command::new("aplay").args(["--version"]).output() {
            capabilities.alsa_available = output.status.success();
        }
        
        // Test for SoX
        if let Ok(output) = Command::new("sox").args(["--version"]).output() {
            capabilities.sox_available = output.status.success();
        }
        
        // Test for FFmpeg
        if let Ok(output) = Command::new("ffmpeg").args(["-version"]).output() {
            capabilities.ffmpeg_available = output.status.success();
        }
        
        info!("Audio capabilities: espeak={}, festival={}, flite={}, pulseaudio={}, sox={}, ffmpeg={}",
              capabilities.espeak_available,
              capabilities.festival_available,
              capabilities.flite_available,
              capabilities.pulseaudio_available,
              capabilities.sox_available,
              capabilities.ffmpeg_available);
        
        Ok(capabilities)
    }
    
    async fn discover_available_voices(capabilities: &AudioEngineCapabilities) -> Result<Vec<VoiceInfo>> {
        let mut voices = Vec::new();
        
        // Discover espeak voices
        if capabilities.espeak_available {
            if let Ok(output) = Command::new("espeak").args(["--voices"]).output() {
                let voices_text = String::from_utf8_lossy(&output.stdout);
                for line in voices_text.lines().skip(1) { // Skip header
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 4 {
                        voices.push(VoiceInfo {
                            name: parts[4].to_string(),
                            language: parts[1].to_string(),
                            gender: "neutral".to_string(),
                            quality: "standard".to_string(),
                            engine: TTSEngine::Espeak,
                        });
                    }
                }
            }
        }
        
        // Discover festival voices
        if capabilities.festival_available {
            // Festival voice discovery would be more complex
            voices.push(VoiceInfo {
                name: "kal_diphone".to_string(),
                language: "en-US".to_string(),
                gender: "male".to_string(),
                quality: "good".to_string(),
                engine: TTSEngine::Festival,
            });
        }
        
        // Discover flite voices
        if capabilities.flite_available {
            if let Ok(output) = Command::new("flite").args(["-lv"]).output() {
                let voices_text = String::from_utf8_lossy(&output.stdout);
                for line in voices_text.lines() {
                    if line.contains("voice") {
                        let voice_name = line.split_whitespace().next().unwrap_or("unknown");
                        voices.push(VoiceInfo {
                            name: voice_name.to_string(),
                            language: "en-US".to_string(),
                            gender: "neutral".to_string(),
                            quality: "standard".to_string(),
                            engine: TTSEngine::Flite,
                        });
                    }
                }
            }
        }
        
        Ok(voices)
    }
    
    pub async fn synthesize_speech(&mut self, session_id: String, text: &str, output_path: Option<&str>) -> Result<AudioSession> {
        info!("Synthesizing speech for session: {}", session_id);
        let start_time = Instant::now();
        
        // Prepare output path
        let output_file = if let Some(path) = output_path {
            PathBuf::from(path)
        } else {
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
            PathBuf::from(format!("/tmp/tts_{}_{}.wav", session_id, timestamp))
        };
        
        // Create session
        let mut session = AudioSession {
            session_id: session_id.clone(),
            start_time,
            end_time: None,
            output_files: vec![output_file.clone()],
            config: self.tts_config.clone(),
            status: AudioStatus::Synthesizing,
            metrics: AudioMetrics::default(),
        };
        
        // Store session
        {
            let mut sessions = self.active_sessions.write().await;
            sessions.insert(session_id.clone(), session.clone());
        }
        
        // Perform synthesis based on configured engine
        let synthesis_result = match self.tts_config.engine {
            TTSEngine::Espeak => self.synthesize_with_espeak(text, &output_file).await,
            TTSEngine::Festival => self.synthesize_with_festival(text, &output_file).await,
            TTSEngine::Flite => self.synthesize_with_flite(text, &output_file).await,
            TTSEngine::System => self.synthesize_with_system(text, &output_file).await,
            _ => Err(anyhow!("TTS engine not yet implemented: {:?}", self.tts_config.engine)),
        };
        
        // Update session based on result
        {
            let mut sessions = self.active_sessions.write().await;
            if let Some(session) = sessions.get_mut(&session_id) {
                match synthesis_result {
                    Ok(_) => {
                        session.status = AudioStatus::Processing;
                        session.end_time = Some(Instant::now());
                        session.metrics.processing_time = session.end_time.unwrap() - session.start_time;
                        
                        // Calculate metrics
                        if let Ok(metadata) = fs::metadata(&output_file).await {
                            session.metrics.file_size_bytes = metadata.len();
                        }
                        
                        // Calculate words per minute
                        let word_count = text.split_whitespace().count();
                        let duration_minutes = session.metrics.processing_time.as_secs_f64() / 60.0;
                        if duration_minutes > 0.0 {
                            session.metrics.words_per_minute = word_count as f64 / duration_minutes;
                        }
                        
                        session.status = AudioStatus::Completed;
                    }
                    Err(e) => {
                        session.status = AudioStatus::Failed(e.to_string());
                        error!("Speech synthesis failed: {}", e);
                    }
                }
            }
        }
        
        // Return updated session
        let sessions = self.active_sessions.read().await;
        Ok(sessions.get(&session_id).unwrap().clone())
    }
    
    async fn synthesize_with_espeak(&self, text: &str, output_path: &Path) -> Result<()> {
        let speed = ((self.tts_config.speech_rate * 175.0) as i32).to_string();
        let pitch = ((self.tts_config.pitch * 50.0) as i32).to_string();
        let amplitude = ((self.tts_config.volume * 200.0) as i32).to_string();
        
        let mut args = vec![
            "-v", &self.tts_config.voice,
            "-s", &speed, // espeak uses words per minute
            "-p", &pitch, // espeak pitch range 0-99
            "-a", &amplitude, // espeak amplitude 0-200
            "-w", output_path.to_str().unwrap(),
        ];
        
        // Add quality settings based on output format
        match self.tts_config.quality {
            AudioQuality::Low => args.extend(["-q"]), // Quiet mode for faster processing
            _ => {} // Default quality
        }
        
        args.push(text);
        
        let output = Command::new("espeak")
            .args(&args)
            .output()?;
            
        if output.status.success() {
            info!("Espeak synthesis completed: {:?}", output_path);
            Ok(())
        } else {
            Err(anyhow!("Espeak synthesis failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
    
    async fn synthesize_with_festival(&self, text: &str, output_path: &Path) -> Result<()> {
        // Create a temporary Festival script
        let script_content = format!(
            r#"
(voice_{voice})
(set! utt1 (Utterance Text "{text}"))
(utt.synth utt1)
(utt.save.wave utt1 "{output}" 'riff)
"#,
            voice = self.tts_config.voice,
            text = text.replace('"', "\\\""),
            output = output_path.to_str().unwrap()
        );
        
        let script_path = "/tmp/festival_script.scm";
        fs::write(script_path, script_content).await?;
        
        let output = Command::new("festival")
            .args(["-b", script_path])
            .output()?;
            
        // Clean up script file
        let _ = fs::remove_file(script_path).await;
        
        if output.status.success() {
            info!("Festival synthesis completed: {:?}", output_path);
            Ok(())
        } else {
            Err(anyhow!("Festival synthesis failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
    
    async fn synthesize_with_flite(&self, text: &str, output_path: &Path) -> Result<()> {
        let args = [
            "-voice", &self.tts_config.voice,
            "-o", output_path.to_str().unwrap(),
            "-t", text,
        ];
        
        let output = Command::new("flite")
            .args(&args)
            .output()?;
            
        if output.status.success() {
            info!("Flite synthesis completed: {:?}", output_path);
            Ok(())
        } else {
            Err(anyhow!("Flite synthesis failed: {}", String::from_utf8_lossy(&output.stderr)))
        }
    }
    
    async fn synthesize_with_system(&self, text: &str, output_path: &Path) -> Result<()> {
        // Use system's built-in TTS (macOS: say, Linux: spd-say)
        
        #[cfg(target_os = "macos")]
        {
            let rate = ((self.tts_config.speech_rate * 200.0) as i32).to_string();
            let args = [
                "-v", &self.tts_config.voice,
                "-r", &rate,
                "-o", output_path.to_str().unwrap(),
                text,
            ];
            
            let output = Command::new("say")
                .args(&args)
                .output()?;
                
            if output.status.success() {
                Ok(())
            } else {
                Err(anyhow!("System TTS failed"))
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            let args = [
                "-t", text,
                "-w",
                "-o", "pulse",
            ];
            
            let output = Command::new("spd-say")
                .args(&args)
                .output()?;
                
            if output.status.success() {
                Ok(())
            } else {
                Err(anyhow!("System TTS failed"))
            }
        }
        
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            Err(anyhow!("System TTS not supported on this platform"))
        }
    }
    
    pub async fn get_session_status(&self, session_id: &str) -> Option<AudioSession> {
        let sessions = self.active_sessions.read().await;
        sessions.get(session_id).cloned()
    }
    
    pub async fn list_active_sessions(&self) -> Vec<AudioSession> {
        let sessions = self.active_sessions.read().await;
        sessions.values().cloned().collect()
    }
    
    pub fn get_available_voices(&self) -> &Vec<VoiceInfo> {
        &self.available_voices
    }
    
    pub fn get_capabilities(&self) -> &AudioEngineCapabilities {
        &self.audio_engine_capabilities
    }
}