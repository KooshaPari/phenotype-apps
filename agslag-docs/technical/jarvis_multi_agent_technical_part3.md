# Jarvis SWE Agent Multi-Agent Coordination - Technical Implementation (Part 3)

## 6. Message Bus Implementation

### 6.1 Message Bus Service

```typescript
class MessageBus {
  private redis: Redis.Redis;
  private db: MongoDB.Db;
  private collection: MongoDB.Collection;
  private subscribers: Map<string, Set<MessageSubscriber>>;
  
  constructor(config: MessageBusConfig) {
    this.redis = new Redis(config.redisUri);
    
    const client = new MongoDB.MongoClient(config.mongoUri);
    this.db = client.db(config.database || 'jarvis');
    this.collection = this.db.collection(config.collection || 'messages');
    
    this.subscribers = new Map();
    
    // Create indexes
    this.collection.createIndex({ id: 1 }, { unique: true });
    this.collection.createIndex({ conversationId: 1 });
    this.collection.createIndex({ 'sender.agentId': 1 });
    this.collection.createIndex({ 'recipients.agentId': 1 });
    this.collection.createIndex({ timestamp: 1 });
    
    // Setup Redis subscription
    this.setupRedisSubscription();
  }
  
  private async setupRedisSubscription(): Promise<void> {
    const subscriber = new Redis(this.redis.options);
    
    // Subscribe to message channel
    await subscriber.subscribe('jarvis:messages');
    
    // Handle messages
    subscriber.on('message', (channel, message) => {
      if (channel === 'jarvis:messages') {
        try {
          const parsedMessage = JSON.parse(message) as Message;
          this.notifySubscribers(parsedMessage);
        } catch (error) {
          console.error('Failed to parse message:', error);
        }
      }
    });
  }
  
  async sendMessage(message: Omit<Message, 'id' | 'timestamp'>): Promise<string> {
    const id = uuidv4();
    
    const fullMessage: Message = {
      ...message,
      id,
      timestamp: Date.now()
    };
    
    // Store in MongoDB
    await this.collection.insertOne(fullMessage);
    
    // Publish to Redis
    await this.redis.publish('jarvis:messages', JSON.stringify(fullMessage));
    
    return id;
  }
  
  async getMessage(id: string): Promise<Message | null> {
    const message = await this.collection.findOne({ id });
    return message as Message;
  }
  
  async getConversationMessages(conversationId: string, limit: number = 100): Promise<Message[]> {
    const messages = await this.collection
      .find({ conversationId })
      .sort({ timestamp: 1 })
      .limit(limit)
      .toArray();
    
    return messages as Message[];
  }
  
  async getAgentMessages(agentId: string, limit: number = 100): Promise<Message[]> {
    const messages = await this.collection
      .find({
        $or: [
          { 'sender.agentId': agentId },
          { 'recipients.agentId': agentId }
        ]
      })
      .sort({ timestamp: -1 })
      .limit(limit)
      .toArray();
    
    return messages as Message[];
  }
  
  subscribe(topic: string, subscriber: MessageSubscriber): void {
    if (!this.subscribers.has(topic)) {
      this.subscribers.set(topic, new Set());
    }
    
    this.subscribers.get(topic)!.add(subscriber);
  }
  
  unsubscribe(topic: string, subscriber: MessageSubscriber): void {
    if (this.subscribers.has(topic)) {
      this.subscribers.get(topic)!.delete(subscriber);
    }
  }
  
  private notifySubscribers(message: Message): void {
    // Notify subscribers for specific conversation
    const conversationSubscribers = this.subscribers.get(`conversation:${message.conversationId}`);
    if (conversationSubscribers) {
      for (const subscriber of conversationSubscribers) {
        subscriber.onMessage(message);
      }
    }
    
    // Notify subscribers for sender
    const senderSubscribers = this.subscribers.get(`agent:${message.sender.agentId}`);
    if (senderSubscribers) {
      for (const subscriber of senderSubscribers) {
        subscriber.onMessage(message);
      }
    }
    
    // Notify subscribers for recipients
    for (const recipient of message.recipients) {
      const recipientSubscribers = this.subscribers.get(`agent:${recipient.agentId}`);
      if (recipientSubscribers) {
        for (const subscriber of recipientSubscribers) {
          subscriber.onMessage(message);
        }
      }
    }
    
    // Notify global subscribers
    const globalSubscribers = this.subscribers.get('global');
    if (globalSubscribers) {
      for (const subscriber of globalSubscribers) {
        subscriber.onMessage(message);
      }
    }
  }
}
```

