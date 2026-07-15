# Jarvis SWE Agent Memory Architecture - Technical Implementation (Part 3)

## 5. Semantic Memory Implementation

### 5.1 Neo4j-Based Knowledge Graph

Semantic memory uses Neo4j for storing structured knowledge as a graph:

```typescript
class SemanticMemoryService {
  private driver: Neo4j.Driver;
  private collection: MongoDB.Collection;
  
  constructor(config: SemanticMemoryConfig) {
    // Neo4j for knowledge graph
    this.driver = Neo4j.driver(
      config.neo4jUri,
      Neo4j.auth.basic(config.neo4jUser, config.neo4jPassword)
    );
    
    // MongoDB for semantic memory items
    const client = new MongoDB.MongoClient(config.mongoUri);
    const db = client.db(config.database || 'jarvis');
    this.collection = db.collection(config.collection || 'semantic_memories');
    
    // Create indexes
    this.collection.createIndex({ id: 1 }, { unique: true });
    this.collection.createIndex({ 'concept.name': 1 });
    this.collection.createIndex({ 'metadata.tags': 1 });
  }
  
  async store(id: string, content: any, metadata: any): Promise<string> {
    const memoryItem: SemanticMemoryItem = {
      id,
      type: 'semantic',
      content,
      metadata: {
        ...metadata,
        timestamp: new Date(),
        accessCount: 0,
        importance: metadata.importance || 0.5,
        tags: metadata.tags || []
      },
      concept: {
        name: metadata.concept?.name || 'Unnamed Concept',
        type: metadata.concept?.type || 'general',
        definition: metadata.concept?.definition || ''
      },
      confidence: metadata.confidence || 0.8,
      sources: metadata.sources || []
    };
    
    // Store in MongoDB
    await this.collection.insertOne(memoryItem);
    
    // If this is a structured knowledge item, also store in knowledge graph
    if (metadata.storeInGraph !== false && metadata.concept) {
      await this.storeInKnowledgeGraph(memoryItem);
    }
    
    return id;
  }
  
  private async storeInKnowledgeGraph(memory: SemanticMemoryItem): Promise<void> {
    const session = this.driver.session();
    
    try {
      // Create or update concept node
      await session.run(
        `
        MERGE (c:Concept {name: $name})
        ON CREATE SET 
          c.type = $type,
          c.definition = $definition,
          c.created = datetime(),
          c.memoryId = $memoryId
        ON MATCH SET 
          c.definition = $definition,
          c.updated = datetime(),
          c.memoryId = $memoryId
        RETURN c
        `,
        {
          name: memory.concept.name,
          type: memory.concept.type,
          definition: memory.concept.definition,
          memoryId: memory.id
        }
      );
      
      // Extract entities and relationships if content is structured
      if (typeof memory.content === 'object' && memory.content !== null) {
        if (memory.content.entities) {
          for (const entity of memory.content.entities) {
            // Create entity node
            await session.run(
              `
              MERGE (e:Entity {name: $name})
              ON CREATE SET 
                e.type = $type,
                e.created = datetime()
              ON MATCH SET 
                e.updated = datetime()
              RETURN e
              `,
              {
                name: entity.name,
                type: entity.type
              }
            );
            
            // Create relationship to concept
            await session.run(
              `
              MATCH (c:Concept {name: $conceptName})
              MATCH (e:Entity {name: $entityName})
              MERGE (c)-[r:RELATES_TO]->(e)
              ON CREATE SET 
                r.created = datetime(),
                r.confidence = $confidence
              ON MATCH SET 
                r.updated = datetime(),
                r.confidence = $confidence
              RETURN r
              `,
              {
                conceptName: memory.concept.name,
                entityName: entity.name,
                confidence: memory.confidence
              }
            );
          }
        }
        
        if (memory.content.relationships) {
          for (const rel of memory.content.relationships) {
            // Create relationship between entities
            await session.run(
              `
              MATCH (source:Entity {name: $sourceName})
              MATCH (target:Entity {name: $targetName})
              MERGE (source)-[r:${rel.type}]->(target)
              ON CREATE SET 
                r.created = datetime(),
                r.confidence = $confidence
              ON MATCH SET 
                r.updated = datetime(),
                r.confidence = $confidence
              RETURN r
              `,
              {
                sourceName: rel.source,
                targetName: rel.target,
                confidence: memory.confidence
              }
            );
          }
        }
      }
    } finally {
      await session.close();
    }
  }
  
  async retrieve(id: string): Promise<SemanticMemoryItem | null> {
    const memoryItem = await this.collection.findOne({ id });
    
    if (!memoryItem) return null;
    
    // Update access count and last accessed timestamp
    await this.collection.updateOne(
      { id },
      { 
        $inc: { 'metadata.accessCount': 1 },
        $set: { 'metadata.lastAccessed': new Date() }
      }
    );
    
    return memoryItem as SemanticMemoryItem;
  }
  
  async searchConcepts(query: string, limit: number = 10): Promise<SemanticMemoryItem[]> {
    const memories = await this.collection
      .find({ 
        $or: [
          { 'concept.name': { $regex: query, $options: 'i' } },
          { 'concept.definition': { $regex: query, $options: 'i' } }
        ]
      })
      .sort({ 'metadata.importance': -1 })
      .limit(limit)
      .toArray();
    
    return memories as SemanticMemoryItem[];
  }
  
  async queryKnowledgeGraph(query: string): Promise<any[]> {
    const session = this.driver.session();
    
    try {
      // Extract key entities from query using LLM or NLP
      const entities = await this.extractEntitiesFromQuery(query);
      
      // Build Cypher query based on extracted entities
      let cypherQuery = `
        MATCH (c:Concept)-[r]-(e)
        WHERE c.name IN $entities OR e.name IN $entities
        RETURN c, r, e
        LIMIT 50
      `;
      
      const result = await session.run(cypherQuery, { entities });
      
      // Transform Neo4j result to more usable format
      return result.records.map(record => {
        const concept = record.get('c').properties;
        const entity = record.get('e').properties;
        const relationship = record.get('r').properties;
        
        return {
          concept: {
            name: concept.name,
            type: concept.type,
            definition: concept.definition
          },
          entity: {
            name: entity.name,
            type: entity.type
          },
          relationship: {
            type: record.get('r').type,
            ...relationship
          }
        };
      });
    } finally {
      await session.close();
    }
  }
  
  private async extractEntitiesFromQuery(query: string): Promise<string[]> {
    // This would typically use NLP or an LLM to extract entities
    // Simplified implementation for now
    const words = query.split(/\s+/);
    return words
      .filter(word => word.length > 3)
      .map(word => word.replace(/[^\w]/g, ''));
  }
  
  async update(id: string, updates: Partial<SemanticMemoryItem>): Promise<SemanticMemoryItem | null> {
    const updateDoc: any = { $set: {} };
    
    if (updates.content !== undefined) {
      updateDoc.$set.content = updates.content;
    }
    
    if (updates.metadata) {
      Object.keys(updates.metadata).forEach(key => {
        updateDoc.$set[`metadata.${key}`] = updates.metadata[key];
      });
      updateDoc.$set['metadata.lastUpdated'] = new Date();
    }
    
    if (updates.concept) {
      Object.keys(updates.concept).forEach(key => {
        updateDoc.$set[`concept.${key}`] = updates.concept[key];
      });
    }
    
    if (updates.confidence !== undefined) {
      updateDoc.$set.confidence = updates.confidence;
    }
    
    if (updates.sources !== undefined) {
      updateDoc.$set.sources = updates.sources;
    }
    
    const result = await this.collection.updateOne({ id }, updateDoc);
    
    if (result.modifiedCount === 0) return null;
    
    const updatedMemory = await this.retrieve(id);
    
    // Update knowledge graph if needed
    if (updatedMemory && (updates.concept || updates.content)) {
      await this.storeInKnowledgeGraph(updatedMemory);
    }
    
    return updatedMemory;
  }
  
  async delete(id: string): Promise<boolean> {
    const memory = await this.retrieve(id);
    
    if (!memory) return false;
    
    // Remove from MongoDB
    const result = await this.collection.deleteOne({ id });
    
    // Remove from knowledge graph if applicable
    if (memory.concept) {
      const session = this.driver.session();
      try {
        await session.run(
          `
          MATCH (c:Concept {memoryId: $memoryId})
          DETACH DELETE c
          `,
          { memoryId: id }
        );
      } finally {
        await session.close();
      }
    }
    
    return result.deletedCount > 0;
  }
  
  async createSemanticMemoryFromInsight(insight: string, sourceId: string, llmService: LLMService): Promise<string> {
    // Use LLM to structure the insight
    const structuringPrompt = `
      Convert the following insight into a structured knowledge representation:
      
      "${insight}"
      
      Format your response as a JSON object with the following structure:
      {
        "concept": {
          "name": "Main concept name",
          "type": "Concept type (e.g., principle, pattern, observation)",
          "definition": "Clear definition of the concept"
        },
        "entities": [
          {
            "name": "Entity name",
            "type": "Entity type"
          }
        ],
        "relationships": [
          {
            "source": "Source entity name",
            "target": "Target entity name",
            "type": "RELATIONSHIP_TYPE"
          }
        ]
      }
    `;
    
    const structuredResponse = await llmService.generateCompletion(structuringPrompt, 'knowledge_structuring');
    
    try {
      const structuredKnowledge = JSON.parse(structuredResponse);
      
      // Create semantic memory
      const id = uuidv4();
      await this.store(id, structuredKnowledge, {
        concept: structuredKnowledge.concept,
        importance: 0.7,
        confidence: 0.8,
        sources: [sourceId],
        tags: ['insight', structuredKnowledge.concept.type],
        storeInGraph: true
      });
      
      return id;
    } catch (error) {
      console.error('Failed to create semantic memory from insight:', error);
      return '';
    }
  }
}
```

