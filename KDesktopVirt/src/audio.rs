use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub device_type: String, // "input", "output", "virtual"
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsRequest {
    pub text: String,
    pub voice: String,
    pub speed: f32,
    pub pitch: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioRecording {
    pub id: String,
    pub session_id: Option<String>,
    pub output_path: String,
    pub format: String,
    pub status: String,
    pub started_at: String,
    pub duration: Option<u64>,
}

pub struct AudioManager {
    devices: HashMap<String, AudioDevice>,
    recordings: HashMap<String, AudioRecording>,
    tts_enabled: bool,
}

impl AudioManager {
    pub async fn new() -> Result<Self> {
        info!("Initializing Audio Manager");

        let mut manager = Self {
            devices: HashMap::new(),
            recordings: HashMap::new(),
            tts_enabled: true,
        };

        // Initialize virtual audio devices
        manager.setup_virtual_devices().await?;

        Ok(manager)
    }

    pub async fn setup_virtual_devices(&mut self) -> Result<()> {
        info!("Setting up virtual audio devices");

        // Create virtual microphone
        let virtual_mic = AudioDevice {
            id: "kvirtualstage-mic".to_string(),
            name: "KVirtualStage Virtual Microphone".to_string(),
            device_type: "input".to_string(),
            status: "active".to_string(),
        };

        // Create virtual speakers
        let virtual_speakers = AudioDevice {
            id: "kvirtualstage-speakers".to_string(),
            name: "KVirtualStage Virtual Speakers".to_string(),
            device_type: "output".to_string(),
            status: "active".to_string(),
        };

        self.devices.insert(virtual_mic.id.clone(), virtual_mic);
        self.devices
            .insert(virtual_speakers.id.clone(), virtual_speakers);

        // Set up platform-specific virtual devices
        self.create_platform_virtual_devices().await?;

        Ok(())
    }

    pub async fn text_to_speech(&self, request: TtsRequest) -> Result<Vec<u8>> {
        info!("Converting text to speech: {}", request.text);

        if !self.tts_enabled {
            return Err(anyhow!("TTS is disabled"));
        }

        // Use espeak or festival for TTS
        let output = tokio::process::Command::new("espeak")
            .args([
                "-s",
                &((request.speed * 175.0) as i32).to_string(), // Speed in WPM
                "-p",
                &((request.pitch * 50.0) as i32).to_string(), // Pitch
                "-w",
                "/tmp/kvirtualstage-tts.wav", // Output to file
                &request.text,
            ])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!(
                "TTS failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Read the generated audio file
        let audio_data = tokio::fs::read("/tmp/kvirtualstage-tts.wav").await?;

        // Clean up temporary file
        let _ = tokio::fs::remove_file("/tmp/kvirtualstage-tts.wav").await;

        Ok(audio_data)
    }

    pub async fn play_audio_to_virtual_mic(&self, audio_data: Vec<u8>) -> Result<()> {
        info!("Playing audio to virtual microphone");

        // Write audio data to temporary file
        let temp_path = "/tmp/kvirtualstage-audio.wav";
        tokio::fs::write(temp_path, audio_data).await?;

        // Play to virtual microphone using platform-appropriate method
        self.play_to_virtual_device(temp_path).await?;

        Ok(())
    }

    pub async fn start_audio_recording(
        &mut self,
        output_path: String,
        format: String,
        session_id: Option<String>,
    ) -> Result<String> {
        info!("Starting audio recording: {}", output_path);

        let recording_id = uuid::Uuid::new_v4().to_string();

        // Start recording using ffmpeg
        let mut cmd = tokio::process::Command::new("ffmpeg");
        cmd.args([
            "-f",
            "pulse",
            "-i",
            "default",
            "-acodec",
            "libmp3lame",
            "-ab",
            "128k",
            "-y",
            &output_path,
        ]);

        cmd.spawn()?;

        let recording = AudioRecording {
            id: recording_id.clone(),
            session_id,
            output_path,
            format,
            status: "recording".to_string(),
            started_at: chrono::Utc::now().to_rfc3339(),
            duration: None,
        };

        self.recordings.insert(recording_id.clone(), recording);

        Ok(recording_id)
    }

    pub async fn stop_audio_recording(&mut self, recording_id: String) -> Result<()> {
        info!("Stopping audio recording: {}", recording_id);

        if let Some(recording) = self.recordings.get_mut(&recording_id) {
            recording.status = "stopped".to_string();
            // In a real implementation, we would stop the ffmpeg process
            Ok(())
        } else {
            Err(anyhow!("Recording not found: {}", recording_id))
        }
    }

    pub async fn speech_to_text(&self, audio_data: Vec<u8>) -> Result<String> {
        info!("Converting speech to text");

        // Write audio data to temporary file
        let temp_path = "/tmp/kvirtualstage-stt.wav";
        tokio::fs::write(temp_path, audio_data).await?;

        // Use a simple speech-to-text engine
        // In a real implementation, this would use services like Google Speech-to-Text,
        // Azure Speech Services, or local engines like Vosk

        // For now, return a placeholder
        Ok("[Speech to text not implemented]".to_string())
    }

    pub async fn list_audio_devices(&self) -> Result<Vec<AudioDevice>> {
        Ok(self.devices.values().cloned().collect())
    }

    pub async fn get_audio_device(&self, device_id: String) -> Result<AudioDevice> {
        self.devices
            .get(&device_id)
            .cloned()
            .ok_or_else(|| anyhow!("Audio device not found: {}", device_id))
    }

    pub async fn set_default_audio_device(&mut self, device_id: String) -> Result<()> {
        info!("Setting default audio device: {}", device_id);

        if !self.devices.contains_key(&device_id) {
            return Err(anyhow!("Audio device not found: {}", device_id));
        }

        // Use platform-appropriate method to set default device
        self.set_platform_default_device(&device_id).await?;

        Ok(())
    }

    async fn play_to_virtual_device(&self, audio_file: &str) -> Result<()> {
        let platform = std::env::consts::OS;

        match platform {
            "linux" => self.linux_play_to_virtual_device(audio_file).await,
            "macos" => self.macos_play_to_virtual_device(audio_file).await,
            "windows" => self.windows_play_to_virtual_device(audio_file).await,
            _ => {
                warn!("Audio playback not supported on platform: {}", platform);
                Ok(())
            }
        }
    }

    async fn linux_play_to_virtual_device(&self, audio_file: &str) -> Result<()> {
        // Try PipeWire first
        if let Ok(_) = tokio::process::Command::new("which")
            .args(["wpctl"])
            .output()
            .await
        {
            let output = tokio::process::Command::new("pw-play")
                .args([audio_file, "--target=kvirtualstage_speakers"])
                .output()
                .await;

            if let Ok(result) = output {
                if result.status.success() {
                    return Ok(());
                }
            }
        }

        // Fall back to PulseAudio
        let output = tokio::process::Command::new("pactl")
            .args([
                "load-module",
                "module-pipe-source",
                "source_name=kvirtualstage_mic_input",
                &format!("file={}", audio_file),
                "format=s16le",
                "rate=44100",
                "channels=2",
            ])
            .output()
            .await;

        if let Ok(result) = output {
            if !result.status.success() {
                warn!(
                    "Failed to load PulseAudio module: {}",
                    String::from_utf8_lossy(&result.stderr)
                );
            }
        }

        Ok(())
    }

    async fn macos_play_to_virtual_device(&self, audio_file: &str) -> Result<()> {
        // Use afplay or sox if available
        let output = tokio::process::Command::new("afplay")
            .args([audio_file])
            .output()
            .await;

        if let Ok(result) = output {
            if result.status.success() {
                return Ok(());
            }
        }

        warn!("Audio playback requires BlackHole or similar virtual audio driver on macOS");
        Ok(())
    }

    async fn windows_play_to_virtual_device(&self, audio_file: &str) -> Result<()> {
        // Use ffplay if available
        let output = tokio::process::Command::new("ffplay")
            .args(["-nodisp", "-autoexit", audio_file])
            .output()
            .await;

        if let Ok(result) = output {
            if result.status.success() {
                return Ok(());
            }
        }

        warn!("Audio playback requires Virtual Audio Cable on Windows");
        Ok(())
    }

    async fn set_platform_default_device(&self, device_id: &str) -> Result<()> {
        let platform = std::env::consts::OS;

        match platform {
            "linux" => self.linux_set_default_device(device_id).await,
            "macos" => self.macos_set_default_device(device_id).await,
            "windows" => self.windows_set_default_device(device_id).await,
            _ => {
                warn!(
                    "Setting default audio device not supported on platform: {}",
                    platform
                );
                Ok(())
            }
        }
    }

    async fn linux_set_default_device(&self, device_id: &str) -> Result<()> {
        // Try wpctl (PipeWire) first
        let wpctl_output = tokio::process::Command::new("wpctl")
            .args(["set-default", device_id])
            .output()
            .await;

        if let Ok(result) = wpctl_output {
            if result.status.success() {
                return Ok(());
            }
        }

        // Fall back to pactl (PulseAudio)
        let output = tokio::process::Command::new("pactl")
            .args(["set-default-sink", device_id])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!(
                "Failed to set default audio device: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    async fn macos_set_default_device(&self, _device_id: &str) -> Result<()> {
        warn!("Setting default audio device requires system configuration on macOS");
        Ok(())
    }

    async fn windows_set_default_device(&self, _device_id: &str) -> Result<()> {
        warn!("Setting default audio device requires system configuration on Windows");
        Ok(())
    }

    async fn create_platform_virtual_devices(&self) -> Result<()> {
        info!("Detecting platform and setting up virtual audio devices");

        let platform = std::env::consts::OS;
        info!("Detected platform: {}", platform);

        match platform {
            "linux" => self.setup_linux_audio().await?,
            "macos" => self.setup_macos_audio().await?,
            "windows" => self.setup_windows_audio().await?,
            _ => {
                warn!(
                    "Unsupported platform: {}. Virtual audio disabled.",
                    platform
                );
            }
        }

        Ok(())
    }

    async fn setup_linux_audio(&self) -> Result<()> {
        info!("Setting up Linux audio system");

        // Try PipeWire first (modern Linux standard)
        if let Ok(pipewire_result) = self.setup_pipewire().await {
            return Ok(pipewire_result);
        }

        // Fall back to PulseAudio
        if let Ok(pulse_result) = self.setup_pulseaudio().await {
            return Ok(pulse_result);
        }

        // Fall back to JACK
        if let Ok(jack_result) = self.setup_jack().await {
            return Ok(jack_result);
        }

        warn!("No compatible audio system found on Linux.");
        warn!("Install PipeWire, PulseAudio, or JACK for virtual audio support.");
        Ok(())
    }

    async fn setup_pipewire(&self) -> Result<()> {
        // Check for PipeWire tools
        let pw_cli_check = tokio::process::Command::new("which")
            .args(["pw-cli"])
            .output()
            .await;

        let wpctl_check = tokio::process::Command::new("which")
            .args(["wpctl"])
            .output()
            .await;

        if pw_cli_check.is_ok() || wpctl_check.is_ok() {
            info!("PipeWire detected, creating virtual devices");

            // Create virtual sink using wpctl (preferred) or pw-cli
            if wpctl_check.is_ok() {
                // Create null sink for virtual speakers
                let output = tokio::process::Command::new("wpctl")
                    .args([
                        "create-device",
                        "--class=Audio/Sink",
                        "kvirtualstage_speakers",
                    ])
                    .output()
                    .await;

                match output {
                    Ok(result) if result.status.success() => {
                        info!("PipeWire virtual speakers created successfully");
                    }
                    Ok(result) => {
                        warn!(
                            "Failed to create PipeWire virtual speakers: {}",
                            String::from_utf8_lossy(&result.stderr)
                        );
                    }
                    Err(e) => {
                        warn!("Error creating PipeWire virtual speakers: {}", e);
                    }
                }

                // Create virtual source (microphone)
                let output = tokio::process::Command::new("wpctl")
                    .args(["create-device", "--class=Audio/Source", "kvirtualstage_mic"])
                    .output()
                    .await;

                match output {
                    Ok(result) if result.status.success() => {
                        info!("PipeWire virtual microphone created successfully");
                    }
                    Ok(result) => {
                        warn!(
                            "Failed to create PipeWire virtual microphone: {}",
                            String::from_utf8_lossy(&result.stderr)
                        );
                    }
                    Err(e) => {
                        warn!("Error creating PipeWire virtual microphone: {}", e);
                    }
                }
            }
            return Ok(());
        }

        Err(anyhow::anyhow!("PipeWire not available"))
    }

    async fn setup_pulseaudio(&self) -> Result<()> {
        let pactl_check = tokio::process::Command::new("which")
            .args(["pactl"])
            .output()
            .await;

        if let Ok(output) = pactl_check {
            if output.status.success() {
                info!("PulseAudio detected, creating virtual devices");

                // Create virtual sink (speakers)
                let output = tokio::process::Command::new("pactl")
                    .args([
                        "load-module",
                        "module-null-sink",
                        "sink_name=kvirtualstage_speakers",
                        "sink_properties=device.description='KVirtualStage Virtual Speakers'",
                    ])
                    .output()
                    .await?;

                if !output.status.success() {
                    warn!(
                        "Failed to create PulseAudio virtual speakers: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }

                // Create virtual source (microphone)
                let output = tokio::process::Command::new("pactl")
                    .args([
                        "load-module",
                        "module-null-source",
                        "source_name=kvirtualstage_mic",
                        "source_properties=device.description='KVirtualStage Virtual Microphone'",
                    ])
                    .output()
                    .await?;

                if !output.status.success() {
                    warn!(
                        "Failed to create PulseAudio virtual microphone: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }

                return Ok(());
            }
        }

        Err(anyhow::anyhow!("PulseAudio not available"))
    }

    async fn setup_jack(&self) -> Result<()> {
        let jack_check = tokio::process::Command::new("which")
            .args(["jack_lsp"])
            .output()
            .await;

        if let Ok(output) = jack_check {
            if output.status.success() {
                info!("JACK detected, but virtual device creation requires custom setup");
                warn!("JACK virtual devices require manual configuration or JACK-specific tools");
                warn!("Consider using PipeWire or PulseAudio for automatic virtual device setup");
                return Ok(());
            }
        }

        Err(anyhow::anyhow!("JACK not available"))
    }

    async fn setup_macos_audio(&self) -> Result<()> {
        info!("Setting up macOS audio system");

        // Check for SoX (Swiss Army knife of audio manipulation)
        let sox_check = tokio::process::Command::new("which")
            .args(["sox"])
            .output()
            .await;

        if let Ok(output) = sox_check {
            if output.status.success() {
                info!("SoX available for audio processing on macOS");
                return Ok(());
            }
        }

        // Check for system audio tools
        let system_audio_check = tokio::process::Command::new("which")
            .args(["afplay"])
            .output()
            .await;

        if let Ok(output) = system_audio_check {
            if output.status.success() {
                info!("macOS system audio tools available");
                warn!("Virtual audio devices on macOS require BlackHole or similar virtual audio driver");
                warn!("Install BlackHole: https://github.com/ExistentialAudio/BlackHole for full functionality");
                return Ok(());
            }
        }

        warn!("Limited audio support on macOS without additional tools");
        warn!("For full functionality, install:");
        warn!("  - BlackHole (virtual audio driver)");
        warn!("  - SoX (audio processing): brew install sox");

        Ok(())
    }

    async fn setup_windows_audio(&self) -> Result<()> {
        info!("Setting up Windows audio system");

        // Check for common Windows audio tools
        let ffmpeg_check = tokio::process::Command::new("where")
            .args(["ffmpeg"])
            .output()
            .await;

        if let Ok(output) = ffmpeg_check {
            if output.status.success() {
                info!("FFmpeg available for audio processing on Windows");
            }
        }

        warn!("Virtual audio devices on Windows require Virtual Audio Cable or VB-Cable");
        warn!("Install VB-Cable: https://vb-audio.com/Cable/ for virtual audio support");
        warn!("Or use Windows Container with Linux audio stack for full functionality");

        Ok(())
    }

    pub async fn cleanup_virtual_devices(&self) -> Result<()> {
        info!("Cleaning up virtual audio devices");

        let platform = std::env::consts::OS;

        match platform {
            "linux" => self.cleanup_linux_audio().await?,
            "macos" => self.cleanup_macos_audio().await?,
            "windows" => self.cleanup_windows_audio().await?,
            _ => {
                info!("No cleanup needed for platform: {}", platform);
            }
        }

        Ok(())
    }

    async fn cleanup_linux_audio(&self) -> Result<()> {
        // Try to clean up PipeWire devices first
        let wpctl_check = tokio::process::Command::new("which")
            .args(["wpctl"])
            .output()
            .await;

        if wpctl_check.is_ok() {
            // PipeWire cleanup
            let _ = tokio::process::Command::new("wpctl")
                .args(["destroy", "kvirtualstage_speakers"])
                .output()
                .await;

            let _ = tokio::process::Command::new("wpctl")
                .args(["destroy", "kvirtualstage_mic"])
                .output()
                .await;
        }

        // Try PulseAudio cleanup
        let pactl_check = tokio::process::Command::new("which")
            .args(["pactl"])
            .output()
            .await;

        if pactl_check.is_ok() {
            let _ = tokio::process::Command::new("pactl")
                .args(["unload-module", "module-null-sink"])
                .output()
                .await;

            let _ = tokio::process::Command::new("pactl")
                .args(["unload-module", "module-null-source"])
                .output()
                .await;
        }

        Ok(())
    }

    async fn cleanup_macos_audio(&self) -> Result<()> {
        info!("macOS audio cleanup - no virtual devices to remove");
        Ok(())
    }

    async fn cleanup_windows_audio(&self) -> Result<()> {
        info!("Windows audio cleanup - no virtual devices to remove");
        Ok(())
    }
}
