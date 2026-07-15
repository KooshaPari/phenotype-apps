# AGSLAG Custom Tools: Workflow Management

This document describes the MCP tools provided within AGSLAG for defining, executing, and managing multi-step workflows involving multiple agents. These tools are typically implemented in `src/tools/workflows.ts` and interact with the system's internal workflow engine and database.

## Tools Overview

*   **`define_workflow`**: Creates a new workflow definition.
*   **`get_workflow_details`**: Retrieves details and status of a specific workflow.
*   **`list_workflows`**: Lists existing workflow definitions.
*   **`delete_workflow`**: Deletes a workflow definition.
*   **`execute_workflow`**: Initiates the execution of a defined workflow.
*   **`report_step_completion`**: Allows an agent to mark an assigned step as complete.
*   **`report_step_failure`**: Allows an agent to mark an assigned step as failed.

## Tool Details

### 1. `define_workflow`

Defines a new multi-step workflow.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "creator_id": {
      "type": "string",
      "description": "Your unique Agent ID."
    },
    "name": {
      "type": "string",
      "description": "A descriptive name for the workflow."
    },
    "description": {
      "type": "string",
      "description": "Optional detailed description of the workflow's purpose."
    },
    "steps": {
      "type": "array",
      "minItems": 1,
      "description": "An ordered list of steps defining the workflow.",
      "items": {
        "type": "object",
        "properties": {
          "step_id": {
            "type": "string",
            "description": "Unique identifier for this step within the workflow."
          },
          "description": {
            "type": "string",
            "description": "Description of the task for this step."
          },
          "assigned_role": {
            "type": "string",
            "description": "Optional: Role required to perform this step."
          },
          "assigned_agent_id": {
            "type": "string",
            "description": "Optional: Specific agent ID assigned to this step."
          },
          "depends_on": {
            "type": "array",
            "items": { "type": "string" },
            "description": "Optional: List of step_ids that must be completed before this step can start."
          },
          "tool_to_call": {
            "type": "string",
            "description": "Optional: Specific tool to execute for this step."
          },
          "tool_arguments": {
            "type": "object",
            "additionalProperties": true,
            "description": "Arguments for the tool, if specified."
          }
        },
        "required": ["step_id", "description"]
      }
    }
  },
  "required": ["creator_id", "name", "steps"]
}
```

**Output:** Confirmation message and the newly created workflow object, including its generated ID and initial 'pending' status.

### 2. `get_workflow_details`

Retrieves the full definition and current status (including step statuses) of a specific workflow.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "workflow_id": {
      "type": "string",
      "description": "The ID of the workflow to retrieve details for."
    }
  },
  "required": ["workflow_id"]
}
```

**Output:** The full workflow object as stored in the database.

### 3. `list_workflows`

Lists summaries of existing workflows, optionally filtering by the creator agent.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "creator_id": {
      "type": "string",
      "description": "Optional: Filter workflows by creator agent ID."
    }
  }
}
```

**Output:** An array of workflow summaries, including ID, name, creator, step count, and status.

### 4. `delete_workflow`

Deletes a workflow definition. Only the agent who created the workflow can delete it.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "workflow_id": {
      "type": "string",
      "description": "The ID of the workflow to delete."
    },
    "requesting_agent_id": {
      "type": "string",
      "description": "Your unique Agent ID (must be the workflow creator)."
    }
  },
  "required": ["workflow_id", "requesting_agent_id"]
}
```

**Output:** Confirmation message.

### 5. `execute_workflow`

Starts the execution of a defined workflow. This sets the workflow status to 'running' and attempts to delegate the initial steps (those with no dependencies) to appropriate agents based on role or specific assignment.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "workflow_id": {
      "type": "string",
      "description": "The ID of the workflow to start executing."
    },
    "triggering_agent_id": {
      "type": "string",
      "description": "Your unique Agent ID initiating the execution."
    }
  },
  "required": ["workflow_id", "triggering_agent_id"]
}
```

**Output:** Confirmation message indicating how many initial steps were delegated and any issues encountered during delegation (e.g., no available agents for a required role).

### 6. `report_step_completion`

Used by an agent to report that it has successfully completed its assigned step in a workflow. This updates the step's status to 'completed' and triggers the system to check if any subsequent steps can now be initiated.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "workflow_id": {
      "type": "string",
      "description": "ID of the workflow containing the step."
    },
    "step_id": {
      "type": "string",
      "description": "ID of the step being reported as complete."
    },
    "agent_id": {
      "type": "string",
      "description": "Your unique Agent ID reporting completion."
    },
    "result": {
      "description": "Optional: The output or result produced by the step.",
      "anyOf": [ { "type": "object" }, { "type": "array" }, { "type": "string" }, { "type": "number" }, { "type": "boolean" }, { "type": "null" } ]
    }
  },
  "required": ["workflow_id", "step_id", "agent_id"]
}
```

**Output:** Confirmation message.

### 7. `report_step_failure`

Used by an agent to report that it encountered an error and could not complete its assigned step. This updates the step's status to 'failed', records the error message, and sets the overall workflow status to 'failed'.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "workflow_id": {
      "type": "string",
      "description": "ID of the workflow containing the step."
    },
    "step_id": {
      "type": "string",
      "description": "ID of the step being reported as failed."
    },
    "agent_id": {
      "type": "string",
      "description": "Your unique Agent ID reporting the failure."
    },
    "error_message": {
      "type": "string",
      "description": "A description of the error that occurred."
    }
  },
  "required": ["workflow_id", "step_id", "agent_id", "error_message"]
}
```

**Output:** Confirmation message.

## Workflow Lifecycle

1.  **Definition:** An agent uses `define_workflow` to create the workflow structure. Status: `pending`.
2.  **Execution:** An agent uses `execute_workflow` to start the process. Status: `running`. Initial steps are assigned.
3.  **Progression:** Assigned agents perform their tasks. Upon completion, they call `report_step_completion`. The system automatically checks dependencies and assigns the next available steps (`_advanceWorkflow` logic). Step Statuses: `pending` -> `in_progress` -> `completed`.
4.  **Failure:** If an agent encounters an error, it calls `report_step_failure`. Step Status: `failed`. Workflow Status: `failed`.
5.  **Completion:** Once all steps are successfully completed, the workflow status becomes `completed`.
