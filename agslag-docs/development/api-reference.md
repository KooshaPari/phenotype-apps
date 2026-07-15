# MCP Platform API Reference

This document provides a reference for the APIs used in the MCP Platform.

## MCP Server API

### WebSocket API

The MCP Server provides a WebSocket API for real-time communication.

#### Connection

```
ws://localhost:8081
```

#### Events

| Event | Description | Payload |
|-------|-------------|---------|
| `connect` | Emitted when a client connects | `{ agentId: string }` |
| `disconnect` | Emitted when a client disconnects | `{ agentId: string }` |
| `message` | Emitted when a message is received | `{ from: string, to: string, content: string }` |
| `tool-call` | Emitted when a tool is called | `{ agentId: string, toolName: string, args: any }` |
| `tool-result` | Emitted when a tool call completes | `{ agentId: string, toolCallId: string, result: string }` |

#### Commands

| Command | Description | Payload |
|---------|-------------|---------|
| `register` | Register an agent | `{ id: string, type: string, capabilities: string[] }` |
| `subscribe` | Subscribe to events for an agent | `{ agentId: string }` |
| `unsubscribe` | Unsubscribe from events for an agent | `{ agentId: string }` |
| `send-message` | Send a message to an agent | `{ to: string, content: string }` |
| `call-tool` | Call a tool | `{ toolName: string, args: any }` |

### REST API

The MCP Server also provides a REST API for management operations.

#### Base URL

```
http://localhost:8081
```

#### Endpoints

| Endpoint | Method | Description | Request | Response |
|----------|--------|-------------|---------|----------|
| `/health` | GET | Check server health | None | `{ status: "ok" }` |
| `/agents` | GET | List all agents | None | `Agent[]` |
| `/agents/:id` | GET | Get agent details | None | `Agent` |
| `/agents/:id` | DELETE | Delete an agent | None | `{ success: true }` |
| `/tools` | GET | List available tools | None | `Tool[]` |
| `/tools/:name` | GET | Get tool details | None | `Tool` |

## Jarvis SWE Agent API

### REST API

The Jarvis SWE Agent provides a REST API for agent operations.

#### Base URL

```
http://localhost:3100
```

#### Endpoints

| Endpoint | Method | Description | Request | Response |
|----------|--------|-------------|---------|----------|
| `/agents` | GET | List all agents | None | `Agent[]` |
| `/agents` | POST | Create a new agent | `{ provider: string, model: string, ... }` | `Agent` |
| `/agents/:id` | GET | Get agent details | None | `Agent` |
| `/agents/:id` | DELETE | Delete an agent | None | `{ success: true }` |
| `/agents/:id/messages` | POST | Send a message to an agent | `{ message: string }` | `{ content: string, toolCalls?: ToolCall[] }` |
| `/agents/:id/tool-calls/:toolCallId` | POST | Handle a tool call result | `{ result: string }` | `{ content: string, toolCalls?: ToolCall[] }` |

### WebSocket API

The Jarvis SWE Agent also provides a WebSocket API for real-time updates.

#### Connection

```
ws://localhost:3100
```

#### Events

| Event | Description | Payload |
|-------|-------------|---------|
| `agent-event` | Emitted for any agent event | `{ type: string, agentId: string, data: any, timestamp: Date }` |

#### Commands

| Command | Description | Payload |
|---------|-------------|---------|
| `subscribe` | Subscribe to events for an agent | `agentId` |
| `unsubscribe` | Unsubscribe from events for an agent | `agentId` |

## Frontend API

The Frontend Dashboard communicates with both the MCP Server and the Jarvis SWE Agent.

### MCP API Client

