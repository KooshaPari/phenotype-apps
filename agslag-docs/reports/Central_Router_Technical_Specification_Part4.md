# Central Router Technical Specification - Part 4: Implementation and Integration

## 1. Integration with Oblix

### 1.1 Overview

The Oblix integration provides intelligent orchestration between local edge models and cloud-based LLMs, enabling dynamic routing based on system resources, network connectivity, and other factors. This section details the implementation of the Oblix integration in the Central Router system.

### 1.2 Oblix Integration Implementation

```typescript
class OblixIntegration implements OblixIntegrationInterface {
  private client: OblixClient;
  private models: Map<string, OblixModelInfo>;
  private initialized: boolean = false;
  
  constructor(private config: OblixConfig) {
    this.client = new OblixClient({
      apiKey: config.apiKey,
      host: config.host,
      port: config.port,
    });
    this.models = new Map();
  }
  
  async initialize(): Promise<void> {
    if (this.initialized) {
      return;
    }
    
    try {
      // Initialize Oblix client
      await this.client.initialize();
      
      // Register models
      await this.registerModels();
      
      // Add monitoring agents
      await this.registerMonitoringAgents();
      
      this.initialized = true;
      console.log('Oblix integration initialized successfully');
    } catch (error) {
      console.error('Failed to initialize Oblix integration:', error);
      throw new Error(`Oblix initialization failed: ${error.message}`);
    }
  }
  
  async registerModels(): Promise<void> {
    // Register local models
    for (const model of this.config.localModels) {
      try {
        await this.client.hookModel(
          ModelType.OLLAMA,
          model.name,
          model.params
        );
        
        this.models.set(model.id, {
          id: model.id,
          name: model.name,
          type: ModelType.OLLAMA,
          isLocal: true,
        });
        
        console.log(`Registered local model: ${model.name}`);
      } catch (error) {
        console.error(`Failed to register local model ${model.name}:`, error);
      }
    }
    
    // Register cloud models
    for (const model of this.config.cloudModels) {
      try {
        await this.client.hookModel(
          model.type,
          model.name,
          {
            api_key: model.apiKey,
            ...model.params,
          }
        );
        
        this.models.set(model.id, {
          id: model.id,
          name: model.name,
          type: model.type,
          isLocal: false,
        });
        
        console.log(`Registered cloud model: ${model.name}`);
      } catch (error) {
        console.error(`Failed to register cloud model ${model.name}:`, error);
      }
    }
  }
  
  async registerMonitoringAgents(): Promise<void> {
    // Register resource monitor
    this.client.hookAgent(new ResourceMonitor());
    
    // Register connectivity agent
    this.client.hookAgent(new ConnectivityAgent());
    
    // Register cost optimization agent
    this.client.hookAgent(new CostOptimizationAgent());
    
    console.log('Registered Oblix monitoring agents');
  }
  
  async routeRequest(
    request: OblixRequest,
    options: OblixRoutingOptions
  ): Promise<OblixResponse> {
    if (!this.initialized) {
      await this.initialize();
    }
    
    try {
      // Build routing options
      const routingOptions = this.buildRoutingOptions(options);
      
      // Execute request with automatic routing
      const response = await this.client.execute(
        request.prompt,
        routingOptions
      );
      
      return {
        response: response.response,
        modelId: response.model_id,
        modelType: response.model_type,
        tokenUsage: {
          input: response.token_usage?.input_tokens || 0,
          output: response.token_usage?.output_tokens || 0,
          total: response.token_usage?.total_tokens || 0,
        },
        cost: response.cost || 0,
        executionTime: response.execution_time || 0,
      };
    } catch (error) {
      console.error('Error routing request through Oblix:', error);
      throw new Error(`Oblix routing failed: ${error.message}`);
    }
  }
  
  async getAvailableModels(): Promise<OblixModelInfo[]> {
    if (!this.initialized) {
      await this.initialize();
    }
    
    return Array.from(this.models.values());
  }
  
  async getModelPerformance(modelId: string): Promise<OblixModelPerformance> {
    if (!this.initialized) {
      await this.initialize();
    }
    
    try {
      const model = this.models.get(modelId);
      if (!model) {
        throw new Error(`Model not found: ${modelId}`);
      }
      
      const performance = await this.client.getModelPerformance(
        model.type,
        model.name
      );
      
      return {
        modelId,
        avgLatency: performance.avg_latency,
        avgTokensPerSecond: performance.avg_tokens_per_second,
        successRate: performance.success_rate,
        costPerRequest: performance.cost_per_request,
        totalRequests: performance.total_requests,
      };
    } catch (error) {
      console.error(`Error getting model performance for ${modelId}:`, error);
      throw new Error(`Failed to get model performance: ${error.message}`);
    }
  }
  
  private buildRoutingOptions(options: OblixRoutingOptions): any {
    return {
      routing_strategy: options.strategy || 'auto',
      cost_limit: options.costLimit,
      latency_requirement: options.latencyRequirement,
      privacy_level: options.privacyLevel,
      prefer_local: options.preferLocal,
      fallback_enabled: options.fallbackEnabled !== false,
      max_retries: options.maxRetries || 2,
      timeout: options.timeout || 30000,
    };
  }
}
```

