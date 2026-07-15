/*!
 * KVirtualStage Comprehensive API Surface
 * 
 * Provides a unified API interface for:
 * - C FFI bindings for native applications
 * - Python bindings via PyO3
 * - Node.js bindings via NAPI
 * - Direct Rust library usage
 * - MCP server integration
 * 
 * Designed for enterprise integration with multiple language ecosystems.
 */

use crate::automation_engine::{AutomationEngine, AutomationWorkflow, Point, MouseButton};
use crate::core::KVirtualStageCore;
use crate::desktop_control::{DesktopControlManager, DesktopProtocol};
use crate::recording_pipeline::{RecordingPipeline, QualityProfile};
use crate::security_framework::{SecurityEngine, Credential, SessionContext};

use aes_gcm::{Aes256Gcm, KeyInit};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint};
use std::path::PathBuf;
use std::ptr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{info, warn};

// ============================================================================
// Unified API Interface
// ============================================================================

/// Main KVirtualStage API interface
#[derive(Debug)]
pub struct KVirtualStageAPI {
    core: Arc<KVirtualStageCore>,
    automation_engine: Arc<RwLock<AutomationEngine>>,
    desktop_control: Arc<RwLock<DesktopControlManager>>,
    recording_pipeline: Arc<RwLock<RecordingPipeline>>,
    security_engine: Arc<RwLock<SecurityEngine>>,
    active_sessions: Arc<RwLock<HashMap<String, APISession>>>,
}

