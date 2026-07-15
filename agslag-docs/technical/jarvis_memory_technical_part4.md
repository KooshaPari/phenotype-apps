# Jarvis SWE Agent Memory Architecture - Technical Implementation (Part 4)

## 6. Vector Store Implementation

### 6.1 Pinecone-Based Vector Store

The vector store component uses Pinecone for efficient similarity search:

```typescript
class VectorStore {
  private pinecone: PineconeClient;
  private indexName: string;
  private namespace: string;
  private dimension: number;
  
  constructor(config: VectorStoreConfig) {
    this.pinecone = new PineconeClient();
    this.indexName = config.indexName;
    this.namespace = config.namespace || 'default';
    this.dimension = config.dimension || 1536; // Default for OpenAI embeddings
  }
  
  async initialize(): Promise<void> {
    await this.pinecone.init({
      apiKey: process.env.PINECONE_API_KEY || '',
      environment: process.env.PINECONE_ENVIRONMENT || ''
    });
    
    // Check if index exists, create if not
    const indexList = await this.pinecone.listIndexes();
    if (!indexList.includes(this.indexName)) {
      await this.pinecone.createIndex({
        createRequest: {
          name: this.indexName,
          dimension: this.dimension,
          metric: 'cosine'
        }
      });
    }
  }
  
  async storeEmbedding(id: string, vector: number[], metadata: any): Promise<void> {
    const index = this.pinecone.Index(this.indexName);
    await index.upsert({
      upsertRequest: {
        vectors: [{
          id,
          values: vector,
          metadata
        }],
        namespace: this.namespace
      }
    });
  }
  
  async searchSimilar(vector: number[], topK: number = 5, filter?: any): Promise<SearchResult[]> {
    const index = this.pinecone.Index(this.indexName);
    const queryResponse = await index.query({
      queryRequest: {
        vector,
        topK,
        includeMetadata: true,
        namespace: this.namespace,
        filter
      }
    });
    
    return queryResponse.matches.map(match => ({
      id: match.id,
      score: match.score,
      metadata: match.metadata
    }));
  }
  
  async deleteEmbedding(id: string): Promise<void> {
    const index = this.pinecone.Index(this.indexName);
    await index.delete({
      deleteRequest: {
        ids: [id],
        namespace: this.namespace
      }
    });
  }
  
  async deleteByFilter(filter: any): Promise<void> {
    const index = this.pinecone.Index(this.indexName);
    await index.delete({
      deleteRequest: {
        filter,
        namespace: this.namespace
      }
    });
  }
  
  async updateMetadata(id: string, metadata: any): Promise<void> {
    // Pinecone doesn't support updating just metadata
    // Need to fetch the vector, then update the whole record
    const index = this.pinecone.Index(this.indexName);
    const fetchResponse = await index.fetch({
      ids: [id],
      namespace: this.namespace
    });
    
    const vector = fetchResponse.vectors[id];
    if (!vector) {
      throw new Error(`Vector with ID ${id} not found`);
    }
    
    await this.storeEmbedding(id, vector.values, {
      ...vector.metadata,
      ...metadata
    });
  }
  
  async getStats(): Promise<any> {
    const index = this.pinecone.Index(this.indexName);
    const statsResponse = await index.describeIndexStats({
      describeIndexStatsRequest: {
        filter: {}
      }
    });
    
    return statsResponse;
  }
}
```

### 6.2 Embedding Service

