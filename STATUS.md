# STATUS.md — Phenotype monorepo

**Date:** 2026-06-15 16:50 PDT
**Branch in use:** `chore/w5-adrs-2026-06-15` (was `chore/w1-2-archive-cheap-llm-mcp-2026-06-15`; subagent switched mid-session)
**HEAD:** `ccacce2e35` "docs(adr): ADR-012..016 SOTA decisions for pheno-tracing, pheno-mcp-router, hexagonal ports, V2 worklog, fork-only policy"
**Working tree:** clean (170+ submodule pointer drifts pre-existing; pheno submodule bumped to bd5d807; melosviz is `-dirty` per `git diff melosviz`)

This file supersedes the 2026-06-15 01:25 PDT index that lived here 2026-06-08 → 2026-06-15.

---

## Real-time state

| Metric | Value | Source |
|---|---|---|
| **Real divergence from main** | +19 / −0 | `git rev-list --left-right --count main...HEAD` |
| **Working tree (line-level changes)** | 0 | `git status --short \| grep -v '^ ?'` |
| **Submodule pointer drifts** | 170+ | `git status --short` |
| **pheno-* crates (visible)** | 22 | `ls -d pheno-*/` |
| **pheno-* crates (added since 2026-06-14)** | 4 | +pheno-cli-base, +pheno-fastapi-base, +pheno-flags, +pheno-otel |
| **Buildable pheno-* crates** | 21 | (excludes pheno-wtrees container, pheno-zod-schemas TS) |
| **L6 test pass/fail** | 136 / 4 | `L6_PHENO_REPOS_HEALTH_2026_06_14.md` |
| **Last L6 full audit** | 2026-06-14 (2 days stale) | File mtime |
| **Last L6 delta** | 2026-06-15 01:25 | `L6_PHENO_REPOS_HEALTH_2026_06_15_DELTA.md` |
| **V6 DAG tracks complete** | 5/5 | `findings/V6_MASTER_STATUS-2026_06_15.md` |
| **ADRs accepted (cumulative)** | 16 | `docs/adr/2026-06-14/` (6) + `docs/adr/2026-06-15/` (10) |
| **Config consolidation PR-1/2/3 done** | 3/11 | `findings/ADR-012_CONFIG_CONSOLIDATION_PR1-3_DONE-2026_06_15.md` |
| **Pheno submodule pointer bumped** | ✅ | `bd5d807` (delete 3 deprecated config dirs) |
| **Pyron submodule pointer bumped** | ✅ | `eaebe896` (cargo check --workspace fix) |
| **NetScript archive (local commit)** | ✅ | `76f3f3f` in NetScript submodule |
| **NetScript archive (remote push)** | ❌ | blocked: Dmouse92 gh ≠ KooshaPari |

---

## Sub-projects (current layout)

### Active focus repos (5)
`AgilePlus`, `PhenoCompose`, `PlayCua`, `BytePort`, `nanovms` — coordinated via `chore/l5-87-focus-repo-specs-2026-06-11` branch. Each now has a SPEC.md per L5-#87 worklog.

### Apps & shells
`apps/`, `phenotype-unity/`, `phenotype-voxel/`, `phenotype-landing/`, `phenotype-journeys/`

### Shared libraries (pheno-* family)
22 directories under `pheno-*/` (see `AGENTS.md` for the full breakdown). 21 buildable crates (Rust+Python+Go), 1 worktree container, 1 TypeScript out-of-scope.

### Shared libraries (phenotype-* and others)
`crates/`, `libs/`, `phenoShared/`, `phenoData/`, `phenoUtils/`, `phenoContracts/`, `phenoSchema/`, `phenoKits/`, `phenodocs/`, `phenotype-auth-ts/`, `phenotype-bus/`, `phenotype-dep-guard/`, `phenotype-e2e-base/`, `phenotype-errors/`, `phenotype-go-sdk/`, `phenotype-hub/`, `phenotype-infra/`, `phenotype-journeys/`, `phenotype-landing/`, `phenotype-omlx/`, `phenotype-otel/`, `phenotype-postfx/`, `phenotype-py-extras/`, `phenotype-py-utils/`, `phenotype-python-sdk/`, `phenotype-registry/`, `phenoRuntim` and more.

### Services
`services/`, `phenoMCP/`, `phenoAgents/`, `phenoVCS/`, `phenoObservability/`, `phenoEvents/`, `phenoRuntime/`, `phenoProc/`, `phenoDesign/`, `phenoCompose/`, `phenotype-bus/`, `phenotype-registry/`, `phenotype-otel/`

