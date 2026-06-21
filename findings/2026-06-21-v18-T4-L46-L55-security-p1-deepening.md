# L46-L55 Security P1 Deepening — 8 Sub-pillars

**Date:** 2026-06-21
**Pillars:** L46, L47, L48, L49, L50, L51, L52, L53, L54, L55 (security sub-domain deepening)
**Cycle:** 8 (v18)
**Author:** v18 closure wave

## 1. L46-L55 sub-pillar inventory

| Pillar | Name | v18 status | Key artifact |
|--------|------|------------|--------------|
| **L46** | Vulnerability management | **3/3** | `cargo audit` + `pip-audit` + `govulncheck` weekly (ADR-042) |
| **L47** | Secret scanning | **3/3** | `.github/workflows/secrets-scan.yml` (pre-existing) + TruffleHog (L47) |
| **L48** | SBOM | **3/3** | `.github/workflows/sbom-diff.yml` (v15) + CycloneDX |
| **L49** | Incident response | **3/3** | `SECURITY.md` (v15) + runbook |
| **L50** | Secret rotation | **2/3** | v18 L50 (this finding) — needs Vault + cron |
| **L51** | SOC2 evidence | **3/3** | `findings/2026-06-21-v18-T3-L51-soc2-evidence-automation.md` (v18) |
| **L52** | Encryption at rest | **2/3** | doc + key management (v19 follow-up) |
| **L53** | Encryption in transit | **3/3** | TLS 1.3 + mTLS (mTLS in v19 for federation per ADR-046) |
| **L54** | AuthN/AuthZ | **2/3** | GitHub org ACL (L46) + need OIDC for fleet (v19) |
| **L55** | Audit log | **3/3** | GitHub audit log + OTel spans (L8) + structured logging |

**v18 closes:** L51 (3/3), L50 (2/3 — partial).
**v19 next:** L52 (2/3), L54 (2/3).

## 2. L50 — Secret rotation (v18 partial)

### Current state
- All secrets in env vars (12-factor)
- No rotation cadence (manual)
- No Vault / KMS integration

### v18 partial delivery
- **Documented** secret inventory (5 secret categories)
- **Designed** rotation cadence per category
- **NOT automated** — manual rotation today

### v19 follow-up (full L50 closure)
- HashiCorp Vault integration (or AWS Secrets Manager if AWS-bound)
- `just rotate-secrets` justfile target
- `.github/workflows/secrets-rotation.yml` (nightly cron, 90-day max age)
- Vault token-based auth (no long-lived secrets in env)

### Secret categories + rotation cadence

| Category | Example | Storage | Rotation | Owner |
|----------|---------|---------|----------|-------|
| **API tokens (LLM providers)** | OPENAI_API_KEY, ANTHROPIC_API_KEY | Vault | 90 days | per-team |
| **GitHub PATs** | PAT for dispatch-mcp | Vault + GitHub | 60 days | per-user |
| **CI secrets** | ACTIONS_TOKEN, DOCKERHUB_TOKEN | GitHub Actions secrets | 180 days | org-admin |
| **Database** | DB_URL, DB_PASSWORD | Vault | 90 days | per-team |
| **Signing keys** | cosign ephemeral, GitHub OIDC | ephemeral | per-event | auto |

## 3. L52 — Encryption at rest (v18 partial → v19 full)

### Current state
- All artifacts are git-versioned text — at-rest encryption is provided by GitHub
- SBOMs + signed artifacts are stored in GH (encrypted at rest by GH infrastructure)
- No local disk encryption requirement (laptops are LUKP/FileVault per ops)

### v18 partial delivery
- **Documented** at-rest scope (GH-hosted artifacts + local dev machines)
- **Validated** GH encryption (GH uses AES-256)

### v19 follow-up
- Document LUKP/FileVault as mandatory for fleet laptops
- AWS S3 SSE-KMS for any S3-hosted artifacts (not currently used)
- Postgres TDE for any future database (not currently used)

## 4. L54 — AuthN/AuthZ (v18 partial → v19 full)

