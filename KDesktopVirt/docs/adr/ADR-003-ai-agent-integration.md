# ADR-003: AI Agent Integration

**Document ID:** PHENOTYPE_KDESKTOPVIRT_ADR_003  
**Status:** Proposed  
**Last Updated:** 2026-04-03  
**Author:** Phenotype Architecture Team  
**Supersedes:** N/A  
**Related:** ADR-001, ADR-002, SPEC.md

---

## Context

KDesktopVirt's primary differentiator is being designed for AI agents as first-class consumers, not as an afterthought. The platform must provide seamless integration with AI models for desktop automation while maintaining self-hosted deployment capability.

### AI Agent Requirements

```
AI Agent Desktop Control Requirements:
┌─────────────────────────────────────────────────────────────┐
│  1. Structured Tool Interface                               │
│     ├── Well-defined tool schemas (JSON Schema)             │
│     ├── Consistent error responses                          │
│     └── Tool discovery (tools/list)                         │
│                                                             │
│  2. State Understanding                                     │
│     ├── Screenshot capture for visual state                 │
│     ├── Session metadata for programmatic state             │
│     └── Recording access for historical state               │
│                                                             │
│  3. Action Execution                                        │
│     ├── Atomic actions (click, type, move)                  │
│     ├── Composite workflows (multi-step sequences)          │
│     └── Natural interaction (WindMouse, human-like typing)  │
│                                                             │
│  4. Session Management                                      │
│     ├── Create/dispose desktop sessions                     │
│     ├── Monitor session health                              │
│     └── Isolate agent sessions from each other              │
│                                                             │
│  5. Self-Hosted Capability                                  │
│     ├── No cloud dependency for core functionality          │
│     ├── Local model inference support                       │
│     └── Offline operation capability                        │
└─────────────────────────────────────────────────────────────┘
```

### Integration Alternatives Considered

```
Alternative 1: Direct API Integration
┌─────────────────────────────────────────────────────┐
│  Pros: Simple, direct control, low latency          │
│  Cons: Custom integration per AI model, no standard │
│  Result: Rejected - doesn't scale across models     │
└─────────────────────────────────────────────────────┘

Alternative 2: Custom Agent Protocol
┌─────────────────────────────────────────────────────┐
│  Pros: Tailored to KDesktopVirt's needs             │
│  Cons: Requires AI model vendors to adopt it        │
│  Result: Rejected - chicken-and-egg adoption problem│
└─────────────────────────────────────────────────────┘

Alternative 3: Model Context Protocol (MCP)
┌─────────────────────────────────────────────────────┐
│  Pros: Emerging standard, growing ecosystem         │
│  Cons: New protocol, evolving specification         │
│  Result: ACCEPTED                                   │
└─────────────────────────────────────────────────────┘
```

---

## Decision

We adopt **MCP (Model Context Protocol)** as the primary AI agent integration interface, with a secondary REST API for non-MCP clients:

```
┌─────────────────────────────────────────────────────────────┐
│              AI Agent Integration Architecture               │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  AI Model Layer                                             │
│  ┌──────────────┐ ┌──────────────┐ ┌──────────────┐       │
│  │ Claude       │ │ GPT-4o       │ │ Local LLM    │       │
│  │ (MCP Client) │ │ (MCP Client) │ │ (REST API)   │       │
│  └──────┬───────┘ └──────┬───────┘ └──────┬───────┘       │
│         └────────────────┼────────────────┘                │
│                          │                                 │
│  ┌───────────────────────┴───────────────────────────┐    │
│  │  Integration Layer                                │    │
│  │  ┌─────────────────────────────────────────┐     │    │
│  │  │  MCP Server (stdio/SSE transport)       │     │    │
│  │  │  ┌─────────────────────────────────┐   │     │    │
│  │  │  │ 10 Tools:                       │   │     │    │
│  │  │  │  Session: create, list, info    │   │     │    │
│  │  │  │  Control: move, click, type     │   │     │    │
│  │  │  │  Capture: screenshot            │   │     │    │
│  │  │  │  Recording: start, stop         │   │     │    │
│  │  │  │  Automation: execute_workflow   │   │     │    │
│  │  │  └─────────────────────────────────┘   │     │    │
│  │  └─────────────────────────────────────────┘     │    │
│  │  ┌─────────────────────────────────────────┐     │    │
│  │  │  REST API (Axum)                        │     │    │
│  │  │  POST /api/v1/sessions                  │     │    │
│  │  │  POST /api/v1/sessions/{id}/actions/*   │     │    │
│  │  │  POST /api/v1/automation/run            │     │    │
│  │  │  GET  /api/v1/recordings                │     │    │
│  │  └─────────────────────────────────────────┘     │    │
│  └───────────────────────────────────────────────────┘    │
│                          │                                 │
│  ┌───────────────────────┴───────────────────────────┐    │
│  │  Core Engine                                      │    │
│  │  ┌──────────────┐ ┌──────────────┐ ┌───────────┐ │    │
│  │  │ Automation   │ │ Virtualiza-  │ │ Recording │ │    │
│  │  │ Engine       │ │ tion Manager │ │ Pipeline  │ │    │
│  │  └──────────────┘ └──────────────┘ └───────────┘ │    │
│  └───────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

### MCP Tool Design

Each tool follows a consistent pattern:

```rust
// Tool definition structure
struct McpTool {
    name: String,           // e.g., "kvs_create_session"
    description: String,    // Human-readable description
    input_schema: Value,    // JSON Schema for parameters
}

