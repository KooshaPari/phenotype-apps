# Fluujo Integration Guide

## Overview

This guide details the process of integrating Fluujo, a workflow orchestration platform, into the AGSLAG MCP Integration system. Fluujo provides visual flow building, model integration, and tool management capabilities that can enhance AGSLAG's workflow orchestration.

## Prerequisites

- AGSLAG MCP Integration system set up and running
- Fluujo repository cloned and accessible
- Node.js and npm installed
- TypeScript development environment configured

## Integration Steps

### 1. Create Flow Orchestration Interface

First, we need to define an interface that represents the flow orchestration capabilities we want to expose from Fluujo:

```typescript
// interfaces/flow_orchestration.ts
export interface FlowOrchestration {
  /**
   * Create a new flow
   */
  createFlow(params: {
    name: string;
    description?: string;
  }): Promise<{
    id: string;
    name: string;
  }>;
  
  /**
   * Add a node to a flow
   */
  addNode(params: {
    flowId: string;
    nodeType: string;
    position: { x: number; y: number };
    data: any;
  }): Promise<{
    id: string;
    type: string;
  }>;
  
  /**
   * Connect nodes in a flow
   */
  connectNodes(params: {
    flowId: string;
    sourceNodeId: string;
    targetNodeId: string;
    label?: string;
  }): Promise<{
    id: string;
  }>;
  
  /**
   * Execute a flow
   */
  executeFlow(params: {
    flowId: string;
    input?: any;
  }): Promise<{
    output: any;
    executionId: string;
  }>;
  
  /**
   * Get flow execution status
   */
  getFlowExecutionStatus(params: {
    executionId: string;
  }): Promise<{
    status: 'running' | 'completed' | 'failed';
    output?: any;
    error?: string;
  }>;
  
  /**
   * List all flows
   */
  listFlows(): Promise<Array<{
    id: string;
    name: string;
    description?: string;
  }>>;
  
  /**
   * Get flow details
   */
  getFlow(params: {
    flowId: string;
  }): Promise<{
    id: string;
    name: string;
    description?: string;
    nodes: any[];
    edges: any[];
  }>;
}
```

### 2. Create Fluujo Adapter

Next, implement the adapter that connects to Fluujo's API:

```typescript
// adapters/fluujo_adapter.ts
import { FlowOrchestration } from '../interfaces/flow_orchestration';
import axios from 'axios';
import { v4 as uuidv4 } from 'uuid';

export class AgslagFluujoAdapter implements FlowOrchestration {
  private baseUrl: string;
  
  constructor(config: {
    baseUrl: string;
  }) {
    this.baseUrl = config.baseUrl;
  }
  
  /**
   * Create a new flow
   */
  async createFlow(params: {
    name: string;
    description?: string;
  }): Promise<{
    id: string;
    name: string;
  }> {
    try {
      const response = await axios.post(`${this.baseUrl}/api/flow`, {
        action: 'createNewFlow',
        name: params.name,
        description: params.description
      });
      
      return {
        id: response.data.id,
        name: response.data.name
      };
    } catch (error) {
      console.error('[AgslagFluujoAdapter] Error creating flow:', error);
      throw new Error(`Failed to create flow: ${error.message}`);
    }
  }
  
  /**
   * Add a node to a flow
   */
  async addNode(params: {
    flowId: string;
    nodeType: string;
    position: { x: number; y: number };
    data: any;
  }): Promise<{
    id: string;
    type: string;
  }> {
    try {
      // Get the current flow
      const flow = await this.getFlow({ flowId: params.flowId });
      
      // Create a new node
      const nodeId = uuidv4();
      const newNode = {
        id: nodeId,
        type: params.nodeType,
        position: params.position,
        data: {
          ...params.data,
          type: params.nodeType
        }
      };
      
      // Add the node to the flow
      const updatedNodes = [...flow.nodes, newNode];
      
      // Update the flow
      await axios.post(`${this.baseUrl}/api/flow`, {
        action: 'updateFlow',
        id: params.flowId,
        nodes: updatedNodes,
        edges: flow.edges
      });
      
      return {
        id: nodeId,
        type: params.nodeType
      };
    } catch (error) {
      console.error('[AgslagFluujoAdapter] Error adding node:', error);
      throw new Error(`Failed to add node: ${error.message}`);
    }
  }
  
  /**
   * Connect nodes in a flow
   */
  async connectNodes(params: {
    flowId: string;
    sourceNodeId: string;
    targetNodeId: string;
    label?: string;
  }): Promise<{
    id: string;
  }> {
    try {
      // Get the current flow
      const flow = await this.getFlow({ flowId: params.flowId });
      
      // Create a new edge
      const edgeId = uuidv4();
      const newEdge = {
        id: edgeId,
        source: params.sourceNodeId,
        target: params.targetNodeId,
        label: params.label
      };
      
      // Add the edge to the flow
      const updatedEdges = [...flow.edges, newEdge];
      
      // Update the flow
      await axios.post(`${this.baseUrl}/api/flow`, {
        action: 'updateFlow',
        id: params.flowId,
        nodes: flow.nodes,
        edges: updatedEdges
      });
      
      return {
        id: edgeId
      };
    } catch (error) {
      console.error('[AgslagFluujoAdapter] Error connecting nodes:', error);
      throw new Error(`Failed to connect nodes: ${error.message}`);
    }
  }
  
  /**
   * Execute a flow
   */
  async executeFlow(params: {
    flowId: string;
    input?: any;
  }): Promise<{
    output: any;
    executionId: string;
  }> {
    try {
      const response = await axios.post(`${this.baseUrl}/api/flow`, {
        action: 'executeFlow',
        id: params.flowId,
        input: params.input
      });
      
      return {
        output: response.data.output,
        executionId: response.data.executionId
      };
    } catch (error) {
      console.error('[AgslagFluujoAdapter] Error executing flow:', error);
      throw new Error(`Failed to execute flow: ${error.message}`);
    }
  }
  
  /**
   * Get flow execution status
   */
  async getFlowExecutionStatus(params: {
    executionId: string;
  }): Promise<{
    status: 'running' | 'completed' | 'failed';
    output?: any;
    error?: string;
  }> {
    try {
      const response = await axios.get(`${this.baseUrl}/api/flow/execution`, {
        params: {
          executionId: params.executionId
        }
      });
      
      return {
        status: response.data.status,
        output: response.data.output,
        error: response.data.error
      };
    } catch (error) {
      console.error('[AgslagFluujoAdapter] Error getting flow execution status:', error);
      throw new Error(`Failed to get flow execution status: ${error.message}`);
    }
  }
  
  /**
   * List all flows
   */
  async listFlows(): Promise<Array<{
    id: string;
    name: string;
    description?: string;
  }>> {
    try {
      const response = await axios.get(`${this.baseUrl}/api/flow`, {
        params: {
          action: 'listFlows'
        }
      });
      
      return response.data.flows;
    } catch (error) {
      console.error('[AgslagFluujoAdapter] Error listing flows:', error);
      throw new Error(`Failed to list flows: ${error.message}`);
    }
  }
  
  /**
   * Get flow details
   */
  async getFlow(params: {
    flowId: string;
  }): Promise<{
    id: string;
    name: string;
    description?: string;
    nodes: any[];
    edges: any[];
  }> {
    try {
      const response = await axios.get(`${this.baseUrl}/api/flow`, {
        params: {
          action: 'getFlow',
          id: params.flowId
        }
      });
      
      return response.data.flow;
    } catch (error) {
      console.error('[AgslagFluujoAdapter] Error getting flow:', error);
      throw new Error(`Failed to get flow: ${error.message}`);
    }
  }
}
```

