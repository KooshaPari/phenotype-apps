# Jarvis SWE Agent Model Orchestration - Technical Implementation (Part 2)

## 4. Provider Manager Implementation

### 4.1 Provider Adapter Interface

```typescript
interface ProviderAdapter {
  id: string;
  name: string;
  
  initialize(config: any): Promise<void>;
  
  generateCompletion(prompt: string, model: string, options?: any): Promise<CompletionResult>;
  
  generateChatCompletion(messages: any[], model: string, options?: any): Promise<CompletionResult>;
  
  generateEmbedding(text: string | string[], model: string): Promise<EmbeddingResult>;
  
  getModels(): Promise<ModelInfo[]>;
  
  validateApiKey(apiKey: string): Promise<boolean>;
}

interface CompletionResult {
  text: string;
  usage: {
    prompt_tokens: number;
    completion_tokens: number;
    total_tokens: number;
  };
  model: string;
  finish_reason: string;
}

interface EmbeddingResult {
  embeddings: number[][];
  usage: {
    prompt_tokens: number;
    total_tokens: number;
  };
  model: string;
}

interface ModelInfo {
  id: string;
  name: string;
  capabilities: {
    completion: boolean;
    chat: boolean;
    embeddings: boolean;
    function_calling: boolean;
    vision: boolean;
    audio: boolean;
  };
  context_length: number;
}
```

### 4.2 OpenAI Provider Adapter

```typescript
class OpenAIProviderAdapter implements ProviderAdapter {
  id: string = 'openai';
  name: string = 'OpenAI';
  private client: OpenAIApi;
  
  async initialize(config: any): Promise<void> {
    const configuration = new Configuration({
      apiKey: config.apiKey
    });
    this.client = new OpenAIApi(configuration);
  }
  
  async generateCompletion(prompt: string, model: string, options?: any): Promise<CompletionResult> {
    try {
      const response = await this.client.createCompletion({
        model,
        prompt,
        max_tokens: options?.max_tokens || 1000,
        temperature: options?.temperature || 0.7,
        top_p: options?.top_p || 1,
        frequency_penalty: options?.frequency_penalty || 0,
        presence_penalty: options?.presence_penalty || 0,
        stop: options?.stop || null
      });
      
      return {
        text: response.data.choices[0].text || '',
        usage: {
          prompt_tokens: response.data.usage?.prompt_tokens || 0,
          completion_tokens: response.data.usage?.completion_tokens || 0,
          total_tokens: response.data.usage?.total_tokens || 0
        },
        model: response.data.model,
        finish_reason: response.data.choices[0].finish_reason || 'unknown'
      };
    } catch (error) {
      throw this.formatError(error);
    }
  }
  
  async generateChatCompletion(messages: any[], model: string, options?: any): Promise<CompletionResult> {
    try {
      const response = await this.client.createChatCompletion({
        model,
        messages,
        max_tokens: options?.max_tokens || 1000,
        temperature: options?.temperature || 0.7,
        top_p: options?.top_p || 1,
        frequency_penalty: options?.frequency_penalty || 0,
        presence_penalty: options?.presence_penalty || 0,
        stop: options?.stop || null,
        functions: options?.functions || undefined,
        function_call: options?.function_call || undefined
      });
      
      return {
        text: response.data.choices[0].message?.content || '',
        usage: {
          prompt_tokens: response.data.usage?.prompt_tokens || 0,
          completion_tokens: response.data.usage?.completion_tokens || 0,
          total_tokens: response.data.usage?.total_tokens || 0
        },
        model: response.data.model,
        finish_reason: response.data.choices[0].finish_reason || 'unknown'
      };
    } catch (error) {
      throw this.formatError(error);
    }
  }
  
  async generateEmbedding(text: string | string[], model: string): Promise<EmbeddingResult> {
    try {
      const input = Array.isArray(text) ? text : [text];
      
      const response = await this.client.createEmbedding({
        model,
        input
      });
      
      const embeddings = response.data.data.map(item => item.embedding);
      
      return {
        embeddings,
        usage: {
          prompt_tokens: response.data.usage?.prompt_tokens || 0,
          total_tokens: response.data.usage?.total_tokens || 0
        },
        model: response.data.model
      };
    } catch (error) {
      throw this.formatError(error);
    }
  }
  
  async getModels(): Promise<ModelInfo[]> {
    try {
      const response = await this.client.listModels();
      
      return response.data.data.map(model => ({
        id: model.id,
        name: model.id,
        capabilities: {
          completion: model.id.startsWith('text-') || model.id.includes('gpt-3.5') || model.id.includes('gpt-4'),
          chat: model.id.includes('gpt-3.5') || model.id.includes('gpt-4'),
          embeddings: model.id.includes('embedding'),
          function_calling: model.id.includes('gpt-3.5') || model.id.includes('gpt-4'),
          vision: model.id.includes('vision'),
          audio: false
        },
        context_length: this.getContextLength(model.id)
      }));
    } catch (error) {
      throw this.formatError(error);
    }
  }
  
  async validateApiKey(apiKey: string): Promise<boolean> {
    try {
      const configuration = new Configuration({
        apiKey
      });
      const tempClient = new OpenAIApi(configuration);
      
      await tempClient.listModels();
      return true;
    } catch (error) {
      return false;
    }
  }
  
  private getContextLength(modelId: string): number {
    if (modelId.includes('gpt-4-turbo')) return 128000;
    if (modelId.includes('gpt-4-32k')) return 32768;
    if (modelId.includes('gpt-4')) return 8192;
    if (modelId.includes('gpt-3.5-turbo-16k')) return 16385;
    if (modelId.includes('gpt-3.5-turbo')) return 4096;
    return 4096; // Default
  }
  
  private formatError(error: any): Error {
    if (error.response) {
      const status = error.response.status;
      const data = error.response.data;
      
      if (status === 401) {
        return new Error('Authentication error: Invalid API key');
      } else if (status === 429) {
        return new Error('Rate limit exceeded: Too many requests');
      } else if (status === 400) {
        return new Error(`Bad request: ${data.error?.message || 'Unknown error'}`);
      } else {
        return new Error(`OpenAI API error (${status}): ${data.error?.message || 'Unknown error'}`);
      }
    }
    
    return error;
  }
}
```

