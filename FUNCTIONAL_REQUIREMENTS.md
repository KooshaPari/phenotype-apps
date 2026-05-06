# Functional Requirements

| FR-ID | Title | Status | Tests |
|-------|-------|--------|-------|
| FR-001 | Codebase Ingestion and Language Detection | Implemented | 3 |
| FR-002 | Multi-Language Static Analysis | Implemented | 5 |
| FR-003 | Containerized Runtime Validation | Implemented | 4 |
| FR-004 | Security Vulnerability Scanning | Implemented | 6 |
| FR-005 | Integration Testing Service | Implemented | 3 |
| FR-006 | Quality Score Calculation | Implemented | 2 |
| FR-007 | REST API Endpoints | Implemented | 8 |
| FR-008 | CI/CD Webhook Integration | Implemented | 4 |
| FR-009 | Results Export (JSON/SARIF/JUnit) | Implemented | 3 |
| FR-010 | Container Isolation and Resource Limits | Implemented | 2 |
| FR-011 | JWT Authentication and RBAC | Draft | 0 |
| FR-012 | Multi-Tenant Architecture | Draft | 0 |

---

## User Stories

### US-001: Developer Submits Codebase for Validation
**As a** developer using CI/CD  
**I want to** submit my codebase for automated validation  
**So that** I can identify issues before merging to main

**Acceptance Criteria:**
- [ ] API accepts codebase via git URL or archive upload
- [ ] Language detection identifies Go, Rust, JavaScript, Python
- [ ] Validation results returned within 5 minutes for <100 files
- [ ] Webhook notification sent on completion

### US-002: Security Engineer Reviews Vulnerability Report
**As a** security engineer  
**I want to** view detailed vulnerability findings  
**So that** I can prioritize remediation efforts

**Acceptance Criteria:**
- [ ] Vulnerabilities grouped by severity (Critical/High/Medium/Low)
- [ ] CVE/GHSA references linked for each finding
- [ ] False positive marking capability
- [ ] Export to SARIF format for SIEM integration

### US-003: Project Manager Tracks Quality Trends
**As a** project manager  
**I want to** view quality score trends over time  
**So that** I can assess team velocity and code health

**Acceptance Criteria:**
- [ ] Dashboard shows 30/60/90 day quality trends
- [ ] Per-dimension breakdown (Correctness, Security, Performance, Maintainability, Reliability)
- [ ] Comparison against baseline or target scores
- [ ] Drill-down to individual validation runs

### US-004: DevOps Configures CI/CD Integration
**As a** DevOps engineer  
**I want to** integrate Kwality with GitHub Actions  
**So that** validation runs automatically on pull requests

**Acceptance Criteria:**
- [ ] GitHub Action workflow template provided
- [ ] Branch protection rules can require minimum quality gate
- [ ] Validation status appears as PR check
- [ ] Detailed results available via PR comment

### US-005: Admin Manages Team Access
**As a** platform administrator  
**I want to** manage user roles and permissions  
**So that** teams have appropriate access levels

**Acceptance Criteria:**
- [ ] Admin role can create/update/delete users
- [ ] Three roles: Admin, Developer, ReadOnly
- [ ] API key management for CI/CD integrations
- [ ] Audit log of all administrative actions

---

## System Requirements

### SR-001: Performance Requirements
| Metric | Target | Maximum |
|--------|--------|---------|
| API response time (validation start) | < 500ms | 2s |
| Static analysis throughput | 1000 files/min | 500 files/min floor |
| Concurrent validations | 10 | 50 hard limit |
| Result retrieval | < 200ms | 1s |

### SR-002: Scalability Requirements
| Component | Soft Limit | Hard Limit |
|-----------|-----------|------------|
| Codebase size | 10,000 files | 50,000 files |
| Concurrent users | 50 | 200 |
| Storage per validation | 100MB | 500MB |
| WebSocket connections | 100 | 500 |