### 1.3 Oblix Configuration

```typescript
interface OblixConfig {
  apiKey: string;
  host?: string;
  port?: number;
  localModels: OblixLocalModelConfig[];
  cloudModels: OblixCloudModelConfig[];
}

interface OblixLocalModelConfig {
  id: string;
  name: string;
  params?: Record<string, any>;
}

interface OblixCloudModelConfig {
  id: string;
  name: string;
  type: ModelType;
  apiKey: string;
  params?: Record<string, any>;
}

enum ModelType {
  OPENAI = 'openai',
  ANTHROPIC = 'anthropic',
  OLLAMA = 'ollama',
  GOOGLE = 'google',
  MISTRAL = 'mistral',
  COHERE = 'cohere',
}
```

## 2. Integration with Wren Engine

### 2.1 Overview

The Wren Engine integration provides prompt routing capabilities, selecting the best model based on prompt content, cost, latency, and quality requirements. It also handles prompt rewriting and adaptation for different models.

### 2.2 Wren Integration Implementation

```typescript
class WrenIntegration implements WrenIntegrationInterface {
  private engine: WrenClient;
  private adapters: Map<string, ModelAdapter>;
  private initialized: boolean = false;
  
  constructor(private config: WrenConfig) {
    this.engine = new WrenClient({
      apiKey: config.apiKey,
      baseUrl: config.baseUrl,
    });
    this.adapters = new Map();
  }
  
  async initialize(): Promise<void> {
    if (this.initialized) {
      return;
    }
    
    try {
      // Initialize Wren client
      await this.engine.initialize();
      
      // Register model adapters
      await this.registerAdapters();
      
      // Configure routing strategies
      await this.configureRoutingStrategies();
      
      this.initialized = true;
      console.log('Wren Engine integration initialized successfully');
    } catch (error) {
      console.error('Failed to initialize Wren Engine integration:', error);
      throw new Error(`Wren initialization failed: ${error.message}`);
    }
  }
  
  async registerAdapters(): Promise<void> {
    // Register adapters for different model types
    for (const adapter of this.config.adapters) {
      try {
        this.adapters.set(
          adapter.modelId,
          new ModelAdapter(adapter.modelId, adapter.config)
        );
        
        await this.engine.registerAdapter(
          adapter.modelId,
          adapter.config
        );
        
        console.log(`Registered adapter for model: ${adapter.modelId}`);
      } catch (error) {
        console.error(`Failed to register adapter for ${adapter.modelId}:`, error);
      }
    }
  }
  
  async configureRoutingStrategies(): Promise<void> {
    // Configure routing strategies
    for (const strategy of this.config.routingStrategies) {
      try {
        await this.engine.registerRoutingStrategy(
          strategy.name,
          strategy.config
        );
        
        console.log(`Registered routing strategy: ${strategy.name}`);
      } catch (error) {
        console.error(`Failed to register routing strategy ${strategy.name}:`, error);
      }
    }
  }
  
  async routePrompt(
    prompt: string,
    options: WrenRoutingOptions
  ): Promise<WrenRoutingResult> {
    if (!this.initialized) {
      await this.initialize();
    }
    
    try {
      // Analyze prompt
      const analysis = await this.engine.analyzePrompt(prompt);
      
      // Select appropriate model
      const modelId = await this.engine.selectModel(
        analysis,
        this.buildRoutingOptions(options)
      );
      
      // Adapt prompt for selected model
      const adaptedPrompt = await this.adaptPrompt(
        prompt,
        modelId
      );
      
      return {
        modelId,
        adaptedPrompt,
        analysis: {
          complexity: analysis.complexity,
          taskType: analysis.task_type,
          tokenEstimate: analysis.token_estimate,
          features: analysis.features,
        },
      };
    } catch (error) {
      console.error('Error routing prompt through Wren Engine:', error);
      throw new Error(`Wren routing failed: ${error.message}`);
    }
  }
  
  async adaptPrompt(
    prompt: string,
    modelId: string
  ): Promise<string> {
    if (!this.initialized) {
      await this.initialize();
    }
    
    try {
      const adapter = this.adapters.get(modelId);
      if (!adapter) {
        throw new Error(`Adapter not found for model: ${modelId}`);
      }
      
      return await adapter.adaptPrompt(prompt);
    } catch (error) {
      console.error(`Error adapting prompt for model ${modelId}:`, error);
      throw new Error(`Prompt adaptation failed: ${error.message}`);
    }
  }
  
  async getAvailableAdapters(): Promise<WrenAdapterInfo[]> {
    if (!this.initialized) {
      await this.initialize();
    }
    
    return Array.from(this.adapters.entries()).map(([modelId, adapter]) => ({
      modelId,
      capabilities: adapter.getCapabilities(),
    }));
  }
  
  private buildRoutingOptions(options: WrenRoutingOptions): any {
    return {
      strategy: options.strategy || 'default',
      cost_weight: options.weights?.cost || 0.2,
      latency_weight: options.weights?.latency || 0.2,
      quality_weight: options.weights?.quality || 0.2,
      model_tier: options.modelTier,
      excluded_models: options.excludedModels || [],
      preferred_models: options.preferredModels || [],
    };
  }
}

class ModelAdapter {
  constructor(
    private modelId: string,
    private config: any
  ) {}
  
  async adaptPrompt(prompt: string): Promise<string> {
    // Apply model-specific adaptations
    let adaptedPrompt = prompt;
    
    // Apply prefix if configured
    if (this.config.prefix) {
      adaptedPrompt = `${this.config.prefix}\n\n${adaptedPrompt}`;
    }
    
    // Apply suffix if configured
    if (this.config.suffix) {
      adaptedPrompt = `${adaptedPrompt}\n\n${this.config.suffix}`;
    }
    
    // Apply token limit adjustments if needed
    if (this.config.maxTokens && this.getTokenCount(adaptedPrompt) > this.config.maxTokens) {
      adaptedPrompt = this.truncateToTokenLimit(adaptedPrompt, this.config.maxTokens);
    }
    
    // Apply model-specific transformations
    if (this.config.transformations) {
      for (const transformation of this.config.transformations) {
        adaptedPrompt = this.applyTransformation(adaptedPrompt, transformation);
      }
    }
    
    return adaptedPrompt;
  }
  
  getCapabilities(): string[] {
    return this.config.capabilities || [];
  }
  
  private getTokenCount(text: string): number {
    // Simple estimation: ~4 characters per token
    return Math.ceil(text.length / 4);
  }
  
  private truncateToTokenLimit(text: string, limit: number): string {
    // Simple truncation strategy
    const estimatedChars = limit * 4;
    if (text.length <= estimatedChars) {
      return text;
    }
    
    // Try to truncate at a sentence boundary
    const truncated = text.substring(0, estimatedChars);
    const lastPeriod = truncated.lastIndexOf('.');
    
    if (lastPeriod > estimatedChars * 0.7) {
      return truncated.substring(0, lastPeriod + 1);
    }
    
    return truncated;
  }
  
  private applyTransformation(text: string, transformation: any): string {
    switch (transformation.type) {
      case 'replace':
        return text.replace(
          new RegExp(transformation.pattern, transformation.flags || 'g'),
          transformation.replacement
        );
      case 'format':
        return transformation.template.replace(
          /\{content\}/g,
          text
        );
      default:
        return text;
    }
  }
}
```

