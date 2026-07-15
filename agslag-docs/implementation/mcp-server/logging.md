# MCP Server Logging Implementation

This document outlines the implementation plan for enhanced logging in the MCP server.

## Overview

The logging system will provide:
- Structured logging with JSON format
- Multiple severity levels
- Request/response logging
- Sensitive data redaction
- Log rotation and management
- Integration with monitoring tools

## Implementation Details

### 1. Logger Configuration

Create a centralized logger using Winston:

```typescript
// src/utils/logger.ts
import winston from 'winston';
import 'winston-daily-rotate-file';
import path from 'path';
import { Config } from '../config';

// Get log level from config
const logLevel = Config.logging.level || 'info';

// Define log format
const logFormat = winston.format.combine(
  winston.format.timestamp({ format: 'YYYY-MM-DD HH:mm:ss.SSS' }),
  winston.format.errors({ stack: true }),
  winston.format.json()
);

// Create file transport for rotating logs
const fileTransport = new winston.transports.DailyRotateFile({
  filename: path.join(Config.logging.directory || 'logs', 'mcp-server-%DATE%.log'),
  datePattern: 'YYYY-MM-DD',
  zippedArchive: true,
  maxSize: '20m',
  maxFiles: '14d',
  level: logLevel
});

// Create console transport
const consoleTransport = new winston.transports.Console({
  format: winston.format.combine(
    winston.format.colorize(),
    winston.format.printf(({ level, message, timestamp, ...meta }) => {
      return `${timestamp} ${level}: ${
        typeof message === 'object' ? JSON.stringify(message) : message
      } ${Object.keys(meta).length ? JSON.stringify(meta, null, 2) : ''}`;
    })
  ),
  level: logLevel
});

// Create the logger
const logger = winston.createLogger({
  level: logLevel,
  format: logFormat,
  defaultMeta: { service: 'mcp-server' },
  transports: [
    fileTransport,
    consoleTransport
  ],
  exitOnError: false
});

// Add stream for Morgan
logger.stream = {
  write: (message: string) => {
    logger.http(message.trim());
  }
};

export default logger;
```

### 2. Request Logging Middleware

Create middleware for logging HTTP requests:

```typescript
// src/middleware/request-logger.ts
import morgan from 'morgan';
import logger from '../utils/logger';
import { Config } from '../config';

// Create a token for response time
morgan.token('response-time', (req, res) => {
  if (!req._startAt || !res._startAt) {
    return '';
  }
  
  const ms = (res._startAt[0] - req._startAt[0]) * 1000 +
    (res._startAt[1] - req._startAt[1]) * 1e-6;
  
  return ms.toFixed(3);
});

// Create a token for user ID (if authenticated)
morgan.token('user-id', (req: any) => {
  return req.user ? req.user.id : '-';
});

// Create a token for request body (with sensitive data redaction)
morgan.token('request-body', (req: any) => {
  if (!req.body || Object.keys(req.body).length === 0) {
    return '-';
  }
  
  // Create a copy of the body
  const sanitizedBody = { ...req.body };
  
  // List of fields to redact
  const sensitiveFields = ['password', 'token', 'apiKey', 'secret', 'authorization'];
  
  // Redact sensitive fields
  for (const field of sensitiveFields) {
    if (sanitizedBody[field]) {
      sanitizedBody[field] = '[REDACTED]';
    }
  }
  
  return JSON.stringify(sanitizedBody);
});

// Create a token for response body
morgan.token('response-body', (req: any, res: any) => {
  if (!res.body) {
    return '-';
  }
  
  return JSON.stringify(res.body);
});

// Define log format based on environment
const getFormat = () => {
  if (Config.environment === 'production') {
    // JSON format for production
    return (tokens: any, req: any, res: any) => {
      return JSON.stringify({
        method: tokens.method(req, res),
        url: tokens.url(req, res),
        status: parseInt(tokens.status(req, res)),
        responseTime: parseFloat(tokens['response-time'](req, res)),
        contentLength: tokens.res(req, res, 'content-length') || '-',
        userId: tokens['user-id'](req, res),
        userAgent: tokens['user-agent'](req, res),
        ip: tokens['remote-addr'](req, res),
        referrer: tokens.referrer(req, res) || '-',
        requestBody: tokens['request-body'](req, res),
        timestamp: new Date().toISOString()
      });
    };
  } else {
    // Dev format for non-production
    return ':method :url :status :response-time ms - :res[content-length] - :user-id';
  }
};

// Create the middleware
export const requestLogger = morgan(getFormat(), {
  stream: logger.stream,
  skip: (req) => {
    // Skip logging for health check endpoints to reduce noise
    return req.url.includes('/health');
  }
});

// Middleware to capture response body for logging
export const responseBodyCapture = (req: any, res: any, next: any) => {
  const originalSend = res.send;
  
  res.send = function(body: any) {
    res.body = body;
    return originalSend.call(this, body);
  };
  
  next();
};
```

