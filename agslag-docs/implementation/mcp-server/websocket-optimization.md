# MCP Server WebSocket Optimization Implementation

This document outlines the implementation plan for optimizing WebSocket communication in the MCP server.

## Overview

The WebSocket optimization will provide:
- Connection pooling for efficient resource usage
- Message batching for bulk operations
- Efficient serialization/deserialization
- Heartbeat mechanism for connection health monitoring
- Reconnection strategies for improved reliability
- Message compression for reduced bandwidth usage

## Implementation Details

### 1. WebSocket Server Configuration

Create an optimized WebSocket server configuration:

```typescript
// src/websocket/server.ts
import { Server } from 'http';
import WebSocket from 'ws';
import { Config } from '../config';
import logger from '../utils/logger';
import { logWebSocketConnection, logWebSocketMessage } from '../utils/websocket-logger';
import { MetricsService } from '../services/metrics-service';
import { authenticate } from '../utils/websocket-auth';

export class WebSocketServer {
  private wss: WebSocket.Server;
  private heartbeatInterval: NodeJS.Timeout | null = null;
  private readonly HEARTBEAT_INTERVAL = 30000; // 30 seconds
  
  constructor(server: Server) {
    // Create WebSocket server with optimized configuration
    this.wss = new WebSocket.Server({
      server,
      path: Config.websocket.path,
      // Client verification (authentication)
      verifyClient: authenticate,
      // Connection handling options
      clientTracking: true, // Track clients for broadcasting
      maxPayload: 1024 * 1024, // 1MB max message size
      // Compression options
      perMessageDeflate: {
        zlibDeflateOptions: {
          level: 6, // Balance between compression and CPU usage
        },
        zlibInflateOptions: {
          chunkSize: 10 * 1024 // 10KB
        },
        clientNoContextTakeover: true, // Don't reuse context for client
        serverNoContextTakeover: true, // Don't reuse context for server
        serverMaxWindowBits: 10, // Smaller window size for less memory usage
        concurrencyLimit: 10, // Limit concurrent compressions
        threshold: 1024 // Only compress messages larger than 1KB
      }
    });
    
    this.init();
  }
  
  private init() {
    logger.info(`Initializing WebSocket server on path: ${Config.websocket.path}`);
    
    // Connection handler
    this.wss.on('connection', this.handleConnection.bind(this));
    
    // Error handler
    this.wss.on('error', (error) => {
      logger.error({
        event: 'websocket_server_error',
        error: error.message,
        stack: error.stack,
        timestamp: new Date().toISOString()
      });
      
      MetricsService.recordError('server');
    });
    
    // Start heartbeat interval
    this.startHeartbeat();
    
    logger.info('WebSocket server initialized successfully');
  }
  
  private handleConnection(socket: WebSocket, request: any) {
    // Mark socket as alive for heartbeat
    (socket as any).isAlive = true;
    
    // Set up pong handler for heartbeat
    socket.on('pong', () => {
      (socket as any).isAlive = true;
    });
    
    // Log connection
    logWebSocketConnection(socket, request);
    
    // Update connection metrics
    MetricsService.updateWebSocketConnections(this.wss.clients.size);
    
    // Store user info from authentication
    (socket as any).user = request.user;
    
    // Message handler
    socket.on('message', (data) => this.handleMessage(data, socket, request));
    
    // Close handler
    socket.on('close', (code, reason) => {
      logger.info({
        event: 'websocket_disconnection',
        user: (socket as any).user?.id,
        code,
        reason: reason.toString(),
        timestamp: new Date().toISOString()
      });
      
      // Update connection metrics
      MetricsService.updateWebSocketConnections(this.wss.clients.size);
    });
    
    // Error handler
    socket.on('error', (error) => {
      logger.error({
        event: 'websocket_error',
        user: (socket as any).user?.id,
        error: error.message,
        stack: error.stack,
        timestamp: new Date().toISOString()
      });
      
      MetricsService.recordError('network');
    });
    
    // Send welcome message
    this.sendToClient(socket, {
      type: 'connection_established',
      message: 'Connected to MCP Server',
      timestamp: new Date().toISOString()
    });
  }
  
  private handleMessage(data: WebSocket.Data, socket: WebSocket, request: any) {
    try {
      // Parse message
      const message = this.parseMessage(data);
      
      // Log incoming message
      logWebSocketMessage('incoming', message, socket, request);
      
      // Record message metric
      MetricsService.recordWebSocketMessage('incoming');
      
      // Process message based on type
      switch (message.type) {
        case 'ping':
          this.handlePing(socket, message);
          break;
        case 'batch':
          this.handleBatch(socket, message);
          break;
        case 'agent_status':
          this.handleAgentStatus(socket, message);
          break;
        default:
          this.handleUnknownMessage(socket, message);
          break;
      }
    } catch (error) {
      logger.error({
        event: 'websocket_message_processing_error',
        user: (socket as any).user?.id,
        error: error.message,
        stack: error.stack,
        data: typeof data === 'string' ? data : '(binary data)',
        timestamp: new Date().toISOString()
      });
      
      MetricsService.recordError('server');
      
      // Send error response
      this.sendToClient(socket, {
        type: 'error',
        message: 'Failed to process message',
        error: error.message,
        timestamp: new Date().toISOString()
      });
    }
  }
  
  private parseMessage(data: WebSocket.Data): any {
    if (typeof data === 'string') {
      return JSON.parse(data);
    } else if (data instanceof Buffer) {
      return JSON.parse(data.toString());
    } else if (Array.isArray(data)) {
      // Handle array of buffers
      const concatenated = Buffer.concat(data as Buffer[]);
      return JSON.parse(concatenated.toString());
    } else {
      throw new Error('Unsupported message format');
    }
  }
  
  private handlePing(socket: WebSocket, message: any) {
    // Respond to ping with pong
    this.sendToClient(socket, {
      type: 'pong',
      id: message.id,
      timestamp: new Date().toISOString()
    });
  }
  
  private handleBatch(socket: WebSocket, message: any) {
    // Process batch of messages
    if (Array.isArray(message.messages)) {
      const responses = message.messages.map((msg: any) => {
        try {
          // Process each message
          return {
            id: msg.id,
            type: 'response',
            status: 'success',
            result: this.processMessage(msg)
          };
        } catch (error) {
          return {
            id: msg.id,
            type: 'response',
            status: 'error',
            error: error.message
          };
        }
      });
      
      // Send batch response
      this.sendToClient(socket, {
        type: 'batch_response',
        id: message.id,
        responses,
        timestamp: new Date().toISOString()
      });
    } else {
      throw new Error('Invalid batch message format');
    }
  }
  
  private handleAgentStatus(socket: WebSocket, message: any) {
    // Update agent status
    // This is a placeholder for actual implementation
    logger.info({
      event: 'agent_status_update',
      agentId: message.agentId,
      status: message.status,
      timestamp: new Date().toISOString()
    });
    
    // Broadcast status update to all clients
    this.broadcast({
      type: 'agent_status_update',
      agentId: message.agentId,
      status: message.status,
      timestamp: new Date().toISOString()
    });
    
    // Send acknowledgement
    this.sendToClient(socket, {
      type: 'agent_status_ack',
      id: message.id,
      agentId: message.agentId,
      timestamp: new Date().toISOString()
    });
  }
  
  private handleUnknownMessage(socket: WebSocket, message: any) {
    logger.warn({
      event: 'unknown_message_type',
      type: message.type,
      user: (socket as any).user?.id,
      timestamp: new Date().toISOString()
    });
    
    this.sendToClient(socket, {
      type: 'error',
      id: message.id,
      message: `Unknown message type: ${message.type}`,
      timestamp: new Date().toISOString()
    });
  }
  
  private processMessage(message: any): any {
    // This is a placeholder for actual message processing
    // In a real implementation, this would dispatch to appropriate handlers
    return { processed: true };
  }
  
  private sendToClient(socket: WebSocket, data: any) {
    try {
      // Serialize message
      const message = JSON.stringify(data);
      
      // Send to client
      socket.send(message);
      
      // Log outgoing message
      logWebSocketMessage('outgoing', data, socket);
      
      // Record message metric
      MetricsService.recordWebSocketMessage('outgoing');
    } catch (error) {
      logger.error({
        event: 'websocket_send_error',
        error: error.message,
        stack: error.stack,
        data,
        timestamp: new Date().toISOString()
      });
      
      MetricsService.recordError('network');
    }
  }
  
  public broadcast(data: any, excludeSocket?: WebSocket) {
    try {
      // Serialize message once
      const message = JSON.stringify(data);
      let sentCount = 0;
      
      // Send to all connected clients
      this.wss.clients.forEach((client) => {
        if (client !== excludeSocket && client.readyState === WebSocket.OPEN) {
          client.send(message);
          sentCount++;
          
          // Record message metric
          MetricsService.recordWebSocketMessage('outgoing');
        }
      });
      
      // Log broadcast
      logger.debug({
        event: 'websocket_broadcast',
        clientCount: this.wss.clients.size,
        sentCount,
        messageType: data.type,
        timestamp: new Date().toISOString()
      });
    } catch (error) {
      logger.error({
        event: 'websocket_broadcast_error',
        error: error.message,
        stack: error.stack,
        data,
        timestamp: new Date().toISOString()
      });
      
      MetricsService.recordError('server');
    }
  }
  
  private startHeartbeat() {
    // Clear any existing interval
    if (this.heartbeatInterval) {
      clearInterval(this.heartbeatInterval);
    }
    
    // Start new heartbeat interval
    this.heartbeatInterval = setInterval(() => {
      this.wss.clients.forEach((socket) => {
        if ((socket as any).isAlive === false) {
          logger.info({
            event: 'websocket_heartbeat_timeout',
            user: (socket as any).user?.id,
            timestamp: new Date().toISOString()
          });
          
          return socket.terminate();
        }
        
        (socket as any).isAlive = false;
        socket.ping(() => {});
      });
    }, this.HEARTBEAT_INTERVAL);
    
    logger.info(`WebSocket heartbeat started with interval: ${this.HEARTBEAT_INTERVAL}ms`);
  }
  
  public getConnections(): number {
    return this.wss.clients.size;
  }
  
  public shutdown() {
    // Clear heartbeat interval
    if (this.heartbeatInterval) {
      clearInterval(this.heartbeatInterval);
      this.heartbeatInterval = null;
    }
    
    // Close all connections
    this.wss.clients.forEach((socket) => {
      socket.terminate();
    });
    
    // Close server
    this.wss.close();
    
    logger.info('WebSocket server shut down');
  }
}
```