### 2.3 Wren Configuration

```typescript
interface WrenConfig {
  apiKey: string;
  baseUrl?: string;
  adapters: WrenAdapterConfig[];
  routingStrategies: WrenRoutingStrategyConfig[];
}

interface WrenAdapterConfig {
  modelId: string;
  config: {
    prefix?: string;
    suffix?: string;
    maxTokens?: number;
    capabilities?: string[];
    transformations?: WrenTransformation[];
  };
}

interface WrenRoutingStrategyConfig {
  name: string;
  config: Record<string, any>;
}

interface WrenTransformation {
  type: 'replace' | 'format';
  pattern?: string;
  flags?: string;
  replacement?: string;
  template?: string;
}
```

## 3. Integration with MCP Auto Register

### 3.1 Overview

The MCP Auto Register integration provides automatic discovery and registration of MCP servers and tools, enabling dynamic tool discovery and usage in the Central Router system.

### 3.2 MCP Auto Register Implementation

```typescript
class McpAutoRegister implements McpAutoRegisterInterface {
  private registry: McpRegistry;
  private discoveredServers: Map<string, McpServerInfo>;
  private initialized: boolean = false;
  
  constructor(private config: McpAutoRegisterConfig) {
    this.registry = new McpRegistry();
    this.discoveredServers = new Map();
  }
  
  async initialize(): Promise<void> {
    if (this.initialized) {
      return;
    }
    
    try {
      // Initialize registry
      await this.registry.initialize(this.config);
      
      // Discover available servers
      await this.discoverServers();
      
      // Start health check monitoring
      this.startHealthChecks();
      
      this.initialized = true;
      console.log('MCP Auto Register initialized successfully');
    } catch (error) {
      console.error('Failed to initialize MCP Auto Register:', error);
      throw new Error(`MCP Auto Register initialization failed: ${error.message}`);
    }
  }
  
  async discoverServers(): Promise<McpServerInfo[]> {
    try {
      // Discover servers from configured sources
      const servers: McpServerInfo[] = [];
      
      // Discover from local configuration
      if (this.config.localServers) {
        for (const server of this.config.localServers) {
          servers.push({
            id: server.id,
            name: server.name,
            url: server.url,
            type: 'local',
            status: 'unknown',
          });
        }
      }
      
      // Discover from network
      if (this.config.enableNetworkDiscovery) {
        const networkServers = await this.registry.discoverNetworkServers();
        servers.push(...networkServers);
      }
      
      // Discover from registry service
      if (this.config.registryServiceUrl) {
        const registryServers = await this.registry.discoverFromRegistry(
          this.config.registryServiceUrl
        );
        servers.push(...registryServers);
      }
      
      // Register discovered servers
      for (const server of servers) {
        await this.registerServer(server);
      }
      
      return Array.from(this.discoveredServers.values());
    } catch (error) {
      console.error('Error discovering MCP servers:', error);
      throw new Error(`Server discovery failed: ${error.message}`);
    }
  }
  
  async registerServer(server: McpServerInfo): Promise<void> {
    try {
      // Check if server is already registered
      if (this.discoveredServers.has(server.id)) {
        // Update existing server
        this.discoveredServers.set(server.id, {
          ...this.discoveredServers.get(server.id),
          ...server,
        });
        console.log(`Updated MCP server: ${server.id} (${server.name})`);
        return;
      }
      
      // Register server with registry
      await this.registry.registerServer(server);
      
      // Store server information
      this.discoveredServers.set(server.id, server);
      
      console.log(`Registered MCP server: ${server.id} (${server.name})`);
    } catch (error) {
      console.error(`Error registering MCP server ${server.id}:`, error);
      throw new Error(`Server registration failed: ${error.message}`);
    }
  }
  
  async getAvailableTools(): Promise<McpToolInfo[]> {
    if (!this.initialized) {
      await this.initialize();
    }
    
    try {
      const tools: McpToolInfo[] = [];
      
      // Get tools from all registered servers
      for (const server of this.discoveredServers.values()) {
        if (server.status === 'online') {
          const serverTools = await this.registry.getServerTools(server.id);
          tools.push(...serverTools);
        }
      }
      
      return tools;
    } catch (error) {
      console.error('Error getting available tools:', error);
      throw new Error(`Failed to get available tools: ${error.message}`);
    }
  }
  
  async executeTool(
    toolId: string,
    params: any
  ): Promise<any> {
    if (!this.initialized) {
      await this.initialize();
    }
    
    try {
      // Find tool
      const tool = await this.findTool(toolId);
      if (!tool) {
        throw new Error(`Tool not found: ${toolId}`);
      }
      
      // Execute tool
      return await this.registry.executeTool(
        tool.serverId,
        tool.id,
        params
      );
    } catch (error) {
      console.error(`Error executing tool ${toolId}:`, error);
      throw new Error(`Tool execution failed: ${error.message}`);
    }
  }
  
  private async findTool(toolId: string): Promise<McpToolInfo | null> {
    const tools = await this.getAvailableTools();
    return tools.find(tool => tool.id === toolId) || null;
  }
  
  private startHealthChecks(): void {
    // Start periodic health checks
    setInterval(async () => {
      try {
        for (const [serverId, server] of this.discoveredServers.entries()) {
          try {
            const status = await this.registry.checkServerHealth(serverId);
            
            // Update server status
            this.discoveredServers.set(serverId, {
              ...server,
              status,
              lastChecked: new Date(),
            });
            
            if (status === 'offline' && server.status === 'online') {
              console.warn(`MCP server ${serverId} is now offline`);
            } else if (status === 'online' && server.status === 'offline') {
              console.log(`MCP server ${serverId} is now online`);
            }
          } catch (error) {
            console.error(`Error checking health of server ${serverId}:`, error);
            
            // Mark server as offline
            this.discoveredServers.set(serverId, {
              ...server,
              status: 'offline',
              lastChecked: new Date(),
            });
          }
        }
      } catch (error) {
        console.error('Error during health checks:', error);
      }
    }, this.config.healthCheckInterval || 60000);
  }
}
```

