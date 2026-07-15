# Jarvis SWE Agent Multi-Agent Coordination - Technical Implementation (Part 1)

## 1. Multi-Agent System Overview

The Jarvis multi-agent coordination system enables collaboration between specialized agent instances to tackle complex software engineering tasks. This document details the technical implementation of the core components of this system.

## 2. Core Data Models

### 2.1 Agent

```typescript
interface Agent {
  id: string;                 // Unique agent identifier
  type: AgentType;            // Agent type (coordinator, planner, coder, etc.)
  status: AgentStatus;        // 'available', 'busy', 'offline'
  capabilities: string[];     // Agent capabilities
  roles: string[];            // Assigned role IDs
  teams: string[];            // Team memberships
  model: {                    // LLM model configuration
    provider: string;         // LLM provider
    model: string;            // Model name
    parameters: any;          // Model parameters
  };
  performance: {              // Performance metrics
    taskCompletion: number;   // Task completion rate
    responseTime: number;     // Average response time
    qualityScore: number;     // Quality assessment score
  };
  metadata: {                 // Agent metadata
    created: Date;            // Creation timestamp
    lastActive: Date;         // Last activity timestamp
    version: string;          // Agent version
  };
}
```

### 2.2 Role

```typescript
interface Role {
  id: string;                 // Unique role identifier
  name: string;               // Role name
  description: string;        // Role description
  capabilities: {             // Required capabilities
    required: string[];       // Must-have capabilities
    preferred: string[];      // Nice-to-have capabilities
  };
  responsibilities: string[]; // Role responsibilities
  authority: {                // Role authority
    decisions: string[];      // Decisions this role can make
    resources: string[];      // Resources this role can access
  };
  relationships: {            // Role relationships
    reports_to: string[];     // Roles this role reports to
    manages: string[];        // Roles this role manages
    collaborates_with: string[]; // Roles this role collaborates with
  };
}
```

### 2.3 Team

```typescript
interface Team {
  id: string;                 // Unique team identifier
  name: string;               // Team name
  objective: string;          // Team objective
  members: {                  // Team members
    agentId: string;          // Agent identifier
    roleId: string;           // Role within team
    joinedAt: Date;           // Join timestamp
  }[];
  workflows: string[];        // Associated workflow IDs
  status: TeamStatus;         // 'forming', 'active', 'disbanded'
  performance: {              // Team performance metrics
    objectiveCompletion: number; // Objective completion rate
    collaboration: number;    // Collaboration effectiveness
    efficiency: number;       // Resource efficiency
  };
  metadata: {                 // Team metadata
    created: Date;            // Creation timestamp
    lastActive: Date;         // Last activity timestamp
    type: string;             // Team type (fixed, dynamic, ad-hoc)
  };
}
```

### 2.4 Message

```typescript
interface Message {
  id: string;                 // Unique message identifier
  conversationId: string;     // Conversation thread identifier
  type: MessageType;          // Message type (directive, query, response, etc.)
  sender: {                   // Sender information
    agentId: string;          // Agent identifier
    role: string;             // Agent role
  };
  recipients: {               // Recipient information
    agentId: string;          // Agent identifier
    role: string;             // Agent role
  }[];
  content: {                  // Message content
    text: string;             // Main message text
    attachments?: any[];      // Additional data
    metadata?: any;           // Message metadata
  };
  context: {                  // Message context
    taskId?: string;          // Related task
    workflowId?: string;      // Related workflow
    references?: string[];    // Referenced messages
  };
  timestamp: number;          // Message timestamp
  expiresAt?: number;         // Expiration timestamp
}
```

### 2.5 Task