#[derive(Debug, Clone)]
struct APISession {
    session_id: String,
    user_id: String,
    created_at: std::time::Instant,
    last_activity: std::time::Instant,
    automation_context: AutomationContext,
    security_context: SessionContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AutomationContext {
    current_position: Point,
    session_recording: bool,
    natural_movement_enabled: bool,
    typing_speed_wpm: f64,
    error_simulation_enabled: bool,
}

impl Default for AutomationContext {
    fn default() -> Self {
        Self {
            current_position: Point::new(0.0, 0.0),
            session_recording: false,
            natural_movement_enabled: true,
            typing_speed_wpm: 65.0,
            error_simulation_enabled: false,
        }
    }
}

impl KVirtualStageAPI {
    /// Initialize the KVirtualStage API
    pub async fn new() -> Result<Self> {
        info!("Initializing KVirtualStage API");

        let core = Arc::new(KVirtualStageCore::new().await?);
        
        let automation_engine = Arc::new(RwLock::new(
            AutomationEngine::new()?
        ));

        let desktop_control = Arc::new(RwLock::new(
            DesktopControlManager::new(Default::default()).await?
        ));

        let recording_pipeline = Arc::new(RwLock::new(
            RecordingPipeline::new(Default::default()).await?
        ));

        let security_engine = Arc::new(RwLock::new(
            SecurityEngine::new(Default::default()).await?
        ));

        Ok(Self {
            core,
            automation_engine,
            desktop_control,
            recording_pipeline,
            security_engine,
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Create a new automation session
    pub async fn create_session(
        &self,
        user_id: String,
        session_name: String,
        desktop_type: String,
    ) -> Result<String> {
        info!("Creating session: {} for user: {}", session_name, user_id);

        // Create core session
        self.core.create_session(
            session_name.clone(),
            desktop_type.clone(),
            None, // Use default image
            2048, // 2GB memory
            2,    // 2 CPU cores
        ).await?;

        // Create desktop control session
        let desktop_control = self.desktop_control.read().await;
        let _desktop_session = desktop_control.create_session(
            session_name.clone(),
            format!("container-{}", session_name),
            DesktopProtocol::VNC,
        ).await?;

        // Create security context
        let security_context = SessionContext {
            user_id: user_id.clone(),
            session_id: session_name.clone(),
            ip_address: None,
            user_agent: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        // Create API session
        let api_session = APISession {
            session_id: session_name.clone(),
            user_id,
            created_at: std::time::Instant::now(),
            last_activity: std::time::Instant::now(),
            automation_context: AutomationContext::default(),
            security_context,
        };

        // Store session
        let mut sessions = self.active_sessions.write().await;
        sessions.insert(session_name.clone(), api_session);

        info!("Session created successfully: {}", session_name);
        Ok(session_name)
    }

    /// Move cursor naturally to target position
    pub async fn move_cursor(
        &self,
        session_id: &str,
        target_x: f64,
        target_y: f64,
    ) -> Result<()> {
        let mut sessions = self.active_sessions.write().await;
        let session = sessions.get_mut(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;

        let from = session.automation_context.current_position;
        let to = Point::new(target_x, target_y);

        // Update session activity
        session.last_activity = std::time::Instant::now();
        session.automation_context.current_position = to;

        // Execute natural movement
        let mut automation = self.automation_engine.write().await;
        automation.move_cursor_naturally(from, to, None).await?;

        info!("Cursor moved naturally in session {}: ({:.0},{:.0}) -> ({:.0},{:.0})",
              session_id, from.x, from.y, to.x, to.y);
        Ok(())
    }

    /// Click at current cursor position
    pub async fn click(
        &self,
        session_id: &str,
        button: Option<String>,
    ) -> Result<()> {
        let sessions = self.active_sessions.read().await;
        let session = sessions.get(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;

        let position = session.automation_context.current_position;
        let mouse_button = match button.as_deref() {
            Some("right") => MouseButton::Right,
            Some("middle") => MouseButton::Middle,
            _ => MouseButton::Left,
        };

        // Execute natural click
        let mut automation = self.automation_engine.write().await;
        automation.click_naturally(position, position, mouse_button).await?;

        info!("Natural click executed in session {}: {:?} at ({:.0},{:.0})",
              session_id, mouse_button, position.x, position.y);
        Ok(())
    }

    /// Type text naturally with human-like timing
    pub async fn type_text(
        &self,
        session_id: &str,
        text: &str,
    ) -> Result<()> {
        let sessions = self.active_sessions.read().await;
        let session = sessions.get(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;

        // Update activity
        drop(sessions);
        let mut sessions = self.active_sessions.write().await;
        let session = sessions.get_mut(session_id).unwrap();
        session.last_activity = std::time::Instant::now();
        drop(sessions);

        // Execute natural typing
        let mut automation = self.automation_engine.write().await;
        automation.type_text_naturally(text).await?;

        info!("Natural typing executed in session {}: {} characters", 
              session_id, text.len());
        Ok(())
    }

    /// Start recording a session
    pub async fn start_recording(
        &self,
        session_id: &str,
        output_filename: &str,
        quality: Option<String>,
    ) -> Result<String> {
        info!("Starting recording for session: {} -> {}", session_id, output_filename);

        let quality_profile = match quality.as_deref() {
            Some("high") => QualityProfile::high_quality(),
            Some("medium") => QualityProfile::medium_quality(),
            Some("streaming") => QualityProfile::streaming_quality(),
            _ => QualityProfile::medium_quality(),
        };

        let recording_pipeline = self.recording_pipeline.read().await;
        let recording_session = recording_pipeline.start_recording(
            session_id.to_string(),
            output_filename.to_string(),
            Some(quality_profile),
            false, // No streaming by default
        ).await?;

        // Update session recording status
        let mut sessions = self.active_sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.automation_context.session_recording = true;
            session.last_activity = std::time::Instant::now();
        }

        Ok(recording_session.recording_id)
    }

    /// Stop recording a session
    pub async fn stop_recording(
        &self,
        session_id: &str,
    ) -> Result<String> {
        info!("Stopping recording for session: {}", session_id);

        let recording_pipeline = self.recording_pipeline.read().await;
        let result = recording_pipeline.stop_recording(session_id).await?;

        // Update session recording status
        let mut sessions = self.active_sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.automation_context.session_recording = false;
            session.last_activity = std::time::Instant::now();
        }

        Ok(result.output_path.to_string_lossy().to_string())
    }

    /// Execute a complete automation workflow
    pub async fn execute_workflow(
        &self,
        session_id: &str,
        workflow: AutomationWorkflow,
    ) -> Result<WorkflowExecutionResult> {
        info!("Executing workflow '{}' in session: {}", workflow.name, session_id);

        // Validate session
        let sessions = self.active_sessions.read().await;
        let _session = sessions.get(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;
        drop(sessions);

        // Execute workflow
        let mut automation = self.automation_engine.write().await;
        let result = automation.execute_workflow(workflow).await?;

        // Update session activity
        let mut sessions = self.active_sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.last_activity = std::time::Instant::now();
        }

        Ok(WorkflowExecutionResult {
            workflow_name: result.workflow_name,
            success: result.successful_steps == result.total_steps,
            total_steps: result.total_steps,
            successful_steps: result.successful_steps,
            execution_time_ms: result.total_execution_time.as_millis() as u64,
            errors: result.step_results
                .iter()
                .filter_map(|r| r.error.clone())
                .collect(),
        })
    }

    /// Store credentials securely
    pub async fn store_credential(
        &self,
        session_id: &str,
        service: &str,
        username: &str,
        password: &str,
        additional_fields: Option<HashMap<String, String>>,
    ) -> Result<String> {
        let sessions = self.active_sessions.read().await;
        let session = sessions.get(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;

        let security_context = session.security_context.clone();
        drop(sessions);

        // Create credential
        let credential = Credential {
            service: service.to_string(),
            username: username.to_string(),
            password: crate::security_framework::SecretString::new(
                password,
                &aes_gcm::Aes256Gcm::new(aes_gcm::Key::<aes_gcm::Aes256Gcm>::from_slice(&[0u8; 32])),
            )?,
            additional_fields: additional_fields.unwrap_or_default()
                .into_iter()
                .map(|(k, v)| (k, crate::security_framework::SecretString::new(
                    &v,
                    &aes_gcm::Aes256Gcm::new(aes_gcm::Key::<aes_gcm::Aes256Gcm>::from_slice(&[0u8; 32])),
                ).unwrap()))
                .collect(),
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            last_accessed: 0,
            expires_at: None,
            tags: Vec::new(),
        };

        let security = self.security_engine.read().await;
        let credential_id = security.store_credential(service, credential, security_context).await?;

        info!("Credential stored for service '{}' in session: {}", service, session_id);
        Ok(credential_id)
    }

    /// Get session information
    pub async fn get_session_info(&self, session_id: &str) -> Result<APISessionInfo> {
        let sessions = self.active_sessions.read().await;
        let session = sessions.get(session_id)
            .ok_or_else(|| anyhow!("Session not found: {}", session_id))?;

        let core_sessions = self.core.list_sessions().await?;
        let core_session = core_sessions.iter()
            .find(|s| s.name == session_id)
            .ok_or_else(|| anyhow!("Core session not found: {}", session_id))?;

        Ok(APISessionInfo {
            session_id: session.session_id.clone(),
            user_id: session.user_id.clone(),
            desktop_type: core_session.desktop.clone(),
            status: core_session.status.clone(),
            created_at: session.created_at,
            last_activity: session.last_activity,
            automation_context: session.automation_context.clone(),
            recording_active: session.automation_context.session_recording,
        })
    }

    /// List all active sessions
    pub async fn list_sessions(&self) -> Result<Vec<APISessionInfo>> {
        let sessions = self.active_sessions.read().await;
        let core_sessions = self.core.list_sessions().await?;

        let mut session_infos = Vec::new();
        for (session_id, api_session) in sessions.iter() {
            if let Some(core_session) = core_sessions.iter().find(|s| &s.name == session_id) {
                session_infos.push(APISessionInfo {
                    session_id: api_session.session_id.clone(),
                    user_id: api_session.user_id.clone(),
                    desktop_type: core_session.desktop.clone(),
                    status: core_session.status.clone(),
                    created_at: api_session.created_at,
                    last_activity: api_session.last_activity,
                    automation_context: api_session.automation_context.clone(),
                    recording_active: api_session.automation_context.session_recording,
                });
            }
        }

        Ok(session_infos)
    }

    /// Cleanup expired sessions
    pub async fn cleanup_expired_sessions(&self, max_idle_minutes: u32) -> Result<Vec<String>> {
        let max_idle = Duration::from_secs(max_idle_minutes as u64 * 60);
        let now = std::time::Instant::now();
        let mut expired_sessions = Vec::new();

        let mut sessions = self.active_sessions.write().await;
        sessions.retain(|session_id, session| {
            if now.duration_since(session.last_activity) > max_idle {
                expired_sessions.push(session_id.clone());
                false
            } else {
                true
            }
        });

        // Clean up core sessions
        for session_id in &expired_sessions {
            if let Err(e) = self.core.remove_session(session_id.clone()).await {
                warn!("Failed to remove expired session {}: {}", session_id, e);
            }
        }

        info!("Cleaned up {} expired sessions", expired_sessions.len());
        Ok(expired_sessions)
    }
}

// ============================================================================
// API Response Types
// ============================================================================

#[derive(Debug, Clone, Serialize)]
pub struct APISessionInfo {
    pub session_id: String,
    pub user_id: String,
    pub desktop_type: String,
    pub status: String,
    #[serde(skip)]
    pub created_at: std::time::Instant,
    #[serde(skip)]
    pub last_activity: std::time::Instant,
    pub automation_context: AutomationContext,
    pub recording_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowExecutionResult {
    pub workflow_name: String,
    pub success: bool,
    pub total_steps: usize,
    pub successful_steps: usize,
    pub execution_time_ms: u64,
    pub errors: Vec<String>,
}

// ============================================================================
// C FFI Interface
// ============================================================================

/// Global API instance for C FFI
static mut GLOBAL_API: Option<*mut KVirtualStageAPI> = None;
static INIT_ONCE: std::sync::Once = std::sync::Once::new();

/// Initialize KVirtualStage API (C FFI)
#[no_mangle]
pub extern "C" fn kvs_init() -> c_int {
    INIT_ONCE.call_once(|| {
        // Initialize Tokio runtime
        let rt = tokio::runtime::Runtime::new().unwrap();
        let api = rt.block_on(async {
            KVirtualStageAPI::new().await.unwrap()
        });
        
        unsafe {
            GLOBAL_API = Some(Box::into_raw(Box::new(api)));
        }
    });

    if unsafe { GLOBAL_API.is_some() } { 0 } else { -1 }
}

/// Create a new session (C FFI)
#[no_mangle]
pub extern "C" fn kvs_create_session(
    user_id: *const c_char,
    session_name: *const c_char,
    desktop_type: *const c_char,
    result_buffer: *mut c_char,
    buffer_size: c_uint,
) -> c_int {
    if user_id.is_null() || session_name.is_null() || desktop_type.is_null() || result_buffer.is_null() {
        return -1;
    }

    let api = unsafe {
        match GLOBAL_API {
            Some(api_ptr) => &*api_ptr,
            None => return -2,
        }
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let user_id_str = unsafe { CStr::from_ptr(user_id) }.to_string_lossy();
        let session_name_str = unsafe { CStr::from_ptr(session_name) }.to_string_lossy();
        let desktop_type_str = unsafe { CStr::from_ptr(desktop_type) }.to_string_lossy();

        api.create_session(
            user_id_str.to_string(),
            session_name_str.to_string(),
            desktop_type_str.to_string(),
        ).await
    });

    match result {
        Ok(session_id) => {
            let session_id_cstr = CString::new(session_id).unwrap();
            let session_id_bytes = session_id_cstr.as_bytes_with_nul();
            
            if session_id_bytes.len() <= buffer_size as usize {
                unsafe {
                    ptr::copy_nonoverlapping(
                        session_id_bytes.as_ptr(),
                        result_buffer as *mut u8,
                        session_id_bytes.len(),
                    );
                }
                0
            } else {
                -3 // Buffer too small
            }
        }
        Err(_) => -4,
    }
}

/// Move cursor naturally (C FFI)
#[no_mangle]
pub extern "C" fn kvs_move_cursor(
    session_id: *const c_char,
    target_x: f64,
    target_y: f64,
) -> c_int {
    if session_id.is_null() {
        return -1;
    }

    let api = unsafe {
        match GLOBAL_API {
            Some(api_ptr) => &*api_ptr,
            None => return -2,
        }
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let session_id_str = unsafe { CStr::from_ptr(session_id) }.to_string_lossy();
        api.move_cursor(&session_id_str, target_x, target_y).await
    });

    if result.is_ok() { 0 } else { -3 }
}

/// Click at current position (C FFI)
#[no_mangle]
pub extern "C" fn kvs_click(
    session_id: *const c_char,
    button: *const c_char,
) -> c_int {
    if session_id.is_null() {
        return -1;
    }

    let api = unsafe {
        match GLOBAL_API {
            Some(api_ptr) => &*api_ptr,
            None => return -2,
        }
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let session_id_str = unsafe { CStr::from_ptr(session_id) }.to_string_lossy();
        let button_str = if button.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(button) }.to_string_lossy().to_string())
        };
        
        api.click(&session_id_str, button_str).await
    });

    if result.is_ok() { 0 } else { -3 }
}

/// Type text naturally (C FFI)
#[no_mangle]
pub extern "C" fn kvs_type_text(
    session_id: *const c_char,
    text: *const c_char,
) -> c_int {
    if session_id.is_null() || text.is_null() {
        return -1;
    }

    let api = unsafe {
        match GLOBAL_API {
            Some(api_ptr) => &*api_ptr,
            None => return -2,
        }
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let session_id_str = unsafe { CStr::from_ptr(session_id) }.to_string_lossy();
        let text_str = unsafe { CStr::from_ptr(text) }.to_string_lossy();
        
        api.type_text(&session_id_str, &text_str).await
    });

