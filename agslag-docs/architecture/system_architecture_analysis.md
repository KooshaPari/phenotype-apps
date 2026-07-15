# AGSLAG System Architecture Analysis

## Components Overview

### 1. `agslag/`

- Contains core server code (`src/`)
- Implements an MCP server with multi-agent orchestration
- Provides REST APIs, WebSocket server, and MCP integration
- Acts as a **central hub** for agent coordination and tool proxying

### 2. `agslag/mcp-server/`

- A **standalone backend server**
- Implements REST APIs, WebSocket, NATS messaging
- Manages agents, projects, workflows, budgets, analytics
- Integrates with Claude, Gemini, GPT clients
- Handles orchestration, notifications, and multi-agent workflows

### 3. `agslag/old-frontend/`

- A **Next.js React frontend**
- Provides UI for agent management, analytics, workflows
- Connects to backend APIs and WebSockets
- Contains components, contexts, hooks, and pages for the platform

### 4. `agslag/jarvis-swe-agent/`

- A **sophisticated AI agent platform**
- Implements multi-agent coordination, tool use, workflow orchestration
- Integrates with MCP servers and LLMs
- Provides browser automation, code analysis, RAG, and more
- Has its own UI components and dashboards
- Can run standalone or connect to other servers

---

## Current Integration

- `old-frontend` connects to `mcp-server` and/or `agslag` backend for APIs and WebSockets
- `jarvis-swe-agent` can connect to MCP servers (including `agslag` or `mcp-server`)
- `agslag` acts as a central MCP server and proxy
- `mcp-server` is a full backend with agent management and orchestration
- There is **some overlap** between `agslag` and `mcp-server` in orchestration and agent management

---

## Strengths

- Modular design with clear separation of frontend, backend, and agent logic
- Use of MCP protocol for standardized agent communication
- Support for multiple LLMs and tools
- Multi-agent coordination capabilities
- Rich UI for management and analytics
- Extensible with plugins and external tools

---

## Weaknesses

- **Duplication of functionality** between `agslag` and `mcp-server`
- Fragmented architecture with multiple servers and agents
- Complex integration paths, potential for inconsistent state
- Multiple startup scripts and configs increase operational complexity
- Some legacy code and partial migrations (e.g., old JSON DB remnants)
- Lack of unified data model and API surface

---

## Integration Opportunities

- **Merge `agslag` and `mcp-server`** into a single unified backend
- Expose a **single MCP server** with all orchestration, agent management, and tool proxying
- Integrate `jarvis-swe-agent` more tightly as a **plugin or module** rather than a separate service
- Simplify the frontend to connect to **one backend endpoint**
- Use a **shared database and message bus** for all components
- Standardize on **MCP protocol** for all agent communication
- Consolidate configuration and startup scripts

---

## Recommendations

1. **Unify Backends:**
   - Merge `agslag` and `mcp-server` into a single service
   - Remove duplicated orchestration and agent management logic
   - Expose a single API and MCP endpoint

2. **Modularize Jarvis:**
   - Integrate `jarvis-swe-agent` as a module/plugin within the unified backend
   - Expose its tools and workflows via MCP
   - Share data and state seamlessly

3. **Simplify Frontend:**
   - Point `old-frontend` to the unified backend
   - Streamline API calls and WebSocket connections
   - Unify authentication and session management

4. **Standardize Protocols:**
   - Use MCP everywhere for agent communication
   - Avoid ad-hoc REST or WebSocket protocols

5. **Consolidate Data:**
   - Use a single database (e.g., MongoDB) for all components
   - Standardize schemas and access patterns

6. **Improve DevOps:**
   - Create unified startup scripts and configs
   - Containerize the entire stack
   - Automate deployment and scaling

---

*Generated on 2025-04-08.*