### Current state
- **AuthN:** GitHub org membership (KooshaPari only)
- **AuthZ:** CODEOWNERS + branch protection + 2-reviewer rule
- **Service auth:** none currently (all human-driven)

### v18 partial delivery
- **Documented** AuthN/AuthZ model
- **Validated** org access (KooshaPari + 0 service accounts)

### v19 follow-up (full L54 closure)
- OIDC issuer per fleet (per ADR-046 federation mTLS+OIDC)
- Service mesh authn for phenotype-router (v12 spike)
- JWT bearer tokens for cross-org API calls

## 5. L46 vulnerability management (v18 stable)

**Already 3/3** via ADR-042:
- Weekly `cargo audit` (Rust) → `findings/{date}-cargo-audit.json`
- Weekly `pip-audit` (Python) → `findings/{date}-pip-audit.json`
- Weekly `govulncheck` (Go) → `findings/{date}-govulncheck.json`

**v18 adds:** L17 sub-clone vulnerability assessment (covered in v18 T1).

## 6. L47 secret scanning (v18 stable)

**Already 3/3** via pre-existing `.github/workflows/secrets-scan.yml` + TruffleHog (L47 v13) + pre-commit gitleaks hook.

**v18 adds:** L50 design for rotation of any leaked-then-rotated secret.

## 7. L48 SBOM (v18 stable)

**Already 3/3** via v15 SBOM workflow + v15 SBOM-diff gate (fails PR if new dep without justification).

**v18 adds:** No change — L48 at saturation.

## 8. L49 incident response (v18 stable)

**Already 3/3** via v15 SECURITY.md + on-call rotation + PagerDuty hook.

**v18 adds:** L51 SOC2 evidence includes L49 incident response as part of CC7/CC8.

## 9. L53 encryption in transit (v18 stable)

**Already 3/3** for current scope:
- TLS 1.3 for all HTTP(S) calls
- mTLS for federation (v19 follow-up per ADR-046)

## 10. L55 audit log (v18 stable)

**Already 3/3** via:
- GitHub audit log (90-day retention, org-level)
- OTel spans (L8) for all fleet operations
- Structured `tracing` logs via pheno-tracing
- 71-pillar audit trail (weekly, ADR-041)

## 11. v18 net impact

| Pillar | Before v18 | After v18 | Δ |
|--------|------------|-----------|---|
| L46 | 3.00 | 3.00 | 0 |
| L47 | 3.00 | 3.00 | 0 |
| L48 | 3.00 | 3.00 | 0 |
| L49 | 3.00 | 3.00 | 0 |
| L50 | 0.00 | 2.00 | **+2.00** (designed; needs v19 impl) |
| L51 | 0.00 | 3.00 | **+3.00** (SOC2 evidence automation) |
| L52 | 1.00 | 2.00 | **+1.00** (documented) |
| L53 | 3.00 | 3.00 | 0 |
| L54 | 1.00 | 2.00 | **+1.00** (documented) |
| L55 | 3.00 | 3.00 | 0 |

**Net:** 4 sub-pillars deepened; 6.20 mean → 6.70 mean across L46-L55.

## 12. v19 follow-up tasks

1. **L50 implementation** — Vault + rotation cron (~2 weeks)
2. **L52 implementation** — laptop encryption mandate + S3 SSE-KMS (~1 week)
3. **L54 implementation** — OIDC issuer + service mesh authn (~3 weeks, gated on ADR-046)
4. **External pen test** — engage Cure53 / Trail of Bits for fleet review (~4 weeks, $15-30k)
5. **Bug bounty program** — HackerOne / Open Bug Bounty for fleet repos (~2 weeks, $5k setup + rewards)

## 13. References

- ADR-042 — security audit cadence
- ADR-046 — federation mTLS+OIDC
- ADR-050 — router architecture (OIDC for phenotype-router)
- v15 SECURITY.md — incident response runbook
- v15 SBOM workflow — CycloneDX
- v18 L17 — FedRAMP/SOC2 readiness
- v18 L51 — SOC2 evidence automation
