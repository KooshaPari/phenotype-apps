# SPEC.md - Kwality (AI Codebase Validation Platform)

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    Kwality Validation Platform                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                   │
│  ┌─────────────────────────────────────────────────────────────┐│
│  │                    API / CLI Layer                           ││
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐       ││
│  │  │   CLI Tool   │  │   REST API   │  │   Webhooks   │       ││
│  │  │   (Cobra)    │  │   (Gin)      │  │   (Events)   │       ││
│  │  │              │  │              │  │              │       ││
│  │  │ • validate   │  │ • POST /api/ │  │ • GitHub     │       ││
│  │  │ • server     │  │   validate   │  │ • GitLab     │       ││
│  │  │ • health     │  │ • GET /health│  │ • Slack      │       ││
│  │  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘       ││
│  │         └─────────────────┴─────────────────┘                ││
│  │                           │                                  ││
│  └───────────────────────────┼───────────────────────────────────┘│
│                             │                                  │
│  ┌──────────────────────────┴───────────────────────────────────┐│
│  │              Orchestration Layer (Go)                        ││
│  │                                                              ││
│  │  ┌─────────────────────────────────────────────────────┐    ││
│  │  │            Validation Coordinator                   │    ││
│  │  │  • Request validation                             │    ││
│  │  │  • Engine selection                               │    ││
│  │  │  • Parallel execution                             │    ││
│  │  │  • Result aggregation                             │    ││
│  │  └──────────────────────┬────────────────────────────┘    ││
│  │                         │                                    ││
│  │  ┌──────────────────────┴────────────────────────────┐      ││
│  │  │              Task Queue Manager                   │      ││
│  │  │  • Redis/RabbitMQ task distribution             │      ││
│  │  │  • Worker scaling                                │      ││
│  │  │  • Retry logic                                   │      ││
│  │  │  • Priority queues                               │      ││
│  │  └─────────────────────────────────────────────────┘      ││
│  │                                                              ││
│  └──────────────────────────────────────────────────────────────┘│
│                                                                   │
│  ┌──────────────────────────────────────────────────────────────┐│
│  │              Validation Engines                                ││
│  │                                                                ││
│  │  ┌──────────────────┐  ┌──────────────────┐                    ││
│  │  │  Static Analysis │  │ Runtime Validator│                    ││
│  │  │     (Go)         │  │    (Rust)        │                    ││
│  │  │                  │  │                  │                    ││
│  │  │ • AST Parsing    │  │ • Container Exec │                    ││
│  │  │ • Multi-lang     │  │ • Performance    │                    ││
│  │  │   Linters        │  │ • Memory Analysis│                    ││
│  │  │ • Code Quality   │  │ • Fuzzing Engine │                    ││
│  │  │ • Dependencies   │  │                  │                    ││
│  │  └────────┬─────────┘  └────────┬─────────┘                    ││
│  │           │                     │                                ││
│  │  ┌────────┴─────────┐  ┌────────┴─────────┐                  ││
│  │  │ Security Scanner │  │ Integration Test │                  ││
│  │  │     (Go)         │  │     (Go)         │                  ││
│  │  │                  │  │                  │                  ││
│  │  │ • SAST Analysis  │  │ • API Validation │                  ││
│  │  │ • Vulnerability  │  │ • E2E Testing    │                  ││
│  │  │   Detection      │  │ • Contract Tests │                  ││
│  │  │ • Secrets Scan   │  │                  │                  ││
│  │  └────────┬─────────┘  └────────┬─────────┘                  ││
│  │           └─────────────────────┘                              ││
│  │                     │                                          ││
│  │  ┌──────────────────┴──────────────────┐                      ││
│  │  │      Engine Interface (Standard)    │                      ││
│  │  │  • Validate(input) → Result         │                      ││
│  │  │  • GetScore() → float64           │                      ││
│  │  │  • GetFindings() → []Finding       │                      ││
│  │  └───────────────────────────────────┘                      ││
│  └──────────────────────────────────────────────────────────────┘│
│                                                                   │
│  ┌──────────────────────────────────────────────────────────────┐│
│  │              Isolation & Safety Layer                          ││
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        ││
│  │  │   Docker     │  │   Resource   │  │   Security   │        ││
│  │  │   Containers │  │   Limits     │  │   Monitor    │        ││
│  │  │              │  │              │  │              │        ││
│  │  │ • Sandbox    │  │ • CPU quotas │  │ • Syscall    │        ││
│  │  │ • Network    │  │ • Memory cap │  │   audit      │        ││
│  │  │   isolation  │  │ • Disk limit │  │ • Behavior   │        ││
│  │  │ • Ephemeral  │  │ • Timeout    │  │   analysis   │        │
│  │  └──────────────┘  └──────────────┘  └──────────────┘        ││
│  └──────────────────────────────────────────────────────────────┘│
│                                                                   │
│  ┌──────────────────────────────────────────────────────────────┐│
│  │              Data Layer                                        ││
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐        ││
│  │  │  PostgreSQL    │  │    Redis     │  │    S3/MinIO  │        ││
│  │  │  (State)       │  │   (Cache)    │  │  (Artifacts) │        ││
│  │  │                │  │              │  │              │        ││
│  │  │ • Validation   │  │ • Queue      │  │ • Reports    │        ││
│  │  │   results      │  │ • Session    │  │ • Logs       │        ││
│  │  │ • Audit log    │  │ • Rate limit │  │ • Screenshots│        ││
│  │  └──────────────┘  └──────────────┘  └──────────────┘        ││
│  └──────────────────────────────────────────────────────────────┘│
│                                                                   │
└─────────────────────────────────────────────────────────────────┘
```

## Component Breakdown

### 1. CLI (`cmd/kwality-cli/main.go`)
- **Commands**:
  - `kwality validate <path>` - Validate codebase
  - `kwality server` - Start validation server
  - `kwality health` - System health check
  - `kwality security-scan` - Security analysis
  - `kwality compliance-check` - Compliance validation
- **Cobra**: Command framework with help generation
- **Output**: JSON, table, or silent modes

### 2. REST API (`internal/server/gin_server.go`)
- **Gin framework** with middleware
- **Endpoints**:
  - `POST /api/v1/validate/codebase` - Submit validation
  - `GET /api/v1/validate/{task-id}` - Get results
  - `GET /api/v1/tasks` - List tasks
  - `GET /health` - Health check
- **Middleware**:
  - Request ID tracking
  - Rate limiting
  - CORS
  - Authentication (optional)

### 3. Orchestration Layer (`internal/orchestrator/`)
**Validation Coordinator** (`orchestrator.go`)
- Parses validation requests
- Selects appropriate engines
- Manages parallel execution
- Aggregates results
- Quality gate evaluation

**Task Queue Manager**
- Redis/RabbitMQ integration
- Worker pool management
- Retry with exponential backoff
- Dead letter queue for failures

### 4. Validation Engines

#### Static Analysis Engine (`internal/engines/static_analysis.go`)
- **Language Support**:
  - Go: golangci-lint, go vet, staticcheck, gosec
  - Rust: clippy, cargo audit
  - JavaScript: ESLint, TSLint
  - Python: pylint, bandit
  - Java: SpotBugs, PMD
- **Metrics**: Complexity, maintainability, coverage
- **Output**: SARIF-compatible findings

#### Runtime Validator (`engines/runtime-validator/` - Rust)
- **Container Execution**: Docker-based isolation
- **Performance Profiler**: CPU, memory, I/O
- **Fuzzing Engine**: Property-based testing
- **Memory Analysis**: Leak detection
- **Safety**: Network isolation, resource limits

#### Security Scanner (`internal/engines/security.go`)
- **SAST**: Semgrep, CodeQL patterns
- **Vulnerability DB**: NVD, GHSA integration
- **Secrets Detection**: Gitleaks, truffleHog patterns
- **Dependency Scanning**: CVE checking
- **Compliance**: SOC2, ISO27001, GDPR rules

#### Integration Tester (`internal/engines/integration.go`)
- **API Validation**: OpenAPI spec compliance
- **Contract Testing**: Consumer-driven contracts
- **E2E Testing**: Full workflow validation
- **Service Virtualization**: Mock external deps

### 5. Isolation Layer
**Container Management**
- Docker API integration
- Non-root execution
- Read-only root filesystem
- Cap drop for security

**Resource Limits**
- CPU: Configurable cores (default: 1)
- Memory: Configurable MB (default: 512)
- Disk: Configurable GB (default: 10)
- Network: Isolated by default

**Security Monitoring**
- Syscall auditing with seccomp
- File access logging
- Network activity monitoring
- Behavior analysis

### 6. Data Layer
**PostgreSQL** (`internal/database/`)
- Validation task storage
- Result persistence
- Audit logging
- Migration support (GORM)

**Redis**
- Task queue
- Caching for repeated validations
- Session storage
- Rate limiting counters

**Object Storage** (S3/MinIO)
- Validation reports
- Build artifacts
- Screenshots/videos
- Log archives

## Data Models

### ValidationTask
```go
type ValidationTask struct {
    ID          uuid.UUID
    Name        string                    // Task name
    Status      TaskStatus                // pending, running, completed, failed
    Source      CodebaseSource            // Git, local path, archive
    
    // Configuration
    Config      ValidationConfig          // Enabled engines, thresholds
    
    // Results
    Results     map[string]EngineResult   // Per-engine results
    OverallScore float64                  // 0-100
    QualityGate bool                      // Pass/fail
    
    // Timing
    CreatedAt   time.Time
    StartedAt   *time.Time
    CompletedAt *time.Time
    Duration    *time.Duration
    
    // Metadata
    CreatedBy   string
    Tags        []string
}