### 6.2 Message Subscriber Interface

```typescript
interface MessageSubscriber {
  onMessage(message: Message): void;
}

class AgentMessageHandler implements MessageSubscriber {
  private agentId: string;
  private agentInstance: AgentInstance;
  
  constructor(agentId: string, agentInstance: AgentInstance) {
    this.agentId = agentId;
    this.agentInstance = agentInstance;
  }
  
  async onMessage(message: Message): Promise<void> {
    // Check if message is for this agent
    const isRecipient = message.recipients.some(r => r.agentId === this.agentId);
    
    if (!isRecipient) return;
    
    // Process message
    try {
      const response = await this.agentInstance.processMessage(message);
      
      // Response will be sent by the agent instance
    } catch (error) {
      console.error(`Agent ${this.agentId} failed to process message:`, error);
    }
  }
}
```

### 6.3 Conversation Manager

```typescript
class ConversationManager {
  private messageBus: MessageBus;
  private db: MongoDB.Db;
  private collection: MongoDB.Collection;
  
  constructor(config: ConversationManagerConfig) {
    this.messageBus = config.messageBus;
    
    const client = new MongoDB.MongoClient(config.mongoUri);
    this.db = client.db(config.database || 'jarvis');
    this.collection = this.db.collection(config.collection || 'conversations');
    
    // Create indexes
    this.collection.createIndex({ id: 1 }, { unique: true });
    this.collection.createIndex({ participants: 1 });
    this.collection.createIndex({ status: 1 });
  }
  
  async createConversation(participants: string[], metadata: any = {}): Promise<string> {
    const id = uuidv4();
    
    const conversation = {
      id,
      participants,
      status: 'active',
      created: new Date(),
      lastActivity: new Date(),
      metadata
    };
    
    // Store in MongoDB
    await this.collection.insertOne(conversation);
    
    return id;
  }
  
  async getConversation(id: string): Promise<any> {
    return this.collection.findOne({ id });
  }
  
  async getAgentConversations(agentId: string): Promise<any[]> {
    return this.collection.find({ participants: agentId }).toArray();
  }
  
  async sendMessage(
    conversationId: string,
    senderId: string,
    senderRole: string,
    recipientIds: string[],
    content: string | object,
    messageType: MessageType = 'text',
    context: any = {}
  ): Promise<string> {
    // Get conversation
    const conversation = await this.getConversation(conversationId);
    
    if (!conversation) {
      throw new Error(`Conversation ${conversationId} not found`);
    }
    
    // Validate sender and recipients
    if (!conversation.participants.includes(senderId)) {
      throw new Error(`Sender ${senderId} is not a participant in conversation ${conversationId}`);
    }
    
    const invalidRecipients = recipientIds.filter(id => !conversation.participants.includes(id));
    if (invalidRecipients.length > 0) {
      throw new Error(`Recipients ${invalidRecipients.join(', ')} are not participants in conversation ${conversationId}`);
    }
    
    // Format recipients
    const recipients = recipientIds.map(agentId => ({
      agentId,
      role: 'unknown' // Would need to look up roles
    }));
    
    // Create message
    const message: Omit<Message, 'id' | 'timestamp'> = {
      conversationId,
      type: messageType,
      sender: {
        agentId: senderId,
        role: senderRole
      },
      recipients,
      content: {
        text: typeof content === 'string' ? content : JSON.stringify(content),
        ...(typeof content === 'object' ? { metadata: content } : {})
      },
      context
    };
    
    // Send message
    const messageId = await this.messageBus.sendMessage(message);
    
    // Update conversation last activity
    await this.collection.updateOne(
      { id: conversationId },
      { $set: { lastActivity: new Date() } }
    );
    
    return messageId;
  }
  
  async getConversationMessages(conversationId: string, limit: number = 100): Promise<Message[]> {
    return this.messageBus.getConversationMessages(conversationId, limit);
  }
  
  async closeConversation(conversationId: string): Promise<boolean> {
    const result = await this.collection.updateOne(
      { id: conversationId },
      { $set: { status: 'closed', closedAt: new Date() } }
    );
    
    return result.modifiedCount > 0;
  }
}
```

