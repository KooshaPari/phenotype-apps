# SPEC.md - KDesktopVirt

**Document ID:** PHENOTYPE_KDESKTOPVIRT_SPEC  
**Status:** Active Specification  
**Last Updated:** 2026-04-03  
**Author:** Phenotype Architecture Team  
**Version:** 2.0

---

## Table of Contents

1. [Project Overview](#1-project-overview)
2. [Mission and Tenets](#2-mission-and-tenets)
3. [Architecture Overview](#3-architecture-overview)
4. [Component Specification](#4-component-specification)
5. [Virtualization Layer](#5-virtualization-layer)
6. [AI Automation Engine](#6-ai-automation-engine)
7. [Functionality Specification](#7-functionality-specification)
8. [Technical Architecture](#8-technical-architecture)
9. [API Reference](#9-api-reference)
10. [Error Handling](#10-error-handling)
11. [Security](#11-security)
12. [Performance Requirements](#12-performance-requirements)
13. [Configuration](#13-configuration)
14. [Deployment](#14-deployment)
15. [Testing Strategy](#15-testing-strategy)
16. [Observability](#16-observability)
17. [Future Roadmap](#17-future-roadmap)
18. [Glossary](#18-glossary)

---

## 1. Project Overview

### 1.1 Project Name

**KDesktopVirt** (internal crate name: `kvirtualstage`)

### 1.2 Tagline

A Playwright-equivalent desktop automation platform for AI agents.

### 1.3 Description

KDesktopVirt bridges the gap between web automation (Playwright) and desktop automation by providing a comprehensive platform for AI agents to interact with full desktop environments. It combines container-based virtualization with AI-powered UI automation, natural interaction algorithms, and enterprise-grade security.

### 1.4 Technology Stack

| Layer | Technology | Version |
|-------|-----------|---------|
| Core Runtime | Rust (Tokio async) | Edition 2021 |
| Web Framework | Axum + Tower | 0.7 / 0.4 |
| Virtualization | Docker (bollard) | 0.16 |
| Kubernetes | kube + k8s-openapi | 0.87 / 0.20 |
| Desktop Environment | Kubuntu 24.04 / KDE Plasma 6 | Latest |
| UI Automation | xdotool + WindMouse 2.0 | Custom |
| Audio System | PipeWire / PulseAudio | System |
| Security | AES-256-GCM + Argon2 | ring 0.17 / argon2 0.5 |
| Database | SQLite + Redis | sqlx 0.7 / redis 0.24 |
| TUI | Ratatui + Crossterm | 0.28 / 0.27 |
| Language Bindings | PyO3, NAPI-RS | 0.20 / 2.0 |
| WebSocket | Tungstenite | 0.21 |
| Recording | FFmpeg pipeline | External |

### 1.5 Repository

- **Source**: https://github.com/KooshaPari/KVirtualStage
- **Homepage**: https://kvirtualstage.dev
- **License**: MIT

### 1.6 Binary Targets

| Binary | Path | Features | Description |
|--------|------|----------|-------------|
| `kvirtualstage` | `src/main.rs` | Default | Primary CLI |
| `kvs-demo` | `src/bin/demo.rs` | Default | Automation demo |
| `kvs-server` | `src/bin/server.rs` | web-ui | API server |
| `kvs-tui` | `src/bin/tui.rs` | tui | Terminal UI |

### 1.7 Library Target

- **Name**: `kvirtualstage`
- **Type**: `cdylib`, `rlib`
- **Purpose**: Language bindings (Python, Node.js, C FFI)

---

## 2. Mission and Tenets

### 2.1 Mission

KDesktopVirt provides disposable, AI-native desktop environments for automation. It combines container-based virtualization with AI-powered UI automation to enable agents to interact with desktop applications at scale.

### 2.2 Use Cases

- AI agents requiring desktop application access
- Automated testing with video recording
- Ephemeral workspaces for sensitive tasks
- CI/CD pipelines needing GUI automation
- Agent training data generation
- Security research sandboxes

### 2.3 Tenets

#### Tenet 1: Ephemeral by Design

Desktop sessions are disposable, not persistent. They exist for the duration of a task and are then destroyed.

**Implications**:
- Clean state for every automation run
- Reduced security surface
- Predictable resource lifecycle
- Cost optimization through resource reclamation

#### Tenet 2: AI-Native Interface

The system is designed for AI agents first, human users second.

**Implications**:
- Natural language task descriptions
- Screenshot-based state understanding
- Autonomous action execution
- Self-healing to UI changes
- MCP protocol as primary interface

#### Tenet 3: Container Efficiency

Leverage container technology for resource efficiency.

**Implications**:
- 2-3 second cold start (vs minutes for VMs)
- 2-4GB RAM per session (vs 8-16GB for VMs)
- Shared kernel reduces overhead
- Docker ecosystem compatibility

#### Tenet 4: Security through Isolation

Each session runs in an isolated environment.

**Implications**:
- Container namespaces (pid, net, mount)
- Network policies (deny-by-default)
- No persistence unless explicitly configured
- Audit logging of all actions

#### Tenet 5: Recording as First-Class

All automation can be recorded by default.

**Implications**:
- Video capture synchronized with actions
- Replay for debugging and verification
- Documentation generation
- Compliance and audit trails

---

## 3. Architecture Overview

### 3.1 System Architecture

```
+---------------------------------------------------------------------+
|                    KDesktopVirt Platform                             |
+---------------------------------------------------------------------+
|                                                                      |
|  +------------------+  +------------------+  +------------------+   |
|  |   Web UI         |  |   CLI/TUI        |  |   MCP/API        |   |
|  |   (Next.js)      |  |   (Rust)         |  |   (stdio/SSE)    |   |
|  |                  |  |                  |  |                  |   |
|  |  Dashboard       |  |  Interactive     |  |  MCP Tools       |   |
|  |  Session mgmt    |  |  Session list    |  |  REST API        |   |
|  |  Recording       |  |  Quick actions   |  |  WebSocket       |   |
|  |  Analytics       |  |  Log tailing     |  |  GraphQL         |   |
|  +--------+---------+  +--------+---------+  +--------+---------+   |
|           |                      |                       |          |
|           +----------------------+-----------------------+          |
|                                  |                                   |
|  +-------------------------------+-------------------------------+  |
|  |                  Core Engine (Rust/Tokio)                    |  |
|  |                                                               |  |
|  |  +--------------+ +--------------+ +--------------+          |  |
|  |  |   Session    | |  Automation  | |   Recording  |          |  |
|  |  |   Manager    | |   Engine     | |   Pipeline   |          |  |
|  |  |              | |              | |              |          |  |
|  |  |  Lifecycle   | |  WindMouse   | |  FFmpeg      |          |  |
|  |  |  State       | |  2.0         | |  30fps       |          |  |
|  |  |  Metadata    | |  4 modes     | |  MP4/WebM    |          |  |
|  |  +------+-------+ +------+-------+ +------+-------+          |  |
|  |         +----------------+-----------------+                  |  |
|  |  +-------------------------------+-----------------------+    |  |
|  |  |            Resource Manager                    |           |  |
|  |  |   CPU  Memory  Disk  Network quotas           |           |  |
|  |  +-----------------------------------------------+           |  |
|  +---------------------------------------------------------------+  |
|                                                                      |
|  +---------------------------------------------------------------+  |
|  |              Virtualization Layer                              |  |
|  |  +--------------+ +--------------+ +--------------+          |  |
|  |  |   Docker     | | Kubernetes   | |   Desktop    |          |  |
|  |  |   Engine     | |   (K3s)      | |   Sessions   |          |  |
|  |  |              | |              | |              |          |  |
|  |  |  Kubuntu     | |  Scaling     | |  Isolated    |          |  |
|  |  |  24.04       | |  K8s API     | |  env         |          |  |
|  |  |  KDE P6      | |  Multi-node  | |  Apps        |          |  |
|  |  +--------------+ +--------------+ +--------------+          |  |
|  +---------------------------------------------------------------+  |
|                                                                      |
|  +---------------------------------------------------------------+  |
|  |              AI Automation Engine                              |  |
|  |  +--------------+ +--------------+ +--------------+          |  |
|  |  |   UI-TARS    | |   X11        | |   Wayland    |          |  |
|  |  |   Reasoning  | |   Control    | |   (future)   |          |  |
|  |  |              | |              | |              |          |  |
|  |  |  Element     | |  xdotool     | |  Protocol    |          |  |
|  |  |  detection   | |  wmctrl      | |  support     |          |  |
|  |  |  AI-driven   | |  xwininfo    | |              |          |  |
|  |  |  actions     | |              | |              |          |  |
|  |  +--------------+ +--------------+ +--------------+          |  |
|  |                                                               |  |
|  |  +----------------------------------------------------+       |  |
|  |  |            Automation Modes                        |       |  |
|  |  |  +--------+ +--------+ +--------+ +--------+     |       |  |
|  |  |  | Normal | | MCP    | | ACI    | | Record |     |       |  |
|  |  |  | Script | | Live   | | Agent  | | Desktop|     |       |  |
|  |  |  +--------+ +--------+ +--------+ +--------+     |       |  |
|  |  +----------------------------------------------------+       |  |
|  +---------------------------------------------------------------+  |
|                                                                      |
|  +---------------------------------------------------------------+  |
|  |              Audio/Video Layer                                 |  |
|  |  +--------------+ +--------------+ +--------------+          |  |
|  |  |   FFmpeg     | |   PipeWire   | |   TTS        |          |  |
|  |  |   Pipeline   | |   Audio      | |   (LLM)      |          |  |
|  |  |              | |   System     | |              |          |  |
|  |  |  30fps       | |  Virtual     | |  Text to     |          |  |
|  |  |  recording   | |  mic/speaker | |  speech      |          |  |
|  |  |  H.264       | |  Audio       | |  Virtual     |          |  |
|  |  |  encoding    | |  injection   | |  injection   |          |  |
|  |  +--------------+ +--------------+ +--------------+          |  |
|  +---------------------------------------------------------------+  |
|                                                                      |
|  +---------------------------------------------------------------+  |
|  |              Security Layer                                    |  |
|  |  +--------------+ +--------------+ +--------------+          |  |
|  |  |   AES-256    | |   OAuth      | |   Vault      |          |  |
|  |  |   GCM        | |   (Google,   | |   (Cred      |          |  |
|  |  |              | |   Steam)     | |   Storage)   |          |  |
|  |  |  Session     | |  MFA         | |  Encrypted   |          |  |
|  |  |  encryption  | |  SSO         | |  Argon2      |          |  |
|  |  +--------------+ +--------------+ +--------------+          |  |
|  +---------------------------------------------------------------+  |
|                                                                      |
|  +---------------------------------------------------------------+  |
|  |              Storage Layer                                     |  |
|  |  +--------------+ +--------------+ +--------------+          |  |
|  |  |   SQLite     | |   Redis      | |   S3         |          |  |
|  |  |   (State)    | |   (Cache)    | |   (Assets)   |          |  |
|  |  +--------------+ +--------------+ +--------------+          |  |
|  +---------------------------------------------------------------+  |
|                                                                      |
+---------------------------------------------------------------------+
```

### 3.2 Module Structure

```
src/
+-- main.rs              # CLI entry point
+-- lib.rs               # Library exports
+-- cli.rs               # Clap CLI definitions
+-- core.rs              # KVirtualStageCore orchestrator
+-- api.rs               # KVirtualStageAPI surface
+-- api_surface.rs       # API session/workflow types
+-- automation/          # Comprehensive automation platform
+-- automation_engine.rs # WindMouse 2.0 + NaturalTypingEngine
+-- ui_automation.rs     # UiAutomationEngine with gestures
+-- desktop_control.rs   # DesktopControlManager
+-- virtualization.rs    # VirtualizationManager (Docker/Podman/libvirt)
+-- containerization.rs  # ContainerizationEngine abstraction
+-- podman_integration.rs# Rootless container support
+-- desktop_provisioning.rs # Desktop environment setup
+-- resource_manager.rs  # Resource allocation and monitoring
+-- mcp.rs               # MCP server (10 tools)
+-- recording.rs         # Recording management
+-- recording_pipeline.rs# FFmpeg pipeline with quality profiles
+-- ffmpeg_pipeline.rs   # FFmpeg process management
+-- audio.rs             # Audio system
+-- audio_video_engine.rs# AudioVideoEngine (TTS/STT)
+-- audio_video_integration.rs # Audio/Video integration
+-- tts_audio_system.rs  # TTS with virtual audio injection
+-- multimodal_detection.rs # Multi-modal UI element detection
+-- web.rs               # Axum web server
+-- security.rs          # SecurityManager (credential vault)
+-- security_framework.rs# SecurityEngine
+-- security_monitoring.rs# SecurityMonitor + threat analysis
+-- audit_compliance.rs  # AuditEngine + compliance reports
+-- session_storage.rs   # Session persistence
```

### 3.3 Data Flow

```
User/AI Request
    |
    v
+-----------------+
|   Interface     |  CLI / TUI / Web UI / MCP / REST API
+--------+--------+
         |
         v
+-----------------+
|   API Layer     |  KVirtualStageAPI
|                 |  - Session management
|                 |  - Action dispatch
|                 |  - Workflow execution
+--------+--------+
         |
    +----+----+------------+------------+
    v         v            v            v
+-------+ +-------+ +----------+ +----------+
|Session| |Auto   | |Recording | |Security  |
|Manager| |Engine | |Pipeline  | |Framework |
+---+---+ +---+---+ +----+-----+ +----+-----+
    |         |          |           |
    v         v          v           v
+---------------------------------------------------+
|              Virtualization Layer                  |
|  Docker / Podman / Kubernetes                     |
|  +---------------------------------------------+  |
|  |  Container: Xvfb + WM + Apps + x11vnc       |  |
|  +---------------------------------------------+  |
+---------------------------------------------------+
```

---

## 4. Component Specification

### 4.1 Interface Layer

#### 4.1.1 CLI (`src/cli.rs`)

The CLI provides command-line control through Clap-based argument parsing.

**Subcommands**:

```bash
# Session management
kvirtualstage session create --name "test" --desktop kubuntu
kvirtualstage session list --status running
kvirtualstage session stop <id>
kvirtualstage session logs <id>

# Screenshot operations
kvirtualstage screenshot --session <id> --output screenshot.png
kvirtualstage screenshot analyze --session <id> --prompt "Find the submit button"

# Recording
kvirtualstage record start --session <id>
kvirtualstage record stop <recording-id>
kvirtualstage record list

# Automation
kvirtualstage run --session <id> --script workflow.json
kvirtualstage run --session <id> --natural-language "Open Chrome and search for Rust"

# MCP server
kvirtualstage mcp start              # Start MCP server
kvirtualstage mcp install            # Install to Claude Desktop

# Configuration
kvirtualstage config get/set
kvirtualstage status
kvirtualstage doctor                 # Diagnostics

# Server
kvirtualstage server --port 8080 --host 0.0.0.0

# TUI
kvirtualstage tui

# Automation commands
kvirtualstage auto move --session <id> <x> <y>
kvirtualstage auto click --session <id> [--x <x>] [--y <y>] [--button left]
kvirtualstage auto type --session <id> <text>
kvirtualstage auto screenshot --session <id> --output <file>

# Workflow commands
kvirtualstage workflow run --session <id> <file>
kvirtualstage workflow create --output workflow.json --template calculator
kvirtualstage workflow templates
```

**Features**:
- Clap-based argument parsing with derive macros
- Interactive prompts for confirmations
- JSON output mode (`--json`)
- Shell completion generation
- Progress indicators
- Colored output

#### 4.1.2 TUI (`src/bin/tui.rs`)

The Terminal UI provides an interactive terminal interface using Ratatui and Crossterm.

**Screens**:

```
+-----------------------------------------+
|  KDesktopVirt TUI - Sessions            |
+-----------------------------------------+
|  [R]unning  [S]topped  [A]ll            |
|                                         |
|  > session-001  Running  2m  KDE       |
|    session-002  Running  5m  XFCE      |
|    session-003  Stopped  1h  KDE       |
|                                         |
|  [F1] Help  [F2] Create  [F5] Refresh   |
|  [Enter] Connect  [Del] Terminate       |
+-----------------------------------------+
```

**Features**:
- Live session list with real-time updates
- Keyboard shortcuts for all operations
- Log tailing in split-pane
- Quick action menu
- Mouse support

#### 4.1.3 MCP Server (`src/mcp.rs`)

The MCP server exposes desktop control to AI agents through the Model Context Protocol.

**Transport**: stdio (default), SSE (optional)

**Protocol Version**: 2024-11-05

**Available Tools**:

| Tool | Category | Parameters | Description |
|------|----------|-----------|-------------|
| `kvs_create_session` | Session | user_id, session_name?, desktop_type? | Create desktop session |
| `kvs_move_cursor` | Control | session_id, x, y | Natural cursor movement |
| `kvs_click` | Control | session_id, x?, y?, button? | Click at position |
| `kvs_type_text` | Control | session_id, text, wpm? | Natural typing |
| `kvs_screenshot` | Capture | session_id, filename? | Take screenshot |
| `kvs_start_recording` | Recording | session_id, filename?, quality? | Start video capture |
| `kvs_stop_recording` | Recording | session_id | Stop recording |
| `kvs_execute_workflow` | Automation | session_id, workflow | Multi-step automation |
| `kvs_list_sessions` | Session | - | List active sessions |
| `kvs_get_session_info` | Session | session_id | Session details |

**Available Resources**:

| Resource URI | Description |
|-------------|-------------|
| `kvs://sessions` | List of all active desktop automation sessions |
| `kvs://capabilities` | Available automation capabilities and features |

#### 4.1.4 REST API (`src/api.rs`)

Axum-based REST API for non-MCP clients.

**Endpoints**:

```yaml
# Sessions
POST   /api/v1/sessions              # Create session
GET    /api/v1/sessions              # List sessions
GET    /api/v1/sessions/{id}         # Get session
DELETE /api/v1/sessions/{id}         # Terminate session
POST   /api/v1/sessions/{id}/pause  # Pause session
POST   /api/v1/sessions/{id}/resume  # Resume session

# Screenshots
POST   /api/v1/sessions/{id}/screenshot              # Take screenshot
GET    /api/v1/sessions/{id}/screenshot/latest       # Get latest
POST   /api/v1/sessions/{id}/screenshot/analyze      # Analyze with AI

# Actions
POST   /api/v1/sessions/{id}/actions/click           # Click
POST   /api/v1/sessions/{id}/actions/type            # Type text
POST   /api/v1/sessions/{id}/actions/key             # Press key
POST   /api/v1/sessions/{id}/actions/launch          # Launch app

# Automation
POST   /api/v1/automation/run                        # Run script
GET    /api/v1/automation/scripts                    # List scripts
POST   /api/v1/automation/scripts                    # Create script
GET    /api/v1/automation/scripts/{id}               # Get script
DELETE /api/v1/automation/scripts/{id}               # Delete script

# Recording
POST   /api/v1/sessions/{id}/recordings/start        # Start recording
POST   /api/v1/recordings/{id}/stop                  # Stop recording
GET    /api/v1/recordings                            # List recordings
GET    /api/v1/recordings/{id}                       # Get recording
GET    /api/v1/recordings/{id}/download              # Download recording

# Health/Status
GET    /api/v1/health                                # Health check
GET    /api/v1/status                                # System status
GET    /api/v1/metrics                               # Prometheus metrics
```

### 4.2 Core Engine

#### 4.2.1 Session Manager

Manages the lifecycle of desktop sessions.

**Session State Machine**:

```
[Created] --> [Provisioning] --> [Running] --> [Active]
                 |                  |
                 |                  +--> [Recording]
                 |                  +--> [Automation]
                 |                  +--> [Idle] --> [Pausing]
                 |                                     |
                 |                                     v
                 |                               [Paused]
                 |                                     |
                 +-------------------------------------+
                                       |
                                       v
                                [Terminating] --> [Terminated]
```

**State Transitions**:

| From | To | Trigger |
|------|-----|---------|
| Created | Provisioning | `create_session()` called |
| Provisioning | Running | Container started successfully |
| Running | Active | First interaction detected |
| Running | Recording | Recording started |
| Running | Automation | Automation workflow started |
| Active | Idle | No interaction for timeout period |
| Idle | Pausing | Pause requested or timeout |
| Paused | Running | Resume requested |
| Any | Terminating | Terminate requested or error |
| Terminating | Terminated | Cleanup complete |

#### 4.2.2 Automation Engine

See ADR-001 for detailed architecture.

**Four Automation Modes**:

| Mode | Description | Use Case |
|------|-------------|----------|
| Normal Script | Sequential execution with explicit steps | Pre-defined workflows |
| MCP Live | Real-time tool execution for interactive control | AI agent control |
| ACI Agent | Autonomous AI agent desktop control | Self-directed tasks |
| Desktop Recording | Video capture with synchronized automation | Demo generation |

**WindMouse 2.0 Physics Model**:

```
F_total = F_gravity + F_wind + F_tremor + F_context

F_gravity = direction_to_target * gravity * adaptive_strength(progress)
F_wind    = previous_wind * decay + random_noise * wind_strength
F_tremor  = sin(phase * frequency) * amplitude * fatigue_multiplier
F_context = obstacle_avoidance_force + user_preference_force

velocity = (velocity + F_total * dt) * friction
position = position + velocity * dt
```

**Natural Typing Engine**:

- Character-specific timing (punctuation slower than letters)
- Burst typing for common words (60% speed reduction)
- Adjacent-key error simulation (2% base rate)
- Fatigue-based slowdown model
- Natural pauses at word boundaries (10-30% chance, 200-800ms)

---

## 5. Virtualization Layer

### 5.1 Container Orchestration

#### 5.1.1 Docker Integration

Uses the `bollard` crate for async Docker API access.

**Container Configuration**:

```rust
let host_config = HostConfig {
    port_bindings: Some(port_bindings),
    memory: Some((memory_mb * 1024 * 1024) as i64),
    nano_cpus: Some(cpu_cores as i64 * 1_000_000_000),
    shm_size: Some(2147483648),  // 2GB shared memory
    cpuset_cpus: Some(cpu_affinity),
    restart_policy: Some(RestartPolicy {
        name: Some(RestartPolicyNameEnum::UNLESS_STOPPED),
        maximum_retry_count: Some(3),
    }),
    memory_swappiness: Some(10),
    ..Default::default()
};
```

**Environment Variables**:

| Variable | Description | Default |
|----------|-------------|---------|
| `DISPLAY` | X11 display | `:0` |
| `VNC_PASSWORD` | Secure random password | Generated |
| `RESOLUTION` | Desktop resolution | `1920x1080` |
| `DESKTOP_SESSION` | Desktop session type | Varies |
| `XDG_SESSION_DESKTOP` | XDG desktop identifier | Varies |

**Desktop-Specific Environment**:

| Desktop | DESKTOP_SESSION | XDG_SESSION_DESKTOP |
|---------|----------------|--------------------|
| Kubuntu | `plasma` | `KDE` |
| Ubuntu | `ubuntu` | `ubuntu:GNOME` |

#### 5.1.2 Port Pool Management

VNC ports allocated from pool 5900-5999:

```rust
struct PortPool {
    available_ports: Vec<u16>,      // 5900-5999
    allocated_ports: HashMap<u16, String>,  // port -> session_id
    next_port_index: usize,
}
```

**Allocation Strategy**:
- Sequential allocation from pool
- Stale port cleanup on allocation exhaustion
- Port release on session termination

#### 5.1.3 Resource Monitoring

```rust
struct ResourceMonitor {
    cpu_usage: HashMap<String, f64>,
    memory_usage: HashMap<String, u64>,
    disk_usage: HashMap<String, f64>,
    network_stats: HashMap<String, (u64, u64)>,
    last_update: Instant,
}
```

#### 5.1.4 Image Caching

```rust
struct ImageCache {
    cached_images: HashMap<String, CachedImage>,
    base_layers: HashMap<String, String>,
    optimization_settings: ImageOptimization,
}
```

**Cache Policy**:
- Maximum cache size: 50GB
- LRU eviction when threshold exceeded
- Removes 50% of cache when cleanup triggered
- Least recently used images removed first

### 5.2 Hybrid Orchestration

Support for both containers and VMs:

```rust
pub struct VirtualizationConfig {
    pub hybrid_mode: bool,
    pub prefer_containers: bool,
    pub enable_gpu_passthrough: bool,
    pub enable_nested_virtualization: bool,
    pub resource_limits: ResourceLimits,
    pub networking: NetworkConfig,
}

pub struct ResourceLimits {
    pub max_containers: u32,    // Default: 10
    pub max_vms: u32,           // Default: 5
    pub cpu_overcommit_ratio: f64,    // Default: 2.0
    pub memory_overcommit_ratio: f64, // Default: 1.5
}

pub struct NetworkConfig {
    pub bridge_name: String,    // Default: "kvs-br0"
    pub subnet: String,         // Default: "172.16.0.0/24"
    pub dns_servers: Vec<String>, // Default: ["8.8.8.8", "8.8.4.4"]
    pub enable_nat: bool,       // Default: true
}
```

### 5.3 Desktop Types

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

### 5.4 Base Images

| Image | Desktop | Size | Use Case |
|-------|---------|------|----------|
| `ghcr.io/kvirtualstage/kubuntu-desktop:latest` | KDE Plasma 6 | ~3GB | Full desktop testing |
| `ghcr.io/kvirtualstage/ubuntu-desktop:latest` | GNOME | ~2.5GB | Standard Ubuntu |
| `ghcr.io/kvirtualstage/debian-desktop:latest` | XFCE | ~1.5GB | Lightweight testing |

---

## 6. AI Automation Engine

### 6.1 MCP Server Implementation

The MCP server implements the Model Context Protocol with JSON-RPC 2.0 over stdio/SSE.

**Request/Response Flow**:

```
AI Agent                              KDesktopVirt MCP Server
    |                                            |
    +--- initialize ---------------------------> |
    |<--- {protocolVersion, capabilities} -------|
    |                                            |
    +--- tools/list ----------------------------> |
    |<--- {tools: [10 tools]} -------------------|
    |                                            |
    +--- tools/call ----------------------------> |
    |     {name: "kvs_create_session",           |
    |      arguments: {user_id: "agent1",        |
    |                  desktop_type: "ubuntu"}}   |
    |                                            |
    |<--- {content: [{type: "text",              |
    |     text: "Session created: abc-123"}] }---|
```

**Error Response Format**:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32000,
    "message": "Session 'invalid-session' not found",
    "data": null
  }
}
```

### 6.2 AI Model Integration

```rust
pub enum AIModel {
    UITars { endpoint: String, model_size: String },
    GPT4V { api_key: String },
    Claude { api_key: String },
}
```

**Integration Pattern**:

1. Capture screenshot of desktop session
2. Send screenshot + task description to AI model
3. Parse predicted action (click, type, scroll, wait)
4. Execute action through AutomationEngine
5. Record in history for context
6. Repeat until task complete or max steps reached

### 6.3 Automation Modes Detail

#### Mode 1: Normal Scripting

Sequential execution with explicit JSON-defined steps:

```json
{
  "name": "Calculator Demo",
  "description": "Demonstrate calculator usage",
  "continue_on_error": false,
  "steps": [
    {
      "name": "Move to calculator",
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
}
```

#### Mode 2: MCP Live Scripting

Real-time tool calls from AI agents through MCP protocol. Each tool call is executed immediately and results returned.

#### Mode 3: ACI Agent Interface

Autonomous AI agent desktop control with vision-language model:

```rust
pub struct AciAgentAutomation {
    goal: String,
    max_steps: u32,
    model: Arc<dyn VisionLanguageModel>,
    history: Vec<(Screenshot, Action)>,
}
```

**Agent Loop**:
1. Capture screenshot
2. Query AI model for next action
3. Execute action
4. Record in history
5. Check if goal achieved
6. Repeat until goal or max_steps

#### Mode 4: Desktop Recording

Video capture with synchronized automation:

```rust
pub struct DesktopRecordingAutomation {
    recording: RecordingHandle,
    automation: Box<dyn AutomationEngine>,
    ffmpeg: FFmpegPipeline,
}
```

---

## 7. Functionality Specification

### 7.1 Session Management

#### Create Session

**Input**:
- `name`: Session identifier (required)
- `desktop`: Desktop environment type (default: "kubuntu")
- `image`: Custom container image (optional)
- `memory`: Memory allocation in MB (default: 2048)
- `cpu`: CPU cores allocation (default: 2)

**Process**:
1. Generate unique session ID
2. Pull/check base image
3. Allocate VNC port from pool
4. Create container with desktop environment
5. Configure networking and port bindings
6. Start container
7. Store session metadata
8. Return session info

**Output**:
- Session ID
- VNC port
- Container ID
- Status

#### List Sessions

**Input**: None

**Output**:
- Array of session objects with:
  - session_id
  - user_id
  - desktop_type
  - status
  - recording_active
  - created_at
  - last_activity

#### Stop Session

**Input**: Session name/ID

**Process**:
1. Signal graceful shutdown
2. Stop container
3. Update session state
4. Release VNC port

#### Remove Session

**Input**: Session name/ID

**Process**:
1. Force stop container
2. Remove container
3. Cleanup volumes
4. Delete session metadata
5. Release all resources

### 7.2 UI Automation

#### Natural Cursor Movement

**Input**:
- `from`: Current position (Point)
- `to`: Target position (Point)
- `context`: Optional movement context

**Process**:
1. Generate WindMouse 2.0 trajectory
2. Execute frame-by-frame cursor movement
3. Apply natural timing (60 FPS)
4. Update performance metrics

**Output**: Success/failure with timing info

#### Natural Click

**Input**:
- `current_pos`: Current cursor position
- `target`: Target click position
- `button`: Mouse button (Left, Right, Middle)

**Process**:
1. Move cursor naturally to target
2. Natural pre-click pause (50-150ms)
3. Execute mouse click
4. Natural post-click pause (30-100ms)

#### Natural Typing

**Input**:
- `text`: Text to type

**Process**:
1. Generate typing sequence with NaturalTypingEngine
2. Execute character-by-character with natural timing
3. Simulate errors and corrections (2% rate)
4. Apply fatigue model
5. Natural pauses at word boundaries

### 7.3 Screen Capture

#### Screenshot

**Input**:
- `session_id`: Target session
- `output_path`: Output file path (optional)

**Process**:
1. Execute `import -window root` via xdotool
2. Save to specified path or auto-generated filename
3. Return image metadata

#### Recording

**Input**:
- `session_id`: Target session
- `output_path`: Output file path
- `format`: Recording format (mp4, gif, webm)
- `quality`: Quality profile (low, medium, high, streaming)

**Process**:
1. Start FFmpeg x11grab process
2. Apply quality profile settings
3. Record until stop requested
4. Graceful shutdown (SIGINT)
5. Return output file path

### 7.4 Workflow Execution

**Input**:
- `session_id`: Target session
- `workflow`: AutomationWorkflow object

**Process**:
1. Parse workflow steps
2. Execute each step sequentially
3. Handle errors per `continue_on_error` flag
4. Track per-step results
5. Return aggregate results

**Output**:
```rust
pub struct WorkflowResult {
    pub workflow_name: String,
    pub total_steps: usize,
    pub successful_steps: usize,
    pub total_execution_time: Duration,
    pub step_results: Vec<StepResult>,
}
```

---

## 8. Technical Architecture

### 8.1 Async Runtime

Built on Tokio with full async/await support:

```rust
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cmd = KVirtualStageCommand::parse();
    match cmd.execute().await {
        Ok(_) => { info!("KVirtualStage completed successfully"); Ok(()) }
        Err(e) => { eprintln!("Error: {}", e); std::process::exit(1); }
    }
}
```

### 8.2 Error Handling

Uses `anyhow` for application errors and `thiserror` for library errors:

```rust
#[derive(Debug, thiserror::Error)]
pub enum AutomationError {
    #[error("Platform operation failed: {0}")]
    PlatformError(String),

    #[error("Invalid coordinates: ({0}, {1})")]
    InvalidCoordinates(f64, f64),

    #[error("Session not active: {0}")]
    SessionNotActive(String),

    #[error("Automation timeout after {0:?}")]
    Timeout(Duration),
}
```

### 8.3 Serialization

All public types derive `Serialize` and `Deserialize` for JSON interchange:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
```

### 8.4 Feature Flags

```toml
[features]
default = ["tui"]
tui = ["ratatui", "crossterm"]
web-ui = ["axum", "tower", "tower-http", "hyper"]
database = ["sqlx", "redis"]
python-bindings = ["pyo3"]
nodejs-bindings = ["napi", "napi-derive"]
c-ffi = []
kubernetes = ["kube", "k8s-openapi"]
wayland = ["wayland-client"]
x11-support = ["x11", "screenshots"]
audio-support = ["gstreamer", "gstreamer-video", "gstreamer-audio", "libpulse-binding"]
full-desktop = ["x11-support", "audio-support"]
full-server = ["web-ui", "database"]
enterprise = ["full-server", "kubernetes"]
all-bindings = ["python-bindings", "nodejs-bindings", "c-ffi"]
```

### 8.5 Build Profiles

```toml
[profile.release]
lto = "fat"
codegen-units = 1
panic = "abort"
strip = "symbols"
opt-level = 3
overflow-checks = false

[profile.performance]
inherits = "release"
# Ultra-optimized for maximum performance

[profile.size-optimized]
inherits = "release"
opt-level = "z"
# Optimized for smallest binary
```

---

## 9. API Reference

### 9.1 KVirtualStageAPI

The main API surface that all interfaces delegate to:

```rust
pub struct KVirtualStageAPI {
    core: Arc<KVirtualStageCore>,
    sessions: Arc<RwLock<HashMap<String, SessionInfo>>>,
}

impl KVirtualStageAPI {
    pub async fn new() -> Result<Self>;
    pub async fn create_session(&self, user_id: String, name: String, desktop: String) -> Result<String>;
    pub async fn list_sessions(&self) -> Result<Vec<APISessionInfo>>;
    pub async fn get_session_info(&self, session_id: &str) -> Result<APISessionInfo>;
    pub async fn move_cursor(&self, session_id: &str, x: f64, y: f64) -> Result<()>;
    pub async fn click(&self, session_id: &str, button: Option<String>) -> Result<()>;
    pub async fn type_text(&self, session_id: &str, text: &str) -> Result<()>;
    pub async fn start_recording(&self, session_id: &str, filename: &str, quality: Option<String>) -> Result<String>;
    pub async fn stop_recording(&self, session_id: &str) -> Result<String>;
    pub async fn execute_workflow(&self, session_id: &str, workflow: AutomationWorkflow) -> Result<WorkflowExecutionResult>;
}
```

### 9.2 Session Info Types

```rust
pub struct APISessionInfo {
    pub session_id: String,
    pub user_id: String,
    pub desktop_type: String,
    pub status: String,
    pub recording_active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

pub struct WorkflowExecutionResult {
    pub workflow_name: String,
    pub success: bool,
    pub total_steps: usize,
    pub successful_steps: usize,
    pub execution_time_ms: u64,
    pub errors: Vec<String>,
}
```

### 9.3 MCP Tool Schemas

Each MCP tool has a JSON Schema for parameter validation:

```json
{
  "name": "kvs_create_session",
  "description": "Create a new desktop automation session",
  "input_schema": {
    "type": "object",
    "properties": {
      "user_id": { "type": "string", "description": "User identifier" },
      "session_name": { "type": "string", "description": "Optional session name" },
      "desktop_type": {
        "type": "string",
        "enum": ["ubuntu", "ubuntu-xfce", "ubuntu-kde", "centos", "fedora", "arch", "debian"],
        "default": "ubuntu"
      }
    },
    "required": ["user_id"]
  }
}
```

---

## 10. Error Handling

### 10.1 Error Categories

| Category | Source | Recovery |
|----------|--------|----------|
| PlatformError | OS-level operation failed | Retry with backoff |
| InvalidCoordinates | Out-of-bounds coordinates | Clamp to screen bounds |
| SessionNotActive | Session terminated or not found | Re-create session |
| Timeout | Operation exceeded time limit | Retry or abort |
| ContainerError | Docker/Podman operation failed | Check daemon status |
| RecordingError | FFmpeg pipeline failure | Restart pipeline |
| SecurityError | Encryption/auth failure | Re-authenticate |

### 10.2 Error Response Format

**MCP Error**:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -32000,
    "message": "Detailed error message",
    "data": null
  }
}
```

**REST API Error**:
```json
{
  "error": "SessionNotActive",
  "message": "Session 'abc-123' is not active",
  "code": 404,
  "timestamp": "2026-04-03T12:00:00Z"
}
```

**CLI Error**:
```
Error: Session 'abc-123' not found

Caused by:
    No active session with ID 'abc-123'
```

### 10.3 Retry Strategy

- Transient errors (network, container): 3 retries with exponential backoff (100ms, 200ms, 400ms)
- Permanent errors (invalid coordinates, missing session): No retry, immediate failure
- Timeout errors: Single retry with doubled timeout

---

## 11. Security

### 11.1 Credential Vault

AES-256-GCM encryption with Argon2 key derivation:

```rust
pub struct SecurityManager {
    vault_path: PathBuf,
    master_key: Option<Vec<u8>>,
    credentials: HashMap<String, Credential>,
    encryption_enabled: bool,
}
```

**Encryption Flow**:
1. User provides vault password
2. Argon2 hashes password with salt to derive master key
3. AES-256-GCM encrypts credential data with random nonce
4. Nonce prepended to ciphertext, base64-encoded for storage

**Credential Types**:
- OAuth tokens (access_token, refresh_token, scope, expires_in)
- Passwords (username, encrypted password, service)
- Certificates (PEM-encoded, service)
- Generic secrets (key-value pairs)

### 11.2 VNC Security

- Passwords generated using `ring::rand::SecureRandom` (32 bytes)
- Base64-encoded, filtered to alphanumeric, 24 characters
- VNC ports bound to 127.0.0.1 only (no external access)
- Port pool isolation prevents cross-session access

### 11.3 Container Security

| Layer | Mechanism | Purpose |
|-------|----------|---------|
| Namespaces | pid, net, mount, ipc, uts | Process isolation |
| Cgroups | CPU, memory, I/O limits | Resource isolation |
| Capabilities | Dropped privileges | Reduced attack surface |
| Seccomp | Syscall filtering | Kernel protection |
| Network | Bridge isolation, NAT | Network isolation |

### 11.4 OAuth Integration

Support for Google, Steam, and custom OAuth providers:
- OAuth flow initiation with state verification
- Token exchange and storage in encrypted vault
- MFA support for sensitive operations
- Token refresh and rotation

---

## 12. Performance Requirements

### 12.1 Session Creation

| Metric | Target | Notes |
|--------|--------|-------|
| Cold start | 2-3 seconds | Image pull + container start |
| Warm start | <1 second | Image cached |
| Termination | <0.5 seconds | Container stop + remove |

### 12.2 Automation Performance

| Operation | Latency | Notes |
|-----------|---------|-------|
| Cursor movement | 100-500ms | WindMouse trajectory dependent |
| Click execution | <50ms | xdotool round-trip |
| Text typing | Variable | 65 WPM base with natural variation |
| Screenshot capture | 200-500ms | import command |
| Workflow execution | Per-step | Depends on step count and complexity |

### 12.3 Resource Usage

| Metric | Value | Notes |
|--------|-------|-------|
| Binary size (release) | ~15MB | LTO + strip |
| Memory (idle) | ~50MB | Core engine only |
| Memory (per session) | ~2GB | Container + desktop |
| CPU (idle) | <1% | Tokio runtime |
| Concurrent sessions | 30-50 | On 8GB RAM host |

### 12.4 Recording Performance

| Format | Resolution | FPS | CPU Usage |
|--------|-----------|-----|-----------|
| MP4 (H.264) | 1920x1080 | 30 | Medium |
| MP4 (H.264) | 1920x1080 | 60 | High |
| WebM (VP9) | 1920x1080 | 30 | High |
| GIF | 1280x720 | 15 | Low |

---

## 13. Configuration

### 13.1 Environment Variables

```bash
export KVIRTUALSTAGE_CONFIG_PATH="~/.kvirtualstage/config.toml"
export KVIRTUALSTAGE_LOG_LEVEL="info"
export KVIRTUALSTAGE_DOCKER_HOST="unix:///var/run/docker.sock"
```

### 13.2 Configuration File

```toml
[general]
container_runtime = "docker"
default_desktop = "kubuntu"
log_level = "info"

[resources]
default_memory_mb = 2048
default_cpu_cores = 2
default_disk_gb = 10

[recording]
default_format = "mp4"
quality = "high"
fps = 30

[audio]
enable_tts = true
tts_voice = "default"
enable_recording = true

[security]
enable_encryption = true
vault_path = "~/.kvirtualstage/vault"
enable_mfa = false

[mcp]
server_port = 3001
enable_tools = true
stdio_mode = false
auth_required = false
max_sessions = 10
session_timeout = 3600
```

### 13.3 Configuration Commands

```bash
kvirtualstage config init       # Initialize default config
kvirtualstage config show       # Display current config
kvirtualstage config set <key> <value>  # Update config value
```

---

## 14. Deployment

### 14.1 Local Development

```bash
# Clone and build
git clone https://github.com/KooshaPari/KVirtualStage.git
cd KVirtualStage
cargo build --release

# Run demos
cargo build --release --bin kvs-demo
./target/release/kvs-demo

# Start with web UI
./target/release/kvirtualstage start --ui --port 3000

# Start MCP server
./target/release/kvirtualstage mcp start --port 3001
```

### 14.2 Docker Deployment

```bash
# Build image
docker build -t kvirtualstage:latest .

# Run with Docker socket
docker run -it --rm \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -p 3000:3000 \
  -p 3001:3001 \
  kvirtualstage:latest \
  mcp start --port 3001
```

### 14.3 Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: kvirtualstage
spec:
  replicas: 1
  selector:
    matchLabels:
      app: kvirtualstage
  template:
    metadata:
      labels:
        app: kvirtualstage
    spec:
      containers:
      - name: kvirtualstage
        image: kooshapari/kvirtualstage:latest
        ports:
        - containerPort: 3000
        - containerPort: 3001
        volumeMounts:
        - name: docker-sock
          mountPath: /var/run/docker.sock
      volumes:
      - name: docker-sock
        hostPath:
          path: /var/run/docker.sock
```

### 14.4 Docker Compose

```yaml
version: "3.8"
services:
  kvirtualstage:
    build: .
    ports:
      - "3000:3000"
      - "3001:3001"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
    environment:
      - KVIRTUALSTAGE_LOG_LEVEL=info
```

---

## 15. Testing Strategy

### 15.1 Unit Tests

```bash
cargo test
```

Tests cover:
- WindMouse 2.0 trajectory generation
- NaturalTypingEngine sequence generation
- Port pool allocation/release
- Encryption/decryption round-trips
- Workflow parsing and execution

### 15.2 Integration Tests

```bash
cargo test --test integration
```

Tests cover:
- Container lifecycle (create, start, stop, remove)
- MCP server tool calls
- REST API endpoints
- Recording pipeline

### 15.3 Benchmark Tests

```bash
cargo bench
```

Benchmarks cover:
- WindMouse trajectory generation performance
- Container creation latency
- API response times
- Recording throughput

### 15.4 Demo Validation

```bash
./target/release/kvs-demo
```

The demo validates:
- Session creation
- Natural cursor movement
- Click execution
- Text typing
- Workflow execution
- Screenshot capture

---

## 16. Observability

### 16.1 Logging

Built on `tracing` with `tracing-subscriber`:

```bash
export KVIRTUALSTAGE_LOG_LEVEL="debug"  # trace, debug, info, warn, error
```

**Log Format**:
```
2026-04-03T12:00:00.000Z  INFO kvirtualstage::automation_engine: Executing natural cursor movement: (0,0) -> (100,200) with 42 frames
```

### 16.2 Metrics

Prometheus metrics exposed at `/api/v1/metrics`:

- `kvs_sessions_active` - Current active sessions
- `kvs_sessions_created_total` - Total sessions created
- `kvs_actions_executed_total` - Total automation actions
- `kvs_action_duration_seconds` - Action execution time histogram
- `kvs_recording_duration_seconds` - Recording duration histogram
- `kvs_errors_total` - Total errors by type

### 16.3 Health Checks

```bash
GET /api/v1/health
```

**Response**:
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime_seconds": 3600,
  "active_sessions": 5,
  "docker_connected": true,
  "mcp_server_active": true
}
```

---

## 17. Future Roadmap

### Phase 1: Foundation (Complete)

- [x] WindMouse 2.0 physics-based cursor movement
- [x] Natural typing simulation with error patterns
- [x] FFmpeg integration for recording
- [x] Docker container lifecycle management
- [x] MCP server with 10 tools
- [x] Security framework with encrypted vault
- [x] CLI with comprehensive subcommands

### Phase 2: Enhanced Automation (In Progress)

- [ ] Wayland support via virtual-keyboard protocol
- [ ] Python bindings (PyO3)
- [ ] Node.js bindings (NAPI-RS)
- [ ] Kubernetes scaling support
- [ ] Web UI dashboard

### Phase 3: AI Integration (Planned)

- [ ] Local UI-TARS model deployment
- [ ] Multi-model support (GPT-4V, Claude, UI-TARS)
- [ ] Self-healing automation with vision feedback
- [ ] Natural language task execution
- [ ] Agent training data generation pipeline

### Phase 4: Enterprise Features (Planned)

- [ ] Windows desktop support
- [ ] macOS desktop support
- [ ] RBAC and multi-tenant support
- [ ] Audit log export and compliance reporting
- [ ] SSO integration (SAML, OIDC)

---

## 18. Glossary

| Term | Definition |
|------|------------|
| ACI | Agent Control Interface - Protocol for autonomous agent control |
| ADR | Architecture Decision Record |
| AT-SPI | Assistive Technology Service Provider Interface (Linux accessibility) |
| Bollard | Async Docker API client for Rust |
| CDP | Chrome DevTools Protocol |
| DaaS | Desktop-as-a-Service |
| FFmpeg | Multimedia framework for recording/conversion/streaming |
| KDesktopVirt | Project name - desktop automation platform |
| KVirtualStage | Internal crate name for KDesktopVirt |
| MCP | Model Context Protocol - Standard for AI tool integration |
| NoVNC | Browser-based VNC client |
| RDP | Remote Desktop Protocol (Microsoft) |
| TUI | Terminal User Interface |
| UI-TARS | UI Task Automation with Reasoning and Skills |
| VDI | Virtual Desktop Infrastructure |
| VNC | Virtual Network Computing |
| WindMouse | Physics-based cursor movement algorithm |
| Xvfb | X virtual framebuffer (headless X11) |

## Appendix A: Complete CLI Reference

### Session Commands

```bash
# Create a new desktop session
kvirtualstage session create \
  --name "my-session" \
  --desktop kubuntu \
  --image "ghcr.io/kvirtualstage/kubuntu-desktop:latest" \
  --memory 4096 \
  --cpu 4

# List all sessions with filtering
kvirtualstage session list --status running
kvirtualstage session list --status all

# Connect to a session (VNC)
kvirtualstage session connect my-session

# Stop a running session
kvirtualstage session stop my-session

# Remove a session and cleanup
kvirtualstage session remove my-session
```

### Automation Commands

```bash
# Move cursor with natural movement
kvirtualstage auto move --session abc-123 100 200

# Click at position
kvirtualstage auto click --session abc-123 --x 100 --y 200 --button left

# Click at current position
kvirtualstage auto click --session abc-123

# Type text naturally
kvirtualstage auto type --session abc-123 "Hello, World!"

# Take screenshot
kvirtualstage auto screenshot --session abc-123 --output /tmp/screen.png
```

### Workflow Commands

```bash
# Run workflow from JSON file
kvirtualstage workflow run --session abc-123 workflow.json

# Create workflow from template
kvirtualstage workflow create --output calc.json --template calculator
kvirtualstage workflow create --output editor.json --template text-editor

# List available templates
kvirtualstage workflow templates
# Output:
#   calculator    - Basic calculator demonstration
#   text-editor   - Text editor automation
#   file-manager  - File management operations
#   web-browser   - Browser automation
```

### MCP Commands

```bash
# Start MCP server
kvirtualstage mcp start --port 3001

# List available MCP tools
kvirtualstage mcp tools

# Test MCP connection
kvirtualstage mcp test http://localhost:3001
```

### Server Commands

```bash
# Start API server
kvirtualstage server --port 8080 --host 0.0.0.0

# Start with web UI
kvirtualstage start --ui --port 3000 --host localhost

# Start headless
kvirtualstage start
```

## Appendix B: Workflow JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "AutomationWorkflow",
  "type": "object",
  "required": ["name", "steps"],
  "properties": {
    "name": {
      "type": "string",
      "description": "Workflow name"
    },
    "description": {
      "type": "string",
      "description": "Workflow description"
    },
    "continue_on_error": {
      "type": "boolean",
      "default": false,
      "description": "Continue execution on step errors"
    },
    "steps": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["name", "action_type", "parameters"],
        "properties": {
          "name": { "type": "string" },
          "action_type": {
            "type": "string",
            "enum": ["move_cursor", "click", "type", "wait", "key", "screenshot"]
          },
          "parameters": {
            "type": "object",
            "properties": {
              "x": { "type": "number" },
              "y": { "type": "number" },
              "text": { "type": "string" },
              "button": { "type": "string", "enum": ["left", "right", "middle"] },
              "key": { "type": "string" },
              "duration_ms": { "type": "integer" },
              "output": { "type": "string" }
            }
          },
          "timeout_seconds": {
            "type": "integer",
            "default": 30
          }
        }
      }
    }
  }
}
```

## Appendix C: REST API Request/Response Examples

### Create Session

**Request**:
```http
POST /api/v1/sessions
Content-Type: application/json

{
  "name": "test-session",
  "desktop_type": "kubuntu",
  "memory_mb": 2048,
  "cpu_cores": 2
}
```

**Response** (201 Created):
```json
{
  "session_id": "abc-123-def",
  "name": "test-session",
  "desktop_type": "kubuntu",
  "status": "provisioning",
  "vnc_port": 5900,
  "container_id": "xyz789",
  "created_at": "2026-04-03T12:00:00Z"
}
```

### Execute Action

**Request**:
```http
POST /api/v1/sessions/abc-123-def/actions/click
Content-Type: application/json

{
  "x": 100,
  "y": 200,
  "button": "left"
}
```

**Response** (200 OK):
```json
{
  "action": "click",
  "status": "success",
  "execution_time_ms": 145,
  "position": { "x": 100, "y": 200 }
}
```

### Error Response

**Response** (404 Not Found):
```json
{
  "error": "SessionNotActive",
  "message": "Session 'abc-123-def' is not active",
  "code": 404,
  "timestamp": "2026-04-03T12:00:00Z"
}
```

## Appendix D: Container Image Layer Details

```
Layer 1: Base OS (Ubuntu 24.04)           ~80MB
  - Ubuntu minimal installation
  - System libraries and utilities
  - Package manager (apt)

Layer 2: X11 Stack                        ~120MB
  - X.Org Server
  - Xvfb (virtual framebuffer)
  - x11vnc (VNC server)
  - X11 utilities (xprop, xdpyinfo)

Layer 3: Desktop Environment              ~800MB-1.5GB
  - KDE Plasma 6 (Kubuntu) or XFCE (Debian)
  - Window manager (KWin or Xfwm)
  - Display manager (SDDM or LightDM)
  - System settings and utilities

Layer 4: Applications                     ~500MB-1GB
  - Firefox / Chromium
  - LibreOffice (optional)
  - Terminal emulator (Konsole / xfce4-terminal)
  - File manager (Dolphin / Thunar)

Layer 5: Automation Tools                 ~50MB
  - xdotool
  - wmctrl
  - xwininfo
  - import (ImageMagick)
  - FFmpeg

Layer 6: KDesktopVirt Agent               ~20MB
  - kvirtualstage binary (size-optimized)
  - Configuration files
  - Entry point script

Total: ~1.5-3GB per image
```

## Appendix E: Security Threat Model

### Threat 1: Container Escape

**Risk**: Malicious automation script escapes container to host.

**Mitigations**:
- Seccomp profile restricts syscalls
- Dropped capabilities (no CAP_SYS_ADMIN)
- Read-only root filesystem (except /tmp)
- No privileged mode

**Residual Risk**: Low (kernel exploits only)

### Threat 2: Credential Theft

**Risk**: Automation script extracts stored credentials.

**Mitigations**:
- Encrypted vault (AES-256-GCM)
- Argon2 key derivation (memory-hard)
- Credentials only injected on explicit request
- Audit logging of all credential access

**Residual Risk**: Medium (if vault password compromised)

### Threat 3: VNC Eavesdropping

**Risk**: Attacker intercepts VNC traffic.

**Mitigations**:
- VNC bound to 127.0.0.1 only
- Secure random passwords (32 bytes)
- SSH tunneling recommended for remote access

**Residual Risk**: Low (localhost-only binding)

### Threat 4: Resource Exhaustion

**Risk**: Automation script consumes excessive resources.

**Mitigations**:
- Per-container memory limits (cgroups)
- CPU shares allocation
- Session timeout (default: 3600s)
- Max concurrent sessions limit

**Residual Risk**: Low (enforced by cgroups)

## Appendix F: Migration Guide from Legacy

### From Python Scripts

```python
# Old: Python + xdotool
import subprocess
subprocess.run(["xdotool", "mousemove", "100", "200"])
subprocess.run(["xdotool", "click", "1"])
```

```rust
// New: KDesktopVirt Rust API
let mut engine = AutomationEngine::new()?;
engine.click_naturally(
    Point::new(0.0, 0.0),
    Point::new(100.0, 200.0),
    MouseButton::Left,
).await?;
```

### From Selenium Grid

```python
# Old: Selenium Grid
from selenium import webdriver
driver = webdriver.Remote("http://localhost:4444")
driver.get("http://example.com")
driver.find_element(By.ID, "submit").click()
```

```bash
# New: KDesktopVirt MCP
kvs_create_session({user_id: "test", desktop_type: "ubuntu"})
kvs_move_cursor({session_id: "abc", x: 100, y: 200})
kvs_click({session_id: "abc"})
```

---

## Appendix G: Environment Variable Reference

| Variable | Default | Description |
|----------|---------|-------------|
| `KVIRTUALSTAGE_CONFIG_PATH` | `~/.kvirtualstage/config.toml` | Path to configuration file |
| `KVIRTUALSTAGE_LOG_LEVEL` | `info` | Logging level (trace, debug, info, warn, error) |
| `KVIRTUALSTAGE_DOCKER_HOST` | `unix:///var/run/docker.sock` | Docker daemon socket path |
| `KVIRTUALSTAGE_VAULT_PATH` | `~/.kvirtualstage/vault` | Credential vault directory |
| `KVIRTUALSTAGE_RECORDING_DIR` | `./recordings` | Default recording output directory |
| `KVIRTUALSTAGE_MAX_SESSIONS` | `10` | Maximum concurrent sessions |
| `KVIRTUALSTAGE_SESSION_TIMEOUT` | `3600` | Session idle timeout in seconds |

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2025-04-04 | KVirtualStage Team | Initial specification |
| 2.0 | 2026-04-03 | Phenotype Architecture Team | Comprehensive rewrite with full API, security, and deployment specs |

---

*This specification is a living document. Update as the project evolves.*