```typescript
interface Task {
  id: string;                 // Unique task identifier
  title: string;              // Task title
  description: string;        // Task description
  type: TaskType;             // Task type
  priority: Priority;         // Task priority
  status: TaskStatus;         // 'pending', 'assigned', 'in_progress', 'completed', 'failed'
  assignee?: string;          // Assigned agent ID
  dependencies: string[];     // Dependent task IDs
  deadline?: Date;            // Task deadline
  estimatedEffort: number;    // Estimated effort (hours)
  actualEffort?: number;      // Actual effort (hours)
  progress: number;           // Progress percentage (0-100)
  result?: any;               // Task result
  metadata: {                 // Task metadata
    created: Date;            // Creation timestamp
    lastUpdated: Date;        // Last update timestamp
    creator: string;          // Creator agent ID
    tags: string[];           // Task tags
  };
}
```

### 2.6 Workflow

```typescript
interface Workflow {
  id: string;                 // Unique workflow identifier
  name: string;               // Workflow name
  description: string;        // Workflow description
  template: string;           // Workflow template ID
  team: string;               // Team ID executing the workflow
  steps: {                    // Workflow steps
    id: string;               // Step identifier
    name: string;             // Step name
    description: string;      // Step description
    assignee: string;         // Agent ID assigned to step
    dependencies: string[];   // Step dependencies
    status: StepStatus;       // 'pending', 'in-progress', 'completed', 'failed'
    result: any;              // Step result
    startTime?: Date;         // Start timestamp
    endTime?: Date;           // End timestamp
  }[];
  status: WorkflowStatus;     // 'created', 'running', 'paused', 'completed', 'failed'
  progress: number;           // Progress percentage
  startTime?: Date;           // Start timestamp
  endTime?: Date;             // End timestamp
  result?: any;               // Workflow result
}
```

## 3. Agent Registry Implementation

### 3.1 Agent Registry Service

