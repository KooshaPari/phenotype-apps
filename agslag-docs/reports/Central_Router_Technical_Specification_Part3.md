# Central Router Technical Specification - Part 3: Model Selection and Privacy Controls

## 1. Model Selection Algorithm

### 1.1 Overview

The Model Selection Algorithm is responsible for choosing the optimal AI model for a given request based on multiple factors including task type, complexity, cost, latency, quality requirements, and privacy considerations. It uses a scoring system to rank available models and select the best match.

### 1.2 Model Selection Implementation

```typescript
class ModelSelector {
  private modelRegistry: ModelRegistry;
  private modelScorer: ModelScorer;
  
  constructor() {
    this.modelRegistry = new ModelRegistry();
    this.modelScorer = new ModelScorer();
  }
  
  async selectModel(
    analysis: RequestAnalysis,
    optimizationParams: OptimizationParams
  ): Promise<SelectedModel> {
    // Get candidate models based on task type and complexity
    const candidateModels = this.getCandidateModels(
      analysis.taskType,
      analysis.complexity,
      optimizationParams.costParams.modelTier
    );
    
    // Score models based on multiple factors
    const scoredModels = await this.scoreModels(
      candidateModels,
      analysis,
      optimizationParams
    );
    
    // Select the highest-scoring model
    const selectedModel = this.selectHighestScoringModel(scoredModels);
    
    // Determine model parameters
    const modelParams = this.determineModelParams(
      selectedModel,
      analysis,
      optimizationParams
    );
    
    return {
      id: selectedModel.id,
      name: selectedModel.name,
      provider: selectedModel.provider,
      params: modelParams,
    };
  }
  
  private getCandidateModels(
    taskType: TaskType,
    complexity: ComplexityLevel,
    modelTier: ModelTier
  ): ModelInfo[] {
    // Get all available models
    const allModels = this.modelRegistry.getAllModels();
    
    // Filter models by task type capability
    const taskCompatibleModels = allModels.filter(model => 
      this.isModelCompatibleWithTask(model, taskType)
    );
    
    // Filter models by complexity capability
    const complexityCompatibleModels = taskCompatibleModels.filter(model => 
      this.isModelCompatibleWithComplexity(model, complexity)
    );
    
    // Filter models by tier
    return complexityCompatibleModels.filter(model => 
      model.tier === modelTier
    );
  }
  
  private async scoreModels(
    models: ModelInfo[],
    analysis: RequestAnalysis,
    optimizationParams: OptimizationParams
  ): Promise<ScoredModel[]> {
    return Promise.all(
      models.map(async model => {
        // Calculate cost score
        const costScore = this.modelScorer.calculateCostScore(
          model,
          analysis.tokenEstimate,
          optimizationParams.costParams.costLimit
        );
        
        // Calculate latency score
        const latencyScore = this.modelScorer.calculateLatencyScore(
          model,
          optimizationParams.latencyParams.latencyTier
        );
        
        // Calculate quality score
        const qualityScore = this.modelScorer.calculateQualityScore(
          model,
          analysis.qualityRequirement
        );
        
        // Calculate privacy score
        const privacyScore = this.modelScorer.calculatePrivacyScore(
          model,
          analysis.privacyLevel
        );
        
        // Calculate historical performance score
        const historyScore = await this.modelScorer.calculateHistoricalScore(
          model,
          analysis.taskType
        );
        
        // Calculate combined score
        const combinedScore = this.modelScorer.calculateCombinedScore(
          costScore,
          latencyScore,
          qualityScore,
          privacyScore,
          historyScore,
          optimizationParams
        );
        
        return {
          model,
          scores: {
            cost: costScore,
            latency: latencyScore,
            quality: qualityScore,
            privacy: privacyScore,
            history: historyScore,
            combined: combinedScore,
          },
        };
      })
    );
  }
  
  private selectHighestScoringModel(scoredModels: ScoredModel[]): ModelInfo {
    // Sort models by combined score (descending)
    const sortedModels = [...scoredModels].sort(
      (a, b) => b.scores.combined - a.scores.combined
    );
    
    // Return the highest-scoring model
    return sortedModels[0].model;
  }
  
  private determineModelParams(
    model: ModelInfo,
    analysis: RequestAnalysis,
    optimizationParams: OptimizationParams
  ): ModelParameters {
    // Base parameters
    const params: ModelParameters = {
      temperature: this.determineTemperature(analysis.taskType),
      maxTokens: optimizationParams.costParams.tokenBudget.output,
      topP: 1.0,
      presencePenalty: 0.0,
      frequencyPenalty: 0.0,
    };
    
    // Adjust parameters based on task type
    switch (analysis.taskType) {
      case TaskType.CREATIVE_WRITING:
        params.temperature = 0.8;
        params.topP = 0.9;
        params.frequencyPenalty = 0.5;
        break;
      case TaskType.CODE_GENERATION:
        params.temperature = 0.3;
        params.topP = 0.95;
        params.frequencyPenalty = 0.2;
        break;
      case TaskType.SUMMARIZATION:
        params.temperature = 0.2;
        params.topP = 1.0;
        break;
      case TaskType.REASONING:
        params.temperature = 0.1;
        params.topP = 0.9;
        break;
    }
    
    // Adjust parameters based on model-specific recommendations
    if (model.recommendedParams) {
      return { ...params, ...model.recommendedParams };
    }
    
    return params;
  }
  
  private determineTemperature(taskType: TaskType): number {
    // Default temperature
    let temperature = 0.5;
    
    // Adjust based on task type
    switch (taskType) {
      case TaskType.CREATIVE_WRITING:
        temperature = 0.8;
        break;
      case TaskType.CODE_GENERATION:
        temperature = 0.3;
        break;
      case TaskType.SUMMARIZATION:
        temperature = 0.2;
        break;
      case TaskType.REASONING:
        temperature = 0.1;
        break;
      case TaskType.QUESTION_ANSWERING:
        temperature = 0.3;
        break;
      case TaskType.TRANSLATION:
        temperature = 0.2;
        break;
      case TaskType.PLANNING:
        temperature = 0.4;
        break;
      default:
        temperature = 0.5;
    }
    
    return temperature;
  }
}
```