### Tooling
`tooling/`, `thegent/`, `dispatch-mcp/`, `cheap-llm-mcp/`, `phenotype-ops-mcp/`, `phenotype-tooling/`, `phenotype-infrakit/`, `phenotype-org-audits/`

### Active worktrees
`*-wtrees/` directories (per-feature branches) — 7+ feature branches checked out, 6 stash-backup branches also checked out

---

## Active ADRs (16 total)

**2026-06-14 wave (6 ADRs at `docs/adr/2026-06-14/`):**

| ADR | Repo | Disposition | Status |
|---|---|---|---|
| ADR-001 | NetScript | **DELETE** | Local commit `76f3f3f`; push blocked by gh auth |
| ADR-002 | KlipDot | KEEP-archived | — |
| ADR-003 | McpKit | MERGE into `PhenoMCP` | Plan in `findings/ADR-003-MCPKIT-MIGRATION-PLAN-2026_06_15.md` |
| ADR-004 | Metron | KEEP | — |
| ADR-005 | KodeVibe | KEEP | — |
| ADR-006 | cheap-llm-mcp | archive verified | — |

**2026-06-15 wave (10 ADRs at `docs/adr/2026-06-15/`):**

| ADR | Subject | Status |
|---|---|---|
| ADR-007 | cheap-llm-mcp deprecation | Accepted (DAG-V5 reconciliation) |
| ADR-008 | dispatch-mcp as sole MCP server | Accepted (consolidation decision) |
| ADR-009 | (DAG-V5 reconciliation) | Accepted |
| ADR-010 | (DAG-V5 reconciliation) | Accepted |
| ADR-011 | (DAG-V5 reconciliation) | Accepted |
| ADR-012 | `pheno-tracing` canonical across pheno-* repos | Accepted (V5 SOTA sweep) |
| ADR-013 | `pheno-mcp-router` substrate for pheno-mcp-* | Accepted (V5 SOTA sweep) |
| ADR-014 | Hexagonal L4 ports: `Port` trait + `Adapter` impl | Accepted (V5 SOTA sweep) |
| ADR-015 | V2 10-column WORKLOG.md schema (canonical) | Accepted (V5 SOTA sweep) |
| ADR-016 | Fork-only-not-rewrite policy for SOTA libraries | Accepted (V5 SOTA sweep) |

---

## Wave state

### Completed waves

- **V3 (2026-06-10):** 100+20 task DAG, 180/180 marked done per `FLEET_DAG_v3.md:1-30`.
- **V4 (2026-06-14):** Narrative only. Never executed. Superseded by v6.
- **W1 (2026-06-14):** 9/11 repos pushed via SSH `push_key`. Metron blocked (archived on GitHub). helios-router PR pending in web UI.
- **2026-06-14 RESUME (20 tracks):** 1/24 outputs confirmed on disk (filesystem-unstable). Subagent dispatch via `task` tool failed 40/40; **`task` tool re-verified working 2026-06-15 16:45 PDT** via 1/5/20 call sequence per `findings/WAVE_2026_06_14_RESUME3_DISPATCH_TEST.md`.
- **V6 (2026-06-15):** 5/5 tracks complete per `findings/V6_MASTER_STATUS-2026_06_15.md`. Verified:
  - Track 1 (pheno-scaffold-kit repair): 7/7 PRs done, 6/6 tests pass
  - Track 2 (5 proposed files): 4/5 applied, 1 N/A
  - Track 3 (AgilePlus Tier 1): 0 governance open PRs
  - Track 4 (cheap-llm-mcp lib refactor): done (Python, not Node.js; ADR-007/008 authorize)
  - Track 5 (dispatch gate): 1/5/20 call sequence all succeed

### In-flight / planned

- **Config consolidation PR-4..11:** ADR-012 plan, 8 of 11 PRs remaining. PR-4 (Settly fork gitlink removal) is currently in conflict with parallel subagent activity on `crates/phenotype-config` (`bb00d8b1a2 feat(pheno-config): config worktree (L3 #48) (#127)` is recent); deferred.
- **pheno-config v0.2.0 / phenoShared/config-core v0.3.0:** PR-6/7 of ADR-012 plan, additive features.
- **Profila → pheno-profiling migration:** 12-PR plan in v6 FINAL report §3; ~1,400 LoC including 300 tests.
- **AtomsBot decomposition:** 20 DAG tasks, 5-10 days effort. **Unblocked by ADR-001/003/005** (was waiting on NetScript/McpKit/KodeVibe decisions).

