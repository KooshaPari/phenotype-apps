# Model Context Protocol (MCP) Installation Guide

This guide provides instructions for installing and using the Model Context Protocol (MCP) implementation for cross-agent communication between various AI models.

## Prerequisites

- Node.js 18 or later
- npm

## Installation Options

### Option 1: From the Zip File (Recommended)

1. Extract the zip file to a directory of your choice:
   ```bash
   unzip mcp_official.zip -d /path/to/destination
   ```

2. Navigate to the extracted directory:
   ```bash
   cd /path/to/destination/mcp_official
   ```

3. Install dependencies:
   ```bash
   npm install
   ```

4. Build the project:
   ```bash
   npm run build
   ```

### Option 2: From Source Code

1. Clone the repository (if available):
   ```bash
   git clone https://github.com/your-username/mcp-official.git
   cd mcp-official
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Build the project:
   ```bash
   npm run build
   ```

## Running the MCP Server

### Using stdio Transport (for Command-Line Integration)

```bash
npm start
```

### Using SSE Transport (for Web-Based Integration)

```bash
npm start sse
```

## Client Integration

### Claude Desktop Integration

1. Run the Claude Desktop client:
   ```bash
   node examples/claude-desktop-client.js claude-desktop-1 stdio ./build/index.js
   ```

   Parameters:
   - `claude-desktop-1`: Client ID (can be any unique identifier)
   - `stdio`: Transport type (stdio or sse)
   - `./build/index.js`: Path to the server executable (for stdio transport)

2. For SSE transport:
   ```bash
   node examples/claude-desktop-client.js claude-desktop-1 sse http://localhost:3000
   ```

### Roocode CLI Integration

1. Run the Roocode CLI client:
   ```bash
   node examples/roocode-cli-client.js roocode-cli-1 stdio ./build/index.js
   ```

   Parameters:
   - `roocode-cli-1`: Client ID (can be any unique identifier)
   - `stdio`: Transport type (stdio or sse)
   - `./build/index.js`: Path to the server executable (for stdio transport)

2. For SSE transport:
   ```bash
   node examples/roocode-cli-client.js roocode-cli-1 sse http://localhost:3000
   ```

## Using the Clients

Both clients support a variety of commands for cross-agent communication:

### Basic Commands

- `help`: Show available commands
- `tools`: List available tools
- `resources`: List available resources
- `prompts`: List available prompts
- `exit`: Exit the client

### Communication Commands

- `send <recipient> <message>`: Send a message to another agent
- `broadcast <recipient1,recipient2,...> <message>`: Broadcast a message to multiple agents
- `get-messages [thread-id]`: Get messages, optionally filtered by thread
- `create-thread <title> <participant1,participant2,...>`: Create a new thread
- `join-thread <thread-id>`: Join an existing thread

### Agent and Channel Commands

- `agents [model-type]`: List available agents, optionally filtered by model type
- `agent <agent-id>`: Get details about a specific agent
- `channels`: List available channels
- `threads <channel-id>`: List threads in a channel

### Collaboration Commands

- `decompose <task>`: Decompose a complex task for multi-model collaboration
- `collaborate <title> <description> <participant1,participant2,...>`: Create a collaboration session
- `compare <agent1,agent2,...>`: Compare capabilities of different agents

## Cross-Agent Communication Example

1. Start the MCP server:
   ```bash
   npm start
   ```

2. In a new terminal, start the Claude Desktop client:
   ```bash
   node examples/claude-desktop-client.js claude-1 stdio ./build/index.js
   ```

3. In another terminal, start the Roocode CLI client:
   ```bash
   node examples/roocode-cli-client.js roocode-1 stdio ./build/index.js
   ```

4. In the Claude Desktop client, send a message to the Roocode client:
   ```
   send roocode-1 Hello from Claude Desktop!
   ```

5. In the Roocode CLI client, check for messages:
   ```
   get-messages
   ```

6. Create a collaboration session:
   ```
   collaborate "Code Review" "Review and improve the implementation" claude-1,roocode-1
   ```

## Troubleshooting

### Common Issues

1. **Connection errors**: Make sure the server is running before starting the clients.

2. **Module not found errors**: Ensure you've built the project with `npm run build`.

3. **Transport errors**: Check that you're using the correct transport type and parameters.

4. **Permission issues**: Make sure the build/index.js file is executable (chmod +x build/index.js).

### Getting Help

If you encounter any issues not covered here, please:

1. Check the console output for error messages
2. Refer to the README.md file for more information
3. Check the official MCP documentation at https://modelcontextprotocol.io/
