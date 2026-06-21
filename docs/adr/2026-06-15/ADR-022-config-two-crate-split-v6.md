# ADR-022: Config consolidation — two-crate canonical split

**Status:** SUPERSEDED (renamed)
**Date:** 2026-06-15
**Superseded by:** ADR-031 (Configra absorb) + ADR-035 (Configra migration gates)
**Author:** orchestrator (v6 closure)
**Refs:**
- ADR-031 (Configra absorb, 2026-06-17)
- ADR-035 (Configra migration gates, 2026-06-18)
- ADR-017 (settly-archive, 2026-06-15)

---

## Context (historical)

In the v6 closure (2026-06-15), config was split into two crates:
- `phenotype-config` (Rust core) — for backend services
- `phenotype-config-ts` (TS edge) — for the Conft TS bindings

The split was a v6 finding; it was not given a stable ADR number until v8 renumbered it as ADR-022.

## Decision (historical)

Two-crate canonical split:
- Rust core (`phenotype-config`) — env-var, TOML, 12-factor cascade
- TS edge (`phenotype-config-ts`) — JS/TS bindings, schema-validated

## Why superseded (renamed only)

- ADR-031 (2026-06-17) renamed the canonical config repo from `phenotype-config` to `KooshaPari/Configra`. The Rust/TS edge split remains valid in principle but the canonical Rust name is now `Configra`.
- ADR-035 (2026-06-18) codifies the migration gates for the 8 source repos.
- The v6 statement remains accurate but ADR-031 + ADR-035 are the source of truth going forward.

## Cross-references

- ADR-031 (Configra absorb — successor for naming)
- ADR-035 (Configra migration gates — successor for migration plan)
- ADR-017 (settly-archive — predecessor that freed the config consolidation space)
- ADR-023 (substrate placement — `phenotype-*-sdk` tier for the TS edge)