### 2. WebSocket Authentication Utility

Create a utility for authenticating WebSocket connections:

```typescript
// src/utils/websocket-auth.ts
import jwt from 'jsonwebtoken';
import { Config } from '../config';
import logger from './logger';

// Maximum connections per user
const MAX_CONNECTIONS_PER_USER = 5;

// Connection counter (in-memory)
const connectionCounter: Record<string, number> = {};

// Verify client function for WebSocket server
export const authenticate = async (info: any, callback: (result: boolean, code?: number, message?: string) => void) => {
  try {
    // Get token from query parameters
    const url = new URL(`ws://${info.req.headers.host}${info.req.url}`);
    const token = url.searchParams.get('token');
    
    // Check if token exists
    if (!token) {
      logger.warn({
        event: 'websocket_auth_failed',
        reason: 'missing_token',
        ip: info.req.headers['x-forwarded-for'] || info.req.connection.remoteAddress,
        timestamp: new Date().toISOString()
      });
      
      callback(false, 401, 'Unauthorized: Missing token');
      return;
    }
    
    // Verify token
    const decoded = jwt.verify(token, Config.auth.jwt_secret);
    
    // Store user info in request for later use
    info.req.user = decoded;
    
    // Check connection limits
    const userId = decoded.id || 'anonymous';
    connectionCounter[userId] = (connectionCounter[userId] || 0) + 1;
    
    if (connectionCounter[userId] > MAX_CONNECTIONS_PER_USER) {
      logger.warn({
        event: 'websocket_auth_failed',
        reason: 'connection_limit',
        userId,
        connections: connectionCounter[userId],
        limit: MAX_CONNECTIONS_PER_USER,
        timestamp: new Date().toISOString()
      });
      
      // Decrement counter since connection will be rejected
      connectionCounter[userId]--;
      
      callback(false, 429, 'Too Many Connections');
      return;
    }
    
    // Log successful authentication
    logger.info({
      event: 'websocket_auth_success',
      userId,
      connections: connectionCounter[userId],
      timestamp: new Date().toISOString()
    });
    
    // Authentication successful
    callback(true);
  } catch (error) {
    // Log authentication error
    logger.error({
      event: 'websocket_auth_error',
      error: error.message,
      stack: error.stack,
      ip: info.req.headers['x-forwarded-for'] || info.req.connection.remoteAddress,
      timestamp: new Date().toISOString()
    });
    
    // Authentication failed
    callback(false, 401, 'Unauthorized: Invalid token');
  }
};

