# MCP Server Database Integration Implementation

This document outlines the implementation plan for database integration in the MCP server, focusing on MongoDB, Redis, and NATS.

## Overview

The database integration will provide:
- MongoDB for persistent storage
- Redis for real-time state and pub/sub
- NATS for high-throughput messaging
- Connection pooling and optimization
- Error handling and reconnection strategies
- Data models and schemas

## Implementation Details

### 1. MongoDB Integration

Create a MongoDB connection manager:

```typescript
// src/database/mongodb.ts
import mongoose from 'mongoose';
import { Config } from '../config';
import logger from '../utils/logger';

// Connection options
const connectionOptions = {
  useNewUrlParser: true,
  useUnifiedTopology: true,
  maxPoolSize: 10,
  serverSelectionTimeoutMS: 5000,
  socketTimeoutMS: 45000,
  connectTimeoutMS: 10000,
  heartbeatFrequencyMS: 10000,
  retryWrites: true,
  retryReads: true
};

// Connection states
const connectionStates = {
  0: 'disconnected',
  1: 'connected',
  2: 'connecting',
  3: 'disconnecting',
  99: 'uninitialized'
};

class MongoDBManager {
  private isConnected = false;
  private reconnectTimer: NodeJS.Timeout | null = null;
  private reconnectAttempts = 0;
  private readonly maxReconnectAttempts = 10;
  
  constructor() {
    // Set up connection event handlers
    mongoose.connection.on('connected', this.handleConnected.bind(this));
    mongoose.connection.on('error', this.handleError.bind(this));
    mongoose.connection.on('disconnected', this.handleDisconnected.bind(this));
    
    // Handle process termination
    process.on('SIGINT', this.handleTermination.bind(this));
    process.on('SIGTERM', this.handleTermination.bind(this));
  }
  
  public async connect(): Promise<boolean> {
    if (this.isConnected) {
      logger.info('MongoDB already connected');
      return true;
    }
    
    try {
      logger.info(`Connecting to MongoDB at ${this.getRedactedUri()}`);
      
      await mongoose.connect(Config.database.mongodb_uri, connectionOptions);
      
      // Connection is handled by event handlers
      return true;
    } catch (error) {
      logger.error({
        message: 'MongoDB connection error',
        error: error.message,
        stack: error.stack
      });
      
      this.scheduleReconnect();
      return false;
    }
  }
  
  private handleConnected() {
    this.isConnected = true;
    this.reconnectAttempts = 0;
    
    logger.info({
      message: 'Connected to MongoDB',
      host: mongoose.connection.host,
      port: mongoose.connection.port,
      name: mongoose.connection.name
    });
  }
  
  private handleError(error: Error) {
    logger.error({
      message: 'MongoDB connection error',
      error: error.message,
      stack: error.stack
    });
    
    if (this.isConnected) {
      this.isConnected = false;
      this.scheduleReconnect();
    }
  }
  
  private handleDisconnected() {
    if (this.isConnected) {
      this.isConnected = false;
      logger.warn('Disconnected from MongoDB');
      this.scheduleReconnect();
    }
  }
  
  private scheduleReconnect() {
    // Clear any existing timer
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
    }
    
    // Check if max reconnect attempts reached
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      logger.error({
        message: 'Max MongoDB reconnect attempts reached',
        attempts: this.reconnectAttempts
      });
      return;
    }
    
    // Increment reconnect attempts
    this.reconnectAttempts++;
    
    // Calculate backoff time (exponential backoff with jitter)
    const baseDelay = 1000; // 1 second
    const backoff = Math.min(30000, baseDelay * Math.pow(1.5, this.reconnectAttempts - 1));
    const jitter = Math.random() * 0.3 * backoff;
    const delay = backoff + jitter;
    
    logger.info({
      message: 'Scheduling MongoDB reconnect',
      attempt: this.reconnectAttempts,
      delay: Math.round(delay)
    });
    
    // Schedule reconnect
    this.reconnectTimer = setTimeout(() => {
      this.connect();
    }, delay);
  }
  
  private handleTermination() {
    logger.info('Closing MongoDB connection due to application termination');
    
    // Clear reconnect timer
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
    }
    
    // Close connection
    mongoose.connection.close()
      .then(() => {
        logger.info('MongoDB connection closed');
      })
      .catch((error) => {
        logger.error({
          message: 'Error closing MongoDB connection',
          error: error.message
        });
      });
  }
  
  public async disconnect(): Promise<void> {
    // Clear reconnect timer
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
    }
    
    // Close connection
    if (mongoose.connection.readyState !== 0) {
      await mongoose.connection.close();
      this.isConnected = false;
      logger.info('MongoDB connection closed');
    }
  }
  
  public getStatus(): { status: string; details: any } {
    const state = mongoose.connection.readyState;
    
    return {
      status: this.isConnected ? 'UP' : 'DOWN',
      details: {
        state: connectionStates[state] || 'unknown',
        host: mongoose.connection.host,
        port: mongoose.connection.port,
        name: mongoose.connection.name,
        models: Object.keys(mongoose.models),
        reconnectAttempts: this.reconnectAttempts
      }
    };
  }
  
  private getRedactedUri(): string {
    try {
      const uri = new URL(Config.database.mongodb_uri);
      
      // Redact password if present
      if (uri.password) {
        uri.password = '****';
      }
      
      return uri.toString();
    } catch (error) {
      // If URI parsing fails, return a generic string
      return 'mongodb://<redacted>';
    }
  }
}

// Create and export singleton instance
export const MongoDB = new MongoDBManager();
```