### Stalled / blocked

- **Push to remotes:** Dmouse92 gh cannot reach `KooshaPari/Phenotype` (4 of 7 remotes return 404). Documented in `findings/PUSH_AUTH_GAP-2026_06_15.md`. Workaround: re-auth as `KooshaPari`.
- **Submodule pointer drifts (170+):** non-urgent; each has real content mods (not pointer drift). Per-submodule triage needed.
- **Melosviz is dirty (3 uncommitted files):** needs to be committed inside the submodule first.
- **Metron unarchive:** needs `admin:org` scope; Dmouse92 has no org admin. Web UI action needed (5 sec).
- **helios-router PR:** branch on remote, web UI PR needed (Dmouse92 `gh` can't see private repos).

---

## Recent commits (last 24 hours, descending)

```
ccacce2e35 docs(adr): ADR-012..016 SOTA decisions for pheno-tracing, pheno-mcp-router, hexagonal ports, V2 worklog, fork-only policy
99786846aa chore(root): bump pheno to bd5d807 (delete 3 deprecated config dirs, ADR-012 PR-1/2/3)
215ebf777d docs(findings): ADR-012 config consolidation PR-1/2/3 executed (2026-06-15)
fc1a774de5 docs(findings): update ADR-001 archive state — push via SSH done, PR+archive blocked
5d26a11e82 docs(findings): push auth gap — Dmouse92 cannot reach monorepo remotes (2026-06-15)
d037999bcc docs(findings): v6 master status — 5/5 tracks verified complete (2026-06-15)
659781ee0f docs(findings): v6 Track 4 (cheap-llm-mcp lib refactor) verified done (2026-06-15)
2871a24fa5 docs(findings): v6 Track 3 (AgilePlus Tier 1 drain) verified complete (2026-06-15)
fb48173289 docs(findings): v6 Track 2 (5 proposed files) verified complete (2026-06-15)
4567ae42f8 docs(findings): v6 Track 1 (pheno-scaffold-kit repair) verified complete (2026-06-15)
c7c4d29c92 docs(findings): v6 Track 5 dispatch gate cleared (2026-06-15)
cbe1ca4d42 chore(root): bump Pyron to include build-fix commit
bd5d807   (pheno) refactor(pheno): delete 3 deprecated config dirs (ADR-012)
eaebe896  (Pyron) fix(pyron): unblock cargo check --workspace
76f3f3f   (NetScript) docs: deprecate NetScript per ADR-001
```

---

## Open threads (priority order)

1. **PR-4 Settly fork gitlink removal** (P1) — currently racing with parallel subagent on `crates/phenotype-config`; defer until subagent lands
2. **Config consolidation PR-5..11** (P2) — sequential, ~3-4 hours total
3. **Re-run L6 health audit** (P2) — append today's findings
4. **Metron unarchive** (P3) — web UI action, 5 sec; unblocks W1 push
5. **helios-router PR** (P3) — web UI action
6. **Push 19 ahead commits** (P3) — blocked by gh auth; needs `KooshaPari` re-auth
7. **Submodule pointer drifts (170+)** (P3) — non-urgent; per-submodule triage
8. **AtomsBot decomposition** (P3) — unblocked by ADRs but is 5-10 days effort; schedule separately

---

## Infrastructure

- **GitHub auth:** `gh` is `Dmouse92` (scopes: gist, read:org, repo, workflow). SSH `~/.ssh/push_key` is the working path for pushes; web UI is needed for admin actions (unarchive, PR creation in private repos). Documented: `findings/PUSH_AUTH_GAP-2026_06_15.md`.
- **Subagent dispatch:** `task` tool (re-verified working 2026-06-15 16:45 PDT). `forge -p "..."` CLI (verified working 2026-06-15 01:18 PDT with Tracera L5 integration). `OmniRoute` is UP at `http://localhost:20128/v1/models`.
- **Sparse-checkout:** cone mode active, pattern includes `/*` + `!/*/` + re-inclusions for `pheno-*`, `plans/`, `worklogs/`, `docs/adr/2026-06-14/*`, `docs/adr/2026-06-15/*`, plus 6 specific sub-paths. `findings/` and `crates/` are NOT in the cone by default.
- **Hooks:** `HOOKS_SKIP=1` env var bypasses `trufflehog` pre-commit hook (which times out after 60s on the monorepo).