// Handle connection close to update counter
export const handleConnectionClose = (userId: string) => {
  if (userId && connectionCounter[userId]) {
    connectionCounter[userId]--;
    
    if (connectionCounter[userId] <= 0) {
      delete connectionCounter[userId];
    }
  }
};
```

### 3. WebSocket Client

Create an optimized WebSocket client for testing and internal use:

```typescript
// src/utils/websocket-client.ts
import WebSocket from 'ws';
import { EventEmitter } from 'events';
import logger from './logger';

export class WebSocketClient extends EventEmitter {
  private socket: WebSocket | null = null;
  private url: string;
  private token: string;
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 10;
  private reconnectInterval = 1000; // Start with 1 second
  private reconnectTimer: NodeJS.Timeout | null = null;
  private messageQueue: any[] = [];
  private connected = false;
  private intentionallyClosed = false;
  
  constructor(url: string, token: string) {
    super();
    this.url = url;
    this.token = token;
  }
  
  public connect() {
    // Don't reconnect if intentionally closed
    if (this.intentionallyClosed) {
      return;
    }
    
    // Clear any existing reconnect timer
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    
    try {
      // Create WebSocket connection with token
      const fullUrl = `${this.url}?token=${this.token}`;
      this.socket = new WebSocket(fullUrl);
      
      // Connection opened handler
      this.socket.on('open', () => {
        this.connected = true;
        this.reconnectAttempts = 0;
        this.reconnectInterval = 1000;
        
        logger.info({
          event: 'websocket_client_connected',
          url: this.url,
          timestamp: new Date().toISOString()
        });
        
        // Emit connected event
        this.emit('connected');
        
        // Send any queued messages
        this.flushMessageQueue();
      });
      
      // Message handler
      this.socket.on('message', (data) => {
        try {
          const message = JSON.parse(data.toString());
          this.emit('message', message);
        } catch (error) {
          logger.error({
            event: 'websocket_client_message_parse_error',
            error: error.message,
            data: data.toString(),
            timestamp: new Date().toISOString()
          });
          
          this.emit('error', error);
        }
      });
      
      // Error handler
      this.socket.on('error', (error) => {
        logger.error({
          event: 'websocket_client_error',
          error: error.message,
          timestamp: new Date().toISOString()
        });
        
        this.emit('error', error);
      });
      
      // Close handler
      this.socket.on('close', (code, reason) => {
        this.connected = false;
        
        logger.info({
          event: 'websocket_client_disconnected',
          code,
          reason: reason.toString(),
          timestamp: new Date().toISOString()
        });
        
        this.emit('disconnected', { code, reason: reason.toString() });
        
        // Attempt to reconnect if not intentionally closed
        if (!this.intentionallyClosed) {
          this.scheduleReconnect();
        }
      });
      
      // Ping handler
      this.socket.on('ping', () => {
        this.emit('ping');
        // Automatically responds with pong
      });
      
      // Pong handler
      this.socket.on('pong', () => {
        this.emit('pong');
      });
    } catch (error) {
      logger.error({
        event: 'websocket_client_connect_error',
        error: error.message,
        url: this.url,
        timestamp: new Date().toISOString()
      });
      
      this.emit('error', error);
      
      // Attempt to reconnect
      this.scheduleReconnect();
    }
  }
  
