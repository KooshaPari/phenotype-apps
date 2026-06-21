# ADR-046: Federation mTLS + OIDC

**Date:** 2026-06-19 (declared 2026-06-18; ratified 2026-06-19; first persisted 2026-06-22)
**Status:** ACCEPTED
**Pillar:** L54 (Identity / Federation)
**Deciders:** phenotype-architecture circle + fleet-owners
**Supersedes:** none
**Superseded by:** none
**Backlinks:** ADR-079 (OIDC federation reference, T3 of v19 cycle 9)

## Context

The Phenotype fleet is **federating**: at v19 cycle 9 closure, 9 federated services (`phenoMCP`, `phenoObservability`, `phenoEvents`, etc.) are deployed across at least 3 trust boundaries (the public `phenotype-apps` org, the internal staging cluster, and the per-developer `pheno-flake` Nix shells). Each boundary has a distinct identity provider, and the fleet needs to:

1. **Mutually authenticate** between services across boundaries (mTLS)
2. **Federate user/agent identity** for cross-boundary API calls (OIDC)
3. **Rotate credentials** without service restart (dynamic creds)
4. **Revoke compromised credentials** within 60 seconds (CT log propagation)

Prior to this ADR, every service had its own ad-hoc auth: bearer tokens in headers, hardcoded API keys in env, or — for the substrate crates — no auth at all (relying on the cluster network policy alone). This was the cause of L46 (secrets management) and L54 (identity) scoring 1.5 / 2.0 respectively on the 71-pillar cycle 3 probe.

## Decision

We adopt **mTLS for service-to-service** and **OIDC for user/agent identity**, with the following concrete choices:

### Service-to-service: mTLS (mutual TLS)

- **Library:** `rustls` 0.23+ (Rust) / `crypto/tls` stdlib (Go) / `node:tls` (TS)
- **CA:** Internal step-ca cluster for staging, public CA via cert-manager for prod
- **Cert TTL:** 24h for service certs, 7d for intermediate CAs
- **Rotation:** Automatic via SPIFFE/SPIRE workload identity, every 24h
- **Revocation:** CRL + OCSP stapling, 60s propagation budget

### User/agent: OIDC (OpenID Connect)

- **Providers:** Auth0 (org primary), Keycloak (self-hosted fallback), GitHub Actions OIDC (CI)
- **Library:** `openidconnect` 3.x (Rust) / `go-oidc` (Go) / `openid-client` (TS)
- **Token format:** JWT with RS256, 1h access, 30d refresh
- **Scopes:** `phenotype:read`, `phenotype:write`, `phenotype:admin`
- **Refresh:** Sliding window with 5min proactive refresh

### Reference implementation

See **ADR-079 — OIDC federation reference** for the canonical consumer example in `pheno-mcp-router`.

## Consequences

### Positive

- **L46 (secrets) lifts 1.5 → 2.5** on cycle 4 probe (mTLS certs managed by SPIRE, not env)
- **L54 (identity) lifts 2.0 → 3.0** on cycle 4 probe (OIDC is the SOTA for federation)
- Cross-boundary API calls now have a single auth model
- Compromise blast radius reduced from "all services with the env var" to "the single cert"
- ADR-077 (Vault) and ADR-078 (encryption-at-rest) are the storage-side complements

### Negative

- **Operational cost:** SPIRE control plane must be deployed + maintained (3 cluster operators)
- **Latency:** mTLS handshake adds ~3-15ms on first request (subsequent requests reuse TLS session)
- **Debugging:** Cert rotation issues are harder to diagnose than bearer tokens
- **Library sprawl:** 3 languages × 2 protocols = 6 client libraries to keep current

### Neutral

- ADR-024 (71-pillar) cycle 4 must re-score L46 + L54 to confirm lift
- The substrate crates (pheno-config, pheno-tracing, etc.) need not implement federation themselves — they remain local-only; the federated wrappers (phenoMCP, phenoObservability) handle the boundary

## Alternatives considered

### A. Bearer tokens (status quo)

Continue using long-lived bearer tokens in headers.

- **Pro:** Zero infra to deploy; all languages have battle-tested clients
- **Con:** Token rotation requires service restart; compromise blast radius is the entire org; no per-request identity binding
- **Verdict:** REJECTED — does not meet the 60s revocation budget

### B. Service mesh (Istio/Linkerd) for mTLS

Deploy Istio and let the mesh handle mTLS automatically.

- **Pro:** Zero code change; mesh handles cert rotation
- **Con:** Istio is 200MB+ per sidecar, requires CRD installation, conflicts with our Nix-flake approach (ADR-039), and adds a deploy surface we don't control
- **Verdict:** REJECTED — violates the substrate minimum-surface principle (ADR-023 Rule 3)

### C. macaroons / capability-based tokens

Use macaroons (caveat-based bearer tokens) instead of OIDC.

- **Pro:** Better delegation semantics; can scope per-request
- **Con:** Library support is sparse (no first-party Rust/Go/TS); not industry-standard; harder to debug
- **Verdict:** DEFERRED — consider for v22+ if per-request scoping becomes a hard requirement

## Implementation plan

| Wave | Track | Deliverable | Status |
|------|-------|-------------|--------|
| v19 cycle 9 | T1 (ADR-077) | Vault migration roadmap | ✅ Done |
| v19 cycle 9 | T2 (ADR-078) | Encryption-at-rest mandate | ✅ Done |
| v19 cycle 9 | T3 (ADR-079) | OIDC federation reference | ✅ Done |
| v19 cycle 9 | T5 (ADR-080) | Pen-test + bug-bounty roadmap | ✅ Done |
| v20 cycle 10 | T9 (mTLS — heavy-runner) | `pheno-mtls` crate + 3 adoptions | ⏳ Planned |
| v20 cycle 10 | T10 (OIDC client — heavy-runner) | `pheno-oidc` crate | ⏳ Planned |

## References

- **ADR-077** — Vault migration roadmap (L52, T1 of v19)
- **ADR-078** — Encryption-at-rest mandate (L55, T2 of v19)
- **ADR-079** — OIDC federation reference (L54, T3 of v19) — *canonical consumer example*
- **ADR-080** — Pen-test + bug-bounty roadmap (L49, T5 of v19)
- **ADR-024** — 71-pillar audit framework (scoring rubric for L46 + L54)
- **ADR-039** — pheno-flake refresh template (Nix-flake compatibility for SPIRE deploy)
- **ADR-042B** — Substrate quality bar (L46 + L54 adoption gate)
- **NIST SP 800-204A** — Building Secure Microservices
- **SPIFFE/SPIRE** — <https://spiffe.io/docs/latest/spire-about/>
- **OWASP ASVS v4.0** — V3 (Session Management), V6 (Cryptography)
- **IETF RFC 8705** — OAuth 2.0 Mutual-TLS Client Authentication
- **IETF RFC 7519** — JSON Web Token (JWT)
- **Project context:** `findings/2026-06-20-71-pillar-cycle-3-probe.md` (L46/L54 baseline)
- **Worklog:** `worklogs/2026-06-21-v19-cycle-9-71-pillar-p0-orchestrator.json` (T1-T5 launch)
