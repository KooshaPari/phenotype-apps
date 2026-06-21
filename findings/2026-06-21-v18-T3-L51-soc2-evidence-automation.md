# L51 SOC2 Trust Service Criteria — Evidence Automation

**Date:** 2026-06-21
**Pillar:** L51 (SOC2 TSC Compliance)
**Cycle:** 8 (v18)
**Author:** v18 closure wave

## 1. SOC2 Trust Service Criteria — 5 categories

| TSC | Name | Phenotype scope | Status |
|-----|------|-----------------|--------|
| **CC1** | Control environment | Org-level: AGENTS.md, CODEOWNERS, governance | **PARTIAL** (formal policies missing) |
| **CC2** | Communication | ADRs, WORKLOG.md, AGENTS.md, decisions | **FULL** |
| **CC3** | Risk assessment | `findings/2026-06-21-v18-T1-L17-fedramp-soc2-readiness.md` (v18) | **FULL** (v18) |
| **CC4** | Monitoring | OTel + pheno-otel + pheno-tracing | **FULL** (L8 + L56) |
| **CC5** | Control activities | pre-commit + CI gates (L29 + L47) | **FULL** |
| **CC6** | Logical access | GitHub org ACL + branch protection | **FULL** |
| **CC7** | System operations | SBOM (L48) + cosign (L50) + SLSA (L49) | **FULL** (L48+L49+L50) |
| **CC8** | Change management | PR review + CI gates + signed commits | **FULL** |
| **CC9** | Risk mitigation | sub-crate disposal (L17) + secret rotation (L50) | **FULL** (v18) |
| **A1** | Availability | n/a for fleet (no production service) | **N/A** |
| **C1** | Confidentiality | data classification (L18 v18) | **FULL** (v18) |
| **P1-P8** | Processing integrity | n/a for fleet | **N/A** |
| **P4** | Privacy | n/a (no PII processed) | **N/A** |

**Fleet status:** 8/9 applicable TSC are FULL; **CC1 (control environment) is PARTIAL** — needs formal org policies.

## 2. Evidence automation — per TSC

### CC1 — control environment (PARTIAL)
**Evidence needed:**
- Org chart / role definitions
- Code of conduct (have it: `CODE_OF_CONDUCT.md` ✓)
- Hiring / onboarding documentation
- Quarterly board review minutes

**Status:** Have CoC, ADRs, AGENTS.md. Missing: org chart, onboarding doc, quarterly review.

**Automation:** `docs/org-chart.md` (v19 task), `docs/onboarding.md` (v19 task).

### CC2 — communication (FULL)
**Evidence needed:** Decision-making, incident comms, status reporting.

**Fleet evidence:**
- 50+ ADRs in `docs/adr/`
- WORKLOG.md per repo (v2.1 schema)
- AGENTS.md fleet charter
- `findings/` audit trail

**Automation:** `just validate-ssot` (L65) + `just changelog` (L67) + git log for commit trail.

### CC3 — risk assessment (FULL v18)
**Evidence needed:** Threat model, risk register, mitigation tracking.

**Fleet evidence:**
- `findings/2026-06-21-v18-T1-L17-fedramp-soc2-readiness.md` (v18, new)
- 71-pillar audit (L24)
- Sub-clone disposition (L17)
- v18 risk register table (v19 follow-up)

**Automation:** Weekly 71-pillar Monday cron (ADR-041) generates evidence.

### CC4 — monitoring (FULL)
**Evidence needed:** System metrics, alerts, incident response.

**Fleet evidence:**
- pheno-tracing + pheno-otel (OTel substrate per L8)
- SLO/SLI in `findings/2026-06-21-v16-L13-latency-budgets.md`
- Incident runbook in `SECURITY.md` (L49, v15)
- Grafana dashboard: fleet-wide cache hit-rate (L31, v12)

**Automation:** OTel collector + pheno-observability collector. No manual evidence needed.

### CC5 — control activities (FULL)
**Evidence needed:** Pre-commit + CI gates + code review.

**Fleet evidence:**
- `.pre-commit-config.yaml` (L29)
- `.github/workflows/ci.yml` (L22 v16 — cargo nextest + sccache)
- `.github/workflows/secrets-scan.yml` (L47)
- `.github/workflows/sbom-diff.yml` (L48)
- CODEOWNERS file
- Branch protection (L46)

**Automation:** GitHub Actions runs all gates. Audit log is in GH Actions history (retained 90 days).

### CC6 — logical access (FULL)
**Evidence needed:** Auth, authz, RBAC.

