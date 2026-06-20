# ADR-057 — Boundary between `phenotype-apps`, `apps`, and `repos/`

- **Status:** Accepted — 2026-06-20 (per user directive *"phenotype-apps; apps; repos both need to be absorbed\deleted into relevant repos. start by explaining the boundary of each to me"*)
- **Date:** 2026-06-20
- **Decision:** @KooshaPari — 2026-06-20 (explained to user; awaiting execution confirmation; this ADR codifies the verbal boundary explanation)
- **Wave:** v12 L7 — Documentation / Governance (T7.x)
- **Related:** ADR-023 (App-effort governance — device + dogfood + app substrate placement); ADR-029 (Dmouse92 → KooshaPari migration); ADR-033 (phenotype-monorepo-state deletion — closed 2026-06-19); L5-109 v2 plan (`plans/2026-06-20-L5-109-4-repo-retirement-v2.md`)
- **Supersedes:** Verbal-only boundary explanation 2026-06-20; the implicit "three repos" assumption that arose from the deletion of `KooshaPari/apps` on 2026-06-20

## Context

On 2026-06-20, the user asked: *"phenotype-apps; apps; repos both need to be absorbed\deleted into relevant repos. start by explaining the boundary of each to me"*. Three identifiers had become conflated in working notes:

1. `KooshaPari/phenotype-apps` — the canonical monorepo (git remote)
2. `KooshaPari/apps` — an orphan shell repo that was DELETED on GitHub at **2026-06-20** (`gh api repos/KooshaPari/apps` returns HTTP 404 as of 2026-06-20 18:45 PDT)
3. `/Users/kooshapari/CodeProjects/Phenotype/repos/` — the local working clone of `phenotype-apps`, on a non-`main` branch (`chore/v12-71-pillar-p0-remediation-2026-06-20`, 129 commits ahead of `main`)

The user's question was triggered by **subagent-B**'s audit of **17 standalone Dmouse92 sub-clones** inside `repos/` (the audit at `findings/2026-06-20-subagent-b-dmouse92-clones-audit.md`, referenced from the orchestrator dispatch). That audit concluded the 17 sub-clones fall into three disposition categories: **5 DELETE-LOCAL**, **6 PUSH-TO-KP**, **6 KEEP-AS-WORKSPACE**.

But before any of the 17 can be retired, the **boundary** between `phenotype-apps` (the canonical monorepo on GitHub), `apps` (the orphan shell that no longer exists), and `repos/` (the local clone) must be unambiguous. This ADR codifies that boundary.

### Why this matters

The conflation has real cost:

- A non-trivial amount of working-time in earlier sessions was spent treating `apps/` (a local sub-clone) as if it were a separate GitHub repo when its remote `origin` already points to `KooshaPari/apps.git` (HTTP 404).
- The local clone `repos/` is sometimes described as "the monorepo" — true at the **filesystem level**, false at the **git level** (the actual git remote is `phenotype-apps`, not `repos`).
- The 151+ sub-directories under `repos/` are a mix of: KooshaPari-owned substrate (`pheno-*`, `phenotype-*`), Dmouse92 standalone sub-clones (`AgilePlus`, `PhenoCompose`, `OmniRoute`, `KWatch`, `HeliosCLI`, `Pyron`, `HexaKit`, `Tracera`, `PhenoContracts`, `Civis`, `PhenoPlugins`, `PhenoProc`, etc.), forked mirrors (`argis-extensions`, `melosviz`), bulk archival copies (`phenotype-monorepo-state`, `phenotype-org-audits`), and local-only worktrees (`-wt-*` siblings). Without an explicit boundary, the disposition matrix cannot be safely executed.

## Decision

There is **exactly one** canonical monorepo at the **GitHub** level, and **exactly one** local clone at the **filesystem** level. The three identifiers resolve as follows.

### 1. Canonical mapping

| Identifier | What it is | GitHub status | Role |
|---|---|---|---|
| `KooshaPari/phenotype-apps` | **The canonical monorepo** (git remote URL: `git@github.com:KooshaPari/phenotype-apps.git`) | ACTIVE (id 1272990345) | Single source of truth for fleet coordination: governance, ADRs, plans, findings, worklogs, registry rows, dispatch rules |
| `KooshaPari/apps` | Orphan shell repo (was a parallel "apps" home that pre-dated the consolidation onto `phenotype-apps`) | **DELETED 2026-06-20** (HTTP 404) | **None.** The GitHub tombstone will expire after 90 days. |
| `/Users/kooshapari/CodeProjects/Phenotype/repos/` | **Local working clone of `phenotype-apps`** (NOT a separate repo; its `phenotype-apps` git remote points at `KooshaPari/phenotype-apps.git`) | n/a (filesystem only) | Working copy. All work branches, stashes, worktrees, and the 151+ sub-clones live here on disk. |

