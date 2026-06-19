# DAG v9 — Phenotype Fleet W2-W3-W4 Wrap-up

**Date authored:** 2026-06-19
**Status:** SPEC READY (launching in this session)
**Authoritative source:** AGENTS.md § "Wave Plan" + this file
**Supersedes:** v8 (`plans/2026-06-18-v8-dag-stable.md`) — v8 closure COMPLETE 2026-06-18 (per `audit-71-pillar-2026-06-17-wrapup.md`)
**Ships-to:** This file (replaces v8)
**Extends:** v8 carryover scope (T15-T23 fleet sweep tracks, T0.5 wrap-up) + new v9 L5-110..117 governance waves + pheno-worklog-schema v2.1 fix branch

---

## Table of Contents

1. Executive Summary
2. Track Inventory
3. Per-Track Task List
4. Topological Sort
5. PR Matrix
6. Risk Register
7. Success Criteria
8. Cross-References
9. Appendices A-D

---

## 1. Executive Summary

**v9 is the W2-W3-W4 wrap-up plan for 2026-06-19.** v8 closure landed on main 2026-06-18 (HEAD `a4ccb6c41d`). 82/160 DAG tasks done; 84 ready; 0 in flight. The 84 ready tasks are side-DAG fillers (`side-01`..`side-84`) — SOTA research, guardrail enforcement, optimizations, and audits for the canonical `pheno` Rust monorepo. v9 also inherits:

- **5 new ADRs** (ADR-036, 041, 042, 043, 044) authored during the v8→v9 gap
- **L5-110..117 wave** (substrate audit, drift-detector absorption, graduation discipline, predictive discipline, drift detection, cron deployment, pheno-capacity absorb)
- **`pheno-worklog-schema` v2.1 fix branch** (`fix/parse-worklog-v2-1-header-format-strictness`, 9 commits ahead of main, ready to merge)

**v9 unique goals vs v8:**

- **Drain the 84 ready side-DAG tasks** for the `pheno` monorepo (SOTA research on async-trait/tokio-console/rust-analyzer; deny.toml audit; cargo-outdated CI; SPDX headers; sccache/mold linker; duplicate detection)
- **Adopt L5-110..117 governance** (substrate audit, drift detection, predictive discipline, graduation discipline, cron deployment) — these are real v9 tracks that v8 didn't have
- **Land the v2.1 worklog schema fix branch** (parse_worklog v2.1 header format strictness, 9 commits, deprecation deadline 2026-06-22)
- **W4 wrap-up**: postmortem + v9 closure doc + final AGENTS.md/STATUS.md refresh

**Owners:** orchestrator + 6 parallel subagents (`orch-w2-a` through `orch-w2-f`).

**v9 supersedes v8** (18 tracks, ~210 tasks, ~200 PRs).

---

## 2. Track Inventory

