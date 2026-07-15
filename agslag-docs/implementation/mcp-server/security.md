# MCP Server Security Implementation

This document outlines the implementation plan for security enhancements in the MCP server.

## Overview

The security implementation will provide:
- Authentication and authorization
- API key management
- Secure WebSocket connections
- Rate limiting and request validation
- CORS configuration
- Input validation and sanitization

## Implementation Details

### 1. Authentication and Authorization

Create JWT-based authentication middleware:

```typescript
// src/middleware/auth-middleware.ts
import { Request, Response, NextFunction } from 'express';
import jwt from 'jsonwebtoken';
import { Config } from '../config';
import { UnauthorizedError, ForbiddenError } from '../errors/http-errors';
import logger from '../utils/logger';
import { User } from '../models/user';

// Interface for JWT payload
interface JwtPayload {
  id: string;
  email: string;
  role: string;
  iat: number;
  exp: number;
}

// Extend Express Request interface
declare global {
  namespace Express {
    interface Request {
      user?: JwtPayload;
      apiKey?: any;
    }
  }
}

// Authentication middleware
export const authenticate = async (req: Request, res: Response, next: NextFunction) => {
  try {
    // Get token from Authorization header
    const authHeader = req.headers.authorization;
    
    if (!authHeader || !authHeader.startsWith('Bearer ')) {
      // Check for API key as fallback
      const apiKey = req.headers['x-api-key'];
      
      if (apiKey) {
        return authenticateWithApiKey(req, res, next);
      }
      
      throw new UnauthorizedError('Authentication required');
    }
    
    // Extract token
    const token = authHeader.split(' ')[1];
    
    if (!token) {
      throw new UnauthorizedError('Invalid token format');
    }
    
    try {
      // Verify token
      const decoded = jwt.verify(token, Config.auth.jwt_secret) as JwtPayload;
      
      // Store user info in request
      req.user = decoded;
      
      // Log authentication
      logger.debug({
        message: 'User authenticated',
        userId: decoded.id,
        role: decoded.role,
        path: req.path,
        method: req.method
      });
      
      next();
    } catch (error) {
      if (error.name === 'TokenExpiredError') {
        throw new UnauthorizedError('Token expired');
      } else if (error.name === 'JsonWebTokenError') {
        throw new UnauthorizedError('Invalid token');
      } else {
        throw new UnauthorizedError('Authentication failed');
      }
    }
  } catch (error) {
    next(error);
  }
};

// API key authentication
export const authenticateWithApiKey = async (req: Request, res: Response, next: NextFunction) => {
  try {
    const apiKey = req.headers['x-api-key'] as string;
    
    if (!apiKey) {
      throw new UnauthorizedError('API key required');
    }
    
    // Find API key in database
    const key = await ApiKey.findOne({ key: apiKey, isActive: true });
    
    if (!key) {
      throw new UnauthorizedError('Invalid API key');
    }
    
    // Check if key is expired
    if (key.expiresAt && key.expiresAt < new Date()) {
      throw new UnauthorizedError('API key expired');
    }
    
    // Find user associated with API key
    const user = await User.findById(key.userId);
    
    if (!user) {
      throw new UnauthorizedError('User not found');
    }
    
    // Store user info in request
    req.user = {
      id: user._id.toString(),
      email: user.email,
      role: user.role,
      iat: Math.floor(Date.now() / 1000),
      exp: Math.floor(Date.now() / 1000) + 3600 // 1 hour
    };
    
    // Store API key in request
    req.apiKey = key;
    
    // Update last used timestamp
    key.lastUsed = new Date();
    await key.save();
    
    // Log authentication
    logger.debug({
      message: 'User authenticated with API key',
      userId: user._id.toString(),
      role: user.role,
      apiKeyId: key._id.toString(),
      path: req.path,
      method: req.method
    });
    
    next();
  } catch (error) {
    next(error);
  }
};

// Role-based authorization middleware
export const authorize = (roles: string | string[]) => {
  return (req: Request, res: Response, next: NextFunction) => {
    try {
      if (!req.user) {
        throw new UnauthorizedError('Authentication required');
      }
      
      const allowedRoles = Array.isArray(roles) ? roles : [roles];
      
      if (allowedRoles.length > 0 && !allowedRoles.includes(req.user.role)) {
        throw new ForbiddenError(`Access denied. Required role: ${allowedRoles.join(' or ')}`);
      }
      
      next();
    } catch (error) {
      next(error);
    }
  };
};

// Generate JWT token
export const generateToken = (user: any): string => {
  const payload = {
    id: user._id.toString(),
    email: user.email,
    role: user.role
  };
  
  return jwt.sign(payload, Config.auth.jwt_secret, {
    expiresIn: Config.auth.jwt_expiration
  });
};
```