```typescript
class EmbeddingService {
  private openai: OpenAIApi;
  private model: string;
  private batchSize: number;
  private cache: Map<string, number[]>;
  private cacheSize: number;
  
  constructor(config: EmbeddingServiceConfig) {
    const configuration = new Configuration({
      apiKey: process.env.OPENAI_API_KEY
    });
    this.openai = new OpenAIApi(configuration);
    this.model = config.model || 'text-embedding-ada-002';
    this.batchSize = config.batchSize || 100;
    this.cache = new Map();
    this.cacheSize = config.cacheSize || 1000;
  }
  
  async createEmbedding(text: string): Promise<number[]> {
    // Check cache first
    const cacheKey = `${this.model}:${text}`;
    if (this.cache.has(cacheKey)) {
      return this.cache.get(cacheKey)!;
    }
    
    // Create embedding
    const response = await this.openai.createEmbedding({
      model: this.model,
      input: text
    });
    
    const embedding = response.data.data[0].embedding;
    
    // Update cache
    this.cache.set(cacheKey, embedding);
    
    // Prune cache if needed
    if (this.cache.size > this.cacheSize) {
      const oldestKey = this.cache.keys().next().value;
      this.cache.delete(oldestKey);
    }
    
    return embedding;
  }
  
  async createBatchEmbeddings(texts: string[]): Promise<number[][]> {
    // Process in batches to respect API limits
    const embeddings: number[][] = [];
    
    for (let i = 0; i < texts.length; i += this.batchSize) {
      const batch = texts.slice(i, i + this.batchSize);
      
      // Check cache for each text
      const uncachedIndices: number[] = [];
      const uncachedTexts: string[] = [];
      const batchEmbeddings: (number[] | null)[] = batch.map(() => null);
      
      batch.forEach((text, index) => {
        const cacheKey = `${this.model}:${text}`;
        if (this.cache.has(cacheKey)) {
          batchEmbeddings[index] = this.cache.get(cacheKey)!;
        } else {
          uncachedIndices.push(index);
          uncachedTexts.push(text);
        }
      });
      
      // Create embeddings for uncached texts
      if (uncachedTexts.length > 0) {
        const response = await this.openai.createEmbedding({
          model: this.model,
          input: uncachedTexts
        });
        
        // Update batch embeddings and cache
        response.data.data.forEach((item, index) => {
          const batchIndex = uncachedIndices[index];
          const text = batch[batchIndex];
          const embedding = item.embedding;
          
          batchEmbeddings[batchIndex] = embedding;
          
          // Update cache
          const cacheKey = `${this.model}:${text}`;
          this.cache.set(cacheKey, embedding);
        });
      }
      
      // Add batch embeddings to result
      embeddings.push(...batchEmbeddings.filter(e => e !== null) as number[][]);
    }
    
    // Prune cache if needed
    while (this.cache.size > this.cacheSize) {
      const oldestKey = this.cache.keys().next().value;
      this.cache.delete(oldestKey);
    }
    
    return embeddings;
  }
}
```

## 7. Memory Manager Implementation

### 7.1 Unified Memory Manager

The Memory Manager coordinates all memory components:

```typescript
class MemoryManager {
  private workingMemory: WorkingMemoryService;
  private episodicMemory: EpisodicMemoryService;
  private semanticMemory: SemanticMemoryService;
  private vectorStore: VectorStore;
  private embeddingService: EmbeddingService;
  private llmService: LLMService;
  private episodeBoundaryDetector: EpisodeBoundaryDetector;
  private knowledgeValidator: KnowledgeValidator;
  private currentEpisodeId: string;
  
  constructor(config: MemoryManagerConfig) {
    this.workingMemory = new WorkingMemoryService(config.workingMemory);
    this.episodicMemory = new EpisodicMemoryService(config.episodicMemory);
    this.semanticMemory = new SemanticMemoryService(config.semanticMemory);
    this.vectorStore = new VectorStore(config.vectorStore);
    this.embeddingService = new EmbeddingService(config.embedding);
    this.llmService = config.llmService;
    
    this.episodeBoundaryDetector = new EpisodeBoundaryDetector({
      episodicMemory: this.episodicMemory,
      llmService: this.llmService
    });
    
    this.knowledgeValidator = new KnowledgeValidator({
      semanticMemory: this.semanticMemory,
      llmService: this.llmService
    });
    
    // Initialize with a new episode
    this.currentEpisodeId = '';
  }
  
  async initialize(): Promise<void> {
    await this.vectorStore.initialize();
    
    // Create initial episode if needed
    if (!this.currentEpisodeId) {
      this.currentEpisodeId = uuidv4();
      await this.episodicMemory.createEpisode({
        id: this.currentEpisodeId,
        title: 'Initial Session',
        type: 'session',
        startTime: new Date()
      });
    }
  }
  
  async storeMemory(content: string | object, type: MemoryType, metadata: any): Promise<string> {
    // Generate a unique ID for the memory
    const memoryId = uuidv4();
    
    // Create embedding for vector search
    const contentText = typeof content === 'string' 
      ? content 
      : JSON.stringify(content);
    
    const embedding = await this.embeddingService.createEmbedding(contentText);
    
    // Store in vector database for similarity search
    await this.vectorStore.storeEmbedding(memoryId, embedding, {
      type,
      timestamp: Date.now(),
      ...metadata
    });
    
    // Store in appropriate memory system
    switch (type) {
      case 'working':
        await this.workingMemory.store(memoryId, content, metadata);
        break;
        
      case 'episodic':
        // Check if this should start a new episode
        if (this.currentEpisodeId) {
          const boundaryCheck = await this.episodeBoundaryDetector.detectBoundary(
            this.currentEpisodeId,
            content
          );
          
          if (boundaryCheck.isNewEpisode && boundaryCheck.newEpisodeId) {
            this.currentEpisodeId = boundaryCheck.newEpisodeId;
          }
        }
        
        // Add episode info to metadata
        const episodeMetadata = {
          ...metadata,
          episode: {
            id: this.currentEpisodeId,
            ...(metadata.episode || {})
          }
        };
        
        await this.episodicMemory.store(memoryId, content, episodeMetadata);
        break;
        
      case 'semantic':
        await this.semanticMemory.store(memoryId, content, metadata);
        
        // Validate knowledge if appropriate
        if (metadata.validate !== false) {
          await this.knowledgeValidator.validateKnowledge(memoryId);
        }
        
        break;
    }
    
    return memoryId;
  }
  
  async retrieveMemory(id: string): Promise<MemoryItem | null> {
    // Try each memory store
    const workingMemory = await this.workingMemory.retrieve(id);
    if (workingMemory) return workingMemory;
    
    const episodicMemory = await this.episodicMemory.retrieve(id);
    if (episodicMemory) return episodicMemory;
    
    const semanticMemory = await this.semanticMemory.retrieve(id);
    if (semanticMemory) return semanticMemory;
    
    return null;
  }
  
  async retrieveRelevantMemories(query: string, options: RetrievalOptions = {}): Promise<MemoryItem[]> {
    // Create embedding for query
    const queryEmbedding = await this.embeddingService.createEmbedding(query);
    
    // Search for similar memories in vector store
    const filter: any = {};
    if (options.types && options.types.length > 0) {
      filter.type = { $in: options.types };
    }
    
    const similarMemories = await this.vectorStore.searchSimilar(
      queryEmbedding,
      options.limit || 10,
      filter
    );
    
    // Retrieve full memory objects
    const memories: MemoryItem[] = [];
    for (const result of similarMemories) {
      const memory = await this.retrieveMemory(result.id);
      
      if (memory) {
        memories.push({
          ...memory,
          relevance: result.score
        });
      }
    }
    
    return memories;
  }
  
  async updateMemory(id: string, updates: Partial<MemoryItem>): Promise<MemoryItem | null> {
    // Determine memory type and update appropriate store
    const memory = await this.retrieveMemory(id);
    
    if (!memory) return null;
    
    let updatedMemory: MemoryItem | null = null;
    
    switch (memory.type) {
      case 'working':
        updatedMemory = await this.workingMemory.update(id, updates as Partial<WorkingMemoryItem>);
        break;
        
      case 'episodic':
        updatedMemory = await this.episodicMemory.update(id, updates as Partial<EpisodicMemoryItem>);
        break;
        
      case 'semantic':
        updatedMemory = await this.semanticMemory.update(id, updates as Partial<SemanticMemoryItem>);
        break;
    }
    
    if (updatedMemory && (updates.content || updates.metadata)) {
      // Update vector embedding if content changed
      if (updates.content) {
        const contentText = typeof updatedMemory.content === 'string' 
          ? updatedMemory.content 
          : JSON.stringify(updatedMemory.content);
        
        const embedding = await this.embeddingService.createEmbedding(contentText);
        await this.vectorStore.storeEmbedding(id, embedding, {
          type: updatedMemory.type,
          timestamp: Date.now(),
          ...(updatedMemory.metadata || {})
        });
      } 
      // Update just metadata if only metadata changed
      else if (updates.metadata) {
        await this.vectorStore.updateMetadata(id, {
          ...(memory.metadata || {}),
          ...(updates.metadata || {})
        });
      }
    }
    
    return updatedMemory;
  }
  
  async deleteMemory(id: string): Promise<boolean> {
    // Determine memory type and delete from appropriate store
    const memory = await this.retrieveMemory(id);
    
    if (!memory) return false;
    
    let success = false;
    
    switch (memory.type) {
      case 'working':
        success = await this.workingMemory.delete(id);
        break;
        
      case 'episodic':
        success = await this.episodicMemory.delete(id);
        break;
        
      case 'semantic':
        success = await this.semanticMemory.delete(id);
        break;
    }
    
    if (success) {
      // Delete from vector store
      await this.vectorStore.deleteEmbedding(id);
    }
    
    return success;
  }
  
  async getCurrentContext(limit: number = 10): Promise<MemoryItem[]> {
    // Get current working memory items
    const workingMemories = await this.workingMemory.getSessionMemories('default', limit);
    
    // Get recent episodic memories from current episode
    const episodicMemories = await this.episodicMemory.getEpisodeMemories(this.currentEpisodeId);
    
    // Combine and sort by recency
    const allMemories = [...workingMemories, ...episodicMemories];
    
    allMemories.sort((a, b) => {
      const timeA = new Date(a.metadata.timestamp).getTime();
      const timeB = new Date(b.metadata.timestamp).getTime();
      return timeB - timeA; // Most recent first
    });
    
    return allMemories.slice(0, limit);
  }
  
  async summarizeCurrentContext(): Promise<string> {
    const context = await this.getCurrentContext(20);
    
    if (context.length === 0) {
      return "No current context available.";
    }
    
    // Format context for summarization
    const contextText = context.map(memory => {
      const contentText = typeof memory.content === 'string' 
        ? memory.content 
        : JSON.stringify(memory.content);
      
      return `[${memory.type.toUpperCase()}] ${contentText}`;
    }).join('\n\n');
    
    // Generate summary using LLM
    const prompt = `
      Summarize the following context information:
      
      ${contextText}
      
      Provide a concise summary that captures the key points and current state.
    `;
    
    return this.llmService.generateCompletion(prompt, 'context_summarization');
  }
}
```

