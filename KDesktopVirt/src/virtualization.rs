use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use bollard::models::{
    ContainerCreateBody, ContainerCreateResponse, HostConfig, PortBinding, RestartPolicy,
    RestartPolicyNameEnum,
};
use bollard::query_parameters::{
    CreateContainerOptions, CreateImageOptions, RemoveContainerOptions, StartContainerOptions,
    StopContainerOptions,
};
use bollard::Docker;
use futures::StreamExt;
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;
use std::net::TcpListener;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: Vec<u16>,
    pub vnc_port: Option<u16>,
    pub desktop_type: DesktopType,
    pub resource_usage: ResourceUsage,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub vm_info: Option<VmInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DesktopType {
    Kubuntu,
    Ubuntu,
    Debian,
    Windows10,
    Windows11,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_percent: f64,
    pub memory_used_mb: u64,
    pub memory_limit_mb: u64,
    pub disk_used_gb: f64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmInfo {
    pub libvirt_domain: Option<String>,
    pub qemu_pid: Option<u32>,
    pub vm_type: VmType,
    pub vcpus: u32,
    pub memory_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VmType {
    KVM,
    QEMU,
    Container,
    Hybrid,
}

pub struct VirtualizationManager {
    // Container orchestration
    docker: Docker,
    podman_client: Option<PodmanClient>,
    containers: HashMap<String, ContainerInfo>,
    
    // VM orchestration
    libvirt_connection: Option<LibvirtConnection>,
    vm_instances: HashMap<String, VmInfo>,
    
    // Resource management
    port_pool: Arc<Mutex<PortPool>>,
    resource_monitor: Arc<Mutex<ResourceMonitor>>,
    image_cache: Arc<Mutex<ImageCache>>,
    
    // Configuration
    config: VirtualizationConfig,
}

#[derive(Debug, Clone)]
pub struct VirtualizationConfig {
    pub hybrid_mode: bool,
    pub prefer_containers: bool,
    pub enable_gpu_passthrough: bool,
    pub enable_nested_virtualization: bool,
    pub resource_limits: ResourceLimits,
    pub networking: NetworkConfig,
}

#[derive(Debug, Clone)]
pub struct ResourceLimits {
    pub max_containers: u32,
    pub max_vms: u32,
    pub cpu_overcommit_ratio: f64,
    pub memory_overcommit_ratio: f64,
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub bridge_name: String,
    pub subnet: String,
    pub dns_servers: Vec<String>,
    pub enable_nat: bool,
}

impl Default for VirtualizationConfig {
    fn default() -> Self {
        Self {
            hybrid_mode: false,
            prefer_containers: true,
            enable_gpu_passthrough: false,
            enable_nested_virtualization: false,
            resource_limits: ResourceLimits {
                max_containers: 10,
                max_vms: 5,
                cpu_overcommit_ratio: 2.0,
                memory_overcommit_ratio: 1.5,
            },
            networking: NetworkConfig {
                bridge_name: "kvs-br0".to_string(),
                subnet: "172.16.0.0/24".to_string(),
                dns_servers: vec!["8.8.8.8".to_string(), "8.8.4.4".to_string()],
                enable_nat: true,
            },
        }
    }
}

// Podman client for rootless containers
#[derive(Debug)]
pub struct PodmanClient {
    socket_path: String,
    connection: Option<reqwest::Client>,
}

// LibVirt connection for VM management
#[derive(Debug)]
pub struct LibvirtConnection {
    uri: String,
    connection_handle: Option<std::process::Child>,
}

// Resource monitoring system
#[derive(Debug)]
pub struct ResourceMonitor {
    cpu_usage: HashMap<String, f64>,
    memory_usage: HashMap<String, u64>,
    disk_usage: HashMap<String, f64>,
    network_stats: HashMap<String, (u64, u64)>,
    last_update: Instant,
}

// Image caching and optimization
#[derive(Debug)]
pub struct ImageCache {
    cached_images: HashMap<String, CachedImage>,
    base_layers: HashMap<String, String>,
    optimization_settings: ImageOptimization,
}

#[derive(Debug, Clone)]
pub struct CachedImage {
    pub id: String,
    pub name: String,
    pub size_mb: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used: chrono::DateTime<chrono::Utc>,
    pub usage_count: u32,
}

#[derive(Debug, Clone)]
pub struct ImageOptimization {
    pub enable_compression: bool,
    pub enable_deduplication: bool,
    pub enable_lazy_loading: bool,
    pub max_cache_size_gb: u64,
}

#[derive(Debug)]
struct PortPool {
    available_ports: Vec<u16>,
    allocated_ports: HashMap<u16, String>,
    next_port_index: usize,
}

impl PortPool {
    fn new(start_port: u16, count: u16) -> Self {
        let available_ports: Vec<u16> = (start_port..start_port + count).collect();
        Self {
            available_ports,
            allocated_ports: HashMap::new(),
            next_port_index: 0,
        }
    }

    fn allocate_port(&mut self, session_id: &str) -> Option<u16> {
        if self.next_port_index >= self.available_ports.len() {
            // Try to find returned ports
            self.cleanup_stale_allocations();
        }

        if self.next_port_index < self.available_ports.len() {
            let port = self.available_ports[self.next_port_index];
            self.allocated_ports.insert(port, session_id.to_string());
            self.next_port_index += 1;
            Some(port)
        } else {
            None
        }
    }

    fn release_port(&mut self, port: u16) {
        if let Some(_) = self.allocated_ports.remove(&port) {
            // Move the port back to available pool
            if let Some(pos) = self.available_ports.iter().position(|&p| p == port) {
                if pos >= self.next_port_index {
                    self.available_ports.swap(pos, self.next_port_index - 1);
                    self.next_port_index -= 1;
                }
            }
        }
    }

    fn cleanup_stale_allocations(&mut self) {
        // In a real implementation, this would check if containers are still running
        // For now, we'll implement a basic cleanup strategy
        let stale_ports: Vec<u16> = self.allocated_ports.keys().cloned().collect();
        for port in stale_ports {
            if !self.is_port_in_use(port) {
                self.release_port(port);
            }
        }
    }

    fn is_port_in_use(&self, port: u16) -> bool {
        TcpListener::bind(format!("127.0.0.1:{}", port)).is_err()
    }
}

impl VirtualizationManager {
    pub async fn new() -> Result<Self> {
        info!("Initializing VirtualizationManager");

        let docker = Docker::connect_with_defaults()?;

        // Test connection
        match docker.ping().await {
            Ok(_) => info!("Docker connection successful"),
            Err(e) => {
                error!("Docker connection failed: {}", e);
                return Err(anyhow!("Docker connection failed: {}", e));
            }
        }

        // Initialize port pool (VNC ports 5900-5999)
        let port_pool = Arc::new(Mutex::new(PortPool::new(5900, 100)));

        Ok(Self {
            docker,
            podman_client: None,
            containers: HashMap::new(),
            libvirt_connection: None,
            vm_instances: HashMap::new(),
            port_pool,
            resource_monitor: Arc::new(Mutex::new(ResourceMonitor::new())),
            image_cache: Arc::new(Mutex::new(ImageCache::new())),
            config: VirtualizationConfig::default(),
        })
    }

    // Legacy method for backward compatibility
    pub async fn create_container(
        &mut self,
        session_id: String,
        desktop: String,
        image: Option<String>,
        memory_mb: u64,
        cpu_cores: u32,
    ) -> Result<String> {
        self.create_instance(session_id, desktop, image, memory_mb, cpu_cores, false).await
    }

    pub async fn create_instance(
        &mut self,
        session_id: String,
        desktop: String,
        image: Option<String>,
        memory_mb: u64,
        cpu_cores: u32,
        _prefer_vm: bool,
    ) -> Result<String> {
        info!("Creating container for session: {}", session_id);

        let image_name = image.unwrap_or_else(|| self.get_default_image(&desktop));

        // Ensure image exists
        self.ensure_image(&image_name).await?;

        let container_name = format!("kvirtualstage-{session_id}");

        // Allocate VNC port from pool
        let vnc_port = {
            let mut pool = self.port_pool.lock().unwrap();
            pool.allocate_port(&session_id)
                .ok_or_else(|| anyhow!("No available VNC ports in pool"))?
        };

        // Configure container
        let mut port_bindings = HashMap::new();
        port_bindings.insert(
            "5900/tcp".to_string(),
            Some(vec![PortBinding {
                host_ip: Some("127.0.0.1".to_string()),
                host_port: Some(vnc_port.to_string()),
            }]),
        );

        let host_config = HostConfig {
            port_bindings: Some(port_bindings),
            memory: Some((memory_mb * 1024 * 1024) as i64), // Convert to bytes
            nano_cpus: Some(cpu_cores as i64 * 1_000_000_000), // Convert to nanocpus
            shm_size: Some(2147483648), // 2GB shared memory for desktop workloads
            // CPU affinity for better performance isolation
            cpuset_cpus: Some(self.get_cpu_affinity(cpu_cores)),
            // Restart policy for reliability
            restart_policy: Some(RestartPolicy {
                name: Some(RestartPolicyNameEnum::UNLESS_STOPPED),
                maximum_retry_count: Some(3),
            }),
            // Additional optimizations
            oom_kill_disable: Some(false),
            memory_swappiness: Some(10), // Reduce swap usage
            ..Default::default()
        };

        // Generate secure random VNC password
        let vnc_password = self.generate_secure_password()?;
        let mut env = vec![
            "DISPLAY=:0".to_string(),
            format!("VNC_PASSWORD={}", vnc_password),
            "RESOLUTION=1920x1080".to_string(),
        ];

        // Desktop-specific environment variables
        match desktop.as_str() {
            "kubuntu" => {
                env.push("DESKTOP_SESSION=plasma".to_string());
                env.push("XDG_SESSION_DESKTOP=KDE".to_string());
            }
            "ubuntu" => {
                env.push("DESKTOP_SESSION=ubuntu".to_string());
                env.push("XDG_SESSION_DESKTOP=ubuntu:GNOME".to_string());
            }
            _ => {
                warn!("Unknown desktop environment: {}, using default", desktop);
            }
        }

        let config = ContainerCreateBody {
            image: Some(image_name.clone()),
            env: Some(env),
            host_config: Some(host_config),
            exposed_ports: Some(vec!["5900/tcp".to_string()]),
            ..Default::default()
        };

        // Create container
        let response: ContainerCreateResponse = self
            .docker
            .create_container(
                Some(CreateContainerOptions {
                    name: Some(container_name.clone()),
                    ..Default::default()
                }),
                config,
            )
            .await?;

        let container_id = response.id;

        // Start container
        self.docker
            .start_container(&container_id, None::<StartContainerOptions>)
            .await?;

        // Store container info
        let container_info = ContainerInfo {
            id: container_id.clone(),
            name: container_name,
            image: image_name,
            status: "running".to_string(),
            ports: vec![vnc_port],
            vnc_port: Some(vnc_port),
            desktop_type: DesktopType::Kubuntu, // Default, could be determined from desktop param
            resource_usage: ResourceUsage {
                cpu_percent: 0.0,
                memory_used_mb: 0,
                memory_limit_mb: memory_mb,
                disk_used_gb: 0.0,
                network_rx_bytes: 0,
                network_tx_bytes: 0,
            },
            created_at: chrono::Utc::now(),
            vm_info: None,
        };

        self.containers.insert(session_id, container_info);

        info!("Container created successfully: {}", container_id);
        Ok(container_id)
    }

    pub async fn stop_container(&self, container_id: String) -> Result<()> {
        info!("Stopping container: {}", container_id);

        self.docker
            .stop_container(&container_id, None::<StopContainerOptions>)
            .await?;

        Ok(())
    }

    pub async fn remove_container(&self, container_id: String) -> Result<()> {
        info!("Removing container: {}", container_id);

        // Stop container first
        let _ = self.stop_container(container_id.clone()).await;

        // Remove container
        self.docker
            .remove_container(
                &container_id,
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await?;

        Ok(())
    }

    pub async fn list_containers(&self) -> Result<Vec<ContainerInfo>> {
        Ok(self.containers.values().cloned().collect())
    }

    pub async fn get_container_info(&self, session_id: &str) -> Option<&ContainerInfo> {
        self.containers.get(session_id)
    }

    async fn ensure_image(&self, image_name: &str) -> Result<()> {
        info!("Ensuring image exists: {}", image_name);

        // Check if image exists locally
        let images = self
            .docker
            .list_images(None::<bollard::query_parameters::ListImagesOptions>)
            .await?;

        for image in images {
            if !image.repo_tags.is_empty() && image.repo_tags.contains(&image_name.to_string()) {
                info!("Image {} already exists locally", image_name);
                return Ok(());
            }
        }

        // Pull image
        info!("Pulling image: {}", image_name);
        let mut stream = self.docker.create_image(
            Some(CreateImageOptions {
                from_image: Some(image_name.to_string()),
                ..Default::default()
            }),
            None,
            None,
        );

        while let Some(result) = stream.next().await {
            match result {
                Ok(info) => {
                    if let Some(status) = info.status {
                        info!("Image pull: {}", status);
                    }
                }
                Err(e) => {
                    error!("Image pull error: {}", e);
                    return Err(anyhow!("Failed to pull image: {}", e));
                }
            }
        }

        Ok(())
    }

    fn get_default_image(&self, desktop: &str) -> String {
        match desktop {
            "kubuntu" => "ghcr.io/kvirtualstage/kubuntu-desktop:latest".to_string(),
            "ubuntu" => "ghcr.io/kvirtualstage/ubuntu-desktop:latest".to_string(),
            "debian" => "ghcr.io/kvirtualstage/debian-desktop:latest".to_string(),
            _ => "ghcr.io/kvirtualstage/kubuntu-desktop:latest".to_string(),
        }
    }

    fn get_cpu_affinity(&self, cpu_cores: u32) -> String {
        // Allocate CPU cores efficiently
        // For enterprise workloads, we want to avoid CPU 0 (often used by system)
        let start_cpu = 1;
        let end_cpu = std::cmp::min(start_cpu + cpu_cores - 1, num_cpus::get() as u32 - 1);
        
        if cpu_cores == 1 {
            start_cpu.to_string()
        } else {
            format!("{}-{}", start_cpu, end_cpu)
        }
    }

    pub async fn release_session_resources(&mut self, session_id: &str) -> Result<()> {
        if let Some(container_info) = self.containers.get(session_id) {
            if let Some(vnc_port) = container_info.vnc_port {
                // Release port back to pool
                let mut pool = self.port_pool.lock().unwrap();
                pool.release_port(vnc_port);
                info!("Released VNC port {} for session {}", vnc_port, session_id);
            }
        }
        Ok(())
    }

    fn generate_secure_password(&self) -> Result<String> {
        let rng = SystemRandom::new();
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes).map_err(|_| anyhow!("Failed to generate random bytes"))?;
        
        // Convert to base64 and clean up for VNC password compatibility
        let password = general_purpose::STANDARD.encode(&bytes)
            .chars()
            .filter(|c| c.is_alphanumeric())
            .take(24)
            .collect::<String>();
        
        Ok(password)
    }
}

// Implementation for new structs
impl ResourceMonitor {
    fn new() -> Self {
        Self {
            cpu_usage: HashMap::new(),
            memory_usage: HashMap::new(),
            disk_usage: HashMap::new(),
            network_stats: HashMap::new(),
            last_update: Instant::now(),
        }
    }
}

impl ImageCache {
    fn new() -> Self {
        Self {
            cached_images: HashMap::new(),
            base_layers: HashMap::new(),
            optimization_settings: ImageOptimization {
                enable_compression: true,
                enable_deduplication: true,
                enable_lazy_loading: true,
                max_cache_size_gb: 50,
            },
        }
    }
    
    fn cleanup_if_needed(&mut self) {
        let total_size_gb: u64 = self.cached_images.values()
            .map(|img| img.size_mb / 1024)
            .sum();
            
        if total_size_gb > self.optimization_settings.max_cache_size_gb {
            // Remove least recently used images
            let mut images: Vec<_> = self.cached_images.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            images.sort_by_key(|(_, img)| img.last_used);
            
            let mut removed_size = 0u64;
            let target_size = self.optimization_settings.max_cache_size_gb / 2; // Remove 50% of cache
            
            for (name, img) in images {
                if removed_size >= target_size {
                    break;
                }
                
                removed_size += img.size_mb / 1024;
                self.cached_images.remove(&name);
                info!("Removed cached image: {} ({}MB)", name, img.size_mb);
            }
        }
    }
}

// Utility functions for hybrid orchestration
impl VirtualizationManager {
    pub async fn get_system_resources(&self) -> Result<SystemResources> {
        let cpu_count = num_cpus::get();
        
        // Get memory info
        let memory_info = if let Ok(contents) = std::fs::read_to_string("/proc/meminfo") {
            let lines: Vec<&str> = contents.lines().collect();
            let total_line = lines.iter().find(|line| line.starts_with("MemTotal:"))
                .unwrap_or(&"MemTotal: 0 kB");
            let available_line = lines.iter().find(|line| line.starts_with("MemAvailable:"))
                .unwrap_or(&"MemAvailable: 0 kB");
                
            let total_kb: u64 = total_line.split_whitespace()
                .nth(1).unwrap_or("0").parse().unwrap_or(0);
            let available_kb: u64 = available_line.split_whitespace()
                .nth(1).unwrap_or("0").parse().unwrap_or(0);
                
            (total_kb / 1024, available_kb / 1024) // Convert to MB
        } else {
            (0, 0)
        };
        
        Ok(SystemResources {
            cpu_cores: cpu_count as u32,
            memory_total_mb: memory_info.0,
            memory_available_mb: memory_info.1,
            disk_available_gb: self.get_disk_space().await.unwrap_or(0),
            gpu_available: self.config.enable_gpu_passthrough,
        })
    }
    
    async fn get_disk_space(&self) -> Result<u64> {
        let output = tokio::process::Command::new("df")
            .args(["-BG", "/"])
            .output()
            .await?;
            
        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = output_str.lines().collect();
            if lines.len() >= 2 {
                let parts: Vec<&str> = lines[1].split_whitespace().collect();
                if parts.len() >= 4 {
                    let available_str = parts[3].trim_end_matches('G');
                    return Ok(available_str.parse().unwrap_or(0));
                }
            }
        }
        
        Ok(0)
    }
    
    pub fn get_virtualization_capabilities(&self) -> VirtualizationCapabilities {
        VirtualizationCapabilities {
            containers_supported: true,
            vms_supported: self.libvirt_connection.is_some(),
            podman_available: self.podman_client.is_some(),
            gpu_passthrough: self.config.enable_gpu_passthrough,
            nested_virtualization: self.config.enable_nested_virtualization,
            max_containers: self.config.resource_limits.max_containers,
            max_vms: self.config.resource_limits.max_vms,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemResources {
    pub cpu_cores: u32,
    pub memory_total_mb: u64,
    pub memory_available_mb: u64,
    pub disk_available_gb: u64,
    pub gpu_available: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VirtualizationCapabilities {
    pub containers_supported: bool,
    pub vms_supported: bool,
    pub podman_available: bool,
    pub gpu_passthrough: bool,
    pub nested_virtualization: bool,
    pub max_containers: u32,
    pub max_vms: u32,
}
