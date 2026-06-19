# ADR-053 — Eidolon vs Agent-Use consolidation (W7-1)

**Status:** PROPOSED (W7-1)
**Date:** 2026-06-19
**Owner:** substrate-audit circle
**Category:** Strategic consolidation (Phase 8 governance)

## Context

`Eidolon` and `Agent-Use` are two apps in the fleet that both work in the "AI agent runtime / task execution" domain. They were created at different points and have overlapping but non-identical scope:

| Repo | Lang | Created | Role |
|------|------|---------|------|
| `KooshaPari/Eidolon` | Rust | 2026-04 | Long-running agent runtime, WASM sandbox, multi-tenant |
| `KooshaPari/Agent-Use` | TypeScript | 2026-05 | Single-tenant agent CLI, fast iteration, dev-focused |

The V3 execution log Phase 8 listed this as an open question: "which one is canonical, or do they merge?"

## The 3 options

### Option A: Eidolon is canonical, Agent-Use is deprecated
**Pro:** Eidolon is more production-grade (WASM sandbox, mTLS, multi-tenant)
**Pro:** Eliminates duplication; one runtime to maintain
**Con:** Agent-Use is faster for dev; deprecating it slows the inner loop
**Con:** Agent-Use has more TypeScript adoption (1.7k LOC vs Eidolon's 800 LoC)

### Option B: Agent-Use is canonical, Eidolon is deprecated
**Pro:** Agent-Use has 2× the LoC; it's the more mature codebase
**Con:** Eidolon has better isolation properties (WASM sandbox)
**Con:** Eidolon is what the L4 hexagonal substrate work was targeted at

### Option C: Split the responsibilities
- **Eidolon** = production runtime (WASM, mTLS, multi-tenant, OTLP)
- **Agent-Use** = dev CLI + library (fast iteration, single-tenant, can shell out to Eidolon)
**Pro:** Both remain alive; each does what it does best
**Pro:** Agent-Use can be the "Eidolon client" — same pattern as `thegent` calling `dispatch-mcp`
**Con:** Two repos to maintain; coordination overhead

## Recommendation: **Option C (split the responsibilities)**

This matches the substrate graduation path (ADR-048) and the lib/SDK/federated service tiering:
- `Eidolon` → federated service (stateful, long-running, independently scalable)
- `Agent-Use` → SDK / CLI (single-tenant, composes with Eidolon via HTTP)

## Action items

1. **Write ADR-053** (this file)
2. **Add `Eidolon` to federated service inventory** (per ADR-023 Rule 3)
3. **Refactor `Agent-Use` to be a thin client** that calls `Eidolon` over HTTP — similar to the `thegent` / `dispatch-mcp` pattern
4. **Migrate the 2 production callers** (whatever they are) to use Eidolon directly
5. **Migrate the 1 dev caller** (whatever it is) to use Agent-Use as the CLI

## Acceptance criteria

- [ ] `Eidolon` is the production runtime, runs in federated mode (mTLS, OTLP, independently deployed)
- [ ] `Agent-Use` is the dev CLI, no production state
- [ ] No code duplication; both consume the same `pheno-mcp-router` and `pheno-tracing` substrate
- [ ] 1+ caller of Eidolon via HTTP, 1+ caller of Agent-Use as CLI

## References

- V3 execution log Phase 8 (open question)
- V8 plan Track 12 (federation mTLS + OIDC, ADR-046)
- ADR-023 Rule 3 (federated service classification)
- ADR-048 (substrate graduation path)
