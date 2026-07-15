# MCP Platform Setup Guide

This guide will help you set up the MCP Platform components for local development.

## Prerequisites

- Node.js 18+ and npm
- Git
- Redis (optional, for advanced features)
- MongoDB (optional, for persistent storage)

## 1. Clone the Repository

```bash
git clone https://github.com/kooshapari/agslag.git
cd agslag
```

## 2. Set Up MCP Server

```bash
# Install dependencies
npm install

# Build the project
npm run build

# Start the server
npm run start:server
```

The MCP Server will start on port 8081 by default.

## 3. Set Up Jarvis SWE Agent

```bash
# Navigate to the Jarvis SWE Agent directory
cd jarvis-swe-agent

# Install dependencies
npm install

# Build the project
npm run build

# Start the agent
npm run start
```

The Jarvis SWE Agent will start on port 3100 by default.

## 4. Set Up Frontend Dashboard

```bash
# Navigate to the frontend directory
cd old-frontend

# Install dependencies
npm install

# Build the project
npm run build

# Start the frontend
npm run start
```

The Frontend Dashboard will start on port 3000 by default.

## 5. Set Up MCP Dashboard Next (Optional)

```bash
# Navigate to the MCP Dashboard Next directory
cd mcp-dashboard-next

# Install dependencies
npm install

# Build the project
npm run build

# Start the dashboard
npm run start
```

The MCP Dashboard Next will start on port 3000 by default (ensure the old frontend is not running).

## Configuration

### Environment Variables

Create a `.env` file in each component's directory with the following variables:

#### MCP Server

```
PORT=8081
REDIS_URL=redis://localhost:6379 (optional)
MONGODB_URI=mongodb://localhost:27017/mcp (optional)
```

#### Jarvis SWE Agent

```
PORT=3100
OPENAI_API_KEY=your_openai_api_key
MCP_WS_SERVER=ws://localhost:8081
```

#### Frontend Dashboard

```
NEXT_PUBLIC_API_URL=http://localhost:3002
NEXT_PUBLIC_JARVIS_API_URL=http://localhost:3100
NEXT_PUBLIC_WS_URL=ws://localhost:8081
```

### Config Files

The frontend requires a `config.json` file in the `public` directory:

```json
{
  "apiUrl": "http://localhost:3002",
  "jarvisApiUrl": "http://localhost:3100",
  "wsUrl": "ws://localhost:8081"
}
```

## Troubleshooting

### WebSocket Connection Issues

If you encounter WebSocket connection issues:

1. Ensure the MCP Server is running
2. Check that the WebSocket URL is correctly configured
3. Verify that no other process is using the WebSocket port

### Missing Avatar Images

If avatar images are missing:

1. Create an `avatars` directory in the `public` directory
2. Add placeholder images: `agent-1.png`, `agent-2.png`, `research-agent.png`, `writing-agent.png`, `qa-agent.png`, `current-user.png`

### API Connection Errors

If you see API connection errors:

1. Ensure all backend services are running
2. Check the API URLs in the configuration
3. Verify that the services are accessible from the frontend

### Build Errors

If you encounter build errors:

1. Check for TypeScript errors
2. Ensure all dependencies are installed
3. Verify that the correct Node.js version is being used
