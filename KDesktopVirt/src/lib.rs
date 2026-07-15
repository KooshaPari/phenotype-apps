// Re-export commonly-used types so crate modules don't need redundant imports
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate tracing;

pub use std::collections::HashMap;
pub use std::time::Duration;
pub use std::process::{Command as SyncCommand, Child, Stdio};
pub use tokio::process::Command as TokioCommand;
pub use tokio::time::timeout;

pub mod api;
pub mod api_surface;
pub mod audio;
pub mod audio_video_engine;
pub mod audio_video_integration;
pub mod automation;
pub mod automation_engine;
pub mod cli;
pub mod core;
pub mod desktop_control;
pub mod mcp;
pub mod recording;
pub mod recording_pipeline;
pub mod security;
pub mod security_framework;
pub mod security_monitoring;
pub mod audit_compliance;
pub mod session_storage;
pub mod ui_automation;
pub mod automation_ports;
pub mod multimodal_detection;
pub mod ffmpeg_pipeline;
pub mod tts_audio_system;
// FIXED: Core engine modules (UI automation, FFmpeg pipeline, TTS) are now working
// TODO: Fix remaining syntax errors in these supplementary modules:
// pub mod animation_framework;
// pub mod visual_feedback;
// pub mod natural_automation_demo;
pub mod virtualization;
pub mod web;
// Enhanced virtualization modules
pub mod podman_integration;
pub mod desktop_provisioning;
pub mod resource_manager;
pub mod containerization;

pub use api_surface::{KVirtualStageAPI, APISessionInfo, WorkflowExecutionResult};
pub use audio_video_engine::{AudioVideoEngine, AudioVideoConfig, TtsProvider, SttProvider};
pub use audio_video_integration::{AudioVideoIntegration, IntegrationConfig, VoiceCommandResponse};
pub use automation::ComprehensiveAutomationPlatform;
pub use automation_engine::{AutomationEngine, WindMouseEngine, NaturalTypingEngine, AutomationWorkflow, WorkflowStep, StepAction, Point, MouseButton};
pub use cli::KVirtualStageCommand;
pub use core::KVirtualStageCore;
pub use desktop_control::DesktopControlManager;
pub use recording_pipeline::{RecordingPipeline, QualityProfile};
pub use security_framework::SecurityEngine;
pub use security_monitoring::{SecurityMonitor, SecurityEvent, ThreatAnalysisResult};
pub use audit_compliance::{
    AuditEngine, AuditEventBuilder, ComplianceReport, ComplianceStandard, ExportFormat,
    AuditConfig, AuditLogger, ComplianceFramework, RetentionManager, IntegrityVerifier,
    AuditEventType, AuditCategory, AuditSeverity, Actor,
};
pub use containerization::{ContainerizationEngine, ContainerConfig, ContainerCreationRequest, ContainerHandle};
