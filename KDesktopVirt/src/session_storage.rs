use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::sync::{Mutex, RwLock};
use tokio::time;
use tracing::{error, info, warn};

use crate::core::SessionInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionData {
    pub sessions: HashMap<String, SessionInfo>,
    pub session_index: HashMap<String, String>, // name -> session_id for fast lookup
    pub last_updated: String,
}

pub struct SessionStorage {
    data: Arc<RwLock<SessionData>>,
    batch_writer: Arc<Mutex<BatchWriter>>,
    dirty: Arc<Mutex<bool>>,
    connection_pool: Arc<ConnectionPool>,
}

impl std::fmt::Debug for SessionStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionStorage").finish()
    }
}

#[derive(Debug)]
struct BatchWriter {
    pending_writes: Vec<WriteOperation>,
    last_write: std::time::Instant,
    write_interval: Duration,
}

#[derive(Debug, Clone)]
enum WriteOperation {
    AddSession { name: String, session: SessionInfo },
    UpdateSession { name: String, session: SessionInfo },
    RemoveSession { name: String },
    Cleanup,
}

impl BatchWriter {
    fn new() -> Self {
        Self {
            pending_writes: Vec::new(),
            last_write: std::time::Instant::now(),
            write_interval: Duration::from_secs(5), // Batch writes every 5 seconds
        }
    }

    fn add_operation(&mut self, operation: WriteOperation) {
        self.pending_writes.push(operation);
    }

    fn should_flush(&self) -> bool {
        !self.pending_writes.is_empty() && 
        (self.last_write.elapsed() >= self.write_interval || self.pending_writes.len() >= 10)
    }

    fn drain_operations(&mut self) -> Vec<WriteOperation> {
        self.last_write = std::time::Instant::now();
        std::mem::take(&mut self.pending_writes)
    }
}

// Connection pooling for database operations
pub struct ConnectionPool {
    pool: Arc<Mutex<Vec<DatabaseConnection>>>,
    max_connections: usize,
    active_connections: Arc<Mutex<usize>>,
}

#[derive(Debug)]
struct DatabaseConnection {
    id: uuid::Uuid,
    created_at: std::time::Instant,
    last_used: std::time::Instant,
}

