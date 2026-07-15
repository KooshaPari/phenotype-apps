# MCP Hub-and-Spoke WebSocket Architecture

This document describes the hub-and-spoke WebSocket architecture implemented in the MCP server. This architecture centralizes WebSocket communication through a single server while keeping non-WebSocket communication decentralized.

## Overview

In the hub-and-spoke model:

- A single **Central WebSocket Server** acts as the hub
- Multiple **MCP Agents** connect to the central server as clients
- WebSocket communication is routed through the central server
- Non-WebSocket communication (stdio, SSE) remains direct and decentralized

This approach solves port conflict issues when running multiple MCP instances on the same machine and provides a more efficient communication model.

## Components

### Central WebSocket Server

The central server (`centralServer.js`) is responsible for:

- Accepting connections from MCP agents
- Routing messages between agents
- Tracking connected agents
- Sending heartbeats to detect disconnections
- Providing a stable WebSocket endpoint

### MCP Agent Clients

MCP agents (`index.js` with `--mode=client`) connect to the central server and:

- Automatically identify themselves with unique IDs and types
- Send and receive messages through the central server
- Maintain connection with reconnection logic
- Process heartbeats to keep connections alive

## Configuration

### Central Server Configuration

The central server can be configured with:

- `--port=PORT`: The port to run the WebSocket server on (default: 8080)
- `--heartbeat=SECONDS`: Heartbeat interval in seconds (default: 30)

Environment variables:
- `MCP_PORT`: Port to run the server on
- `MCP_HEARTBEAT_INTERVAL`: Heartbeat interval in seconds

### MCP Agent Configuration

MCP agents can be configured with:

- `--mode=client`: Run in client mode, connecting to the central server
- `--ws-server=URL`: WebSocket server URL (e.g., ws://localhost:8080)
- `--auto-identify=true|false`: Enable automatic agent identification

Environment variables:
- `MCP_MODE`: Set to "client" for client mode
- `MCP_SERVER_URL`: WebSocket server URL
- `MCP_AUTO_IDENTIFY`: Set to "true" to enable auto-identification
- `AGENT_ID`: Explicitly set agent ID (optional)
- `AGENT_TYPE`: Explicitly set agent type (optional)

## Automatic Agent Identification

When `--auto-identify=true` is set, the MCP agent will:

1. Detect the environment (Claude, GPT, Gemini, etc.)
2. Generate a unique ID based on agent type, hostname, and timestamp
3. Register with the central server using this ID and type

This eliminates the need to manually configure each agent with a unique ID.

## Usage

### Starting the Central Server

```bash
# Start with default settings
./scripts/start-central-server.sh

# Start with custom port and heartbeat interval
./scripts/start-central-server.sh --port=9000 --heartbeat=60
```

### Starting an MCP Agent in Client Mode

```bash
# Start with default settings (connects to ws://localhost:8080)
./scripts/start-mcp-client.sh

# Start with custom server URL
./scripts/start-mcp-client.sh --server=ws://example.com:9000

# Start with explicit agent ID and type
./scripts/start-mcp-client.sh --agent-id=claude_agent1 --agent-type=claude
```

## Benefits

- **Eliminates Port Conflicts**: Only one WebSocket server runs on a known port
- **Simplified Configuration**: Agents just need to know the central server address
- **Automatic Identification**: No need to manually configure agent IDs
- **Improved Reliability**: Better reconnection handling and heartbeat mechanism
- **Centralized Management**: Single point for monitoring and managing connections
- **Reduced Resource Usage**: Fewer WebSocket servers running simultaneously

## Example Setup

For a typical setup with multiple LLM agents:

1. Start the central server:
   ```bash
   ./scripts/start-central-server.sh
   ```

2. Start Claude agent:
   ```bash
   AGENT_TYPE=claude ./scripts/start-mcp-client.sh
   ```

3. Start Gemini agent:
   ```bash
   AGENT_TYPE=gemini ./scripts/start-mcp-client.sh
   ```

4. Start GPT agent:
   ```bash
   AGENT_TYPE=gpt ./scripts/start-mcp-client.sh
   ```

All agents will connect to the central server, automatically identify themselves, and be able to communicate with each other through the central hub.