### 4.3 Provider Manager Service

```typescript
class ProviderManager {
  private adapters: Map<string, ProviderAdapter> = new Map();
  private modelRegistry: ModelRegistry;
  private apiKeyManager: APIKeyManager;
  
  constructor(config: ProviderManagerConfig) {
    this.modelRegistry = config.modelRegistry;
    this.apiKeyManager = config.apiKeyManager;
    
    // Register default adapters
    this.registerDefaultAdapters();
  }
  
  private registerDefaultAdapters(): void {
    this.registerAdapter(new OpenAIProviderAdapter());
    // Register other adapters
  }
  
  registerAdapter(adapter: ProviderAdapter): void {
    this.adapters.set(adapter.id, adapter);
  }
  
  async getAdapter(providerId: string): Promise<ProviderAdapter | null> {
    const adapter = this.adapters.get(providerId);
    
    if (!adapter) return null;
    
    // Initialize adapter if needed
    if (!adapter.initialized) {
      const apiKey = await this.apiKeyManager.getApiKey(providerId);
      
      if (!apiKey) {
        throw new Error(`No API key found for provider ${providerId}`);
      }
      
      await adapter.initialize({ apiKey });
      adapter.initialized = true;
    }
    
    return adapter;
  }
  
  async listAdapters(): Promise<{ id: string, name: string }[]> {
    return Array.from(this.adapters.values()).map(adapter => ({
      id: adapter.id,
      name: adapter.name
    }));
  }
  
  async syncModels(providerId: string): Promise<number> {
    const adapter = await this.getAdapter(providerId);
    
    if (!adapter) {
      throw new Error(`Provider ${providerId} not found`);
    }
    
    // Get models from provider
    const models = await adapter.getModels();
    
    // Register models in registry
    let count = 0;
    
    for (const model of models) {
      await this.modelRegistry.registerModel({
        id: `${providerId}/${model.id}`,
        provider: providerId,
        name: model.id,
        capabilities: model.capabilities,
        parameters: {
          context_length: model.context_length,
          token_limit: 4096, // Default
          temperature_range: {
            min: 0,
            max: 1,
            default: 0.7
          },
          supported_languages: ['en'] // Default
        },
        status: 'active'
      });
      
      count++;
    }
    
    return count;
  }
  
  async validateApiKey(providerId: string, apiKey: string): Promise<boolean> {
    const adapter = this.adapters.get(providerId);
    
    if (!adapter) {
      throw new Error(`Provider ${providerId} not found`);
    }
    
    return adapter.validateApiKey(apiKey);
  }
  
  async generateCompletion(
    providerId: string,
    modelName: string,
    prompt: string,
    options?: any
  ): Promise<CompletionResult> {
    const adapter = await this.getAdapter(providerId);
    
    if (!adapter) {
      throw new Error(`Provider ${providerId} not found`);
    }
    
    return adapter.generateCompletion(prompt, modelName, options);
  }
  
  async generateChatCompletion(
    providerId: string,
    modelName: string,
    messages: any[],
    options?: any
  ): Promise<CompletionResult> {
    const adapter = await this.getAdapter(providerId);
    
    if (!adapter) {
      throw new Error(`Provider ${providerId} not found`);
    }
    
    return adapter.generateChatCompletion(messages, modelName, options);
  }
  
  async generateEmbedding(
    providerId: string,
    modelName: string,
    text: string | string[]
  ): Promise<EmbeddingResult> {
    const adapter = await this.getAdapter(providerId);
    
    if (!adapter) {
      throw new Error(`Provider ${providerId} not found`);
    }
    
    return adapter.generateEmbedding(text, modelName);
  }
}
```