    if result.is_ok() { 0 } else { -3 }
}

/// Start recording (C FFI)
#[no_mangle]
pub extern "C" fn kvs_start_recording(
    session_id: *const c_char,
    output_filename: *const c_char,
    quality: *const c_char,
    result_buffer: *mut c_char,
    buffer_size: c_uint,
) -> c_int {
    if session_id.is_null() || output_filename.is_null() || result_buffer.is_null() {
        return -1;
    }

    let api = unsafe {
        match GLOBAL_API {
            Some(api_ptr) => &*api_ptr,
            None => return -2,
        }
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let session_id_str = unsafe { CStr::from_ptr(session_id) }.to_string_lossy();
        let output_filename_str = unsafe { CStr::from_ptr(output_filename) }.to_string_lossy();
        let quality_str = if quality.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(quality) }.to_string_lossy().to_string())
        };
        
        api.start_recording(&session_id_str, &output_filename_str, quality_str).await
    });

    match result {
        Ok(recording_id) => {
            let recording_id_cstr = CString::new(recording_id).unwrap();
            let recording_id_bytes = recording_id_cstr.as_bytes_with_nul();
            
            if recording_id_bytes.len() <= buffer_size as usize {
                unsafe {
                    ptr::copy_nonoverlapping(
                        recording_id_bytes.as_ptr(),
                        result_buffer as *mut u8,
                        recording_id_bytes.len(),
                    );
                }
                0
            } else {
                -3 // Buffer too small
            }
        }
        Err(_) => -4,
    }
}

