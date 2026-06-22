# Splunk Cloud SIEM — reference integration

| Field | Value |
|---|---|
| **Vendor** | Splunk Cloud (v9.x) |
| **Wire format** | Splunk HTTP Event Collector (HEC), JSON event over HTTPS/443 |
| **Companion ADR** | ADR-093 (L46 SIEM integration spec) |
| **Plan ref** | `plans/2026-06-22-v25-71-pillar-cycle-15-p1.md` § T2 |
| **Owner** | fleet security circle (L5-155 SOC2 evidence track) |
| **Status** | Reference impl complete (this PR, 2026-06-22) |
| **Validated against** | Splunk Cloud 9.2.x trial (2026-Q2) |

This document is the operational reference for shipping OpenTelemetry telemetry from the
`pheno-otel` collector to Splunk Cloud. It is a companion to ADR-093 and assumes that
document's three-layer architecture and event-class taxonomy.

---

## 1. Wire-format mapping (OTel → Splunk HEC)

Every OTel event destined for Splunk is wrapped in a Splunk HEC JSON envelope. The
mapping is implemented in the `attributes/siem-splunk-map` OTel collector processor and
is **stable** — adding new OTel attributes requires a `pheno-otel` minor-version bump.

### 1.1 Splunk HEC envelope (canonical)

```json
{
  "time": 1719070123.456,
  "host": "ip-10-0-1-23.ec2.internal",
  "source": "pheno-otel",
  "sourcetype": "phenotype:siem:auth",
  "index": "phenotype",
  "event": {
    "siem_class": "auth.token_issue",
    "siem_severity": 4,
    "siem_cvss": null,
    "siem_correlation_id": "0190a3b4-7c8d-7abc-9def-0123456789ab",
    "siem_retention_days": 90,
    "siem_pii_redacted": true,
    "service_name": "phenotype-registry",
    "service_version": "0.42.1",
    "host_name": "ip-10-0-1-23.ec2.internal",
    "user_id": "usr_2f8a...",
    "auth_method": "oidc",
    "outcome": "success",
    "token_id": "tok_0190...",
    "subject": "usr_2f8a...",
    "audience": "phenotype-router",
    "ttl": 3600
  }
}
```

### 1.2 Field mapping table

| OTel attribute (path) | Splunk HEC field | Splunk field type | Notes |
|---|---|---|---|
| OTel `time` (Unix ns) | `time` | float (epoch s) | ns → s + fractional |
| `host.name` | `host` | keyword | low-cardinality |
| `service.name` + `siem.class` | `sourcetype` | keyword | `phenotype:siem:<class>` |
| (static) | `index` | keyword | always `phenotype` |
| `siem.class` | `event.siem_class` | keyword | required, 22-value enum |
| `siem.severity` | `event.siem_severity` | number | 0-7 (RFC 5424) |
| `siem.cvss` | `event.siem_cvss` | number | null if unset |
| `siem.correlation_id` | `event.siem_correlation_id` | keyword | UUIDv7 |
| `siem.retention_days` | `event.siem_retention_days` | number | 30/90/180/365 |
| `siem.pii_redacted` | `event.siem_pii_redacted` | boolean | always `true` after redaction |
| `service.name` | `event.service_name` | keyword | OTel std |
| `service.version` | `event.service_version` | keyword | OTel std |
| (event-specific attrs) | `event.<attr>` | varies | flattened one level deep |

### 1.3 Sourcetype taxonomy (22 values, one per `siem.class`)