### 1.3 Model Scoring Algorithm

```typescript
class ModelScorer {
  private performanceTracker: PerformanceTracker;
  
  constructor() {
    this.performanceTracker = new PerformanceTracker();
  }
  
  calculateCostScore(
    model: ModelInfo,
    tokenEstimate: TokenEstimate,
    costLimit: number
  ): number {
    // Calculate estimated cost
    const inputCost = model.pricing.inputPrice * tokenEstimate.input / 1000;
    const outputCost = model.pricing.outputPrice * tokenEstimate.output / 1000;
    const totalCost = inputCost + outputCost;
    
    // Calculate cost score (higher is better)
    // Score is 1.0 if cost is 0, and approaches 0 as cost approaches costLimit
    if (totalCost <= 0) {
      return 1.0;
    }
    
    return Math.max(0, 1.0 - (totalCost / costLimit));
  }
  
  calculateLatencyScore(
    model: ModelInfo,
    latencyTier: LatencyTier
  ): number {
    // Get model's average latency (ms per token)
    const avgLatency = model.performance.avgLatencyPerToken;
    
    // Define latency thresholds based on tier
    let excellentThreshold: number;
    let goodThreshold: number;
    let fairThreshold: number;
    
    switch (latencyTier) {
      case 'real-time':
        excellentThreshold = 5;
        goodThreshold = 10;
        fairThreshold = 20;
        break;
      case 'low-latency':
        excellentThreshold = 10;
        goodThreshold = 20;
        fairThreshold = 40;
        break;
      case 'standard':
        excellentThreshold = 20;
        goodThreshold = 40;
        fairThreshold = 80;
        break;
      default:
        excellentThreshold = 15;
        goodThreshold = 30;
        fairThreshold = 60;
    }
    
    // Calculate latency score (higher is better)
    if (avgLatency <= excellentThreshold) {
      return 1.0;
    } else if (avgLatency <= goodThreshold) {
      return 0.75;
    } else if (avgLatency <= fairThreshold) {
      return 0.5;
    } else {
      return Math.max(0, 0.5 - ((avgLatency - fairThreshold) / fairThreshold) * 0.5);
    }
  }
  
  calculateQualityScore(
    model: ModelInfo,
    qualityRequirement: QualityRequirement
  ): number {
    // Get model's quality rating (0-10 scale)
    const qualityRating = model.performance.qualityRating;
    
    // Define quality thresholds based on requirement
    let threshold: number;
    switch (qualityRequirement) {
      case QualityRequirement.PREMIUM:
        threshold = 9.0;
        break;
      case QualityRequirement.HIGH:
        threshold = 7.5;
        break;
      case QualityRequirement.STANDARD:
        threshold = 6.0;
        break;
      default:
        threshold = 6.0;
    }
    
    // Calculate quality score (higher is better)
    if (qualityRating >= threshold) {
      return 1.0;
    } else {
      return Math.max(0, qualityRating / threshold);
    }
  }
  
  calculatePrivacyScore(
    model: ModelInfo,
    privacyLevel: PrivacyLevel
  ): number {
    // Check if model meets privacy requirements
    switch (privacyLevel) {
      case PrivacyLevel.HIGHLY_SENSITIVE:
        return model.privacy.localOnly ? 1.0 : 0.0;
      case PrivacyLevel.SENSITIVE:
        return model.privacy.localOnly ? 1.0 : 
               (model.privacy.encryptedTransit && model.privacy.encryptedStorage ? 0.7 : 0.0);
      case PrivacyLevel.INTERNAL:
        return model.privacy.localOnly ? 1.0 : 
               (model.privacy.encryptedTransit ? 0.8 : 0.3);
      case PrivacyLevel.PUBLIC:
        return 1.0; // All models meet public privacy requirements
      default:
        return 0.5;
    }
  }
  
  async calculateHistoricalScore(
    model: ModelInfo,
    taskType: TaskType
  ): Promise<number> {
    // Get historical performance data
    const performanceData = await this.performanceTracker.getModelPerformance(
      model.id,
      taskType
    );
    
    if (!performanceData || performanceData.totalRequests === 0) {
      return 0.5; // Neutral score for models with no history
    }
    
    // Calculate success rate
    const successRate = performanceData.successfulRequests / performanceData.totalRequests;
    
    // Calculate average quality rating
    const avgQuality = performanceData.qualityRatings.length > 0 ?
                      performanceData.qualityRatings.reduce((sum, val) => sum + val, 0) / 
                      performanceData.qualityRatings.length :
                      0.5;
    
    // Combine success rate and quality for final score
    return (successRate * 0.6) + (avgQuality * 0.4);
  }
  
  calculateCombinedScore(
    costScore: number,
    latencyScore: number,
    qualityScore: number,
    privacyScore: number,
    historyScore: number,
    optimizationParams: OptimizationParams
  ): number {
    // Get weights from optimization parameters
    const weights = {
      cost: optimizationParams.weights?.cost || 0.2,
      latency: optimizationParams.weights?.latency || 0.2,
      quality: optimizationParams.weights?.quality || 0.2,
      privacy: optimizationParams.weights?.privacy || 0.2,
      history: optimizationParams.weights?.history || 0.2,
    };
    
    // Ensure weights sum to 1.0
    const totalWeight = Object.values(weights).reduce((sum, val) => sum + val, 0);
    const normalizedWeights = Object.entries(weights).reduce(
      (obj, [key, val]) => ({ ...obj, [key]: val / totalWeight }),
      {} as Record<string, number>
    );
    
    // Calculate weighted score
    return (
      costScore * normalizedWeights.cost +
      latencyScore * normalizedWeights.latency +
      qualityScore * normalizedWeights.quality +
      privacyScore * normalizedWeights.privacy +
      historyScore * normalizedWeights.history
    );
  }
}
```

