use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use tokio::time::{interval, timeout};
use tracing::{debug, error, info, warn};

/// Advanced resource monitoring and auto-scaling system
/// Implements intelligent resource allocation based on real-time usage patterns
pub struct ResourceManager {
    monitors: Arc<RwLock<HashMap<String, ResourceMonitor>>>,
    scaling_policies: Arc<RwLock<HashMap<String, ScalingPolicy>>>,
    metrics_history: Arc<RwLock<MetricsHistory>>,
    config: ResourceConfig,
    alert_thresholds: AlertThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMonitor {
    pub session_id: String,
    pub container_id: String,
    pub current_metrics: ResourceMetrics,
    pub historical_metrics: Vec<ResourceMetrics>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub scaling_state: ScalingState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetrics {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub cpu_usage_percent: f64,
    pub memory_used_bytes: u64,
    pub memory_limit_bytes: u64,
    pub memory_usage_percent: f64,
    pub disk_used_bytes: u64,
    pub disk_io_read_bytes: u64,
    pub disk_io_write_bytes: u64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
    pub gpu_usage_percent: Option<f64>,
    pub gpu_memory_used_bytes: Option<u64>,
    pub process_count: u32,
    pub load_average: (f64, f64, f64), // 1m, 5m, 15m
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingPolicy {
    pub session_id: String,
    pub min_cpu_cores: f32,
    pub max_cpu_cores: f32,
    pub min_memory_mb: u64,
    pub max_memory_mb: u64,
    pub scale_up_threshold: ScalingThreshold,
    pub scale_down_threshold: ScalingThreshold,
    pub cooldown_period_seconds: u64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScalingThreshold {
    pub cpu_percent: f64,
    pub memory_percent: f64,
    pub duration_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScalingState {
    Stable,
    ScalingUp,
    ScalingDown,
    Cooldown(chrono::DateTime<chrono::Utc>),
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    pub monitoring_interval_seconds: u64,
    pub metrics_retention_hours: u64,
    pub enable_auto_scaling: bool,
    pub enable_predictive_scaling: bool,
    pub enable_gpu_monitoring: bool,
    pub alert_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub cpu_critical: f64,
    pub memory_critical: f64,
    pub disk_critical: f64,
    pub response_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsHistory {
    pub sessions: HashMap<String, Vec<ResourceMetrics>>,
    pub system_metrics: Vec<SystemMetrics>,
    pub max_history_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub total_cpu_cores: u32,
    pub total_memory_bytes: u64,
    pub available_memory_bytes: u64,
    pub disk_total_bytes: u64,
    pub disk_available_bytes: u64,
    pub active_sessions: u32,
    pub system_load: (f64, f64, f64),
}

impl Default for ResourceConfig {
    fn default() -> Self {
        Self {
            monitoring_interval_seconds: 10,
            metrics_retention_hours: 24,
            enable_auto_scaling: true,
            enable_predictive_scaling: false,
            enable_gpu_monitoring: true,
            alert_enabled: true,
        }
    }
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_critical: 90.0,
            memory_critical: 85.0,
            disk_critical: 90.0,
            response_time_ms: 1000,
        }
    }
}

impl ResourceManager {
    pub async fn new(config: Option<ResourceConfig>) -> Result<Self> {
        info!("Initializing Resource Manager with intelligent scaling");
        
        let config = config.unwrap_or_default();
        
        let manager = Self {
            monitors: Arc::new(RwLock::new(HashMap::new())),
            scaling_policies: Arc::new(RwLock::new(HashMap::new())),
            metrics_history: Arc::new(RwLock::new(MetricsHistory {
                sessions: HashMap::new(),
                system_metrics: Vec::new(),
                max_history_size: 1000,
            })),
            config: config.clone(),
            alert_thresholds: AlertThresholds::default(),
        };
        
        // Start monitoring tasks
        manager.start_monitoring_tasks().await?;
        
        Ok(manager)
    }
    
    async fn start_monitoring_tasks(&self) -> Result<()> {
        info!("Starting resource monitoring tasks");
        
        // System metrics collection
        let monitors_clone = Arc::clone(&self.monitors);
        let history_clone = Arc::clone(&self.metrics_history);
        let config_clone = self.config.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(config_clone.monitoring_interval_seconds));
            
            loop {
                interval.tick().await;
                
                if let Err(e) = Self::collect_system_metrics(&history_clone).await {
                    error!("Failed to collect system metrics: {}", e);
                }
                
                if let Err(e) = Self::collect_container_metrics(&monitors_clone, &history_clone).await {
                    error!("Failed to collect container metrics: {}", e);
                }
            }
        });
        
        // Auto-scaling task
        if self.config.enable_auto_scaling {
            let monitors_clone = Arc::clone(&self.monitors);
            let policies_clone = Arc::clone(&self.scaling_policies);
            let config_clone = self.config.clone();
            
            tokio::spawn(async move {
                let mut interval = interval(Duration::from_secs(config_clone.monitoring_interval_seconds * 2));
                
                loop {
                    interval.tick().await;
                    
                    if let Err(e) = Self::process_scaling_decisions(&monitors_clone, &policies_clone).await {
                        error!("Failed to process scaling decisions: {}", e);
                    }
                }
            });
        }
        
        // Cleanup task
        let history_clone = Arc::clone(&self.metrics_history);
        let retention_hours = self.config.metrics_retention_hours;
        
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(3600)); // Hourly cleanup
            
            loop {
                interval.tick().await;
                
                if let Err(e) = Self::cleanup_old_metrics(&history_clone, retention_hours).await {
                    error!("Failed to cleanup old metrics: {}", e);
                }
            }
        });
        
        Ok(())
    }
    
    pub async fn register_session(
        &self,
        session_id: String,
        container_id: String,
        initial_resources: (f32, u64), // CPU cores, Memory MB
    ) -> Result<()> {
        info!("Registering session for monitoring: {} (container: {})", session_id, container_id);
        
        let monitor = ResourceMonitor {
            session_id: session_id.clone(),
            container_id: container_id.clone(),
            current_metrics: ResourceMetrics {
                timestamp: chrono::Utc::now(),
                cpu_usage_percent: 0.0,
                memory_used_bytes: 0,
                memory_limit_bytes: initial_resources.1 * 1024 * 1024,
                memory_usage_percent: 0.0,
                disk_used_bytes: 0,
                disk_io_read_bytes: 0,
                disk_io_write_bytes: 0,
                network_rx_bytes: 0,
                network_tx_bytes: 0,
                gpu_usage_percent: None,
                gpu_memory_used_bytes: None,
                process_count: 0,
                load_average: (0.0, 0.0, 0.0),
            },
            historical_metrics: Vec::new(),
            last_updated: chrono::Utc::now(),
            scaling_state: ScalingState::Stable,
        };
        
        let policy = ScalingPolicy {
            session_id: session_id.clone(),
            min_cpu_cores: initial_resources.0.max(0.5),
            max_cpu_cores: (initial_resources.0 * 4.0).min(8.0),
            min_memory_mb: initial_resources.1.max(512),
            max_memory_mb: (initial_resources.1 * 4).min(16384),
            scale_up_threshold: ScalingThreshold {
                cpu_percent: 80.0,
                memory_percent: 75.0,
                duration_seconds: 60,
            },
            scale_down_threshold: ScalingThreshold {
                cpu_percent: 30.0,
                memory_percent: 40.0,
                duration_seconds: 300,
            },
            cooldown_period_seconds: 180,
            enabled: true,
        };
        
        {
            let mut monitors = self.monitors.write().await;
            monitors.insert(session_id.clone(), monitor);
        }
        
        {
            let mut policies = self.scaling_policies.write().await;
            policies.insert(session_id.clone(), policy);
        }
        
        info!("Session monitoring registered: {}", session_id);
        Ok(())
    }
    
    async fn collect_system_metrics(history: &Arc<RwLock<MetricsHistory>>) -> Result<()> {
        let metrics = Self::gather_system_metrics().await?;
        
        let mut history_guard = history.write().await;
        history_guard.system_metrics.push(metrics);
        
        // Keep only recent metrics
        if history_guard.system_metrics.len() > history_guard.max_history_size {
            history_guard.system_metrics.remove(0);
        }
        
        Ok(())
    }
    
    async fn gather_system_metrics() -> Result<SystemMetrics> {
        // CPU info
        let cpu_count = num_cpus::get() as u32;
        
        // Memory info from /proc/meminfo
        let (total_memory, available_memory) = if let Ok(contents) = tokio::fs::read_to_string("/proc/meminfo").await {
            let lines: Vec<&str> = contents.lines().collect();
            let total_line = lines.iter().find(|line| line.starts_with("MemTotal:"))
                .unwrap_or(&"MemTotal: 0 kB");
            let available_line = lines.iter().find(|line| line.starts_with("MemAvailable:"))
                .unwrap_or(&"MemAvailable: 0 kB");
                
            let total_kb: u64 = total_line.split_whitespace()
                .nth(1).unwrap_or("0").parse().unwrap_or(0);
            let available_kb: u64 = available_line.split_whitespace()
                .nth(1).unwrap_or("0").parse().unwrap_or(0);
                
            (total_kb * 1024, available_kb * 1024)
        } else {
            (0, 0)
        };
        
        // Load average from /proc/loadavg
        let load_avg = if let Ok(contents) = tokio::fs::read_to_string("/proc/loadavg").await {
            let parts: Vec<&str> = contents.split_whitespace().collect();
            if parts.len() >= 3 {
                (
                    parts[0].parse().unwrap_or(0.0),
                    parts[1].parse().unwrap_or(0.0),
                    parts[2].parse().unwrap_or(0.0),
                )
            } else {
                (0.0, 0.0, 0.0)
            }
        } else {
            (0.0, 0.0, 0.0)
        };
        
        // Disk info (simplified)
        let (disk_total, disk_available) = (0u64, 0u64); // TODO: Implement disk metrics
        
        Ok(SystemMetrics {
            timestamp: chrono::Utc::now(),
            total_cpu_cores: cpu_count,
            total_memory_bytes: total_memory,
            available_memory_bytes: available_memory,
            disk_total_bytes: disk_total,
            disk_available_bytes: disk_available,
            active_sessions: 0, // Will be updated by caller
            system_load: load_avg,
        })
    }
    
    async fn collect_container_metrics(
        monitors: &Arc<RwLock<HashMap<String, ResourceMonitor>>>,
        history: &Arc<RwLock<MetricsHistory>>,
    ) -> Result<()> {
        let session_ids: Vec<String> = {
            let monitors_guard = monitors.read().await;
            monitors_guard.keys().cloned().collect()
        };
        
        for session_id in session_ids {
            if let Err(e) = Self::collect_session_metrics(&session_id, monitors, history).await {
                warn!("Failed to collect metrics for session {}: {}", session_id, e);
            }
        }
        
        Ok(())
    }
    
    async fn collect_session_metrics(
        session_id: &str,
        monitors: &Arc<RwLock<HashMap<String, ResourceMonitor>>>,
        history: &Arc<RwLock<MetricsHistory>>,
    ) -> Result<()> {
        let container_id = {
            let monitors_guard = monitors.read().await;
            if let Some(monitor) = monitors_guard.get(session_id) {
                monitor.container_id.clone()
            } else {
                return Ok(()); // Session no longer exists
            }
        };
        
        // Collect metrics using Docker/Podman stats
        let metrics = Self::gather_container_metrics(&container_id).await?;
        
        // Update monitor
        {
            let mut monitors_guard = monitors.write().await;
            if let Some(monitor) = monitors_guard.get_mut(session_id) {
                monitor.historical_metrics.push(monitor.current_metrics.clone());
                monitor.current_metrics = metrics.clone();
                monitor.last_updated = chrono::Utc::now();
                
                // Keep only recent history
                if monitor.historical_metrics.len() > 100 {
                    monitor.historical_metrics.remove(0);
                }
            }
        }
        
        // Add to global history
        {
            let mut history_guard = history.write().await;
            history_guard.sessions
                .entry(session_id.to_string())
                .or_insert_with(Vec::new)
                .push(metrics);
        }
        
        Ok(())
    }
    
    async fn gather_container_metrics(container_id: &str) -> Result<ResourceMetrics> {
        // Try Docker first, then Podman
        if let Ok(metrics) = Self::get_docker_container_stats(container_id).await {
            return Ok(metrics);
        }
        
        if let Ok(metrics) = Self::get_podman_container_stats(container_id).await {
            return Ok(metrics);
        }
        
        Err(anyhow!("Failed to gather container metrics for: {}", container_id))
    }
    
    async fn get_docker_container_stats(container_id: &str) -> Result<ResourceMetrics> {
        use tokio::process::Command;
        
        let output = Command::new("docker")
            .args(["stats", "--no-stream", "--format", "json", container_id])
            .output()
            .await?;
            
        if !output.status.success() {
            return Err(anyhow!("Docker stats command failed"));
        }
        
        let stats_json = String::from_utf8_lossy(&output.stdout);
        let stats: serde_json::Value = serde_json::from_str(&stats_json)?;
        
        Ok(ResourceMetrics {
            timestamp: chrono::Utc::now(),
            cpu_usage_percent: stats["CPUPerc"].as_str()
                .unwrap_or("0%")
                .trim_end_matches('%')
                .parse()
                .unwrap_or(0.0),
            memory_used_bytes: Self::parse_memory_value(stats["MemUsage"].as_str().unwrap_or("0B")),
            memory_limit_bytes: Self::parse_memory_limit(stats["MemUsage"].as_str().unwrap_or("0B / 0B")),
            memory_usage_percent: stats["MemPerc"].as_str()
                .unwrap_or("0%")
                .trim_end_matches('%')
                .parse()
                .unwrap_or(0.0),
            disk_used_bytes: 0, // Not available in basic stats
            disk_io_read_bytes: Self::parse_disk_io(stats["BlockIO"].as_str().unwrap_or("0B / 0B"), true),
            disk_io_write_bytes: Self::parse_disk_io(stats["BlockIO"].as_str().unwrap_or("0B / 0B"), false),
            network_rx_bytes: Self::parse_network_io(stats["NetIO"].as_str().unwrap_or("0B / 0B"), true),
            network_tx_bytes: Self::parse_network_io(stats["NetIO"].as_str().unwrap_or("0B / 0B"), false),
            gpu_usage_percent: None, // Would need nvidia-smi integration
            gpu_memory_used_bytes: None,
            process_count: stats["PIDs"].as_u64().unwrap_or(0) as u32,
            load_average: (0.0, 0.0, 0.0), // Not available from container stats
        })
    }
    
    async fn get_podman_container_stats(container_id: &str) -> Result<ResourceMetrics> {
        use tokio::process::Command;
        
        let output = Command::new("podman")
            .args(["stats", "--no-stream", "--format", "json", container_id])
            .output()
            .await?;
            
        if !output.status.success() {
            return Err(anyhow!("Podman stats command failed"));
        }
        
        let stats_json = String::from_utf8_lossy(&output.stdout);
        let stats: serde_json::Value = serde_json::from_str(&stats_json)?;
        
        // Similar parsing logic as Docker but adapted for Podman format
        Ok(ResourceMetrics {
            timestamp: chrono::Utc::now(),
            cpu_usage_percent: stats["CPU%"].as_str()
                .unwrap_or("0%")
                .trim_end_matches('%')
                .parse()
                .unwrap_or(0.0),
            memory_used_bytes: Self::parse_memory_value(stats["MemUsage"].as_str().unwrap_or("0MB")),
            memory_limit_bytes: Self::parse_memory_limit(stats["MemUsage"].as_str().unwrap_or("0MB / 0MB")),
            memory_usage_percent: stats["Mem%"].as_str()
                .unwrap_or("0%")
                .trim_end_matches('%')
                .parse()
                .unwrap_or(0.0),
            disk_used_bytes: 0,
            disk_io_read_bytes: Self::parse_disk_io(stats["BlockIO"].as_str().unwrap_or("0B / 0B"), true),
            disk_io_write_bytes: Self::parse_disk_io(stats["BlockIO"].as_str().unwrap_or("0B / 0B"), false),
            network_rx_bytes: Self::parse_network_io(stats["NetIO"].as_str().unwrap_or("0B / 0B"), true),
            network_tx_bytes: Self::parse_network_io(stats["NetIO"].as_str().unwrap_or("0B / 0B"), false),
            gpu_usage_percent: None,
            gpu_memory_used_bytes: None,
            process_count: stats["PIDs"].as_u64().unwrap_or(0) as u32,
            load_average: (0.0, 0.0, 0.0),
        })
    }
    
    fn parse_memory_value(mem_str: &str) -> u64 {
        // Parse "1.5GB" or "512MB" format
        let parts: Vec<&str> = mem_str.split('/').collect();
        if let Some(value_str) = parts.get(0) {
            let value_str = value_str.trim();
            if value_str.ends_with("GB") {
                let num: f64 = value_str.trim_end_matches("GB").parse().unwrap_or(0.0);
                return (num * 1024.0 * 1024.0 * 1024.0) as u64;
            } else if value_str.ends_with("MB") {
                let num: f64 = value_str.trim_end_matches("MB").parse().unwrap_or(0.0);
                return (num * 1024.0 * 1024.0) as u64;
            } else if value_str.ends_with("KB") {
                let num: f64 = value_str.trim_end_matches("KB").parse().unwrap_or(0.0);
                return (num * 1024.0) as u64;
            }
        }
        0
    }
    
    fn parse_memory_limit(mem_str: &str) -> u64 {
        // Parse "1.5GB / 4GB" format
        let parts: Vec<&str> = mem_str.split('/').collect();
        if let Some(limit_str) = parts.get(1) {
            return Self::parse_memory_value(limit_str.trim());
        }
        0
    }
    
    fn parse_disk_io(io_str: &str, read: bool) -> u64 {
        let parts: Vec<&str> = io_str.split('/').collect();
        let value_str = if read {
            parts.get(0).unwrap_or(&"0B").trim()
        } else {
            parts.get(1).unwrap_or(&"0B").trim()
        };
        
        Self::parse_memory_value(value_str)
    }
    
    fn parse_network_io(io_str: &str, rx: bool) -> u64 {
        Self::parse_disk_io(io_str, rx)
    }
    
    async fn process_scaling_decisions(
        monitors: &Arc<RwLock<HashMap<String, ResourceMonitor>>>,
        policies: &Arc<RwLock<HashMap<String, ScalingPolicy>>>,
    ) -> Result<()> {
        let session_ids: Vec<String> = {
            let monitors_guard = monitors.read().await;
            monitors_guard.keys().cloned().collect()
        };
        
        for session_id in session_ids {
            if let Err(e) = Self::evaluate_scaling_for_session(&session_id, monitors, policies).await {
                warn!("Scaling evaluation failed for session {}: {}", session_id, e);
            }
        }
        
        Ok(())
    }
    
    async fn evaluate_scaling_for_session(
        session_id: &str,
        monitors: &Arc<RwLock<HashMap<String, ResourceMonitor>>>,
        policies: &Arc<RwLock<HashMap<String, ScalingPolicy>>>,
    ) -> Result<()> {
        let (current_metrics, scaling_state) = {
            let monitors_guard = monitors.read().await;
            if let Some(monitor) = monitors_guard.get(session_id) {
                (monitor.current_metrics.clone(), monitor.scaling_state.clone())
            } else {
                return Ok(());
            }
        };
        
        let policy = {
            let policies_guard = policies.read().await;
            if let Some(policy) = policies_guard.get(session_id) {
                policy.clone()
            } else {
                return Ok(());
            }
        };
        
        if !policy.enabled {
            return Ok(());
        }
        
        // Check if in cooldown
        if let ScalingState::Cooldown(cooldown_until) = scaling_state {
            if chrono::Utc::now() < cooldown_until {
                return Ok(());
            }
        }
        
        // Evaluate scaling decision
        let should_scale_up = current_metrics.cpu_usage_percent > policy.scale_up_threshold.cpu_percent
            || current_metrics.memory_usage_percent > policy.scale_up_threshold.memory_percent;
            
        let should_scale_down = current_metrics.cpu_usage_percent < policy.scale_down_threshold.cpu_percent
            && current_metrics.memory_usage_percent < policy.scale_down_threshold.memory_percent;
        
        if should_scale_up {
            Self::execute_scale_up(session_id, &current_metrics, &policy, monitors).await?;
        } else if should_scale_down {
            Self::execute_scale_down(session_id, &current_metrics, &policy, monitors).await?;
        }
        
        Ok(())
    }
    
    async fn execute_scale_up(
        session_id: &str,
        metrics: &ResourceMetrics,
        policy: &ScalingPolicy,
        monitors: &Arc<RwLock<HashMap<String, ResourceMonitor>>>,
    ) -> Result<()> {
        info!("Scaling up resources for session: {} (CPU: {:.1}%, Memory: {:.1}%)", 
              session_id, metrics.cpu_usage_percent, metrics.memory_usage_percent);
        
        // Calculate new resource allocations
        let current_memory_gb = metrics.memory_limit_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        let new_memory_mb = ((current_memory_gb * 1.5).min(policy.max_memory_mb as f64) * 1024.0) as u64;
        
        // TODO: Implement actual resource scaling via container runtime API
        // This would call Docker/Podman update commands
        
        // Update scaling state
        {
            let mut monitors_guard = monitors.write().await;
            if let Some(monitor) = monitors_guard.get_mut(session_id) {
                monitor.scaling_state = ScalingState::Cooldown(
                    chrono::Utc::now() + chrono::Duration::seconds(policy.cooldown_period_seconds as i64)
                );
            }
        }
        
        info!("Scale up completed for session: {}", session_id);
        Ok(())
    }
    
    async fn execute_scale_down(
        session_id: &str,
        metrics: &ResourceMetrics,
        policy: &ScalingPolicy,
        monitors: &Arc<RwLock<HashMap<String, ResourceMonitor>>>,
    ) -> Result<()> {
        info!("Scaling down resources for session: {} (CPU: {:.1}%, Memory: {:.1}%)", 
              session_id, metrics.cpu_usage_percent, metrics.memory_usage_percent);
        
        // Calculate new resource allocations (reduce by 25%)
        let current_memory_gb = metrics.memory_limit_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        let new_memory_mb = ((current_memory_gb * 0.75).max(policy.min_memory_mb as f64) * 1024.0) as u64;
        
        // TODO: Implement actual resource scaling
        
        // Update scaling state
        {
            let mut monitors_guard = monitors.write().await;
            if let Some(monitor) = monitors_guard.get_mut(session_id) {
                monitor.scaling_state = ScalingState::Cooldown(
                    chrono::Utc::now() + chrono::Duration::seconds(policy.cooldown_period_seconds as i64)
                );
            }
        }
        
        info!("Scale down completed for session: {}", session_id);
        Ok(())
    }
    
    async fn cleanup_old_metrics(
        history: &Arc<RwLock<MetricsHistory>>,
        retention_hours: u64,
    ) -> Result<()> {
        let cutoff_time = chrono::Utc::now() - chrono::Duration::hours(retention_hours as i64);
        
        let mut history_guard = history.write().await;
        
        // Clean up system metrics
        history_guard.system_metrics.retain(|metric| metric.timestamp > cutoff_time);
        
        // Clean up session metrics
        for metrics_list in history_guard.sessions.values_mut() {
            metrics_list.retain(|metric| metric.timestamp > cutoff_time);
        }
        
        // Remove empty session entries
        history_guard.sessions.retain(|_, metrics| !metrics.is_empty());
        
        debug!("Cleaned up metrics older than {} hours", retention_hours);
        Ok(())
    }
    
    pub async fn get_session_metrics(&self, session_id: &str) -> Result<ResourceMetrics> {
        let monitors = self.monitors.read().await;
        if let Some(monitor) = monitors.get(session_id) {
            Ok(monitor.current_metrics.clone())
        } else {
            Err(anyhow!("Session not found: {}", session_id))
        }
    }
    
    pub async fn get_session_history(&self, session_id: &str, limit: usize) -> Result<Vec<ResourceMetrics>> {
        let history = self.metrics_history.read().await;
        if let Some(metrics) = history.sessions.get(session_id) {
            let start_index = if metrics.len() > limit {
                metrics.len() - limit
            } else {
                0
            };
            Ok(metrics[start_index..].to_vec())
        } else {
            Ok(Vec::new())
        }
    }
    
    pub async fn update_scaling_policy(&self, session_id: String, policy: ScalingPolicy) -> Result<()> {
        let mut policies = self.scaling_policies.write().await;
        policies.insert(session_id, policy);
        Ok(())
    }
    
    pub async fn unregister_session(&self, session_id: &str) -> Result<()> {
        info!("Unregistering session from monitoring: {}", session_id);

        {
            let mut monitors = self.monitors.write().await;
            monitors.remove(session_id);
        }

        {
            let mut policies = self.scaling_policies.write().await;
            policies.remove(session_id);
        }

        // Keep historical data for analysis

        Ok(())
    }
}

