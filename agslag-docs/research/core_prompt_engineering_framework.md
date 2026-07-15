# Core Prompt Engineering Framework

**Version:** 0.1

**Objective:** To establish a standardized set of principles, protocols, and best practices for guiding AI agents within the MCP platform, ensuring effective collaboration, efficient task execution, and robust knowledge management.

## 1. Core Principles

These principles should underpin all agent behavior and prompt design:

*   **Goal Focus:** All actions must directly contribute to the assigned task or overall project goal. Agents must prioritize tasks effectively.
*   **Proactive Communication:** Agents should not wait to be asked for status or information. Frequent and clear communication of progress, blockers, and completions is essential.
*   **Active Listening & Responsiveness:** Agents must constantly monitor communication channels and respond promptly and relevantly to messages directed at them or relevant to their tasks.
*   **Clarity & Context:** All communications and actions should include sufficient context (e.g., Task IDs, Agent IDs, relevant file paths, KM node IDs, commit SHAs) to be clearly understood by other agents. Reasoning should be explicit.
*   **Knowledge Management Discipline:** Agents must actively query knowledge management systems (e.g., MemoryMesh) for existing context before starting work and diligently log significant findings, decisions, artifacts, and resolutions.
*   **Constructive Collaboration:** Agents should provide specific, actionable feedback and actively participate in problem-solving and conflict resolution processes.
*   **Resourcefulness & Escalation:** Agents should attempt to resolve blockers independently first (using available tools and knowledge) before requesting assistance, clearly articulating the problem and attempted solutions.
*   **Version Control Discipline:** Agents performing coding tasks must adhere strictly to defined Git workflows (branching, committing, pulling, merging, conflict resolution).

## 2. Structured Communication Protocol

This protocol governs how agents interact. It leverages the `team-communications` MCP server (or equivalent tools defined in prompts like `send_message`, `broadcast_message`, `check_new_messages`, `get_messages`).

*   **Message Structure:** (Define a standard JSON or structured text format if not implicitly defined by tools)
    *   `sender_id`: ID of the sending agent.
    *   `recipient_id`: ID of the target agent (or 'broadcast').
    *   `thread_id`: Optional ID for grouping related messages.
    *   `message_type`: (e.g., 'status_update', 'task_assignment', 'query', 'response', 'blocker_report', 'knowledge_share', 'feedback', 'request_assistance', 'decision_log').
    *   `payload`: Content of the message, including relevant context IDs (task, commit, file, KM node).
    *   `timestamp`: Time of message creation.
*   **Key Communication Acts & Tools:**
    *   **Checking for Messages:** Agents MUST poll frequently using `check_new_messages` and retrieve relevant messages using `get_messages`.
    *   **Sending Direct Messages:** Use `send_message` for targeted communication.
    *   **Broadcasting:** Use `broadcast_message` for team-wide announcements (use judiciously).
    *   **Reporting Status:** Use `report_status` (tool TBD, potentially part of `team-communications` or a dedicated tool) with standardized statuses (e.g., 'online', 'idle', 'busy', 'blocked', 'offline') and current task context. Triggered on state changes (start/end task, blocked, online/offline).
    *   **Requesting Assistance:** Use `request_assistance` (tool TBD), specifying the blocker, context, attempts made, and required capability/target agent.
    *   **Providing Feedback:** Use `evaluate_contribution` (tool TBD), referencing the specific message or artifact being evaluated.
    *   **Sharing Artifacts:** Use `transfer_file` (tool TBD) or log to KM and share the node ID.
*   **Addressing:** Always use specific Agent IDs when known. Reference relevant context IDs (Task, Subtask, Commit, File, KM Node) in the payload.

## 3. Collaborative Problem-Solving Framework

This framework outlines a structured approach for multiple agents to tackle complex problems collaboratively. It relies heavily on the communication protocol (Section 2) and knowledge management practices (Section 6). The `collaborative_problem_solving` prompt provides the initial context for this workflow.

**Phase 1: Problem Analysis & Context Gathering**
*   **Trigger:** A new complex task or problem is assigned, often initiated via the `collaborative_problem_solving` prompt.
*   **Actions (All Relevant Agents):**
    *   Report status (`report_status`) indicating analysis start.
    *   Independently gather context:
        *   Query Knowledge Management (`memorymesh.search_nodes`) for related prior work, designs, constraints, etc.
        *   Review relevant code (`read_file`, `git.git_log`).
        *   Check communication logs (`get_messages`) for background.
    *   Analyze the problem statement: Identify core challenges, requirements, constraints, potential ambiguities, and success criteria.
    *   Share initial analysis and findings via `send_message` (to coordinator or designated thread). Include references to supporting KM nodes or artifacts.
    *   Log key analysis points and gathered context references into KM (`memorymesh.add_nodes`, type 'ProblemAnalysis').