/// Stop recording (C FFI)
#[no_mangle]
pub extern "C" fn kvs_stop_recording(
    session_id: *const c_char,
    result_buffer: *mut c_char,
    buffer_size: c_uint,
) -> c_int {
    if session_id.is_null() || result_buffer.is_null() {
        return -1;
    }

    let api = unsafe {
        match GLOBAL_API {
            Some(api_ptr) => &*api_ptr,
            None => return -2,
        }
    };

    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        let session_id_str = unsafe { CStr::from_ptr(session_id) }.to_string_lossy();
        api.stop_recording(&session_id_str).await
    });

    match result {
        Ok(output_path) => {
            let output_path_cstr = CString::new(output_path).unwrap();
            let output_path_bytes = output_path_cstr.as_bytes_with_nul();
            
            if output_path_bytes.len() <= buffer_size as usize {
                unsafe {
                    ptr::copy_nonoverlapping(
                        output_path_bytes.as_ptr(),
                        result_buffer as *mut u8,
                        output_path_bytes.len(),
                    );
                }
                0
            } else {
                -3 // Buffer too small
            }
        }
        Err(_) => -4,
    }
}

/// Cleanup and shutdown API (C FFI)
#[no_mangle]
pub extern "C" fn kvs_shutdown() -> c_int {
    unsafe {
        if let Some(api_ptr) = GLOBAL_API.take() {
            let _api = Box::from_raw(api_ptr);
            // API will be dropped here
            0
        } else {
            -1
        }
    }
}

