# SOTA — KodeVibe

> **Last researched:** 2026-06-16  
> **Methods:** KodeVibeGo migration audit, quality-platform architecture review, LANGUAGE_PLACEMENT, comparative static-analysis tooling

## Executive summary

| Dimension | Our choice | Confidence | Deep dive |
|-----------|------------|------------|-----------|
| Technical | Go `engine/` + shell CLI UX | high | [docs/sota/technical.md](docs/sota/technical.md) |
| DX | `make engine-build`; CLI delegates to Go binary | med | [docs/sota/dx.md](docs/sota/dx.md) |
| UX | Shell CLI + hook install for developers | med | [docs/sota/ux.md](docs/sota/ux.md) |
| AX | MCP/daemon + genesis docs for agent PR review | high | [docs/sota/ax.md](docs/sota/ax.md) |
| Security | Secret/vulnerability vibes + review blocklist | high | [docs/sota/security.md](docs/sota/security.md) |
| Ops | Targeted engine build/test; doc PRs skip full matrix | med | [docs/sota/ops.md](docs/sota/ops.md) |
| Cost | Single quality runtime vs duplicated linters | high | [docs/sota/cost.md](docs/sota/cost.md) |

## Why this is optimal (for our constraints)

Go `engine/` is a **Tier 3 justified edge** for the `quality` role: the analyzer migrated from KodeVibeGo, file-walking performance, and MCP/daemon integration are already implemented in Go. Rewriting in Rust would delay fleet adoption without clear correctness gain. Shell CLI preserves approachable UX; kwality owns LLM validation separately.

## Fork status

- **Is fork:** no (successor to archived KodeVibeGo, not an OSS fork)

## Evolution triggers

Re-open research when:

- Rust static-analysis crate matches engine feature set with benchmark win
- kwality absorbs deterministic checks (charter conflict — escalate)
- Upstream Go analyzer ecosystem ships equivalent MCP-native daemon

## Linkage

- Charter scope: [charter.md](charter.md)
- Review enforcement: [review.md](review.md)
- Intent goals: [intent.md](intent.md)
- Quality platform: [docs/architecture/quality-platform.md](docs/architecture/quality-platform.md)
