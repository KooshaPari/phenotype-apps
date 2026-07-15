# Central Router Technical Specification - Part 2: Optimization Algorithms

## 1. Cost Optimization

### 1.1 Overview

The Cost Optimization system intelligently routes requests to the most cost-effective models and tools while maintaining quality requirements. It analyzes request complexity, estimates token usage, and selects models based on cost-efficiency.

### 1.2 Cost Optimization Algorithm

```typescript
class CostOptimizer {
  private modelCostMap: Map<string, ModelCostInfo>;
  private budgetManager: BudgetManager;
  
  constructor() {
    this.modelCostMap = new Map();
    this.budgetManager = new BudgetManager();
    this.initializeModelCostMap();
  }
  
  optimize(analysis: RequestAnalysis, config: SystemConfig): CostOptimizationParams {
    // Get available budget
    const availableBudget = this.budgetManager.getAvailableBudget(analysis.taskType);
    
    // Estimate cost for different model tiers
    const costEstimates = this.estimateCostsForModelTiers(analysis);
    
    // Determine cost-optimal tier based on complexity and budget
    const optimalTier = this.determineOptimalTier(
      analysis.complexity,
      costEstimates,
      availableBudget,
      config.costSensitivity
    );
    
    // Calculate token budget
    const tokenBudget = this.calculateTokenBudget(
      optimalTier,
      analysis.tokenEstimate,
      availableBudget
    );
    
    return {
      modelTier: optimalTier,
      tokenBudget,
      costLimit: this.calculateCostLimit(optimalTier, availableBudget),
    };
  }
  
  private estimateCostsForModelTiers(analysis: RequestAnalysis): ModelTierCostEstimates {
    const { tokenEstimate } = analysis;
    
    // Get cost per token for each model tier
    const basicTierCost = this.getAverageCostPerToken('basic');
    const standardTierCost = this.getAverageCostPerToken('standard');
    const premiumTierCost = this.getAverageCostPerToken('premium');
    
    // Calculate estimated costs
    return {
      basic: {
        inputCost: basicTierCost.input * tokenEstimate.input,
        outputCost: basicTierCost.output * tokenEstimate.output,
        totalCost: (basicTierCost.input * tokenEstimate.input) + 
                   (basicTierCost.output * tokenEstimate.output),
      },
      standard: {
        inputCost: standardTierCost.input * tokenEstimate.input,
        outputCost: standardTierCost.output * tokenEstimate.output,
        totalCost: (standardTierCost.input * tokenEstimate.input) + 
                   (standardTierCost.output * tokenEstimate.output),
      },
      premium: {
        inputCost: premiumTierCost.input * tokenEstimate.input,
        outputCost: premiumTierCost.output * tokenEstimate.output,
        totalCost: (premiumTierCost.input * tokenEstimate.input) + 
                   (premiumTierCost.output * tokenEstimate.output),
      },
    };
  }
  
  private determineOptimalTier(
    complexity: ComplexityLevel,
    costEstimates: ModelTierCostEstimates,
    availableBudget: number,
    costSensitivity: number
  ): ModelTier {
    // Default tier based on complexity
    let defaultTier: ModelTier;
    switch (complexity) {
      case ComplexityLevel.SIMPLE:
        defaultTier = 'basic';
        break;
      case ComplexityLevel.MODERATE:
        defaultTier = 'standard';
        break;
      case ComplexityLevel.COMPLEX:
      case ComplexityLevel.VERY_COMPLEX:
        defaultTier = 'premium';
        break;
      default:
        defaultTier = 'standard';
    }
    
    // If cost sensitivity is high, try to use lower tier
    if (costSensitivity > 0.7) {
      if (defaultTier === 'premium' && 
          this.isTierSufficientForComplexity('standard', complexity)) {
        defaultTier = 'standard';
      } else if (defaultTier === 'standard' && 
                this.isTierSufficientForComplexity('basic', complexity)) {
        defaultTier = 'basic';
      }
    }
    
    // If cost sensitivity is low, try to use higher tier
    if (costSensitivity < 0.3) {
      if (defaultTier === 'basic') {
        defaultTier = 'standard';
      } else if (defaultTier === 'standard' && 
                costEstimates.premium.totalCost <= availableBudget) {
        defaultTier = 'premium';
      }
    }
    
    // Ensure we don't exceed budget
    if (defaultTier === 'premium' && 
        costEstimates.premium.totalCost > availableBudget) {
      defaultTier = 'standard';
    }
    
    if (defaultTier === 'standard' && 
        costEstimates.standard.totalCost > availableBudget) {
      defaultTier = 'basic';
    }
    
    return defaultTier;
  }
  
  private calculateTokenBudget(
    tier: ModelTier,
    tokenEstimate: TokenEstimate,
    availableBudget: number
  ): TokenBudget {
    const costPerToken = this.getAverageCostPerToken(tier);
    
    // Calculate maximum tokens based on budget
    const maxInputTokens = Math.floor(availableBudget / costPerToken.input);
    const maxOutputTokens = Math.floor(availableBudget / costPerToken.output);
    
    // Adjust based on estimates
    const inputTokenBudget = Math.min(
      maxInputTokens,
      tokenEstimate.input * 1.2 // Add 20% buffer
    );
    
    const outputTokenBudget = Math.min(
      maxOutputTokens,
      tokenEstimate.output * 1.2 // Add 20% buffer
    );
    
    return {
      input: inputTokenBudget,
      output: outputTokenBudget,
      total: inputTokenBudget + outputTokenBudget,
    };
  }
  
  private calculateCostLimit(tier: ModelTier, availableBudget: number): number {
    // Use a percentage of available budget based on tier
    switch (tier) {
      case 'basic':
        return availableBudget * 0.5; // 50% of available budget
      case 'standard':
        return availableBudget * 0.7; // 70% of available budget
      case 'premium':
        return availableBudget * 0.9; // 90% of available budget
      default:
        return availableBudget * 0.7;
    }
  }
}
```