```typescript
class AgentRegistry {
  private db: MongoDB.Db;
  private collection: MongoDB.Collection;
  private redis: Redis.Redis;
  
  constructor(config: AgentRegistryConfig) {
    const client = new MongoDB.MongoClient(config.mongoUri);
    this.db = client.db(config.database || 'jarvis');
    this.collection = this.db.collection(config.collection || 'agents');
    this.redis = new Redis(config.redisUri);
    
    // Create indexes
    this.collection.createIndex({ id: 1 }, { unique: true });
    this.collection.createIndex({ type: 1 });
    this.collection.createIndex({ status: 1 });
    this.collection.createIndex({ capabilities: 1 });
  }
  
  async registerAgent(agent: Partial<Agent>): Promise<string> {
    const id = agent.id || uuidv4();
    
    const newAgent: Agent = {
      id,
      type: agent.type || 'general',
      status: agent.status || 'available',
      capabilities: agent.capabilities || [],
      roles: agent.roles || [],
      teams: agent.teams || [],
      model: agent.model || {
        provider: 'openai',
        model: 'gpt-4',
        parameters: {}
      },
      performance: agent.performance || {
        taskCompletion: 0,
        responseTime: 0,
        qualityScore: 0
      },
      metadata: {
        created: new Date(),
        lastActive: new Date(),
        version: agent.metadata?.version || '1.0.0'
      }
    };
    
    // Store in MongoDB
    await this.collection.insertOne(newAgent);
    
    // Store in Redis for quick lookups
    await this.redis.set(`agent:${id}`, JSON.stringify(newAgent));
    
    // Add to capability indices
    for (const capability of newAgent.capabilities) {
      await this.redis.sadd(`capability:${capability}`, id);
    }
    
    // Add to type index
    await this.redis.sadd(`type:${newAgent.type}`, id);
    
    // Add to status index
    await this.redis.sadd(`status:${newAgent.status}`, id);
    
    return id;
  }
  
  async getAgent(id: string): Promise<Agent | null> {
    // Try Redis first for performance
    const cachedAgent = await this.redis.get(`agent:${id}`);
    
    if (cachedAgent) {
      return JSON.parse(cachedAgent);
    }
    
    // Fall back to MongoDB
    const agent = await this.collection.findOne({ id });
    
    if (!agent) return null;
    
    // Update cache
    await this.redis.set(`agent:${id}`, JSON.stringify(agent));
    
    return agent as Agent;
  }
  
  async updateAgent(id: string, updates: Partial<Agent>): Promise<Agent | null> {
    const updateDoc: any = { $set: {} };
    
    // Build update document
    if (updates.status !== undefined) {
      updateDoc.$set.status = updates.status;
      
      // Update status index in Redis
      const agent = await this.getAgent(id);
      if (agent && agent.status !== updates.status) {
        await this.redis.srem(`status:${agent.status}`, id);
        await this.redis.sadd(`status:${updates.status}`, id);
      }
    }
    
    if (updates.capabilities !== undefined) {
      updateDoc.$set.capabilities = updates.capabilities;
      
      // Update capability indices in Redis
      const agent = await this.getAgent(id);
      if (agent) {
        // Remove from old capability indices
        for (const capability of agent.capabilities) {
          await this.redis.srem(`capability:${capability}`, id);
        }
        
        // Add to new capability indices
        for (const capability of updates.capabilities) {
          await this.redis.sadd(`capability:${capability}`, id);
        }
      }
    }
    
    if (updates.roles !== undefined) {
      updateDoc.$set.roles = updates.roles;
    }
    
    if (updates.teams !== undefined) {
      updateDoc.$set.teams = updates.teams;
    }
    
    if (updates.model !== undefined) {
      updateDoc.$set.model = updates.model;
    }
    
    if (updates.performance !== undefined) {
      updateDoc.$set.performance = updates.performance;
    }
    
    // Always update lastActive
    updateDoc.$set['metadata.lastActive'] = new Date();
    
    // Update in MongoDB
    const result = await this.collection.updateOne({ id }, updateDoc);
    
    if (result.modifiedCount === 0) return null;
    
    // Get updated agent
    const updatedAgent = await this.collection.findOne({ id });
    
    if (!updatedAgent) return null;
    
    // Update cache
    await this.redis.set(`agent:${id}`, JSON.stringify(updatedAgent));
    
    return updatedAgent as Agent;
  }
  
  async deleteAgent(id: string): Promise<boolean> {
    const agent = await this.getAgent(id);
    
    if (!agent) return false;
    
    // Remove from MongoDB
    const result = await this.collection.deleteOne({ id });
    
    if (result.deletedCount === 0) return false;
    
    // Remove from Redis
    await this.redis.del(`agent:${id}`);
    
    // Remove from indices
    for (const capability of agent.capabilities) {
      await this.redis.srem(`capability:${capability}`, id);
    }
    
    await this.redis.srem(`type:${agent.type}`, id);
    await this.redis.srem(`status:${agent.status}`, id);
    
    return true;
  }
  
  async findAgents(query: any): Promise<Agent[]> {
    const agents = await this.collection.find(query).toArray();
    return agents as Agent[];
  }
  
  async findAgentsByCapabilities(capabilities: string[], requireAll: boolean = false): Promise<Agent[]> {
    if (capabilities.length === 0) {
      return [];
    }
    
    if (requireAll) {
      // Use MongoDB for complex query
      const agents = await this.collection.find({
        capabilities: { $all: capabilities }
      }).toArray();
      
      return agents as Agent[];
    } else {
      // Use Redis for better performance
      let agentIds: string[] = [];
      
      // Get first capability's agents
      agentIds = await this.redis.smembers(`capability:${capabilities[0]}`);
      
      // Union with other capabilities
      if (capabilities.length > 1) {
        const keys = capabilities.map(c => `capability:${c}`);
        agentIds = await this.redis.sunion(...keys);
      }
      
      // Get full agent objects
      const agents: Agent[] = [];
      for (const id of agentIds) {
        const agent = await this.getAgent(id);
        if (agent) {
          agents.push(agent);
        }
      }
      
      return agents;
    }
  }
  
  async findAvailableAgents(type?: string, capabilities?: string[]): Promise<Agent[]> {
    // Get available agents
    const availableIds = await this.redis.smembers('status:available');
    
    // Filter by type if specified
    let filteredIds = availableIds;
    if (type) {
      const typeIds = await this.redis.smembers(`type:${type}`);
      filteredIds = availableIds.filter(id => typeIds.includes(id));
    }
    
    // Filter by capabilities if specified
    if (capabilities && capabilities.length > 0) {
      const agents: Agent[] = [];
      
      for (const id of filteredIds) {
        const agent = await this.getAgent(id);
        
        if (agent && capabilities.every(c => agent.capabilities.includes(c))) {
          agents.push(agent);
        }
      }
      
      return agents;
    } else {
      // Get full agent objects
      const agents: Agent[] = [];
      for (const id of filteredIds) {
        const agent = await this.getAgent(id);
        if (agent) {
          agents.push(agent);
        }
      }
      
      return agents;
    }
  }
  
  async updateAgentStatus(id: string, status: AgentStatus): Promise<boolean> {
    const agent = await this.getAgent(id);
    
    if (!agent) return false;
    
    // Update in MongoDB and Redis
    const updated = await this.updateAgent(id, { status });
    
    return !!updated;
  }
  
  async updateAgentPerformance(id: string, metrics: Partial<Agent['performance']>): Promise<boolean> {
    const agent = await this.getAgent(id);
    
    if (!agent) return false;
    
    // Update performance metrics
    const performance = {
      ...agent.performance,
      ...metrics
    };
    
    // Update in MongoDB and Redis
    const updated = await this.updateAgent(id, { performance });
    
    return !!updated;
  }
}
```

