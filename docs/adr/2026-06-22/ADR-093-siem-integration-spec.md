# ADR-093: SIEM integration specification — OpenTelemetry → SIEM shipping via pheno-otel (L46)

| Field | Value |
|---|---|
| **Status** | PROPOSED (v25 T2, 2026-06-22) |
| **Date** | 2026-06-22 |
| **Pillar** | L46 (Security observability — SIEM integration) |
| **Cycle** | 15 (v25) |
| **Author** | orchestrator (v25 T2) |
| **Sponsors** | fleet security circle (L5-155 SOC2 evidence track, L46 prior owner) |
| **Supersedes** | None (first SIEM-integration ADR) |
| **Related** | ADR-024 (71-pillar framework), ADR-042 (security cadence), ADR-051 (SOC2 evidence automation, forthcoming), ADR-078 (encryption at rest), ADR-080 (pen-test roadmap), NIST SP 800-92 (Guide to SIEM), OWASP ASVS V1.14 §1.7 / §7, SOC2 CC7.2 / CC7.3, ISO 27001 A.16.1.2 |
| **Plan ref** | `plans/2026-06-22-v25-71-pillar-cycle-15-p1.md` § T2 |
| **Companions** | `docs/siem/splunk-reference.md`, `docs/siem/elastic-reference.md` |

---

## 1. Context

The 71-pillar cycle-15 probe (`findings/2026-06-22-71-pillar-cycle-15-probe.md`) scored **L46 (Security observability — SIEM integration) at 2.5 / 3** — a structural gap in the security domain that blocks SOC2 CC7.2 / CC7.3 attestation and any customer MSA that names "forwarded audit telemetry to a SIEM". Driving findings:

1. **0 SIEM-integration policy** is documented anywhere in the fleet. No `docs/adr/2026-*` ADR covers L46, no `docs/siem/` reference runbook, no SOC2 control mapping for SIEM ingestion paths.
2. **3 distinct log/telemetry surfaces** are emitted today but land in 3 different places with no canonical collector:
   - **Structured application logs (JSON)** from 14 substrate crates via `pheno-tracing` (ADR-012 / ADR-036B): currently shipped to stdout only; no `fluent-bit` / `vector` sidecar configured for any production deployment.
   - **OTel traces + metrics** from 11 services via `pheno-otel` (ADR-036B substrate): shipped to a single OTLP collector at `otel-collector.phenotype.internal:4317` (gRPC); no OTLP-to-SIEM relay configured.
   - **Audit-log events** from 7 services (auth, RBAC, billing, federation, signing, key ceremony, force-push to `main`) via `pheno-events` (ADR-035B substrate): written to PostgreSQL only; no forwarding path.
3. **2 SOC2 findings** in the 2026-Q2 readiness review (carried in `findings/2026-06-22-L5-154-soc2-q2-readiness.md`) cite L46 directly:
   - **CC7.2 (Monitoring of system components):** "Audit-relevant events are not forwarded to a SIEM; detection of credential misuse, federated-token replay, or KMS key-ceremony anomaly requires a manual log-pull against the application database."
   - **CC7.3 (Evaluation of security events):** "No documented alert correlation rules exist for the 6 high-risk event classes enumerated in ADR-077 (Vault token issuance), ADR-078 (KMS key access), ADR-079 (OIDC token exchange), ADR-080 (pen-test findings), ADR-046 (mTLS handshake failure), ADR-046b (RFC 8693 token exchange)."
4. **4 customer contracts** signed in 2026-Q1 require SIEM forwarding within 90 days of contract signing (sox-replay / finra-replay / hipaa-batch / gdpr-batch). Three of those contracts are now out of contractual compliance because their 90-day window has elapsed.
5. **No OTLP-to-SIEM collector** is deployed in the fleet. The existing `otel-collector.phenotype.internal` instance runs the `otlpreceiver` only — no `splunk_hec` exporter, no `elasticsearch` exporter, no `file`-to-SIEM bridge.
6. **3 SIEM vendors** are in active conversation (Splunk Cloud, Elastic Cloud on AWS, Datadog Cloud SIEM). Without a vendor-neutral specification, every new vendor integration requires a one-off engineering effort.
7. **No alert taxonomy** maps the 6 high-risk event classes (per finding 3) to concrete SIEM queries. SOC2 CC7.3 requires "documented detection rules" — we have neither the rules nor the documentation.
8. **0 chaos tests** for SIEM pipeline failure modes. If the OTLP collector dies, no application-level degradation mode is documented. If the SIEM ingestion endpoint rate-limits, no back-pressure contract is defined.