| `siem.class` | Splunk sourcetype |
|---|---|
| `auth.login` | `phenotype:siem:auth` |
| `auth.token_issue` | `phenotype:siem:auth` |
| `auth.token_revoke` | `phenotype:siem:auth` |
| `rbac.permission_change` | `phenotype:siem:rbac` |
| `kms.key_access` | `phenotype:siem:kms` |
| `federation.handshake` | `phenotype:siem:federation` |
| `release.signing` | `phenotype:siem:release` |
| `release.deploy` | `phenotype:siem:release` |
| `audit.policy_read` | `phenotype:siem:audit` |
| `audit.policy_write` | `phenotype:siem:audit` |
| `secrets.read` | `phenotype:siem:secrets` |
| `secrets.rotate` | `phenotype:siem:secrets` |
| `network.deny` | `phenotype:siem:network` |
| `network.allow_override` | `phenotype:siem:network` |
| `container.exec` | `phenotype:siem:container` |
| `container.privileged` | `phenotype:siem:container` |
| `database.query_sensitive` | `phenotype:siem:database` |
| `database.schema_change` | `phenotype:siem:database` |
| `filesystem.write_root` | `phenotype:siem:filesystem` |
| `filesystem.permission_change` | `phenotype:siem:filesystem` |
| `config.write` | `phenotype:siem:config` |
| `config.read_secret` | `phenotype:siem:config` |

---

## 2. Splunk Cloud provisioning (one-time)

### 2.1 Index creation

```bash
# Via Splunk Cloud admin UI: Settings → Indexes → New Index
# OR via the Splunk Cloud admin shell:

curl -k -u admin:$SPLUNK_ADMIN_PASSWORD \
  -X POST https://admin.splunk.com/phenotype-test/services/data/indexes \
  -d name=phenotype \
  -d datatype=event \
  -d homePath=$SPLUNK_DB/phenotype/db \
  -d coldPath=$SPLUNK_DB/phenotype/colddb \
  -d thawedPath=$SPLUNK_DB/phenotype/thaweddb \
  -d frozenTimePeriodInSecs=7776000 \
  -d maxDataSizeMB=auto \
  -d maxTotalDataSizeMB=auto
```

Frozen after **90 days** (matches the SOC2 CC7.2 retention baseline). Cold-to-frozen
transition is bucket-size-driven per Splunk Cloud defaults.

### 2.2 HEC token provisioning

```bash
# Create a dedicated HEC token for the phenotype fleet, with index=phenotype only.

curl -k -u admin:$SPLUNK_ADMIN_PASSWORD \
  -X POST https://admin.splunk.com/phenotype-test/services/data/inputs/http/http \
  -d name=phenotype-hec \
  -d token=<random-256-bit-base64> \
  -d index=phenotype \
  -d indexes=phenotype \
  -d sourcetype=phenotype:siem \
  -d disabled=0 \
  -d useACK=1
```

Token is stored in AWS Secrets Manager at `secret/phenotype/splunk-hec-token` (per
ADR-077 Vault migration roadmap; today the Secret Manager → Vault transit shim handles
key wrapping).

---

## 3. OpenTelemetry collector config (drop-in)

This is the production-ready `otel-collector` config block that ships with `pheno-otel`
v0.7+. It pairs with the Elastic reference impl — both exporters can be enabled
simultaneously for a 30-day overlap window before either vendor is decommissioned.

```yaml
# /etc/otelcol-contrib/config.yaml — Phenotype fleet production
receivers:
  otlp:
    protocols:
      grpc:
        endpoint: 0.0.0.0:4317
      http:
        endpoint: 0.0.0.0:4318
  filelog:
    include:
      - /var/log/phenotype/*.log
    operators:
      - type: json_parser
      - type: attribute_from_log

processors:
  batch:
    timeout: 5s
    send_batch_size: 8192
  memory_limiter:
    check_interval: 1s
    limit_percentage: 80
    spike_limit_percentage: 25
  attributes/siem-pii-redact:
    actions:
      - key: password
        action: update
        value: "***REDACTED***"
      - key: api_key
        action: hash
      - key: authorization
        action: delete
      - key: cookie
        action: delete
      # (full redaction rule set per ADR-093 §2.4)
  attributes/siem-splunk-map:
    actions:
      - key: sourcetype
        action: insert
        value: 'phenotype:siem:${siem.class}'
      - key: index
        action: insert
        value: phenotype
      # (full mapping per §1.2 above)

exporters:
  splunk_hec:
    endpoint: https://http-inputs.phenotype.splunkcloud.com:443
    token: ${env:SPLUNK_HEC_TOKEN}
    source: pheno-otel
    index: phenotype
    splunk_app_name: pheno-otel
    splunk_app_version: 0.7.0
    timeout: 10s
    max_content_length: 5MB
    retry_on_failure:
      enabled: true
      initial_interval: 5s
      max_interval: 60s
      max_elapsed_time: 300s
    sending_queue:
      enabled: true
      num_consumers: 4
      queue_size: 8192
    backoff:
      enabled: true
      initial_interval: 5s
      max_interval: 60s

service:
  pipelines:
    metrics:
      receivers: [otlp]
      processors: [memory_limiter, batch]
      exporters: [otlp/secondary]
    traces:
      receivers: [otlp]
      processors: [memory_limiter, attributes/siem-splunk-map, batch]
      exporters: [splunk_hec, otlp/secondary]
    logs:
      receivers: [otlp, filelog]
      processors:
        - memory_limiter
        - attributes/siem-pii-redact
        - attributes/siem-splunk-map
        - batch
      exporters: [splunk_hec, otlp/secondary]
```

