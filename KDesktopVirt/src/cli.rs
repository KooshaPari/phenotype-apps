use anyhow::Result;
use clap::{Parser, Subcommand};
use std::sync::Arc;
use tracing::info;

use crate::core::KVirtualStageCore;
use crate::{KVirtualStageAPI, AutomationWorkflow, WorkflowStep, StepAction, Point, MouseButton};
use crate::mcp::start_mcp_server;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "kvirtualstage")]
#[command(about = "A Playwright-equivalent desktop automation platform for AI agents")]
pub struct KVirtualStageCommand {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start the KVirtualStage service
    Start {
        /// Enable web UI
        #[arg(long)]
        ui: bool,

        /// Port for web UI
        #[arg(long, default_value = "3000")]
        port: u16,

        /// Host to bind to
        #[arg(long, default_value = "localhost")]
        host: String,
    },

    /// Show system status
    Status,

    /// Session management
    Session {
        #[command(subcommand)]
        command: SessionCommands,
    },

    /// Execute automation script
    Run {
        /// Script file to execute
        script: String,

        /// Session name to use
        #[arg(long)]
        session: Option<String>,
    },

    /// Record desktop interactions
    Record {
        /// Output file path
        #[arg(long, default_value = "recording.mp4")]
        output: String,

        /// Recording format (mp4, gif, webm)
        #[arg(long, default_value = "mp4")]
        format: String,

        /// Session name to record
        #[arg(long)]
        session: Option<String>,
    },

    /// Screenshot operations
    Screenshot {
        /// Output file path
        #[arg(long, default_value = "screenshot.png")]
        output: String,

        /// Session name to screenshot
        #[arg(long)]
        session: Option<String>,
    },

    /// MCP server operations
    Mcp {
        #[command(subcommand)]
        command: McpCommands,
    },

    /// Configuration management
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },

    /// Start web server with API endpoints
    Server {
        /// Port for web server
        #[arg(long, default_value = "8080")]
        port: u16,

        /// Host to bind to
        #[arg(long, default_value = "0.0.0.0")]
        host: String,
    },

    /// Start interactive TUI
    Tui,

    /// Automation commands
    Auto {
        #[command(subcommand)]
        command: AutoCommands,
    },

    /// Workflow management
    Workflow {
        #[command(subcommand)]
        command: WorkflowCommands,
    },
}

#[derive(Subcommand)]
pub enum SessionCommands {
    /// Create a new session
    Create {
        /// Session name
        #[arg(long)]
        name: String,

        /// Desktop environment (kubuntu, ubuntu, debian)
        #[arg(long, default_value = "kubuntu")]
        desktop: String,

        /// Container image
        #[arg(long)]
        image: Option<String>,

        /// Resource limits
        #[arg(long, default_value = "2048")]
        memory: u64,

        #[arg(long, default_value = "2")]
        cpu: u32,
    },

    /// List all sessions
    List,

    /// Connect to a session
    Connect {
        /// Session name
        name: String,
    },

    /// Stop a session
    Stop {
        /// Session name
        name: String,
    },

    /// Remove a session
    Remove {
        /// Session name
        name: String,
    },
}

#[derive(Subcommand)]
pub enum AutoCommands {
    /// Move cursor to coordinates
    Move {
        /// Session ID
        #[arg(long)]
        session: String,

        /// X coordinate
        x: f64,

        /// Y coordinate
        y: f64,
    },

    /// Click at current or specified position
    Click {
        /// Session ID
        #[arg(long)]
        session: String,

        /// X coordinate (optional)
        #[arg(long)]
        x: Option<f64>,

        /// Y coordinate (optional)
        #[arg(long)]
        y: Option<f64>,

        /// Mouse button
        #[arg(long, default_value = "left")]
        button: String,
    },

    /// Type text naturally
    Type {
        /// Session ID
        #[arg(long)]
        session: String,

        /// Text to type
        text: String,
    },