### 2. API Key Management

Create API key model and service:

```typescript
// src/models/api-key.ts
import mongoose, { Document, Schema } from 'mongoose';
import { v4 as uuidv4 } from 'uuid';
import crypto from 'crypto';

export interface ApiKeyDocument extends Document {
  key: string;
  name: string;
  userId: mongoose.Types.ObjectId;
  scopes: string[];
  lastUsed: Date;
  expiresAt: Date | null;
  isActive: boolean;
  createdAt: Date;
  updatedAt: Date;
}

const ApiKeySchema = new Schema<ApiKeyDocument>(
  {
    key: {
      type: String,
      required: true,
      unique: true,
      default: () => {
        // Generate a secure random API key
        const buffer = crypto.randomBytes(32);
        return buffer.toString('hex');
      }
    },
    name: {
      type: String,
      required: true,
      trim: true
    },
    userId: {
      type: Schema.Types.ObjectId,
      ref: 'User',
      required: true,
      index: true
    },
    scopes: {
      type: [String],
      default: []
    },
    lastUsed: {
      type: Date,
      default: null
    },
    expiresAt: {
      type: Date,
      default: null
    },
    isActive: {
      type: Boolean,
      default: true
    }
  },
  {
    timestamps: true
  }
);

// Create indexes
ApiKeySchema.index({ key: 1 });
ApiKeySchema.index({ userId: 1, name: 1 }, { unique: true });
ApiKeySchema.index({ expiresAt: 1 }, { sparse: true });

export const ApiKey = mongoose.model<ApiKeyDocument>('ApiKey', ApiKeySchema);
```