**Rule 1: One GitHub monorepo.** `KooshaPari/phenotype-apps` is the only GitHub-level monorepo. All governance artifacts (ADRs, plans, findings, worklogs, registry rows) MUST be authored here. No parallel "apps" or "repos" GitHub repo exists or should be created.

**Rule 2: `apps` is gone.** `KooshaPari/apps` was deleted on 2026-06-20. Any local clone pointing at `apps.git` (HTTP 404) is an orphan. The local `/repos/apps/` directory is such an orphan — its `origin` remote still resolves to `apps.git`, but pushing to it fails. The local clone's 3 commits (`8df1881 wip: local snapshot 2026-06-18`, `ab51346 chore(governance): add CODEOWNERS...`, `f9dbce2 chore: tier-0 hygiene snapshot 2026-06-20`) are historical and have NO active role. Treat `/repos/apps/` as an **orphan clone to be retired** (L5-109 v2 plan, this wave).

**Rule 3: `repos/` is a clone, not a repo.** `/Users/kooshapari/CodeProjects/Phenotype/repos/` is a local working clone of `phenotype-apps`. Its current branch is `chore/v12-71-pillar-p0-remediation-2026-06-20` (129 commits ahead of `main`). The 151+ sub-directories under it are **sub-clones** of OTHER repos — they are not part of `phenotype-apps` proper. The parent clone (`repos/` itself) only owns the files at the root: `AGENTS.md`, `STATUS.md`, `SSOT.md`, `SPEC.md`, `Cargo.toml`, `go.mod`, `findings/`, `plans/`, `docs/adr/`, `worklogs/`, etc.

### 2. The 151+ sub-clones: what they are

The sub-directories under `repos/` fall into **five categories** by source-of-truth ownership:

| Category | Count (approx) | Examples | Disposition policy |
|---|---:|---|---|
| **KooshaPari substrate** (canonical repos owned by the org) | ~50 | `pheno-config`, `pheno-context`, `pheno-tracing`, `pheno-mcp-router`, `phenotype-go-sdk`, `phenotype-python-sdk`, `phenotype-registry`, `phenotype-tooling`, `HexaKit`, `AuthKit`, `Configra` | Per ADR-023 Rule 3 (substrate placement). Active or paused per `phenotype-registry` rows. |
| **Dmouse92 standalone sub-clones** (the 17 audited) | 17 | `AgilePlus`, `PhenoCompose`, `OmniRoute`, `KWatch`, `HeliosCLI`, `Pyron`, `HexaKit`, `Tracera`, `PhenoContracts`, `Civis`, `PhenoPlugins`, `PhenoProc`, `pheno`, `phenodocs`, `phenotype-ops`, `phenotype-otel`, `Nanovms` | **L5-109 v2 plan disposition matrix** (this wave) |
| **Forked mirrors** (third-party forks) | ~5 | `argis-extensions` (fork of `maximhq/bifrost`), `melosviz`, `KlipDot`, `phenotype-auth-ts` (archived 2026-06-18) | Mirror of upstream; reconciled per fork policy (ADR-016) |
| **Bulk archival copies** (snapshots of deleted repos) | ~25 | `phenotype-monorepo-state` (deleted), `phenotype-org-audits` (deleted 2026-06-20), `phenotype-registry-*` (various deletions), `pheno-otel-wt`, `pheno-secret-scan` | Read-only local archive; do NOT push from these |
| **Local-only worktrees** (`-wt-*` siblings) | ~25 | `PhenoMCP-wt-fix-cve-bumps-2026-06-08`, `Tokn-wt-feat-clap-ext-adopt-rebased-2026-06-14`, `PhenoContracts-wt-fix-orch-v10-005-rebase`, etc. | Worktree copies of their parent repo's branches; follow their parent's disposition |

The 17 sub-clones audited by subagent-B are **Category 2** (Dmouse92 standalone sub-clones). All other categories are out of scope for L5-109 v2.

### 3. The 17 Dmouse92 sub-clones: disposition matrix

Per the subagent-B audit (referenced from the orchestrator dispatch; full text at the audit doc path the orchestrator used to dispatch this task), the 17 sub-clones fall into three disposition buckets. This ADR formalizes those buckets as the canonical disposition going forward.