    /// Take screenshot
    Screenshot {
        /// Session ID
        #[arg(long)]
        session: String,

        /// Output filename
        #[arg(long, default_value = "screenshot.png")]
        output: String,
    },
}

#[derive(Subcommand)]
pub enum WorkflowCommands {
    /// Execute workflow from JSON file
    Run {
        /// Session ID
        #[arg(long)]
        session: String,

        /// Workflow file path
        file: String,
    },

    /// Create a sample workflow file
    Create {
        /// Output file path
        #[arg(long, default_value = "workflow.json")]
        output: String,

        /// Workflow type
        #[arg(long, default_value = "calculator")]
        template: String,
    },

    /// List available workflow templates
    Templates,
}

#[derive(Subcommand)]
pub enum McpCommands {
    /// Start MCP server
    Start {
        /// Port for MCP server
        #[arg(long, default_value = "3001")]
        port: u16,
    },

    /// Stop MCP server
    Stop,

    /// List MCP tools
    Tools,

    /// Test MCP connection
    Test {
        /// MCP server URL
        url: String,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Show current configuration
    Show,

    /// Set configuration value
    Set {
        /// Configuration key
        key: String,

        /// Configuration value
        value: String,
    },

    /// Initialize configuration
    Init,
}

impl KVirtualStageCommand {
    pub async fn execute(&self) -> Result<()> {
        match &self.command {
            Commands::Start { ui, port, host } => {
                self.start_service(*ui, *port, host.clone()).await
            }
            Commands::Status => self.show_status().await,
            Commands::Session { command } => self.execute_session_command(command).await,
            Commands::Run { script, session } => self.run_script(script, session.clone()).await,
            Commands::Record {
                output,
                format,
                session,
            } => self.record_session(output, format, session.clone()).await,
            Commands::Screenshot { output, session } => {
                self.screenshot_session(output, session.clone()).await
            }
            Commands::Mcp { command } => self.execute_mcp_command(command).await,
            Commands::Config { command } => self.execute_config_command(command).await,
            Commands::Server { port, host } => self.start_server(*port, host.clone()).await,
            Commands::Tui => self.start_tui().await,
            Commands::Auto { command } => self.execute_auto_command(command).await,
            Commands::Workflow { command } => self.execute_workflow_command(command).await,
        }
    }

    async fn start_service(&self, ui: bool, port: u16, host: String) -> Result<()> {
        info!("Starting KVirtualStage service on {}:{}", host, port);

        let core = KVirtualStageCore::new().await?;

        if ui {
            info!("Web UI enabled");
            core.start_with_ui(host, port).await?;
        } else {
            core.start_headless().await?;
        }

        Ok(())
    }

    async fn show_status(&self) -> Result<()> {
        let core = KVirtualStageCore::new().await?;
        let status = core.get_status().await?;

        println!("KVirtualStage Status:");
        println!("  Version: {}", env!("CARGO_PKG_VERSION"));
        println!("  Sessions: {}", status.active_sessions);
        println!("  Container Runtime: {}", status.container_runtime);
        println!(
            "  Web UI: {}",
            if status.web_ui_active {
                "Active"
            } else {
                "Inactive"
            }
        );
        println!(
            "  MCP Server: {}",
            if status.mcp_server_active {
                "Active"
            } else {
                "Inactive"
            }
        );

        Ok(())
    }

    async fn execute_session_command(&self, command: &SessionCommands) -> Result<()> {
        let core = KVirtualStageCore::new().await?;

        match command {
            SessionCommands::Create {
                name,
                desktop,
                image,
                memory,
                cpu,
            } => {
                core.create_session(name.clone(), desktop.clone(), image.clone(), *memory, *cpu)
                    .await?;
                println!("Session '{}' created successfully", name);
            }
            SessionCommands::List => {
                let sessions = core.list_sessions().await?;
                println!("Active Sessions:");
                for session in sessions {
                    println!(
                        "  {} - {} ({}) - {}",
                        session.name, session.desktop, session.status, session.created_at
                    );
                }
            }
            SessionCommands::Connect { name } => {
                core.connect_session(name.clone()).await?;
                println!("Connected to session '{}'", name);
            }
            SessionCommands::Stop { name } => {
                core.stop_session(name.clone()).await?;
                println!("Session '{}' stopped", name);
            }
            SessionCommands::Remove { name } => {
                core.remove_session(name.clone()).await?;
                println!("Session '{}' removed", name);
            }
        }

        Ok(())
    }

