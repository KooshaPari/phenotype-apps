# AGSLAG Custom Tools: Collaboration & Orchestration

This document describes the MCP tools provided within AGSLAG designed to facilitate collaboration, task management, and orchestration among multiple AI agents. These tools enable complex workflows where agents work together on tasks, share progress, request assistance, and evaluate contributions. They are typically implemented in `src/tools/collaboration.ts`.

## Tools Overview

*   **`decompose_task`**: Breaks a complex task into subtasks. *(Mock Implementation)*
*   **`assign_subtasks`**: Assigns subtasks to suitable agents. *(Mock Implementation)*
*   **`merge_results`**: Combines results from multiple subtasks/agents. *(Mock Implementation)*
*   **`compare_models`**: Compares capabilities of different agents. *(Mock Implementation)*
*   **`create_collaboration_session`**: Formally starts a collaboration session. *(Mock Implementation)*
*   **`request_assistance`**: Allows an agent to ask for help from others.
*   **`delegate_subtask`**: Assigns a specific subtask from a coordinator to another agent.
*   **`evaluate_contribution`**: Provides feedback on an agent's work.
*   **`share_plan_update`**: Shares an agent's current plan and status.
*   **`spawn_agent`**: (Simulated) Initiates the creation of a new agent instance.

## Tool Details

### 1. `decompose_task` *(Mock Implementation)*

Takes a high-level task and breaks it down into smaller, manageable subtasks, considering the capabilities of available agents.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "task": { "type": "string", "description": "The high-level, complex task description to be broken down." },
    "available_models": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "model_type": { "type": "string" },
          "capabilities": { "type": "array", "items": { "type": "string" } }
        },
        "required": ["id", "model_type", "capabilities"]
      },
      "description": "List of available agent objects, including their IDs and capabilities, to inform decomposition."
    }
  },
  "required": ["task", "available_models"]
}
```

**Output:** An array of subtask objects, each with an `id`, `description`, and `required_capabilities`.

### 2. `assign_subtasks` *(Mock Implementation)*

Assigns decomposed subtasks to available agents based on their required capabilities.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "subtasks": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "description": { "type": "string" },
          "required_capabilities": { "type": "array", "items": { "type": "string" } }
        },
        "required": ["id", "description", "required_capabilities"]
      },
      "description": "List of subtask objects, each including an ID, description, and required capabilities."
    },
    "available_models": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "id": { "type": "string" },
          "model_type": { "type": "string" },
          "capabilities": { "type": "array", "items": { "type": "string" } }
        },
        "required": ["id", "model_type", "capabilities"]
      },
      "description": "List of available agent objects, including their IDs and capabilities."
    }
  },
  "required": ["subtasks", "available_models"]
}
```

**Output:** An array detailing the assignment of each subtask to an agent ID (or indicates if unassigned).

### 3. `merge_results` *(Mock Implementation)*

Combines results received from different agents for various subtasks into a single, coherent output.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "results": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "subtask_id": { "type": "string", "description": "ID of the subtask this result corresponds to." },
          "model_id": { "type": "string", "description": "ID of the agent that produced this result." },
          "content": { "type": "string", "description": "The textual content of the result produced for the subtask." }
        },
        "required": ["subtask_id", "model_id", "content"]
      },
      "description": "An array of result objects from different agents/subtasks to be combined."
    }
  },
  "required": ["results"]
}
```

**Output:** A string containing the merged results.

### 4. `compare_models` *(Mock Implementation)*

Compares the capabilities of a list of specified agents.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "model_ids": { "type": "array", "items": { "type": "string" }, "description": "List of unique Agent IDs to compare." },
    "capabilities": { "type": "array", "items": { "type": "string" }, "description": "Optional: List of specific capability names to focus the comparison on." }
  },
  "required": ["model_ids"]
}
```

**Output:** An object containing the comparison data for the specified models and capabilities.

### 5. `create_collaboration_session` *(Mock Implementation)*

Sets up a formal context for collaboration, potentially creating a dedicated communication channel or context.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "title": { "type": "string", "description": "A clear title for the collaboration session (e.g., 'Develop Authentication Feature')." },
    "description": { "type": "string", "description": "A detailed description of the overall goal and objectives of the collaboration." },
    "participant_model_ids": { "type": "array", "items": { "type": "string" }, "description": "List of unique Agent IDs of all agents participating in this session." },
    "coordinator_model_id": { "type": "string", "description": "The unique Agent ID of the agent designated to coordinate this session." }
  },
  "required": ["title", "description", "participant_model_ids", "coordinator_model_id"]
}
```

**Output:** Details of the created session, including its ID and status.

### 6. `request_assistance`

Allows an agent to request help on a task, either broadcasting to agents with needed capabilities or targeting specific agents.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "requesting_agent_id": { "type": "string", "description": "Your unique Agent ID." },
    "task_description": { "type": "string", "description": "Clear description of the task or problem you need help with." },
    "required_capabilities": { "type": "array", "items": { "type": "string" }, "description": "Optional: List of specific capabilities needed from the assisting agent(s). Used for broadcast requests." },
    "target_agent_id": { "oneOf": [ { "type": "string" }, { "type": "array", "items": { "type": "string" } } ], "description": "Optional: The unique Agent ID (or array of IDs) of specific agent(s) you want to request help from directly." },
    "urgency": { "type": "string", "enum": ["high", "medium", "low"], "default": "medium", "description": "How urgently assistance is needed." }
  },
  "required": ["requesting_agent_id", "task_description"]
}
```