### 1.3 Token Estimation Algorithm

```typescript
class TokenEstimator {
  private tokenizers: Map<string, Tokenizer>;
  
  constructor() {
    this.tokenizers = new Map();
    this.initializeTokenizers();
  }
  
  estimateTokens(text: string, modelType: string = 'gpt-3.5-turbo'): number {
    const tokenizer = this.getTokenizer(modelType);
    return tokenizer.countTokens(text);
  }
  
  estimateRequestTokens(request: RouterRequest): TokenEstimate {
    // Estimate input tokens
    let inputTokens = 0;
    
    // Count tokens in the main content
    inputTokens += this.estimateTokens(request.content.text);
    
    // Count tokens in the context
    if (request.context.systemPrompt) {
      inputTokens += this.estimateTokens(request.context.systemPrompt);
    }
    
    if (request.context.conversation) {
      for (const message of request.context.conversation) {
        inputTokens += this.estimateTokens(message.content);
      }
    }
    
    // Estimate output tokens based on task type and complexity
    const outputTokens = this.estimateOutputTokens(
      request.content.text,
      request.context
    );
    
    return {
      input: inputTokens,
      output: outputTokens,
      total: inputTokens + outputTokens,
    };
  }
  
  private estimateOutputTokens(text: string, context: any): number {
    // Base estimate on input length
    const inputTokens = this.estimateTokens(text);
    
    // Adjust based on task type if available in context
    const taskType = context.metadata?.taskType;
    let multiplier = 1.5; // Default multiplier
    
    if (taskType) {
      switch (taskType) {
        case 'code_generation':
          multiplier = 3.0; // Code generation typically produces more output
          break;
        case 'summarization':
          multiplier = 0.7; // Summarization typically produces less output
          break;
        case 'translation':
          multiplier = 1.2; // Translation typically produces similar output
          break;
        case 'creative_writing':
          multiplier = 2.5; // Creative writing typically produces more output
          break;
        default:
          multiplier = 1.5;
      }
    }
    
    return Math.ceil(inputTokens * multiplier);
  }
}
```

