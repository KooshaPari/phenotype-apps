# ADR-003: Model Context Protocol (MCP) as Primary Integration Interface

Date: 2025-04-04

## Context

KDesktopVirt requires a standardized interface for external systems to control desktop sessions. This includes:
- AI agents (Claude, GPT, custom agents)
- CI/CD pipelines
- Human operators via CLI/GUI
- Other automation frameworks

The integration landscape has fragmented approaches:

| Approach | Protocol | Ecosystem | AI-Native |
|----------|----------|-----------|-----------|
| REST API | HTTP/JSON | Universal | No |
| GraphQL | HTTP | Modern web | No |
| gRPC | HTTP/2 | Microservices | No |
| WebSocket | WS | Real-time | No |
| LangChain Tools | Python | Python AI | Partial |
| MCP | stdio/SSE | Claude/GPT | Yes |

The Model Context Protocol (MCP), introduced by Anthropic in 2024, standardizes how AI systems interact with external tools.

## Decision

We will adopt MCP as the primary integration interface, with REST API as secondary for non-AI clients.

### Architecture

```
Integration Architecture:

┌─────────────────────────────────────────┐
│           Client Layer                  │
├─────────────────────────────────────────┤
│  ┌──────────┐ ┌──────────┐ ┌─────────┐│
│  │  Claude  │ │  Custom  │ │   CI/CD ││
│  │  Desktop │ │   Agent  │ │ Pipeline││
│  └────┬───────┘ └────┬───────┘ └────┬────┘│
│       │              │              │     │
│       └──────────────┼──────────────┘     │
│                      │                     │
│              ┌───────┴───────┐             │
│              │     MCP       │             │
│              │  stdio / SSE  │             │
│              └───────┬───────┘             │
├───────────────────────┼─────────────────────┤
│           KDesktopVirt │ MCP Server          │
│  ┌─────────────────────┴─────────────────┐│
│  │          MCP Tool Handlers              ││
│  │  ├─ create_session                     ││
│  │  ├─ get_sessions                       ││
│  │  ├─ take_screenshot                    ││
│  │  ├─ click_element                      ││
│  │  ├─ type_text                          ││
│  │  ├─ run_automation                     ││
│  │  └─ ...                                ││
│  └────────────────────────────────────────┘│
├─────────────────────────────────────────┤
│  ┌───────────────────────────────────────┐│
│  │     REST API (secondary)              ││
│  │     /api/v1/sessions                  ││
│  │     /api/v1/screenshots               ││
│  │     ...                               ││
│  └───────────────────────────────────────┘│
└─────────────────────────────────────────┘
```

### MCP Server Implementation

The MCP server exposes desktop control as tools:

```typescript
// MCP Server Tool Definition
interface KDesktopVirtMCPServer {
  // Session Management
  create_session(params: {
    name: string;
    desktop_type?: "kubuntu" | "ubuntu" | "debian";
    resources?: { cpu: number; memory_mb: number };
  }): Promise<Session>;
  
  get_sessions(params: {
    status?: "running" | "stopped" | "all";
  }): Promise<Session[]>;
  
  terminate_session(params: {
    session_id: string;
  }): Promise<void>;
  
  // Screenshot & Analysis
  take_screenshot(params: {
    session_id: string;
    format?: "png" | "jpeg";
  }): Promise<Screenshot>;
  
  // UI Interaction
  click_element(params: {
    session_id: string;
    description: string;  // Natural language (AI-powered)
    x?: number;         // Optional explicit coordinates
    y?: number;
  }): Promise<ActionResult>;
  
  type_text(params: {
    session_id: string;
    text: string;
    element_description?: string;
  }): Promise<ActionResult>;
  
  press_key(params: {
    session_id: string;
    key: string;
    modifiers?: string[];
  }): Promise<ActionResult>;
  
  // Automation
  run_automation(params: {
    session_id: string;
    script_id?: string;
    natural_language_task?: string;
    mode: "normal" | "aci_agent";
  }): Promise<AutomationResult>;
  
  // Recording
  start_recording(params: {
    session_id: string;
    output_format?: "mp4" | "webm" | "gif";
  }): Promise<RecordingHandle>;
  
  stop_recording(params: {
    recording_id: string;
  }): Promise<Recording>;
}
```

