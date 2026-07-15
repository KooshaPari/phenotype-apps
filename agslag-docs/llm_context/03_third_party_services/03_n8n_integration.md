# N8N Integration Guide

## Overview
N8N is a workflow automation platform that connects different services and APIs:
- Node-based workflow editor
- 300+ pre-built integrations
- Custom JavaScript code
- Webhook triggers
- Error handling and retries

## Deployment Options
- Self-hosted (Docker, npm)
- N8N Cloud
- Custom deployment (AWS, GCP, Azure)

## Core Concepts

### Workflows
- Sequence of connected nodes
- Trigger node starts workflow
- Action nodes perform operations
- Flow control nodes manage execution path

### Nodes
- Trigger nodes: HTTP Request, Webhook, Schedule
- Action nodes: HTTP Request, Function, API-specific nodes
- Flow control: IF, Switch, Merge, Split

### Data
- JSON-based data flow
- Expression syntax with $
- Binary data handling for files
- Variable storage and retrieval

## Integration with MCP

### Webhook Integration
```javascript
// N8N HTTP Request node configuration
{
  "url": "https://your-mcp-server.com/api/workflow",
  "method": "POST",
  "authentication": "headerAuth",
  "headerParameters": {
    "Authorization": "Bearer {{$env.MCP_API_KEY}}"
  },
  "bodyParameters": {
    "workflow_id": "{{$json.workflowId}}",
    "triggering_agent_id": "{{$json.agentId}}"
  }
}
```

### Custom Function Node
```javascript
// Process MCP agent responses
function processAgentResponse(items) {
  for (const item of items) {
    // Extract agent response
    const response = item.json.response;
    
    // Process based on status
    if (response.status === "success") {
      // Handle successful response
      item.json.processedData = response.data;
    } else {
      // Handle error
      item.json.error = response.error;
    }
  }
  return items;
}
```

## Workflow Templates

### Agent Coordination Workflow
1. HTTP Webhook (receive agent task)
2. Switch (based on task type)
3. HTTP Request (call appropriate MCP endpoint)
4. Function (process response)
5. HTTP Request (return result to MCP)

### Error Handling Workflow
1. Catch all workflow errors
2. Function (format error details)
3. HTTP Request (notify MCP of error)
4. If (critical error) → Send notification