// Example: kvs_create_session
McpTool {
    name: "kvs_create_session".to_string(),
    description: "Create a new desktop automation session".to_string(),
    input_schema: json!({
        "type": "object",
        "properties": {
            "user_id": {
                "type": "string",
                "description": "User identifier for the session"
            },
            "session_name": {
                "type": "string",
                "description": "Optional name for the session"
            },
            "desktop_type": {
                "type": "string",
                "enum": ["ubuntu", "ubuntu-xfce", "ubuntu-kde",
                         "centos", "fedora", "arch", "debian"],
                "description": "Desktop environment type",
                "default": "ubuntu"
            }
        },
        "required": ["user_id"]
    }),
}
```

### Tool Categories

```
MCP Tool Taxonomy:
┌─────────────────────────────────────────────────────────────┐
│  Session Management (3 tools)                               │
│  ├── kvs_create_session    - Provision new desktop          │
│  ├── kvs_list_sessions     - Discover active sessions       │
│  └── kvs_get_session_info  - Inspect session state          │
│                                                             │
│  UI Control (3 tools)                                       │
│  ├── kvs_move_cursor       - Natural cursor movement        │
│  ├── kvs_click             - Click with natural timing      │
│  └── kvs_type_text         - Human-like text input          │
│                                                             │
│  Capture & Recording (3 tools)                              │
│  ├── kvs_screenshot        - Visual state capture           │
│  ├── kvs_start_recording   - Begin video capture            │
│  └── kvs_stop_recording    - End and save recording         │
│                                                             │
│  Automation (1 tool)                                        │
│  └── kvs_execute_workflow  - Multi-step automation          │
└─────────────────────────────────────────────────────────────┘
```

### MCP Server Implementation

```rust
pub struct McpServer {
    api: Arc<KVirtualStageAPI>,
    tools: Vec<McpTool>,
    resources: Vec<McpResource>,
    active_sessions: Arc<RwLock<HashMap<String, String>>>,
}

impl McpServer {
    pub async fn handle_request(&self, request: McpRequest) -> McpResponse {
        match request.method.as_str() {
            "initialize"      => self.handle_initialize(id).await,
            "tools/list"      => self.handle_list_tools(id).await,
            "resources/list"  => self.handle_list_resources(id).await,
            "tools/call"      => self.handle_tool_call(id, request.params).await,
            "resources/read"  => self.handle_read_resource(id, request.params).await,
            _                 => self.method_not_found(id).await,
        }
    }
}
```

### AI Agent Workflow Example

```
AI Agent Desktop Automation Flow:
┌─────────────────────────────────────────────────────────────┐
│  Agent Goal: "Open calculator, compute 123 * 456"           │
│                                                             │
│  Step 1: Create session                                     │
│  └── kvs_create_session({user_id: "agent", desktop: "ubuntu"})│
│      → "Session created: abc-123"                          │
│                                                             │
│  Step 2: Capture initial state                              │
│  └── kvs_screenshot({session_id: "abc-123"})                │
│      → [screenshot data]                                    │
│                                                             │
│  Step 3: AI analyzes screenshot, decides to click launcher  │
│  └── kvs_move_cursor({session_id: "abc-123", x: 50, y: 950})│
│      → "Cursor moved to (50, 950)"                          │
│                                                             │
│  Step 4: Click launcher                                     │
│  └── kvs_click({session_id: "abc-123"})                     │
│      → "Left click executed"                                │
│                                                             │
│  Step 5: Type "calculator"                                  │
│  └── kvs_type_text({session_id: "abc-123", text: "calculator"})│
│      → "Typed text naturally"                               │
│                                                             │
│  Step 6: Press Enter                                        │
│  └── kvs_execute_workflow({session_id: "abc-123",           │
│       workflow: {steps: [{action_type: "key", ...}]}})      │
│      → "Workflow completed: 1/1 steps"                      │
│                                                             │
│  Step 7: Capture result, perform calculation via UI         │
│  └── [repeat click/type cycle for calculator input]         │
│                                                             │
│  Step 8: Capture final result                               │
│  └── kvs_screenshot({session_id: "abc-123"})                │
│      → [screenshot showing result: 56088]                   │
│                                                             │
│  Step 9: Clean up                                           │
│  └── Session auto-terminates after timeout                  │
└─────────────────────────────────────────────────────────────┘
```

---

## Consequences

### Positive

1. **Standard Protocol**: MCP is an emerging industry standard, ensuring compatibility with Claude, GPT, and any future MCP-compatible AI model without custom integration.

2. **Tool Discovery**: AI agents can dynamically discover available tools via `tools/list`, enabling adaptive behavior based on platform capabilities.

3. **Structured Parameters**: JSON Schema validation ensures AI agents provide correctly typed parameters, reducing runtime errors.

4. **Resource Access**: MCP resources (`kvs://sessions`, `kvs://capabilities`) provide structured access to platform state beyond tool calls.

