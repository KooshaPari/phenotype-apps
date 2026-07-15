# MCP Server Monitoring Implementation

This document outlines the implementation plan for monitoring and metrics collection in the MCP server.

## Overview

The monitoring system will provide:
- Health check endpoints
- System metrics collection (CPU, memory, network)
- Application metrics (request rate, error rate, response time)
- WebSocket connection metrics
- Agent activity metrics
- Prometheus integration for metrics exposure

## Implementation Details

### 1. Health Check Endpoint

Create a comprehensive health check endpoint:

```typescript
// src/routes/health.ts
import { Router } from 'express';
import os from 'os';
import { getDbStatus } from '../services/db-service';
import { getRedisStatus } from '../services/redis-service';
import { getNatsStatus } from '../services/nats-service';
import { getAgentStats } from '../services/agent-service';
import { getWebSocketStats } from '../services/websocket-service';
import { Config } from '../config';
import logger from '../utils/logger';

const router = Router();

// Simple health check
router.get('/', async (req, res) => {
  res.status(200).json({ status: 'UP' });
});

// Detailed health check
router.get('/details', async (req, res) => {
  try {
    // Get component statuses
    const [dbStatus, redisStatus, natsStatus, agentStats, wsStats] = await Promise.all([
      getDbStatus(),
      getRedisStatus(),
      getNatsStatus(),
      getAgentStats(),
      getWebSocketStats()
    ]);
    
    // Calculate overall status
    const overallStatus = [dbStatus.status, redisStatus.status, natsStatus.status]
      .every(status => status === 'UP') ? 'UP' : 'DOWN';
    
    // System information
    const systemInfo = {
      hostname: os.hostname(),
      platform: os.platform(),
      arch: os.arch(),
      cpus: os.cpus().length,
      memory: {
        total: os.totalmem(),
        free: os.freemem(),
        usage: (1 - os.freemem() / os.totalmem()) * 100
      },
      uptime: os.uptime(),
      loadAvg: os.loadavg()
    };
    
    // Process information
    const processInfo = {
      pid: process.pid,
      uptime: process.uptime(),
      memory: process.memoryUsage(),
      version: process.version,
      env: process.env.NODE_ENV
    };
    
    // Response
    const healthData = {
      status: overallStatus,
      timestamp: new Date().toISOString(),
      version: Config.version,
      components: {
        database: dbStatus,
        redis: redisStatus,
        nats: natsStatus
      },
      agents: agentStats,
      websocket: wsStats,
      system: systemInfo,
      process: processInfo
    };
    
    res.json(healthData);
  } catch (error) {
    logger.error({
      message: 'Health check failed',
      error: error.message,
      stack: error.stack
    });
    
    res.status(500).json({
      status: 'DOWN',
      timestamp: new Date().toISOString(),
      error: error.message
    });
  }
});

export default router;
```

### 2. Metrics Collection

Create a metrics service for collecting and exposing metrics:

```typescript
// src/services/metrics-service.ts
import client from 'prom-client';
import os from 'os';
import { Config } from '../config';

// Create a Registry to register metrics
const register = new client.Registry();

// Add default metrics
client.collectDefaultMetrics({ register });

// HTTP request duration metric
const httpRequestDurationMicroseconds = new client.Histogram({
  name: 'http_request_duration_seconds',
  help: 'Duration of HTTP requests in seconds',
  labelNames: ['method', 'route', 'status_code'],
  buckets: [0.01, 0.05, 0.1, 0.5, 1, 2, 5, 10]
});

// HTTP request counter
const httpRequestCounter = new client.Counter({
  name: 'http_requests_total',
  help: 'Total number of HTTP requests',
  labelNames: ['method', 'route', 'status_code']
});

// WebSocket connections gauge
const websocketConnectionsGauge = new client.Gauge({
  name: 'websocket_connections',
  help: 'Current number of WebSocket connections'
});

// WebSocket messages counter
const websocketMessagesCounter = new client.Counter({
  name: 'websocket_messages_total',
  help: 'Total number of WebSocket messages',
  labelNames: ['direction'] // 'incoming' or 'outgoing'
});

// Agent count gauge
const agentCountGauge = new client.Gauge({
  name: 'agents_count',
  help: 'Current number of registered agents',
  labelNames: ['status'] // 'active', 'inactive', 'error'
});

// Task count gauge
const taskCountGauge = new client.Gauge({
  name: 'tasks_count',
  help: 'Current number of tasks',
  labelNames: ['status'] // 'pending', 'running', 'completed', 'failed'
});

// Error counter
const errorCounter = new client.Counter({
  name: 'errors_total',
  help: 'Total number of errors',
  labelNames: ['type'] // 'client', 'server', 'network'
});

// Register metrics
register.registerMetric(httpRequestDurationMicroseconds);
register.registerMetric(httpRequestCounter);
register.registerMetric(websocketConnectionsGauge);
register.registerMetric(websocketMessagesCounter);
register.registerMetric(agentCountGauge);
register.registerMetric(taskCountGauge);
register.registerMetric(errorCounter);

// Export metrics service
export const MetricsService = {
  // Get metrics in Prometheus format
  getMetrics: async () => {
    return register.metrics();
  },
  
  // Record HTTP request
  recordHttpRequest: (method: string, route: string, statusCode: number, durationMs: number) => {
    httpRequestCounter.inc({ method, route, status_code: statusCode });
    httpRequestDurationMicroseconds.observe(
      { method, route, status_code: statusCode },
      durationMs / 1000 // Convert to seconds
    );
  },
  
  // Update WebSocket connections
  updateWebSocketConnections: (count: number) => {
    websocketConnectionsGauge.set(count);
  },
  
  // Record WebSocket message
  recordWebSocketMessage: (direction: 'incoming' | 'outgoing') => {
    websocketMessagesCounter.inc({ direction });
  },
  
  // Update agent counts
  updateAgentCounts: (active: number, inactive: number, error: number) => {
    agentCountGauge.set({ status: 'active' }, active);
    agentCountGauge.set({ status: 'inactive' }, inactive);
    agentCountGauge.set({ status: 'error' }, error);
  },
  
  // Update task counts
  updateTaskCounts: (pending: number, running: number, completed: number, failed: number) => {
    taskCountGauge.set({ status: 'pending' }, pending);
    taskCountGauge.set({ status: 'running' }, running);
    taskCountGauge.set({ status: 'completed' }, completed);
    taskCountGauge.set({ status: 'failed' }, failed);
  },
  
  // Record error
  recordError: (type: 'client' | 'server' | 'network') => {
    errorCounter.inc({ type });
  }
};
```

### 3. Metrics Middleware

Create middleware for collecting HTTP metrics:

```typescript
// src/middleware/metrics-middleware.ts
import { Request, Response, NextFunction } from 'express';
import { MetricsService } from '../services/metrics-service';
import onFinished from 'on-finished';

export const metricsMiddleware = (req: Request, res: Response, next: NextFunction) => {
  // Skip metrics for the metrics endpoint itself to avoid circular reporting
  if (req.path === '/metrics') {
    return next();
  }
  
  // Record start time
  const startTime = process.hrtime();
  
  // Process the request
  onFinished(res, (err, res) => {
    // Calculate duration
    const diff = process.hrtime(startTime);
    const durationMs = (diff[0] * 1e9 + diff[1]) / 1e6; // Convert to milliseconds
    
    // Get route pattern (replace params with placeholders)
    const route = req.route ? 
      req.baseUrl + req.route.path : 
      req.path;
    
    // Record metrics
    MetricsService.recordHttpRequest(
      req.method,
      route,
      res.statusCode,
      durationMs
    );
    
    // Record error if applicable
    if (res.statusCode >= 400) {
      const errorType = res.statusCode >= 500 ? 'server' : 'client';
      MetricsService.recordError(errorType);
    }
  });
  
  next();
};
```

### 4. Metrics Endpoint

Create an endpoint for exposing metrics:

```typescript
// src/routes/metrics.ts
import { Router } from 'express';
import { MetricsService } from '../services/metrics-service';
import { authenticate, authorize } from '../middleware/auth-middleware';

const router = Router();

// Metrics endpoint (protected with authentication)
router.get('/', authenticate, authorize(['admin']), async (req, res) => {
  try {
    const metrics = await MetricsService.getMetrics();
    res.set('Content-Type', 'text/plain');
    res.send(metrics);
  } catch (error) {
    res.status(500).send('Error collecting metrics');
  }
});

export default router;
```

### 5. WebSocket Monitoring Integration

Integrate metrics collection with WebSocket server:

```typescript
// src/websocket/server.ts
import { Server } from 'http';
import WebSocket from 'ws';
import { Config } from '../config';
import logger from '../utils/logger';
import { logWebSocketConnection, logWebSocketMessage } from '../utils/websocket-logger';
import { MetricsService } from '../services/metrics-service';

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
      
      // Update connection metrics
      MetricsService.updateWebSocketConnections(this.wss.clients.size);
      
      // Handle messages
      socket.on('message', (message) => {
        // Log incoming message
        logWebSocketMessage('incoming', message, socket, request);
        
        // Record message metric
        MetricsService.recordWebSocketMessage('incoming');
        
        // Process message
        this.handleMessage(message, socket, request);
      });
      
      // Handle disconnection
      socket.on('close', () => {
        // Update connection metrics
        MetricsService.updateWebSocketConnections(this.wss.clients.size);
      });
    });
    
    // Handle server errors
    this.wss.on('error', (error) => {
      logger.error({
        event: 'websocket_server_error',
        error: error.message,
        stack: error.stack,
        timestamp: new Date().toISOString()
      });
      
      // Record error metric
      MetricsService.recordError('server');
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
      
      // Record message metric
      MetricsService.recordWebSocketMessage('outgoing');
    } catch (error) {
      logger.error({
        event: 'websocket_message_processing_error',
        error: error.message,
        stack: error.stack,
        timestamp: new Date().toISOString()
      });
      
      // Record error metric
      MetricsService.recordError('server');
      
      // Send error response
      const errorResponse = { 
        status: 'error', 
        message: 'Failed to process message' 
      };
      socket.send(JSON.stringify(errorResponse));
      
      // Log outgoing error message
      logWebSocketMessage('outgoing', errorResponse, socket, request);
      
      // Record message metric
      MetricsService.recordWebSocketMessage('outgoing');
    }
  }
  
  public broadcast(message: any) {
    const messageStr = JSON.stringify(message);
    let sentCount = 0;
    
    this.wss.clients.forEach((client) => {
      if (client.readyState === WebSocket.OPEN) {
        client.send(messageStr);
        sentCount++;
        
        // Log outgoing broadcast message
        logWebSocketMessage('outgoing', message, client);
        
        // Record message metric
        MetricsService.recordWebSocketMessage('outgoing');
      }
    });
    
    logger.debug({
      event: 'websocket_broadcast',
      clientCount: this.wss.clients.size,
      sentCount,
      timestamp: new Date().toISOString()
    });
  }
  
  public getConnections(): number {
    return this.wss.clients.size;
  }
}
```

### 6. Agent Monitoring Integration

Integrate metrics collection with agent service:

```typescript
// src/services/agent-service.ts
import logger from '../utils/logger';
import { Agent } from '../models/agent';
import { MetricsService } from './metrics-service';

export class AgentService {
  // Update agent metrics
  async updateAgentMetrics() {
    try {
      // Count agents by status
      const [activeCount, inactiveCount, errorCount] = await Promise.all([
        Agent.countDocuments({ status: 'active' }),
        Agent.countDocuments({ status: 'inactive' }),
        Agent.countDocuments({ status: 'error' })
      ]);
      
      // Update metrics
      MetricsService.updateAgentCounts(activeCount, inactiveCount, errorCount);
      
      return {
        active: activeCount,
        inactive: inactiveCount,
        error: errorCount,
        total: activeCount + inactiveCount + errorCount
      };
    } catch (error) {
      logger.error({
        message: 'Failed to update agent metrics',
        error: error.message,
        stack: error.stack
      });
      
      throw error;
    }
  }
  
  // Get agent statistics
  async getAgentStats() {
    try {
      const stats = await this.updateAgentMetrics();
      
      // Get additional stats
      const latestAgents = await Agent.find()
        .sort({ lastSeen: -1 })
        .limit(5)
        .select('name type status lastSeen');
      
      return {
        counts: stats,
        latestAgents
      };
    } catch (error) {
      logger.error({
        message: 'Failed to get agent statistics',
        error: error.message,
        stack: error.stack
      });
      
      throw error;
    }
  }
}
```

### 7. Integration with Express

Integrate the monitoring components with Express:

```typescript
// src/app.ts
import express from 'express';
import { errorHandler } from './middleware/error-handler';
import { notFoundHandler } from './middleware/not-found-handler';
import { requestLogger, responseBodyCapture } from './middleware/request-logger';
import { metricsMiddleware } from './middleware/metrics-middleware';
import routes from './routes';
import healthRoutes from './routes/health';
import metricsRoutes from './routes/metrics';
import logger from './utils/logger';

const app = express();

// Start request processing
logger.info('Initializing MCP Server');

// Request logging middleware
app.use(requestLogger);
app.use(responseBodyCapture);

// Metrics middleware
app.use(metricsMiddleware);

// Body parsing middleware
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Health check routes
app.use('/health', healthRoutes);

// Metrics routes
app.use('/metrics', metricsRoutes);

// API routes
app.use('/api', routes);

// Handle 404 errors
app.use(notFoundHandler);

// Global error handler - must be last
app.use(errorHandler);

export default app;
```

## Implementation Steps

1. Install required dependencies:
   ```bash
   npm install prom-client on-finished
   ```

2. Create the health check endpoint
3. Implement the metrics service
4. Create the metrics middleware
5. Implement the metrics endpoint
6. Integrate with WebSocket server
7. Integrate with agent service
8. Integrate with Express application
9. Document usage patterns for the team
