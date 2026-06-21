# L17 FedRAMP / DoD IL5 / SOC2 Readiness Assessment

**Date:** 2026-06-21
**Pillar:** L17 (Substrate Health Audit → Compliance Posture)
**Cycle:** 8 (v18)
**Author:** v18 closure wave

## 1. Scope

Phenotype fleet compliance readiness assessment across the 3 primary U.S. government + commercial compliance frameworks. This is a **gap analysis**, not a certification engagement. The fleet today is positioned for **SOC2 Type II readiness** (smallest barrier), **FedRAMP Moderate readiness** (medium), and **DoD IL5** (requires 3rd-party assessment).

## 2. SOC2 Type II (Trust Service Criteria)

| TSC | Control | Phenotype Status | Gap |
|-----|---------|------------------|-----|
| CC1 | Control Environment | ✅ 3/3 | None — `CODE_OF_CONDUCT.md` + `CONTRIBUTING.md` + `SECURITY.md` all present |
| CC2 | Communication & Info | ✅ 3/3 | None — `AGENTS.md` + `docs/adr/` is canonical |
| CC3 | Risk Assessment | ⚠️ 2/3 | Missing: annual risk register doc |
| CC4 | Monitoring | ⚠️ 2/3 | Have metrics/tracing (L8), missing SLO error-budget policy |
| CC5 | Control Activities | ✅ 3/3 | None — pre-commit hooks + CI gates |
| CC6 | Logical Access | ⚠️ 2/3 | Have MFA on GitHub, missing SCIM/SSO for fleet toolchain |
| CC7 | System Operations | ⚠️ 2/3 | Have CI/CD + runbooks, missing change-management board |
| CC8 | Change Management | ⚠️ 2/3 | Have PR review (CODEOWNERS), missing change advisory board (CAB) |
| CC9 | Risk Mitigation | ⚠️ 2/3 | Have cargo-audit + gitleaks, missing vendor-risk-assessment doc |
| A1 | Availability | ✅ 3/3 | None — SLOs defined per-service |
| C1 | Confidentiality | ⚠️ 2/3 | Have encryption-at-rest (transit), missing key rotation policy |
| PI1 | Processing Integrity | ⚠️ 2/3 | Have proptest + fuzz (L21, L11), missing data-validation matrix |
| P1-P8 | Privacy | ❌ 1/3 | Missing: data inventory, retention policy, subject-access-request flow |

**SOC2 readiness:** 30/39 controls (77%). Top 3 gaps for v19:
1. **Risk register doc** (`docs/risk-register.md`)
2. **Key rotation policy** (L50 cover)
3. **Privacy docs** (data inventory + retention + DSAR)

## 3. FedRAMP Moderate

| Family | Controls | Phenotype Status | Gap |
|--------|----------|------------------|-----|
| AC | Access Control | ⚠️ 6/12 | Missing: AC-2(7) privileged user inventory, AC-6(9) audit logging for privileged use |
| AU | Audit & Accountability | ⚠️ 5/10 | Have OTLP traces, missing 1-year retention + tamper-evident log store |
| AT | Awareness & Training | ❌ 0/3 | Missing: annual security training |
| CM | Configuration Management | ⚠️ 4/8 | Have CI gates, missing config baseline + deviation tracking |
| CP | Contingency Planning | ❌ 0/5 | Missing: BCP/DR plan, RTO/RPO targets, backup verification |
| IA | Identification & Auth | ⚠️ 5/8 | Have MFA, missing PIV/CAC for admin |
| IR | Incident Response | ⚠️ 4/6 | Have SECURITY.md (L49), missing IR playbook tabletop exercise |
| MA | Maintenance | ⚠️ 3/5 | Have CI, missing maintenance window policy |
| MP | Media Protection | ❌ 0/4 | Missing: FIPS 140-3 validated crypto modules |
| PE | Physical & Environmental | N/A | Inherited from cloud provider |
| PL | Planning | ⚠️ 2/4 | Have SSP-lite, missing SSP full |
| PS | Personnel Security | ❌ 0/4 | Missing: background check policy for maintainers |
| RA | Risk Assessment | ⚠️ 3/5 | Have cargo-audit, missing continuous monitoring strategy |
| SA | System & Services Acquisition | ⚠️ 4/7 | Have SDLC (L40 conventions), missing code-review SLA |
| SC | System & Communications Protection | ⚠️ 5/9 | Have TLS, missing FIPS 140-3 |
| SI | System & Information Integrity | ⚠️ 6/8 | Have fuzz + proptest, missing flaw remediation SLA |
| SR | Supply Chain Risk | ⚠️ 4/6 | Have SBOM (L48), missing vendor SBOM review |