### 7.2 Memory Optimization

```typescript
class MemoryOptimizer {
  private memoryManager: MemoryManager;
  private episodicMemory: EpisodicMemoryService;
  private semanticMemory: SemanticMemoryService;
  private llmService: LLMService;
  
  constructor(config: MemoryOptimizerConfig) {
    this.memoryManager = config.memoryManager;
    this.episodicMemory = config.episodicMemory;
    this.semanticMemory = config.semanticMemory;
    this.llmService = config.llmService;
  }
  
  async runOptimization(): Promise<void> {
    // Consolidate old episodes
    await this.consolidateOldEpisodes();
    
    // Validate and resolve knowledge conflicts
    await this.resolveKnowledgeConflicts();
    
    // Prune low-importance memories
    await this.pruneLowImportanceMemories();
  }
  
  private async consolidateOldEpisodes(daysThreshold: number = 7): Promise<void> {
    const thresholdDate = new Date();
    thresholdDate.setDate(thresholdDate.getDate() - daysThreshold);
    
    // Find old episodes that haven't been consolidated
    const oldEpisodes = await this.episodicMemory.episodeCollection.find({
      startTime: { $lt: thresholdDate },
      consolidated: { $ne: true }
    }).toArray();
    
    for (const episode of oldEpisodes) {
      // Summarize the episode
      const summary = await this.episodicMemory.summarizeEpisode(episode.id, this.llmService);
      
      // Extract key insights
      const memories = await this.episodicMemory.getEpisodeMemories(episode.id);
      const insightsPrompt = `
        Based on this episode summary:
        ${summary}
        
        And these detailed events:
        ${memories.slice(0, 10).map(m => JSON.stringify(m.content)).join('\n')}
        
        Extract 3-5 key insights or lessons that should be remembered long-term.
        Format each insight as a concise statement.
      `;
      
      const insightsResponse = await this.llmService.generateCompletion(insightsPrompt, 'insights');
      const insights = insightsResponse
        .split('\n')
        .filter(line => line.trim().length > 0)
        .map(line => line.replace(/^\d+\.\s*/, '').trim());
      
      // Store consolidated version
      await this.episodicMemory.episodeCollection.updateOne(
        { id: episode.id },
        { 
          $set: { 
            consolidated: true,
            summary,
            insights
          } 
        }
      );
      
      // Create semantic memories from insights
      for (const insight of insights) {
        await this.semanticMemory.createSemanticMemoryFromInsight(insight, episode.id, this.llmService);
      }
    }
  }
  
  private async resolveKnowledgeConflicts(): Promise<void> {
    // Find concepts with multiple definitions
    const concepts = await this.semanticMemory.collection.aggregate([
      { $group: { _id: '$concept.name', count: { $sum: 1 }, ids: { $push: '$id' } } },
      { $match: { count: { $gt: 1 } } }
    ]).toArray();
    
    for (const concept of concepts) {
      if (concept.ids.length > 1) {
        await this.semanticMemory.knowledgeValidator.resolveConflicts(concept.ids);
      }
    }
  }
  
  private async pruneLowImportanceMemories(): Promise<void> {
    // Prune old, low-importance episodic memories
    const oldThreshold = new Date();
    oldThreshold.setMonth(oldThreshold.getMonth() - 3);
    
    const lowImportanceEpisodic = await this.episodicMemory.collection.find({
      'metadata.timestamp': { $lt: oldThreshold },
      'metadata.importance': { $lt: 0.3 },
      'metadata.accessCount': { $lt: 3 }
    }).toArray();
    
    for (const memory of lowImportanceEpisodic) {
      await this.memoryManager.deleteMemory(memory.id);
    }
    
    // Prune redundant semantic memories
    const lowConfidenceSemantic = await this.semanticMemory.collection.find({
      confidence: { $lt: 0.4 },
      'metadata.resolvedBy': { $exists: true }
    }).toArray();
    
    for (const memory of lowConfidenceSemantic) {
      await this.memoryManager.deleteMemory(memory.id);
    }
  }
}
```
