# Knowledge Management Strategies for Large Context Windows

Leveraging large context windows (e.g., 2M tokens) requires effective strategies for managing the vast amount of information that can be loaded. This document outlines patterns for efficient knowledge retrieval and application within such contexts.

## Core Principles

1.  **Structured Context:** Don't just dump information. Organize the context logically using clear headings, separators, and file identifiers to help the model navigate and locate relevant information.
2.  **Prioritization:** Load the most critical information first (e.g., core logic, interfaces, task description) followed by supporting details.
3.  **Summarization and Abstraction:** Use the model itself or pre-processing steps to summarize large code sections or documents before loading them, especially if the full detail isn't immediately necessary.
4.  **Persistent Knowledge:** Utilize tools like `memorymesh` to store key findings, summaries, decisions, and architectural details persistently, reducing the need to reload or re-derive the same information repeatedly.
5.  **Targeted Retrieval Prompts:** Craft prompts that specifically ask the model to retrieve and synthesize information from different parts of the loaded context.

## Patterns and Strategies

### 1. Context Structuring and Indexing

*   **Strategy:** Use Markdown headings, code fences with filenames, or XML-like tags (if the model responds well) to clearly delineate different files or sections within the prompt context. Create a "Table of Contents" or index at the beginning of the prompt listing the loaded files/sections.
*   **Example Prompt Structure:**

```
## Task Description
[Detailed task]

## Context Index
- `src/core/auth.py`: Core authentication logic
- `src/models/user.py`: User data model
- `docs/api/auth.md`: Existing API documentation for auth endpoints
- `logs/auth_errors.log`: Recent authentication error logs (snippet)

## File: src/core/auth.py
```python
[Content of auth.py]
```

## File: src/models/user.py
```python
[Content of user.py]
```

## Document: docs/api/auth.md
```markdown
[Content of auth.md]
```

## Log Snippet: logs/auth_errors.log
```
[Error log lines]
```

**Request:** Based *only* on the context provided above, analyze the recent authentication errors in `logs/auth_errors.log` and correlate them with the logic in `src/core/auth.py` and the user model in `src/models/user.py`. Suggest potential causes.
```

### 2. On-Demand Detail Expansion

*   **Strategy:** Start with summaries or high-level descriptions (e.g., generated using the Hierarchical Summarization pattern from Code Analysis). If the model needs more detail about a specific component during its reasoning process, use a subsequent prompt to load the full source code for that component.
*   **Example Flow:**
    1.  **Prompt 1:** Load summaries of Module A, B, C. Ask for high-level analysis.
    2.  **Model Response:** Identifies a potential issue in the interaction between Module A and B.
    3.  **Prompt 2:** "You identified an issue between Module A and B. Here is the full source code for `module_a/service.py` and `module_b/client.py` [Load files using `read_file`]. Please perform a detailed analysis of their interaction related to the previously identified issue."

### 3. Contextual Markers and Retrieval Prompts

*   **Strategy:** Insert unique markers or comments within the loaded context (especially for large files or documents). Then, instruct the model to specifically search for these markers to find relevant sections.
*   **Example:**
    *   In the loaded context for `large_config.yaml`:
        ```yaml
        # --- Database Configuration --- START ---
        database:
          host: ...
          # ... other db settings
        # --- Database Configuration --- END ---

        # --- Payment Gateway Settings --- START ---
        payment:
          api_key: ...
        # --- Payment Gateway Settings --- END ---
        ```
    *   **Prompt:** "Retrieve the database configuration settings from the `large_config.yaml` file provided in context. Look for the section marked between `# --- Database Configuration --- START ---` and `# --- Database Configuration --- END ---`."

### 4. Persistent Knowledge with `memorymesh`

*   **Strategy:** As analysis progresses or documentation is generated, store key pieces of information as nodes and edges in `memorymesh`.
    *   Store summaries of modules/files.
    *   Store identified dependencies between components (`ModuleA` -> `DEPENDS_ON` -> `ModuleB`).
    *   Store architectural decisions or identified patterns (`AuthService` -> `IMPLEMENTS` -> `SingletonPattern`).
    *   Store requirements or task breakdowns.
*   **Retrieval:** Before starting a new related task, query `memorymesh` (`search_nodes` or `open_nodes`) for relevant existing knowledge and load it into the context window to bootstrap the model.
*   **Example `memorymesh` Usage:**
    ```tool_code
    <use_mcp_tool>
      <server_name>memorymesh</server_name>
      <tool_name>add_nodes</tool_name>
      <arguments>
      {
        "nodes": [
          {
            "name": "AuthService",
            "nodeType": "ComponentSummary",
            "metadata": ["Handles user authentication using JWT.", "Depends on UserRepository."]
          },
          {
            "name": "UserRepository",
            "nodeType": "ComponentSummary",
            "metadata": ["Provides data access for User objects.", "Uses Postgres database."]
          }
        ]
      }
      </arguments>
    </use_mcp_tool>
    <use_mcp_tool>
      <server_name>memorymesh</server_name>
      <tool_name>add_edges</tool_name>
      <arguments>
      {
        "edges": [
          { "from": "AuthService", "to": "UserRepository", "edgeType": "DEPENDS_ON" }
        ]
      }
      </arguments>
    </use_mcp_tool>
    ```
    *   **Later Prompt:** "I need to add password reset functionality. First, retrieve the summaries for `AuthService` and `UserRepository` from `memorymesh` [Use `open_nodes`]. Then, analyze the provided code for `AuthService` [Load using `read_file`] and propose changes to implement password reset, considering the existing dependencies."

### 5. Query Decomposition and Synthesis

*   **Strategy:** For complex queries requiring information scattered across a large context, break the query down. Ask the model to first find relevant pieces of information from different sections/files and then synthesize them into a final answer in a second step.
*   **Example:**
    1.  **Prompt 1:** "From the loaded context (including `api_spec.yaml`, `frontend/src/apiClient.js`, `backend/src/controller.py`), identify: (a) The API endpoint definition for user profile updates. (b) The frontend function calling this endpoint. (c) The backend controller function handling this endpoint."
    2.  **Prompt 2:** "Based on the information you just retrieved (endpoint definition, frontend call, backend handler), analyze the end-to-end flow for updating a user profile. Are the request/response payloads consistent across the stack?"

## Tool Integration

*   **`read_file`:** Primary tool for loading context. Use start/end lines judiciously for very large files.
*   **`memorymesh`:** Essential for persistent knowledge storage and retrieval across multiple interactions or tasks.
*   **`search_files` / `mcp-server-everything-search`:** Helps identify *what* to load into context initially.
*   **`list_code_definition_names`:** Useful for creating high-level summaries or indexes without loading full file content.

Effective knowledge management within large context windows transforms them from simple information dumps into powerful analytical environments. Structuring, summarizing, persisting, and targeted retrieval are key to unlocking their full potential.