| # | Track | P-level | Goal | Task count | PR count | Effort (parallel) | Owner |
|---|---|---|---|---|---|---|---|
| **T0** | Pre-flight Gate | P0 | Verify auth + forge + OmniRoute + clean tree before launching | 5 | 0 | ~10 min | orchestrator |
| **T25** | v2.1 Worklog Schema Fix (L5-103.x) | P0 | Merge `fix/parse-worklog-v2-1-header-format-strictness` branch to main; verify 4-format support; finalize deprecation 2026-06-22 | 6 | 1 | ~30 min | orchestrator |
| **T26** | L5-110 Substrate Audit | P0 | 7 substrate-drift findings + 1 substrate-audit finding (already staged); verify all land on main; close ADR-043 dependency | 9 | 7 | ~45 min | orchestrator + 2 subagents |
| **T27** | L5-111 pheno-drift-detector Absorption | P0 | 5 P1-P5 patches for `pheno-drift-detector` (claim catalog fixes; PAX pillar wiring); required by ADR-043 cron first run 2026-06-23 09:00 PDT | 6 | 5 | ~75 min | orchestrator + 2 subagents |
| **T28** | L5-112 pheno-predict Absorption | P1 | 4 predictive-discipline patches for `pheno-predict` (ADR-041); retroactive hit detection; cycle-time guard | 5 | 4 | ~60 min | orchestrator + 1 subagent |
| **T29** | L5-113 pheno-framework-lint Absorption | P1 | 5 graduation-discipline patches for `pheno-framework-lint` (ADR-042); 4-tier gate table; substrate graduation scoring | 6 | 5 | ~75 min | orchestrator + 1 subagent |
| **T30** | L5-114 Services Retirement (ADR-040 Step 5) | P0 | Complete the 4 services migration; flip registry fsm to terminal `archived`; close out L5-114 | 8 | 6 | ~60 min | orchestrator |
| **T31** | L5-115 pheno-capacity Extraction (HW Ledger) | P0 | Extract `pheno-capacity` math lib from `HwLedger` per ADR-035A; 60-test pure-math Rust crate (already created); map to substrate per ADR-023 Rule 3 | 7 | 5 | ~60 min | orchestrator + 1 subagent |
| **T32** | L5-117 pheno-capacity Absorb → phenotype-gateway | P0 | Absorb the 60-test `pheno-capacity` lib into `phenotype-gateway` collection per ADR-036; latest commit `59744d667a` is the ADR; collection-merge plan staged | 6 | 4 | ~45 min | orchestrator |
| **T15** | Pheno-flake Refresh (carryover) | P1 | Homogenize meta-bundle + tests + CI across 22 pheno-* repos | 22 | 22 | ~150 min | orchestrator + 4 subagents |
| **T17** | Lean Code Quality (carryover) | P2 | `cargo clippy --strict` + dead-code + unused-imports sweep across pheno-* crates | 25 | 25 | ~180 min | orchestrator + 4 subagents |
| **T18** | Test Coverage Pass (carryover) | P1 | Bring each substrate tier to gate (80% lib / 70% framework / 60% service) | 22 | 22 | ~150 min | orchestrator + 4 subagents |
| **T19** | CI Templates Refresh (carryover) | P1 | `pheno-ci-templates`: HOOKS_SKIP, SKIP, dependabot, OIDC, SBOM | 10 | 8 | ~75 min | orchestrator |
| **T20** | Documentation Refresh (carryover) | P1 | llms.txt + SPEC.md + ADR-015 quickstart across 20 substrate repos | 18 | 18 | ~120 min | orchestrator + 3 subagents |
| **T21** | Security Audit (carryover) | P0 | Secret-scan fleet, SBOM generation, dependency vulnerability scan, SLSA provenance | 12 | 8 | ~90 min | orchestrator + 2 subagents |
| **T22** | Observability Wiring (carryover) | P1 | Adopt `pheno-tracing` + OTLP export across substrate fleet | 15 | 15 | ~120 min | orchestrator + 3 subagents |
| **T23** | Registry Refresh (carryover) | P1 | `phenotype-registry`: rebase stale entries, adopt ADR-013 substrate model | 8 | 6 | ~60 min | orchestrator |
| **T33** | Side-DAG Filler (NEW v9) | P1 | Drain 84 ready side-DAG tasks for `pheno` monorepo (SOTA research, guardrails, optimizes, audits) | 84 | 60+ | ~180 min | 6 subagents rotating |
| **T0.5** | Wrap-up | P0 | Author `v9-wrapup.md`, push final branch to origin, write postmortem | 5 | 0 | ~15 min | orchestrator |
| | | | **Total** | **~280 tasks** | **~210 PRs** | **~25h parallel / ~55h sequential** | |

**Tracks with internal parallelism: 14 of 20** (T25-T33, T15-T23, T0.5 have 2-6 subagent streams; T0, T30, T32 are orchestrator-only)

---

## 3. Per-Track Task List

### 3.0 Track T0 — Pre-flight Gate (P0, ~10 min, orchestrator)

