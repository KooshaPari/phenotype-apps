# AGSLAG Custom Tools: Role Management

This document describes the MCP tools provided within AGSLAG for managing the roles assigned to different agents within the multi-agent system. Roles help define the responsibilities and capabilities of agents. These tools are typically implemented in `src/tools/roles.ts`.

## Tools Overview

*   **`list_roles`**: Lists the predefined roles available in the system.
*   **`assign_role`**: Assigns or updates the role for a specific agent.

## Tool Details

### 1. `list_roles`

Retrieves the list of predefined roles recognized by the AGSLAG system.

**Input Schema:**

```json
{} // No input parameters
```

**Output:** An array containing the names of the predefined roles (e.g., ["Orchestrator", "Worker", "SWE", "QA", "Researcher", ...]).

### 2. `assign_role`

Assigns a specific role to an agent or updates their existing role.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "agent_id": {
      "type": "string",
      "description": "The unique ID of the agent whose role is being assigned/updated."
    },
    "role": {
      "type": "string",
      "description": "The new role to assign to the agent. Should ideally be from the list provided by `list_roles`."
    },
    "assigning_agent_id": {
      "type": "string",
      "description": "Your unique Agent ID (acting as coordinator/admin)."
      // Note: Authorization based on this ID might be implemented.
    }
  },
  "required": ["agent_id", "role", "assigning_agent_id"]
}
```

**Output:** Confirmation message indicating the role update.

## Predefined Roles (Example List)

The system typically defines a set of standard roles, which might include:

*   `Orchestrator`: Manages workflows and coordinates other agents.
*   `Worker`: General purpose agent for executing tasks.
*   `SWE`: Software Engineer agent, focused on code development.
*   `QA`: Quality Assurance agent, focused on testing and validation.
*   `Researcher`: Agent specialized in gathering information.
*   `Data Analyst`: Agent specialized in analyzing data.
*   `Planner`: Agent responsible for creating plans and strategies.
*   `Reviewer`: Agent specialized in reviewing code or documents.

*(Use the `list_roles` tool to get the current, definitive list within the running system).*