type TaskStatus string
const (
    TaskPending    TaskStatus = "pending"
    TaskRunning    TaskStatus = "running"
    TaskCompleted  TaskStatus = "completed"
    TaskFailed     TaskStatus = "failed"
    TaskCancelled  TaskStatus = "cancelled"
)

type CodebaseSource struct {
    Type       SourceType     // git, local, archive
    Repository *GitRepository // For git type
    LocalPath  string         // For local type
    ArchiveURL string         // For archive type
}
```

### ValidationConfig
```go
type ValidationConfig struct {
    EnabledEngines []string          // static, runtime, security, integration
    Timeout        time.Duration     // Max validation time
    
    // Per-engine config
    StaticAnalysis StaticAnalysisConfig
    Runtime        RuntimeConfig
    Security       SecurityConfig
    Integration    IntegrationConfig
    
    // Quality gate
    QualityGate QualityGateConfig
}

type QualityGateConfig struct {
    MinOverallScore   float64          // 0-100
    MinSecurityScore  float64          // 0-100
    MaxCriticalIssues int              // 0 = unlimited
    BlockOnSecrets    bool             // Fail if secrets found
}

type RuntimeConfig struct {
    ContainerImage    string
    MemoryLimitMB     int
    CPULimitCores     float64
    TimeoutSeconds    int
    NetworkIsolation  bool
    EnableFuzzing     bool
}
```

### EngineResult
```go
type EngineResult struct {
    Engine      string          // Engine name
    Status      EngineStatus    // success, partial, failed
    Score       float64         // 0-100
    
    Findings    []Finding       // Issues found
    Metrics     EngineMetrics   // Detailed metrics
    
    StartedAt   time.Time
    CompletedAt time.Time
    Duration    time.Duration
    
    RawOutput   string          // Original engine output
}

