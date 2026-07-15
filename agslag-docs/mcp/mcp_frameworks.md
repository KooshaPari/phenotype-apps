# MCP Frameworks: LiteMCP and ws-mcp

## Introduction

This document details the selected MCP frameworks for our Large Scale Agent Development platform, focusing on LiteMCP as our primary framework and ws-mcp as a WebSocket transport adapter. These technologies are critical components in our system architecture, enabling efficient agent communication and web client integration.

## LiteMCP Overview

LiteMCP is a lightweight, elegant TypeScript framework for building Model Context Protocol (MCP) servers. It provides a clean API for defining tools, resources, and prompts, with built-in support for type safety and multiple transport mechanisms.

### Key Features

- **Simple and Intuitive API**: Clean, declarative approach to defining MCP components
- **Type Safety**: Built-in integration with Zod for schema validation
- **Multiple Transport Support**: Native support for stdio and SSE transports
- **Development Tools**: Utilities for debugging and inspecting MCP servers
- **Resource Management**: Clean API for exposing and accessing resources
- **Prompt Templates**: Support for reusable prompt templates
- **Comprehensive Logging**: Configurable logging capabilities

### Role in System Architecture

Within our Large Scale Agent Development platform, LiteMCP serves as the primary framework for:

1. **Specialized Agent Servers**: Creating purpose-specific servers for search, file management, communications, etc.
2. **Standardized Communication**: Ensuring consistent communication patterns across different agent types
3. **Type-Safe Interactions**: Validating input parameters for all tool invocations
4. **Lightweight Implementation**: Maintaining high performance with minimal overhead

## ws-mcp Integration

ws-mcp is a utility that wraps MCP servers with a WebSocket interface, enabling bidirectional, real-time communication between web clients and MCP servers.

### Key Features

- **WebSocket Transport**: Adds WebSocket support to stdio-based MCP servers
- **Multiple Server Management**: Manages multiple MCP servers through a single interface
- **Configuration Flexibility**: Supports custom configuration through JSON or environment variables
- **Web Client Integration**: Facilitates integration with browser-based interfaces
- **Standard Protocol Compliance**: Maintains compatibility with the MCP specification

### Role in System Architecture

ws-mcp enhances our platform architecture by:

1. **Expanding Transport Options**: Complementing the native stdio and SSE transport mechanisms
2. **Web Client Support**: Enabling browser-based clients to interact with our agent ecosystem
3. **Multiple Server Orchestration**: Managing various specialized agent servers through a unified interface
4. **Real-time Communication**: Supporting bidirectional, low-latency interactions for time-sensitive operations

## Implementation Guide

### Setting Up LiteMCP

1. **Installation**:
   ```bash
   npm install litemcp zod typescript ts-node @types/node --save
   npm install --save-dev nodemon
   ```

2. **Basic Server Structure**:
   ```typescript
   import { LiteMCP } from 'litemcp';
   import { z } from 'zod';

   // Create server instance
   const server = new LiteMCP('agent-server', '1.0.0');

   // Add tool example
   server.addTool({
     name: 'agent_tool',
     description: 'Example agent tool',
     parameters: z.object({
       param1: z.string().describe('Parameter description'),
     }),
     execute: async (args) => {
       // Tool implementation
       return `Processed: ${args.param1}`;
     },
   });

   // Add resource example
   server.addResource({
     uri: 'resource://agent/capabilities',
     name: 'Agent Capabilities',
     mimeType: 'application/json',
     async load() {
       // Resource implementation
       return {
         text: JSON.stringify({
           capabilities: ['capability1', 'capability2']
         }),
       };
     },
   });

   // Start the server
   server.start();
   ```

### Advanced LiteMCP Implementation

#### Custom Logger Integration

For enterprise-scale deployments, integrate a robust logging system:

```typescript
import { LiteMCP } from 'litemcp';
import winston from 'winston';

// Configure Winston logger
const logger = winston.createLogger({
  level: 'info',
  format: winston.format.combine(
    winston.format.timestamp(),
    winston.format.json()
  ),
  transports: [
    new winston.transports.File({ filename: 'error.log', level: 'error' }),
    new winston.transports.File({ filename: 'combined.log' })
  ]
});

// Pass custom logger to LiteMCP
const server = new LiteMCP('agent-server', '1.0.0', {
  logger: {
    debug: (message) => logger.debug(message),
    info: (message) => logger.info(message),
    warn: (message) => logger.warn(message),
    error: (message) => logger.error(message)
  }
});
```

#### Resource Caching Strategy

Implement efficient resource caching for improved performance:

```typescript
// Simple in-memory cache
const resourceCache = new Map();

server.addResource({
  uri: 'resource://agent/capabilities',
  name: 'Agent Capabilities',
  mimeType: 'application/json',
  async load() {
    // Check cache first
    const cacheKey = 'agent-capabilities';
    if (resourceCache.has(cacheKey)) {
      const { data, timestamp } = resourceCache.get(cacheKey);
      // Cache valid for 5 minutes
      if (Date.now() - timestamp < 5 * 60 * 1000) {
        return { text: data };
      }
    }
    
    // Fetch fresh data
    const data = await fetchAgentCapabilities();
    
    // Update cache
    resourceCache.set(cacheKey, {
      data: JSON.stringify(data),
      timestamp: Date.now()
    });
    
    return { text: JSON.stringify(data) };
  }
});
```

#### Modular Tool Organization

Structure your codebase for maintainability and scalability:

```typescript
// tools/communication.ts
import { z } from 'zod';

export const communicationTools = [
  {
    name: 'send_message',
    description: 'Send a message to another agent',
    parameters: z.object({
      sender_id: z.string().describe('Your unique Agent ID'),
      recipient_id: z.string().describe('The unique Agent ID of the recipient'),
      content: z.string().describe('The textual content of the message')
    }),
    execute: async (args) => {
      // Implementation
    }
  },
  // Other communication tools
];

// index.ts
import { LiteMCP } from 'litemcp';
import { communicationTools } from './tools/communication';
import { collaborationTools } from './tools/collaboration';
import { workflowTools } from './tools/workflows';

const server = new LiteMCP('agent-server', '1.0.0');

// Register all tools
for (const tool of [...communicationTools, ...collaborationTools, ...workflowTools]) {
  server.addTool(tool);
}
```

### Integrating ws-mcp

1. **Configuration**:
   Create a configuration file (e.g., `ws-mcp-config.json`):
   ```json
   {
     "servers": [
       {
         "name": "agent-server",
         "command": "node",
         "args": ["dist/agent-server.js"],
         "path": "/agent"
       },
       {
         "name": "communication-server",
         "command": "node",
         "args": ["dist/communication-server.js"],
         "path": "/communication"
       }
     ],
     "port": 8080
   }
   ```

2. **Running ws-mcp**:
   ```bash
   npx ws-mcp -c ws-mcp-config.json
   ```

3. **Client Integration**:
   ```javascript
   // Browser-based client
   const socket = new WebSocket('ws://localhost:8080/agent');

   socket.onopen = () => {
     // MCP initialize message
     socket.send(JSON.stringify({
       type: "initialize",
       version: "1.0"
     }));
   };

   socket.onmessage = (event) => {
     const message = JSON.parse(event.data);
     // Handle message
   };

   // Example tool invocation
   function invokeAgentTool(param1) {
     socket.send(JSON.stringify({
       type: "invoke",
       invoke: {
         tool: "agent_tool",
         parameters: {
           param1: param1
         }
       }
     }));
   }
   ```

### Advanced ws-mcp Implementation

#### Production Deployment Architecture

For enterprise-scale deployments, implement a robust architecture:

```
┌─────────────────┐     ┌───────────────┐    ┌────────────────┐
│                 │     │               │    │                │
│  Web Clients    │◄────┤  Load Balancer├────┤  ws-mcp Server │
│                 │     │               │    │                │
└─────────────────┘     └───────────────┘    └────────┬───────┘
                                                     │
                                                     ▼
                                      ┌─────────────────────────┐
                                      │                         │
                                      │  LiteMCP Agent Servers  │
                                      │                         │
                                      └─────────────────────────┘
```

#### Secure WebSocket Configuration

Implement proper security for production:

```javascript
// Secure ws-mcp configuration
const config = {
  "servers": [
    // Server configurations
  ],
  "port": 8443,
  "ssl": {
    "cert": "/path/to/cert.pem",
    "key": "/path/to/key.pem"
  },
  "websocket": {
    "perMessageDeflate": true,
    "maxPayload": 5242880, // 5MB
    "clientTracking": true
  },
  "auth": {
    "type": "bearer",
    "validateToken": "/path/to/validate-token.js"
  }
};
```

#### Client Implementation with Authentication and Reconnection

Create robust client implementations:

```javascript
class MCPWebSocketClient {
  constructor(url, authToken) {
    this.url = url;
    this.authToken = authToken;
    this.socket = null;
    this.reconnectAttempts = 0;
    this.maxReconnectAttempts = 5;
    this.reconnectDelay = 1000;
    this.pendingRequests = new Map();
    this.messageHandlers = new Map();
    this.connected = false;
    this.connecting = false;
    this.requestId = 1;
  }

  connect() {
    if (this.connected || this.connecting) return;
    
    this.connecting = true;
    this.socket = new WebSocket(this.url);
    
    this.socket.onopen = () => {
      console.log('Connected to MCP server');
      this.connected = true;
      this.connecting = false;
      this.reconnectAttempts = 0;
      
      // Initialize connection with authentication
      this.socket.send(JSON.stringify({
        type: "initialize",
        version: "1.0",
        auth: {
          type: "bearer",
          token: this.authToken
        }
      }));
    };
    
    this.socket.onmessage = (event) => {
      const message = JSON.parse(event.data);
      this.handleMessage(message);
    };
    
    this.socket.onclose = () => {
      this.connected = false;
      this.connecting = false;
      this.handleReconnect();
    };
    
    this.socket.onerror = (error) => {
      console.error('WebSocket error:', error);
    };
  }
  
  // Reconnection logic
  handleReconnect() {
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      console.error('Max reconnection attempts reached');
      return;
    }
    
    this.reconnectAttempts++;
    const delay = this.reconnectDelay * Math.pow(1.5, this.reconnectAttempts - 1);
    
    console.log(`Attempting to reconnect in ${delay}ms (attempt ${this.reconnectAttempts})`);
    setTimeout(() => this.connect(), delay);
  }
  
  // Tool invocation with Promise-based API
  invokeTool(toolName, parameters) {
    return new Promise((resolve, reject) => {
      if (!this.connected) {
        reject(new Error('Not connected to MCP server'));
        return;
      }
      
      const id = `req_${this.requestId++}`;
      
      this.pendingRequests.set(id, { resolve, reject });
      
      this.socket.send(JSON.stringify({
        type: "invoke",
        id: id,
        invoke: {
          tool: toolName,
          parameters: parameters
        }
      }));
    });
  }
  
  // Message handling
  handleMessage(message) {
    if (message.type === "response" && message.id && this.pendingRequests.has(message.id)) {
      const { resolve, reject } = this.pendingRequests.get(message.id);
      this.pendingRequests.delete(message.id);
      
      if (message.error) {
        reject(new Error(message.error.message));
      } else {
        resolve(message.response);
      }
    } else if (this.messageHandlers.has(message.type)) {
      this.messageHandlers.get(message.type)(message);
    }
  }
  
  // Register custom message handlers
  onMessageType(type, handler) {
    this.messageHandlers.set(type, handler);
  }
  
  disconnect() {
    if (this.socket) {
      this.socket.close();
    }
  }
}

// Usage example
const client = new MCPWebSocketClient('wss://mcp-server.example.com/agent', 'auth-token-here');
client.connect();

client.invokeTool('agent_tool', { param1: 'value' })
  .then(response => {
    console.log('Tool response:', response);
  })
  .catch(error => {
    console.error('Tool error:', error);
  });
```

## Best Practices

### LiteMCP Development

1. **Modular Tool Organization**: Group related tools into separate files based on functionality
2. **Comprehensive Parameter Validation**: Use Zod to create thorough parameter validation
3. **Informative Descriptions**: Provide detailed descriptions for tools, parameters, and resources
4. **Error Handling**: Implement proper error handling and return informative error messages
5. **Resource Caching**: Cache resource data when appropriate to improve performance

### ws-mcp Deployment

1. **Security Considerations**: Implement authentication for WebSocket connections
2. **Load Balancing**: Consider load balancing for high-traffic deployments
3. **Connection Management**: Implement proper connection handling and cleanup
4. **Monitoring**: Set up monitoring for WebSocket connections and server health
5. **Error Recovery**: Implement reconnection logic in clients

## Integration with Existing MCP Codebase

Our current MCP implementation already includes the following components:

1. **Cross-Agent Communication Tools**: `send_message`, `get_messages`, `create_thread`, etc.
2. **Collaboration Tools**: Task decomposition, assignment, and result merging
3. **Agent Management**: Registration, deregistration, and status management
4. **Workflow Engine**: Definition and execution of multi-step processes

LiteMCP and ws-mcp will enhance this existing infrastructure by:

1. Providing a more type-safe, elegant API for defining new agent tools and resources
2. Enabling WebSocket-based communication for web client integration
3. Supporting real-time, bidirectional communication patterns
4. Facilitating a more modular, maintainable agent implementation approach

## Recommended Approach for Phase Integration

As we progress through the implementation roadmap, we recommend:

### Phase 1-2: Prototype & MVP
- Use LiteMCP for rapid development of specialized agent servers
- Focus on stdio transport for simplicity during early development

### Phase 3-4: Enhanced & Enterprise Ready
- Integrate ws-mcp to enable web client interactions
- Expand transport options to include WebSockets for real-time communication
- Implement proper authentication and security for network-exposed servers

### Phase 5-6: Advanced & Enterprise Scale
- Scale ws-mcp deployments with load balancing
- Implement advanced monitoring and recovery mechanisms
- Optimize WebSocket connection management for large-scale deployments

## Conclusion

LiteMCP and ws-mcp provide powerful, flexible frameworks for building and connecting MCP servers in our Large Scale Agent Development platform. LiteMCP offers an elegant, type-safe API for building agent capabilities, while ws-mcp extends these capabilities to web clients through WebSocket transport. Together, they form a robust foundation for our multi-agent system architecture.