### 2. Redis Integration

Create a Redis connection manager:

```typescript
// src/database/redis.ts
import Redis from 'ioredis';
import { Config } from '../config';
import logger from '../utils/logger';

class RedisManager {
  private client: Redis | null = null;
  private subscriber: Redis | null = null;
  private publisher: Redis | null = null;
  private isConnected = false;
  private reconnectAttempts = 0;
  private readonly maxReconnectAttempts = 10;
  
  constructor() {
    // Initialize connections
    this.initializeConnections();
  }
  
  private initializeConnections() {
    try {
      logger.info(`Connecting to Redis at ${this.getRedactedUri()}`);
      
      // Create main client
      this.client = new Redis(Config.database.redis_uri, {
        maxRetriesPerRequest: 3,
        retryStrategy: (times) => {
          if (times > this.maxReconnectAttempts) {
            logger.error({
              message: 'Max Redis reconnect attempts reached',
              attempts: times
            });
            return null; // Stop retrying
          }
          
          const delay = Math.min(times * 50, 2000);
          return delay;
        },
        reconnectOnError: (error) => {
          const targetError = 'READONLY';
          if (error.message.includes(targetError)) {
            // Only reconnect on specific errors
            return true;
          }
          return false;
        }
      });
      
      // Set up event handlers for main client
      this.client.on('connect', () => {
        logger.info('Redis client connected');
      });
      
      this.client.on('ready', () => {
        this.isConnected = true;
        this.reconnectAttempts = 0;
        logger.info('Redis client ready');
      });
      
      this.client.on('error', (error) => {
        logger.error({
          message: 'Redis client error',
          error: error.message,
          stack: error.stack
        });
      });
      
      this.client.on('close', () => {
        this.isConnected = false;
        logger.warn('Redis client connection closed');
      });
      
      this.client.on('reconnecting', (delay) => {
        this.reconnectAttempts++;
        logger.info({
          message: 'Redis client reconnecting',
          attempt: this.reconnectAttempts,
          delay
        });
      });
      
      // Create subscriber client
      this.subscriber = this.client.duplicate();
      
      // Create publisher client
      this.publisher = this.client.duplicate();
      
      // Set up event handlers for subscriber
      this.subscriber.on('error', (error) => {
        logger.error({
          message: 'Redis subscriber error',
          error: error.message,
          stack: error.stack
        });
      });
      
      // Set up event handlers for publisher
      this.publisher.on('error', (error) => {
        logger.error({
          message: 'Redis publisher error',
          error: error.message,
          stack: error.stack
        });
      });
    } catch (error) {
      logger.error({
        message: 'Redis initialization error',
        error: error.message,
        stack: error.stack
      });
    }
  }
  
  public getClient(): Redis | null {
    return this.client;
  }
  
  public getSubscriber(): Redis | null {
    return this.subscriber;
  }
  
  public getPublisher(): Redis | null {
    return this.publisher;
  }
  
  public async subscribe(channel: string, callback: (channel: string, message: string) => void): Promise<void> {
    if (!this.subscriber) {
      throw new Error('Redis subscriber not initialized');
    }
    
    try {
      await this.subscriber.subscribe(channel);
      
      this.subscriber.on('message', (ch, message) => {
        if (ch === channel) {
          callback(ch, message);
        }
      });
      
      logger.info({
        message: 'Subscribed to Redis channel',
        channel
      });
    } catch (error) {
      logger.error({
        message: 'Redis subscription error',
        channel,
        error: error.message,
        stack: error.stack
      });
      
      throw error;
    }
  }
  
  public async publish(channel: string, message: string | object): Promise<number> {
    if (!this.publisher) {
      throw new Error('Redis publisher not initialized');
    }
    
    try {
      const messageStr = typeof message === 'string' ? message : JSON.stringify(message);
      const result = await this.publisher.publish(channel, messageStr);
      
      logger.debug({
        message: 'Published message to Redis channel',
        channel,
        recipients: result
      });
      
      return result;
    } catch (error) {
      logger.error({
        message: 'Redis publish error',
        channel,
        error: error.message,
        stack: error.stack
      });
      
      throw error;
    }
  }
  
  public async disconnect(): Promise<void> {
    const promises = [];
    
    if (this.client) {
      promises.push(this.client.quit());
    }
    
    if (this.subscriber && this.subscriber !== this.client) {
      promises.push(this.subscriber.quit());
    }
    
    if (this.publisher && this.publisher !== this.client) {
      promises.push(this.publisher.quit());
    }
    
    await Promise.all(promises);
    
    this.client = null;
    this.subscriber = null;
    this.publisher = null;
    this.isConnected = false;
    
    logger.info('Redis connections closed');
  }
  
  public getStatus(): { status: string; details: any } {
    return {
      status: this.isConnected ? 'UP' : 'DOWN',
      details: {
        connected: this.isConnected,
        reconnectAttempts: this.reconnectAttempts,
        clientStatus: this.client ? this.client.status : 'not_initialized',
        subscriberStatus: this.subscriber ? this.subscriber.status : 'not_initialized',
        publisherStatus: this.publisher ? this.publisher.status : 'not_initialized'
      }
    };
  }
  
  private getRedactedUri(): string {
    try {
      const uri = new URL(Config.database.redis_uri);
      
      // Redact password if present
      if (uri.password) {
        uri.password = '****';
      }
      
      return uri.toString();
    } catch (error) {
      // If URI parsing fails, return a generic string
      return 'redis://<redacted>';
    }
  }
}

// Create and export singleton instance
export const Redis = new RedisManager();
```