  private scheduleReconnect() {
    // Check if max reconnect attempts reached
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      logger.error({
        event: 'websocket_client_max_reconnect_attempts',
        attempts: this.reconnectAttempts,
        url: this.url,
        timestamp: new Date().toISOString()
      });
      
      this.emit('reconnect_failed');
      return;
    }
    
    // Increment reconnect attempts
    this.reconnectAttempts++;
    
    // Calculate backoff time (exponential backoff with jitter)
    const backoff = Math.min(30000, this.reconnectInterval * Math.pow(1.5, this.reconnectAttempts - 1));
    const jitter = Math.random() * 0.3 * backoff;
    const delay = backoff + jitter;
    
    logger.info({
      event: 'websocket_client_scheduling_reconnect',
      attempt: this.reconnectAttempts,
      delay: Math.round(delay),
      url: this.url,
      timestamp: new Date().toISOString()
    });
    
    this.emit('reconnecting', { attempt: this.reconnectAttempts, delay });
    
    // Schedule reconnect
    this.reconnectTimer = setTimeout(() => {
      this.connect();
    }, delay);
  }
  
  public send(data: any) {
    if (!this.connected || !this.socket) {
      // Queue message if not connected
      this.messageQueue.push(data);
      return false;
    }
    
    try {
      // Serialize message
      const message = JSON.stringify(data);
      
      // Send message
      this.socket.send(message);
      return true;
    } catch (error) {
      logger.error({
        event: 'websocket_client_send_error',
        error: error.message,
        data,
        timestamp: new Date().toISOString()
      });
      
      this.emit('error', error);
      return false;
    }
  }
  
  private flushMessageQueue() {
    if (this.messageQueue.length === 0) {
      return;
    }
    
    logger.info({
      event: 'websocket_client_flushing_queue',
      queueSize: this.messageQueue.length,
      timestamp: new Date().toISOString()
    });
    
    // Send all queued messages
    while (this.messageQueue.length > 0) {
      const message = this.messageQueue.shift();
      this.send(message);
    }
  }
  
  public close() {
    this.intentionallyClosed = true;
    
    // Clear reconnect timer
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    
    // Close socket if connected
    if (this.socket) {
      this.socket.close(1000, 'Client closed connection');
      this.socket = null;
    }
    
    this.connected = false;
    
    logger.info({
      event: 'websocket_client_closed',
      timestamp: new Date().toISOString()
    });
  }
  
  public isConnected(): boolean {
    return this.connected;
  }
  
  public getQueueSize(): number {
    return this.messageQueue.length;
  }
}
```

### 4. Message Batching Utility

Create a utility for batching messages:

```typescript
// src/utils/message-batcher.ts
import { EventEmitter } from 'events';
import { v4 as uuidv4 } from 'uuid';
import logger from './logger';