| # | Sub-clone | Disposition | Reason |
|---:|---|---|---|
| 1 | `HeliosCLI` | **DELETE-LOCAL** | Trunk-only; all content absorbed into `helios-cli` (commit `c29d279ed0 chore(retire): absorb final 4 files`). No upstream KP repo. |
| 2 | `Pyron` | **DELETE-LOCAL** | Trunk-only; no canonical KP upstream. PR #63 (tier-0 baseline) was on the archived Dmouse92 repo (closed via `archived_repo` bucket per PR-triage report). |
| 3 | `HexaKit` | **DELETE-LOCAL** | Trunk-only; canonical HexaKit lives on KooshaPari (`KooshaPari/HexaKit`), and the local sub-clone is a Dmouse92 mirror. (NB: see Caveats §5.1 — the local sub-clone does carry the active L72/L73/L74 governance-tool work merged via PR #292, so deletion MUST be coordinated with the orchestrator's safe-migration gate.) |
| 4 | `Tracera` | **DELETE-LOCAL** | Trunk-only; canonical Tracera lives on KooshaPari. Local sub-clone has 3 unpushed hygiene commits + PR #636 on the archived Dmouse92 repo. |
| 5 | `PhenoContracts` | **DELETE-LOCAL** | Trunk-only; canonical PhenoContracts lives on KooshaPari. Local sub-clone has 38 unpushed hygiene commits on the trunk + 3 worktrees. |
| 6 | `AgilePlus` | **PUSH-TO-KP** | 3 unpushed commits + PR `#3` (agents ADR cross-ref) on `KooshaPari/AgilePlus`. Push branch `feat/agents-adr-crossref-2026-06-20`, open PR if not already open, then `rm -rf` local. |
| 7 | `pheno` | **PUSH-TO-KP** | 11 unpushed commits + worktree on `chore/l5-110-adr-031-configra-canonical-markers-2026-06-18`. Push + PR to `KooshaPari/pheno`, then `rm -rf` local. |
| 8 | `phenodocs` | **PUSH-TO-KP** | 2 unpushed commits on `chore/absorb-cargo-template-2026-06-20` + open PRs `#178`, `#179`, `#180`, `#190`, `#191`. Push + close, then `rm -rf` local. |
| 9 | `phenotype-ops` | **PUSH-TO-KP** | 1 unpushed commit on `t12-devcontainer-ci` + open PRs `#6`, `#7`, `#8`, `#9`, `#10`. Push + close, then `rm -rf` local. |
| 10 | `phenotype-otel` | **PUSH-TO-KP** | Sub-clone is a Dmouse92 mirror; canonical lives on KooshaPari via `PhenoObservability` (which folded `phenotype-otel` per `findings/2026-06-19-L5-114-...`). Confirm KP canonical before deletion. |
| 11 | `Nanovms` | **PUSH-TO-KP** | 141 unpushed commits on `chore/orch-v12-s4-006-deny-audit` + worktree. Push + PR to `KooshaPari/nanovms`, then `rm -rf` local. |
| 12 | `Civis` | **KEEP-AS-WORKSPACE** | Active dev repo. Has open PR #476 (perf: incremental boundary_flux_pass) + #585 (SSOT bundle). Retain as workspace; consider `.worktrees/` integration in a future sub-track. |
| 13 | `PhenoPlugins` | **KEEP-AS-WORKSPACE** | Active dev repo. PRs #82, #84, #86 in the PR-triage report. Retain as workspace. |
| 14 | `PhenoCompose` | **KEEP-AS-WORKSPACE** | Active dev repo. PRs #55, #56 in the PR-triage report. Retain as workspace. |
| 15 | `OmniRoute` | **KEEP-AS-WORKSPACE** | Active dev repo. 60 unpushed commits on `chore/l5-121-bifrost-kill-switch-wiring-2026-06-20` + worktree `OmniRoute-combos-split`. Retain as workspace. |
| 16 | `KWatch` | **KEEP-AS-WORKSPACE** | Active dev repo. PRs #39, #40 in the PR-triage report. Retain as workspace. |
| 17 | `PhenoProc` | **KEEP-AS-WORKSPACE** | Active dev repo. 43 unpushed commits on `wip/2026-06-17-phenoproc-dirty-full-snapshot` + 11 sub-crates + 5 worktrees. Retain as workspace. |

### 4. Naming and location rules

| Rule | Detail |
|---|---|
| **N1: Local-only paths** | Files under `repos/` itself (not inside any sub-clone) are owned by the `phenotype-apps` clone. Sub-clones are owned by their own git remotes. |
| **N2: Governance goes to `phenotype-apps`** | All new ADRs, plans, findings, worklogs, registry rows go into the `phenotype-apps` clone (`docs/adr/`, `plans/`, `findings/`, `worklogs/`). NEVER into a sub-clone. |
| **N3: Sub-clones follow their own git remote** | A sub-clone's working tree is owned by its own git remote (`git -C <sub-clone> remote -v`). Pushes flow through that remote, not through `phenotype-apps`. |
| **N4: `.worktrees/` for shared scaffolding** | Future: a top-level `repos/.worktrees/repo/<name>/` could centralize worktrees of sub-clones. Out of scope this turn; logged as FU1 in L5-109 v2 plan. |
| **N5: Orphan clones get retired** | Any sub-clone whose git remote returns HTTP 404 (orphan) is in scope for the L5-109 v2 DELETE-LOCAL bucket. |

## Consequences

### Positive

1. **One canonical GitHub repo.** Eliminates the "is it `phenotype-apps` or `apps` or `repos`?" confusion. Governance artifacts have exactly one home.
2. **Clear ownership of `repos/` sub-clones.** Each sub-clone has its own git remote; the parent `phenotype-apps` clone is unaffected by sub-clone activity.
3. **The 17 Dmouse92 sub-clones have a disposition.** DELETE-LOCAL removes 5 orphan mirrors; PUSH-TO-KP migrates 6 to KooshaPari canonicals; KEEP-AS-WORKSPACE preserves 6 active dev repos. Net result: zero content loss + cleaner `repos/` layout.
4. **Aligns with ADR-023 Rule 3.** App substrate placement (substrate type by concern) is unaffected; this ADR only clarifies the git-level boundary.

### Negative

1. **5 DELETE-LOCAL clones contain local work.** HeliosCLI's `c29d279ed0` absorbed 4 files into `helios-cli`; HexaKit has 2 stashes + 2 worktrees; PhenoContracts has 38 unpushed commits + 3 worktrees; Tracera has 3 unpushed commits; Pyron has 5 unpushed commits + a recovered stash. Each requires the **safety gates** in L5-109 v2 §3 (work preservation) before `rm -rf`.
2. **6 PUSH-TO-KP clones require orchestrator-coordinated PRs.** Each PR must open, pass CI, and merge on the KooshaPari canonical before the local `rm -rf`. The orchestrator owns this loop.
3. **6 KEEP-AS-WORKSPACE clones retain 151+-repo footprint.** No reduction in disk footprint for these; only the 5 DELETE-LOCAL clones free disk.
4. **Future worktrees may shift.** FU1 (`.worktrees/` integration) would relocate sub-clone worktrees under `repos/.worktrees/repo/`. This is a future sub-track, not part of v2.

### Neutral

1. **The 25+ `-wt-*` sibling worktrees follow their parent's disposition.** No separate ADR needed for them; they are part of their parent repo's git object database.
2. **Bulk archival copies stay read-only.** `phenotype-monorepo-state`, `phenotype-org-audits`, `phenotype-registry-*` mirror copies are historical; they stay on disk for reference but are not pushed from.
3. **The local branch `chore/v12-71-pillar-p0-remediation-2026-06-20` is unrelated.** It belongs to the `phenotype-apps` clone (per ADR-024 / v12 DAG) and is not affected by this ADR.

## Follow-ups

| ID | Priority | Action | Owner | Track |
|---|---|---|---|---|
| FU1 | P2 | Future sub-track: relocate sub-clone worktrees under `repos/.worktrees/repo/` (centralized worktree tree) | orchestrator | Post-v12 |
| FU2 | P1 | Verify each PUSH-TO-KP PR merges cleanly on `KooshaPari/*` before local `rm -rf` (L5-109 v2 Step 2) | orchestrator | L5-109 v2 |
| FU3 | P1 | Update `phenotype-registry/registry/disposition-index.json` for each of the 17 Dmouse92 sub-clones with `fsm: deleted_local` or `fsm: migrated_to_kp` | orchestrator | L5-109 v2 |
| FU4 | P2 | Update AGENTS.md with the boundary table from §1 + §2 above | @KooshaPari | Post-v12 |
| FU5 | P3 | Document the 151+ sub-clone footprint in `phenotype-registry` as a tree view (Category 1-5 from §2) | registry owner | Post-v12 |

## Alternatives considered

### Alternative A — Create a new `KooshaPari/repos` repo to consolidate the local clone *(rejected)*

- **Pros:** Makes the local clone a first-class GitHub entity; pushes can flow through it.
- **Cons:** Splits the monorepo across two GitHub repos (`phenotype-apps` for governance, `repos` for everything else). Defeats the point of having a single monorepo. Adds a new repo to manage without retiring any.
- **Decision:** Rejected. The user directive explicitly said "absorbed\deleted into relevant repos", not "create a new repo to absorb into".

### Alternative B — Treat `apps` as a separate fleet and migrate its 3 local commits to `phenotype-apps` *(rejected)*

- **Pros:** Preserves the `apps` history (3 commits on a now-deleted repo).
- **Cons:** `apps` is HTTP 404 on GitHub; the commits already live in the orphan local clone. Migrating them to `phenotype-apps` adds noise (3 commits of tier-0 hygiene on an orphan path) without value.
- **Decision:** Rejected. The local `/repos/apps/` directory will be retired per L5-109 v2; its 3 commits are documented in `findings/2026-06-20-local-work-inventory.md` row 99 for traceability and do not need migration.

### Alternative C — Keep all 17 sub-clones on disk indefinitely *(rejected)*

- **Pros:** Zero risk of losing local work. Simplest disposition (do nothing).
- **Cons:** The 5 DELETE-LOCAL clones are Dmouse92 orphan mirrors that no longer have an upstream. Their only role is to consume disk + generate confusion in `git status --short` (submodule pointer drift). They offer no archival value because their content already lives on KooshaPari canonicals.
- **Decision:** Rejected. The 5 DELETE-LOCAL are trunk-only orphan mirrors; safety gates in L5-109 v2 §3 guarantee no content loss.

### This ADR's choice — Boundary codification + L5-109 v2 disposition matrix

- **Pros:** Sharp boundary (one GitHub repo, one local clone, 151+ sub-clones with explicit Category 1-5 placement). 17 Dmouse92 sub-clones have explicit disposition. Zero content loss (every DELETE-LOCAL clone's content is mirrored on KooshaPari).
- **Cons:** 5 DELETE-LOCAL clones require safety-gated `rm -rf`. 6 PUSH-TO-KP clones require orchestrator-coordinated PRs. 6 KEEP-AS-WORKSPACE clones retain disk footprint.
- **Decision:** **Adopted.** Aligns with the user directive ("absorbed\deleted into relevant repos") + ADR-023 Rule 3 + the subagent-B audit.

## References

- **Subagent-B audit (external, this turn):** `findings/2026-06-20-subagent-b-dmouse92-clones-audit.md` (referenced from the orchestrator dispatch; not committed to the local clone).
- **v2 retirement plan (this turn):** [`plans/2026-06-20-L5-109-4-repo-retirement-v2.md`](../../plans/2026-06-20-L5-109-4-repo-retirement-v2.md)
- **v1 retirement finding (2026-06-18, executed):** `findings/2026-06-19-L5-110-112-second-half-4-repo-absorption-audit.md` (the 9-repo batch + 4-repo v1 retirement; all absorbed)
- **Local work inventory:** `findings/2026-06-20-local-work-inventory.md` (186 repos; 7,948 unpushed commits; 223 worktree handles)
- **PR triage (225 open PRs):** `findings/2026-06-20-pr-triage.md` (bucket breakdown: supersedable 17, archived_repo 59, docs_only 30, ci_chore 65, absorb 2, feature 21, fix 8, misc 23)
- **Dmouse92 → KooshaPari migration (ADR-029, 2026-06-17):** `docs/adr/2026-06-15/ADR-029-dmouse92-to-kooshapari.md`
- **App-effort governance (ADR-023, 2026-06-15):** `docs/adr/2026-06-15/ADR-023-agent-effort-governance.md`
- **phenotype-monorepo-state deletion (ADR-033, closed 2026-06-19):** `docs/adr/2026-06-17/ADR-033-phenotype-monorepo-state-deletion.md`
- **AGENTS.md (fleet overview):** [`AGENTS.md`](../../../AGENTS.md) — has the v11 closure + 2026-06-19 status
- **`phenotype-registry/registry/disposition-index.json`:** registry rows for `sr-monorepo-state` (closed 2026-06-19), `l5-109-114` (4-repo retirement), `l5-110-112` (second-half 4-repo absorption), `l5-113` (forge-runner-scripts), `l5-115` (pheno-llms-txt)
- **v12 DAG:** [`plans/2026-06-20-v12-71-pillar-p0-remediation.md`](../../plans/2026-06-20-v12-71-pillar-p0-remediation.md)
- **`apps` GitHub state (verified 2026-06-20 18:45 PDT):** `gh api repos/KooshaPari/apps` → HTTP 404 (deleted)
- **`phenotype-apps` GitHub state (verified 2026-06-20 18:45 PDT):** `gh api repos/KooshaPari/phenotype-apps` → id 1272990345, ACTIVE
