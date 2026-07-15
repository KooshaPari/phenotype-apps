# MCP Agentic Architecture: Overview

## Executive Summary

This document outlines the architecture for a fully automated Model Context Protocol (MCP) system supporting both local UI-automated clients and cloud-based programmatic implementations. The architecture follows a containerized approach with Git-based collaboration, focusing on code quality through comprehensive stage gating processes.

The design defines how various MCP agentic clients (Cline, Roo Code, Aider, and custom implementations) will be deployed in containers, how they will interact with repositories via team branches, and how a robust CI/CD pipeline will enforce quality standards through multiple stage gates.

---

## Architecture Overview

The proposed architecture consists of two primary environments:

1. **Local Environment (UI Automation)**
   - Leverages UI automation techniques to interact with VS Code extensions (Cline, Roo Code)
   - Includes terminal-based agents like Aider
   - Operates on developers' machines for testing and experimentation

2. **Cloud Environment (Programmatic API)**
   - Uses headless MCP clients via programmatic APIs
   - Leverages remote MCP servers deployed on cloud platforms
   - Operates in containerized environments for scalability and reliability

All agents interact with a central GitHub repository following a branching strategy:
- Feature branches for individual tasks
- Team branches for coordinated work
- Main branch for production code

---

## Platform Components

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
  - Project management
  - Agent monitoring
  - Workflow visualization

---

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

---

## ASCII Architecture Diagram

```
┌───────────────────────────────────────────────────────────────────────────┐
│                       MCP Agentic Architecture                             │
└───────────────────┬───────────────────────────────────────┬───────────────┘
                    │                                       │
    ┌───────────────▼────────────────┐        ┌────────────▼────────────────┐
    │     Local Environment          │        │     Cloud Environment       │
    │     (UI Automation)            │        │     (Programmatic API)      │
    └───────────────┬────────────────┘        └────────────┬────────────────┘
                    │                                      │
┌───────────────────▼──────────────────┐    ┌─────────────▼─────────────────┐
│ ┌──────────────────────────────────┐ │    │ ┌─────────────────────────────┐
│ │          VS Code Extensions      │ │    │ │     Headless MCP Clients    │
│ │          (Cline, Roo Code)       │ │    │ │                             │
│ └──────────────────────────────────┘ │    │ └─────────────────────────────┘
│ ┌──────────────────────────────────┐ │    │ ┌─────────────────────────────┐
│ │        Terminal-based Agents     │ │    │ │     Docker Containers       │
│ │             (Aider)              │ │    │ │                             │
│ └──────────────────────────────────┘ │    │ └─────────────────────────────┘
└──────────────────┬───────────────────┘    └──────────────┬────────────────┘
                   │                                       │
```

---

## Key Components

### 1. Agent Types

- **VS Code Extensions**
  - Cline: General-purpose AI coding agent
  - Roo Code: Enhanced AI coding assistant with specialized roles
  
- **Terminal-based Agents**
  - Aider: CLI-based AI pair programming assistant
  
- **Custom MCP Clients**
  - Headless implementations for cloud deployment
  - Specialized agents for specific roles (QA, DevOps, etc.)

### 2. Orchestration Layer

- **Team Coordinator**: Manages team-level activities and branch coordination
- **Task Assignment**: Distributes tasks to appropriate agents based on capabilities
- **Workflow Engine**: Executes defined workflows across multiple agents

### 3. Git Infrastructure

- **Repository Structure**: Central repository with feature and team branches
- **PR Management**: Automated pull request creation and review
- **Branch Policies**: Enforced quality standards for different branch types

### 4. Stage Gates

- **Agent Setup Gate**: Validates agent configuration and API access
- **Local Validation Gate**: Tests code locally before committing
- **Feature Branch Gate**: Quality checks for feature branch PRs
- **Team Branch Gate**: Integration checks for team branches
- **Integration Gate**: System-level integration testing
- **Release Gate**: Final approval for production release

---

## Integration Points

The architecture provides integration with:

1. **VS Code**: Through extensions and UI automation
2. **Git**: Via direct Git operations and GitHub API
3. **CI/CD Systems**: Through GitHub Actions and webhooks
4. **MCP Servers**: Using standard MCP protocols
5. **Monitoring Systems**: With Prometheus metrics integration

---

## Technical Deep Dive

See [System Architecture](./system_architecture.md) for detailed technical layers and components.

---

## Future Enhancements

See [Enhanced Architecture Plan](./enhanced_architecture.md) for upcoming improvements and roadmap.

---

## Additional Diagrams

- [MCP Architecture Diagram](../mcp_architecture_diagram.md)
- [MCP Workflow Diagram](../mcp_workflow_diagram.md)
- [Container Architecture Diagram](../container_architecture_diagram.md)
