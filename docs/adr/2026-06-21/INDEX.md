# ADR Index — 2026-06-21 wave (v19 cycle 9, P1 reduction)

Wave-specific index for Architecture Decision Records authored on 2026-06-21.
This wave contains the **v19 cycle-9 P1 reduction ADRs** — security & performance deepening tracks.

**Numbering note (v19 wave):** Originally planned as ADR-077..080 in
`plans/2026-06-21-v19-71-pillar-cycle-9-p0.md` §2. ADR-077 was already taken by
`2026-06-20/ADR-077-slsa-l3-provenance.md` and ADR-078 by
`2026-06-20/ADR-078-cosign-keyless-signing.md`. This wave therefore uses the
**next free numbers** in sequence (ADR-079, ADR-080, ADR-081, ADR-082).
Tracks T2-T5 in the v19 plan must be renumbered accordingly when authored.

| Plan track | Original ADR # | Effective ADR # | Topic |
|---|---|---|---|
| T1 | 077 | **079** | Vault secrets migration roadmap (L50) — this PR |
| T2 | 078 | **080** | Encryption-at-rest mandate (L52) |
| T3 | 079 | **081** | OIDC federation reference impl (L54) |
| T4 | 080 | **082** | Performance benchmarking infrastructure (L19) |
| T5 | — | **083** | Pen-test + bug-bounty roadmap (L53) |

---

## ADR-079 — Secrets-Vault Migration Roadmap (L50)

- **Path:** `docs/adr/2026-06-21/ADR-079-vault-migration-roadmap.md`
- **Status:** ACCEPTED (v19 T1, this PR)
- **Owner:** orch-v19 + forge subagent (L5-153) on macbook
- **Pillar:** L50 (Secrets Management), L52 cross-reference, ADR-046 compat
- **Companion artifacts:**
  - `docs/architecture/secrets-management-convention.md` (path conventions + OIDC binding + rotation + runbook)
  - `scripts/vault_smoke.sh` (executable; reachability + write + read + rotation smoke)
- **Closes:** L50 1.5 → 2.0 (Adequate)
- **Cross-refs:** ADR-046 (federation mTLS+OIDC), ADR-012 (pheno-tracing), ADR-026 (Factory AI), ADR-027 (LFS), ADR-042 (security cadence), ADR-077/2026-06-20 (SLSA L3), ADR-078/2026-06-20 (cosign), ADR-015 v2.1 (worklog schema)

---

## ADR-080 (planned, T2) — Encryption-at-Rest Mandate (L52)

- **Path:** `docs/adr/2026-06-21/ADR-080-encryption-at-rest-mandate.md` *(TBD)*
- **Status:** PLANNED (v19 T2)
- **Owner:** orch-v19
- **Target close:** L52 1.5 → 2.0
- **Depends on:** ADR-079 (this PR — Vault provides encryption-at-rest for transit storage)

## ADR-081 (planned, T3) — OIDC Federation Reference Implementation (L54)

- **Path:** `docs/adr/2026-06-21/ADR-081-oidc-federation-reference.md` *(TBD)*
- **Status:** PLANNED (v19 T3)
- **Owner:** orch-v19 + subagent on heavy-runner
- **Target close:** L54 2.0 → 2.5
- **Depends on:** ADR-079 + ADR-080; uses Vault as OIDC issuer (per ADR-046)

## ADR-082 (planned, T4) — Performance Benchmarking Infrastructure (L19)

- **Path:** `docs/adr/2026-06-21/ADR-082-perf-benchmarking-infra.md` *(TBD)*
- **Status:** PLANNED (v19 T4)
- **Owner:** subagent on heavy-runner (macbook unsafe — >10min cargo builds)
- **Target close:** L19 2.0 → 2.5
- **Depends on:** None

## ADR-083 (planned, T5) — Pen-Test + Bug-Bounty Roadmap (L53)

- **Path:** `docs/adr/2026-06-21/ADR-083-pen-test-bug-bounty-roadmap.md` *(TBD)*
- **Status:** PLANNED (v19 T5)
- **Owner:** orch-v19
- **Target close:** L53 2.0 → 2.5
- **Depends on:** ADR-081 (test harness uses OIDC client)

---

## Refresh cadence

- Created: 2026-06-21 (v19 T1 ship, this PR)
- Refreshed: weekly Monday 09:00 PDT per ADR-041
- Next refresh: 2026-06-28 (cycle-9 mid-refresh — L50 should hit 2.0)