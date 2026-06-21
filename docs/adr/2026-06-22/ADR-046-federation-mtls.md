# ADR-046: Federation mTLS architecture

**Date:** 2026-06-22
**Status:** ACCEPTED
**Cycle:** v21 cycle-11 P0 (carry-over from v20 cycle-10 P1 net-new federation/interop track)
**Owner:** orch-w1-a (L5-158)
**Pillars touched:** L54 (Federation mTLS + OIDC), L48 (Dependency auditing — transitive trust)
**Companions (forthcoming):** ADR-046b (OIDC federation reference implementation), ADR-046c (peer-to-peer federation)

---

## Context

The Phenotype fleet currently runs federated services across multiple trust boundaries
(`phenoMCP`, `phenoObservability`, `phenoEvents`, `phenotype-router`, `Configra`,
`AuthKit`, `Eidolon`, `Eventra`, `HeliosLab`). As of v19 cycle-9 closure (2026-06-21,
fleet mean 2.95), authentication between services within a single org is handled by
in-cluster mTLS (Istio/Linkerd sidecars) or by AWS IAM roles for cross-account service
mesh — neither of which extends cleanly across **organizational boundaries**.

Three concrete failure modes are observed as the fleet grows:

1. **Cross-org service-to-service calls lack a uniform identity.** When a Phenotype
   consumer (e.g. `phenotype-router`) needs to call a federated provider that lives in
   a partner org (e.g. a third-party inference gateway or a customer-hosted
   `phenoEvents`), the call is currently authenticated by static API keys rotated by
   hand, or by long-lived bearer tokens with no expiration audit. There is no mutual
   proof of identity — the client cannot verify the server's certificate chain beyond
   the public WebPKI, and the server cannot cryptographically bind the request to a
   known workload identity.

2. **Cert rotation is ad-hoc and unbounded.** When mTLS is enabled, leaf certificates
   are issued with lifetimes that range from 30 days (operator convention) to 2 years
   (the cert-manager default). There is no fleet-wide policy enforcing a maximum
   validity period, no automated rotation hook into `pheno-config`, and no audit
   trail for rotation events. Per OWASP ASVS v4.0.3 §6.4.1, transport-layer
   certificates MUST have a defined maximum validity; a 2-year leaf is incompatible
   with the fleet's threat model where partner orgs are added/removed monthly.

3. **Root CA federation is undefined.** When a new partner org joins the federation,
   the operator must (a) obtain that org's signing CA bundle out-of-band, (b) trust
   that bundle for all fleet services, and (c) accept the transitive-trust implications
   without any policy gate. There is no concept of a federation root CA, no pinning
   posture, and no explicit allow/deny list for partner CAs. A compromised partner CA
   today can sign a certificate that any fleet service will accept by default.

The PRCP pattern (ADR-018) and the federation interface work in ADR-051 / ADR-052
established that `phenotype-router` will need to speak to peer decision layers across
orgs. The L54 (Federation mTLS + OIDC) pillar at score 2.0 in v19 was the explicit
gap. ADR-046b (OIDC federation reference implementation) and ADR-046c (peer-to-peer
federation) will be authored next to complete the L54 cycle-11 closure; this ADR
sets the **mTLS-specific** substrate.

---

## Decision

We adopt **mTLS with per-org issuing CAs chained to a federation root CA** as the
canonical cross-org service-to-service authentication mechanism for the Phenotype
fleet, with the following concrete decisions:

### 1. mTLS is mandatory for cross-org calls

Every service-to-service call that crosses an organizational boundary MUST terminate
in a verified mTLS handshake. Both client and server present an X.509 leaf certificate
issued by their own org's issuing CA, and both verify the peer's certificate against
a known set of trusted roots.

- **Client side:** the calling service presents its workload identity certificate
  during the TLS handshake. The receiving service MUST validate it against the
  federation root CA store.
- **Server side:** the receiving service presents its own workload identity
  certificate. The calling service MUST validate it before sending any request body.
- **Library:** `rustls` (Rust), `pyOpenSSL` with `cryptography` (Python), `crypto/tls`
  (Go), `node:tls` (TypeScript). WebPKI validation alone is insufficient — federation
  requires explicit root CA pinning.

### 2. Cert rotation strategy — 90-day maximum leaf lifetime

All workload identity certificates issued under the federation PKI MUST have a
maximum validity period of **90 days** from issuance to expiry.

- **Why 90 days:** strikes the operational balance between rotation overhead and
  blast radius for a leaked private key. Aligns with NIST SP 800-57 §5.6
  recommendation for "short-lived credentials" and exceeds OWASP ASVS v4.0.3 §6.4.1
  minimum (which requires a defined maximum, no specific value).