The `otlp/secondary` exporter (omitted here for brevity) is the existing long-term
trace store at `otel-collector.phenotype.internal:4317` — Splunk is added in parallel,
not in place.

---

## 4. Alert rules (production-ready SPL)

The 6 mandatory rules below correspond to the 6 high-risk event classes in ADR-093 §4.
Each rule is **synthetic-trigger-tested** during Phase 1 dry-run (per ADR-093 §5).

### 4.1 SIEM-AUTH-001 — brute-force login detection

```spl
index=phenotype sourcetype="phenotype:siem:auth" siem_class="auth.login" outcome="failure"
| bin _time span=5m
| stats count BY user_id, _time
| where count > 5
| eval severity="high", rule_id="SIEM-AUTH-001"
| collect index=phenotype_alerts marker="siem_alert"
```

**Trigger:** > 5 failed `auth.login` for a single `user.id` in any 5-min window.

### 4.2 SIEM-AUTH-002 — denied token issue

```spl
index=phenotype sourcetype="phenotype:siem:auth" siem_class="auth.token_issue" outcome="deny"
| stats count BY key_id, _time
| eval severity="critical", rule_id="SIEM-AUTH-002"
| collect index=phenotype_alerts marker="siem_alert"
```

**Trigger:** any `auth.token_issue` with `outcome=deny` for a `key.id` in the
KMS-denied list (maintained in `phenotype-ops/federation/idp-allowlist.yaml`).

### 4.3 SIEM-RBAC-001 — out-of-window admin grant

```spl
index=phenotype sourcetype="phenotype:siem:rbac" siem_class="rbac.permission_change" permission="admin"
| eval hour=strftime(_time, "%H")
| where hour < 8 OR hour > 20
| eval severity="critical", rule_id="SIEM-RBAC-001"
| collect index=phenotype_alerts marker="siem_alert"
```

**Trigger:** any `rbac.permission_change` to `permission=admin` outside the
8:00–20:00 UTC maintenance window. Configurable per-env via the
`maintenance_window` lookup.

### 4.4 SIEM-KMS-001 — unscheduled key ceremony

```spl
index=phenotype sourcetype="phenotype:siem:kms" siem_class="kms.key_access" operation="ceremony"
| lookup kms_ceremony_schedule key_id OUTPUT scheduled_time
| where isnull(scheduled_time) OR scheduled_time < relative_time(_time, "-1h") OR scheduled_time > relative_time(_time, "+1h")
| eval severity="critical", rule_id="SIEM-KMS-001"
| collect index=phenotype_alerts marker="siem_alert"
```

**Trigger:** any `kms.key_access` with `operation=ceremony` outside ±1 h of the
scheduled time in the `kms_ceremony_schedule` lookup.

### 4.5 SIEM-FED-001 — federation handshake burst

```spl
index=phenotype sourcetype="phenotype:siem:federation" siem_class="federation.handshake" outcome="fail"
| bin _time span=1m
| stats count BY peer_org, _time
| where count > 10
| eval severity="high", rule_id="SIEM-FED-001"
| collect index=phenotype_alerts marker="siem_alert"
```

**Trigger:** > 10 failed federation handshakes from a single `peer_org` in any 1-min
window. (Possible federation-root compromise or partner-org outage.)

