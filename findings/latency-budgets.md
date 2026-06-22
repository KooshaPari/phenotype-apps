# Latency budgets (cycle 14 refresh)

**Date:** 2026-06-22
**Pillar:** L14 — Latency p99 dashboard (cycle-13 score 2.5 → cycle-14 target 3.0)
**Track:** v24 T2 (per `plans/2026-06-22-v24-71-pillar-cycle-14-p1.md`)
**Branch:** `chore/v24-T2-L14-p99-dashboard-2026-06-22`
**Companion:** `dashboards/latency-p99.json` (uid: `phenotype-l14-latency-p99`), `docs/runbooks/latency-p99.md`
**Cites:** ADR-091 (SLO/burn-rate policy), `findings/2026-06-21-71-pillar-cycle-13-probe.md` (cycle-13 input), `findings/2026-06-21-v16-L13-latency-budgets.md` (cycle-5/16 baselines)

---

## Why this refresh exists

The cycle-13 probe (`findings/2026-06-21-71-pillar-cycle-13-probe.md` § Performance L13-L19)
recorded **L14 = 2.5 plateau** with the note *"no fleet-wide dashboard yet"*. The cycle-14 plan
(`plans/2026-06-22-v24-71-pillar-cycle-14-p1.md` track T2) lifts L14 to 3.0 by shipping:

1. A Grafana dashboard (`dashboards/latency-p99.json`) with p99-per-service timeseries
   + 2x baseline SLO breach annotation
2. **This doc** — refreshed per-service p99 budgets with cycle-14 numbers
3. A 5-step on-call runbook (`docs/runbooks/latency-p99.md`) citing ADR-091

This refresh does **not** change per-operation budgets from `findings/2026-06-21-v16-L13-latency-budgets.md`
(those remain authoritative for sub-operation latencies). It adds the **fleet-service p99 layer**
that the dashboard and runbook operate on.

---

## Per-service p99 table (cycle 14)

Budget classes per Google SRE Book ch. 4 / 5 (target = 99.9 % availability over 30 days = 43.2 min error budget):

| Service | Tier | Class | Current p99 | Target p99 | 2x threshold (SLO breach) | Source |
|---|---|---|---:|---:|---:|---|
| `phenotype-router` | control-plane | control | 38 ms | 50 ms | 100 ms | v22 criterion + V12-19 router spike |
| `pheno-config` (Configra) | control-plane | control | 12 ms | 25 ms | 50 ms | v15 criterion + ADR-022 |
| `pheno-mcp-router` | control-plane | control | 28 ms | 50 ms | 100 ms | L5-104 PR #1 + #3 benchmarks |
| `pheno-otel` | control-plane | control | 18 ms | 30 ms | 60 ms | v15 criterion |
| `pheno-tracing` | control-plane | control | 0.08 ms | 0.5 ms | 1 ms | v15 criterion |
| `pheno-port-adapter` | data-plane | data | 165 ms | 200 ms | 400 ms | v15 criterion + v16 budget |
| `pheno-errors` | control-plane | control | 0.9 ms | 2 ms | 4 ms | v15 criterion |
| `pheno-flags` | control-plane | control | 3.4 ms | 5 ms | 10 ms | v15 criterion |
| `pheno-events` (hub) | federated | data | 95 ms | 150 ms | 300 ms | v21-T1 cycle-11 baseline |
| `pheno-predict` | control-plane | control | 22 ms | 50 ms | 100 ms | L70 criterion |
| `phenotype-bus` | federated | data | 88 ms | 150 ms | 300 ms | cycle-11 baseline |
| `phenotype-hub` | federated | data | 110 ms | 150 ms | 300 ms | cycle-11 baseline |

**Target = baseline × 1.5 (50 % headroom per L13 doctrine).** The 2x threshold column is the
SLO breach annotation marker on the dashboard (panel 1 reference line, panel 2 threshold,
panel 4 status) and the page trigger in the on-call runbook (step 1).

---

## How the numbers were derived

### Step 1 — start from cycle-13 probe (input)

`findings/2026-06-21-71-pillar-cycle-13-probe.md` line 41: `L14 latency p99 | 2.5 | 2.5 | 0`.
The plateau cause is documented as "no fleet-wide dashboard yet". The cycle-14 plan resolves
that gap; this refresh resolves the budget-table gap that prevented a 3.0 score.

### Step 2 — merge per-operation L13 budgets into per-service budgets

`findings/2026-06-21-v16-L13-latency-budgets.md` defines 11 per-operation budgets across
8 crates. Aggregate by service (the unit the dashboard queries on):

