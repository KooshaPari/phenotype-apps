# Cross-Agent MCP Server

Model Context Protocol (MCP) server implementation that enables communication between different AI models including Claude, DeepSeek, OpenAI, and Gemini instances.

## Features

- **Full MCP protocol support**
- **Cross-agent communication**
- **Multi-model collaboration**
- **Real-time message streaming**
- **Channel-based messaging**
- **Thread support**
- **TypeScript implementation**

## Quick Start

### Installation

```bash
npm install -g mcp-cross-agent
```

### Configure Claude Desktop

Locate your config file:
- Mac: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Windows: `%APPDATA%\Claude\claude_desktop_config.json`
- Linux: `~/.config/Claude/claude_desktop_config.json`

Add Cross-Agent MCP configuration:

```json
{
  "mcpServers": {
    "cross-agent": {
      "command": "npx",
      "args": ["-y", "mcp-cross-agent"],
      "env": {
        "AGENT_ID": "claude-desktop-1",
        "AGENT_TYPE": "claude"
      }
    }
  }
}
```

### Configure Roocode CLI

For command-line instances, add to your configuration:

```json
{
  "mcpServers": {
    "cross-agent": {
      "command": "npx",
      "args": ["-y", "mcp-cross-agent"],
      "env": {
        "AGENT_ID": "roocode-cli-1",
        "AGENT_TYPE": "roocode"
      }
    }
  }
}
```

### Restart Claude Desktop or Roocode CLI

## Documentation

- [Setup Guide](docs/setup-guide.md) - Detailed setup instructions
- [Usage Examples](docs/usage-examples.md) - Usage examples and advanced configuration
- [Implementation Notes](docs/implementation-notes.md) - Technical implementation details
- [Development Guide](docs/development-guide.md) - Guide for developers
- [Troubleshooting Guide](docs/troubleshooting-guide.md) - Common issues and solutions

## Local Development

1. Clone the repository
2. Install dependencies: `npm install`
3. Build the project: `npm run build`
4. Run the server: `npm start`

## Available Tools

The Cross-Agent MCP Server provides the following tools:

- `send_message` - Send a message to another agent
- `get_messages` - Get messages, optionally filtered by thread
- `create_thread` - Create a new thread
- `join_thread` - Join an existing thread
- `broadcast_message` - Broadcast a message to multiple agents
- `decompose_task` - Decompose a complex task for multi-model collaboration
- `assign_subtasks` - Assign subtasks to specific models
- `merge_results` - Merge results from different models
- `compare_models` - Compare capabilities of different models
- `create_collaboration_session` - Create a collaboration session

## Available Resources

- `agents` - Information about available agents
- `agent` - Details about a specific agent
- `agent_capabilities` - Capabilities of a specific agent
- `channels` - Available communication channels
- `threads` - Threads in a channel

## Available Prompts

- `collaboration_initiation` - Prompt for initiating collaboration between different AI models
- `model_strengths_utilization` - Prompt for leveraging the specific strengths of different AI models
- `knowledge_transfer` - Prompt for transferring knowledge between different AI models
- `collaborative_problem_solving` - Structured workflow for collaborative problem solving
- `model_conflict_resolution` - Prompt for resolving conflicts between different AI models

## License

ISC