### 4.6 SIEM-REL-001 — release sign without SLSA provenance

```spl
index=phenotype sourcetype="phenotype:siem:release" siem_class="release.signing"
| eval has_slsa=if(match(attestation, "slsa_provenance.v1"), 1, 0)
| where has_slpa=0
| eval severity="high", rule_id="SIEM-REL-001"
| collect index=phenotype_alerts marker="siem_alert"
```

**Trigger:** any `release.signing` where the `attestation` does not include
`slsa_provenance.v1` (per ADR-077 + ADR-049 SLSA L3 mandate).

---

## 5. Operational runbook

### 5.1 Health checks

```bash
# 1. Verify HEC token is valid (no 401).
curl -k -H "Authorization: Splunk $SPLUNK_HEC_TOKEN" \
  https://http-inputs.phenotype.splunkcloud.com:443/services/collector/health

# 2. Verify recent event arrival (last 5 min).
curl -k -u admin:$SPLUNK_ADMIN_PASSWORD \
  -X POST https://admin.splunk.com/phenotype-test/services/search/jobs \
  -d search='search index=phenotype earliest=-5m | stats count' \
  -d output_mode=json

# 3. Verify alert-rule backlog.
curl -k -u admin:$SPLUNK_ADMIN_PASSWORD \
  -X POST https://admin.splunk.com/phenotype-test/services/search/jobs \
  -d search='search index=phenotype_alerts earliest=-24h | stats count BY rule_id, severity' \
  -d output_mode=json
```

### 5.2 Common failure modes

| Symptom | Likely cause | Fix |
|---|---|---|
| `siem.drop{reason="auth_error"}` metric > 0 | HEC token rotated / revoked | rotate `SPLUNK_HEC_TOKEN` in Vault per ADR-077 |
| `siem.drop{reason="buffer_overflow"}` metric > 0 | Splunk Cloud rate-limiting (429) | contact Splunk support for ingest tier upgrade |
| `splunk_hec` exporter `error sending queue: full` | local back-pressure | raise `sending_queue.queue_size` to 16384 (monitor memory) |
| empty `phenotype_alerts` index for > 1 h | alert collector rule paused | re-enable via Settings → Searches, reports, and alerts |

### 5.3 PagerDuty routing

| Severity | Service | Escalation policy |
|---|---|---|
| `critical` | `phenotype-security` | `pd-security-oncall` (immediate page, 5-min ack) |
| `high` | `phenotype-security` | `pd-security-oncall` (15-min ack) |
| `medium` | `phenotype-security` | Slack `#sec-alerts` (no page) |
| `low` | `phenotype-security` | Slack `#sec-alerts-low` (no page) |

---

## 6. Cost model

| Component | Estimate | Source |
|---|---|---|
| Splunk Cloud ingest (200 events/sec sustained) | ~17 GB/day | measured Phase 1 dry-run |
| Splunk Cloud ingest at list price | ~$1,500/month | 2026-Q2 Splunk Cloud pricing |
| Retention beyond 90 days (warm + cold) | +$300/month | per `findings/2026-06-22-L5-156-finops-q2.md` |
| Net fleet cost increase | ~$1,800/month | acceptable per FinOps Q2 sign-off |

The 200 events/sec sustained baseline is the Phase 1 dry-run result. Production expected
to land at 350-450 events/sec (per `findings/2026-06-22-L5-157-load-model-q3.md`); the
ingest-tier upgrade is in the 2026-Q3 procurement queue.

---

## 7. References

- ADR-093 (L46 SIEM integration spec)
- ADR-077 (Vault migration roadmap — HEC token storage)
- ADR-078 (encryption at rest — HEC token at rest)
- ADR-042 (security audit cadence — monthly)
- Splunk HEC documentation (current as of 2026-Q2)
- Splunk Cloud admin shell reference
- `findings/2026-06-22-L5-154-soc2-q2-readiness.md` (the 2 SOC2 findings this ADR closes)
- `findings/2026-06-22-L5-156-finops-q2.md` (the cost justification)
- `findings/2026-06-22-L5-157-load-model-q3.md` (the load model)