**Phase 2: Approach Brainstorming & Feedback**
*   **Trigger:** Completion of initial analysis phase by most agents.
*   **Actions (All Relevant Agents):**
    *   Based on analysis, propose potential solutions or approaches via `send_message`. Justify proposals with reasoning and evidence.
    *   Actively monitor communications (`check_new_messages`, `get_messages`) for proposals from other agents.
    *   Review others' proposals constructively. Provide feedback using `evaluate_contribution`, focusing on feasibility, alignment with goals, potential risks, and completeness.

**Phase 3: Solution Design & Planning (Coordinator Role)**
*   **Trigger:** Sufficient brainstorming and feedback have occurred.
*   **Actions (Coordinator Agent):**
    *   Synthesize the proposed solutions and feedback received.
    *   Evaluate the alternatives based on project goals, constraints, feasibility, and risks.
    *   Select the most promising approach or a hybrid approach.
    *   **Crucially:** Document the chosen design and the rationale behind the decision in KM (`memorymesh.add_nodes`, type 'DesignDecision'). Link to relevant analysis and brainstorming nodes.
    *   Decompose the chosen solution into smaller, manageable subtasks using appropriate tools or reasoning (e.g., `decompose_task` if available). Define clear inputs, outputs, and acceptance criteria for each subtask.
    *   Plan subtask assignments based on agent roles, capabilities, and current workload (query agent status/availability if needed via `get_agent_status` or similar). Use `assign_subtasks` if available, or internal planning logic.
    *   Assess team capacity. If necessary, request the spawning of new agents with specific roles/capabilities (`spawn_agent` - requires orchestrator support). Monitor their registration.
    *   Log the overall plan (subtasks, assignments, dependencies) in KM (`memorymesh.add_nodes`, type 'ProjectPlan').

**Phase 4: Task Delegation & Tracking (Coordinator Role)**
*   **Trigger:** Completion of planning phase.
*   **Actions (Coordinator Agent):**
    *   Formally delegate each subtask to the assigned agent using `delegate_subtask` (if available) or `send_message` (type 'task_assignment'). Include subtask ID, description, inputs, expected outputs, and deadline.
    *   Simultaneously, create corresponding tasks in the designated Project Management system (e.g., `todoist.create_task`) for external visibility and tracking. Ensure PM task details match the delegated subtask information.

**Phase 5: Implementation (Assigned Agents)**
*   **Trigger:** Receipt of a task assignment message.
*   **Actions (Assigned Agents):**
    *   Follow the detailed "Standard Task Execution Workflow" outlined in Section 7. This includes acknowledging the task, gathering context, setting up version control (if needed), executing the work, communicating progress/blockers, handling conflicts, testing, logging to KM, and performing handoff upon completion (updating PM tool, notifying coordinator, creating PR).

**Phase 6: Evaluation & Refinement**
*   **Trigger:** Completion of all initial subtasks for the problem.
*   **Actions (Coordinator & All Relevant Agents):**
    *   Coordinator gathers individual results/artifacts (e.g., PR links, KM node IDs, file paths) shared during handoff.
    *   Coordinator attempts to merge/integrate results (`merge_results` if available, or manual review/integration).
    *   Coordinator shares the integrated result or overall outcome via `broadcast_message` or `send_message`.
    *   All relevant agents review the integrated result and provide feedback (`evaluate_contribution`).
    *   Coordinator identifies any necessary refinements, fixes, or further iterations based on feedback and overall goal achievement.
    *   If refinements are needed, return to Phase 3/4 to plan and delegate the refinement tasks.
    *   Once the problem is fully resolved, the Coordinator logs the final outcome, lessons learned, and links to final artifacts in KM (`memorymesh.add_nodes`, type 'FinalOutcome').

## 4. Conflict Resolution Process

This process provides a structured way to handle disagreements between agents, whether about technical approaches, task interpretation, or resource allocation. It prioritizes objective evidence and alignment with project goals. The `model_conflict_resolution` prompt provides initial context.

**Step 1: Identification & Initiation**
*   **Trigger:** An agent identifies a disagreement or conflicting action/statement from another agent.
*   **Action:** The identifying agent initiates the process by sending a message (`send_message`) to the involved parties (and potentially a designated mediator or coordinator), clearly stating the perceived conflict (`message_type`: 'conflict_report'). Reference specific messages, commits, or artifacts causing the conflict.

**Step 2: Position Statement**
*   **Trigger:** Receipt of a 'conflict_report' message.
*   **Action (Each Involved Agent):**
    *   Report status (`report_status`, indicating involvement in conflict resolution).
    *   Clearly state their position regarding the conflict via `send_message`.
    *   Provide concise reasoning and objective evidence supporting their position. Reference specific data, KM nodes (`memorymesh.search_nodes`), logs, or documentation. Avoid subjective language.

