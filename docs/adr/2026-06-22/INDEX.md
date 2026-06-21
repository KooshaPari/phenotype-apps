# ADR Index — 2026-06-22 wave

Wave-specific index for Architecture Decision Records authored on 2026-06-22.
This wave comprises the v20 cycle-10 P1 reduction closure (ADR-081) plus the
v21 cycle-11 P0 carry-over ADR-046 (federation mTLS architecture) authored as
net-new federation/interop work.

---

## Wave overview

| Wave | Date | Cycle | Pillar focus | Status |
|------|------|-------|--------------|--------|
| v20 cycle-10 P1 reduction | 2026-06-22 | 10 | L23, L27, L36, L38, L44 | CLOSED |
| v21 cycle-11 P0 carry-over | 2026-06-22 | 11 | L54 (federation mTLS + OIDC) | IN PROGRESS |

---

## ADR-081 — v20 Cycle 10 P1 Reduction

- **Path:** `docs/adr/2026-06-22/ADR-081-v20-cycle-10-p1-reduction.md`
- **Status:** ACCEPTED
- **Owner:** v20 orchestrator + 5 track subagents
- **Cycle:** 10 (P1 reduction; fleet mean 2.86 → 2.95 in v19, target 3.02 in v20)
- **Pillars touched:** L23 (Test-data factories), L27 (Contract tests),
  L36 (Chaos engineering depth), L38 (UX research), L44 (Performance deep-dives)

---

## ADR-046 — Federation mTLS architecture

- **Path:** `docs/adr/2026-06-22/ADR-046-federation-mtls.md`
- **Status:** ACCEPTED
- **Owner:** orch-w1-a (L5-158)
- **Cycle:** v21 cycle-11 P0 (carry-over from v20 cycle-10 P1 net-new
  federation/interop track)
- **Pillars touched:** L54 (Federation mTLS + OIDC), L48 (Dependency auditing —
  transitive trust)
- **Scope:** cross-org service-to-service authentication via mTLS with 90-day
  maximum leaf lifetime, federation root CA with explicit allow-list, SHA-256
  SPKI key pinning, OCSP/CRL revocation checks.
- **Companions (forthcoming):**
  - ADR-046b — OIDC federation reference implementation
    (`pheno-context/src/oidc.rs` Rust crate shape; see ADR-079)
  - ADR-046c — Peer-to-peer federation topology
    (hub-and-spoke vs mesh failover semantics)

### Key policy decisions

| Decision | Value | Rationale |
|----------|-------|-----------|
| Maximum leaf cert lifetime | **90 days** | NIST SP 800-57 §5.6 short-lived credentials |
| Re-issue trigger | **60 days** | 2/3 lifetime; 30-day overlap window |
| Pin algorithm | **SHA-256 of SPKI** | Defense-in-depth vs CA compromise |
| Pin rotation cadence | **180 days** | Decoupled from cert rotation (90d) |
| Revocation check | **OCSP + CRL fallback** | 2-second timeout; fail-closed |
| Federation root CA | **Phenotype governance circle** | Single well-known root; explicit allow-list |

### Standards adopted

- OWASP ASVS v4.0.3 §6.4 (Cryptographic Storage & Transport)
- NIST SP 800-57 Part 1 Rev. 5 §5.6 (Cryptographic Periods)
- NIST SP 800-52 Rev. 2 §3.5 (Certificate Validation)
- CNCF Cloud Native Security Whitepaper §4.3 / §4.4
- SPIFFE Trust Domain Federation spec
- Mozilla TLS Config v5 (Intermediate profile)

---

## Companion ADRs in adjacent waves

| ADR | Path | Wave | Relation |
|-----|------|------|----------|
| ADR-046 (2026-06-18) | `docs/adr/2026-06-18/ADR-046-federation-mtls-oidc.md` (catalog entry only) | 2026-06-18 | Catalog row; superseded by this ADR as canonical content |
| ADR-079 (2026-06-21) | `docs/adr/2026-06-21/ADR-079-oidc-federation-reference.md` | 2026-06-21 | Companion ADR-046b — Rust crate shape for OIDC |

---

## Refresh cadence

- Refreshed: 2026-06-22 (wave authoring)
- Next refresh: 2026-06-29 (per ADR-041 weekly Monday 09:00 PDT cadence)
- On ADR-046b / ADR-046c authoring: this INDEX must be updated to list the
  companion ADRs and revise the "Companions (forthcoming)" section.