## 5. API Key Manager Implementation

### 5.1 API Key Manager Service

```typescript
class APIKeyManager {
  private db: MongoDB.Db;
  private collection: MongoDB.Collection;
  private encryptionKey: string;
  
  constructor(config: APIKeyManagerConfig) {
    const client = new MongoDB.MongoClient(config.mongoUri);
    this.db = client.db(config.database || 'jarvis');
    this.collection = this.db.collection(config.collection || 'api_keys');
    this.encryptionKey = config.encryptionKey;
    
    // Create indexes
    this.collection.createIndex({ provider: 1, label: 1 }, { unique: true });
  }
  
  async addApiKey(provider: string, apiKey: string, label: string = 'default'): Promise<string> {
    // Encrypt API key
    const encryptedKey = this.encrypt(apiKey);
    
    // Check if key already exists
    const existingKey = await this.collection.findOne({ provider, label });
    
    if (existingKey) {
      // Update existing key
      await this.collection.updateOne(
        { provider, label },
        { 
          $set: { 
            key: encryptedKey,
            updated: new Date()
          } 
        }
      );
      
      return existingKey._id.toString();
    } else {
      // Add new key
      const result = await this.collection.insertOne({
        provider,
        label,
        key: encryptedKey,
        created: new Date(),
        updated: new Date(),
        last_used: null,
        usage_count: 0,
        status: 'active'
      });
      
      return result.insertedId.toString();
    }
  }
  
  async getApiKey(provider: string, label: string = 'default'): Promise<string | null> {
    const keyRecord = await this.collection.findOne({ provider, label, status: 'active' });
    
    if (!keyRecord) return null;
    
    // Update usage statistics
    await this.collection.updateOne(
      { _id: keyRecord._id },
      { 
        $set: { last_used: new Date() },
        $inc: { usage_count: 1 }
      }
    );
    
    // Decrypt and return key
    return this.decrypt(keyRecord.key);
  }
  
  async listApiKeys(provider?: string): Promise<any[]> {
    const query = provider ? { provider } : {};
    
    const keys = await this.collection.find(query).project({
      key: 0 // Don't return the encrypted key
    }).toArray();
    
    return keys;
  }
  
  async deleteApiKey(provider: string, label: string = 'default'): Promise<boolean> {
    const result = await this.collection.deleteOne({ provider, label });
    
    return result.deletedCount > 0;
  }
  
  async rotateApiKey(provider: string, newApiKey: string, label: string = 'default'): Promise<boolean> {
    // Encrypt new API key
    const encryptedKey = this.encrypt(newApiKey);
    
    const result = await this.collection.updateOne(
      { provider, label },
      { 
        $set: { 
          key: encryptedKey,
          updated: new Date(),
          rotation_count: { $inc: 1 }
        } 
      }
    );
    
    return result.modifiedCount > 0;
  }
  
  private encrypt(text: string): string {
    const iv = crypto.randomBytes(16);
    const cipher = crypto.createCipheriv('aes-256-cbc', Buffer.from(this.encryptionKey, 'hex'), iv);
    
    let encrypted = cipher.update(text, 'utf8', 'hex');
    encrypted += cipher.final('hex');
    
    return `${iv.toString('hex')}:${encrypted}`;
  }
  
  private decrypt(text: string): string {
    const [ivHex, encryptedText] = text.split(':');
    const iv = Buffer.from(ivHex, 'hex');
    const decipher = crypto.createDecipheriv('aes-256-cbc', Buffer.from(this.encryptionKey, 'hex'), iv);
    
    let decrypted = decipher.update(encryptedText, 'hex', 'utf8');
    decrypted += decipher.final('utf8');
    
    return decrypted;
  }
}
```