## 7. Task Allocator Implementation

### 7.1 Task Service

```typescript
class TaskService {
  private db: MongoDB.Db;
  private collection: MongoDB.Collection;
  private agentRegistry: AgentRegistry;
  private messageBus: MessageBus;
  
  constructor(config: TaskServiceConfig) {
    const client = new MongoDB.MongoClient(config.mongoUri);
    this.db = client.db(config.database || 'jarvis');
    this.collection = this.db.collection(config.collection || 'tasks');
    this.agentRegistry = config.agentRegistry;
    this.messageBus = config.messageBus;
    
    // Create indexes
    this.collection.createIndex({ id: 1 }, { unique: true });
    this.collection.createIndex({ status: 1 });
    this.collection.createIndex({ assignee: 1 });
    this.collection.createIndex({ 'metadata.creator': 1 });
    this.collection.createIndex({ dependencies: 1 });
  }
  
  async createTask(task: Partial<Task>): Promise<string> {
    const id = task.id || uuidv4();
    
    const newTask: Task = {
      id,
      title: task.title || 'Untitled Task',
      description: task.description || '',
      type: task.type || 'general',
      priority: task.priority || 'medium',
      status: task.status || 'pending',
      assignee: task.assignee,
      dependencies: task.dependencies || [],
      deadline: task.deadline,
      estimatedEffort: task.estimatedEffort || 1,
      progress: task.progress || 0,
      metadata: {
        created: new Date(),
        lastUpdated: new Date(),
        creator: task.metadata?.creator || 'system',
        tags: task.metadata?.tags || []
      }
    };
    
    // Store in MongoDB
    await this.collection.insertOne(newTask);
    
    // Notify assignee if specified
    if (newTask.assignee) {
      await this.notifyTaskAssignment(newTask);
    }
    
    return id;
  }
  
  async getTask(id: string): Promise<Task | null> {
    const task = await this.collection.findOne({ id });
    return task as Task;
  }
  
  async updateTask(id: string, updates: Partial<Task>): Promise<Task | null> {
    const task = await this.getTask(id);
    
    if (!task) return null;
    
    const updateDoc: any = { $set: {} };
    
    if (updates.title !== undefined) {
      updateDoc.$set.title = updates.title;
    }
    
    if (updates.description !== undefined) {
      updateDoc.$set.description = updates.description;
    }
    
    if (updates.type !== undefined) {
      updateDoc.$set.type = updates.type;
    }
    
    if (updates.priority !== undefined) {
      updateDoc.$set.priority = updates.priority;
    }
    
    if (updates.status !== undefined) {
      updateDoc.$set.status = updates.status;
    }
    
    if (updates.deadline !== undefined) {
      updateDoc.$set.deadline = updates.deadline;
    }
    
    if (updates.estimatedEffort !== undefined) {
      updateDoc.$set.estimatedEffort = updates.estimatedEffort;
    }
    
    if (updates.actualEffort !== undefined) {
      updateDoc.$set.actualEffort = updates.actualEffort;
    }
    
    if (updates.progress !== undefined) {
      updateDoc.$set.progress = updates.progress;
    }
    
    if (updates.result !== undefined) {
      updateDoc.$set.result = updates.result;
    }
    
    if (updates.dependencies !== undefined) {
      updateDoc.$set.dependencies = updates.dependencies;
    }
    
    // Always update lastUpdated
    updateDoc.$set['metadata.lastUpdated'] = new Date();
    
    // Handle assignee change separately
    const assigneeChanged = updates.assignee !== undefined && updates.assignee !== task.assignee;
    if (assigneeChanged) {
      updateDoc.$set.assignee = updates.assignee;
    }
    
    // Update task
    const result = await this.collection.updateOne({ id }, updateDoc);
    
    if (result.modifiedCount === 0) return null;
    
    const updatedTask = await this.getTask(id);
    
    // Notify new assignee if changed
    if (assigneeChanged && updatedTask && updatedTask.assignee) {
      await this.notifyTaskAssignment(updatedTask);
    }
    
    return updatedTask;
  }
  
  async deleteTask(id: string): Promise<boolean> {
    const result = await this.collection.deleteOne({ id });
    return result.deletedCount > 0;
  }
  
  async listTasks(filter: any = {}): Promise<Task[]> {
    const tasks = await this.collection.find(filter).toArray();
    return tasks as Task[];
  }
  
  async getAgentTasks(agentId: string): Promise<Task[]> {
    const tasks = await this.collection.find({ assignee: agentId }).toArray();
    return tasks as Task[];
  }
  
  async assignTask(taskId: string, agentId: string): Promise<boolean> {
    const task = await this.getTask(taskId);
    const agent = await this.agentRegistry.getAgent(agentId);
    
    if (!task || !agent) return false;
    
    // Update task assignee
    const updatedTask = await this.updateTask(taskId, { 
      assignee: agentId,
      status: 'assigned'
    });
    
    return !!updatedTask;
  }
  
  async updateTaskStatus(taskId: string, status: TaskStatus, progress?: number): Promise<boolean> {
    const updates: Partial<Task> = { status };
    
    if (progress !== undefined) {
      updates.progress = progress;
    }
    
    const updatedTask = await this.updateTask(taskId, updates);
    
    return !!updatedTask;
  }
  
  async completeTask(taskId: string, result: any): Promise<boolean> {
    const updatedTask = await this.updateTask(taskId, {
      status: 'completed',
      progress: 100,
      result
    });
    
    if (updatedTask) {
      // Check if this task was a dependency for other tasks
      const dependentTasks = await this.collection.find({
        dependencies: taskId
      }).toArray();
      
      // Update dependent tasks
      for (const dependentTask of dependentTasks) {
        // Check if all dependencies are completed
        const dependencies = await this.collection.find({
          id: { $in: dependentTask.dependencies },
          status: { $ne: 'completed' }
        }).toArray();
        
        if (dependencies.length === 0) {
          // All dependencies completed, update status
          await this.updateTask(dependentTask.id, {
            status: 'ready'
          });
          
          // Notify assignee if assigned
          if (dependentTask.assignee) {
            await this.notifyTaskReady(dependentTask);
          }
        }
      }
    }
    
    return !!updatedTask;
  }
  
  private async notifyTaskAssignment(task: Task): Promise<void> {
    if (!task.assignee) return;
    
    // Create conversation if needed
    const conversationId = `task:${task.id}`;
    
    // Send message
    await this.messageBus.sendMessage({
      conversationId,
      type: 'task_assignment',
      sender: {
        agentId: 'system',
        role: 'system'
      },
      recipients: [
        {
          agentId: task.assignee,
          role: 'assignee'
        }
      ],
      content: {
        text: `You have been assigned a new task: ${task.title}`,
        attachments: [task]
      },
      context: {
        taskId: task.id
      }
    });
  }
  
  private async notifyTaskReady(task: Task): Promise<void> {
    if (!task.assignee) return;
    
    // Create conversation if needed
    const conversationId = `task:${task.id}`;
    
    // Send message
    await this.messageBus.sendMessage({
      conversationId,
      type: 'task_ready',
      sender: {
        agentId: 'system',
        role: 'system'
      },
      recipients: [
        {
          agentId: task.assignee,
          role: 'assignee'
        }
      ],
      content: {
        text: `Task is now ready to start: ${task.title}`,
        attachments: [task]
      },
      context: {
        taskId: task.id
      }
    });
  }
}
```

