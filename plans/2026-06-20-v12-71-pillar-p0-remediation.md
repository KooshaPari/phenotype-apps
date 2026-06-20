# DAG v12 — 71-Pillar P0 Remediation Wave (Org-Wide)

**Date authored:** 2026-06-20
**Status:** SPEC READY
**Supersedes:** v11 (102/102 WPs done, 100% saturation on melosviz scope)
**Extends:** v11 closure (`findings/forge-wave-2026-06-20/V11_CLOSURE_FINAL.md`) + 71-pillar cycle 1 (47 P0 gaps) + cycle 2 (8 substrate repos)

---

## 1. Executive Summary

v11 drained all 102 melosviz work packages to 100% saturation. v12 pivots to **org-wide 71-pillar P0 remediation**: address the 47 P0 gaps from cycle 1 across the 7 active focus repos (AgilePlus, pheno, dispatch-mcp, phenotype-ops, PhenoCompose, PlayCua, BytePort), starting with the top-10 most-common gaps.

**Scope**: 7 repos × 10 P0 actions = up to 70 PRs (some actions span multiple repos). v12 targets **closing ≥30 P0 gaps** in this wave (realistic 1-day wall with 20-wide parallel orchestrator).

---

## 2. Top-10 P0 Action List (from cycle 1, ADR-024 schema)

| # | Pillar | Description | Repos affected | v12 Track | Effort |
|---|---|---|---|---|---|
| 1 | **L47** | Secret scanning in CI (trufflehog/gitleaks) | AgilePlus, PhenoCompose, BytePort, dispatch-mcp | T12-A | ~2h |
| 2 | **L38** | Repo-level `AGENTS.md` | dispatch-mcp, PhenoCompose, BytePort, phenotype-ops | T12-B | ~1h |
| 3 | **L57** | Wire `pheno-tracing` to sub-apps | pheno, dispatch-mcp, PlayCua, AgilePlus | T12-C | ~3h |
| 4 | **L30** | `.devcontainer/` per pheno-flake template | dispatch-mcp, phenotype-ops, BytePort | T12-D | ~1h |
| 5 | **L4** | Hexagonal ports: `Port` trait + `Adapter` impl | dispatch-mcp, PhenoCompose, PlayCua, BytePort | T12-E | ~4h |
| 6 | **L46** | Branch protection rules consistent | pheno, dispatch-mcp, PlayCua, BytePort | T12-F | ~30min |
| 7 | **L56** | tracing-subscriber configured (structured logs) | dispatch-mcp, PhenoCompose, BytePort | T12-G | ~1h |
| 8 | **L29** | CI pipeline (min: cargo test + clippy + fmt) | PhenoCompose, BytePort, dispatch-mcp | T12-H | ~2h |
| 9 | **L13** | Latency budgets / SLO targets | dispatch-mcp, PhenoCompose, BytePort | T12-I | ~1h |
| 10 | **L71** | ADR cross-refs in repo-local ADRs | dispatch-mcp, phenotype-ops, BytePort | T12-J | ~1h |

---

## 3. DAG Structure (10 tracks × 6 stages = 60 tasks, expandable to 100)

### Stage 1: Stabilize (5 tracks, 20 tasks)
- **T12-A** L47 secret scanning (4 tasks: 1 per repo)
- **T12-B** L38 AGENTS.md (4 tasks: 1 per repo)
- **T12-C** L57 pheno-tracing wire (4 tasks: 1 per repo)
- **T12-D** L30 devcontainer (3 tasks: 1 per repo)
- **T12-F** L46 branch protection (4 tasks: 1 per repo, 1 admin pool task)

### Stage 2: Hexagonal core (2 tracks, 10 tasks)
- **T12-E** L4 ports (4 tasks: 1 per repo + 1 shared `phenotype-ports` crate task)
- **T12-G** L56 logging (3 tasks + 2 sub-tasks: shared `pheno-logging` setup, repo init)

### Stage 3: CI + latency (2 tracks, 8 tasks)
- **T12-H** L29 CI (3 tasks + 1 shared `pheno-ci-templates` task)
- **T12-I** L13 SLO (4 tasks: 1 per repo)

### Stage 4: Governance (1 track, 4 tasks)
- **T12-J** L71 ADR cross-refs (4 tasks: 1 per repo + 1 fleet-wide `ADR-001..074` index task)

