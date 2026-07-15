# Implementing Custom MCP Servers

## Introduction

This guide provides a comprehensive overview of how to design, implement, and deploy custom Model Context Protocol (MCP) servers. Custom MCP servers allow you to extend AI capabilities with specialized tools and data sources tailored to your specific use cases.

## Why Build a Custom MCP Server?

1. **Domain-Specific Functionality**: Create tools optimized for your industry or use case
2. **Proprietary Data Access**: Connect AI models to your internal data sources
3. **Custom Workflows**: Implement specialized business processes
4. **Enhanced Security**: Apply your organization's security policies
5. **Integration**: Connect with existing systems and services

## Planning Your MCP Server

### 1. Define Your Use Case

Start by clearly defining what your MCP server will do:

- What problem will it solve?
- Who are the target users?
- What specific capabilities will it provide?
- What data sources will it access?

### 2. Design Your Resources

Resources represent the data your server will expose:

- **Structured Data**: Databases, APIs, file formats
- **Unstructured Data**: Documents, images, audio
- **Hybrid Data**: Mixed structured/unstructured sources

For each resource, define:
- Schema (fields, types, relationships)
- Access patterns (read, write, query)
- Update frequency (static, dynamic, real-time)

### 3. Design Your Tools

Tools are the functions your server will provide:

- **CRUD Operations**: Create, read, update, delete
- **Search & Query**: Find and filter data
- **Analysis**: Process and interpret data
- **Actions**: Perform operations on external systems

For each tool, define:
- Parameters (names, types, validation rules)
- Return values (format, structure)
- Error conditions and handling
- Performance characteristics

## Development Frameworks

Choose a framework based on your language preference and requirements:

### Official SDKs

1. **TypeScript SDK**
   - GitHub: https://github.com/anthropics/anthropic-sdk-typescript
   - Features: Complete implementation, strong typing
   - Best for: Production-grade TypeScript/JavaScript servers

2. **Python SDK**
   - GitHub: https://github.com/anthropics/anthropic-sdk-python
   - Features: Pythonic interface, easy integration with data science tools
   - Best for: Data-intensive applications, scientific computing

### Community Frameworks

1. **FastMCP** (TypeScript)
   - Features: High-level abstractions, rapid development
   - Best for: Quick prototyping, simple servers

2. **LiteMCP** (JavaScript/TypeScript)
   - Features: Lightweight, minimal dependencies
   - Best for: Resource-constrained environments, simple tools

3. **Quarkus MCP Server** (Java)
   - Features: Enterprise-grade, high performance
   - Best for: Java environments, enterprise integration

4. **Foxy Contexts** (Golang)
   - Features: High performance, low resource usage
   - Best for: Performance-critical applications

## Implementation Steps

### 1. Set Up Your Project

#### Using TypeScript SDK

```bash
# Create a new project
mkdir my-mcp-server
cd my-mcp-server
npm init -y

# Install dependencies
npm install @anthropic-ai/sdk
npm install typescript ts-node @types/node --save-dev

# Initialize TypeScript
npx tsc --init
```

#### Using Python SDK

```bash
# Create a new project
mkdir my-mcp-server
cd my-mcp-server

# Set up virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install anthropic
```

### 2. Define Your Resources

Resources describe the data your server can access.

#### TypeScript Example

```typescript
// resources.ts
import { ResourceDefinition } from '@anthropic-ai/sdk';

export const userResource: ResourceDefinition = {
  name: 'user',
  description: 'User account information',
  schema: {
    type: 'object',
    properties: {
      id: {
        type: 'string',
        description: 'Unique identifier for the user'
      },
      name: {
        type: 'string',
        description: 'User\'s full name'
      },
      email: {
        type: 'string',
        description: 'User\'s email address'
      },
      role: {
        type: 'string',
        enum: ['admin', 'user', 'guest'],
        description: 'User\'s role in the system'
      },
      createdAt: {
        type: 'string',
        format: 'date-time',
        description: 'When the user account was created'
      }
    },
    required: ['id', 'name', 'email', 'role']
  }
};
```

#### Python Example

```python
# resources.py
from anthropic import ResourceDefinition

user_resource = ResourceDefinition(
    name="user",
    description="User account information",
    schema={
        "type": "object",
        "properties": {
            "id": {
                "type": "string",
                "description": "Unique identifier for the user"
            },
            "name": {
                "type": "string",
                "description": "User's full name"
            },
            "email": {
                "type": "string",
                "description": "User's email address"
            },
            "role": {
                "type": "string",
                "enum": ["admin", "user", "guest"],
                "description": "User's role in the system"
            },
            "createdAt": {
                "type": "string",
                "format": "date-time",
                "description": "When the user account was created"
            }
        },
        "required": ["id", "name", "email", "role"]
    }
)
```

### 3. Implement Your Tools

