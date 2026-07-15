# AGSLAG Custom Tools: Project & Goal Management

This document describes the MCP tools provided within AGSLAG for creating and managing projects and goals, which are core concepts in the platform's digital product development lifecycle. These tools are typically implemented in `src/tools/project_management.ts` and interact with the system's internal database.

## Tools Overview

*   **`create_project`**: Creates a new project with detailed initial setup.
*   **`create_goal`**: Defines a new goal, optionally linking it to a project.
*   **`get_project_details`**: Retrieves details for a specific project.
*   **`get_goal_details`**: Retrieves details for a specific goal.
*   **`list_projects`**: Lists existing projects.
*   **`list_goals`**: Lists existing goals.
*   **`link_workflow_to_goal`**: Associates a workflow with a goal.
*   **`link_step_to_external_task`**: Links a workflow step to an external task tracker.

## Tool Details

### 1. `create_project`

Creates a new project within the AGSLAG system, including metadata, methodology, team structure, and initial agent assignments.

**Input Schema:** (Complex, based on `CreateProjectInputSchema` in the source)
*   `projectMetadata`: Object containing `projectId`, `projectName`, `projectDescription`, `creationTimestamp`, `status`.
*   `selectedMethodology`: String (e.g., 'agile').
*   `teamStructure`: Object defining required roles, counts, and capabilities.
*   `agentAssignments`: Array of objects mapping `roleName` to `assignedAgentId`.
*   `assignmentFailures`: Array of objects detailing roles that couldn't be assigned and the reason.

**Output:** An object containing the creation `status` ('Success' or 'Partial Failure'), the new `projectId`, a `projectUrl`, and a summary `message`.

### 2. `create_goal`

Defines a new goal or objective, optionally associating it with a parent project.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "name": {
      "type": "string",
      "description": "The name of the goal."
    },
    "description": {
      "type": "string",
      "description": "Optional: A detailed description of the goal."
    },
    "project_id": {
      "type": "string",
      "description": "Optional: The ID of the parent project this goal belongs to."
    },
    "initial_status": {
      "type": "string",
      "enum": ["backlog", "todo", "in_progress", "done", "blocked"],
      "default": "backlog",
      "description": "Initial status of the goal."
    }
  },
  "required": ["name"]
}
```

**Output:** Confirmation message and the newly created goal object, including its generated ID.

### 3. `get_project_details`

Retrieves the full details of a specific project, including its metadata, status, goals, team structure, assignments, etc.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "project_id": {
      "type": "string",
      "description": "The ID of the project to retrieve."
    }
  },
  "required": ["project_id"]
}
```

**Output:** The full project object as stored in the database.

### 4. `get_goal_details`

Retrieves the full details of a specific goal, including its description, status, parent project, and linked workflows.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "goal_id": {
      "type": "string",
      "description": "The ID of the goal to retrieve."
    }
  },
  "required": ["goal_id"]
}
```

**Output:** The full goal object as stored in the database.

### 5. `list_projects`

Lists summaries of existing projects, optionally filtering by status.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "status": {
      "type": "string",
      "enum": ["planning", "active", "completed", "on_hold", "archived"],
      "description": "Optional: Filter projects by status."
    }
  }
}
```

**Output:** An array of project objects matching the filter.

### 6. `list_goals`

Lists summaries of existing goals, optionally filtering by parent project ID or status.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "project_id": {
      "type": "string",
      "description": "Optional: Filter goals by parent project ID."
    },
    "status": {
      "type": "string",
      "enum": ["backlog", "todo", "in_progress", "done", "blocked"],
      "description": "Optional: Filter goals by status."
    }
  }
}
```

**Output:** An array of goal objects matching the filters.

### 7. `link_workflow_to_goal`

Associates a previously defined workflow (using `define_workflow`) with a specific goal. This helps track which workflow is intended to achieve which objective.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "workflow_id": {
      "type": "string",
      "description": "The ID of the workflow."
    },
    "goal_id": {
      "type": "string",
      "description": "The ID of the goal to link the workflow to."
    }
  },
  "required": ["workflow_id", "goal_id"]
}
```

**Output:** Confirmation message.

### 8. `link_step_to_external_task`

Links a specific step within an AGSLAG workflow to a task ID in an external project management system (e.g., Jira, Linear). This allows for traceability between internal automated steps and external tracking systems.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "workflow_id": {
      "type": "string",
      "description": "The ID of the workflow containing the step."
    },
    "step_id": {
      "type": "string",
      "description": "The ID of the step to link."
    },
    "external_system": {
      "type": "string",
      "description": "The name of the external system (e.g., 'linear', 'jira')."
    },
    "external_task_id": {
      "type": "string",
      "description": "The ID of the task in the external system."
    }
  },
  "required": ["workflow_id", "step_id", "external_system", "external_task_id"]
}
```

**Output:** Confirmation message.
