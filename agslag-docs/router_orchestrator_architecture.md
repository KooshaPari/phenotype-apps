# AGSLAG Router/Orchestrator Architecture

This document describes the technical architecture of the AGSLAG router/orchestrator, focusing on how it optimizes cost, performance, complexity, agent type selection, modal selection, and cloud/edge/provider usage.

## 1. System Overview

The AGSLAG platform employs a hybrid approach to routing and orchestration, combining a centralized `WorkflowEngine` for general workflows with a distributed `MultiAgentSweProcess` for software engineering tasks.

### 1.1. Key Components

- **WorkflowEngine:** A centralized engine for defining and executing general-purpose workflows.
- **MultiAgentSweProcess:** A distributed process manager for orchestrating multi-agent software engineering tasks.
- **Tool Executor:** Executes external tools and services.
- **Metrics Service:** Collects and analyzes metrics to predict cost and optimize performance.
- **AI Routes:** Manages access to AI models from different providers.

## 2. Architecture Diagram

```mermaid
graph LR
    subgraph WorkflowEngine
        A[Workflow Definition] --> B(WorkflowEngine);
        B --> C{Step Execution};
        C -- Task --> D[Tool Executor];
        C -- Decision --> E{Condition};
        C -- Sequential --> F[Sub-Steps];
        C -- Parallel --> G[Parallel Execution];
        D --> H[External Tool];
        E -- True --> I[True Step];
        E -- False --> J[False Step];
    end

    subgraph MultiAgentSweProcess
        K[Coordinator] --> L{Task Assignment};
        L --> M[Development Agent];
        M --> N[Code Changes];
        N --> O[Pull Request];
        O --> P[Review Agent];
        P -- Approved --> Q[Merge to Main];
        P -- Changes Requested --> M;
    end

    subgraph AI Model Selection
        R[WorkflowEngine/MultiAgentSweProcess] --> S{Model Selection Logic};
        S --> T[AI Routes];
        T --> U[Model Fetchers];
        U --> V[LLM Providers (OpenAI, Anthropic, etc.)];
    end

    subgraph Metrics and Optimization
        W[WorkflowEngine/MultiAgentSweProcess] --> X{Metrics Collection};
        X --> Y[Metrics Service];
        Y --> Z{Cost Prediction & Performance Analysis};
        Z --> S;
    end
```

## 3. Routing and Orchestration Process

### 3.1. Workflow Definition

- Workflows are defined using the `WorkflowDefinition` interface in `agslag/orchestration/workflow_engine.ts`.
- Each workflow consists of a series of steps, which can be tasks, decisions, sequential steps, or parallel steps.
- Tasks are executed using the `ToolExecutor`, which spawns a separate process for the tool and communicates with it via stdin/stdout.

### 3.2. Agent Selection

- The `agentRoutes.ts` file provides API endpoints for managing agents.
- The `GET /` endpoint allows filtering agents by `modelType`, `status`, `capabilities`, `role`, and `group`.
- This filtering mechanism is used to select the appropriate agent for a given task.

### 3.3. Model Selection

- The `aiRoutes.ts` file manages access to AI models from different providers.
- The `fetchOpenAIModels`, `fetchAnthropicModels`, etc. functions in `modelFetchers.ts` fetch the available models from each provider.
- The `predictCost` function in `MetricsService.ts` uses model pricing information to estimate the cost of using a particular model.
- The system can dynamically select the most cost-effective model for a given task based on its requirements and the available resources.

### 3.4. Cost and Performance Optimization

- The `MetricsService` collects usage metrics for different entities (agents, workflows, tasks, projects, chats).
- These metrics are used to:
    - Predict the cost of a prompt or task.
    - Identify performance bottlenecks.
    - Optimize resource allocation.
- The `predictCost` function estimates the token count and complexity of a prompt and uses this information to calculate the estimated cost.
- The system can use this information to select a more efficient model or to optimize the prompt to reduce its cost.

## 4. Key Optimization Strategies

### 4.1. Cost Optimization

- **Model Selection:** The system can dynamically select the most cost-effective model for a given task based on its requirements and the available resources.
- **Prompt Optimization:** The system can analyze the complexity of a prompt and suggest ways to reduce its token count and cost.
- **Caching:** The system can cache the results of previous requests to reduce the number of API calls.

### 4.2. Performance Optimization

- **Asynchronous Processing:** The system uses asynchronous processing for non-blocking operations.
- **Parallel Execution:** The system supports parallel execution of workflow steps.
- **Resource Allocation:** The system can dynamically allocate resources to different agents and tasks based on their needs.

### 4.3. Complexity Management

- **Modular Design:** The system is designed with a modular architecture, which makes it easier to understand and maintain.
- **Declarative Workflows:** The system uses a declarative workflow definition, which makes it easier to define and manage complex workflows.
- **Centralized Orchestration:** The `WorkflowEngine` provides a centralized point of control for managing workflows.

### 4.4. Agent Type and Modal Selection

- The system can select the appropriate agent type (UI-driven, CLI, headless) and modal (text, file, data) based on the task requirements and the available resources.
- The `agentRoutes.ts` file allows filtering agents by `modelType`, `status`, `capabilities`, `role`, and `group`.

### 4.5. Cloud/Edge/Provider Usage

- The system can dynamically select the appropriate cloud/edge/provider based on cost, performance, and availability.
- The `modelFetchers.ts` file fetches AI models from different providers, allowing the system to choose the best provider for a given task.

## 5. Conclusion

The AGSLAG platform employs a sophisticated routing and orchestration system that optimizes cost, performance, complexity, agent type selection, modal selection, and cloud/edge/provider usage. By combining a centralized `WorkflowEngine` with a distributed `MultiAgentSweProcess` and using metrics to drive optimization, the system enables effective collaboration between AI agents and supports the development of high-quality software.