### Side DAG (1 track, ~18 tasks for 100% width fill)
- **T12-Side** v11 fleet hygiene: clean 19 worktrees (`/tmp/melosviz-wt*`); recover lost branches (`wip/recovered-v10-025-stash-2026-06-20`); dedup findings/forge-wave-2026-06-20/; re-apply v11 final closure

### Stage 5: Verify (1 track, 4 tasks)
- Re-run 71-pillar cycle 3 on the 7 repos
- Generate cycle 3 scorecard
- Compare cycle 1 → cycle 2 → cycle 3 deltas
- v12 wrap-up + ADR-076 "v12 closure"

### Stage 6: Forward (1 track, 4 tasks)
- v13 scope: 71-pillar P1 remediation (31 P1 gaps from cycle 1)
- v13 scope: cheap-llm-mcp absorption (L5-104 still incomplete per T15)
- v13 scope: HexaKit retarget (L5-110/111/112 EPILOGUE 3)
- v13 scope: 5-repository post-Cycle 3 audit (add nanovms, helios-router, helioscope, authvault, planify)

---

## 4. 20-Wide Parallel Orchestrator Pattern (v11-proven)

Per the worktree-isolation pattern proven in v11 (5 waves, 102/102 WPs drained):

1. **Pre-launch**: `git worktree add` for each of 20 worker tasks in `/private/tmp/melosviz-wt-v12/{wp-N-slug}/`
2. **Dispatch**: each worker scaffolds + commits in its own worktree
3. **Merge**: `git merge --no-ff -m "merge: wp/N-..." wp/N-...` from main branch
4. **Verify**: `git diff --cached --quiet` check before push
5. **Push**: `git push origin HEAD --force-with-lease` (idempotent)

**Throughput target**: 20 WPs in ~30 min wall (1.5min/WP amortized). v12 = 60 WPs / 3 batches = ~1.5h wall.

---

## 5. Pre-Conditions (v12 launch gate)

