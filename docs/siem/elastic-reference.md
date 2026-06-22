# Elastic Cloud SIEM — reference integration (ECS)

| Field | Value |
|---|---|
| **Vendor** | Elastic Cloud on AWS (Elastic 8.x) |
| **Wire format** | Elasticsearch Bulk API, Elastic Common Schema (ECS) v8.x |
| **Companion ADR** | ADR-093 (L46 SIEM integration spec) |
| **Plan ref** | `plans/2026-06-22-v25-71-pillar-cycle-15-p1.md` § T2 |
| **Owner** | fleet security circle (L5-155 SOC2 evidence track) |
| **Status** | Reference impl complete (this PR, 2026-06-22) |
| **Validated against** | Elastic Cloud 8.15.x trial (2026-Q2) |

This document is the operational reference for shipping OpenTelemetry telemetry from the
`pheno-otel` collector to Elastic Cloud on AWS using the Elastic Common Schema (ECS).
It is a companion to ADR-093 and assumes that document's three-layer architecture and
event-class taxonomy.

---

## 1. Wire-format mapping (OTel → ECS)

Every OTel event destined for Elastic is transformed to an ECS-compliant JSON document
and indexed via the Bulk API. The mapping is implemented in the
`attributes/siem-elastic-ecs-map` OTel collector processor and is **stable** — adding
new OTel attributes requires a `pheno-otel` minor-version bump.

### 1.1 ECS document (canonical)

```json
{
  "@timestamp": "2026-06-22T19:45:23.456Z",
  "ecs": { "version": "8.11.0" },
  "event": {
    "kind": "event",
    "category": ["authentication"],
    "type": ["info"],
    "outcome": "success",
    "severity_name": "warning",
    "severity_id": 4,
    "dataset": "phenotype.siem.auth",
    "module": "phenotype",
    "correlation_id": "0190a3b4-7c8d-7abc-9def-0123456789ab",
    "duration": 0
  },
  "host": {
    "name": "ip-10-0-1-23.ec2.internal",
    "hostname": "ip-10-0-1-23",
    "os": { "type": "linux" }
  },
  "service": {
    "name": "phenotype-registry",
    "version": "0.42.1",
    "environment": "production"
  },
  "user": {
    "id": "usr_2f8a...",
    "name": "koosha.pari"
  },
  "phenotype": {
    "siem": {
      "class": "auth.token_issue",
      "cvss": null,
      "retention_days": 90,
      "pii_redacted": true,
      "auth_method": "oidc",
      "token_id": "tok_0190...",
      "subject": "usr_2f8a...",
      "audience": "phenotype-router",
      "ttl": 3600
    }
  }
}
```

### 1.2 Field mapping table

| OTel attribute (path) | ECS field | ECS type | Notes |
|---|---|---|---|
| OTel `time` (Unix ns) | `@timestamp` | date | ns → ISO 8601 with ms precision |
| `service.name` | `service.name` | keyword | OTel std |
| `service.version` | `service.version` | keyword | OTel std |
| (env var) | `service.environment` | keyword | `production` / `staging` / `dev` |
| `host.name` | `host.name` | keyword | OTel std |
| (detected) | `host.os.type` | keyword | `linux` / `darwin` / `windows` |
| `siem.class` | `phenotype.siem.class` | keyword | required, 22-value enum |
| (mapped) | `event.dataset` | keyword | `phenotype.siem.<class>` |
| (mapped) | `event.module` | keyword | always `phenotype` |
| `siem.severity` | `event.severity_id` | long | 0-7 (RFC 5424) |
| (mapped) | `event.severity_name` | keyword | `debug` / `info` / `notice` / ... |
| `siem.class` (first token) | `event.category` | keyword | `authentication` / `iam` / ... |
| (mapped) | `event.type` | keyword | `info` / `creation` / `deletion` / `change` |
| `outcome` | `event.outcome` | keyword | `success` / `failure` / `unknown` |
| `siem.correlation_id` | `event.correlation_id` | keyword | UUIDv7 |
| `user.id` | `user.id` | keyword | OTel `enduser.id` → ECS `user.id` |
| `user.name` | `user.name` | keyword | OTel `enduser.name` → ECS `user.name` |
| `siem.cvss` | `phenotype.siem.cvss` | float | null if unset |
| `siem.retention_days` | `phenotype.siem.retention_days` | long | 30/90/180/365 |
| `siem.pii_redacted` | `phenotype.siem.pii_redacted` | boolean | always `true` after redaction |
| (event-specific attrs) | `phenotype.siem.<attr>` | varies | namespaced under `phenotype.siem.*` |

