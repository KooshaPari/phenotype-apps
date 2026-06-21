# ADR Index — 2026-06-22 wave

Wave-specific index for Architecture Decision Records authored on 2026-06-22.
This wave comprises the v20 cycle-10 P1 reduction closure (ADR-081) plus the
v21 cycle-11 P0 carry-over companion-ADR triplet (ADR-046 mTLS architecture,
ADR-046b OIDC token exchange, ADR-046c p2p topology forthcoming) authored as
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
- **Companions:**
  - ADR-046b — OIDC token exchange reference architecture
    (ACCEPTED, this PR) — application-layer delegated-authentication contract
  - ADR-046c — Peer-to-peer federation topology
    (forthcoming) — hub-and-spoke vs mesh failover semantics

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

## ADR-046b — Federation OIDC token exchange reference architecture

- **Path:** `docs/adr/2026-06-22/ADR-046b-federation-oidc.md`
- **Status:** ACCEPTED
- **Owner:** orch-w1-b (L5-159)
- **Cycle:** v21 cycle-11 P0 (2nd companion to ADR-046; carry-over from v20
  cycle-10 P1 net-new federation/interop track)
- **Pillars touched:** L54 (Federation mTLS + OIDC), L49 (Identity —
  delegation/impersonation)
- **Scope:** cross-org delegated-authentication protocol on top of the ADR-046
  mTLS substrate. Adopts RFC 8693 OAuth 2.0 Token Exchange for on-behalf-of
  delegation, act-as impersonation, and step-up re-authentication. Defines
  identity-provider trust establishment (two-phase governance-gated handshake),
  audience/scope claim enforcement, refresh-token rotation policy, JWKS key
  discovery and caching, clock-skew tolerance, and online/local revocation
  mechanisms.
- **Companion relationship:**
  - **To ADR-046:** transport substrate — every RFC 8693 exchange traverses an
    mTLS handshake.
  - **To ADR-079:** composition layer — ADR-079 covers inbound JWT verification;
    ADR-046b covers outbound token acquisition.
  - **To ADR-046c (forthcoming):** topology overlay — ADR-046c decides
    hub-and-spoke vs mesh; ADR-046b runs unchanged on either topology.

### Key policy decisions

| Decision | Value | Rationale |
|----------|-------|-----------|
| Token exchange protocol | **RFC 8693** (OAuth 2.0 Token Exchange) | IETF RFC, January 2020; complements RFC 6749/6750/7519 |
| Delegation modes sanctioned | delegation / impersonation / step-up | Three concrete `grant_type` profiles |
| Identity-provider trust | **explicit allow-list** (`phenotype-ops/federation/idp-allowlist.yaml`) | No auto-discovery; governance PR per IdP |
| Refresh-token rotation | **single-use**, sender-constrained where supported | OAuth Security BCP §4.13 |
| Refresh-token lifetime | **30 days** max, then re-authentication | Bounds blast radius of leaked refresh tokens |
| Access-token refresh window | **proactive, 5 min before expiry** | Bounds latency of mid-flight expiration |
| JWKS cache TTL | **5 min ± 10% jitter** | Avoids thundering-herd refresh |
| `kid` miss handling | **1 retry after JWKS refresh** | Bounded retry before failing |
| Clock-skew tolerance | **60 s symmetric** (per-IdP median offset) | RFC 7519 §4.1.4 / §4.1.5 |
| Revocation (online) | **RFC 7662 introspection**, 500 ms budget | For high-sensitivity operations (ADR-077, ADR-039) |
| Revocation propagation budget | **5 minutes** (worst case) | Within SOC2 evidence collection timelines |

### Standards adopted

- IETF RFC 8693 — OAuth 2.0 Token Exchange (January 2020)
- IETF RFC 7009 — OAuth 2.0 Token Revocation
- IETF RFC 7662 — OAuth 2.0 Token Introspection
- IETF RFC 7519 — JSON Web Token (JWT)
- IETF RFC 8705 — OAuth 2.0 Mutual-TLS Client Authentication and Certificate-Bound Access Tokens
- IETF RFC 9449 — DPoP (Demonstrating Proof-of-Possession)
- OpenID Connect Core 1.0 — `act`, `azp`, `aud` claims
- OpenID Connect Discovery 1.0 — `/.well-known/openid-configuration` metadata
- OAuth Security BCP (draft-ietf-oauth-security-topics) §4.13
- OWASP ASVS v4.0.3 V3.5 / V3.7 / V6.4
- NIST SP 800-63B §7.1 (Session binding)

---

## Companion ADRs in adjacent waves

| ADR | Path | Wave | Relation |
|-----|------|------|----------|
| ADR-046 (2026-06-18) | `docs/adr/2026-06-18/ADR-046-federation-mtls-oidc.md` (catalog entry only) | 2026-06-18 | Catalog row; superseded by the 2026-06-22 triplet (ADR-046 + ADR-046b + ADR-046c) as canonical content |
| ADR-079 (2026-06-21) | `docs/adr/2026-06-21/ADR-079-oidc-federation-reference.md` | 2026-06-21 | Inbound JWT verification; ADR-046b composes on top |
| ADR-046 (2026-06-22) | `docs/adr/2026-06-22/ADR-046-federation-mtls.md` | 2026-06-22 | Transport substrate (companion #1) |
| ADR-046b (2026-06-22) | `docs/adr/2026-06-22/ADR-046b-federation-oidc.md` | 2026-06-22 | Delegated-authentication protocol (companion #2) |
| ADR-046c (2026-06-22) | forthcoming | 2026-06-22 | Topology overlay (companion #3) |

---

## Refresh cadence

- Refreshed: 2026-06-22 (wave authoring; ADR-046b added)
- Next refresh: 2026-06-29 (per ADR-041 weekly Monday 09:00 PDT cadence)
- On ADR-046c authoring: this INDEX must be updated to list the third
  companion ADR and revise the "Companions" section in ADR-046b.