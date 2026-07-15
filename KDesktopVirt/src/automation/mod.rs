/*!
Comprehensive Automation Module for KVirtualStage
Supports multiple automation modes with pixel-perfect accuracy
*/

pub mod accuracy;
pub mod mcp_live;
pub mod aci_agent;
pub mod desktop_recording;
pub mod user_story_verification;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationMode {
    NormalScript,
    McpLive,
    AciAgent,
    DesktopRecording,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationAction {
    pub action_type: String,
    pub target: Option<String>,
    pub coordinates: Option<(i32, i32)>,
    pub text: Option<String>,
    pub delay: Option<f64>,
    pub verify: bool,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationScript {
    pub name: String,
    pub description: String,
    pub actions: Vec<AutomationAction>,
    pub mode: AutomationMode,
    pub settings: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationResult {
    pub success: bool,
    pub actions_completed: usize,
    pub total_actions: usize,
    pub errors: Vec<String>,
    pub execution_log: Vec<String>,
    pub screenshots: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStory {
    pub id: String,
    pub title: String,
    pub description: String,
    pub steps: Vec<UserStoryStep>,
    pub expected_outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStoryStep {
    pub step_number: usize,
    pub action: String,
    pub expected_result: String,
    pub screenshot_path: Option<String>,
    pub verification_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub step_number: usize,
    pub expected: String,
    pub actual: String,
    pub matches: bool,
    pub screenshot_path: String,
    pub confidence: f64,
}

/// Main automation platform supporting all modes
pub struct ComprehensiveAutomationPlatform {
    pub accuracy_engine: accuracy::AccuracyEngine,
    pub mcp_server: mcp_live::McpLiveServer,
    pub aci_interface: aci_agent::AciInterface,
    pub recording_engine: desktop_recording::RecordingEngine,
    pub verification_engine: user_story_verification::VerificationEngine,
    pub active_sessions: RwLock<HashMap<String, serde_json::Value>>,
}

impl ComprehensiveAutomationPlatform {
    pub fn new() -> Result<Self> {
        Ok(Self {
            accuracy_engine: accuracy::AccuracyEngine::new()?,
            mcp_server: mcp_live::McpLiveServer::new()?,
            aci_interface: aci_agent::AciInterface::new()?,
            recording_engine: desktop_recording::RecordingEngine::new()?,
            verification_engine: user_story_verification::VerificationEngine::new()?,
            active_sessions: RwLock::new(HashMap::new()),
        })
    }

    /// Execute automation with comprehensive verification
    pub async fn execute_with_verification(
        &self,
        script: AutomationScript,
        user_story: UserStory,
    ) -> Result<(AutomationResult, Vec<VerificationResult>)> {
        tracing::info!("🚀 Starting automation with user story verification");
        tracing::info!("User Story: {}", user_story.title);

        // Start recording if requested
        let recording_id = if matches!(script.mode, AutomationMode::DesktopRecording) {
            Some(Uuid::new_v4().to_string())
        } else {
            None
        };

        if let Some(ref id) = recording_id {
            self.recording_engine.start_recording(id, "automation_verification.mp4").await?;
        }

        // Execute the automation
        let automation_result = match script.mode {
            AutomationMode::NormalScript => {
                self.execute_normal_script(script).await?
            }
            AutomationMode::McpLive => {
                self.execute_mcp_live(script).await?
            }
            AutomationMode::AciAgent => {
                self.execute_aci_agent(script).await?
            }
            AutomationMode::DesktopRecording => {
                self.execute_with_recording(script).await?
            }
        };

        // Verify against user story
        let verification_results = self
            .verification_engine
            .verify_user_story(&user_story, &automation_result.screenshots)
            .await?;

        // Stop recording if active
        if let Some(id) = recording_id {
            self.recording_engine.stop_recording(&id).await?;
        }

        // Log verification results
        for result in &verification_results {
            if result.matches {
                tracing::info!("✅ Step {}: {}", result.step_number, result.expected);
            } else {
                tracing::warn!("❌ Step {}: Expected '{}', got '{}'", 
                    result.step_number, result.expected, result.actual);
            }
        }

        Ok((automation_result, verification_results))
    }

    async fn execute_normal_script(&self, script: AutomationScript) -> Result<AutomationResult> {
        tracing::info!("🐍 Executing normal script: {}", script.name);
        
        let mut result = AutomationResult {
            success: false,
            actions_completed: 0,
            total_actions: script.actions.len(),
            errors: Vec::new(),
            execution_log: Vec::new(),
            screenshots: Vec::new(),
            metadata: HashMap::new(),
        };

        for (i, action) in script.actions.iter().enumerate() {
            tracing::info!("Step {}/{}: {}", i + 1, script.actions.len(), action.action_type);

            match self.execute_single_action(action, i + 1).await {
                Ok(screenshot_path) => {
                    result.actions_completed += 1;
                    result.execution_log.push(format!("✅ Step {}: {}", i + 1, action.action_type));
                    if let Some(path) = screenshot_path {
                        result.screenshots.push(path);
                    }
                }
                Err(e) => {
                    let error_msg = format!("❌ Step {} failed: {}", i + 1, e);
                    result.errors.push(error_msg.clone());
                    result.execution_log.push(error_msg);
                }
            }

            // Natural delay between actions
            if let Some(delay) = action.delay {
                tokio::time::sleep(tokio::time::Duration::from_secs_f64(delay)).await;
            } else {
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }

        result.success = result.actions_completed == result.total_actions;
        Ok(result)
    }

    async fn execute_single_action(&self, action: &AutomationAction, step_number: usize) -> Result<Option<String>> {
        match action.action_type.as_str() {
            "screenshot" => {
                let path = action.target.clone().unwrap_or_else(|| {
                    format!("/tmp/kvs_step_{:02}.png", step_number)
                });
                self.accuracy_engine.take_screenshot(&path).await?;
                Ok(Some(path))
            }
            "launch" => {
                if let Some(target) = &action.target {
                    self.accuracy_engine.launch_application(target).await?;
                }
                // Take screenshot after launch
                let screenshot_path = format!("/tmp/kvs_step_{:02}_after_launch.png", step_number);
                self.accuracy_engine.take_screenshot(&screenshot_path).await?;
                Ok(Some(screenshot_path))
            }
            "click" => {
                if let Some((x, y)) = action.coordinates {
                    self.accuracy_engine.precise_click(x, y, action.target.as_deref()).await?;
                    // Take screenshot after click
                    let screenshot_path = format!("/tmp/kvs_step_{:02}_after_click.png", step_number);
                    self.accuracy_engine.take_screenshot(&screenshot_path).await?;
                    Ok(Some(screenshot_path))
                } else {
                    Err(anyhow::anyhow!("Click action requires coordinates"))
                }
            }
            "type" => {
                if let Some(text) = &action.text {
                    self.accuracy_engine.type_text(text).await?;
                    // Take screenshot after typing
                    let screenshot_path = format!("/tmp/kvs_step_{:02}_after_type.png", step_number);
                    self.accuracy_engine.take_screenshot(&screenshot_path).await?;
                    Ok(Some(screenshot_path))
                } else {
                    Err(anyhow::anyhow!("Type action requires text"))
                }
            }
            "wait" => {
                let delay = action.delay.unwrap_or(1.0);
                tokio::time::sleep(tokio::time::Duration::from_secs_f64(delay)).await;
                Ok(None)
            }
            _ => Err(anyhow::anyhow!("Unknown action type: {}", action.action_type)),
        }
    }

    async fn execute_mcp_live(&self, script: AutomationScript) -> Result<AutomationResult> {
        tracing::info!("🔧 Executing MCP live script: {}", script.name);
        self.mcp_server.execute_script(script).await
    }

    async fn execute_aci_agent(&self, script: AutomationScript) -> Result<AutomationResult> {
        tracing::info!("🤖 Executing ACI agent script: {}", script.name);
        self.aci_interface.execute_script(script).await
    }

    async fn execute_with_recording(&self, script: AutomationScript) -> Result<AutomationResult> {
        tracing::info!("📹 Executing with recording: {}", script.name);
        
        let recording_id = Uuid::new_v4().to_string();
        let output_file = format!("{}_recording.mp4", script.name.replace(' ', "_"));
        
        self.recording_engine.start_recording(&recording_id, &output_file).await?;
        
        let result = self.execute_normal_script(script).await?;
        
        self.recording_engine.stop_recording(&recording_id).await?;
        
        Ok(result)
    }

    /// Generate comprehensive video demonstration with verification
    pub async fn generate_verified_demo(&self, demo_name: &str) -> Result<Vec<VerificationResult>> {
        tracing::info!("🎬 Generating verified demo: {}", demo_name);

        // Define user story for the demo
        let user_story = UserStory {
            id: Uuid::new_v4().to_string(),
            title: format!("{} User Story", demo_name),
            description: "Comprehensive automation demonstration with verification".to_string(),
            steps: vec![
                UserStoryStep {
                    step_number: 1,
                    action: "Take initial desktop screenshot".to_string(),
                    expected_result: "Clean desktop visible".to_string(),
                    screenshot_path: None,
                    verification_data: None,
                },
                UserStoryStep {
                    step_number: 2,
                    action: "Launch calculator application".to_string(),
                    expected_result: "Calculator window opens and is visible".to_string(),
                    screenshot_path: None,
                    verification_data: None,
                },
                UserStoryStep {
                    step_number: 3,
                    action: "Perform calculation 8 × 7".to_string(),
                    expected_result: "Calculator shows result 56".to_string(),
                    screenshot_path: None,
                    verification_data: None,
                },
                UserStoryStep {
                    step_number: 4,
                    action: "Launch text editor".to_string(),
                    expected_result: "Text editor window opens".to_string(),
                    screenshot_path: None,
                    verification_data: None,
                },
                UserStoryStep {
                    step_number: 5,
                    action: "Type demonstration text".to_string(),
                    expected_result: "Text appears in editor window".to_string(),
                    screenshot_path: None,
                    verification_data: None,
                },
            ],
            expected_outcome: "Complete automation workflow with visible results".to_string(),
        };

        // Define automation script
        let script = AutomationScript {
            name: demo_name.to_string(),
            description: "Comprehensive automation demo with user story verification".to_string(),
            mode: AutomationMode::DesktopRecording,
            actions: vec![
                AutomationAction {
                    action_type: "screenshot".to_string(),
                    target: Some("/tmp/demo_01_desktop.png".to_string()),
                    coordinates: None,
                    text: None,
                    delay: Some(2.0),
                    verify: true,
                    metadata: None,
                },
                AutomationAction {
                    action_type: "launch".to_string(),
                    target: Some("galculator".to_string()),
                    coordinates: None,
                    text: None,
                    delay: Some(3.0),
                    verify: true,
                    metadata: None,
                },
                AutomationAction {
                    action_type: "click".to_string(),
                    target: Some("Calculator button 8".to_string()),
                    coordinates: Some((200, 250)), // Will be calculated dynamically
                    text: None,
                    delay: Some(1.0),
                    verify: true,
                    metadata: None,
                },
                AutomationAction {
                    action_type: "click".to_string(),
                    target: Some("Calculator button ×".to_string()),
                    coordinates: Some((250, 200)), // Will be calculated dynamically
                    text: None,
                    delay: Some(1.0),
                    verify: true,
                    metadata: None,
                },
                AutomationAction {
                    action_type: "click".to_string(),
                    target: Some("Calculator button 7".to_string()),
                    coordinates: Some((150, 250)), // Will be calculated dynamically
                    text: None,
                    delay: Some(1.0),
                    verify: true,
                    metadata: None,
                },
                AutomationAction {
                    action_type: "click".to_string(),
                    target: Some("Calculator button =".to_string()),
                    coordinates: Some((250, 300)), // Will be calculated dynamically
                    text: None,
                    delay: Some(2.0),
                    verify: true,
                    metadata: None,
                },
                AutomationAction {
                    action_type: "launch".to_string(),
                    target: Some("mousepad".to_string()),
                    coordinates: None,
                    text: None,
                    delay: Some(3.0),
                    verify: true,
                    metadata: None,
                },
                AutomationAction {
                    action_type: "type".to_string(),
                    target: Some("Demonstration text".to_string()),
                    coordinates: None,
                    text: Some("RUST AUTOMATION PLATFORM DEMO

Calculation Result: 8 × 7 = 56 ✓

This demonstrates:
• Pixel-perfect automation
• User story verification
• Real screenshot validation
• Professional Rust implementation".to_string()),
                    delay: Some(3.0),
                    verify: true,
                    metadata: None,
                },
                AutomationAction {
                    action_type: "screenshot".to_string(),
                    target: Some("/tmp/demo_final.png".to_string()),
                    coordinates: None,
                    text: None,
                    delay: Some(2.0),
                    verify: true,
                    metadata: None,
                },
            ],
            settings: None,
        };

        // Execute with verification
        let (_automation_result, verification_results) = self
            .execute_with_verification(script, user_story)
            .await?;

        tracing::info!("🏆 Demo completed with {} verification results", verification_results.len());
        Ok(verification_results)
    }
}

impl Default for ComprehensiveAutomationPlatform {
    fn default() -> Self {
        Self::new().expect("Failed to create automation platform")
    }
}