### 3. NATS Integration

Create a NATS connection manager:

```typescript
// src/database/nats.ts
import { connect, NatsConnection, Subscription, StringCodec } from 'nats';
import { Config } from '../config';
import logger from '../utils/logger';

// String codec for encoding/decoding messages
const sc = StringCodec();

class NatsManager {
  private connection: NatsConnection | null = null;
  private subscriptions: Map<string, Subscription> = new Map();
  private isConnected = false;
  private reconnectAttempts = 0;
  private readonly maxReconnectAttempts = 10;
  private reconnectTimer: NodeJS.Timeout | null = null;
  
  public async connect(): Promise<boolean> {
    if (this.isConnected && this.connection) {
      logger.info('NATS already connected');
      return true;
    }
    
    try {
      logger.info(`Connecting to NATS at ${Config.database.nats_uri}`);
      
      this.connection = await connect({
        servers: Config.database.nats_uri,
        maxReconnectAttempts: this.maxReconnectAttempts,
        reconnectTimeWait: 1000, // 1 second
        reconnectJitter: 100, // 100ms
        reconnectJitterTLS: 100, // 100ms
        reconnectDelayHandler: (attempts) => {
          // Exponential backoff with jitter
          const backoff = Math.min(5000, 1000 * Math.pow(1.5, attempts - 1));
          const jitter = Math.random() * 0.3 * backoff;
          return backoff + jitter;
        }
      });
      
      this.isConnected = true;
      this.reconnectAttempts = 0;
      
      logger.info({
        message: 'Connected to NATS',
        server: this.connection.getServer()
      });
      
      // Handle connection events
      (async () => {
        for await (const status of this.connection!.status()) {
          logger.info({
            message: 'NATS connection status update',
            type: status.type,
            data: status.data
          });
          
          if (status.type === 'disconnect') {
            this.isConnected = false;
            logger.warn('Disconnected from NATS');
          } else if (status.type === 'reconnect') {
            this.isConnected = true;
            logger.info({
              message: 'Reconnected to NATS',
              server: this.connection!.getServer()
            });
          } else if (status.type === 'error') {
            logger.error({
              message: 'NATS connection error',
              error: status.data
            });
          }
        }
      })().catch((error) => {
        logger.error({
          message: 'NATS status handler error',
          error: error.message,
          stack: error.stack
        });
      });
      
      // Handle connection close
      this.connection.closed()
        .then((err) => {
          this.isConnected = false;
          
          if (err) {
            logger.error({
              message: 'NATS connection closed with error',
              error: err.message,
              code: err.code
            });
            
            this.scheduleReconnect();
          } else {
            logger.info('NATS connection closed gracefully');
          }
        })
        .catch((error) => {
          logger.error({
            message: 'NATS closed handler error',
            error: error.message,
            stack: error.stack
          });
        });
      
      return true;
    } catch (error) {
      logger.error({
        message: 'NATS connection error',
        error: error.message,
        stack: error.stack
      });
      
      this.scheduleReconnect();
      return false;
    }
  }
  
  private scheduleReconnect() {
    // Clear any existing timer
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
    }
    
    // Check if max reconnect attempts reached
    if (this.reconnectAttempts >= this.maxReconnectAttempts) {
      logger.error({
        message: 'Max NATS reconnect attempts reached',
        attempts: this.reconnectAttempts
      });
      return;
    }
    
    // Increment reconnect attempts
    this.reconnectAttempts++;
    
    // Calculate backoff time (exponential backoff with jitter)
    const baseDelay = 1000; // 1 second
    const backoff = Math.min(30000, baseDelay * Math.pow(1.5, this.reconnectAttempts - 1));
    const jitter = Math.random() * 0.3 * backoff;
    const delay = backoff + jitter;
    
    logger.info({
      message: 'Scheduling NATS reconnect',
      attempt: this.reconnectAttempts,
      delay: Math.round(delay)
    });
    
    // Schedule reconnect
    this.reconnectTimer = setTimeout(() => {
      this.connect();
    }, delay);
  }
  
  public async subscribe(subject: string, callback: (data: any, subject: string, reply?: string) => void): Promise<void> {
    if (!this.connection) {
      throw new Error('NATS not connected');
    }
    
    try {
      // Create subscription
      const subscription = this.connection.subscribe(subject);
      
      // Store subscription
      this.subscriptions.set(subject, subscription);
      
      logger.info({
        message: 'Subscribed to NATS subject',
        subject
      });
      
      // Process messages
      (async () => {
        for await (const msg of subscription) {
          try {
            // Decode message
            const data = sc.decode(msg.data);
            
            // Parse JSON if possible
            let parsedData;
            try {
              parsedData = JSON.parse(data);
            } catch (error) {
              parsedData = data;
            }
            
            // Call callback
            callback(parsedData, msg.subject, msg.reply);
          } catch (error) {
            logger.error({
              message: 'NATS message handler error',
              subject,
              error: error.message,
              stack: error.stack
            });
          }
        }
        
        logger.info({
          message: 'NATS subscription ended',
          subject
        });
      })().catch((error) => {
        logger.error({
          message: 'NATS subscription handler error',
          subject,
          error: error.message,
          stack: error.stack
        });
      });
    } catch (error) {
      logger.error({
        message: 'NATS subscription error',
        subject,
        error: error.message,
        stack: error.stack
      });
      
      throw error;
    }
  }
  
  public async publish(subject: string, data: any, reply?: string): Promise<void> {
    if (!this.connection) {
      throw new Error('NATS not connected');
    }
    
    try {
      // Convert data to string if needed
      const message = typeof data === 'string' ? data : JSON.stringify(data);
      
      // Encode and publish message
      this.connection.publish(subject, sc.encode(message), { reply });
      
      logger.debug({
        message: 'Published message to NATS subject',
        subject,
        hasReply: !!reply
      });
    } catch (error) {
      logger.error({
        message: 'NATS publish error',
        subject,
        error: error.message,
        stack: error.stack
      });
      
      throw error;
    }
  }
  
  public async request(subject: string, data: any, timeout = 5000): Promise<any> {
    if (!this.connection) {
      throw new Error('NATS not connected');
    }
    
    try {
      // Convert data to string if needed
      const message = typeof data === 'string' ? data : JSON.stringify(data);
      
      // Encode and send request
      const response = await this.connection.request(subject, sc.encode(message), { timeout });
      
      // Decode response
      const responseData = sc.decode(response.data);
      
      // Parse JSON if possible
      try {
        return JSON.parse(responseData);
      } catch (error) {
        return responseData;
      }
    } catch (error) {
      logger.error({
        message: 'NATS request error',
        subject,
        error: error.message,
        stack: error.stack
      });
      
      throw error;
    }
  }
  
  public async unsubscribe(subject: string): Promise<void> {
    const subscription = this.subscriptions.get(subject);
    
    if (subscription) {
      try {
        subscription.unsubscribe();
        this.subscriptions.delete(subject);
        
        logger.info({
          message: 'Unsubscribed from NATS subject',
          subject
        });
      } catch (error) {
        logger.error({
          message: 'NATS unsubscribe error',
          subject,
          error: error.message,
          stack: error.stack
        });
        
        throw error;
      }
    }
  }
  
  public async disconnect(): Promise<void> {
    // Clear reconnect timer
    if (this.reconnectTimer) {
      clearTimeout(this.reconnectTimer);
      this.reconnectTimer = null;
    }
    
    if (this.connection) {
      try {
        // Drain connection (process pending messages before closing)
        await this.connection.drain();
        
        this.connection = null;
        this.isConnected = false;
        this.subscriptions.clear();
        
        logger.info('NATS connection closed');
      } catch (error) {
        logger.error({
          message: 'NATS disconnect error',
          error: error.message,
          stack: error.stack
        });
        
        throw error;
      }
    }
  }
  
  public getStatus(): { status: string; details: any } {
    return {
      status: this.isConnected ? 'UP' : 'DOWN',
      details: {
        connected: this.isConnected,
        server: this.connection ? this.connection.getServer() : null,
        reconnectAttempts: this.reconnectAttempts,
        subscriptions: Array.from(this.subscriptions.keys())
      }
    };
  }
}

// Create and export singleton instance
export const Nats = new NatsManager();
```