This ADR defines a vendor-neutral SIEM integration specification built on top of the existing `pheno-otel` OTLP substrate (ADR-036B) so that the fleet gains first-class SIEM forwarding with minimal new surface area and without coupling to any one vendor's wire format.

---

## 2. Decision

We adopt a **three-layer SIEM shipping architecture** layered on top of the existing OTLP substrate, vendor-neutral at the SIEM boundary, with explicit per-vendor reference implementations for Splunk and Elastic (companion docs). The mandate covers:

### 2.1 Architecture — three layers

```
┌─────────────────────────────────────────────────────────────────────┐
│ Layer 1 — Application source (existing, no change required)         │
│                                                                     │
│  pheno-tracing (JSON structured logs, ADR-012 / ADR-036B)           │
│  pheno-otel   (OTLP traces + metrics, ADR-036B substrate)           │
│  pheno-events (audit events, ADR-035B substrate)                    │
│                                                                     │
│  All sources emit to the same `pheno-otel` OTLP collector.          │
└─────────────────────────────────────────────────────────────────────┘
                              │  OTLP/gRPC (port 4317)
                              │  OTLP/HTTP (port 4318)
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│ Layer 2 — OTLP collector with SIEM relay (NEW, this ADR)            │
│                                                                     │
│  otel-collector.phenotype.internal with:                             │
│    • receivers:  otlp (existing), filelog (NEW — stdout tailed)      │
│    • processors: batch, memory_limiter, attributes/siem-enrich       │
│    • exporters:  otlp/secondary (existing — keep long-term store),  │
│                  splunk_hec (NEW), elasticsearch (NEW),             │
│                  file/siem-archive (NEW — 90-day local buffer)      │
│                                                                     │
│  Fail-closed behavior: if SIEM endpoint is unreachable, batch is     │
│  spilled to disk (`/var/lib/otelcol/siem-buffer/`, 10 GB cap) and    │
│  retried with exponential backoff up to 24 h.                       │
└─────────────────────────────────────────────────────────────────────┘
                              │  Vendor-specific wire format
                              │  Splunk HEC (HTTPS/443, JSON event)
                              │  Elastic ECS (HTTPS/443, bulk API)
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│ Layer 3 — SIEM vendor (vendor-managed)                              │
│                                                                     │
│  Splunk Cloud   — index `phenotype`, sourcetypes per layer          │
│  Elastic Cloud  — index `phenotype-*`, data streams per layer       │
│  Datadog SIEM   — out of scope for this ADR (track-2 follow-up)     │
└─────────────────────────────────────────────────────────────────────┘
```

### 2.2 What ships (event classes)

The following 6 high-risk event classes are mandated for SIEM forwarding. Each class carries a `siem.class` OTel resource attribute (canonical taxonomy), a `siem.severity` attribute (RFC 5424 levels), and a `siem.cvss` attribute (informational; not all events carry CVSS).

| `siem.class` | Source | Event examples | Required attributes |
|---|---|---|---|
| `auth.login` | pheno-events | password login, OIDC callback, federated SSO | `user.id`, `auth.method`, `outcome` |
| `auth.token_issue` | pheno-events | JWT mint, refresh, RFC 8693 exchange | `token.id`, `subject`, `audience`, `ttl` |
| `rbac.permission_change` | pheno-events | role grant/revoke, policy update | `actor`, `target`, `permission`, `before`, `after` |
| `kms.key_access` | pheno-events (emitted by `pheno-context::kms`) | decrypt, rotate, ceremony, deny | `key.id`, `operation`, `actor`, `outcome` |
| `federation.handshake` | pheno-tracing span event | mTLS handshake success/fail, OCSP fail | `peer.spki_sha256`, `peer.org`, `outcome` |
| `release.signing` | pheno-events | cosign signing, SLSA provenance attach | `artifact.digest`, `signer`, `attestation` |

The full 22-class taxonomy (these 6 plus 16 lower-severity classes) is defined in `docs/siem/event-taxonomy.md` (forthcoming, v25 T0.5).

### 2.3 OTel attribute semantics (vendor-neutral envelope)

Every SIEM-bound event carries the following OTel resource / span / event attributes. The `siem.*` namespace is **mandatory** and **stable** (semver-locked per `pheno-otel` v0.7+).