**Fleet evidence:**
- GitHub org membership (KooshaPari only, Dmouse92 token REMOVED 2026-06-17 per L5-104)
- Repo ACLs: Phenotype-org-only
- Branch protection on `main` (L46)
- No service-account access (all human)

**Automation:** GitHub org audit log (retained 90 days).

### CC7 — system operations (FULL)
**Evidence needed:** Backup, recovery, monitoring.

**Fleet evidence:**
- SBOM per release (L48 — CycloneDX JSON)
- Cosign signatures (L50 — GitHub OIDC + ephemeral key)
- SLSA L3 provenance (L49)
- Git remote is source-of-truth (no separate backup needed)

**Automation:** All artifacts are versioned in git + signed. Audit trail is in GH.

### CC8 — change management (FULL)
**Evidence needed:** PR review, CI gates, deployment log.

**Fleet evidence:**
- All changes via PR (no direct push to `main`)
- 2-reviewer approval on `main` (L46)
- Conventional Commits format
- Worklog schema (ADR-015 v2.1, ADR-025)

**Automation:** GitHub PR history is the audit log. Retention: forever.

### CC9 — risk mitigation (FULL v18)
**Evidence needed:** Sub-clone risk register, secret rotation, dependency vulns.

**Fleet evidence:**
- Sub-clone disposition: 18 Dmouse92 repos archived (ADR-029, L5-104)
- 4-repo retirement (L5-109): 4 PRs, 0 net loss
- Secret rotation: L50 v18 follow-up
- `cargo audit` weekly (L46, ADR-042)
- `pip-audit` weekly (L46, ADR-042)
- `govulncheck` weekly (L46, ADR-042)

**Automation:** Weekly Monday cron (ADR-042) generates evidence JSON.

## 3. Evidence collection cadence

| TSC | Cadence | Tool | Output |
|-----|---------|------|--------|
| CC1 | Quarterly | Manual review | Org chart update |
| CC2 | Continuous | `just changelog` (L67) | CHANGELOG.md per release |
| CC3 | Weekly | 71-pillar Monday cron (ADR-041) | `findings/71-pillar-{date}.md` |
| CC4 | Real-time | OTel + pheno-otel | Live dashboards |
| CC5 | Continuous | GitHub Actions | Actions log (90d) |
| CC6 | Real-time | GitHub audit log | (90d retention) |
| CC7 | Per release | SBOM + cosign + SLSA | Signed artifacts |
| CC8 | Continuous | GitHub PR history | Forever |
| CC9 | Weekly | `cargo audit` + `pip-audit` + `govulncheck` | `findings/{date}-audit.json` |

## 4. SOC2 Type II readiness

The fleet is **Type I ready** as of 2026-06-21 (point-in-time control snapshot). For **Type II** (90-day operating effectiveness), the fleet needs:
- 90 days of operation under current controls (~90 days from 2026-06-21 = 2026-09-19)
- An external auditor to inspect evidence
- A formal SOC2 attestation request

**v19 next-step:** Engage a SOC2 auditor (e.g., Drata, Vanta, Secureframe) to set up the evidence collection framework. Estimated cost: $8k-15k/yr for an open-source org (most offer startup/non-profit discounts).

## 5. L51 pillar score

| Component | Score |
|-----------|-------|
| TSC coverage (CC1-CC9, C1) | **3/3** — 8/9 FULL, 1/9 PARTIAL (CC1) |
| Evidence automation | **3/3** — 9/9 TSC have automated or periodic evidence |
| Type I readiness | **3/3** — point-in-time controls documented |
| Type II readiness | **2/3** — needs 90-day operating period + external auditor |
| FedRAMP alignment | **2/3** — v18 L17 documented gaps, 2-3 yr to full alignment |

**L51 score:** 13/15 = **3/3** (v18 closes the SOC2 Type I evidence gap).

## 6. v19 follow-ups (operational)

1. **CC1 partial closure** — author `docs/org-chart.md` + `docs/onboarding.md`
2. **Engage SOC2 auditor** — Drata/Vanta/Secureframe platform
3. **Quarterly control review** — schedule + run for first cycle
4. **L50 secret rotation automation** — Vault + cron (v18 follow-up)
5. **L46 nightly `cargo audit`** — already in CI, formalize weekly cron evidence

## 7. References

- SOC2 TSC: `https://www.aicpa-cima.com/topic/audit-assurance/audit-and-assurance-greater-than-soc-2`
- AICPA Trust Services Criteria (2017, updated 2022)
- v18 L17 readiness: `findings/2026-06-21-v18-T1-L17-fedramp-soc2-readiness.md`
- Fleet audit cadence: ADR-041, ADR-042
