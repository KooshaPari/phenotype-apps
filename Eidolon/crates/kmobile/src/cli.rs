use anyhow::Result;
use tracing::{info, warn};

use crate::config::Config;
use crate::device_basic::{DeviceCommands, DeviceManager};
use crate::mcp::McpServer;
use crate::project::{ProjectCommands, ProjectManager};
use crate::simulator_basic::{SimulatorCommands, SimulatorManager};
use crate::testing::{TestCommands, TestRunner};

pub struct KMobileCli {
    config: Config,
    device_manager: DeviceManager,
    simulator_manager: SimulatorManager,
    project_manager: ProjectManager,
    test_runner: TestRunner,
}

impl KMobileCli {
    pub async fn new(config: Config) -> Result<Self> {
        let device_manager = DeviceManager::new(&config).await?;
        let simulator_manager = SimulatorManager::new(&config).await?;
        let project_manager = ProjectManager::new(&config).await?;
        let test_runner = TestRunner::new(&config).await?;

        Ok(Self {
            config,
            device_manager,
            simulator_manager,
            project_manager,
            test_runner,
        })
    }

    pub async fn init_project(&self, name: &str, template: Option<&str>) -> Result<()> {
        info!("Initializing project: {}", name);
        self.project_manager.init_project(name, template).await?;
        println!("✅ Project '{name}' initialized successfully");
        Ok(())
    }

    pub async fn handle_device_command(&self, command: DeviceCommands) -> Result<()> {
        match command {
            DeviceCommands::List => {
                let devices = self.device_manager.list_devices().await?;
                println!("📱 Connected Devices:");
                for device in devices {
                    println!("  {} - {} ({})", device.id, device.name, device.platform);
                }
            }
            DeviceCommands::Connect { id } => {
                self.device_manager.connect_device(&id).await?;
                println!("✅ Connected to device: {id}");
            }
            DeviceCommands::Install { id, app } => {
                self.device_manager.install_app(&id, &app).await?;
                println!("✅ Installed app on device: {id}");
            }
            DeviceCommands::Deploy { id, project } => {
                self.device_manager
                    .deploy_project(&id, project.as_deref())
                    .await?;
                println!("✅ Deployed project to device: {id}");
            }
            DeviceCommands::Test { id, suite } => {
                self.test_runner
                    .run_device_tests(&id, suite.as_deref())
                    .await?;
                println!("✅ Tests completed on device: {id}");
            }
        }
        Ok(())
    }

    pub async fn handle_simulator_command(&self, command: SimulatorCommands) -> Result<()> {
        match command {
            SimulatorCommands::List => {
                let simulators = self.simulator_manager.list_simulators().await?;
                println!("🔧 Available Simulators:");
                for sim in simulators {
                    println!("  {} - {} ({})", sim.id, sim.name, sim.platform);
                }
            }
            SimulatorCommands::Start { id } => {
                self.simulator_manager.start_simulator(&id).await?;
                println!("✅ Started simulator: {id}");
            }
            SimulatorCommands::Stop { id } => {
                self.simulator_manager.stop_simulator(&id).await?;
                println!("✅ Stopped simulator: {id}");
            }
            SimulatorCommands::Reset { id } => {
                self.simulator_manager.reset_simulator(&id).await?;
                println!("✅ Reset simulator: {id}");
            }
            SimulatorCommands::Install { id, app } => {
                self.simulator_manager.install_app(&id, &app).await?;
                println!("✅ Installed app on simulator: {id}");
            }
        }
        Ok(())
    }

    pub async fn handle_project_command(&self, command: ProjectCommands) -> Result<()> {
        match command {
            ProjectCommands::Build { target } => {
                self.project_manager
                    .build_project(target.as_deref())
                    .await?;
                println!("✅ Project built successfully");
            }
            ProjectCommands::Clean => {
                self.project_manager.clean_project().await?;
                println!("✅ Project cleaned");
            }
            ProjectCommands::Status => {
                let status = self.project_manager.get_project_status().await?;
                println!("📊 Project Status: {status}");
            }
        }
        Ok(())
    }

    pub async fn handle_test_command(&self, command: TestCommands) -> Result<()> {
        match command {
            TestCommands::Run { suite, device } => {
                self.test_runner
                    .run_tests(suite.as_deref(), device.as_deref())
                    .await?;
                println!("✅ Tests completed");
            }
            TestCommands::Record { output } => {
                self.test_runner.record_test(&output).await?;
                println!("✅ Test recorded to: {output}");
            }
            TestCommands::Replay { file } => {
                self.test_runner.replay_test(&file).await?;
                println!("✅ Test replayed from: {file}");
            }
        }
        Ok(())
    }

    pub async fn start_api_server(&self, host: &str, port: u16) -> Result<()> {
        info!("Starting API server on {}:{}", host, port);
        // TODO: Implement API server
        warn!("API server not yet implemented");
        Ok(())
    }

    pub async fn start_mcp_server(&self, config_path: Option<&str>) -> Result<()> {
        info!("Starting MCP server");
        let mcp_server = McpServer::new(&self.config, config_path).await?;
        mcp_server.start().await?;
        Ok(())
    }

    pub async fn start_tui(&self) -> Result<()> {
        info!("Starting TUI interface");
        // TODO: Implement TUI
        warn!("TUI not yet implemented");
        Ok(())
    }
}
