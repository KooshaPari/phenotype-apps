# ADR Index — 2026-06-15

This index lists the ADRs accepted on 2026-06-15 for the V5 SOTA-decision sweep
(`plans/2026-06-14-DAG-V5.md` handoff). All files live in this directory.

| #     | Title                                                                                                | Status   | File                                                 |
| :---- | :--------------------------------------------------------------------------------------------------- | :------- | :--------------------------------------------------- |
| 012   | `pheno-tracing` as the canonical tracing crate across all pheno-* repos                               | Accepted | `ADR-012-pheno-tracing-canonical.md`                 |
| 013   | `pheno-mcp-router` as the substrate for all pheno-mcp-* servers                                       | Accepted | `ADR-013-pheno-mcp-router-substrate.md`              |
| 014   | Hexagonal L4 ports pattern: `Port` trait + `Adapter` impl                                            | Accepted | `ADR-014-hexagonal-l4-ports.md`                      |
| 015   | V2 10-column WORKLOG.md schema (canonical across all pheno-* repos)                                   | Accepted | `ADR-015-v2-worklog-schema.md`                       |
| 016   | Fork-only-not-rewrite policy for SOTA libraries                                                      | Accepted | `ADR-016-fork-only-not-rewrite.md`                   |
| 017   | `settly-*` archive — full deprecation, source retired to monorepo history                              | Accepted | `ADR-017-settly-archive.md`                          |
| 018   | `prcp-*` pattern (Polyglot Reuse via Canonical Ports) for all 4 languages                               | Accepted | `ADR-018-prcp-pattern.md`                            |
| 019   | `pheno-vessel-*` full deprecation (replaced by `pheno-dep-guard` + ADR-006/006 nvms modality)          | Accepted | `ADR-019-pheno-vessel-deprecation-complete.md`       |
| 020   | `pheno-types-*` full deprecation (replaced by `pheno-pydantic` + `phenotype-zod-schemas` etc.)          | Accepted | `ADR-020-pheno-types-deprecation-complete.md`       |
| 021   | `pheno-profiling-*` replaces `Profila` as canonical cross-language profiler                            | Accepted | `ADR-021-pheno-profiling-replaces-profila.md`        |

## Scope

- **ADR-012..014** cover L3/L4 substrate decisions (tracing, MCP, hex ports).
- **ADR-015** covers the cross-repo worklog schema (a fleet-wide contract).
- **ADR-016** covers the policy for adopting upstream SOTA libraries (the
  "don't rewrite" rule that the 3 V3/V4 abandoned rewrites motivated).
- **ADR-017..021** (Track 5 of V6 plan) close out the remaining 5 fleet-wide
  consolidation + deprecation decisions identified by the V6 fan-out.

## Note on ADR-007..011

The earlier ADRs (007..011) referenced by the V5 plan live in their respective
sub-repos (e.g. `Tokn/docs/adrs/`, `Parpoura/docs/adr/`) rather than in this
meta-repo. This directory is the meta-repo's own ADR set for V5 SOTA decisions
that span the fleet, not a duplicate of the per-repo ADR collections.
