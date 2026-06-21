# ADR-047: Predictive DRY discipline (4-criterion rule)

**Status:** ACCEPTED
**Date:** 2026-06-18
**Author:** orchestrator (claude opus 4.7)
**L5-112** (v8 wave C)
**Refs:**
- ADR-018 (PRCP pattern — cross-language reuse)
- ADR-023 Rule 3 (substrate placement — where DRY code lives)
- ADR-048 (substrate graduation path — graduation criteria include DRY)
- ADR-049 (drift detector — detects DRY violations)

---

## Context

Premature DRY (abstract-before-concrete) is the most common failure mode in polyglot substrate. The team has historically:
- Extracted a "shared" lib with one real consumer + one hypothetical consumer → unused abstraction
- Combined two similar-but-different utilities into a single "common" lib → both consumers get worse
- Created a facade before the second consumer exists → facade drifts from actual usage

A 4-criterion rule lets the team apply DRY only when there's evidence the abstraction will hold.

## Decision

**DRY is applied only when ALL 4 criteria are met. Each criterion is named, measurable, and verifiable by a tool (`pheno-predict`).**

### The 4 criteria

| # | Criterion | What it means | Measurement |
|---|---|---|---|
| 1 | **Concrete consumer count ≥ 2** | The abstraction has at least 2 real consumers in production, not hypothetical | `grep -r 'use crate::' /fleet --include='*.rs' \| wc -l ≥ 2` |
| 2 | **Stable API for ≥ 30 days** | The proposed API has not changed in the last 30 days (no churn signal) | `git log --since='30 days ago' --oneline` on the consumer repos |
| 3 | **Cross-language reuse OR cross-substrate-domain** | The abstraction crosses either (a) language boundary (e.g. same API in Rust + Go) or (b) substrate domain (e.g. config + tracing both need it) | `pheno-predict` static analysis |
| 4 | **Quality bar passes** | The proposed extraction target passes ADR-042B 7-check bar | `pheno-quality-bar --repo <extraction-target>` |

If any criterion fails, the abstraction is NOT extracted. The code stays duplicated. A re-check is run every 90 days (or when a new consumer appears).

### Tooling: `pheno-predict`

`pheno-predict` is a CLI that:
- Scans a proposed PR
- Checks each of the 4 criteria
- Outputs `PASS` / `FAIL: <criterion> (<reason>)`
- Suggests the re-check date (today + 90 days, or sooner if a new consumer is added)

```bash
$ pheno-predict extract pheno-config-shared pheno-config pheno-config-loader
CRITERION 1 (consumer count): PASS (2)
CRITERION 2 (stable 30 days):  PASS (last change 47 days ago)
CRITERION 3 (cross-L/domain):  PASS (cross-substrate: config + tracing)
CRITERION 4 (quality bar):     PASS (7/7 checks)
VERDICT: EXTRACT (all 4 criteria met)
```

### Anti-pattern: the "random phenoShared" placement

ADR-023 Rule 3 explicitly forbids `phenoShared` (a random per-app `lib/` dir) as a placement. DRY extractions that fail criterion 3 (cross-L OR cross-domain) are NOT placed in `phenoShared` — they go in the substrate tier that makes sense:
- Cross-language reuse → `phenotype-*-sdk` (per-language binding) or `phenotype-*-framework` (single shared impl)
- Cross-substrate-domain → `pheno-*-lib` (single-domain lib)
- Within-app reuse → `pheno-*-lib` in the app's own workspace, NOT `phenoShared`

## Consequence

- 0 premature DRY extractions (measurable: 0 `phenoShared/` dirs created since adoption)
- All extractions are evidence-based (4 criteria verified)
- The fleet substrate stays focused (no speculative abstractions)
- DRY violations detected within 1 week by `pheno-drift-detector` (ADR-049)

## Cross-references

- ADR-018 (PRCP pattern — the cross-language reuse pattern this ADR formalizes)
- ADR-023 Rule 3 (substrate placement — where DRY code lives)
- ADR-042B (quality bar — criterion 4 references this)
- ADR-048 (substrate graduation path — graduation criteria include DRY check)
- ADR-049 (drift detector — detects DRY violations)
- ADR-050 / ADR-077 (vault migration — example of criterion 3 met: cross-L + cross-domain)
