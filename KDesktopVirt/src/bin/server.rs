/*!
 * KVirtualStage Web Server & API Endpoint
 * 
 * Provides:
 * - REST API endpoints for remote control
 * - WebSocket streaming for live desktop viewing
 * - Web UI dashboard
 * - Performance monitoring endpoints
 * - Health checks and metrics
 */

use axum::{
    extract::{Path, State, WebSocketUpgrade},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post, put, delete},
    Router,
};
use kvirtualstage::{
    KVirtualStageAPI, APISessionInfo, WorkflowExecutionResult,
    automation_engine::{AutomationWorkflow, WorkflowStep, StepAction, Point, MouseButton}
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};
use tower_http::cors::CorsLayer;
use tracing::{info, warn, error};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    api: Arc<KVirtualStageAPI>,
    streaming_sessions: Arc<RwLock<HashMap<String, StreamingSession>>>,
}

#[derive(Debug, Clone)]
struct StreamingSession {
    session_id: String,
    user_id: String,
    active: bool,
    quality: String,
}

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
struct CreateSessionRequest {
    user_id: String,
    session_name: String,
    desktop_type: String,
}

#[derive(Debug, Serialize)]
struct CreateSessionResponse {
    session_id: String,
    status: String,
    vnc_url: Option<String>,
    ws_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MoveCursorRequest {
    target_x: f64,
    target_y: f64,
}

#[derive(Debug, Deserialize)]
struct ClickRequest {
    button: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TypeTextRequest {
    text: String,
}

#[derive(Debug, Deserialize)]
struct StartRecordingRequest {
    output_filename: String,
    quality: Option<String>,
}

#[derive(Debug, Serialize)]
struct RecordingResponse {
    recording_id: String,
    status: String,
}

#[derive(Debug, Deserialize)]
struct WorkflowRequest {
    name: String,
    description: String,
    continue_on_error: Option<bool>,
    steps: Vec<WorkflowStepRequest>,
}

#[derive(Debug, Deserialize)]
struct WorkflowStepRequest {
    name: String,
    action_type: String,
    parameters: serde_json::Value,
    timeout_seconds: Option<u64>,
}

#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now(),
        }
    }

    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: chrono::Utc::now(),
        }
    }
}

// ============================================================================
// Main Server
// ============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();
    
    info!("Starting KVirtualStage Web Server");

    // Initialize API
    let api = Arc::new(KVirtualStageAPI::new().await?);
    let streaming_sessions = Arc::new(RwLock::new(HashMap::new()));

    let state = AppState {
        api,
        streaming_sessions,
    };

    // Build router
    let app = Router::new()
        // API endpoints
        .route("/api/v1/sessions", post(create_session))
        .route("/api/v1/sessions", get(list_sessions))
        .route("/api/v1/sessions/:session_id", get(get_session))
        .route("/api/v1/sessions/:session_id", delete(remove_session))
        .route("/api/v1/sessions/:session_id/cursor/move", post(move_cursor))
        .route("/api/v1/sessions/:session_id/mouse/click", post(click))
        .route("/api/v1/sessions/:session_id/keyboard/type", post(type_text))
        .route("/api/v1/sessions/:session_id/recording/start", post(start_recording))
        .route("/api/v1/sessions/:session_id/recording/stop", post(stop_recording))
        .route("/api/v1/sessions/:session_id/workflow", post(execute_workflow))
        
        // WebSocket streaming
        .route("/api/v1/sessions/:session_id/stream", get(stream_desktop))
        
        // Health and metrics
        .route("/api/v1/health", get(health_check))
        .route("/api/v1/metrics", get(get_metrics))
        
        // Web UI static files
        .route("/", get(serve_index))
        .route("/dashboard", get(serve_dashboard))
        .route("/sessions", get(serve_sessions))
        
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("KVirtualStage Web Server listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    let service = app.into_make_service_with_connect_info::<SocketAddr>();

    axum::serve(listener, service)
        .await?;

    Ok(())
}

