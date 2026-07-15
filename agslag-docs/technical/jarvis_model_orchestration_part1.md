# Jarvis SWE Agent Model Orchestration - Technical Implementation (Part 1)

## 1. Model Orchestration Overview

The Jarvis model orchestration system provides sophisticated capabilities for managing multiple LLM providers and models, with dynamic selection, fallback mechanisms, and cost optimization. This document details the technical implementation of the core components of this system.

## 2. Core Data Models

### 2.1 Model

```typescript
interface Model {
  id: string;                 // Unique model identifier
  provider: string;           // Provider identifier
  name: string;               // Model name
  version: string;            // Model version
  capabilities: {             // Model capabilities
    completion: boolean;      // Text completion
    chat: boolean;            // Chat completion
    embeddings: boolean;      // Vector embeddings
    function_calling: boolean; // Function calling
    vision: boolean;          // Image understanding
    audio: boolean;           // Audio processing
  };
  parameters: {               // Model parameters
    context_length: number;   // Maximum context length
    token_limit: number;      // Maximum tokens per request
    temperature_range: {      // Supported temperature range
      min: number;
      max: number;
      default: number;
    };
    supported_languages: string[]; // Supported languages
  };
  performance: {              // Performance metrics
    latency: {                // Latency metrics
      avg: number;            // Average latency (ms)
      p50: number;            // 50th percentile latency
      p95: number;            // 95th percentile latency
      p99: number;            // 99th percentile latency
    };
    reliability: {            // Reliability metrics
      success_rate: number;   // Success rate (0-1)
      error_rate: number;     // Error rate (0-1)
      availability: number;   // Availability (0-1)
    };
    quality: {                // Quality metrics
      benchmark_scores: {     // Benchmark scores
        [benchmark: string]: number;
      };
    };
  };
  cost: {                     // Cost information
    input_per_1k_tokens: number; // Cost per 1K input tokens
    output_per_1k_tokens: number; // Cost per 1K output tokens
    currency: string;         // Currency (USD, EUR, etc.)
  };
  status: ModelStatus;        // 'active', 'deprecated', 'unavailable'
  metadata: {                 // Additional metadata
    created: Date;            // Creation timestamp
    updated: Date;            // Last update timestamp
    description: string;      // Model description
    tags: string[];           // Categorization tags
  };
}
```

### 2.2 Provider

```typescript
interface Provider {
  id: string;                 // Unique provider identifier
  name: string;               // Provider name
  status: ProviderStatus;     // 'active', 'degraded', 'unavailable'
  models: string[];           // Available model IDs
  capabilities: {             // Provider capabilities
    streaming: boolean;       // Supports streaming
    batch_processing: boolean; // Supports batch processing
    fine_tuning: boolean;     // Supports fine-tuning
  };
  rate_limits: {              // Rate limiting information
    requests_per_minute: number; // Requests per minute
    tokens_per_minute: number; // Tokens per minute
  };
  authentication: {           // Authentication methods
    api_key: boolean;         // Supports API key auth
    oauth: boolean;           // Supports OAuth
  };
  reliability: {              // Reliability metrics
    uptime: number;           // Uptime percentage
    incident_count: number;   // Recent incident count
  };
  metadata: {                 // Additional metadata
    website: string;          // Provider website
    documentation: string;    // Documentation URL
    support: string;          // Support contact
  };
}
```

### 2.3 Request

```typescript
interface Request {
  id: string;                 // Unique request identifier
  type: RequestType;          // 'completion', 'chat', 'embedding'
  input: any;                 // Request input
  parameters: any;            // Request parameters
  routing: {                  // Routing information
    provider: string;         // Selected provider
    model: string;            // Selected model
    strategy: string;         // Selection strategy
    fallbacks: string[];      // Fallback models
  };
  performance: {              // Performance metrics
    start_time: Date;         // Start timestamp
    end_time?: Date;          // End timestamp
    latency?: number;         // Latency in ms
    input_tokens: number;     // Input token count
    output_tokens?: number;   // Output token count
    retries: number;          // Retry count
  };
  cost: {                     // Cost information
    amount: number;           // Total cost
    currency: string;         // Currency
    breakdown: {              // Cost breakdown
      input: number;          // Input cost
      output: number;         // Output cost
    };
  };
  status: RequestStatus;      // 'pending', 'processing', 'completed', 'failed'
  result?: any;               // Request result
  error?: {                   // Error information
    code: string;             // Error code
    message: string;          // Error message
    provider_error?: any;     // Provider-specific error
  };
}
```