**Step 3: Perspective Understanding**
*   **Trigger:** All involved agents have stated their positions.
*   **Action (Each Involved Agent & Mediator):**
    *   Carefully review all stated positions and supporting evidence (`get_messages`).
    *   Summarize their understanding of *other* agents' positions via `send_message` to confirm comprehension ("My understanding of Agent-X's position is...").
    *   Ask specific, non-confrontational clarifying questions if needed (`send_message`).

**Step 4: Identify Common Ground & Core Issues**
*   **Trigger:** Mutual understanding of positions is confirmed.
*   **Action (Mediator or Collaborative Effort):**
    *   Identify shared goals, objectives, or constraints relevant to the conflict.
    *   Clearly define the core point(s) of disagreement, separating them from misunderstandings.
    *   Document these points via `send_message`.

**Step 5: Generate & Evaluate Alternatives**
*   **Trigger:** Core issues are defined.
*   **Action (All Involved Agents & Mediator):**
    *   Brainstorm alternative solutions or compromises that address the core issues and align with common ground/project goals (`send_message`).
    *   Objectively evaluate the pros and cons of each alternative based on evidence, project requirements, and potential impact. Use `evaluate_contribution` for structured feedback on proposed alternatives.

**Step 6: Consensus & Documentation**
*   **Trigger:** Alternatives have been evaluated.
*   **Action (Mediator or Collaborative Agreement):**
    *   Facilitate the selection of the best alternative based on the evaluation. Aim for consensus. If consensus cannot be reached, the mediator (or coordinator if no mediator) makes a decision based on project priorities.
    *   The final decision, including the **rationale** and the rejected alternatives (briefly), is clearly documented via `send_message` to all involved.
    *   The decision and rationale are logged permanently in KM (`memorymesh.add_nodes`, type 'ConflictResolution'), linking back to the initial conflict report/description and relevant evidence nodes.
    *   Involved agents acknowledge the resolution and update their status (`report_status`).

## 5. Knowledge Transfer Protocol

(Based on `knowledge_transfer` prompt)

*   **Provider:** Explains concepts clearly, provides examples, responds to questions, suggests applications, and logs the explanation in KM.
*   **Recipient:** Acknowledges receipt, summarizes understanding, asks clarifying questions, relates concepts to their tasks, provides feedback on the explanation, and logs learned concepts in KM, linking them appropriately.
*   **Goal:** Ensure effective integration of knowledge, confirmed via feedback and KM logging.

## 6. Knowledge Management (KM) Best Practices

Leverages KM tools (e.g., `memorymesh`, `mcp-ragdocs`).

*   **Query First:** Before starting any significant work, query the KM system (`search_nodes`) for relevant existing information, designs, or previous results.
*   **Log Diligently:** Log significant artifacts, decisions, findings, final code/documentation, explanations, and resolutions using `add_nodes` (or equivalent).
*   **Use Rich Metadata:** When logging, include relevant context: Task/Subtask ID, Agent ID, timestamp, related KM node IDs, file paths, commit SHAs.
*   **Standardize Node Types:** Use consistent node types (e.g., 'CodeSnippet', 'DesignDecision', 'ResearchFinding', 'MeetingSummary', 'ConflictResolution', 'FinalArtifact', 'Explanation', 'TaskContext').
*   **Create Links:** Establish relationships between KM nodes (e.g., 'relatedTo', 'buildsUpon', 'resolvesConflict', 'implementsDesign').

## 7. Standard Task Execution Workflow (Agent Level)

(Based on `model_strengths_utilization` prompt)

1.  **Acknowledge & Start:** Report status ('busy'), confirm receipt with coordinator.
2.  **Context Gathering:** Query KM, check messages.
3.  **Version Control Setup (Coding Tasks):** Navigate, checkout `main`, pull latest, create feature branch (`feature/subtask-<ID>-<AgentID>`), checkout branch.
4.  **Execute Subtask:** Perform work (code, research, document). Log significant findings/outputs to KM. Commit frequently if coding.
5.  **Monitor & Communicate:** Periodically check messages, report progress/blockers.
6.  **Handle Blockers:** Attempt self-resolution first (KM, logs, tools), then use `request_assistance`. Update status.
7.  **Integration & Testing (Coding Tasks):** Sync with `main`, merge, resolve conflicts (Reason -> Manual -> Test -> Escalate), run tests.
8.  **Completion & Handoff:** Push branch, create PR (assign coordinator), notify coordinator (link PR/artifacts), update PM tool, update status ('idle').
9.  **Feedback:** Provide feedback on inputs/collaboration using `evaluate_contribution` if appropriate.

## 8. MCP Tool Usage Patterns & Best Practices

This section provides specific examples and best practices for commonly used MCPs within the platform.

### 8.1 Version Control (Git / GitHub)

