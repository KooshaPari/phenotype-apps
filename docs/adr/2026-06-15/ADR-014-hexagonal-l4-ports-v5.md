# ADR-014: Hexagonal L4 ports (Port trait + Adapter impl)

**Status:** SUPERSEDED
**Date:** 2026-06-15
**Superseded by:** ADR-038 (Hexagonal port-adapter L4 policy, 2026-06-18 v8 wave A)
**Author:** orchestrator (v5 SOTA sweep)
**Refs:**
- ADR-038 (canonical successor for v8+)
- ADR-018 (PRCP pattern — formalization of cross-language port/adapter)

---

## Context (historical)

In the v5 SOTA sweep (2026-06-15), the hexagonal L4 Port/Adapter pattern was introduced for substrate: each substrate crate exposes a `Port` trait (interface) and ships with concrete `Adapter` impls (in-tree) plus an open extension point (out-of-tree). This decision was an informal v5 finding; it was not given a stable ADR number until v8 renumbered it.

## Decision (historical)

Every substrate crate has:
- A `Port` trait (the interface) — the only public API surface
- Concrete `Adapter` impls (in-tree) — implementations
- An open extension point (out-of-tree) — users can add custom adapters

## Why superseded

- ADR-038 (2026-06-18) is the formal, v8-wave-A ratified version
- ADR-038 adds: lint enforcement (`scripts/check-hex-ports.sh`), tier-specific port surface rules, observability via ADR-036B
- ADR-038 adds: coverage gate via ADR-040, quality bar via ADR-042B
- The v5 statement remains accurate but ADR-038 is the source of truth going forward

## Cross-references

- ADR-038 (canonical successor)
- ADR-018 (PRCP pattern)
- ADR-036B (tracing substrate)
- ADR-040 (coverage gates)
- ADR-042B (quality bar)
- ADR-048 (graduation path)
