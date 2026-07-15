# AGSLAG Custom Tools: Knowledge Proxy

This document describes the MCP tools provided within AGSLAG that act as proxies for interacting with dedicated knowledge management MCP servers. These tools help agents formulate the correct parameters for calls to external knowledge servers like `server-memory` (graph-based) or `graphlit-mcp-server` (document/RAG-based). They are typically implemented in `src/tools/knowledge_proxy.ts`.

**Note:** These tools **do not** directly interact with the knowledge servers. They return the necessary `server_name`, `tool_name`, and `arguments` for the calling agent (or orchestrator) to then execute using the standard `use_mcp_tool` mechanism.

## Tools Overview

*   **`prepare_add_kg_entity`**: Prepares parameters to add an entity to `server-memory`.
*   **`prepare_add_kg_relation`**: Prepares parameters to add a relation to `server-memory`.
*   **`prepare_query_kg`**: Prepares parameters to query `server-memory`.
*   **`prepare_ingest_doc_to_kb`**: Prepares parameters to ingest text into `graphlit-mcp-server`.
*   **`prepare_retrieve_from_doc_kb`**: Prepares parameters to retrieve information from `graphlit-mcp-server`.

## Tool Details

### 1. `prepare_add_kg_entity`

Prepares the arguments needed to call the `create_entities` tool on the `server-memory` MCP server.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "entity": {
      "type": "object",
      "description": "The entity object to add.",
      "properties": {
        "name": { "type": "string", "description": "The name of the entity" },
        "entityType": { "type": "string", "description": "The type of the entity" },
        "observations": { "type": "array", "items": { "type": "string" }, "description": "An array of observation contents associated with the entity" }
      },
      "required": ["name", "entityType", "observations"]
    }
  },
  "required": ["entity"]
}
```

**Output:** An object containing:
*   `server_name`: "server-memory"
*   `tool_name`: "create_entities"
*   `arguments`: `{ "entities": [ <input_entity_object> ] }`

### 2. `prepare_add_kg_relation`

Prepares the arguments needed to call the `create_relations` tool on the `server-memory` MCP server.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "relation": {
      "type": "object",
      "description": "The relation object to add.",
      "properties": {
        "from": { "type": "string", "description": "The name of the entity where the relation starts" },
        "to": { "type": "string", "description": "The name of the entity where the relation ends" },
        "relationType": { "type": "string", "description": "The type of the relation" }
      },
      "required": ["from", "to", "relationType"]
    }
  },
  "required": ["relation"]
}
```

**Output:** An object containing:
*   `server_name`: "server-memory"
*   `tool_name`: "create_relations"
*   `arguments`: `{ "relations": [ <input_relation_object> ] }`

### 3. `prepare_query_kg`

Prepares the arguments needed to call the `search_nodes` tool on the `server-memory` MCP server.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "query": {
      "type": "string",
      "description": "The search query to match against entity names, types, and observation content"
    }
  },
  "required": ["query"]
}
```

**Output:** An object containing:
*   `server_name`: "server-memory"
*   `tool_name`: "search_nodes"
*   `arguments`: `{ "query": "<input_query>" }`

### 4. `prepare_ingest_doc_to_kb`

Prepares the arguments needed to call the `ingestText` tool on the `graphlit-mcp-server`.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "name": { "type": "string", "description": "Name for the content object." },
    "text": { "type": "string", "description": "Text content to ingest." },
    "textType": { "type": "string", "enum": ["HTML", "MARKDOWN", "PLAIN"], "default": "MARKDOWN", "description": "Optional: Text type (Plain, Markdown, Html). Defaults to Markdown." }
  },
  "required": ["name", "text"]
}
```

**Output:** An object containing:
*   `server_name`: "graphlit-mcp-server"
*   `tool_name`: "ingestText"
*   `arguments`: `{ "name": "...", "text": "...", "textType": "..." }` (matching input)

### 5. `prepare_retrieve_from_doc_kb`

Prepares the arguments needed to call the `retrieveSources` tool on the `graphlit-mcp-server`.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "prompt": { "type": "string", "description": "LLM user prompt for content retrieval (keywords/phrases)." },
    "inLast": { "type": "string", "description": "Optional: Recency filter (ISO 8601 duration)." },
    "contentType": { "type": "string", "enum": ["EMAIL", "EVENT", "FILE", "ISSUE", "MESSAGE", "PAGE", "POST", "TEXT"], "description": "Optional: Filter by content type." },
    "fileType": { "type": "string", "enum": ["ANIMATION", "AUDIO", "CODE", "DATA", "DOCUMENT", "DRAWING", "EMAIL", "GEOMETRY", "IMAGE", "MANIFEST", "PACKAGE", "POINT_CLOUD", "SHAPE", "UNKNOWN", "VIDEO"], "description": "Optional: Filter by file type." },
    "feeds": { "type": "array", "items": { "type": "string" }, "description": "Optional: Filter by specific feeds." },
    "collections": { "type": "array", "items": { "type": "string" }, "description": "Optional: Filter by specific collections." }
  },
  "required": ["prompt"]
}
```

**Output:** An object containing:
*   `server_name`: "graphlit-mcp-server"
*   `tool_name`: "retrieveSources"
*   `arguments`: `{ "prompt": "...", "inLast": "...", ... }` (matching input)
