# Codebase Structure and Feature Leakage Analysis

**Version:** 1.0
**Date:** 2025-04-07

## 1. Introduction

This report analyzes the structure of the codebase within the `c:/Users/koosh/Dev/mcp/agslag` directory to identify potential "feature leakage" or fragmentation. The goal is to find functionalities implemented in directories *outside* the four designated core areas (`agslag/`, `jarvis-swe-agent/`, `mcp-server/`, `old-frontend/`) that might logically belong within them, and conversely, identify features potentially missing from these core areas based on external implementations or documentation.

## 2. Designated Core Areas

For this analysis, the intended core application areas are assumed to be:

*   `agslag/`: Presumed main application logic or platform core.
*   `jarvis-swe-agent/`: The primary software engineering agent implementation.
*   `mcp-server/`: Intended location for custom MCP server implementations developed within this project.
*   `old-frontend/`: The original or primary user interface/dashboard.

## 3. Analysis of Directories Outside Core Areas

Based on listing code definitions, the following directories outside the core areas were analyzed:

*   **`adapters/`**
    *   **Content:** MCP adapter classes (Manus, Serena, WCGW, Memorymesh, Team Comms).
    *   **Leakage:** Minimal, unless adapter implementations contain complex business logic beyond simple API translation.
    *   **Missing:** Core areas might lack clear abstractions/interfaces if they don't rely on these adapters for external MCP communication.

*   **`agent-dashboard-server/`**
    *   **Content:** Standalone Express-based dashboard server using `lowdb`.
    *   **Leakage:** Represents UI/monitoring functionality fragmentation. Core dashboard features might reside here instead of a unified frontend.
    *   **Missing:** The primary frontend (`old-frontend` or `mcp-dashboard-next`) might lack the monitoring features implemented here.

*   **`augment-swebench-agent/`**
    *   **Content:** Python scripts for running agents against the SWE-bench benchmark.
    *   **Leakage:** Potential leakage if the core SWE-bench solving logic *only* exists here. Evaluation/ensembling logic might belong in a core framework.
    *   **Missing:** `jarvis-swe-agent` might lack an integrated benchmarking/evaluation harness.

*   **`cline/`**
    *   **Content:** A foundational Python framework (Agent, API, Orchestrator, Scheduler, Monitoring, Dashboard).
    *   **Leakage:** Potential redundancy if `jarvis-swe-agent`/`agslag` reimplement these core framework features. Dashboard duplicates other UI efforts.
    *   **Missing:** Core areas might lack robust orchestration, scheduling, monitoring if they don't utilize this framework.

*   **`config/`**
    *   **Content:** `hub-spoke-config.json` defining central server, agent types, capabilities.
    *   **Leakage:** Configuration fragmentation if core areas define these settings elsewhere.
    *   **Missing:** Core areas might lack external configurability if they don't load from a central source like this.

*   **`examples/`**
    *   **Content:** Usage examples, including `AgslagProjectManager` class and CLI clients.
    *   **Leakage:** `AgslagProjectManager` might contain core AGSLAG orchestration logic that belongs in `agslag` or `jarvis-swe-agent`.
    *   **Missing:** Core areas might lack the high-level workflow orchestration demonstrated in `AgslagProjectManager`.

*   **`external tooling/`**
    *   **Content:** No source code found. Unlikely leakage.

*   **`filesystem-tools/`**
    *   **Content:** Custom MCP server for filesystem operations using `LiteMCP`.
    *   **Leakage:** Fragmentation if `mcp-server/` is the intended location for all custom servers.
    *   **Missing:** `mcp-server/` might lack this functionality. Core agents might lack standardized filesystem access if not using an MCP server.

*   **`huggingface-mcp-server/`**
    *   **Content:** Custom MCP server for Hugging Face/local model interactions. Depends on types in `src/`.
    *   **Leakage:** Fragmentation if not in `mcp-server/`. Core agents might handle HF interactions directly.
    *   **Missing:** `mcp-server/` might lack this functionality. Core agents might lack standardized HF/local model management.

*   **`integration/`**
    *   **Content:** High-level workflow implementations (`AgslagMcpIntegration`, collaborative dev, research patterns) and documentation.
    *   **Leakage:** Significant potential leakage of core AGSLAG orchestration logic if not used by `agslag`/`jarvis-swe-agent`.
    *   **Missing:** Core agents likely lack these sophisticated multi-agent workflow capabilities if this code isn't integrated.

