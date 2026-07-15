# Phase 2: Cloud Environment Setup (Weeks 5-8)

This document details the implementation of the cloud environment components for the MCP agentic architecture.

## 2.1 Headless MCP Client Implementation

The headless MCP client enables programmatic interaction with MCP servers without requiring a UI.

### Implementation Example

```javascript
// mcp-client.js
const { McpClient } = require('@anthropic-ai/mcp-client');
const axios = require('axios');
const fs = require('fs/promises');

class HeadlessMcpClient {
  constructor(config) {
    this.config = config;
    this.client = new McpClient({
      apiKey: config.apiKey,
      modelId: config.modelId || 'claude-3-sonnet-20240229'
    });
  }
  
  async initialize() {
    // Connect to MCP servers
    await this.client.connect();
    
    // Register tools
    await this.client.registerTools(this.config.tools || []);
  }
  
  async executeTask(task) {
    const response = await this.client.sendMessage({
      content: task,
      role: 'user'
    });
    
    return this.processResponse(response);
  }
  
  async processResponse(response) {
    // Extract file modifications, commands, and other actions
    const actions = [];
    
    // Process file changes
    if (response.fileChanges) {
      for (const change of response.fileChanges) {
        await fs.writeFile(change.path, change.content);
        actions.push({
          type: 'file_modified',
          path: change.path
        });
      }
    }
    
    // Process git actions
    if (response.gitActions) {
      for (const action of response.gitActions) {
        if (action.type === 'commit') {
          const { stdout } = await execCommand(`git add ${action.files.join(' ')}`);
          const { stdout: commitOut } = await execCommand(`git commit -m "${action.message}"`);
          actions.push({
            type: 'git_commit',
            message: action.message,
            files: action.files
          });
        }
      }
    }
    
    return {
      message: response.content,
      actions
    };
  }
}

module.exports = HeadlessMcpClient;
```

### Technical Considerations

- **Client Configuration**: Flexible configuration options for different model types
- **Tool Registration**: Dynamic tool registration based on agent role
- **Response Processing**: Handling of file changes, Git actions, and other operations
- **Error Handling**: Robust error handling for MCP client operations
- **State Management**: Maintaining client state during task execution

## 2.2 Remote MCP Server Deployment

Deploying MCP servers on cloud platforms enables scalable, accessible agents.

### Server Implementation

```javascript
// remote-mcp-server.js
const { McpServer } = require('@anthropic-ai/mcp-server');
const { CloudflareAuth } = require('@anthropic-ai/mcp-server-auth');

// Create MCP server
const server = new McpServer({
  name: 'Remote Agent Server',
  version: '1.0.0',
  auth: new CloudflareAuth({
    clientId: process.env.CLIENT_ID,
    clientSecret: process.env.CLIENT_SECRET
  }),
  cors: {
    origins: ['https://your-app-domain.com']
  }
});

// Register tools
server.registerTool({
  name: 'get_file_contents',
  description: 'Get the contents of a file from the repository',
  parameters: {
    type: 'object',
    properties: {
      path: {
        type: 'string',
        description: 'Path to the file relative to the repository root'
      }
    },
    required: ['path']
  },
  handler: async ({ path }) => {
    // Implementation to access file contents
    // ...
  }
});

// Add more tools
// ...

// Start the server
server.listen(process.env.PORT || 3000);
```

### Cloudflare Workers Configuration

```toml
# wrangler.toml
name = "mcp-remote-server"
type = "webpack"
account_id = "your-account-id"
workers_dev = true
route = "mcp-server.your-domain.com/*"
```

### Deployment Considerations

- **Authentication**: Secure authentication for MCP server access
- **CORS Configuration**: Proper CORS setup for web client access
- **Tool Registration**: Implementation of necessary tools for agent operation
- **Error Handling**: Robust error handling for server operations
- **Scalability**: Configuration for handling multiple concurrent requests

## 2.3 Containerized Agent Implementation

Containerized cloud agents provide consistent, isolated environments for agent execution.

### Dockerfile Implementation

```dockerfile
FROM node:16-slim

# Install Git and required dependencies
RUN apt-get update && apt-get install -y git

# Set up working directory
WORKDIR /app

# Copy package files and install dependencies
COPY package*.json ./
RUN npm install

# Copy MCP client code
COPY src/ ./src/

# Environment variables
ENV MODEL_ID="claude-3-sonnet-20240229"
ENV API_KEY=""
ENV REPO_URL=""
ENV TEAM_BRANCH=""
ENV TASK_FILE="/app/task/task.json"

# Start script
CMD ["node", "src/index.js"]
```

### Agent Implementation Considerations

- **Base Image**: Lightweight Node.js image for efficient containers
- **Dependencies**: Minimal dependencies for reduced container size
- **Configuration**: Flexible configuration through environment variables
- **Task Processing**: Ability to process tasks from file or environment
- **Git Integration**: Built-in Git support for repository operations

## Next Steps

After implementing Phase 2, the following components should be available:

1. Headless MCP client for programmatic agent interactions
2. Remote MCP server deployment on Cloudflare or similar platform
3. Containerized cloud agent implementation
4. Basic cloud-based task execution capabilities

These components form the foundation for the cloud environment setup, which will be integrated with the local environment components in subsequent phases.