## 6. Selection Engine Implementation

### 6.1 Selection Engine Service

```typescript
class SelectionEngine {
  private modelRegistry: ModelRegistry;
  private performanceMonitor: PerformanceMonitor;
  private costOptimizer: CostOptimizer;
  
  constructor(config: SelectionEngineConfig) {
    this.modelRegistry = config.modelRegistry;
    this.performanceMonitor = config.performanceMonitor;
    this.costOptimizer = config.costOptimizer;
  }
  
  async selectModel(request: ModelSelectionRequest): Promise<ModelSelectionResult> {
    // Get strategy implementation
    const strategy = this.getStrategy(request.strategy || 'balanced');
    
    // Apply strategy
    return strategy.selectModel(request);
  }
  
  private getStrategy(strategyName: string): SelectionStrategy {
    switch (strategyName) {
      case 'performance':
        return new PerformanceStrategy(this.modelRegistry, this.performanceMonitor);
      case 'cost':
        return new CostStrategy(this.modelRegistry, this.costOptimizer);
      case 'reliability':
        return new ReliabilityStrategy(this.modelRegistry, this.performanceMonitor);
      case 'balanced':
      default:
        return new BalancedStrategy(this.modelRegistry, this.performanceMonitor, this.costOptimizer);
    }
  }
}

interface ModelSelectionRequest {
  type: 'completion' | 'chat' | 'embedding';
  input_length?: number;
  expected_output_length?: number;
  required_capabilities?: string[];
  preferred_model?: string;
  preferred_provider?: string;
  strategy?: 'performance' | 'cost' | 'reliability' | 'balanced';
  budget_constraint?: number;
  timeout_constraint?: number;
}

interface ModelSelectionResult {
  model_id: string;
  provider: string;
  model_name: string;
  fallbacks: string[];
  estimated_cost: number;
  estimated_latency: number;
  confidence: number;
  reasoning: string;
}
```

### 6.2 Selection Strategies

