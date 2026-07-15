# JavaScript/TypeScript Development Guide

## Core Language Features
- Modern JavaScript (ES2022+)
- TypeScript type system
- Asynchronous programming (Promises, async/await)
- Modules and imports
- Error handling

## TypeScript Type Definitions for MCP

```typescript
// Agent types
interface Agent {
  id: string;
  name: string;
  role: string;
  model_type: string;
  model_version: string;
  capabilities: string[];
  status: 'available' | 'working' | 'idle' | 'offline' | 'error';
  current_task?: string;
  registered_at: Date;
  last_heartbeat: Date;
}

// Message types
interface Message {
  id: string;
  sender_id: string;
  recipient_id?: string;
  recipient_ids?: string[];
  content: string;
  thread_id?: string;
  urgent: boolean;
  timestamp: Date;
}

// Workflow types
interface Workflow {
  id: string;
  name: string;
  description: string;
  created_at: Date;
  creator_id: string;
  steps: WorkflowStep[];
  status: 'draft' | 'active' | 'completed' | 'failed';
  updated_at: Date;
}

interface WorkflowStep {
  step_id: string;
  description: string;
  assigned_role?: string;
  assigned_agent_id?: string;
  depends_on: string[];
  tool_to_call?: string;
  tool_arguments?: any;
  status: 'pending' | 'in_progress' | 'completed' | 'failed';
  result?: any;
  error?: string;
}
```

## API Client Implementation

```typescript
// MCP API client
class MCPClient {
  private baseUrl: string;
  private apiKey: string;
  
  constructor(baseUrl: string, apiKey: string) {
    this.baseUrl = baseUrl;
    this.apiKey = apiKey;
  }
  
  async registerAgent(agent: Omit<Agent, 'id' | 'registered_at' | 'last_heartbeat'>): Promise<Agent> {
    const response = await fetch(`${this.baseUrl}/agents/register`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${this.apiKey}`
      },
      body: JSON.stringify(agent)
    });
    
    if (!response.ok) {
      const error = await response.json();
      throw new Error(`Failed to register agent: ${error.message}`);
    }
    
    return response.json();
  }
  
  async sendMessage(message: Omit<Message, 'id' | 'timestamp'>): Promise<Message> {
    const response = await fetch(`${this.baseUrl}/messages/send`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${this.apiKey}`
      },
      body: JSON.stringify(message)
    });
    
    if (!response.ok) {
      const error = await response.json();
      throw new Error(`Failed to send message: ${error.message}`);
    }
    
    return response.json();
  }
  
  // Additional API methods...
}
```

## Testing Framework

```typescript
// Jest test example
import { MCPClient } from './mcp-client';

// Mock fetch
global.fetch = jest.fn();

describe('MCPClient', () => {
  let client: MCPClient;
  
  beforeEach(() => {
    client = new MCPClient('https://api.mcp.example.com', 'test-api-key');
    (global.fetch as jest.Mock).mockClear();
  });
  
  describe('registerAgent', () => {
    it('should register an agent successfully', async () => {
      // Mock implementation
      (global.fetch as jest.Mock).mockResolvedValueOnce({
        ok: true,
        json: async () => ({
          id: 'agent-123',
          name: 'Test Agent',
          role: 'developer',
          model_type: 'gemini',
          model_version: '2.5-pro',
          capabilities: ['coding', 'planning'],
          status: 'available',
          registered_at: new Date().toISOString(),
          last_heartbeat: new Date().toISOString()
        })
      });
      
      const result = await client.registerAgent({
        name: 'Test Agent',
        role: 'developer',
        model_type: 'gemini',
        model_version: '2.5-pro',
        capabilities: ['coding', 'planning'],
        status: 'available'
      });
      
      expect(result.id).toBe('agent-123');
      expect(result.role).toBe('developer');
      expect(global.fetch).toHaveBeenCalledTimes(1);
    });
  });
});
```

## Error Handling Patterns

```typescript
// Error handling with custom error classes
class MCPError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'MCPError';
  }
}

class AgentRegistrationError extends MCPError {
  constructor(message: string) {
    super(`Agent registration failed: ${message}`);
    this.name = 'AgentRegistrationError';
  }
}

// Error handling in async functions
async function safeRegisterAgent(client: MCPClient, agent: Agent): Promise<Agent | null> {
  try {
    return await client.registerAgent(agent);
  } catch (error) {
    console.error(`Failed to register agent ${agent.name}:`, error);
    return null;
  }
}

// Error handling with custom hooks (React)
function useAgentRegistration() {
  const [agent, setAgent] = useState<Agent | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<Error | null>(null);
  
  const registerAgent = async (agentData: Omit<Agent, 'id' | 'registered_at' | 'last_heartbeat'>) => {
    setIsLoading(true);
    setError(null);
    
    try {
      const client = new MCPClient('https://api.mcp.example.com', 'api-key');
      const registeredAgent = await client.registerAgent(agentData);
      setAgent(registeredAgent);
      return registeredAgent;
    } catch (error) {
      const mcpError = new AgentRegistrationError(error.message);
      setError(mcpError);
      throw mcpError;
    } finally {
      setIsLoading(false);
    }
  };
  
  return { agent, isLoading, error, registerAgent };
}
```