## 2. Privacy Controls

### 2.1 Overview

The Privacy Controls system ensures that data is processed according to its sensitivity level, routing sensitive information to appropriate models and implementing necessary safeguards. It includes data classification, privacy-aware routing, and audit logging.

### 2.2 Data Classification Algorithm

```typescript
class PrivacyAnalyzer {
  private sensitivePatterns: RegExp[];
  private internalPatterns: RegExp[];
  private classificationModel: ClassificationModel;
  
  constructor() {
    this.sensitivePatterns = this.initializeSensitivePatterns();
    this.internalPatterns = this.initializeInternalPatterns();
    this.classificationModel = this.initializeClassificationModel();
  }
  
  async analyze(request: RouterRequest): Promise<PrivacyLevel> {
    // Check for explicit privacy level in request
    if (request.constraints?.privacyLevel) {
      return request.constraints.privacyLevel;
    }
    
    // Check for sensitive patterns
    if (this.containsSensitivePatterns(request.content.text)) {
      return PrivacyLevel.SENSITIVE;
    }
    
    // Check for internal patterns
    if (this.containsInternalPatterns(request.content.text)) {
      return PrivacyLevel.INTERNAL;
    }
    
    // Use classification model for more nuanced analysis
    const classificationResult = await this.classificationModel.classify(request.content.text);
    
    // Determine privacy level based on classification
    if (classificationResult.sensitivityScore > 0.8) {
      return PrivacyLevel.HIGHLY_SENSITIVE;
    } else if (classificationResult.sensitivityScore > 0.5) {
      return PrivacyLevel.SENSITIVE;
    } else if (classificationResult.sensitivityScore > 0.2) {
      return PrivacyLevel.INTERNAL;
    } else {
      return PrivacyLevel.PUBLIC;
    }
  }
  
  private containsSensitivePatterns(text: string): boolean {
    return this.sensitivePatterns.some(pattern => pattern.test(text));
  }
  
  private containsInternalPatterns(text: string): boolean {
    return this.internalPatterns.some(pattern => pattern.test(text));
  }
  
  private initializeSensitivePatterns(): RegExp[] {
    return [
      // PII patterns
      /\b\d{3}[-.\s]?\d{2}[-.\s]?\d{4}\b/, // SSN
      /\b\d{4}[-.\s]?\d{4}[-.\s]?\d{4}[-.\s]?\d{4}\b/, // Credit card
      /\b(?:\d{1,3}\.){3}\d{1,3}\b/, // IP address
      
      // Financial patterns
      /\baccount\s+number\s*[:=]?\s*\d+/i,
      /\bbank\s+account\b/i,
      
      // Healthcare patterns
      /\bpatient\s+id\b/i,
      /\bmedical\s+record\b/i,
      /\bdiagnosis\b/i,
      
      // Authentication patterns
      /\bpassword\s*[:=]?\s*\w+/i,
      /\bapi\s+key\s*[:=]?\s*\w+/i,
      /\btoken\s*[:=]?\s*\w+/i,
      
      // Other sensitive patterns
      /\bconfidential\b/i,
      /\bsecret\b/i,
      /\bprivate\b/i,
    ];
  }
  
  private initializeInternalPatterns(): RegExp[] {
    return [
      // Internal document patterns
      /\binternal\s+use\s+only\b/i,
      /\bdo\s+not\s+distribute\b/i,
      
      // Project patterns
      /\bproject\s+plan\b/i,
      /\bstrategy\b/i,
      /\broadmap\b/i,
      
      // Business patterns
      /\brevenue\b/i,
      /\bforecast\b/i,
      /\bbudget\b/i,
      
      // HR patterns
      /\bemployee\b/i,
      /\bperformance\s+review\b/i,
      /\bsalary\b/i,
      
      // Other internal patterns
      /\bmeeting\s+notes\b/i,
      /\bdraft\b/i,
    ];
  }
}
```

