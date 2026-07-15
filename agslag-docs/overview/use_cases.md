# Use Cases

This document describes key use cases for the Agslag platform, detailing interactions between actors (users/systems) and the platform to achieve specific goals.

## Use Case 1: Connect a New MCP Server

*   **Actor:** Developer
*   **Goal:** To register and connect a newly developed MCP server to the central Agslag system.
*   **Preconditions:**
    *   Developer has developed an MCP server compliant with the protocol.
    *   Developer has access credentials/permissions to the Agslag platform.
    *   The MCP server is running and accessible.
*   **Main Success Scenario:**
    1.  Developer accesses the Agslag dashboard or uses a CLI tool.
    2.  Developer initiates the "Add New Server" process.
    3.  Developer provides the necessary connection details (e.g., address, port, authentication info, server name).
    4.  Agslag system attempts to connect to the provided MCP server endpoint.
    5.  System verifies the connection and registers the server.
    6.  System confirms successful registration to the Developer.
    7.  The new MCP server appears in the list of connected servers on the dashboard.
*   **Extensions (Alternative Flows):**
    *   5a. Connection Fails: System reports connection error details to the Developer. Use case ends.
    *   5b. Authentication Fails: System reports authentication error. Use case ends.
*   **Postconditions:**
    *   On success: The MCP server is registered and available for use by agents within the Agslag platform.
    *   On failure: The MCP server is not registered.

## Use Case 2: Execute a Workflow

*   **Actor:** Product Manager (or Automated Trigger)
*   **Goal:** To execute a pre-defined workflow involving agents and MCP tools.
*   **Preconditions:**
    *   A workflow has been defined and saved in the Agslag platform.
    *   Required agents and MCP servers for the workflow are running and connected.
    *   Input data for the workflow is available (if required).
*   **Main Success Scenario:**
    1.  Actor triggers the workflow execution via the dashboard or an API call.
    2.  Agslag system identifies the workflow definition.
    3.  System orchestrator initiates the first step of the workflow, potentially assigning it to an agent.
    4.  Agent performs its task, possibly interacting with MCP tools via connected servers.
    5.  Workflow progresses through defined steps, passing data between agents/tools as needed.
    6.  Orchestrator monitors the execution status of each step.
    7.  Workflow completes successfully.
    8.  System logs the execution details and results.
    9.  System notifies the Actor (if configured) of completion.
*   **Extensions (Alternative Flows):**
    *   4a. Agent Fails: System logs the error, potentially triggers a retry or failure handler defined in the workflow.
    *   4b. MCP Tool Fails: Agent reports failure to the orchestrator. System logs the error, potentially triggers a retry or failure handler.
    *   7a. Workflow Fails: System logs the failure state and details. Notifies Actor.
*   **Postconditions:**
    *   On success: The workflow is executed, results are generated and stored/delivered as defined. Execution logs are available.
    *   On failure: The workflow execution is halted or completed with errors. Failure logs are available.

---

*This is an initial draft. Use cases should be expanded with more detail, covering various scenarios and edge cases.*