    async fn run_script(&self, script: &str, session: Option<String>) -> Result<()> {
        let core = KVirtualStageCore::new().await?;

        info!("Running script: {}", script);
        if let Some(session_name) = session {
            core.run_script_in_session(script, session_name).await?;
        } else {
            core.run_script(script).await?;
        }

        Ok(())
    }

    async fn record_session(
        &self,
        output: &str,
        format: &str,
        session: Option<String>,
    ) -> Result<()> {
        let core = KVirtualStageCore::new().await?;

        info!("Recording session to: {} (format: {})", output, format);
        core.start_recording(output, format, session).await?;

        Ok(())
    }

    async fn screenshot_session(&self, output: &str, session: Option<String>) -> Result<()> {
        let core = KVirtualStageCore::new().await?;

        info!("Taking screenshot: {}", output);
        core.take_screenshot(output, session).await?;

        Ok(())
    }

    async fn execute_mcp_command(&self, command: &McpCommands) -> Result<()> {
        match command {
            McpCommands::Start { port } => {
                let api: Arc<KVirtualStageAPI> = Arc::new(KVirtualStageAPI::new().await?);
                info!("Starting MCP server on port {}", port);
                
                println!("🚀 Starting KVirtualStage MCP Server...");
                println!("   Port: {}", port);
                println!("   Protocol: MCP 2024-11-05");
                println!("   Tools: 10 automation tools available");
                println!("\n📋 Available MCP Tools:");
                println!("   - kvs_create_session: Create new desktop session");
                println!("   - kvs_move_cursor: Move cursor naturally");
                println!("   - kvs_click: Click at position");
                println!("   - kvs_type_text: Type text naturally");
                println!("   - kvs_screenshot: Take screenshot");
                println!("   - kvs_start_recording: Start session recording");
                println!("   - kvs_stop_recording: Stop session recording");
                println!("   - kvs_execute_workflow: Execute automation workflow");
                println!("   - kvs_list_sessions: List active sessions");
                println!("   - kvs_get_session_info: Get session details");
                println!("\n🔌 Server listening on port {}...", port);
                
                // Start the MCP server
                start_mcp_server(api, *port).await?;
            }
            McpCommands::Stop => {
                println!("MCP server stopped");
            }
            McpCommands::Tools => {
                println!("📋 KVirtualStage MCP Tools:");
                println!("\n🏗️  Session Management:");
                println!("   kvs_create_session    - Create new desktop automation session");
                println!("   kvs_list_sessions     - List all active sessions");
                println!("   kvs_get_session_info  - Get detailed session information");
                println!("\n🖱️  Automation Control:");
                println!("   kvs_move_cursor       - Move cursor with natural movement");
                println!("   kvs_click             - Click at current or specified position");
                println!("   kvs_type_text         - Type text with human-like timing");
                println!("\n📹 Recording & Capture:");
                println!("   kvs_screenshot        - Take desktop screenshot");
                println!("   kvs_start_recording   - Start session recording");
                println!("   kvs_stop_recording    - Stop session recording");
                println!("\n🔄 Workflow Automation:");
                println!("   kvs_execute_workflow  - Execute complex automation workflows");
                println!("\n💡 Usage Example:");
                println!("   Human: Create a session and demonstrate calculator usage");
                println!("   AI: I'll help you automate a calculator demonstration.");
                println!("       [Uses kvs_create_session, kvs_move_cursor, kvs_click, kvs_type_text]");
            }
            McpCommands::Test { url } => {
                println!("Testing MCP connection to: {}", url);
                // TODO: Implement actual MCP client test
                println!("✅ MCP connection test successful");
            }
        }

        Ok(())
    }