| Attribute | Type | Cardinality | Required | Example |
|---|---|---|---|---|
| `siem.class` | string enum | 22 values | yes | `auth.token_issue` |
| `siem.severity` | int (0-7) | 8 values | yes | `4` (warning) |
| `siem.cvss` | float | 0.0-10.0 | no | `7.5` |
| `siem.correlation_id` | string (UUIDv7) | unbounded | yes | `0190a3b4-7c8d-7abc-9def-...` |
| `siem.retention_days` | int | 30/90/180/365 | yes | `90` |
| `siem.pii_redacted` | bool | 2 values | yes | `true` |
| `service.name` | string | low | yes (OTel std) | `phenotype-registry` |
| `service.version` | string | low | yes (OTel std) | `0.42.1` |
| `host.name` | string | low | yes (OTel std) | `ip-10-0-1-23.ec2.internal` |

The 22-class taxonomy is locked to a string-enum; adding a new class requires a major-version bump of `pheno-otel` (mirrors the OTel semver guarantee for `semconv`).

### 2.4 PII redaction policy (mandatory before SIEM shipping)

Per `pheno-events` ADR-035B §4.2, the SIEM-enrich processor MUST run `attributes/siem-pii-redact` before any vendor exporter. Redaction rules:

| Field | Pattern | Redaction |
|---|---|---|
| `password` / `passphrase` | any value | full replacement with `***REDACTED***` |
| `api_key` / `token` (bearer) | any value | SHA-256 prefix (first 8 hex chars only); full value dropped |
| `email` | RFC 5322 | keep domain only (`***@example.com`) |
| `phone` | E.164 | keep country code only (`+1***`) |
| `ssn` / `national_id` | any value | full replacement |
| `authorization` header | HTTP header | drop entire header |
| `cookie` header | HTTP header | drop entire header |

PII redaction is implemented as an OTel collector processor (Go) and as a `serde` derive at the `pheno-events` emission site. Two layers so that an attacker who bypasses one is still caught by the other (defense in depth).

### 2.5 Fail-closed semantics

The OTLP collector MUST NOT silently drop SIEM-bound events. The contract:

| Condition | Behavior |
|---|---|
| SIEM endpoint reachable, HTTP 2xx | forward + ack |
| SIEM endpoint HTTP 4xx (bad request) | drop event + emit `siem.drop` metric with reason |
| SIEM endpoint HTTP 5xx (server error) | retry with exponential backoff (max 5 attempts) |
| SIEM endpoint unreachable (network error) | retry with exponential backoff; spill to `/var/lib/otelcol/siem-buffer/` (10 GB cap, oldest evicted) |
| Local buffer full + endpoint still unreachable | drop event + emit `siem.drop` metric with reason `buffer_overflow` + PagerDuty page (`severity: page`) |
| Collector process crashes | systemd restarts; on-disk buffer survives restart; PagerDuty alert on restart-loop (3 restarts in 5 min) |

The 24-hour maximum buffer window is a deliberate bound — events older than 24 h are dropped with `reason: buffer_overflow` rather than forwarded late (forwarded-late events would corrupt SOC2 timelines).

### 2.6 Out-of-scope (explicitly NOT covered by this ADR)

- **Non-OTel legacy log sources** (plain `syslog`, journald) — covered by track-2 follow-up ADR (per ADR-042 monthly cadence).
- **Datadog Cloud SIEM** wire format — track-2 follow-up ADR (this ADR is vendor-neutral but only ships reference impls for Splunk + Elastic).
- **Real-time alerting** — this ADR covers forwarding + retention; alert-rule authoring is in `docs/siem/splunk-reference.md` §4 and `docs/siem/elastic-reference.md` §4.
- **Tier-1 forensics** (full-packet capture, memory dump) — out of scope; SOC2 CC7.2 does not require PCAP.
- **On-prem SIEM** (Splunk Enterprise self-hosted, Elastic self-managed) — out of scope; customer contracts name cloud SIEM only.

---

## 3. Per-vendor reference implementations (companion docs)

| Vendor | Path | Wire format | Status |
|---|---|---|---|
| Splunk Cloud | `docs/siem/splunk-reference.md` | Splunk HEC (HTTP Event Collector), JSON event over HTTPS | Reference impl complete (this PR) |
| Elastic Cloud on AWS | `docs/siem/elastic-reference.md` | Elastic Common Schema (ECS) via Bulk API | Reference impl complete (this PR) |
| Datadog Cloud SIEM | (forthcoming) | Datadog Logs API | Track-2 follow-up |
| Microsoft Sentinel | (out of scope, this ADR) | Log Analytics ingestion API | Track-3 follow-up |
| Google Chronicle | (out of scope, this ADR) | Chronicle Ingestion API | Track-3 follow-up |

Each reference doc covers:

1. **Wire-format mapping** (OTel attribute → vendor field)
2. **Config YAML** (otel-collector exporter block)
3. **Index / data-stream strategy** (vendor-side)
4. **Alert rules** (≥3 production rules per vendor; full set in §4 of each doc)

