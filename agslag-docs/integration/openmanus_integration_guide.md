# OpenManus Integration Guide

## Overview

This guide details the process of integrating OpenManus, an open-source implementation of the Manus AI agent, into the AGSLAG MCP Integration system. OpenManus provides multi-agent capabilities for autonomous task execution that can enhance AGSLAG's ability to perform complex tasks.

## Prerequisites

- AGSLAG MCP Integration system set up and running
- OpenManus repository cloned and accessible
- Docker and Docker Compose installed
- Node.js and npm installed
- TypeScript development environment configured

## Integration Steps

### 1. Create Autonomous Task Execution Interface

First, we need to define an interface that represents the autonomous task execution capabilities we want to expose from OpenManus:

```typescript
// interfaces/autonomous_task_execution.ts
export interface AutonomousTaskExecution {
  /**
   * Execute a task autonomously
   */
  executeTask(params: {
    task: string;
    context?: string;
    constraints?: string[];
  }): Promise<{
    result: string;
    steps: string[];
    resources: string[];
  }>;
  
  /**
   * Get task execution status
   */
  getTaskStatus(params: {
    taskId: string;
  }): Promise<{
    status: 'pending' | 'running' | 'completed' | 'failed';
    progress?: number;
    result?: string;
    error?: string;
  }>;
  
  /**
   * List available agents
   */
  listAgents(): Promise<Array<{
    id: string;
    name: string;
    role: string;
    capabilities: string[];
  }>>;
  
  /**
   * Assign task to specific agent
   */
  assignTask(params: {
    taskId: string;
    agentId: string;
  }): Promise<{
    success: boolean;
    message?: string;
  }>;
  
  /**
   * Create a team of agents
   */
  createTeam(params: {
    name: string;
    agents: string[];
  }): Promise<{
    id: string;
    name: string;
    agents: string[];
  }>;
  
  /**
   * Assign task to team
   */
  assignTaskToTeam(params: {
    taskId: string;
    teamId: string;
  }): Promise<{
    success: boolean;
    message?: string;
  }>;
}
```

### 2. Create OpenManus Adapter

Next, implement the adapter that connects to OpenManus's API:

```typescript
// adapters/openmanus_adapter.ts
import { AutonomousTaskExecution } from '../interfaces/autonomous_task_execution';
import axios from 'axios';
import { v4 as uuidv4 } from 'uuid';

export class AgslagOpenManusAdapter implements AutonomousTaskExecution {
  private baseUrl: string;
  private tasks: Map<string, any> = new Map();
  
  constructor(config: {
    baseUrl: string;
  }) {
    this.baseUrl = config.baseUrl;
  }
  
  /**
   * Execute a task autonomously
   */
  async executeTask(params: {
    task: string;
    context?: string;
    constraints?: string[];
  }): Promise<{
    result: string;
    steps: string[];
    resources: string[];
  }> {
    try {
      const response = await axios.post(`${this.baseUrl}/task`, {
        task: params.task,
        context: params.context,
        constraints: params.constraints
      });
      
      // Store the task for later reference
      const taskId = response.data.task_id;
      this.tasks.set(taskId, {
        id: taskId,
        status: 'running',
        task: params.task
      });
      
      // Wait for the task to complete
      let status = await this.getTaskStatus({ taskId });
      
      while (status.status === 'running' || status.status === 'pending') {
        // Wait for 2 seconds before checking again
        await new Promise(resolve => setTimeout(resolve, 2000));
        status = await this.getTaskStatus({ taskId });
      }
      
      if (status.status === 'failed') {
        throw new Error(`Task execution failed: ${status.error}`);
      }
      
      // Get the detailed result
      const detailResponse = await axios.get(`${this.baseUrl}/task/${taskId}/detail`);
      
      return {
        result: detailResponse.data.result,
        steps: detailResponse.data.steps,
        resources: detailResponse.data.resources
      };
    } catch (error) {
      console.error('[AgslagOpenManusAdapter] Error executing task:', error);
      throw new Error(`Failed to execute task: ${error.message}`);
    }
  }
  
  /**
   * Get task execution status
   */
  async getTaskStatus(params: {
    taskId: string;
  }): Promise<{
    status: 'pending' | 'running' | 'completed' | 'failed';
    progress?: number;
    result?: string;
    error?: string;
  }> {
    try {
      const response = await axios.get(`${this.baseUrl}/task/${params.taskId}/status`);
      
      // Update the stored task
      if (this.tasks.has(params.taskId)) {
        const task = this.tasks.get(params.taskId);
        task.status = response.data.status;
        this.tasks.set(params.taskId, task);
      }
      
      return {
        status: response.data.status,
        progress: response.data.progress,
        result: response.data.result,
        error: response.data.error
      };
    } catch (error) {
      console.error('[AgslagOpenManusAdapter] Error getting task status:', error);
      throw new Error(`Failed to get task status: ${error.message}`);
    }
  }
  
  /**
   * List available agents
   */
  async listAgents(): Promise<Array<{
    id: string;
    name: string;
    role: string;
    capabilities: string[];
  }>> {
    try {
      const response = await axios.get(`${this.baseUrl}/agents`);
      
      return response.data.agents;
    } catch (error) {
      console.error('[AgslagOpenManusAdapter] Error listing agents:', error);
      throw new Error(`Failed to list agents: ${error.message}`);
    }
  }
  
  /**
   * Assign task to specific agent
   */
  async assignTask(params: {
    taskId: string;
    agentId: string;
  }): Promise<{
    success: boolean;
    message?: string;
  }> {
    try {
      const response = await axios.post(`${this.baseUrl}/task/${params.taskId}/assign`, {
        agent_id: params.agentId
      });
      
      return {
        success: response.data.success,
        message: response.data.message
      };
    } catch (error) {
      console.error('[AgslagOpenManusAdapter] Error assigning task:', error);
      throw new Error(`Failed to assign task: ${error.message}`);
    }
  }
  
  /**
   * Create a team of agents
   */
  async createTeam(params: {
    name: string;
    agents: string[];
  }): Promise<{
    id: string;
    name: string;
    agents: string[];
  }> {
    try {
      const response = await axios.post(`${this.baseUrl}/team`, {
        name: params.name,
        agents: params.agents
      });
      
      return {
        id: response.data.team_id,
        name: response.data.name,
        agents: response.data.agents
      };
    } catch (error) {
      console.error('[AgslagOpenManusAdapter] Error creating team:', error);
      throw new Error(`Failed to create team: ${error.message}`);
    }
  }
  
  /**
   * Assign task to team
   */
  async assignTaskToTeam(params: {
    taskId: string;
    teamId: string;
  }): Promise<{
    success: boolean;
    message?: string;
  }> {
    try {
      const response = await axios.post(`${this.baseUrl}/task/${params.taskId}/assign-team`, {
        team_id: params.teamId
      });
      
      return {
        success: response.data.success,
        message: response.data.message
      };
    } catch (error) {
      console.error('[AgslagOpenManusAdapter] Error assigning task to team:', error);
      throw new Error(`Failed to assign task to team: ${error.message}`);
    }
  }
}
```

### 3. Extend AGSLAG MCP Integration

Update the AGSLAG MCP Integration class to include the OpenManus adapter:

