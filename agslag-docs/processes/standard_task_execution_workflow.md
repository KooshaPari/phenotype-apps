# Standard Task Execution Workflow for Agents

**Version:** 1.0
**Date:** 2025-04-05

## 1. Introduction

This document details the standard, step-by-step workflow that individual AI agents within the MCP platform should follow when assigned a subtask. This workflow ensures consistency, proper context handling, effective use of tools, adherence to version control practices, proactive communication, and integration with knowledge management and project management systems. It expands upon Section 7 of the Core Prompt Engineering Framework and assumes the agent has received a `task_assignment` message (see Structured Communication Protocol).

## 2. Workflow Steps

**Step 1: Task Acknowledgment & Status Update**
*   **Trigger:** Receipt of a `task_assignment` message.
*   **Actions:**
    *   Parse the task assignment payload (ID, description, inputs, outputs, etc.).
    *   Immediately send an acknowledgment message (`send_message`, `message_type`: 'ack') back to the coordinator (or sender of the assignment). Include the `task_id` in the payload.
    *   Update agent status (`report_status`) to 'busy', including the `task_id` and a brief description (e.g., "Starting Subtask-123: Implement login endpoint").
*   **Tools:** `get_messages`, `send_message`, `report_status`.

**Step 2: Context Gathering**
*   **Objective:** Collect all necessary information before starting work.
*   **Actions:**
    *   **Query Knowledge Management:** Use `memorymesh.search_nodes` with queries related to the task description, keywords, parent task ID, and specified inputs (e.g., "design decision for Subtask-123", "API spec for /users endpoint"). Review results thoroughly.
    *   **Check Communications:** Use `check_new_messages` and `get_messages` to retrieve any recent messages related to this task or its context (e.g., clarifications, related discussions in the thread).
    *   **Review Codebase (if applicable):** If the task involves code, use `read_file` to examine relevant existing files mentioned in the task inputs or identified during KM search. Use `git.git_log` (potentially via `execute_command`) on relevant files to understand recent history.
*   **Tools:** `memorymesh.search_nodes`, `check_new_messages`, `get_messages`, `read_file`, `git.git_log` (or `execute_command`).

**Step 3: Version Control Setup (Coding Tasks Only)**
*   **Objective:** Prepare the local repository environment for coding work.
*   **Actions:**
    *   Ensure the agent is operating in the correct project directory (this might be implicit in the container setup or require a `cd` via `execute_command` if not).
    *   Checkout the main branch: `git.git_checkout {'branch_name': 'main'}`.
    *   Pull the latest changes from the remote main branch: `execute_command git pull origin main`. Handle potential conflicts immediately (rare at this stage, but possible).
    *   Create a new feature branch specific to the subtask and agent: `git.git_create_branch {'branch_name': 'feature/subtask-<ID>-<AgentID>'}`. Use the actual Subtask ID and Agent ID.
    *   Checkout the newly created feature branch: `git.git_checkout {'branch_name': 'feature/subtask-<ID>-<AgentID>'}`.
*   **Tools:** `execute_command`, `git.git_checkout`, `git.git_create_branch`.

**Step 4: Execute Subtask**
*   **Objective:** Perform the core work required by the subtask description.
*   **Actions:**
    *   **Perform Work:** Utilize appropriate MCP tools based on the task type:
        *   **Coding:** Use `read_file`, `replace_in_file`, `write_to_file`. Analyze code structure, implement logic, write tests.
        *   **Research:** Use `brave-search`, `fetch`. Analyze and synthesize information.
        *   **Documentation:** Use `read_file`, `write_to_file`, `mcp-pandoc.convert-contents`. Structure and write content.
        *   **Analysis:** Use database query tools, data processing tools, etc.
    *   **Commit Frequently (Coding Tasks):** Make small, logical commits using `git.git_add` and `git.git_commit`. Commit messages MUST reference the `subtask_id`. Example: `git.git_commit {'message': 'Subtask-123: Add request validation for login endpoint'}`.
    *   **Log to Knowledge Management:** Log significant findings, key code snippets, design choices made during implementation, summaries of research, or generated artifacts to KM using `memorymesh.add_nodes`. Include `subtask_id`, `agent_id`, `timestamp`, and link to related nodes or commits.
*   **Tools:** Domain-specific tools (`read_file`, `replace_in_file`, `write_to_file`, `brave-search`, `fetch`, `mcp-pandoc`, `memorymesh.add_nodes`, `git.git_add`, `git.git_commit`, etc.).

**Step 5: Monitor Communications & Report Progress/Blockers**
*   **Objective:** Stay responsive and keep the team informed.
*   **Actions:**
    *   **Check Messages:** Periodically (especially during longer computations or waits) call `check_new_messages` and `get_messages`. Respond promptly if required.
    *   **Report Progress:** For long tasks, send periodic progress updates (`send_message`, `message_type`: 'progress_update') to the coordinator.
    *   **Report Blockers:** If blocked, immediately follow Step 6.
