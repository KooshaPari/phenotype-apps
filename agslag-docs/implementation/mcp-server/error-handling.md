# MCP Server Error Handling Implementation

This document outlines the implementation plan for enhanced error handling in the MCP server.

## Overview

The error handling system will provide:
- Structured error logging with severity levels
- Centralized error handling middleware
- Error classification and standardization
- Client-friendly error responses
- Error tracking and monitoring

## Implementation Details

### 1. Error Types and Classification

Create a hierarchy of error types to standardize error handling:

```typescript
// src/errors/base-error.ts
export class BaseError extends Error {
  public readonly name: string;
  public readonly httpCode: number;
  public readonly isOperational: boolean;
  public readonly errorCode: string;
  public readonly context?: Record<string, any>;

  constructor(
    name: string,
    httpCode: number,
    description: string,
    isOperational: boolean,
    errorCode: string,
    context?: Record<string, any>
  ) {
    super(description);
    Object.setPrototypeOf(this, new.target.prototype);

    this.name = name;
    this.httpCode = httpCode;
    this.isOperational = isOperational;
    this.errorCode = errorCode;
    this.context = context;

    Error.captureStackTrace(this);
  }
}
```

```typescript
// src/errors/api-error.ts
import { BaseError } from './base-error';

export class APIError extends BaseError {
  constructor(
    name: string,
    httpCode = 500,
    description = 'Internal server error',
    isOperational = true,
    errorCode = 'API_ERROR',
    context?: Record<string, any>
  ) {
    super(name, httpCode, description, isOperational, errorCode, context);
  }
}
```

```typescript
// src/errors/http-errors.ts
import { APIError } from './api-error';

export class BadRequestError extends APIError {
  constructor(description = 'Bad request', errorCode = 'BAD_REQUEST', context?: Record<string, any>) {
    super('BadRequestError', 400, description, true, errorCode, context);
  }
}

export class UnauthorizedError extends APIError {
  constructor(description = 'Unauthorized', errorCode = 'UNAUTHORIZED', context?: Record<string, any>) {
    super('UnauthorizedError', 401, description, true, errorCode, context);
  }
}

export class ForbiddenError extends APIError {
  constructor(description = 'Forbidden', errorCode = 'FORBIDDEN', context?: Record<string, any>) {
    super('ForbiddenError', 403, description, true, errorCode, context);
  }
}

export class NotFoundError extends APIError {
  constructor(description = 'Resource not found', errorCode = 'NOT_FOUND', context?: Record<string, any>) {
    super('NotFoundError', 404, description, true, errorCode, context);
  }
}

export class ConflictError extends APIError {
  constructor(description = 'Conflict', errorCode = 'CONFLICT', context?: Record<string, any>) {
    super('ConflictError', 409, description, true, errorCode, context);
  }
}

export class TooManyRequestsError extends APIError {
  constructor(description = 'Too many requests', errorCode = 'TOO_MANY_REQUESTS', context?: Record<string, any>) {
    super('TooManyRequestsError', 429, description, true, errorCode, context);
  }
}

export class InternalServerError extends APIError {
  constructor(description = 'Internal server error', errorCode = 'INTERNAL_SERVER_ERROR', context?: Record<string, any>) {
    super('InternalServerError', 500, description, false, errorCode, context);
  }
}
```

### 2. Error Handling Middleware

Create a centralized error handling middleware:

```typescript
// src/middleware/error-handler.ts
import { Request, Response, NextFunction } from 'express';
import { v4 as uuidv4 } from 'uuid';
import { BaseError } from '../errors/base-error';
import logger from '../utils/logger';

export const errorHandler = (
  err: Error | BaseError,
  req: Request,
  res: Response,
  next: NextFunction
) => {
  // Generate a unique error ID for tracking
  const errorId = uuidv4();
  
  // Default error values
  let statusCode = 500;
  let errorCode = 'UNKNOWN_ERROR';
  let errorMessage = 'An unexpected error occurred';
  let isOperational = false;
  
  // If it's our custom error type
  if (err instanceof BaseError) {
    statusCode = err.httpCode;
    errorCode = err.errorCode;
    errorMessage = err.message;
    isOperational = err.isOperational;
  }
  
  // Determine log level based on status code
  const logLevel = statusCode >= 500 ? 'error' : 'warn';
  
  // Log the error
  logger[logLevel]({
    errorId,
    message: err.message,
    stack: err.stack,
    statusCode,
    errorCode,
    path: req.path,
    method: req.method,
    ip: req.ip,
    userId: req.user?.id, // If you have user info in request
    timestamp: new Date().toISOString(),
    isOperational,
    context: err instanceof BaseError ? err.context : undefined
  });
  
  // Send appropriate response to client
  // Only send detailed error info for operational errors
  res.status(statusCode).json({
    error: {
      id: errorId,
      code: errorCode,
      message: isOperational ? errorMessage : 'An unexpected error occurred',
      // Only include stack trace in development
      ...(process.env.NODE_ENV === 'development' && { stack: err.stack })
    }
  });
  
  // If this is a critical error (non-operational), you might want to do something more
  if (!isOperational) {
    // Log to error monitoring service
    // Potentially restart the process in production
    // process.exit(1)
  }
};
```