5. **Transport Flexibility**: MCP supports both stdio (local) and SSE (remote) transports, enabling both local agent integration and cloud-based AI access.

6. **Session Isolation**: Each AI agent operates in its own containerized session, preventing cross-agent interference and enabling parallel automation.

7. **Natural Interaction**: All UI control tools use WindMouse 2.0 and NaturalTypingEngine internally, ensuring AI-generated actions look human-like in recordings.

8. **Workflow Composition**: The `kvs_execute_workflow` tool enables multi-step automation in a single call, reducing agent round-trips and improving efficiency.

9. **Self-Hosted**: The MCP server runs locally as part of KDesktopVirt, requiring no cloud connectivity for core functionality.

10. **Audit Trail**: All MCP tool calls are logged, providing a complete audit trail of AI agent actions for debugging and compliance.

### Negative

1. **MCP Maturity**: MCP is a relatively new protocol (2024). The specification may evolve, requiring updates to the server implementation.

2. **Latency**: Each tool call requires a round-trip through the MCP protocol (JSON-RPC serialization/deserialization), adding ~5-10ms overhead per call.

3. **No Streaming**: MCP doesn't natively support streaming tool results. Long-running workflows must poll for completion or use separate notification mechanisms.

4. **Tool Limitations**: 10 tools may be insufficient for complex automation scenarios. Extending the tool set requires careful schema design to avoid overwhelming AI agents.

5. **State Management**: The MCP server maintains session state in memory (`active_sessions` HashMap). Server restarts lose session mappings, requiring re-discovery.

6. **AI Model Dependency**: While the infrastructure is self-hosted, the AI reasoning layer (Claude, GPT-4V) still requires external API calls unless local models are deployed.

### Neutral

1. **JSON-RPC Protocol**: MCP uses JSON-RPC 2.0, which is verbose compared to binary protocols. This is acceptable for AI agent integration but may be inefficient for high-throughput scenarios.

2. **Error Handling**: MCP errors use numeric codes (-32601 for method not found, -32000 for tool errors). These must be mapped to meaningful messages for AI agents.

3. **Authentication**: The current MCP implementation doesn't include authentication. Production deployments should add API key or token-based auth.

---

## Cross-References

- **ADR-001**: Automation Engine Architecture - the automation engine provides the core functionality exposed through MCP tools
- **ADR-002**: Cross-Platform Strategy - MCP tools operate on containerized desktop sessions managed by the virtualization layer
- **SPEC.md**: Section 6 (AI Automation Engine) - detailed specification of AI integration patterns
- **src/mcp.rs**: Full MCP server implementation with 10 tools and 2 resources
- **src/api.rs**: REST API surface that MCP tools delegate to

---

## Appendix A: MCP Tool Implementation Details

