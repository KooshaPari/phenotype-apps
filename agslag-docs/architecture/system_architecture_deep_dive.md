# AGSLAG System Architecture Deep Dive

## 1. `agslag/` (Central MCP Server & Proxy)

- Implements an **MCP server** using the official SDK
- Provides a **proxy layer** to route tool calls to child processes or plugins
- Handles **WebSocket server** for real-time communication
- Exposes **REST APIs** for agents, threads, projects, workflows, analytics, budgets
- Acts as a **central hub** for multi-agent orchestration
- Can spawn and manage **child MCP servers** as plugins
- Integrates with **Claude, GPT, Gemini** via plugin architecture
- Contains **compatibility layers** for legacy JSON DB and MongoDB
- Designed to be **extensible** with new tools and resources

### Strengths
- Flexible plugin architecture
- Centralized control of agent communication
- Supports multiple LLMs and tools
- Good use of MCP protocol for standardization

### Weaknesses
- Some legacy code remains (JSON DB)
- Complex proxy logic
- Overlaps with `mcp-server` functionality
- Needs better modularization

---

## 2. `agslag/mcp-server/` (Standalone Backend Server)

- Full-featured **Express backend**
- Implements **REST APIs** for agents, projects, workflows, budgets, analytics
- Integrates with **Claude, GPT, Gemini** via dedicated clients
- Uses **NATS messaging** for inter-service communication
- Manages **agent registration, status, orchestration**
- Provides **notification and event services**
- Has **dedicated models, controllers, services, routes**
- Designed as a **monolithic backend** for the platform

### Strengths
- Comprehensive backend capabilities
- Supports multi-agent orchestration
- Integrates multiple LLMs
- Uses NATS for scalable messaging

### Weaknesses
- Duplicates orchestration logic from `agslag/`
- Monolithic design limits flexibility
- Separate from MCP server, causing fragmentation
- Could be modularized or merged with `agslag/`

---

## 3. `agslag/old-frontend/` (Next.js Frontend)

- Built with **Next.js App Router**
- Provides UI for:
  - Agent management
  - Workflow orchestration
  - Analytics dashboards
  - Project and budget management
- Connects to backend APIs and WebSockets
- Uses **React Contexts** for state management
- Integrates with **Genkit** and other AI services
- Contains **legacy pages** and **new app directory**

### Strengths
- Modern React architecture
- Rich UI components and dashboards
- Supports multi-agent workflows visually
- Extensible with new features

### Weaknesses
- Some legacy code and routing issues
- Needs cleanup and consolidation
- Could better integrate with unified backend

---

## 4. `agslag/jarvis-swe-agent/` (Advanced AI Agent Platform)

- Implements **multi-agent coordination** and **tool use**
- Provides **browser automation, code analysis, RAG, LLM integration**
- Has **workflow orchestration** and **task management**
- Integrates with MCP servers for tool and resource access
- Contains **UI components** and **dashboards**
- Can run standalone or as a plugin
- Has **advanced error handling, retries, and monitoring**
- Designed for **software engineering tasks** and **agent collaboration**

### Strengths
- Sophisticated multi-agent capabilities
- Rich toolset for code, browser, RAG, LLMs
- Modular and extensible
- Can integrate with other MCP servers

### Weaknesses
- Somewhat siloed from the rest of the platform
- Could be better integrated as a plugin/module
- Overlaps with orchestration logic in other components
- Needs unified data and communication layer

---

## Cross-System Observations

- **Multiple overlapping orchestration layers** (agslag, mcp-server, jarvis)
- **Fragmented data models** and APIs
- **Redundant agent management and tool proxying**
- **Multiple startup scripts and configs**
- **Partial migrations from legacy code**
- **Potential for inconsistent state and duplicated effort**

---

## Deep Integration Recommendations

1. **Unify Orchestration:**
   - Centralize all agent coordination in a single MCP server
   - Remove duplicated orchestration logic
   - Use plugin architecture for extensibility

2. **Modularize Jarvis:**
   - Integrate as a plugin/module within the unified backend
   - Expose its tools and workflows via MCP
   - Share data and state seamlessly

3. **Consolidate Data Layer:**
   - Use a single MongoDB instance
   - Standardize schemas across components
   - Remove legacy JSON DB remnants

4. **Simplify Frontend Integration:**
   - Point frontend to unified backend
   - Streamline API and WebSocket usage
   - Unify authentication and session management

5. **Standardize Protocols:**
   - Use MCP everywhere for agent communication
   - Avoid ad-hoc REST or WebSocket protocols

6. **Improve DevOps:**
   - Containerize the entire stack
   - Automate deployment and scaling
   - Use unified configs and secrets management

7. **Enhance Observability:**
   - Centralized logging and monitoring
   - Unified error handling and retries
   - Performance dashboards for all agents

---

*Generated on 2025-04-08.*