| Task | Description | P-level | Owner | Acceptance |
|---|---|---|---|---|
| **T0.1** | Verify `gh auth status` → KooshaPari active with `delete_repo` scope | P0 | orchestrator | `gh auth status 2>&1 | grep -q 'Logged in to github.com account KooshaPari'` returns 0; `gh api /users/KooshaPari/repos --jq '.[0].name'` succeeds |
| **T0.2** | Verify `task` tool (or `forge` CLI fallback) works in < 30s | P0 | orchestrator | `task` tool dispatches subagent that returns within 30s |
| **T0.3** | Verify OmniRoute liveness (or fall back to direct subagent dispatch) | P0 | orchestrator | `curl -sf -m 3 http://localhost:20128/v1/models` returns 4+ models; if DOWN, fall back to direct `task` tool dispatch (already proven in v8) |
| **T0.4** | Verify clean working tree | P0 | orchestrator | `git status --short | wc -l` ≤ 5 (submodule pointer drifts expected); main is 0/0 vs origin |
| **T0.5** | Verify 0 stashes + ≤ 2 worktrees | P0 | orchestrator | `git stash list | wc -l` = 0; `git worktree list | wc -l` ≤ 2 |

**Pre-flight status (verified 2026-06-19):** T0.1 ✅ PASS (KooshaPari, delete_repo scope); T0.2 ✅ PASS (task tool working); T0.3 ⚠️ OmniRoute DOWN (non-blocking — `task` tool fallback); T0.4 ✅ PASS (main 0/0); T0.5 ✅ PASS (0 stashes, 1 worktree).

### 3.1 Track T25 — v2.1 Worklog Schema Fix (L5-103.x, P0, ~30 min, orchestrator)

| Task | Description | P-level | Owner | Acceptance |
|---|---|---|---|---|
| **T25.1** | Verify `pheno-worklog-schema` branch `fix/parse-worklog-v2-1-header-format-strictness` is 9 commits ahead of main with all 4-format support | P0 | orchestrator | `git -C pheno-worklog-schema log --oneline main..HEAD | wc -l` = 9 |
| **T25.2** | Run `pytest tests/` on the fix branch; verify 30/30 tests pass (4-format: V2 spaced, V2 unspaced, V1 6-col, JSONL) | P0 | orchestrator | `pytest tests/ -v | tail -5` shows 30/30 pass |
| **T25.3** | Rebase fix branch onto latest main; resolve any conflicts | P0 | orchestrator | `git -C pheno-worklog-schema rebase main` exits 0 |
| **T25.4** | Open PR `KooshaPari/pheno-worklog-schema` `fix/parse-worklog-v2-1-header-format-strictness` → `main` | P0 | orchestrator | PR URL captured; CI green |
| **T25.5** | Verify deprecation warning for v2.0 format (target date 2026-06-22) | P0 | orchestrator | `pytest tests/ | grep -c 'deprecat'` ≥ 1 |
| **T25.6** | Update AGENTS.md § "Active ADRs" with ADR-025 status (v2.1 deprecation) | P0 | orchestrator | `grep -c 'ADR-025' AGENTS.md` ≥ 2 |

**T25 PR count: 1** (the fix branch PR)

### 3.2 Track T26 — L5-110 Substrate Audit (P0, ~45 min, orchestrator + 2 subagents)

Already 80% done in the v8→v9 gap. The 7 drift findings + 1 audit finding are staged:

| Task | Description | P-level | Owner | Acceptance |
|---|---|---|---|---|
| **T26.1** | Verify `worklogs/2026-06-19-L5-110-substrate-audit.json` landed in main | P0 | orchestrator | File present; 7 drift entries |
| **T26.2** | Verify 7 substrate-drift findings (pheno-cargo-template, pheno-ci-templates, pheno-drift-detector, pheno-predict, pheno-ssot-template, pheno-tracing, pheno-vibecoding-guard) | P0 | orchestrator | 7 `worklogs/2026-06-19-L5-110-substrate-drift-*.md` files present |
| **T26.3** | Author ADR-041 (predictive-discipline) verification PR | P0 | orchestrator | ADR-041 in `docs/adr/2026-06-19/`; references L5-110 |
| **T26.4** | Author ADR-042 (graduation-discipline) verification PR | P0 | orchestrator | ADR-042 in `docs/adr/2026-06-19/`; references L5-110 |
| **T26.5** | Open `KooshaPari/pheno` PR for substrate graduation gate table | P0 | orchestrator + subagent A | PR URL; CI green |
| **T26.6** | Open `KooshaPari/pheno` PR for substrate predictive cycle-time guard | P0 | orchestrator + subagent A | PR URL; CI green |
| **T26.7** | Update `findings/2026-06-19-L5-110-substrate-audit.md` with closure status | P0 | orchestrator | Finding updated; "STATUS: CLOSED 2026-06-19" |
| **T26.8** | Mark T26.1-7 done in DAG; release subagent A | P0 | orchestrator | `dagctl done` calls succeed |
| **T26.9** | Subagent A: research "pheno-cargo-template NOT a lib" remediation (move to `pheno-*-lib`?) | P1 | subagent A | SOTA research file; recommendation |

**T26 PR count: 7** (1 ADR-041, 1 ADR-042, 5 sub-PRs)

### 3.3 Track T27 — L5-111 pheno-drift-detector Absorption (P0, ~75 min, orchestrator + 2 subagents)

ADR-043 (drift-detection discipline) is **PROPOSED** 2026-06-19 and **requires the 5 L5-111 P1-P5 patches** to land before the cron first run on 2026-06-23 09:00 PDT.

| Task | Description | P-level | Owner | Acceptance |
|---|---|---|---|---|
| **T27.1** | P1: Fix `pheno-drift-detector/README.md` "Retroactive hits" table (8 of 13 claims fabricated/wrong) | P0 | orchestrator | PR `KooshaPari/pheno-drift-detector#1`; CI green |
| **T27.2** | P2: Fix `pheno_drift_detector.py:134-144` discover pass (PAUSED/CONDITIONAL/CAPSTONE sets) | P0 | subagent B | PR `KooshaPari/pheno-drift-detector#2`; tests pass |
| **T27.3** | P3: Fix `pheno_drift_detector.py:147-204` find-capabilities pass (5 regexes) | P0 | subagent B | PR `KooshaPari/pheno-drift-detector#3`; tests pass |
| **T27.4** | P4: Fix `pheno_drift_detector.py:211-246` score-drift pass (scoring weights) | P0 | subagent C | PR `KooshaPari/pheno-drift-detector#4`; tests pass |
| **T27.5** | P5: Add L74 PAX pillar wiring (cron integration; ADR-043 cross-ref) | P0 | subagent C | PR `KooshaPari/pheno-drift-detector#5`; tests pass |
| **T27.6** | Verify cron first-run mockup (`ops/heavy-runner-cron/FIRST_RUN_MOCKUP.md`) matches P1-P5 behavior | P0 | orchestrator | Mockup checked; expected output matches |

**T27 PR count: 5** (P1-P5)

### 3.4 Track T28 — L5-112 pheno-predict Absorption (P1, ~60 min, orchestrator + 1 subagent)

ADR-041 (predictive discipline) — uses pheno-predict as the L72 measurement instrument.

| Task | Description | P-level | Owner | Acceptance |
|---|---|---|---|---|
| **T28.1** | Audit `pheno-predict` against ADR-041 rubric (4-criterion rule) | P1 | orchestrator | Audit doc; gaps identified |
| **T28.2** | Patch 1: Add retroactive hit detection (cycle-time pattern) | P1 | subagent D | PR `KooshaPari/pheno-predict#1` |
| **T28.3** | Patch 2: Add cycle-time guard (LOC+ time) | P1 | subagent D | PR `KooshaPari/pheno-predict#2` |
| **T28.4** | Patch 3: Wire L72 PAX pillar (cron integration; ADR-041 cross-ref) | P1 | subagent D | PR `KooshaPari/pheno-predict#3` |
| **T28.5** | Update ADR-041 with v9 absorption status | P1 | orchestrator | ADR-041 updated; v9 stamp |