```typescript
interface SelectionStrategy {
  selectModel(request: ModelSelectionRequest): Promise<ModelSelectionResult>;
}

class BalancedStrategy implements SelectionStrategy {
  private modelRegistry: ModelRegistry;
  private performanceMonitor: PerformanceMonitor;
  private costOptimizer: CostOptimizer;
  
  constructor(
    modelRegistry: ModelRegistry,
    performanceMonitor: PerformanceMonitor,
    costOptimizer: CostOptimizer
  ) {
    this.modelRegistry = modelRegistry;
    this.performanceMonitor = performanceMonitor;
    this.costOptimizer = costOptimizer;
  }
  
  async selectModel(request: ModelSelectionRequest): Promise<ModelSelectionResult> {
    // If preferred model is specified and meets requirements, use it
    if (request.preferred_model) {
      const model = await this.modelRegistry.getModel(request.preferred_model);
      
      if (model && this.modelMeetsRequirements(model, request)) {
        return this.createResult(model, request, []);
      }
    }
    
    // Get all active models
    let models = await this.modelRegistry.listModels({ status: 'active' });
    
    // Filter by provider if specified
    if (request.preferred_provider) {
      models = models.filter(m => m.provider === request.preferred_provider);
    }
    
    // Filter by required capabilities
    if (request.required_capabilities && request.required_capabilities.length > 0) {
      models = models.filter(m => 
        request.required_capabilities!.every(cap => 
          m.capabilities[cap as keyof typeof m.capabilities]
        )
      );
    }
    
    // Filter by request type
    switch (request.type) {
      case 'completion':
        models = models.filter(m => m.capabilities.completion);
        break;
      case 'chat':
        models = models.filter(m => m.capabilities.chat);
        break;
      case 'embedding':
        models = models.filter(m => m.capabilities.embeddings);
        break;
    }
    
    // Filter by context length if input length is provided
    if (request.input_length) {
      models = models.filter(m => m.parameters.context_length >= request.input_length);
    }
    
    if (models.length === 0) {
      throw new Error('No suitable models found for the request');
    }
    
    // Score models based on balanced criteria
    const scoredModels = await Promise.all(models.map(async model => {
      const performanceScore = await this.getPerformanceScore(model);
      const costScore = await this.getCostScore(model, request);
      const reliabilityScore = await this.getReliabilityScore(model);
      
      // Balanced scoring with equal weights
      const score = (performanceScore + costScore + reliabilityScore) / 3;
      
      return { model, score };
    }));
    
    // Sort by score (descending)
    scoredModels.sort((a, b) => b.score - a.score);
    
    // Select top model
    const selectedModel = scoredModels[0].model;
    
    // Select fallbacks (next 2 models from different providers)
    const fallbacks = this.selectFallbacks(scoredModels, selectedModel);
    
    return this.createResult(selectedModel, request, fallbacks);
  }
  
  private modelMeetsRequirements(model: Model, request: ModelSelectionRequest): boolean {
    // Check if model supports the request type
    if (request.type === 'completion' && !model.capabilities.completion) return false;
    if (request.type === 'chat' && !model.capabilities.chat) return false;
    if (request.type === 'embedding' && !model.capabilities.embeddings) return false;
    
    // Check required capabilities
    if (request.required_capabilities && request.required_capabilities.length > 0) {
      if (!request.required_capabilities.every(cap => 
        model.capabilities[cap as keyof typeof model.capabilities]
      )) {
        return false;
      }
    }
    
    // Check context length
    if (request.input_length && model.parameters.context_length < request.input_length) {
      return false;
    }
    
    return true;
  }
  
  private async getPerformanceScore(model: Model): Promise<number> {
    // Get performance metrics
    const metrics = await this.performanceMonitor.getModelMetrics(model.id);
    
    // Calculate performance score (0-1)
    // Lower latency is better
    const latencyScore = metrics ? 
      Math.max(0, 1 - (metrics.latency.avg / 5000)) : // Normalize to 0-1 (5000ms as max)
      0.5; // Default if no metrics
    
    return latencyScore;
  }
  
  private async getCostScore(model: Model, request: ModelSelectionRequest): Promise<number> {
    // Calculate estimated cost
    const estimatedCost = await this.costOptimizer.estimateCost(
      model.id,
      request.input_length || 1000,
      request.expected_output_length || 500
    );
    
    // Calculate cost score (0-1)
    // Lower cost is better
    const maxCost = 0.1; // $0.10 as max cost
    const costScore = Math.max(0, 1 - (estimatedCost / maxCost));
    
    return costScore;
  }
  
  private async getReliabilityScore(model: Model): Promise<number> {
    // Get reliability metrics
    const metrics = await this.performanceMonitor.getModelMetrics(model.id);
    
    // Calculate reliability score (0-1)
    const reliabilityScore = metrics ? 
      metrics.reliability.success_rate : 
      0.95; // Default if no metrics
    
    return reliabilityScore;
  }
  
  private selectFallbacks(scoredModels: { model: Model, score: number }[], selectedModel: Model): string[] {
    const fallbacks: string[] = [];
    const seenProviders = new Set([selectedModel.provider]);
    
    // Select up to 2 fallbacks from different providers
    for (let i = 1; i < scoredModels.length && fallbacks.length < 2; i++) {
      const model = scoredModels[i].model;
      
      if (!seenProviders.has(model.provider)) {
        fallbacks.push(model.id);
        seenProviders.add(model.provider);
      }
    }
    
    // If we don't have 2 fallbacks yet, add models from same providers
    for (let i = 1; i < scoredModels.length && fallbacks.length < 2; i++) {
      const model = scoredModels[i].model;
      
      if (!fallbacks.includes(model.id)) {
        fallbacks.push(model.id);
      }
    }
    
    return fallbacks;
  }
  
  private createResult(model: Model, request: ModelSelectionRequest, fallbacks: string[]): ModelSelectionResult {
    return {
      model_id: model.id,
      provider: model.provider,
      model_name: model.name,
      fallbacks,
      estimated_cost: this.costOptimizer.estimateCost(
        model.id,
        request.input_length || 1000,
        request.expected_output_length || 500
      ),
      estimated_latency: model.performance.latency.avg,
      confidence: 0.9, // High confidence for balanced strategy
      reasoning: `Selected ${model.name} from ${model.provider} based on balanced consideration of performance, cost, and reliability.`
    };
  }
}
```
