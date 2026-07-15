# KVirtualStage API Framework

## Overview

KVirtualStage is a comprehensive API framework providing Playwright-equivalent desktop automation capabilities for AI agents. Following the Playwright model, it offers multiple interfaces and language bindings for seamless integration across different environments and use cases.

## 🎯 Framework Architecture

### Core Components

```
┌─────────────────────────────────────────────────────────────┐
│                    KVirtualStage API Framework              │
├─────────────────────────────────────────────────────────────┤
│  Language Bindings     │  Interfaces      │  Integration    │
│  ─────────────────     │  ──────────────  │  ─────────────  │
│  • Rust (Core)        │  • CLI Commands  │  • MCP Server   │
│  • Python (PyO3)      │  • TUI Monitor   │  • REST API     │
│  • Node.js (NAPI)     │  • Web UI        │  • WebSocket    │
│  • C/C++ (FFI)        │  • REPL          │  • OpenAPI      │
├─────────────────────────────────────────────────────────────┤
│                     Core Automation Engine                  │
│  • Natural Movement   • Human Typing     • Session Mgmt   │
│  • Recording/Playback • Workflow Engine  • Security       │
├─────────────────────────────────────────────────────────────┤
│                  Virtualization Platform                   │
│  • Docker/Podman     • Desktop Envs      • Cross-Platform │
│  • VNC/X11           • Audio Support     • GPU Accel      │
└─────────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### Installation

```bash
# Install from source
git clone https://github.com/KooshaPari/KVirtualStage.git
cd KVirtualStage
cargo build --release

# Install binaries
cargo install --path .
```

### Basic Usage

#### 1. Start the System

```bash
# Start API server
kvirtualstage server --port 8080

# Or start with TUI
kvirtualstage tui

# Or start MCP server for AI agents
kvirtualstage mcp start --port 3001
```

#### 2. Create Session

```bash
# Create a new desktop session
kvirtualstage session create --name demo --desktop ubuntu

# List sessions
kvirtualstage session list
```

#### 3. Automation

```bash
# Direct automation commands
kvirtualstage auto move --session demo --x 400 --y 300
kvirtualstage auto click --session demo --button left
kvirtualstage auto type --session demo "Hello KVirtualStage!"

# Execute workflows
kvirtualstage workflow create --template calculator
kvirtualstage workflow run --session demo --file workflow.json
```

## 📚 Language Bindings

### Python

```python
import asyncio
import kvirtualstage as kvs

async def main():
    # Create automation instance
    automation = kvs.KVirtualStage()
    
    # Create session
    session = await automation.create_session(
        user_id="demo_user",
        desktop_type=kvs.DesktopType.UBUNTU
    )
    
    # Automation sequence
    await session.move_cursor(400, 300)
    await session.click()
    await session.type_text("Hello from Python!")
    
    # Workflow execution
    workflow = kvs.Workflow("Calculator Demo")
    workflow.move_cursor(100, 100).click().type_text("2 + 2 =")
    result = await session.execute_workflow(workflow)
    
    await session.close()

asyncio.run(main())
```

### Node.js

```javascript
const kvs = require('kvirtualstage');

async function main() {
    // Create automation instance
    const automation = new kvs.KVirtualStage();
    
    // Create session
    const session = await automation.createSession({
        userId: 'demo_user',
        desktopType: kvs.DesktopType.UBUNTU
    });
    
    // Automation sequence
    await session.moveCursor(400, 300);
    await session.click();
    await session.typeText('Hello from Node.js!');
    
    // Workflow execution
    const workflow = new kvs.Workflow('Text Editor Demo');
    workflow
        .moveCursor(200, 150)
        .click()
        .typeText('Automated text input');
    
    const result = await session.executeWorkflow(workflow);
    console.log(`Workflow success: ${result.success}`);
    
    await session.close();
}

main().catch(console.error);
```

### C/C++

```c
#include "kvirtualstage.h"
#include <stdio.h>

int main() {
    // Initialize
    if (kvs_init() != KVS_SUCCESS) {
        return 1;
    }
    
    // Create session
    char session_id[256];
    kvs_create_session("demo_user", "demo_session", "ubuntu", 
                      session_id, sizeof(session_id));
    
    // Automation
    kvs_move_cursor(session_id, 400.0, 300.0);
    kvs_click(session_id, "left");
    kvs_type_text(session_id, "Hello from C!");
    
    // Cleanup
    kvs_remove_session(session_id);
    kvs_shutdown();
    return 0;
}
```

## 🔌 API Interfaces

### REST API

```bash
# Session management
POST /api/v1/sessions
GET /api/v1/sessions
GET /api/v1/sessions/{id}
DELETE /api/v1/sessions/{id}

# Automation control
POST /api/v1/sessions/{id}/cursor/move
POST /api/v1/sessions/{id}/mouse/click
POST /api/v1/sessions/{id}/keyboard/type

# Recording
POST /api/v1/sessions/{id}/recording/start
POST /api/v1/sessions/{id}/recording/stop

# Workflow execution
POST /api/v1/sessions/{id}/workflow