**T28 PR count: 4**

### 3.5 Track T29 — L5-113 pheno-framework-lint Absorption (P1, ~75 min, orchestrator + 1 subagent)

ADR-042 (graduation discipline) — uses pheno-framework-lint as the L73 measurement instrument.

| Task | Description | P-level | Owner | Acceptance |
|---|---|---|---|---|
| **T29.1** | Audit `pheno-framework-lint` against ADR-042 rubric (4-tier gate table) | P1 | orchestrator | Audit doc; gaps identified |
| **T29.2** | Patch 1: Add 4-tier gate table (lib / sdk / framework / federated-service) | P1 | subagent E | PR `KooshaPari/pheno-framework-lint#1` |
| **T29.3** | Patch 2: Add substrate graduation scoring | P1 | subagent E | PR `KooshaPari/pheno-framework-lint#2` |
| **T29.4** | Patch 3: Wire L73 PAX pillar (cron integration; ADR-042 cross-ref) | P1 | subagent E | PR `KooshaPari/pheno-framework-lint#3` |
| **T29.5** | Patch 4: Add canonical lib / sdk / framework / service type detection | P1 | subagent E | PR `KooshaPari/pheno-framework-lint#4` |
| **T29.6** | Update ADR-042 with v9 absorption status | P1 | orchestrator | ADR-042 updated; v9 stamp |

**T29 PR count: 5**

### 3.6 Track T30 — L5-114 Services Retirement (ADR-040 Step 5) (P0, ~60 min, orchestrator)

L5-114 services retirement is **4 of 5 steps complete**; Step 5 (registry fsm flip) pending.

| Task | Description | P-level | Owner | Acceptance |
|---|---|---|---|---|
| **T30.1** | Verify `findings/2026-06-18-L5-114-services-retirement-complete.md` landed | P0 | orchestrator | Finding present; 4 of 5 steps complete |
| **T30.2** | Identify 4 services that flipped fsm to terminal `archived` in registry | P0 | orchestrator | 4 `phenotype-registry` entries with fsm=archived |
| **T30.3** | Author `KooshaPari/phenotype-registry` PR to add 4 SUPERSEDE entries (pre-archive) | P0 | orchestrator | PR `KooshaPari/phenotype-registry#N+1` |
| **T30.4** | Author `KooshaPari/phenotype-registry` PR to add 4 archive entries (post-archive, terminal) | P0 | orchestrator | PR `KooshaPari/phenotype-registry#N+2` |
| **T30.5** | Mark L5-114 complete in AGENTS.md | P0 | orchestrator | "L5-114: COMPLETE 2026-06-19" |
| **T30.6** | Update ADR-040 with Step 5 completion | P0 | orchestrator | ADR-040 Step 5 marked complete |
| **T30.7** | Close out `findings/2026-06-18-L5-114-services-retirement-complete.md` Step 5 section | P0 | orchestrator | Step 5 marked complete |
| **T30.8** | Update STATUS.md with L5-114 closure | P0 | orchestrator | STATUS.md § L5-114: COMPLETE |

**T30 PR count: 2** (registry PRs)

### 3.7 Track T31 — L5-115 pheno-capacity Extraction (HW Ledger) (P0, ~60 min, orchestrator + 1 subagent)

`pheno-capacity` (v0.2.0, L5-115, 2026-06-18) is a **60-test pure-math Rust crate** extracted from HwLedger per ADR-035A. It was created as a **standalone** package — its `Cargo.toml:31-35` explicitly declares an empty `[workspace]` table with the comment "intentionally NOT a member of the root monorepo workspace."