```typescript
// src/services/api-key-service.ts
import { ApiKey, ApiKeyDocument } from '../models/api-key';
import { User } from '../models/user';
import { NotFoundError, BadRequestError } from '../errors/http-errors';
import logger from '../utils/logger';

export class ApiKeyService {
  // Create a new API key
  public static async createApiKey(
    userId: string,
    name: string,
    scopes: string[] = [],
    expiresInDays: number | null = null
  ): Promise<ApiKeyDocument> {
    try {
      // Check if user exists
      const user = await User.findById(userId);
      
      if (!user) {
        throw new NotFoundError(`User with ID ${userId} not found`);
      }
      
      // Check if key with same name already exists for this user
      const existingKey = await ApiKey.findOne({ userId, name });
      
      if (existingKey) {
        throw new BadRequestError(`API key with name '${name}' already exists for this user`);
      }
      
      // Calculate expiration date if provided
      let expiresAt = null;
      if (expiresInDays !== null && expiresInDays > 0) {
        expiresAt = new Date();
        expiresAt.setDate(expiresAt.getDate() + expiresInDays);
      }
      
      // Create new API key
      const apiKey = new ApiKey({
        name,
        userId,
        scopes,
        expiresAt
      });
      
      await apiKey.save();
      
      logger.info({
        message: 'API key created',
        keyId: apiKey._id.toString(),
        userId,
        name
      });
      
      return apiKey;
    } catch (error) {
      logger.error({
        message: 'API key creation error',
        error: error.message,
        stack: error.stack,
        userId,
        name
      });
      
      throw error;
    }
  }
  
  // Get API key by ID
  public static async getApiKeyById(id: string): Promise<ApiKeyDocument | null> {
    return ApiKey.findById(id);
  }
  
  // Get API keys for user
  public static async getApiKeysForUser(userId: string): Promise<ApiKeyDocument[]> {
    return ApiKey.find({ userId });
  }
  
  // Update API key
  public static async updateApiKey(
    id: string,
    updates: {
      name?: string;
      scopes?: string[];
      isActive?: boolean;
      expiresAt?: Date | null;
    }
  ): Promise<ApiKeyDocument | null> {
    try {
      const apiKey = await ApiKey.findById(id);
      
      if (!apiKey) {
        throw new NotFoundError(`API key with ID ${id} not found`);
      }
      
      // Update fields
      if (updates.name !== undefined) {
        apiKey.name = updates.name;
      }
      
      if (updates.scopes !== undefined) {
        apiKey.scopes = updates.scopes;
      }
      
      if (updates.isActive !== undefined) {
        apiKey.isActive = updates.isActive;
      }
      
      if (updates.expiresAt !== undefined) {
        apiKey.expiresAt = updates.expiresAt;
      }
      
      await apiKey.save();
      
      logger.info({
        message: 'API key updated',
        keyId: apiKey._id.toString(),
        userId: apiKey.userId.toString()
      });
      
      return apiKey;
    } catch (error) {
      logger.error({
        message: 'API key update error',
        error: error.message,
        stack: error.stack,
        keyId: id
      });
      
      throw error;
    }
  }
  
  // Regenerate API key
  public static async regenerateApiKey(id: string): Promise<ApiKeyDocument | null> {
    try {
      const apiKey = await ApiKey.findById(id);
      
      if (!apiKey) {
        throw new NotFoundError(`API key with ID ${id} not found`);
      }
      
      // Generate new key
      const buffer = crypto.randomBytes(32);
      apiKey.key = buffer.toString('hex');
      
      await apiKey.save();
      
      logger.info({
        message: 'API key regenerated',
        keyId: apiKey._id.toString(),
        userId: apiKey.userId.toString()
      });
      
      return apiKey;
    } catch (error) {
      logger.error({
        message: 'API key regeneration error',
        error: error.message,
        stack: error.stack,
        keyId: id
      });
      
      throw error;
    }
  }
  
  // Delete API key
  public static async deleteApiKey(id: string): Promise<boolean> {
    try {
      const result = await ApiKey.deleteOne({ _id: id });
      
      if (result.deletedCount === 0) {
        throw new NotFoundError(`API key with ID ${id} not found`);
      }
      
      logger.info({
        message: 'API key deleted',
        keyId: id
      });
      
      return true;
    } catch (error) {
      logger.error({
        message: 'API key deletion error',
        error: error.message,
        stack: error.stack,
        keyId: id
      });
      
      throw error;
    }
  }
  
  // Validate API key
  public static async validateApiKey(key: string): Promise<ApiKeyDocument | null> {
    try {
      const apiKey = await ApiKey.findOne({ key, isActive: true });
      
      if (!apiKey) {
        return null;
      }
      
      // Check if key is expired
      if (apiKey.expiresAt && apiKey.expiresAt < new Date()) {
        return null;
      }
      
      return apiKey;
    } catch (error) {
      logger.error({
        message: 'API key validation error',
        error: error.message,
        stack: error.stack
      });
      
      return null;
    }
  }
}
```

### 3. Rate Limiting

Create rate limiting middleware:

```typescript
// src/middleware/rate-limiter.ts
import { Request, Response, NextFunction } from 'express';
import { Redis } from '../database/redis';
import { TooManyRequestsError } from '../errors/http-errors';
import logger from '../utils/logger';

// Rate limit options
interface RateLimitOptions {
  windowMs: number;
  max: number;
  standardHeaders: boolean;
  legacyHeaders: boolean;
  keyGenerator: (req: Request) => string;
  handler: (req: Request, res: Response, next: NextFunction, options: RateLimitOptions) => void;
  skip: (req: Request) => boolean;
}

// Default options
const defaultOptions: RateLimitOptions = {
  windowMs: 60 * 1000, // 1 minute
  max: 100, // 100 requests per minute
  standardHeaders: true, // Return rate limit info in the `RateLimit-*` headers
  legacyHeaders: false, // Do not return rate limit info in the `X-RateLimit-*` headers
  keyGenerator: (req: Request) => {
    // Use IP address as default key
    return req.ip || 'unknown';
  },
  handler: (req: Request, res: Response, next: NextFunction, options: RateLimitOptions) => {
    next(new TooManyRequestsError(`Too many requests, please try again after ${Math.ceil(options.windowMs / 1000)} seconds`));
  },
  skip: (req: Request) => {
    // Skip rate limiting for health check endpoints
    return req.path.includes('/health');
  }
};

// Create rate limiter middleware
export const rateLimiter = (customOptions: Partial<RateLimitOptions> = {}) => {
  // Merge options
  const options = { ...defaultOptions, ...customOptions };
  
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      // Skip rate limiting if specified
      if (options.skip(req)) {
        return next();
      }
      
      // Get Redis client
      const redisClient = Redis.getClient();
      
      if (!redisClient) {
        // If Redis is not available, skip rate limiting
        logger.warn('Rate limiting skipped: Redis not available');
        return next();
      }
      
      // Generate key
      const key = `rate-limit:${options.keyGenerator(req)}`;
      
      // Get current count
      const current = await redisClient.get(key);
      const count = current ? parseInt(current, 10) : 0;
      
      // Check if limit exceeded
      if (count >= options.max) {
        // Set headers if enabled
        if (options.standardHeaders) {
          res.setHeader('RateLimit-Limit', options.max);
          res.setHeader('RateLimit-Remaining', 0);
          res.setHeader('RateLimit-Reset', Math.ceil((await redisClient.pttl(key)) / 1000));
        }
        
        if (options.legacyHeaders) {
          res.setHeader('X-RateLimit-Limit', options.max);
          res.setHeader('X-RateLimit-Remaining', 0);
          res.setHeader('X-RateLimit-Reset', Math.ceil((await redisClient.pttl(key)) / 1000));
        }
        
        // Call handler
        return options.handler(req, res, next, options);
      }
      
      // Increment count
      await redisClient.incr(key);
      
      // Set expiration if this is the first request
      if (count === 0) {
        await redisClient.pexpire(key, options.windowMs);
      }
      
      // Get remaining time to live
      const ttl = await redisClient.pttl(key);
      
      // Set headers if enabled
      if (options.standardHeaders) {
        res.setHeader('RateLimit-Limit', options.max);
        res.setHeader('RateLimit-Remaining', Math.max(0, options.max - count - 1));
        res.setHeader('RateLimit-Reset', Math.ceil(ttl / 1000));
      }
      
      if (options.legacyHeaders) {
        res.setHeader('X-RateLimit-Limit', options.max);
        res.setHeader('X-RateLimit-Remaining', Math.max(0, options.max - count - 1));
        res.setHeader('X-RateLimit-Reset', Math.ceil(ttl / 1000));
      }
      
      next();
    } catch (error) {
      logger.error({
        message: 'Rate limiting error',
        error: error.message,
        stack: error.stack,
        path: req.path,
        method: req.method,
        ip: req.ip
      });
      
      // If rate limiting fails, allow the request
      next();
    }
  };
};

// Create different rate limiters for different routes
export const apiLimiter = rateLimiter({
  windowMs: 60 * 1000, // 1 minute
  max: 100 // 100 requests per minute
});

export const authLimiter = rateLimiter({
  windowMs: 15 * 60 * 1000, // 15 minutes
  max: 10, // 10 requests per 15 minutes
  keyGenerator: (req: Request) => {
    // Use email or IP address
    return (req.body.email || req.ip || 'unknown').toLowerCase();
  }
});

export const websocketLimiter = rateLimiter({
  windowMs: 60 * 1000, // 1 minute
  max: 1000 // 1000 requests per minute
});
```

### 4. CORS Configuration

Create CORS middleware:

```typescript
// src/middleware/cors-middleware.ts
import cors from 'cors';
import { Config } from '../config';

// Parse CORS origin configuration
const parseOrigin = (origin: string): boolean | string | RegExp | (string | RegExp)[] => {
  if (origin === '*') {
    return true; // Allow any origin
  }
  
  if (origin === 'false') {
    return false; // Disable CORS
  }
  
  // Split by comma and trim
  return origin.split(',').map(o => o.trim());
};

// Create CORS middleware
export const corsMiddleware = cors({
  origin: parseOrigin(Config.server.cors_origin),
  methods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS'],
  allowedHeaders: ['Content-Type', 'Authorization', 'X-API-Key'],
  exposedHeaders: ['Content-Length', 'Content-Type'],
  credentials: true,
  maxAge: 86400 // 24 hours
});
```