### 4. Database Service

Create a service for database operations:

```typescript
// src/services/db-service.ts
import { MongoDB } from '../database/mongodb';
import { Redis } from '../database/redis';
import { Nats } from '../database/nats';
import logger from '../utils/logger';

export class DbService {
  // Initialize all database connections
  public static async initialize(): Promise<boolean> {
    try {
      logger.info('Initializing database connections');
      
      // Connect to MongoDB
      const mongoConnected = await MongoDB.connect();
      
      // Redis is initialized in constructor
      const redisStatus = Redis.getStatus();
      
      // Connect to NATS
      const natsConnected = await Nats.connect();
      
      const allConnected = mongoConnected && redisStatus.status === 'UP' && natsConnected;
      
      if (allConnected) {
        logger.info('All database connections initialized successfully');
      } else {
        logger.warn('Some database connections failed to initialize');
      }
      
      return allConnected;
    } catch (error) {
      logger.error({
        message: 'Database initialization error',
        error: error.message,
        stack: error.stack
      });
      
      return false;
    }
  }
  
  // Get database status
  public static async getDbStatus(): Promise<{ status: string; details: any }> {
    const mongoStatus = MongoDB.getStatus();
    
    return mongoStatus;
  }
  
  // Get Redis status
  public static async getRedisStatus(): Promise<{ status: string; details: any }> {
    const redisStatus = Redis.getStatus();
    
    return redisStatus;
  }
  
  // Get NATS status
  public static async getNatsStatus(): Promise<{ status: string; details: any }> {
    const natsStatus = Nats.getStatus();
    
    return natsStatus;
  }
  
  // Shutdown all database connections
  public static async shutdown(): Promise<void> {
    logger.info('Shutting down database connections');
    
    try {
      // Disconnect from MongoDB
      await MongoDB.disconnect();
      
      // Disconnect from Redis
      await Redis.disconnect();
      
      // Disconnect from NATS
      await Nats.disconnect();
      
      logger.info('All database connections closed');
    } catch (error) {
      logger.error({
        message: 'Database shutdown error',
        error: error.message,
        stack: error.stack
      });
    }
  }
}

// Export functions for health checks
export const getDbStatus = DbService.getDbStatus;
export const getRedisStatus = DbService.getRedisStatus;
export const getNatsStatus = DbService.getNatsStatus;
```

