# ADR-046b: Federation OIDC token exchange reference architecture

**Date:** 2026-06-22
**Status:** ACCEPTED
**Cycle:** v21 cycle-11 P0 (companion to ADR-046 mTLS architecture)
**Owner:** orch-w1-b (L5-159)
**Pillars touched:** L54 (Federation mTLS + OIDC), L49 (Identity — delegation/impersonation)
**Companions:** ADR-046 (mTLS, transport layer — ACCEPTED), ADR-046c (p2p topology — forthcoming)

---

## Context

ADR-046 (this wave, ACCEPTED) sets the **transport-layer** federation contract: mTLS with federation-root-anchored certificate chains, 90-day leaf lifetime, and SHA-256 SPKI pinning. ADR-079 (2026-06-21, ACCEPTED) sets the **application-layer identity verification** contract: every fleet service validates inbound bearer JWTs through `pheno-context::oidc::FederationClient`, enforcing the 5 standard claims (`iss`, `aud`, `exp`, `iat`, `nbf`) plus `azp`.

What is still missing — and what ADR-046b closes — is the **delegation and impersonation** contract that runs on top of these two layers. Three concrete use cases have emerged in v20 cycle-10 work:

1. **Cross-org on-behalf-of delegation.** A user authenticates to IdP-A inside org-A. The user calls service S inside org-B. S must act on the user's behalf to call a downstream service T inside org-C. Without RFC 8693 token exchange, S currently forwards the user's raw JWT — leaking the user's identity to T and making audit attribution impossible (T cannot tell whether the request came from the user directly or was delegated by S).
2. **Service impersonation for trust bridging.** Two partner orgs federate but do not share an IdP. Service A in org-A needs to call service B in org-B on behalf of a workload, but B's IdP has no trust relationship with A's IdP. RFC 8693 impersonation grants provide the bridge.
3. **Step-up authentication.** A user with a low-assurance session attempts a high-risk operation (e.g. signing a release artifact per ADR-077). The fleet needs to request a fresh, higher-assurance token via RFC 8693 with the original subject_token serving as proof of prior authentication.

ADR-079 covers **inbound** token verification (a service asking "is this token valid?"). ADR-046b covers **outbound** token acquisition (a service asking "please give me a token that lets me call X on behalf of Y"). The two are complementary, not overlapping.

L54 (Federation mTLS + OIDC) at score 2.0 in v19 cycle-9 was the explicit gap; ADR-046 + ADR-046b + ADR-046c together close L54 to 3.0 at v21 cycle-11.

---

## Decision

We adopt **OAuth 2.0 Token Exchange (RFC 8693)** as the canonical cross-org delegated-authentication protocol for the Phenotype fleet, layered on top of the mTLS substrate from ADR-046 and the JWT verification substrate from ADR-079. The following concrete decisions apply.

### 1. RFC 8693 token exchange — protocol adoption

Every cross-org delegated-authentication flow in the fleet MUST use the RFC 8693 `urn:ietf:params:oauth:grant-type:token-exchange` grant type. Three concrete exchange modes are sanctioned:

- **Delegation (`on-behalf-of`):** the requestor exchanges a subject_token (the user's JWT) for a new token bound to the requestor. The new token's `sub` claim is the user; the new token's `act` claim carries the requestor as actor. Used when service A needs to call service B on behalf of user U.
- **Impersonation (`act-as`):** the requestor exchanges a subject_token (a workload identity JWT issued by the requestor's IdP) for a new token whose `sub` claim is set to the requestor. Used for trust bridging across IdPs that do not have direct trust.
- **Step-up (`reauth`):** the requestor exchanges a low-assurance subject_token for a high-assurance token of the same `sub`. Used before high-risk operations (release signing per ADR-077, force-push to `main` per ADR-039).

The canonical wire-format request is:

```http
POST /oauth2/token HTTP/1.1
Host: idp.partner-org.example
Content-Type: application/x-www-form-urlencoded

grant_type=urn:ietf:params:oauth:grant-type:token-exchange
&subject_token=<user_jwt>
&subject_token_type=urn:ietf:params:oauth:token-type:jwt
&actor_token=<workload_jwt>
&actor_token_type=urn:ietf:params:oauth:token-type:jwt
&audience=https://phenotype-router.partner-org.example
&scope=phenotype:read phenotype:write
&requested_token_type=urn:ietf:params:oauth:token-type:jwt
```

The 5 token types from the IANA "OAuth Token Type Registration" registry are the only sanctioned `subject_token_type` / `actor_token_type` / `requested_token_type` values: `access_token` (RFC 6749), `refresh_token` (RFC 6749), `id_token` (OIDC Core 1.0), `saml1` / `saml2` (OASIS), `jwt` (RFC 7519).

### 2. Identity provider trust establishment

Federation IdPs are onboarded through a **two-phase trust handshake**, gated by governance review:

- **Phase 1 — Discovery.** The new IdP's metadata is fetched from `/.well-known/openid-configuration` per OpenID Connect Discovery 1.0 §4. Required fields: `issuer`, `authorization_endpoint`, `token_endpoint`, `jwks_uri`, `subject_types_supported`, `id_token_signing_alg_values_supported`.
- **Phase 2 — Trust anchor.** The IdP's signing keys are pinned into the fleet JWKS cache (per §5) under a stable, governance-approved alias (e.g. `https://idp.partner-org.example`). The alias is recorded in `phenotype-ops/federation/idp-allowlist.yaml` with a governance PR approval from at least one orch-w1 reviewer; **no auto-discovery of IdPs** is permitted.

Federation trust is **explicit and auditable** — there is no transitive trust, no automatic onboarding, and no anonymous IdP acceptance. A compromised IdP can have its alias revoked by removing the entry from the allow-list (PR + two-reviewer approval per the ADR-046 §4 pin-compromise flow).

### 3. Audience and scope claims

Audience (`aud`) and scope (`scope`) claims are the **two primary authorization gates** for every federated token issued under RFC 8693:

- **`aud` enforcement:** the `aud` claim of every exchanged token MUST equal the canonical URI of the receiving service (e.g. `https://phenotype-router.partner-org.example`). Multiple-audience tokens (array-valued `aud`) are permitted, but every entry MUST match a pinned audience in `pheno-config` under `federation.{partner-org-id}.audiences`. Tokens whose `aud` does not match any pinned audience are rejected.
- **`scope` enforcement:** the `scope` claim of every exchanged token MUST be a subset of the scopes permitted for the requesting workload, as configured under `federation.{partner-org-id}.scope-bounds`. The fleet's baseline scope vocabulary is `phenotype:read`, `phenotype:write`, `phenotype:admin` (per ADR-046 § "OIDC"). Custom scopes require a governance ADR.
- **`azp` (authorized party) enforcement:** per ADR-079, if the exchanged token has an `azp` claim, it MUST equal one of the entries in `PHENO_FEDERATION_AUTHORIZED_PARTIES` for the receiving service. This prevents confused-deputy attacks where a token issued for service A is replayed against service B.
- **`act` (actor) claim:** when the exchange is a delegation, the resulting token's `act` claim MUST contain a `sub` (the actor) and an optional `iss` (the actor's IdP). Missing or malformed `act` claims cause the token to be rejected by ADR-079 verification.

### 4. Refresh token rotation

Refresh tokens issued by federated IdPs follow **single-use rotation with sender-constrained re-authentication**:

- **Single-use rotation:** every refresh of a refresh token MUST issue a new refresh token; the old refresh token is invalidated by the IdP immediately. Per OAuth 2.0 Security BCP draft-ietf-oauth-security-topics §4.13.
- **Sender-constrained tokens:** where the IdP supports DPoP (RFC 9449) or MTLS-bound refresh tokens (RFC 8705), the fleet MUST use the sender-constrained variant. Public clients MAY use unbound refresh tokens but MUST rotate on every use.
- **Re-authentication cadence:** refresh tokens are valid for a maximum of **30 days** from issuance. After 30 days, the user MUST re-authenticate with the original IdP; the IdP MUST refuse exchanges of a refresh token older than 30 days.
- **Refresh window:** the access token MUST be refreshed **proactively** when its remaining lifetime falls below **5 minutes**, per ADR-046 § "OIDC Refresh". This bounds the latency of mid-flight expiration.
- **Refresh audit:** every refresh event is logged via `pheno-tracing` (ADR-012 / ADR-036B) with attributes `refresh.subject`, `refresh.idp_alias`, `refresh.cause` (`scheduled` | `rotation` | `emergency` | `expired`).

### 5. Key discovery (JWKS)

JWKS discovery and caching follow the **OpenID Connect Discovery 1.0** specification with fleet-specific cache parameters:

- **Discovery endpoint:** every federated IdP MUST expose its metadata at `/.well-known/openid-configuration` per OIDC Discovery 1.0 §4. The `jwks_uri` field is consumed to discover the signing keys.
- **JWKS cache TTL:** **5 minutes** with **10% jitter** (300s ± 30s) to prevent thundering-herd refresh storms across the fleet. Cache key is `(idp_alias, kid)`. Per ADR-079 §5.
- **`kid` miss handling:** if a token references a `kid` not in the cache, the verifier triggers a JWKS refresh and retries **once** before failing. Per ADR-079 §6.
- **Cache storage:** JWKS entries are cached in-memory per service and exported to `phenoObservability` as the `jwks.cache_hit_ratio` Prometheus gauge. A cache hit ratio below 80% over 24 hours triggers a P3 alert.
- **JWKS rotation:** when an IdP rotates its signing keys, the new `kid` appears in the JWKS response. The fleet's `kid` miss handling ensures the new key is picked up within 5 minutes (cache TTL) plus the 2-second refresh round-trip.

### 6. Clock-skew tolerance

Token validation times (`exp`, `iat`, `nbf`) tolerate clock skew between the issuer and the verifier:

- **Default skew:** **60 seconds** symmetric — tokens are accepted up to 60s before `nbf` and up to 60s after `exp` to absorb clock drift across partner IdPs. Per RFC 7519 §4.1.4 / §4.1.5 recommendation.
- **Fleet-wide NTP enforcement:** per ADR-046 §5, every fleet service runs `chrony` configured against the Phenotype NTP pool (`time.phenotype-apps.internal`). Clock drift beyond **5 seconds** triggers a P1 alert; drift beyond **60 seconds** triggers a P0 alert and the service enters fail-closed mode.
- **Asymmetric skew handling:** the verifier tracks a per-IdP skew offset (median over the last 100 verified tokens) and uses that offset rather than the global 60s default. This is necessary because some partner IdPs run on less-reliable clock sources.

### 7. Revocation

Two revocation mechanisms are supported, in order of preference:

- **Online revocation check (RFC 7009):** for high-sensitivity operations (release signing per ADR-077, force-push per ADR-039), the verifier performs an RFC 7662 token introspection call against the IdP's introspection endpoint before the operation. Latency budget: **500ms**; fail-closed if the IdP introspection endpoint is unreachable.
- **Local revocation cache:** when RFC 7009 introspection is unavailable (or for low-sensitivity operations), the verifier consults a locally maintained revocation list under `federation.{partner-org-id}.revocations` in `pheno-config`. The list is refreshed every **60 seconds** from the IdP's published revocation endpoint. Maximum staleness: **5 minutes** (per ADR-046 §3 fail-closed posture for cross-boundary state).
- **Revocation propagation budget:** once a token is revoked at the IdP, the maximum fleet-wide visibility lag is **5 minutes** (60s for the refresh window + up to 4 minutes of clock skew plus jitter). This is within the SOC2 evidence collection timelines referenced in v18 T3 (L51 SOC2 evidence automation).

### 8. References

External standards adopted:

- **IETF RFC 8693** — OAuth 2.0 Token Exchange (January 2020).
- **IETF RFC 7009** — OAuth 2.0 Token Revocation.
- **IETF RFC 7662** — OAuth 2.0 Token Introspection.
- **IETF RFC 7519** — JSON Web Token (JWT).
- **IETF RFC 8705** — OAuth 2.0 Mutual-TLS Client Authentication and Certificate-Bound Access Tokens.
- **IETF RFC 9449** — DPoP (Demonstrating Proof-of-Possession).
- **OpenID Connect Core 1.0** — `act`, `azp`, `aud` claims.
- **OpenID Connect Discovery 1.0** — `/.well-known/openid-configuration` metadata.
- **OAuth Security BCP (draft-ietf-oauth-security-topics)** §4.13 — refresh-token rotation.
- **OWASP ASVS v4.0.3** V3.5 (Reauthentication), V3.7 (Session termination), V6.4 (Transport-layer credential delegation).
- **NIST SP 800-63B** §7.1 (Session binding).

---

## Consequences

### Positive

- **Delegated auth becomes auditable.** Every delegation/impersonation produces a token with explicit `act` and `azp` claims; the downstream service can reconstruct the full chain (user → requestor → resource).
- **IdP onboarding is governed.** The two-phase trust handshake (§2) ensures every federated IdP has a governance PR trail; compromised IdPs can be revoked in 60 seconds.
- **Refresh token rotation is mandatory.** Single-use rotation (§4) bounds the blast radius of a leaked refresh token to a single use window.
- **Revocation propagation is bounded.** The 5-minute fleet-wide visibility lag (§7) is within SOC2 evidence collection timelines.
- **Standards-aligned.** RFC 8693, RFC 7009, RFC 7662, RFC 7519, OIDC Core 1.0, and OIDC Discovery 1.0 are all explicitly adopted.

### Negative

- **Two-phase IdP onboarding friction.** Every new partner IdP requires a governance PR + reviewer approval. Mitigation: provide `scripts/federation_idp_onboard.py` helper that produces the allow-list PR body from the IdP's `/.well-known/openid-configuration` response.
- **Refresh token rotation requires IdP support.** Some legacy IdPs do not implement single-use rotation. Mitigation: the IdP allow-list (§2) explicitly requires `rotation: single-use` in the IdP metadata; legacy IdPs are out of scope.
- **Clock skew monitoring requires fleet-wide NTP.** Mitigation: ADR-046 already mandates fleet-wide NTP via `chrony`; this ADR inherits that mandate.
- **RFC 7009 introspection adds latency.** For high-sensitivity operations, the 500ms introspection check (§7) is on the critical path. Mitigation: cache the introspection response for the access token's lifetime (typically 1 hour); the 500ms is amortized.

### Neutral

- **Inbound JWT verification is unchanged.** ADR-079 remains the canonical inbound verifier; ADR-046b is the outbound token acquisition layer. The two compose but do not overlap.
- **mTLS substrate is unchanged.** ADR-046 remains the canonical transport layer; ADR-046b runs on top of mTLS. The transport-layer identity (X.509 SVIDs from SPIFFE) and the application-layer identity (RFC 8693-delegated JWTs) coexist by design.

---

## Alternatives

### Alternative A — Forward the original JWT (no RFC 8693)

A service forwards the user's original JWT to the downstream service without token exchange.

- **Why rejected:** breaks audit attribution (downstream cannot distinguish user-direct vs delegated calls), makes revocation hard (revoking the user's JWT invalidates ALL delegated sessions), and increases the user's blast radius (a compromised service can reuse the user's JWT for arbitrary downstream calls).

### Alternative B — SAML 2.0 token exchange (OASIS profile)

Use the OASIS SAML 2.0 profile for token exchange.

- **Why rejected:** SAML 2.0 token exchange is not standardized at the same level as RFC 8693; the IETF OAuth working group has explicitly deprecated new SAML-OAuth bridging in favor of JWT-based flows. Library support is sparse and unmaintained.

### Alternative C — Custom JWT exchange protocol

Define a fleet-specific JWT exchange protocol not derived from OIDC Core 1.0.

- **Why rejected:** violates the substrate minimum-surface principle (ADR-023 Rule 3) — every new protocol adds a maintenance burden. RFC 8693 already provides everything we need.

### Alternative D — Mutual OAuth (draft-ietf-oauth-mutual)

Use the IETF `draft-ietf-oauth-mutual` specification instead of the RFC 8693 + ADR-046 mTLS combination.

- **Why rejected:** `draft-ietf-oauth-mutual` is still in IETF draft stage (no RFC number assigned); pinning fleet federation to an unstable draft would lock us into a spec that may change. The RFC 8693 + ADR-046 mTLS + ADR-046b combination is built entirely on stable, finalized RFCs.

### Alternative E — Status quo (ad-hoc JWT forwarding per service)

Continue with the current per-service JWT-forwarding pattern, with each service implementing its own delegation logic.

- **Why rejected:** explicitly incompatible with the L54 pillar closure plan. ADR-079 already moved inbound verification to a single canonical verifier; ADR-046b completes the picture by moving outbound acquisition to a single canonical client.

---

## Appendix A — Companion ADRs

This ADR is the second of three companion ADRs that together close L54 (Federation mTLS + OIDC) at v21 cycle-11:

- **ADR-046 — Federation mTLS architecture** (ACCEPTED, 2026-06-22): the transport-layer identity substrate. Provides mutual authentication via X.509 SVIDs chained to a federation root CA, with 90-day leaf lifetime and SHA-256 SPKI pinning. Companion relationship: **transport substrate**.
- **ADR-046c — Peer-to-peer federation topology** (forthcoming): the mesh-topology decisions for federation: when is the central hub-and-spoke model (Phenotype governance circle as the root) preferred vs a peer-to-peer mesh of federation root CAs. Covers the failover semantics when one partner's CA or IdP goes offline. Companion relationship: **topology overlay**.

The three ADRs are sequenced because each has a hard dependency on the previous: ADR-046 sets the transport identity substrate; ADR-046b adds the application-layer delegated-authentication contract; ADR-046c adds the topology decisions for partial-network / partial-IdP-availability scenarios.

These are v21 cycle-11 carries-over from the v20 cycle-10 net-new federation/interop track listed in AGENTS.md § "Active ADRs" § "v20 cycle-10 P1 carry-overs".

---

## Appendix B — Cross-references

- `AGENTS.md` § Active ADRs — ADR-046b row in the v21 cycle-11 P0 carry-overs table.
- `docs/adr/2026-06-22/ADR-046-federation-mtls.md` — ADR-046 (transport substrate; companion).
- `docs/adr/2026-06-22/ADR-081-v20-cycle-10-p1-reduction.md` — the v20 wave plan listing the federation/interop tracks as net-new work.
- `docs/adr/2026-06-21/ADR-079-oidc-federation-reference.md` — the Rust crate shape (`pheno-context::oidc::FederationClient`) that ADR-046b composes with. ADR-079 covers **inbound** verification; ADR-046b covers **outbound** acquisition.
- `docs/adr/2026-06-18/ADR-046-federation-mtls-oidc.md` — earlier-wave ADR-046 catalog entry; superseded by the 2026-06-22 triplet (ADR-046 + ADR-046b + ADR-046c) as canonical content.
- `docs/adr/2026-06-18/ADR-049-app-substrate-drift-detector.md` — drift detection for unauthorized IdP allow-list additions (§2 failure mode).
- `docs/adr/2026-06-18/ADR-051-bifrost-as-library.md` — transport-library decision that makes per-call mTLS termination feasible (consumed by ADR-046).
- `ADR-012` / `ADR-036B` — `pheno-tracing` substrate for refresh / exchange / revocation spans (§4, §7).
- `ADR-040` — test coverage gates; RFC 8693 client code paths require 80% lib coverage per ADR-040's lib tier.
- `ADR-042` — security audit cadence (monthly); this ADR's token exchange flows are in-scope for the monthly sweep.
- `ADR-077` — SLSA L3 provenance; the high-sensitivity operation that triggers §7 RFC 7009 introspection.
- `findings/71-pillar-2026-06-17-schema.md` — L54 (Federation mTLS + OIDC) pillar definition; this ADR is the canonical content for L54.
- `findings/71-pillar-2026-06-17.md` — current 71-pillar scorecard; L54 currently at 2.0, target 3.0 after v21 cycle-11 closure of all three companion ADRs.

---

## Refresh cadence

This ADR is reviewed:

- **Quarterly** (every 90 days) by the orch-w1 governance circle for alignment with IETF OAuth working group output (RFC 8693 errata, `draft-ietf-oauth-security-topics` updates).
- **On partner-IdP add/remove** — every partner IdP allow-list change MUST cite this ADR in its governance PR.
- **On library CVE** — if `jsonwebtoken`, `reqwest`, `tokio-rustls`, `oauth2` (Rust), `go-oidc` (Go), or `openid-client` (TypeScript) publishes a CVE affecting token exchange behavior, this ADR is re-reviewed within 48 hours per ADR-042.
- **On RFC 8693 update** — when the IETF publishes a new RFC that updates or obsoletes RFC 8693, this ADR is re-reviewed within 30 days.

Next scheduled refresh: 2026-09-22 (90-day cadence).