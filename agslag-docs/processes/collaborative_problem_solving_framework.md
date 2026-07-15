# Collaborative Problem-Solving Framework

**Version:** 1.0
**Date:** 2025-04-05

## 1. Introduction

This document details the structured workflow for AI agents within the MCP platform to collaboratively analyze, plan, execute, and refine solutions for complex problems or tasks. This framework ensures a systematic approach, leveraging agent specialization, effective communication, and robust knowledge management. It expands upon Section 3 of the Core Prompt Engineering Framework.

## 2. Framework Phases

The framework consists of the following distinct phases:

**Phase 1: Problem Analysis & Context Gathering**
*   **Objective:** Ensure all participating agents have a shared and thorough understanding of the problem, its context, constraints, and success criteria.
*   **Trigger:** Assignment of a new complex task, often via the `collaborative_problem_solving` prompt.
*   **Key Activities:**
    *   **Individual Context Gathering:** Each agent independently uses available tools (`memorymesh.search_nodes`, `read_file`, `get_messages`, `fetch`, `brave-search`, relevant domain-specific tools) to gather background information related to the problem statement.
    *   **Problem Decomposition (Initial):** Agents analyze the core requirements, identify potential sub-problems, constraints (technical, time, resource), ambiguities, and define measurable success criteria.
    *   **Sharing Analysis:** Agents share their individual analysis, key findings, identified constraints, and potential ambiguities via `send_message` to the coordinator or a designated communication thread. References to supporting evidence (KM nodes, file paths) are crucial.
    *   **Knowledge Logging:** Key analysis points, critical constraints, and references to gathered context are logged into the Knowledge Management system (`memorymesh.add_nodes`, type 'ProblemAnalysis' or 'InitialContext').
*   **Tools:** `report_status`, `memorymesh.search_nodes`, `read_file`, `git.git_log`, `get_messages`, `fetch`, `brave-search`, `send_message`, `memorymesh.add_nodes`.
*   **Exit Criteria:** All relevant agents have shared their initial analysis; coordinator has acknowledged receipt.

**Phase 2: Approach Brainstorming & Feedback**
*   **Objective:** Generate a diverse set of potential solutions or approaches to the problem.
*   **Trigger:** Completion of Phase 1.
*   **Key Activities:**
    *   **Propose Solutions:** Based on the shared analysis, agents propose high-level solutions or technical approaches via `send_message`. Proposals should include brief justifications and address identified constraints.
    *   **Review & Critique:** Agents actively monitor (`check_new_messages`, `get_messages`) for proposals from others. They provide constructive feedback using `evaluate_contribution`, assessing feasibility, alignment with goals, potential risks, scalability, and completeness. Alternative perspectives are encouraged.
*   **Tools:** `send_message`, `check_new_messages`, `get_messages`, `evaluate_contribution`.
*   **Exit Criteria:** A sufficient range of potential solutions has been proposed and discussed; initial feedback has been shared.

**Phase 3: Solution Design & Planning (Coordinator Role)**
*   **Objective:** Select the optimal approach and create a detailed, actionable plan with decomposed subtasks and assignments.
*   **Trigger:** Completion of Phase 2.
*   **Key Activities (Coordinator):**
    *   **Synthesize & Evaluate:** Review all proposed solutions and feedback. Evaluate alternatives against success criteria, constraints, risks, and resource availability.
    *   **Select Approach:** Choose the most promising solution or synthesize a hybrid approach.
    *   **Document Decision:** Log the chosen design, **critical rationale**, and briefly why other major alternatives were rejected into KM (`memorymesh.add_nodes`, type 'DesignDecision'). Link to relevant Phase 1 & 2 artifacts.
    *   **Decompose Task:** Break down the chosen solution into smaller, well-defined, and ideally independent subtasks. Use `decompose_task` tool if available, or apply structured reasoning. Define clear inputs, outputs, dependencies, and acceptance criteria for each subtask. Assign unique IDs (e.g., `Subtask-XYZ`).
    *   **Resource Planning:** Review the capabilities and current workload of available agents (`get_agent_status`, `access_mcp_resource team-communications:///agents`). Identify any skill gaps or resource needs.
    *   **Agent Allocation/Spawning:** Plan assignments using `assign_subtasks` (if available) or internal logic. If new agents are required, use the `spawn_agent` mechanism (requires orchestrator support) and monitor their successful registration and readiness (`get_agent_status`).
    *   **Log Plan:** Document the complete plan (subtasks, dependencies, assignments, estimated effort if possible) in KM (`memorymesh.add_nodes`, type 'ProjectPlan' or 'ExecutionPlan').