*   **`interfaces/`**
    *   **Content:** Centralized TypeScript interfaces (MCPs, AGSLAG concepts).
    *   **Leakage:** None directly, but defines intended structure.
    *   **Missing:** Core areas might lack implementations adhering to these interfaces (e.g., `AgslagProject`, `AgslagTask`).

*   **`knowledge/`**
    *   **Content:** MemoryMesh interaction patterns/classes (`KnowledgeCapture`, `KnowledgeRetrieval`) and documentation.
    *   **Leakage:** Potential fragmentation if core agents implement KM differently.
    *   **Missing:** Core agents might lack standardized KM if these patterns aren't used.

*   **`litemcp-implementation/`**
    *   **Content:** Config only. Missing implementation code.

*   **`manus-open/`**
    *   **Content:** Python API client/server script for "Manus".
    *   **Leakage:** Fragmentation if it's an MCP server not in `mcp-server/`. Potential leakage if core agents use this client directly instead of the adapter.
    *   **Missing:** `mcp-server/` might lack this. Core agents might lack direct Manus interaction if not using this/adapter.

*   **`mcp-client/`**
    *   **Content:** Config only (`mcp_config.json` duplicates `jarvis-swe-agent` config).
    *   **Leakage:** Significant configuration duplication/fragmentation.
    *   **Missing:** Client application code (if intended). Core agents might lack external config loading.

*   **`mcp-dashboard-next/`**
    *   **Content:** Next.js dashboard project config (code missing from analysis).
    *   **Leakage:** UI fragmentation (duplicate effort with `old-frontend`, `agent-dashboard-server`, `working-routes`). Config duplication.
    *   **Missing:** Core dashboard implementation. `old-frontend` might lack features prototyped here.

*   **`mcp-integration/`**
    *   **Content:** Documentation for an MCP Integration Framework.
    *   **Leakage:** None (documentation).
    *   **Missing:** Highlights potentially unimplemented architectural features in core systems (Registry, Proxy, Federation, advanced error handling).

*   **`mdCrawler/`**
    *   **Content:** Python documentation crawler utility.
    *   **Leakage:** Potential duplication if core agents implement crawling separately.
    *   **Missing:** Core agents might lack standardized web documentation ingestion.

*   **`OpenManus/`**
    *   **Content:** Config only for a Next.js project.
    *   **Leakage:** Fragmentation.
    *   **Missing:** Implementation code.

*   **`orchestration/`**
    *   **Content:** Core `WorkflowEngine` implementation (sequential, parallel, conditional, retry, timeout) and documentation.
    *   **Leakage:** Significant potential leakage if this engine isn't used by `jarvis-swe-agent`/`agslag`.
    *   **Missing:** Core agents likely lack these advanced workflow capabilities if this engine isn't integrated.

*   **`process/`**
    *   **Content:** High-level SWE process definition and `MultiAgentSweProcess` implementation.
    *   **Leakage:** Significant potential leakage if this process manager isn't used by `agslag`/`jarvis-swe-agent`.
    *   **Missing:** Core systems might lack structured multi-agent SWE process management.

*   **`prompt-engineering/` & `prompts/`**
    *   **Content:** Centralized prompt definitions, templates, and guidelines.
    *   **Leakage:** Potential fragmentation if core agents use hardcoded prompts.
    *   **Missing:** Core agents might lack sophisticated/specialized behaviors if these prompts aren't loaded/used.

*   **`scripts/`**
    *   **Content:** Specific server runner (`start-claude-mcp.js`).
    *   **Leakage:** Potential fragmentation if core systems have different startup logic.
    *   **Missing:** Core might lack a generalized server startup mechanism.

*   **`src/`**
    *   **Content:** Core backend (WebSocket comms, DB utils, central server?).
    *   **Leakage:** Unlikely leakage *out*, but other components might duplicate its logic (e.g., DB access).
    *   **Missing:** Seems to be the intended core but might lack integration with frameworks/logic from `cline`, `orchestration`, `process`, `knowledge`, `integration`. May lack robust DB implementation. May lack AGSLAG interface implementations.

