# Runbook — Latency p99 SLO breach (5-step on-call procedure)

**Date:** 2026-06-22
**Pillar:** L14 — Latency p99 dashboard (cycle-14 lift 2.5 → 3.0)
**Track:** v24 T2 (per `plans/2026-06-22-v24-71-pillar-cycle-14-p1.md`)
**Companion:** `dashboards/latency-p99.json` (uid: `phenotype-l14-latency-p99`), `findings/latency-budgets.md`
**Policy:** ADR-091 (SLO/burn-rate alerting), ADR-092 (cycle-14 P1 reduction scope)
**Severity:** P1 (fleet-wide user-facing latency SLO)
**On-call rotation:** `#inc-p99` channel + PagerDuty service `phenotype-fleet-latency`

---

## When to use this runbook

Use this runbook when you receive a **Latency p99 SLO breach** page from PagerDuty, OR when
you observe p99 > 2× baseline for any service on the Grafana dashboard. The page trigger
conditions are defined in **Step 1** below. The SLO target is **99.9 % availability over
30 days = 43.2 min total error budget** (per ADR-091 § Compliance).

---

## Step 1 — Detect (page trigger conditions)

A page fires when **any** of the following conditions are met (per ADR-091 multi-window
burn-rate alerting policy):

| Trigger | Window | Threshold | Severity | Page? |
|---|---|---|---|---|
| Fleet p99 (any service) | 5 min sustained | > 2× baseline (per `findings/latency-budgets.md`) | P2 warn | Yes (auto-page) |
| Fleet p99 (single service) | 15 min sustained | > 2× baseline | P2 warn | Yes |
| Burn rate (1h fast-burn) | 1 hour | > 1.0 | P2 warn | Slack `#inc-p99` (no page) |
| Burn rate (1h fast-burn) | 1 hour | > 2.0 | P1 page | **PAGE** |
| Burn rate (6h slow-burn) | 6 hour | > 1.0 | P3 review | Slack (no page) |
| Burn rate (24h slow-burn) | 24 hour | > 1.0 | P3 review | Slack (no page) |
| Burn rate (3d slow-burn) | 3 day | > 1.0 | P3 review | Weekly digest |

**Multi-window rationale (ADR-091):** the 1h/6h pair catches fast regressions without
flapping; the 6h/24h/3d trio catches slow-burn regressions that compound before users notice.
A single 5-min page that resolves inside the 1h window is noise; a 5-min page that is
corroborated by 1h burn-rate > 2.0 is a real incident.

**Acknowledge within 5 min.** Note the dashboard URL, the service label, and the current
p99 value before proceeding.

---

## Step 2 — Check dashboard (URL + panel)

Open the dashboard and read panels in this order:

1. **Panel 1 — "p99 latency per service (timeseries)"** (5-min rate window)
   - URL: `https://grafana.phenotype.internal/d/phenotype-l14-latency-p99`
   - Look for: which service(s) crossed 2× baseline (red annotation marker)
   - Look for: when did the breach start (timestamp of first annotation)

2. **Panel 2 — "Fleet p99 (current)"** (single-stat)
   - Confirm: current fleet p99 vs target vs 2× threshold
   - Green ≤ 50 ms, yellow ≤ 100 ms, red > 100 ms

3. **Panel 3 — "p99 latency per service (1h trend)"** (long window)
   - Look for: slow-burn trend in any service (was it creeping up over the last hour?)

4. **Panel 4 — "Per-service p99 vs target (cycle 14)"** (table)
   - Confirm: which services are red (above 2×), yellow (above target), green (within budget)

**Snapshot the dashboard** (camera icon → PNG) and attach to the incident channel. The
snapshot is the canonical artefact for the post-mortem (Step 5).

---

## Step 3 — Identify service (label matcher, service routing)

Use the `service` label from Panel 1 / Panel 4 to route:

```bash
# Pull the offending service from the annotation / panel 1 legend
SERVICE="phenotype-router"  # example

# Find the owner of the service (CODEOWNERS)
gh api repos/KooshaPari/${SERVICE}/contents/CODEOWNERS --jq '.content' | base64 -d

# Find the on-call rotation for the service
gh api graphql -F query='
  query($name: String!) {
    repository(name: $name, owner: "KooshaPari") {
      metaData: issue(number: 1) { title }
    }
  }
' -F name="${SERVICE}" 2>/dev/null || true

# Get the latest deploy sha for the service
gh api repos/KooshaPari/${SERVICE}/deployments --jq '.[0].sha'
```

**Routing rules:**

- `phenotype-router`, `phenotype-bus`, `phenotype-hub`, `pheno-mcp-router` → fleet-architecture on-call (Slack `#fleet-arch`)
- `pheno-config`, `Configra` → config on-call (Slack `#config-oncall`)
- `pheno-port-adapter`, `pheno-flags`, `pheno-errors` → substrate on-call (Slack `#substrate`)
- `pheno-otel`, `pheno-tracing`, `pheno-events` → observability on-call (Slack `#obs`)
- `pheno-predict`, `pheno-drift-detector`, `pheno-vibecoding-guard` → platform on-call (Slack `#platform`)

If the service label is missing or the routing is unclear, page the **secondary on-call**
(PagerDuty escalation policy `phenotype-fleet-latency` level 2) within 10 minutes.

---

## Step 4 — Mitigate (rollback, scale, shed)

Choose the mitigation that matches the root-cause hypothesis. **Mitigate first, fix root
cause second** — the goal is to stop the burn, not to understand the burn.

### 4a. Rollback (preferred for deploy-induced regressions)