### SR-003: Security Requirements
- All API communication over HTTPS (TLS 1.3)
- Secrets encrypted at rest (AES-256)
- Container execution with unprivileged user
- Network isolation in validation containers
- Rate limiting: 100 requests/minute per API key
- Audit logging for all mutations

### SR-004: Availability Requirements
- 99.5% uptime SLA (excluding planned maintenance)
- Graceful degradation when external scanners unavailable
- Validation queue persistence across restarts
- Health check endpoint for load balancer

---

## Functional Requirement Details

### FR-001: Codebase Ingestion and Language Detection

**Description:** Accept codebases from various sources and automatically detect programming languages.

**Input Sources:**
- Git repository URL (public/private with credentials)
- Tar/Gzip archive upload
- Direct file upload (zip)

**Supported Languages:**
| Language | Detection Method | Static Analysis |
|----------|-----------------|-----------------|
| Go | File extension + shebang | golangci-lint |
| Rust | Cargo.toml | cargo clippy + fmt |
| JavaScript | .js/.jsx/.mjs | eslint |
| TypeScript | .ts/.tsx | eslint + tsc |
| Python | .py + pyproject.toml | ruff, mypy |

**Edge Cases:**
- Monorepo with multiple languages: validate each language separately
- No detectable language: return error with suggestion
- Private repo without credentials: prompt for authentication

### FR-002: Multi-Language Static Analysis

**Description:** Execute language-specific linters and static analyzers to identify code quality issues.

**Analysis Categories:**
1. **Syntax & Semantics**
   - AST parsing validation
   - Type checking (where applicable)
   - Import resolution

2. **Code Quality**
   - Cyclomatic complexity
   - Maintainability index
   - Code duplication
   - Comment coverage

3. **Best Practices**
   - Lint rule violations
   - Deprecated API usage
   - Security anti-patterns

4. **Documentation**
   - Missing doc comments on public APIs
   - Incomplete README coverage

**Output Schema:**
```json
{
  "issues": [
    {
      "file": "src/main.go",
      "line": 42,
      "column": 5,
      "severity": "error",
      "category": "lint",
      "rule": "gocritic",
      "message": "consider using fmt.Fprintf",
      "link": "https://staticcheck.io/docs/checks/#fmt"
    }
  ],
  "metrics": {
    "complexity": 12.5,
    "maintainability": 78.2,
    "duplication": 3.2
  }
}
```

### FR-003: Containerized Runtime Validation

**Description:** Safely execute code in isolated containers with resource limits.

**Container Configuration:**
- Base image: debian:bookworm-slim
- Memory limit: 512MB (configurable)
- CPU limit: 1 core (configurable)
- Disk limit: 1GB
- Network: none (isolated)
- Timeout: 5 minutes max

**Execution Phases:**
1. Image pull/refresh (cached)
2. Code extraction to container
3. Dependency installation (if needed)
4. Build verification
5. Test execution (if test files present)
6. Result collection
7. Container cleanup

### FR-004: Security Vulnerability Scanning

**Description:** Identify known vulnerabilities in code and dependencies.

**Scan Targets:**
1. **Dependency Scanning**
   - Lock file analysis (go.sum, Cargo.lock, package-lock.json)
   - CVE database matching
   - GHSA cross-reference

2. **Secret Detection**
   - API key patterns
   - Private key detection
   - Credentials in comments

3. **Supply Chain**
   - License compliance
   - Deprecated package detection
   - Malicious package patterns

**Vulnerability Database:**
- NVD (National Vulnerability Database)
- GHSA (GitHub Security Advisories)
- OSV (Open Source Vulnerabilities)
- Update frequency: daily

### FR-005: Integration Testing Service

**Description:** Execute integration tests against validated codebases.

**Test Types:**
1. **Unit Test Execution**
   - `go test ./...`
   - `cargo test`
   - `npm test`
   - `pytest`

2. **API Contract Testing**
   - OpenAPI spec validation
   - Request/response schema checking

3. **Database Integration**
   - Migration validation
   - Query performance benchmarks