### 3. Extend AGSLAG MCP Integration

Update the AGSLAG MCP Integration class to include the Fluujo adapter:

```typescript
// integration/agslag_mcp_integration.ts
import { AgslagFluujoAdapter } from '../adapters/fluujo_adapter';

export class AgslagMcpIntegration {
  // Existing code...
  
  private fluujo: AgslagFluujoAdapter;
  
  constructor(config: {
    // Existing config...
    fluujo?: {
      baseUrl: string;
    }
  }) {
    // Existing initialization...
    
    // Initialize Fluujo adapter if config is provided
    if (config.fluujo) {
      this.fluujo = new AgslagFluujoAdapter(config.fluujo);
    }
  }
  
  // New methods for flow orchestration
  
  /**
   * Create a new workflow
   */
  async createWorkflow(name: string, description?: string) {
    if (!this.fluujo) {
      throw new Error('Fluujo adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Creating workflow: ${name}`);
    
    return await this.fluujo.createFlow({
      name,
      description
    });
  }
  
  /**
   * Add a node to a workflow
   */
  async addWorkflowNode(workflowId: string, nodeType: string, position: { x: number; y: number }, data: any) {
    if (!this.fluujo) {
      throw new Error('Fluujo adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Adding node to workflow: ${workflowId}`);
    
    return await this.fluujo.addNode({
      flowId: workflowId,
      nodeType,
      position,
      data
    });
  }
  
  /**
   * Connect nodes in a workflow
   */
  async connectWorkflowNodes(workflowId: string, sourceNodeId: string, targetNodeId: string, label?: string) {
    if (!this.fluujo) {
      throw new Error('Fluujo adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Connecting nodes in workflow: ${workflowId}`);
    
    return await this.fluujo.connectNodes({
      flowId: workflowId,
      sourceNodeId,
      targetNodeId,
      label
    });
  }
  
  /**
   * Execute a workflow
   */
  async executeWorkflow(workflowId: string, input?: any) {
    if (!this.fluujo) {
      throw new Error('Fluujo adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Executing workflow: ${workflowId}`);
    
    return await this.fluujo.executeFlow({
      flowId: workflowId,
      input
    });
  }
  
  /**
   * Get workflow execution status
   */
  async getWorkflowExecutionStatus(executionId: string) {
    if (!this.fluujo) {
      throw new Error('Fluujo adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Getting workflow execution status: ${executionId}`);
    
    return await this.fluujo.getFlowExecutionStatus({
      executionId
    });
  }
  
  /**
   * List all workflows
   */
  async listWorkflows() {
    if (!this.fluujo) {
      throw new Error('Fluujo adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Listing workflows`);
    
    return await this.fluujo.listFlows();
  }
  
  /**
   * Get workflow details
   */
  async getWorkflow(workflowId: string) {
    if (!this.fluujo) {
      throw new Error('Fluujo adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Getting workflow: ${workflowId}`);
    
    return await this.fluujo.getFlow({
      flowId: workflowId
    });
  }
}
```

### 4. Create Example Usage

Create an example file that demonstrates how to use the Fluujo integration:

```typescript
// examples/fluujo_example.ts
import { AgslagMcpIntegration } from '../integration/agslag_mcp_integration';

async function fluujoExample() {
  // Initialize AGSLAG MCP Integration with Fluujo config
  const integration = new AgslagMcpIntegration({
    fluujo: {
      baseUrl: 'http://localhost:3000' // Adjust based on your Fluujo setup
    }
  });
  
  try {
    // Create a new workflow
    console.log('Creating a new workflow...');
    const workflow = await integration.createWorkflow(
      'Research and Development Workflow',
      'A workflow for conducting research and developing code'
    );
    console.log('Workflow created:', workflow);
    
    // Add a start node
    console.log('Adding a start node...');
    const startNode = await integration.addWorkflowNode(
      workflow.id,
      'start',
      { x: 100, y: 100 },
      { label: 'Start' }
    );
    console.log('Start node added:', startNode);
    
    // Add a research node
    console.log('Adding a research node...');
    const researchNode = await integration.addWorkflowNode(
      workflow.id,
      'mcp',
      { x: 300, y: 100 },
      { 
        label: 'Research',
        properties: {
          mcpServer: 'mcp-server-fetch',
          promptTemplate: 'Research the following topic: {{topic}}'
        }
      }
    );
    console.log('Research node added:', researchNode);
    
    // Add a development node
    console.log('Adding a development node...');
    const developmentNode = await integration.addWorkflowNode(
      workflow.id,
      'mcp',
      { x: 500, y: 100 },
      { 
        label: 'Development',
        properties: {
          mcpServer: 'mcp-server-code',
          promptTemplate: 'Develop code based on the research: {{research}}'
        }
      }
    );
    console.log('Development node added:', developmentNode);
    
    // Connect the nodes
    console.log('Connecting nodes...');
    await integration.connectWorkflowNodes(
      workflow.id,
      startNode.id,
      researchNode.id,
      'Start -> Research'
    );
    await integration.connectWorkflowNodes(
      workflow.id,
      researchNode.id,
      developmentNode.id,
      'Research -> Development'
    );
    console.log('Nodes connected');
    
    // Execute the workflow
    console.log('Executing workflow...');
    const execution = await integration.executeWorkflow(
      workflow.id,
      { topic: 'Artificial Intelligence' }
    );
    console.log('Workflow execution started:', execution);
    
    // Poll for execution status
    let status = await integration.getWorkflowExecutionStatus(execution.executionId);
    console.log('Initial status:', status);
    
    while (status.status === 'running') {
      console.log('Workflow still running, waiting...');
      await new Promise(resolve => setTimeout(resolve, 5000)); // Wait 5 seconds
      status = await integration.getWorkflowExecutionStatus(execution.executionId);
      console.log('Updated status:', status);
    }
    
    console.log('Workflow execution completed with status:', status);
    
    if (status.status === 'completed') {
      console.log('Workflow output:', status.output);
    } else {
      console.error('Workflow failed with error:', status.error);
    }
  } catch (error) {
    console.error('Error in Fluujo example:', error);
  }
}

// Run the example
fluujoExample().catch(console.error);
```

## Configuration

To configure the Fluujo integration, you need to set up the following:

1. **Fluujo Server**: Ensure the Fluujo server is running and accessible
2. **Base URL**: Configure the base URL for the Fluujo API
3. **Authentication**: Set up any required authentication for the Fluujo API

See the [Configuration Management Guide](./configuration_management_guide.md) for more details.

## Testing

To test the Fluujo integration:

1. Run the Fluujo server
2. Execute the example script: `npx ts-node examples/fluujo_example.ts`
3. Verify that the workflow is created, nodes are added, and the workflow executes successfully

## Troubleshooting

Common issues and solutions:

1. **Connection Errors**: Ensure the Fluujo server is running and the base URL is correct
2. **Authentication Errors**: Check that any required authentication is properly configured
3. **API Errors**: Verify that the Fluujo API endpoints match the ones used in the adapter

## Next Steps

After integrating Fluujo, consider:

1. Creating more complex workflows that combine Fluujo with other MCP capabilities
2. Developing a UI for visualizing and managing workflows
3. Implementing error handling and recovery mechanisms for workflow execution
4. Creating templates for common workflow patterns