### 2.3 Privacy-Aware Routing

```typescript
class PrivacyRouter {
  private modelRegistry: ModelRegistry;
  
  constructor() {
    this.modelRegistry = new ModelRegistry();
  }
  
  filterModelsByPrivacy(
    models: ModelInfo[],
    privacyLevel: PrivacyLevel
  ): ModelInfo[] {
    switch (privacyLevel) {
      case PrivacyLevel.HIGHLY_SENSITIVE:
        // Only allow local models for highly sensitive data
        return models.filter(model => model.privacy.localOnly);
        
      case PrivacyLevel.SENSITIVE:
        // Allow local models or those with strong privacy guarantees
        return models.filter(model => 
          model.privacy.localOnly || 
          (model.privacy.encryptedTransit && 
           model.privacy.encryptedStorage && 
           model.privacy.dataDeletion)
        );
        
      case PrivacyLevel.INTERNAL:
        // Allow models with basic privacy guarantees
        return models.filter(model => 
          model.privacy.localOnly || 
          model.privacy.encryptedTransit
        );
        
      case PrivacyLevel.PUBLIC:
        // Allow all models for public data
        return models;
        
      default:
        // Default to internal privacy level
        return models.filter(model => 
          model.privacy.localOnly || 
          model.privacy.encryptedTransit
        );
    }
  }
  
  applyPrivacyTransformations(
    request: RouterRequest,
    privacyLevel: PrivacyLevel
  ): RouterRequest {
    // Create a copy of the request
    const transformedRequest = { ...request };
    
    // Apply transformations based on privacy level
    switch (privacyLevel) {
      case PrivacyLevel.HIGHLY_SENSITIVE:
      case PrivacyLevel.SENSITIVE:
        // Redact potentially sensitive information
        transformedRequest.content.text = this.redactSensitiveInformation(
          transformedRequest.content.text
        );
        break;
        
      case PrivacyLevel.INTERNAL:
        // Minimal transformations for internal data
        break;
        
      case PrivacyLevel.PUBLIC:
        // No transformations needed for public data
        break;
    }
    
    return transformedRequest;
  }
  
  private redactSensitiveInformation(text: string): string {
    // Redact common PII patterns
    let redactedText = text;
    
    // Redact SSNs
    redactedText = redactedText.replace(
      /\b\d{3}[-.\s]?\d{2}[-.\s]?\d{4}\b/g,
      '[REDACTED-SSN]'
    );
    
    // Redact credit card numbers
    redactedText = redactedText.replace(
      /\b\d{4}[-.\s]?\d{4}[-.\s]?\d{4}[-.\s]?\d{4}\b/g,
      '[REDACTED-CC]'
    );
    
    // Redact IP addresses
    redactedText = redactedText.replace(
      /\b(?:\d{1,3}\.){3}\d{1,3}\b/g,
      '[REDACTED-IP]'
    );
    
    // Redact email addresses
    redactedText = redactedText.replace(
      /\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b/g,
      '[REDACTED-EMAIL]'
    );
    
    // Redact phone numbers
    redactedText = redactedText.replace(
      /\b\d{3}[-.\s]?\d{3}[-.\s]?\d{4}\b/g,
      '[REDACTED-PHONE]'
    );
    
    return redactedText;
  }
}
```

