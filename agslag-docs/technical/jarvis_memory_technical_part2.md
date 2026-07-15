# Jarvis SWE Agent Memory Architecture - Technical Implementation (Part 2)

## 4. Episodic Memory Implementation

### 4.1 MongoDB-Based Episodic Memory

Episodic memory is implemented using MongoDB for structured storage of episodes and events:

```typescript
class EpisodicMemoryService {
  private db: MongoDB.Db;
  private collection: MongoDB.Collection;
  private episodeCollection: MongoDB.Collection;
  
  constructor(config: EpisodicMemoryConfig) {
    const client = new MongoDB.MongoClient(config.mongoUri);
    this.db = client.db(config.database || 'jarvis');
    this.collection = this.db.collection(config.collection || 'episodic_memories');
    this.episodeCollection = this.db.collection(config.episodeCollection || 'episodes');
    
    // Create indexes
    this.collection.createIndex({ id: 1 }, { unique: true });
    this.collection.createIndex({ 'episode.id': 1 });
    this.collection.createIndex({ 'metadata.timestamp': 1 });
    this.collection.createIndex({ 'metadata.tags': 1 });
    this.episodeCollection.createIndex({ id: 1 }, { unique: true });
  }
  
  async createEpisode(episode: Partial<Episode>): Promise<string> {
    const id = episode.id || uuidv4();
    
    const newEpisode: Episode = {
      id,
      title: episode.title || 'Untitled Episode',
      startTime: episode.startTime || new Date(),
      type: episode.type || 'general',
      status: episode.status || 'active',
      metadata: episode.metadata || {
        source: 'system',
        importance: 0.5,
        tags: []
      }
    };
    
    await this.episodeCollection.insertOne(newEpisode);
    
    return id;
  }
  
  async closeEpisode(id: string, outcome?: string): Promise<boolean> {
    const result = await this.episodeCollection.updateOne(
      { id },
      { 
        $set: { 
          endTime: new Date(),
          status: 'completed',
          outcome
        } 
      }
    );
    
    return result.modifiedCount > 0;
  }
  
  async store(id: string, content: any, metadata: any): Promise<string> {
    const episodeId = metadata.episode?.id;
    
    // Ensure episode exists
    if (episodeId) {
      const episode = await this.episodeCollection.findOne({ id: episodeId });
      if (!episode) {
        await this.createEpisode({ id: episodeId });
      }
    }
    
    const memoryItem: EpisodicMemoryItem = {
      id,
      type: 'episodic',
      content,
      metadata: {
        ...metadata,
        timestamp: new Date(),
        accessCount: 0,
        importance: metadata.importance || 0.5,
        tags: metadata.tags || []
      },
      episode: {
        id: episodeId || uuidv4(),
        title: metadata.episode?.title || 'Untitled Episode',
        startTime: metadata.episode?.startTime || new Date(),
        type: metadata.episode?.type || 'general'
      },
      sequence: metadata.sequence || 0,
      actors: metadata.actors || [],
      actions: metadata.actions || []
    };
    
    // Store in MongoDB
    await this.collection.insertOne(memoryItem);
    
    return id;
  }
  
  async retrieve(id: string): Promise<EpisodicMemoryItem | null> {
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
    
    return memoryItem as EpisodicMemoryItem;
  }
  
  async getEpisodeMemories(episodeId: string): Promise<EpisodicMemoryItem[]> {
    const memories = await this.collection
      .find({ 'episode.id': episodeId })
      .sort({ sequence: 1 })
      .toArray();
    
    return memories as EpisodicMemoryItem[];
  }
  
  async getRecentEpisodes(limit: number = 10): Promise<Episode[]> {
    const episodes = await this.episodeCollection
      .find()
      .sort({ startTime: -1 })
      .limit(limit)
      .toArray();
    
    return episodes as Episode[];
  }
  
  async searchEpisodes(query: any, limit: number = 10): Promise<Episode[]> {
    const episodes = await this.episodeCollection
      .find(query)
      .sort({ startTime: -1 })
      .limit(limit)
      .toArray();
    
    return episodes as Episode[];
  }
  
  async update(id: string, updates: Partial<EpisodicMemoryItem>): Promise<EpisodicMemoryItem | null> {
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
    
    if (updates.episode) {
      Object.keys(updates.episode).forEach(key => {
        updateDoc.$set[`episode.${key}`] = updates.episode[key];
      });
    }
    
    if (updates.sequence !== undefined) {
      updateDoc.$set.sequence = updates.sequence;
    }
    
    if (updates.actors !== undefined) {
      updateDoc.$set.actors = updates.actors;
    }
    
    if (updates.actions !== undefined) {
      updateDoc.$set.actions = updates.actions;
    }
    
    const result = await this.collection.updateOne({ id }, updateDoc);
    
    if (result.modifiedCount === 0) return null;
    
    return this.retrieve(id);
  }
  
  async delete(id: string): Promise<boolean> {
    const result = await this.collection.deleteOne({ id });
    
    return result.deletedCount > 0;
  }
  
  async summarizeEpisode(episodeId: string, llmService: LLMService): Promise<string> {
    const memories = await this.getEpisodeMemories(episodeId);
    const episode = await this.episodeCollection.findOne({ id: episodeId });
    
    if (!memories.length || !episode) return '';
    
    // Format memories for summarization
    const memoryTexts = memories.map(memory => {
      const contentText = typeof memory.content === 'string' 
        ? memory.content 
        : JSON.stringify(memory.content);
      
      return `[${new Date(memory.metadata.timestamp).toISOString()}] ${contentText}`;
    }).join('\n\n');
    
    // Generate summary using LLM
    const prompt = `
      Summarize the following episode of events:
      
      Episode: ${episode.title}
      Type: ${episode.type}
      Start Time: ${new Date(episode.startTime).toISOString()}
      ${episode.endTime ? `End Time: ${new Date(episode.endTime).toISOString()}` : ''}
      ${episode.outcome ? `Outcome: ${episode.outcome}` : ''}
      
      Events:
      ${memoryTexts}
      
      Please provide a concise summary that captures the key points, actions, and outcomes of this episode.
    `;
    
    const summary = await llmService.generateCompletion(prompt, 'summarization');
    
    // Store the summary in the episode
    await this.episodeCollection.updateOne(
      { id: episodeId },
      { $set: { summary } }
    );
    
    return summary;
  }
}
```