### 5. Input Validation

Create validation middleware using Joi:

```typescript
// src/middleware/validation-middleware.ts
import { Request, Response, NextFunction } from 'express';
import Joi from 'joi';
import { BadRequestError } from '../errors/http-errors';

// Validation middleware
export const validate = (schema: Joi.ObjectSchema) => {
  return (req: Request, res: Response, next: NextFunction) => {
    try {
      // Validate request body
      const { error, value } = schema.validate(req.body, {
        abortEarly: false,
        stripUnknown: true
      });
      
      if (error) {
        // Format validation errors
        const errorMessage = error.details
          .map(detail => detail.message)
          .join(', ');
        
        throw new BadRequestError(errorMessage);
      }
      
      // Replace request body with validated value
      req.body = value;
      
      next();
    } catch (error) {
      next(error);
    }
  };
};

// Common validation schemas
export const schemas = {
  // User schemas
  user: {
    create: Joi.object({
      email: Joi.string().email().required(),
      password: Joi.string().min(8).required(),
      name: Joi.string().required(),
      role: Joi.string().valid('user', 'admin').default('user')
    }),
    update: Joi.object({
      email: Joi.string().email(),
      password: Joi.string().min(8),
      name: Joi.string(),
      role: Joi.string().valid('user', 'admin')
    }),
    login: Joi.object({
      email: Joi.string().email().required(),
      password: Joi.string().required()
    })
  },
  
  // API key schemas
  apiKey: {
    create: Joi.object({
      name: Joi.string().required(),
      scopes: Joi.array().items(Joi.string()),
      expiresInDays: Joi.number().integer().min(1).allow(null)
    }),
    update: Joi.object({
      name: Joi.string(),
      scopes: Joi.array().items(Joi.string()),
      isActive: Joi.boolean(),
      expiresInDays: Joi.number().integer().min(1).allow(null)
    })
  },
  
  // Agent schemas
  agent: {
    create: Joi.object({
      name: Joi.string().required(),
      type: Joi.string().required(),
      capabilities: Joi.array().items(Joi.string()),
      metadata: Joi.object()
    }),
    update: Joi.object({
      name: Joi.string(),
      type: Joi.string(),
      capabilities: Joi.array().items(Joi.string()),
      status: Joi.string().valid('active', 'inactive', 'error'),
      metadata: Joi.object()
    })
  }
};
```

### 6. Integration with Express

Integrate the security middleware with Express:

```typescript
// src/app.ts
import express from 'express';
import helmet from 'helmet';
import { errorHandler } from './middleware/error-handler';
import { notFoundHandler } from './middleware/not-found-handler';
import { requestLogger, responseBodyCapture } from './middleware/request-logger';
import { metricsMiddleware } from './middleware/metrics-middleware';
import { corsMiddleware } from './middleware/cors-middleware';
import { apiLimiter, authLimiter } from './middleware/rate-limiter';
import routes from './routes';
import authRoutes from './routes/auth';
import healthRoutes from './routes/health';
import metricsRoutes from './routes/metrics';
import logger from './utils/logger';

const app = express();

// Start request processing
logger.info('Initializing MCP Server');

// Security middleware
app.use(helmet());
app.use(corsMiddleware);

// Request logging middleware
app.use(requestLogger);
app.use(responseBodyCapture);

// Metrics middleware
app.use(metricsMiddleware);

// Body parsing middleware
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// Rate limiting
app.use('/api', apiLimiter);
app.use('/api/auth', authLimiter);

// Health check routes (no authentication)
app.use('/health', healthRoutes);

// Auth routes (no authentication)
app.use('/api/auth', authRoutes);

// Metrics routes (with authentication)
app.use('/metrics', metricsRoutes);

// API routes (with authentication)
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
   npm install jsonwebtoken cors helmet joi
   ```

2. Create the authentication and authorization middleware
3. Implement the API key model and service
4. Create the rate limiting middleware
5. Implement the CORS middleware
6. Create the input validation middleware
7. Integrate with Express application
8. Document usage patterns for the team