### 5. Integration with Express

Integrate the database connections with Express:

```typescript
// src/index.ts
import http from 'http';
import app from './app';
import { WebSocketServer } from './websocket/server';
import { Config } from './config';
import logger from './utils/logger';
import { DbService } from './services/db-service';

// Initialize database connections
DbService.initialize()
  .then((success) => {
    if (!success) {
      logger.warn('Starting server with partial database connectivity');
    }
    
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
    const shutdown = async () => {
      logger.info('Shutting down server...');
      
      // Close WebSocket server
      wsServer.shutdown();
      
      // Close HTTP server
      server.close(() => {
        logger.info('HTTP server closed');
        
        // Shutdown database connections
        DbService.shutdown()
          .then(() => {
            process.exit(0);
          })
          .catch(() => {
            process.exit(1);
          });
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
  })
  .catch((error) => {
    logger.error({
      message: 'Failed to initialize server',
      error: error.message,
      stack: error.stack
    });
    
    process.exit(1);
  });
```

## Implementation Steps

1. Install required dependencies:
   ```bash
   npm install mongoose ioredis nats
   ```

2. Create the MongoDB connection manager
3. Implement the Redis connection manager
4. Create the NATS connection manager
5. Implement the database service
6. Integrate with Express application
7. Document usage patterns for the team
