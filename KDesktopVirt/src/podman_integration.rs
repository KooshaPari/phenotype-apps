use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use tokio::process::Command as AsyncCommand;
use tracing::{debug, error, info, warn};

/// Podman integration for rootless, secure container orchestration
/// Following research recommendations for enhanced security and licensing benefits
pub struct PodmanOrchestrator {
    socket_path: String,
    connection: Option<reqwest::Client>,
    pods: HashMap<String, PodInfo>,
    config: PodmanConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodmanConfig {
    pub rootless_mode: bool,
    pub enable_systemd: bool,
    pub cgroup_version: String,
    pub storage_driver: String,
    pub network_backend: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodInfo {
    pub id: String,
    pub name: String,
    pub containers: Vec<String>,
    pub status: String,
    pub network_mode: String,
    pub infra_container_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodmanContainer {
    pub id: String,
    pub name: String,
    pub image: String,
    pub pod_id: Option<String>,
    pub status: String,
    pub ports: Vec<PodmanPort>,
    pub mounts: Vec<PodmanMount>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodmanPort {
    pub host_port: u16,
    pub container_port: u16,
    pub protocol: String,
    pub host_ip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PodmanMount {
    pub source: String,
    pub destination: String,
    pub mount_type: String,
    pub options: Vec<String>,
}

impl Default for PodmanConfig {
    fn default() -> Self {
        Self {
            rootless_mode: true,
            enable_systemd: true,
            cgroup_version: "v2".to_string(),
            storage_driver: "overlay".to_string(),
            network_backend: "netavark".to_string(),
        }
    }
}

impl PodmanOrchestrator {
    pub async fn new(config: Option<PodmanConfig>) -> Result<Self> {
        info!("Initializing Podman orchestrator with rootless security");
        
        let config = config.unwrap_or_default();
        
        // Verify Podman installation
        Self::verify_podman_installation().await?;
        
        // Configure rootless mode
        Self::configure_rootless_mode().await?;
        
        let socket_path = if config.rootless_mode {
            format!("/run/user/{}/podman/podman.sock", uzers::get_current_uid())
        } else {
            "/run/podman/podman.sock".to_string()
        };
        
        Ok(Self {
            socket_path,
            connection: Some(reqwest::Client::new()),
            pods: HashMap::new(),
            config,
        })
    }
    
    async fn verify_podman_installation() -> Result<()> {
        let output = AsyncCommand::new("podman")
            .arg("--version")
            .output()
            .await
            .map_err(|_| anyhow!("Podman not installed or not in PATH"))?;
            
        if !output.status.success() {
            return Err(anyhow!("Podman installation not functional"));
        }
        
        let version = String::from_utf8_lossy(&output.stdout);
        info!("Podman version verified: {}", version.trim());
        
        // Check for required features
        let output = AsyncCommand::new("podman")
            .args(["info", "--format", "json"])
            .output()
            .await?;
            
        if output.status.success() {
            let info_str = String::from_utf8_lossy(&output.stdout);
            if let Ok(info_json) = serde_json::from_str::<serde_json::Value>(&info_str) {
                info!("Podman capabilities: {}", 
                      serde_json::to_string_pretty(&info_json["host"]["security"]).unwrap_or_default());
            }
        }
        
        Ok(())
    }
    
    async fn configure_rootless_mode() -> Result<()> {
        info!("Configuring rootless Podman for enhanced security");
        
        // Check if user namespaces are available
        let output = AsyncCommand::new("podman")
            .args(["unshare", "cat", "/proc/self/uid_map"])
            .output()
            .await?;
            
        if !output.status.success() {
            warn!("User namespaces may not be properly configured");
        }
        
        // Enable systemd support for desktop environments
        let _output = AsyncCommand::new("podman")
            .args(["system", "service", "--time=0"])
            .spawn();
            
        Ok(())
    }
    
    /// Create a pod for desktop virtualization with proper networking
    pub async fn create_desktop_pod(
        &mut self,
        session_id: &str,
        desktop_type: &str,
        vnc_port: u16,
    ) -> Result<String> {
        info!("Creating desktop pod for session: {} ({})", session_id, desktop_type);
        
        let pod_name = format!("kvs-desktop-{}-{}", desktop_type, session_id);
        
        let mut cmd = AsyncCommand::new("podman");
        cmd.args([
            "pod", "create",
            "--name", &pod_name,
            "--hostname", &format!("kvs-{}", session_id),
            // Network configuration
            "--publish", &format!("{}:5900", vnc_port),
            "--publish", &format!("{}:22", vnc_port + 1000), // SSH
            "--publish", &format!("{}:6080", vnc_port + 2000), // noVNC
            // Security enhancements
            "--security-opt", "no-new-privileges=true",
            "--cap-drop", "ALL",
            "--cap-add", "SETUID,SETGID,SYS_CHROOT", // Minimal caps for desktop
            // Resource management
            "--memory", "2g",
            "--cpus", "2.0",
            "--pids-limit", "1024",
            // Shared IPC for desktop applications
            "--ipc", "shareable",
        ]);
        
        let output = cmd.output().await?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Failed to create pod: {}", error));
        }
        
        let pod_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        let pod_info = PodInfo {
            id: pod_id.clone(),
            name: pod_name,
            containers: Vec::new(),
            status: "created".to_string(),
            network_mode: "host".to_string(),
            infra_container_id: None,
        };
        
        self.pods.insert(session_id.to_string(), pod_info);
        
        info!("Desktop pod created: {}", pod_id);
        Ok(pod_id)
    }
    
    /// Create and start a desktop container within a pod
    pub async fn create_desktop_container(
        &mut self,
        session_id: &str,
        desktop_type: &str,
        image: &str,
        vnc_password: &str,
    ) -> Result<String> {
        info!("Creating desktop container in pod for session: {}", session_id);
        
        let container_name = format!("kvs-desktop-{}", session_id);
        let pod_name = format!("kvs-desktop-{}-{}", desktop_type, session_id);
        
        let mut cmd = AsyncCommand::new("podman");
        cmd.args([
            "run", "-d",
            "--name", &container_name,
            "--pod", &pod_name,
            // Environment variables
            "--env", "DISPLAY=:0",
            "--env", &format!("VNC_PASSWORD={}", vnc_password),
            "--env", "RESOLUTION=1920x1080",
            "--env", "DEBIAN_FRONTEND=noninteractive",
        ]);
        
        // Desktop-specific environment
        match desktop_type {
            "kubuntu" => {
                cmd.args([
                    "--env", "DESKTOP_SESSION=plasma",
                    "--env", "XDG_SESSION_DESKTOP=KDE",
                    "--env", "XDG_CURRENT_DESKTOP=KDE",
                ]);
            },
            "ubuntu" => {
                cmd.args([
                    "--env", "DESKTOP_SESSION=ubuntu",
                    "--env", "XDG_SESSION_DESKTOP=ubuntu:GNOME",
                    "--env", "XDG_CURRENT_DESKTOP=ubuntu:GNOME",
                ]);
            },
            _ => {
                cmd.args([
                    "--env", "DESKTOP_SESSION=xfce",
                    "--env", "XDG_SESSION_DESKTOP=XFCE",
                ]);
            }
        }
        
        // Volume mounts for persistence
        cmd.args([
            "--volume", &format!("kvs-home-{}:/home/kvs:Z", session_id),
            "--volume", &format!("kvs-config-{}:/etc/kvs:Z", session_id),
            "--tmpfs", "/tmp:noexec,nosuid,size=1g",
            "--tmpfs", "/var/tmp:noexec,nosuid,size=512m",
        ]);
        
        // Security hardening
        cmd.args([
            "--read-only=true",
            "--tmpfs", "/run:rw,noexec,nosuid,size=100m",
            "--security-opt", "label=type:container_runtime_t",
            "--user", "1000:1000",
        ]);
        
        cmd.arg(image);
        
        let output = cmd.output().await?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Failed to create container: {}", error));
        }
        
        let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
        
        // Wait for container to be ready
        self.wait_for_container_ready(&container_id).await?;
        
        info!("Desktop container created and ready: {}", container_id);
        Ok(container_id)
    }
    
    async fn wait_for_container_ready(&self, container_id: &str) -> Result<()> {
        let max_attempts = 30;
        let mut attempts = 0;
        
        while attempts < max_attempts {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            attempts += 1;
            
            let output = AsyncCommand::new("podman")
                .args(["exec", container_id, "pgrep", "-f", "vnc"])
                .output()
                .await;
                
            if let Ok(output) = output {
                if output.status.success() && !output.stdout.is_empty() {
                    info!("Desktop environment ready in container: {}", container_id);
                    return Ok(());
                }
            }
        }
        
        warn!("Desktop environment may not be fully ready: {}", container_id);
        Ok(())
    }
    
    /// Get container resource usage statistics
    pub async fn get_container_stats(&self, container_id: &str) -> Result<PodmanStats> {
        let output = AsyncCommand::new("podman")
            .args(["stats", "--format", "json", "--no-stream", container_id])
            .output()
            .await?;
            
        if !output.status.success() {
            return Err(anyhow!("Failed to get container stats"));
        }
        
        let stats_json = String::from_utf8_lossy(&output.stdout);
        let stats: serde_json::Value = serde_json::from_str(&stats_json)?;
        
        Ok(PodmanStats {
            cpu_percent: stats["CPU%"].as_str().unwrap_or("0%").trim_end_matches('%').parse().unwrap_or(0.0),
            memory_usage: stats["MemUsage"].as_str().unwrap_or("0MB").to_string(),
            memory_percent: stats["Mem%"].as_str().unwrap_or("0%").trim_end_matches('%').parse().unwrap_or(0.0),
            network_io: stats["NetIO"].as_str().unwrap_or("0B / 0B").to_string(),
            block_io: stats["BlockIO"].as_str().unwrap_or("0B / 0B").to_string(),
            pids: stats["PIDs"].as_u64().unwrap_or(0),
        })
    }
    
    /// Update container resource limits dynamically
    pub async fn update_container_resources(
        &self,
        container_id: &str,
        memory_limit: &str,
        cpu_limit: &str,
    ) -> Result<()> {
        info!("Updating container resources: {} (CPU: {}, Memory: {})", container_id, cpu_limit, memory_limit);
        
        let output = AsyncCommand::new("podman")
            .args([
                "update",
                "--memory", memory_limit,
                "--cpus", cpu_limit,
                container_id
            ])
            .output()
            .await?;
            
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("Failed to update resources: {}", error));
        }
        
        info!("Container resources updated successfully: {}", container_id);
        Ok(())
    }
    
    /// Cleanup pod and associated containers
    pub async fn cleanup_session(&mut self, session_id: &str) -> Result<()> {
        info!("Cleaning up Podman session: {}", session_id);
        
        if let Some(pod_info) = self.pods.remove(session_id) {
            // Stop and remove all containers in the pod
            for container_id in &pod_info.containers {
                let _output = AsyncCommand::new("podman")
                    .args(["container", "stop", container_id])
                    .output()
                    .await;
                    
                let _output = AsyncCommand::new("podman")
                    .args(["container", "rm", container_id])
                    .output()
                    .await;
            }
            
            // Remove the pod
            let _output = AsyncCommand::new("podman")
                .args(["pod", "rm", "--force", &pod_info.id])
                .output()
                .await;
                
            // Remove associated volumes
            let _output = AsyncCommand::new("podman")
                .args(["volume", "rm", &format!("kvs-home-{}", session_id)])
                .output()
                .await;
                
            let _output = AsyncCommand::new("podman")
                .args(["volume", "rm", &format!("kvs-config-{}", session_id)])
                .output()
                .await;
                
            info!("Podman session cleaned up: {}", session_id);
        }
        
        Ok(())
    }
    
    /// Get system information and capabilities
    pub async fn get_system_info(&self) -> Result<PodmanSystemInfo> {
        let output = AsyncCommand::new("podman")
            .args(["system", "info", "--format", "json"])
            .output()
            .await?;
            
        if !output.status.success() {
            return Err(anyhow!("Failed to get system info"));
        }
        
        let info_json = String::from_utf8_lossy(&output.stdout);
        let info: serde_json::Value = serde_json::from_str(&info_json)?;
        
        Ok(PodmanSystemInfo {
            version: info["version"]["Version"].as_str().unwrap_or("unknown").to_string(),
            api_version: info["version"]["APIVersion"].as_str().unwrap_or("unknown").to_string(),
            storage_driver: info["store"]["graphDriverName"].as_str().unwrap_or("overlay").to_string(),
            cgroup_version: info["host"]["cgroupVersion"].as_str().unwrap_or("v2").to_string(),
            rootless: info["host"]["security"]["rootless"].as_bool().unwrap_or(true),
            containers_count: info["store"]["containerStore"]["number"].as_u64().unwrap_or(0),
            images_count: info["store"]["imageStore"]["number"].as_u64().unwrap_or(0),
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PodmanStats {
    pub cpu_percent: f64,
    pub memory_usage: String,
    pub memory_percent: f64,
    pub network_io: String,
    pub block_io: String,
    pub pids: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PodmanSystemInfo {
    pub version: String,
    pub api_version: String,
    pub storage_driver: String,
    pub cgroup_version: String,
    pub rootless: bool,
    pub containers_count: u64,
    pub images_count: u64,
}