### 5.2 Knowledge Validation

```typescript
class KnowledgeValidator {
  private semanticMemory: SemanticMemoryService;
  private llmService: LLMService;
  
  constructor(config: KnowledgeValidatorConfig) {
    this.semanticMemory = config.semanticMemory;
    this.llmService = config.llmService;
  }
  
  async validateKnowledge(memoryId: string): Promise<{ valid: boolean, confidence: number, conflicts?: string[] }> {
    const memory = await this.semanticMemory.retrieve(memoryId);
    
    if (!memory) {
      return { valid: false, confidence: 0 };
    }
    
    // Find potentially conflicting memories
    const relatedMemories = await this.semanticMemory.searchConcepts(memory.concept.name, 5);
    const otherMemories = relatedMemories.filter(m => m.id !== memoryId);
    
    if (otherMemories.length === 0) {
      // No other memories to compare with, consider valid but with lower confidence
      return { valid: true, confidence: 0.6 };
    }
    
    // Format memories for comparison
    const memoryText = `
      Concept: ${memory.concept.name}
      Type: ${memory.concept.type}
      Definition: ${memory.concept.definition}
      Content: ${JSON.stringify(memory.content)}
    `;
    
    const otherMemoriesText = otherMemories.map(m => `
      Concept: ${m.concept.name}
      Type: ${m.concept.type}
      Definition: ${m.concept.definition}
      Content: ${JSON.stringify(m.content)}
      Confidence: ${m.confidence}
    `).join('\n\n');
    
    // Use LLM to validate
    const validationPrompt = `
      Evaluate whether the following knowledge is consistent with existing knowledge:
      
      NEW KNOWLEDGE:
      ${memoryText}
      
      EXISTING KNOWLEDGE:
      ${otherMemoriesText}
      
      Determine if the new knowledge:
      1. Is consistent with existing knowledge
      2. Contradicts existing knowledge
      3. Partially conflicts with existing knowledge
      
      If there are conflicts, identify them specifically.
      
      Format your response as a JSON object:
      {
        "valid": true/false,
        "confidence": 0.0-1.0,
        "conflicts": ["Description of conflict 1", "Description of conflict 2"]
      }
    `;
    
    const validationResponse = await this.llmService.generateCompletion(validationPrompt, 'knowledge_validation');
    
    try {
      const validation = JSON.parse(validationResponse);
      
      // Update memory confidence based on validation
      await this.semanticMemory.update(memoryId, {
        confidence: validation.valid ? 
          Math.max(memory.confidence, validation.confidence) : 
          Math.min(memory.confidence, validation.confidence)
      });
      
      return validation;
    } catch (error) {
      console.error('Failed to validate knowledge:', error);
      return { valid: true, confidence: memory.confidence };
    }
  }
  
  async resolveConflicts(memoryIds: string[]): Promise<string> {
    const memories = await Promise.all(
      memoryIds.map(id => this.semanticMemory.retrieve(id))
    );
    
    const validMemories = memories.filter(m => m !== null) as SemanticMemoryItem[];
    
    if (validMemories.length < 2) {
      return '';
    }
    
    // Format memories for resolution
    const memoriesText = validMemories.map(m => `
      Memory ID: ${m.id}
      Concept: ${m.concept.name}
      Type: ${m.concept.type}
      Definition: ${m.concept.definition}
      Content: ${JSON.stringify(m.content)}
      Confidence: ${m.confidence}
      Sources: ${m.sources.join(', ')}
    `).join('\n\n');
    
    // Use LLM to resolve conflicts
    const resolutionPrompt = `
      The following knowledge items have conflicts:
      
      ${memoriesText}
      
      Please resolve these conflicts by creating a new, unified knowledge representation that:
      1. Incorporates the most reliable information from each source
      2. Resolves contradictions by favoring higher confidence information
      3. Notes any uncertainties or areas where information is incomplete
      
      Format your response as a JSON object with the following structure:
      {
        "concept": {
          "name": "Main concept name",
          "type": "Concept type",
          "definition": "Clear definition that resolves conflicts"
        },
        "entities": [
          {
            "name": "Entity name",
            "type": "Entity type"
          }
        ],
        "relationships": [
          {
            "source": "Source entity name",
            "target": "Target entity name",
            "type": "RELATIONSHIP_TYPE"
          }
        ],
        "resolution_notes": "Explanation of how conflicts were resolved"
      }
    `;
    
    const resolutionResponse = await this.llmService.generateCompletion(resolutionPrompt, 'conflict_resolution');
    
    try {
      const resolution = JSON.parse(resolutionResponse);
      
      // Create new memory with resolved knowledge
      const id = uuidv4();
      await this.semanticMemory.store(id, {
        ...resolution,
        resolution_notes: resolution.resolution_notes
      }, {
        concept: resolution.concept,
        importance: Math.max(...validMemories.map(m => m.metadata.importance)),
        confidence: 0.9, // High confidence for resolved knowledge
        sources: validMemories.map(m => m.id),
        tags: ['resolved', resolution.concept.type],
        storeInGraph: true
      });
      
      // Update original memories to reference resolution
      for (const memory of validMemories) {
        await this.semanticMemory.update(memory.id, {
          metadata: {
            ...memory.metadata,
            resolvedBy: id
          }
        });
      }
      
      return id;
    } catch (error) {
      console.error('Failed to resolve knowledge conflicts:', error);
      return '';
    }
  }
}
```
