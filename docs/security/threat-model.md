# FocalPoint Threat Model

_Last updated: 2026-06-30. Maintainer: security@phenotype.dev_

## 1. Assets

| Asset | Sensitivity | Notes |
|---|---|---|
| OAuth tokens (Canvas, GCal, GitHub) | HIGH | Stored via `SecureSecretStore`; encrypted at rest on device |
| FamilyControls entitlement grant | HIGH | Apple-signed; must never be transmitted or proxied |
| Audit-chain ledger | HIGH | Tamper-evident hash chain; integrity verified on startup |
| Rule definitions + schedules | MEDIUM | User configuration; loss = degraded enforcement |
| Reward/penalty wallet balances | MEDIUM | Integrity tied to audit chain |
| Connector event payloads | MEDIUM | May contain assignment titles, calendar event summaries |
| Telemetry / Sentry breadcrumbs | LOW | Stripped PII before transmission (see `focus-telemetry`) |
| Local SQLite DB | MEDIUM | Contains ledger + rules + tokens (encrypted fields) |

## 2. Trust Boundaries

```
┌────────────────────────────────────────────────────────────────┐
│  iOS App Sandbox (FamilyControls entitlement required)         │
│  ┌──────────────┐   ┌──────────────────────────────────────┐  │
│  │  SwiftUI UI  │──▶│  focus-ffi (UniFFI Rust bindings)    │  │
│  └──────────────┘   └──────────┬───────────────────────────┘  │
│                                │ in-process FFI (safe boundary)│
│                    ┌───────────▼──────────────────────────┐   │
│                    │  Rust Core (rules, audit, wallet, …)  │   │
│                    │  focus-domain / focus-storage         │   │
│                    └───────────┬──────────────────────────┘   │
└────────────────────────────────│───────────────────────────────┘
                                 │ HTTPS (TLS 1.2+)
         ┌───────────────────────▼──────────────────────────┐
         │  Upstream OAuth Providers (Canvas, GCal, GitHub) │
         │  Webhook Ingest (services/webhook-ingest)        │
         └──────────────────────────────────────────────────┘
```

**Key trust boundaries:**
- **App sandbox ↔ OS:** FamilyControls APIs require Apple-signed entitlement; sandbox prevents other apps from reading keychain items.
- **SwiftUI ↔ Rust FFI:** Crossing via UniFFI-generated bindings; all data is serialized, preventing memory sharing across the boundary.
- **Rust core ↔ network:** All outbound connections use system TLS. Certificate pinning is not yet enforced (tracked: #TBD).
- **App ↔ backend services:** JWT-authenticated; services are not yet deployed (scaffold). Auth-broker service will enforce per-user scopes.

## 3. STRIDE Analysis

### Spoofing
| Threat | Likelihood | Mitigation | Gap |
|---|---|---|---|
| Attacker replays stale OAuth token | MEDIUM | Short-lived tokens + refresh cycle in `focus-connectors` | No token rotation enforced on abnormal logout |
| Forged connector event | LOW | Webhook HMAC signature verification in `focus-webhook-server` | HMAC secret rotation not automated |
| Impersonated sync-api service | LOW | JWT required; service not yet deployed | mTLS not implemented |

### Tampering
| Threat | Likelihood | Mitigation | Gap |
|---|---|---|---|
| Audit-chain record modification | LOW | SHA256 hash chain verified on startup (`focus-audit`) | Verification only on startup, not continuous |
| SQLite DB modified outside app | MEDIUM | App-sandbox prevents direct access on device; `TokenWrap` AES-256-GCM protects token fields | Wallet balances not AEAD-protected (only token fields are) |
| Rule DSL injection via connector event | LOW | Connector events mapped through typed schema, not eval'd | No property-based fuzz test for rule parser yet |

### Repudiation
| Threat | Likelihood | Mitigation | Gap |
|---|---|---|---|
| User denies penalty application | LOW | Tamper-evident audit chain with per-event hash | Audit chain not signed with device key |

### Information Disclosure
| Threat | Likelihood | Mitigation | Gap |
|---|---|---|---|
| OAuth token leaked in logs | MEDIUM | `secrecy::SecretString` prevents accidental logging | Sentry breadcrumbs not audited for token leakage |
| Connector event payload leaked | LOW | Telemetry strips PII in `focus-telemetry` | No CI assertion confirming strip behavior |

### Denial of Service
| Threat | Likelihood | Mitigation | Gap |
|---|---|---|---|
| Connector flood fills event queue | MEDIUM | In-memory rate limiter in `phenotype-ops/review-surface` | Not extracted to shared middleware; no backpressure headers |
| Malformed webhook crashes ingest | LOW | Typed deserialization rejects invalid payloads | No property-based/fuzz test for webhook parser |

### Elevation of Privilege
| Threat | Likelihood | Mitigation | Gap |
|---|---|---|---|
| FamilyControls bypass via FFI | LOW | Platform-enforced; cannot be called without entitlement | Entitlement check not tested in CI (requires real device) |
| Rule authoring escalates to system block | MEDIUM | Rules validated against DSL schema before persistence | Schema enforcement not enforced server-side (no backend yet) |

## 4. Cryptographic Risk Areas

| Component | Algorithm | Key source | Risk |
|---|---|---|---|
| `TokenWrap::new` | AES-256-GCM (ring) | 32-byte random, stored in `SecureSecretStore` | Key rotation not yet implemented |
| Audit chain entries | SHA-256 (ring) | Chained; seed = random genesis block | Genesis block not device-attested |
| Template pack signing | Ed25519 (ring) | Per-pack key pair in `focus-plugin-sdk` | Key revocation not implemented |
| Connector OAuth | Provider-managed | Stored via `SecureSecretStore` | Token refresh on device-wipe not tested |

## 5. Tenancy Assumptions

FocalPoint v0 is single-user/single-device. Multi-user or family-account tenancy is **not yet designed**:
- Wallet and audit chain are per-device SQLite instances.
- No org/workspace isolation model exists.
- FamilyControls can manage child devices but there is no cross-device audit aggregation.
- **Gap:** If multi-user is added, wallet credits and audit entries must be scoped to a `user_id` foreign key and the sync protocol must enforce per-user isolation.

## 6. Retention and Data Deletion

| Data class | Retention | Deletion path |
|---|---|---|
| Audit chain | Indefinite on device | App uninstall removes SQLite DB |
| OAuth tokens | Until revoked or app uninstall | `SecureSecretStore::delete()` + OAuth revoke endpoint |
| Connector event payloads | Not persisted beyond processing | Events held in-memory during rule evaluation only |
| Sentry telemetry | Sentry project retention (30 days default) | Sentry data deletion request |

## 7. Open Security Gaps (Prioritized)

1. **Certificate pinning** — no pinning on connector OAuth endpoints. Mitigated by system TLS but MITM possible on jailbroken devices.
2. **Wallet/balance fields not AEAD-protected** — only token fields use `TokenWrap`; ledger credits are plaintext in SQLite.
3. **No fuzz tests for webhook parser or rule DSL** — property-based testing planned in `fuzz/` but not wired to CI.
4. **Audit chain genesis not device-attested** — seed randomness is `SystemRandom`, not a Secure Enclave attestation.
5. **Rate-limit middleware not shared** — in-memory limiter in `phenotype-ops` is not extracted to a fleet-wide crate (tracked in cross-project reuse opportunities).

## 8. Out of Scope

- Backend infrastructure (services are scaffold; no production deployment exists).
- Apple App Store review process (separate from code security).
- Connector upstream security (Canvas, GCal, GitHub own their own security posture).