```rust
// Tool: kvs_execute_workflow
async fn tool_execute_workflow(&self, arguments: &Value) -> Result<String> {
    let params: WorkflowParams = serde_json::from_value(arguments.clone())?;

    // Convert workflow definition to internal format
    let steps: Vec<WorkflowStep> = params.workflow.steps.into_iter().map(|step| {
        let action = match step.action_type.as_str() {
            "move_cursor" => {
                let x = step.parameters.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let y = step.parameters.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                StepAction::MoveCursor { to: Point::new(x, y) }
            }
            "click" => {
                let x = step.parameters.get("x").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let y = step.parameters.get("y").and_then(|v| v.as_f64()).unwrap_or(0.0);
                let button = match step.parameters.get("button").and_then(|v| v.as_str()) {
                    Some("right") => MouseButton::Right,
                    Some("middle") => MouseButton::Middle,
                    _ => MouseButton::Left,
                };
                StepAction::Click { position: Point::new(x, y), button }
            }
            "type" => {
                let text = step.parameters.get("text")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                StepAction::Type { text }
            }
            _ => StepAction::Type {
                text: format!("Unknown action: {}", step.action_type)
            }
        };

        WorkflowStep {
            name: step.name,
            action,
            timeout: step.timeout_seconds.map(|s| Duration::from_secs(s)),
        }
    }).collect();

    let workflow = AutomationWorkflow {
        name: params.workflow.name.clone(),
        description: params.workflow.description.unwrap_or_default(),
        continue_on_error: params.workflow.continue_on_error.unwrap_or(false),
        steps,
    };

    let result = self.api.execute_workflow(&params.session_id, workflow).await?;

    if result.success {
        Ok(format!(
            "Workflow '{}' completed successfully!\nSteps: {}/{}\nTime: {}ms",
            result.workflow_name,
            result.successful_steps,
            result.total_steps,
            result.execution_time_ms
        ))
    } else {
        Ok(format!(
            "Workflow '{}' completed with errors.\nSteps: {}/{}\nErrors: {}",
            result.workflow_name,
            result.successful_steps,
            result.total_steps,
            result.errors.join(", ")
        ))
    }
}
```

---

## Appendix B: MCP Client Configuration

### Claude Desktop

```json
{
  "mcpServers": {
    "kvirtualstage": {
      "command": "kvirtualstage",
      "args": ["mcp", "start", "--port", "3001"]
    }
  }
}
```

### Docker-Based

```json
{
  "mcpServers": {
    "kvirtualstage": {
      "command": "docker",
      "args": [
        "run", "-it", "--rm",
        "-v", "/var/run/docker.sock:/var/run/docker.sock",
        "-p", "3001:3001",
        "kooshapari/kvirtualstage:latest",
        "mcp", "start", "--port", "3001"
      ]
    }
  }
}
```

---

## Appendix C: Local Model Integration

```rust
/// Support for local model inference
pub enum AIModel {
    /// Local UI-TARS deployment
    UITars {
        endpoint: String,      // e.g., "http://localhost:8000"
        model_size: String,    // e.g., "7b", "72b"
    },
    /// Cloud API (Anthropic Claude)
    Claude {
        api_key: String,
        model: String,         // e.g., "claude-3-5-sonnet-20241022"
    },
    /// Cloud API (OpenAI GPT-4V)
    GPT4V {
        api_key: String,
        model: String,         // e.g., "gpt-4o"
    },
}

impl AIAgentEngine {
    async fn predict_action(
        &self,
        screenshot: &[u8],
        task: &str,
        history: &[AgentState],
    ) -> Result<PredictedAction> {
        match &self.model {
            AIModel::UITars { endpoint, model_size } => {
                // Local inference via HTTP
                let response = reqwest::Client::new()
                    .post(format!("{}/predict", endpoint))
                    .json(&UITarsRequest {
                        image: base64::encode(screenshot),
                        task: task.to_string(),
                        history: history.iter().map(|s| s.action.clone()).collect(),
                    })
                    .send()
                    .await?
                    .json::<UITarsResponse>()
                    .await?;

                Ok(PredictedAction::from(response))
            }
            AIModel::Claude { api_key, model } => {
                // Cloud API call to Anthropic
                self.predict_with_claude(api_key, model, screenshot, task, history).await
            }
            AIModel::GPT4V { api_key, model } => {
                // Cloud API call to OpenAI
                self.predict_with_gpt4v(api_key, model, screenshot, task, history).await
            }
        }
    }
}
```

---

## Appendix D: Security Considerations for AI Agent Integration

```
AI Agent Security Model:
┌─────────────────────────────────────────────────────────────┐
│  1. Session Isolation                                       │
│     ├── Each agent gets its own container                   │
│     ├── No cross-session access                             │
│     └── Resource limits prevent abuse                       │
│                                                             │
│  2. Tool Permissions                                        │
│     ├── Configurable tool access per agent                  │
│     ├── Sensitive tools require additional auth             │
│     └── Audit logging for all tool calls                    │
│                                                             │
│  3. Credential Management                                   │
│     ├── Encrypted vault (AES-256-GCM)                       │
│     ├── Argon2 password hashing                             │
│     └── Credential injection only on explicit request       │
│                                                             │
│  4. Network Isolation                                       │
│     ├── Container network policies (deny-by-default)        │
│     ├── No outbound access unless explicitly allowed        │
│     └── VNC ports bound to 127.0.0.1 only                   │
│                                                             │
│  5. Rate Limiting                                           │
│     ├── Per-session action rate limits                      │
│     ├── Workflow execution timeouts                         │
│     └── Concurrent session limits                           │
└─────────────────────────────────────────────────────────────┘
```

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-04-03 | Phenotype Architecture Team | Initial ADR (Proposed) |
