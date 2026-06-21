# ADR-012: pheno-tracing canonical across pheno-* repos

**Status:** SUPERSEDED
**Date:** 2026-06-15
**Superseded by:** ADR-036B (pheno-tracing substrate canonical, 2026-06-18 v8 wave A)
**Author:** orchestrator (v5 SOTA sweep)
**Refs:**
- ADR-036B (canonical successor for v8+)
- ADR-018 (PRCP pattern — cross-language tracing normalization)

---

## Context (historical)

In the v5 SOTA sweep (2026-06-15), `pheno-tracing` was declared the canonical tracing substrate across all pheno-* repos. This decision was an informal v5 finding; it was not given a stable ADR number until v8 renumbered it.

## Decision (historical)

`pheno-tracing` is the single tracing entry point. All other pheno-* repos emit OTLP via `pheno-tracing`, not their own wrapper.

## Why superseded

- ADR-036B (2026-06-18) is the formal, v8-wave-A ratified version of this decision
- ADR-036B adds: tier-specific emission (lib vs framework), coverage gates (ADR-040), quality bar (ADR-042B)
- ADR-036B adds: substrate graduation criteria (ADR-048)
- The v5 statement remains accurate but ADR-036B is the source of truth going forward

## Cross-references

- ADR-036B (canonical successor)
- ADR-018 (PRCP pattern)
- ADR-040 (coverage gates)
- ADR-042B (quality bar)
- ADR-048 (graduation path)
