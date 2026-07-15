# ADR-002: Cross-Platform Strategy

**Document ID:** PHENOTYPE_KDESKTOPVIRT_ADR_002  
**Status:** Accepted  
**Last Updated:** 2026-04-03  
**Author:** Phenotype Architecture Team  
**Supersedes:** N/A  
**Related:** ADR-001, ADR-003, SPEC.md

---

## Context

KDesktopVirt must support multiple desktop environments and operating systems to serve its target markets (AI/ML teams, QA/testing, security researchers). The platform currently runs on Linux with X11, but the desktop ecosystem is evolving:

### Platform Landscape

```
Desktop Platform Ecosystem (2026):
┌─────────────────────────────────────────────────────────────────┐
│  Linux (Primary)                                                │
│  ├── X11 (Mature, declining)                                    │
│  │   ├── KDE Plasma 6 (KWin)                                    │
│  │   ├── GNOME (Mutter)                                         │
│  │   └── XFCE (Xfwm)                                            │
│  │                                                              │
│  └── Wayland (Growing, future)                                  │
│      ├── KDE Plasma 6 (KWin/Wayland)                            │
│      ├── GNOME (Mutter/Wayland)                                 │
│      └── Sway (tiling)                                          │
│                                                                 │
│  Windows (Future)                                               │
│  └── DWM + UI Automation API                                    │
│                                                                 │
│  macOS (Future)                                                 │
│  └── Quartz + Accessibility API                                 │
└─────────────────────────────────────────────────────────────────┘
```

### Technical Challenges

1. **X11 to Wayland Migration**: Major Linux distributions are defaulting to Wayland. X11 automation tools (xdotool) don't work on Wayland.
2. **Cross-OS Input Simulation**: Each OS has different APIs for synthetic input (XTest on X11, SendInput on Windows, CGEvent on macOS).
3. **Container Compatibility**: Docker containers typically run X11 (Xvfb). Wayland in containers is experimental.
4. **Element Detection**: Accessibility APIs differ significantly across platforms (AT-SPI on Linux, UI Automation on Windows, AX API on macOS).

### Design Alternatives Considered

```
Alternative 1: X11 Only
┌─────────────────────────────────────────────────────┐
│  Pros: Mature tooling, simple, container-friendly   │
│  Cons: Becoming obsolete, Wayland adoption growing  │
│  Result: Rejected - limits future viability         │
└─────────────────────────────────────────────────────┘

Alternative 2: Wayland Only
┌─────────────────────────────────────────────────────┐
│  Pros: Modern, secure, future-proof                 │
│  Cons: Immature tooling, container challenges       │
│  Result: Rejected - not production-ready yet        │
└─────────────────────────────────────────────────────┘

Alternative 3: Platform Abstraction Layer
┌─────────────────────────────────────────────────────┐
│  Pros: Supports all platforms, extensible           │
│  Cons: More complex, requires per-platform impl     │
│  Result: ACCEPTED                                   │
└─────────────────────────────────────────────────────┘
```

---

## Decision

We adopt a **Platform Abstraction Layer (PAL)** with feature-gated implementations:

```
┌─────────────────────────────────────────────────────────────┐
│              Platform Abstraction Layer                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Trait: PlatformInput                               │   │
│  │  ┌─────────────────────────────────────────────┐   │   │
│  │  │ async fn move_cursor(&self, x, y)           │   │   │
│  │  │ async fn click(&self, button)               │   │   │
│  │  │ async fn type_character(&self, char)        │   │   │
│  │  │ async fn press_key(&self, key)              │   │   │
│  │  │ async fn screenshot(&self) -> Vec<u8>       │   │   │
│  │  │ async fn find_window(&self, pattern)        │   │   │
│  │  │ async fn get_window_geometry(&self)         │   │   │
│  │  │ async fn list_windows(&self)                │   │   │
│  │  └─────────────────────────────────────────────┘   │   │
│  └─────────────────────────────────────────────────────┘   │
│                          │                                  │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐       │
│  │  X11Impl     │ │ WaylandImpl  │ │ WindowsImpl  │       │
│  │  (primary)   │ │ (planned)    │ │ (future)     │       │
│  │              │ │              │ │              │       │
│  │ xdotool      │ │ virtual-     │ │ SendInput    │       │
│  │ XTest        │ │ keyboard     │ │ UIAuto       │       │
│  │ x11 crate    │ │ protocol     │ │ crate        │       │
│  │ wmctrl       │ │ wlr-*        │ │              │       │
│  │              │ │ protocols    │ │              │       │
│  └──────────────┘ └──────────────┘ └──────────────┘       │
│                          │                                  │
│  Feature Flags:                                               │
│  ├── x11-support = ["x11", "screenshots"]                   │
│  ├── wayland = ["wayland-client"]                           │
│  └── full-desktop = ["x11-support", "audio-support"]        │
└─────────────────────────────────────────────────────────────┘
```

### Current Implementation (X11)

The current implementation uses external tools via `tokio::process::Command`:

```rust
// X11 implementation via xdotool
async fn click_action(&mut self, action: &UiAction, session_id: Option<String>) -> Result<()> {
    let (x, y) = action.coordinates.ok_or_else(|| anyhow!("Click requires coordinates"))?;

    // WindMouse 2.0 natural movement
    let target = Point::new(x as f64, y as f64);
    self.windmouse_move_to(target, session_id.clone()).await?;

    // Human-like delay before click
    let delay = 0.02 + rand::thread_rng().gen::<f64>() * 0.03;
    tokio::time::sleep(Duration::from_secs_f64(delay)).await;

    // Execute click via xdotool
    let display = self.get_display_for_session(session_id)?;
    Command::new("xdotool")
        .env("DISPLAY", display)
        .args(["click", "1"])
        .output()
        .await?;

    Ok(())
}
```

### Container Desktop Strategy

```
Container Desktop Matrix:
┌─────────────────────────────────────────────────────────────┐
│  Runtime    │ Display      │ WM           │ Use Case        │
├─────────────┼──────────────┼──────────────┼─────────────────┤
│  Docker     │ Xvfb + X11   │ KWin         │ Primary (KDE)   │
│  Docker     │ Xvfb + X11   │ XFCE         │ Lightweight     │
│  Docker     │ Xvfb + X11   │ Openbox      │ Minimal         │
│  Podman     │ Xvfb + X11   │ KWin         │ Rootless        │
│  K8s        │ Xvfb + X11   │ KWin         │ Enterprise      │
│  Docker     │ Wayland      │ KWin/Wayland │ Future          │
└─────────────────────────────────────────────────────────────┘
```

### Virtualization Manager Architecture

```rust
pub struct VirtualizationManager {
    // Container orchestration
    docker: Docker,
    podman_client: Option<PodmanClient>,
    containers: HashMap<String, ContainerInfo>,

    // VM orchestration (hybrid mode)
    libvirt_connection: Option<LibvirtConnection>,
    vm_instances: HashMap<String, VmInfo>,

    // Resource management
    port_pool: Arc<Mutex<PortPool>>,        // VNC ports 5900-5999
    resource_monitor: Arc<Mutex<ResourceMonitor>>,
    image_cache: Arc<Mutex<ImageCache>>,

    // Configuration
    config: VirtualizationConfig,
}

pub struct VirtualizationConfig {
    pub hybrid_mode: bool,                   // Container + VM
    pub prefer_containers: bool,             // Default to containers
    pub enable_gpu_passthrough: bool,        // GPU acceleration
    pub enable_nested_virtualization: bool,  // VMs in containers
    pub resource_limits: ResourceLimits,
    pub networking: NetworkConfig,
}
```

### Desktop Type Support

```rust
pub enum DesktopType {
    Kubuntu,    // KDE Plasma 6 (primary)
    Ubuntu,     // GNOME
    Debian,     // XFCE
    Windows10,  // Future
    Windows11,  // Future
    Custom(String),
}
```