| Task | Description | P-level | Owner | Acceptance |
|---|---|---|---|---|
| **T31.1** | Verify `KooshaPari/pheno-capacity` repo exists, 60 tests pass | P0 | orchestrator | `gh api /repos/KooshaPari/pheno-capacity` returns 200; `pytest` not applicable (Rust) |
| **T31.2** | Run `cargo test -p pheno-capacity` locally; verify 60/60 pass | P0 | orchestrator | 60/60 pass |
| **T31.3** | Audit `pheno-capacity` against substrate placement policy (ADR-023 Rule 3) | P0 | subagent F | Audit doc; placement recommendation |
| **T31.4** | Decide: keep standalone OR absorb into `phenotype-gateway` (per ADR-036) | P0 | orchestrator | Decision doc; ADR-036 cross-ref |
| **T31.5** | If absorb: open `KooshaPari/phenotype-gateway` PR with pheno-capacity source | P0 | subagent F | PR URL; CI green |
| **T31.6** | Update `findings/2026-06-18-L5-115-hwledger-extraction-plan.md` with closure status | P0 | orchestrator | Finding updated; "STATUS: CLOSED 2026-06-19" |
| **T31.7** | Mark L5-115 complete in AGENTS.md | P0 | orchestrator | "L5-115: COMPLETE 2026-06-19" |

**T31 PR count: 1** (absorb PR, if decision is absorb)

### 3.8 Track T32 — L5-117 pheno-capacity Absorb → phenotype-gateway (P0, ~45 min, orchestrator)

Latest commit `59744d667a` (2026-06-19) is the ADR-036 absorb doc; collection-merge plan is staged at `findings/2026-06-19-L5-117-pheno-capacity-collection-merge.md`.

| Task | Description | P-level | Owner | Acceptance |
|---|---|---|---|---|
| **T32.1** | Verify `findings/2026-06-19-L5-117-pheno-capacity-collection-merge.md` is complete (758 lines) | P0 | orchestrator | File present; 758 lines |
| **T32.2** | Verify `findings/2026-06-19-L5-117-absorb-staging-report.md` is READY FOR HUMAN REVIEW | P0 | orchestrator | File present; status: READY |
| **T32.3** | Author `KooshaPari/phenotype-gateway` PR to absorb pheno-capacity | P0 | orchestrator | PR URL; CI green |
| **T32.4** | Update ADR-036 with v9 absorb status | P0 | orchestrator | ADR-036 updated; v9 stamp |
| **T32.5** | Mark L5-117 complete in AGENTS.md | P0 | orchestrator | "L5-117: COMPLETE 2026-06-19" |
| **T32.6** | Update STATUS.md with L5-117 closure | P0 | orchestrator | STATUS.md § L5-117: COMPLETE |

**T32 PR count: 1** (gateway absorb PR)

### 3.9 Track T33 — Side-DAG Filler (NEW v9, P1, ~180 min, 6 subagents rotating)

The 84 ready side-DAG tasks are all for the `pheno` Rust monorepo. They break down as:
- **SOTA research** (10 tasks): side-03..side-12
- **Guardrail** (12 tasks): side-13..side-24
- **Optimize** (10 tasks): side-25..side-34
- **Audit** (12 tasks): side-35..side-46
- **Coverage** (10 tasks): side-47..side-56
- **Doc** (10 tasks): side-57..side-66
- **CI** (10 tasks): side-67..side-76
- **Trace** (10 tasks): side-77..side-86 (84 total, side-03..side-86)

**Owner rotation:** 6 subagents `orch-w2-a`..`orch-w2-f` each pick from the ready pool, do the task, mark done via `dagctl done`, pick next.

**Width-fill mechanism:** if a subagent finishes its current task and the next `ready` task is for a different repo, the subagent can `dagctl fill` to auto-pick from the side pool.