---

## 4. Sample alert rules (per vendor)

The alert taxonomy is identical across vendors (vendor-neutral query intent); the query syntax differs. Each reference doc ships the full set of 6 mandatory rules (one per high-risk event class) plus 3 vendor-specific operational rules.

### 4.1 Common query intent (all SIEMs)

| Rule ID | Intent | Severity |
|---|---|---|
| `SIEM-AUTH-001` | Detect > 5 failed `auth.login` for a single `user.id` in 5 min | high |
| `SIEM-AUTH-002` | Detect `auth.token_issue` with `outcome=deny` for any `key.id` in KMS-denied list | critical |
| `SIEM-RBAC-001` | Detect `rbac.permission_change` to a `permission=admin` from outside a maintenance window | critical |
| `SIEM-KMS-001` | Detect `kms.key_access` with `operation=ceremony` outside scheduled key-ceremony window | critical |
| `SIEM-FED-001` | Detect `federation.handshake` with `outcome=fail` count > 10/min from a single `peer.org` | high |
| `SIEM-REL-001` | Detect `release.signing` where `attestation` does not include `slsa_provenance.v1` | high |

The full rule set (22 rules, one per `siem.class` taxonomy entry) is in `docs/siem/event-taxonomy.md` (forthcoming, v25 T0.5).

---

## 5. Phased Rollout (3 weeks, this T2 scope)

### Phase 1 — Reference impls + dry-run (week 1, 2026-06-22 → 2026-06-28)

1. Land this ADR + 2 reference docs (this PR).
2. Provision Splunk Cloud trial index `phenotype-dryrun` + Elastic Cloud trial data stream `phenotype-dryrun-*`.
3. Deploy a staging-only `otel-collector` instance with both exporters enabled.
4. Forward a 72-hour synthetic load (200 events/sec, 6 classes, mix of severity).
5. Verify vendor-side ingestion latency (target: p95 < 30 s end-to-end).
6. Verify alert-rule firing (6 mandatory rules, 1 synthetic trigger per rule).

### Phase 2 — Production cutover (week 2, 2026-06-29 → 2026-06-05)

1. Enable forwarding for `staging` environment first (1-day soak).
2. Enable forwarding for `production` environment (3-day soak with PagerDuty `severity: page` on `siem.drop`).
3. Run chaos tests per ADR-042 monthly cadence:
   - kill `otel-collector` mid-flow; verify on-disk buffer fills + drains on restart.
   - inject 4xx + 5xx errors from Splunk HEC endpoint; verify drop metrics fire.
   - inject network partition; verify 24-h buffer bound.
4. Sign off on SOC2 CC7.2 / CC7.3 evidence for the 2026-Q3 readiness review.

### Phase 3 — Customer-acceptance + contract closure (week 3, 2026-07-06 → 2026-07-12)

1. Provide each of the 4 out-of-compliance customer accounts with a SIEM ingestion guide (Splunk HEC endpoint + index name + retention guarantee).
2. Walk through 1 acceptance test per customer (typically: "show me a successful auth.login event landing in our SIEM within 60 s of the user logging in").
3. Update the 4 customer MSAs with the SIEM forwarding acknowledgment.
4. Land `docs/siem/event-taxonomy.md` (forthcoming; not blocking this ADR).

---

## 6. Consequences

### 6.1 Positive

- **SOC2 CC7.2 / CC7.3 closure** — the 2 outstanding readiness findings (per `findings/2026-06-22-L5-154-soc2-q2-readiness.md`) are addressed by the rollout in §5.
- **4 customer MSA compliance** — 90-day SIEM-forwarding clauses are now satisfiable.
- **Vendor neutrality** — the wire-format mapping is contained in the collector's exporter block; switching SIEM vendors does not require application changes.
- **Defense in depth on PII** — redaction runs both at emission (`pheno-events`) and at the collector boundary, so an attacker who bypasses one is still caught by the other.
- **Fail-closed by default** — no silent drops; the on-disk buffer caps the blast radius of any SIEM-side outage.

### 6.2 Negative / costs

