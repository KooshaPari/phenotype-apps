# MCP Codebase Analysis

## Overview
The Model Context Protocol (MCP) is a standardized communication protocol designed to facilitate interactions between Large Language Model (LLM) applications and external services. The implementation focuses on enabling different AI models to communicate and collaborate effectively for better teamwork and problem solving.

## Key Components

### Core Features
- **Cross-Agent Communication**: Send and receive messages between different AI models
- **Multi-Model Collaboration**: Tools for task decomposition, assignment, and result merging
- **Model-Agnostic Design**: Support for Claude, DeepSeek, OpenAI, and Gemini models
- **Multiple Transport Mechanisms**: Support for both stdio and SSE transports
- **Collaboration Tools**: Task decomposition, subtask assignment, and result merging
- **Collaboration Prompts**: Structured prompts for effective multi-model collaboration
- **Agent Resources**: Information about available agents and their capabilities

### Project Structure
- `src/`: Source code for the MCP server
  - `index.ts`: Main server entry point
  - `tools/`: Tool implementations
    - `communication.ts`: Tools for agent-to-agent communication
    - `collaboration.ts`: Tools for multi-model collaboration
    - `workflows.ts`: Tools for workflow management
    - `knowledge_proxy.ts`: Tools for knowledge sharing
    - `roles.ts`: Tools for role management
    - `file_locking.ts`: Tools for file locking
    - `anti_jam.ts`: Tools for preventing system jams
    - `project_management.ts`: Tools for project management
  - `resources/`: Resource implementations
    - `agents.ts`: Resources for agent information
  - `prompts/`: Prompt implementations
    - `collaboration.ts`: Prompts for multi-model collaboration
  - `dbUtils.ts`: Database utilities for data persistence
- `examples/`: Example client implementations
  - `claude-desktop-client.js`: Client for Claude Desktop
  - `roocode-cli-client.js`: Client for command-line instances
- `docs/`: Documentation
- `tests/`: Test files

### Data Models
The system uses several key data models defined in `dbUtils.ts`:

1. **AgentInfo**: Represents an agent in the system
   - Properties: id, name, role, model_type, model_version, capabilities, status, current_task, registered_at, last_heartbeat, group, progress_metric, error_info, current_plan_summary
   - Status values: "available" | "working" | "idle" | "offline" | "error"

2. **Message**: Represents a message between agents
   - Properties: id, sender_id, recipient_id, recipient_ids, content, thread_id, urgent, timestamp

3. **Thread**: Represents a discussion thread
   - Properties: id, title, creator_id, participants, created_at

4. **Workflow/WorkflowStep**: Represents workflows and their steps
   - Workflow properties: id, name, description, created_at, creator_id, steps, status, updated_at
   - WorkflowStep properties: step_id, description, assigned_role, assigned_agent_id, depends_on, tool_to_call, tool_arguments, status, result, error, project_id, goal_id, external_task_id

5. **Project/Goal**: Represents projects and goals
   - Project properties: id, name, description, status, lead_agent_id, goal_ids, created_at, updated_at
   - Goal properties: id, name, description, status, parent_project_id, workflow_ids, created_at, updated_at

### Communication Tools
The system provides several tools for agent communication:

1. **send_message**: Send a message to another agent or thread
2. **get_messages**: Retrieve messages for an agent
3. **create_thread**: Create a new discussion thread
4. **join_thread**: Join an existing thread
5. **broadcast_message**: Send a message to multiple agents
6. **check_new_messages**: Check for unread messages
7. **transfer_file**: Transfer files between agents

### Agent Management
The system includes tools for agent registration and status management:

1. **register_agent**: Register an agent with the server
2. **deregister_agent**: Remove an agent from the system
3. **set_agent_status**: Set an agent's status
4. **report_status**: Report an agent's status and details
5. **get_agent_status**: Get an agent's status
6. **find_agents**: Find agents based on criteria

### Database Management
The system uses a JSON file (`db.json`) for data persistence with helper functions:

1. **readDb**: Read data from the database
2. **writeDb**: Write data to the database
3. **getAgent**: Get a specific agent
4. **getAllAgents**: Get all agents, optionally filtered
5. **updateAgentStatus**: Update an agent's status

### Server Initialization
The server is initialized in `index.ts` with various components:

1. Register communication tools
2. Register collaboration tools
3. Register agent resources
4. Register workflow tools
5. Register knowledge proxy tools
6. Register role tools
7. Register file locking tools
8. Register anti-jam tools
9. Register project management tools
10. Register collaboration prompts

The server also includes periodic cleanup for inactive agents and old threads.

## Cline Task Analysis
The Cline task involves adding agent status functionality to the MCP system. The task requires:

1. Adding status values: available, working, idle, offline, error
2. Implementing tools for setting and checking agent status
3. Updating the agent data structure to include status information
4. Ensuring other agents can check the status of an agent

This functionality appears to already be implemented in the current codebase, with the AgentInfo interface including a status field with the appropriate values, and tools for setting and checking agent status.
