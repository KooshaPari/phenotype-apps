# L5-109 v2 — 4-Repo Retirement Plan (Wave 2: 17 Dmouse92 sub-clones) — 2026-06-20

**Status:** Plan ready; awaiting orchestrator dispatch
**Wave:** v12 L5-109 v2 (extends L5-109 v1 from 2026-06-18)
**Depends on:** L5-109 v1 (4-repo retirement, executed 2026-06-18) ✅; ADR-057 (apps/repos boundary) ✅; subagent-B audit (`findings/2026-06-20-subagent-b-dmouse92-clones-audit.md`)
**Width:** 1 orchestrator + up to 4 sub-agents in parallel
**Tasks:** 17 dispositions (5 DELETE-LOCAL, 6 PUSH-TO-KP, 6 KEEP-AS-WORKSPACE) + 1 boundary ADR

---

## Context

The original **L5-109 4-repo retirement** (Wave 1) was executed on **2026-06-18**. The 4 repos were:

| # | Source repo | Target repo | PR |
|---|---|---|---|
| 1 | `KooshaPari/dagctl` | `KooshaPari/phenodag` | [phenodag#13](https://github.com/KooshaPari/phenodag/pull/13) |
| 2 | `KooshaPari/kwality` | `KooshaPari/phenotype-tooling` | [phenotype-tooling#158](https://github.com/KooshaPari/phenotype-tooling/pull/158) |
| 3 | `KooshaPari/phenotype-auth-ts` | `KooshaPari/AuthKit` | [AuthKit#120](https://github.com/KooshaPari/AuthKit/pull/120) |
| 4 | `KooshaPari/dinoforge-packs` | `KooshaPari/Dino` | [Dino#297](https://github.com/KooshaPari/Dino/pull/297) |

All 4 source repos were **archived** (read-only) on 2026-06-18; the migration PRs were opened; the 4 canonical targets absorbed the source content. Wave 1 closed with a finding at `findings/2026-06-19-L5-110-112-second-half-4-repo-absorption-audit.md`.

**Wave 2 (this plan)** extends L5-109 from "4 source repos" to "**17 standalone Dmouse92 sub-clones** inside `/Users/kooshapari/CodeProjects/Phenotype/repos/`". Subagent-B's 2026-06-20 audit categorized the 17 into three disposition buckets:

- **5 DELETE-LOCAL** — Dmouse92 trunk-only orphan mirrors; canonical lives on KooshaPari or has been absorbed
- **6 PUSH-TO-KP** — Dmouse92 sub-clones with active work; need a PR to `KooshaPari/*` canonical, then local `rm -rf`
- **6 KEEP-AS-WORKSPACE** — active dev repos; retain as-is

The user directive (2026-06-20): *"phenotype-apps; apps; repos both need to be absorbed\deleted into relevant repos. start by explaining the boundary of each to me"*. The boundary is codified in **ADR-057** (`docs/adr/2026-06-20/ADR-057-apps-repos-boundary.md`); this plan executes the deletion half of that directive.

## Disposition matrix (the 17)

### DELETE-LOCAL bucket (5 repos, Step 1)

These are Dmouse92 trunk-only orphan mirrors. Each canonical lives on KooshaPari (or has been absorbed into a different repo on KooshaPari). Local content has either been mirrored already or is hygiene-tier with no upstream consumer.

| # | Sub-clone | Local HEAD | Canonical on KP | Safety gate before `rm -rf` |
|---:|---|---|---|---|
| 1 | `HeliosCLI` | `c29d279ed0 chore(retire): absorb final 4 files (CHANGELOG, justfile, codex-rs stub) into helios-cli` | Absorbed into `KooshaPari/helios-cli` (commit `c29d279ed0` is the absorbing commit) | Confirm `helios-cli` HEAD contains the 4 absorbed files (CHANGELOG.md, justfile, codex-rs/ stub, …). Source repo is archived. |
| 2 | `Pyron` | `84b23f96 chore(Pyron): recover stash@{0} (2026-06-20)` | `KooshaPari/Pyron` (archived; merged into Pyron histories on KP per registry) | Confirm KP Pyron mirror exists; the local 5 unpushed commits are hygiene-tier. Source repo is archived. |
| 3 | `HexaKit` | `2008eac chore(t21): security audit hardening — SECURITY.md, gitleaks, audit.sh, SBOM+SLSA` | `KooshaPari/HexaKit` | ⚠ **CAVEAT 1**: local HexaKit has 2 stashes + 2 worktrees; PR #292 (`feat/audit-tools-predict-framework-lint-drift-detector-2026-06-20`) is open and MERGEABLE on `KooshaPari/HexaKit#292`. **Verify the audit-tools branch is pushed + merged before `rm -rf`**. (See §5 Caveats.) |
| 4 | `Tracera` | `b6c4ef85b chore: tier-0 hygiene snapshot 2026-06-20` | `KooshaPari/Tracera` | The local 3 unpushed commits are hygiene-tier. Tracera PR #636 (tier-0 hygiene) is on the archived Dmouse92 repo; KP Tracera already has equivalent hygiene from another merge. |
| 5 | `PhenoContracts` | `f9defc7 fix(ci): switch TruffleHog to filesystem mode; ignore coverage/ dir` | `KooshaPari/PhenoContracts` | ⚠ **CAVEAT 2**: local has 38 unpushed commits + 3 worktrees. Verify each worktree's branch is merged to KP PhenoContracts OR pushed to a KP worktree branch before `rm -rf`. The 38 commits are hygiene-tier (TruffleHog mode, coverage/ ignore). |

### PUSH-TO-KP bucket (6 repos, Step 2)

These are Dmouse92 sub-clones with active work that has not yet landed on KooshaPari canonicals. Each needs a PR opened (or verified open) and merged before local `rm -rf`.

| # | Sub-clone | Branch | Unpushed | Open PRs (per PR-triage) | Action |
|---:|---|---|---:|---|---|
| 6 | `AgilePlus` | `feat/agents-adr-crossref-2026-06-20` | 3 | `#3` (agents ADR cross-ref) | Push branch + verify PR merged; then `rm -rf` |
| 7 | `pheno` | `chore/l5-110-adr-031-configra-canonical-markers-2026-06-18` | 11 | n/a (new) | Push branch + open PR; merge before `rm -rf` |
| 8 | `phenodocs` | `chore/absorb-cargo-template-2026-06-20` | 2 | `#178`, `#179`, `#180`, `#190`, `#191` | Verify all 5 PRs merged on KP; then `rm -rf` |
| 9 | `phenotype-ops` | `t12-devcontainer-ci` | 1 | `#6`, `#7`, `#8`, `#9`, `#10` | Verify all 5 PRs merged on KP; then `rm -rf` |
| 10 | `phenotype-otel` | (trunk) | 0 | n/a (folded into `PhenoObservability`) | Confirm KP canonical is `PhenoObservability` (which folded `phenotype-otel` per `findings/2026-06-19-L5-114-...`); then `rm -rf` |
| 11 | `Nanovms` | `chore/orch-v12-s4-006-deny-audit` | 141 | n/a (large) | Push branch + open PR; merge before `rm -rf` |

### KEEP-AS-WORKSPACE bucket (6 repos, Step 3)

These are active dev repos. Retain as-is. No action this turn.

| # | Sub-clone | Branch | Unpushed | Open PRs | Note |
|---:|---|---|---:|---|---|
| 12 | `Civis` | `main` | 0 | `#476`, `#585` | Active dev; retain |
| 13 | `PhenoPlugins` | `main` | 0 | `#82`, `#84`, `#86` | Active dev; retain |
| 14 | `PhenoCompose` | `t12-devcontainer-ci` | 1 | `#55`, `#56` | Active dev; retain |
| 15 | `OmniRoute` | `chore/l5-121-bifrost-kill-switch-wiring-2026-06-20` | 60 | n/a | Active dev; retain (has worktree `OmniRoute-combos-split`) |
| 16 | `KWatch` | `chore/security-audit-v11-081` | 2 | `#39`, `#40` | Active dev; retain |
| 17 | `PhenoProc` | `wip/2026-06-17-phenoproc-dirty-full-snapshot` | 43 | n/a | Active dev; retain (has 11 sub-crates + 5 worktrees) |

---

## Execution sequence (orchestrator dispatch)

### Step 0 — Pre-flight gate

Before any step, verify:

1. **Local clone `repos/` is on a clean state.** Run:
   ```bash
   git -C /Users/kooshapari/CodeProjects/Phenotype/repos status --short
   ```
   Expected: empty output (or only intentionally-tracked items). Working tree must not have uncommitted changes.

2. **Sub-clones are inventoried.** Run:
   ```bash
   git -C /Users/kooshapari/CodeProjects/Phenotype/repos/<sub-clone> status --short
   ```
   For each of the 17 sub-clones. Catalog the dirty files + stashes + worktrees.

3. **`phenotype-apps` is the upstream.** Run:
   ```bash
   git -C /Users/kooshapari/CodeProjects/Phenotype/repos remote -v
   ```
   Expected: `phenotype-apps` → `git@github.com:KooshaPari/phenotype-apps.git`. If not, fix before continuing.

4. **No module dependencies on the soon-to-be-deleted clones.** Run:
   ```bash
   git -C /Users/kooshapari/CodeProjects/Phenotype/repos submodule status
   ```
   Verify no submodules point at the 5 DELETE-LOCAL clones. (None should, but verify.)

### Step 1 — DELETE-LOCAL (5 clones)

For each of `HeliosCLI`, `Pyron`, `HexaKit`, `Tracera`, `PhenoContracts`:

1. **Verify canonical is in place** on KooshaPari:
   ```bash
   gh api repos/KooshaPari/<canonical> | jq '.full_name, .archived'
   ```
   Expected: `full_name` matches, `archived` is `false` (or `true` for archived-but-valid).

2. **Verify no orphan work exists.** For each sub-clone, check:
   ```bash
   git -C /Users/kooshapari/CodeProjects/Phenotype/repos/<sub-clone> log --oneline origin/main..HEAD 2>/dev/null
   ```
   If any unpushed commits exist, **STOP** — promote that sub-clone from DELETE-LOCAL to PUSH-TO-KP.

3. **Snapshot local branch refs to `phenotype-apps`.** For traceability, append a row to `findings/2026-06-20-L5-109-v2-retirement-log.md`:
   ```bash
   git -C /Users/kooshapari/CodeProjects/Phenotype/repos/<sub-clone> rev-parse HEAD
   git -C /Users/kooshapari/CodeProjects/Phenotype/repos/<sub-clone> branch -a
   ```
   Record the SHAs in the retirement-log.

4. **`rm -rf` the local clone:**
   ```bash
   rm -rf /Users/kooshapari/CodeProjects/Phenotype/repos/<sub-clone>
   ```

5. **Verify:**
   ```bash
   ls /Users/kooshapari/CodeProjects/Phenotype/repos/<sub-clone> 2>&1
   ```
   Expected: `No such file or directory`.

### Step 2 — PUSH-TO-KP (6 clones)

For each of `AgilePlus`, `pheno`, `phenodocs`, `phenotype-ops`, `phenotype-otel`, `Nanovms`:

1. **Verify the canonical is on KooshaPari:**
   ```bash
   gh api repos/KooshaPari/<canonical> | jq '.full_name'
   ```

2. **For each open PR on the sub-clone's Dmouse92 archive**, verify the same PR exists on KP canonical. If it does, follow it through to merge. If it does not, re-create it on KP canonical and link back to the Dmouse92 PR for traceability.

3. **For unpushed local branches** (e.g., `chore/l5-110-adr-031-configra-canonical-markers-2026-06-18` on `pheno`):
   ```bash
   git -C /Users/kooshapari/CodeProjects/Phenotype/repos/<sub-clone> push <kp-canonical-remote> <branch>
   ```
   Then open a PR via:
   ```bash
   gh pr create --repo KooshaPari/<canonical> --head <branch> --base main --title "..." --body "..."
   ```

4. **Wait for CI green + owner review + merge.** The orchestrator owns this loop; this is the long pole.

5. **After merge, `rm -rf` the local clone:**
   ```bash
   rm -rf /Users/kooshapari/CodeProjects/Phenotype/repos/<sub-clone>
   ```

6. **Append a registry row** in `phenotype-registry/registry/disposition-index.json`:
   ```json
   {
     "id": "l5-109-v2-<sub-clone>",
     "source": "KooshaPari/<sub-clone>",
     "target": "KooshaPari/<canonical>",
     "pr": "<pr-number>",
     "merge_sha": "<merge-commit>",
     "local_action": "deleted",
     "date": "2026-06-20"
   }
   ```

### Step 3 — KEEP-AS-WORKSPACE (6 clones, no action)

For each of `Civis`, `PhenoPlugins`, `PhenoCompose`, `OmniRoute`, `KWatch`, `PhenoProc`:

- No action this turn. Leave on disk.
- Future sub-track: relocate under `repos/.worktrees/repo/<name>/` (ADR-057 FU1).

### Step 4 — Closure

After Steps 1-3 complete:

1. **Append a retirement-log row per disposition.** File: `findings/2026-06-20-L5-109-v2-retirement-log.md`. Each row contains: sub-clone name, disposition bucket, KP canonical, PR(s), merge SHA, date, executor (subagent ID), safety-gate pass/fail.
2. **Open a PR on `KooshaPari/phenotype-registry`** with the disposition-index updates (Step 2 #6).
3. **Append to `AGENTS.md`** (separate PR — KooshaPari-directive): the L5-109 v2 wave under "L5-109..114 retirement wave" or equivalent. (NB: per the orchestrator instruction, this turn does NOT modify AGENTS.md — the orchestrator will dispatch that PR.)
4. **Mark this plan SUPERSEDED** in the plan file header:
   ```
   **Status:** SUPERSEDED — 2026-06-20T<HH:MM> PDT — all 11 dispositioned (5 deleted, 6 migrated, 6 retained); see `findings/2026-06-20-L5-109-v2-retirement-log.md`
   ```

---

## Safety gates (must pass before each `rm -rf`)

### Gate G1 — Submodule integrity

```bash
git -C /Users/kooshapari/CodeProjects/Phenotype/repos submodule status
```

If any submodule points at one of the 5 DELETE-LOCAL clones, **STOP** and resolve the submodule pointer first (either migrate the submodule to KP canonical or remove it).

### Gate G2 — No uncommitted work

```bash
git -C /Users/kooshapari/CodeProjects/Phenotype/repos/<sub-clone> status --short
git -C /Users/kooshapari/CodeProjects/Phenotype/repos/<sub-clone> diff --stat
git -C /Users/kooshapari/CodeProjects/Phenotype/repos/<sub-clone> stash list
```

If dirty files or stashes exist:
- Commit them to a WIP branch on KP canonical (PUSH-TO-KP the local clone instead).
- Or document them in the retirement-log (acceptable for hygiene-tier changes only).

### Gate G3 — All branches accounted for

```bash
git -C /Users/kooshapari/CodeProjects/Phenotype/repos/<sub-clone> branch -a
git -C /Users/kooshapari/CodeProjects/Phenotype/repos/<sub-clone> worktree list
```

For each branch and each worktree, verify:
- Branch HEAD is reachable on KP canonical remote, OR
- Branch HEAD is documented in the retirement-log with a KP PR reference, OR
- Branch is a hygiene-only commit and is safe to discard (after `git log --format=%B` review).

### Gate G4 — KP canonical exists and is reachable

```bash
gh api repos/KooshaPari/<canonical> | jq '.full_name, .archived, .default_branch'
```

Expected: `archived = false` (or `true` if archived), `default_branch` is set. If 404, **STOP** — the KP canonical does not exist.

### Gate G5 — Registry updated

After each disposition, append a row to `phenotype-registry/registry/disposition-index.json`. Open a PR on `KooshaPari/phenotype-registry`. Do not close this plan until the registry PR merges.

---

## Critical path

```
Step 0 (pre-flight, ~5 min)
   │
   ├──> Step 1 (5 DELETE-LOCAL, ~15 min total — all can run sequentially; verify gates per clone)
   │
   ├──> Step 2 (6 PUSH-TO-KP, ~30 min to 4 hours per clone — depends on PR merge cadence)
   │       │
   │       └──> 1 sub-agent per PUSH-TO-KP clone (parallel up to 4)
   │
   └──> Step 3 (6 KEEP-AS-WORKSPACE, no action)

Step 4 (closure, ~15 min)
```

**Critical path total:** ~30 min for Step 1 + ~2-4 hours for the slowest Step 2 PR (Nanovms, 141 unpushed commits; PhenoProc-style heavy-review). **Plan total: ~4-5 hours wall time.**

---

## Risk register

| Risk | Mitigation |
|---|---|
| DELETE-LOCAL clone has unpushed work the audit missed (e.g., HexaKit's 2 stashes, PhenoContracts' 3 worktrees) | Gate G3 + the CAVEATS in §5. Each DELETE-LOCAL has an explicit safety gate that promotes it to PUSH-TO-KP if unpushed work is found. |
| KP canonical does not exist or is archived (orphan) | Gate G4. Sub-clones whose KP canonical is missing go back to the disposition review (potentially KEEP-AS-WORKSPACE if no canonical exists). |
| PUSH-TO-KP PR fails CI or review | The local clone is NOT deleted until the PR merges. If the PR is rejected, the local clone stays on disk and goes back to KEEP-AS-WORKSPACE. |
| Registry PR (Step 4) is delayed | Step 4's closure is non-blocking on Step 1 + 2 + 3. The local clones can be deleted without the registry PR; the registry just records the post-hoc state. |
| Sub-clone has an open worktree branch that the parent owns | The worktree's parent repo's git object database is unaffected by the local clone's `rm -rf`. The worktree continues to exist on disk under its `-wt-*` sibling name (e.g., `PhenoMCP-wt-fix-cve-bumps-2026-06-08`); only the parent's local root is removed. |
| `apps` orphan clone (HTTP 404 upstream) is not on the disposition matrix | Per ADR-057 Rule 2, `/repos/apps/` is an orphan. Add it as **row 18 — DELETE-LOCAL** to this plan; gate G4 must be relaxed (HTTP 404 IS the safety check confirming `apps.git` is dead). |

---

## Caveats (discrepancies with the subagent-B audit)

### §5.1 — HexaKit is not "trunk-only"

The subagent-B audit classified `HexaKit` as DELETE-LOCAL on the grounds of "trunk-only". **Local HexaKit is not trunk-only**:

- **2 stashes** exist: `stash@{0}: WIP on main: 1ebf6b0 chore(orch-v12-s1-009): tier-0 hygiene + governance for HexaKit (#292)` and `stash@{1}: WIP on main: 1ebf6b0 chore(orch-v12-s1-009): tier-0 hygiene + governance for HexaKit (#292)`.
- **2 worktrees** exist (per `findings/2026-06-20-local-work-inventory.md` row 162): one local + one detached at `/private/tmp/hexakit-merge`.
- **HEAD is `2008eac chore(t21): security audit hardening`** — a P0-aligned security hardening commit, not a trunk-only mirror.
- **The L72/L73/L74 governance tools** (`pheno-predict`, `pheno-framework-lint`, `pheno-drift-detector`) were absorbed into HexaKit via PR `KooshaPari/HexaKit#292` (`feat/audit-tools-predict-framework-lint-drift-detector-2026-06-20`) — the audit-tools branch is **the canonical home for those tools now**. **DO NOT `rm -rf` HexaKit until PR #292 is verified merged** (recovery epilogue in `findings/2026-06-19-L5-110-112-second-half-4-repo-absorption-audit.md` §"EPILOGUE 3 — HEXAKIT RE-TARGET" tracks this).

**Recommendation:** Promote `HexaKit` from DELETE-LOCAL to PUSH-TO-KP, OR require a Gate G2 + G3 promotion before `rm -rf`. **The orchestrator must verify PR #292 is merged on `KooshaPari/HexaKit` before deletion.**

### §5.2 — PhenoContracts has 38 unpushed commits + 3 worktrees

The subagent-B audit classified `PhenoContracts` as DELETE-LOCAL on the grounds of "trunk-only". **Local PhenoContracts has 38 unpushed commits on the current branch** (`fix/l5-119-quality-p0-2026-06-20`) and **3 worktrees** (`PhenoContracts-wt-fix-orch-v10-005-rebase`, `PhenoContracts-wt-fix-orch-v10-005-rebase-2026-06-14`, plus the parent). The unpushed commits are hygiene-tier (TruffleHog mode, coverage/ ignore), but per Gate G3 they must be either pushed to KP canonical or documented in the retirement-log before `rm -rf`.

**Recommendation:** Promote `PhenoContracts` from DELETE-LOCAL to PUSH-TO-KP (push the 38 commits to `KooshaPari/PhenoContracts` as a hygiene PR; merge; then `rm -rf`). **The orchestrator should NOT `rm -rf` PhenoContracts without first pushing the 38 unpushed commits.**

### §5.3 — Tracera has 3 unpushed commits + an open PR #636

The subagent-B audit classified `Tracera` as DELETE-LOCAL on the grounds of "trunk-only". **Local Tracera has 3 unpushed commits on `chore/tier-0-hygiene-batch`** + an open PR `KooshaPari/Tracera#636` (tier-0 hygiene, 11 comments). Per Gate G3, the 3 commits must be either pushed to KP canonical or documented in the retirement-log.

**Recommendation:** Promote `Tracera` from DELETE-LOCAL to PUSH-TO-KP (push the 3 commits as a new PR; merge; then `rm -rf`). OR document the 3 commits in the retirement-log if they are exactly the same content as PR #636 (in which case, no push needed).

### §5.4 — Pyron has 5 unpushed commits

The subagent-B audit classified `Pyron` as DELETE-LOCAL. **Local Pyron has 5 unpushed commits on `chore/pyron-deps-audit-2026-06-19`** (top commit `84b23f96 chore(Pyron): recover stash@{0} (2026-06-20)`). Per Gate G3, these 5 commits must be pushed or documented.

**Recommendation:** Promote `Pyron` from DELETE-LOCAL to PUSH-TO-KP (push the 5 commits to `KooshaPari/Pyron`; merge; then `rm -rf`). OR document in the retirement-log if the commits are purely tier-0 hygiene already covered by an open PR.

### §5.5 — HeliosCLI is already retired

The subagent-B audit classified `HeliosCLI` as DELETE-LOCAL. **Local HeliosCLI's HEAD is `c29d279ed0 chore(retire): absorb final 4 files (CHANGELOG, justfile, codex-rs stub) into helios-cli`**. The absorbing commit IS the local HEAD — `helios-cli` (KP canonical) already has the absorbed content. **This DELETE-LOCAL is safe and consistent with the audit.**

**Recommendation:** Proceed with the DELETE-LOCAL `rm -rf` for HeliosCLI. No promotion needed.

### §5.6 — Subagent-B audit may have been based on stale state

The audit was performed at an earlier point in the day (per the orchestrator dispatch, the audit was a fresh subagent run). Between the audit and this plan authoring, the local work state may have changed (new commits landed, new stashes created). The Gate G2 + G3 + G4 + G5 are the canonical safety checks; the audit is a hint, not a final verdict.

---

## File plan

This plan produces the following artifacts:

| Path | Purpose |
|---|---|
| `plans/2026-06-20-L5-109-4-repo-retirement-v2.md` | **THIS FILE** — the v2 plan |
| `docs/adr/2026-06-20/ADR-057-apps-repos-boundary.md` | Boundary ADR (authored this turn) |
| `findings/2026-06-20-L5-109-v2-retirement-log.md` | Per-disposition retirement log (authored during Step 4 closure) |
| `phenotype-registry/registry/disposition-index.json` | 17 new rows (6 for PUSH-TO-KP, 5 for DELETE-LOCAL, 6 for KEEP-AS-WORKSPACE) |
| `phenotype-registry/registry/components.lock` | Markers updated to match disposition-index |
| `AGENTS.md` | L5-109 v2 wave closure summary (orchestrator-owned; this turn does NOT modify) |

## References

- **ADR-057 (boundary):** [`docs/adr/2026-06-20/ADR-057-apps-repos-boundary.md`](../docs/adr/2026-06-20/ADR-057-apps-repos-boundary.md)
- **L5-109 v1 finding (executed 2026-06-18):** `findings/2026-06-19-L5-110-112-second-half-4-repo-absorption-audit.md` (the 4-repo v1 retirement + 4-repo second-half absorption)
- **Subagent-B audit (external, this turn):** `findings/2026-06-20-subagent-b-dmouse92-clones-audit.md` (referenced from the orchestrator dispatch)
- **Local work inventory:** `findings/2026-06-20-local-work-inventory.md` (186 repos; 7,948 unpushed commits; 223 worktree handles)
- **PR triage:** `findings/2026-06-20-pr-triage.md` (225 open PRs; 90 unique repos)
- **ADR-023 (app substrate):** `docs/adr/2026-06-15/ADR-023-agent-effort-governance.md`
- **ADR-029 (Dmouse92 → KooshaPari):** `docs/adr/2026-06-15/ADR-029-dmouse92-to-kooshapari.md`
- **ADR-033 (monorepo-state deletion, closed 2026-06-19):** `docs/adr/2026-06-17/ADR-033-phenotype-monorepo-state-deletion.md`
- **v12 DAG:** [`plans/2026-06-20-v12-71-pillar-p0-remediation.md`](../plans/2026-06-20-v12-71-pillar-p0-remediation.md)
- **AGENTS.md (fleet overview):** [`AGENTS.md`](../AGENTS.md)