---

## Consequences

### Positive

1. **X11 Stability**: X11 is mature, well-documented, and fully supported in container environments. xdotool, wmctrl, and xwininfo provide comprehensive automation capabilities.

2. **Feature-Gated Builds**: Users can compile only the features they need, reducing binary size and dependencies. The `x11-support` feature flag gates X11-specific crates.

3. **Container Compatibility**: Xvfb (X virtual framebuffer) works reliably in Docker containers, enabling headless desktop automation without a physical display.

4. **Port Pool Management**: Dedicated VNC port pool (5900-5999) with automatic allocation and cleanup prevents port conflicts across concurrent sessions.

5. **Hybrid Orchestration**: Support for both containers (Docker/Podman) and VMs (libvirt/KVM) enables flexibility for different isolation requirements.

6. **CPU Affinity**: Container CPU pinning (`cpuset_cpus`) ensures performance isolation between concurrent desktop sessions.

7. **Resource Limits**: Per-container memory limits, CPU shares, and shared memory (2GB shm_size) prevent resource contention.

8. **Image Caching**: Automatic image caching with LRU eviction (50GB max) reduces pull times for frequently used desktop images.

### Negative

1. **Wayland Gap**: Wayland support is planned but not implemented. As major distros (Ubuntu, Fedora) default to Wayland, X11-only support becomes a limitation.

2. **External Tool Dependency**: The X11 implementation depends on xdotool, wmctrl, and import being installed in containers, increasing image size and complexity.

3. **Windows/macOS Not Implemented**: The platform abstraction trait is defined but Windows and macOS implementations are future work, limiting cross-platform claims.

4. **Container Overhead**: Each desktop session requires a full container (2-4GB RAM), limiting concurrent session density compared to process-level isolation.

5. **VNC Security**: VNC passwords, while securely generated, are transmitted over the protocol. Additional tunneling (SSH) is needed for production use.

### Neutral

1. **Docker Dependency**: The primary virtualization backend uses Docker (bollard crate). Podman support exists but is optional.

2. **Image Management**: Base images (~1.5-3GB each) require significant storage. The image cache helps but doesn't eliminate this requirement.

3. **Network Configuration**: Custom bridge networking (`kvs-br0`, 172.16.0.0/24) requires host-level network configuration.

---

## Cross-References

- **ADR-001**: Automation Engine Architecture - the automation engine uses the platform abstraction layer for input simulation
- **ADR-003**: AI Agent Integration - AI agents interact with desktop sessions through the containerized environment
- **SPEC.md**: Section 5 (Virtualization Layer) - detailed specification of container and VM management
- **src/virtualization.rs**: VirtualizationManager implementation with Docker, Podman, and libvirt support
- **src/containerization.rs**: ContainerizationEngine abstraction
- **src/podman_integration.rs**: Rootless container support

---

## Appendix A: Container Resource Configuration

```rust
// Container resource allocation
let host_config = HostConfig {
    port_bindings: Some(port_bindings),
    memory: Some((memory_mb * 1024 * 1024) as i64),  // Bytes
    nano_cpus: Some(cpu_cores as i64 * 1_000_000_000),
    shm_size: Some(2147483648),  // 2GB shared memory for desktop
    cpuset_cpus: Some(self.get_cpu_affinity(cpu_cores)),
    restart_policy: Some(RestartPolicy {
        name: Some(RestartPolicyNameEnum::UNLESS_STOPPED),
        maximum_retry_count: Some(3),
    }),
    memory_swappiness: Some(10),  // Reduce swap usage
    oom_kill_disable: Some(false),
    ..Default::default()
};
```

### CPU Affinity Calculation