```typescript
class MCPApiClient {
  constructor(baseUrl: string);
  
  // Agents
  async listAgents(): Promise<Agent[]>;
  async getAgent(id: string): Promise<Agent>;
  async deleteAgent(id: string): Promise<void>;
  
  // Threads
  async listThreads(): Promise<Thread[]>;
  async getThread(id: string): Promise<Thread>;
  async createThread(data: ThreadData): Promise<Thread>;
  async deleteThread(id: string): Promise<void>;
  
  // Projects
  async listProjects(): Promise<Project[]>;
  async getProject(id: string): Promise<Project>;
  async createProject(data: ProjectData): Promise<Project>;
  async deleteProject(id: string): Promise<void>;
  
  // Workflows
  async listWorkflows(): Promise<Workflow[]>;
  async getWorkflow(id: string): Promise<Workflow>;
  async createWorkflow(data: WorkflowData): Promise<Workflow>;
  async deleteWorkflow(id: string): Promise<void>;
}
```

### Jarvis API Client

```typescript
class JarvisApiClient {
  constructor(baseUrl: string);
  
  // Connection
  async connect(): Promise<void>;
  disconnect(): void;
  
  // Event handling
  addEventListener(listener: (event: AgentEvent) => void): void;
  removeEventListener(listener: (event: AgentEvent) => void): void;
  
  // Agent subscription
  subscribeToAgent(agentId: string): void;
  unsubscribeFromAgent(agentId: string): void;
  
  // Agent operations
  async createAgent(options: CreateAgentOptions): Promise<AgentInstance>;
  async getAgent(agentId: string): Promise<AgentInstance>;
  async listAgents(): Promise<AgentInstance[]>;
  async deleteAgent(agentId: string): Promise<void>;
  
  // Messaging
  async sendMessage(agentId: string, message: string): Promise<AgentResponse>;
  async handleToolCall(agentId: string, toolCallId: string, result: string): Promise<AgentResponse>;
}
```

## Data Types

### Agent

```typescript
interface Agent {
  id: string;
  type: string;
  capabilities: string[];
  status: "online" | "offline";
  lastSeen: string;
}
```

### AgentInstance

```typescript
interface AgentInstance {
  id: string;
  modelConfig: {
    provider: "openai" | "anthropic" | "google" | "openrouter";
    model: string;
    temperature?: number;
    maxTokens?: number;
  };
  systemPrompt: string;
  conversationHistory: Message[];
  createdAt: string;
  lastUsedAt: string;
  toolCalls?: ToolCall[];
}
```

### Message

```typescript
interface Message {
  role: "system" | "user" | "assistant" | "tool";
  content: string;
  name?: string;
  tool_call_id?: string;
}
```

### ToolCall

```typescript
interface ToolCall {
  id: string;
  type: string;
  function: {
    name: string;
    arguments: string;
  };
}
```

### AgentEvent

```typescript
interface AgentEvent {
  type: "message" | "toolCall" | "toolResult" | "thinking" | "error" | "agentCreated" | "agentDeleted" | "agentUpdated" | "status";
  agentId: string;
  data: any;
  timestamp: Date;
}
```

### Thread

```typescript
interface Thread {
  id: string;
  title: string;
  participants: string[];
  messages: Message[];
  createdAt: string;
  updatedAt: string;
}
```

### Project

```typescript
interface Project {
  id: string;
  name: string;
  description: string;
  members: string[];
  tasks: Task[];
  createdAt: string;
  updatedAt: string;
}
```

### Workflow

```typescript
interface Workflow {
  id: string;
  name: string;
  description: string;
  steps: WorkflowStep[];
  createdAt: string;
  updatedAt: string;
}
```

### WorkflowStep

```typescript
interface WorkflowStep {
  id: string;
  name: string;
  type: string;
  config: any;
  nextSteps: string[];
}
```

### Task

```typescript
interface Task {
  id: string;
  title: string;
  description: string;
  assignee: string;
  status: "todo" | "in_progress" | "done" | "blocked";
  priority: "low" | "medium" | "high";
  dueDate?: string;
  createdAt: string;
  updatedAt: string;
}
```