**Output:** Confirmation that the request was sent, including the message ID and how many/which agents were targeted. Returns a failure status if no suitable agents are found for a broadcast request.

### 7. `delegate_subtask`

Enables a coordinator agent to assign a specific subtask to another agent.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "coordinator_agent_id": { "type": "string", "description": "Your unique Agent ID (acting as coordinator)." },
    "assigned_agent_id": { "type": "string", "description": "The unique Agent ID of the agent who will perform the subtask." },
    "subtask_id": { "type": "string", "description": "A unique identifier for this specific subtask instance." },
    "subtask_description": { "type": "string", "description": "A clear and detailed description of the work to be done for this subtask." },
    "parent_task_id": { "type": "string", "description": "Optional: The ID of the main task this subtask contributes to." },
    "deadline": { "type": "string", "format": "date-time", "description": "Optional: Suggested completion deadline in ISO 8601 format (e.g., '2025-04-01T17:00:00Z')." },
    "contextual_data": { "oneOf": [ { "type": "string" }, { "type": "object", "additionalProperties": true } ], "description": "Optional: Relevant data/context for the subtask (e.g., file paths, previous results)." }
  },
  "required": ["coordinator_agent_id", "assigned_agent_id", "subtask_id", "subtask_description"]
}
```

**Output:** Confirmation that the subtask was delegated, including the message ID sent to the assigned agent. Updates the assigned agent's status.

### 8. `evaluate_contribution`

Allows an agent to provide structured feedback or comments on another agent's contribution (identified by a reference like a message ID or subtask ID).

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "evaluating_agent_id": { "type": "string", "description": "Your unique Agent ID." },
    "contributing_agent_id": { "type": "string", "description": "The unique Agent ID of the agent whose work you are evaluating." },
    "contribution_ref": { "type": "string", "description": "A reference identifying the specific contribution being evaluated (e.g., a message ID like 'msg-12345', or a subtask ID like 'subtask-2')." },
    "evaluation_criteria": {
      "type": "object",
      "properties": {
        "clarity": { "type": "number", "minimum": 1, "maximum": 5, "description": "Rating for clarity (1=Unclear, 5=Very Clear)" },
        "relevance": { "type": "number", "minimum": 1, "maximum": 5, "description": "Rating for relevance to the task/goal (1=Irrelevant, 5=Highly Relevant)" },
        "completeness": { "type": "number", "minimum": 1, "maximum": 5, "description": "Rating for completeness (1=Incomplete, 5=Fully Complete)" },
        "actionability": { "type": "number", "minimum": 1, "maximum": 5, "description": "Rating for how easy it is to act upon or use this contribution (1=Not Actionable, 5=Very Actionable)" }
      },
      "description": "Optional: Provide structured ratings based on criteria."
    },
    "comments": { "type": "string", "description": "Optional: Provide specific textual feedback, suggestions, or comments." }
  },
  "required": ["evaluating_agent_id", "contributing_agent_id", "contribution_ref"]
}
```

**Output:** Confirmation that the evaluation was logged. (The actual feedback mechanism might involve sending a message or updating a database record).

### 9. `share_plan_update`

Enables an agent to share its current plan, progress, next steps, blockers, and confidence level with other agents or within a thread.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "sender_id": { "type": "string", "description": "Your unique Agent ID." },
    "recipient_id": { "type": "string", "description": "Optional: Direct recipient Agent ID." },
    "thread_id": { "type": "string", "description": "Optional: Thread ID to post the update in." },
    "plan_summary": { "type": "string", "description": "A concise summary of the current plan or update." },
    "current_step": { "type": "string", "description": "Optional: Description of the step currently being worked on." },
    "next_steps": { "type": "array", "items": { "type": "string" }, "description": "Optional: List of planned next steps." },
    "blockers": { "type": "array", "items": { "type": "string" }, "description": "Optional: Any obstacles or blockers encountered." },
    "confidence_level": { "type": "number", "minimum": 0, "maximum": 1, "description": "Optional: Confidence level (0-1) in the current plan's success." },
    "related_task_id": { "type": "string", "description": "Optional: Link this plan update to a specific task/subtask ID." }
  },
  "required": ["sender_id", "plan_summary"]
  // Note: Logic ensures either recipient_id or thread_id is provided.
}
```

**Output:** Confirmation that the plan update was sent, including the message ID.

### 10. `spawn_agent` *(Simulated)*

Initiates the process of creating a new agent instance. **Important:** This tool does *not* fully create the agent. It prepares the initial system prompt and returns instructions for an external process (like the user, a script, or UI automation) to actually launch the agent instance (e.g., start Claude Desktop, run a command).

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "requesting_agent_id": { "type": "string", "description": "Your unique Agent ID making the spawn request." },
    "agent_type": { "type": "string", "enum": ["cline-vscode", "claude-desktop", "roocode-vscode", "generic-worker"], "description": "Type of agent instance to spawn." },
    "role": { "type": "string", "description": "Desired role for the new agent (e.g., 'SWE', 'QA', 'Data Analyst')." },
    "capabilities": { "type": "array", "items": { "type": "string" }, "description": "List of capabilities for the new agent." },
    "initial_task_description": { "type": "string", "description": "Optional: A brief description of the first task for the new agent." }
  },
  "required": ["requesting_agent_id", "agent_type", "role", "capabilities"]
}
```

**Output:** Instructions for the next step required to actually spawn the agent (e.g., "Execute command X", "Use UI automation sequence Y"). Includes the proposed agent ID and the initial system prompt the new agent should receive upon launch.
