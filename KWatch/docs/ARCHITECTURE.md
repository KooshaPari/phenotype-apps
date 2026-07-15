# Architecture

KWatch is a Go-only process watchdog and CLI surface. The historical TypeScript / Node.js layer was removed in commit `e7eff93` ("feat(kwatch): restore PR #1 changes as clean replacement branch") along with `bun.lock`, `claude-flow`, the `dist/` outputs, and the TypeScript test harness. This document describes the post-removal architecture.

## Language and Runtime

- **Language:** Go (see `go.mod`).
- **Binary:** Single statically-linked binary, `kwatch`, built with `CGO_ENABLED=0`.
- **Entry point:** `main.go` — a thin shim that delegates to the `cmd` package.
- **Build orchestration:** `Makefile` (targets: `build`, `dev`, `install`, `uninstall`, `test`, `deps`, `build-all`, `clean`).
- **No external runtime dependencies** beyond the Go standard library and a small set of pure-Go modules.

## Module Layout

| Package | Purpose |
|---------|---------|
| `main.go` | Process entry point; calls `cmd.Execute()` and exits non-zero on error. |
| `cmd/` | Cobra-style subcommand tree: `root`, `daemon`, `run`, `status`, `history`, `security`, `mcp`, `config`. |
| `runner/` | Watchdog engine: process supervision, restart policy, exit-code handling, signals. |
| `server/` | Long-lived daemon mode: HTTP endpoint, RPC handlers, request/response types. |
| `tui/` | Terminal UI built on `bubbletea` / `lipgloss`-style primitives; reads runner state. |
| `mcp/` | Model Context Protocol server surface for tool-call integration. |
| `security/` | Policy engine: command allow/deny lists, secret scanning, audit log. |
| `config/` | Config loading, defaults, environment overrides. |
| `kwatch/` | Runtime data directory (logs, history, state). |

## Data Flow

```
main.go
  └─ cmd.Execute()
       ├─ cmd/root.go        → flag parsing, config load
       ├─ cmd/daemon.go      → server/ + runner/ (long-lived)
       ├─ cmd/run.go         → runner/ (foreground, single target)
       ├─ cmd/status.go      → runner/ + history snapshot
       ├─ cmd/security.go    → security/ scan + policy enforcement
       ├─ cmd/mcp.go         → mcp/ server
       └─ cmd/config.go      → config/ introspection
```

`runner/` is the single source of truth for process state. `server/`, `tui/`, and `mcp/` are read-side surfaces that observe the runner; only `runner/` may mutate supervised-process state.

## Concurrency Model

- One goroutine per supervised process (the watch loop).
- The runner maintains an in-memory state machine; persistence is event-sourced to `.kwatch/kwatch.log` and `.proc-history.json`.
- Daemon-mode IPC uses a mutex-guarded state struct, exposed via HTTP handlers in `server/`.

## Configuration

- Defaults live in `config/config.go`.
- Runtime overrides: CLI flags, environment variables (`KWATCH_*`), and an optional `kwatch.yaml` in the working directory.
- Resolution order: CLI > env > file > defaults.

## Historical: TypeScript Layer Removal

Commit `e7eff93` removed the legacy TypeScript client and CLI bridge, along with:

- `package.json`, `bun.lock`, `claude-flow`
- `proc-service.ts`, `proc-cli.ts`, `tsconfig.json`, `jest.config.mjs`
- `dist/` outputs, `node_modules/` (untracked in subsequent commits)
- `MCP_README.md`, `SECURE_AUTH.md` (replaced by `SECURITY.md` and inline docs)

The `package.json` file remains in the tree as a vestigial manifest — it is **not** executed and has no effect on the Go build. See `docs/FAQ.md` for the rationale.

## Extension Points

- **New subcommand:** add a file under `cmd/`, register it in `root.go`.
- **New policy:** add a rule under `security/`, register in the policy list.
- **New TUI panel:** add a view under `tui/`, subscribe to runner events.
- **New MCP tool:** add a handler under `mcp/`, expose via the tool registry.
