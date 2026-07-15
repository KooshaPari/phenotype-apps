# Jarvis SWE Agent Memory Architecture - Technical Implementation (Part 1)

## 1. Memory System Overview

The Jarvis memory architecture implements a sophisticated hierarchical memory system with three primary components:

1. **Working Memory**: Short-term, high-accessibility memory for current context
2. **Episodic Memory**: Medium-term storage for sequences of events and experiences
3. **Semantic Memory**: Long-term storage for knowledge, concepts, and relationships

This document details the technical implementation of these components and their integration.

## 2. Core Data Models

### 2.1 Base Memory Item

```typescript
interface MemoryItem {
  id: string;                 // Unique memory identifier
  type: MemoryType;           // 'working', 'episodic', 'semantic'
  content: string | object;   // Memory content (text or structured data)
  embedding?: number[];       // Vector embedding (if applicable)
  metadata: {                 // Memory metadata
    source: string;           // Source of the memory (user, agent, tool)
    timestamp: Date;          // Creation timestamp
    lastAccessed?: Date;      // Last access timestamp
    accessCount: number;      // Number of times accessed
    importance: number;       // Importance score (0-1)
    tags: string[];           // Categorization tags
    task?: string;            // Associated task ID
    entities?: string[];      // Referenced entity IDs
    context?: string;         // Context identifier
  };
  relationships?: {           // Related memories
    id: string;               // Related memory ID
    type: string;             // Relationship type
    strength: number;         // Relationship strength (0-1)
  }[];
}
```

### 2.2 Working Memory Item

```typescript
interface WorkingMemoryItem extends MemoryItem {
  expiresAt?: Date;           // Expiration timestamp
  priority: number;           // Priority level (1-10)
  status: 'active' | 'background'; // Memory status
  session: string;            // Session identifier
}
```

### 2.3 Episodic Memory Item

```typescript
interface EpisodicMemoryItem extends MemoryItem {
  episode: {                  // Episode information
    id: string;               // Episode identifier
    title: string;            // Episode title
    startTime: Date;          // Episode start time
    endTime?: Date;           // Episode end time
    type: string;             // Episode type
    outcome?: string;         // Episode outcome
  };
  sequence: number;           // Sequence within episode
  actors: string[];           // Entities involved
  actions: string[];          // Actions performed
}
```

### 2.4 Semantic Memory Item

```typescript
interface SemanticMemoryItem extends MemoryItem {
  concept: {                  // Concept information
    name: string;             // Concept name
    type: string;             // Concept type
    definition: string;       // Concept definition
    examples?: string[];      // Concept examples
  };
  confidence: number;         // Confidence score (0-1)
  sources: string[];          // Source references
  lastVerified?: Date;        // Last verification timestamp
}
```

### 2.5 Knowledge Entity

```typescript
interface KnowledgeEntity {
  id: string;                 // Unique entity identifier
  type: string;               // Entity type (concept, code, file, etc.)
  name: string;               // Entity name
  description?: string;       // Entity description
  properties: {               // Entity properties
    [key: string]: any;       // Property values
  };
  metadata: {                 // Entity metadata
    source: string;           // Source of the entity
    timestamp: Date;          // Creation timestamp
    lastUpdated: Date;        // Last update timestamp
    confidence: number;       // Confidence score (0-1)
    importance: number;       // Importance score (0-1)
  };
}
```

### 2.6 Knowledge Relationship

```typescript
interface KnowledgeRelationship {
  id: string;                 // Unique relationship identifier
  type: string;               // Relationship type
  sourceId: string;           // Source entity ID
  targetId: string;           // Target entity ID
  properties: {               // Relationship properties
    [key: string]: any;       // Property values
  };
  metadata: {                 // Relationship metadata
    source: string;           // Source of the relationship
    timestamp: Date;          // Creation timestamp
    lastUpdated: Date;        // Last update timestamp
    confidence: number;       // Confidence score (0-1)
    importance: number;       // Importance score (0-1)
  };
}
```

## 3. Working Memory Implementation

### 3.1 Redis-Based Working Memory

Working memory is implemented using Redis for fast access and automatic expiration:

```typescript
class WorkingMemoryService {
  private redis: Redis.Redis;
  private namespace: string;
  private defaultTTL: number;
  
  constructor(config: WorkingMemoryConfig) {
    this.redis = new Redis(config.redisUri);
    this.namespace = config.namespace || 'jarvis:working-memory';
    this.defaultTTL = config.defaultTTL || 3600; // 1 hour default
  }
  
  async store(id: string, content: any, metadata: any, ttl?: number): Promise<string> {
    const memoryKey = `${this.namespace}:${id}`;
    
    const memoryItem: WorkingMemoryItem = {
      id,
      type: 'working',
      content,
      metadata: {
        ...metadata,
        timestamp: new Date(),
        accessCount: 0,
        importance: metadata.importance || 0.5,
        tags: metadata.tags || []
      },
      priority: metadata.priority || 5,
      status: metadata.status || 'active',
      session: metadata.session || 'default'
    };
    
    if (ttl || this.defaultTTL) {
      memoryItem.expiresAt = new Date(Date.now() + (ttl || this.defaultTTL) * 1000);
    }
    
    // Store in Redis with expiration
    await this.redis.set(
      memoryKey,
      JSON.stringify(memoryItem),
      'EX',
      ttl || this.defaultTTL
    );
    
    // Store in session index
    await this.redis.zadd(
      `${this.namespace}:session:${memoryItem.session}`,
      memoryItem.priority,
      id
    );
    
    // Set expiration on session index
    await this.redis.expire(
      `${this.namespace}:session:${memoryItem.session}`,
      ttl || this.defaultTTL
    );
    
    return id;
  }
  
  async retrieve(id: string): Promise<WorkingMemoryItem | null> {
    const memoryKey = `${this.namespace}:${id}`;
    const data = await this.redis.get(memoryKey);
    
    if (!data) return null;
    
    const memoryItem: WorkingMemoryItem = JSON.parse(data);
    
    // Update access count and last accessed timestamp
    memoryItem.metadata.accessCount += 1;
    memoryItem.metadata.lastAccessed = new Date();
    
    // Update in Redis
    await this.redis.set(
      memoryKey,
      JSON.stringify(memoryItem),
      'EX',
      this.getTTLRemaining(memoryKey)
    );
    
    return memoryItem;
  }
  
  async getSessionMemories(session: string, limit: number = 10): Promise<WorkingMemoryItem[]> {
    const sessionKey = `${this.namespace}:session:${session}`;
    
    // Get top priority items
    const ids = await this.redis.zrevrange(sessionKey, 0, limit - 1);
    
    const memories: WorkingMemoryItem[] = [];
    for (const id of ids) {
      const memory = await this.retrieve(id);
      if (memory) {
        memories.push(memory);
      }
    }
    
    return memories;
  }
  
  async update(id: string, updates: Partial<WorkingMemoryItem>): Promise<WorkingMemoryItem | null> {
    const memoryKey = `${this.namespace}:${id}`;
    const data = await this.redis.get(memoryKey);
    
    if (!data) return null;
    
    const memoryItem: WorkingMemoryItem = JSON.parse(data);
    
    // Apply updates
    if (updates.content !== undefined) {
      memoryItem.content = updates.content;
    }
    
    if (updates.metadata) {
      memoryItem.metadata = {
        ...memoryItem.metadata,
        ...updates.metadata,
        lastUpdated: new Date()
      };
    }
    
    if (updates.priority !== undefined) {
      memoryItem.priority = updates.priority;
      
      // Update priority in session index
      await this.redis.zadd(
        `${this.namespace}:session:${memoryItem.session}`,
        memoryItem.priority,
        id
      );
    }
    
    if (updates.status !== undefined) {
      memoryItem.status = updates.status;
    }
    
    // Update in Redis
    await this.redis.set(
      memoryKey,
      JSON.stringify(memoryItem),
      'EX',
      this.getTTLRemaining(memoryKey)
    );
    
    return memoryItem;
  }
  
  async delete(id: string): Promise<boolean> {
    const memoryKey = `${this.namespace}:${id}`;
    const data = await this.redis.get(memoryKey);
    
    if (!data) return false;
    
    const memoryItem: WorkingMemoryItem = JSON.parse(data);
    
    // Remove from session index
    await this.redis.zrem(
      `${this.namespace}:session:${memoryItem.session}`,
      id
    );
    
    // Delete from Redis
    await this.redis.del(memoryKey);
    
    return true;
  }
  
  private async getTTLRemaining(key: string): Promise<number> {
    const ttl = await this.redis.ttl(key);
    return ttl > 0 ? ttl : this.defaultTTL;
  }
}
```

### 3.2 Working Memory Management

```typescript
class WorkingMemoryManager {
  private workingMemory: WorkingMemoryService;
  private maxItems: number;
  
  constructor(config: WorkingMemoryManagerConfig) {
    this.workingMemory = new WorkingMemoryService(config.workingMemory);
    this.maxItems = config.maxItems || 100;
  }
  
  async addToWorkingMemory(content: any, metadata: any): Promise<string> {
    const id = uuidv4();
    
    // Store in working memory
    await this.workingMemory.store(id, content, metadata);
    
    // Check if we need to prune
    await this.pruneIfNeeded(metadata.session);
    
    return id;
  }
  
  private async pruneIfNeeded(session: string): Promise<void> {
    const sessionMemories = await this.workingMemory.getSessionMemories(session, 1000);
    
    if (sessionMemories.length > this.maxItems) {
      // Sort by importance and recency
      const sortedMemories = sessionMemories.sort((a, b) => {
        // Calculate score based on importance, priority, and recency
        const scoreA = a.metadata.importance * 0.4 + 
                      a.priority * 0.3 + 
                      (a.metadata.lastAccessed ? 
                        (Date.now() - a.metadata.lastAccessed.getTime()) / 3600000 : 0) * 0.3;
        
        const scoreB = b.metadata.importance * 0.4 + 
                      b.priority * 0.3 + 
                      (b.metadata.lastAccessed ? 
                        (Date.now() - b.metadata.lastAccessed.getTime()) / 3600000 : 0) * 0.3;
        
        return scoreB - scoreA; // Higher score first
      });
      
      // Keep top items, remove the rest
      const toRemove = sortedMemories.slice(this.maxItems);
      
      for (const memory of toRemove) {
        await this.workingMemory.delete(memory.id);
      }
    }
  }
}
```