### 3.3 MCP Auto Register Configuration

```typescript
interface McpAutoRegisterConfig {
  localServers?: McpLocalServerConfig[];
  enableNetworkDiscovery?: boolean;
  registryServiceUrl?: string;
  healthCheckInterval?: number;
  autoReconnect?: boolean;
  securitySettings?: McpSecuritySettings;
}

interface McpLocalServerConfig {
  id: string;
  name: string;
  url: string;
  apiKey?: string;
}

interface McpSecuritySettings {
  requireAuthentication?: boolean;
  allowedServers?: string[];
  allowedTools?: string[];
  disallowedTools?: string[];
}
```

## 4. Implementation Plan

### 4.1 Phase 1: Core Infrastructure (Weeks 1-2)

| Week | Task | Description | Owner | Dependencies |
|------|------|-------------|-------|--------------|
| 1 | Set up project structure | Create repository, configure build tools, set up CI/CD | Lead Developer | None |
| 1 | Implement core interfaces | Define and implement core interfaces and data models | Backend Developer | Project setup |
| 1 | Implement Request Analyzer | Create basic request analysis functionality | ML Engineer | Core interfaces |
| 1 | Implement Routing Engine | Create basic routing functionality | Backend Developer | Core interfaces |
| 1 | Implement Execution Engine | Create basic execution functionality | Backend Developer | Core interfaces |
| 2 | Implement API Gateway | Create API endpoints and request handling | Backend Developer | Core components |
| 2 | Implement Configuration Manager | Create configuration loading and management | DevOps Engineer | Core components |
| 2 | Implement basic monitoring | Set up logging and basic metrics | DevOps Engineer | Core components |
| 2 | Integration tests | Create and run integration tests for core components | QA Engineer | All core components |

