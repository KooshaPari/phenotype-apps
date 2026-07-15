# KodeVibe Configuration Schema

**Source:** [KodeVibeGo](https://github.com/KooshaPari/KodeVibeGo) (deprecated, consolidated into HexaKit)  
**Canonical config file:** `.kodevibe.yaml`  
**Extracted:** 2026-05-31

This document is the authoritative schema reference for the KodeVibe Shell CLI. It was ported from the Go implementation's `internal/models/models.go` and `pkg/config/config.go`.

---

## Top-Level Structure

```yaml
vibes:          # Per-category checker configuration
project:        # Project metadata
exclude:        # File/pattern exclusions
custom_rules:   # Regex-backed custom rules
integrations:   # Slack, GitHub, Jira, Teams, webhook
advanced:       # Entropy analysis, concurrency, caching
languages:      # Per-language overrides
ci_cd:          # CI/CD quality gates
reporting:      # Output format and logging
```

---

## Vibes

Each vibe category maps to a `Checker` in the Go registry (now implemented in Shell + HexaKit compliance scanner).

| Vibe | Type | Default Level | Key Settings |
|------|------|---------------|--------------|
| `security` | `security` | strict | vulnerability scanning, secret detection |
| `code` | `code` | moderate | `max_function_length`, `max_nesting_depth` |
| `performance` | `performance` | moderate | `max_bundle_size` |
| `file` | `file` | strict | naming conventions, file structure |
| `git` | `git` | moderate | `min_commit_message_length` |
| `dependency` | `dependency` | moderate | `check_vulnerabilities` |
| `documentation` | `documentation` | moderate | doc coverage (often disabled for small projects) |

```yaml
vibes:
  security:
    enabled: true
    level: strict          # strict | moderate | relaxed
    settings: {}           # vibe-specific key/value pairs
  code:
    enabled: true
    level: moderate
    settings:
      max_function_length: 50
      max_nesting_depth: 4
  performance:
    enabled: true
    level: moderate
    settings:
      max_bundle_size: 2MB
  file:
    enabled: true
    level: strict
  git:
    enabled: true
    level: moderate
    settings:
      min_commit_message_length: 10
  dependency:
    enabled: true
    level: moderate
    settings:
      check_vulnerabilities: true
  documentation:
    enabled: false
    level: moderate
```

### Level Semantics

| Level | Behavior |
|-------|----------|
| `strict` | All rules enforced; errors block CI gates |
| `moderate` | Standard rules; warnings for minor issues |
| `relaxed` | Critical/security only |

---

## Project

```yaml
project:
  type: auto-detect       # web | mobile | desktop | library | cli | auto-detect
  language: auto-detect   # javascript | python | go | rust | auto-detect
  framework: ""           # react | vue | express | etc.
  name: ""                # optional project name
  description: ""         # optional
```

---

## Exclude

```yaml
exclude:
  files:
    - "node_modules/**/*"
    - ".git/**/*"
    - "coverage/**/*"
    - "*.min.js"
    - "vendor/**/*"
    - "build/**/*"
    - "dist/**/*"
  patterns:
    - "test-*"
    - "*.test.*"
    - "*.spec.*"
```

---

## Custom Rules

Regex-backed rules evaluated during scan:

```yaml
custom_rules:
  - name: "no-console-log"
    pattern: "console\\.log\\("
    message: "Remove console.log statements before committing"
    severity: warning     # error | warning | info | critical
  - name: "todo-comments"
    pattern: "(?i)\\b(todo|fixme|hack|xxx)\\b"
    message: "TODO comments should be tracked in issues"
    severity: info
```

---

## Integrations

```yaml
integrations:
  slack:
    enabled: false
    webhook_url: "${SLACK_WEBHOOK}"
    channel: ""
  github:
    enabled: false
    token: ""
    owner: ""
    repo: ""
    create_issues: false
  jira:
    enabled: false
    url: ""
    username: ""
    token: ""
    project_key: ""
  teams:
    enabled: false
    webhook_url: ""
  webhook:
    enabled: false
    url: ""
```

---

## Advanced

```yaml
advanced:
  entropy_analysis: true       # Detect high-entropy strings (secrets)
  entropy_threshold: 4.5
  ai_detection: false            # AI-generated code detection
  ai_provider: ""
  ai_model: ""
  external_scanners: []          # Paths to external scanner binaries
  performance_profiling: false
  cache_enabled: true
  cache_ttl: 1h0m0s
  max_concurrency: 10            # Parallel file scanning
  timeout: 5m0s
  custom_analyzers: []
```

---

## CI/CD Quality Gates

```yaml
ci_cd:
  github_actions:
    enabled: false
    fail_on: []                  # List of vibe types that fail the build
  gitlab_ci:
    enabled: false
  jenkins:
    enabled: false
  quality_gates:
    min_code_coverage: 0         # 0 = disabled
    max_complexity_score: 0
    max_security_issues: 0
    max_performance_issues: 0
```

---

## Reporting

```yaml
reporting:
  generate_reports: true
  report_format: text            # text | json | html | junit | csv
  report_path: ./kodevibe-reports
  logging:
    enabled: true
    level: info                  # debug | info | warn | error
    format: json                 # json | text
    file: ""
```

---

## Agent Quick Endpoints (Daemon Mode)

When running KodeVibeGo daemon (`cmd/server`), these minimal endpoints served CI agents:

| Endpoint | Purpose |
|----------|---------|
| `GET /quick` | One-shot scan summary (score + top issues) |
| `GET /status/compact` | Health + last scan timestamp |
| `GET /metrics` | Prometheus metrics |

The Shell CLI equivalent: `kodevibe scan --format json | jq '.overall_score'`.

---

## MCP Context Payload

For AI fix loops, scan results attach to MCP context (see HexaKit governance docs):

```json
{
  "project_path": "/path/to/project",
  "scan_results": { "scan_id": "...", "issues": [] },
  "quality_targets": {
    "min_score": 80.0,
    "max_issues": 10,
    "required_grade": "B",
    "focus_areas": ["security", "performance"]
  }
}
```

---

## Predecessor Lineage

KodeVibeGo (Go) is deprecated and consolidated into **KodeVibe** (Shell CLI) + **HexaKit** (governance). See [predecessor-kodevibego.md](./predecessor-kodevibego.md).