- **Re-issue policy:** certificates are re-issued by the org's issuing CA at the
  60-day mark (i.e. 2/3 of the lifetime), giving a 30-day overlap window during which
  both the old and new certs are valid. This overlap is required to prevent rolling
  restarts from causing spurious handshake failures.
- **Automation hook:** rotation events emit an OTLP span via `pheno-tracing` (per
  ADR-012 / ADR-036B) with attributes `cert.serial`, `cert.cn`, `cert.rotation.cause`
  (`scheduled` | `emergency` | `renewal`). Rotation metrics
  (`cert.days_to_expiry`, `cert.rotation.count`) are exported as Prometheus gauges.
- **Emergency rotation:** in the event of a suspected key compromise, the
  federation root CA can revoke an org's entire issuing CA via a CRL or OCSP
  response. Revocation is propagated within 60 seconds target, 5 minutes SLO.

### 3. Root CA federation — explicit allow-list, no implicit trust

The fleet maintains a **federation root CA store** at
`/etc/phenotype/federation/ca-bundle.pem` (or the equivalent in-memory config loaded
via `pheno-config`). Partner orgs are added to this store explicitly via a governance
PR; there is **no auto-discovery** of CAs and **no transitive trust**.

- **Federation root CA:** a single, well-known CA operated by the Phenotype
  governance circle (initially a long-lived self-signed cert; can be delegated to
  an external CA provider in a future ADR if business needs dictate).
- **Partner CA bundle format:** PEM-encoded, one file per partner org, named
  `{partner-org-id}-bundle.pem`. Each bundle is checked into the `phenotype-ops`
  repo under `federation/ca-bundles/` with an ADR reference and a PR approval from
  at least one governance-circle reviewer.
- **Pinning posture:** by default, partner leaf certs are pinned to the partner's
  public key (SHA-256 of the SPKI). A pinned leaf cert cannot be silently replaced
  by a different cert from the same CA without an operator override — providing
  defense-in-depth against compromised partner CAs.
- **Revocation check:** every mTLS handshake MUST perform an OCSP stapling check or
  CRL fetch against the issuing CA's revocation endpoint. Fail-closed: if revocation
  status cannot be determined within 2 seconds, the handshake is rejected.

### 4. Key pinning at the leaf level

Every leaf certificate presented during an mTLS handshake is **public-key pinned** to
the SHA-256 hash of its SubjectPublicKeyInfo (SPKI). The pin is stored in
`pheno-config` under `federation.{partner-org-id}.pins.{cert-cn}`.

- **Pin rotation:** pins are rotated every 180 days (independent of cert rotation) to
  bound the exposure window of a leaked pin. The old pin and new pin both validate
  during a 7-day overlap window.
- **Pin compromise response:** a pin can be emergency-rolled by an out-of-band
  governance PR with mandatory two-reviewer approval. The PR MUST include an
  incident link and the suspected compromise vector.
- **Pin audit:** every TLS handshake logs the matched pin ID (not the pin value) via
  `pheno-tracing` to enable forensic analysis of which pin was active for a given
  connection.

### 5. Failure modes and mitigations

The following failure modes are explicitly modeled:

| Failure mode | Detection | Mitigation |
|--------------|-----------|------------|
| **Cert expired** (clock skew, missed rotation) | TLS handshake fails with `certificate has expired` | Fleet-wide NTP enforcement; 30-day overlap window; alerting at 14d / 7d / 1d before expiry via Prometheus `cert.days_to_expiry` gauge |
| **Cert revoked** (compromise) | OCSP response returns `revoked` | CRL refresh every 10 min; emergency rotation PR template with `reason: compromise` field |
| **CA bundle drift** (new partner CA added without review) | GitOps gate on `federation/ca-bundles/` directory — any new bundle requires an ADR PR | `pheno-drift-detector` (per ADR-049) detects unauthorized CA additions nightly |
| **Pin mismatch** (cert was reissued with a different key) | TLS handshake fails with `certificate verify failed: pin mismatch` | 7-day overlap window; pin audit log entry for forensic root-cause |
| **OCSP responder down** | OCSP request times out | Fall back to CRL; if both fail, **fail-closed** after 2s timeout |
| **Cross-clock-skew** | Handshake succeeds but request signing fails | RFC 3161 timestamp server; clock skew tolerance = 60s |

### 6. References

The following external standards are explicitly adopted:

- **OWASP ASVS v4.0.3** §6.4 (Cryptographic Storage & Transport) — §6.4.1 cert lifetime,
  §6.4.2 key strength, §6.4.3 trust chain validation. Source:
  <https://owasp.org/www-project-application-security-verification-standard/>
- **NIST SP 800-57 Part 1 Rev. 5** — Recommendation for Key Management, §5.6
  Cryptographic Periods. Source: <https://csrc.nist.gov/publications/detail/sp/800-57/part-1/rev-5/final>
- **NIST SP 800-52 Rev. 2** — Guidelines for the Selection, Configuration, and Use of
  TLS Implementations, §3.5 Certificate Validation. Source:
  <https://csrc.nist.gov/publications/detail/sp/800-52/rev-2/final>
- **CNCF Cloud Native Security Whitepaper** §4.3 (Identity and Access Management),
  §4.4 (Trust Boundary Identification). Source:
  <https://github.com/cncf/tag-security/blob/main/security-whitepaper/v4/CNCF-cloud-native-security-whitepaper.md>
- **SPIFFE/SPIRE** — Workload identity specification. Federation root CA structure is
  aligned with SPIFFE Trust Domain Federation
  (<https://github.com/spiffe/spiffe/blob/main/standards/SPIFFE-Federation.md>).
- **Mozilla TLS Config v5** — Intermediate compatibility profile for cipher suite
  selection. Source: <https://wiki.mozilla.org/Security/Server_Side_TLS>

---

## Consequences

### Positive

- **Uniform identity:** every cross-org call has cryptographic mutual proof of
  identity. Compromised static API keys and unbounded bearer tokens are eliminated
  from the federation path.
- **Bounded blast radius:** a leaked partner CA key can only sign certs valid for
  the next 90 days, and only for certs that pass OCSP/CRL checks; the explicit
  pin store adds a second layer of defense.
- **Audit trail:** every handshake logs to `pheno-tracing`; every rotation event is
  exported to `phenoObservability`; every CA bundle change is gated by a governance
  PR. Forensic analysis of cross-org traffic becomes tractable.
- **Aligned with industry standards:** OWASP ASVS v4.0.3 §6.4, NIST SP 800-57 §5.6,
  CNCF security whitepaper §4.3, and SPIFFE federation spec are all explicitly
  referenced and adopted.

### Negative

- **Operational overhead:** every cert rotation is now a fleet-wide event; rotation
  failure can cascade. Mitigation: 30-day overlap window per §2, plus automated
  alerting at 14d / 7d / 1d pre-expiry.
- **OCSP dependency:** every handshake depends on the issuing CA's OCSP responder.
  Mitigation: 2-second timeout with CRL fallback, fail-closed per §5.
- **Pin management burden:** every pin rotation requires a governance PR. Mitigation:
  pin rotation is decoupled from cert rotation (180 days vs 90 days), reducing churn.
- **OCSP/CRL network egress:** every handshake now requires an outbound network
  call to the partner's revocation endpoint. In high-throughput paths this can
  become a bottleneck; OCSP stapling is the recommended mitigation and is enabled by
  default.

### Neutral

- **Same-org calls unchanged:** intra-cluster / intra-org mTLS (Istio sidecars, AWS
  IAM) is unaffected by this ADR. The ADR scope is strictly cross-org.
- **Library choice is polyglot-allowed:** `rustls`, `pyOpenSSL`, `crypto/tls`,
  `node:tls` all support the relevant standards. No fleet-wide library mandate.

---

## Alternatives

The following alternatives were considered and rejected:

### Alternative A — Mutual TLS with no federation root (WebPKI only)

Use public CA-signed certificates (Let's Encrypt, DigiCert) for cross-org mTLS and
rely on the public WebPKI for trust.

- **Why rejected:** fails the threat model for cross-org federation. WebPKI certs
  can be issued to anyone, for any domain, by any of the ~150 public CAs; there is
  no way to enforce that only specific partners can authenticate to fleet services.
  Also requires every partner to obtain a public cert, which is operationally
  friction and may not be available in air-gapped deployments.

### Alternative B — JWT bearer tokens (no mTLS)

Use short-lived JWT bearer tokens signed by a central identity provider, exchanged
out-of-band between orgs.

- **Why rejected:** does not provide mutual authentication. The server proves its
  identity only via TLS server cert (single direction); a compromised server can
  accept any client's token without being able to verify the client's identity
  cryptographically. Also: token revocation is hard at scale; mTLS revocation via
  OCSP/CRL is more mature.

### Alternative C — Wireguard / IPsec tunnel per partner

Establish a Wireguard or IPsec tunnel between every pair of partner orgs, terminate
all cross-org traffic through the tunnel, and rely on tunnel identity for auth.

- **Why rejected:** does not scale (N² tunnels for N partners), does not provide
  per-request identity (only per-tunnel), and couples the auth model to network
  topology. mTLS at the application layer is more portable and granular.

### Alternative D — Cloud-Provider-Native federation (AWS RAM, GCP Service Catalog)

Use cloud-provider-native federation primitives (AWS RAM for shared resources,
GCP Service Catalog for cross-project service discovery).

- **Why rejected:** locks the federation model to a single cloud provider. The
  Phenotype fleet is multi-cloud (Fly.io + Vercel + Railway + Render + Docker/K8s,
  per ADR-008); cloud-provider-native federation would force a single-cloud
  topology and contradict the multi-platform deployment decision.

### Alternative E — Status quo (static API keys + ad-hoc bearer tokens)

Continue with the current per-call static API keys and long-lived bearer tokens.

- **Why rejected:** explicitly incompatible with the L54 (Federation mTLS + OIDC)
  pillar closure plan. The current model has documented incidents of stale bearer
  tokens being used post-departure of an org admin, and has no audit trail for
  rotation events. Status quo is not a defensible posture for cross-org federation.

---

## Companion ADRs (forthcoming)

This ADR is the first of three companion ADRs that together close L54 (Federation
mTLS + OIDC) at v21 cycle-11:

- **ADR-046b — OIDC federation reference implementation** (forthcoming): the
  application-layer identity layer that runs on top of the mTLS substrate defined
  here. Covers JWT validation, `iss`/`aud`/`exp`/`iat`/`nbf`/`azp` claim checks,
  and the `FederationClient` outbound helper. See ADR-079 (existing reference for
  the Rust crate shape: `pheno-context/src/oidc.rs`).
- **ADR-046c — Peer-to-peer federation topology** (forthcoming): the mesh
  topology decisions for federation: when is the central hub-and-spoke model
  (Phenotype governance circle as the root) preferred vs a peer-to-peer mesh of
  federation root CAs. Covers the failover semantics when one partner's CA goes
  offline.

These will be authored as v21 cycle-11 carries-over from the v20 cycle-10 net-new
federation/interop track listed in AGENTS.md § "Active ADRs" § "v20 cycle-10 P1
carry-overs".

---

## Cross-references

- `AGENTS.md` § Active ADRs — ADR-046 row in the v20 cycle-10 P1 carry-overs table.
- `docs/adr/2026-06-22/ADR-081-v20-cycle-10-p1-reduction.md` — the v20 wave plan
  that lists ADR-046 as a net-new federation/interop track.
- `docs/adr/2026-06-18/INDEX.md` — earlier-wave ADR-046 ("Federation mTLS + OIDC")
  catalog entry; superseded by this ADR as the canonical reference.
- `docs/adr/2026-06-21/ADR-079-oidc-federation-reference.md` — companion ADR-046b
  reference (Rust crate shape).
- `docs/adr/2026-06-18/ADR-051-bifrost-as-library.md` — the transport-library
  decision that makes per-call mTLS termination feasible.
- `docs/adr/2026-06-18/ADR-049-app-substrate-drift-detector.md` — drift detection
  for unauthorized CA bundle additions (§5 failure mode).
- `ADR-012` / `ADR-036B` — `pheno-tracing` substrate for cert rotation spans.
- `ADR-042` — security audit cadence (monthly); this ADR's cert rotation events are
  in-scope for the monthly sweep.
- `ADR-040` — test coverage gates; mTLS handshake code paths require 80% lib
  coverage per ADR-040's lib tier.
- `findings/71-pillar-2026-06-17-schema.md` — L54 (Federation mTLS + OIDC) pillar
  definition; this ADR is the canonical content for L54.
- `findings/71-pillar-2026-06-17.md` — current 71-pillar scorecard; L54 currently
  at 2.0, target 3.0 after v21 cycle-11 closure of all three companion ADRs.

---

## Refresh cadence

This ADR is reviewed:

- **Quarterly** (every 90 days) by the orch-w1-a governance circle for cert
  rotation policy alignment with NIST SP 800-57 §5.6 updates.
- **On partner-org add/remove** — every partner-org CA bundle change MUST cite this
  ADR in its governance PR.
- **On library CVE** — if `rustls`, `pyOpenSSL`, `crypto/tls`, or `node:tls`
  publishes a CVE affecting federation behavior, this ADR is re-reviewed within
  48 hours per ADR-042 (security audit cadence).

Next scheduled refresh: 2026-09-22 (90-day cadence).