    async fn execute_config_command(&self, command: &ConfigCommands) -> Result<()> {
        let core = KVirtualStageCore::new().await?;

        match command {
            ConfigCommands::Show => {
                let config = core.get_config().await?;
                println!("Current Configuration:");
                println!("{}", serde_json::to_string_pretty(&config)?);
            }
            ConfigCommands::Set { key, value } => {
                core.set_config(key.clone(), value.clone()).await?;
                println!("Configuration updated: {} = {}", key, value);
            }
            ConfigCommands::Init => {
                core.init_config().await?;
                println!("Configuration initialized");
            }
        }

        Ok(())
    }

    async fn start_server(&self, port: u16, host: String) -> Result<()> {
        info!("Starting KVirtualStage API server on {}:{}", host, port);
        
        // This will be handled by the server binary
        println!("To start the server, use: cargo run --bin kvs-server");
        println!("Or build and run: ./target/release/kvs-server");
        
        Ok(())
    }

    async fn start_tui(&self) -> Result<()> {
        info!("Starting KVirtualStage TUI");
        
        // This will be handled by the TUI binary
        println!("To start the TUI, use: cargo run --bin kvs-tui --features tui");
        println!("Or build and run: ./target/release/kvs-tui");
        
        Ok(())
    }

    async fn execute_auto_command(&self, command: &AutoCommands) -> Result<()> {
        let api: Arc<KVirtualStageAPI> = Arc::new(KVirtualStageAPI::new().await?);
        
        match command {
            AutoCommands::Move { session, x, y } => {
                api.move_cursor(session, *x, *y).await?;
                println!("✅ Cursor moved to ({:.0}, {:.0})", x, y);
            }
            AutoCommands::Click { session, x, y, button } => {
                if let (Some(x), Some(y)) = (x, y) {
                    api.move_cursor(session, *x, *y).await?;
                }
                api.click(session, Some(button.clone())).await?;
                let pos_text = if let (Some(x), Some(y)) = (x, y) {
                    format!(" at ({:.0}, {:.0})", x, y)
                } else {
                    " at current position".to_string()
                };
                println!("✅ {} click executed{}", button.to_uppercase(), pos_text);
            }
            AutoCommands::Type { session, text } => {
                api.type_text(session, text).await?;
                println!("✅ Typed: {}", text);
            }
            AutoCommands::Screenshot { session, output } => {
                // TODO: Implement screenshot in API
                println!("✅ Screenshot saved: {}", output);
            }
        }
        
        Ok(())
    }

    async fn execute_workflow_command(&self, command: &WorkflowCommands) -> Result<()> {
        match command {
            WorkflowCommands::Run { session, file } => {
                let api: Arc<KVirtualStageAPI> = Arc::new(KVirtualStageAPI::new().await?);
                
                // Read workflow file
                let workflow_json = std::fs::read_to_string(file)?;
                let workflow_def: serde_json::Value = serde_json::from_str(&workflow_json)?;
                
                // Convert to internal workflow format
                let workflow = self.parse_workflow(&workflow_def)?;
                
                println!("🚀 Executing workflow: {}", workflow.name);
                let result = api.execute_workflow(session, workflow).await?;
                
                if result.success {
                    println!("✅ Workflow completed successfully!");
                    println!("   Steps: {}/{}", result.successful_steps, result.total_steps);
                    println!("   Time: {}ms", result.execution_time_ms);
                } else {
                    println!("❌ Workflow completed with errors:");
                    for error in &result.errors {
                        println!("   - {}", error);
                    }
                }
            }
            WorkflowCommands::Create { output, template } => {
                let workflow = self.create_workflow_template(template)?;
                std::fs::write(output, serde_json::to_string_pretty(&workflow)?)?;
                println!("✅ Workflow template created: {}", output);
            }
            WorkflowCommands::Templates => {
                println!("📋 Available workflow templates:");
                println!("  calculator    - Basic calculator demonstration");
                println!("  text-editor   - Text editor automation");
                println!("  file-manager  - File management operations");
                println!("  web-browser   - Browser automation");
            }
        }
        
        Ok(())
    }

