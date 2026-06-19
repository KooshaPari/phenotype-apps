# Source Inventory — pheno-cost-card

**Repo:** `KooshaPari/pheno-cost-card`
**Local path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/pheno-cost-card/`
**Current branch:** `chore/adopt-vibecoding-guard-2026-06-15` @ `885f419`
**Default branch (remote):** `main` (per `git remote show origin` — note: `origin/HEAD` symbolic ref is unset; remote `HEAD branch` reports `chore/adopt-vibecoding-guard-2026-06-15`)
**Author identity:** all 11 reachable commits authored by `koosha-ai`; the 12th (unreachable from HEAD) authored by `KooshaPari`
**Generated:** 2026-06-18 (read-only inventory; no source files modified)

---

## 0. Repo At-a-Glance

| Attribute | Value |
|---|---|
| Package name (`pyproject.toml:6`) | `pheno-cost-card` |
| Version (`pyproject.toml:7`) | `0.1.0` |
| Python requirement (`pyproject.toml:10`) | `>=3.10` |
| Build backend | `hatchling` (`pyproject.toml:2-3`) |
| License (declared `pyproject.toml:11`) | `MIT` (text) — but `LICENSE`, `LICENSE-MIT`, **and** `LICENSE-APACHE` are all present (dual MIT + Apache-2.0 in reality) |
| Runtime deps (`pyproject.toml:15`) | `[]` (zero — stdlib-only) |
| Dev deps (`pyproject.toml:18`) | `["pytest"]` |
| Dev deps (`requirements-dev.txt:1`) | `pheno-vibecoding-guard>=0.1.0` |
| Console scripts (`pyproject.toml:5-15`) | **NONE** — no `[project.scripts]` block |
| Entry points | **NONE** |
| Submodules | **NONE** (no `.gitmodules`) |
| Tags | **0** (annotated: 0, lightweight: 0) |
| Local branches | 3 (1 default, 1 chore, 1 wip) |
| Remote branches | 4 (3 mirror local + 1 remote-only `chore/pheno-flake-refresh-pheno-cost-card-2026-06-18`) |
| Total commits reachable from HEAD | 11 |
| Total commits across all refs | 12 |
| Merge commits | 0 |
| Revert commits | 0 |
| Fixup commits | 0 |
| Empty commits | 0 |
| `wip:` commits | 2 (`54c2085`, `27b12d7`) |
| `scaffold` commits | 1 (`630efb8` "initial scaffold") |
| Files in working tree (excl. `.git`, `.pytest_cache`, `__pycache__`) | 33 |
| Total LoC in those 33 files | 1,425 |
| Test files | 1 (`tests/test_smoke.py`, 41 lines) |
| Test count | 2 |
| Test pass rate | 100% (2/2 pass in 1.45s on Python 3.14.5, pytest 9.0.3) |
| Audit scorecard (`audit_scorecard.json:3-4`) | overall: 56, grade: C- |
| Working tree status | clean (no uncommitted changes; no untracked files) |
| Last commit on HEAD | `885f419` — `feat(meta-bundle): SPEC, LICENSE-APACHE, deny.toml, examples, pheno_cost_card __init__` (2026-06-18 03:33:50 -0700, koosha-ai) |
| Last commit on any ref | `7fbfb18` — `chore(meta): pheno-flake refresh — AGENTS.md + llms.txt + WORKLOG.md + CHANGELOG.md + CI (T15.11)` (2026-06-18 17:15:59 -0700, KooshaPari) — **remote-only, not in local clone's working set** |

---

## 1. Working Tree

**33 files** (excl. `.git`, `.pytest_cache`, `__pycache__`). Total: 1,425 LoC.

### 1.1 Complete file inventory (sorted by LoC desc, with last-modified and byte size)

| # | Path | Lines | Bytes | Last-Modified | Category |
|---|---|---:|---:|---|---|
| 1 | `audit_scorecard.json` | 392 | 9,020 | 2026-06-17 | governance (machine) |
| 2 | `.github/PULL_REQUEST_TEMPLATE.md` | 114 | 3,315 | 2026-06-14 | CI / template |
| 3 | `README.md` | 73 | 1,624 | 2026-06-11 | doc |
| 4 | `examples/fleet_card.py` | 69 | 2,086 | 2026-06-18 | example |
| 5 | `src/pheno_cost_card/render.py` | 64 | 2,123 | 2026-06-11 | src code |
| 6 | `src/pheno_cost_card/collectors.py` | 46 | 1,410 | 2026-06-11 | src code |
| 7 | `SPEC.md` | 46 | 1,566 | 2026-06-18 | spec |
| 8 | `llms.txt` | 44 | 938 | 2026-06-11 | doc (LLM-facing) |
| 9 | `CONTRIBUTING.md` | 43 | 1,191 | 2026-06-14 | governance |
| 10 | `tests/test_smoke.py` | 41 | 1,332 | 2026-06-11 | test |
| 11 | `AGENTS.md` | 40 | 1,469 | 2026-06-17 | governance (agent-facing) |
| 12 | `CODE_OF_CONDUCT.md` | 39 | 2,215 | 2026-06-14 | governance |
| 13 | `deny.toml` | 36 | 703 | 2026-06-18 | config (cargo-deny, but Python project) |
| 14 | `justfile` | 33 | 599 | 2026-06-17 | CI / build runner |
| 15 | `.github/ISSUE_TEMPLATE/security_report.md` | 33 | 661 | 2026-06-14 | CI / template |
| 16 | `CHANGELOG.md` | 32 | 936 | 2026-06-14 | doc (release history) |
| 17 | `.github/ISSUE_TEMPLATE/bug_report.md` | 30 | 348 | 2026-06-14 | CI / template |
| 18 | `pyproject.toml` | 25 | 540 | 2026-06-17 | manifest |
| 19 | `SECURITY.md` | 24 | 669 | 2026-06-14 | governance |
| 20 | `.pre-commit-config.yaml` | 24 | 861 | 2026-06-17 | CI |
| 21 | `.github/workflows/ci.yml` | 24 | 507 | 2026-06-17 | CI / workflow |
| 22 | `.github/ISSUE_TEMPLATE/feature_request.md` | 22 | 323 | 2026-06-14 | CI / template |
| 23 | `LICENSE-MIT` | 21 | 1,066 | 2026-06-11 | license |
| 24 | `LICENSE` | 21 | 1,068 | 2026-06-14 | license (MIT, separate copyright) |
| 25 | `.gitignore` | 21 | 194 | 2026-06-11 | config |
| 26 | `.github/ISSUE_TEMPLATE/question.md` | 18 | 267 | 2026-06-14 | CI / template |
| 27 | `src/pheno_cost_card/__init__.py` | 17 | 393 | 2026-06-11 | src code |
| 28 | `LICENSE-APACHE` | 14 | 675 | 2026-06-18 | license |
| 29 | `WORKLOG.md` | 8 | 803 | 2026-06-17 | doc (worklog) |
| 30 | `.github/ISSUE_TEMPLATE/config.yml` | 5 | 165 | 2026-06-14 | CI / template |
| 31 | `.github/CODEOWNERS` | 5 | 139 | 2026-06-14 | governance |
| 32 | `requirements-dev.txt` | 1 | 30 | 2026-06-17 | manifest (dev) |
| 33 | `tests/__pycache__/...` (and similar) | — | — | — | **EXCLUDED** per task spec |

Note: `.cost-card/` directory **does not exist locally** (per `ls -la .cost-card/` → "No such file or directory"). All three collectors (`gh_actions_minutes`, `lfm_token_ledger`, `du_storage`) gracefully return `0.0` when the corresponding `.cost-card/*.json` ledger is missing (`src/pheno_cost_card/collectors.py:15-16`, `:28-29`). Only `du_storage` (`src/pheno_cost_card/collectors.py:37-46`) actually executes on the repo path itself via `subprocess.run(["du", "-sk", str(repo)])`.

### 1.2 Category rollup

| Category | Files | LoC | % of total |
|---|---:|---:|---:|
| **src code** (`src/pheno_cost_card/`) | 3 | 127 | 8.9% |
| **test** (`tests/`) | 1 | 41 | 2.9% |
| **example** (`examples/`) | 1 | 69 | 4.8% |
| **doc** (README, CHANGELOG, SPEC, AGENTS, llms, WORKLOG) | 6 | 243 | 17.1% |
| **governance** (CODE_OF_CONDUCT, CONTRIBUTING, SECURITY, CODEOWNERS, audit_scorecard.json) | 5 | 491 | 34.5% |
| **CI / build** (`.github/`, justfile, .pre-commit-config.yaml, deny.toml, .gitignore) | 11 | 335 | 23.5% |
| **license** | 3 | 56 | 3.9% |
| **manifest** (pyproject.toml, requirements-dev.txt) | 2 | 26 | 1.8% |
| Other (config.yml, etc.) | 1 | 5 | 0.4% |
| **Total** | **33** | **1,425** | 100.0% |

Source for LoC: `find . -type f -not -path './.git/*' -not -path './.pytest_cache/*' -not -path '*/__pycache__/*' -exec wc -l {} +`.

### 1.3 Notable absent files

- **No** `pyrightconfig.json` (exists only on `wip/stash-w5-3-vibecoding-adoption-2026-06-17` branch — not on HEAD; see §2.2)
- **No** `tests/conftest.py` (none)
- **No** `tests/fixtures/` (none)
- **No** `docs/ARCHITECTURE.md` (per `audit_scorecard.json:192` `ARCHITECTURE: false`, `:193` `SSOT: false`, `:194` `CLAUDE: false`)
- **No** `tests/test_*.py` files other than `test_smoke.py`
- **No** `.cost-card/` directory (the canonical ledger location per `collectors.py:14`, `:27`)
- **No** `docs/governance/background_agent_policy.md` (referenced by `CONTRIBUTING.md:39`; does not exist)
- **No** `dist/`, `build/`, `site/`, `mkdocs.yml`, `Sphinx` config — no doc-site tooling

---

## 2. Branches

### 2.1 Branch listing (local + remote)

```
* chore/adopt-vibecoding-guard-2026-06-15                      885f419 feat(meta-bundle): SPEC, LICENSE-APACHE, deny.toml, examples, pheno_cost_card __init__
  main                                                         50c1ac0 docs(changelog): add CHANGELOG.md with [Unreleased] section
  wip/stash-w5-3-vibecoding-adoption-2026-06-17                54c2085 wip: restore stash w5-3-vibecoding-adoption [auto]
  remotes/origin/chore/adopt-vibecoding-guard-2026-06-15       885f419 (same as local)
  remotes/origin/main                                          50c1ac0 (same as local)
  remotes/origin/wip/stash-w5-3-vibecoding-adoption-2026-06-17 54c2085 (same as local)
  remotes/origin/chore/pheno-flake-refresh-pheno-cost-card-2026-06-18 7fbfb18 [REMOTE-ONLY — not in local clone]
```

Source: `git branch -a -v` + `git ls-remote --heads origin`. There is **no** local tracking branch for `chore/pheno-flake-refresh-pheno-cost-card-2026-06-18` — it was fetched (`git fetch origin chore/pheno-flake-refresh-pheno-cost-card-2026-06-18` succeeded 2026-06-18) but never `git checkout`'d.

### 2.2 Branch divergence + unique files

| Branch | SHA | vs `origin/main` ahead/behind | merge-base with `origin/main` | Unique files vs `origin/main` (path:line counts) | Status |
|---|---|---|---|---|---|
| `main` | `50c1ac0` | 0/0 | `50c1ac0` (self) | — | **default** |
| `chore/adopt-vibecoding-guard-2026-06-15` (HEAD) | `885f419` | **4/0** | `50c1ac0` | 9 files: `.pre-commit-config.yaml:1-24 (24L)`, `AGENTS.md:1-40 (40L, modified)`, `LICENSE-APACHE:1-14 (14L)`, `SPEC.md:1-46 (46L)`, `WORKLOG.md:1-8 (8L, modified)`, `audit_scorecard.json:1-392 (392L)`, `deny.toml:1-36 (36L)`, `examples/fleet_card.py:1-69 (69L)`, `requirements-dev.txt:1-1 (1L)` | **active** — `chore/` branch, ahead of main, currently checked out |
| `wip/stash-w5-3-vibecoding-adoption-2026-06-17` | `54c2085` | **1/0** | `50c1ac0` | 4 files: `.github/workflows/ci.yml:1-24 (24L, modified)`, `justfile:1-33 (33L, modified)`, `pyproject.toml:1-25 (25L, modified)`, `pyrightconfig.json:1-46 (46L, NEW)` | **wip/stash** — older worktree-restored WIP commit, behind HEAD by 4 commits (`chore/adopt` is 4 ahead, 1 behind) |
| `origin/chore/pheno-flake-refresh-pheno-cost-card-2026-06-18` | `7fbfb18` | **1/0** (vs `origin/main`) | `50c1ac0` (shared ancestor) | 14 files changed, 776 insertions(+), 113 deletions(-) (per `git diff --shortstat origin/main origin/chore/pheno-flake-refresh-pheno-cost-card-2026-06-18`): `M .github/workflows/ci.yml`, `A .pre-commit-config.yaml`, `M AGENTS.md`, `M CHANGELOG.md`, `A LICENSE-APACHE`, `M LICENSE-MIT`, `A SPEC.md`, `M WORKLOG.md`, `A audit_scorecard.json`, `A deny.toml`, `A examples/fleet_card.py`, `M llms.txt`, `M pyproject.toml`, `A requirements-dev.txt` | **stale-remote** — pheno-flake template refresh (T15.11) sitting on remote only; not pulled into local clone; not in `git rev-list` reachable from local HEAD |

### 2.3 Cross-branch divergence

- `chore/adopt-vibecoding-guard-2026-06-15` vs `wip/stash-w5-3-vibecoding-adoption-2026-06-17`: `4 ahead / 1 behind` (4 unique on `chore/adopt`, 1 unique on `wip/stash`).
- `chore/pheno-flake-refresh-pheno-cost-card-2026-06-18` is a **near-superset** of `chore/adopt-vibecoding-guard-2026-06-15` (T15.11) — it contains the meta-bundle (SPEC, LICENSE-APACHE, deny.toml, examples, audit_scorecard.json) plus a CI modernization commit touching `ci.yml`, `AGENTS.md`, `CHANGELOG.md`, `LICENSE-MIT`, `WORKLOG.md`, `llms.txt`, `pyproject.toml`. **The local `chore/adopt` branch does NOT include the T15.11 refresh** — that work lives only on the remote.
- **Divergence summary:** `chore/adopt` (local) is the older T15.10-equivalent; `origin/chore/pheno-flake-refresh` is the newer T15.11 superset.

### 2.4 Branch naming compliance

- `main` — compliant
- `chore/adopt-vibecoding-guard-2026-06-15` — compliant (`chore/<req-id>-<slug>-<date>` per `AGENTS.md` repo conventions)
- `wip/stash-w5-3-vibecoding-adoption-2026-06-17` — `wip/` prefix signals WIP state, not yet ready for merge
- `chore/pheno-flake-refresh-pheno-cost-card-2026-06-18` (remote-only) — compliant, but its `req-id` is implicit (T15.11 — fleet meta-bundle refresh)

---

## 3. Tags

**0 tags** (annotated: 0, lightweight: 0). Verified via:

- `git tag --list` → empty output
- `git for-each-ref --format='%(refname:short) %(objecttype) %(*objectname:short) %(subject)' refs/tags/` → empty output

`audit_scorecard.json:325-326` corroborates: `"tag_count": 0, "semver_tags": 0`. The package `version` (`pyproject.toml:7`) is `0.1.0` but is not backed by any git tag — i.e., version is declared in metadata only, not enforced by tag pinning. There are no release notes beyond the `CHANGELOG.md:22` `## [0.1.0] - 2026-06-11` block.

---

## 4. Commits

### 4.1 Full commit inventory (chronological asc → desc, all 12 across all refs)

| # | SHA | Author | Date (PT) | Message | Conventional prefix | Files | Insertions / Deletions | Intent category | Notes |
|---|---|---|---|---|---|---:|---|---|---|
| 1 | `630efb8` | koosha-ai | 2026-06-11 12:43 | `feat: initial scaffold (V11 §77 AI-DD crutch, V14 §81)` | `feat:` | 14 | +468 / -0 | **scaffold / init** | First commit. Creates the entire repo: 3 src files, 1 test, README, AGENTS.md, llms.txt, justfile, ci.yml, pyproject.toml, .gitignore, CHANGELOG.md, LICENSE-MIT, WORKLOG.md seed. |
| 2 | `f2f3a25` | koosha-ai | 2026-06-12 18:51 | `docs(worklog): V20 entry (V20)` | `docs:` | 1 | +1 / -0 | **wip / crutch** | Adds 1 line to WORKLOG.md (V20-1.7 row). The `V20` reference matches `V20-1.7` row in `WORKLOG.md:8`. |
| 3 | `764bafe` | koosha-ai | 2026-06-14 23:36 | `ci: add comprehensive PULL_REQUEST_TEMPLATE.md` | `ci:` | 1 | +114 / -0 | governance | Adds 114-line PR template (full P0-P3 template with 9 sections: What/Why/How/Testing/Checklist/Risk/Affected Surfaces/Related/Reviewer). |
| 4 | `3541fc6` | koosha-ai | 2026-06-14 23:40 | `Add ISSUE_TEMPLATE: bug, feature, security, question + chooser config` | (none) | 5 | +108 / -0 | governance | Adds 4 issue templates (bug, feature, security, question) + `config.yml` chooser. Note: **missing `chore:`/`docs:` prefix** — only commit in repo with no conventional prefix. |
| 5 | `cbfd3b1` | koosha-ai | 2026-06-14 23:42 | `chore(governance): add CODE_OF_CONDUCT.md,CONTRIBUTING.md,SECURITY.md,LICENSE` | `chore(governance):` | 4 | +127 / -0 | governance | Boilerplate governance drop. |
| 6 | `a9ddff6` | koosha-ai | 2026-06-14 23:47 | `chore(governance): add CODEOWNERS with @kooshapari as default owner` | `chore(governance):` | 1 | +5 / -0 | governance | 5-line CODEOWNERS, default owner `@kooshapari`. |
| 7 | `50c1ac0` | koosha-ai | 2026-06-14 23:50 | `docs(changelog): add CHANGELOG.md with [Unreleased] section` | `docs(changelog):` | 1 | +15 / -0 | docs | CHANGELOG.md created (not the 32-line version we see today — that was edited later; this is the initial 15-line drop, with `[0.1.0]` block added at `50c1ac0` per `CHANGELOG.md:22`). This is the `main` branch HEAD. |
| 8 | `632693a` | koosha-ai | 2026-06-15 15:56 | `chore: adopt pheno-vibecoding-guard pre-commit hook (V11 §70.3 L16 AX acceptance)` | `chore:` | 3 | +30 / -0 | governance / CI | Adds `.pre-commit-config.yaml` (24L) + `requirements-dev.txt` (1L: `pheno-vibecoding-guard>=0.1.0`) + 5L to AGENTS.md (the "Do Not Touch" section). Per `AGENTS.md:30-34`, this introduces the lockfile + secrets protection policy. **Note:** `deny.toml:2` says "Per ADR-023 Rule 3.1" but the `deny.toml` is added later in commit #11. |
| 9 | `27b12d7` | koosha-ai | 2026-06-17 19:57 | `wip: pre-push snapshot 2026-06-18T02:28:47Z from wrap-up session` | **`wip:`** | 1 | +393 / -0 | **wip** | Adds full `audit_scorecard.json` (392L). **Marked WIP** — the commit message includes a timestamp from a different timezone (UTC vs the rest of the commits which are -0700). Not on `main`; on `chore/adopt` only. |
| 10 | `54c2085` | koosha-ai | 2026-06-17 20:09 | `wip: restore stash w5-3-vibecoding-adoption [auto]` | **`wip:`** | 4 | +59 / -1 | **wip** | Adds `pyrightconfig.json` (47L, strict mode) + 7L to `.github/workflows/ci.yml` + 4L to `justfile` + 1-line tweak to `pyproject.toml`. **Marked WIP** `[auto]` — was an auto-restore from a stash in a different worktree. **Not on HEAD** (only on `wip/stash` branch). |
| 11 | `16bb08c` | koosha-ai | 2026-06-17 23:10 | `chore(worklog): migrate to v2.1 schema (ADR-015, device: column)` | `chore(worklog):` | 1 | +6 / -6 | governance | WORKLOG.md v2.1 schema bump — adds the 11th `device:` column per ADR-015/ADR-025. **But all 4 existing rows have empty `device:` field** (see `WORKLOG.md:5-8`). |
| 12 | `885f419` | koosha-ai | 2026-06-18 03:33 | `feat(meta-bundle): SPEC, LICENSE-APACHE, deny.toml, examples, pheno_cost_card __init__` | `feat(meta-bundle):` | 4 | +169 / -0 | **meta-bundle** (T15.10 / pre-T15.11) | Adds: `SPEC.md` (47L), `LICENSE-APACHE` (15L), `deny.toml` (37L), `examples/fleet_card.py` (70L). **Current HEAD**. **Critical: `examples/fleet_card.py` is broken** — references `card.month`, `card.egress_gb`, `card.llm_tokens` fields that do not exist in `CostCard` (see §7.4). |
| 12a | `7fbfb18` | **KooshaPari** | 2026-06-18 17:15 | `chore(meta): pheno-flake refresh — AGENTS.md + llms.txt + WORKLOG.md + CHANGELOG.md + CI (T15.11)` | `chore(meta):` | 7 | +188 / -117 | meta-bundle (T15.11) | **REMOTE-ONLY, not in local clone's working set.** Author is `KooshaPari` (the human GitHub user, not `koosha-ai`). Modifies `ci.yml`, `AGENTS.md`, `CHANGELOG.md`, `LICENSE-MIT`, `WORKLOG.md`, `llms.txt`, `pyproject.toml`. Pheno-flake template refresh from a different workspace. |

### 4.2 Conventional-prefix breakdown

| Prefix | Count | Commits |
|---|---:|---|
| `feat:` / `feat(meta-bundle):` | 2 | `630efb8`, `885f419` |
| `chore:` / `chore(governance):` / `chore(worklog):` / `chore(meta):` | 5 | `632693a`, `cbfd3b1`, `a9ddff6`, `16bb08c`, `7fbfb18` |
| `docs:` / `docs(worklog):` / `docs(changelog):` | 3 | `f2f3a25`, `50c1ac0` (actually `docs(changelog):`) |
| `ci:` | 1 | `764bafe` |
| `wip:` | 2 | `27b12d7`, `54c2085` |
| (no prefix) | 1 | `3541fc6` ("Add ISSUE_TEMPLATE: ...") |

### 4.3 Empty / fixup / merge / revert

- **Empty commits:** 0
- **Fixup commits:** 0
- **Merge commits:** 0 (`git log --all --merges --first-parent` returns nothing)
- **Revert commits:** 0
- **WIP commits:** 2 (`27b12d7`, `54c2085` — both prefixed `wip:`, both not on `main`, both signal in-flight work)
- **Scaffold commits:** 1 (`630efb8` "initial scaffold")

### 4.4 Author breakdown

| Author | Commits | Notes |
|---|---:|---|
| `koosha-ai` | 11 | All local-branch commits; appears to be a bot/agent identity (consistent `-0700` timezone, lowercase, dash-suffixed) |
| `KooshaPari` | 1 | Only commit `7fbfb18` (the T15.11 pheno-flake refresh, remote-only) — the human org-owner account |

**Author identity inconsistency:** the WIP commits (`27b12d7`, `54c2085`) and the meta-bundle commit (`885f419`) are all by `koosha-ai`, while the more recent T15.11 pheno-flake refresh on remote (`7fbfb18`) is by `KooshaPari`. There is no signed-off-by or co-authored-by trailers anywhere in `git log --pretty=fuller`.

### 4.5 Intent-bearing signal phrases (verbatim)

Quoted from commit messages and `WORKLOG.md` rows:

- `630efb8`: *"V11 §77 AI-DD crutch, V14 §81"* — refers to fleet DAG §77 (AI-DD = AI-driven design? crutch = scaffold?).
- `f2f3a25`: *"V20 entry (V20)"* — links to V20-1.7 row in `WORKLOG.md:8`: *"crutch verification complete"*.
- `632693a`: *"adopt pheno-vibecoding-guard pre-commit hook (V11 §70.3 L16 AX acceptance)"* — L16 = the 16th pillar in the 30-pillar framework (Architecture domain); "AX" likely = "AX" acceptance tier. The acceptance criterion is the pre-commit hook adoption.
- `885f419`: *"meta-bundle"* — refers to a fleet-wide convention of releasing repos with a meta-bundle (SPEC + LICENSE-APACHE + deny.toml + examples + __init__).
- `7fbfb18`: *"pheno-flake refresh — ... (T15.11)"* — `T15.11` is a fleet DAG task ID; "pheno-flake" is a fleet meta-template (compare to `nix flake`, `cog flake`).
- `27b12d7`: *"pre-push snapshot 2026-06-18T02:28:47Z from wrap-up session"* — auto-generated wrap-up session snapshot.
- `54c2085`: *"restore stash w5-3-vibecoding-adoption [auto]"* — `[auto]` indicates it was restored by an automation rather than hand-applied.

### 4.6 WORKLOG.md row entries (verbatim)

`WORKLOG.md:5-8` (4 rows, all with empty `device:` field — see §6.12):

```
| V11-CC-Y3 | 2026-06-11 | pheno-cost-card | L16-AX | initial scaffold (per-repo + fleet cost card) | (this commit) | (none) | merged | koosha-ai |  | V10 Side Y from V4 §64 |
| V14-AA-3 | 2026-06-11 | pheno-cost-card | L14-UX | smoke tests added | (this commit) | (none) | merged | koosha-ai |  | 2/2 tests pass |
| V14-AA-4 | 2026-06-11 | pheno-cost-card | L15-DX | justfile + ci.yml + AGENTS.md + llms.txt | (this commit) | (none) | merged | koosha-ai |  | standard convention |
| V20-1.7 | 2026-06-12 | pheno-cost-card | V20 | crutch verification complete | (this commit) | (none) | merged | koosha-ai |  | crutch verification complete |
```

Note: every row has `commit_sha = (this commit)`, `pr_number = (none)`, `status = merged` — i.e., the worklog was filled in at the time of each commit rather than reconciled to a real git history. All `device:` cells are empty (an ADR-025 v2.1 schema violation).

---

## 5. Dependencies + Submodules

### 5.1 Submodules

**0 submodules.** No `.gitmodules` file exists (verified: `ls -la .gitmodules` → "No such file or directory"). `git submodule status` returns empty.

### 5.2 Runtime dependencies

`pyproject.toml:15`: `dependencies = []` — **zero runtime deps**. The package is pure stdlib (`dataclasses`, `datetime`, `json`, `subprocess`, `pathlib`).

### 5.3 Dev dependencies (from `pyproject.toml`)

`pyproject.toml:17-18`:

```toml
[project.optional-dependencies]
dev = ["pytest"]
```

Single dev dep: `pytest`.

### 5.4 Dev dependencies (from `requirements-dev.txt`)

`requirements-dev.txt:1`:

```
pheno-vibecoding-guard>=0.1.0
```

Single pinned dev dep. The `>=0.1.0` is the only version specifier in the repo. **Note:** `requirements-dev.txt` is **not** referenced by `pyproject.toml`'s `optional-dependencies.dev` list — they are two separate dev-dep lists that have to be reconciled manually.

### 5.5 Console scripts / entry points

**0 console scripts.** `pyproject.toml:5-15` has no `[project.scripts]` block. **Critical claim:** `README.md:30-33` documents the CLI usage:

```bash
pheno-cost-card repo /path/to/repo --month 2026-06
pheno-cost-card fleet /Users/kooshapari/CodeProjects/Phenotype/repos --month 2026-06
```

— **but no `pheno-cost-card` console script is defined in the package metadata.** Invoking this CLI will fail with `command not found: pheno-cost-card`. This is also flagged by `audit_scorecard.json:181-186` (`cli.exists: false`, `cli.cmd: null`, `cli.has_subcommands: false`).

### 5.6 Build system

`pyproject.toml:1-3`:

```toml
[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

- `pyproject.toml:20-21`: `[tool.hatch.build.targets.wheel] packages = ["src/pheno_cost_card"]`
- `justfile:12-13`: `build: python -m build` (depends on `build` package not in dev deps)
- `justfile:27-29`: `release: clean build → twine check dist/*` (depends on `twine` not in dev deps)

### 5.7 Dev tooling (justfile, pre-commit)

`justfile` (33 lines, all targets):

| Target | Command | Notes |
|---|---|---|
| `dev` | `pip install -e ".[dev]"` | only editable-install target |
| `test` | `pytest -v` | |
| `build` | `python -m build` | depends on `build` (not in dev deps) |
| `lint` | `ruff check src tests \|\| true` | **`\|\| true` silently swallows failures** |
| `typecheck` | `mypy src \|\| true` | **`\|\| true` silently swallows failures** |
| `clean` | `rm -rf build dist src/*.egg-info .pytest_cache .ruff_cache .mypy_cache` | |
| `release` | `clean build` → `twine check dist/*` | depends on `twine` (not in dev deps) |
| `audit` | `pip list --outdated` | |

`.pre-commit-config.yaml:1-24` (24 lines, 2 hook sources):

1. `local` hook: `pheno-vibecoding-guard` (line 4). Note (lines 6-9): *"the upstream `pheno-vibecoding-guard scan` CLI subcommand is not yet released (cli.py imports a non-existent `check` symbol from guard.py as of v0.1.0). Until upstream is fixed, we invoke the Python API directly."* Entry: `python -c "import sys; from pheno_vibecoding_guard import scan_repo; r = scan_repo('.'); sys.exit(0 if r.clean else 1)"`. `pass_filenames: false`.
2. `https://github.com/pre-commit/pre-commit-hooks` rev `v5.0.0`: `check-yaml`, `check-toml`, `end-of-file-fixer`, `trailing-whitespace`.

### 5.8 `deny.toml` (37 lines) — wrong tool for this repo

`deny.toml:1-2`: *"cargo-deny config for pheno-cost-card / Per ADR-023 Rule 3.1"*. The file is a valid `cargo-deny` config (advisories, licenses, bans, sources) — but **this is a Python project, not a Rust project**. There is no `Cargo.toml` in the repo. `cargo-deny` cannot be run against this codebase. Per `AGENTS.md:5-7` ("Standard Python src/ layout with hatchling") the deny.toml is a copy-paste from a Rust template (ADR-023 Rule 3.1 says "every substrate ships with cargo-deny config" — but that rule applies to Rust substrates; pheno-cost-card is a Python library, so the rule doesn't apply). **This file is decorative.**

### 5.9 CI workflows

`.github/workflows/ci.yml` (24 lines, 1 job `test`):

- Triggers: `push: branches: [main]`, `pull_request:`
- Matrix: `os: [ubuntu-latest, macos-latest]`, `python-version: ["3.10", "3.11", "3.12", "3.13"]` (8 combinations)
- Steps: `actions/checkout@v4` → `actions/setup-python@v5` → `pip install -e ".[dev]"` → `pytest -v`
- **No scheduled jobs** (despite `SECURITY.md:18-24` claiming "Monday cron" cargo-deny and "Tuesday weekly" CodeQL — neither workflow file exists in `.github/workflows/`).
- **No** CodeQL config
- **No** dependabot config
- **No** release workflow

`audit_scorecard.json:236-238` confirms: `workflow_files: ["ci.yml"]`, `has_precommit: true`.

---

## 6. Governance Artifacts

13 governance-artifact files. Each reviewed below.

### 6.1 `AGENTS.md` (40 lines)

`AGENTS.md:1` title: *"# AGENTS.md — pheno-cost-card"*. Sections: `## Purpose`, `## Build & Test`, `## Repo conventions`, `## Do Not Touch`, `## Reference`. **Claims:**

- Purpose (L5-7): *"Per-repo and fleet cost card. Tracks monthly CI minutes, LLM token spend in USD, and storage GB. Renders as 1-page markdown per repo + 1-page fleet card aggregating all repos."*
- Conventions (L18-22): src/ layout, frozen `CostCard`, `render.*` pure, `collectors.*` reads checked-in `.cost-card/*.json` ledgers.
- Do-Not-Touch (L26-34): `CostCard` fields are wire format, `render_*` signatures are external API, lockfile + submodule pin + secret patterns enforced by `pheno-vibecoding-guard`.
- Reference (L38-39): hardcoded local path `FLEET_100TASK_DAG_V4.md` §64 Side Y + §78.6 (V13 grand-total).

**Issue:** hardcoded local machine path in §Reference (`/Users/kooshapari/CodeProjects/Phenotype/repos/FLEET_100TASK_DAG_V4.md`) will not resolve for any other contributor.

### 6.2 `SPEC.md` (47 lines) — added 2026-06-18, **inconsistent with implementation**

`SPEC.md:1-47` title: *"pheno-cost-card — SPEC"*. **Claims** (verbatim):

- `SPEC.md:12-14`: *"pheno_cost_card.CostCard — frozen dataclass with fields: `repo`, `ci_minutes`, `storage_gb`, `egress_gb`, `llm_tokens`, `month` (str, ISO YYYY-MM)."*
- `SPEC.md:15-16`: *"pheno_cost_card.collectors.gh_actions_minutes(repo: Path, month: str) -> float — query the GitHub Actions API for monthly minutes used."*
- `SPEC.md:17-18`: *"pheno_cost_card.collectors.local_storage_gb(repo: Path) -> float — sum `.git` size + artifacts on disk."*
- `SPEC.md:36-37`: *"71-pillar score: 22/71 (Tier 0)"*
- `SPEC.md:38`: *"Test matrix: 5 unit tests (smoke + per-collector + render)"* — **actual: 2 unit tests.**
- `SPEC.md:40`: *"License: dual (MIT + Apache-2.0)"* — **accurate** (LICENSE + LICENSE-MIT + LICENSE-APACHE all present).

**Critical discrepancies vs implementation** (per `src/pheno_cost_card/__init__.py:7-14`):

| SPEC claims | Implementation has | Status |
|---|---|---|
| `egress_gb` field | **NOT** in `CostCard` | MISMATCH |
| `llm_tokens` field | `llm_tokens_usd` (different name, different semantic) | MISMATCH |
| `month` field | **NOT** in `CostCard` | MISMATCH |
| `local_storage_gb` collector | **`du_storage`** (different name, uses `du -sk`, not `.git` size + artifacts) | MISMATCH |
| 5 unit tests | **2** unit tests | MISMATCH |
| 22/71 pillar score | audit_scorecard.json reports **overall: 56, grade: C-** (which is the 30-pillar scoring, not 71-pillar) | UNCLEAR / MIXED FRAMEWORKS |

### 6.3 `README.md` (73 lines)

Sections: Purpose, Install, Usage, Output, Collectors. **Claims:**

- `README.md:3`: *"Track per-repo monthly burn in CI minutes + LLM tokens + storage. Produces a cost card in markdown for each repo, rolled up to a fleet card."*
- `README.md:30-33`: CLI usage `pheno-cost-card repo /path/to/repo --month 2026-06` and `pheno-cost-card fleet /Users/kooshapari/CodeProjects/Phenotype/repos --month 2026-06` — **but no console script is defined** (see §5.5).
- `README.md:36-65`: sample output tables — match what `render.render_repo_card` and `render.render_fleet_card` actually produce.
- `README.md:67-72`: collectors list (`gh_actions_minutes`, `lfm_token_ledger`, `du_storage`) — accurate, matches `src/pheno_cost_card/collectors.py:8`, `:21`, `:37`.

### 6.4 `llms.txt` (44 lines)

LLM-facing quickstart. Accurate wrt `__init__.py` / `collectors.py` (no broken field references). `llms.txt:15-26` example uses `repo`, `ci_minutes`, `llm_tokens_usd`, `storage_gb`, `contributors` — all valid `CostCard` fields. `llms.txt:32-36` collector example is also accurate.

### 6.5 `WORKLOG.md` (8 lines)

4-row v2.1-schema table. All rows have **empty `device:` cell** (column added in commit `16bb08c` per ADR-015/ADR-025, but rows are not back-filled). See full table in §4.6.

### 6.6 `CHANGELOG.md` (32 lines)

- L1-7: header citing Keep a Changelog 1.1.0 + Semantic Versioning 2.0.0
- L8-21: `[Unreleased]` block (all 6 sub-headers empty: Added/Changed/Deprecated/Removed/Fixed/Security)
- L22-31: `[0.1.0] - 2026-06-11` block listing the initial scaffold
- L32: `[Unreleased]` comparison link (note: link text is `[Unreleased]` but the URL is `compare/HEAD` — could be `compare/v0.1.0...HEAD`)

### 6.7 `CODE_OF_CONDUCT.md` (39 lines)

Standard Contributor Covenant v2.1, with reporting contact set to `KooshaPari (https://github.com/KooshaPari)` (L31). No custom modifications.

### 6.8 `CONTRIBUTING.md` (43 lines)

Standard fork-feature-PR flow + conventional-commits reference (L11, L17-24). L33-43: governance section pointing at `docs/governance/background_agent_policy.md` (L39) — **file does not exist in repo** (orphaned reference).

### 6.9 `SECURITY.md` (24 lines)

L1-15: standard GitHub Security Advisories reporting flow.
L18-20: *"Rust projects in this org enforce a zero-advisory floor via `cargo-deny.yml` workflow (Monday cron + on-demand)."* — **this is a Python project, and the referenced workflow does not exist.**
L22-24: *"Static analysis runs Tuesday weekly via `codeql-rust.yml` workflow."* — **`codeql-rust.yml` does not exist**, and this is a Python project, not Rust.

**Both workflow files referenced in `SECURITY.md` are absent from `.github/workflows/`.** The only workflow is `ci.yml`.

### 6.10 `LICENSE` (21 lines)

MIT License, Copyright (c) 2026 Koosha Pari. Identical text to `LICENSE-MIT` except for the copyright holder (`Koosha Pari` vs `Phenotype`).

### 6.11 `LICENSE-MIT` (21 lines)

MIT License, Copyright (c) 2026 Phenotype.

### 6.12 `LICENSE-APACHE` (15 lines)

Apache License 2.0 (truncated — only the preamble; does not include the full Apache 2.0 body text that runs to ~200 lines). Verified: only 15 lines.

### 6.13 `CODEOWNERS` (5 lines)

Default owner: `* @kooshapari`. No path-specific overrides.

### 6.14 `audit_scorecard.json` (392 lines)

Machine-readable audit output. `audit_scorecard.json:2-4` header: `repo: "pheno-cost-card"`, `overall: 56`, `grade: "C-"`. The 30 sub-scores range from 25 (`L6 Performance`) to 100 (`L1 Architecture`, `L5 Security`, `L9 Complexity`, `L25 Vendor Lockin`). The `raw` section at L159-391 is auto-generated with explicit zeros for features the project does not implement (e.g., `L11 Dependencies: dep_count: 0` at L258, `L14 Data Layer: orm_refs: 0, migration_files: 0, redis_refs: 0, sqlite_refs: 0` at L271-274, `L27 Infrastructure: dockerfile: 0, docker_compose: 0, k8s_manifests: 0, terraform_files: 0` at L348-351). Notable findings:

- `audit_scorecard.json:43`: `L2 Dev Loop: "1 test files, 2 collected, 0 errors."` (matches actual test count of 2)
- `audit_scorecard.json:46-47`: `L3 Agent Loop: "CLI: MISSING. CI: 1 workflows."` (matches actual: no console script + 1 ci.yml)
- `audit_scorecard.json:160-164`: `source.total: 3, over_500: 0, over_350: 0, oversized_files: []` (3 src files, all under 350 lines)
- `audit_scorecard.json:171-173`: `tests.files: ["tests/test_smoke.py"]` (matches)
- `audit_scorecard.json:181-186`: `cli.exists: false, cmd: null, has_subcommands: false, help_length: 0` (confirms no console script)
- `audit_scorecard.json:191-198`: `docs.files: {README: true, ARCHITECTURE: false, SSOT: false, CLAUDE: false, AGENTS: true, CONTRIBUTING: true, CHANGELOG: true, LICENSE: true}` (4 of 8 canonical docs present; ARCHITECTURE/SSOT/CLAUDE absent — no SSOT.md, no CLAUDE.md, no ARCHITECTURE.md)
- `audit_scorecard.json:230-232`: `git.recent_commits: 8` (snapshot at the time of the scorecard generation — 8 commits at that time; current is 11)

**The scorecard uses a 30-pillar framework (L1-L30), not the 71-pillar framework (L1-L71) that AGENTS.md and SPEC.md both reference.** This is a framework-version mismatch in the org-wide audit rollout (per `findings/2026-06-17-71-pillar-mapping.md`, the 30-pillar version is now superseded by 71-pillar as of 2026-06-17).

### 6.15 CI / GitHub

- `.github/CODEOWNERS` (5 lines, see §6.13)
- `.github/PULL_REQUEST_TEMPLATE.md` (114 lines, comprehensive P0-P3 template with sections: What/Why/How/Type of Change/Testing/Test Commands/Test Evidence/Checklist/Risk & Rollout/Affected Surfaces/Related/Reviewer Notes)
- `.github/workflows/ci.yml` (24 lines, see §5.9)
- `.github/ISSUE_TEMPLATE/config.yml` (5 lines, chooser config pointing at `https://github.com/KooshaPari`)
- `.github/ISSUE_TEMPLATE/bug_report.md` (30 lines, `labels: ["bug"]`)
- `.github/ISSUE_TEMPLATE/feature_request.md` (22 lines, `labels: ["enhancement"]`)
- `.github/ISSUE_TEMPLATE/security_report.md` (33 lines, `labels: ["security"]`)
- `.github/ISSUE_TEMPLATE/question.md` (18 lines, `labels: ["question"]`)

---

## 7. Public API

### 7.1 Module structure

```
src/pheno_cost_card/
    __init__.py        (17 lines) — exports `CostCard` only
    collectors.py      (46 lines) — 3 collector functions
    render.py          (64 lines) — 3 render functions, imports `CostCard` from `pheno_cost_card`
```

### 7.2 `pheno_cost_card.CostCard` (`src/pheno_cost_card/__init__.py:7-14`)

```python
@dataclass(frozen=True)
class CostCard:
    repo: str
    ci_minutes: float
    llm_tokens_usd: float
    storage_gb: float
    computed_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))
    contributors: tuple[str, ...] = ()
```

- **Frozen dataclass** (immutable; `frozen=True` on L7).
- **6 fields**: `repo` (str, required), `ci_minutes` (float, required), `llm_tokens_usd` (float, required), `storage_gb` (float, required), `computed_at` (datetime, defaults to `datetime.now(timezone.utc)`), `contributors` (tuple of str, defaults to `()`).
- **`__all__ = ["CostCard"]`** (L17) — only `CostCard` is exported from the package root.

### 7.3 `pheno_cost_card.collectors` (`src/pheno_cost_card/collectors.py`)

| Function | Signature | Docstring summary | LoC |
|---|---|---|---|
| `gh_actions_minutes` | `(repo: Path, month: str) -> float` (L8) | "Collect GitHub Actions minutes for a repo/month. This reads a checked-in or generated billing export when present. The expected file is `.cost-card/gh-actions-minutes.json` with `{\"YYYY-MM\": minutes}`." | 8-18 |
| `lfm_token_ledger` | `(repo: Path, month: str) -> float` (L21) | "Collect LLM token spend in USD from a local ledger. The expected file is `.cost-card/lfm-token-ledger.json` with either `{\"YYYY-MM\": usd}` or `{\"YYYY-MM\": {\"usd\": usd}}`." | 21-34 |
| `du_storage` | `(repo: Path) -> float` (L37) | "Measure repository storage in GB using `du -sk`." | 37-46 |

All three are pure read-only functions. Only `du_storage` shells out (`subprocess.run(["du", "-sk", str(repo)])` at L39-44 with `check=True, capture_output=True, text=True`).

### 7.4 `pheno_cost_card.render` (`src/pheno_cost_card/render.py`)

| Function | Signature | Docstring | LoC |
|---|---|---|---|
| `cost_trend_arrow` | `(current_usd: float, previous_usd: float \| None = None) -> str` (L8) | (no docstring) | 8-15 |
| `render_repo_card` | `(card: CostCard, previous_total_usd: float \| None = None) -> str` (L18) | (no docstring) | 18-38 |
| `render_fleet_card` | `(cards: Iterable[CostCard], previous_total_usd: float \| None = None) -> str` (L41) | (no docstring) | 41-64 |

These are imported as `from pheno_cost_card.render import ...` (per `tests/test_smoke.py:4` and `llms.txt:16`). They are **NOT re-exported** through `pheno_cost_card/__init__.py`, so `from pheno_cost_card import render_fleet_card` would fail (only `CostCard` is in `__all__`).

**`cost_trend_arrow` arrow characters:** `->` (no-change, L10 and L15), `↑` (up, L12), `↓` (down, L14).

### 7.5 Console-script entry points

**0 console scripts** (see §5.5). `pyproject.toml:5-15` has no `[project.scripts]` block. `README.md:30-33` and `SPEC.md:29-33` both reference a `pheno-cost-card` CLI that does not exist in the installed package.

### 7.6 `__all__` exports

- `src/pheno_cost_card/__init__.py:17`: `__all__ = ["CostCard"]` — only public symbol at package root.
- `src/pheno_cost_card/collectors.py`: **no `__all__`** — all 3 functions are public by Python convention.
- `src/pheno_cost_card/render.py`: **no `__all__`** — all 3 functions are public by Python convention.

### 7.7 **BROKEN: `examples/fleet_card.py` references non-existent `CostCard` fields**

`examples/fleet_card.py:18-69` defines its own `render_fleet_card` and `render_repo_card` that access `card.month`, `card.egress_gb`, `card.llm_tokens` — **none of these fields exist on `CostCard`** (see §7.2). Verified empirically:

```
$ python3 -c "from pheno_cost_card import CostCard; c = CostCard(repo='x', ci_minutes=1.0, llm_tokens_usd=1.0, storage_gb=1.0); print(c.month)"
AttributeError: 'CostCard' object has no attribute 'month'
```

- `examples/fleet_card.py:5-10` (docstring): example code that fails the same way
- `examples/fleet_card.py:22`: `headers = ["repo", "month", "ci_min", "storage_gb", "egress_gb", "llm_tokens"]` — references `month`, `egress_gb`, `llm_tokens` (NONE exist)
- `examples/fleet_card.py:28`: `f"{c.ci_minutes:.1f}"` — works (field exists)
- `examples/fleet_card.py:29`: `f"{c.egress_gb:.2f}"` — **FAILS at runtime** (`AttributeError`)
- `examples/fleet_card.py:30`: `f"{c.llm_tokens:,}"` — **FAILS at runtime** (`AttributeError`)
- `examples/fleet_card.py:34-37`: `sum(c.egress_gb for c in cards)`, `sum(c.llm_tokens for c in cards)` — **FAILS at runtime**
- `examples/fleet_card.py:55-58` (`render_repo_card`): same broken field references

Additionally, `examples/fleet_card.py:18` defines `render_fleet_card(cards: list[CostCard])` with a **different signature** from the working `pheno_cost_card.render.render_fleet_card(cards: Iterable[CostCard], previous_total_usd=None)` at `render.py:41`. Two definitions of the same name with different signatures; the example one is broken.

The `examples/` directory is also not importable as a Python package (no `__init__.py`); `python3 -c "from examples.fleet_card import ..."` fails with `ModuleNotFoundError: No module named 'examples.fleet_card'`.

This `examples/fleet_card.py` is the **defining artifact of SPEC.md** (which has identical field names). It is a **broken, stale draft** that was committed in the meta-bundle commit `885f419` without verification — `pytest` does not exercise it (only `tests/test_smoke.py` runs), and there is no `examples` test in `pyproject.toml:23-24` (`testpaths = ["tests"]`).

### 7.8 Import dependency graph (within `src/`)

```
__init__.py  ──[imports: dataclasses, datetime]──>  (no internal imports)
collectors.py ──[imports: json, subprocess, pathlib]──>  (no internal imports)
render.py    ──[imports: collections.abc.Iterable, pheno_cost_card.CostCard]──>  __init__.py
```

`render.py:5`: `from pheno_cost_card import CostCard` — this works because `pyproject.toml:24` sets `pythonpath = ["src"]`.

---

## 8. Tests

### 8.1 Test files

**1 test file:** `tests/test_smoke.py` (41 lines).

There is **no** `tests/conftest.py`. There is **no** `tests/fixtures/` directory. There are **no** parametrized tests, no `mock`/`patch`/`hypothesis`/`property` tests. The audit scorecard confirms: `audit_scorecard.json:312-316` `"parametrize": 0, "fixtures": 0, "mock": 0, "patch": 0`, `:319-321` `"hypothesis": 0, "fuzzing": 0, "property_tests": 0`.

### 8.2 Test functions

`tests/test_smoke.py` (2 functions):

| # | Test | LoC | What it tests |
|---|---|---:|---|
| 1 | `test_repo_card_renders_core_metrics` (L7-24) | 18 | Constructs a `CostCard(repo="example", ci_minutes=120.0, llm_tokens_usd=4.25, storage_gb=1.5, computed_at=datetime(2026, 6, 11, tzinfo=timezone.utc), contributors=("a", "b", "a"))`, calls `render_repo_card(card, previous_total_usd=3.0)`, asserts 6 string fragments present (header, CI line, LLM line, Storage line, Contributors line, trend arrow ↑) |
| 2 | `test_fleet_card_aggregates_cards` (L27-41) | 14 | Constructs 2 `CostCard` instances, calls `render_fleet_card(cards, previous_total_usd=4.0)`, asserts 7 string fragments present (header, repos, CI total, LLM total, Storage total, Contributors count, trend arrow ↓) |

### 8.3 Pytest collect-only

```
$ python3 -m pytest --collect-only -q
tests/test_smoke.py::test_repo_card_renders_core_metrics
tests/test_smoke.py::test_fleet_card_aggregates_cards

2 tests collected in 3.96s
```

### 8.4 Pytest run

```
$ python3 -m pytest -v
============================= test session starts ==============================
platform darwin -- Python 3.14.5, pytest-9.0.3, pluggy-1.6.0 -- /opt/homebrew/opt/python@3.14/bin/python3.14
cachedir: .pytest_cache
hypothesis profile 'default'
rootdir: /Users/kooshapari/CodeProjects/Phenotype/repos/pheno-cost-card
configfile: pyproject.toml
testpaths: tests
plugins: hypothesis-6.155.2, anyio-4.11.0, cov-7.1.0, asyncio-1.4.0, respx-0.23.1
asyncio: mode=Mode.STRICT, debug=False, asyncio_default_fixture_loop_scope=None, asyncio_default_test_loop_scope=function
collecting ... collected 2 items

tests/test_smoke.py::test_repo_card_renders_core_metrics PASSED          [ 50%]
tests/test_smoke.py::test_fleet_card_aggregates_cards PASSED             [100%]

============================== 2 passed in 1.45s ===============================
```

- **Total: 2 / Pass: 2 / Fail: 0 / Error: 0 / Time: 1.45s**
- Python 3.14.5 (project `requires-python = ">=3.10"` per `pyproject.toml:10`)
- pytest 9.0.3 (project pins `pytest` only as a name in dev extras, no version)
- hypothesis plugin detected (unused by tests)
- asyncio plugin detected (unused)
- Coverage not run by default (`pytest -v` without `--cov`)

### 8.5 Fixtures / conftest

- `tests/conftest.py`: **does not exist**
- Fixtures: **0** (`audit_scorecard.json:317` `"fixtures": 0`)
- Mocks/patches: **0** (`audit_scorecard.json:316` `"mock": 0, "patch": 0`)

### 8.6 Test coverage of source

| Source module | Tested? | Notes |
|---|---|---|
| `__init__.py` (CostCard dataclass) | Indirectly (via tests) | Field-default behavior (`computed_at`, `contributors`) is not asserted |
| `collectors.py` (3 functions) | **NO** | None of `gh_actions_minutes`, `lfm_token_ledger`, `du_storage` are exercised by any test. SPEC.md:38 claims "smoke + per-collector + render" — the "per-collector" tests are **missing**. |
| `render.py` (3 functions) | Partially (2/3) | `render_repo_card` ✓, `render_fleet_card` ✓, `cost_trend_arrow` indirectly. Test does not cover the `previous_total_usd is None` branch. |
| `examples/fleet_card.py` | **NO** | Not on the test path; not importable. |

**Test coverage gap:** 3 collector functions + the example file are entirely untested. The example file would in fact **fail at import time** if imported (it defines functions referencing non-existent `CostCard` fields).

---

## 9. Code Metrics

### 9.1 LoC by category (full breakdown)

| Category | Files | LoC | Avg LoC/file |
|---|---:|---:|---:|
| `src/` Python | 3 | 127 | 42.3 |
| `tests/` Python | 1 | 41 | 41.0 |
| `examples/` Python | 1 | 69 | 69.0 |
| `docs/` (README, CHANGELOG, SPEC, AGENTS, llms, WORKLOG) | 6 | 243 | 40.5 |
| Governance prose (CODE_OF_CONDUCT, CONTRIBUTING, SECURITY, CODEOWNERS) | 4 | 111 | 27.8 |
| Governance machine (audit_scorecard.json) | 1 | 392 | 392.0 |
| License (LICENSE + LICENSE-MIT + LICENSE-APACHE) | 3 | 56 | 18.7 |
| CI / build (`.github/`, justfile, .pre-commit-config.yaml, deny.toml, .gitignore) | 11 | 335 | 30.5 |
| Manifest (pyproject.toml, requirements-dev.txt) | 2 | 26 | 13.0 |
| **Total** | **33** | **1,425** | **43.2** |

Source: `find . -type f -not -path './.git/*' -not -path './.pytest_cache/*' -not -path '*/__pycache__/*' -exec wc -l {} +`

### 9.2 Per-file Python LoC (src + tests + examples)

| File | LoC |
|---|---:|
| `src/pheno_cost_card/render.py` | 64 |
| `src/pheno_cost_card/collectors.py` | 46 |
| `src/pheno_cost_card/__init__.py` | 17 |
| `tests/test_smoke.py` | 41 |
| `examples/fleet_card.py` | 69 |
| **Total Python** | **237** |

### 9.3 Longest files (top 10, all categories)

| Rank | File | LoC |
|---:|---|---:|
| 1 | `audit_scorecard.json` | 392 |
| 2 | `.github/PULL_REQUEST_TEMPLATE.md` | 114 |
| 3 | `README.md` | 73 |
| 4 | `examples/fleet_card.py` | 69 |
| 5 | `src/pheno_cost_card/render.py` | 64 |
| 6 | `src/pheno_cost_card/collectors.py` | 46 |
| 7 | `SPEC.md` | 46 |
| 8 | `llms.txt` | 44 |
| 9 | `CONTRIBUTING.md` | 43 |
| 10 | `tests/test_smoke.py` | 41 |

### 9.4 Most-imported modules (within `src/`)

- `pheno_cost_card` (i.e., the local package itself) — imported by `src/pheno_cost_card/render.py:5`
- Stdlib `json` — `src/pheno_cost_card/collectors.py:3` (1 import)
- Stdlib `subprocess` — `src/pheno_cost_card/collectors.py:4` (1 import)
- Stdlib `pathlib.Path` — `src/pheno_cost_card/collectors.py:5` (1 import)
- Stdlib `dataclasses.dataclass`, `dataclasses.field` — `src/pheno_cost_card/__init__.py:3` (1 import)
- Stdlib `datetime.datetime`, `datetime.timezone` — `src/pheno_cost_card/__init__.py:4` (1 import)
- Stdlib `collections.abc.Iterable` — `src/pheno_cost_card/render.py:3` (1 import)

**No external (non-stdlib) imports anywhere in `src/`.** This matches `pyproject.toml:15` `dependencies = []`.

### 9.5 Public functions / classes / dataclasses

| Symbol | Defined at | Kind |
|---|---|---|
| `CostCard` | `src/pheno_cost_card/__init__.py:7-14` | frozen dataclass (1) |
| `gh_actions_minutes` | `src/pheno_cost_card/collectors.py:8-18` | public function |
| `lfm_token_ledger` | `src/pheno_cost_card/collectors.py:21-34` | public function |
| `du_storage` | `src/pheno_cost_card/collectors.py:37-46` | public function |
| `cost_trend_arrow` | `src/pheno_cost_card/render.py:8-15` | public function |
| `render_repo_card` | `src/pheno_cost_card/render.py:18-38` | public function |
| `render_fleet_card` | `src/pheno_cost_card/render.py:41-64` | public function |

**7 public symbols total** (1 dataclass + 6 functions). **0 custom exceptions, 0 enums, 0 type aliases, 0 Protocols, 0 TypedDicts.** Corroborated by `audit_scorecard.json:248-252` `"annotated_funcs": 6, "total_funcs": 6, "dataclasses": 0, "protocols": 0, "typeddicts": 0, "generics": 0` — note: the dataclass count is 0 because the audit script appears to only count non-frozen dataclasses (the scorecard at L75 claims 100% type coverage with 6/6 annotated funcs, but the `dataclasses: 0` line is at L249, missing `CostCard`).

### 9.6 Type annotation coverage

`audit_scorecard.json:74-77`: `L10 Type Safety: "Type coverage: 6/6 (100%). Dataclasses: 0."` — 6/6 functions fully annotated, but the dataclass `CostCard` is not counted. Per `audit_scorecard.json:247`: `annotated_funcs: 6, total_funcs: 6`. **All 6 functions are fully annotated**; `CostCard` is also fully annotated (every field has a type).

### 9.7 TODOs / FIXMEs / NotImplementedError / scaffold markers

`grep -rn "TODO\|FIXME\|XXX\|HACK\|NotImplementedError" src/ tests/ examples/` → **0 matches**. No TODO/FIXME/HACK comments, no `pass`-only functions, no `NotImplementedError` raises. This is a clean codebase by those markers.

### 9.8 Pass-only functions / scaffold-only code

- **0 `pass`-only functions** in `src/`
- **0 `raise NotImplementedError`** in `src/`
- The 3 collector functions in `collectors.py` are minimal but **complete** (not scaffold-only). The 3 render functions in `render.py` are **complete** and produce working markdown.
- The single `__init__.py` is **complete** (single dataclass, no stubs).
- The only **broken** code is `examples/fleet_card.py` (references non-existent fields) — this is a **stale draft that was committed without verification**, not scaffold-only code.

### 9.9 Code complexity

`audit_scorecard.json:71-72`: `L9 Complexity: "Long funcs: 0, nested blocks: 6, branches: 6."` — 0 long functions, 6 nested blocks, 6 branches. All functions fit on a single screen. `audit_scorecard.json:241-244` confirms: `"long_functions": 0, "nested_blocks": 6, "branches": 6`.

### 9.10 Subprocess / shell usage

`grep -rn "subprocess\|os.system\|shell=True" src/` → 2 matches, both in `src/pheno_cost_card/collectors.py`:

- `src/pheno_cost_card/collectors.py:4`: `import subprocess`
- `src/pheno_cost_card/collectors.py:39`: `result = subprocess.run(...)` (inside `du_storage`)

**No `shell=True`.** The single subprocess call is `["du", "-sk", str(repo)]` (L40), with `check=True, capture_output=True, text=True` (L41-43). **This is a security-positive pattern** (no shell injection surface). However, it is not testable without `du` on PATH or a mock, and there are no tests for `du_storage` (see §8.6).

### 9.11 Async / concurrency

`audit_scorecard.json:212-217`: `"async_def": 0, "await": 0, "asyncio_import": 0, "httpx_import": 0, "aiohttp_import": 0` — no async code. The project is fully synchronous.

### 9.12 Error handling

`audit_scorecard.json:83`: `L12 Error Handling: "Try blocks: 0, bare excepts: 0, custom exceptions: 0, retry: 0."` and `audit_scorecard.json:260-265`: `"try_blocks": 0, "bare_excepts": 0, "custom_exceptions": 0, "retry_decorators": 0`. **No try/except blocks anywhere in `src/`.** `subprocess.run(..., check=True)` will raise `subprocess.CalledProcessError` on non-zero exit (e.g., if `du` is missing), which is uncaught and will propagate. `json.loads(...)` will raise `json.JSONDecodeError` on malformed ledger, uncaught.

### 9.13 Logging

`audit_scorecard.json:88-89`: `L13 Logging: "Logger imports: 0, structured: 0."` and `audit_scorecard.json:266-268`: `"logger_imports": 0, "structured_logging": 0`. **No logging anywhere in `src/`.** Silent failures (collectors return 0.0 on missing ledger; `du_storage` raises on subprocess failure).

### 9.14 Test depth

`audit_scorecard.json:119-120`: `L21 Testing Depth: "Parametrize: 0, Fixtures: 0, Mock: 0, Patch: 0."` and `audit_scorecard.json:312-316`: same. Only smoke tests; no parameterization, no mocking, no edge-case coverage.

### 9.15 Versioning

- `pyproject.toml:7`: `version = "0.1.0"` (declared in metadata)
- 0 git tags (§3)
- `CHANGELOG.md:22`: `## [0.1.0] - 2026-06-11` (single released version)
- `pyproject.toml:7` version does **not** match the `0.1.0` 71-pillar score in `SPEC.md:36-37` (which is a different metric, not a version)

### 9.16 Inline summaries

- **Total `src/` Python LoC: 127** (in 3 files; 17+46+64)
- **Total Python LoC: 237** (127 src + 41 tests + 69 examples)
- **Test:src ratio: 41/127 = 0.32** (low; below the conventional 0.5 floor)
- **Examples:src ratio: 69/127 = 0.54**
- **Doc LoC: 243 (raw) + 56 (license) + 392 (audit_scorecard.json) = 691** governance/docs — **2.9x the Python src** (691/237 total Python)
- **Test pass rate: 100%** (2/2)
- **Public symbols: 7** (1 dataclass + 6 functions)
- **Imports: 100% stdlib** (no third-party deps)
- **Type annotation coverage: 100%** (every function fully annotated; every `CostCard` field typed)
- **External test surface (collectors): untested**

---

## 10. Historical Intent

### 10.1 What the repo was TRYING to do

Synthesizing the commit messages, branch names, `AGENTS.md`, `SPEC.md`, `README.md`, `llms.txt`, `WORKLOG.md`, and the V-numbers in commit messages (V4, V10, V11, V14, V20, T15.11):

> **Per-repo and fleet cost card: a fleet-wide cost observability substrate** that tracks monthly burn (CI minutes + LLM token spend + storage) per repo and aggregates into a fleet card. It is one component of a larger fleet cost-retro pipeline (referenced as *"V4 §64 Side Y"* in `AGENTS.md:38` and *"V10 Side Y from V4 §64"* in `WORKLOG.md:5`). The card is designed to be **reproducible in CI without API access** by reading checked-in `.cost-card/*.json` ledgers (per `AGENTS.md:21-22`).

The V-numbers point to a fleet-wide DAG:

- **V4 §64 "Side Y (Cost / Economics)"** — the side-channel of the FLEET_100TASK_DAG that tracks cost observability
- **V11 §77 "AI-DD crutch"** — *"crutch"* appears to be a fleet-meta-bundle template (also referenced in `WORKLOG.md:8` "crutch verification complete" V20-1.7)
- **V11 §70.3 "L16 AX acceptance"** — adoption of the `pheno-vibecoding-guard` pre-commit hook was the **L16 (Architecture, pillar 16) acceptance criterion** for V11
- **V14 §81** — initial scaffold origin
- **T15.10 / T15.11 "pheno-flake refresh"** — the `pheno-flake` template is a fleet meta-bundle convention (SPEC + LICENSE-APACHE + deny.toml + examples + __init__ + audit_scorecard.json + ci.yml + AGENTS.md + llms.txt + WORKLOG.md + CHANGELOG.md + requirements-dev.txt + .pre-commit-config.yaml)
- **ADR-015 / ADR-025** — worklog schema v2.0 → v2.1 (adds `device:` column)
- **ADR-023 Rule 3.1** — quality bar for new substrate: spec, docs, tests, observability, coverage gate, CI gate, worklog
- **ADR-024** — 71-pillar audit framework (L1-L71, 9 domains)
- **ADR-039** — pheno-flake template (referenced by SPEC.md:46)

### 10.2 What it was SUPPOSED to ship (per `SPEC.md`)

`SPEC.md:36-41` claims:

- 71-pillar score: 22/71 (Tier 0)
- Test matrix: 5 unit tests (smoke + per-collector + render)
- CI: pytest on push
- License: dual (MIT + Apache-2.0)

**Actual delivery** (per this audit):

- audit_scorecard.json reports overall: 56 / grade: C- in the **30-pillar** framework, not the 71-pillar framework
- **2** unit tests (smoke only — collectors are **untested**)
- CI: pytest on push (✓ present in `.github/workflows/ci.yml`)
- License: dual MIT + Apache-2.0 (✓ all three LICENSE files present, though `LICENSE-APACHE` is truncated to 15 lines)

### 10.3 What the architect's intent was (verbatim from `AGENTS.md:5-7` and `README.md:3`)

> *"Per-repo and fleet cost card. Tracks monthly CI minutes, LLM token spend in USD, and storage GB. Renders as 1-page markdown per repo + 1-page fleet card aggregating all repos."*
> — `AGENTS.md:5-7`

> *"Track per-repo monthly burn in CI minutes + LLM tokens + storage. Produces a cost card in markdown for each repo, rolled up to a fleet card."*
> — `README.md:3`

> *"Collectors are intentionally small and replaceable so the project can adapt to local ledgers, GitHub exports, or future billing APIs."*
> — `README.md:73`

The architect's design philosophy is explicit: **reproducibility over connectivity**. The card must be regenerable in CI without GitHub API access. The collectors are pure read-only functions over checked-in JSON. This is a fleet observability substrate designed for **dogfooding** (running the fleet's own tooling on itself).

### 10.4 Architectural intent for public API (verbatim from `AGENTS.md:26-29`)

> *"The `CostCard` dataclass fields — they're the wire format for `.cost-card/` JSON exports consumed by downstream dashboards."*
> *"The `render_*` function signatures — they're called from external scripts (e.g. weekly fleet reports)."*

**The dataclass IS the wire format.** The JSON-ledger format used by the collectors must match the `CostCard` field set. This makes the `examples/fleet_card.py` mismatch (which uses `month` / `egress_gb` / `llm_tokens` fields not in `CostCard`) a **wire-format inconsistency** — the example would produce JSON that no consumer (including the existing `render.render_repo_card`) could read.

### 10.5 Historical red flags (issues that emerged during this audit window)

1. **Broken example file** (`examples/fleet_card.py:18-69`) — references `month`, `egress_gb`, `llm_tokens` fields that don't exist on `CostCard`. The example would fail at runtime, and the docstring example (L5-10) would also fail. Committed in `885f419` (2026-06-18) without pytest exercising it (testpaths = `tests` only; `examples/` is not collected).
2. **CLI not implemented** — `README.md:30-33` and `SPEC.md:29-33` both reference a `pheno-cost-card` CLI; no `[project.scripts]` block in `pyproject.toml`. `audit_scorecard.json:46-47, 181-186` flags this as `CLI: MISSING`.
3. **License metadata drift** — `pyproject.toml:11` declares `license = { text = "MIT" }` only, but three LICENSE files exist (MIT + Apache-2.0). Should be `SPDX-License-Identifier: MIT AND Apache-2.0` or two classifiers.
4. **`SECURITY.md:18-24` references workflows that don't exist** (`cargo-deny.yml` Monday cron, `codeql-rust.yml` Tuesday weekly) — these are not in `.github/workflows/`. The only workflow is `ci.yml`.
5. **`CONTRIBUTING.md:39` references `docs/governance/background_agent_policy.md`** — file does not exist.
6. **`AGENTS.md:38-39` references a hardcoded local machine path** (`/Users/kooshapari/CodeProjects/Phenotype/repos/FLEET_100TASK_DAG_V4.md`) — will not resolve for any other contributor.
7. **`deny.toml`** is a Rust `cargo-deny` config in a Python project. `cargo-deny` cannot run against this codebase. The `deny.toml:1-2` comment explicitly cites *"Per ADR-023 Rule 3.1"* — but that rule applies to Rust substrates, not Python libraries.
8. **`justfile:16-21` `lint` and `typecheck` use `|| true`** — silently swallow ruff/mypy failures.
9. **`justfile:12-13, 28-29` `build` and `release` targets** reference `build` and `twine` packages not in dev deps (`pyproject.toml:18` only has `pytest`; `requirements-dev.txt:1` only has `pheno-vibecoding-guard>=0.1.0`).
10. **`WORKLOG.md` v2.1 schema migration incomplete** — `16bb08c` adds the 11th `device:` column per ADR-015/ADR-025, but **all 4 existing rows have empty `device:` cells** (`WORKLOG.md:5-8`). Per ADR-025, the v2.0 schema is deprecated **2026-06-22** — 4 days from this audit.
11. **Framework version mismatch in audit** — `SPEC.md:36-37` claims 71-pillar score 22/71, but `audit_scorecard.json` reports 30-pillar overall: 56 / grade: C-. The 71-pillar re-scoring has not been done for this repo.
12. **`SPEC.md:38` claims 5 unit tests** (smoke + per-collector + render); **actual: 2 unit tests** (smoke only — collectors and trend-arrow edge cases are untested).
13. **WIP commits not in main** — `27b12d7` (audit_scorecard.json, 393 lines) and `54c2085` (pyrightconfig.json + ci tweaks, 59 lines) are on `chore/adopt` and `wip/stash` respectively but **not on `main`**. The audit_scorecard.json is therefore not on the main branch HEAD.
14. **Remote-only branch divergence** — `origin/chore/phore-pheno-flake-refresh-pheno-cost-card-2026-06-18` (`7fbfb18`) is a T15.11 pheno-flake template refresh that supersedes the local `chore/adopt` (T15.10-equivalent) meta-bundle. **This work is on remote but not pulled into the local clone.**
15. **`examples/fleet_card.py` defines its own `render_fleet_card` and `render_repo_card`** with different signatures from the working `pheno_cost_card.render.*` versions — two same-named functions, both at the top of the import graph, with the example being broken and the implementation being correct. This is a namespace conflict waiting to happen if anyone tries to import the example as a module.

### 10.6 Summary: what the repo IS, what it's NOT

- **IS:** A minimal, well-typed, stdlib-only Python library with 1 dataclass + 6 pure functions. Renders markdown. Reads checked-in JSON ledgers. Has dual-license metadata. Has a single CI workflow. Has 2 passing smoke tests. Is part of a fleet-wide observability substrate (FLEET_100TASK_DAG §64 Side Y / V10 Side Y).
- **IS NOT:** Production-billing. CLI-enabled. API-connected. Async. Tested beyond smoke. Documented in a single source of truth (3 doc formats: `README.md`, `SPEC.md`, `llms.txt`, `AGENTS.md`, `WORKLOG.md`, `CHANGELOG.md` — all with overlapping but not identical claims). 71-pillar-scored (only 30-pillar scorecard exists). Tagged or released (0 tags). On a clean `main` (WIP commits live on `chore/adopt` and `wip/stash`).

The repo is at **v0.1.0-scaffold-with-meta-bundle** maturity. The **functional core works** (`CostCard` + 3 collectors + 3 render functions + 2 passing tests). The **meta-bundle is partially broken** (broken `examples/fleet_card.py`, no CLI, framework-mismatched audit score, incomplete v2.1 worklog backfill, `SPEC.md`/`audit_scorecard.json` score-card mismatch, `deny.toml` is decorative). The **operational substrate is incomplete** (no scheduled CI jobs despite `SECURITY.md` claims, no dependabot, no release workflow, no console script).
