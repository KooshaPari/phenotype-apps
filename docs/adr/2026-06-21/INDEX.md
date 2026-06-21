# 2026-06-21 Wave — ADR Index

This is the wave-specific INDEX for the 2026-06-21 v19 cycle-9 P1 reduction wave.

**Wave:** v19 cycle 9 (P1 reduction)
**Source plan:** `plans/2026-06-21-v19-71-pillar-cycle-9-p0.md`
**ADRs in this wave:** 5 (ADR-077 through ADR-081)
**Last refresh:** 2026-06-21 (post-T5 closure, L5-155)

---

## ADR table

| ADR  | Subject                                                | Pillar | Track | Status   | Author         |
|------|--------------------------------------------------------|--------|-------|----------|----------------|
| ADR-077 | Vault migration roadmap (L50 implementation plan)   | L50    | T1    | PROPOSED | orch-v19       |
| ADR-078 | Encryption-at-rest mandate (L52)                   | L52    | T2    | ACCEPTED | orch-v19       |
| ADR-079 | OIDC federation reference (L54)                    | L54    | T3    | PROPOSED | orch-v19       |
| ADR-080 | (reserved — see plan)                              | —      | —     | —        | —              |
| ADR-081 | L53 pen-test + bug-bounty roadmap (cycle 9)        | L53    | T5    | PROPOSED | orch-v19 forge subagent (L5-155) |

---

## Per-ADR details

### ADR-077 — Vault migration roadmap

- **Path:** `docs/adr/2026-06-21/ADR-077-vault-migration-roadmap.md`
- **Pillar:** L50 (Secrets Management)
- **Companion to:** `docs/adr/2026-06-15/ADR-077-secrets-vault-migration-roadmap.md` (policy)
- **Status:** PROPOSED (v19 T1, 2026-06-21)
- **Scope:** HashiCorp Vault 1.16.x, 4-phase 12-week migration, 3 FTE

### ADR-078 — Encryption-at-rest mandate

- **Path:** `docs/adr/2026-06-21/ADR-078-encryption-at-rest-mandate.md`
- **Pillar:** L52 (OWASP ASVS V2 + V8, ISO 27001 A.10, NIST SSDF PW.4)
- **Status:** ACCEPTED (2026-06-21)
- **Scope:** `zeroize::Zeroize` for all secret-holding structs; cargo deny rules; devcontainer + EBS + S3 + K8s encryption enforcement

### ADR-079 — OIDC federation reference

- **Path:** `docs/adr/2026-06-21/ADR-079-oidc-federation-reference.md`
- **Pillar:** L54 (Federated Identity)
- **Status:** PROPOSED (v19 T3, 2026-06-21)
- **Scope:** OIDC client reference impl supporting Auth0 + Okta + Keycloak; JWKS caching; token-claim → PhenoContext mapping

### ADR-081 — L53 pen-test + bug-bounty roadmap

- **Path:** `docs/adr/2026-06-21/ADR-081-pentest-roadmap.md`
- **Pillar:** L53 (Penetration Testing) — currently 1/3 per cycle-8 probe
- **Status:** PROPOSED (v19 T5, 2026-06-21)
- **Author:** orch-v19 forge subagent (L5-155)
- **Worklog:** `worklogs/L5-155-v19-T5-pentest-roadmap-2026-06-21.json`
- **Scope:** 12-month rolling program — 3 internal (quarterly) + 1 external (NCC Group annual); 4 attack surfaces (LLM prompt injection, tool-call injection via mcp-router, registry poisoning, Bifrost plugin deserialization); 4 in-scope substrate repos (`pheno-mcp-router`, `pheno-port-adapter`, `phenotype-router`, `phenotype-registry`); tooling stack (semgrep + snyk + nuclei + custom prompt-injection fuzzer + custom Bifrost plugin fuzzer); pass/fail bar (zero Critical, ≤2 High); remediation SLA (Critical 24h, High 7d, Medium 30d, Low 90d); budget $65K/year
- **Standards cited:** NIST 800-115 §4 + §6.5, OWASP ASVS 4.0.3 V5.5 / V6 / V8 / V11.1 / V12.1 / V13.1 / V14.2-3, OWASP Top 10 for LLM Applications (2025) LLM01 / LLM02 / LLM07
- **Sub-tasks T5.1-T5.4:** prompt-injection fuzzer, semgrep + nuclei rulesets, Bifrost plugin fuzzer, first internal cycle (A+C surfaces, 2026-07-10)

---

## Closure cross-reference

No ADRs in this wave are closed as of 2026-06-21.

## Verification & refresh cadence

- **Refresh cadence:** weekly Monday 09:00 PDT (per ADR-041 codification)
- **Verification rule:** every ADR row in this INDEX must resolve to a file at the documented path
- **Last refresh:** 2026-06-21 (post-T5 closure, L5-155)
