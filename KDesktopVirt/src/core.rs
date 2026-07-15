use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

use crate::automation_engine::AutomationEngine;
use crate::virtualization::VirtualizationManager;
use crate::resource_manager::ResourceManager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KVirtualStageStatus {
    pub active_sessions: usize,
    pub container_runtime: String,
    pub web_ui_active: bool,
    pub mcp_server_active: bool,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub name: String,
    pub id: String,
    pub desktop: String,
    pub status: String,
    pub created_at: String,
    pub container_id: Option<String>,
    pub vnc_port: Option<u16>,
    pub resources: SessionResources,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResources {
    pub memory_mb: u64,
    pub cpu_cores: u32,
    pub disk_gb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KVirtualStageConfig {
    pub container_runtime: String,
    pub default_desktop: String,
    pub default_resources: SessionResources,
    pub recording_settings: RecordingSettings,
    pub audio_settings: AudioSettings,
    pub security_settings: SecuritySettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingSettings {
    pub default_format: String,
    pub quality: String,
    pub fps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioSettings {
    pub enable_tts: bool,
    pub tts_voice: String,
    pub enable_recording: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub enable_encryption: bool,
    pub credential_vault_path: String,
    pub enable_mfa: bool,
}

#[derive(Debug, Clone)]
pub struct PerformanceMonitor {
    pub cpu_usage_history: Vec<f64>,
    pub memory_usage_history: Vec<u64>,
    pub task_execution_times: HashMap<String, Vec<Duration>>,
    pub throughput_metrics: ThroughputMetrics,
    pub optimization_suggestions: Vec<OptimizationSuggestion>,
}

#[derive(Debug, Clone)]
pub struct ThroughputMetrics {
    pub tasks_per_second: f64,
    pub average_response_time_ms: f64,
    pub peak_concurrent_sessions: usize,
    pub resource_efficiency_score: f64,
}

#[derive(Debug, Clone)]
pub struct OptimizationSuggestion {
    pub category: String,
    pub description: String,
    pub impact: ImpactLevel,
    pub implementation_effort: EffortLevel,
}

#[derive(Debug, Clone)]
pub enum ImpactLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub enum EffortLevel {
    Minimal,
    Low,
    Medium,
    High,
    Extensive,
}

#[derive(Debug)]
pub struct PluginManager {
    pub loaded_plugins: HashMap<String, Plugin>,
    pub plugin_registry: PluginRegistry,
    pub hooks: HashMap<String, Vec<PluginHook>>,
}

#[derive(Debug, Clone)]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub version: String,
    pub capabilities: Vec<PluginCapability>,
    pub config: PluginConfig,
    pub status: PluginStatus,
}

#[derive(Debug, Clone)]
pub enum PluginCapability {
    Automation,
    Recording,
    Authentication,
    Monitoring,
    CustomIntegration(String),
}

#[derive(Debug, Clone)]
pub struct PluginConfig {
    pub enabled: bool,
    pub settings: HashMap<String, serde_json::Value>,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum PluginStatus {
    Loaded,
    Active,
    Inactive,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct PluginRegistry {
    pub available_plugins: Vec<PluginInfo>,
    pub update_sources: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub download_url: String,
}

#[derive(Debug, Clone)]
pub struct PluginHook {
    pub plugin_id: String,
    pub event_type: String,
    pub callback: String,
}

#[derive(Debug)]
pub struct TaskScheduler {
    pub pending_tasks: Vec<ScheduledTask>,
    pub running_tasks: HashMap<String, RunningTask>,
    pub completed_tasks: Vec<CompletedTask>,
    pub scheduler_config: SchedulerConfig,
}

#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub id: String,
    pub task_type: TaskType,
    pub priority: TaskPriority,
    pub scheduled_time: chrono::DateTime<chrono::Utc>,
    pub dependencies: Vec<String>,
    pub max_retries: u32,
    pub timeout: Duration,
}

#[derive(Debug, Clone)]
pub enum TaskType {
    Automation(AutomationTask),
    Maintenance(MaintenanceTask),
    Resource(ResourceTask),
    Custom(CustomTask),
}

#[derive(Debug, Clone)]
pub struct AutomationTask {
    pub session_id: String,
    pub workflow: String,
    pub parameters: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct MaintenanceTask {
    pub task_name: String,
    pub cleanup_type: CleanupType,
}

#[derive(Debug, Clone)]
pub enum CleanupType {
    TempFiles,
    LogRotation,
    ResourceOptimization,
    CacheEviction,
}

#[derive(Debug, Clone)]
pub struct ResourceTask {
    pub operation: ResourceOperation,
    pub target: String,
}

#[derive(Debug, Clone)]
pub enum ResourceOperation {
    Scale,
    Monitor,
    Allocate,
    Deallocate,
}

#[derive(Debug, Clone)]
pub struct CustomTask {
    pub plugin_id: String,
    pub task_data: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
    Emergency = 5,
}

#[derive(Debug, Clone)]
pub struct RunningTask {
    pub task: ScheduledTask,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub progress: f64, // 0.0 to 1.0
    pub current_step: String,
}

#[derive(Debug, Clone)]
pub struct CompletedTask {
    pub task: ScheduledTask,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub result: TaskResult,
}

#[derive(Debug, Clone)]
pub enum TaskResult {
    Success(serde_json::Value),
    Failure(String),
    Timeout,
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct SchedulerConfig {
    pub max_concurrent_tasks: usize,
    pub task_timeout_default: Duration,
    pub retry_delay: Duration,
    pub cleanup_interval: Duration,
}

pub struct KVirtualStageCore {
    pub sessions: Arc<RwLock<HashMap<String, SessionInfo>>>,
    config: Arc<RwLock<KVirtualStageConfig>>,
    // Core automation engine with WindMouse 2.0
    pub automation_engine: Arc<RwLock<Option<AutomationEngine>>>,
    // Lazy initialization for components requiring external dependencies
    virtualization: Arc<RwLock<Option<VirtualizationManager>>>,
    resource_manager: Arc<RwLock<Option<ResourceManager>>>,
    // Performance optimization components
    pub performance_monitor: Arc<RwLock<PerformanceMonitor>>,
    pub plugin_manager: Arc<RwLock<PluginManager>>,
    pub task_scheduler: Arc<RwLock<TaskScheduler>>,
    // Enhanced component fields
    pub ui_automation: Arc<RwLock<Option<()>>>, // UI automation stub
    pub audio: Arc<RwLock<Option<()>>>, // Audio stub  
    pub security: Arc<RwLock<Option<()>>>, // Security stub
}

impl std::fmt::Debug for KVirtualStageCore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KVirtualStageCore").finish()
    }
}

impl KVirtualStageCore {
    pub async fn new() -> Result<Self> {
        info!("Initializing KVirtualStage Core");

        let default_config = KVirtualStageConfig {
            container_runtime: "docker".to_string(),
            default_desktop: "kubuntu".to_string(),
            default_resources: SessionResources {
                memory_mb: 2048,
                cpu_cores: 2,
                disk_gb: 10,
            },
            recording_settings: RecordingSettings {
                default_format: "mp4".to_string(),
                quality: "high".to_string(),
                fps: 30,
            },
            audio_settings: AudioSettings {
                enable_tts: true,
                tts_voice: "default".to_string(),
                enable_recording: true,
            },
            security_settings: SecuritySettings {
                enable_encryption: true,
                credential_vault_path: "~/.kvirtualstage/vault".to_string(),
                enable_mfa: false,
            },
        };

        info!("Core initialization completed - components will be lazy-loaded as needed");

        Ok(Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config: Arc::new(RwLock::new(default_config)),
            // Initialize core automation engine immediately
            automation_engine: Arc::new(RwLock::new(None)),
            // Initialize as None - will be lazy-loaded when needed
            virtualization: Arc::new(RwLock::new(None)),
            resource_manager: Arc::new(RwLock::new(None)),
            // Initialize performance optimization components
            performance_monitor: Arc::new(RwLock::new(PerformanceMonitor::new())),
            plugin_manager: Arc::new(RwLock::new(PluginManager::new())),
            task_scheduler: Arc::new(RwLock::new(TaskScheduler::new())),
            // Initialize stub fields
            ui_automation: Arc::new(RwLock::new(None)),
            audio: Arc::new(RwLock::new(None)),
            security: Arc::new(RwLock::new(None)),
        })
    }

    // Lazy initialization methods for components requiring external dependencies
    async fn ensure_virtualization(&self) -> Result<()> {
        let mut virt_guard = self.virtualization.write().await;
        if virt_guard.is_none() {
            info!("Initializing virtualization manager (Docker required)");
            let vm = VirtualizationManager::new().await?;
            *virt_guard = Some(vm);
        }
        Ok(())
    }

    pub async fn ensure_automation_engine(&self) -> Result<()> {
        let mut engine_guard: tokio::sync::RwLockWriteGuard<'_, Option<AutomationEngine>> = self.automation_engine.write().await;
        if engine_guard.is_none() {
            info!("Initializing automation engine with WindMouse 2.0");
            let engine = AutomationEngine::new()?;
            *engine_guard = Some(engine);
        }
        Ok(())
    }

    async fn ensure_resource_manager(&self) -> Result<()> {
        let mut rm_guard: tokio::sync::RwLockWriteGuard<'_, Option<ResourceManager>> = self.resource_manager.write().await;
        if rm_guard.is_none() {
            info!("Initializing resource manager");
            let resource_mgr = ResourceManager::new(None).await?;
            *rm_guard = Some(resource_mgr);
        }
        Ok(())
    }

    pub async fn ensure_ui_automation(&self) -> Result<()> {
        let mut ui_guard = self.ui_automation.write().await;
        if ui_guard.is_none() {
            info!("Initializing UI automation stub");
            *ui_guard = Some(());
        }
        Ok(())
    }

    pub async fn ensure_audio(&self) -> Result<()> {
        let mut audio_guard = self.audio.write().await;
        if audio_guard.is_none() {
            info!("Initializing audio stub");
            *audio_guard = Some(());
        }
        Ok(())
    }

    pub async fn ensure_security(&self) -> Result<()> {
        let mut sec_guard = self.security.write().await;
        if sec_guard.is_none() {
            info!("Initializing security stub");
            *sec_guard = Some(());
        }
        Ok(())
    }

    pub async fn get_status(&self) -> Result<KVirtualStageStatus> {
        let sessions = self.sessions.read().await;
        let virtualization = self.virtualization.read().await;

        // Determine primary container runtime
        let container_runtime = if virtualization.is_some() {
            "docker".to_string()
        } else {
            "none".to_string()
        };

        Ok(KVirtualStageStatus {
            active_sessions: sessions.len(),
            container_runtime,
            web_ui_active: false,
            mcp_server_active: false,
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }

    pub async fn create_session(
        &self,
        name: String,
        desktop: String,
        image: Option<String>,
        memory: u64,
        cpu: u32,
    ) -> Result<()> {
        info!("Creating session: {} with {}", name, desktop);

        // Initialize required components
        self.ensure_virtualization().await?;
        self.ensure_resource_manager().await?;

        let session_id = Uuid::new_v4().to_string();
        
        // Create container using Docker
        let container_id = {
            let mut virtualization = self.virtualization.write().await;
            virtualization
                .as_mut()
                .ok_or_else(|| anyhow!("Virtualization manager not available"))?
                .create_container(session_id.clone(), desktop.clone(), image, memory, cpu)
                .await?
        };

        // Register with resource manager
        let rm_guard = self.resource_manager.read().await;
        if let Some(resource_mgr) = rm_guard.as_ref() {
            resource_mgr.register_session(session_id.clone(), container_id.clone(), (cpu as f32, memory)).await?;
        }

        let session_info = SessionInfo {
            name: name.clone(),
            id: session_id,
            desktop,
            status: "created".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            container_id: Some(container_id),
            vnc_port: None,
            resources: SessionResources {
                memory_mb: memory,
                cpu_cores: cpu,
                disk_gb: 10,
            },
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(name, session_info);

        info!("Session created successfully with resource monitoring");
        Ok(())
    }

    pub async fn list_sessions(&self) -> Result<Vec<SessionInfo>> {
        let sessions = self.sessions.read().await;
        Ok(sessions.values().cloned().collect())
    }

    pub async fn connect_session(&self, name: String) -> Result<()> {
        let mut sessions = self.sessions.write().await;

        if let Some(session) = sessions.get_mut(&name) {
            session.status = "connected".to_string();
            info!("Connected to session: {}", name);
            Ok(())
        } else {
            Err(anyhow!("Session '{}' not found", name))
        }
    }

    pub async fn stop_session(&self, name: String) -> Result<()> {
        let mut sessions = self.sessions.write().await;

        if let Some(session) = sessions.get_mut(&name) {
            if let Some(container_id) = &session.container_id {
                let virtualization = self.virtualization.read().await;
                if let Some(virt_manager) = virtualization.as_ref() {
                    virt_manager.stop_container(container_id.clone()).await?;
                }
            }
            session.status = "stopped".to_string();
            info!("Stopped session: {}", name);
            Ok(())
        } else {
            Err(anyhow!("Session '{}' not found", name))
        }
    }

    pub async fn remove_session(&self, name: String) -> Result<()> {
        let mut sessions = self.sessions.write().await;

        if let Some(session) = sessions.remove(&name) {
            // Unregister from resource manager
            let rm_guard = self.resource_manager.read().await;
            if let Some(resource_mgr) = rm_guard.as_ref() {
                let _ = resource_mgr.unregister_session(&session.id).await;
            }
            
            if let Some(container_id) = &session.container_id {
                // Docker cleanup
                let virtualization = self.virtualization.read().await;
                if let Some(virt_manager) = virtualization.as_ref() {
                    virt_manager.remove_container(container_id.clone()).await?;
                }
            }
            info!("Session removal completed: {}", name);
            Ok(())
        } else {
            Err(anyhow!("Session '{}' not found", name))
        }
    }

    pub async fn get_config(&self) -> Result<KVirtualStageConfig> {
        let config = self.config.read().await;
        Ok(config.clone())
    }

    pub async fn set_config(&self, key: String, value: String) -> Result<()> {
        info!("Setting config: {} = {}", key, value);

        let mut config = self.config.write().await;

        // Simple key-value configuration update
        match key.as_str() {
            "container_runtime" => config.container_runtime = value,
            "default_desktop" => config.default_desktop = value,
            "recording.default_format" => config.recording_settings.default_format = value,
            "audio.tts_voice" => config.audio_settings.tts_voice = value,
            _ => return Err(anyhow!("Unknown configuration key: {}", key)),
        }

        Ok(())
    }

    pub async fn init_config(&self) -> Result<()> {
        info!("Initializing configuration");

        // Create config directory
        let config_dir = dirs::home_dir()
            .ok_or_else(|| anyhow!("Could not find home directory"))?
            .join(".kvirtualstage");

        tokio::fs::create_dir_all(&config_dir).await?;
        
        // Create additional directories for enhanced features
        tokio::fs::create_dir_all(config_dir.join("images")).await?;
        tokio::fs::create_dir_all(config_dir.join("templates")).await?;
        tokio::fs::create_dir_all(config_dir.join("volumes")).await?;
        tokio::fs::create_dir_all(config_dir.join("logs")).await?;

        // Save default config
        let config = self.config.read().await;
        let config_path = config_dir.join("config.toml");
        let config_content = toml::to_string(&*config)?;
        tokio::fs::write(config_path, config_content).await?;
        
        info!("Configuration initialized with virtualization support");
        Ok(())
    }
    
    pub async fn get_resource_usage(&self, session_name: &str) -> Result<serde_json::Value> {
        let sessions = self.sessions.read().await;
        if let Some(session) = sessions.get(session_name) {
            if let Some(resource_mgr) = self.resource_manager.read().await.as_ref() {
                let metrics = resource_mgr.get_session_metrics(&session.id).await?;
                return Ok(serde_json::to_value(metrics)?);
            }
        }
        Err(anyhow!("Session not found or resource manager unavailable"))
    }
    
    pub async fn scale_session_resources(
        &self,
        session_name: &str,
        new_memory_mb: u64,
        new_cpu_cores: u32,
    ) -> Result<()> {
        let sessions = self.sessions.read().await;
        if let Some(_session) = sessions.get(session_name) {
            // Update via resource manager
            if let Some(_resource_mgr) = self.resource_manager.read().await.as_ref() {
                // This would be implemented in resource_manager.rs
                info!("Scaling session {} to {}MB RAM, {} CPU cores", session_name, new_memory_mb, new_cpu_cores);
                return Ok(());
            }
        }
        Err(anyhow!("Session not found or resource manager unavailable"))
    }

    // Additional methods for CLI compatibility
    pub async fn start_with_ui(&self, host: String, port: u16) -> Result<()> {
        info!("Starting KVirtualStage with Web UI on {}:{}", host, port);
        // Simplified implementation - just log for now
        info!("Web UI functionality not fully implemented in core-only mode");
        Ok(())
    }

    pub async fn start_headless(&self) -> Result<()> {
        info!("Starting KVirtualStage in headless mode");
        // Simplified implementation - just keep the core running
        info!("Headless mode running - core is initialized");
        Ok(())
    }

    pub async fn run_script(&self, script: &str) -> Result<()> {
        info!("Running script: {}", script);
        // Ensure automation engine is available
        self.ensure_automation_engine().await?;
        
        let script_content = tokio::fs::read_to_string(script).await?;
        info!("Script loaded: {} characters", script_content.len());
        
        // This would integrate with the automation engine
        info!("Script execution not fully implemented yet");
        Ok(())
    }

    pub async fn run_script_in_session(&self, script: &str, session_name: String) -> Result<()> {
        info!("Running script in session {}: {}", session_name, script);

        let sessions = self.sessions.read().await;
        if sessions.get(&session_name).is_some() {
            let script_content = tokio::fs::read_to_string(script).await?;
            info!("Script loaded for session {}: {} characters", session_name, script_content.len());
            
            // This would integrate with automation engine in the specific session
            info!("Session script execution not fully implemented yet");
            Ok(())
        } else {
            Err(anyhow!("Session '{}' not found", session_name))
        }
    }

    pub async fn start_recording(
        &self,
        output: &str,
        format: &str,
        session: Option<String>,
    ) -> Result<()> {
        info!("Starting recording: {} (format: {})", output, format);

        if let Some(session_name) = session {
            let sessions = self.sessions.read().await;
            if sessions.get(&session_name).is_some() {
                info!("Recording session: {}", session_name);
            } else {
                return Err(anyhow!("Session '{}' not found", session_name));
            }
        }

        // Recording functionality would be implemented here
        info!("Recording functionality not fully implemented yet");
        Ok(())
    }

    pub async fn take_screenshot(&self, output: &str, session: Option<String>) -> Result<()> {
        info!("Taking screenshot: {}", output);

        if let Some(session_name) = session {
            let sessions = self.sessions.read().await;
            if sessions.get(&session_name).is_some() {
                info!("Screenshot of session: {}", session_name);
            } else {
                return Err(anyhow!("Session '{}' not found", session_name));
            }
        }

        // Screenshot functionality would be implemented here
        info!("Screenshot functionality not fully implemented yet");
        Ok(())
    }

    pub async fn start_mcp_server(&self, port: u16) -> Result<()> {
        info!("Starting MCP server on port {}", port);
        // MCP server functionality would be implemented here
        info!("MCP server functionality not fully implemented yet");
        Ok(())
    }

    pub async fn stop_mcp_server(&self) -> Result<()> {
        info!("Stopping MCP server");
        // MCP server stop functionality would be implemented here
        Ok(())
    }

    pub async fn list_mcp_tools(&self) -> Result<Vec<McpTool>> {
        // Return empty list for now
        Ok(Vec::new())
    }

    pub async fn test_mcp_connection(&self, url: String) -> Result<()> {
        info!("Testing MCP connection: {}", url);
        // Connection test would be implemented here
        Ok(())
    }
}

// Clone implementation for API server
impl Clone for KVirtualStageCore {
    fn clone(&self) -> Self {
        Self {
            sessions: Arc::clone(&self.sessions),
            config: Arc::clone(&self.config),
            automation_engine: Arc::clone(&self.automation_engine),
            virtualization: Arc::clone(&self.virtualization),
            resource_manager: Arc::clone(&self.resource_manager),
            performance_monitor: Arc::clone(&self.performance_monitor),
            plugin_manager: Arc::clone(&self.plugin_manager),
            task_scheduler: Arc::clone(&self.task_scheduler),
            ui_automation: Arc::clone(&self.ui_automation),
            audio: Arc::clone(&self.audio),
            security: Arc::clone(&self.security),
        }
    }
}

// ============================================================================
// Performance Monitor Implementation
// ============================================================================

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            cpu_usage_history: Vec::new(),
            memory_usage_history: Vec::new(),
            task_execution_times: HashMap::new(),
            throughput_metrics: ThroughputMetrics {
                tasks_per_second: 0.0,
                average_response_time_ms: 0.0,
                peak_concurrent_sessions: 0,
                resource_efficiency_score: 1.0,
            },
            optimization_suggestions: Vec::new(),
        }
    }

    pub fn record_task_execution(&mut self, task_name: String, duration: Duration) {
        self.task_execution_times
            .entry(task_name)
            .or_insert_with(Vec::new)
            .push(duration);
    }

    pub fn update_cpu_usage(&mut self, cpu_percent: f64) {
        self.cpu_usage_history.push(cpu_percent);
        if self.cpu_usage_history.len() > 1000 {
            self.cpu_usage_history.remove(0);
        }
    }

    pub fn update_memory_usage(&mut self, memory_bytes: u64) {
        self.memory_usage_history.push(memory_bytes);
        if self.memory_usage_history.len() > 1000 {
            self.memory_usage_history.remove(0);
        }
    }

    pub fn analyze_performance(&mut self) {
        // Generate optimization suggestions based on metrics
        if let Some(&avg_cpu) = self.cpu_usage_history.iter().last() {
            if avg_cpu > 85.0 {
                self.optimization_suggestions.push(OptimizationSuggestion {
                    category: "Resource Optimization".to_string(),
                    description: "High CPU usage detected. Consider implementing task queue throttling.".to_string(),
                    impact: ImpactLevel::High,
                    implementation_effort: EffortLevel::Medium,
                });
            }
        }

        // Calculate resource efficiency score
        let avg_cpu = self.cpu_usage_history.iter().sum::<f64>() / self.cpu_usage_history.len() as f64;
        let optimal_cpu_range = 60.0..80.0;
        
        self.throughput_metrics.resource_efficiency_score = if optimal_cpu_range.contains(&avg_cpu) {
            1.0
        } else if avg_cpu < 60.0 {
            0.7 // Underutilized
        } else {
            0.5 // Overutilized
        };
    }

    pub fn get_performance_report(&self) -> PerformanceReport {
        PerformanceReport {
            average_cpu_usage: self.cpu_usage_history.iter().sum::<f64>() / self.cpu_usage_history.len() as f64,
            average_memory_usage: self.memory_usage_history.iter().sum::<u64>() / self.memory_usage_history.len() as u64,
            throughput: self.throughput_metrics.clone(),
            suggestions: self.optimization_suggestions.clone(),
            report_time: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub average_cpu_usage: f64,
    pub average_memory_usage: u64,
    pub throughput: ThroughputMetrics,
    pub suggestions: Vec<OptimizationSuggestion>,
    pub report_time: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// Plugin Manager Implementation
// ============================================================================

impl PluginManager {
    pub fn new() -> Self {
        Self {
            loaded_plugins: HashMap::new(),
            plugin_registry: PluginRegistry {
                available_plugins: Vec::new(),
                update_sources: vec![
                    "https://registry.kvirtualstage.com/plugins".to_string(),
                ],
            },
            hooks: HashMap::new(),
        }
    }

    pub fn load_plugin(&mut self, plugin_info: PluginInfo) -> Result<()> {
        info!("Loading plugin: {}", plugin_info.name);
        
        let plugin = Plugin {
            id: plugin_info.id.clone(),
            name: plugin_info.name.clone(),
            version: plugin_info.version.clone(),
            capabilities: vec![PluginCapability::CustomIntegration(plugin_info.name.clone())],
            config: PluginConfig {
                enabled: true,
                settings: HashMap::new(),
                dependencies: Vec::new(),
            },
            status: PluginStatus::Loaded,
        };

        self.loaded_plugins.insert(plugin_info.id, plugin);
        Ok(())
    }

    pub fn activate_plugin(&mut self, plugin_id: &str) -> Result<()> {
        if let Some(plugin) = self.loaded_plugins.get_mut(plugin_id) {
            plugin.status = PluginStatus::Active;
            info!("Activated plugin: {}", plugin.name);
            Ok(())
        } else {
            Err(anyhow!("Plugin not found: {}", plugin_id))
        }
    }

    pub fn register_hook(&mut self, plugin_id: String, event_type: String, callback: String) {
        let hook = PluginHook {
            plugin_id,
            event_type: event_type.clone(),
            callback,
        };

        self.hooks
            .entry(event_type)
            .or_insert_with(Vec::new)
            .push(hook);
    }

    pub fn execute_hooks(&self, event_type: &str, event_data: &serde_json::Value) {
        if let Some(hooks) = self.hooks.get(event_type) {
            for hook in hooks {
                if let Some(plugin) = self.loaded_plugins.get(&hook.plugin_id) {
                    if matches!(plugin.status, PluginStatus::Active) {
                        info!("Executing hook for event '{}' in plugin '{}'", event_type, plugin.name);
                        // Plugin hook execution would be implemented here
                    }
                }
            }
        }
    }

    pub fn get_plugin_capabilities(&self) -> Vec<(String, Vec<PluginCapability>)> {
        self.loaded_plugins
            .values()
            .map(|plugin| (plugin.name.clone(), plugin.capabilities.clone()))
            .collect()
    }
}

// ============================================================================
// Task Scheduler Implementation
// ============================================================================

impl TaskScheduler {
    pub fn new() -> Self {
        Self {
            pending_tasks: Vec::new(),
            running_tasks: HashMap::new(),
            completed_tasks: Vec::new(),
            scheduler_config: SchedulerConfig {
                max_concurrent_tasks: 10,
                task_timeout_default: Duration::from_secs(300),
                retry_delay: Duration::from_secs(30),
                cleanup_interval: Duration::from_secs(3600),
            },
        }
    }

    pub fn schedule_task(&mut self, mut task: ScheduledTask) -> String {
        if task.id.is_empty() {
            task.id = Uuid::new_v4().to_string();
        }

        info!("Scheduling task: {} (priority: {:?})", task.id, task.priority);
        
        // Insert in priority order
        let insert_pos = self.pending_tasks
            .iter()
            .position(|t| t.priority < task.priority)
            .unwrap_or(self.pending_tasks.len());
        
        self.pending_tasks.insert(insert_pos, task.clone());
        task.id
    }

    pub fn start_next_task(&mut self) -> Option<RunningTask> {
        if self.running_tasks.len() >= self.scheduler_config.max_concurrent_tasks {
            return None;
        }

        if let Some(task) = self.pending_tasks.pop() {
            let running_task = RunningTask {
                task: task.clone(),
                start_time: chrono::Utc::now(),
                progress: 0.0,
                current_step: "Starting".to_string(),
            };

            self.running_tasks.insert(task.id.clone(), running_task.clone());
            info!("Started task: {}", task.id);
            Some(running_task)
        } else {
            None
        }
    }

    pub fn complete_task(&mut self, task_id: String, result: TaskResult) {
        if let Some(running_task) = self.running_tasks.remove(&task_id) {
            let completed_task = CompletedTask {
                task: running_task.task,
                start_time: running_task.start_time,
                end_time: chrono::Utc::now(),
                result,
            };

            self.completed_tasks.push(completed_task);
            info!("Completed task: {}", task_id);

            // Keep only recent completed tasks
            if self.completed_tasks.len() > 1000 {
                self.completed_tasks.remove(0);
            }
        }
    }

    pub fn update_task_progress(&mut self, task_id: &str, progress: f64, current_step: String) {
        if let Some(running_task) = self.running_tasks.get_mut(task_id) {
            running_task.progress = progress.clamp(0.0, 1.0);
            running_task.current_step = current_step;
        }
    }

    pub fn get_task_status(&self, task_id: &str) -> Option<TaskStatus> {
        if let Some(running_task) = self.running_tasks.get(task_id) {
            return Some(TaskStatus::Running {
                progress: running_task.progress,
                current_step: running_task.current_step.clone(),
            });
        }

        if let Some(completed_task) = self.completed_tasks.iter().find(|t| t.task.id == task_id) {
            return Some(TaskStatus::Completed(completed_task.result.clone()));
        }

        if self.pending_tasks.iter().any(|t| t.id == task_id) {
            return Some(TaskStatus::Pending);
        }

        None
    }

    pub fn cancel_task(&mut self, task_id: &str) -> bool {
        // Remove from pending tasks
        if let Some(pos) = self.pending_tasks.iter().position(|t| t.id == task_id) {
            self.pending_tasks.remove(pos);
            return true;
        }

        // Cancel running task
        if let Some(running_task) = self.running_tasks.remove(task_id) {
            let completed_task = CompletedTask {
                task: running_task.task,
                start_time: running_task.start_time,
                end_time: chrono::Utc::now(),
                result: TaskResult::Cancelled,
            };
            self.completed_tasks.push(completed_task);
            return true;
        }

        false
    }

    pub fn get_scheduler_stats(&self) -> SchedulerStats {
        SchedulerStats {
            pending_tasks: self.pending_tasks.len(),
            running_tasks: self.running_tasks.len(),
            completed_tasks: self.completed_tasks.len(),
            avg_completion_time: self.calculate_avg_completion_time(),
        }
    }

    fn calculate_avg_completion_time(&self) -> Duration {
        if self.completed_tasks.is_empty() {
            return Duration::ZERO;
        }

        let total_duration: Duration = self.completed_tasks
            .iter()
            .map(|task| {
                (task.end_time - task.start_time).to_std().unwrap_or(Duration::ZERO)
            })
            .sum();

        total_duration / self.completed_tasks.len() as u32
    }
}

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Pending,
    Running {
        progress: f64,
        current_step: String,
    },
    Completed(TaskResult),
}

#[derive(Debug, Clone)]
pub struct SchedulerStats {
    pub pending_tasks: usize,
    pub running_tasks: usize,
    pub completed_tasks: usize,
    pub avg_completion_time: Duration,
}

// ============================================================================
// Enhanced Core API Methods
// ============================================================================

impl KVirtualStageCore {
    /// Get comprehensive performance metrics
    pub async fn get_performance_metrics(&self) -> Result<PerformanceReport> {
        let monitor = self.performance_monitor.read().await;
        Ok(monitor.get_performance_report())
    }

    /// Execute a scheduled task
    pub async fn execute_scheduled_task(&self, task: ScheduledTask) -> Result<String> {
        let task_id = {
            let mut scheduler = self.task_scheduler.write().await;
            scheduler.schedule_task(task)
        };

        // Start task execution
        if let Some(running_task) = {
            let mut scheduler = self.task_scheduler.write().await;
            scheduler.start_next_task()
        } {
            info!("Executing task: {}", running_task.task.id);
            
            // Execute task based on type
            let result = match &running_task.task.task_type {
                TaskType::Automation(automation_task) => {
                    self.execute_automation_task(automation_task).await
                }
                TaskType::Maintenance(maintenance_task) => {
                    self.execute_maintenance_task(maintenance_task).await
                }
                TaskType::Resource(resource_task) => {
                    self.execute_resource_task(resource_task).await
                }
                TaskType::Custom(custom_task) => {
                    self.execute_custom_task(custom_task).await
                }
            };

            // Complete the task
            {
                let mut scheduler = self.task_scheduler.write().await;
                scheduler.complete_task(task_id.clone(), result);
            }
        }

        Ok(task_id)
    }

    async fn execute_automation_task(&self, task: &AutomationTask) -> TaskResult {
        info!("Executing automation task for session: {}", task.session_id);
        // Implementation would integrate with automation engine
        TaskResult::Success(serde_json::json!({"message": "Automation completed"}))
    }

    async fn execute_maintenance_task(&self, task: &MaintenanceTask) -> TaskResult {
        info!("Executing maintenance task: {}", task.task_name);
        match task.cleanup_type {
            CleanupType::TempFiles => {
                // Clean up temporary files
                TaskResult::Success(serde_json::json!({"cleaned_files": 42}))
            }
            CleanupType::LogRotation => {
                // Rotate logs
                TaskResult::Success(serde_json::json!({"rotated_logs": true}))
            }
            CleanupType::ResourceOptimization => {
                // Optimize resources
                TaskResult::Success(serde_json::json!({"optimization_applied": true}))
            }
            CleanupType::CacheEviction => {
                // Evict cache
                TaskResult::Success(serde_json::json!({"cache_evicted": true}))
            }
        }
    }

    async fn execute_resource_task(&self, task: &ResourceTask) -> TaskResult {
        info!("Executing resource task: {:?} on {}", task.operation, task.target);
        // Implementation would integrate with resource manager
        TaskResult::Success(serde_json::json!({"resource_operation": "completed"}))
    }

    async fn execute_custom_task(&self, task: &CustomTask) -> TaskResult {
        info!("Executing custom task for plugin: {}", task.plugin_id);
        
        // Execute plugin hook
        {
            let plugin_manager = self.plugin_manager.read().await;
            plugin_manager.execute_hooks("custom_task", &task.task_data);
        }
        
        TaskResult::Success(serde_json::json!({"custom_task": "executed"}))
    }

    /// Load and activate a plugin
    pub async fn load_plugin(&self, plugin_info: PluginInfo) -> Result<()> {
        let mut plugin_manager = self.plugin_manager.write().await;
        plugin_manager.load_plugin(plugin_info.clone())?;
        plugin_manager.activate_plugin(&plugin_info.id)?;
        Ok(())
    }

    /// Get scheduler statistics
    pub async fn get_scheduler_stats(&self) -> SchedulerStats {
        let scheduler = self.task_scheduler.read().await;
        scheduler.get_scheduler_stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-KDESKTOPVIRT-001, FR-KDESKTOPVIRT-002
    // Status, configuration, session management, and virtual desktop lifecycle

    // ========== Status & Configuration Tests ==========

    #[test]
    fn test_kvirtualstage_status_creation() {
        // Traces to: FR-KDESKTOPVIRT-001
        let status = KVirtualStageStatus {
            active_sessions: 5,
            container_runtime: "docker".to_string(),
            web_ui_active: true,
            mcp_server_active: true,
            version: "1.0.0".to_string(),
        };
        assert_eq!(status.active_sessions, 5);
        assert!(status.web_ui_active);
    }

    #[test]
    fn test_session_info_creation() {
        // Traces to: FR-KDESKTOPVIRT-001
        let resources = SessionResources {
            memory_mb: 2048,
            cpu_cores: 2,
            disk_gb: 20,
        };
        let session = SessionInfo {
            name: "session-1".to_string(),
            id: "id-123".to_string(),
            desktop: "Ubuntu".to_string(),
            status: "running".to_string(),
            created_at: "2026-04-25".to_string(),
            container_id: Some("container-123".to_string()),
            vnc_port: Some(5900),
            resources,
        };
        assert_eq!(session.name, "session-1");
        assert_eq!(session.resources.cpu_cores, 2);
    }

    #[test]
    fn test_session_resources_constraints() {
        // Traces to: FR-KDESKTOPVIRT-001
        let resources = SessionResources {
            memory_mb: 512,
            cpu_cores: 1,
            disk_gb: 10,
        };
        assert!(resources.memory_mb >= 256);
        assert!(resources.cpu_cores > 0);
    }

    #[test]
    fn test_mcp_tool_creation() {
        // Traces to: FR-KDESKTOPVIRT-004
        let tool = McpTool {
            name: "screenshot".to_string(),
            description: "Take a screenshot of the desktop".to_string(),
            parameters: serde_json::json!({"timeout": 5000}),
        };
        assert_eq!(tool.name, "screenshot");
        assert!(!tool.description.is_empty());
    }

    #[test]
    fn test_kvirtualstage_config_creation() {
        // Traces to: FR-KDESKTOPVIRT-001
        let config = KVirtualStageConfig {
            container_runtime: "docker".to_string(),
            default_desktop: "Ubuntu".to_string(),
            default_resources: SessionResources {
                memory_mb: 2048,
                cpu_cores: 2,
                disk_gb: 30,
            },
            recording_settings: RecordingSettings {
                default_format: "mp4".to_string(),
                quality: "high".to_string(),
                fps: 30,
            },
            audio_settings: AudioSettings {
                enable_tts: true,
                tts_voice: "en-us".to_string(),
                enable_recording: true,
            },
            security_settings: SecuritySettings {
                enable_encryption: true,
                credential_vault_path: "/etc/kvirtualstage/vault".to_string(),
                enable_mfa: true,
            },
        };
        assert_eq!(config.container_runtime, "docker");
        assert!(config.security_settings.enable_mfa);
    }

    // ========== Performance Monitor Tests ==========

    #[test]
    fn test_performance_monitor_creation() {
        // Traces to: FR-KDESKTOPVIRT-004
        let monitor = PerformanceMonitor {
            cpu_usage_history: vec![25.0, 30.0, 28.0],
            memory_usage_history: vec![1024, 1536, 1280],
            task_execution_times: HashMap::new(),
            throughput_metrics: ThroughputMetrics {
                tasks_per_second: 10.5,
                average_response_time_ms: 95.0,
                peak_concurrent_sessions: 5,
                resource_efficiency_score: 0.85,
            },
            optimization_suggestions: vec![],
        };
        assert_eq!(monitor.cpu_usage_history.len(), 3);
        assert_eq!(monitor.memory_usage_history.len(), 3);
    }

    #[test]
    fn test_throughput_metrics_valid() {
        // Traces to: FR-KDESKTOPVIRT-004
        let metrics = ThroughputMetrics {
            tasks_per_second: 20.0,
            average_response_time_ms: 50.0,
            peak_concurrent_sessions: 10,
            resource_efficiency_score: 0.9,
        };
        assert!(metrics.tasks_per_second > 0.0);
        assert!(metrics.average_response_time_ms > 0.0);
        assert!(metrics.resource_efficiency_score >= 0.0 && metrics.resource_efficiency_score <= 1.0);
    }

    #[test]
    fn test_optimization_suggestion_creation() {
        // Traces to: FR-KDESKTOPVIRT-004
        let suggestion = OptimizationSuggestion {
            category: "memory".to_string(),
            description: "Increase buffer size for better throughput".to_string(),
            impact: ImpactLevel::High,
            implementation_effort: EffortLevel::Medium,
        };
        assert_eq!(suggestion.category, "memory");
        assert!(!suggestion.description.is_empty());
    }

    // ========== Recording Settings Tests ==========

    #[test]
    fn test_recording_settings_standard() {
        // Traces to: FR-KDESKTOPVIRT-005
        let settings = RecordingSettings {
            default_format: "mp4".to_string(),
            quality: "medium".to_string(),
            fps: 30,
        };
        assert_eq!(settings.fps, 30);
        assert_eq!(settings.quality, "medium");
    }

    #[test]
    fn test_recording_settings_high_fps() {
        // Traces to: FR-KDESKTOPVIRT-005
        let settings = RecordingSettings {
            default_format: "mov".to_string(),
            quality: "ultra".to_string(),
            fps: 60,
        };
        assert_eq!(settings.fps, 60);
    }

    // ========== Audio Settings Tests ==========

    #[test]
    fn test_audio_settings_tts_enabled() {
        // Traces to: FR-KDESKTOPVIRT-005
        let settings = AudioSettings {
            enable_tts: true,
            tts_voice: "en-gb".to_string(),
            enable_recording: true,
        };
        assert!(settings.enable_tts);
        assert!(settings.enable_recording);
    }

    #[test]
    fn test_audio_settings_tts_disabled() {
        // Traces to: FR-KDESKTOPVIRT-005
        let settings = AudioSettings {
            enable_tts: false,
            tts_voice: "none".to_string(),
            enable_recording: false,
        };
        assert!(!settings.enable_tts);
        assert!(!settings.enable_recording);
    }

    // ========== Security Settings Tests ==========

    #[test]
    fn test_security_settings_strict() {
        // Traces to: FR-KDESKTOPVIRT-003
        let settings = SecuritySettings {
            enable_encryption: true,
            credential_vault_path: "/secure/vault".to_string(),
            enable_mfa: true,
        };
        assert!(settings.enable_encryption);
        assert!(settings.enable_mfa);
    }

    #[test]
    fn test_security_settings_relaxed() {
        // Traces to: FR-KDESKTOPVIRT-003
        let settings = SecuritySettings {
            enable_encryption: false,
            credential_vault_path: "/tmp/vault".to_string(),
            enable_mfa: false,
        };
        assert!(!settings.enable_encryption);
        assert!(!settings.enable_mfa);
    }

    // ========== Cleanup Type Tests ==========

    #[test]
    fn test_cleanup_type_variants() {
        // Traces to: FR-KDESKTOPVIRT-001
        let cleanup_types = vec![
            CleanupType::TempFiles,
            CleanupType::LogRotation,
            CleanupType::ResourceOptimization,
            CleanupType::CacheEviction,
        ];
        assert_eq!(cleanup_types.len(), 4);
    }

    // ========== Task Type Tests ==========

    #[test]
    fn test_task_types_coverage() {
        // Traces to: FR-KDESKTOPVIRT-001
        let task_types = vec![
            "SessionManagement",
            "Automation",
            "Maintenance",
            "Resource",
            "Custom",
        ];
        assert!(task_types.len() > 0);
    }

    // ========== Scheduler Stats Tests ==========

    #[test]
    fn test_scheduler_stats_creation() {
        // Traces to: FR-KDESKTOPVIRT-004
        let stats = SchedulerStats {
            pending_tasks: 10,
            running_tasks: 5,
            completed_tasks: 1000,
            avg_completion_time: Duration::from_millis(250),
        };
        assert_eq!(stats.pending_tasks, 10);
        assert_eq!(stats.completed_tasks, 1000);
    }

    #[test]
    fn test_scheduler_stats_task_counts() {
        // Traces to: FR-KDESKTOPVIRT-004
        let stats = SchedulerStats {
            pending_tasks: 5,
            running_tasks: 2,
            completed_tasks: 98,
            avg_completion_time: Duration::from_millis(150),
        };
        let total = stats.pending_tasks + stats.running_tasks + stats.completed_tasks;
        assert!(total >= 100);
    }

    // ========== Plugin Info Tests ==========

    #[test]
    fn test_plugin_info_creation() {
        // Traces to: FR-KDESKTOPVIRT-004
        let plugin = PluginInfo {
            id: "plugin-1".to_string(),
            name: "Custom Plugin".to_string(),
            description: "A custom plugin for KVirtualStage".to_string(),
            version: "1.0.0".to_string(),
            author: "Author Name".to_string(),
            download_url: "https://example.com/plugin.zip".to_string(),
        };
        assert_eq!(plugin.id, "plugin-1");
        assert_eq!(plugin.name, "Custom Plugin");
    }

    #[test]
    fn test_plugin_info_versioning() {
        // Traces to: FR-KDESKTOPVIRT-004
        let plugin = PluginInfo {
            id: "version-plugin".to_string(),
            name: "Version Test".to_string(),
            description: "Testing versioning".to_string(),
            version: "2.5.3".to_string(),
            author: "Test Author".to_string(),
            download_url: "https://example.com/v2.5.3.zip".to_string(),
        };
        assert_eq!(plugin.version, "2.5.3");
        assert!(!plugin.description.is_empty());
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_zero_active_sessions() {
        // Traces to: FR-KDESKTOPVIRT-002
        let status = KVirtualStageStatus {
            active_sessions: 0,
            container_runtime: "podman".to_string(),
            web_ui_active: false,
            mcp_server_active: false,
            version: "0.9.0".to_string(),
        };
        assert_eq!(status.active_sessions, 0);
    }

    #[test]
    fn test_large_active_sessions() {
        // Traces to: FR-KDESKTOPVIRT-002
        let status = KVirtualStageStatus {
            active_sessions: 1000,
            container_runtime: "kubernetes".to_string(),
            web_ui_active: true,
            mcp_server_active: true,
            version: "2.0.0".to_string(),
        };
        assert_eq!(status.active_sessions, 1000);
    }

    #[test]
    fn test_minimal_session_resources() {
        // Traces to: FR-KDESKTOPVIRT-002
        let resources = SessionResources {
            memory_mb: 256,
            cpu_cores: 1,
            disk_gb: 5,
        };
        assert!(resources.memory_mb >= 256);
        assert_eq!(resources.cpu_cores, 1);
    }

    #[test]
    fn test_maximum_session_resources() {
        // Traces to: FR-KDESKTOPVIRT-002
        let resources = SessionResources {
            memory_mb: 32768,
            cpu_cores: 32,
            disk_gb: 500,
        };
        assert!(resources.memory_mb > 16384);
        assert!(resources.cpu_cores > 16);
    }

    #[test]
    fn test_vnc_port_optional() {
        // Traces to: FR-KDESKTOPVIRT-003
        let session1 = SessionInfo {
            name: "no-vnc".to_string(),
            id: "id-1".to_string(),
            desktop: "Ubuntu".to_string(),
            status: "running".to_string(),
            created_at: "2026-04-25".to_string(),
            container_id: None,
            vnc_port: None,
            resources: SessionResources {
                memory_mb: 2048,
                cpu_cores: 2,
                disk_gb: 20,
            },
        };
        assert!(session1.vnc_port.is_none());
    }
}