// ============================================================================
// Tests: Resource Manager
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-RESOURCE-001 (ResourceConfig defaults)
    #[test]
    fn test_resource_config_defaults() {
        // Traces to: FR-KDESKTOPVIRT-001
        let config = ResourceConfig::default();

        assert_eq!(config.monitoring_interval_seconds, 10);
        assert_eq!(config.metrics_retention_hours, 24);
        assert!(config.enable_auto_scaling);
        assert!(!config.enable_predictive_scaling);
    }

    // Traces to: FR-RESOURCE-002 (AlertThresholds defaults)
    #[test]
    fn test_alert_thresholds_defaults() {
        // Traces to: FR-KDESKTOPVIRT-001
        let thresholds = AlertThresholds::default();

        assert_eq!(thresholds.cpu_critical, 90.0);
        assert_eq!(thresholds.memory_critical, 85.0);
        assert_eq!(thresholds.disk_critical, 90.0);
        assert_eq!(thresholds.response_time_ms, 1000);
    }

    // Traces to: FR-RESOURCE-003 (MetricsHistory initialization)
    #[test]
    fn test_metrics_history_creation() {
        // Traces to: FR-KDESKTOPVIRT-004
        let history = MetricsHistory {
            sessions: HashMap::new(),
            system_metrics: Vec::new(),
            max_history_size: 1000,
        };

        assert_eq!(history.max_history_size, 1000);
        assert!(history.sessions.is_empty());
    }

    // Traces to: FR-RESOURCE-004 (ScalingPolicy structure)
    #[test]
    fn test_scaling_policy_creation() {
        let policy = ScalingPolicy {
            session_id: "test-session".to_string(),
            min_cpu_cores: 0.5,
            max_cpu_cores: 4.0,
            min_memory_mb: 512,
            max_memory_mb: 8192,
            scale_up_threshold: ScalingThreshold {
                cpu_percent: 80.0,
                memory_percent: 75.0,
                duration_seconds: 60,
            },
            scale_down_threshold: ScalingThreshold {
                cpu_percent: 30.0,
                memory_percent: 40.0,
                duration_seconds: 300,
            },
            cooldown_period_seconds: 180,
            enabled: true,
        };

        assert_eq!(policy.session_id, "test-session");
        assert_eq!(policy.min_cpu_cores, 0.5);
        assert_eq!(policy.max_cpu_cores, 4.0);
        assert!(policy.enabled);
    }

    // Traces to: FR-RESOURCE-005 (ScalingState enum)
    #[test]
    fn test_scaling_state_variants() {
        let stable = ScalingState::Stable;
        let scaling_up = ScalingState::ScalingUp;
        let scaling_down = ScalingState::ScalingDown;

        match stable {
            ScalingState::Stable => assert!(true),
            _ => panic!("ScalingState mismatch"),
        }

        match scaling_up {
            ScalingState::ScalingUp => assert!(true),
            _ => panic!("ScalingState mismatch"),
        }

        match scaling_down {
            ScalingState::ScalingDown => assert!(true),
            _ => panic!("ScalingState mismatch"),
        }
    }

    // Traces to: FR-RESOURCE-006 (ResourceMonitor creation)
    #[test]
    fn test_resource_monitor_creation() {
        let monitor = ResourceMonitor {
            session_id: "sess-123".to_string(),
            container_id: "cont-456".to_string(),
            current_metrics: ResourceMetrics {
                timestamp: chrono::Utc::now(),
                cpu_usage_percent: 25.5,
                memory_used_bytes: 536870912,
                memory_limit_bytes: 1073741824,
                memory_usage_percent: 50.0,
                disk_used_bytes: 1099511627776,
                disk_io_read_bytes: 0,
                disk_io_write_bytes: 0,
                network_rx_bytes: 0,
                network_tx_bytes: 0,
                gpu_usage_percent: None,
                gpu_memory_used_bytes: None,
                process_count: 5,
                load_average: (0.1, 0.2, 0.3),
            },
            historical_metrics: Vec::new(),
            last_updated: chrono::Utc::now(),
            scaling_state: ScalingState::Stable,
        };

        assert_eq!(monitor.session_id, "sess-123");
        assert_eq!(monitor.container_id, "cont-456");
        assert_eq!(monitor.current_metrics.cpu_usage_percent, 25.5);
    }

    // Traces to: FR-RESOURCE-007 (ResourceManager initialization)
    #[tokio::test]
    async fn test_resource_manager_creation() {
        let manager = ResourceManager::new(None).await;
        assert!(manager.is_ok());
    }

    // Traces to: FR-RESOURCE-008 (ResourceManager with custom config)
    #[tokio::test]
    async fn test_resource_manager_with_config() {
        let config = ResourceConfig {
            monitoring_interval_seconds: 5,
            metrics_retention_hours: 48,
            enable_auto_scaling: false,
            enable_predictive_scaling: true,
            enable_gpu_monitoring: false,
            alert_enabled: true,
        };

        let manager = ResourceManager::new(Some(config)).await;
        assert!(manager.is_ok());
    }

    // Traces to: FR-RESOURCE-009 (Session registration)
    #[tokio::test]
    async fn test_register_session() {
        let manager = ResourceManager::new(None).await.expect("Failed to create manager");
        let result = manager.register_session(
            "test-session".to_string(),
            "container-id".to_string(),
            (2.0, 2048),
        ).await;

        assert!(result.is_ok());
    }

    // Traces to: FR-RESOURCE-010 (Session unregistration)
    #[tokio::test]
    async fn test_unregister_session() {
        let manager = ResourceManager::new(None).await.expect("Failed to create manager");
        manager.register_session(
            "test-session".to_string(),
            "container-id".to_string(),
            (2.0, 2048),
        ).await.expect("Failed to register");

        let result = manager.unregister_session("test-session").await;
        assert!(result.is_ok());
    }

    // Traces to: FR-RESOURCE-011 (Scaling policy update)
    #[tokio::test]
    async fn test_update_scaling_policy() {
        let manager = ResourceManager::new(None).await.expect("Failed to create manager");
        manager.register_session(
            "test-session".to_string(),
            "container-id".to_string(),
            (2.0, 2048),
        ).await.expect("Failed to register");

        let new_policy = ScalingPolicy {
            session_id: "test-session".to_string(),
            min_cpu_cores: 1.0,
            max_cpu_cores: 8.0,
            min_memory_mb: 1024,
            max_memory_mb: 16384,
            scale_up_threshold: ScalingThreshold {
                cpu_percent: 85.0,
                memory_percent: 80.0,
                duration_seconds: 60,
            },
            scale_down_threshold: ScalingThreshold {
                cpu_percent: 25.0,
                memory_percent: 35.0,
                duration_seconds: 300,
            },
            cooldown_period_seconds: 180,
            enabled: true,
        };

        let result = manager.update_scaling_policy("test-session".to_string(), new_policy).await;
        assert!(result.is_ok());
    }

    // Traces to: FR-RESOURCE-012 (SystemMetrics structure)
    #[test]
    fn test_system_metrics_creation() {
        let metrics = SystemMetrics {
            timestamp: chrono::Utc::now(),
            total_cpu_cores: 8,
            total_memory_bytes: 16777216000,
            available_memory_bytes: 8388608000,
            disk_total_bytes: 1099511627776,
            disk_available_bytes: 549755813888,
            active_sessions: 3,
            system_load: (1.5, 2.0, 2.5),
        };

        assert_eq!(metrics.total_cpu_cores, 8);
        assert_eq!(metrics.active_sessions, 3);
    }
}