### 4.2 Phase 2: Integration Components (Weeks 3-4)

| Week | Task | Description | Owner | Dependencies |
|------|------|-------------|-------|--------------|
| 3 | Implement Oblix Integration | Create Oblix client and integration | Backend Developer | Core components |
| 3 | Implement Wren Integration | Create Wren client and integration | Backend Developer | Core components |
| 3 | Implement MCP Auto Register | Create MCP Auto Register functionality | Backend Developer | Core components |
| 3 | Implement Model Registry | Create model registration and management | Backend Developer | Integration components |
| 4 | Implement Tool Registry | Create tool registration and management | Backend Developer | Integration components |
| 4 | Implement Model Execution | Create model execution functionality | Backend Developer | Model Registry |
| 4 | Implement Tool Execution | Create tool execution functionality | Backend Developer | Tool Registry |
| 4 | Integration tests | Create and run integration tests for integration components | QA Engineer | All integration components |

### 4.3 Phase 3: Optimization Algorithms (Weeks 5-6)

| Week | Task | Description | Owner | Dependencies |
|------|------|-------------|-------|--------------|
| 5 | Implement Cost Optimization | Create cost optimization algorithms | ML Engineer | Core components |
| 5 | Implement Latency Optimization | Create latency optimization algorithms | ML Engineer | Core components |
| 5 | Implement Quality Optimization | Create quality optimization algorithms | ML Engineer | Core components |
| 5 | Implement Privacy Controls | Create privacy control functionality | Security Engineer | Core components |
| 6 | Implement Model Selection | Create model selection algorithms | ML Engineer | Optimization algorithms |
| 6 | Implement Caching | Create response caching functionality | Backend Developer | Core components |
| 6 | Implement Fallback Mechanisms | Create fallback handling | Backend Developer | Core components |
| 6 | Integration tests | Create and run integration tests for optimization components | QA Engineer | All optimization components |