### 1.4 Budget Management

```typescript
class BudgetManager {
  private budgetConfig: BudgetConfig;
  private usageTracker: UsageTracker;
  
  constructor() {
    this.budgetConfig = this.loadBudgetConfig();
    this.usageTracker = new UsageTracker();
  }
  
  getAvailableBudget(taskType: TaskType): number {
    // Get total budget
    const totalBudget = this.budgetConfig.dailyBudget;
    
    // Get budget allocation for task type
    const allocation = this.budgetConfig.taskAllocation[taskType] || 
                      this.budgetConfig.taskAllocation.default;
    
    // Calculate allocated budget
    const allocatedBudget = totalBudget * allocation;
    
    // Get current usage
    const currentUsage = this.usageTracker.getDailyUsage(taskType);
    
    // Calculate available budget
    return Math.max(0, allocatedBudget - currentUsage);
  }
  
  updateBudgetUsage(taskType: TaskType, cost: number): void {
    this.usageTracker.trackUsage(taskType, cost);
  }
  
  getBudgetStatus(): BudgetStatus {
    const totalBudget = this.budgetConfig.dailyBudget;
    const totalUsage = this.usageTracker.getTotalDailyUsage();
    
    return {
      totalBudget,
      totalUsage,
      remainingBudget: totalBudget - totalUsage,
      percentUsed: (totalUsage / totalBudget) * 100,
      taskUsage: this.usageTracker.getTaskUsage(),
    };
  }
}
```

## 2. Latency Optimization

### 2.1 Overview

The Latency Optimization system routes requests to models and tools based on latency requirements, network conditions, and historical performance data. It implements strategies for reducing response time and handling time-sensitive requests.

### 2.2 Latency Optimization Algorithm

```typescript
class LatencyOptimizer {
  private modelLatencyMap: Map<string, ModelLatencyInfo>;
  private networkMonitor: NetworkMonitor;
  
  constructor() {
    this.modelLatencyMap = new Map();
    this.networkMonitor = new NetworkMonitor();
    this.initializeModelLatencyMap();
  }
  
  optimize(analysis: RequestAnalysis, config: SystemConfig): LatencyOptimizationParams {
    // Get current network conditions
    const networkConditions = this.networkMonitor.getCurrentConditions();
    
    // Determine latency tier based on requirements
    const latencyTier = this.determineLatencyTier(
      analysis.latencyRequirement,
      networkConditions,
      config.latencySensitivity
    );
    
    // Select execution strategy based on latency tier
    const executionStrategy = this.selectExecutionStrategy(latencyTier, networkConditions);
    
    // Determine timeout settings
    const timeout = this.calculateTimeout(latencyTier, analysis.complexity);
    
    return {
      latencyTier,
      executionStrategy,
      timeout,
      prioritizeLocalModels: latencyTier === 'real-time',
      networkConditions,
    };
  }
  
  private determineLatencyTier(
    requirement: LatencyRequirement,
    networkConditions: NetworkConditions,
    latencySensitivity: number
  ): LatencyTier {
    // Default tier based on requirement
    let tier: LatencyTier;
    switch (requirement) {
      case LatencyRequirement.REAL_TIME:
        tier = 'real-time';
        break;
      case LatencyRequirement.LOW_LATENCY:
        tier = 'low-latency';
        break;
      case LatencyRequirement.STANDARD:
        tier = 'standard';
        break;
      default:
        tier = 'standard';
    }
    
    // Adjust based on network conditions
    if (networkConditions.quality === 'poor') {
      // Downgrade tier if network is poor
      if (tier === 'real-time') {
        tier = 'low-latency';
      } else if (tier === 'low-latency') {
        tier = 'standard';
      }
    } else if (networkConditions.quality === 'excellent' && latencySensitivity > 0.7) {
      // Upgrade tier if network is excellent and latency sensitivity is high
      if (tier === 'standard') {
        tier = 'low-latency';
      }
    }
    
    return tier;
  }
  
  private selectExecutionStrategy(
    latencyTier: LatencyTier,
    networkConditions: NetworkConditions
  ): ExecutionStrategy {
    switch (latencyTier) {
      case 'real-time':
        return ExecutionStrategy.PARALLEL;
      case 'low-latency':
        return networkConditions.quality === 'poor' ? 
               ExecutionStrategy.SEQUENTIAL : 
               ExecutionStrategy.HYBRID;
      case 'standard':
        return ExecutionStrategy.SEQUENTIAL;
      default:
        return ExecutionStrategy.SEQUENTIAL;
    }
  }
  
  private calculateTimeout(
    latencyTier: LatencyTier,
    complexity: ComplexityLevel
  ): number {
    // Base timeout in milliseconds
    let baseTimeout: number;
    switch (latencyTier) {
      case 'real-time':
        baseTimeout = 1000; // 1 second
        break;
      case 'low-latency':
        baseTimeout = 3000; // 3 seconds
        break;
      case 'standard':
        baseTimeout = 10000; // 10 seconds
        break;
      default:
        baseTimeout = 5000; // 5 seconds
    }
    
    // Adjust based on complexity
    let complexityMultiplier: number;
    switch (complexity) {
      case ComplexityLevel.SIMPLE:
        complexityMultiplier = 1.0;
        break;
      case ComplexityLevel.MODERATE:
        complexityMultiplier = 1.5;
        break;
      case ComplexityLevel.COMPLEX:
        complexityMultiplier = 2.0;
        break;
      case ComplexityLevel.VERY_COMPLEX:
        complexityMultiplier = 3.0;
        break;
      default:
        complexityMultiplier = 1.5;
    }
    
    return baseTimeout * complexityMultiplier;
  }
}
```