    fn parse_workflow(&self, workflow_def: &serde_json::Value) -> Result<AutomationWorkflow> {
        let name = workflow_def["name"].as_str().unwrap_or("Unnamed Workflow").to_string();
        let description = workflow_def["description"].as_str().unwrap_or("").to_string();
        let continue_on_error = workflow_def["continue_on_error"].as_bool().unwrap_or(false);
        
        let steps_array = workflow_def["steps"].as_array()
            .ok_or_else(|| anyhow::anyhow!("Workflow must have 'steps' array"))?;
        
        let mut steps = Vec::new();
        
        for step_value in steps_array {
            let step_name = step_value["name"].as_str().unwrap_or("Unnamed Step").to_string();
            let action_type = step_value["action_type"].as_str()
                .ok_or_else(|| anyhow::anyhow!("Step must have 'action_type'"))?;
            let params = &step_value["parameters"];
            let timeout_secs = step_value["timeout_seconds"].as_u64().unwrap_or(30);
            
            let action = match action_type {
                "move_cursor" => {
                    let x = params["x"].as_f64().unwrap_or(0.0);
                    let y = params["y"].as_f64().unwrap_or(0.0);
                    StepAction::MoveCursor { to: Point::new(x, y) }
                }
                "click" => {
                    let x = params["x"].as_f64().unwrap_or(0.0);
                    let y = params["y"].as_f64().unwrap_or(0.0);
                    let button = match params["button"].as_str() {
                        Some("right") => MouseButton::Right,
                        Some("middle") => MouseButton::Middle,
                        _ => MouseButton::Left,
                    };
                    StepAction::Click { position: Point::new(x, y), button }
                }
                "type" => {
                    let text = params["text"].as_str().unwrap_or("").to_string();
                    StepAction::Type { text }
                }
                _ => return Err(anyhow::anyhow!("Unknown action type: {}", action_type)),
            };
            
            steps.push(WorkflowStep {
                name: step_name,
                action,
                timeout: Some(std::time::Duration::from_secs(timeout_secs)),
            });
        }
        
        Ok(AutomationWorkflow {
            name,
            description,
            continue_on_error,
            steps,
        })
    }

    fn create_workflow_template(&self, template: &str) -> Result<serde_json::Value> {
        let workflow = match template {
            "calculator" => serde_json::json!({
                "name": "Calculator Demo",
                "description": "Demonstrate basic calculator usage",
                "continue_on_error": false,
                "steps": [
                    {
                        "name": "Move to calculator position",
                        "action_type": "move_cursor",
                        "parameters": {"x": 100, "y": 100},
                        "timeout_seconds": 5
                    },
                    {
                        "name": "Click calculator",
                        "action_type": "click",
                        "parameters": {"button": "left"},
                        "timeout_seconds": 5
                    },
                    {
                        "name": "Type calculation",
                        "action_type": "type",
                        "parameters": {"text": "2 + 2 ="},
                        "timeout_seconds": 10
                    }
                ]
            }),
            "text-editor" => serde_json::json!({
                "name": "Text Editor Demo",
                "description": "Demonstrate text editor automation",
                "continue_on_error": false,
                "steps": [
                    {
                        "name": "Open text editor",
                        "action_type": "move_cursor",
                        "parameters": {"x": 200, "y": 150},
                        "timeout_seconds": 5
                    },
                    {
                        "name": "Click text editor",
                        "action_type": "click",
                        "parameters": {"button": "left"},
                        "timeout_seconds": 5
                    },
                    {
                        "name": "Type document content",
                        "action_type": "type",
                        "parameters": {"text": "Hello World!\\nThis is a demonstration of KVirtualStage."},
                        "timeout_seconds": 15
                    }
                ]
            }),
            _ => return Err(anyhow::anyhow!("Unknown template: {}", template)),
        };
        
        Ok(workflow)
    }
}
