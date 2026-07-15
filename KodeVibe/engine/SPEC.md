# KodeVibe — Technical Specification

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                      CLI (cmd/cli)                       │
│                   cobra command tree                     │
├──────────┬──────────┬───────────────┬───────────────────┤
│  scan    │  daemon  │  watch        │  fix              │
├──────────┴──────────┴───────────────┴───────────────────┤
│                   Scanner Engine                         │
│          concurrent vibe execution + semaphore           │
├──────────┬──────────┬───────────────┬───────────────────┤
│SecurityVibe│CodeVibe│PerfVibe       │FileVibe/GitVibe/  │
│18+ patterns│smells  │N+1,mem leaks  │DepsVibe/DocsVibe  │
├──────────┴──────────┴───────────────┴───────────────────┤
│               Report / Auto-Fix Engine                   │
│       text, json, html, xml, junit, csv formats          │
├─────────────────────────────────────────────────────────┤
│              HTTP Server (daemon mode)                    │
│       REST API + WebSocket + File Watcher                │
├─────────────────────────────────────────────────────────┤
│                   Config (YAML)                           │
│              .kodevibe.yaml per project                   │
└─────────────────────────────────────────────────────────┘
```

## Components

| Component | Location | Responsibility |
|-----------|----------|---------------|
| CLI | `cmd/cli/` | Cobra-based command dispatch |
| Server | `cmd/server/` | HTTP daemon with REST + WS |
| Scanner | `pkg/scanner/` | Concurrent vibe orchestration |
| Vibes | `pkg/vibes/` | Individual check implementations |
| Report | `pkg/report/` | Multi-format report generation |
| Fix | `pkg/fix/` | Rule-based auto-fix with backup |
| Watch | `pkg/watch/` | Filesystem monitoring with debounce |
| Config | `pkg/config/` | YAML config loading and validation |
| Models | `internal/models/` | Core data structures |

## Data Models

```go
type ScanResult struct {
    Directory   string
    Timestamp   time.Time
    Vibes       map[string]VibeResult
}

type VibeResult struct {
    Passed      bool
    IssueCount  int
    Duration    time.Duration
    Issues      []Issue
}

type Issue struct {
    Rule        string
    Severity    Severity  // error | warning | info
    File        string
    Line        int
    Message     string
    Fix         *FixSuggestion
}

type Severity string // "error" | "warning" | "info"
```

## API Design

### REST Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| POST | `/api/v1/scan` | Create new scan |
| GET | `/api/v1/scan/:id` | Get scan result |
| GET | `/api/v1/scans` | List all scans |
| DELETE | `/api/v1/scan/:id` | Delete scan |
| GET | `/api/v1/config` | Get configuration |
| PUT | `/api/v1/config` | Update configuration |
| GET | `/api/v1/vibes` | List available vibes |
| POST | `/api/v1/fix` | Start auto-fix |
| POST | `/api/v1/watch/start` | Start file watching |
| GET | `/api/v1/watch/status` | Watch status |
| GET | `/ws` | WebSocket real-time updates |

### Quick Endpoints (AI Agent)

| Method | Path | Purpose |
|--------|------|---------|
| GET | `/quick` | Health check ("ok") |
| GET | `/status` | Full JSON status |
| GET | `/status/compact` | Single-line status |
| GET | `/history` | Scan history |
| GET | `/metrics` | Performance metrics |

## Configuration

```yaml
project:
  type: web          # web | api | library
  language: javascript
vibes:
  security: { enabled: true, level: strict }
  code: { enabled: true, max_function_length: 50, max_nesting_depth: 4 }
  performance: { enabled: true, max_bundle_size: "2MB" }
server:
  host: "0.0.0.0"
  port: 8080
  rate_limit: { rps: 100 }
custom_rules:
  - name: "no-console-log"
    pattern: "console\\.log\\("
    severity: warning
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Small project (<1k files) | <3s |
| Medium project (1k-10k files) | <15s |
| Large project (10k+ files) | <90s |
| Memory usage | <100MB |
| Default concurrency | 10 vibes parallel |
| File cache TTL | 1 hour |
| HTTP response time | <100ms for `/quick` |

## Extension Points

New vibes implement the `Checker` interface and register in `registry.go`.
New fix rules go in `pkg/fix/fixer.go` with pattern, replacement, and validation.
