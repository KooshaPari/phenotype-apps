# Comprehensive MCP Agent Architecture Plan (Dual Local/Cloud Approach)

**Version:** 1.0
**Date:** 2025-04-05

## 1. Overview

This document outlines a plan for a sophisticated, distributed AI agent architecture designed to support complex software development workflows within a projectized organizational structure. The architecture employs a dual approach, enabling agents to operate both within local development environments (via UI automation) and as containerized, headless services in a cloud environment.

The core model is: **(1 Agent <-> 1 Container/Process) * X Agents per Team -> Team Branch -> Central Project Repository**.

This architecture facilitates collaboration within specialized teams (Frontend, Backend, QA, DevOps, Security, etc.), managed code contribution via Gitflow (team branches, PRs), and incorporates stage gating and reviews.

## 2. Architecture Goals

*   **Scalability:** Support multiple concurrent agents and teams working on different aspects of a project.
*   **Flexibility:** Allow agents to operate effectively in both local developer environments and scalable cloud infrastructure.
*   **Specialization:** Enable the creation of agents tailored for specific roles (developer, QA, DevOps, etc.) equipped with relevant MCP tools.
*   **Collaboration:** Facilitate efficient communication and knowledge sharing between agents within a team and potentially across teams.
*   **Robust Workflow:** Implement a structured Git workflow with team branches, pull requests, code reviews, and automated checks (CI/CD, security scans, QA tests) managed by specialized agents/teams.
*   **Headless Operation:** Ensure cloud-based agents can run reliably without requiring a graphical user interface.
*   **Local Integration:** Provide mechanisms for agents to interact with or automate local developer tools (specifically VS Code with Cline/Roo Code/Aider) when running locally.

## 3. MCP Client Strategy

A hybrid approach utilizing different MCP clients based on the deployment environment and required capabilities is recommended.

**3.1. Local UI Clients (for Local Developer Environment Integration):**

*   **Clients:** Cline, Roo Code, Aider.
*   **Challenge:** These are primarily UI-based (VS Code extensions or terminal UI). Direct programmatic control ("injection") into an arbitrary *existing* local VS Code instance is complex and likely unreliable due to security restrictions and lack of stable APIs for external control.
*   **Proposed Local Strategy (Requires UI Automation):**
    *   **Assumption:** Robust UI automation methods exist (as per user instruction). This could involve:
        *   **Accessibility APIs:** Leveraging OS-level accessibility features (Platform-specific, potentially brittle).
        *   **Scripting Extensions:** Developing a dedicated VS Code extension that exposes an API controllable by an external process (requires development effort).
        *   **Visual Automation:** Using tools like `omniparser_autogui_mcp` (if sufficiently advanced) or similar screen-scraping/input simulation (can be very fragile).
    *   **Workflow:** An orchestrator service running locally would need to identify the target VS Code window/process and use the chosen automation method to interact with the Cline/Roo Code/Aider UI (e.g., inputting prompts, triggering actions, reading output).
    *   **Alternative (More Controlled):** Run a dedicated, containerized VS Code Server instance locally (as explored previously) and automate *that* instance using Puppeteer or similar, providing a more controlled environment than automating the user's primary VS Code.

**3.2. Cloud Headless Clients (for Containerized Cloud Deployment):**

