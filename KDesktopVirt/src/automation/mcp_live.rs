/*!
MCP Live Scripting Server for Real-time Desktop Automation
Provides Playwright-equivalent live tool calls for desktop automation
*/

use crate::automation::{AutomationScript, AutomationResult, AutomationAction};
use crate::automation::accuracy::AccuracyEngine;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use tracing::{info, error};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpToolCall {
    pub tool_name: String,
    pub parameters: HashMap<String, serde_json::Value>,
    pub call_id: String,
    pub timestamp: f64,
    pub result: Option<HashMap<String, serde_json::Value>>,
    pub success: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiveSession {
    pub session_id: String,
    pub active: bool,
    pub tool_calls: Vec<McpToolCall>,
    pub current_step: usize,
    pub automation_state: HashMap<String, serde_json::Value>,
    pub created_at: f64,
}

type ToolFunction = Box<dyn Fn(&LiveSession, &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> + Send + Sync>;

/// MCP Live Scripting Server for real-time automation
pub struct McpLiveServer {
    pub accuracy_engine: Arc<AccuracyEngine>,
    pub active_sessions: Arc<RwLock<HashMap<String, LiveSession>>>,
    pub tool_registry: HashMap<String, String>, // tool_name -> description
}

impl McpLiveServer {
    pub fn new() -> Result<Self> {
        let mut tool_registry = HashMap::new();
        
        // Register available tools
        tool_registry.insert("create_session".to_string(), "Create a new live scripting session".to_string());
        tool_registry.insert("take_screenshot".to_string(), "Take a screenshot of the current desktop".to_string());
        tool_registry.insert("precise_click".to_string(), "Click at precise coordinates with description".to_string());
        tool_registry.insert("type_text".to_string(), "Type text with natural timing".to_string());
        tool_registry.insert("launch_application".to_string(), "Launch an application by command".to_string());
        tool_registry.insert("wait_for_application".to_string(), "Wait for application to be ready".to_string());
        tool_registry.insert("get_window_info".to_string(), "Get information about a window".to_string());
        tool_registry.insert("focus_window".to_string(), "Focus a specific window".to_string());
        tool_registry.insert("list_windows".to_string(), "List all open windows".to_string());
        tool_registry.insert("click_calculator_button".to_string(), "Click a specific calculator button".to_string());
        tool_registry.insert("perform_calculation".to_string(), "Perform a complete calculation".to_string());
        tool_registry.insert("click_text_area".to_string(), "Click in a text editor area".to_string());
        tool_registry.insert("verify_action".to_string(), "Verify the last action with screenshot".to_string());
        tool_registry.insert("get_session_state".to_string(), "Get current session state and history".to_string());
        tool_registry.insert("get_current_state".to_string(), "Get current desktop state".to_string());
        
        Ok(Self {
            accuracy_engine: Arc::new(AccuracyEngine::new()?),
            active_sessions: Arc::new(RwLock::new(HashMap::new())),
            tool_registry,
        })
    }

    /// Execute automation script with live tool calls
    pub async fn execute_script(&self, script: AutomationScript) -> Result<AutomationResult> {
        info!("🔧 Executing MCP live script: {}", script.name);
        
        let session_id = Uuid::new_v4().to_string();
        
        // Create session
        self.create_live_session(&session_id).await?;
        
        let mut result = AutomationResult {
            success: false,
            actions_completed: 0,
            total_actions: script.actions.len(),
            errors: Vec::new(),
            execution_log: Vec::new(),
            screenshots: Vec::new(),
            metadata: HashMap::new(),
        };
        
        // Convert actions to tool calls and execute
        for (i, action) in script.actions.iter().enumerate() {
            let tool_call = self.action_to_tool_call(action, i + 1)?;
            
            match self.execute_live_tool_call(&session_id, &tool_call.tool_name, &tool_call.parameters).await {
                Ok(call_result) => {
                    result.actions_completed += 1;
                    result.execution_log.push(format!("✅ Tool call {}: {}", i + 1, tool_call.tool_name));
                    
                    // Extract screenshot path if available
                    if let Some(screenshot_path) = call_result.result.as_ref()
                        .and_then(|r| r.get("screenshot_path"))
                        .and_then(|p| p.as_str()) {
                        result.screenshots.push(screenshot_path.to_string());
                    }
                }
                Err(e) => {
                    let error_msg = format!("❌ Tool call {} failed: {}", i + 1, e);
                    result.errors.push(error_msg.clone());
                    result.execution_log.push(error_msg);
                }
            }
            
            // Brief pause between actions
            sleep(Duration::from_millis(500)).await;
        }
        
        result.success = result.actions_completed == result.total_actions;
        
        // Add session metadata
        result.metadata.insert("session_id".to_string(), serde_json::Value::String(session_id));
        result.metadata.insert("mcp_mode".to_string(), serde_json::Value::Bool(true));
        
        info!("🏆 MCP script completed: {}/{} actions successful", result.actions_completed, result.total_actions);
        
        Ok(result)
    }

    /// Execute live MCP tool call
    pub async fn execute_live_tool_call(
        &self,
        session_id: &str,
        tool_name: &str,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<McpToolCall> {
        let call_id = format!("{}_{}", session_id, Uuid::new_v4());
        let timestamp = chrono::Utc::now().timestamp() as f64;
        
        info!("🔧 Executing MCP tool call: {}", tool_name);
        
        let mut tool_call = McpToolCall {
            tool_name: tool_name.to_string(),
            parameters: parameters.clone(),
            call_id: call_id.clone(),
            timestamp,
            result: None,
            success: false,
            error: None,
        };
        
        // Execute the tool
        match self.execute_tool(session_id, tool_name, parameters).await {
            Ok(result) => {
                tool_call.result = Some(result);
                tool_call.success = true;
                
                // Update session
                self.update_session_with_call(session_id, &tool_call).await?;
                
                info!("✅ Tool call completed: {}", tool_name);
            }
            Err(e) => {
                tool_call.error = Some(e.to_string());
                tool_call.success = false;
                error!("❌ Tool call failed: {} - {}", tool_name, e);
            }
        }
        
        Ok(tool_call)
    }

    /// Execute individual tool
    async fn execute_tool(
        &self,
        session_id: &str,
        tool_name: &str,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<HashMap<String, serde_json::Value>> {
        match tool_name {
            "create_session" => self.tool_create_session(session_id, parameters).await,
            "take_screenshot" => self.tool_take_screenshot(session_id, parameters).await,
            "precise_click" => self.tool_precise_click(session_id, parameters).await,
            "type_text" => self.tool_type_text(session_id, parameters).await,
            "launch_application" => self.tool_launch_application(session_id, parameters).await,
            "wait_for_application" => self.tool_wait_for_application(session_id, parameters).await,
            "get_window_info" => self.tool_get_window_info(session_id, parameters).await,
            "focus_window" => self.tool_focus_window(session_id, parameters).await,
            "list_windows" => self.tool_list_windows(session_id, parameters).await,
            "click_calculator_button" => self.tool_click_calculator_button(session_id, parameters).await,
            "perform_calculation" => self.tool_perform_calculation(session_id, parameters).await,
            "click_text_area" => self.tool_click_text_area(session_id, parameters).await,
            "verify_action" => self.tool_verify_action(session_id, parameters).await,
            "get_session_state" => self.tool_get_session_state(session_id, parameters).await,
            "get_current_state" => self.tool_get_current_state(session_id, parameters).await,
            _ => Err(anyhow::anyhow!("Unknown tool: {}", tool_name)),
        }
    }

    /// Convert automation action to tool call
    fn action_to_tool_call(&self, action: &AutomationAction, step: usize) -> Result<McpToolCall> {
        let mut parameters = HashMap::new();
        
        let tool_name = match action.action_type.as_str() {
            "screenshot" => {
                if let Some(target) = &action.target {
                    parameters.insert("output_path".to_string(), serde_json::Value::String(target.clone()));
                } else {
                    parameters.insert("output_path".to_string(), 
                        serde_json::Value::String(format!("/tmp/mcp_step_{:02}.png", step)));
                }
                "take_screenshot"
            }
            "launch" => {
                if let Some(target) = &action.target {
                    parameters.insert("app_name".to_string(), serde_json::Value::String(target.clone()));
                    parameters.insert("command".to_string(), serde_json::Value::String(target.clone()));
                }
                "launch_application"
            }
            "click" => {
                if let Some((x, y)) = action.coordinates {
                    parameters.insert("x".to_string(), serde_json::Value::Number(x.into()));
                    parameters.insert("y".to_string(), serde_json::Value::Number(y.into()));
                }
                if let Some(description) = &action.target {
                    parameters.insert("description".to_string(), serde_json::Value::String(description.clone()));
                }
                "precise_click"
            }
            "type" => {
                if let Some(text) = &action.text {
                    parameters.insert("text".to_string(), serde_json::Value::String(text.clone()));
                }
                "type_text"
            }
            "wait" => {
                parameters.insert("duration".to_string(), 
                    serde_json::Value::Number(((action.delay.unwrap_or(1.0) * 1000.0) as i64).into()));
                return Err(anyhow::anyhow!("Wait action not supported in MCP mode"));
            }
            _ => return Err(anyhow::anyhow!("Unknown action type: {}", action.action_type)),
        };
        
        Ok(McpToolCall {
            tool_name: tool_name.to_string(),
            parameters,
            call_id: Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now().timestamp() as f64,
            result: None,
            success: false,
            error: None,
        })
    }

    /// Create live session
    async fn create_live_session(&self, session_id: &str) -> Result<()> {
        let session = LiveSession {
            session_id: session_id.to_string(),
            active: true,
            tool_calls: Vec::new(),
            current_step: 0,
            automation_state: HashMap::new(),
            created_at: chrono::Utc::now().timestamp() as f64,
        };
        
        let mut sessions = self.active_sessions.write().await;
        sessions.insert(session_id.to_string(), session);
        
        info!("📝 Created live session: {}", session_id);
        Ok(())
    }

    /// Update session with tool call result
    async fn update_session_with_call(&self, session_id: &str, tool_call: &McpToolCall) -> Result<()> {
        let mut sessions = self.active_sessions.write().await;
        
        if let Some(session) = sessions.get_mut(session_id) {
            session.tool_calls.push(tool_call.clone());
            session.current_step += 1;
            
            // Update state from tool result
            if let Some(result) = &tool_call.result {
                if let Some(state_updates) = result.get("state_updates") {
                    if let Some(updates_obj) = state_updates.as_object() {
                        for (key, value) in updates_obj {
                            session.automation_state.insert(key.clone(), value.clone());
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    // Tool implementations
    async fn tool_create_session(&self, session_id: &str, _params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let mut result = HashMap::new();
        result.insert("session_created".to_string(), serde_json::Value::Bool(true));
        result.insert("session_id".to_string(), serde_json::Value::String(session_id.to_string()));
        Ok(result)
    }

    async fn tool_take_screenshot(&self, _session_id: &str, params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let default_path = format!("/tmp/mcp_screenshot_{}.png", chrono::Utc::now().timestamp());
        let output_path = params.get("output_path")
            .and_then(|v| v.as_str())
            .unwrap_or(&default_path);
        
        self.accuracy_engine.take_screenshot(output_path).await?;
        
        let mut result = HashMap::new();
        result.insert("screenshot_taken".to_string(), serde_json::Value::Bool(true));
        result.insert("screenshot_path".to_string(), serde_json::Value::String(output_path.to_string()));
        result.insert("timestamp".to_string(), serde_json::Value::Number(chrono::Utc::now().timestamp().into()));
        
        Ok(result)
    }

    async fn tool_precise_click(&self, _session_id: &str, params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let x = params.get("x").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let y = params.get("y").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let description = params.get("description").and_then(|v| v.as_str());
        
        self.accuracy_engine.precise_click(x, y, description).await?;
        
        let mut result = HashMap::new();
        result.insert("clicked".to_string(), serde_json::Value::Bool(true));
        result.insert("coordinates".to_string(), serde_json::Value::Array(vec![x.into(), y.into()]));
        
        Ok(result)
    }

    async fn tool_type_text(&self, _session_id: &str, params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let text = params.get("text").and_then(|v| v.as_str()).unwrap_or("");
        
        self.accuracy_engine.type_text(text).await?;
        
        let mut result = HashMap::new();
        result.insert("text_typed".to_string(), serde_json::Value::Bool(true));
        result.insert("text_length".to_string(), serde_json::Value::Number(text.len().into()));
        
        Ok(result)
    }

    async fn tool_launch_application(&self, _session_id: &str, params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let app_name = params.get("app_name").and_then(|v| v.as_str()).unwrap_or("");
        
        self.accuracy_engine.launch_application(app_name).await?;
        
        let mut result = HashMap::new();
        result.insert("application_launched".to_string(), serde_json::Value::Bool(true));
        result.insert("app_name".to_string(), serde_json::Value::String(app_name.to_string()));
        
        Ok(result)
    }

    async fn tool_wait_for_application(&self, _session_id: &str, params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let app_name = params.get("app_name").and_then(|v| v.as_str()).unwrap_or("");
        let timeout = params.get("timeout").and_then(|v| v.as_u64()).unwrap_or(10);
        
        let ready = self.accuracy_engine.wait_for_application(app_name, timeout).await?;
        
        let mut result = HashMap::new();
        result.insert("application_ready".to_string(), serde_json::Value::Bool(ready));
        result.insert("app_name".to_string(), serde_json::Value::String(app_name.to_string()));
        
        Ok(result)
    }

    async fn tool_get_window_info(&self, _session_id: &str, params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let window_identifier = params.get("window_class").and_then(|v| v.as_str()).unwrap_or("");
        
        let window_info = self.accuracy_engine.find_window_info(window_identifier).await?;
        
        let mut result = HashMap::new();
        result.insert("window_found".to_string(), serde_json::Value::Bool(window_info.is_some()));
        
        if let Some(info) = window_info {
            result.insert("window_info".to_string(), serde_json::to_value(&info)?);
        }
        
        Ok(result)
    }

    async fn tool_focus_window(&self, _session_id: &str, params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let window_title = params.get("window_title").and_then(|v| v.as_str()).unwrap_or("");
        
        let output = tokio::process::Command::new("wmctrl")
            .args(&["-a", window_title])
            .output()
            .await?;
        
        let success = output.status.success();
        
        let mut result = HashMap::new();
        result.insert("window_focused".to_string(), serde_json::Value::Bool(success));
        result.insert("window_title".to_string(), serde_json::Value::String(window_title.to_string()));
        
        Ok(result)
    }

    async fn tool_list_windows(&self, _session_id: &str, _params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let windows = self.accuracy_engine.list_windows().await?;
        
        let mut result = HashMap::new();
        result.insert("windows_listed".to_string(), serde_json::Value::Bool(true));
        result.insert("window_count".to_string(), serde_json::Value::Number(windows.len().into()));
        result.insert("windows".to_string(), serde_json::to_value(&windows)?);
        
        Ok(result)
    }

    async fn tool_click_calculator_button(&self, _session_id: &str, params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let button = params.get("button").and_then(|v| v.as_str()).unwrap_or("");
        
        // Get calculator window
        let window_info = self.accuracy_engine.find_window_info("galculator").await?
            .ok_or_else(|| anyhow::anyhow!("Calculator window not found"))?;
        
        // Get button layout
        let layout = self.accuracy_engine.calculate_calculator_buttons(&window_info).await?;
        
        let (x, y) = layout.buttons.get(button)
            .ok_or_else(|| anyhow::anyhow!("Button '{}' not found", button))?;
        
        self.accuracy_engine.precise_click(*x, *y, Some(&format!("Calculator button: {}", button))).await?;
        
        let mut result = HashMap::new();
        result.insert("button_clicked".to_string(), serde_json::Value::Bool(true));
        result.insert("button".to_string(), serde_json::Value::String(button.to_string()));
        result.insert("coordinates".to_string(), serde_json::Value::Array(vec![(*x).into(), (*y).into()]));
        
        Ok(result)
    }

    async fn tool_perform_calculation(&self, _session_id: &str, params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let expression = params.get("expression").and_then(|v| v.as_str()).unwrap_or("");
        
        // Parse expression and click buttons
        let mut buttons_to_press = Vec::new();
        for char in expression.chars() {
            match char {
                '0'..='9' => buttons_to_press.push(char.to_string()),
                '+' => buttons_to_press.push("+".to_string()),
                '-' => buttons_to_press.push("-".to_string()),
                '*' | '×' => buttons_to_press.push("×".to_string()),
                '/' | '÷' => buttons_to_press.push("÷".to_string()),
                '=' => buttons_to_press.push("=".to_string()),
                _ => {}
            }
        }
        
        if !buttons_to_press.contains(&"=".to_string()) {
            buttons_to_press.push("=".to_string());
        }
        
        // Get calculator layout
        let window_info = self.accuracy_engine.find_window_info("galculator").await?
            .ok_or_else(|| anyhow::anyhow!("Calculator window not found"))?;
        let layout = self.accuracy_engine.calculate_calculator_buttons(&window_info).await?;
        
        // Press buttons
        for button in &buttons_to_press {
            if let Some((x, y)) = layout.buttons.get(button) {
                self.accuracy_engine.precise_click(*x, *y, Some(&format!("Calculator: {}", button))).await?;
                sleep(Duration::from_millis(500)).await;
            }
        }
        
        let mut result = HashMap::new();
        result.insert("calculation_performed".to_string(), serde_json::Value::Bool(true));
        result.insert("expression".to_string(), serde_json::Value::String(expression.to_string()));
        result.insert("buttons_pressed".to_string(), serde_json::to_value(&buttons_to_press)?);
        
        Ok(result)
    }

    async fn tool_click_text_area(&self, _session_id: &str, params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let editor_name = params.get("editor_name").and_then(|v| v.as_str()).unwrap_or("mousepad");
        
        let window_info = self.accuracy_engine.find_window_info(editor_name).await?
            .ok_or_else(|| anyhow::anyhow!("Text editor '{}' not found", editor_name))?;
        
        // Click in center of text area
        let text_x = window_info.x + window_info.width / 2;
        let text_y = window_info.y + window_info.height / 2 + 20;
        
        self.accuracy_engine.precise_click(text_x, text_y, Some(&format!("Text area in {}", editor_name))).await?;
        
        let mut result = HashMap::new();
        result.insert("text_area_clicked".to_string(), serde_json::Value::Bool(true));
        result.insert("editor_name".to_string(), serde_json::Value::String(editor_name.to_string()));
        result.insert("coordinates".to_string(), serde_json::Value::Array(vec![text_x.into(), text_y.into()]));
        
        Ok(result)
    }

    async fn tool_verify_action(&self, session_id: &str, params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let action_type = params.get("action_type").and_then(|v| v.as_str()).unwrap_or("screenshot");
        
        if action_type == "screenshot" {
            let verify_path = format!("/tmp/verify_{}_{}.png", session_id, chrono::Utc::now().timestamp());
            self.accuracy_engine.take_screenshot(&verify_path).await?;
            
            let mut result = HashMap::new();
            result.insert("verification_completed".to_string(), serde_json::Value::Bool(true));
            result.insert("verification_type".to_string(), serde_json::Value::String("screenshot".to_string()));
            result.insert("screenshot_path".to_string(), serde_json::Value::String(verify_path));
            
            return Ok(result);
        }
        
        Err(anyhow::anyhow!("Unknown verification type: {}", action_type))
    }

    async fn tool_get_session_state(&self, session_id: &str, _params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let sessions = self.active_sessions.read().await;
        
        if let Some(session) = sessions.get(session_id) {
            let mut result = HashMap::new();
            result.insert("session_active".to_string(), serde_json::Value::Bool(session.active));
            result.insert("current_step".to_string(), serde_json::Value::Number(session.current_step.into()));
            result.insert("total_tool_calls".to_string(), serde_json::Value::Number(session.tool_calls.len().into()));
            result.insert("automation_state".to_string(), serde_json::to_value(&session.automation_state)?);
            
            return Ok(result);
        }
        
        Err(anyhow::anyhow!("Session not found: {}", session_id))
    }

    async fn tool_get_current_state(&self, session_id: &str, _params: &HashMap<String, serde_json::Value>) -> Result<HashMap<String, serde_json::Value>> {
        let state_screenshot = format!("/tmp/state_{}_{}.png", session_id, chrono::Utc::now().timestamp());
        self.accuracy_engine.take_screenshot(&state_screenshot).await?;
        
        let windows = self.accuracy_engine.list_windows().await?;
        
        let mut result = HashMap::new();
        result.insert("current_state_captured".to_string(), serde_json::Value::Bool(true));
        result.insert("screenshot_path".to_string(), serde_json::Value::String(state_screenshot));
        result.insert("open_windows".to_string(), serde_json::to_value(&windows)?);
        result.insert("window_count".to_string(), serde_json::Value::Number(windows.len().into()));
        
        Ok(result)
    }
}

impl Default for McpLiveServer {
    fn default() -> Self {
        Self::new().expect("Failed to create MCP live server")
    }
}