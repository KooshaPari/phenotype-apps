# ADR-054 — PhenoCompose vs nanovms consolidation (W7-2)

**Status:** PROPOSED (W7-2)
**Date:** 2026-06-19
**Owner:** substrate-audit circle
**Category:** Strategic consolidation (Phase 8 governance)

## Context

`PhenoCompose` and `nanovms` are two focus repos in the fleet that both work in the "composable service / WASM runtime" domain. They were created at different points and have overlapping but non-identical scope:

| Repo | Lang | Created | Role |
|------|------|---------|------|
| `KooshaPari/PhenoCompose` | Rust | 2026-05 | Composable service layer (L4 hexagonal, port-adapter) |
| `KooshaPari/nanovms` | Rust | 2026-03 | WASM-based micro-VM runtime |

The V3 execution log Phase 8 listed this as an open question: "same domain, different scope; consolidate or document boundary?"

## The 3 options

### Option A: PhenoCompose is canonical, nanovms is archived
**Pro:** PhenoCompose is more recent and follows the L4 hexagonal pattern (ADR-014)
**Pro:** nanovms WASM micro-VMs are an "implementation detail" that should live in a substrate lib
**Con:** nanovms has production usage (1+ fleet member calls it)
**Con:** 5 L2 pre-commit baseline worktrees on nanovms (L2-33)

### Option B: nanovms is canonical, PhenoCompose is archived
**Pro:** nanovms is more battle-tested (3 months older)
**Con:** PhenoCompose has the L4 hexagonal design that aligns with the substrate graduation path
**Con:** PhenoCompose is what V4 §5 calls out as the canonical L4 layer

### Option C: Document the boundary (split the responsibilities)
- **PhenoCompose** = the L4 hexagonal *interface* (port traits, adapter impls, conformance tests)
- **nanovms** = the L4 *implementation* (WASM micro-VM is one of several possible backends)
**Pro:** Both remain alive; each does what it does best
**Pro:** Matches the substrate pattern: PhenoCompose = trait surface, nanovms = one implementation
**Con:** Two repos; but the boundary is clean

## Recommendation: **Option C (document the boundary)**

This is the **cleanest answer** and matches the substrate graduation path (ADR-048). PhenoCompose should be a *trait surface* (like pheno-tower from V17), and nanovms should be one of several possible *implementations* (like pheno-tokio-base / pheno-axum-stack are alternative impls).

## Action items

1. **Write ADR-054** (this file)
2. **Promote PhenoCompose to substrate** (port trait surface, no impl)
3. **Add nanovms as a `pheno-runtime-wasm` substrate** (or similar) that *implements* the PhenoCompose trait
4. **Move the WASM micro-VM code from nanovms into the new substrate repo**
5. **Migrate the 1+ caller** to depend on the substrate + the implementation (instead of nanovms directly)

## Acceptance criteria

- [ ] `PhenoCompose` is a substrate (trait surface only, no impl)
- [ ] `nanovms` becomes `pheno-runtime-wasm` (or similar), implements the trait
- [ ] WASM code is in one place, not duplicated
- [ ] 1+ caller uses substrate + impl correctly

## References

- V3 execution log Phase 8 (open question)
- V4 §5 (L4 hexagonal layer, the port-adapter pattern)
- ADR-014 (Hexagonal port-adapter L4 policy)
- ADR-038 (Hexagonal port-adapter L4 policy formal)
- ADR-048 (substrate graduation path)
