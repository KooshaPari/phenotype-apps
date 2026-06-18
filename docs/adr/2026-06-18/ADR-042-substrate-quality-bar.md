# ADR-042: Substrate quality bar (Rule 3.1 — codified)

**Status:** ACCEPTED
**Date:** 2026-06-18
**Author:** orchestrator (claude opus 4.7) — v8 batch T14.5
**L8-008** (v8 track T14.5)
**Refs:**
- ADR-023 (Agent-effort governance — Rule 3.1 origin)
- ADR-040 (Test coverage gates per substrate tier)
- `AGENTS.md` § "Quality bar for new substrate (Rule 3.1)"
- `plans/2026-06-18-v8-dag-stable.md` § 3.6 (T14.5)

---

## Context

ADR-023 Rule 3 introduced four substrate placement types
(`pheno-*-lib` / `phenotype-*-sdk` / `phenotype-*-framework` / federated service)
and a 7-element "quality bar" for new substrate. The bar lived only in `AGENTS.md`
as natural-language prose. v8 (T14.5) formalizes it as an ADR so the bar can be
referenced from CI templates, registry lint, and PR review checklists.

Without a codified bar, new substrate PRs ship without spec, without docs, without
tests, or without observability — and the regression is invisible until the next
71-pillar probe (which is now weekly per ADR-041).

## Decision

**Every new `pheno-*-lib`, `phenotype-*-sdk`, `phenotype-*-framework`, or
federated service MUST ship with the 7-element quality bar before merge.**

### The 7 elements

| # | Element | Requirement | Owner of check |
|---|---|---|---|
| 1 | **Spec** | `SPEC.md` or equivalent at repo root, ≤ 1 page | PR review |
| 2 | **Docs** | `README.md` + 1 concept doc; covers what / when / when NOT / 5-line quickstart | PR review |
| 3 | **Test matrix** | Unit tests + integration tests minimum; e2e + perf + chaos strongly preferred for the 4 fleet-critical substrates (config, tracing, MCP-router, observability) | `pheno-ci-templates` test job |
| 4 | **Observability** | OTLP export via `pheno-tracing` (ADR-036); info-level minimum | `pheno-ci-templates` otel smoke test |
| 5 | **Coverage gate** | 80% lib/SDK, 70% framework, 60% federated service (per ADR-040) | `pheno-coverage` CLI in CI |
| 6 | **CI gate** | `pheno-ci-templates` runs the test matrix, coverage gate, OTLP smoke test | `pheno-ci-templates/cargo.yml` (or per-language equiv) |
| 7 | **Worklog v2.1** | `WORKLOG.md` includes the new `device:` field (ADR-025) | PR review |

### Per-tier application matrix

| Element | `pheno-*-lib` | `phenotype-*-sdk` | `phenotype-*-framework` | Federated service |
|---|:---:|:---:|:---:|:---:|
| 1. Spec | ✓ | ✓ | ✓ | ✓ |
| 2. Docs | ✓ | ✓ | ✓ | ✓ |
| 3. Test matrix | unit+integ | unit+integ | unit+integ+e2e | unit+integ+e2e+chaos |
| 4. Observability | ✓ | ✓ | ✓ | ✓ |
| 5. Coverage gate | 80% | 80% | 70% | 60% |
| 6. CI gate | ✓ | ✓ | ✓ | ✓ |
| 7. Worklog v2.1 | ✓ | ✓ | ✓ | ✓ |

### Enforcement

Three layers, defense in depth:

1. **PR template** (`pheno-ci-templates/pull_request_template.md`): the 7 elements
   are a checklist; author self-attests before requesting review.
2. **CI lint** (`pheno-ci-templates/quality-bar.yml`): workflow fails if any of
   the 7 elements is missing. Lints file presence (spec, docs, worklog) and
   coverage threshold (per ADR-040).
3. **Registry lint** (`phenotype-registry` per ADR-013): every entry must declare
   which tier it is and link to the 7-element checklist results from its last
   PR.

### Exceptions

- **Spike repos** (`*-spike`, `*-prototype`, `*-poc`): exempt from elements 5 and
  6; must still ship 1, 2, 3, 4, 7. Expiry: 90 days from creation, after which
  the repo graduates to a real substrate or is archived.
- **Internal-only substrate** (used by 0 external consumers): element 2 docs
  may be terse (1 paragraph, not 1 concept doc).

## Consequence

- New substrate PRs that miss any of the 7 elements fail CI, not silent at next
  71-pillar probe
- The bar is uniform across `pheno-*-lib` / `phenotype-*-sdk` / framework / service
  — no per-language carve-outs
- "Random `phenoShared`" pattern is now impossible to ship (CI blocks it)
- Existing substrate that predates the bar gets a one-time migration to compliance
  via T15 (pheno-flake refresh) and T18 (coverage pass)
- HITL-less dev: a one-line intent ("I need a `Config` struct for my service
  that reads from env and a TOML file, with a 12-factor cascade") produces a
  PR that already has spec + docs + tests + coverage + observability + CI gate,
  without the human specifying each one

## Cross-references

- ADR-023 (substrate placement authority)
- ADR-040 (coverage gates per tier)
- ADR-036 (pheno-tracing canonical — observability element)
- ADR-013 (substrate model — registry lint)
- ADR-025 (worklog v2.1 schema)
- `AGENTS.md` § "Quality bar for new substrate (Rule 3.1)" (natural-language source)
- `pheno-ci-templates/pull_request_template.md` (enforcement layer 1)
- `pheno-ci-templates/quality-bar.yml` (enforcement layer 2)
- `phenotype-registry/.github/lint.yml` (enforcement layer 3)