### 3. WebSocket Logging

Create a utility for logging WebSocket events:

```typescript
// src/utils/websocket-logger.ts
import WebSocket from 'ws';
import logger from './logger';
import { redactSensitiveData } from './redact';

export const logWebSocketConnection = (socket: WebSocket, request: any) => {
  const clientIp = request.headers['x-forwarded-for'] || request.connection.remoteAddress;
  
  logger.info({
    event: 'websocket_connection',
    clientIp,
    userAgent: request.headers['user-agent'],
    timestamp: new Date().toISOString()
  });
  
  // Log disconnection
  socket.on('close', (code, reason) => {
    logger.info({
      event: 'websocket_disconnection',
      clientIp,
      code,
      reason: reason.toString(),
      timestamp: new Date().toISOString()
    });
  });
  
  // Log errors
  socket.on('error', (error) => {
    logger.error({
      event: 'websocket_error',
      clientIp,
      error: error.message,
      stack: error.stack,
      timestamp: new Date().toISOString()
    });
  });
};

export const logWebSocketMessage = (
  direction: 'incoming' | 'outgoing',
  message: any,
  socket: WebSocket,
  request?: any
) => {
  const clientIp = request ? 
    (request.headers['x-forwarded-for'] || request.connection.remoteAddress) : 
    'unknown';
  
  // Parse message if it's a string
  let parsedMessage;
  try {
    parsedMessage = typeof message === 'string' ? JSON.parse(message) : message;
  } catch (error) {
    parsedMessage = { rawMessage: message };
  }
  
  // Redact sensitive data
  const sanitizedMessage = redactSensitiveData(parsedMessage);
  
  logger.debug({
    event: `websocket_message_${direction}`,
    clientIp,
    message: sanitizedMessage,
    timestamp: new Date().toISOString()
  });
};
```

### 4. Data Redaction Utility

Create a utility for redacting sensitive data:

```typescript
// src/utils/redact.ts
// List of fields to redact
const SENSITIVE_FIELDS = [
  'password',
  'token',
  'apiKey',
  'secret',
  'authorization',
  'accessToken',
  'refreshToken',
  'key',
  'credential',
  'jwt',
  'ssn',
  'creditCard',
  'cardNumber'
];

// Function to check if a field name contains sensitive information
const isSensitiveField = (fieldName: string): boolean => {
  const lowerField = fieldName.toLowerCase();
  return SENSITIVE_FIELDS.some(field => lowerField.includes(field.toLowerCase()));
};

// Recursive function to redact sensitive data
export const redactSensitiveData = (data: any): any => {
  if (data === null || data === undefined) {
    return data;
  }
  
  if (typeof data !== 'object') {
    return data;
  }
  
  // Handle arrays
  if (Array.isArray(data)) {
    return data.map(item => redactSensitiveData(item));
  }
  
  // Handle objects
  const result: Record<string, any> = {};
  
  for (const [key, value] of Object.entries(data)) {
    if (isSensitiveField(key)) {
      result[key] = '[REDACTED]';
    } else if (typeof value === 'object' && value !== null) {
      result[key] = redactSensitiveData(value);
    } else {
      result[key] = value;
    }
  }
  
  return result;
};
```

### 5. Integration with Express

Integrate the logging middleware with Express:

```typescript
// src/app.ts
import express from 'express';
import { errorHandler } from './middleware/error-handler';
import { notFoundHandler } from './middleware/not-found-handler';
import { requestLogger, responseBodyCapture } from './middleware/request-logger';
import routes from './routes';
import logger from './utils/logger';

const app = express();

// Start request processing
logger.info('Initializing MCP Server');

// Request logging middleware
app.use(requestLogger);
app.use(responseBodyCapture);

// Body parsing middleware
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// API routes
app.use('/api', routes);

// Health check endpoint
app.get('/health', (req, res) => {
  res.status(200).json({ status: 'UP' });
});

// Handle 404 errors
app.use(notFoundHandler);

// Global error handler - must be last
app.use(errorHandler);

export default app;
```