// ============================================================================
// Session Management Endpoints
// ============================================================================

async fn create_session(
    State(state): State<AppState>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<ApiResponse<CreateSessionResponse>>, StatusCode> {
    info!("Creating session: {} for user: {}", payload.session_name, payload.user_id);

    match state.api.create_session(
        payload.user_id.clone(),
        payload.session_name.clone(),
        payload.desktop_type,
    ).await {
        Ok(session_id) => {
            let response = CreateSessionResponse {
                session_id: session_id.clone(),
                status: "active".to_string(),
                vnc_url: Some(format!("ws://localhost:8080/api/v1/sessions/{}/stream", session_id)),
                ws_url: Some(format!("ws://localhost:8080/api/v1/sessions/{}/stream", session_id)),
            };
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to create session: {}", e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

async fn list_sessions(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<APISessionInfo>>>, StatusCode> {
    match state.api.list_sessions().await {
        Ok(sessions) => Ok(Json(ApiResponse::success(sessions))),
        Err(e) => {
            error!("Failed to list sessions: {}", e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

async fn get_session(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> Result<Json<ApiResponse<APISessionInfo>>, StatusCode> {
    match state.api.get_session_info(&session_id).await {
        Ok(session_info) => Ok(Json(ApiResponse::success(session_info))),
        Err(e) => {
            error!("Failed to get session {}: {}", session_id, e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

async fn remove_session(
    State(_state): State<AppState>,
    Path(_session_id): Path<String>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // TODO: Implement session removal
    Ok(Json(ApiResponse::success("Session removed".to_string())))
}

// ============================================================================
// Automation Control Endpoints
// ============================================================================

async fn move_cursor(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(payload): Json<MoveCursorRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.api.move_cursor(&session_id, payload.target_x, payload.target_y).await {
        Ok(_) => Ok(Json(ApiResponse::success(format!(
            "Cursor moved to ({:.0}, {:.0})", 
            payload.target_x, 
            payload.target_y
        )))),
        Err(e) => {
            error!("Failed to move cursor in session {}: {}", session_id, e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

async fn click(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(payload): Json<ClickRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.api.click(&session_id, payload.button).await {
        Ok(_) => Ok(Json(ApiResponse::success("Click executed".to_string()))),
        Err(e) => {
            error!("Failed to click in session {}: {}", session_id, e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

async fn type_text(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(payload): Json<TypeTextRequest>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.api.type_text(&session_id, &payload.text).await {
        Ok(_) => Ok(Json(ApiResponse::success(format!(
            "Typed {} characters", 
            payload.text.len()
        )))),
        Err(e) => {
            error!("Failed to type text in session {}: {}", session_id, e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

// ============================================================================
// Recording Endpoints
// ============================================================================

async fn start_recording(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(payload): Json<StartRecordingRequest>,
) -> Result<Json<ApiResponse<RecordingResponse>>, StatusCode> {
    match state.api.start_recording(
        &session_id, 
        &payload.output_filename,
        payload.quality,
    ).await {
        Ok(recording_id) => {
            let response = RecordingResponse {
                recording_id,
                status: "recording".to_string(),
            };
            Ok(Json(ApiResponse::success(response)))
        }
        Err(e) => {
            error!("Failed to start recording in session {}: {}", session_id, e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

async fn stop_recording(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    match state.api.stop_recording(&session_id).await {
        Ok(output_path) => Ok(Json(ApiResponse::success(output_path))),
        Err(e) => {
            error!("Failed to stop recording in session {}: {}", session_id, e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

// ============================================================================
// Workflow Execution
// ============================================================================

async fn execute_workflow(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
    Json(payload): Json<WorkflowRequest>,
) -> Result<Json<ApiResponse<WorkflowExecutionResult>>, StatusCode> {
    // Convert workflow request to internal format
    let steps: Result<Vec<WorkflowStep>, String> = payload.steps.into_iter().map(|step| {
        let action = match step.action_type.as_str() {
            "move_cursor" => {
                let x = step.parameters.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let y = step.parameters.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                StepAction::MoveCursor { to: Point::new(x, y) }
            }
            "click" => {
                let x = step.parameters.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let y = step.parameters.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let button = match step.parameters.get("button").and_then(|v| v.as_str()) {
                    Some("right") => MouseButton::Right,
                    Some("middle") => MouseButton::Middle,
                    _ => MouseButton::Left,
                };
                StepAction::Click { position: Point::new(x, y), button }
            }
            "type" => {
                let text = step.parameters.get("text")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                StepAction::Type { text }
            }
            _ => return Err(format!("Unknown action type: {}", step.action_type))
        };

        Ok(WorkflowStep {
            name: step.name,
            action,
            timeout: step.timeout_seconds.map(|s| std::time::Duration::from_secs(s)),
        })
    }).collect();

    let steps = match steps {
        Ok(steps) => steps,
        Err(e) => return Ok(Json(ApiResponse::error(e))),
    };

    let workflow = AutomationWorkflow {
        name: payload.name,
        description: payload.description,
        continue_on_error: payload.continue_on_error.unwrap_or(false),
        steps,
    };

    match state.api.execute_workflow(&session_id, workflow).await {
        Ok(result) => Ok(Json(ApiResponse::success(result))),
        Err(e) => {
            error!("Failed to execute workflow in session {}: {}", session_id, e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

// ============================================================================
// WebSocket Streaming
// ============================================================================

async fn stream_desktop(
    ws: WebSocketUpgrade,
    Path(session_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    info!("WebSocket connection requested for session: {}", session_id);
    
    ws.on_upgrade(move |socket| handle_socket(socket, session_id, state))
}

async fn handle_socket(
    socket: axum::extract::ws::WebSocket,
    session_id: String,
    _state: AppState,
) {
    use axum::extract::ws::Message;
    use futures::{SinkExt, StreamExt};
    
    let (mut sender, mut receiver) = socket.split();
    
    // Send initial connection message
    if sender.send(Message::Text(format!(
        r#"{{"type":"connected","session_id":"{}","timestamp":"{}"}}"#,
        session_id,
        chrono::Utc::now().to_rfc3339()
    ))).await.is_err() {
        return;
    }

    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received WebSocket message: {}", text);
                // TODO: Handle control messages
            }
            Ok(Message::Binary(_data)) => {
                // TODO: Handle binary data
            }
            Ok(Message::Close(_)) => {
                info!("WebSocket connection closed for session: {}", session_id);
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

// ============================================================================
// Health and Metrics
// ============================================================================

async fn health_check() -> Json<ApiResponse<serde_json::Value>> {
    let health_data = serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now(),
        "version": env!("CARGO_PKG_VERSION"),
        "uptime": "unknown" // TODO: Track actual uptime
    });
    
    Json(ApiResponse::success(health_data))
}

async fn get_metrics() -> Json<ApiResponse<serde_json::Value>> {
    // TODO: Implement real metrics collection
    let metrics = serde_json::json!({
        "active_sessions": 0,
        "total_requests": 0,
        "average_response_time_ms": 0.0,
        "memory_usage_mb": 0,
        "cpu_usage_percent": 0.0
    });
    
    Json(ApiResponse::success(metrics))
}

// ============================================================================
// Static Web UI
// ============================================================================

async fn serve_index() -> impl IntoResponse {
    axum::response::Html(include_str!("../../web/index.html"))
}

async fn serve_dashboard() -> impl IntoResponse {
    axum::response::Html(include_str!("../../web/dashboard.html"))
}

async fn serve_sessions() -> impl IntoResponse {
    axum::response::Html(include_str!("../../web/sessions.html"))
}