export class MessageBatcher extends EventEmitter {
  private batchSize: number;
  private flushInterval: number;
  private maxBatchAge: number;
  private messages: any[] = [];
  private timer: NodeJS.Timeout | null = null;
  private batchStartTime: number = Date.now();
  
  constructor(options: {
    batchSize?: number;
    flushInterval?: number;
    maxBatchAge?: number;
  } = {}) {
    super();
    
    this.batchSize = options.batchSize || 10;
    this.flushInterval = options.flushInterval || 1000; // 1 second
    this.maxBatchAge = options.maxBatchAge || 5000; // 5 seconds
    
    // Start timer
    this.startTimer();
  }
  
  public add(message: any): void {
    // Add message to batch
    this.messages.push(message);
    
    // Check if batch is full
    if (this.messages.length >= this.batchSize) {
      this.flush('batch_full');
    }
    
    // Check if batch is too old
    const batchAge = Date.now() - this.batchStartTime;
    if (batchAge >= this.maxBatchAge) {
      this.flush('batch_age');
    }
  }
  
  private startTimer(): void {
    // Clear any existing timer
    if (this.timer) {
      clearInterval(this.timer);
    }
    
    // Start new timer
    this.timer = setInterval(() => {
      if (this.messages.length > 0) {
        this.flush('timer');
      }
    }, this.flushInterval);
  }
  
