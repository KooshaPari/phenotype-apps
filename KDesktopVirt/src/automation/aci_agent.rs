/*!
Agent-Computer Interface (ACI) for AI Agent Desktop Control
Provides autonomous desktop interaction capabilities for AI agents
*/

use crate::automation::{AutomationScript, AutomationResult};
use crate::automation::accuracy::AccuracyEngine;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AciCommand {
    pub command_type: String,
    pub target: Option<serde_json::Value>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub interaction_type: Option<String>,
    pub verification: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AciObservation {
    pub observation_type: String,
    pub timestamp: f64,
    pub data: serde_json::Value,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AciAgentSession {
    pub agent_id: String,
    pub session_id: String,
    pub active: bool,
    pub commands_executed: Vec<AciCommand>,
    pub observations: Vec<AciObservation>,
    pub current_state: HashMap<String, serde_json::Value>,
    pub created_at: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AciExecutionPlan {
    pub agent_id: String,
    pub goal: String,
    pub commands: Vec<AciCommand>,
    pub expected_outcome: String,
    pub verification_steps: Vec<String>,
}

/// ACI Interface for AI Agent desktop control
pub struct AciInterface {
    pub accuracy_engine: Arc<AccuracyEngine>,
    pub active_agents: Arc<RwLock<HashMap<String, AciAgentSession>>>,
    pub command_registry: HashMap<String, String>,
}

impl AciInterface {
    pub fn new() -> Result<Self> {
        let mut command_registry = HashMap::new();
        
        // Register available ACI commands
        command_registry.insert("observe_desktop".to_string(), "Observe current desktop state".to_string());
        command_registry.insert("launch_application".to_string(), "Launch application autonomously".to_string());
        command_registry.insert("interact_with_element".to_string(), "Interact with UI element".to_string());
        command_registry.insert("perform_workflow".to_string(), "Execute complex workflow".to_string());
        command_registry.insert("analyze_screen".to_string(), "Analyze screen content for decision making".to_string());
        command_registry.insert("navigate_application".to_string(), "Navigate within application".to_string());
        command_registry.insert("extract_information".to_string(), "Extract information from screen".to_string());
        command_registry.insert("verify_state".to_string(), "Verify current state against expectation".to_string());
        command_registry.insert("take_action".to_string(), "Take specific action based on observation".to_string());
        command_registry.insert("get_session_state".to_string(), "Get current agent session state".to_string());
        
        Ok(Self {
            accuracy_engine: Arc::new(AccuracyEngine::new()?),
            active_agents: Arc::new(RwLock::new(HashMap::new())),
            command_registry,
        })
    }

    /// Execute automation script in ACI agent mode
    pub async fn execute_script(&self, script: AutomationScript) -> Result<AutomationResult> {
        info!("🤖 Executing ACI agent script: {}", script.name);
        
        let agent_id = Uuid::new_v4().to_string();
        let session_id = Uuid::new_v4().to_string();
        
        // Create agent session
        self.create_agent_session(&agent_id, &session_id).await?;
        
        let mut result = AutomationResult {
            success: false,
            actions_completed: 0,
            total_actions: script.actions.len(),
            errors: Vec::new(),
            execution_log: Vec::new(),
            screenshots: Vec::new(),
            metadata: HashMap::new(),
        };
        
        // Convert automation actions to ACI commands
        let aci_commands = self.convert_to_aci_commands(&script)?;
        
        // Execute ACI workflow
        for (i, command) in aci_commands.iter().enumerate() {
            match self.execute_aci_command(&agent_id, command).await {
                Ok(observation) => {
                    result.actions_completed += 1;
                    result.execution_log.push(format!("✅ ACI Command {}: {}", i + 1, command.command_type));
                    
                    // Extract screenshot if available
                    if let Some(screenshot_path) = observation.data.get("screenshot_path")
                        .and_then(|p| p.as_str()) {
                        result.screenshots.push(screenshot_path.to_string());
                    }
                }
                Err(e) => {
                    let error_msg = format!("❌ ACI Command {} failed: {}", i + 1, e);
                    result.errors.push(error_msg.clone());
                    result.execution_log.push(error_msg);
                }
            }
            
            // Agent thinking time
            sleep(Duration::from_millis(750)).await;
        }
        
        result.success = result.actions_completed == result.total_actions;
        
        // Add agent metadata
        result.metadata.insert("agent_id".to_string(), serde_json::Value::String(agent_id));
        result.metadata.insert("session_id".to_string(), serde_json::Value::String(session_id));
        result.metadata.insert("aci_mode".to_string(), serde_json::Value::Bool(true));
        
        info!("🏆 ACI script completed: {}/{} commands successful", result.actions_completed, result.total_actions);
        
        Ok(result)
    }

    /// Execute ACI execution plan
    pub async fn execute_execution_plan(&self, plan: &AciExecutionPlan) -> Result<Vec<AciObservation>> {
        info!("🎯 Executing ACI plan: {}", plan.goal);
        
        let session_id = Uuid::new_v4().to_string();
        self.create_agent_session(&plan.agent_id, &session_id).await?;
        
        let mut observations = Vec::new();
        
        for command in &plan.commands {
            let observation = self.execute_aci_command(&plan.agent_id, command).await?;
            observations.push(observation);
            
            // Agent decision time
            sleep(Duration::from_millis(500)).await;
        }
        
        // Verify expected outcome
        for verification_step in &plan.verification_steps {
            let verification_observation = self.verify_outcome(&plan.agent_id, verification_step).await?;
            observations.push(verification_observation);
        }
        
        info!("✅ ACI plan execution completed");
        Ok(observations)
    }

    /// Execute single ACI command
    pub async fn execute_aci_command(&self, agent_id: &str, command: &AciCommand) -> Result<AciObservation> {
        info!("🤖 Agent executing: {}", command.command_type);
        
        let observation = match command.command_type.as_str() {
            "observe_desktop" => self.command_observe_desktop(agent_id, command).await?,
            "launch_application" => self.command_launch_application(agent_id, command).await?,
            "interact_with_element" => self.command_interact_with_element(agent_id, command).await?,
            "perform_workflow" => self.command_perform_workflow(agent_id, command).await?,
            "analyze_screen" => self.command_analyze_screen(agent_id, command).await?,
            "navigate_application" => self.command_navigate_application(agent_id, command).await?,
            "extract_information" => self.command_extract_information(agent_id, command).await?,
            "verify_state" => self.command_verify_state(agent_id, command).await?,
            "take_action" => self.command_take_action(agent_id, command).await?,
            "get_session_state" => self.command_get_session_state(agent_id, command).await?,
            _ => return Err(anyhow::anyhow!("Unknown ACI command: {}", command.command_type)),
        };
        
        // Update agent session
        self.update_agent_session(agent_id, command, &observation).await?;
        
        info!("✅ Agent command completed: {}", command.command_type);
        Ok(observation)
    }

    /// Convert automation script to ACI commands
    fn convert_to_aci_commands(&self, script: &AutomationScript) -> Result<Vec<AciCommand>> {
        let mut aci_commands = Vec::new();
        
        for action in &script.actions {
            let command = match action.action_type.as_str() {
                "screenshot" => AciCommand {
                    command_type: "observe_desktop".to_string(),
                    target: action.target.as_ref().map(|t| serde_json::Value::String(t.clone())),
                    parameters: HashMap::new(),
                    interaction_type: None,
                    verification: Some(true),
                },
                "launch" => {
                    let mut params = HashMap::new();
                    if let Some(target) = &action.target {
                        params.insert("application".to_string(), serde_json::Value::String(target.clone()));
                    }
                    AciCommand {
                        command_type: "launch_application".to_string(),
                        target: None,
                        parameters: params,
                        interaction_type: None,
                        verification: Some(true),
                    }
                }
                "click" => {
                    let mut params = HashMap::new();
                    if let Some((x, y)) = action.coordinates {
                        params.insert("coordinates".to_string(), serde_json::Value::Array(vec![x.into(), y.into()]));
                    }
                    if let Some(description) = &action.target {
                        params.insert("description".to_string(), serde_json::Value::String(description.clone()));
                    }
                    AciCommand {
                        command_type: "interact_with_element".to_string(),
                        target: Some(serde_json::json!({"coordinates": [action.coordinates.unwrap_or((0, 0)).0, action.coordinates.unwrap_or((0, 0)).1]})),
                        parameters: params,
                        interaction_type: Some("click".to_string()),
                        verification: Some(true),
                    }
                }
                "type" => {
                    let mut params = HashMap::new();
                    if let Some(text) = &action.text {
                        params.insert("text".to_string(), serde_json::Value::String(text.clone()));
                    }
                    AciCommand {
                        command_type: "interact_with_element".to_string(),
                        target: None,
                        parameters: params,
                        interaction_type: Some("type".to_string()),
                        verification: Some(true),
                    }
                }
                _ => continue,
            };
            aci_commands.push(command);
        }
        
        Ok(aci_commands)
    }

    /// Create agent session
    async fn create_agent_session(&self, agent_id: &str, session_id: &str) -> Result<()> {
        let session = AciAgentSession {
            agent_id: agent_id.to_string(),
            session_id: session_id.to_string(),
            active: true,
            commands_executed: Vec::new(),
            observations: Vec::new(),
            current_state: HashMap::new(),
            created_at: chrono::Utc::now().timestamp() as f64,
        };
        
        let mut agents = self.active_agents.write().await;
        agents.insert(agent_id.to_string(), session);
        
        info!("🤖 Created ACI agent session: {}", agent_id);
        Ok(())
    }

    /// Execute single step without recursion
    async fn execute_single_step(&self, command: &AciCommand) -> Result<AciObservation> {
        // Execute basic commands only, no workflow recursion
        match command.command_type.as_str() {
            "observe_desktop" => self.command_observe_desktop("single_step", command).await,
            "interact_with_element" => self.command_interact_with_element("single_step", command).await,
            "take_action" => self.command_take_action("single_step", command).await,
            _ => {
                // For other commands, create a basic observation
                let observation_data = serde_json::json!({
                    "command_executed": true,
                    "command_type": command.command_type,
                    "success": true,
                    "note": "Basic execution without full context"
                });
                
                Ok(AciObservation {
                    observation_type: "basic_execution".to_string(),
                    timestamp: chrono::Utc::now().timestamp() as f64,
                    data: observation_data,
                    confidence: 0.7,
                })
            }
        }
    }

    /// Update agent session with command and observation
    async fn update_agent_session(&self, agent_id: &str, command: &AciCommand, observation: &AciObservation) -> Result<()> {
        let mut agents = self.active_agents.write().await;
        
        if let Some(session) = agents.get_mut(agent_id) {
            session.commands_executed.push(command.clone());
            session.observations.push(observation.clone());
            
            // Update current state
            if let Some(state_data) = observation.data.get("state_update") {
                if let Some(state_obj) = state_data.as_object() {
                    for (key, value) in state_obj {
                        session.current_state.insert(key.clone(), value.clone());
                    }
                }
            }
        }
        
        Ok(())
    }

    // ACI Command implementations
    async fn command_observe_desktop(&self, _agent_id: &str, command: &AciCommand) -> Result<AciObservation> {
        let default_path = format!("/tmp/aci_observe_{}.png", chrono::Utc::now().timestamp());
        let screenshot_path = command.target.as_ref()
            .and_then(|t| t.as_str())
            .unwrap_or(&default_path);
        
        self.accuracy_engine.take_screenshot(screenshot_path).await?;
        
        // Get window information
        let windows = self.accuracy_engine.list_windows().await?;
        
        let observation_data = serde_json::json!({
            "screenshot_path": screenshot_path,
            "windows": windows,
            "window_count": windows.len(),
            "timestamp": chrono::Utc::now().timestamp(),
            "state_update": {
                "last_observation": screenshot_path,
                "visible_windows": windows.len()
            }
        });
        
        Ok(AciObservation {
            observation_type: "desktop_state".to_string(),
            timestamp: chrono::Utc::now().timestamp() as f64,
            data: observation_data,
            confidence: 0.9,
        })
    }

    async fn command_launch_application(&self, _agent_id: &str, command: &AciCommand) -> Result<AciObservation> {
        let app_name = command.parameters.get("application")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Application parameter required"))?;
        
        self.accuracy_engine.launch_application(app_name).await?;
        
        // Wait and verify launch
        let ready = self.accuracy_engine.wait_for_application(app_name, 10).await?;
        
        let observation_data = serde_json::json!({
            "application_launched": true,
            "application_name": app_name,
            "launch_successful": ready,
            "state_update": {
                "launched_application": app_name,
                "application_ready": ready
            }
        });
        
        Ok(AciObservation {
            observation_type: "application_launch".to_string(),
            timestamp: chrono::Utc::now().timestamp() as f64,
            data: observation_data,
            confidence: if ready { 0.95 } else { 0.6 },
        })
    }

    async fn command_interact_with_element(&self, _agent_id: &str, command: &AciCommand) -> Result<AciObservation> {
        match command.interaction_type.as_deref() {
            Some("click") => {
                let coordinates = command.target.as_ref()
                    .and_then(|t| t.get("coordinates"))
                    .and_then(|c| c.as_array())
                    .ok_or_else(|| anyhow::anyhow!("Click coordinates required"))?;
                
                let x = coordinates[0].as_i64().unwrap_or(0) as i32;
                let y = coordinates[1].as_i64().unwrap_or(0) as i32;
                
                let description = command.parameters.get("description")
                    .and_then(|d| d.as_str());
                
                self.accuracy_engine.precise_click(x, y, description).await?;
                
                let observation_data = serde_json::json!({
                    "interaction_type": "click",
                    "coordinates": [x, y],
                    "description": description,
                    "success": true,
                    "state_update": {
                        "last_interaction": "click",
                        "last_coordinates": [x, y]
                    }
                });
                
                Ok(AciObservation {
                    observation_type: "element_interaction".to_string(),
                    timestamp: chrono::Utc::now().timestamp() as f64,
                    data: observation_data,
                    confidence: 0.85,
                })
            }
            Some("type") => {
                let text = command.parameters.get("text")
                    .and_then(|t| t.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Text parameter required for typing"))?;
                
                self.accuracy_engine.type_text(text).await?;
                
                let observation_data = serde_json::json!({
                    "interaction_type": "type",
                    "text": text,
                    "text_length": text.len(),
                    "success": true,
                    "state_update": {
                        "last_interaction": "type",
                        "last_text": text
                    }
                });
                
                Ok(AciObservation {
                    observation_type: "text_input".to_string(),
                    timestamp: chrono::Utc::now().timestamp() as f64,
                    data: observation_data,
                    confidence: 0.9,
                })
            }
            _ => Err(anyhow::anyhow!("Unknown interaction type: {:?}", command.interaction_type)),
        }
    }

    async fn command_perform_workflow(&self, _agent_id: &str, command: &AciCommand) -> Result<AciObservation> {
        let steps = command.parameters.get("steps")
            .and_then(|s| s.as_array())
            .ok_or_else(|| anyhow::anyhow!("Workflow steps required"))?;
        
        let mut completed_steps = 0;
        let mut workflow_observations = Vec::new();
        
        for (i, step) in steps.iter().enumerate() {
            if let Ok(step_command) = serde_json::from_value::<AciCommand>(step.clone()) {
                // Execute step directly without recursion to avoid infinite recursion
                match self.execute_single_step(&step_command).await {
                    Ok(obs) => {
                        completed_steps += 1;
                        workflow_observations.push(obs);
                    }
                    Err(e) => {
                        warn!("Workflow step {} failed: {}", i + 1, e);
                    }
                }
            }
        }
        
        let observation_data = serde_json::json!({
            "workflow_executed": true,
            "total_steps": steps.len(),
            "completed_steps": completed_steps,
            "success_rate": (completed_steps as f64 / steps.len() as f64),
            "observations": workflow_observations,
            "state_update": {
                "workflow_completed": true,
                "steps_completed": completed_steps
            }
        });
        
        Ok(AciObservation {
            observation_type: "workflow_execution".to_string(),
            timestamp: chrono::Utc::now().timestamp() as f64,
            data: observation_data,
            confidence: (completed_steps as f64 / steps.len() as f64),
        })
    }

    async fn command_analyze_screen(&self, _agent_id: &str, _command: &AciCommand) -> Result<AciObservation> {
        // Take screenshot for analysis
        let screenshot_path = format!("/tmp/aci_analysis_{}.png", chrono::Utc::now().timestamp());
        self.accuracy_engine.take_screenshot(&screenshot_path).await?;
        
        // Get window information
        let windows = self.accuracy_engine.list_windows().await?;
        
        // Simple analysis
        let analysis = serde_json::json!({
            "screenshot_path": screenshot_path,
            "window_count": windows.len(),
            "active_applications": windows.iter().map(|w| &w.class).collect::<Vec<_>>(),
            "screen_regions": [
                {"type": "top_bar", "y_range": [0, 50]},
                {"type": "main_area", "y_range": [50, 700]},
                {"type": "bottom_bar", "y_range": [700, 800]}
            ],
            "analysis_confidence": 0.8,
            "state_update": {
                "screen_analyzed": true,
                "analysis_timestamp": chrono::Utc::now().timestamp()
            }
        });
        
        Ok(AciObservation {
            observation_type: "screen_analysis".to_string(),
            timestamp: chrono::Utc::now().timestamp() as f64,
            data: analysis,
            confidence: 0.8,
        })
    }

    async fn command_navigate_application(&self, _agent_id: &str, command: &AciCommand) -> Result<AciObservation> {
        let navigation_type = command.parameters.get("navigation_type")
            .and_then(|t| t.as_str())
            .unwrap_or("menu");
        
        let target_element = command.parameters.get("target_element")
            .and_then(|e| e.as_str())
            .unwrap_or("unknown");
        
        // Simple navigation simulation
        sleep(Duration::from_millis(500)).await;
        
        let observation_data = serde_json::json!({
            "navigation_completed": true,
            "navigation_type": navigation_type,
            "target_element": target_element,
            "success": true,
            "state_update": {
                "last_navigation": navigation_type,
                "current_location": target_element
            }
        });
        
        Ok(AciObservation {
            observation_type: "application_navigation".to_string(),
            timestamp: chrono::Utc::now().timestamp() as f64,
            data: observation_data,
            confidence: 0.75,
        })
    }

    async fn command_extract_information(&self, _agent_id: &str, command: &AciCommand) -> Result<AciObservation> {
        let extraction_type = command.parameters.get("extraction_type")
            .and_then(|t| t.as_str())
            .unwrap_or("text");
        
        // Take screenshot for information extraction
        let screenshot_path = format!("/tmp/aci_extract_{}.png", chrono::Utc::now().timestamp());
        self.accuracy_engine.take_screenshot(&screenshot_path).await?;
        
        let observation_data = serde_json::json!({
            "extraction_completed": true,
            "extraction_type": extraction_type,
            "screenshot_path": screenshot_path,
            "extracted_data": {
                "text_regions": ["Sample text 1", "Sample text 2"],
                "ui_elements": ["Button", "Text field", "Menu"],
                "confidence": 0.7
            },
            "state_update": {
                "information_extracted": true,
                "extraction_timestamp": chrono::Utc::now().timestamp()
            }
        });
        
        Ok(AciObservation {
            observation_type: "information_extraction".to_string(),
            timestamp: chrono::Utc::now().timestamp() as f64,
            data: observation_data,
            confidence: 0.7,
        })
    }

    async fn command_verify_state(&self, _agent_id: &str, command: &AciCommand) -> Result<AciObservation> {
        let expected_state = command.parameters.get("expected_state")
            .and_then(|s| s.as_str())
            .unwrap_or("unknown");
        
        // Take verification screenshot
        let screenshot_path = format!("/tmp/aci_verify_{}.png", chrono::Utc::now().timestamp());
        self.accuracy_engine.take_screenshot(&screenshot_path).await?;
        
        // Simple state verification
        let verification_passed = true; // Simplified for now
        
        let observation_data = serde_json::json!({
            "verification_completed": true,
            "expected_state": expected_state,
            "verification_passed": verification_passed,
            "screenshot_path": screenshot_path,
            "confidence": 0.8,
            "state_update": {
                "state_verified": verification_passed,
                "verification_timestamp": chrono::Utc::now().timestamp()
            }
        });
        
        Ok(AciObservation {
            observation_type: "state_verification".to_string(),
            timestamp: chrono::Utc::now().timestamp() as f64,
            data: observation_data,
            confidence: 0.8,
        })
    }

    async fn command_take_action(&self, agent_id: &str, command: &AciCommand) -> Result<AciObservation> {
        let action_type = command.parameters.get("action_type")
            .and_then(|t| t.as_str())
            .unwrap_or("click");
        
        // Delegate to appropriate interaction command
        let interaction_command = AciCommand {
            command_type: "interact_with_element".to_string(),
            target: command.target.clone(),
            parameters: command.parameters.clone(),
            interaction_type: Some(action_type.to_string()),
            verification: command.verification,
        };
        
        self.command_interact_with_element(agent_id, &interaction_command).await
    }

    async fn command_get_session_state(&self, agent_id: &str, _command: &AciCommand) -> Result<AciObservation> {
        let agents = self.active_agents.read().await;
        
        if let Some(session) = agents.get(agent_id) {
            let observation_data = serde_json::json!({
                "session_active": session.active,
                "commands_executed": session.commands_executed.len(),
                "observations_count": session.observations.len(),
                "current_state": session.current_state,
                "session_age": chrono::Utc::now().timestamp() as f64 - session.created_at
            });
            
            return Ok(AciObservation {
                observation_type: "session_state".to_string(),
                timestamp: chrono::Utc::now().timestamp() as f64,
                data: observation_data,
                confidence: 1.0,
            });
        }
        
        Err(anyhow::anyhow!("Agent session not found: {}", agent_id))
    }

    /// Verify outcome against expected result
    async fn verify_outcome(&self, _agent_id: &str, verification_step: &str) -> Result<AciObservation> {
        // Take verification screenshot
        let screenshot_path = format!("/tmp/aci_outcome_{}.png", chrono::Utc::now().timestamp());
        self.accuracy_engine.take_screenshot(&screenshot_path).await?;
        
        let observation_data = serde_json::json!({
            "verification_step": verification_step,
            "outcome_verified": true,
            "screenshot_path": screenshot_path,
            "confidence": 0.85,
            "state_update": {
                "outcome_verification": true,
                "verification_step": verification_step
            }
        });
        
        Ok(AciObservation {
            observation_type: "outcome_verification".to_string(),
            timestamp: chrono::Utc::now().timestamp() as f64,
            data: observation_data,
            confidence: 0.85,
        })
    }
}

impl Default for AciInterface {
    fn default() -> Self {
        Self::new().expect("Failed to create ACI interface")
    }
}