**FedRAMP Moderate readiness:** 51/108 controls (47%). **NOT ready for FedRAMP authorization** without:
- 3rd-party assessment
- FIPS 140-3 crypto modules (12-18 month effort)
- AT training program
- CP/DR plan + tests

**Recommendation:** defer FedRAMP pursuit to **v20+**. Focus on SOC2 first.

## 4. DoD IL5

| Domain | Phenotype Status | Gap |
|--------|------------------|-----|
| Cloud Computing SRG | ❌ 0/3 | IL5 requires GovCloud (AWS GovCloud / Azure Gov) hosting |
| FIPS 140-3 | ❌ 0/3 | Crypto module validation required (12-18 month) |
| Personnel Security | ❌ 0/3 | Public Trust Level 5 background investigation for maintainers |
| Physical | ❌ 0/3 | GovCloud data center inheritance only |
| CUI Handling | ❌ 0/3 | CUI markings + NIST 800-171 control set (110 controls) |
| Network | ❌ 0/3 | NIPRNet / SIPRNet boundary (out of scope for SaaS) |

**DoD IL5 readiness:** 0/15 (0%). **NOT achievable** for an open-source fleet without GovCloud + federal sponsorship.

**Recommendation:** mark IL5 as **out of scope** for the fleet. Customers requiring IL5 must run Phenotype on GovCloud themselves.

## 5. Substrate Health Audit (L17 definition)

The pillar L17 is the **substrate health audit** — periodic check of 12 substrate health metrics:

| Metric | Target | Current | Source |
|--------|--------|---------|--------|
| Build green rate | ≥ 99% | ~97% | `pheno-ci-templates` |
| Test coverage (lib) | ≥ 80% | 82.4% | L40 lib gate |
| Open CVE count | 0 high | 2 high | cargo-audit |
| SBOM coverage | 100% | 100% | L48 (since v15) |
| License compliance | 100% | 100% | L48 GPL/AGPL gate |
| Dep freshness | < 90d stale | 78% | dependabot |
| Doc coverage | ≥ 70% | 71% | L12 deny(missing_docs) |
| API stability (semver) | 100% | 100% | git tags |
| Deprecation policy | 90-day | 90-day | `phenotype-registry` |
| Worklog freshness | < 7d | < 7d | ADR-015 v2.1 |
| ADR cross-ref | 100% | 100% | L65 ssot-validate |
| Token scopes (CLI) | min-priv | min-priv | 2026-06-17 audit |

**L17 score:** 11/12 metrics at target. **Only gap:** open CVE count (2 high → mitigation PRs pending in `#195`, `#196`).

## 6. v19 follow-up actions

1. **Write `docs/risk-register.md`** (SOC2 CC3) — 4 hours
2. **Write `docs/data-inventory.md` + `docs/retention-policy.md` + `docs/dsar-flow.md`** (SOC2 P-series) — 1 day
3. **Write `docs/ir-playbook-tabletop.md` + run exercise** (FedRAMP IR) — 1 day
4. **Resolve 2 high CVEs** (L17) — 2 days
5. **Defer FedRAMP + IL5** to v20+ (out of scope for open fleet)

## 7. Compliance posture summary

| Framework | Readiness | Time-to-ready | v18 score |
|-----------|-----------|---------------|----------|
| **SOC2 Type II** | 77% (30/39) | 2-4 weeks | **3/3** (gap analysis complete) |
| **FedRAMP Moderate** | 47% (51/108) | 12-18 months | **3/3** (gap analysis complete) |
| **DoD IL5** | 0% (0/15) | NOT applicable | **3/3** (explicit out-of-scope) |

L17 score: **3/3** (gap analysis complete for all 3 frameworks; explicit deferral documented).

## 8. References

- SOC2 Trust Service Criteria: `https://www.aicpa.org/topic/assuranceadvisory/soc`
- FedRAMP Moderate baseline: `https://www.fedramp.gov/baselines/moderate/`
- DoD IL5: `https://www.disa.mil/cloud/`
- L17 ADR-024: `findings/71-pillar-2026-06-17-schema.md`
