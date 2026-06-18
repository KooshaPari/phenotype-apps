# ADR-041: Worklog Format — Both Markdown and JSONL Stay

**Date:** 2026-06-18
**Status:** ACCEPTED
**Deciders:** orchestrator (T11.1 decision), worklog-schema circle
**Supersedes:** (none)
**Related:** `findings/2026-06-18-T11-worklog-merge-decision.md`, `findings/2026-06-18-T11-worklog-consumer-probe.md`, ADR-032

## Context

Two worklog formats coexist in the Phenotype fleet:

1. **Markdown (`WORKLOG.md`)** — human-readable, table format (`Date | Task ID | Layer | Action | Files | Notes`),
   validated by `pheno-worklog-schema` (Python lib).
2. **JSONL (`worklog-L<level>-<date>.json`)** — machine-readable, one JSON object per line,
   used by `AgilePlus` and the wider fleet DAG.

Question: should they be unified?

## Decision

**BOTH STAY.** The two formats are **complementary, not duplicating**.

## Rationale

1. **Different audiences**
   - Markdown: humans, contributors, on-call engineers reading commit-time work logs
   - JSONL: machines, DAG generators, audit tools, automation pipelines

2. **Zero in-repo consumers of either format** (T11.3 probe)
   - 40+ markdown `WORKLOG.md` files exist
   - 38+ JSONL `worklog-L*.json` files exist
   - **0 in-repo files reference either format as input**
   - Both are **leaves** (generated/authored, not consumed)

3. **Cost of merging > benefit**
   - Cost: 3-5 h engineering + ongoing maintenance burden
   - Benefit: marginal (no internal consumer needs the unification)

4. **Different schemas serve different purposes**
   - Markdown validates via `pheno-worklog-schema` (Python lib, ADR-015 v2.0)
   - JSONL validates via `pheno-worklog-schema` JSON parser (proposed ADR-015 v2.1)

## Consequences

### Positive

- No migration work needed
- Both formats continue to evolve independently
- Each format can optimize for its own use case (humans vs machines)
- Future ADR-015 v2.1 adds `device:` field to BOTH formats (single bump)

### Negative

- Documentation burden: contributors need to know which format to use
- Onboarding friction: new contributors see two formats and ask "why?"
- Schema drift risk if not synchronized (mitigated by ADR-015 v2.1)

## Enforcement

- **Markdown format**: validated by `pheno-worklog-schema` Python lib (L1-3 fleet-critical substrate)
- **JSONL format**: validated by `pheno-worklog-schema` JSON parser (L4-5 task audit trail)
- **Both formats**: must include `device:` field per ADR-015 v2.1 (ADR-025)
- **Cross-format queries**: future T20 may add a query tool that joins both formats; not in v8

## Alternatives Considered

| Option | Description | Decision |
|---|---|---|
| Unify to Markdown | Make JSONL a derived view of markdown | ❌ Rejected — loses machine-parseable detail |
| Unify to JSONL | Make markdown a derived view of JSONL | ❌ Rejected — loses human readability |
| **Both stay** | **Independent formats, no merge** | ✅ **ACCEPTED** |
| Build a new unified schema | Create a new format that supersedes both | ❌ Rejected — 3-5 h cost, marginal benefit |

## Related

- ADR-015 v2.0 (10-column WORKLOG.md schema)
- ADR-015 v2.1 (adds `device:` field, deprecation 2026-06-22)
- ADR-032 (worklog format split — T11.1 decision codification)
- `findings/2026-06-18-T11-worklog-merge-decision.md` (75 LoC, BOTH STAY)
- `findings/2026-06-18-T11-worklog-consumer-probe.md` (53 LoC, 0 consumers)