### 3.2 Agent Factory

```typescript
class AgentFactory {
  private agentRegistry: AgentRegistry;
  private llmService: LLMService;
  private roleManager: RoleManager;
  
  constructor(config: AgentFactoryConfig) {
    this.agentRegistry = config.agentRegistry;
    this.llmService = config.llmService;
    this.roleManager = config.roleManager;
  }
  
  async createAgent(type: AgentType, options: AgentCreationOptions = {}): Promise<string> {
    // Get agent template based on type
    const template = this.getAgentTemplate(type);
    
    // Merge template with options
    const agentConfig: Partial<Agent> = {
      ...template,
      ...options,
      capabilities: [
        ...template.capabilities,
        ...(options.capabilities || [])
      ],
      model: {
        ...template.model,
        ...(options.model || {})
      }
    };
    
    // Register agent
    const agentId = await this.agentRegistry.registerAgent(agentConfig);
    
    // Assign default role if specified
    if (template.defaultRole) {
      const role = await this.roleManager.getRole(template.defaultRole);
      
      if (role) {
        await this.roleManager.assignRole(agentId, role.id);
      }
    }
    
    return agentId;
  }
  
  private getAgentTemplate(type: AgentType): Partial<Agent> {
    switch (type) {
      case 'coordinator':
        return {
          type: 'coordinator',
          capabilities: ['planning', 'coordination', 'decision_making'],
          model: {
            provider: 'anthropic',
            model: 'claude-3-opus-20240229',
            parameters: {
              temperature: 0.7,
              max_tokens: 4000
            }
          },
          defaultRole: 'team_lead'
        };
        
      case 'planner':
        return {
          type: 'planner',
          capabilities: ['task_decomposition', 'estimation', 'dependency_analysis'],
          model: {
            provider: 'anthropic',
            model: 'claude-3-sonnet-20240229',
            parameters: {
              temperature: 0.2,
              max_tokens: 4000
            }
          },
          defaultRole: 'planner'
        };
        
      case 'researcher':
        return {
          type: 'researcher',
          capabilities: ['information_gathering', 'analysis', 'summarization'],
          model: {
            provider: 'anthropic',
            model: 'claude-3-haiku-20240307',
            parameters: {
              temperature: 0.3,
              max_tokens: 2000
            }
          },
          defaultRole: 'researcher'
        };
        
      case 'coder':
        return {
          type: 'coder',
          capabilities: ['code_generation', 'debugging', 'refactoring'],
          model: {
            provider: 'anthropic',
            model: 'claude-3-opus-20240229',
            parameters: {
              temperature: 0.1,
              max_tokens: 4000
            }
          },
          defaultRole: 'developer'
        };
        
      case 'reviewer':
        return {
          type: 'reviewer',
          capabilities: ['code_review', 'testing', 'quality_assurance'],
          model: {
            provider: 'anthropic',
            model: 'claude-3-sonnet-20240229',
            parameters: {
              temperature: 0.2,
              max_tokens: 3000
            }
          },
          defaultRole: 'reviewer'
        };
        
      default:
        return {
          type: 'general',
          capabilities: [],
          model: {
            provider: 'anthropic',
            model: 'claude-3-sonnet-20240229',
            parameters: {
              temperature: 0.5,
              max_tokens: 2000
            }
          }
        };
    }
  }
  
  async createAgentInstance(agentId: string): Promise<AgentInstance> {
    const agent = await this.agentRegistry.getAgent(agentId);
    
    if (!agent) {
      throw new Error(`Agent ${agentId} not found`);
    }
    
    // Create LLM instance based on agent configuration
    const llm = this.llmService.createLLMInstance(agent.model);
    
    // Create agent instance
    return new AgentInstance(agent, llm);
  }
}
```