### 7.2 Task Allocator

```typescript
class TaskAllocator {
  private taskService: TaskService;
  private agentRegistry: AgentRegistry;
  private roleManager: RoleManager;
  
  constructor(config: TaskAllocatorConfig) {
    this.taskService = config.taskService;
    this.agentRegistry = config.agentRegistry;
    this.roleManager = config.roleManager;
  }
  
  async allocateTask(taskId: string): Promise<string | null> {
    const task = await this.taskService.getTask(taskId);
    
    if (!task) return null;
    
    // Check if task already has assignee
    if (task.assignee) return task.assignee;
    
    // Check if task has unmet dependencies
    if (task.dependencies.length > 0) {
      const dependencies = await this.taskService.listTasks({
        id: { $in: task.dependencies }
      });
      
      const incompleteDependencies = dependencies.filter(
        d => d.status !== 'completed'
      );
      
      if (incompleteDependencies.length > 0) {
        // Cannot allocate task with incomplete dependencies
        return null;
      }
    }
    
    // Find suitable agents based on task type and capabilities
    const requiredCapabilities = this.getRequiredCapabilities(task);
    const agents = await this.agentRegistry.findAgentsByCapabilities(requiredCapabilities, true);
    
    // Filter to available agents
    const availableAgents = agents.filter(a => a.status === 'available');
    
    if (availableAgents.length === 0) {
      return null;
    }
    
    // Select best agent (could implement more sophisticated selection)
    const selectedAgent = this.selectBestAgent(availableAgents, task);
    
    // Assign task
    const assigned = await this.taskService.assignTask(taskId, selectedAgent.id);
    
    return assigned ? selectedAgent.id : null;
  }
  
  private getRequiredCapabilities(task: Task): string[] {
    // Map task type to required capabilities
    switch (task.type) {
      case 'code_generation':
        return ['code_generation'];
      case 'code_review':
        return ['code_review'];
      case 'debugging':
        return ['debugging'];
      case 'research':
        return ['information_gathering'];
      case 'planning':
        return ['task_decomposition'];
      case 'testing':
        return ['testing'];
      default:
        return [];
    }
  }
  
  private selectBestAgent(agents: Agent[], task: Task): Agent {
    // Simple selection strategy - could be much more sophisticated
    // For now, just select the first available agent
    return agents[0];
  }
  
  async decomposeTasks(objective: string, llmService: LLMService): Promise<string[]> {
    // Use LLM to decompose objective into tasks
    const prompt = `
      Decompose the following objective into a set of tasks:
      
      ${objective}
      
      For each task, provide:
      1. Title
      2. Description
      3. Type (code_generation, code_review, debugging, research, planning, testing, or general)
      4. Priority (high, medium, low)
      5. Estimated effort (in hours)
      6. Dependencies (if any)
      
      Format your response as a JSON array of task objects.
    `;
    
    const response = await llmService.generateCompletion(prompt, 'task_decomposition');
    
    try {
      const tasks = JSON.parse(response);
      
      // Create tasks
      const taskIds: string[] = [];
      
      for (const taskData of tasks) {
        const taskId = await this.taskService.createTask({
          title: taskData.title,
          description: taskData.description,
          type: taskData.type,
          priority: taskData.priority,
          estimatedEffort: taskData.estimatedEffort,
          dependencies: [] // Will update after all tasks are created
        });
        
        taskIds.push(taskId);
        
        // Store dependency information for later
        taskData.taskId = taskId;
      }
      
      // Update dependencies
      for (const taskData of tasks) {
        if (taskData.dependencies && taskData.dependencies.length > 0) {
          const dependencyIds = taskData.dependencies.map((depTitle: string) => {
            const dependency = tasks.find((t: any) => t.title === depTitle);
            return dependency ? dependency.taskId : null;
          }).filter((id: string | null) => id !== null);
          
          if (dependencyIds.length > 0) {
            await this.taskService.updateTask(taskData.taskId, {
              dependencies: dependencyIds
            });
          }
        }
      }
      
      return taskIds;
    } catch (error) {
      console.error('Failed to parse task decomposition:', error);
      return [];
    }
  }
}
```
