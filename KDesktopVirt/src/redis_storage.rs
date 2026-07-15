use anyhow::{anyhow, Result};
use redis::{aio::ConnectionManager, AsyncCommands, Client, RedisResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{interval, sleep};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::core::SessionInfo;

/// Redis-based distributed session storage for enterprise deployment
#[derive(Clone)]
pub struct RedisSessionStorage {
    connection_manager: ConnectionManager,
    config: RedisConfig,
    local_cache: Arc<RwLock<LocalCache>>,
    metrics: Arc<RwLock<RedisMetrics>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
    pub connection_timeout: Duration,
    pub command_timeout: Duration,
    pub retry_attempts: u32,
    pub session_ttl: Duration,
    pub cleanup_interval: Duration,
    pub max_sessions_per_user: u32,
    pub cluster_mode: bool,
    pub enable_local_cache: bool,
    pub cache_ttl: Duration,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            pool_size: 20,
            connection_timeout: Duration::from_secs(5),
            command_timeout: Duration::from_secs(3),
            retry_attempts: 3,
            session_ttl: Duration::from_secs(86400), // 24 hours
            cleanup_interval: Duration::from_secs(3600), // 1 hour
            max_sessions_per_user: 10,
            cluster_mode: false,
            enable_local_cache: true,
            cache_ttl: Duration::from_secs(300), // 5 minutes
        }
    }
}

#[derive(Debug, Default)]
struct LocalCache {
    sessions: HashMap<String, CachedSession>,
    session_index: HashMap<String, String>, // name -> session_id
    last_cleanup: std::time::Instant,
}

#[derive(Debug, Clone)]
struct CachedSession {
    session: SessionInfo,
    cached_at: std::time::Instant,
    ttl: Duration,
}

#[derive(Debug, Default, Clone)]
pub struct RedisMetrics {
    pub total_operations: u64,
    pub failed_operations: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub connection_errors: u64,
    pub retry_count: u64,
    pub average_response_time_ms: f64,
    pub active_sessions: u64,
    pub sessions_by_status: HashMap<String, u64>,
}

impl RedisSessionStorage {
    /// Create a new Redis session storage with the given configuration
    pub async fn new(config: RedisConfig) -> Result<Self> {
        info!("Initializing Redis session storage with URL: {}", config.url);
        
        // Create Redis client with connection pooling
        let client = Client::open(config.url.clone())
            .map_err(|e| anyhow!("Failed to create Redis client: {}", e))?;
        
        // Create connection manager for automatic connection pooling and reconnection
        let connection_manager = ConnectionManager::new(client)
            .await
            .map_err(|e| anyhow!("Failed to create Redis connection manager: {}", e))?;
        
        // Test connection
        let mut conn = connection_manager.clone();
        let _: String = conn.ping().await
            .map_err(|e| anyhow!("Failed to ping Redis server: {}", e))?;
        
        info!("Successfully connected to Redis server");
        
        let storage = Self {
            connection_manager,
            config: config.clone(),
            local_cache: Arc::new(RwLock::new(LocalCache {
                sessions: HashMap::new(),
                session_index: HashMap::new(),
                last_cleanup: std::time::Instant::now(),
            })),
            metrics: Arc::new(RwLock::new(RedisMetrics::default())),
        };
        
        // Start background tasks
        storage.start_background_tasks().await;
        
        Ok(storage)
    }
    
    /// Start background maintenance tasks
    async fn start_background_tasks(&self) {
        let storage_clone = self.clone();
        tokio::spawn(async move {
            storage_clone.cleanup_task().await;
        });
        
        let storage_clone = self.clone();
        tokio::spawn(async move {
            storage_clone.cache_maintenance_task().await;
        });
        
        let storage_clone = self.clone();
        tokio::spawn(async move {
            storage_clone.metrics_collection_task().await;
        });
    }
    