### Transport Options

MCP supports multiple transports:

| Transport | Use Case | Latency | Complexity |
|-----------|----------|---------|------------|
| stdio | Local CLI integration | Low | Simple |
| SSE | Remote browser clients | Medium | Medium |
| WebSocket | Real-time bidirectional | Low | Medium |

**Default**: stdio for local Claude Desktop integration
**Optional**: SSE for remote access

### Claude Desktop Integration

Configuration for Claude Desktop:

```json
{
  "mcpServers": {
    "kdesktopvirt": {
      "command": "kdesktopvirt",
      "args": ["mcp", "start"],
      "env": {
        "KVIRTUALSTAGE_API_URL": "http://localhost:8080"
      }
    }
  }
}
```

This enables natural language desktop control:

```
User: "Create a new desktop session and open Chrome"
Claude: I'll create a desktop session and open Chrome for you.
       [Calls kdesktopvirt.create_session]
       [Calls kdesktopvirt.click_element with "Chrome icon"]

User: "Take a screenshot of the current state"
Claude: I'll capture the current desktop state.
       [Calls kdesktopvirt.take_screenshot]
       [Returns image for Claude to analyze]
```

### REST API Secondary

For non-MCP clients, a REST API provides equivalent functionality:

```yaml
OpenAPI Specification (simplified):

paths:
  /api/v1/sessions:
    post:
      summary: Create desktop session
      requestBody:
        content:
          application/json:
            schema:
              type: object
              properties:
                name: { type: string }
                desktop_type: { enum: [kubuntu, ubuntu, debian] }
    get:
      summary: List sessions
      
  /api/v1/sessions/{id}/screenshot:
    post:
      summary: Capture screenshot
      
  /api/v1/sessions/{id}/actions/click:
    post:
      summary: Click element
      requestBody:
        content:
          application/json:
            schema:
              oneOf:
                - type: object
                  properties:
                    description: { type: string }  # AI-powered
                - type: object
                  properties:
                    x: { type: number }
                    y: { type: number }
```

## Status

Accepted

## Consequences

### Positive

- **AI-native interface**: Designed for LLM tool use
- **Standardization**: Emerging industry standard (Anthropic, OpenAI supporting)
- **Ecosystem**: Growing tool library, shared conventions
- **Flexibility**: Multiple transports (stdio, SSE, WebSocket)
- **Discoverability**: Self-describing tool schemas

### Negative

- **Emerging standard**: Still evolving, potential breaking changes
- **Limited non-AI clients**: Requires MCP client library
- **Debugging complexity**: Indirect execution via LLM
- **Claude-centric**: Anthropic-led, may favor Claude patterns

### Mitigations

| Concern | Mitigation |
|---------|------------|
| Standard evolution | Abstract MCP layer; adapt to spec changes |
| Non-AI clients | Maintain REST API parity |
| Debugging | Comprehensive logging; replay capability |
| Vendor lock-in | Support multiple AI providers |

### Neutral

- **Tool discovery**: Clients discover tools at runtime
- **Capability negotiation**: Server advertises available tools
- **Security**: Stdio transport provides natural sandboxing

## Alternatives Considered

### Pure REST API

Rejected as primary due to:
- Not optimized for LLM tool calling
- Requires custom prompt engineering
- No standardized discovery

Kept as secondary for broad compatibility.

### gRPC

Rejected due to:
- Limited browser/client support
- Complex for AI integration
- Binary protocol harder for LLMs

### GraphQL

Rejected due to:
- Overkill for this use case
- Limited AI tooling integration
- Complexity for simple operations

### LangChain Tools

Rejected due to:
- Python-specific
- Vendor framework lock-in
- Less standardized than MCP

## Related Decisions

- ADR-001: Container-Based Desktop Virtualization
- ADR-002: AI-Native UI Automation

## References

1. Anthropic. (2024). "Model Context Protocol Specification". https://modelcontextprotocol.io/
2. Anthropic. (2024). "Claude Desktop MCP Integration". Claude Documentation.
3. OpenAI. (2024). "Function Calling API". OpenAI Documentation.
4. LangChain. (2024). "Tool Interface". LangChain Documentation.