### 4.4 Phase 4: Monitoring and Deployment (Weeks 7-8)

| Week | Task | Description | Owner | Dependencies |
|------|------|-------------|-------|--------------|
| 7 | Implement Comprehensive Monitoring | Create detailed monitoring and alerting | DevOps Engineer | All components |
| 7 | Implement Analytics Dashboard | Create analytics dashboard | Frontend Developer | Monitoring system |
| 7 | Performance Testing | Conduct performance testing and optimization | Performance Engineer | All components |
| 7 | Security Testing | Conduct security testing and hardening | Security Engineer | All components |
| 8 | Documentation | Create comprehensive documentation | Technical Writer | All components |
| 8 | User Guides | Create user guides and tutorials | Technical Writer | All components |
| 8 | Deployment Preparation | Prepare for production deployment | DevOps Engineer | All components |
| 8 | Production Deployment | Deploy to production environment | DevOps Engineer | All components |

## 5. Conclusion

The Central Router system provides a comprehensive solution for intelligent routing of requests to the most appropriate AI models and tools based on multiple factors including cost, performance, latency, and privacy requirements. By leveraging Oblix for model orchestration, Wren Engine for prompt routing, and MCP Auto Register for dynamic tool discovery, the system creates a flexible, efficient, and scalable foundation for the Jarvis agent ecosystem.

The modular architecture and comprehensive optimization strategies ensure that the system can adapt to changing requirements and scale to support future growth. The detailed monitoring and analytics capabilities provide valuable insights for continuous improvement and optimization.

This technical specification provides a comprehensive blueprint for implementing the Central Router system, including system architecture, core components, algorithms, integration points, and an implementation plan. By following this specification, we can create a robust, efficient, and flexible routing system that enhances the overall Jarvis experience.
