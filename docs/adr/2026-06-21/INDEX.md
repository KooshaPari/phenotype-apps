# ADR Index — 2026-06-21 wave

Index of Architecture Decision Records authored as part of the **v19 — 71-Pillar Cycle 9 (P1 Reduction Wave)** (see `plans/2026-06-21-v19-71-pillar-cycle-9-p0.md`).

This wave addresses 5 P1 pillars (L50, L52, L54, L19, L53) that scored 1.5–2.0 in the v18 cycle-8 probe. Each ADR carries the prefix matching its plan track:

- T1 → ADR-079 (L50 — Vault migration roadmap)
- T2 → **ADR-080 (L52 — Encryption-at-rest mandate)** *(this ADR)*
- T3 → ADR-081 (L54 — OIDC federation reference)
- T4 → ADR-082 (L19 — Performance benchmarking infrastructure)
- T5 → ADR-083 (L53 — Pen-test + bug-bounty roadmap)

**Ordering convention:** Vault (ADR-079) precedes Encryption-at-rest (ADR-080) precedes OIDC (ADR-081) — key material must exist before it is protected; protected key material must exist before it is federated. Performance (ADR-082) and pen-test (ADR-083) are independent of the order.

---

## 2026-06-21 wave (ADR-077..083)

| ADR | Subject | Pillar | Track | Status | Cross-refs |
|---|---|---|---|---|---|
| ADR-077 | Vault migration roadmap (policy) | L50 | T1 | PROPOSED | Superseded by ADR-079 (operational plan) |
| ADR-079 | Vault migration roadmap (operational) | L50 | T1 | PROPOSED | Depends on ADR-046; precedes ADR-080 |
| **ADR-080** | **Encryption-at-rest mandate** | **L52** | **T2** | **PROPOSED** | **Depends on ADR-079 (Phase 1 GA); NIST SP 800-57, CIS v8** |
| ADR-081 | OIDC federation reference (Auth0/Okta/Keycloak) | L54 | T3 | PROPOSED | Depends on ADR-079 (Phase 2); cross-cuts ADR-046 |
| ADR-082 | Performance benchmarking infrastructure | L19 | T4 | PROPOSED | Depends on ADR-040 (test coverage gates) |
| ADR-083 | Pen-test + bug-bounty roadmap | L53 | T5 | PROPOSED | Depends on ADR-042 (security cadence); cross-references ADR-081 |

### Closure cross-reference

(none closed yet — wave is in flight)

### Verification & refresh cadence

- **Refresh cadence:** weekly Monday 09:00 PDT (per ADR-041 codification); wave INDEX refresh every Friday EOD during active v19.
- **Verification rule:** every row above must also appear in `docs/adr/INDEX.md` master index and in `AGENTS.md` § "Active ADRs".
- **Last refresh:** 2026-06-21 (v19 launch — initial INDEX created).
- **Next refresh:** 2026-06-26 (Friday EOD during v19 T1+T2 active execution window).