- **New operational surface** — `otel-collector` gains 2 vendor exporters and 1 on-disk buffer; needs monitoring + alerting (new work, ~50 LOC runbook in `docs/siem/splunk-reference.md` §5).
- **Vendor-side cost** — Splunk Cloud ingest at 200 events/sec sustained = ~17 GB/day = ~$1,500/month at list price; Elastic Cloud equivalent = ~$800/month (ingest-based pricing). Net fleet cost increase: ~$2,300/month. Acceptable per `findings/2026-06-22-L5-156-finops-q2.md` §3.
- **PII redaction false-negatives** — the regex-based rules in §2.4 cover the 7 known patterns but a novel PII shape (e.g. a new national-ID format) could leak through. Mitigation: a follow-up ADR (track-3) introduces a tokenization-based redaction layer.
- **Buffer-eviction = data loss** — events older than 24 h that haven't been forwarded are dropped. SOC2 timelines are forgiving here (evidence windows are quarterly, not daily) but customer MSA timelines may not be. Mitigation: a customer-specific override via the `siem.retention_days` attribute on the per-customer alert rule.

### 6.3 Neutral / risks

- **OTel attribute schema stability** — the `siem.*` namespace is semver-locked to `pheno-otel` v0.7+. Any attribute addition requires a major-version bump. Acceptable: we already gate attribute additions this way for the OTel semconv substrate.
- **SIEM vendor API churn** — Splunk HEC and Elastic ECS both evolved in the past 24 months. The reference impls in the companion docs are pinned to specific vendor versions (Splunk Cloud 9.x, Elastic 8.x); future versions may need a reference-doc refresh.
- **Datadog coverage gap** — track-2 follow-up ADR needed for the 1 customer whose MSA names Datadog Cloud SIEM.

---

## 7. Validation plan

| Validation | Method | Pass criterion | When |
|---|---|---|---|
| OTLP attribute schema stability | `pheno-otel` schema-registry CI gate | `siem.*` namespace locked to v0.7+ | Phase 1 (this PR) |
| Splunk HEC wire-format mapping | end-to-end test against Splunk Cloud trial | 200 events/sec land in `phenotype-dryrun` index | Phase 1 week 1 |
| Elastic ECS wire-format mapping | end-to-end test against Elastic Cloud trial | 200 events/sec land in `phenotype-dryrun-*` data streams | Phase 1 week 1 |
| PII redaction (7 patterns) | chaos test with synthetic PII inputs | 7/7 patterns redacted before exporter | Phase 1 week 1 |
| Fail-closed buffer semantics | chaos test: kill collector, fill buffer, restart | no event loss for buffer-resident events | Phase 2 week 2 |
| Alert-rule firing (6 mandatory) | synthetic trigger per rule | 6/6 rules fire within 60 s of trigger | Phase 1 week 1 |
| Customer-acceptance (4 MSAs) | walk-through with each customer's security team | written sign-off from each | Phase 3 week 3 |

---

## 8. References

### 8.1 Internal

- ADR-024 (71-pillar framework)
- ADR-035B (event-bus substrate consolidation)
- ADR-036B (`pheno-otel` substrate canonical)
- ADR-042 (security audit cadence — monthly)
- ADR-077 (Vault migration roadmap)
- ADR-078 (encryption at rest mandate)
- ADR-079 (OIDC federation reference)
- ADR-080 (pen-test + bug-bounty roadmap)
- `findings/2026-06-22-L5-154-soc2-q2-readiness.md` (the 2 SOC2 findings this ADR closes)
- `findings/2026-06-22-L5-156-finops-q2.md` (the cost justification)
- `findings/2026-06-22-71-pillar-cycle-15-probe.md` (cycle-15 scorecard)
- `plans/2026-06-22-v25-71-pillar-cycle-15-p1.md` (v25 plan)

### 8.2 External standards

- NIST SP 800-92 — Guide to Computer Security Log Management (2006)
- NIST SP 800-137 — Information Security Continuous Monitoring (ISCM)
- OWASP ASVS v4.0.3 §1.7 (Error logging), §7 (Error handling and logging)
- SOC2 Trust Services Criteria CC7.2 (System monitoring), CC7.3 (Security event evaluation)
- ISO 27001:2022 A.16.1.2 (Reporting information security events)
- ISO 27001:2022 A.12.4.1 (Event logging)
- CNCF Cloud Native Security Whitepaper §4.5 (Observability)
- OpenTelemetry Semantic Conventions v1.27 (resource / event attribute stability)
- Splunk HEC documentation (current as of 2026-Q2)
- Elastic Common Schema (ECS) v8.x reference

---

## 9. Decision log

- **2026-06-22:** PROPOSED (this PR) — v25 T2.
- **2026-06-22 (target, post-Phase-1 dry-run):** ACCEPTED, conditional on §5 phase-1 dry-run results.
- **2026-06-29 (target, post-Phase-2 prod cutover):** ACCEPTED, full.
- **ADR refresh cadence:** quarterly (per ADR-041 weekly refresh cadence applies only to 71-pillar scorecard; ADR refresh is per §5 phased-rollout gate).
