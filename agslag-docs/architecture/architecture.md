# MCP Platform Architecture

## Overview

The MCP (Model Context Protocol) Platform is a comprehensive system for multi-agent orchestration that mimics project development and full organizational structures. It consists of four main components:

1. **MCP Server (agslag)**: The core server that handles WebSocket communication and agent coordination
2. **Jarvis SWE Agent (jarvis-swe-agent)**: A specialized agent for software engineering tasks
3. **Frontend Dashboard (old-frontend)**: A Next.js-based UI for interacting with the platform
4. **MCP Dashboard Next (mcp-dashboard-next)**: A newer version of the dashboard with enhanced features

## Communication Flow

The platform uses a hub-and-spoke model for WebSocket communication:

```
                    ┌─────────────────┐
                    │                 │
                    │  MCP Server     │
                    │  (Port 8081)    │
                    │                 │
                    └─────────────────┘
                            ▲
                            │
                            │ WebSocket
                            │
                            ▼
┌─────────────┐     ┌─────────────────┐     ┌─────────────────┐
│             │     │                 │     │                 │
│ Frontend    │◄───►│  Jarvis SWE     │◄───►│  Other Agents   │
│ (Port 3000) │     │  (Port 3100)    │     │                 │
│             │     │                 │     │                 │
└─────────────┘     └─────────────────┘     └─────────────────┘
```

## Key Components

### MCP Server

- **Purpose**: Central coordination of agents and communication
- **Technologies**: TypeScript, WebSocket, Redis
- **Key Features**:
  - Agent registration and discovery
  - Message routing
  - Tool execution

### Jarvis SWE Agent

- **Purpose**: Specialized agent for software engineering tasks
- **Technologies**: TypeScript, OpenAI API, WebSocket
- **Key Features**:
  - Code generation and review
  - Terminal interaction
  - Browser automation

### Frontend Dashboard

- **Purpose**: User interface for interacting with the platform
- **Technologies**: Next.js, React, shadcn/ui
- **Key Features**:
  - Agent chat interface
  - Visualization of agent networks
  - Project management

### MCP Dashboard Next

- **Purpose**: Enhanced dashboard with advanced features
- **Technologies**: Next.js, React, Three.js
- **Key Features**:
  - 3D visualization of agent networks
  - Advanced analytics
  - Workflow builder

## Data Flow

1. User interacts with the frontend
2. Frontend sends requests to Jarvis SWE Agent
3. Jarvis communicates with the MCP Server
4. MCP Server routes messages to appropriate agents
5. Agents process requests and return results
6. Results flow back through the same path to the user

## Deployment Architecture

The system is designed to be deployed in various configurations:

1. **Local Development**: All components run on localhost with different ports
2. **Distributed**: Components can run on different machines
3. **Cloud**: Can be deployed to cloud providers with appropriate networking

## Security Considerations

- WebSocket connections should be secured with proper authentication
- API keys for external services (OpenAI, etc.) should be properly managed
- Role-based access control for different user types