    /// Background task for cleaning up expired sessions
    async fn cleanup_task(&self) {
        let mut interval = interval(self.config.cleanup_interval);
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.cleanup_expired_sessions().await {
                error!("Failed to cleanup expired sessions: {}", e);
            }
        }
    }
    
    /// Background task for local cache maintenance
    async fn cache_maintenance_task(&self) {
        let mut interval = interval(Duration::from_secs(60)); // Every minute
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.cleanup_local_cache().await {
                error!("Failed to cleanup local cache: {}", e);
            }
        }
    }
    
    /// Background task for metrics collection
    async fn metrics_collection_task(&self) {
        let mut interval = interval(Duration::from_secs(30)); // Every 30 seconds
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.collect_metrics().await {
                error!("Failed to collect metrics: {}", e);
            }
        }
    }
    
    /// Add a new session to distributed storage
    pub async fn add_session(&self, name: String, session: SessionInfo) -> Result<()> {
        let start_time = std::time::Instant::now();
        
        // Check session limits per user
        if let Err(e) = self.check_session_limits(&name).await {
            self.increment_failed_operations().await;
            return Err(e);
        }
        
        // Serialize session data
        let session_data = serde_json::to_string(&session)
            .map_err(|e| anyhow!("Failed to serialize session: {}", e))?;
        
        // Store in Redis with TTL
        let session_key = format!("kvs:session:{}", name);
        let index_key = format!("kvs:index:{}:{}", name, session.id);
        
        let result = self.execute_with_retry(async {
            let mut conn = self.connection_manager.clone();
            
            // Use Redis pipeline for atomic operations
            let mut pipe = redis::pipe();
            pipe.set_ex(&session_key, &session_data, self.config.session_ttl.as_secs());
            pipe.set_ex(&index_key, &name, self.config.session_ttl.as_secs());
            pipe.sadd("kvs:active_sessions", &name);
            pipe.hincrby("kvs:stats:sessions_by_status", &session.status, 1);
            pipe.incr("kvs:stats:total_sessions", 1);
            
            let _: Vec<RedisResult<()>> = pipe.query_async(&mut conn).await?;
            Ok::<(), redis::RedisError>(())
        }).await;
        
        match result {
            Ok(_) => {
                // Update local cache if enabled
                if self.config.enable_local_cache {
                    self.update_local_cache(name.clone(), session.clone()).await;
                }
                
                self.record_operation_time(start_time).await;
                info!("Successfully added session: {}", name);
                Ok(())
            }
            Err(e) => {
                self.increment_failed_operations().await;
                Err(anyhow!("Failed to add session to Redis: {}", e))
            }
        }
    }
    
    /// Update an existing session
    pub async fn update_session(&self, name: &str, session: SessionInfo) -> Result<()> {
        let start_time = std::time::Instant::now();
        
        // Check if session exists
        if !self.session_exists(name).await {
            self.increment_failed_operations().await;
            return Err(anyhow!("Session '{}' not found", name));
        }
        
        // Serialize session data
        let session_data = serde_json::to_string(&session)
            .map_err(|e| anyhow!("Failed to serialize session: {}", e))?;
        
        let session_key = format!("kvs:session:{}", name);
        
        let result = self.execute_with_retry(async {
            let mut conn = self.connection_manager.clone();
            
            // Update with extended TTL
            let _: () = conn.set_ex(&session_key, &session_data, self.config.session_ttl.as_secs()).await?;
            let _: () = conn.hincrby("kvs:stats:sessions_by_status", &session.status, 1).await?;
            
            Ok::<(), redis::RedisError>(())
        }).await;
        
        match result {
            Ok(_) => {
                // Update local cache
                if self.config.enable_local_cache {
                    self.update_local_cache(name.to_string(), session).await;
                }
                
                self.record_operation_time(start_time).await;
                debug!("Successfully updated session: {}", name);
                Ok(())
            }
            Err(e) => {
                self.increment_failed_operations().await;
                Err(anyhow!("Failed to update session in Redis: {}", e))
            }
        }
    }
    
    /// Get a session by name
    pub async fn get_session(&self, name: &str) -> Option<SessionInfo> {
        let start_time = std::time::Instant::now();
        
        // Check local cache first
        if self.config.enable_local_cache {
            if let Some(cached_session) = self.get_from_local_cache(name).await {
                self.increment_cache_hits().await;
                self.record_operation_time(start_time).await;
                return Some(cached_session);
            }
        }
        
        self.increment_cache_misses().await;
        
        // Fetch from Redis
        let session_key = format!("kvs:session:{}", name);
        
        let result = self.execute_with_retry(async {
            let mut conn = self.connection_manager.clone();
            let session_data: Option<String> = conn.get(&session_key).await?;
            Ok::<Option<String>, redis::RedisError>(session_data)
        }).await;
        
        match result {
            Ok(Some(session_data)) => {
                match serde_json::from_str::<SessionInfo>(&session_data) {
                    Ok(session) => {
                        // Update local cache
                        if self.config.enable_local_cache {
                            self.update_local_cache(name.to_string(), session.clone()).await;
                        }
                        
                        self.record_operation_time(start_time).await;
                        Some(session)
                    }
                    Err(e) => {
                        error!("Failed to deserialize session data: {}", e);
                        self.increment_failed_operations().await;
                        None
                    }
                }
            }
            Ok(None) => {
                self.record_operation_time(start_time).await;
                None
            }
            Err(e) => {
                error!("Failed to get session from Redis: {}", e);
                self.increment_failed_operations().await;
                None
            }
        }
    }
    
    /// Remove a session
    pub async fn remove_session(&self, name: &str) -> Result<Option<SessionInfo>> {
        let start_time = std::time::Instant::now();
        
        // Get session before removing
        let session = self.get_session(name).await;
        
        let session_key = format!("kvs:session:{}", name);
        let index_pattern = format!("kvs:index:{}:*", name);
        
        let result = self.execute_with_retry(async {
            let mut conn = self.connection_manager.clone();
            
            // Use pipeline for atomic removal
            let mut pipe = redis::pipe();
            pipe.del(&session_key);
            pipe.srem("kvs:active_sessions", name);
            pipe.decr("kvs:stats:total_sessions", 1);
            
            // Remove index entries
            let index_keys: Vec<String> = conn.keys(&index_pattern).await?;
            for key in index_keys {
                pipe.del(&key);
            }
            
            let _: Vec<RedisResult<()>> = pipe.query_async(&mut conn).await?;
            Ok::<(), redis::RedisError>(())
        }).await;
        
        match result {
            Ok(_) => {
                // Remove from local cache
                if self.config.enable_local_cache {
                    self.remove_from_local_cache(name).await;
                }
                
                self.record_operation_time(start_time).await;
                info!("Successfully removed session: {}", name);
                Ok(session)
            }
            Err(e) => {
                self.increment_failed_operations().await;
                Err(anyhow!("Failed to remove session from Redis: {}", e))
            }
        }
    }
    
    /// Check if a session exists
    pub async fn session_exists(&self, name: &str) -> bool {
        // Check local cache first
        if self.config.enable_local_cache {
            let cache = self.local_cache.read().await;
            if cache.session_index.contains_key(name) {
                // Verify cache entry is still valid
                if let Some(cached) = cache.sessions.get(name) {
                    if cached.cached_at.elapsed() < cached.ttl {
                        return true;
                    }
                }
            }
        }
        
        // Check Redis
        let session_key = format!("kvs:session:{}", name);
        
        let result = self.execute_with_retry(async {
            let mut conn = self.connection_manager.clone();
            let exists: bool = conn.exists(&session_key).await?;
            Ok::<bool, redis::RedisError>(exists)
        }).await;
        
        result.unwrap_or(false)
    }
    
    /// List all active sessions
    pub async fn list_sessions(&self) -> Vec<SessionInfo> {
        let result = self.execute_with_retry(async {
            let mut conn = self.connection_manager.clone();
            let session_names: Vec<String> = conn.smembers("kvs:active_sessions").await?;
            Ok::<Vec<String>, redis::RedisError>(session_names)
        }).await;
        
        match result {
            Ok(session_names) => {
                let mut sessions = Vec::new();
                for name in session_names {
                    if let Some(session) = self.get_session(&name).await {
                        sessions.push(session);
                    }
                }
                sessions
            }
            Err(e) => {
                error!("Failed to list sessions: {}", e);
                Vec::new()
            }
        }
    }
    
    /// Get session count
    pub async fn session_count(&self) -> usize {
        let result = self.execute_with_retry(async {
            let mut conn = self.connection_manager.clone();
            let count: usize = conn.scard("kvs:active_sessions").await?;
            Ok::<usize, redis::RedisError>(count)
        }).await;
        
        result.unwrap_or(0)
    }
    
    /// Get session by ID (for backward compatibility)
    pub async fn get_session_by_id(&self, session_id: &str) -> Option<SessionInfo> {
        let pattern = format!("kvs:index:*:{}", session_id);
        
        let result = self.execute_with_retry(async {
            let mut conn = self.connection_manager.clone();
            let keys: Vec<String> = conn.keys(&pattern).await?;
            Ok::<Vec<String>, redis::RedisError>(keys)
        }).await;
        
        if let Ok(keys) = result {
            for key in keys {
                if let Ok(Some(name)) = self.execute_with_retry(async {
                    let mut conn = self.connection_manager.clone();
                    let name: Option<String> = conn.get(&key).await?;
                    Ok::<Option<String>, redis::RedisError>(name)
                }).await {
                    return self.get_session(&name).await;
                }
            }
        }
        
        None
    }
    
    /// Get storage metrics
    pub async fn get_metrics(&self) -> RedisMetrics {
        let metrics = self.metrics.read().await;
        metrics.clone()
    }
    
    /// Execute Redis operation with retry logic
    async fn execute_with_retry<F, T, E>(&self, operation: F) -> Result<T, E>
    where
        F: Fn() -> tokio::task::JoinHandle<Result<T, E>> + Send + Sync,
        T: Send + 'static,
        E: Send + std::fmt::Debug + 'static,
    {
        let mut attempts = 0;
        
        while attempts < self.config.retry_attempts {
            match operation().await {
                Ok(result) => return result,
                Err(e) => {
                    attempts += 1;
                    
                    if attempts < self.config.retry_attempts {
                        let delay = Duration::from_millis(100 * (2_u64.pow(attempts - 1)));
                        warn!("Redis operation failed (attempt {}), retrying in {:?}: {:?}", 
                              attempts, delay, e);
                        
                        self.increment_retry_count().await;
                        sleep(delay).await;
                    } else {
                        error!("Redis operation failed after {} attempts: {:?}", attempts, e);
                        self.increment_connection_errors().await;
                        return Err(e);
                    }
                }
            }
        }
        
        unreachable!()
    }
    
    /// Cleanup expired sessions
    async fn cleanup_expired_sessions(&self) -> Result<()> {
        debug!("Starting cleanup of expired sessions");
        
        // Get all active session names
        let session_names = self.execute_with_retry(async {
            let mut conn = self.connection_manager.clone();
            let names: Vec<String> = conn.smembers("kvs:active_sessions").await?;
            Ok::<Vec<String>, redis::RedisError>(names)
        }).await?;
        
        let mut cleaned_count = 0;
        
        for name in session_names {
            let session_key = format!("kvs:session:{}", name);
            
            // Check if session key exists
            let exists = self.execute_with_retry(async {
                let mut conn = self.connection_manager.clone();
                let exists: bool = conn.exists(&session_key).await?;
                Ok::<bool, redis::RedisError>(exists)
            }).await?;
            
            if !exists {
                // Remove from active sessions set
                let _ = self.execute_with_retry(async {
                    let mut conn = self.connection_manager.clone();
                    let _: () = conn.srem("kvs:active_sessions", &name).await?;
                    Ok::<(), redis::RedisError>(())
                }).await;
                
                cleaned_count += 1;
            }
        }
        
        if cleaned_count > 0 {
            info!("Cleaned up {} expired sessions", cleaned_count);
        }
        
        Ok(())
    }
    
    /// Check session limits for a user
    async fn check_session_limits(&self, _name: &str) -> Result<()> {
        // Implementation for session limits per user
        // This is a placeholder for enterprise features
        Ok(())
    }
    
    /// Update local cache
    async fn update_local_cache(&self, name: String, session: SessionInfo) {
        if !self.config.enable_local_cache {
            return;
        }
        
        let mut cache = self.local_cache.write().await;
        let cached_session = CachedSession {
            session: session.clone(),
            cached_at: std::time::Instant::now(),
            ttl: self.config.cache_ttl,
        };
        
        cache.sessions.insert(name.clone(), cached_session);
        cache.session_index.insert(name, session.id);
    }
    
    /// Get from local cache
    async fn get_from_local_cache(&self, name: &str) -> Option<SessionInfo> {
        let cache = self.local_cache.read().await;
        
        if let Some(cached) = cache.sessions.get(name) {
            if cached.cached_at.elapsed() < cached.ttl {
                return Some(cached.session.clone());
            }
        }
        
        None
    }
    
    /// Remove from local cache
    async fn remove_from_local_cache(&self, name: &str) {
        let mut cache = self.local_cache.write().await;
        cache.sessions.remove(name);
        cache.session_index.remove(name);
    }
    
    /// Cleanup local cache
    async fn cleanup_local_cache(&self) -> Result<()> {
        let mut cache = self.local_cache.write().await;
        let now = std::time::Instant::now();
        
        cache.sessions.retain(|_, cached| now.duration_since(cached.cached_at) < cached.ttl);
        cache.session_index.retain(|name, _| cache.sessions.contains_key(name));
        
        cache.last_cleanup = now;
        Ok(())
    }
    
    /// Collect metrics
    async fn collect_metrics(&self) -> Result<()> {
        let session_count = self.session_count().await;
        
        let mut metrics = self.metrics.write().await;
        metrics.active_sessions = session_count as u64;
        
        // Collect additional metrics from Redis
        if let Ok(stats) = self.execute_with_retry(async {
            let mut conn = self.connection_manager.clone();
            let stats: HashMap<String, u64> = conn.hgetall("kvs:stats:sessions_by_status").await?;
            Ok::<HashMap<String, u64>, redis::RedisError>(stats)
        }).await {
            metrics.sessions_by_status = stats;
        }
        
        Ok(())
    }
    
    // Metrics helper methods
    async fn increment_failed_operations(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.failed_operations += 1;
        metrics.total_operations += 1;
    }
    
    async fn increment_cache_hits(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.cache_hits += 1;
    }
    
    async fn increment_cache_misses(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.cache_misses += 1;
    }
    
    async fn increment_connection_errors(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.connection_errors += 1;
    }
    
    async fn increment_retry_count(&self) {
        let mut metrics = self.metrics.write().await;
        metrics.retry_count += 1;
    }
    
    async fn record_operation_time(&self, start_time: std::time::Instant) {
        let mut metrics = self.metrics.write().await;
        let elapsed_ms = start_time.elapsed().as_millis() as f64;
        
        // Calculate moving average
        let alpha = 0.1; // Smoothing factor
        metrics.average_response_time_ms = 
            metrics.average_response_time_ms * (1.0 - alpha) + elapsed_ms * alpha;
        
        metrics.total_operations += 1;
    }
}

impl Default for RedisSessionStorage {
    fn default() -> Self {
        // This is a placeholder implementation since async constructors aren't supported
        // Use RedisSessionStorage::new() instead
        panic!("Use RedisSessionStorage::new() to create instances")
    }
}