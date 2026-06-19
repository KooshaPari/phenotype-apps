# ADR-055 — Pine vs PhenoCompose consolidation (W7-3)

**Status:** PROPOSED (W7-3)
**Date:** 2026-06-19
**Owner:** substrate-audit circle
**Category:** Strategic consolidation (Phase 8 governance)

## Context

`Pine` and `PhenoCompose` are two focus repos in the fleet that both work in the "data shape" domain. They were created at different points and have overlapping but non-identical scope:

| Repo | Lang | Created | Role |
|------|------|---------|------|
| `KooshaPari/Pine` | Rust | 2026-04 | PE/ELF binary loader + parser |
| `KooshaPari/PhenoCompose` | Rust | 2026-05 | Composable service layer (L4 hexagonal, port-adapter) |

The V3 execution log Phase 8 listed this as an open question: "both data-shape substrates; consolidate or document boundary?"

(Note: PhenoCompose is more of a "composable service" substrate than a "data shape" substrate, but the boundary is fuzzy.)

## The 3 options

### Option A: Pine is canonical, PhenoCompose is archived
**Pro:** Pine has actual Rust source (`crates/pine-loader/src/lib.rs`); PhenoCompose is more design-doc than code
**Pro:** PE/ELF parsing is a well-defined scope; composable service is a much wider scope
**Con:** PhenoCompose has the L4 hexagonal design that aligns with substrate graduation

### Option B: PhenoCompose is canonical, Pine is archived
**Pro:** PhenoCompose is the L4 layer; Pine is more of a leaf utility
**Con:** Pine is *used* (at least 1 caller); PhenoCompose has 0 callers

### Option C: Document the boundary (split the responsibilities)
- **PhenoCompose** = the L4 hexagonal *interface* (port traits, no impl)
- **Pine** = a leaf substrate that provides PE/ELF parsing *as a port* in PhenoCompose
**Pro:** Both remain alive; Pine becomes one of many possible "data shape ports" that PhenoCompose composes
**Pro:** PhenoCompose becomes the substrate; Pine is one of N adapters
**Con:** Pine's scope is narrow enough that it could be a single library

## Recommendation: **Option C (document the boundary, with PhenoCompose as substrate)**

Pine is a real Rust crate (190+ LoC) that does PE/ELF parsing. It should remain as a *leaf substrate* (pheno-loader style). PhenoCompose should be the L4 *interface substrate* that composes such leaves.

In practice this means:
- **Pine** = `pheno-pe-loader` (substrate, narrow scope, well-defined)
- **PhenoCompose** = `pheno-compose` (substrate, wide scope, L4 hexagonal interface)
- Pine *implements* a PhenoCompose port for "binary data shapes" (alongside other data-shape adapters)

## Action items

1. **Write ADR-055** (this file)
2. **Rename Pine to pheno-pe-loader** (or accept the existing name; both are OK)
3. **Add a `pe_loader` port trait to PhenoCompose** that Pine implements
4. **Document the boundary** in PhenoCompose's README
5. **Migrate the 1+ caller** to use the PhenoCompose port + the Pine adapter

## Acceptance criteria

- [ ] Pine remains a real Rust crate (no archival)
- [ ] PhenoCompose documents "Pine is one of the data-shape adapters"
- [ ] A `pe_loader` port trait exists in PhenoCompose
- [ ] Pine implements that trait
- [ ] 1+ caller uses the port + adapter

## References

- V3 execution log Phase 8 (open question)
- V4 §5 (L4 hexagonal layer)
- ADR-014, ADR-038 (hexagonal port-adapter L4 policy)
- ADR-048 (substrate graduation path)
- ADR-054 (PhenoCompose vs nanovms; the adjacent question)
