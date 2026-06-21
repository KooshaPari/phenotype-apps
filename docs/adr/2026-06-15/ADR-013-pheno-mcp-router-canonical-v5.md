# ADR-013: pheno-mcp-router substrate canonical

**Status:** SUPERSEDED
**Date:** 2026-06-15
**Superseded by:** ADR-037 (pheno-mcp-router substrate canonical, 2026-06-18 v8 wave A)
**Author:** orchestrator (v5 SOTA sweep)
**Refs:**
- ADR-037 (canonical successor for v8+)
- ADR-008 (dispatch-mcp as sole MCP server — v6 closure)

---

## Context (historical)

In the v5 SOTA sweep (2026-06-15), `pheno-mcp-router` was declared the canonical MCP substrate. This decision was an informal v5 finding; it was not given a stable ADR number until v8 renumbered it.

## Decision (historical)

`pheno-mcp-router` is the single MCP routing entry point. All MCP traffic (provider routing, cost budget, quota, audit) flows through it.

## Why superseded

- ADR-037 (2026-06-18) is the formal, v8-wave-A ratified version
- ADR-037 adds: tier-aware port/adapter mapping (ADR-038), provider-agnostic LlmPort, observability via ADR-036B
- ADR-037 adds: cost/budget/quota/audit cross-refs (from dispatch-mcp migration)
- The v5 statement remains accurate but ADR-037 is the source of truth going forward

## Cross-references

- ADR-037 (canonical successor)
- ADR-008 (dispatch-mcp as sole MCP server)
- ADR-036B (tracing substrate — observed by ADR-037)
- ADR-038 (hexagonal port/adapter L4)
- ADR-042B (quality bar)
- ADR-048 (graduation path)