### 3.3 Agent Instance

```typescript
class AgentInstance {
  private agent: Agent;
  private llm: LLMInstance;
  private memory: MemoryManager;
  private toolService: ToolService;
  
  constructor(agent: Agent, llm: LLMInstance, services?: AgentServices) {
    this.agent = agent;
    this.llm = llm;
    this.memory = services?.memory || new MemoryManager();
    this.toolService = services?.toolService || new ToolService();
  }
  
  async processMessage(message: Message): Promise<Message> {
    // Update agent status
    this.agent.status = 'busy';
    
    try {
      // Store message in memory
      await this.memory.storeMemory(message, 'episodic', {
        importance: 0.7,
        tags: ['message', message.type],
        episode: {
          id: message.conversationId,
          title: `Conversation ${message.conversationId}`
        }
      });
      
      // Get conversation history
      const conversationHistory = await this.getConversationHistory(message.conversationId);
      
      // Get relevant context
      const context = await this.getRelevantContext(message);
      
      // Prepare prompt
      const prompt = this.preparePrompt(message, conversationHistory, context);
      
      // Generate response
      const responseContent = await this.llm.generateCompletion(prompt);
      
      // Parse tool calls if any
      const toolCalls = this.parseToolCalls(responseContent);
      
      // Execute tool calls if any
      let finalResponse = responseContent;
      if (toolCalls.length > 0) {
        const toolResults = await this.executeToolCalls(toolCalls);
        
        // Generate final response with tool results
        finalResponse = await this.generateFinalResponse(message, responseContent, toolResults);
      }
      
      // Create response message
      const response: Message = {
        id: uuidv4(),
        conversationId: message.conversationId,
        type: 'response',
        sender: {
          agentId: this.agent.id,
          role: this.agent.roles[0] || 'unknown'
        },
        recipients: [
          {
            agentId: message.sender.agentId,
            role: message.sender.role
          }
        ],
        content: {
          text: finalResponse
        },
        context: message.context,
        timestamp: Date.now()
      };
      
      // Store response in memory
      await this.memory.storeMemory(response, 'episodic', {
        importance: 0.7,
        tags: ['message', 'response'],
        episode: {
          id: message.conversationId,
          title: `Conversation ${message.conversationId}`
        }
      });
      
      return response;
    } finally {
      // Update agent status
      this.agent.status = 'available';
    }
  }
  
  private async getConversationHistory(conversationId: string): Promise<Message[]> {
    // Retrieve conversation history from memory
    const memories = await this.memory.retrieveRelevantMemories('', {
      types: ['episodic'],
      limit: 20,
      filter: {
        'metadata.tags': 'message',
        'metadata.episode.id': conversationId
      }
    });
    
    // Sort by timestamp
    memories.sort((a, b) => {
      const timeA = new Date(a.metadata.timestamp).getTime();
      const timeB = new Date(b.metadata.timestamp).getTime();
      return timeA - timeB;
    });
    
    return memories.map(m => m.content as Message);
  }
  
  private async getRelevantContext(message: Message): Promise<string> {
    // Get relevant memories based on message content
    const memories = await this.memory.retrieveRelevantMemories(message.content.text, {
      types: ['semantic', 'episodic'],
      limit: 5
    });
    
    // Format context
    return memories.map(m => {
      const contentText = typeof m.content === 'string' 
        ? m.content 
        : JSON.stringify(m.content);
      
      return `[${m.type.toUpperCase()}] ${contentText}`;
    }).join('\n\n');
  }
  
  private preparePrompt(message: Message, history: Message[], context: string): string {
    // Format conversation history
    const historyText = history.map(m => {
      return `${m.sender.role}: ${m.content.text}`;
    }).join('\n\n');
    
    // Build prompt
    return `
      You are ${this.agent.type} agent with ID ${this.agent.id}.
      Your capabilities include: ${this.agent.capabilities.join(', ')}.
      Your current role is: ${this.agent.roles[0] || 'unassigned'}.
      
      Relevant context:
      ${context}
      
      Conversation history:
      ${historyText}
      
      New message from ${message.sender.role}:
      ${message.content.text}
      
      Respond appropriately based on your role and capabilities.
      If you need to use tools, format your response as follows:
      
      <tool>
      {
        "tool": "tool_name",
        "parameters": {
          "param1": "value1",
          "param2": "value2"
        }
      }
      </tool>
      
      Your response:
    `;
  }
  
  private parseToolCalls(response: string): ToolCall[] {
    const toolCalls: ToolCall[] = [];
    const toolRegex = /<tool>([\s\S]*?)<\/tool>/g;
    
    let match;
    while ((match = toolRegex.exec(response)) !== null) {
      try {
        const toolCallJson = match[1].trim();
        const toolCall = JSON.parse(toolCallJson);
        
        toolCalls.push({
          tool: toolCall.tool,
          parameters: toolCall.parameters
        });
      } catch (error) {
        console.error('Failed to parse tool call:', error);
      }
    }
    
    return toolCalls;
  }
  
  private async executeToolCalls(toolCalls: ToolCall[]): Promise<ToolResult[]> {
    const results: ToolResult[] = [];
    
    for (const toolCall of toolCalls) {
      try {
        const result = await this.toolService.executeTool(
          toolCall.tool,
          toolCall.parameters
        );
        
        results.push({
          tool: toolCall.tool,
          parameters: toolCall.parameters,
          result,
          success: true
        });
      } catch (error) {
        results.push({
          tool: toolCall.tool,
          parameters: toolCall.parameters,
          error: error.message,
          success: false
        });
      }
    }
    
    return results;
  }
  
  private async generateFinalResponse(
    message: Message,
    initialResponse: string,
    toolResults: ToolResult[]
  ): Promise<string> {
    // Replace tool calls with results
    let finalResponse = initialResponse;
    
    for (const result of toolResults) {
      const toolCallRegex = new RegExp(
        `<tool>\\s*{\\s*"tool"\\s*:\\s*"${result.tool}"[\\s\\S]*?<\\/tool>`,
        'g'
      );
      
      const replacement = result.success
        ? `Tool Result (${result.tool}): ${JSON.stringify(result.result)}`
        : `Tool Error (${result.tool}): ${result.error}`;
      
      finalResponse = finalResponse.replace(toolCallRegex, replacement);
    }
    
    // If there are still tool calls, they weren't executed
    if (finalResponse.includes('<tool>')) {
      finalResponse = finalResponse.replace(/<tool>[\s\S]*?<\/tool>/g, 'Tool call could not be executed.');
    }
    
    return finalResponse;
  }
}
```