- [x] v11 closure complete (102/102 WPs done, PR #97 opened)
- [x] Cycle 1 71-pillar rollup published (47 P0 gaps identified)
- [x] Cycle 2 71-pillar audit complete (8 substrate repos, 5 DELETED/404 historical)
- [ ] Clean up 19 v11 worktrees in `/tmp/melosviz-wt*` (low priority, deferred to T12-Side)
- [ ] Verify no 71-pillar schema drift between cycle 1 and cycle 2
- [ ] Open v12 branch: `chore/v12-71-pillar-p0-remediation-2026-06-20`

---

## 6. Success Metrics (v12 closure gates)

| Metric | Target | Notes |
|---|---|---|
| P0 gaps closed | ≥ 30 of 47 | per cycle 3 re-audit |
| Repos with AGENTS.md | 7 / 7 | from 3/7 |
| Repos with secret scanning CI | 7 / 7 | from 3/7 |
| Repos with pheno-tracing | 7 / 7 | from 3/7 |
| Repos with hexagonal ports | 7 / 7 | from 3/7 (longer: T12-E) |
| Org mean (cycle 3) | ≥ 1.80 | from 1.43 |
| Org median (cycle 3) | ≥ 2.00 | from 1.55 |
| Repos PASS (mean ≥ 2.00) | ≥ 3 of 7 | from 0/7 |

**v12 closure**: when ≥ 3 repos hit PASS and org mean ≥ 1.80. Re-audit cadence resumes weekly per ADR-041.

---

## 7. Risk Register

| Risk | Mitigation |
|---|---|
| Fleet pivots branch context mid-wave (as in v11) | All work in dedicated `chore/v12-...` branch; force-push idempotent |
| Push target (`phenotype-apps`) gets archived | Use `argis-extensions` as de facto push target (per v11) |
| `/tmp/melosviz-wt*` symlink issues on macOS | Use `/private/tmp/melosviz-wt-v12` (real path) per v11 lesson |
| 71-pillar schema drift between cycles | Pin to ADR-024 schema verbatim; cycle 3 uses identical template |
| Cycle 3 results show no improvement (audit methodology broken) | Add control repos (pheno, AgilePlus) with known-good scores to validate |
| Cheap-llm-mcp L5-104 still incomplete | Out of v12 scope; v13 candidate |

---

## 8. T0 Pre-Launch (T0.0)

- Verify T0.0 retro: subagent dispatch healthy? Fleet processes alive? (Yes, 23 orch processes)
- Verify `gh auth status` (KooshaPari with `delete_repo` scope)
- Verify push target (`argis-extensions` per v11)
- Verify pre-commit hook working
- Verify no merge-blocker in 71-pillar cycle 1 PR chain

---

## 9. T0.5 Wrap-up (post-launch)

- Author v12 closure report (`findings/v12-71-pillar-p0-remediation-2026-06-20.md`)
- Author ADR-076 v12 closure
- Update `AGENTS.md` Wave Plan section
- Update `.agileplus/agileplus.db` with v12 WPs (if reused as substrate)
- Push to origin

---

## 10. v12.5 Supplement — 12-Pillar Remediation (Authored 2026-06-20, post-v11 closure)

**Status:** DRAFT — supplements the v12 plan above; v12 work largely landed per `findings/2026-06-20-v12-closure.md` (6/10 tracks). v12.5 is the next-batch remediation plan that addresses the 12 pillars NOT closed by v12 (mean < 2.00 after v12 closes). v13 plan (`plans/2026-06-20-v13-71-pillar-cycle-2-p0.md`, commit `7b724bbd8f`) overlaps; v12.5 is the bridge from v12 closure to v13 start.

**Strategic frame:** v12.5 shifts from **gap-closing** (v12's approach: top-10 cycle 1 P0 gaps across the 7 active focus repos) to **remediation** — addressing the 12 cross-cutting pillars that scored ≤1 across the post-v12 fleet. These 12 are the ones the v11 closure post-mortem (`findings/2026-06-20-L5-104-v11-closure-postmortem.md` § 3.1) flagged as deferred. v12.5 finishes the remediation loop and primes the fleet for v13's P1 cadence.

### 10.1 Pillar-to-schema mapping

The v12.5 brief authored in the v11 closure post-mortem used pillar numbers that do not align 1:1 with the canonical 71-pillar schema (ADR-024, see `findings/71-pillar-refresh-template.md`). The mapping below translates each v12.5 brief pillar to its actual schema location. **The work described in the brief maps to the following 12 actual schema pillars (post-v12 baseline):**

| Brief # | Brief description (v11 post-mortem § 3.1) | Actual schema pillar | Post-v12 score (cycle 2 + delta) |
|---|---|---|---|
| 1 | L8 (observability) — OTLP not wired to substrate fleet | **L57** Metrics (RED/USE) + **L58** Distributed tracing | 2.0 (cycle 1) → ~2.4 (v12 partial) |
| 2 | L9 (error handling) — substrate error paths not uniform | **L24** Error handling | 1.5 → ~2.0 |
| 3 | L16 (concurrency model) — not documented | **L18** Concurrency model | 1.2 → ~1.5 |
| 4 | L17 (resource limits) — not enforced | **L14** Resource efficiency | 1.4 → ~1.7 |
| 5 | L18 (rate limits) — not configured | **L57** (rate metric) + **L13** Latency budgets | 1.3 → ~1.6 |
| 6 | L19 (circuit breakers) — missing | **L26** Reliability / fault tolerance | 1.1 → ~1.4 |
| 7 | L22 (data integrity) — not tested | **L21** Code review rigor + **L22** Static analysis | 2.0 → ~2.4 |
| 8 | L23 (migrations) — no migration framework | **L60** Deployment automation (DB migration subset) | 1.0 → ~1.3 |
| 9 | L24 (backups) — not configured | **L62** Backup / restore | 1.2 → ~1.5 |
| 10 | L25 (disaster recovery) — not planned | **L61** Incident response | 1.0 → ~1.3 |
| 11 | L26 (capacity planning) — not done | **L63** Capacity planning | 1.3 → ~1.6 |
| 12 | L29 (dependency hygiene) — 57 dependabot alerts | **L25** Supply-chain integrity + **L29** CI/CD pipeline | 1.5 (CI up) / 2.0 (deps down) → ~2.5 |

### 10.2 12-track execution plan (T1-T12)

Each track has the same shape: 1 P0 pillar, 1-3 substrate adopters, exit criterion "score ≥2 on the pillar in cycle 3 re-audit". Estimated 4-6 weeks critical path. **T12 (dependency hygiene) blocks 8 other tracks via security gate.**

| Track | Pillar | Substrate adopters | What | Effort |
|---|---|---|---|---|
| **T1** | L57/L58 OTLP wiring | pheno-config, pheno-tracing, pheno-mcp-router, pheno-observability | Adopt `pheno-tracing`; OTLP smoke test in test matrix; `pheno_tracing::init()` at lib entrypoints | ~3h |
| **T2** | L24 error handling | pheno-config, pheno-tracing, pheno-mcp-router | Uniform error paths via `pheno-errors`; `Result<T, PhenoError>` convention; structured error variant export | ~2h |
| **T3** | L18 concurrency model | pheno-tracing, pheno-mcp-router | Document per-substrate concurrency model (async runtime, sync vs async, channels, locks); add `CONCURRENCY.md` per substrate | ~2h |
| **T4** | L14 resource limits | pheno-config, pheno-mcp-router | Enforce via config: max memory, max CPU, max file descriptors; runtime limits in `pheno-config::Limits` | ~2h |
| **T5** | L13/L57 rate limits | pheno-mcp-router middleware | Rate-limit middleware via `pheno-mcp-router`; per-key QPS/RPS budgets; 429 with retry-after | ~3h |
| **T6** | L26 circuit breakers | pheno-mcp-router, pheno-observability | Adopt `ResilienceKit` (or `pheno-circuit-breaker` substrate); per-provider CB; half-open after 30s; failure threshold config | ~3h |
| **T7** | L21/L22 data integrity | pheno-mcp-router, pheno-secret-scan | Test framework adoption: `proptest` for Rust invariants; `hypothesis` for Python; coverage gate at 80% | ~2h |
| **T8** | L60 migrations | pheno-config (DB schema migrations) | Adopt `pheno-migrate` framework (or scaffold one if absent); reversible migrations; CI smoke test | ~4h |
| **T9** | L62 backups | fleet-wide | Fleet backup policy: daily snapshots for federated services; 30-day retention; restore runbook | ~3h |
| **T10** | L61 disaster recovery | fleet-wide | DR runbook: RTO 4h, RPO 1h for federated services; tabletop exercise per ADR-042 | ~4h |
| **T11** | L63 capacity planning | pheno-capacity (canonical substrate) | Adopt `pheno-capacity` substrate (ADR-036, EXECUTED 2026-06-19); per-service capacity budgets; auto-scaling hooks | ~3h |
| **T12** | L25/L29 dependency hygiene | fleet-wide | **57 dependabot fixes across 9 repos**: cargo update + pip-compile + go mod tidy; SLSA L3 provenance for 3 critical services; blocks T1-T8 via security gate | ~6h |

### 10.3 Critical path

```
T12 (security gate, 6h) ─┐
                         │
                         ├──> T1 (OTLP, 3h) ──> T3 (concurrency docs, 2h) ──> cycle 3 re-audit
                         │
                         ├──> T2 (error handling, 2h)
                         │
                         ├──> T5 (rate limits, 3h) ──> T6 (circuit breakers, 3h)
                         │
                         └──> T11 (capacity, 3h) ──> T9 (backups, 3h) ──> T10 (DR, 4h)

T4 (resource limits, 2h) ──> parallel
T7 (data integrity, 2h) ──> parallel
T8 (migrations, 4h) ──> parallel
```

**Critical path:** T12 → T1 → T3 (security gate → OTLP wire-up → concurrency docs) = **~11h serial wall + 12h parallel** = ~1.5-2 weeks with 2 devs in parallel. **Conservative estimate with full T9/T10 (backups/DR) end-to-end testing: 4-6 weeks.**

### 10.4 Pre-conditions (v12.5 launch gate)

- [x] v11 closure post-mortem authored (`findings/2026-06-20-L5-104-v11-closure-postmortem.md`, this turn)
- [x] v12 closure summary exists (`findings/2026-06-20-v12-closure.md`, 6/10 tracks landed, 7 pillars closed to 3/3 mean)
- [x] v13 plan committed (`plans/2026-06-20-v13-71-pillar-cycle-2-p0.md`, commit `7b724bbd8f`)
- [ ] PR #105 (v12 P0 remediation on KooshaPari/argis-extensions) merged — conflicts to resolve (30 min)
- [ ] v12.5 branch opened: `chore/v12-5-12-pillar-remediation-2026-06-20`
- [ ] T12 dependabot alert list refreshed (currently 57 per brief; cycle 3 re-count TBD)
- [ ] `pheno-capacity` substrate source verified in `KooshaPari/pheno-capacity` (ADR-036 EXECUTED 2026-06-19; L5-117 absorb DEFERRED — see `findings/2026-06-19-L5-117-pr-status.md`)

### 10.5 Success metrics (v12.5 closure gates)

| Metric | Target | Source |
|---|---|---|
| P0 pillars closed to ≥2 (cycle 3 re-audit) | ≥ 8 of 12 | per cycle 3 scorecard (target 2026-07-06 Mon) |
| Org mean (cycle 3) | ≥ 2.10 | from 1.47 (cycle 2 baseline); v12 lifts to ~1.85; v12.5 lifts to ~2.10 |
| Repos PASS (mean ≥ 2.00) | ≥ 5 of 15 | from 0/15 (cycle 2 baseline) |
| OTLP smoke tests in CI | 7/7 fleet-critical substrates | T1 |
| Dependabot alerts closed | ≥ 40 of 57 | T12 |
| DR runbook RTO/RPO test | PASS for 1 federated service | T10 |
| Capacity budgets published | ≥ 3 services | T11 |

**v12.5 closure:** when ≥ 8 of 12 tracks pass cycle 3 re-audit and org mean ≥ 2.10. The remaining 4 tracks roll into v13 as the next P1 batch.

### 10.6 Risk register

| Risk | Mitigation |
|---|---|
| T12 (security gate) blocks 8 tracks → if it slips, v12.5 wall-time doubles | Start T12 in parallel with T1-T4 even though T12 is "blocking"; gate is on `main` merge, not on starting work |
| `pheno-capacity` substrate source not on this branch (L5-117 deferred) | Use the canonical `KooshaPari/pheno-capacity#1` branch as the substrate source; document local-checkout absence in v12.5 worklog |
| OTLP smoke test diverges from `pheno-tracing` env-config (T1 vs T5 already in v12) | Reuse `pheno-tracing/src/env.rs` (commit `pheno-tracing/eb41827`) verbatim; do not re-author |
| Backups/DR (T9/T10) require federated service cooperation (PhenoMCP, phenoObservability, phenoEvents) | Use a single federated service as the pilot (PhenoMCP); scale to others in v13 |
| DR tabletop exercise (T10) is a non-engineering deliverable | Defer to Mission 4 if no owner surfaces in v12.5 launch window |
| 12 tracks in 4-6 weeks with 2 devs requires parallel-agent dispatch | Use v11's 20-wide worktree-isolation pattern; each track gets its own worktree + merge batch |

### 10.7 v12.5 → v13 hand-off

v12.5 closes the remediation loop that v12 started. The 4 tracks v12.5 does NOT close (estimate 2-4 of 12) roll into v13 as P1 work. v13 plan (`plans/2026-06-20-v13-71-pillar-cycle-2-p0.md`) already has 8 tracks targeting cycle-2 P0 gaps; v12.5 deferred items merge into v13's track list without rewriting.

**Combined v12 + v12.5 + v13 trajectory:**

| Wave | Target mean | Target repos PASS | P0 closed (cumulative) |
|---|---:|---:|---:|
| Cycle 2 (baseline) | 1.47 | 0/15 | 0 |
| v12 (in-flight) | ~1.85 | 2/15 | 7 |
| v12.5 (this section) | ~2.10 | 5/15 | 15 |
| v13 (planned) | ~2.55 | 9/15 | 26 |

### 10.8 References

- `findings/2026-06-20-L5-104-v11-closure-postmortem.md` (this turn) — the brief that motivated v12.5
- `findings/2026-06-20-v12-closure.md` — v12 closure summary (6/10 tracks landed)
- `plans/2026-06-20-v13-71-pillar-cycle-2-p0.md` (commit `7b724bbd8f`) — v13 plan, the next wave
- `findings/71-pillar-2026-06-20-weekly-2.md` — cycle 2 substrate rollup
- `findings/71-pillar-refresh-template.md` — canonical 71-pillar schema (ADR-024)
- ADR-024 — 71-pillar audit framework
- ADR-026 — worklog schema (cycle-3 cadence)
- ADR-027 — LFS 3-tier policy (relevant to T12 supply-chain)
- ADR-036 — pheno-capacity substrate canonical (EXECUTED 2026-06-19)
- ADR-041 — 71-pillar refresh cadence (weekly Monday 09:00 PDT)
- ADR-042 — security audit cadence (monthly `cargo audit` + `pip-audit` + `govulncheck`)
- ADR-042B — substrate quality bar
- ADR-046 — federation mTLS + OIDC (relevant to T10 DR)
- ADR-048 — substrate graduation path (relevant to T11 capacity)

---

**v12 plan + v12.5 supplement complete. v12 closure narrative is the source of truth for what landed; v12.5 is the next-batch plan for what didn't. v13 plan (`7b724bbd8f`) extends the trajectory.**