### 2.3 Network Monitoring

```typescript
class NetworkMonitor {
  private latencyHistory: LatencyRecord[] = [];
  private lastCheck: Date = new Date();
  private currentConditions: NetworkConditions = {
    quality: 'good',
    latency: 100,
    jitter: 20,
    bandwidth: 1000,
    reliability: 0.95,
  };
  
  constructor() {
    this.startMonitoring();
  }
  
  getCurrentConditions(): NetworkConditions {
    // Check if we need to refresh
    const now = new Date();
    if (now.getTime() - this.lastCheck.getTime() > 60000) { // 1 minute
      this.refreshConditions();
      this.lastCheck = now;
    }
    
    return { ...this.currentConditions };
  }
  
  private async refreshConditions(): Promise<void> {
    try {
      // Measure latency to key endpoints
      const latencies = await this.measureLatencies();
      
      // Calculate average latency
      const avgLatency = latencies.reduce((sum, val) => sum + val, 0) / latencies.length;
      
      // Calculate jitter
      const jitter = this.calculateJitter(latencies);
      
      // Estimate bandwidth
      const bandwidth = await this.estimateBandwidth();
      
      // Calculate reliability
      const reliability = this.calculateReliability();
      
      // Determine overall quality
      const quality = this.determineQuality(avgLatency, jitter, bandwidth, reliability);
      
      // Update current conditions
      this.currentConditions = {
        quality,
        latency: avgLatency,
        jitter,
        bandwidth,
        reliability,
      };
      
      // Add to history
      this.addToHistory(avgLatency);
    } catch (error) {
      console.error('Error refreshing network conditions:', error);
      // Keep existing conditions on error
    }
  }
  
  private determineQuality(
    latency: number,
    jitter: number,
    bandwidth: number,
    reliability: number
  ): NetworkQuality {
    // Excellent conditions
    if (latency < 50 && jitter < 10 && bandwidth > 5000 && reliability > 0.99) {
      return 'excellent';
    }
    
    // Poor conditions
    if (latency > 300 || jitter > 100 || bandwidth < 500 || reliability < 0.8) {
      return 'poor';
    }
    
    // Good conditions
    if (latency < 150 && jitter < 50 && bandwidth > 1000 && reliability > 0.9) {
      return 'good';
    }
    
    // Fair conditions
    return 'fair';
  }
}
```
