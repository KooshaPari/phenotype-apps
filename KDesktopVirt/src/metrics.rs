use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Prometheus metrics collector for KVirtualStage enterprise deployment
#[derive(Clone)]
pub struct MetricsCollector {
    metrics: Arc<RwLock<ApplicationMetrics>>,
    config: MetricsConfig,
    start_time: Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub collection_interval: Duration,
    pub retention_period: Duration,
    pub export_format: ExportFormat,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    Prometheus,
    Json,
    OpenMetrics,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        let mut labels = HashMap::new();
        labels.insert("service".to_string(), "kvirtualstage".to_string());
        labels.insert("version".to_string(), env!("CARGO_PKG_VERSION").to_string());
        
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(15),
            retention_period: Duration::from_secs(3600), // 1 hour
            export_format: ExportFormat::Prometheus,
            labels,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ApplicationMetrics {
    // System metrics
    pub uptime_seconds: u64,
    pub memory_usage_bytes: u64,
    pub memory_usage_percent: f64,
    pub cpu_usage_percent: f64,
    pub disk_usage_bytes: u64,
    pub disk_usage_percent: f64,
    
    // Application metrics
    pub active_sessions: u64,
    pub total_sessions_created: u64,
    pub total_sessions_destroyed: u64,
    pub failed_session_operations: u64,
    pub avg_session_duration_seconds: f64,
    
    // HTTP metrics
    pub http_requests_total: u64,
    pub http_requests_by_status: HashMap<u16, u64>,
    pub http_request_duration_seconds: HistogramMetric,
    pub http_requests_in_flight: u64,
    
    // Redis metrics
    pub redis_operations_total: u64,
    pub redis_operations_failed: u64,
    pub redis_connection_errors: u64,
    pub redis_cache_hits: u64,
    pub redis_cache_misses: u64,
    pub redis_average_response_time_ms: f64,
    
    // Container metrics
    pub containers_running: u64,
    pub containers_created_total: u64,
    pub containers_destroyed_total: u64,
    pub container_creation_failures: u64,
    
    // Automation metrics
    pub automation_tasks_total: u64,
    pub automation_tasks_successful: u64,
    pub automation_tasks_failed: u64,
    pub automation_avg_execution_time_seconds: f64,
    
    // Security metrics
    pub authentication_attempts: u64,
    pub authentication_failures: u64,
    pub rate_limit_violations: u64,
    pub security_violations: u64,
    
    // Business metrics
    pub concurrent_users: u64,
    pub peak_concurrent_users: u64,
    pub user_sessions_by_type: HashMap<String, u64>,
    
    // Custom metrics
    pub custom_metrics: HashMap<String, MetricValue>,
    
    // Timestamp
    pub last_updated: u64,
}

#[derive(Debug, Clone)]
pub struct HistogramMetric {
    pub buckets: Vec<HistogramBucket>,
    pub count: u64,
    pub sum: f64,
}

#[derive(Debug, Clone)]
pub struct HistogramBucket {
    pub upper_bound: f64,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
    Histogram(Vec<f64>),
    Summary { sum: f64, count: u64 },
}

impl Default for HistogramMetric {
    fn default() -> Self {
        Self {
            buckets: vec![
                HistogramBucket { upper_bound: 0.1, count: 0 },
                HistogramBucket { upper_bound: 0.25, count: 0 },
                HistogramBucket { upper_bound: 0.5, count: 0 },
                HistogramBucket { upper_bound: 1.0, count: 0 },
                HistogramBucket { upper_bound: 2.5, count: 0 },
                HistogramBucket { upper_bound: 5.0, count: 0 },
                HistogramBucket { upper_bound: 10.0, count: 0 },
                HistogramBucket { upper_bound: f64::INFINITY, count: 0 },
            ],
            count: 0,
            sum: 0.0,
        }
    }
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new(config: MetricsConfig) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(ApplicationMetrics::default())),
            config,
            start_time: Instant::now(),
        }
    }
    
    /// Start metrics collection background task
    pub fn start_collection(&self) {
        if !self.config.enabled {
            return;
        }
        
        let metrics_clone = Arc::clone(&self.metrics);
        let interval = self.config.collection_interval;
        let start_time = self.start_time;
        
        tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);
            
            loop {
                interval_timer.tick().await;
                
                if let Err(e) = Self::collect_system_metrics(&metrics_clone, start_time).await {
                    error!("Failed to collect system metrics: {}", e);
                }
            }
        });
    }
    
    /// Collect system-level metrics
    async fn collect_system_metrics(
        metrics: &Arc<RwLock<ApplicationMetrics>>,
        start_time: Instant,
    ) -> Result<()> {
        let mut metrics_guard = metrics.write().await;
        
        // Update uptime
        metrics_guard.uptime_seconds = start_time.elapsed().as_secs();
        
        // Update timestamp
        metrics_guard.last_updated = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // Collect memory usage
        if let Ok(memory_info) = Self::get_memory_usage().await {
            metrics_guard.memory_usage_bytes = memory_info.used;
            metrics_guard.memory_usage_percent = memory_info.percent;
        }
        
        // Collect CPU usage
        if let Ok(cpu_percent) = Self::get_cpu_usage().await {
            metrics_guard.cpu_usage_percent = cpu_percent;
        }
        
        // Collect disk usage
        if let Ok(disk_info) = Self::get_disk_usage().await {
            metrics_guard.disk_usage_bytes = disk_info.used;
            metrics_guard.disk_usage_percent = disk_info.percent;
        }
        
        debug!("Collected system metrics");
        Ok(())
    }
    
    /// Increment counter metric
    pub async fn increment_counter(&self, metric_name: &str, value: u64) {
        let mut metrics = self.metrics.write().await;
        
        match metric_name {
            "http_requests_total" => metrics.http_requests_total += value,
            "redis_operations_total" => metrics.redis_operations_total += value,
            "redis_operations_failed" => metrics.redis_operations_failed += value,
            "redis_connection_errors" => metrics.redis_connection_errors += value,
            "redis_cache_hits" => metrics.redis_cache_hits += value,
            "redis_cache_misses" => metrics.redis_cache_misses += value,
            "containers_created_total" => metrics.containers_created_total += value,
            "containers_destroyed_total" => metrics.containers_destroyed_total += value,
            "container_creation_failures" => metrics.container_creation_failures += value,
            "automation_tasks_total" => metrics.automation_tasks_total += value,
            "automation_tasks_successful" => metrics.automation_tasks_successful += value,
            "automation_tasks_failed" => metrics.automation_tasks_failed += value,
            "authentication_attempts" => metrics.authentication_attempts += value,
            "authentication_failures" => metrics.authentication_failures += value,
            "rate_limit_violations" => metrics.rate_limit_violations += value,
            "security_violations" => metrics.security_violations += value,
            "total_sessions_created" => metrics.total_sessions_created += value,
            "total_sessions_destroyed" => metrics.total_sessions_destroyed += value,
            "failed_session_operations" => metrics.failed_session_operations += value,
            _ => {
                // Custom metric
                let current = metrics.custom_metrics
                    .get(metric_name)
                    .and_then(|v| match v {
                        MetricValue::Counter(c) => Some(*c),
                        _ => None,
                    })
                    .unwrap_or(0);
                metrics.custom_metrics.insert(
                    metric_name.to_string(),
                    MetricValue::Counter(current + value),
                );
            }
        }
    }
    
    /// Set gauge metric
    pub async fn set_gauge(&self, metric_name: &str, value: f64) {
        let mut metrics = self.metrics.write().await;
        
        match metric_name {
            "active_sessions" => metrics.active_sessions = value as u64,
            "containers_running" => metrics.containers_running = value as u64,
            "concurrent_users" => {
                metrics.concurrent_users = value as u64;
                if metrics.concurrent_users > metrics.peak_concurrent_users {
                    metrics.peak_concurrent_users = metrics.concurrent_users;
                }
            }
            "http_requests_in_flight" => metrics.http_requests_in_flight = value as u64,
            "redis_average_response_time_ms" => metrics.redis_average_response_time_ms = value,
            "avg_session_duration_seconds" => metrics.avg_session_duration_seconds = value,
            "automation_avg_execution_time_seconds" => metrics.automation_avg_execution_time_seconds = value,
            _ => {
                // Custom metric
                metrics.custom_metrics.insert(
                    metric_name.to_string(),
                    MetricValue::Gauge(value),
                );
            }
        }
    }
    
    /// Record HTTP request
    pub async fn record_http_request(&self, status_code: u16, duration_seconds: f64) {
        let mut metrics = self.metrics.write().await;
        
        metrics.http_requests_total += 1;
        
        // Track by status code
        *metrics.http_requests_by_status.entry(status_code).or_insert(0) += 1;
        
        // Update histogram
        metrics.http_request_duration_seconds.count += 1;
        metrics.http_request_duration_seconds.sum += duration_seconds;
        
        for bucket in &mut metrics.http_request_duration_seconds.buckets {
            if duration_seconds <= bucket.upper_bound {
                bucket.count += 1;
            }
        }
    }
    
    /// Record session event
    pub async fn record_session_event(&self, event_type: &str, session_type: &str) {
        let mut metrics = self.metrics.write().await;
        
        match event_type {
            "created" => {
                metrics.total_sessions_created += 1;
                metrics.active_sessions += 1;
            }
            "destroyed" => {
                metrics.total_sessions_destroyed += 1;
                metrics.active_sessions = metrics.active_sessions.saturating_sub(1);
            }
            _ => {}
        }
        
        // Track by session type
        if !session_type.is_empty() {
            *metrics.user_sessions_by_type.entry(session_type.to_string()).or_insert(0) += 1;
        }
    }
    
    /// Update Redis metrics from storage
    pub async fn update_redis_metrics(&self, redis_metrics: &crate::redis_storage::RedisMetrics) {
        let mut metrics = self.metrics.write().await;
        
        metrics.redis_operations_total = redis_metrics.total_operations;
        metrics.redis_operations_failed = redis_metrics.failed_operations;
        metrics.redis_connection_errors = redis_metrics.connection_errors;
        metrics.redis_cache_hits = redis_metrics.cache_hits;
        metrics.redis_cache_misses = redis_metrics.cache_misses;
        metrics.redis_average_response_time_ms = redis_metrics.average_response_time_ms;
        metrics.active_sessions = redis_metrics.active_sessions;
    }
    
    /// Export metrics in Prometheus format
    pub async fn export_prometheus(&self) -> String {
        let metrics = self.metrics.read().await;
        let mut output = String::new();
        
        // Add metadata
        output.push_str("# HELP kvirtualstage_info Information about KVirtualStage instance
");
        output.push_str("# TYPE kvirtualstage_info gauge
");
        output.push_str(&format!(
            "kvirtualstage_info{{version=\"{}\",instance=\"{}\"}} 1
",
            env!("CARGO_PKG_VERSION"),
            hostname::get().unwrap_or_default().to_string_lossy()
        ));
        
        // System metrics
        output.push_str("# HELP kvirtualstage_uptime_seconds Total uptime in seconds
");
        output.push_str("# TYPE kvirtualstage_uptime_seconds counter
");
        output.push_str(&format!("kvirtualstage_uptime_seconds {}
", metrics.uptime_seconds));
        
        output.push_str("# HELP kvirtualstage_memory_usage_bytes Memory usage in bytes
");
        output.push_str("# TYPE kvirtualstage_memory_usage_bytes gauge
");
        output.push_str(&format!("kvirtualstage_memory_usage_bytes {}
", metrics.memory_usage_bytes));
        
        output.push_str("# HELP kvirtualstage_memory_usage_percent Memory usage percentage
");
        output.push_str("# TYPE kvirtualstage_memory_usage_percent gauge
");
        output.push_str(&format!("kvirtualstage_memory_usage_percent {}
", metrics.memory_usage_percent));
        
        output.push_str("# HELP kvirtualstage_cpu_usage_percent CPU usage percentage
");
        output.push_str("# TYPE kvirtualstage_cpu_usage_percent gauge
");
        output.push_str(&format!("kvirtualstage_cpu_usage_percent {}
", metrics.cpu_usage_percent));
        
        // Session metrics
        output.push_str("# HELP kvirtualstage_active_sessions Number of active sessions
");
        output.push_str("# TYPE kvirtualstage_active_sessions gauge
");
        output.push_str(&format!("kvirtualstage_active_sessions {}
", metrics.active_sessions));
        
        output.push_str("# HELP kvirtualstage_sessions_created_total Total sessions created
");
        output.push_str("# TYPE kvirtualstage_sessions_created_total counter
");
        output.push_str(&format!("kvirtualstage_sessions_created_total {}
", metrics.total_sessions_created));
        
        // HTTP metrics
        output.push_str("# HELP kvirtualstage_http_requests_total Total HTTP requests
");
        output.push_str("# TYPE kvirtualstage_http_requests_total counter
");
        output.push_str(&format!("kvirtualstage_http_requests_total {}
", metrics.http_requests_total));
        
        for (status, count) in &metrics.http_requests_by_status {
            output.push_str(&format!(
                "kvirtualstage_http_requests_total{{status=\"{}\"}} {}
",
                status, count
            ));
        }
        
        output.push_str("# HELP kvirtualstage_http_request_duration_seconds HTTP request duration
");
        output.push_str("# TYPE kvirtualstage_http_request_duration_seconds histogram
");
        
        for bucket in &metrics.http_request_duration_seconds.buckets {
            let le = if bucket.upper_bound == f64::INFINITY {
                "+Inf".to_string()
            } else {
                bucket.upper_bound.to_string()
            };
            output.push_str(&format!(
                "kvirtualstage_http_request_duration_seconds_bucket{{le=\"{}\"}} {}
",
                le, bucket.count
            ));
        }
        
        output.push_str(&format!(
            "kvirtualstage_http_request_duration_seconds_count {}
",
            metrics.http_request_duration_seconds.count
        ));
        output.push_str(&format!(
            "kvirtualstage_http_request_duration_seconds_sum {}
",
            metrics.http_request_duration_seconds.sum
        ));
        
        // Redis metrics
        output.push_str("# HELP kvirtualstage_redis_operations_total Total Redis operations
");
        output.push_str("# TYPE kvirtualstage_redis_operations_total counter
");
        output.push_str(&format!("kvirtualstage_redis_operations_total {}
", metrics.redis_operations_total));
        
        output.push_str("# HELP kvirtualstage_redis_operations_failed_total Failed Redis operations
");
        output.push_str("# TYPE kvirtualstage_redis_operations_failed_total counter
");
        output.push_str(&format!("kvirtualstage_redis_operations_failed_total {}
", metrics.redis_operations_failed));
        
        output.push_str("# HELP kvirtualstage_redis_cache_hits_total Redis cache hits
");
        output.push_str("# TYPE kvirtualstage_redis_cache_hits_total counter
");
        output.push_str(&format!("kvirtualstage_redis_cache_hits_total {}
", metrics.redis_cache_hits));
        
        output.push_str("# HELP kvirtualstage_redis_cache_misses_total Redis cache misses
");
        output.push_str("# TYPE kvirtualstage_redis_cache_misses_total counter
");
        output.push_str(&format!("kvirtualstage_redis_cache_misses_total {}
", metrics.redis_cache_misses));
        
        // Container metrics
        output.push_str("# HELP kvirtualstage_containers_running Number of running containers
");
        output.push_str("# TYPE kvirtualstage_containers_running gauge
");
        output.push_str(&format!("kvirtualstage_containers_running {}
", metrics.containers_running));
        
        // Security metrics
        output.push_str("# HELP kvirtualstage_authentication_failures_total Authentication failures
");
        output.push_str("# TYPE kvirtualstage_authentication_failures_total counter
");
        output.push_str(&format!("kvirtualstage_authentication_failures_total {}
", metrics.authentication_failures));
        
        // Custom metrics
        for (name, value) in &metrics.custom_metrics {
            match value {
                MetricValue::Counter(v) => {
                    output.push_str(&format!("# TYPE kvirtualstage_{} counter
", name));
                    output.push_str(&format!("kvirtualstage_{} {}
", name, v));
                }
                MetricValue::Gauge(v) => {
                    output.push_str(&format!("# TYPE kvirtualstage_{} gauge
", name));
                    output.push_str(&format!("kvirtualstage_{} {}
", name, v));
                }
                _ => {} // Skip complex types for now
            }
        }
        
        output
    }
    
    /// Export metrics in JSON format
    pub async fn export_json(&self) -> Result<String> {
        let metrics = self.metrics.read().await;
        let json = serde_json::to_string_pretty(&*metrics)?;
        Ok(json)
    }
    
    /// Get current metrics snapshot
    pub async fn get_metrics(&self) -> ApplicationMetrics {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }
    
    /// Create metrics HTTP router
    pub fn create_router(self) -> Router {
        Router::new()
            .route("/metrics", get(metrics_handler))
            .route("/metrics/json", get(json_metrics_handler))
            .route("/health", get(health_handler))
            .with_state(self)
    }
    
    // System metrics collection methods
    async fn get_memory_usage() -> Result<MemoryInfo> {
        // Placeholder implementation - in real deployment, use system APIs
        Ok(MemoryInfo {
            total: 8 * 1024 * 1024 * 1024, // 8GB
            used: 4 * 1024 * 1024 * 1024,  // 4GB
            percent: 50.0,
        })
    }
    
    async fn get_cpu_usage() -> Result<f64> {
        // Placeholder implementation - in real deployment, use system APIs
        Ok(25.0) // 25% CPU usage
    }
    
    async fn get_disk_usage() -> Result<DiskInfo> {
        // Placeholder implementation - in real deployment, use system APIs
        Ok(DiskInfo {
            total: 100 * 1024 * 1024 * 1024, // 100GB
            used: 50 * 1024 * 1024 * 1024,   // 50GB
            percent: 50.0,
        })
    }
}

#[derive(Debug)]
struct MemoryInfo {
    total: u64,
    used: u64,
    percent: f64,
}

#[derive(Debug)]
struct DiskInfo {
    total: u64,
    used: u64,
    percent: f64,
}

// HTTP handlers for metrics endpoints
async fn metrics_handler(State(collector): State<MetricsCollector>) -> Response {
    let prometheus_output = collector.export_prometheus().await;
    
    (
        StatusCode::OK,
        [("Content-Type", "text/plain; version=0.0.4")],
        prometheus_output,
    )
        .into_response()
}

async fn json_metrics_handler(State(collector): State<MetricsCollector>) -> Response {
    match collector.export_json().await {
        Ok(json_output) => (
            StatusCode::OK,
            [("Content-Type", "application/json")],
            json_output,
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [("Content-Type", "application/json")],
            format!(r#"{{"error": "Failed to export metrics: {}"}}"#, e),
        )
            .into_response(),
    }
}

async fn health_handler() -> Response {
    (
        StatusCode::OK,
        [("Content-Type", "application/json")],
        r#"{"status": "healthy", "timestamp": ""}"#,
    )
        .into_response()
}

/// Structured logging for enterprise SIEM integration
#[derive(Debug, Clone, Serialize)]
pub struct StructuredLogEvent {
    pub timestamp: String,
    pub level: String,
    pub message: String,
    pub service: String,
    pub version: String,
    pub instance_id: String,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
    pub user_id: Option<String>,
    pub session_id: Option<String>,
    pub request_id: Option<String>,
    pub source_ip: Option<String>,
    pub user_agent: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub status_code: Option<u16>,
    pub duration_ms: Option<u64>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub custom_fields: HashMap<String, serde_json::Value>,
}

impl StructuredLogEvent {
    pub fn new(level: &str, message: &str) -> Self {
        Self {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: level.to_string(),
            message: message.to_string(),
            service: "kvirtualstage".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            instance_id: hostname::get()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            trace_id: None,
            span_id: None,
            user_id: None,
            session_id: None,
            request_id: None,
            source_ip: None,
            user_agent: None,
            method: None,
            path: None,
            status_code: None,
            duration_ms: None,
            error_code: None,
            error_message: None,
            custom_fields: HashMap::new(),
        }
    }
    
    pub fn with_trace(mut self, trace_id: &str, span_id: &str) -> Self {
        self.trace_id = Some(trace_id.to_string());
        self.span_id = Some(span_id.to_string());
        self
    }
    
    pub fn with_user(mut self, user_id: &str) -> Self {
        self.user_id = Some(user_id.to_string());
        self
    }
    
    pub fn with_session(mut self, session_id: &str) -> Self {
        self.session_id = Some(session_id.to_string());
        self
    }
    
    pub fn with_request(mut self, request_id: &str, method: &str, path: &str) -> Self {
        self.request_id = Some(request_id.to_string());
        self.method = Some(method.to_string());
        self.path = Some(path.to_string());
        self
    }
    
    pub fn with_client(mut self, source_ip: &str, user_agent: &str) -> Self {
        self.source_ip = Some(source_ip.to_string());
        self.user_agent = Some(user_agent.to_string());
        self
    }
    
    pub fn with_response(mut self, status_code: u16, duration_ms: u64) -> Self {
        self.status_code = Some(status_code);
        self.duration_ms = Some(duration_ms);
        self
    }
    
    pub fn with_error(mut self, error_code: &str, error_message: &str) -> Self {
        self.error_code = Some(error_code.to_string());
        self.error_message = Some(error_message.to_string());
        self
    }
    
    pub fn with_custom_field<T: serde::Serialize>(mut self, key: &str, value: T) -> Self {
        if let Ok(json_value) = serde_json::to_value(value) {
            self.custom_fields.insert(key.to_string(), json_value);
        }
        self
    }
    
    pub fn log(&self) {
        let json_str = serde_json::to_string(self).unwrap_or_else(|_| {
            format!(r#"{{"error": "Failed to serialize log event", "message": "{}"}}"#, self.message)
        });
        
        match self.level.as_str() {
            "error" => error!(target: "structured_logs", "{}", json_str),
            "warn" => warn!(target: "structured_logs", "{}", json_str),
            "info" => info!(target: "structured_logs", "{}", json_str),
            "debug" => debug!(target: "structured_logs", "{}", json_str),
            _ => info!(target: "structured_logs", "{}", json_str),
        }
    }
}