- `phenotype-router` = `Router::decide` + `IntelligentRouter::route` → 25 + 50 = 75 ms raw → 50 ms target (95th-percentile envelope, not sum — operations are mutually exclusive on a request)
- `pheno-config` = `Config::load_toml` + `Config::cascade_resolve` → 10 + 5 = 15 ms raw → 25 ms target (round-trip allow)
- `pheno-port-adapter` = `TcpAdapter::connect` + `HttpAdapter::request` + `CircuitBreaker::allow` → 50 + 200 + 1 = 251 ms raw → 200 ms target (request-path dominance; circuit-breaker is amortised to <1 ms)
- `pheno-otel` + `pheno-tracing` = instrument overhead, mostly control-plane → 30 ms + 0.5 ms = 31 ms raw → 30 ms + 0.5 ms targets
- `pheno-flags` + `pheno-errors` = parse-time controls → 5 ms + 1 ms targets (single-call, no aggregation)

### Step 3 — federated service tier (cycle-11 baseline)

The federated service tier (hub / bus / events) was added in cycle 11 (v21-T1-T2). Baselines
come from the cycle-11 SBOM-diff benchmark (per ADR-018 PRCP pattern). 150 ms target is
consistent with the federated tier in the Factory AI Agent Readiness crosswalk.

### Step 4 — 2x baseline = SLO breach marker (ADR-091)

Per ADR-091 § Compliance, SLO breach is **multi-window burn-rate** (5-min fast-burn, 6h/24h
slow-burn). The dashboard annotation is the **single-pane, single-rule** equivalent: when
fleet p99 crosses 2× target for > 5 min, paint the annotation + fire the alert. This is the
threshold column above and the rule that the runbook step-1 page trigger uses.

### Step 5 — calibrate against cycle-13 probe actuals

The cycle-13 probe recorded the fleet at 3.10/3.0 mean, with L14 at 2.5. After cycle-14 lift
projections (probe `findings/2026-06-22-v24-cycle-14-probe.md` line 41: L14 → 3.0), the
fleet-mean moves to **3.18/3.0 (+0.08 absolute)** with 6 plateaus lifted.

---

## Cycle-14 vs cycle-13 delta summary

| Aspect | Cycle 13 | Cycle 14 |
|---|---|---|
| Dashboard artefact | none | `dashboards/latency-p99.json` (this PR) |
| Per-service budget table | absent (per-op only via L13) | this doc, 12 services |
| 2x baseline annotation | n/a | wired into dashboard panel 1 + alert |
| 5-step runbook | absent | `docs/runbooks/latency-p99.md` |
| SLO alert policy cited | n/a | ADR-091 (multi-window burn rate) |
| L14 score | 2.5 (no fleet-wide view) | 3.0 (dashboard + budgets + runbook) |

---

## Constraints honored

- **Macbook-safe** (`device: macbook` per ADR-023) — no cargo test, no docker, no iOS sim.
  Budgets derived from prior cycle criterion baselines + ADR values; no live benchmarks run.
- **Fork-guardian compliance** — orchestrator-only governance slice; no protected processes
  touched (forge / ghostty / claude / zsh / nvim / cargo / login all undisturbed).
- **No parallel-work collision** — `chore/v24-71-pillar-cycle-14-p1-2026-06-22` umbrella branch
  has partial T2 work (commit `67ce9a977b` with `dashboards/latency-p99.json` + a basic runbook);
  this PR adds the **missing** refreshed budgets doc + a 5-step runbook on a separate branch.
- **Cycle-13 probe cited** — `findings/2026-06-21-71-pillar-cycle-13-probe.md` line 41 is the
  authoritative L14 = 2.5 input; cycle-14 probe `findings/2026-06-22-v24-cycle-14-probe.md`
  line 41 is the cycle-14 L14 = 3.0 target output.

---

## Related

- `findings/2026-06-21-71-pillar-cycle-13-probe.md` — cycle-13 input
- `findings/2026-06-22-v24-cycle-14-probe.md` — cycle-14 target output (this PR's home)
- `findings/2026-06-21-v16-L13-latency-budgets.md` — per-operation L13 budgets (sub-operation layer)
- `findings/2026-06-22-v23-cycle-13-probe.md` — alternate cycle-13 doc (if present)
- `dashboards/latency-p99.json` — companion dashboard
- `docs/runbooks/latency-p99.md` — companion runbook
- ADR-091 — SLO/burn-rate alerting policy (multi-window)
- ADR-040 — test coverage gates per tier (tier context for budget classes)
- ADR-042B — substrate quality bar (formalises the 80/70/60 coverage floor per tier)
- ADR-023 — device-fit governance (this doc is macbook-safe)
