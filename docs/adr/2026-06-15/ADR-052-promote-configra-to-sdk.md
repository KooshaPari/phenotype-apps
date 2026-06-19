# ADR-052 — Promote Configra to SDK tier (consolidate `phenotype-config` + `pheno-config` per ADR-022)

**Status:** PROPOSED (W6-4)
**Date:** 2026-06-19
**Owner:** Configra circle
**Tier:** lib → **SDK**

## Decision

Promote `KooshaPari/Configra` to SDK-grade substrate and absorb both `phenotype-config` and `pheno-config` into it. After this promotion, "the config repo" is one repo with one name, one API, one release cadence.

## Background

Three config-flavored repos exist:
1. `KooshaPari/Configra` — created 2026-03-25, the canonical name per Decision A (V3 §1)
2. `KooshaPari/phenotype-config` — has the 2-crate split per ADR-022 (Rust core + TS edge)
3. `pheno-config` (in monorepo) — a parallel L1 stabilization

This is the ADR-022 consolidation in action, and the v8 plan Track 11 (Configra migration gates) is the operational carrier.

## Promotion criteria checklist

- [ ] `Configra` is the canonical name (confirmed V3 §1)
- [ ] ADR-022 2-crate split (Rust core / TS edge) preserved
- [ ] `phenotype-config` archive scheduled for 2026-07-15 → **EXECUTED 2026-06-19** (per AGENTS.md)
- [ ] `pheno-config` (monorepo) needs to be deprecated and callers migrated
- [ ] No `phenotype-config-loader` and `phenotype-shared-config` callers in the 10 focus repos yet (they were re-pointed via `KooshaPari/pheno#238`)
- [ ] Release cadence: weekly (per ADR-041B substrate cadence)

## Action items

1. **Run migration gate** from v8 plan Track 11.6 (`phenotype-config` → `Configra`)
2. **Add `pheno-config` deprecation notice** in monorepo `pheno-config/README.md` pointing to `Configra`
3. **Cut `Configra` v0.2.0** with merged-in `phenotype-config` API surface
4. **Migrate all 10 focus repos** to use `Configra` (1-line import change)
5. **Archive `phenotype-config`** (already done) and `pheno-config` (new — schedule for 2026-08-15)
6. **Generate `llms.txt`** for `Configra` via `pheno-llms-txt`

## Acceptance criteria

- [ ] `KooshaPari/Configra` v0.2.0+ released
- [ ] `phenotype-config` and `pheno-config` both archived (one already done)
- [ ] All 10 focus repos import from `Configra` (no `pheno-config` or `phenotype-config` in Cargo.toml / pyproject.toml)
- [ ] `llms.txt` exists and is ≤ 200 lines
- [ ] 2+ polyglot consumers (Rust core + TS edge)
- [ ] Weekly release cadence established (per ADR-041B)

## References

- ADR-022 (Configra 2-crate split, L5-105 2026-06-18)
- V3 §1 Decision A (Configra is the canonical name)
- V8 plan Track 11 (Configra migration gates)
- `KooshaPari/pheno#238` (L5-110 sub-crate CANONICAL.md re-pointing)
- AGENTS.md (ADR-031/035 close notes)