```typescript
// integration/agslag_mcp_integration.ts
import { AgslagOpenManusAdapter } from '../adapters/openmanus_adapter';

export class AgslagMcpIntegration {
  // Existing code...
  
  private openManus: AgslagOpenManusAdapter;
  
  constructor(config: {
    // Existing config...
    openManus?: {
      baseUrl: string;
    }
  }) {
    // Existing initialization...
    
    // Initialize OpenManus adapter if config is provided
    if (config.openManus) {
      this.openManus = new AgslagOpenManusAdapter(config.openManus);
    }
  }
  
  // New methods for autonomous task execution
  
  /**
   * Execute an autonomous task
   */
  async executeAutonomousTask(task: string, context?: string, constraints?: string[]) {
    if (!this.openManus) {
      throw new Error('OpenManus adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Executing autonomous task: ${task}`);
    
    return await this.openManus.executeTask({
      task,
      context,
      constraints
    });
  }
  
  /**
   * Get task execution status
   */
  async getTaskStatus(taskId: string) {
    if (!this.openManus) {
      throw new Error('OpenManus adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Getting task status: ${taskId}`);
    
    return await this.openManus.getTaskStatus({
      taskId
    });
  }
  
  /**
   * List available agents
   */
  async listAgents() {
    if (!this.openManus) {
      throw new Error('OpenManus adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Listing agents`);
    
    return await this.openManus.listAgents();
  }
  
  /**
   * Assign task to specific agent
   */
  async assignTask(taskId: string, agentId: string) {
    if (!this.openManus) {
      throw new Error('OpenManus adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Assigning task ${taskId} to agent ${agentId}`);
    
    return await this.openManus.assignTask({
      taskId,
      agentId
    });
  }
  
  /**
   * Create a team of agents
   */
  async createTeam(name: string, agents: string[]) {
    if (!this.openManus) {
      throw new Error('OpenManus adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Creating team: ${name}`);
    
    return await this.openManus.createTeam({
      name,
      agents
    });
  }
  
  /**
   * Assign task to team
   */
  async assignTaskToTeam(taskId: string, teamId: string) {
    if (!this.openManus) {
      throw new Error('OpenManus adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Assigning task ${taskId} to team ${teamId}`);
    
    return await this.openManus.assignTaskToTeam({
      taskId,
      teamId
    });
  }
}
```

### 4. Create Example Usage

Create an example file that demonstrates how to use the OpenManus integration:

```typescript
// examples/openmanus_example.ts
import { AgslagMcpIntegration } from '../integration/agslag_mcp_integration';

async function openManusExample() {
  // Initialize AGSLAG MCP Integration with OpenManus config
  const integration = new AgslagMcpIntegration({
    openManus: {
      baseUrl: 'http://localhost:8000' // Adjust based on your OpenManus setup
    }
  });
  
  try {
    // List available agents
    console.log('Listing available agents...');
    const agents = await integration.listAgents();
    console.log('Available agents:', agents);
    
    // Create a research team
    console.log('Creating a research team...');
    const researchTeam = await integration.createTeam(
      'Research Team',
      agents
        .filter(agent => ['researcher', 'browser', 'reporter'].includes(agent.role))
        .map(agent => agent.id)
    );
    console.log('Research team created:', researchTeam);
    
    // Execute an autonomous task
    console.log('Executing an autonomous task...');
    const task = 'Research the latest advancements in artificial intelligence and prepare a summary report';
    const context = 'Focus on practical applications in software development';
    const constraints = [
      'Include at least 5 recent research papers',
      'Focus on the last 2 years',
      'Include code examples where applicable'
    ];
    
    const result = await integration.executeAutonomousTask(task, context, constraints);
    console.log('Task execution completed with result:', result);
    
    // Execute a task and assign it to the research team
    console.log('Executing a task and assigning it to the research team...');
    const taskWithTeam = 'Analyze the performance of different machine learning frameworks on common tasks';
    
    // This would be a real implementation that creates a task and gets its ID
    // For this example, we'll simulate it
    const taskId = 'task-' + Math.random().toString(36).substring(2, 9);
    
    console.log(`Task created with ID: ${taskId}`);
    
    // Assign the task to the research team
    const assignResult = await integration.assignTaskToTeam(taskId, researchTeam.id);
    console.log('Task assignment result:', assignResult);
    
    // In a real implementation, we would poll for the task status
    // For this example, we'll simulate it
    console.log('Task is being processed by the research team...');
    
    // Simulate waiting for the task to complete
    await new Promise(resolve => setTimeout(resolve, 5000));
    
    console.log('Task completed by the research team');
    
  } catch (error) {
    console.error('Error in OpenManus example:', error);
  }
}

// Run the example
openManusExample().catch(console.error);
```

### 5. Set Up Docker Environment

Create a Docker Compose configuration to run OpenManus:

```yaml
// docker-compose.openmanus.yml
version: '3.8'

services:
  openmanus:
    build: 
      context: ./OpenManus
      dockerfile: docker/unified/Dockerfile
    ports:
      - "8000:8000"
    environment:
      - PYTHONPATH=/app
      - FLASK_APP=src/server.py
      - FLASK_ENV=development
    volumes:
      - ./OpenManus:/app
```

## Configuration

To configure the OpenManus integration, you need to set up the following:

1. **Docker Environment**: Ensure Docker and Docker Compose are installed and configured
2. **OpenManus Server**: Build and run the OpenManus Docker container
3. **Base URL**: Configure the base URL for the OpenManus API
4. **Model Configuration**: Configure the LLM models used by OpenManus

See the [Configuration Management Guide](./configuration_management_guide.md) for more details.

## Testing

To test the OpenManus integration:

1. Start the OpenManus Docker container: `docker-compose -f docker-compose.openmanus.yml up -d`
2. Execute the example script: `npx ts-node examples/openmanus_example.ts`
3. Verify that the agents are listed, teams are created, and tasks are executed successfully

## Troubleshooting

Common issues and solutions:

1. **Docker Errors**: Ensure Docker is running and the OpenManus container is built correctly
2. **Connection Errors**: Verify that the OpenManus server is running and the base URL is correct
3. **API Errors**: Check that the OpenManus API endpoints match the ones used in the adapter
4. **Model Errors**: Ensure that the LLM models are properly configured in OpenManus

## Next Steps

After integrating OpenManus, consider:

1. Creating more complex autonomous tasks that leverage multiple agents
2. Developing a UI for monitoring task execution and agent activities
3. Implementing error handling and recovery mechanisms for task execution
4. Creating templates for common task patterns
