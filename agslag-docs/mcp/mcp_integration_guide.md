# Integrating and Using MCP Servers

Model Context Protocol (MCP) servers extend the capabilities of AI assistants by providing specialized tools and access to various resources. This guide explains how to integrate and use MCP servers within your project.

## Overview

MCP servers act as bridges between your AI assistant and external services, tools, or data sources. They allow the assistant to perform actions like:

*   Searching the web (e.g., `Serper-search-mcp`, `tavily-mcp`, `openai-websearch-mcp`)
*   Interacting with development tools (e.g., `Figma-Context-MCP`, `ida-pro-mcp`, `mcp-selenium`)
*   Accessing knowledge bases and documentation (e.g., `DevDocs`, `atlas-docs-mcp`, `cosa-sai`)
*   Managing project tasks (e.g., `mcp-linear`)
*   Performing code analysis and search (e.g., `probe`)
*   Controlling browsers or mobile simulators (e.g., `browse-together-mcp`, `mobile-dev-mcp-server`)
*   And much more...

## Configuration

To use an MCP server, you typically need to configure your client (like Cline, Cursor, Roo Code) to connect to it. This often involves editing a configuration file (e.g., `cline_mcp_settings.json` in this project) to specify the server's connection details (like command to run, port, or URL).

**Example `cline_mcp_settings.json` entry:**

```json
{
  "servers": [
    {
      "name": "web-search",
      "description": "Provides web search capabilities via Tavily AI.",
      "type": "stdio", // or "sse" for remote servers
      "command": ["uvx", "tavily-mcp"], // Command to start the server
      "working_directory": "path/to/tavily-mcp/repo" // Optional: if needed
    },
    {
      "name": "figma-context",
      "description": "Provides Figma layout information.",
      "type": "stdio",
      "command": ["node", "path/to/Figma-Context-MCP/server.js"]
    }
    // ... other server configurations
  ]
}
```

**Note:** The exact configuration steps depend on the specific MCP server and the client application you are using. Always refer to the documentation of both the server and the client.

## Using MCP Tools and Resources

Once a server is connected, you can interact with it using two primary tools:

1.  **`use_mcp_tool`**: Executes a specific tool provided by the server.
2.  **`access_mcp_resource`**: Accesses a data resource exposed by the server.

**Example: Using a Tool (`use_mcp_tool`)**

```xml
<use_mcp_tool>
  <server_name>web-search</server_name>
  <tool_name>search</tool_name>
  <arguments>
    {
      "query": "Model Context Protocol specification",
      "max_results": 5
    }
  </arguments>
</use_mcp_tool>
```

**Example: Accessing a Resource (`access_mcp_resource`)**

```xml
<access_mcp_resource>
  <server_name>figma-context</server_name>
  <uri>figma://file_id/node_id</uri>
</access_mcp_resource>
```

**Important:** The available `tool_name`, `arguments`, and resource `uri` formats are specific to each MCP server. You **must** consult the documentation of the individual MCP server repository to understand its capabilities and how to use them correctly.

## Available MCP Servers (Based on Provided List)

Here's a brief overview of the MCP servers identified in the list. Please visit their respective repositories for detailed setup and usage instructions.

*   **ottomator-agents/crawl4AI-agent, unclecode/crawl4ai**: Web crawling/scraping.
*   **wolfyy970/docs-fetch-mcp**: Fetching web content recursively.
*   **lastmile-ai/openai-agents-mcp**: Extension for OpenAI Agents SDK.
*   **andrewhopper/hivemind**: Knowledge management, memory, and constraints for AI coding.
*   **GLips/Figma-Context-MCP, sonnylazuardi/cursor-talk-to-figma-mcp**: Provides Figma layout information.
*   **M-Gonzalo/cosa-sai**: Retrieving documentation from a knowledge base.
*   **tiovikram/openai-websearch-mcp**: Web search via OpenAI.
*   **liuyoshio/mcp-compass**: MCP server discovery/recommendation.
*   **jsuarezruiz/mobile-dev-mcp-server**: Interact with mobile devices/simulators.
*   **mrexodia/user-feedback-mcp**: Human-in-the-loop workflow.
*   **ODAncona/code2prompt-mcp**: (Purpose needs verification from repo).
*   **NightTrek/Serper-search-mcp**: Google search via Serper.
*   **trafflux/pdf-reader-mcp**: Read PDFs.
*   **buger/probe**: Local semantic code search engine.
*   **CartographAI/atlas-docs-mcp**: Provide hosted documentation for libraries/frameworks.
*   **angiejones/mcp-selenium**: Selenium WebDriver integration.
*   **AgentDeskAI/browser-tools-mcp**: Monitor browser logs.
*   **mrexodia/ida-pro-mcp**: IDA Pro integration.
*   **tacticlaunch/mcp-linear**: Interact with Linear project management.
*   **evalstate/mcp-hfspace**: Use HuggingFace spaces.
*   **cognitivecomputations/dolphin-mcp**: (Likely related to Dolphin LLMs, check repo).
*   **GreatScottyMac/RooFlow**: Enhanced memory bank system.
*   **cyberagiinc/DevDocs**: UI-based Tech Documentation MCP server.
*   **mclenhard/catie-mcp**: (Purpose needs verification from repo).
*   **donphi/MCP-Server**: Access/search private documents, codebases, etc.
*   **canadaduane/browse-together-mcp**: Co-browse with AI via Playwright.
*   **tavily-ai/tavily-mcp**: Web search via Tavily AI.

*(Note: Some items in the original list were SDKs, tools, or issues, not MCP servers themselves, and have been omitted here.)*

## Further Steps

1.  **Identify Needs:** Determine which capabilities you need for your project.
2.  **Explore Repositories:** Visit the GitHub repositories of the relevant MCP servers.
3.  **Follow Setup Instructions:** Clone the repositories and follow their specific setup guides (installation, API keys, etc.).
4.  **Configure Client:** Update your `cline_mcp_settings.json` or equivalent client configuration.
5.  **Utilize Tools/Resources:** Use the `use_mcp_tool` and `access_mcp_resource` commands in your workflow, referencing the server's documentation for correct parameters and URIs.

By leveraging MCP servers, you can significantly enhance the context awareness and action capabilities of your AI assistant.