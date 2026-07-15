# MCP Agentic Architecture

## Overview

This document describes the high-level architecture for our MCP (Model Context Protocol) Agentic system, which combines local UI-automated clients and cloud-based programmatic implementations in a containerized approach with Git-based collaboration.

## Architecture Diagram

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
                   │                                       │
       ┌───────────▼───────────▼───────────▼──────────────▼────────┐
       │                                                            │
       │              Git-based Collaboration                       │
       │                                                            │
       │   ┌───────────────┐  ┌───────────────┐  ┌───────────────┐  │
       │   │ Feature       │  │ Team          │  │ Main          │  │
       │   │ Branches      │  │ Branches      │  │ Branch        │  │
       │   └───────────────┘  └───────────────┘  └───────────────┘  │
       │                                                            │
       └────────────────────────────────────────────────────────────┘
```

## Component Description

### Local Environment (UI Automation)

The Local Environment supports UI-based interaction with MCP clients:

1. **VS Code Extensions**
   - Cline
   - Roo Code (formerly Roo Cline)
   - Automated through UI automation frameworks (Playwright/Puppeteer)

2. **Terminal-based Agents**
   - Aider (CLI-focused agent for pair programming)
   - Automated through direct script execution

### Cloud Environment (Programmatic API)

The Cloud Environment provides headless, containerized MCP clients:

1. **Headless MCP Clients**
   - Programmatic APIs without UI dependencies
   - Custom implementations based on extracted functionality from UI clients
   - Support for various transport mechanisms (stdio, SSE, WebSocket)

2. **Docker Containers**
   - One container per agent
   - Role-specific container configurations
   - Scalable deployment for multi-agent systems

### Git-based Collaboration

The collaboration layer uses Git for version control and coordination:

1. **Feature Branches**
   - Individual task branches
   - Created and managed by specific agents

2. **Team Branches**
   - Integration branches for team-level coordination
   - Managed by team orchestration services

3. **Main Branch**
   - Production-ready code
   - Managed through comprehensive stage gates

## Communication Flows

Agents communicate and collaborate through several mechanisms:

1. **Direct Agent Communication**
   - MCP-based messaging
   - Message queues for asynchronous communication

2. **Git-based Communication**
   - Pull requests and code reviews
   - Commit messages and metadata

3. **Orchestration Services**
   - Task assignment and coordination
   - Progress tracking and reporting

## Integration Points

The architecture integrates with several external systems:

1. **GitHub**
   - Repository hosting
   - Pull request management
   - GitHub Actions for CI/CD

2. **LLM Providers**
   - Anthropic Claude
   - OpenAI GPT
   - Google Gemini
   - Azure OpenAI

3. **Development Tools**
   - VS Code
   - Terminal environments
   - Container orchestration platforms