*   **`tests/`**
    *   **Content:** Integration tests (threading/notifications, Claude client).
    *   **Leakage:** Direct DB manipulation in tests suggests potentially missing core APIs/tools. Highlights dependencies.
    *   **Missing:** Core system might lack robust implementation of tested features (threading, notifications, specific Claude tools).

*   **`tools/`**
    *   **Content:** `RepositorySearch` utility for MCP discovery.
    *   **Leakage:** None likely, seems like a standalone utility.
    *   **Missing:** Core agents might lack dynamic tool discovery capabilities.

*   **`working-routes/`**
    *   **Content:** Another Next.js dashboard project config (code missing from analysis).
    *   **Leakage:** UI fragmentation. Config duplication.
    *   **Missing:** Core dashboard implementation.

*   **`ws-mcp/`**
    *   **Content:** No code found. Missing implementation if intended.

## 4. Key Themes & Patterns

*   **UI/Dashboard Fragmentation:** Multiple dashboard/frontend projects (`old-frontend`, `agent-dashboard-server`, `mcp-dashboard-next`, `working-routes`, `cline/dashboard.py`) exist, suggesting duplicated effort and lack of a unified UI strategy.
*   **Core Logic Outside Core Directories:** Significant application logic related to orchestration (`orchestration/`, `integration/`, `process/`, potentially `cline/`, `examples/AgslagProjectManager`) appears to reside outside the presumed core application directories (`agslag`, `jarvis-swe-agent`).
*   **Configuration Duplication/Fragmentation:** MCP server configurations appear duplicated (`jarvis-swe-agent/config.json`, `mcp-client/mcp_config.json`) and other config files exist in multiple dashboard projects. Central configuration (`config/hub-spoke-config.json`) might not be utilized.
*   **Custom MCP Server Fragmentation:** Custom MCP servers (`filesystem-tools`, `huggingface-mcp-server`, `manus-open`) exist outside the potentially designated `mcp-server/` directory.
*   **Design vs. Implementation Gaps:** Detailed documentation exists for frameworks and protocols (`prompt-engineering/`, `mcp-integration/`, `Documentation/`) that may not be fully implemented or utilized in the core codebases (`src/`, `jarvis-swe-agent/`, `agslag/`).
*   **Centralized Components:** Some areas show good centralization (`adapters/`, `interfaces/`, `knowledge/`, `prompts/`), but their integration with core logic needs verification.

## 5. Recommendations

1.  **Consolidate UI:** Decide on a single primary frontend framework/directory (e.g., `mcp-dashboard-next` or `old-frontend`) and migrate/integrate essential features from other dashboard projects (`agent-dashboard-server`, `working-routes`, `cline/dashboard.py`).
2.  **Refactor Core Logic:** Evaluate the logic within `orchestration/`, `integration/`, `process/`, `cline/`, and `examples/AgslagProjectManager`. Migrate essential, non-framework logic into the appropriate core directories (`agslag` or `jarvis-swe-agent`). If `cline` is the intended core framework, ensure `jarvis-swe-agent` and `agslag` utilize it effectively.
3.  **Unify Configuration:** Establish a single source of truth for MCP server configurations and ensure all relevant components (agents, clients, dashboards) load from it. Remove duplicated configuration files. Utilize the `config/` directory if appropriate.
4.  **Organize Custom MCP Servers:** Move all custom MCP server implementations (e.g., from `filesystem-tools`, `huggingface-mcp-server`, `manus-open`) into the `mcp-server/` directory for better organization.
5.  **Bridge Design-Implementation Gaps:** Review the frameworks defined in documentation (`prompt-engineering`, `mcp-integration`, etc.) and ensure their principles and protocols are implemented consistently within the core agent logic (`jarvis-swe-agent`, `agslag`, `src`, `cline`). Implement missing features identified in the design documents.
6.  **Standardize Knowledge Management:** Ensure core agents consistently use the patterns defined in `knowledge/memorymesh_patterns.ts` for interacting with MemoryMesh.
7.  **Clarify `src/` Role:** Define the exact purpose of the top-level `src/` directory and ensure its components (like WebSocket server, DB utils) are appropriately integrated or replaced by framework components (e.g., from `cline`). Consider moving it into `agslag` or `jarvis-swe-agent` if it's specific to one of them.