### 1.3 Data-stream taxonomy (one per `siem.class` family)

Elastic data streams replace Splunk sourcetypes for this integration. One data stream
per `event.category` to keep retention and ILM policy uniform within a category.

| `event.category` | Data stream | ILM policy |
|---|---|---|
| `authentication` | `phenotype-siem-auth-*` | `phenotype-90d` |
| `iam` | `phenotype-siem-iam-*` | `phenotype-90d` |
| `iam` (RBAC sub-cat) | `phenotype-siem-rbac-*` | `phenotype-365d` (compliance) |
| `iam` (KMS sub-cat) | `phenotype-siem-kms-*` | `phenotype-365d` (compliance) |
| `network` | `phenotype-siem-network-*` | `phenotype-90d` |
| `process` | `phenotype-siem-process-*` | `phenotype-90d` |
| `file` | `phenotype-siem-file-*` | `phenotype-180d` |
| `database` | `phenotype-siem-database-*` | `phenotype-180d` |
| `configuration` | `phenotype-siem-config-*` | `phenotype-365d` (compliance) |
| `package` (release signing) | `phenotype-siem-release-*` | `phenotype-365d` (SLSA) |

---

## 2. Elastic Cloud provisioning (one-time)

### 2.1 Deployment + index templates

```bash
# 1. Create the deployment (via Elastic Cloud UI or API).
#    - Stack version: 8.15.x
#    - Region: us-east-1 (matches primary AWS region per ADR-046)
#    - Size: 4 GB RAM / 32 GB storage (production scale)

# 2. Create the ILM policies.
curl -u elastic:$ELASTIC_PASSWORD \
  -X PUT https://phenotype.es.us-east-1.aws.found.io/_ilm/policy/phenotype-90d \
  -H "Content-Type: application/json" \
  -d '{
    "policy": {
      "phases": {
        "hot":   { "min_age": "0ms",  "actions": { "rollover": { "max_age": "1d",  "max_primary_shard_size": "10gb" } } },
        "warm":  { "min_age": "7d",   "actions": { "shrink": { "number_of_shards": 1 }, "forcemerge": { "max_num_segments": 1 } } },
        "cold":  { "min_age": "30d",  "actions": { "freeze": {} } },
        "delete":{ "min_age": "90d",  "actions": { "delete": {} } }
      }
    }
  }'

# (repeat for phenotype-180d / phenotype-365d with adjusted `min_age` in `delete` phase)

# 3. Create the index template that wires the data streams to the ILM policy.
curl -u elastic:$ELASTIC_PASSWORD \
  -X PUT https://phenotype.es.us-east-1.aws.found.io/_index_template/phenotype-siem \
  -H "Content-Type: application/json" \
  -d '{
    "index_patterns": ["phenotype-siem-*"],
    "template": {
      "settings": {
        "index.lifecycle.name": "phenotype-90d",
        "index.lifecycle.rollover_alias": "phenotype-siem-auth"
      },
      "mappings": {
        "properties": {
          "phenotype": {
            "properties": {
              "siem": {
                "properties": {
                  "class":           { "type": "keyword" },
                  "correlation_id":  { "type": "keyword" },
                  "cvss":            { "type": "float" },
                  "retention_days":  { "type": "long" },
                  "pii_redacted":    { "type": "boolean" }
                }
              }
            }
          }
        }
      }
    },
    "data_stream": {}
  }'
```

### 2.2 API key provisioning

```bash
# Create a dedicated API key for the phenotype fleet, scoped to phenotype-siem-* only.

curl -u elastic:$ELASTIC_PASSWORD \
  -X POST https://phenotype.es.us-east-1.aws.found.io/_security/api_key \
  -H "Content-Type: application/json" \
  -d '{
    "name": "pheno-otel-collector",
    "role_descriptors": {
      "pheno-otel-writer": {
        "cluster": ["monitor", "manage_index_templates"],
        "index": [
          {
            "names": ["phenotype-siem-*"],
            "privileges": ["write", "create_index", "manage_data_stream_lifecycle"]
          }
        ]
      }
    },
    "metadata": {
      "application": "pheno-otel",
      "environment": "production"
    }
  }'
```

API key is stored in AWS Secrets Manager at `secret/phenotype/elastic-api-key` (per
ADR-077 Vault migration roadmap; today the Secret Manager → Vault transit shim handles
key wrapping).

---

## 3. OpenTelemetry collector config (drop-in)