### 2.4 Audit Logging

```typescript
class PrivacyAuditLogger {
  private logStorage: AuditLogStorage;
  
  constructor() {
    this.logStorage = new AuditLogStorage();
  }
  
  async logRequest(
    request: RouterRequest,
    analysis: RequestAnalysis,
    routingPlan: RoutingPlan,
    response: RouterResponse
  ): Promise<void> {
    // Create audit log entry
    const logEntry: PrivacyAuditLogEntry = {
      timestamp: new Date(),
      requestId: request.id,
      userId: request.userId,
      privacyLevel: analysis.privacyLevel,
      modelId: routingPlan.primaryModel.id,
      modelProvider: routingPlan.primaryModel.provider,
      isLocalModel: this.isLocalModel(routingPlan.primaryModel),
      dataTransformationsApplied: this.getAppliedTransformations(analysis.privacyLevel),
      tokenCount: response.metadata.tokenUsage,
    };
    
    // Store audit log
    await this.logStorage.storeLog(logEntry);
  }
  
  private isLocalModel(model: SelectedModel): boolean {
    // Check if model is local based on provider
    return model.provider === 'local' || 
           model.provider === 'ollama' || 
           model.provider.startsWith('local-');
  }
  
  private getAppliedTransformations(privacyLevel: PrivacyLevel): string[] {
    // Return list of transformations applied based on privacy level
    switch (privacyLevel) {
      case PrivacyLevel.HIGHLY_SENSITIVE:
        return ['local_processing', 'pii_redaction', 'encryption'];
      case PrivacyLevel.SENSITIVE:
        return ['pii_redaction', 'encryption'];
      case PrivacyLevel.INTERNAL:
        return ['basic_encryption'];
      case PrivacyLevel.PUBLIC:
        return [];
      default:
        return [];
    }
  }
  
  async getAuditLogs(
    filters: AuditLogFilters
  ): Promise<PrivacyAuditLogEntry[]> {
    return this.logStorage.getLogs(filters);
  }
  
  async generateAuditReport(
    startDate: Date,
    endDate: Date,
    filters?: AuditLogFilters
  ): Promise<AuditReport> {
    // Get logs for the specified period
    const logs = await this.logStorage.getLogs({
      startDate,
      endDate,
      ...filters,
    });
    
    // Generate report
    return {
      period: {
        start: startDate,
        end: endDate,
      },
      totalRequests: logs.length,
      byPrivacyLevel: this.countByPrivacyLevel(logs),
      byModelProvider: this.countByModelProvider(logs),
      localVsCloud: this.countLocalVsCloud(logs),
      transformationsApplied: this.countTransformations(logs),
    };
  }
  
  private countByPrivacyLevel(
    logs: PrivacyAuditLogEntry[]
  ): Record<PrivacyLevel, number> {
    return logs.reduce(
      (counts, log) => {
        counts[log.privacyLevel] = (counts[log.privacyLevel] || 0) + 1;
        return counts;
      },
      {} as Record<PrivacyLevel, number>
    );
  }
  
  private countByModelProvider(
    logs: PrivacyAuditLogEntry[]
  ): Record<string, number> {
    return logs.reduce(
      (counts, log) => {
        counts[log.modelProvider] = (counts[log.modelProvider] || 0) + 1;
        return counts;
      },
      {} as Record<string, number>
    );
  }
  
  private countLocalVsCloud(
    logs: PrivacyAuditLogEntry[]
  ): { local: number; cloud: number } {
    return logs.reduce(
      (counts, log) => {
        if (log.isLocalModel) {
          counts.local += 1;
        } else {
          counts.cloud += 1;
        }
        return counts;
      },
      { local: 0, cloud: 0 }
    );
  }
  
  private countTransformations(
    logs: PrivacyAuditLogEntry[]
  ): Record<string, number> {
    return logs.reduce(
      (counts, log) => {
        for (const transformation of log.dataTransformationsApplied) {
          counts[transformation] = (counts[transformation] || 0) + 1;
        }
        return counts;
      },
      {} as Record<string, number>
    );
  }
}
```