### 3. Not Found Handler

Create a middleware to handle 404 errors:

```typescript
// src/middleware/not-found-handler.ts
import { Request, Response, NextFunction } from 'express';
import { NotFoundError } from '../errors/http-errors';

export const notFoundHandler = (req: Request, res: Response, next: NextFunction) => {
  next(new NotFoundError(`Route not found: ${req.method} ${req.path}`));
};
```

### 4. Async Handler Wrapper

Create a utility to handle async errors:

```typescript
// src/utils/async-handler.ts
import { Request, Response, NextFunction } from 'express';

type AsyncRequestHandler = (
  req: Request,
  res: Response,
  next: NextFunction
) => Promise<any>;

export const asyncHandler = (fn: AsyncRequestHandler) => {
  return (req: Request, res: Response, next: NextFunction) => {
    Promise.resolve(fn(req, res, next)).catch(next);
  };
};
```

### 5. Integration with Express

Integrate the error handling middleware with Express:

```typescript
// src/app.ts
import express from 'express';
import { errorHandler } from './middleware/error-handler';
import { notFoundHandler } from './middleware/not-found-handler';
import routes from './routes';

const app = express();

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

## Usage Examples

### Throwing Errors in Route Handlers

```typescript
// src/routes/agents.ts
import { Router } from 'express';
import { asyncHandler } from '../utils/async-handler';
import { BadRequestError, NotFoundError } from '../errors/http-errors';
import { AgentService } from '../services/agent-service';

const router = Router();
const agentService = new AgentService();

// Get all agents
router.get('/', asyncHandler(async (req, res) => {
  const agents = await agentService.getAllAgents();
  res.json(agents);
}));

// Get agent by ID
router.get('/:id', asyncHandler(async (req, res) => {
  const { id } = req.params;
  
  if (!id) {
    throw new BadRequestError('Agent ID is required');
  }
  
  const agent = await agentService.getAgentById(id);
  
  if (!agent) {
    throw new NotFoundError(`Agent with ID ${id} not found`);
  }
  
  res.json(agent);
}));

export default router;
```

## Testing

### Unit Tests for Error Handling

```typescript
// tests/middleware/error-handler.test.ts
import { Request, Response } from 'express';
import { errorHandler } from '../../src/middleware/error-handler';
import { BadRequestError, InternalServerError } from '../../src/errors/http-errors';

describe('Error Handler Middleware', () => {
  let mockRequest: Partial<Request>;
  let mockResponse: Partial<Response>;
  let nextFunction: jest.Mock;

  beforeEach(() => {
    mockRequest = {
      path: '/test',
      method: 'GET',
      ip: '127.0.0.1'
    };
    
    mockResponse = {
      status: jest.fn().mockReturnThis(),
      json: jest.fn()
    };
    
    nextFunction = jest.fn();
  });

  it('should handle operational errors correctly', () => {
    const error = new BadRequestError('Invalid input');
    
    errorHandler(
      error,
      mockRequest as Request,
      mockResponse as Response,
      nextFunction
    );
    
    expect(mockResponse.status).toHaveBeenCalledWith(400);
    expect(mockResponse.json).toHaveBeenCalledWith(
      expect.objectContaining({
        error: expect.objectContaining({
          code: 'BAD_REQUEST',
          message: 'Invalid input'
        })
      })
    );
  });

  it('should handle non-operational errors correctly', () => {
    const error = new InternalServerError();
    
    errorHandler(
      error,
      mockRequest as Request,
      mockResponse as Response,
      nextFunction
    );
    
    expect(mockResponse.status).toHaveBeenCalledWith(500);
    expect(mockResponse.json).toHaveBeenCalledWith(
      expect.objectContaining({
        error: expect.objectContaining({
          code: 'INTERNAL_SERVER_ERROR',
          message: 'An unexpected error occurred'
        })
      })
    );
  });
});
```

## Implementation Steps

1. Create the error classes in the `src/errors` directory
2. Implement the error handling middleware
3. Create the not found handler
4. Implement the async handler utility
5. Integrate with Express application
6. Write unit tests for error handling
7. Document usage patterns for the team
