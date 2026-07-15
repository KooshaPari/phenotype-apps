# Work Package 1: Backend & Core Systems Enhancement

**Target Agent/Developer:** Agent 1 (e.g., Backend Specialist)

**Objective:** Implement the backend infrastructure for new features (workflow orchestration, project building, cost metrics, knowledge mapping) and enhance core MCP server capabilities, including programmatic agent spawning.

**Key Areas:** `src/` directory (especially `orchestration/`, `tools/`, `models/`, `prompts/`, `centralServer.ts`, `dbUtils.ts`), potentially adding new MCP server modules or enhancing existing ones.

**Detailed Instructions:**

1.  **Workflow Orchestration Engine (`src/orchestration/`, `src/tools/workflows.ts`):**
    *   Design and implement backend logic for defining, storing, and executing multi-step workflows involving multiple agents and MCP tools.
    *   Develop data models (`src/models/`) for workflow definitions (tasks, dependencies, conditions, loops).
    *   Create MCP tools (`src/tools/workflows.ts`) for creating, managing, executing, and monitoring workflows.
    *   Implement task scheduling and distribution logic within the `centralServer.ts` or a dedicated orchestration module.
    *   Develop error handling and recovery mechanisms for workflow execution.

2.  **Project Builder Backend (`src/orchestration/project_initializer.ts`, `src/models/`, `src/tools/project_management.ts`):**
    *   Enhance `project_initializer.ts` to support more comprehensive project definitions (details, docs, knowledge, team structure).
    *   Implement logic to parse and integrate information from provided documentation (PDFs, MDs) during project setup (consider adding a new tool or enhancing filesystem/pandoc tools).
    *   Develop backend logic for automatically suggesting team structures based on project details and allowing user modification.
    *   Create/update data models for storing detailed project information, including associated knowledge and team structures.

3.  **Cost/Time/Usage Metrics (`src/centralServer.ts`, `src/dbUtils.ts`, new `src/analytics/` module?):**
    *   Integrate tracking mechanisms into `centralServer.ts` and MCP tool wrappers (`src/tools/`) to log usage data (LLM calls, tokens, API usage, execution time).
    *   Design database schemas (`src/dbUtils.ts` or similar) to store metrics, categorized by project, session, agent, model, role, MCP tool, etc.
    *   Develop backend logic for calculating, aggregating, and projecting costs/time based on stored metrics.
    *   Create API endpoints (`src/app/api/`) to expose analytics data, predictions, and recommendations to the frontend.
    *   Consider creating a dedicated MCP server or enhancing an existing one (like `server-memory`?) for storing and querying these metrics efficiently.

4.  **Programmatic Agent Spawning (New MCP Tool/Server Module):**
    *   Design and implement an MCP tool (e.g., `spawn_agent_orchestration`) that allows an authorized client/agent to programmatically initiate new agent sessions or orchestrations.
    *   This tool should accept parameters like: agent configuration (model, system prompt, assigned MCPs), initial task, project context, team assignment, etc.
    *   Ensure secure handling of credentials and permissions for spawning agents.

5.  **Knowledge Graph/Map Backend (`src/models/knowledge-graph.ts`, `src/tools/knowledge_proxy.ts`):**
    *   Enhance the backend systems (`memorymesh` integration?) to support storing and querying relationships for project knowledge, agent knowledge, team structures, and work scope.
    *   Develop API endpoints or enhance MCP tools (`knowledge_proxy.ts`?) to allow the frontend to fetch and potentially modify graph data (for interactive graph pages).

6.  **Richer Comms Backend (`src/tools/communication.ts`, `src/tools/messaging.ts`, `src/models/communication.ts`):**
    *   Expand backend capabilities to support richer communication features (e.g., reactions, threads, status indicators, progress bars tied to tasks).
    *   Update data models (`communication.ts`) and MCP tools (`communication.ts`, `messaging.ts`) accordingly. Implement logic for progress tracking linked to workflow tasks.