*   **Tools:** `get_messages`, `evaluate_contribution`, `memorymesh.add_nodes`, `decompose_task` (optional), `assign_subtasks` (optional), `get_agent_status` (optional), `spawn_agent` (optional).
*   **Exit Criteria:** A detailed plan with decomposed subtasks and assignments is created and logged in KM. Necessary agents are available.

**Phase 4: Task Delegation & Tracking (Coordinator Role)**
*   **Objective:** Formally assign subtasks to agents and ensure they are tracked in the project management system.
*   **Trigger:** Completion of Phase 3.
*   **Key Activities (Coordinator):**
    *   **Delegate:** For each subtask, send a formal assignment message (`send_message`, type 'task_assignment') to the designated agent. Include all necessary details from the plan (ID, description, inputs, outputs, criteria, deadline).
    *   **Track in PM:** Simultaneously create a corresponding task in the external Project Management system (e.g., `todoist.create_task`). Ensure consistency between the delegated message and the PM task.
*   **Tools:** `send_message`, `todoist.create_task` (or equivalent PM tool).
*   **Exit Criteria:** All planned subtasks have been delegated via message and created in the PM system.

**Phase 5: Implementation (Assigned Agents)**
*   **Objective:** Execute the assigned subtasks according to the plan and best practices.
*   **Trigger:** Receipt of a `task_assignment` message.
*   **Key Activities (Assigned Agents):**
    *   Adhere strictly to the "Standard Task Execution Workflow" (Section 7 of Core Prompt Engineering Framework). This involves:
        *   Acknowledging the task (`report_status`, `send_message`).
        *   Gathering specific context (KM, comms).
        *   Setting up version control if applicable (branching).
        *   Performing the core work (coding, research, writing, etc.).
        *   Committing frequently (if coding).
        *   Logging significant findings/outputs to KM.
        *   Monitoring communications and reporting progress/blockers proactively.
        *   Handling blockers (self-resolve then `request_assistance`).
        *   Integrating/syncing with `main` and testing (if coding).
        *   Performing handoff upon completion (pushing code, creating PR, notifying coordinator, updating PM tool (`todoist.close_task`), updating status).
        *   Providing feedback (`evaluate_contribution`) where appropriate.
*   **Tools:** Full suite of agent tools as needed (`git.*`, `server-github.*`, `memorymesh.*`, `team-communications.*`, `read_file`, `write_to_file`, `replace_in_file`, `execute_command`, `fetch`, `brave-search`, domain-specific tools, etc.).
*   **Exit Criteria:** Assigned subtask is completed, tested (if applicable), results are handed off (PR, KM log, notification), PM task is closed, and agent status is updated to 'idle'.

**Phase 6: Evaluation & Refinement**
*   **Objective:** Integrate results, evaluate against the overall goal, and iterate if necessary.
*   **Trigger:** Completion of all initial subtasks identified in Phase 3.
*   **Key Activities:**
    *   **Integration (Coordinator):** Gather results (PRs, KM nodes, artifacts). Merge code (reviewing PRs, handling final merge conflicts). Integrate non-code artifacts. Use `merge_results` tool if available.
    *   **Share Integrated Result (Coordinator):** Communicate the overall result or state of the integrated solution via `broadcast_message` or `send_message`.
    *   **Collective Review:** All relevant agents review the integrated result against the original problem statement and success criteria.
    *   **Provide Feedback:** Agents provide feedback on the overall solution using `evaluate_contribution`.
    *   **Identify Refinements (Coordinator):** Based on feedback and evaluation, the coordinator determines if the goal is met or if refinements/fixes are needed.
    *   **Iteration:** If refinements are required, initiate a new mini-cycle starting from Phase 3 (Design/Plan the refinements) or Phase 4 (Delegate refinement tasks).
    *   **Final Documentation (Coordinator):** Once the overall goal is achieved, log the final outcome, key learnings, and links to final artifacts/code state in KM (`memorymesh.add_nodes`, type 'FinalOutcome' or 'ProjectCompletion').
*   **Tools:** `get_messages`, `git.*`, `server-github.*` (PR review/merge), `merge_results` (optional), `broadcast_message`, `send_message`, `evaluate_contribution`, `memorymesh.add_nodes`.
*   **Exit Criteria:** The overall problem is solved according to success criteria, the final outcome is documented in KM, and relevant stakeholders are notified.

## 9. Conclusion

This framework provides a robust structure for multi-agent collaboration. Its success depends on agents strictly adhering to the defined phases, utilizing communication and knowledge management tools effectively, and leveraging the specific workflows detailed in the Core Prompt Engineering Framework and associated prompts. Flexibility for iteration (Phase 6 -> Phase 3/4) is built-in to handle necessary refinements.
