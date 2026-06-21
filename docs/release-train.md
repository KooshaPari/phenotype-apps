# Phenotype Release Train Policy (L31)

**Date:** 2026-06-22
**Status:** ACCEPTED (v22 cycle 12, T3)
**Pillar:** L31 (Release cadence)
**Branch:** `feat/v22-l31-release-2026-06-22`
**Owner:** release-captain circle
**Calendar:** [`docs/release-calendar.json`](./release-calendar.json)
**Supersedes:** ad-hoc per-repo release cadence

## Scope

This policy governs release rhythm across the 47+ Phenotype sub-repos.
Every `pheno-*-lib`, `phenotype-*-sdk`, `phenotype-*-framework`, and
federated-service substrate follows one 6-week release train. Per-tier
cadence is a **multiplier on the train**, not a replacement for it.

## Train schedule (6 weeks)

A single train is 6 weeks. Within the train:

| Week | Phase          | Day   | Milestone                                |
|------|----------------|-------|------------------------------------------|
| W1   | Freeze         | Mon   | Feature freeze; `feat/*` PRs close EOD   |
| W1   | Freeze         | Wed   | RC signs off release-candidate branch    |
| W2   | Release        | Mon   | Tag RC; run full test matrix on runner   |
| W2   | Release        | Tue   | Publish + announce (canonical release)   |
| W3   | Stabilization  | Mon-Fri | Hot-fix triage; soak in fleet consumers |
| W4   | Stabilization  | Mon-Fri | Bug-bash; perf regression review        |
| W5   | Development    | Mon   | Next train's `feat/*` PRs open           |
| W6   | Development    | Mon   | Final review window; RC rotation         |

The W2-Tue release is the canonical release event. W3-W4 are soak +
triage. W5-W6 are quiet dev for the next train.

## SKIP rules

A SKIP promotes an out-of-cycle release inside an in-flight train.

- **Eligibility:** P0 severity-1 security vulnerability **only**. No
  feature SKIPs. No perf SKIPs. No "marketing window" SKIPs.
- **Cap:** **max 1 SKIP per 6-week cycle**. A second SKIP within the
  same cycle requires Release-Captain + Architect-Circle approval and
  must be recorded in the train's worklog.
- **Authority:** Release Captain (RC) signs; Security Lead seconds.
- **Process:** hotfix branch from released tag → CI green → RC sign-off
  → publish as `x.y.z+1` patch within 24h of SKIP approval.

## CUT rules

A CUT drops work that was frozen in W1 from this train's release.

- **P1 CUT:** may cut to next cycle (1 train deferral). No ack needed.
- **P2 CUT:** may cut up to 2 cycles (12 weeks). Owner ack required.
- **P0 CUT:** forbidden. A P0 in W1 must either ship, SKIP, or be
  re-classified by the Architect Circle.
- **v+1 rule:** a CUT may **never** silently roll into a major-version
  bump without an explicit owner ack recorded in the worklog.

## Per-tier cadence

| Tier                  | Cadence   | Train role                                    |
|-----------------------|-----------|-----------------------------------------------|
| `pheno-*-lib`         | Weekly    | Always riding the train; minor + patch only   |
| `phenotype-*-sdk`     | Bi-weekly | Every other W2 release; semver-strict         |
| `phenotype-*-framework`| Monthly  | Aligned to one release per train              |
| Federated service     | On-demand | Adheres to SKIP rules for sec; no fixed slot  |

Cadence is enforced by `scripts/release_coord.py` (L29, v21 T3) — see
[`findings/2026-06-22-v21-T3-L29-release-coordination.md`](../findings/2026-06-22-v21-T3-L29-release-coordination.md).

## Conflict resolution

When two release windows collide (e.g., a substrate consumer blocks a
provider release):

1. **Reverse-topological order wins.** See
   [`findings/2026-06-22-v21-T3-L29-release-coordination.md`](../findings/2026-06-22-v21-T3-L29-release-coordination.md:49-62).
   A breaking-change in `pheno-config` releases **before** its
   consumers (`phenotype-router`, `phenotype-hub`, `phenotype-*-sdk`).
2. **Tier priority:** T3 (fleet-critical) > T2 (stable) > T1 (beta) >
   T0 (alpha). Tier defined in
   [`ADR-048`](./adr/2026-06-18/ADR-048-substrate-graduation-path.md:28-33).
3. **Sec wins.** A SKIP-class sec vuln preempts all non-sec releases.

## Owner ack chain

A release ships only with the following ack chain (each row recorded
in the train's worklog row, per ADR-015 v2.1 schema):

| Step | Role              | Authority                     |
|------|-------------------|-------------------------------|
| 1    | Substrate owner   | Owns the diff; signs W1 freeze |
| 2    | Release Captain   | Owns the train; signs W2 pub  |
| 3    | Security Lead     | Seconds SKIPs only            |
| 4    | Architect Circle  | Signs P0 reclassifications    |

The `device:` field (ADR-015 v2.1) records who ran each phase. Per
[ADR-023 Rule 1](./adr/2026-06-15/ADR-023-agent-effort-governance.md:34-44),
a heavy release (full workspace `cargo test`, 1k-RPS soak) must
record `device: heavy-runner` — never `macbook`.

## Cross-references

- ADR-015 v2.1 worklog schema (device column) — [`docs/adr/2026-06-20/ADR-015-v2.1-worklog-schema.md`](./adr/2026-06-20/ADR-015-v2.1-worklog-schema.md)
- ADR-023 device-fit gate (Rule 1) — [`docs/adr/2026-06-15/ADR-023-agent-effort-governance.md`](./adr/2026-06-15/ADR-023-agent-effort-governance.md:34-44)
- ADR-048 substrate graduation (tier authority) — [`docs/adr/2026-06-18/ADR-048-substrate-graduation-path.md`](./adr/2026-06-18/ADR-048-substrate-graduation-path.md:28-33)
- L29 cross-repo release coordination — [`findings/2026-06-22-v21-T3-L29-release-coordination.md`](../findings/2026-06-22-v21-T3-L29-release-coordination.md)
- SDK versioning (L32) — [`docs/sdk-versioning.md`](./sdk-versioning.md)

## Edge cases (summary)

Full edge-case table lives in
[`findings/2026-06-22-v22-T3-L31-release.md`](../findings/2026-06-22-v22-T3-L31-release.md):

- broken release → revert + postmortem + re-cut to W2 of next train
- revert → `x.y.z-1` patch via SKIP channel (counts against cap)
- hotfix → SKIP path; max 1 per cycle; Security Lead seconds
- sec audit slip → defer SKIP; do not slip W2 release unless sev-1