*   **Goal:** Agents run reliably in containers without GUIs, interacting programmatically via MCP.
*   **Client Options:**
    *   **Aider:**
        *   **Pros:** Mature CLI interface, built-in Git support, easy to containerize.
        *   **Cons:** Primarily focused on code editing; less flexible for broader agentic tasks compared to SDKs.
    *   **Goose:**
        *   **Pros:** Extensible, designed for agentic workflows, integrates with MCP servers, supports various LLMs.
        *   **Cons:** Newer project, potentially less battle-tested than SDKs. Requires Node.js environment.
    *   **mcp-agent:**
        *   **Pros:** Simple, composable framework specifically for MCP-based agents.
        *   **Cons:** May require more boilerplate code compared to higher-level agent frameworks. TypeScript-based.
    *   **Genkit:**
        *   **Pros:** Cross-language, integrates well with Google Cloud, supports flows and instrumentation.
        *   **Cons:** Might be overkill if not using other Genkit features; MCP integration is one part of a larger framework.
    *   **BeeAI Framework:**
        *   **Pros:** Focuses on scalable agentic workflows.
        *   **Cons:** MCP support status needs verification (issue #238 mentioned).
    *   **Custom SDK Implementation (Python/TypeScript):**
        *   **Pros:** Maximum flexibility, tailored precisely to needs.
        *   **Cons:** Requires significant development effort to build the agent logic (task processing, state management, LLM interaction) around the basic MCP client SDK.
*   **Recommended Cloud Strategy:**
    *   **Primary:** Utilize **Aider** for core coding/developer agents due to its CLI nature and Git integration, making containerization straightforward.
    *   **Secondary/Specialized:** Use **Goose** or **mcp-agent** (or a custom SDK implementation) for non-coding roles (e.g., Orchestrator, QA, Security) or where Aider's capabilities are insufficient. Goose offers a good balance of agentic features and MCP integration. `mcp-agent` provides a lower-level but potentially more flexible option if building custom agent logic.

## 4. Agent Specialization

Define agent roles with specific responsibilities and map them to appropriate client types and MCP server access.

| Role                     | Primary Tasks                                      | Recommended Cloud Client | Recommended Local Client | Key MCP Servers (Examples)                                                                                                |
| :----------------------- | :------------------------------------------------- | :----------------------- | :----------------------- | :------------------------------------------------------------------------------------------------------------------------ |
| **Project Coordinator**  | Task decomp/assign, monitoring, comms hub        | Goose / mcp-agent / Custom | (Less applicable)        | `team-communications`, `memorymesh`, PM tools (`todoist`, `linear-mcp-server`), `server-github` (PRs), `spawn_agent` (custom) |
| **Frontend Developer**   | UI implementation, API consumption, testing        | Aider / Goose            | Cline / Roo Code         | `git`, `server-github`, `playwright-server`, `postman`, `GLips/Figma-Context-MCP`                                          |
| **Backend Developer**    | API/Logic implementation, DB management, testing | Aider / Goose            | Cline / Roo Code         | `git`, `server-github`, `docker-mcp`, `postman`, DB servers (`postgres`, etc.), `sql-analyzer`                             |
| **DevOps Engineer**      | CI/CD, Infra management, monitoring, deployment    | Goose / Genkit / Custom  | Aider                    | `git`, `server-github`, `docker-mcp`, `mcp-server-kubernetes`, `alexei-led/*`, Monitoring servers (`sentry`, `grafana`)   |
| **QA Engineer**          | Test planning, automated testing (API/UI), reporting | Goose / mcp-agent / Custom | Aider / Roo Code         | `playwright-server`, `omniparser_autogui_mcp`, `postman`, PM tools, `sentry`                                              |
| **Security Specialist**  | Code scanning, vulnerability analysis, audits      | Goose / Custom           | Aider                    | `semgrep/mcp`, `BurtTheCoder/*`, `13bm/GhidraMCP`, `sentry`                                                               |
| **Technical Writer**     | Documentation creation/update, diagramming       | Goose / mcp-agent        | Roo Code / Cline         | `mcp-pandoc`, `mcp-summarizer`, `memorymesh`, `mindmap`, `git`, `server-github`                                           |
| **Data Analyst**         | Data querying, analysis, visualization           | Goose / Custom           | Roo Code / Cline         | DB Servers, `excel-mcp-server`, `isaacwasserman/mcp-vegalite-server`, `mcp-pandoc`                                        |

## 5. Orchestration Architecture

Two layers of orchestration are needed:

*   **Team Orchestrator:**
    *   Manages the lifecycle of agent containers within a specific team.
    *   Distributes tasks received from the Central Coordinator (or initiated within the team) to appropriate agents based on role/availability.
    *   Facilitates intra-team communication (potentially via message queue or dedicated MCP tools).
    *   Manages the team's feature branch (pulling from `main`, handling merge conflicts potentially via a dedicated agent, triggering pushes).
    *   Could be implemented as a dedicated service (e.g., Node.js/Python using Kubernetes/Docker APIs) or potentially as a specialized "Coordinator" agent using Goose/mcp-agent.
*   **Central Project Coordinator:**
    *   Oversees the entire project repository (`main` branch).
    *   Manages dependencies between teams/features.
    *   Receives PRs from team branches.
    *   Orchestrates the stage-gating process: triggers automated checks (CI, security scans via DevOps/Security agents), assigns code reviews (potentially to agents or human reviewers), and manages merging upon approval.
    *   Handles high-level task decomposition and assignment to Team Orchestrators.
    *   Likely implemented as a dedicated service with robust workflow capabilities.

## 6. Git Workflow and Stage Gating

*   **Branching:** `main` branch is protected. Development happens on team-specific feature branches (e.g., `feature/team-alpha/task-123`). Agents within a team collaborate on this branch.
*   **Commits:** Agents commit frequently to the team branch with clear messages linking to task IDs.
*   **Pull Requests:** Once a feature/task set is complete on the team branch, the Team Orchestrator (or lead agent) initiates a PR against `main`.
*   **Stage Gating & Reviews:**
    1.  **Automated Checks (CI):** Central Coordinator triggers CI pipeline (managed by DevOps agent via `server-github` actions) upon PR creation (build, lint, unit tests).
    2.  **Security Scan:** Central Coordinator triggers Security agent to scan the code (`semgrep/mcp`).
    3.  **QA Testing:** Central Coordinator triggers QA agent to run automated tests (API, E2E) against a staging environment deployed by DevOps agent.
    4.  **Code Review:** Central Coordinator assigns reviews (can be other agents or designated human reviewers). Feedback provided via GitHub comments or `team-communications`.
    5.  **Approval & Merge:** Upon passing all gates and receiving approvals, the Central Coordinator merges the PR into `main`.
    6.  **Sync:** Team Orchestrators periodically pull changes from `main` into their respective team branches.

## 7. Implementation Roadmap (Refined)

*   **Phase 1: Foundation (Cloud First)**
    *   Set up K8s/container orchestration environment.
    *   Develop & containerize **Aider-based** agents for core developer roles (Backend/Frontend). Implement basic Git interactions (clone, checkout, commit, push) within the container entrypoint/script.
    *   Implement basic **Team Orchestrator** (can be a script initially) to manage one team branch and deploy/manage Aider agent containers.
    *   Set up shared persistent volume for the repository.
    *   **Goal:** Demonstrate a single team working on an isolated branch using containerized Aider agents.
*   **Phase 2: Headless Client Expansion & Basic Orchestration**
    *   Develop/containerize **Goose/mcp-agent** based agents for non-coding roles (Coordinator, QA).
    *   Implement basic inter-agent communication within a team (e.g., using `team-communications` MCP or a simple message queue).
    *   Enhance Team Orchestrator to handle task assignment and basic status monitoring.
    *   Implement the **Central Project Coordinator** service (basic version) focusing on PR creation triggered by Team Orchestrator.
    *   **Goal:** Demonstrate multi-role agent collaboration within a team branch and basic PR creation.
*   **Phase 3: Local UI Automation & Advanced Orchestration**
    *   Implement the chosen **Local UI Automation** strategy (e.g., Containerized VS Code Server + Puppeteer, or other methods if deemed feasible) for Cline/Roo Code. Integrate this as an alternative execution mode for relevant agent roles.
    *   Enhance Team and Central Orchestrators with more sophisticated workflow management, task dependency handling, and status tracking.
    *   Implement the full **Stage Gating** process involving CI, Security, QA agents triggered by the Central Coordinator.
    *   Develop robust **conflict resolution** logic within agents and escalation paths to the Coordinator.
    *   **Goal:** Achieve dual local/cloud operation capability and implement the core review/gating workflow.
*   **Phase 4: Optimization & Feature Enrichment**
    *   Refine agent specialization with role-specific prompts and MCP toolsets.
    *   Implement advanced features outlined previously (cost tracking, knowledge graphs, richer comms, etc.).
    *   Develop feedback loops for agent performance improvement.
    *   Implement comprehensive security measures and auditing.
    *   Explore extracting core Cline/Roo Code functionality for a truly headless custom client if needed.
    *   **Goal:** Mature the platform into a fully-featured, robust, and intelligent multi-agent system.

## 8. Recommendations

1.  **Prioritize Cloud Headless:** Focus initial efforts on the containerized headless architecture using **Aider** for coding tasks and **Goose/mcp-agent** for orchestration/specialized roles. This provides a scalable foundation.
2.  **Defer Complex Local UI Automation:** Treat local UI automation as a secondary goal or a separate feature. The "Containerized VS Code Server + Puppeteer" approach seems most viable but still complex. Direct injection into existing user IDEs is likely impractical.
3.  **Build Orchestration Incrementally:** Start with simple Team and Central orchestration logic and gradually add complexity (workflow management, advanced gating).
4.  **Standardize Communication:** Define and enforce the inter-agent communication protocol early.
5.  **Leverage Existing MCPs:** Utilize the extensive list of available MCP servers before building custom ones. Integrate tools like `memorymesh`, `team-communications`, `git`, `server-github`, `docker-mcp`, `postman`, and relevant testing/security servers.

This plan provides a structured approach to building the desired dual local/cloud agent architecture, leveraging existing tools while acknowledging the technical challenges involved.