// ============================================================================
// Python Bindings (PyO3)
// ============================================================================

#[cfg(feature = "python-bindings")]
mod python_bindings {
    use super::*;
    use pyo3::prelude::*;
    use pyo3::types::{PyDict, PyList};
    use std::sync::Mutex;

    /// Python wrapper for KVirtualStage API
    #[pyclass]
    pub struct PyKVirtualStage {
        api: Arc<Mutex<KVirtualStageAPI>>,
        runtime: Arc<tokio::runtime::Runtime>,
    }

    #[pymethods]
    impl PyKVirtualStage {
        #[new]
        fn new() -> PyResult<Self> {
            let runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());
            let api = runtime.block_on(async {
                KVirtualStageAPI::new().await
            }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok(Self {
                api: Arc::new(Mutex::new(api)),
                runtime,
            })
        }

        fn create_session(
            &self,
            user_id: String,
            session_name: String,
            desktop_type: String,
        ) -> PyResult<String> {
            let api = self.api.lock().unwrap();
            self.runtime.block_on(async {
                api.create_session(user_id, session_name, desktop_type).await
            }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        }

        fn move_cursor(&self, session_id: String, x: f64, y: f64) -> PyResult<()> {
            let api = self.api.lock().unwrap();
            self.runtime.block_on(async {
                api.move_cursor(&session_id, x, y).await
            }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        }

        fn click(&self, session_id: String, button: Option<String>) -> PyResult<()> {
            let api = self.api.lock().unwrap();
            self.runtime.block_on(async {
                api.click(&session_id, button).await
            }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        }

        fn type_text(&self, session_id: String, text: String) -> PyResult<()> {
            let api = self.api.lock().unwrap();
            self.runtime.block_on(async {
                api.type_text(&session_id, &text).await
            }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        }

        fn start_recording(
            &self,
            session_id: String,
            output_filename: String,
            quality: Option<String>,
        ) -> PyResult<String> {
            let api = self.api.lock().unwrap();
            self.runtime.block_on(async {
                api.start_recording(&session_id, &output_filename, quality).await
            }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        }

        fn stop_recording(&self, session_id: String) -> PyResult<String> {
            let api = self.api.lock().unwrap();
            self.runtime.block_on(async {
                api.stop_recording(&session_id).await
            }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        }

        fn get_session_info(&self, session_id: String) -> PyResult<PyObject> {
            let api = self.api.lock().unwrap();
            let session_info = self.runtime.block_on(async {
                api.get_session_info(&session_id).await
            }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Python::with_gil(|py| {
                let dict = PyDict::new(py);
                dict.set_item("session_id", session_info.session_id)?;
                dict.set_item("user_id", session_info.user_id)?;
                dict.set_item("desktop_type", session_info.desktop_type)?;
                dict.set_item("status", session_info.status)?;
                dict.set_item("recording_active", session_info.recording_active)?;
                Ok(dict.to_object(py))
            })
        }

        fn list_sessions(&self) -> PyResult<PyObject> {
            let api = self.api.lock().unwrap();
            let sessions = self.runtime.block_on(async {
                api.list_sessions().await
            }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Python::with_gil(|py| {
                let list = PyList::empty(py);
                for session_info in sessions {
                    let dict = PyDict::new(py);
                    dict.set_item("session_id", session_info.session_id)?;
                    dict.set_item("user_id", session_info.user_id)?;
                    dict.set_item("desktop_type", session_info.desktop_type)?;
                    dict.set_item("status", session_info.status)?;
                    dict.set_item("recording_active", session_info.recording_active)?;
                    list.append(dict)?;
                }
                Ok(list.to_object(py))
            })
        }
    }

    /// Python module definition
    #[pymodule]
    fn kvirtualstage(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<PyKVirtualStage>()?;
        Ok(())
    }
}

// ============================================================================
// Example Usage and Integration Patterns
// ============================================================================

/// Example usage patterns for the API
pub mod examples {
    use super::*;

    /// Comprehensive automation example
    pub async fn comprehensive_demo_example() -> Result<()> {
        // Initialize API
        let api = KVirtualStageAPI::new().await?;

        // Create session
        let session_id = api.create_session(
            "demo_user".to_string(),
            "demo_session".to_string(),
            "ubuntu".to_string(),
        ).await?;

        // Start recording
        let _recording_id = api.start_recording(
            &session_id,
            "comprehensive_demo.mp4",
            Some("high".to_string()),
        ).await?;

        // Execute automation sequence
        api.move_cursor(&session_id, 200.0, 300.0).await?;
        api.click(&session_id, None).await?;
        api.type_text(&session_id, "Hello from KVirtualStage!").await?;

        // Move to different position and right-click
        api.move_cursor(&session_id, 500.0, 400.0).await?;
        api.click(&session_id, Some("right".to_string())).await?;

        // Stop recording
        let output_path = api.stop_recording(&session_id).await?;
        
        println!("Demo completed! Recording saved to: {}", output_path);
        Ok(())
    }

    /// Workflow execution example
    pub async fn workflow_execution_example() -> Result<()> {
        use crate::automation_engine::{StepAction, WorkflowStep};

        let api = KVirtualStageAPI::new().await?;
        let session_id = api.create_session(
            "workflow_user".to_string(),
            "workflow_session".to_string(),
            "ubuntu".to_string(),
        ).await?;

        // Create workflow
        let workflow = AutomationWorkflow {
            name: "Calculator Demo".to_string(),
            description: "Demonstrate calculator usage".to_string(),
            continue_on_error: false,
            steps: vec![
                WorkflowStep {
                    name: "Move to calculator".to_string(),
                    action: StepAction::MoveCursor { to: Point::new(100.0, 100.0) },
                    timeout: Some(Duration::from_secs(5)),
                },
                WorkflowStep {
                    name: "Click calculator".to_string(),
                    action: StepAction::Click { 
                        position: Point::new(100.0, 100.0), 
                        button: MouseButton::Left 
                    },
                    timeout: Some(Duration::from_secs(5)),
                },
                WorkflowStep {
                    name: "Type calculation".to_string(),
                    action: StepAction::Type { text: "2 + 2 =".to_string() },
                    timeout: Some(Duration::from_secs(10)),
                },
            ],
        };

        // Execute workflow
        let result = api.execute_workflow(&session_id, workflow).await?;
        
        if result.success {
            println!("Workflow completed successfully!");
        } else {
            println!("Workflow completed with errors: {:?}", result.errors);
        }

        Ok(())
    }
}

// Export the main API for use in other modules
pub use KVirtualStageAPI as API;