*   **Tools:** `check_new_messages`, `get_messages`, `send_message`.

**Step 6: Handle Blockers**
*   **Objective:** Resolve impediments efficiently.
*   **Actions:**
    *   **Attempt Self-Resolution:**
        *   Query KM (`memorymesh.search_nodes`) for solutions to similar problems.
        *   Review logs (`docker-mcp.get-logs` if applicable).
        *   Use search tools (`brave-search`, `fetch`) for external information.
        *   Retry the failing operation if appropriate.
    *   **Update Status:** Report status (`report_status`) as 'blocked', providing a clear `blocker_description`.
    *   **Request Assistance:** If self-resolution fails, use `request_assistance`, detailing the problem, context, attempts made, and required help (see Communication Protocol). Send this message to the coordinator or relevant expert agent.
    *   **Await Response:** Monitor communications (`check_new_messages`, `get_messages`) for assistance or further instructions.
    *   **Resume:** Once unblocked, update status (`report_status`) back to 'busy' and resume Step 4.
*   **Tools:** `memorymesh.search_nodes`, `docker-mcp.get-logs`, `brave-search`, `fetch`, `report_status`, `request_assistance`, `check_new_messages`, `get_messages`.

**Step 7: Integration & Testing (Coding Tasks Only)**
*   **Objective:** Ensure the completed work integrates correctly with the main codebase and passes tests.
*   **Actions:**
    *   **Sync with Main:**
        *   Commit any final changes on the feature branch (`git.git_add`, `git.git_commit`).
        *   Checkout main branch: `git.git_checkout {'branch_name': 'main'}`.
        *   Pull latest changes: `execute_command git pull origin main`.
        *   Checkout feature branch again: `git.git_checkout {'branch_name': 'feature/subtask-<ID>-<AgentID>'}`.
        *   Merge main into the feature branch: `execute_command git merge main`.
    *   **Conflict Resolution:** If the merge results in conflicts:
        *   Follow the detailed Conflict Resolution steps in Section 8.1 of the Core Prompt Engineering Framework (Identify -> Context -> Reason -> Attempt (`replace_in_file`) -> Test -> Escalate (`request_assistance`)). **Do not proceed if tests fail after resolution attempts.**
    *   **Run Tests:** Execute the project's test suite (`execute_command <test_command>`). Ensure all tests pass. If tests fail, debug and fix the issues (return to Step 4/Commit/Re-test).
*   **Tools:** `git.git_add`, `git.git_commit`, `git.git_checkout`, `execute_command` (for pull, merge, test execution), `git.git_status`, `read_file`, `replace_in_file`, `request_assistance`.

**Step 8: Completion & Handoff**
*   **Objective:** Finalize the work, notify stakeholders, and update tracking systems.
*   **Actions:**
    *   **Push Branch (Coding Tasks):** Push the completed and tested feature branch to the remote repository: `execute_command git push origin feature/subtask-<ID>-<AgentID>`.
    *   **Create Pull Request (Coding Tasks):** Create a PR using `server-github.create_pull_request`. Target the `main` branch, assign the coordinator/reviewers, and include the `subtask_id` and a summary in the title/body.
    *   **Notify Coordinator:** Send a completion message (`send_message`, `message_type`: 'completion_report') to the coordinator. Include:
        *   `subtask_id`.
        *   Summary of work done.
        *   Link to PR (if applicable).
        *   Links to key artifacts or KM nodes created (`memorymesh.add_nodes` logs).
    *   **Transfer Artifacts (If Applicable):** If the output is a file not suitable for Git/KM, use `transfer_file` to send it to the coordinator or designated location.
    *   **Update Project Management:** Close the corresponding task in the PM system using the appropriate tool (e.g., `todoist.close_task`).
    *   **Update Status:** Update agent status (`report_status`) to 'idle', clearing the `current_task_id`.
*   **Tools:** `execute_command` (for push), `server-github.create_pull_request`, `send_message`, `memorymesh.add_nodes`, `transfer_file` (optional), `todoist.close_task` (or equivalent), `report_status`.

**Step 9: Feedback (Optional)**
*   **Objective:** Provide constructive feedback on the process or inputs received.
*   **Actions:** If appropriate, use `evaluate_contribution` to provide feedback on the clarity of the task assignment, the usefulness of provided context, or collaboration with other agents during the task.
*   **Tools:** `evaluate_contribution`.

## 9. Conclusion

This standard workflow provides a detailed sequence of actions for agents executing tasks. Adherence ensures that work is performed systematically, context is managed, collaboration protocols are followed, and progress is tracked effectively across different systems (Git, KM, PM, Comms). Prompt engineering should guide agents to follow these steps diligently.