```bash
# Revert the most recent deploy on the offending service
SERVICE="phenotype-router"
DEPLOY_SHA=$(gh api repos/KooshaPari/${SERVICE}/deployments --jq '.[0].sha')
PREV_SHA=$(gh api repos/KooshaPari/${SERVICE}/commits?per_page=2 --jq '.[1].sha')

# Create revert PR (or use the rollback workflow if exists)
gh pr create \
  --repo KooshaPari/${SERVICE} \
  --base main \
  --head revert/${DEPLOY_SHA:0:7} \
  --title "revert: ${DEPLOY_SHA:0:7} (p99 SLO breach)" \
  --body "Reverts to ${PREV_SHA:0:7}. See incident channel #inc-p99-<date>."

# Wait 5 min for re-baseline, then check Panel 1 again
```

### 4b. Scale (preferred for traffic-induced saturation)

```bash
# Increase replica count (kubernetes example)
kubectl -n phenotype scale deployment/${SERVICE} --replicas=$((CURRENT_REPLICAS * 2))

# Or bump pool sizes for substrate services (pheno-port-adapter)
# Edit the per-crate perf-budgets.toml (L13 budget layer) and redeploy
```

### 4c. Shed (preferred for downstream-dependency failures)

```bash
# Enable load shedding via the circuit-breaker (pheno-port-adapter)
pheno-port-adapter-cli --service ${SERVICE} circuit-breaker enable \
  --failure-threshold 0.5 \
  --open-duration 30s

# Or rate-limit at the ingress (Envoy / nginx)
kubectl -n ingress patch envoyfilter rate-limit-${SERVICE} \
  --type merge -p '{"spec":{"rateLimit":{"requestsPerUnit":1000,"unit":"MINUTE"}}}'
```

### 4d. Freeze deploys (mandatory if sustained > 30 min)

Post in `#deploys`: `FREEZE: p99 SLO breach on ${SERVICE}. Revert before further deploys.`
Page secondary on-call. Do not lift the freeze until Panel 1 has been green for 15+ minutes.

### 4e. Escalate (mandatory if mitigation does not resolve in 30 min)

Open incident channel `#inc-p99-<date>` and assign an Incident Commander (IC). The IC owns
the timeline (Step 5) and the decision to escalate to P0 (full fleet impact).

---

## Step 5 — Postmortem (timeline, root cause, action items)

Within **24 hours** of incident resolution, file the postmortem using the
`incident-postmortem` skill workflow:

```bash
# Generate the postmortem scaffold
skill incident-postmortem
```

The postmortem MUST include:

### 5a. Timeline (T-format, UTC)

```
T+0   : page fires (annotation timestamp on Panel 1)
T+5   : on-call acknowledges
T+10  : dashboard snapshot taken; service identified
T+15  : mitigation chosen (rollback / scale / shed)
T+20  : mitigation applied
T+30  : Panel 1 back to green (sustained 15 min)
T+45  : deploy freeze lifted
T+24h : postmortem filed
```

### 5b. Root cause (5-why)

Use the 5-why technique. Cite the deploy SHA, the config change, the upstream-dependency
incident, or the capacity-planning miss that triggered the regression.

### 5c. Contributing factors

- Was the L13 per-operation budget enforced? (check `perf-gate.yml` run history)
- Was the 2x baseline annotation visible BEFORE the page fired?
- Was the multi-window burn-rate corroborating the fast-burn?
- Were there any prior slow-burn warnings that were ignored?

### 5d. Action items (3-5 max, with owners + due dates)

| Action | Owner | Due | Priority |
|---|---|---|---|
| Add regression test for the offending path | service-owner | +7 days | P1 |
| Tighten the budget threshold (if miss was > 3× target) | observability-oncall | +7 days | P2 |
| Add a 71-pillar L14 score check to the release gate | platform-oncall | +14 days | P2 |
| File a finding under `findings/<date>-p99-incident.md` | on-call | +24h | P0 |

### 5e. File the postmortem

Save the postmortem to `findings/<YYYY-MM-DD>-p99-incident.md` and link it from
`STATUS.md` § "Incident history". Update the cycle-14 probe (`findings/2026-06-22-v24-cycle-14-probe.md`)
with the incident summary if it shifted the L14 score (typically it does not — L14 is
about the dashboard + budgets + runbook existing, not about whether breaches happen).

---

## Compliance

- **ADR-091** — SLO/burn-rate alerting policy (this runbook implements Step 1 triggers)
- **ADR-092** — cycle-14 P1 reduction scope (this runbook is v24-T2 deliverable)
- **ADR-023** — device-fit governance (runbook authoring is macbook-safe)
- **ADR-040** — coverage gates per tier (runbook content itself is exempt; the dashboard
  panels and alerts inherit coverage from the substrate services that export the metrics)
- **ADR-042B** — substrate quality bar (the dashboard JSON and alert rules meet the
  substrate tier coverage floor when deployed to the fleet-observability substrate)

---

## Related

- `dashboards/latency-p99.json` — Grafana dashboard (panel IDs referenced in Steps 2-3)
- `findings/latency-budgets.md` — per-service p99 budgets + 2× thresholds (Step 1 inputs)
- `findings/2026-06-22-v24-cycle-14-probe.md` — cycle-14 scorecard (L14 2.5 → 3.0)
- `findings/2026-06-21-71-pillar-cycle-13-probe.md` — cycle-13 baseline (L14 = 2.5)
- `findings/2026-06-21-v16-L13-latency-budgets.md` — per-operation L13 budgets (sub-layer)
- ADR-091 — SLO/burn-rate alerting policy
- ADR-092 — cycle-14 P1 reduction scope
- `plans/2026-06-22-v24-71-pillar-cycle-14-p1.md` — v24 plan, Track T2 row
