# Jarvis SWE Agent Integration Report

## 1. Introduction

This report details how the `jarvis-swe-agent` integrates into the broader AGSLAG system architecture. Based on its design, dependencies, and configuration, Jarvis acts as a core, specialized service provider within the ecosystem.

## 2. Role as a Backend Service

*   **API & WebSocket Provider:** Jarvis runs as a persistent Node.js server using Express and Socket.IO. This allows other components within AGSLAG (e.g., frontends like `mcp-dashboard-next`, orchestration services, or other agents) to interact with it via standard web protocols (HTTP API calls, WebSocket connections).
*   **SWE Capability Hub:** It centralizes software engineering capabilities, offering access to multiple LLMs (OpenAI, Anthropic, Gemini, OpenRouter) and a suite of relevant tools (shell, file I/O, code generation/analysis, testing, etc.). Other parts of the system can delegate SWE tasks to Jarvis instead of implementing these capabilities themselves.
*   **Multi-Instance Management:** The `AgentManager` allows the service to handle multiple concurrent SWE tasks, potentially for different users or workflows simultaneously, each maintaining its own context and history.

## 3. MCP Integration

*   **MCP Node:** The presence of `MCP_SERVER_URL` and `MCP_API_KEY` in its configuration strongly indicates that Jarvis is designed to act as a client connecting to a central MCP server within AGSLAG.
*   **Task Reception:** It can likely receive task instructions or requests from other MCP-enabled components (like an orchestrator or a user-facing agent) via MCP tool calls directed at it.
*   **Capability Exposure:** While the current analysis doesn't show Jarvis *hosting* an MCP server itself, its API/WebSocket interface effectively exposes its SWE capabilities to the rest of the system, which could be wrapped by an adapter or gateway if needed for full MCP service exposure.
*   **Potential Tool Usage:** As an MCP client, Jarvis could potentially *use* tools provided by other MCP servers if its internal `ToolService` were extended or if tasks required it to interact with external MCP resources.

## 4. Interaction Patterns

Several interaction patterns are possible:

1.  **Frontend -> Jarvis:** A user interface (like `mcp-dashboard-next` or `old-frontend`) could directly call the Jarvis API or connect via WebSockets to initiate SWE tasks and display results/progress (using Jarvis's event emissions).
2.  **Orchestrator -> Jarvis:** An orchestration agent could delegate specific SWE sub-tasks (e.g., "generate code for function X", "run tests for module Y") to Jarvis via API or potentially MCP calls.
3.  **Agent -> Jarvis:** Other specialized agents within AGSLAG might leverage Jarvis for its SWE capabilities via MCP or direct API calls.

## 5. Comparison to Standalone Agents

Unlike specialized, potentially single-run agents like `augment-swebench-agent`, Jarvis is designed for continuous operation and broader system integration. Its role is not just task execution but providing a persistent, accessible *service* that enhances the overall capabilities of the AGSLAG platform.

## 6. Agent Comparison Summary (New Section)

Analysis of `OpenManus`, `manus-open`, `anus`, `claude-code`, and `augment-swebench-agent` reveals several capability areas where Jarvis could be enhanced:

*   **Advanced Terminal Management:** Other agents (`manus-open`, `augment-swebench-agent`) feature persistent, interactive terminal sessions with input injection, process control, and contextual execution (SSH/Docker). Jarvis currently has basic, non-interactive shell execution.
*   **Advanced Browser Automation:** `manus-open` demonstrates full browser control (navigation, clicks, input, JS execution, etc.), far exceeding Jarvis's simple API-based web search.
*   **Sophisticated File Editing:** Agents like `manus-open` and `augment-swebench-agent` offer indentation-aware string replacement, insertion, undo capabilities, and regex search within files. Jarvis has basic file I/O.
*   **Specialized Tools:** Other agents include tools like Python REPLs, web crawlers, safe calculators, text manipulation suites, direct Git integration, and specific code understanding/refactoring commands (`explain`, `refactor`, `fix`).
*   **Meta-Cognitive Tools:** `augment-swebench-agent` includes tools for sequential thinking/planning and using agents hierarchically.

## 7. Recommendations for Jarvis Enhancement (New Section)

Based on the comparison, the following enhancements are recommended for `jarvis-swe-agent`:

1.  **Enhance Existing Tools:**
    *   **Shell Tool (`shell.ts`):** Upgrade to manage persistent, interactive sessions (e.g., using `node-pty`), allowing input injection, process control, and potentially contextual execution (Docker/SSH).
    *   **File System Tools (`file-system.ts`):** Add functions for advanced editing like indentation-aware `str_replace`, `insert`, `find_content` (regex search within file), and potentially an `undo` mechanism.
    *   **Execution Integration:** Improve integration between code generation/testing tools and the enhanced shell tool for better environment control.

2.  **Add New Tools:**
    *   **Browser Automation Tool:** Implement a comprehensive browser tool using Playwright or Puppeteer to enable complex web interactions, replacing or supplementing the current web search.
    *   **Git Tool:** Create a dedicated tool for common Git operations (`status`, `diff`, `add`, `commit`, etc.) for more robust version control interaction.
    *   **Refactoring/Explanation Tools:** Develop tools leveraging the LLM specifically for code explanation, refactoring, or automated fixing, inspired by `claude-code`.

3.  **Leverage MCP:**
    *   Investigate using existing MCP servers (e.g., `mcp-server-git`, `playwright-server`, `mcp_server_browser_use`) for capabilities like Git operations, browser automation, or interactive terminals to promote modularity.

4.  **Consider Meta-Cognitive Tools:**
    *   Evaluate adding tools for planning or hierarchical agent interaction if more complex, multi-step problem-solving is a goal for Jarvis.

## 8. Conclusion (Renumbered)

The Jarvis SWE Agent is architected as a key backend service and likely MCP client within the AGSLAG system. It centralizes multi-LLM SWE capabilities and makes them accessible via standard web protocols and potentially the MCP network. Its multi-instance design supports concurrent operations, positioning it as a foundational component for enabling complex software engineering workflows orchestrated within AGSLAG. Incorporating capabilities observed in other agents, particularly in advanced terminal/browser interaction and file editing, would further solidify its role as a comprehensive SWE capability hub. Further documentation on the specific API endpoints and WebSocket events would clarify the exact interaction mechanisms.