## 3. Model Registry Implementation

### 3.1 Model Registry Service

```typescript
class ModelRegistry {
  private db: MongoDB.Db;
  private modelCollection: MongoDB.Collection;
  private providerCollection: MongoDB.Collection;
  private redis: Redis.Redis;
  
  constructor(config: ModelRegistryConfig) {
    const client = new MongoDB.MongoClient(config.mongoUri);
    this.db = client.db(config.database || 'jarvis');
    this.modelCollection = this.db.collection(config.modelCollection || 'models');
    this.providerCollection = this.db.collection(config.providerCollection || 'providers');
    this.redis = new Redis(config.redisUri);
    
    // Create indexes
    this.modelCollection.createIndex({ id: 1 }, { unique: true });
    this.modelCollection.createIndex({ provider: 1 });
    this.modelCollection.createIndex({ name: 1 });
    this.modelCollection.createIndex({ status: 1 });
    this.modelCollection.createIndex({ 'metadata.tags': 1 });
    
    this.providerCollection.createIndex({ id: 1 }, { unique: true });
    this.providerCollection.createIndex({ name: 1 });
    this.providerCollection.createIndex({ status: 1 });
  }
  
  async registerModel(model: Partial<Model>): Promise<string> {
    const id = model.id || `${model.provider}/${model.name}`;
    
    // Check if model already exists
    const existingModel = await this.modelCollection.findOne({ id });
    
    if (existingModel) {
      // Update existing model
      const updateDoc: any = { $set: {} };
      
      // Update fields
      Object.entries(model).forEach(([key, value]) => {
        if (key !== 'id' && value !== undefined) {
          updateDoc.$set[key] = value;
        }
      });
      
      // Always update timestamp
      updateDoc.$set['metadata.updated'] = new Date();
      
      await this.modelCollection.updateOne({ id }, updateDoc);
    } else {
      // Create new model
      const newModel: Model = {
        id,
        provider: model.provider || 'unknown',
        name: model.name || 'unknown',
        version: model.version || '1.0.0',
        capabilities: model.capabilities || {
          completion: true,
          chat: true,
          embeddings: false,
          function_calling: false,
          vision: false,
          audio: false
        },
        parameters: model.parameters || {
          context_length: 4096,
          token_limit: 4096,
          temperature_range: {
            min: 0,
            max: 1,
            default: 0.7
          },
          supported_languages: ['en']
        },
        performance: model.performance || {
          latency: {
            avg: 0,
            p50: 0,
            p95: 0,
            p99: 0
          },
          reliability: {
            success_rate: 1,
            error_rate: 0,
            availability: 1
          },
          quality: {
            benchmark_scores: {}
          }
        },
        cost: model.cost || {
          input_per_1k_tokens: 0,
          output_per_1k_tokens: 0,
          currency: 'USD'
        },
        status: model.status || 'active',
        metadata: {
          created: new Date(),
          updated: new Date(),
          description: model.metadata?.description || '',
          tags: model.metadata?.tags || []
        }
      };
      
      await this.modelCollection.insertOne(newModel);
      
      // Add to provider's models list
      await this.providerCollection.updateOne(
        { id: newModel.provider },
        { $addToSet: { models: id } }
      );
    }
    
    // Update cache
    await this.redis.set(`model:${id}`, JSON.stringify(await this.modelCollection.findOne({ id })));
    
    return id;
  }
  
  async getModel(id: string): Promise<Model | null> {
    // Try Redis first for performance
    const cachedModel = await this.redis.get(`model:${id}`);
    
    if (cachedModel) {
      return JSON.parse(cachedModel);
    }
    
    // Fall back to MongoDB
    const model = await this.modelCollection.findOne({ id });
    
    if (!model) return null;
    
    // Update cache
    await this.redis.set(`model:${id}`, JSON.stringify(model));
    
    return model as Model;
  }
  
  async updateModel(id: string, updates: Partial<Model>): Promise<Model | null> {
    const updateDoc: any = { $set: {} };
    
    // Build update document
    Object.entries(updates).forEach(([key, value]) => {
      if (key !== 'id' && value !== undefined) {
        updateDoc.$set[key] = value;
      }
    });
    
    // Always update timestamp
    updateDoc.$set['metadata.updated'] = new Date();
    
    // Update in MongoDB
    const result = await this.modelCollection.updateOne({ id }, updateDoc);
    
    if (result.modifiedCount === 0) return null;
    
    // Get updated model
    const updatedModel = await this.modelCollection.findOne({ id });
    
    if (!updatedModel) return null;
    
    // Update cache
    await this.redis.set(`model:${id}`, JSON.stringify(updatedModel));
    
    return updatedModel as Model;
  }
  
  async deleteModel(id: string): Promise<boolean> {
    const model = await this.getModel(id);
    
    if (!model) return false;
    
    // Remove from MongoDB
    const result = await this.modelCollection.deleteOne({ id });
    
    if (result.deletedCount === 0) return false;
    
    // Remove from provider's models list
    await this.providerCollection.updateOne(
      { id: model.provider },
      { $pull: { models: id } }
    );
    
    // Remove from cache
    await this.redis.del(`model:${id}`);
    
    return true;
  }
  
  async listModels(filter: any = {}): Promise<Model[]> {
    const models = await this.modelCollection.find(filter).toArray();
    return models as Model[];
  }
  
  async findModelsByCapabilities(capabilities: string[]): Promise<Model[]> {
    if (capabilities.length === 0) {
      return [];
    }
    
    // Build query
    const query: any = {};
    
    capabilities.forEach(capability => {
      query[`capabilities.${capability}`] = true;
    });
    
    // Only active models
    query.status = 'active';
    
    const models = await this.modelCollection.find(query).toArray();
    return models as Model[];
  }
  
  async registerProvider(provider: Partial<Provider>): Promise<string> {
    const id = provider.id || provider.name?.toLowerCase().replace(/\s+/g, '_') || uuidv4();
    
    // Check if provider already exists
    const existingProvider = await this.providerCollection.findOne({ id });
    
    if (existingProvider) {
      // Update existing provider
      const updateDoc: any = { $set: {} };
      
      // Update fields
      Object.entries(provider).forEach(([key, value]) => {
        if (key !== 'id' && value !== undefined) {
          updateDoc.$set[key] = value;
        }
      });
      
      await this.providerCollection.updateOne({ id }, updateDoc);
    } else {
      // Create new provider
      const newProvider: Provider = {
        id,
        name: provider.name || id,
        status: provider.status || 'active',
        models: provider.models || [],
        capabilities: provider.capabilities || {
          streaming: false,
          batch_processing: false,
          fine_tuning: false
        },
        rate_limits: provider.rate_limits || {
          requests_per_minute: 0,
          tokens_per_minute: 0
        },
        authentication: provider.authentication || {
          api_key: true,
          oauth: false
        },
        reliability: provider.reliability || {
          uptime: 1,
          incident_count: 0
        },
        metadata: provider.metadata || {
          website: '',
          documentation: '',
          support: ''
        }
      };
      
      await this.providerCollection.insertOne(newProvider);
    }
    
    // Update cache
    await this.redis.set(`provider:${id}`, JSON.stringify(await this.providerCollection.findOne({ id })));
    
    return id;
  }
  
  async getProvider(id: string): Promise<Provider | null> {
    // Try Redis first for performance
    const cachedProvider = await this.redis.get(`provider:${id}`);
    
    if (cachedProvider) {
      return JSON.parse(cachedProvider);
    }
    
    // Fall back to MongoDB
    const provider = await this.providerCollection.findOne({ id });
    
    if (!provider) return null;
    
    // Update cache
    await this.redis.set(`provider:${id}`, JSON.stringify(provider));
    
    return provider as Provider;
  }
  
  async updateProvider(id: string, updates: Partial<Provider>): Promise<Provider | null> {
    const updateDoc: any = { $set: {} };
    
    // Build update document
    Object.entries(updates).forEach(([key, value]) => {
      if (key !== 'id' && value !== undefined) {
        updateDoc.$set[key] = value;
      }
    });
    
    // Update in MongoDB
    const result = await this.providerCollection.updateOne({ id }, updateDoc);
    
    if (result.modifiedCount === 0) return null;
    
    // Get updated provider
    const updatedProvider = await this.providerCollection.findOne({ id });
    
    if (!updatedProvider) return null;
    
    // Update cache
    await this.redis.set(`provider:${id}`, JSON.stringify(updatedProvider));
    
    return updatedProvider as Provider;
  }
  
  async deleteProvider(id: string): Promise<boolean> {
    const provider = await this.getProvider(id);
    
    if (!provider) return false;
    
    // Check if provider has models
    if (provider.models.length > 0) {
      // Delete all provider models
      for (const modelId of provider.models) {
        await this.deleteModel(modelId);
      }
    }
    
    // Remove from MongoDB
    const result = await this.providerCollection.deleteOne({ id });
    
    if (result.deletedCount === 0) return false;
    
    // Remove from cache
    await this.redis.del(`provider:${id}`);
    
    return true;
  }
  
  async listProviders(filter: any = {}): Promise<Provider[]> {
    const providers = await this.providerCollection.find(filter).toArray();
    return providers as Provider[];
  }
  
  async updateModelPerformance(id: string, metrics: Partial<Model['performance']>): Promise<boolean> {
    const model = await this.getModel(id);
    
    if (!model) return false;
    
    // Update performance metrics
    const performance = {
      ...model.performance,
      ...metrics
    };
    
    // Update in MongoDB and Redis
    const updated = await this.updateModel(id, { performance });
    
    return !!updated;
  }
  
  async updateProviderStatus(id: string, status: ProviderStatus): Promise<boolean> {
    const provider = await this.getProvider(id);
    
    if (!provider) return false;
    
    // Update in MongoDB and Redis
    const updated = await this.updateProvider(id, { status });
    
    return !!updated;
  }
  
  async registerDefaultModels(): Promise<void> {
    // Register default providers
    const providers = [
      {
        id: 'openai',
        name: 'OpenAI',
        status: 'active',
        capabilities: {
          streaming: true,
          batch_processing: false,
          fine_tuning: true
        },
        rate_limits: {
          requests_per_minute: 3500,
          tokens_per_minute: 90000
        },
        authentication: {
          api_key: true,
          oauth: false
        },
        reliability: {
          uptime: 0.995,
          incident_count: 0
        },
        metadata: {
          website: 'https://openai.com',
          documentation: 'https://platform.openai.com/docs',
          support: 'https://help.openai.com'
        }
      },
      {
        id: 'anthropic',
        name: 'Anthropic',
        status: 'active',
        capabilities: {
          streaming: true,
          batch_processing: false,
          fine_tuning: false
        },
        rate_limits: {
          requests_per_minute: 1000,
          tokens_per_minute: 50000
        },
        authentication: {
          api_key: true,
          oauth: false
        },
        reliability: {
          uptime: 0.99,
          incident_count: 0
        },
        metadata: {
          website: 'https://anthropic.com',
          documentation: 'https://docs.anthropic.com',
          support: 'https://support.anthropic.com'
        }
      },
      {
        id: 'google',
        name: 'Google',
        status: 'active',
        capabilities: {
          streaming: true,
          batch_processing: false,
          fine_tuning: false
        },
        rate_limits: {
          requests_per_minute: 1000,
          tokens_per_minute: 50000
        },
        authentication: {
          api_key: true,
          oauth: true
        },
        reliability: {
          uptime: 0.99,
          incident_count: 0
        },
        metadata: {
          website: 'https://cloud.google.com',
          documentation: 'https://cloud.google.com/vertex-ai/docs',
          support: 'https://cloud.google.com/support'
        }
      },
      {
        id: 'openrouter',
        name: 'OpenRouter',
        status: 'active',
        capabilities: {
          streaming: true,
          batch_processing: false,
          fine_tuning: false
        },
        rate_limits: {
          requests_per_minute: 500,
          tokens_per_minute: 30000
        },
        authentication: {
          api_key: true,
          oauth: false
        },
        reliability: {
          uptime: 0.98,
          incident_count: 0
        },
        metadata: {
          website: 'https://openrouter.ai',
          documentation: 'https://openrouter.ai/docs',
          support: 'https://openrouter.ai/discord'
        }
      }
    ];
    
    for (const providerData of providers) {
      await this.registerProvider(providerData);
    }
    
    // Register default models
    const models = [
      // OpenAI models
      {
        provider: 'openai',
        name: 'gpt-4-turbo',
        version: '0125-preview',
        capabilities: {
          completion: true,
          chat: true,
          embeddings: false,
          function_calling: true,
          vision: true,
          audio: false
        },
        parameters: {
          context_length: 128000,
          token_limit: 4096,
          temperature_range: {
            min: 0,
            max: 2,
            default: 1
          },
          supported_languages: ['en', 'es', 'fr', 'de', 'it', 'pt', 'nl', 'ru', 'zh', 'ja', 'ko', 'ar']
        },
        performance: {
          latency: {
            avg: 2000,
            p50: 1800,
            p95: 3500,
            p99: 5000
          },
          reliability: {
            success_rate: 0.995,
            error_rate: 0.005,
            availability: 0.995
          },
          quality: {
            benchmark_scores: {
              mmlu: 0.86,
              hellaswag: 0.95,
              truthfulqa: 0.65
            }
          }
        },
        cost: {
          input_per_1k_tokens: 0.01,
          output_per_1k_tokens: 0.03,
          currency: 'USD'
        },
        status: 'active',
        metadata: {
          description: 'GPT-4 Turbo with improved instruction following and JSON mode',
          tags: ['gpt-4', 'large', 'high-quality']
        }
      },
      {
        provider: 'openai',
        name: 'gpt-3.5-turbo',
        version: '0125',
        capabilities: {
          completion: true,
          chat: true,
          embeddings: false,
          function_calling: true,
          vision: true,
          audio: false
        },
        parameters: {
          context_length: 16385,
          token_limit: 4096,
          temperature_range: {
            min: 0,
            max: 2,
            default: 1
          },
          supported_languages: ['en', 'es', 'fr', 'de', 'it', 'pt', 'nl', 'ru', 'zh', 'ja', 'ko', 'ar']
        },
        performance: {
          latency: {
            avg: 800,
            p50: 700,
            p95: 1500,
            p99: 2500
          },
          reliability: {
            success_rate: 0.998,
            error_rate: 0.002,
            availability: 0.998
          },
          quality: {
            benchmark_scores: {
              mmlu: 0.7,
              hellaswag: 0.85,
              truthfulqa: 0.5
            }
          }
        },
        cost: {
          input_per_1k_tokens: 0.0005,
          output_per_1k_tokens: 0.0015,
          currency: 'USD'
        },
        status: 'active',
        metadata: {
          description: 'GPT-3.5 Turbo with improved instruction following',
          tags: ['gpt-3.5', 'medium', 'cost-effective']
        }
      },
      
      // Anthropic models
      {
        provider: 'anthropic',
        name: 'claude-3-opus',
        version: '20240229',
        capabilities: {
          completion: true,
          chat: true,
          embeddings: false,
          function_calling: true,
          vision: true,
          audio: false
        },
        parameters: {
          context_length: 200000,
          token_limit: 4096,
          temperature_range: {
            min: 0,
            max: 1,
            default: 0.7
          },
          supported_languages: ['en', 'es', 'fr', 'de', 'it', 'pt', 'nl', 'ru', 'zh', 'ja', 'ko']
        },
        performance: {
          latency: {
            avg: 3000,
            p50: 2800,
            p95: 5000,
            p99: 7000
          },
          reliability: {
            success_rate: 0.99,
            error_rate: 0.01,
            availability: 0.99
          },
          quality: {
            benchmark_scores: {
              mmlu: 0.89,
              hellaswag: 0.95,
              truthfulqa: 0.7
            }
          }
        },
        cost: {
          input_per_1k_tokens: 0.015,
          output_per_1k_tokens: 0.075,
          currency: 'USD'
        },
        status: 'active',
        metadata: {
          description: 'Claude 3 Opus - Anthropic\'s most powerful model',
          tags: ['claude', 'large', 'high-quality']
        }
      },
      {
        provider: 'anthropic',
        name: 'claude-3-sonnet',
        version: '20240229',
        capabilities: {
          completion: true,
          chat: true,
          embeddings: false,
          function_calling: true,
          vision: true,
          audio: false
        },
        parameters: {
          context_length: 200000,
          token_limit: 4096,
          temperature_range: {
            min: 0,
            max: 1,
            default: 0.7
          },
          supported_languages: ['en', 'es', 'fr', 'de', 'it', 'pt', 'nl', 'ru', 'zh', 'ja', 'ko']
        },
        performance: {
          latency: {
            avg: 1500,
            p50: 1300,
            p95: 2500,
            p99: 4000
          },
          reliability: {
            success_rate: 0.995,
            error_rate: 0.005,
            availability: 0.995
          },
          quality: {
            benchmark_scores: {
              mmlu: 0.85,
              hellaswag: 0.92,
              truthfulqa: 0.65
            }
          }
        },
        cost: {
          input_per_1k_tokens: 0.003,
          output_per_1k_tokens: 0.015,
          currency: 'USD'
        },
        status: 'active',
        metadata: {
          description: 'Claude 3 Sonnet - Balanced performance and cost',
          tags: ['claude', 'medium', 'balanced']
        }
      }
    ];
    
    for (const modelData of models) {
      await this.registerModel(modelData);
    }
  }
}
```