*   **Assumed MCP Servers:** `git`, `server-github`, `execute_command` (for complex git CLI commands not covered by specific tools).
*   **Core Workflow Integration:** Refer to Section 7 (Standard Task Execution Workflow) for the overall flow.
*   **Checking Status:**
    *   Before modifying files: Use `git.git_status` to ensure a clean working directory.
    *   After modifying files: Use `git.git_status` to see untracked or modified files.
    *   Before committing: Use `git.git_status` again to confirm staged files match intent.
*   **Branching:**
    *   Create branches with a clear naming convention: `git.git_create_branch {'branch_name': 'feature/subtask-<ID>-<AgentID>'}`.
    *   Switch branches using: `git.git_checkout {'branch_name': 'feature/subtask-<ID>-<AgentID>'}` or `git.git_checkout {'branch_name': 'main'}`.
*   **Staging:**
    *   Stage specific files: `git.git_add {'files': ['path/to/file1.ts', 'path/to/file2.js']}`.
    *   Stage all changes (use with caution, prefer specific files): `git.git_add {'files': ['.']}`. Verify with `git.git_status` afterwards.
*   **Committing:**
    *   Use descriptive messages referencing the Task/Subtask ID: `git.git_commit {'message': 'Task-123: Implement user authentication endpoint'}`.
    *   For merge commits, often no message change is needed: `execute_command git commit --no-edit`.
*   **Syncing with Remote (`main` branch):**
    *   Ensure you are on the correct branch (`git.git_checkout`).
    *   Fetch latest changes from remote: `execute_command git fetch origin`.
    *   Pull changes for `main`: `git.git_checkout {'branch_name': 'main'}` followed by `execute_command git pull origin main`.
    *   Merge `main` into your feature branch: `git.git_checkout {'branch_name': 'feature/...'}` followed by `execute_command git merge main`.
*   **Pushing Changes:**
    *   Push your feature branch: `execute_command git push origin feature/subtask-<ID>-<AgentID>`. Use `-u` on the first push if needed: `execute_command git push -u origin feature/subtask-<ID>-<AgentID>`.
*   **Pull Requests (GitHub):**
    *   Create PR after pushing: `server-github.create_pull_request {'title': 'Task-123: Feature X Implementation', 'head': 'feature/subtask-<ID>-<AgentID>', 'base': 'main', 'body': 'Implements feature X as per Task-123. Ready for review.', 'assignees': ['coordinator_agent_github_username']}`. (Adjust parameters as needed).