4. **Service Mocking**
   - HTTP endpoint mocking
   - Database in-memory alternatives

### FR-006: Quality Score Calculation

**Description:** Aggregate validation results into actionable quality scores.

**Scoring Dimensions (0-100):**
| Dimension | Weight | Components |
|-----------|--------|-----------|
| Correctness | 25% | Syntax errors, type errors, test failures |
| Security | 25% | Vulnerabilities, secrets, compliance |
| Performance | 20% | Complexity, resource usage |
| Maintainability | 15% | Duplication, documentation, lint |
| Reliability | 15% | Error handling, test coverage |

**Overall Score Formula:**
```
Overall = (Correctness × 0.25) + (Security × 0.25) + 
          (Performance × 0.20) + (Maintainability × 0.15) + 
          (Reliability × 0.15)

Quality Gate = Overall ≥ 80 AND Security ≥ 90
```

### FR-007: REST API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/v1/validate/codebase` | POST | Submit codebase for validation |
| `/api/v1/validate/{id}` | GET | Get validation status/results |
| `/api/v1/validate/{id}/cancel` | POST | Cancel running validation |
| `/api/v1/validate/{id}/issues` | GET | Get paginated issues |
| `/api/v1/webhook` | POST | Register CI/CD webhook |
| `/api/v1/webhook/{id}` | DELETE | Unregister webhook |
| `/api/v1/metrics/dashboard` | GET | Quality dashboard data |
| `/api/v1/health` | GET | Health check |

**Authentication:**
- Bearer token (JWT) for all endpoints except `/health`
- API key for webhook callbacks

### FR-008: CI/CD Webhook Integration

**Supported Platforms:**
- GitHub Actions
- GitLab CI
- Jenkins (via generic webhook)

**Webhook Events:**
| Event | Trigger |
|-------|---------|
| `pull_request.opened` | New PR submitted |
| `pull_request.synced` | PR updated |
| `push` | Direct push to protected branch |
| `schedule` | Periodic validation |

**Webhook Payload:**
```json
{
  "event": "pull_request.opened",
  "repository": "owner/repo",
  "branch": "feature/xyz",
  "commit": "abc123",
  "validation_url": "https://kwality.app/validate/123"
}
```

### FR-009: Results Export

**Supported Formats:**
| Format | Use Case | Schema |
|--------|----------|--------|
| JSON | API consumption, custom processing | Internal schema |
| SARIF | SIEM integration, GitHub Advanced Security | OASIS standard |
| JUnit XML | CI system consumption | W3C standard |
| PDF | Executive reports | Custom template |
| HTML | Interactive viewer | Kwality dashboard |

### FR-010: Container Isolation and Resource Limits

**Isolation Requirements:**
- Each validation runs in dedicated container
- No cross-validation data leakage
- Filesystem: read-only except temp directory
- Process isolation via cgroups v2
- Network: no outbound connections
- User: non-root (UID 65534)

**Resource Enforcement:**
```yaml
resources:
  limits:
    memory: 512Mi
    cpu: "1.0"
    ephemeral-storage: 1Gi
  requests:
    memory: 256Mi
    cpu: "0.5"
```

---

## Implementation Notes

- Framework: Go orchestration + Rust runtime engine
- Database: PostgreSQL for results, Redis for queue
- Container: Docker with cgroups v2
- Queue: Redis Streams for job management
- Monitoring: Prometheus + Grafana

---

## Traceability Matrix

| FR | User Story | Architecture Component |
|----|------------|------------------------|
| FR-001 | US-001 | Validation Coordinator |
| FR-002 | US-001, US-002 | Static Analysis Engine |
| FR-003 | US-001 | Runtime Validation Engine |
| FR-004 | US-002 | Security Scanner |
| FR-005 | US-001 | Integration Testing Service |
| FR-006 | US-003 | Results Aggregator |
| FR-007 | US-004 | API Gateway |
| FR-008 | US-004 | Webhook Handler |
| FR-009 | US-003 | Export Service |
| FR-010 | US-001 | Docker Container Management |