Tools are the functions that operate on your resources.

#### TypeScript Example

```typescript
// tools.ts
import { ToolDefinition } from '@anthropic-ai/sdk';
import { getUserById, searchUsers, createUser, updateUser } from './userService';

export const getUserTool: ToolDefinition = {
  name: 'get_user',
  description: 'Get a user by their ID',
  parameters: {
    type: 'object',
    properties: {
      userId: {
        type: 'string',
        description: 'The ID of the user to retrieve'
      }
    },
    required: ['userId']
  },
  handler: async ({ userId }) => {
    try {
      const user = await getUserById(userId);
      return { user };
    } catch (error) {
      throw new Error(`Failed to get user: ${error.message}`);
    }
  }
};

export const searchUsersTool: ToolDefinition = {
  name: 'search_users',
  description: 'Search for users by name or email',
  parameters: {
    type: 'object',
    properties: {
      query: {
        type: 'string',
        description: 'Search query (name or email)'
      },
      limit: {
        type: 'integer',
        description: 'Maximum number of results to return',
        default: 10
      }
    },
    required: ['query']
  },
  handler: async ({ query, limit }) => {
    try {
      const users = await searchUsers(query, limit);
      return { users };
    } catch (error) {
      throw new Error(`Failed to search users: ${error.message}`);
    }
  }
};
```

#### Python Example

```python
# tools.py
from anthropic import ToolDefinition
from user_service import get_user_by_id, search_users, create_user, update_user

def get_user_handler(params):
    try:
        user = get_user_by_id(params["userId"])
        return {"user": user}
    except Exception as e:
        raise Exception(f"Failed to get user: {str(e)}")

get_user_tool = ToolDefinition(
    name="get_user",
    description="Get a user by their ID",
    parameters={
        "type": "object",
        "properties": {
            "userId": {
                "type": "string",
                "description": "The ID of the user to retrieve"
            }
        },
        "required": ["userId"]
    },
    handler=get_user_handler
)

def search_users_handler(params):
    try:
        users = search_users(params["query"], params.get("limit", 10))
        return {"users": users}
    except Exception as e:
        raise Exception(f"Failed to search users: {str(e)}")

search_users_tool = ToolDefinition(
    name="search_users",
    description="Search for users by name or email",
    parameters={
        "type": "object",
        "properties": {
            "query": {
                "type": "string",
                "description": "Search query (name or email)"
            },
            "limit": {
                "type": "integer",
                "description": "Maximum number of results to return",
                "default": 10
            }
        },
        "required": ["query"]
    },
    handler=search_users_handler
)
```

### 4. Create the Server

Combine resources and tools into a complete MCP server.

#### TypeScript Example

```typescript
// server.ts
import { MCPServer } from '@anthropic-ai/sdk';
import { userResource } from './resources';
import { getUserTool, searchUsersTool, createUserTool, updateUserTool } from './tools';

const server = new MCPServer({
  resources: [userResource],
  tools: [getUserTool, searchUsersTool, createUserTool, updateUserTool],
  options: {
    logLevel: 'info',
    port: 3000
  }
});

server.start().then(() => {
  console.log('MCP Server started on port 3000');
}).catch(error => {
  console.error('Failed to start MCP Server:', error);
});
```

#### Python Example

```python
# server.py
from anthropic import MCPServer
from resources import user_resource
from tools import get_user_tool, search_users_tool, create_user_tool, update_user_tool

server = MCPServer(
    resources=[user_resource],
    tools=[get_user_tool, search_users_tool, create_user_tool, update_user_tool],
    options={
        "log_level": "info",
        "port": 3000
    }
)

if __name__ == "__main__":
    try:
        server.start()
        print("MCP Server started on port 3000")
    except Exception as e:
        print(f"Failed to start MCP Server: {str(e)}")
```

## Security Best Practices

### 1. Input Validation

Always validate all inputs to prevent injection attacks:

```typescript
// TypeScript example
import { z } from 'zod';

const userIdSchema = z.string().uuid();

export const getUserTool: ToolDefinition = {
  // ...
  handler: async ({ userId }) => {
    try {
      // Validate userId
      const validatedUserId = userIdSchema.parse(userId);
      const user = await getUserById(validatedUserId);
      return { user };
    } catch (error) {
      if (error instanceof z.ZodError) {
        throw new Error(`Invalid user ID format: ${error.message}`);
      }
      throw new Error(`Failed to get user: ${error.message}`);
    }
  }
};
```

### 2. Authentication & Authorization

Implement proper authentication and authorization:

```typescript
// TypeScript example
import { verifyToken, checkPermission } from './auth';

export const getUserTool: ToolDefinition = {
  // ...
  handler: async ({ userId }, context) => {
    try {
      // Verify authentication
      const user = await verifyToken(context.authToken);
      if (!user) {
        throw new Error('Unauthorized');
      }
      
      // Check authorization
      if (!await checkPermission(user, 'users:read', userId)) {
        throw new Error('Forbidden');
      }
      
      const userData = await getUserById(userId);
      return { user: userData };
    } catch (error) {
      throw new Error(`Failed to get user: ${error.message}`);
    }
  }
};
```

### 3. Rate Limiting

Implement rate limiting to prevent abuse:

```typescript
// TypeScript example
import { RateLimiter } from 'limiter';

// Create a rate limiter: 100 requests per minute
const limiter = new RateLimiter({ tokensPerInterval: 100, interval: 'minute' });

export const searchUsersTool: ToolDefinition = {
  // ...
  handler: async ({ query, limit }, context) => {
    try {
      // Check rate limit
      const remainingRequests = await limiter.removeTokens(1);
      if (remainingRequests < 0) {
        throw new Error('Rate limit exceeded. Please try again later.');
      }
      
      const users = await searchUsers(query, limit);
      return { users };
    } catch (error) {
      throw new Error(`Failed to search users: ${error.message}`);
    }
  }
};
```

### 4. Sensitive Data Handling

Be careful with sensitive data:

```typescript
// TypeScript example
import { maskSensitiveData } from './security';

export const getUserTool: ToolDefinition = {
  // ...
  handler: async ({ userId }) => {
    try {
      const user = await getUserById(userId);
      
      // Mask sensitive data before returning
      const maskedUser = maskSensitiveData(user, ['ssn', 'password', 'creditCard']);
      
      return { user: maskedUser };
    } catch (error) {
      throw new Error(`Failed to get user: ${error.message}`);
    }
  }
};
```

## Testing Your MCP Server

### 1. Unit Testing

Test individual tools and resources:

```typescript
// TypeScript example with Jest
import { getUserTool } from './tools';

describe('getUserTool', () => {
  it('should return a user when given a valid ID', async () => {
    // Mock the user service
    jest.mock('./userService', () => ({
      getUserById: jest.fn().mockResolvedValue({
        id: '123',
        name: 'John Doe',
        email: 'john@example.com',
        role: 'user'
      })
    }));
    
    const result = await getUserTool.handler({ userId: '123' }, {});
    expect(result).toEqual({
      user: {
        id: '123',
        name: 'John Doe',
        email: 'john@example.com',
        role: 'user'
      }
    });
  });
  
  it('should throw an error when given an invalid ID', async () => {
    // Mock the user service to throw an error
    jest.mock('./userService', () => ({
      getUserById: jest.fn().mockRejectedValue(new Error('User not found'))
    }));
    
    await expect(getUserTool.handler({ userId: 'invalid' }, {}))
      .rejects
      .toThrow('Failed to get user: User not found');
  });
});
```

### 2. Integration Testing

Test the complete server:

```typescript
// TypeScript example with Supertest
import request from 'supertest';
import { server } from './server';

describe('MCP Server', () => {
  beforeAll(async () => {
    await server.start();
  });
  
  afterAll(async () => {
    await server.stop();
  });
  
  it('should return a user when calling get_user', async () => {
    const response = await request(server.app)
      .post('/mcp')
      .send({
        tool: 'get_user',
        params: { userId: '123' }
      });
    
    expect(response.status).toBe(200);
    expect(response.body).toEqual({
      user: {
        id: '123',
        name: 'John Doe',
        email: 'john@example.com',
        role: 'user'
      }
    });
  });
});
```

### 3. Testing with Claude

Test your server with Claude using the Claude Desktop App:

1. Configure your server in the Claude Desktop App
2. Start a conversation with Claude
3. Ask Claude to use your custom tools
4. Verify that the responses are correct

## Deployment

### 1. Local Deployment

For development and testing:

```bash
# TypeScript
npm start

# Python
python server.py
```

### 2. Docker Deployment

For containerized deployment:

```dockerfile
# Dockerfile
FROM node:18-alpine

WORKDIR /app

COPY package*.json ./
RUN npm install

COPY . .
RUN npm run build

EXPOSE 3000

CMD ["npm", "start"]
```

Build and run:

```bash
docker build -t my-mcp-server .
docker run -p 3000:3000 my-mcp-server
```

### 3. Cloud Deployment

Deploy to cloud platforms:

- **AWS Lambda**: Serverless deployment
- **Google Cloud Run**: Containerized serverless
- **Azure Functions**: Serverless deployment
- **Heroku**: PaaS deployment
- **Digital Ocean**: VPS deployment

## Monitoring and Maintenance

### 1. Logging

Implement comprehensive logging:

```typescript
// TypeScript example
import winston from 'winston';

const logger = winston.createLogger({
  level: 'info',
  format: winston.format.json(),
  defaultMeta: { service: 'mcp-server' },
  transports: [
    new winston.transports.File({ filename: 'error.log', level: 'error' }),
    new winston.transports.File({ filename: 'combined.log' })
  ]
});

if (process.env.NODE_ENV !== 'production') {
  logger.add(new winston.transports.Console({
    format: winston.format.simple()
  }));
}

export const getUserTool: ToolDefinition = {
  // ...
  handler: async ({ userId }) => {
    logger.info('getUserTool called', { userId });
    try {
      const user = await getUserById(userId);
      logger.info('getUserTool succeeded', { userId });
      return { user };
    } catch (error) {
      logger.error('getUserTool failed', { userId, error: error.message });
      throw new Error(`Failed to get user: ${error.message}`);
    }
  }
};
```

### 2. Metrics

Collect performance metrics:

```typescript
// TypeScript example with Prometheus
import prometheus from 'prom-client';

const requestCounter = new prometheus.Counter({
  name: 'mcp_requests_total',
  help: 'Total number of MCP requests',
  labelNames: ['tool']
});

const requestDuration = new prometheus.Histogram({
  name: 'mcp_request_duration_seconds',
  help: 'Duration of MCP requests in seconds',
  labelNames: ['tool']
});

export const getUserTool: ToolDefinition = {
  // ...
  handler: async ({ userId }) => {
    requestCounter.inc({ tool: 'get_user' });
    const timer = requestDuration.startTimer({ tool: 'get_user' });
    
    try {
      const user = await getUserById(userId);
      timer();
      return { user };
    } catch (error) {
      timer();
      throw new Error(`Failed to get user: ${error.message}`);
    }
  }
};
```

### 3. Alerting

Set up alerts for critical issues:

```typescript
// TypeScript example
import { sendAlert } from './alerting';

export const getUserTool: ToolDefinition = {
  // ...
  handler: async ({ userId }) => {
    try {
      const user = await getUserById(userId);
      return { user };
    } catch (error) {
      // Send alert for critical errors
      if (error.code === 'DATABASE_CONNECTION_ERROR') {
        await sendAlert({
          severity: 'critical',
          message: 'Database connection error in getUserTool',
          details: error.message
        });
      }
      
      throw new Error(`Failed to get user: ${error.message}`);
    }
  }
};
```

## Advanced Topics

### 1. Streaming Responses

Implement streaming for large responses:

```typescript
// TypeScript example
export const searchUsersTool: ToolDefinition = {
  // ...
  handler: async ({ query, limit }, context, { stream }) => {
    try {
      // Start streaming
      stream.start();
      
      // Stream results as they become available
      const userStream = await searchUsersStream(query, limit);
      
      let count = 0;
      for await (const user of userStream) {
        stream.write({ user });
        count++;
      }
      
      // End streaming
      stream.end({ totalCount: count });
    } catch (error) {
      stream.error(`Failed to search users: ${error.message}`);
    }
  }
};
```

### 2. Caching

Implement caching for improved performance:

```typescript
// TypeScript example with Node-Cache
import NodeCache from 'node-cache';

const cache = new NodeCache({ stdTTL: 300 }); // 5 minutes TTL

export const getUserTool: ToolDefinition = {
  // ...
  handler: async ({ userId }) => {
    try {
      // Check cache first
      const cacheKey = `user:${userId}`;
      const cachedUser = cache.get(cacheKey);
      
      if (cachedUser) {
        return { user: cachedUser, fromCache: true };
      }
      
      // If not in cache, fetch from database
      const user = await getUserById(userId);
      
      // Store in cache
      cache.set(cacheKey, user);
      
      return { user, fromCache: false };
    } catch (error) {
      throw new Error(`Failed to get user: ${error.message}`);
    }
  }
};
```

### 3. Webhooks

Implement webhooks for event-driven architecture:

```typescript
// TypeScript example
import { registerWebhook } from './webhooks';

// Register webhook for user creation
registerWebhook('user.created', async (event) => {
  const { user } = event.data;
  
  // Perform actions when a user is created
  await sendWelcomeEmail(user.email);
  await createDefaultSettings(user.id);
  
  return { success: true };
});

export const createUserTool: ToolDefinition = {
  // ...
  handler: async ({ userData }) => {
    try {
      const user = await createUser(userData);
      
      // Trigger webhook
      await triggerWebhook('user.created', { user });
      
      return { user };
    } catch (error) {
      throw new Error(`Failed to create user: ${error.message}`);
    }
  }
};
```

## Conclusion

Building a custom MCP server allows you to extend AI capabilities with specialized tools and data sources. By following the guidelines in this document, you can create secure, efficient, and maintainable MCP servers that enhance the capabilities of AI assistants like Claude.

Remember to focus on:
- Clear resource and tool definitions
- Robust security practices
- Comprehensive testing
- Proper monitoring and maintenance

With these principles in mind, you can create powerful MCP servers that unlock new possibilities for AI-powered applications.
