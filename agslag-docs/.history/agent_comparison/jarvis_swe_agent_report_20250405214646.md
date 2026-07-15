rv# Jarvis SWE Agent Report

## 1. Overview

**Name:** `jarvis-swe-agent`
**Purpose:** Headless Software Engineering (SWE) agent designed as a service within the Jarvis AI Assistant ecosystem. It aims to provide general-purpose SWE capabilities.
**Location:** `jarvis-swe-agent/`
**Technology Stack:** Node.js / TypeScript

## 2. Architecture & Design

*   **Service-Oriented:** Built as a backend service using Express.js, likely exposing an HTTP API for interaction.
*   **Real-time Communication:** Includes `socket.io`, suggesting capability for WebSocket communication, potentially for streaming responses or status updates.
*   **Modular Services:** Core logic is separated into distinct services:
    *   `LLMService`: Handles interactions with various LLM providers.
    *   `ToolService`: Manages the registration and execution of available tools.
    *   `AgentManager`: Orchestrates agent instances, conversation history, tool calls, and LLM interactions.
*   **Agent Instances:** Designed to manage multiple, persistent agent instances concurrently, each with its own ID, configuration, and history.
*   **Event-Driven:** Emits events (`message`, `thinking`, `toolCall`, `toolResult`, `error`) to allow external observation of agent activity.
*   **Configuration:** Uses `.env` file for sensitive keys, model defaults, server settings, and MCP connection details.
*   **Validation:** Employs `zod` for schema validation.
*   **Logging:** Uses `winston` and `morgan`.

## 3. LLM Integration

*   **Multi-Provider:** Supports OpenAI, Anthropic (Claude), Google (Gemini), and OpenRouter.
*   **Dynamic Selection:** The specific LLM provider and model are determined per request via a `ModelConfig` object passed to the `LLMService`.
*   **Configuration:** API keys for all supported providers are required in the `.env` file. Default models for each provider can also be set in `.env`, likely used by the `AgentManager` when creating the `ModelConfig` if not specified otherwise.

## 4. Tooling Capabilities

Jarvis possesses a broad, general-purpose toolset suitable for various SWE tasks:

*   **`shell`:** Execute arbitrary shell commands.
*   **`web_search`:** Perform web searches.
*   **`read_file`:** Read file content.
*   **`write_file`:** Write content to files.
*   **`list_files`:** List directory contents.
*   **`generate_code`:** Generate code snippets based on descriptions.
*   **`analyze_code`:** Perform basic code analysis (security, performance, quality, understanding).
*   **`run_tests`:** Execute test suites.
*   **`create_diagram`:** Generate diagrams (flowchart, sequence, etc.) from descriptions.
*   **`create_chart`:** Generate charts from data.

## 5. Integration & Role in AGSLAG

*   **MCP Client:** Configured to connect to an MCP server (`MCP_SERVER_URL`, `MCP_API_KEY`), indicating it functions as a node within the AGSLAG MCP network.
*   **Backend Service:** Acts as a central service providing SWE agent capabilities to other parts of the system (e.g., frontends, orchestrators) via its API and potentially WebSockets.
*   **Multi-Tasking:** Its multi-instance design allows it to handle concurrent SWE tasks initiated by different users or processes within AGSLAG.

## 6. Summary

Jarvis SWE Agent is a robust, multi-LLM, service-oriented agent designed for general software engineering tasks within the AGSLAG ecosystem. Its strengths lie in its flexibility (multiple LLMs, broad toolset), multi-instance capability, and integration potential via its API, WebSockets, and MCP connection. It serves as a foundational SWE capability provider for the broader system.