impl ConnectionPool {
    pub fn new(max_connections: usize) -> Self {
        Self {
            pool: Arc::new(Mutex::new(Vec::new())),
            max_connections,
            active_connections: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn acquire_connection(&self) -> Result<DatabaseConnection> {
        let mut pool = self.pool.lock().await;
        
        // Try to reuse existing connection
        if let Some(mut conn) = pool.pop() {
            conn.last_used = std::time::Instant::now();
            return Ok(conn);
        }
        
        // Create new connection if under limit
        let mut active = self.active_connections.lock().await;
        if *active < self.max_connections {
            *active += 1;
            Ok(DatabaseConnection {
                id: uuid::Uuid::new_v4(),
                created_at: std::time::Instant::now(),
                last_used: std::time::Instant::now(),
            })
        } else {
            Err(anyhow!("Connection pool exhausted"))
        }
    }

    pub async fn release_connection(&self, connection: DatabaseConnection) {
        let mut pool = self.pool.lock().await;
        
        // Only keep connection if pool isn't full and connection is recent
        if pool.len() < self.max_connections / 2 && 
           connection.created_at.elapsed() < Duration::from_secs(300) {
            pool.push(connection);
        } else {
            // Drop connection and decrement active count
            let mut active = self.active_connections.lock().await;
            *active = active.saturating_sub(1);
        }
    }

    pub async fn cleanup_stale_connections(&self) {
        let mut pool = self.pool.lock().await;
        let stale_threshold = Duration::from_secs(600); // 10 minutes
        
        pool.retain(|conn| conn.last_used.elapsed() < stale_threshold);
    }
}

impl SessionStorage {
    pub fn new() -> Self {
        let data = Arc::new(RwLock::new(SessionData {
            sessions: HashMap::new(),
            session_index: HashMap::new(),
            last_updated: chrono::Utc::now().to_rfc3339(),
        }));
        
        let batch_writer = Arc::new(Mutex::new(BatchWriter::new()));
        let dirty = Arc::new(Mutex::new(false));
        let connection_pool = Arc::new(ConnectionPool::new(10)); // Max 10 connections
        
        let storage = Self {
            data,
            batch_writer,
            dirty,
            connection_pool,
        };
        
        // Start background batch writer task
        storage.start_batch_writer();
        
        storage
    }

    fn start_batch_writer(&self) {
        let data = Arc::clone(&self.data);
        let batch_writer = Arc::clone(&self.batch_writer);
        let dirty = Arc::clone(&self.dirty);
        let connection_pool = Arc::clone(&self.connection_pool);
        
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(2));
            
            loop {
                interval.tick().await;
                
                let should_write = {
                    let writer = batch_writer.lock().await;
                    writer.should_flush()
                };
                
                if should_write {
                    if let Err(e) = Self::flush_batch_operations(&data, &batch_writer, &dirty).await {
                        error!("Failed to flush batch operations: {}", e);
                    }
                }
                
                // Cleanup stale connections periodically
                connection_pool.cleanup_stale_connections().await;
            }
        });
    }

    async fn flush_batch_operations(
        data: &Arc<RwLock<SessionData>>,
        batch_writer: &Arc<Mutex<BatchWriter>>,
        dirty: &Arc<Mutex<bool>>,
    ) -> Result<()> {
        let operations = {
            let mut writer = batch_writer.lock().await;
            if !writer.should_flush() {
                return Ok(());
            }
            writer.drain_operations()
        };
        
        if operations.is_empty() {
            return Ok(());
        }
        
        // Apply operations to data
        {
            let mut data_guard = data.write().await;
            for operation in operations {
                match operation {
                    WriteOperation::AddSession { name, session } => {
                        let session_id = session.id.clone();
                        data_guard.sessions.insert(name.clone(), session);
                        data_guard.session_index.insert(name, session_id);
                        data_guard.last_updated = chrono::Utc::now().to_rfc3339();
                    },
                    WriteOperation::UpdateSession { name, session } => {
                        if data_guard.sessions.contains_key(&name) {
                            let session_id = session.id.clone();
                            data_guard.sessions.insert(name.clone(), session);
                            data_guard.session_index.insert(name, session_id);
                            data_guard.last_updated = chrono::Utc::now().to_rfc3339();
                        }
                    },
                    WriteOperation::RemoveSession { name } => {
                        data_guard.sessions.remove(&name);
                        data_guard.session_index.remove(&name);
                        data_guard.last_updated = chrono::Utc::now().to_rfc3339();
                    },
                    WriteOperation::Cleanup => {
                        // Cleanup logic would go here
                    },
                }
            }
        }
        
        // Write to disk
        let data_clone = {
            let data_guard = data.read().await;
            data_guard.clone()
        };
        
        Self::write_to_disk(&data_clone).await?;
        
        // Mark as clean
        {
            let mut dirty_guard = dirty.lock().await;
            *dirty_guard = false;
        }
        
        info!("Flushed batch operations to disk");
        Ok(())
    }

    async fn write_to_disk(data: &SessionData) -> Result<()> {
        let storage_path = Self::get_storage_path()?;
        
        // Create directory if it doesn't exist
        if let Some(parent) = storage_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let content = serde_json::to_string_pretty(data)?;
        fs::write(&storage_path, content).await?;
        
        Ok(())
    }

    fn get_storage_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir().ok_or_else(|| anyhow!("Could not find home directory"))?;
        Ok(home_dir.join(".kvirtualstage").join("sessions.json"))
    }

    pub async fn load() -> Result<Self> {
        let storage_path = Self::get_storage_path()?;

        let session_data = if !storage_path.exists() {
            info!("No existing session storage found, creating new one");
            SessionData {
                sessions: HashMap::new(),
                session_index: HashMap::new(),
                last_updated: chrono::Utc::now().to_rfc3339(),
            }
        } else {
            let content = fs::read_to_string(&storage_path).await?;
            let mut data: SessionData = serde_json::from_str::<SessionData>(&content)
                .map_err(|e| anyhow!("Failed to parse session storage: {}", e))?;
            
            // Rebuild index if missing (for backward compatibility)
            if data.session_index.is_empty() && !data.sessions.is_empty() {
                for (name, session) in &data.sessions {
                    data.session_index.insert(name.clone(), session.id.clone());
                }
            }
            
            data
        };

        info!("Loaded {} sessions from storage", session_data.sessions.len());
        
        let data = Arc::new(RwLock::new(session_data));
        let batch_writer = Arc::new(Mutex::new(BatchWriter::new()));
        let dirty = Arc::new(Mutex::new(false));
        let connection_pool = Arc::new(ConnectionPool::new(10));
        
        let storage = Self {
            data,
            batch_writer,
            dirty,
            connection_pool,
        };
        
        // Start background batch writer task
        storage.start_batch_writer();
        
        Ok(storage)
    }

    pub async fn force_save(&self) -> Result<()> {
        // Force immediate write bypassing batch system
        let data = {
            let data_guard = self.data.read().await;
            data_guard.clone()
        };
        
        Self::write_to_disk(&data).await?;
        info!("Force saved {} sessions to storage", data.sessions.len());
        Ok(())
    }

    pub async fn add_session(&self, name: String, session: SessionInfo) -> Result<()> {
        // Add to batch writer instead of immediate save
        {
            let mut writer = self.batch_writer.lock().await;
            writer.add_operation(WriteOperation::AddSession { name: name.clone(), session });
        }
        
        // Mark as dirty
        {
            let mut dirty_guard = self.dirty.lock().await;
            *dirty_guard = true;
        }
        
        Ok(())
    }

    pub async fn update_session(&self, name: &str, session: SessionInfo) -> Result<()> {
        // Check if session exists using index for fast lookup
        {
            let data_guard = self.data.read().await;
            if !data_guard.session_index.contains_key(name) {
                return Err(anyhow!("Session '{}' not found", name));
            }
        }
        
        // Add to batch writer
        {
            let mut writer = self.batch_writer.lock().await;
            writer.add_operation(WriteOperation::UpdateSession { 
                name: name.to_string(), 
                session 
            });
        }
        
        // Mark as dirty
        {
            let mut dirty_guard = self.dirty.lock().await;
            *dirty_guard = true;
        }
        
        Ok(())
    }

    pub async fn remove_session(&self, name: &str) -> Result<Option<SessionInfo>> {
        let session = {
            let data_guard = self.data.read().await;
            data_guard.sessions.get(name).cloned()
        };
        
        if session.is_some() {
            // Add to batch writer
            {
                let mut writer = self.batch_writer.lock().await;
                writer.add_operation(WriteOperation::RemoveSession { 
                    name: name.to_string() 
                });
            }
            
            // Mark as dirty
            {
                let mut dirty_guard = self.dirty.lock().await;
                *dirty_guard = true;
            }
        }
        
        Ok(session)
    }

    // Fast O(1) session lookup using index
    pub async fn get_session(&self, name: &str) -> Option<SessionInfo> {
        let data_guard = self.data.read().await;
        data_guard.sessions.get(name).cloned()
    }

    pub async fn get_session_by_id(&self, session_id: &str) -> Option<SessionInfo> {
        let data_guard = self.data.read().await;
        // Use index to find session name, then get session
        for (name, indexed_id) in &data_guard.session_index {
            if indexed_id == session_id {
                return data_guard.sessions.get(name).cloned();
            }
        }
        None
    }

    pub async fn session_exists(&self, name: &str) -> bool {
        let data_guard = self.data.read().await;
        data_guard.session_index.contains_key(name)
    }

    pub async fn list_sessions(&self) -> Vec<SessionInfo> {
        let data_guard = self.data.read().await;
        data_guard.sessions.values().cloned().collect()
    }

    pub async fn session_count(&self) -> usize {
        let data_guard = self.data.read().await;
        data_guard.sessions.len()
    }

    pub async fn cleanup_stale_sessions(&self) -> Result<()> {
        let stale_sessions = {
            let data_guard = self.data.read().await;
            let mut stale: Vec<String> = Vec::new();
            
            for (name, session) in &data_guard.sessions {
                // Check if container still exists or if session is very old
                if let Some(_container_id) = &session.container_id {
                    // Here we would check if container still exists
                    // For now, we'll mark sessions older than 24 hours as potentially stale
                    if let Ok(created_time) = chrono::DateTime::parse_from_rfc3339(&session.created_at) {
                        let now = chrono::Utc::now();
                        let age = now.signed_duration_since(created_time.with_timezone(&chrono::Utc));

                        if age.num_hours() > 24 {
                            warn!("Session '{}' is older than 24 hours, may be stale", name);
                            stale.push(name.clone());
                        }
                    }
                }
            }
            stale
        };

        // Remove stale sessions
        for name in stale_sessions {
            warn!("Removing stale session: {}", name);
            self.remove_session(&name).await?;
        }

        Ok(())
    }

    pub async fn backup(&self) -> Result<()> {
        let storage_path = Self::get_storage_path()?;
        let backup_path = storage_path.with_extension("backup.json");

        if storage_path.exists() {
            fs::copy(&storage_path, &backup_path).await?;
            info!("Created backup of session storage");
        }

        Ok(())
    }

    pub async fn restore_from_backup(&self) -> Result<()> {
        let storage_path = Self::get_storage_path()?;
        let backup_path = storage_path.with_extension("backup.json");

        if !backup_path.exists() {
            return Err(anyhow!("No backup file found"));
        }

        let content = fs::read_to_string(&backup_path).await?;
        let mut backup_data: SessionData = serde_json::from_str(&content)
            .map_err(|e| anyhow!("Failed to parse backup storage: {}", e))?;

        // Rebuild index if missing
        if backup_data.session_index.is_empty() && !backup_data.sessions.is_empty() {
            for (name, session) in &backup_data.sessions {
                backup_data.session_index.insert(name.clone(), session.id.clone());
            }
        }

        // Update in-memory data
        {
            let mut data_guard = self.data.write().await;
            *data_guard = backup_data;
            data_guard.last_updated = chrono::Utc::now().to_rfc3339();
        }
        
        // Force save to disk
        self.force_save().await?;

        let session_count = {
            let data_guard = self.data.read().await;
            data_guard.sessions.len()
        };
        
        info!("Restored {} sessions from backup", session_count);
        Ok(())
    }

    pub async fn get_connection_pool_stats(&self) -> Result<(usize, usize)> {
        let pool = self.connection_pool.pool.lock().await;
        let active = self.connection_pool.active_connections.lock().await;
        Ok((pool.len(), *active))
    }
}

impl Default for SessionStorage {
    fn default() -> Self {
        Self::new()
    }
}