**Completion criteria:** 84/84 tasks marked done in DAG, with at least 60 PRs opened (some tasks are doc-only or research-only and don't need a PR).

### 3.10 Track T0.5 — Wrap-up (P0, ~15 min, orchestrator)

| Task | Description | P-level | Owner | Acceptance |
|---|---|---|---|---|
| **T0.5.1** | Author `plans/2026-06-19-v9-wrapup.md` with closure status of all 20 tracks | P0 | orchestrator | File present; 200+ lines |
| **T0.5.2** | Push final branch to origin (or merge to main if no further work) | P0 | orchestrator | `git push origin main` exits 0 |
| **T0.5.3** | Author v9 postmortem | P0 | orchestrator | `findings/2026-06-19-v9-postmortem.md` present; 200+ lines |
| **T0.5.4** | Update AGENTS.md with v9 closure (date bump, ADR list update) | P0 | orchestrator | AGENTS.md § Wave Plan: "v9 closed 2026-06-19" |
| **T0.5.5** | Update STATUS.md with v9 closure | P0 | orchestrator | STATUS.md § State: "v9 closed" |

---

## 4. Topological Sort

```
T0 (pre-flight) ──┬── T25 (worklog-schema fix) ──┐
                  ├── T26 (L5-110 substrate)     │
                  ├── T27 (L5-111 drift-det)     │
                  ├── T28 (L5-112 predict)       ├── T33 (side-DAG) ── T0.5 (wrap-up)
                  ├── T29 (L5-113 framework)     │
                  ├── T30 (L5-114 services)      │
                  ├── T31 (L5-115 capacity)      │
                  ├── T32 (L5-117 gateway)       │
                  └── T15-T23 (carryover)        │
```

**Critical path:** T0 → T25 (30 min) → T26-T32 (60 min parallel) → T33 (180 min) → T0.5 (15 min) = **~5h critical path with 6-way parallelism**.

---

## 5. PR Matrix

| Repo | v9 PRs |
|---|---|
| `KooshaPari/pheno-worklog-schema` | 1 (T25.4) |
| `KooshaPari/pheno` | 5-7 (T26.5/6, T33 side tasks) |
| `KooshaPari/pheno-drift-detector` | 5 (T27.1-5) |
| `KooshaPari/pheno-predict` | 4 (T28.2-5) |
| `KooshaPari/pheno-framework-lint` | 5 (T29.2-6) |
| `KooshaPari/phenotype-registry` | 2 (T30.3-4) |
| `KooshaPari/phenotype-gateway` | 1 (T32.3) |
| 22 pheno-* substrate repos (T15, T17, T18, T20, T22) | 100+ carryover |
| 8 services retired (T30) | 2 registry PRs |
| **Total** | **~210 PRs** |

---

## 6. Risk Register

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| OmniRoute DOWN breaks forge routing | LOW | MEDIUM | `task` tool fallback (proven in v8) |
| pheno-worklog-schema fix branch rebase conflict | MEDIUM | LOW | Branch is 9 commits ahead; rebase is straightforward |
| 84 side-DAG tasks exceed subagent bandwidth | LOW | MEDIUM | 6 subagents rotating; 84/6 = 14 tasks per agent |
| L5-111 P1-P5 patches miss cron deadline (2026-06-23) | LOW | HIGH | 4 days of runway; 5 patches × 15 min = 75 min; tractable |
| Dmouse92 token accidentally reactivated | LOW | CRITICAL | REMOVED from keyring 2026-06-17 (L5-104 kill-switch); KooshaPari only |
| L5-115 absorb decision (standalone vs gateway) | MEDIUM | MEDIUM | ADR-036 already says "absorb into phenotype-gateway"; T32.4 confirms |

---

## 7. Success Criteria

**v9 is complete when:**

1. ✅ T0 pre-flight gate PASS
2. ✅ T25-T32 all tracks DONE (8 tracks, 53 tasks, 30 PRs)
3. ✅ T33 side-DAG filler DONE (84 tasks done in DAG, 60+ PRs opened)
4. ✅ T15-T23 carryover tracks DONE OR DEFERRED to v10
5. ✅ T0.5 wrap-up doc + postmortem + AGENTS.md/STATUS.md refresh
6. ✅ DAG DB shows 160 done / 0 ready / 0 in_progress / 0 failed (i.e., all original 160 tasks drained)
7. ✅ HEAD on main, 0/0 vs origin, all PRs merged

**Failure modes (auto-escalate to user):**

- Any T0 check FAIL → halt launch
- Any track exceeds 2× estimated effort → escalate
- OmniRoute stays DOWN past 24h → note in postmortem; non-blocking
- 84 side-DAG tasks < 60 done in 6h → drop the rest, defer to v10

---

## 8. Cross-References

### ADRs (49 total, +5 v9)

- **v9 new (2026-06-19):** ADR-036 (pheno-capacity absorb), ADR-041 (predictive-discipline), ADR-042 (graduation-discipline), ADR-043 (drift-detection discipline), ADR-044 (cron-deployment)
- **v8 carryover:** ADR-030..ADR-040 (governance backfill)
- **v7 and earlier:** ADR-001..ADR-029 (core fleet decisions)

### Findings (45+ total, +7 v9)

- `findings/2026-06-19-L5-110-substrate-audit.md`
- `findings/2026-06-19-L5-110-ci-template-audit.md`
- `findings/2026-06-19-L5-110-observability-coverage.md`
- `findings/2026-06-19-L5-110-wip-branch-review.md`
- `findings/2026-06-19-L5-117-absorb-staging-report.md`
- `findings/2026-06-19-L5-117-pheno-capacity-collection-merge.md`
- `findings/71-pillar-2026-06-19.md` (weekly refresh)

### Worklogs (8 new v9)

- `worklogs/2026-06-19-L5-110-substrate-audit.json`
- 7 × `worklogs/2026-06-19-L5-110-substrate-drift-*.md`

### Sibling Plans

- `plans/2026-06-18-v8-dag-stable.md` (superseded by this file)
- `plans/2026-06-17-v7-dag-stable.md` (superseded by v8)
- `plans/2026-06-15-CONSOLIDATED-DAG-V5.md` (consolidated history)

---

## Appendix A — Subagent dispatch protocol

```
1. orchestrator picks:  ./dagctl pick -db FLEET_DAG.db -agent <id>
2. agent does work:     cd <repo> && <work> && <commit> && <push> && <open PR>
3. agent marks done:    ./dagctl done -db FLEET_DAG.db -agent <id> -task <task-id>
4. agent picks next:    ./dagctl pick -db FLEET_DAG.db -agent <id>
5. repeat until 84/84 done or budget exhausted
6. heartbeat every 5 min: ./dagctl heartbeat -db FLEET_DAG.db -agent <id>
```

## Appendix B — Width-fill mechanism

```
if subagent current task DONE and next ready task is for SAME repo:
    pick next ready (continue)
elif subagent current task DONE and no ready for current repo:
    fill from side pool (./dagctl fill -agent <id>)
elif subagent current task DONE and no ready anywhere:
    exit (all work done)
```

## Appendix C — Per-track subagent allocation

| Subagent | Tracks | Task count | Repos touched |
|---|---|---|---|
| **orch-w2-a** | T26 (L5-110) | 9 | pheno + 7 substrate drift fixes |
| **orch-w2-b** | T27 P2-P3 (L5-111) | 3 | pheno-drift-detector |
| **orch-w2-c** | T27 P4-P5 (L5-111) | 3 | pheno-drift-detector |
| **orch-w2-d** | T28 (L5-112) | 5 | pheno-predict |
| **orch-w2-e** | T29 (L5-113) | 6 | pheno-framework-lint |
| **orch-w2-f** | T31 (L5-115) | 7 | pheno-capacity, phenotype-gateway |
| **orchestrator** | T0, T25, T30, T32, T33 (rotation), T0.5 | ~50 | cross-cutting + decisions |

## Appendix D — Heartbeat protocol

```
every 5 minutes:
  for each active subagent in DAG:
    check last_heartbeat timestamp
    if > 10 min stale:
      alert orchestrator; subagent may need restart
    else:
      continue monitoring
```

**DAG heartbeats** (visible via `./dagctl status -db FLEET_DAG.db`): live view of all active subagents, their current task, time-on-task, and progress.
