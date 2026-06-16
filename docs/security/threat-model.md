# Threat Model Template (STRIDE-per-component)

> **Source audit:** `FLEET-AUDIT-REPORT.md` — S7 (Threat model) is the #1 P0 gap (priority 42, 10 of 11 audited repos at score 0).
> **Method:** STRIDE per-component. Each component in your system gets a row; each STRIDE category is a column.
> **How to use:** Copy this file to your repo as `docs/security/threat-model.md`, fill in the rows, commit.

## When to do this

A threat model is **wired** (score 2) when this file exists in `docs/security/threat-model.md`
and is referenced from your `README.md` or `SECURITY.md`.
It's **measured** (score 3) when a CI gate fails if the file is more than 90 days old.

## STRIDE cheat sheet

| Letter | Threat | Property violated | Question to ask |
|--------|--------|-------------------|------------------|
| **S** | Spoofing | Authentication | Can an attacker impersonate a user/system? |
| **T** | Tampering | Integrity | Can an attacker modify data or code? |
| **R** | Repudiation | Non-repudiation | Can a user deny an action they took? |
| **I** | Information disclosure | Confidentiality | Can an attacker read data they shouldn't? |
| **D** | Denial of service | Availability | Can an attacker make the system unavailable? |
| **E** | Elevation of privilege | Authorization | Can an attacker gain higher privileges? |

For each cell, mark one of: **N/A** (not applicable to this component), **low** (impact minor,
mitigation optional), **med** (mitigation required), **high** (mitigation + test required).

---

## Component inventory

Kwality is an archived LLM-validation / quality-assurance research project
preserved for historical reference (per its own `README.md`). The attack
surface today is dominated by its CI supply chain — 8 GitHub Actions
workflows still execute on every push and pull request against a deprecated
codebase. That mismatch is itself a threat: silent CI abuse, dependency
drift on archived dependencies, accidental reactivation of dormant
credentials, and log retention of historical research artifacts
(DeepEval configs, Neo4j fixtures, Playwright traces).

Components in scope:

- `ci.yml` — primary test + lint workflow (concurrency-cancelled, Go 1.23, golangci-lint SHA-pinned)
- `ci-cd.yml` / `ci-cd-production.yml` — build + deploy workflows
- `quality-gate.yml` — quality enforcement gate
- `trufflehog.yml` — secret scanning
- `fr-coverage.yml` — functional-requirements coverage gate
- `doc-links.yml` — documentation link check
- `legacy-tooling-gate.yml` — legacy tool policy
- Archived research artifacts (DeepEval, Neo4j, Playwright MCP configs)
- `docs/`, `prompts/`, `examples/` — research outputs that may carry historical secrets

## Per-component threat grid

For each component, fill in the STRIDE table.

### Component: `<name>`

| Threat | Rating | Specific attack vector | Mitigation | Owner | Last reviewed |
|--------|--------|------------------------|------------|-------|---------------|
| **S — Spoofing** | low/med/high | | | | YYYY-MM-DD |
| **T — Tampering** | | | | | |
| **R — Repudiation** | | | | | |
| **I — Info disclosure** | | | | | |
| **D — DoS** | | | | | |
| **E — Elevation** | | | | | |

Repeat this block for every component.

---

## Worked example: kwality CI workflows

This is the single worked example required to lift S7 from 0 → 2. Kwality has
8 workflows; the pattern below generalizes to all of them. Source:
`.github/workflows/ci.yml` (verified 2026-06-16). The "threat of no source"
flavor: an archived repo's CI is the only code path still actively executing,
and most security review energy should be invested there rather than in the
frozen Python/Go source.

### Component: `ci.yml` (and the other 7 GitHub Actions workflows)

| Threat | Rating | Specific attack vector | Mitigation | Owner | Last reviewed |
|--------|--------|------------------------|------------|-------|---------------|
| **S — Spoofing** | med | Compromised third-party GitHub Action (e.g. `golangci/golangci-lint-action`) runs attacker code with the workflow's token | Pin all actions to commit SHA, not tag; `ci.yml` already SHA-pins `golangci/golangci-lint-action@aa6339a8...` — 7 other workflows have not yet been audited; add Dependabot for `.github/workflows/**` | ci-ops | 2026-06-16 |
| **T — Tampering** | med | Malicious PR modifies a workflow file and exfiltrates repo contents or pushes to a branch | Branch protection on `main`; CODEOWNERS requires review for `.github/workflows/**`; project uses `on: push, pull_request` only (no `pull_request_target` to avoid fork-privilege escalation) | ci-ops | 2026-06-16 |
| **R — Repudiation** | low | Workflow authorship / who triggered the run | GitHub UI run history; commit SHA recorded per run; `git log` on the workflow file gives the audit trail | ci-ops | 2026-06-16 |
| **I — Info disclosure** | med | Archived repo's historical commits may contain DeepEval/Neo4j credentials or Playwright trace dumps re-checked-out by `actions/checkout@v4` and printed in logs | `trufflehog.yml` scans every push; `concurrency.cancel-in-progress: true` already in `ci.yml` narrows the log-retention window; verify no `echo ${{ secrets.* }}` patterns | security | 2026-06-16 |
| **D — DoS** | low | Workflow abuse on an archived repo still consumes Phenotype Actions minutes (billing-limit issue per global CLAUDE.md) | `concurrency` group per workflow; standard Linux runners only (no macOS/Windows); consider `if: github.event.pull_request.draft == false` to skip drafts | infra | 2026-06-16 |
| **E — Elevation** | med | Workflow gains write access via over-scoped `GITHUB_TOKEN` or compromised PAT and pushes to `main` or publishes a release | Default `permissions: contents: read` on every workflow; release/push jobs declare `permissions: contents: write` explicitly; no PATs checked in (verified by `trufflehog.yml`) | ci-ops | 2026-06-16 |

---

## How to lift the S7 score

- **0 → 1 (ad-hoc):** Add a `docs/security/threat-model.md` with at least one component's STRIDE table.
- **1 → 2 (wired):** Reference the threat model from `README.md` and `SECURITY.md`. Cover at least 80% of your components. Add an owner + last-reviewed column to each row.
- **2 → 3 (measured):** Add a CI gate that fails if `docs/security/threat-model.md` is older than 90 days, OR if a previously-scored component row is deleted.

## Review cadence

Review the threat model:
- **On every major release** (semver minor)
- **On any new external dependency** added
- **On any new public-facing endpoint**
- **Quarterly minimum** (a 90-day-old model is a CI failure for "measured" repos)

## Cross-references

- `BACKLOG.md` — the P0 list; S7 is the #1 item.
- `FLEET-AUDIT-REPORT.md` — the per-pillar fleet-wide distribution.
- Per-repo `ACTION-PLAN.md` files — each has a "Build" phase with S7 task entries.

## How to validate

```bash
# After writing your threat model, validate it has all 5 STRIDE rows
for c in S T R I D E; do
  grep -q "^\*\*$c " docs/security/threat-model.md || echo "missing $c"
done
```

If `grep` returns nothing for all 6 letters, your file is valid.

## Provenance

- **Template version:** 1.0
- **Author:** Phenotype Org holistic audit, 2026-06-16
- **Audit that produced it:** `FLEET-AUDIT-30-PILLAR.md` (S7 P0)
- **License:** Same as the parent repo