```rust
fn get_cpu_affinity(&self, cpu_cores: u32) -> String {
    // Avoid CPU 0 (system-critical)
    let start_cpu = 1;
    let end_cpu = min(start_cpu + cpu_cores - 1, num_cpus::get() as u32 - 1);

    if cpu_cores == 1 {
        start_cpu.to_string()
    } else {
        format!("{}-{}", start_cpu, end_cpu)
    }
}
```

---

## Appendix B: Rust Code Example - Multi-Platform Trait

```rust
use async_trait::async_trait;
use anyhow::Result;

/// Platform-agnostic input simulation trait
#[async_trait]
pub trait PlatformInput: Send + Sync {
    /// Move cursor to absolute coordinates
    async fn move_cursor(&self, x: f64, y: f64) -> Result<()>;

    /// Click mouse button at current position
    async fn click(&self, button: MouseButton) -> Result<()>;

    /// Type a single character
    async fn type_character(&self, char: char) -> Result<()>;

    /// Press a special key (Return, Escape, etc.)
    async fn press_key(&self, key: &str) -> Result<()>;

    /// Capture full desktop screenshot
    async fn screenshot(&self) -> Result<Vec<u8>>;

    /// Find window by name/class pattern
    async fn find_window(&self, pattern: &str) -> Result<Vec<WindowInfo>>;

    /// Get window geometry (position, size)
    async fn get_window_geometry(&self, window_id: &str) -> Result<WindowGeometry>;

    /// List all visible windows
    async fn list_windows(&self) -> Result<Vec<WindowInfo>>;

    /// Launch an application by name
    async fn launch_app(&self, name: &str) -> Result<ProcessInfo>;
}

/// X11 implementation (current)
pub struct X11Input {
    display: String,
}

#[async_trait]
impl PlatformInput for X11Input {
    async fn move_cursor(&self, x: f64, y: f64) -> Result<()> {
        Command::new("xdotool")
            .env("DISPLAY", &self.display)
            .args(["mousemove", &x.to_string(), &y.to_string()])
            .output()
            .await?;
        Ok(())
    }

    async fn screenshot(&self) -> Result<Vec<u8>> {
        let output = Command::new("import")
            .env("DISPLAY", &self.display)
            .args(["-window", "root", "-format", "png", "-"])
            .output()
            .await?;
        Ok(output.stdout)
    }

    // ... other methods
}

/// Wayland implementation (planned)
pub struct WaylandInput {
    compositor: String,
    // Uses wlr-virtual-pointer-unstable-v1 protocol
}

#[async_trait]
impl PlatformInput for WaylandInput {
    async fn move_cursor(&self, x: f64, y: f64) -> Result<()> {
        // TODO: Implement via virtual-pointer protocol
        unimplemented!("Wayland input simulation pending")
    }

    async fn screenshot(&self) -> Result<Vec<u8>> {
        // TODO: Implement via wlr-screencopy protocol
        unimplemented!("Wayland screenshot pending")
    }

    // ... other methods
}
```

---

## Appendix C: Container Image Strategy

```
Image Layer Structure:
┌─────────────────────────────────────────────────────────────┐
│  Layer 1: Base OS (Ubuntu 24.04)           ~80MB           │
├─────────────────────────────────────────────────────────────┤
│  Layer 2: X11 + Xvfb + x11vnc              ~120MB          │
├─────────────────────────────────────────────────────────────┤
│  Layer 3: Desktop Environment (KDE/XFCE)   ~800MB-1.5GB    │
├─────────────────────────────────────────────────────────────┤
│  Layer 4: Applications (Firefox, etc.)     ~500MB-1GB      │
├─────────────────────────────────────────────────────────────┤
│  Layer 5: Automation Tools (xdotool, etc.) ~50MB           │
├─────────────────────────────────────────────────────────────┤
│  Layer 6: KDesktopVirt Agent               ~20MB           │
├─────────────────────────────────────────────────────────────┤
│  Total: ~1.5-3GB per image                                  │
└─────────────────────────────────────────────────────────────┘
```

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-04-03 | Phenotype Architecture Team | Initial ADR |