type Finding struct {
    RuleID      string          // Rule identifier
    Severity    Severity        // critical, high, medium, low, info
    Title       string
    Description string
    Location    *Location       // File, line, column
    Remediation string          // How to fix
    References  []string        // URLs
}

type Severity string
const (
    SeverityCritical Severity = "critical"
    SeverityHigh     Severity = "high"
    SeverityMedium   Severity = "medium"
    SeverityLow      Severity = "low"
    SeverityInfo     Severity = "info"
)
```

## Performance Specifications

### Validation Throughput
- **Small Projects** (<1000 files): <5 minutes
- **Medium Projects** (1K-10K files): <15 minutes
- **Large Projects** (10K+ files): <30 minutes
- **Concurrent Tasks**: 50+ per orchestrator instance

### Engine Performance
- **Static Analysis**: 100 files/second per engine
- **Runtime Validation**: <5 minutes with fuzzing
- **Security Scan**: <2 minutes for typical project
- **Integration Tests**: Depends on test suite

### API Performance
- **Health Check**: <100ms
- **Task Submission**: <500ms
- **Result Retrieval**: <100ms
- **List Tasks**: <200ms for 1000 tasks

### Resource Usage
- **Orchestrator**: 512MB RAM, 1 CPU core
- **Per Validation Worker**: 512MB-2GB RAM
- **Database**: Scales with project size
- **Storage**: 1GB per 1000 validations (average)

## Integration Points

### GitHub Actions
```yaml
- name: Kwality Validation
  uses: kwality/validate-action@v1
  with:
    engines: static,security
    fail-on-threshold: 80
```

### GitLab CI
```yaml
kwality:
  image: kwality/cli:latest
  script:
    - kwality validate . --engines static,runtime
```

### Docker Compose
```yaml
services:
  kwality:
    image: kwality/platform:latest
    environment:
      - KWALITY_DB_HOST=postgres
      - KWALITY_REDIS_HOST=redis
    ports:
      - "8080:8080"
```

### Kubernetes
- Helm chart for deployment
- Horizontal Pod Autoscaler
- PersistentVolume for artifacts
- NetworkPolicy for isolation

## Security Model

### Isolation
- **Containers**: All code runs in isolated containers
- **Non-root**: Runtime execution as unprivileged user
- **Read-only**: Root filesystem mounted read-only
- **No network**: External access disabled by default

### Secrets
- **Detection**: Automated scanning with 100+ patterns
- **Prevention**: Pre-commit hooks available
- **Rotation**: Automatic suggestion on detection
- **Vault Integration**: AWS SM, Azure Key Vault, HashiCorp Vault

### Audit
- **Immutable Logs**: Append-only with integrity checks
- **Change Tracking**: Who, what, when for all operations
- **Compliance**: SOC2, ISO27001, GDPR ready
- **Retention**: Configurable (default: 90 days)

## Extensibility

### Custom Engines
```go
type ValidationEngine interface {
    Name() string
    SupportedLanguages() []string
    Validate(ctx context.Context, input *EngineInput) (*EngineResult, error)
    GetDefaultConfig() map[string]interface{}
}
```

### Custom Rules
- Semgrep rule integration
- Custom pattern definitions
- Rule severity configuration
- Suppression mechanisms

### Webhooks
- GitHub PR comments
- Slack notifications
- Custom HTTP endpoints
- Event-driven workflows
