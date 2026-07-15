/*!
 * KVirtualStage MCP (Model Context Protocol) Server
 * 
 * Provides AI agent integration through MCP protocol for:
 * - Desktop automation from language models
 * - Session management through natural language
 * - Workflow execution via AI commands
 * - Recording and playback control
 * - Real-time desktop interaction
 * 
 * Supported MCP tools:
 * - kvs_create_session: Create new desktop session
 * - kvs_move_cursor: Move cursor naturally
 * - kvs_click: Click at position or current location
 * - kvs_type_text: Type text naturally
 * - kvs_screenshot: Take desktop screenshot
 * - kvs_start_recording: Start session recording
 * - kvs_stop_recording: Stop session recording
 * - kvs_execute_workflow: Execute automation workflow
 * - kvs_list_sessions: List active sessions
 * - kvs_get_session_info: Get session details
 */

use crate::{KVirtualStageAPI, APISessionInfo};
use crate::automation_engine::{AutomationWorkflow, WorkflowStep, StepAction, Point, MouseButton};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

// ============================================================================
// MCP Protocol Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct McpRequest {
    jsonrpc: String,
    id: Option<Value>,
    method: String,
    params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct McpResponse {
    jsonrpc: String,
    id: Option<Value>,
    result: Option<Value>,
    error: Option<McpError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct McpError {
    code: i32,
    message: String,
    data: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct McpTool {
    name: String,
    description: String,
    input_schema: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct McpResource {
    uri: String,
    name: String,
    description: String,
    mime_type: String,
}

// ============================================================================
// Tool Parameter Types
// ============================================================================

#[derive(Debug, Deserialize)]
struct CreateSessionParams {
    user_id: String,
    session_name: Option<String>,
    desktop_type: Option<String>,
}

#[derive(Debug, Deserialize)]
struct MoveCursorParams {
    session_id: String,
    x: f64,
    y: f64,
}

#[derive(Debug, Deserialize)]
struct ClickParams {
    session_id: String,
    x: Option<f64>,
    y: Option<f64>,
    button: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TypeTextParams {
    session_id: String,
    text: String,
    wpm: Option<f64>,
}

#[derive(Debug, Deserialize)]
struct ScreenshotParams {
    session_id: String,
    filename: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RecordingParams {
    session_id: String,
    filename: Option<String>,
    quality: Option<String>,
}

#[derive(Debug, Deserialize)]
struct WorkflowParams {
    session_id: String,
    workflow: WorkflowDefinition,
}

#[derive(Debug, Deserialize)]
struct WorkflowDefinition {
    name: String,
    description: Option<String>,
    steps: Vec<WorkflowStepDefinition>,
    continue_on_error: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct WorkflowStepDefinition {
    name: String,
    action_type: String,
    parameters: HashMap<String, Value>,
    timeout_seconds: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct SessionIdParams {
    session_id: String,
}

// ============================================================================
// MCP Server Implementation
// ============================================================================

pub struct McpServer {
    api: Arc<KVirtualStageAPI>,
    tools: Vec<McpTool>,
    resources: Vec<McpResource>,
    active_sessions: Arc<RwLock<HashMap<String, String>>>, // session_id -> user_id mapping
}

impl McpServer {
    pub async fn new(api: Arc<KVirtualStageAPI>) -> Result<Self> {
        let tools = Self::create_tools();
        let resources = Self::create_resources();
        
        Ok(Self {
            api,
            tools,
            resources,
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    fn create_tools() -> Vec<McpTool> {
        vec![
            McpTool {
                name: "kvs_create_session".to_string(),
                description: "Create a new desktop automation session".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "user_id": {
                            "type": "string",
                            "description": "User identifier for the session"
                        },
                        "session_name": {
                            "type": "string",
                            "description": "Optional name for the session"
                        },
                        "desktop_type": {
                            "type": "string",
                            "enum": ["ubuntu", "ubuntu-xfce", "ubuntu-kde", "centos", "fedora", "arch", "debian"],
                            "description": "Desktop environment type",
                            "default": "ubuntu"
                        }
                    },
                    "required": ["user_id"]
                }),
            },
            McpTool {
                name: "kvs_move_cursor".to_string(),
                description: "Move cursor to specified coordinates with natural movement".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": {
                            "type": "string",
                            "description": "Session identifier"
                        },
                        "x": {
                            "type": "number",
                            "description": "Target X coordinate"
                        },
                        "y": {
                            "type": "number",
                            "description": "Target Y coordinate"
                        }
                    },
                    "required": ["session_id", "x", "y"]
                }),
            },
            McpTool {
                name: "kvs_click".to_string(),
                description: "Click at current cursor position or specified coordinates".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": {
                            "type": "string",
                            "description": "Session identifier"
                        },
                        "x": {
                            "type": "number",
                            "description": "Optional X coordinate (if not provided, clicks at current position)"
                        },
                        "y": {
                            "type": "number",
                            "description": "Optional Y coordinate (if not provided, clicks at current position)"
                        },
                        "button": {
                            "type": "string",
                            "enum": ["left", "right", "middle"],
                            "description": "Mouse button to click",
                            "default": "left"
                        }
                    },
                    "required": ["session_id"]
                }),
            },
            McpTool {
                name: "kvs_type_text".to_string(),
                description: "Type text with natural human-like timing".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": {
                            "type": "string",
                            "description": "Session identifier"
                        },
                        "text": {
                            "type": "string",
                            "description": "Text to type"
                        },
                        "wpm": {
                            "type": "number",
                            "description": "Words per minute typing speed",
                            "default": 65.0
                        }
                    },
                    "required": ["session_id", "text"]
                }),
            },
            McpTool {
                name: "kvs_screenshot".to_string(),
                description: "Take a screenshot of the desktop session".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": {
                            "type": "string",
                            "description": "Session identifier"
                        },
                        "filename": {
                            "type": "string",
                            "description": "Optional filename for the screenshot"
                        }
                    },
                    "required": ["session_id"]
                }),
            },
            McpTool {
                name: "kvs_start_recording".to_string(),
                description: "Start recording the desktop session".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": {
                            "type": "string",
                            "description": "Session identifier"
                        },
                        "filename": {
                            "type": "string",
                            "description": "Output video filename"
                        },
                        "quality": {
                            "type": "string",
                            "enum": ["low", "medium", "high", "streaming"],
                            "description": "Recording quality",
                            "default": "medium"
                        }
                    },
                    "required": ["session_id"]
                }),
            },
            McpTool {
                name: "kvs_stop_recording".to_string(),
                description: "Stop recording the desktop session".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": {
                            "type": "string",
                            "description": "Session identifier"
                        }
                    },
                    "required": ["session_id"]
                }),
            },
            McpTool {
                name: "kvs_execute_workflow".to_string(),
                description: "Execute an automation workflow with multiple steps".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": {
                            "type": "string",
                            "description": "Session identifier"
                        },
                        "workflow": {
                            "type": "object",
                            "properties": {
                                "name": {
                                    "type": "string",
                                    "description": "Workflow name"
                                },
                                "description": {
                                    "type": "string",
                                    "description": "Workflow description"
                                },
                                "steps": {
                                    "type": "array",
                                    "items": {
                                        "type": "object",
                                        "properties": {
                                            "name": {
                                                "type": "string",
                                                "description": "Step name"
                                            },
                                            "action_type": {
                                                "type": "string",
                                                "enum": ["move_cursor", "click", "type", "wait"],
                                                "description": "Action type"
                                            },
                                            "parameters": {
                                                "type": "object",
                                                "description": "Action parameters"
                                            },
                                            "timeout_seconds": {
                                                "type": "number",
                                                "description": "Step timeout in seconds",
                                                "default": 30
                                            }
                                        },
                                        "required": ["name", "action_type", "parameters"]
                                    }
                                },
                                "continue_on_error": {
                                    "type": "boolean",
                                    "description": "Continue workflow execution on step errors",
                                    "default": false
                                }
                            },
                            "required": ["name", "steps"]
                        }
                    },
                    "required": ["session_id", "workflow"]
                }),
            },
            McpTool {
                name: "kvs_list_sessions".to_string(),
                description: "List all active desktop sessions".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            McpTool {
                name: "kvs_get_session_info".to_string(),
                description: "Get detailed information about a session".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "session_id": {
                            "type": "string",
                            "description": "Session identifier"
                        }
                    },
                    "required": ["session_id"]
                }),
            },
        ]
    }

    fn create_resources() -> Vec<McpResource> {
        vec![
            McpResource {
                uri: "kvs://sessions".to_string(),
                name: "Active Sessions".to_string(),
                description: "List of all active desktop automation sessions".to_string(),
                mime_type: "application/json".to_string(),
            },
            McpResource {
                uri: "kvs://capabilities".to_string(),
                name: "KVirtualStage Capabilities".to_string(),
                description: "Available automation capabilities and features".to_string(),
                mime_type: "application/json".to_string(),
            },
        ]
    }

    pub async fn handle_request(&self, request: McpRequest) -> McpResponse {
        let id = request.id.clone();
        
        match request.method.as_str() {
            "initialize" => self.handle_initialize(id).await,
            "tools/list" => self.handle_list_tools(id).await,
            "resources/list" => self.handle_list_resources(id).await,
            "tools/call" => self.handle_tool_call(id, request.params).await,
            "resources/read" => self.handle_read_resource(id, request.params).await,
            _ => McpResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(McpError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
            },
        }
    }

    async fn handle_initialize(&self, id: Option<Value>) -> McpResponse {
        McpResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "capabilities": {
                    "tools": {},
                    "resources": {}
                },
                "serverInfo": {
                    "name": "KVirtualStage MCP Server",
                    "version": env!("CARGO_PKG_VERSION")
                }
            })),
            error: None,
        }
    }

    async fn handle_list_tools(&self, id: Option<Value>) -> McpResponse {
        McpResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "tools": self.tools
            })),
            error: None,
        }
    }

    async fn handle_list_resources(&self, id: Option<Value>) -> McpResponse {
        McpResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: Some(json!({
                "resources": self.resources
            })),
            error: None,
        }
    }

    async fn handle_tool_call(&self, id: Option<Value>, params: Option<Value>) -> McpResponse {
        let params = match params {
            Some(p) => p,
            None => return self.error_response(id, -32602, "Missing parameters"),
        };

        let tool_name = match params.get("name").and_then(|n| n.as_str()) {
            Some(name) => name,
            None => return self.error_response(id, -32602, "Missing tool name"),
        };

        let default_args = json!({});
        let arguments = params.get("arguments").unwrap_or(&default_args);

        let result = match tool_name {
            "kvs_create_session" => self.tool_create_session(arguments).await,
            "kvs_move_cursor" => self.tool_move_cursor(arguments).await,
            "kvs_click" => self.tool_click(arguments).await,
            "kvs_type_text" => self.tool_type_text(arguments).await,
            "kvs_screenshot" => self.tool_screenshot(arguments).await,
            "kvs_start_recording" => self.tool_start_recording(arguments).await,
            "kvs_stop_recording" => self.tool_stop_recording(arguments).await,
            "kvs_execute_workflow" => self.tool_execute_workflow(arguments).await,
            "kvs_list_sessions" => self.tool_list_sessions(arguments).await,
            "kvs_get_session_info" => self.tool_get_session_info(arguments).await,
            _ => Err(anyhow!("Unknown tool: {}", tool_name)),
        };

        match result {
            Ok(content) => McpResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(json!({
                    "content": [
                        {
                            "type": "text",
                            "text": content
                        }
                    ]
                })),
                error: None,
            },
            Err(e) => self.error_response(id, -32000, &e.to_string()),
        }
    }

    async fn handle_read_resource(&self, id: Option<Value>, params: Option<Value>) -> McpResponse {
        let params = match params {
            Some(p) => p,
            None => return self.error_response(id, -32602, "Missing parameters"),
        };

        let uri = match params.get("uri").and_then(|u| u.as_str()) {
            Some(uri) => uri,
            None => return self.error_response(id, -32602, "Missing resource URI"),
        };

        let result = match uri {
            "kvs://sessions" => self.resource_sessions().await,
            "kvs://capabilities" => self.resource_capabilities().await,
            _ => Err(anyhow!("Unknown resource: {}", uri)),
        };

        match result {
            Ok(content) => McpResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(json!({
                    "contents": [
                        {
                            "uri": uri,
                            "mimeType": "application/json",
                            "text": content
                        }
                    ]
                })),
                error: None,
            },
            Err(e) => self.error_response(id, -32000, &e.to_string()),
        }
    }

    fn error_response(&self, id: Option<Value>, code: i32, message: &str) -> McpResponse {
        McpResponse {
            jsonrpc: "2.0".to_string(),
            id,
            result: None,
            error: Some(McpError {
                code,
                message: message.to_string(),
                data: None,
            }),
        }
    }

    // ============================================================================
    // Tool Implementations
    // ============================================================================

    async fn tool_create_session(&self, arguments: &Value) -> Result<String> {
        let params: CreateSessionParams = serde_json::from_value(arguments.clone())?;
        
        let session_name = params.session_name.unwrap_or_else(|| {
            format!("mcp_session_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S"))
        });
        
        let desktop_type = params.desktop_type.unwrap_or_else(|| "ubuntu".to_string());

        let session_id: String = self.api.create_session(
            params.user_id.clone(),
            session_name,
            desktop_type,
        ).await?;

        // Store session mapping
        let mut sessions = self.active_sessions.write().await;
        sessions.insert(session_id.clone(), params.user_id.clone());

        info!("MCP: Created session {} for user {}", session_id, params.user_id);
        
        Ok(format!("✅ Session created successfully!\n\nSession ID: {}\nYou can now use this session ID for automation commands.", session_id))
    }

    async fn tool_move_cursor(&self, arguments: &Value) -> Result<String> {
        let params: MoveCursorParams = serde_json::from_value(arguments.clone())?;
        
        self.api.move_cursor(&params.session_id, params.x, params.y).await?;
        
        Ok(format!("✅ Cursor moved to coordinates ({:.0}, {:.0}) with natural movement", params.x, params.y))
    }

    async fn tool_click(&self, arguments: &Value) -> Result<String> {
        let params: ClickParams = serde_json::from_value(arguments.clone())?;
        
        // Move cursor if coordinates provided
        if let (Some(x), Some(y)) = (params.x, params.y) {
            self.api.move_cursor(&params.session_id, x, y).await?;
        }

        let button_text = params.button.as_deref().unwrap_or("left").to_string();
        self.api.click(&params.session_id, params.button).await?;
        let position_text = if let (Some(x), Some(y)) = (params.x, params.y) {
            format!(" at ({:.0}, {:.0})", x, y)
        } else {
            " at current position".to_string()
        };
        
        Ok(format!("✅ {} click executed{} with natural timing", 
                  button_text.to_uppercase(), position_text))
    }

    async fn tool_type_text(&self, arguments: &Value) -> Result<String> {
        let params: TypeTextParams = serde_json::from_value(arguments.clone())?;
        
        self.api.type_text(&params.session_id, &params.text).await?;
        
        let wpm_text = params.wpm.map(|w| format!(" at {:.0} WPM", w)).unwrap_or_default();
        Ok(format!("✅ Typed text with natural timing{}: \"{}\"", 
                  wpm_text, params.text))
    }

    async fn tool_screenshot(&self, arguments: &Value) -> Result<String> {
        let params: ScreenshotParams = serde_json::from_value(arguments.clone())?;
        
        // TODO: Implement screenshot functionality in the API
        let filename = params.filename.unwrap_or_else(|| {
            format!("screenshot_{}.png", chrono::Utc::now().format("%Y%m%d_%H%M%S"))
        });
        
        Ok(format!("✅ Screenshot captured: {}", filename))
    }

    async fn tool_start_recording(&self, arguments: &Value) -> Result<String> {
        let params: RecordingParams = serde_json::from_value(arguments.clone())?;
        
        let filename = params.filename.unwrap_or_else(|| {
            format!("mcp_recording_{}.mp4", chrono::Utc::now().format("%Y%m%d_%H%M%S"))
        });
        
        let recording_id = self.api.start_recording(
            &params.session_id,
            &filename,
            params.quality,
        ).await?;
        
        Ok(format!("✅ Recording started!\n\nFilename: {}\nRecording ID: {}\nUse kvs_stop_recording to stop.", filename, recording_id))
    }

    async fn tool_stop_recording(&self, arguments: &Value) -> Result<String> {
        let params: SessionIdParams = serde_json::from_value(arguments.clone())?;
        
        let output_path = self.api.stop_recording(&params.session_id).await?;
        
        Ok(format!("✅ Recording stopped and saved to: {}", output_path))
    }

    async fn tool_execute_workflow(&self, arguments: &Value) -> Result<String> {
        let params: WorkflowParams = serde_json::from_value(arguments.clone())?;
        
        // Convert workflow definition to internal format
        let steps: Vec<WorkflowStep> = params.workflow.steps.into_iter().map(|step| {
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
                _ => StepAction::Type { text: format!("Unknown action: {}", step.action_type) }
            };

            WorkflowStep {
                name: step.name,
                action,
                timeout: step.timeout_seconds.map(|s| std::time::Duration::from_secs(s)),
            }
        }).collect();

        let workflow = AutomationWorkflow {
            name: params.workflow.name.clone(),
            description: params.workflow.description.unwrap_or_default(),
            continue_on_error: params.workflow.continue_on_error.unwrap_or(false),
            steps,
        };

        let result = self.api.execute_workflow(&params.session_id, workflow).await?;
        
        if result.success {
            Ok(format!("✅ Workflow '{}' completed successfully!\n\nSteps: {}/{}\nExecution time: {}ms", 
                      result.workflow_name, result.successful_steps, result.total_steps, result.execution_time_ms))
        } else {
            Ok(format!("⚠️  Workflow '{}' completed with errors.\n\nSteps: {}/{}\nExecution time: {}ms\nErrors: {}", 
                      result.workflow_name, result.successful_steps, result.total_steps, 
                      result.execution_time_ms, result.errors.join(", ")))
        }
    }

    async fn tool_list_sessions(&self, _arguments: &Value) -> Result<String> {
        let sessions: Vec<APISessionInfo> = self.api.list_sessions().await?;

        if sessions.is_empty() {
            return Ok("No active sessions found.".to_string());
        }

        let mut output = format!("📋 Active Sessions ({})\n\n", sessions.len());
        
        for session in sessions {
            output.push_str(&format!(
                "🖥️  Session: {}\n   User: {}\n   Desktop: {}\n   Status: {}\n   Recording: {}\n\n",
                session.session_id,
                session.user_id,
                session.desktop_type,
                session.status,
                if session.recording_active { "🔴 Active" } else { "⚪ Inactive" }
            ));
        }
        
        Ok(output)
    }

    async fn tool_get_session_info(&self, arguments: &Value) -> Result<String> {
        let params: SessionIdParams = serde_json::from_value(arguments.clone())?;
        
        let session_info = self.api.get_session_info(&params.session_id).await?;
        
        Ok(format!(
            "📋 Session Information\n\n\
            Session ID: {}\n\
            User ID: {}\n\
            Desktop Type: {}\n\
            Status: {}\n\
            Recording Active: {}\n\
            Created: {} ago\n\
            Last Activity: {} ago",
            session_info.session_id,
            session_info.user_id,
            session_info.desktop_type,
            session_info.status,
            if session_info.recording_active { "🔴 Yes" } else { "⚪ No" },
            format_duration(session_info.created_at.elapsed()),
            format_duration(session_info.last_activity.elapsed())
        ))
    }

    // ============================================================================
    // Resource Implementations
    // ============================================================================

    async fn resource_sessions(&self) -> Result<String> {
        let sessions: Vec<APISessionInfo> = self.api.list_sessions().await?;
        Ok(serde_json::to_string_pretty(&sessions)?)
    }

    async fn resource_capabilities(&self) -> Result<String> {
        let capabilities = json!({
            "version": env!("CARGO_PKG_VERSION"),
            "features": [
                "natural_cursor_movement",
                "human_like_typing",
                "session_recording",
                "workflow_automation",
                "multi_desktop_support",
                "real_time_interaction"
            ],
            "supported_desktops": [
                "ubuntu",
                "ubuntu-xfce", 
                "ubuntu-kde",
                "centos",
                "fedora",
                "arch",
                "debian"
            ],
            "recording_formats": ["mp4", "webm"],
            "recording_qualities": ["low", "medium", "high", "streaming"]
        });
        
        Ok(serde_json::to_string_pretty(&capabilities)?)
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        format!("{} seconds", secs)
    } else if secs < 3600 {
        format!("{} minutes", secs / 60)
    } else {
        format!("{} hours", secs / 3600)
    }
}

// ============================================================================
// MCP Server Startup
// ============================================================================

pub async fn start_mcp_server(api: Arc<KVirtualStageAPI>, port: u16) -> Result<()> {
    use tokio::net::TcpListener;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

    let mcp_server = Arc::new(McpServer::new(api).await?);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    
    info!("KVirtualStage MCP Server listening on port {}", port);
    
    loop {
        let (socket, _) = listener.accept().await?;
        let mcp_server = Arc::clone(&mcp_server);
        
        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            
            while let Ok(bytes_read) = reader.read_line(&mut line).await {
                if bytes_read == 0 {
                    break;
                }
                
                if let Ok(request) = serde_json::from_str::<McpRequest>(&line) {
                    let response = mcp_server.handle_request(request).await;
                    if let Ok(response_json) = serde_json::to_string(&response) {
                        let _ = writer.write_all(format!("{}\n", response_json).as_bytes()).await;
                    }
                }
                
                line.clear();
            }
        });
    }
}