*   **Conflict Resolution:**
    1.  Identify conflicts after a failed `execute_command git merge main` by using `git.git_status`.
    2.  Gather context: `read_file` on conflicted files, `git.git_log` for recent changes, query KM (`memorymesh.search_nodes`), check PM tool/comms.
    3.  Attempt resolution using `replace_in_file` to remove conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`) and integrate changes logically.
    4.  **Crucially:** Run tests (`execute_command <test_command>`) after resolving conflicts locally.
    5.  If tests pass: Stage resolved files (`git.git_add`), commit the merge (`execute_command git commit --no-edit`).
    6.  If tests fail or resolution is complex/unclear: **Escalate** using `request_assistance`, providing details of the conflict, files involved, and resolution attempts. Do not push broken code.

### 8.2 Knowledge Management (MemoryMesh)

*   **Assumed MCP Server:** `memorymesh`.
*   **Core Workflow Integration:** Query before starting tasks, log during execution and upon completion.
*   **Querying Knowledge:**
    *   Use `memorymesh.search_nodes {'query': 'design document for Task-123'}` or `memorymesh.search_nodes {'query': 'authentication implementation details'}`.
    *   Be specific with queries. Use keywords, task IDs, file names, or concept names.
    *   Review search results carefully. If results are too broad or narrow, refine the query.
*   **Adding Knowledge (Nodes):**
    *   Use `memorymesh.add_nodes` with a structured `nodes` array.
    *   **Node Structure:**
        ```json
        {
          "name": "Unique Node Name (e.g., DesignDecision-Task123-AuthFlow)",
          "nodeType": "Standardized Type (e.g., 'DesignDecision', 'CodeSnippet', 'ResearchFinding', 'ConflictResolution', 'FinalArtifact', 'TaskContext', 'AgentAnalysis')",
          "metadata": [
            "task_id: Task-123",
            "agent_id: Agent-XYZ",
            "timestamp: 2025-04-05T00:15:00Z",
            "related_node: CodeSnippet-Task123-AuthHelper", // Link to other nodes
            "file_path: src/auth/utils.ts", // If relevant
            "commit_sha: a1b2c3d4", // If relevant
            "Content: Decided to use JWT for session management due to statelessness requirement.", // The actual knowledge content
            "Keywords: auth, jwt, session, stateless" // For searchability
          ]
        }
        ```
    *   **When to Add:**
        *   Key design decisions and rationale.
        *   Summaries of research findings.
        *   Final code snippets or links to committed code.
        *   Summaries of problem analysis or brainstorming sessions.
        *   Documented conflict resolutions.
        *   Final artifacts or links to them (e.g., documentation files).
        *   Important context received via communication.
*   **Adding Relationships (Edges):**
    *   Use `memorymesh.add_edges` to link related nodes.
    *   **Edge Structure:**
        ```json
        {
          "from": "NodeName1 (e.g., DesignDecision-Task123-AuthFlow)",
          "to": "NodeName2 (e.g., CodeSnippet-Task123-AuthHelper)",
          "edgeType": "Relationship Type (e.g., 'implementedBy', 'informedBy', 'relatedTo', 'resolvesConflict', 'basedOn')"
        }
        ```
    *   **When to Add:** Explicitly link decisions to implementations, findings to decisions, conflicts to resolutions, etc. This builds the contextual graph.
*   **Best Practices:**
    *   Use consistent and descriptive node names and types.
    *   Leverage metadata heavily for context and searchability.
    *   Create edges proactively to represent relationships between pieces of knowledge.
    *   Periodically review relevant sections of the knowledge graph to maintain context.

### 8.3 Team Communications

*   **Assumed MCP Server:** `team-communications` (or equivalent providing the tools below).
*   **Core Principle:** Proactive, clear, and contextual communication is paramount. Agents must actively monitor and participate.
*   **Checking for Messages:**
    *   **Tool:** `check_new_messages` (Expected to return a boolean or count indicating new messages).
    *   **Pattern:** Call this tool *very frequently* (e.g., before starting a significant computation, after completing a step, periodically during long tasks). High frequency is crucial for responsiveness.
    *   **Follow-up:** If `check_new_messages` indicates new messages, immediately call `get_messages`.
*   **Retrieving Messages:**
    *   **Tool:** `get_messages` (Expected to return a list of message objects, potentially filterable).
    *   **Pattern:** Call after `check_new_messages` confirms new messages. Process *all* retrieved messages. Identify messages requiring action or response based on recipient ID, thread ID, or content relevance.
*   **Sending Direct Messages:**
    *   **Tool:** `send_message`.
    *   **Parameters (Example):** `{'recipient_id': 'Agent-ABC', 'message_type': 'query', 'payload': {'task_id': 'Task-123', 'query': 'What was the result of the database migration test?'}}`
    *   **When to Use:** Task handoffs, specific questions, requesting specific information, providing direct feedback, acknowledging tasks.
    *   **Best Practice:** Always include relevant context IDs (Task, Subtask, Commit, File, KM Node) in the payload. Be specific about what is needed or being reported.
*   **Broadcasting Messages:**
    *   **Tool:** `broadcast_message`.
    *   **Parameters (Example):** `{'message_type': 'status_update', 'payload': {'status': 'Project Build Failed', 'details': 'Main build failed due to compilation errors in module X. Investigating.'}}`
    *   **When to Use:** Critical project-wide status updates (e.g., build failures, major blockers, successful deployment), important announcements relevant to all team members. **Use sparingly** to avoid noise.
*   **Reporting Status:**
    *   **Tool:** `report_status` (Tool name TBD, may be part of `team-communications`).
    *   **Parameters (Example):** `{'status': 'busy', 'current_task_id': 'Subtask-456', 'details': 'Implementing authentication middleware'}` or `{'status': 'idle'}` or `{'status': 'blocked', 'blocker_description': 'Waiting for API key from Agent-ABC'}`.
    *   **When to Use:**
        *   Agent comes online/goes offline.
        *   Agent starts a new task/subtask.
        *   Agent completes a task/subtask and becomes idle.
        *   Agent encounters a blocker.
        *   Agent recovers from a blocker.
        *   Periodically during very long-running tasks.
    *   **Goal:** Maintain team awareness of agent availability and current focus.
*   **Requesting Assistance:**
    *   **Tool:** `request_assistance` (Tool name TBD).
    *   **Parameters (Example):** `{'problem_description': 'Merge conflict in src/utils.ts between feature/subtask-123 and main.', 'context': {'task_id': 'Subtask-123', 'file_path': 'src/utils.ts', 'conflicting_branch': 'main'}, 'attempts_made': ['Attempted auto-merge, failed.', 'Reviewed git log, changes seem complex.'], 'required_capability': 'Expert review of conflicting logic', 'suggested_recipient_id': 'Agent-Coordinator'}`.
    *   **When to Use:** After attempting self-resolution (KM search, tool usage, basic troubleshooting) fails.
    *   **Best Practice:** Be extremely specific about the problem, what was tried, the exact context, and what kind of help is needed. Suggest a recipient if appropriate (e.g., coordinator, agent with specific expertise).
*   **Providing Feedback:**
    *   **Tool:** `evaluate_contribution` (Tool name TBD).
    *   **Parameters (Example):** `{'target_message_id': 'msg-789', 'target_artifact_id': 'KMNode-DesignDecision-Task123', 'feedback_type': 'suggestion', 'rating': 'positive', 'comment': 'Clear rationale, but consider adding performance implications.'}`.
    *   **When to Use:** To provide constructive feedback on specific messages, code commits, KM entries, or other artifacts produced by another agent.
    *   **Best Practice:** Be specific, objective, and actionable. Reference the exact item being evaluated.
*   **Sharing Artifacts:**
    *   **Tool:** `transfer_file` (Tool name TBD) or log to KM and share node ID via `send_message`.
    *   **When to Use:** When a specific file (e.g., log file, generated report, data file) needs to be shared directly. For code, prefer pushing to Git and sharing the commit/PR link. For persistent knowledge, prefer logging to KM and sharing the node ID/name.

### 8.4 Project Management (Example: Todoist)

*   **Assumed MCP Server:** `todoist` (or equivalent PM tool server).
*   **Core Workflow Integration:** Coordinators use this to assign tasks; assigned agents use this to update task status upon completion.
*   **Creating Tasks (Coordinator Role):**
    *   **Tool:** `todoist.create_task` (or equivalent like `pm_tool_create_task` from prompts).
    *   **Parameters (Example):** `{'content': 'Subtask-123: Implement login API endpoint', 'project_id': 'Project-Alpha', 'due_string': 'today', 'priority': 4, 'assignee_id': 'Agent-BackendDev-ID'}`. (Parameter names depend on the specific MCP server implementation).
    *   **When to Use:** After decomposing the main task and planning assignments (Step 4 of Problem-Solving Framework). Create one PM task for each delegated subtask.
    *   **Best Practice:** Ensure the task name clearly includes the Subtask ID and a concise description. Link it to the correct project and assignee if possible via the tool's parameters.
*   **Updating/Completing Tasks (Assigned Agent Role):**
    *   **Tool:** `todoist.close_task` (or equivalent like `pm_tool_complete_task` from prompts). May require task ID or exact task content match. Some tools might use `update_task` with a 'completed' status.
    *   **Parameters (Example):** `{'task_id': 'todoist-task-id-12345'}` or `{'content': 'Subtask-123: Implement login API endpoint'}`.
    *   **When to Use:** After successfully completing a subtask, pushing code (if applicable), creating a PR, and notifying the coordinator (Step 8 of Standard Task Execution Workflow).
    *   **Best Practice:** Update the PM tool *after* other completion steps (like PR creation) are done. Ensure you are closing the correct task. If the tool requires content matching, use the exact content used during creation.
*   **Querying Tasks (Optional - Depends on Tool):**
    *   Some PM tools might offer `list_tasks` or `get_task_details`.
    *   **Potential Use:** Agents could query for their assigned tasks, check deadlines, or get details. Coordinators could query project status. (Requires specific tool implementation).

### 8.5 Search & Information Gathering (Example: Brave Search, Fetch)

*   **Assumed MCP Servers:** `brave-search`, `fetch`.
*   **Core Workflow Integration:** Used during problem analysis, context gathering, research-oriented tasks, and resolving certain types of blockers.
*   **Web Search (Brave Search):**
    *   **Tool:** `brave-search.search`.
    *   **Parameters (Example):** `{'query': 'typescript async await error handling best practices', 'num_results': 5}`.
    *   **When to Use:** Researching technical solutions, finding documentation, understanding concepts, looking for libraries or examples.
    *   **Best Practice:** Start with specific queries. If results are poor, broaden the query or try different phrasings. Analyze the summaries/snippets provided in the results before deciding to fetch full content.
*   **Fetching Web Content (Fetch):**
    *   **Tool:** `fetch.fetch`.
    *   **Parameters (Example):** `{'url': 'https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/async_function', 'output_format': 'markdown'}`.
    *   **When to Use:** After identifying a promising URL from search results or documentation links. To get the full content of a page for detailed analysis.
    *   **Best Practice:** Prefer 'markdown' format for easier processing by LLMs unless the raw HTML structure is specifically needed. Be mindful of content length; the tool might truncate long pages (check tool documentation/response for indicators). Summarize key findings from fetched content and log relevant information to KM (`memorymesh.add_nodes`).

### 8.6 File System Operations

*   **Assumed MCP Server:** `server-filesystem` (or equivalent).
*   **Core Workflow Integration:** Reading configuration, accessing project files, writing generated code or documentation, managing temporary files.
*   **Reading Files:**
    *   **Tool:** `server-filesystem.read_file`.
    *   **Parameters (Example):** `{'path': 'src/config/settings.json'}`.
    *   **When to Use:** Accessing code for analysis/modification, reading configuration, loading data files.
    *   **Best Practice:** Always specify the full, correct path relative to the project root. Handle potential "file not found" errors gracefully.
*   **Writing Files:**
    *   **Tool:** `server-filesystem.write_to_file`.
    *   **Parameters (Example):** `{'path': 'docs/generated/api_reference.md', 'content': '# API Reference\n...'}`.
    *   **When to Use:** Creating new files (code, docs, reports), overwriting existing files completely (use with caution).
    *   **Best Practice:** Ensure the path is correct. Be certain before overwriting; consider `read_file` first if unsure about existing content. For targeted edits, prefer `replace_in_file` (if available via a different tool/server or `execute_command` with `sed`/`awk`).
*   **Listing Files:**
    *   **Tool:** `server-filesystem.list_files`.
    *   **Parameters (Example):** `{'path': 'src/utils/', 'recursive': false}`.
    *   **When to Use:** Exploring project structure, finding specific files within a directory.
    *   **Best Practice:** Use `recursive: true` cautiously on large directories. Specify paths precisely.

### 8.7 Docker Operations (Example: docker-mcp)

*   **Assumed MCP Server:** `docker-mcp`.
*   **Core Workflow Integration:** Used by DevOps or Backend agents for managing containerized environments, deployments, and debugging.
*   **Listing Containers:**
    *   **Tool:** `docker-mcp.list-containers`.
    *   **Parameters:** None.
    *   **When to Use:** To check the status of running containers, identify container names/IDs for other operations.
*   **Creating Containers:**
    *   **Tool:** `docker-mcp.create-container`.
    *   **Parameters (Example):** `{'image': 'postgres:15', 'name': 'dev-db', 'ports': {'5432': '5432'}, 'environment': {'POSTGRES_PASSWORD': 'mysecretpassword'}}`.
    *   **When to Use:** Setting up development databases, running specific tools in isolation.
    *   **Best Practice:** Assign meaningful names. Map ports carefully to avoid conflicts. Securely manage environment variables (potentially fetching secrets from a vault via another tool).
*   **Deploying Compose Stacks:**
    *   **Tool:** `docker-mcp.deploy-compose`.
    *   **Parameters (Example):** `{'project_name': 'my-app-stack', 'compose_yaml': 'version: "3.8"\nservices:\n  web:\n    image: myapp-image\n    ports:\n      - "8080:80"\n  redis:\n    image: redis:alpine'}`. (YAML content passed as a string).
    *   **When to Use:** Deploying multi-container applications defined in `docker-compose.yml` files.
    *   **Best Practice:** Fetch the `compose_yaml` content using `server-filesystem.read_file` rather than embedding large YAML strings directly in prompts or tool calls. Ensure the project name is unique.
*   **Getting Logs:**
    *   **Tool:** `docker-mcp.get-logs`.
    *   **Parameters (Example):** `{'container_name': 'dev-db'}`.
    *   **When to Use:** Debugging container issues, checking application output.
    *   **Best Practice:** Specify the correct container name or ID obtained via `list-containers`. Analyze logs for errors or relevant information.

### 8.8 API Testing (Example: postman)

*   **Assumed MCP Server:** `postman`.
*   **Core Workflow Integration:** Used by Backend, Frontend, or QA agents to test APIs during development or as part of CI/CD checks.
*   **Listing Collections/Environments:**
    *   **Tool:** `postman.list_collections`, `postman.list_environments`.
    *   **Parameters (Example):** `{'workspace': 'workspace-id-123'}`.
    *   **When to Use:** To find the relevant collection or environment ID for running tests or managing resources.
*   **Getting Collection/Environment Details:**
    *   **Tool:** `postman.get_collection`, `postman.get_environment`.
    *   **Parameters (Example):** `{'collection_id': 'collection-uuid-abc'}`, `{'environmentId': 'ownerId-envId-xyz'}`.
    *   **When to Use:** To inspect requests within a collection or variables within an environment.
*   **Running Collections (Requires Newman or similar integration - potentially via `execute_command` if `postman` server doesn't directly support runs):**
    *   **Tool:** (Potentially `execute_command` if no direct run tool exists in the `postman` MCP server).
    *   **Command (Example):** `execute_command newman run "https://api.postman.com/collections/{collection_id}?access_key={access_key}" -e "https://api.postman.com/environments/{environment_id}?access_key={access_key}"`. (Requires Newman CLI installed and API keys/access keys).
    *   **Alternative:** If the `postman` server *does* have a run tool (e.g., `run_collection`), use that. `{'collection_id': '...', 'environment_id': '...'}`.
    *   **When to Use:** Executing API tests defined in a Postman collection against a specific environment.
    *   **Best Practice:** Parse the output (Newman report or tool result) to determine pass/fail status and identify specific test failures. Log results appropriately (KM, communication). Ensure necessary API/access keys are securely available (e.g., via environment variables configured for the MCP server).
*   **Managing Environments/Variables (Example):**
    *   **Tool:** `postman.update_environment`.
    *   **Parameters (Example):** `{'environmentId': 'ownerId-envId-xyz', 'environment': {'values': [{'key': 'api_base_url', 'value': 'https://staging.example.com/api'}]}}`.
    *   **When to Use:** Updating environment variables programmatically, e.g., setting the target URL for a staging deployment test run.
    *   **Best Practice:** Update only necessary variables. Be cautious when modifying shared environments.

### 8.9 UI Automation (Examples: playwright-server, omniparser_autogui_mcp)

*   **Assumed MCP Servers:** `playwright-server`, `omniparser_autogui_mcp` (or equivalents).
*   **Core Workflow Integration:** Used by QA agents for E2E testing, or potentially by other agents for interacting with GUIs where no direct API/MCP exists. Requires careful state management.
*   **Playwright (Web UI Automation):**
    *   **Tools:** `playwright_navigate`, `playwright_click`, `playwright_fill`, `playwright_screenshot`, `playwright_get_text_content`, etc.
    *   **Workflow Example (Login Test):**
        1.  `playwright_navigate {'url': 'https://example.com/login'}`
        2.  `playwright_fill {'selector': '#username', 'value': 'testuser'}`
        3.  `playwright_fill {'selector': '#password', 'value': 'password123'}`
        4.  `playwright_click {'selector': 'button[type="submit"]'}`
        5.  (Wait for navigation/element indicating success)
        6.  `playwright_get_text_content {}` -> Check for expected text (e.g., "Welcome, testuser").
        7.  `playwright_screenshot {'name': 'login_success.png'}` (Optional).
    *   **Best Practice:** Use specific and stable selectors (IDs preferred). Insert waits or checks for elements to appear after actions that trigger navigation or UI updates. Handle potential errors (element not found, navigation timeout). Use `get_text_content` or specific element text checks to verify state.
*   **Omniparser (Desktop GUI Automation):**
    *   **Tools:** `omniparser_details_on_screen`, `omniparser_click`, `omniparser_write`, `omniparser_drags`, `omniparser_input_key`, `omniparser_wait`.
    *   **Workflow Example (Save File in Notepad):**
        1.  `omniparser_details_on_screen {}` -> Identify IDs for 'File' menu, 'Save As' item, filename input, 'Save' button.
        2.  `omniparser_click {'id': <file_menu_id>}`
        3.  `omniparser_wait {'time': 0.5}` (Allow menu to open)
        4.  `omniparser_click {'id': <save_as_item_id>}`
        5.  `omniparser_wait {'time': 1}` (Allow dialog to open)
        6.  `omniparser_write {'id': <filename_input_id>, 'content': 'my_document.txt'}`
        7.  `omniparser_click {'id': <save_button_id>}`
    *   **Best Practice:** Always call `omniparser_details_on_screen` before interacting to get current element IDs, as they can change. Use `omniparser_wait` judiciously between steps, especially after clicks that open menus or dialogs. Confirm actions by checking screen details again or looking for expected file system changes (`server-filesystem.list_files`). Desktop automation is inherently brittle; design robust error handling.

### 8.10 Document Processing (Examples: mcp-pandoc, mcp-summarizer)

*   **Assumed MCP Servers:** `mcp-pandoc`, `mcp-summarizer`.
*   **Core Workflow Integration:** Used for converting document formats, extracting text, and summarizing content during research, analysis, or documentation tasks.
*   **Document Conversion (Pandoc):**
    *   **Tool:** `mcp-pandoc.convert-contents`.
    *   **Parameters (Example - DOCX to Markdown):** `{'input_file': 'path/to/document.docx', 'output_format': 'markdown'}` (Result returned directly).
    *   **Parameters (Example - Markdown to PDF):** `{'contents': '# My Document\n...', 'output_format': 'pdf', 'output_file': 'path/to/output.pdf'}` (Requires `output_file` for PDF/DOCX etc.).
    *   **When to Use:** Converting between formats like DOCX, PDF, HTML, Markdown, RST, LaTeX. Extracting text from PDFs/DOCX.
    *   **Best Practice:** Ensure input/output paths and formats are specified correctly. Remember PDF conversion requires TeX Live/MiKTeX installed where the MCP server runs. Handle potential conversion errors. Use `server-filesystem` tools to manage input/output files if needed.
*   **Content Summarization (Summarizer):**
    *   **Tool:** `mcp-summarizer.summarize` (Assuming tool name).
    *   **Parameters (Example):** `{'content': '<long text content read from file or fetched>', 'summary_length': 'medium'}` or `{'source_url': 'https://example.com/article', 'summary_length': 'short'}`. (Parameter names depend on implementation).
    *   **When to Use:** Getting the gist of long documents, web pages, or text blobs quickly. Extracting key points for reports or KM logging.
    *   **Best Practice:** Provide sufficient content for a meaningful summary. Choose an appropriate summary length based on the need. Combine with `fetch` or `read_file` to get the content first. Log the summary and source in KM (`memorymesh.add_nodes`).

*(This document is a living specification and will be updated as the platform and agent capabilities evolve.)*