### 4.2 Episode Boundary Detection

```typescript
class EpisodeBoundaryDetector {
  private episodicMemory: EpisodicMemoryService;
  private llmService: LLMService;
  
  constructor(config: EpisodeBoundaryDetectorConfig) {
    this.episodicMemory = config.episodicMemory;
    this.llmService = config.llmService;
  }
  
  async detectBoundary(currentEpisodeId: string, event: any): Promise<{ isNewEpisode: boolean, newEpisodeId?: string }> {
    // Get current episode and recent memories
    const currentEpisode = await this.episodicMemory.episodeCollection.findOne({ id: currentEpisodeId });
    const recentMemories = await this.episodicMemory.getEpisodeMemories(currentEpisodeId);
    
    if (!currentEpisode) {
      // If no current episode, create a new one
      const newEpisodeId = uuidv4();
      await this.episodicMemory.createEpisode({
        id: newEpisodeId,
        title: 'New Episode',
        type: 'general',
        startTime: new Date()
      });
      return { isNewEpisode: true, newEpisodeId };
    }
    
    // Format recent memories and current event for LLM
    const recentMemoryTexts = recentMemories.slice(-5).map(memory => {
      const contentText = typeof memory.content === 'string' 
        ? memory.content 
        : JSON.stringify(memory.content);
      
      return `[${new Date(memory.metadata.timestamp).toISOString()}] ${contentText}`;
    }).join('\n');
    
    const currentEventText = typeof event === 'string' 
      ? event 
      : JSON.stringify(event);
    
    // Ask LLM if this event represents a new episode
    const prompt = `
      Current Episode: ${currentEpisode.title}
      Type: ${currentEpisode.type}
      Start Time: ${new Date(currentEpisode.startTime).toISOString()}
      
      Recent events in this episode:
      ${recentMemoryTexts}
      
      New event:
      ${currentEventText}
      
      Based on the current episode and the new event, determine if this event should:
      1. Continue the current episode
      2. Start a new episode
      
      Consider factors like:
      - Topic change
      - Significant time gap
      - Change in context or task
      - Natural conclusion of previous activity
      
      Respond with either "CONTINUE" or "NEW_EPISODE" followed by a brief explanation.
    `;
    
    const response = await this.llmService.generateCompletion(prompt, 'episode_boundary');
    const isNewEpisode = response.includes('NEW_EPISODE');
    
    if (isNewEpisode) {
      // Close current episode
      await this.episodicMemory.closeEpisode(currentEpisodeId);
      
      // Create new episode
      const newEpisodeId = uuidv4();
      
      // Extract title from LLM response
      const titlePrompt = `
        Based on this event:
        ${currentEventText}
        
        Generate a short, descriptive title for a new episode (maximum 5-7 words).
      `;
      
      const titleResponse = await this.llmService.generateCompletion(titlePrompt, 'episode_title');
      const title = titleResponse.replace(/["']/g, '').trim();
      
      await this.episodicMemory.createEpisode({
        id: newEpisodeId,
        title,
        type: 'general',
        startTime: new Date()
      });
      
      return { isNewEpisode: true, newEpisodeId };
    }
    
    return { isNewEpisode: false };
  }
}
```

### 4.3 Episodic Memory Consolidation

```typescript
class EpisodicMemoryConsolidator {
  private episodicMemory: EpisodicMemoryService;
  private llmService: LLMService;
  
  constructor(config: EpisodicMemoryConsolidatorConfig) {
    this.episodicMemory = config.episodicMemory;
    this.llmService = config.llmService;
  }
  
  async consolidateOldEpisodes(daysThreshold: number = 7): Promise<void> {
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
        await this.createSemanticMemoryFromInsight(insight, episode.id);
      }
    }
  }
  
  private async createSemanticMemoryFromInsight(insight: string, episodeId: string): Promise<void> {
    // Implementation will be covered in Semantic Memory section
  }
}
```