# Health and metrics
GET /api/v1/health
GET /api/v1/metrics
```

### WebSocket Streaming

```javascript
// Live desktop streaming
const ws = new WebSocket('ws://localhost:8080/api/v1/sessions/demo/stream');

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    if (data.type === 'frame') {
        // Display desktop frame
        displayFrame(data.frame);
    }
};
```

### MCP Server (AI Agents)

```json
{
  "tools": [
    {
      "name": "kvs_create_session",
      "description": "Create new desktop automation session",
      "input_schema": {
        "type": "object",
        "properties": {
          "user_id": {"type": "string"},
          "desktop_type": {"type": "string", "enum": ["ubuntu", "ubuntu-xfce", "ubuntu-kde"]}
        }
      }
    },
    {
      "name": "kvs_move_cursor", 
      "description": "Move cursor with natural movement",
      "input_schema": {
        "type": "object",
        "properties": {
          "session_id": {"type": "string"},
          "x": {"type": "number"},
          "y": {"type": "number"}
        }
      }
    }
  ]
}
```

## 🎮 TUI Interface

Launch the interactive terminal interface:

```bash
kvirtualstage tui
```

Features:
- Real-time session monitoring
- Live automation control
- Performance metrics visualization
- Session management
- Recording controls
- Workflow execution

Navigation:
- `1-4`: Switch between tabs (Sessions, Automation, Recording, Metrics)
- `↑/↓`, `j/k`: Navigate lists
- `Enter`: Select/View details
- `c/m/t`: Execute automation commands
- `r/s`: Start/stop recording
- `q`: Quit

## 🔧 Configuration

### Environment Variables

```bash
# API Configuration
export KVS_API_HOST="0.0.0.0"
export KVS_API_PORT="8080"
export KVS_MCP_PORT="3001"

# Desktop Configuration
export KVS_DEFAULT_DESKTOP="ubuntu"
export KVS_CONTAINER_RUNTIME="docker"

# Performance Configuration
export KVS_MAX_SESSIONS="10"
export KVS_RESPONSE_TIMEOUT="30s"
export KVS_RECORDING_QUALITY="medium"
```

### Configuration File

```toml
# ~/.config/kvirtualstage/config.toml
[api]
host = "0.0.0.0"
port = 8080
timeout = "30s"

[desktop]
default_type = "ubuntu"
container_runtime = "docker"
max_sessions = 10

[automation]
natural_movement = true
typing_speed_wpm = 65.0
error_simulation = false

[recording]
default_quality = "medium"
max_duration = "1h"
formats = ["mp4", "webm"]

[security]
session_timeout = "2h"
api_key_required = false
```

## 📊 Performance Specifications

### API Response Times
- Session creation: < 2s
- Cursor movement: < 50ms
- Click execution: < 30ms
- Text typing: < 100ms (varies by length)
- Screenshot: < 200ms
- Workflow execution: < 5s (varies by complexity)

### Resource Requirements
- Memory: 512MB - 2GB per session
- CPU: 1-2 cores per session
- Storage: 1GB for system + recordings
- Network: 10Mbps for streaming

### Scaling Limits
- Max concurrent sessions: 50 (hardware dependent)
- Max workflow steps: 1000
- Max recording duration: 4 hours
- Max text input: 10KB per command

## 🔒 Security Features

### Authentication & Authorization
- API key authentication (optional)
- Session-based access control
- User isolation per session
- Rate limiting and quotas

### Data Protection
- Encrypted credential storage
- Secure session communication
- Audit logging
- Container sandboxing

### Network Security
- TLS/SSL support
- CORS configuration
- Firewall integration
- VPN compatibility

## 🧪 Testing & Validation

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
cargo test --test integration_tests
```

### Performance Benchmarks
```bash
cargo bench
```

### API Testing
```bash
# Health check
curl http://localhost:8080/api/v1/health

# Load testing
wrk -t12 -c400 -d30s http://localhost:8080/api/v1/health
```

## 🚀 Advanced Features

### Workflow Templates
```bash
# Create workflow templates
kvirtualstage workflow create --template calculator
kvirtualstage workflow create --template text-editor
kvirtualstage workflow create --template file-manager
```

### Recording & Playback
```bash
# Start recording
kvirtualstage record --session demo --output demo.mp4 --quality high

# Screenshot capture
kvirtualstage screenshot --session demo --output screenshot.png
```

### AI Agent Integration
```bash
# Start MCP server for AI agents
kvirtualstage mcp start --port 3001

# List available tools
kvirtualstage mcp tools
```

## 📖 Examples & Tutorials

See the `/examples` directory for comprehensive usage examples:

- `examples/api_examples.md` - Complete API usage guide
- `examples/basic_automation.py` - Python automation basics
- `examples/advanced_workflow.js` - Node.js complex workflows
- `examples/c_integration.c` - C/C++ integration examples
- `examples/mcp_demo_script.py` - MCP server usage

## 🛠️ Development

### Building from Source
```bash
git clone https://github.com/KooshaPari/KVirtualStage.git
cd KVirtualStage

# Build all components
cargo build --release

# Build specific binaries
cargo build --bin kvs-server --release
cargo build --bin kvs-tui --features tui --release

# Build with all features
cargo build --features "full-server,all-bindings" --release
```

### Language Binding Development
```bash
# Python bindings
cargo build --features python-bindings
pip install maturin
maturin develop

# Node.js bindings
cargo build --features nodejs-bindings
npm install @napi-rs/cli
napi build --platform

# C FFI headers
cargo build --features c-ffi
cbindgen --output bindings/c/kvirtualstage.h
```

## 📚 Documentation

- [API Reference](docs/api/openapi.yaml) - OpenAPI/Swagger specification
- [Architecture Guide](docs/ARCHITECTURE.md) - System design and components
- [Integration Guide](docs/INTEGRATION.md) - Platform integration patterns
- [Security Guide](docs/SECURITY.md) - Security best practices
- [Performance Guide](docs/PERFORMANCE.md) - Optimization strategies

## 🤝 Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Playwright team for API design inspiration
- Docker/Podman communities for containerization
- MCP protocol contributors for AI agent integration
- Open source automation tool developers

---

**KVirtualStage**: Making desktop automation accessible to AI agents through comprehensive APIs and natural interactions.