### 6. WebSocket Server Integration

Integrate logging with the WebSocket server:

```typescript
// src/websocket/server.ts
import { Server } from 'http';
import WebSocket from 'ws';
import { Config } from '../config';
import logger from '../utils/logger';
import { logWebSocketConnection, logWebSocketMessage } from '../utils/websocket-logger';

export class WebSocketServer {
  private wss: WebSocket.Server;
  
  constructor(server: Server) {
    this.wss = new WebSocket.Server({
      server,
      path: Config.websocket.path
    });
    
    this.init();
  }
  
  private init() {
    logger.info(`Initializing WebSocket server on path: ${Config.websocket.path}`);
    
    this.wss.on('connection', (socket, request) => {
      // Log connection
      logWebSocketConnection(socket, request);
      
      // Handle messages
      socket.on('message', (message) => {
        // Log incoming message
        logWebSocketMessage('incoming', message, socket, request);
        
        // Process message
        this.handleMessage(message, socket, request);
      });
    });
    
    this.wss.on('error', (error) => {
      logger.error({
        event: 'websocket_server_error',
        error: error.message,
        stack: error.stack,
        timestamp: new Date().toISOString()
      });
    });
    
    logger.info('WebSocket server initialized successfully');
  }
  
  private handleMessage(message: WebSocket.Data, socket: WebSocket, request: any) {
    try {
      // Process message
      const response = { status: 'ok' }; // Placeholder
      
      // Send response
      socket.send(JSON.stringify(response));
      
      // Log outgoing message
      logWebSocketMessage('outgoing', response, socket, request);
    } catch (error) {
      logger.error({
        event: 'websocket_message_processing_error',
        error: error.message,
        stack: error.stack,
        timestamp: new Date().toISOString()
      });
      
      // Send error response
      const errorResponse = { 
        status: 'error', 
        message: 'Failed to process message' 
      };
      socket.send(JSON.stringify(errorResponse));
      
      // Log outgoing error message
      logWebSocketMessage('outgoing', errorResponse, socket, request);
    }
  }
  
  public broadcast(message: any) {
    const messageStr = JSON.stringify(message);
    
    this.wss.clients.forEach((client) => {
      if (client.readyState === WebSocket.OPEN) {
        client.send(messageStr);
        
        // Log outgoing broadcast message
        logWebSocketMessage('outgoing', message, client);
      }
    });
    
    logger.debug({
      event: 'websocket_broadcast',
      clientCount: this.wss.clients.size,
      timestamp: new Date().toISOString()
    });
  }
  
  public getConnections(): number {
    return this.wss.clients.size;
  }
}
```

## Usage Examples

### Logging in Services

```typescript
// src/services/agent-service.ts
import logger from '../utils/logger';
import { Agent } from '../models/agent';

export class AgentService {
  async getAllAgents() {
    logger.debug('Fetching all agents');
    
    try {
      const agents = await Agent.find();
      
      logger.debug({
        message: 'Agents fetched successfully',
        count: agents.length
      });
      
      return agents;
    } catch (error) {
      logger.error({
        message: 'Failed to fetch agents',
        error: error.message,
        stack: error.stack
      });
      
      throw error;
    }
  }
  
  async getAgentById(id: string) {
    logger.debug({
      message: 'Fetching agent by ID',
      agentId: id
    });
    
    try {
      const agent = await Agent.findById(id);
      
      if (agent) {
        logger.debug({
          message: 'Agent found',
          agentId: id
        });
      } else {
        logger.debug({
          message: 'Agent not found',
          agentId: id
        });
      }
      
      return agent;
    } catch (error) {
      logger.error({
        message: 'Failed to fetch agent',
        agentId: id,
        error: error.message,
        stack: error.stack
      });
      
      throw error;
    }
  }
}
```

## Implementation Steps

1. Install required dependencies:
   ```bash
   npm install winston winston-daily-rotate-file morgan
   ```

2. Create the logger configuration
3. Implement the request logging middleware
4. Create the WebSocket logging utilities
5. Implement the data redaction utility
6. Integrate with Express application
7. Integrate with WebSocket server
8. Document usage patterns for the team