This is the production-ready `otel-collector` config block that ships with `pheno-otel`
v0.7+. It pairs with the Splunk reference impl — both exporters can be enabled
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
  attributes/siem-elastic-ecs-map:
    actions:
      - key: ecs.version
        action: insert
        value: "8.11.0"
      - key: event.module
        action: insert
        value: "phenotype"
      - key: event.dataset
        action: insert
        value: 'phenotype.siem.${siem.class}'
      - key: service.environment
        action: insert
        value: ${env:PHENO_ENV}
      # (full ECS mapping per §1.2 above)
  transform/siem-elastic-data-stream:
    trace_statements:
      - context: log
        statements:
          - set(resource.attributes["event.dataset"], resource.attributes["siem.class"])

exporters:
  elasticsearch:
    endpoint: https://phenotype.es.us-east-1.aws.found.io:443
    api_key: ${env:ELASTIC_API_KEY}
    mapping:
      mode: ecs
      fields:
        - name: service.environment
          value: production
    sending_queue:
      enabled: true
      num_consumers: 4
      queue_size: 8192
    retry_on_failure:
      enabled: true
      initial_interval: 5s
      max_interval: 60s
      max_elapsed_time: 300s
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
      processors: [memory_limiter, attributes/siem-elastic-ecs-map, batch]
      exporters: [elasticsearch, otlp/secondary]
    logs:
      receivers: [otlp, filelog]
      processors:
        - memory_limiter
        - attributes/siem-pii-redact
        - attributes/siem-elastic-ecs-map
        - transform/siem-elastic-data-stream
        - batch
      exporters: [elasticsearch, otlp/secondary]
```

The `otlp/secondary` exporter (omitted here for brevity) is the existing long-term
trace store at `otel-collector.phenotype.internal:4317` — Elastic is added in parallel,
not in place.

---

## 4. Alert rules (production-ready KQL + EQL)

The 6 mandatory rules below correspond to the 6 high-risk event classes in ADR-093 §4.
Each rule is **synthetic-trigger-tested** during Phase 1 dry-run (per ADR-093 §5).
Rules are authored in Kibana → Security → Rules → Create rule.

### 4.1 SIEM-AUTH-001 — brute-force login detection (KQL)

```yaml
name: "Phenotype: brute-force login detection"
type: query
language: kuery
query: |
  event.dataset: "phenotype.siem.auth" and
  phenotype.siem.class: "auth.login" and
  event.outcome: "failure"
group_by:
  - user.id
threshold:
  field: null
  value: 5
time_window: 5m
severity: high
risk_score: 73
tags: ["phenotype", "l46", "auth"]
```

**Trigger:** > 5 failed `auth.login` for a single `user.id` in any 5-min window.

### 4.2 SIEM-AUTH-002 — denied token issue (EQL sequence)

```yaml
name: "Phenotype: denied token issue"
type: eql
language: eql
query: |
  any where
    event.dataset == "phenotype.siem.auth" and
    phenotype.siem.class == "auth.token_issue" and
    event.outcome == "failure"
threshold: 1
time_window: 1m
severity: critical
risk_score: 95
tags: ["phenotype", "l46", "auth", "denied"]
```

**Trigger:** any `auth.token_issue` with `event.outcome="failure"` — typically a
KMS-denied key lookup. Single-event trigger (no threshold).

### 4.3 SIEM-RBAC-001 — out-of-window admin grant (KQL + time-window)

```yaml
name: "Phenotype: out-of-window admin grant"
type: query
language: kuery
query: |
  event.dataset: "phenotype.siem.rbac" and
  phenotype.siem.class: "rbac.permission_change" and
  phenotype.siem.permission: "admin"
schedule:
  interval: 5m
  maintenance_windows:
    - start: "20:00"
      end: "08:00"
      exclude_from_alerting: true
severity: critical
risk_score: 90
tags: ["phenotype", "l46", "rbac"]
```

**Trigger:** any `rbac.permission_change` to `phenotype.siem.permission="admin"`
outside the 20:00–08:00 UTC maintenance window. (Maintenance window excludes the
window from alerting — alerts fire when the event happens outside the window.)

### 4.4 SIEM-KMS-001 — unscheduled key ceremony (EQL)

```yaml
name: "Phenotype: unscheduled key ceremony"
type: eql
language: eql
query: |
  any where
    event.dataset == "phenotype.siem.kms" and
    phenotype.siem.class == "kms.key_access" and
    phenotype.siem.operation == "ceremony"