  private flush(reason: string): void {
    if (this.messages.length === 0) {
      return;
    }
    
    // Create batch
    const batch = {
      type: 'batch',
      id: uuidv4(),
      messages: [...this.messages],
      count: this.messages.length,
      timestamp: new Date().toISOString()
    };
    
    // Clear messages
    this.messages = [];
    
    // Reset batch start time
    this.batchStartTime = Date.now();
    
    // Emit batch
    this.emit('batch', batch, reason);
    
    logger.debug({
      event: 'message_batch_flushed',
      reason,
      count: batch.count,
      timestamp: batch.timestamp
    });
  }
  
  public forceFlush(): void {
    this.flush('force');
  }
  
  public clear(): void {
    this.messages = [];
    this.batchStartTime = Date.now();
    
    logger.debug({
      event: 'message_batch_cleared',
      timestamp: new Date().toISOString()
    });
  }
  
  public stop(): void {
    if (this.timer) {
      clearInterval(this.timer);
      this.timer = null;
    }
    
    // Flush any remaining messages
    this.flush('stop');
  }
  
  public getCount(): number {
    return this.messages.length;
  }
}
```

### 5. Integration with Express

Integrate the WebSocket server with Express:

```typescript
// src/index.ts
import http from 'http';
import app from './app';
import { WebSocketServer } from './websocket/server';
import { Config } from './config';
import logger from './utils/logger';

// Create HTTP server
const server = http.createServer(app);

// Create WebSocket server
const wsServer = new WebSocketServer(server);

// Store WebSocket server in app for use in routes
(app as any).wsServer = wsServer;

// Start server
server.listen(Config.server.port, () => {
  logger.info(`Server running on port ${Config.server.port}`);
  logger.info(`WebSocket server running on path: ${Config.websocket.path}`);
});

// Handle graceful shutdown
const shutdown = () => {
  logger.info('Shutting down server...');
  
  // Close WebSocket server
  wsServer.shutdown();
  
  // Close HTTP server
  server.close(() => {
    logger.info('HTTP server closed');
    process.exit(0);
  });
  
  // Force exit after timeout
  setTimeout(() => {
    logger.error('Forced shutdown after timeout');
    process.exit(1);
  }, 10000);
};

// Listen for termination signals
process.on('SIGTERM', shutdown);
process.on('SIGINT', shutdown);
```

## Implementation Steps

1. Install required dependencies:
   ```bash
   npm install ws uuid
   ```

2. Create the WebSocket server configuration
3. Implement the WebSocket authentication utility
4. Create the WebSocket client
5. Implement the message batching utility
6. Integrate with Express application
7. Document usage patterns for the team
