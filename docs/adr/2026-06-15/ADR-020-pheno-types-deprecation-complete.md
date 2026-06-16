# ADR-020: `phenoTypes` deprecation — complete (folded into `pheno-pydantic-models` + `pheno-zod-schemas`)

**Status:** Accepted 2026-06-15
**Deciders:** PhenoRust 1.0 architecture circle
**Supersedes:** ad-hoc `phenoTypes` Python-typing crate

## Context

`phenoTypes` was a 1× Python typing helper crate (pydantic v1
shim) that bridged `pheno-pydantic-models` and `pheno-zod-schemas`
during V2. The V3 audit found that:

- pydantic v2 deprecated the shim API in 2025-11.
- The cross-language schema sync (Python → TypeScript) moved
  to `pheno-llms-txt` (L3 substrate for typed model export).
- The remaining 4 type aliases (`PhenoId`, `PhenoTimestamp`,
  `PhenoTag`, `PhenoRef`) are now in `pheno-pydantic-models`
  and the `pheno-zod-schemas` mirror.

## Decision

`phenoTypes` is **fully deprecated** (not 1-minor-version —
fully). The 4 type aliases are inlined:

1. `pheno-pydantic-models` adds `PhenoId`, `PhenoTimestamp`,
   `PhenoTag`, `PhenoRef` as frozen pydantic v2 models.
2. `pheno-zod-schemas` adds the corresponding zod schemas.
3. `pheno-llms-txt` exports both as `models.json` so consumers
   in either language pick them up via the schema-sync flow.
4. `phenoTypes` is yanked from crates.io / PyPI; the
   `phenotype-org/phenoTypes` repo is archived with a README
   redirect to `pheno-pydantic-models`.

## Consequences

**Positive**

- 1× crate removed from the public API surface.
- The schema-sync flow (`pheno-llms-txt`) is now the single
  way to keep Python and TypeScript types in sync — no
  hand-rolled shim.
- The 4 type aliases gain runtime validation (they were
  `NewType` only before).

**Negative**

- The Python→TypeScript bridge in `pheno-llms-txt` becomes a
  critical path for the 8+ consumers that used `phenoTypes`
  in V2/V3 code. It is already on the critical path for the
  V5 SOTA work, so this is a lateral move, not a new dep.

**Mitigation**

- `phenoTypes` is yanked, not deprecated; the next consumer
  import fails fast, with a clear "use
  `pheno-pydantic-models.PhenoId`" error.

## Alternatives considered

- **Keep `phenoTypes` and update the pydantic v2 shim.**
  Rejected: `pheno-llms-txt` already does the cross-language
  sync better.
- **Move the 4 types to a `pheno-types` crate that mirrors
  both languages.** Rejected: re-creates the same problem
  (`pheno-types` becomes the next `phenoTypes`).
- **Inline into `pheno-pydantic-models` and `pheno-zod-schemas`
  (the chosen path).** Accepted.

## References

- `phenoTypes/pyproject.toml` (yanked 2026-06-15).
- `pheno-pydantic-models/src/primitives.rs` — 4 frozen models.
- `pheno-zod-schemas/src/primitives.ts` — 4 zod schemas.
- `pheno-llms-txt/src/lib.rs` — schema-sync flow (the
  replacement).
- `V6 §Track 3` — `phenoTypes` deprecation work item.