threshold: 1
time_window: 1m
severity: critical
risk_score: 99
tags: ["phenotype", "l46", "kms", "ceremony"]
```

**Trigger:** any `kms.key_access` with `operation="ceremony"`. Single-event trigger.
A pre-filter enrichment list (`phenotype.siem.key_id IN scheduled_ceremony_keys`)
would refine this to "unscheduled only"; we ship the conservative version first and
add the refinement in the v25 T0.5 closure.

### 4.5 SIEM-FED-001 — federation handshake burst (KQL + threshold)

```yaml
name: "Phenotype: federation handshake burst"
type: query
language: kuery
query: |
  event.dataset: "phenotype.siem.federation" and
  phenotype.siem.class: "federation.handshake" and
  event.outcome: "failure"
group_by:
  - phenotype.siem.peer_org
threshold:
  field: null
  value: 10
time_window: 1m
severity: high
risk_score: 75
tags: ["phenotype", "l46", "federation"]
```

**Trigger:** > 10 failed federation handshakes from a single `peer_org` in any 1-min
window.

### 4.6 SIEM-REL-001 — release sign without SLSA provenance (EQL)

```yaml
name: "Phenotype: release sign without SLSA provenance"
type: eql
language: eql
query: |
  any where
    event.dataset == "phenotype.siem.release" and
    phenotype.siem.class == "release.signing" and
    not phenotype.siem.attestation : "*slsa_provenance.v1*"
threshold: 1
time_window: 1m
severity: high
risk_score: 80
tags: ["phenotype", "l46", "release", "slsa"]
```

**Trigger:** any `release.signing` where `phenotype.siem.attestation` does not
contain `slsa_provenance.v1` (per ADR-077 + ADR-049 SLSA L3 mandate).

---

## 5. Operational runbook

### 5.1 Health checks

```bash
# 1. Verify API key is valid (no 401).
curl -k -H "Authorization: ApiKey $ELASTIC_API_KEY" \
  https://phenotype.es.us-east-1.aws.found.io:443/_cluster/health

# 2. Verify recent event arrival (last 5 min).
curl -k -H "Authorization: ApiKey $ELASTIC_API_KEY" \
  -X POST https://phenotype.es.us-east-1.aws.found.io:443/phenotype-siem-*/_search \
  -H "Content-Type: application/json" \
  -d '{"query":{"range":{"@timestamp":{"gte":"now-5m"}}},"aggs":{"by_class":{"terms":{"field":"phenotype.siem.class"}}}}'

# 3. Verify alert-rule backlog (last 24 h).
curl -k -H "Authorization: ApiKey $ELASTIC_API_KEY" \
  -X GET "https://phenotype.es.us-east-1.aws.found.io:443/.siem-signals-*/_search?q=signal.original_event.module:phenotype"
```

### 5.2 Common failure modes

| Symptom | Likely cause | Fix |
|---|---|---|
| `siem.drop{reason="auth_error"}` metric > 0 | API key rotated / revoked | rotate `ELASTIC_API_KEY` in Vault per ADR-077 |
| `siem.drop{reason="data_stream_rejected"}` metric > 0 | ECS mapping conflict | check index template vs OTel processor for field-type drift |
| `elasticsearch` exporter `error sending queue: full` | local back-pressure | raise `sending_queue.queue_size` to 16384 (monitor memory) |
| ILM policy not rolling over | index template missing `data_stream` block | re-apply index template per §2.1 |

### 5.3 PagerDuty routing

Same routing table as the Splunk reference (per ADR-093 §6.3 — vendor-neutral):

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
| Elastic Cloud ingest (200 events/sec sustained) | ~17 GB/day | measured Phase 1 dry-run |
| Elastic Cloud ingest at list price | ~$800/month | 2026-Q2 Elastic Cloud ingest pricing |
| Retention beyond 90 days (warm + cold tier) | +$150/month | per `findings/2026-06-22-L5-156-finops-q2.md` |
| Net fleet cost increase | ~$950/month | acceptable per FinOps Q2 sign-off |

The 200 events/sec sustained baseline is the Phase 1 dry-run result. Production expected
to land at 350-450 events/sec (per `findings/2026-06-22-L5-157-load-model-q3.md`); the
ingest-tier upgrade is in the 2026-Q3 procurement queue.

---

## 7. References

- ADR-093 (L46 SIEM integration spec)
- ADR-077 (Vault migration roadmap — API key storage)
- ADR-078 (encryption at rest — API key at rest)
- ADR-042 (security audit cadence — monthly)
- Elastic Common Schema (ECS) v8.11 reference
- Elastic Cloud on AWS admin reference
- Kibana Detection Rules authoring reference
- `findings/2026-06-22-L5-154-soc2-q2-readiness.md` (the 2 SOC2 findings this ADR closes)
- `findings/2026-06-22-L5-156-finops-q2.md` (the cost justification)
- `findings/2026-06-22-L5-157-load-model-q3.md` (the load model)
