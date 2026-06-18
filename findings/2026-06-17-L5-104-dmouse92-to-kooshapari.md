# L5-104 — Dmouse92 → KooshaPari Migration Audit (2026-06-17)

**Status:** IN PROGRESS
**Branch:** `chore/w5-adrs-sota-2026-06-15` (this repo)
**Auth:** `gh` is **KooshaPari** (active). Dmouse92 is read-only-collaborator (must NOT push).
**Repo matrix:** 26 Dmouse92 repos total; 20 Phenotype-related; 6 personal (skip).
**Strategy:** (a) merge Dmouse92 → KooshaPari; (b) reconcile/absorb to proper substrate per ADR-013/023; (c) archive emptied Dmouse92 repos.

---

## 1. Discovery — Dmouse92 repos

### 1.1 Phenotype-related (20) — IN SCOPE

| # | Repo | Dmouse92 HEAD | KP HEAD | KP state | Unmerged branches |
|---|---|---|---|---|---|
| 1 | **dispatch-mcp** | 2026-06-15T07:58Z (`a050e06`) | 2026-06-18 (pushed_at for chore branch) | active | `chore/w2-1-dispatch-mcp-2026-06-15` (6 unique commits, ~5000 LOC, NOT on KP main) |
| 2 | **pheno** | 2026-06-15 (`7a803dd`) | 2026-06-18 (pushed_at) | active | `chore/adr-012-config-consolidation-2026-06-15` (7 unique commits, 127 files; NOT on KP main; PRs #130/#131/#132 do NOT contain this work) |
| 3 | **AgilePlus** | 2026-06-16 | 2026-06-18 | active | `dependabot/*` only; 1 unique Dmouse92 commit (`2a8cb6d`) |
| 4 | **phenodocs** | 2026-02-26 | 2026-06-17 | active | `chore/stacked-prs-governance` (stale, 2026-02); main is 4 months behind |
| 5 | **forgecode** | 2026-06-15T05:35:50Z | 2026-06-18T01:39Z | active | 378 branches, 100% mirror of upstream `tailcallhq/forgecode` (NOT `aaronfagan/forgecode`); 364 byte-identical, 13 auto-generated, 1 already on KP (`feat/session-viewer`), 0 unique Phenotype work |
| 6 | **PhenoCompose** | empty (0 KB) | 2026-06-15 | active | 0 branches, 0 commits, 127 KP-ahead commits → archive DM92 |
| 7 | **PhenoPlugins** | empty | 2026-06-16 | active | 0 branches, 0 commits, 94 KP-ahead commits → archive DM92 |
| 8 | **PhenoProc** | empty | 2026-06-13 | **archived** | KP archived; Dmouse92 still active → archive DM92 with note |
| 9 | **HeliosCLI** | empty | 2026-06-16 | active | 0 branches, 0 commits, 292 KP-ahead commits → archive DM92 |
| 10 | **Pyron** | empty | 2026-06-18 | active | 0 branches, 0 commits, 56 KP-ahead commits → archive DM92 |
| 11 | **HexaKit** | empty | 2026-06-18 | active | 0 branches, 0 commits, 504 KP-ahead commits → archive DM92 |
| 12 | **Tracera** | empty | 2026-06-18 | active | 0 branches, 0 commits, 553 KP-ahead commits → archive DM92 |
| 13 | **Civis** | empty | 2026-06-18 | active | 0 branches, 0 commits, KP main thousands of commits (full fetch timed out; depth=1 used) → archive DM92 |
| 14 | **OmniRoute** | empty | 2026-06-17 | active | 0 branches, 0 commits, 209 KP-ahead commits → archive DM92 |
| 15 | **KWatch** | empty | 2026-06-17 | active | 0 branches, 0 commits, 60 KP-ahead commits → archive DM92 |
| 16 | **phenotype-ops** | `a24997f` (32 KB) | 2026-06-18 | active | bit-identical to KP (same SHA) → archive DM92 |
| 17 | **phenotype-otel** | empty | 2026-06-17 | active | 0 branches, 0 commits, 10 KP-ahead commits → archive DM92 |
| 18 | **Nanovms** | empty | 2026-06-16 | **archived** | KP archived; Dmouse92 still active → archive DM92 with note |
| 19 | **PhenoContracts** | empty | 2026-06-16 | active | 0 branches, 0 commits, 16 KP-ahead commits → archive DM92 |
| 20 | **phenotype-teamcomm** | `732e532` (120 KB) | 2026-06-18 | active | bit-identical to KP (same SHA) → archive DM92 |

### 1.2 Personal — OUT OF SCOPE

`mesh-llm`, `localbase-shard`, `Quant`, `TripleM`, `acheron_Symbols`, `qmk_firmware` — pre-Phenotype work, not part of org, skip.

### 1.3 Already deleted/archived

- `cheap-llm-mcp` — deleted on BOTH accounts (per ADR-006/007 verification, archive complete).

---

## 2. Per-repo migration analysis

(See sections 2.1–2.20 below as subagents report.)

### 2.1 dispatch-mcp (HIGH PRIORITY — user explicit)

**Status:** ANALYSIS COMPLETE — see `findings/2026-06-17-L5-104-dispatch-mcp-migration-plan.md` (527 lines, 10-step execution sequence)

**Critical correction vs initial parent doc:** KP dispatch-mcp main HEAD is `a050e06` (2026-06-15T07:58:15Z), NOT 2026-06-18. The 2026-06-18 `pushed_at` was for the `chore/w2-1-dispatch-mcp-2026-06-15` branch HEAD, not main.

**Dmouse92 unique commits (6) in `chore/w2-1-dispatch-mcp-2026-06-15` (verified NOT on KP main):**

| SHA | Subject | LOC | KP branch (unmerged) |
|---|---|---|---|
| `dc4f1a3` | docs(dispatch-mcp): deprecate cheap-llm-mcp (W1.1) | +22 | `chore/w1-1-cheap-llm-mcp-deprecation-note-2026-06-15` (clean cherry-pick) |
| `9486edb` | test: add protocol compliance mock backend harness (W2.1) | +108 | bundled in `feat/openai-compat-2026-06-15`, `wip/migrate-from-dmouse-w2-1-2026-06-17`, `feat/openai-compat-provider-2026-06-15` |
| `f46e356` | test: add mock backend harness for protocol compliance (W2.1) | +106 | bundled in same branches; likely duplicate of #2 |
| `6aad7fa` | feat(core): cost tracking, budget, quota, audit trail | +6,641 / -47 | bundled in `feat/openai-compat-2026-06-15` @ `977cd43` |
| `874a023` | W2.1 (empty marker commit) | 0 | every W2-1 branch |
| `a1aaef2` | feat(W2-1): dispatch-mcp protocol compliance + provider guides | +866 / -6 | identical tree in `feat/openai-compat-2026-06-15`, `wip/migrate-from-dmouse-w2-1-2026-06-17` |

**Substrate state (CRITICAL):** `KooshaPari/pheno-mcp-router` does NOT exist on GitHub (404). Substrate is local-only at `repos/pheno-mcp-router/`, no `origin` remote, 8 commits, uncommitted working tree (3 modified + 2 untracked). **Step 1 of migration plan = publish substrate first.**

**Per-file migration action (from sub-agent plan):**
- 6 modules → substrate (`tiers.py`, `cost.py`, `budget.py`, `quota.py`, `audit.py`, `cost_middleware.py`) — ~2,000 LOC + ~2,400 LOC tests
- 2 adapters → substrate (`LlamaAdapter` from `llama_cpp.py` + `OpenAICompatAdapter` from KP-authored `openai_compat.py`)
- 1 doc → substrate (`PROVIDER_GUIDE.md`)
- 2 Docker files → `phenotype-ops/agent-devops-setups/llama-cpp/`
- 5 files stay in dispatch-mcp (`core/{port,protocol,types}.py`, `adapters/omni_http.py` ext, `server.py` ext, `test_mock_backend.py`)
- 1 cherry-pick already done (merge `dc4f1a3`): `CHEAP_LLM_MCP_DEPRECATION.md`
- 1 duplicate to discard: `9486edb` (replaced by `f46e356`)
- 1 file to discard: `providers/base.py` (dispatch-mcp Provider protocol diverges from substrate LlmPort shape)

**Substrate per ADR-013:** `pheno-mcp-router` (canonical MCP substrate).

**Action plan (Phase 1, 10 steps):** See `findings/2026-06-17-L5-104-dispatch-mcp-migration-plan.md:233-360`. Net impact: +2,400 LOC to substrate, +87 LOC to phenotype-ops, −2,000 LOC from dispatch-mcp.

### 2.2 pheno (HIGH PRIORITY — ADR-012 work)

**Status:** ANALYSIS COMPLETE — see `findings/2026-06-17-L5-104-pheno-adr012-migration-plan.md` (414 lines)

**Dmouse92 unique commits (7) in `chore/adr-012-config-consolidation-2026-06-15` (verified NOT on KP main):**

| SHA | Subject |
|---|---|
| `7a803dd` | chore(pheno): remove phenotype-config-core (moved to phenotype-config-loader) |
| `af0d5d5` | chore(workflows+docs+go): pin workflow actions to SHAs, add SLSA + governance docs, CANONICAL markers |
| `9bf8816` | chore(workflows): pin remaining tag-based action references to SHAs |
| `f83e362` | chore(workflows): standardize to ci/audit/deny/scorecard/release |
| `f6398a6` | chore: add phenotype-migrations to workspace and implement in-tree migration framework |
| `fa19377` | chore: create working Cargo.toml workspace for AgilePlus Rust CLI |
| `e71a4fd` | chore(governance): add missing governance files |

**W5 PRs #130/#131/#132 verification:** All 3 are pre-existing **action-SHA pinning PRs** (merged 2026-04-30 / 2026-05-01), 2-7 weeks BEFORE the Dmouse92 ADR-012 work window of 2026-06-12 → 2026-06-15. **None contain the ADR-012 config consolidation content.** Initial parent-doc claim that "PR #130 (W5 ADR-012 config consolidation PR-1/2/3)" is incorrect — the "W5 ADR-012" label was mis-attributed.

**KP/pheno state:** HEAD = `a109d9c` (2026-06-13T01:10:33Z), dormant on ADR-012 files for 4+ days. 0 of 19 sampled Dmouse92 ADR-012 file paths has any KP/main commit since 2026-06-15.

**4 files do NOT exist on KP/main at all** (Dmouse92-unique additions):
- `docs/slsa.md`
- `.github/workflows/release-attestation.yml`
- `crates/phenotype-config-loader/CANONICAL.md`
- `crates/phenotype-shared-config/CANONICAL.md`

**3 files already pinned on KP** (via PR #132): `.github/workflows/{ci,audit,scorecard}.yml`

**1 critical stale marker:** `crates/phenotype-config-core/CANONICAL.md` on KP points to `phenoShared`; needs re-point to `phenotype-config` substrate per ADR-022 RFC 002.

**Substrate per ADR-022:** `phenotype-config` (canonical Rust core, 2-crate split: `phenotype-config` + `Conft`).

**Action plan (per subagent):** Cherry-pick 2 of 7 commits (the CANONICAL.md markers + phenotype-config-core deletion); port `docs/slsa.md` to `phenotype-config/docs/slsa.md`; re-point existing `CANONICAL.md` markers from `phenoShared` to `phenotype-config`; **discard** the 5 workflow/agileplus commits (already obsolete or divergent on KP); archive `Dmouse92/pheno`.

### 2.3 phenodocs

**Status:** DRAFTED (no subagent needed)

- Dmouse92 main is 4 months stale (2026-02-26)
- KooshaPari has 10+ recent commits (SLSA, attestations, SLOs, coverage ratchet, decision records, AI-DD metadata, etc.)
- `chore/stacked-prs-governance` branch is also stale (2026-02-26)
- **Verdict:** Fully absorbed on KooshaPari. **Action: archive Dmouse92 phenodocs.**

### 2.4 AgilePlus

**Status:** DRAFTED

- Dmouse92 main has 2 unique commits:
  - `f868d18 fix(audit): apply W4 SOTA findings across 12 crates` — duplicate of KP `ad01a98` (same subject, divergent SHA)
  - `2a8cb6d feat(domain+hook): add FeatureState::is_shippable() + chrono dev-dep` — possibly unique
- KooshaPari is significantly ahead (W4 governance rollout, shared-core traceability spine, consolidate all branches)
- **Action:** Cherry-pick `2a8cb6d` if not present on KooshaPari (verify with `gh api repos/KooshaPari/AgilePlus/commits?path=crates/agileplus/domain&per_page=5`), then archive Dmouse92 AgilePlus.

### 2.5 forgecode

**Status:** ANALYSIS COMPLETE — see `findings/2026-06-17-L5-104-forgecode-migration.md` (305 lines, 0 unique Phenotype work)

**Key finding:** `Dmouse92/forgecode` is a **stale 1:1 mirror of upstream `tailcallhq/forgecode`** (NOT `aaronfagan/forgecode`). Both DM92 and KP are forks of `tailcallhq/forgecode`. DM92 has 378 branches (100% mirror); KP has 22 branches (21 KP-only Phenotype work).

**Smoking gun:** DM92 created_at = 2026-06-15T05:35:37Z, pushed_at = 2026-06-15T05:35:50Z — 13-second delta, single fork-bulk-push event.

**Branch matrix:**

| Category | Count | Action |
|---|---|---|
| **A** Same name + same SHA on KP | 1 (`feat/session-viewer` @ `7065903a…`) | Already absorbed |
| **B** Same name, divergent SHA | 1 (`main` — DM92 10 behind upstream, KP 39 ahead + 79 behind) | Resolved (KP is the active fork) |
| **C** Only on DM92 — mirror of upstream | 364 | No migration; fetch from upstream if needed |
| **C** Only on DM92 — auto-generated (Dependabot, Renovate, PR bots) | 13 | Ignore |
| **C** Only on DM92 — unique Phenotype work | **0** | **None** |

**Verdict:** 0 of 378 Dmouse92/forgecode branches need migration to KooshaPari/forgecode. **Action: archive `Dmouse92/forgecode` entirely.**

### 2.6 PhenoCompose

**Status:** ANALYSIS COMPLETE (part of bulk subagent — see `findings/2026-06-17-L5-104-bulk-rust-ts-migration.md:62-69`)

- DM92 empty (0 KB, 0 branches, 0 commits)
- KP ahead by 127 commits on main; tip `ec6ea98 feat(hex): add SecretStore port + in-memory/file adapters + DI container`
- **Verdict:** Cat A. **Action: archive `Dmouse92/PhenoCompose`.**

### 2.7 PhenoPlugins

**Status:** ANALYSIS COMPLETE

- DM92 empty; KP ahead by 94 commits; tip `b4c3e2e docs(history): archive phenoVessel specs from absorbed repo`
- **Verdict:** Cat A. **Action: archive `Dmouse92/PhenoPlugins`.**

### 2.8 PhenoProc

**Status:** ANALYSIS COMPLETE

- DM92 empty; KP **archived** (tip `2437faf chore: bootstrap trufflehog.yml (#55)` before archive)
- **Verdict:** Cat D. **Action: archive `Dmouse92/PhenoProc` with note.**

### 2.9 HeliosCLI

**Status:** ANALYSIS COMPLETE

- DM92 empty; KP ahead by 292 commits; tip `472a737 docs(HeliosCLI): add threat model`. KP default branch is `chore/threat-model-2026-06-16`.
- **Verdict:** Cat A. **Action: archive `Dmouse92/HeliosCLI`.**

### 2.10 Pyron

**Status:** ANALYSIS COMPLETE

- DM92 empty; KP ahead by 56 commits; tip `28703cf Wave 13 lockstep: exclude test crates; git pin test-infra to TestingKit (#56)`. Note `1f37903 feat(config): repoint settly to phenotype-config (RFC 002)` is the canonical substrate move per ADR-022.
- **Verdict:** Cat A. **Action: archive `Dmouse92/Pyron`.**

### 2.11 HexaKit

**Status:** ANALYSIS COMPLETE

- DM92 empty; KP ahead by 504 commits.
- **Verdict:** Cat A. **Action: archive `Dmouse92/HexaKit`.**

### 2.12 Tracera

**Status:** ANALYSIS COMPLETE

- DM92 empty; KP ahead by 553 commits (initial diff snapshot showed "1 commit ahead" — fetch-depth artifact, re-verified).
- **Verdict:** Cat A. **Action: archive `Dmouse92/Tracera`.**

### 2.13 Civis

**Status:** ANALYSIS COMPLETE

- DM92 empty; KP ahead (full fetch timed out at 90s; `--depth=1` used; KP/Civis main is in the thousands of commits; the 2000-file / 834K-line merge commit `dd02c5ec` is the only commit our shallow fetch got).
- **Verdict:** Cat A. **Action: archive `Dmouse92/Civis`.**

### 2.14 OmniRoute

**Status:** ANALYSIS COMPLETE

- DM92 empty; KP ahead by 209 commits.
- **Verdict:** Cat A. **Action: archive `Dmouse92/OmniRoute`.**

### 2.15 KWatch

**Status:** ANALYSIS COMPLETE

- DM92 empty; KP ahead by 60 commits.
- **Verdict:** Cat A. **Action: archive `Dmouse92/KWatch`.**

### 2.16 phenotype-ops

**Status:** ANALYSIS COMPLETE

- DM92 HEAD SHA `a24997f` (32 KB), bit-identical to KP HEAD (`a24997f`).
- **Verdict:** Cat E. **Action: archive `Dmouse92/phenotype-ops` (no migration, identical).**

### 2.17 phenotype-otel

**Status:** ANALYSIS COMPLETE

- DM92 empty; KP ahead by 10 commits.
- **Verdict:** Cat A. **Action: archive `Dmouse92/phenotype-otel`.**

### 2.18 Nanovms

**Status:** ANALYSIS COMPLETE

- DM92 empty; KP **archived**.
- **Verdict:** Cat D. **Action: archive `Dmouse92/Nanovms` with note.**

### 2.19 PhenoContracts

**Status:** ANALYSIS COMPLETE

- DM92 empty; KP ahead by 16 commits; KP default branch is `chore/dependabot-2026-06-08`.
- **Verdict:** Cat A. **Action: archive `Dmouse92/PhenoContracts`.**

### 2.20 phenotype-teamcomm

**Status:** ANALYSIS COMPLETE

- DM92 HEAD SHA `732e532` (120 KB), bit-identical to KP HEAD (`732e532`).
- **Verdict:** Cat E. **Action: archive `Dmouse92/phenotype-teamcomm` (no migration, identical).**

---

## 3. Execution log

### Phase 1 — Analysis (2026-06-17, complete)

| Time | Step | Result |
|---|---|---|
| 19:00 | `gh auth status` → KooshaPari active | OK |
| 19:05 | `gh repo list Dmouse92` → 26 repos | OK |
| 19:08 | `gh repo list KooshaPari` → 138 repos | OK |
| 19:12 | Cross-reference matrix — 20 Phenotype-related DM92 repos | OK |
| 19:18 | Subagent A: dispatch-mcp migration plan (527 lines) | OK |
| 19:18 | Subagent B: pheno ADR-012 migration plan (414 lines) | OK |
| 19:18 | Subagent C: 14-repo bulk migration plan (999 lines) | OK |
| 19:18 | Subagent D: forgecode migration analysis (305 lines) | OK |
| 19:25 | Update parent audit doc with corrections | OK |

### Phase 2 — Execution (2026-06-17, in progress)

Pending execution:
1. Publish `pheno-mcp-router` substrate to KooshaPari GitHub (Step 1 of dispatch-mcp plan)
2. Port 6 Dmouse92 modules to substrate
3. Cherry-pick 2 ADR-012 commits to phenotype-config substrate + Conft
4. Re-point `phenotype-config-core/CANONICAL.md` from `phenoShared` to `phenotype-config`
5. Verify AgilePlus `2a8cb6d` cherry-pick feasibility
6. Archive 16 Dmouse92 repos (14 bulk + forgecode + dispatch-mcp + pheno) — requires Dmouse92 auth
7. Update KooshaPari dispatch-mcp + pheno with "MIGRATED-TO-SUBSTRATE" README marker
8. Update STATUS.md, AGENTS.md, SSOT.md
9. Rebase + push cleaned branch

---

## 4. Decision matrix (FINAL)

| # | Repo | Cat | Action | Target | Owner |
|---|---|---|---|---|---|
| 1 | dispatch-mcp | B (unique W2-1) | Port 6 modules + 2 adapters + 1 doc + 2 docker to substrate | `pheno-mcp-router` + `phenotype-ops` | subagent E |
| 2 | pheno | B (unique ADR-012) | Cherry-pick 2 commits + port `docs/slsa.md` + re-point markers | `phenotype-config` + `Conft` | subagent F |
| 3 | AgilePlus | B (1 unique) | Verify `2a8cb6d` cherry-pick | `AgilePlus` | self |
| 4 | phenodocs | A (stale) | Archive DM92 | — | self |
| 5 | forgecode | C (mirror) | Archive DM92 | — | self |
| 6 | PhenoCompose | A | Archive DM92 | — | self |
| 7 | PhenoPlugins | A | Archive DM92 | — | self |
| 8 | PhenoProc | D (KP archived) | Archive DM92 with note | — | self |
| 9 | HeliosCLI | A | Archive DM92 | — | self |
| 10 | Pyron | A | Archive DM92 | — | self |
| 11 | HexaKit | A | Archive DM92 | — | self |
| 12 | Tracera | A | Archive DM92 | — | self |
| 13 | Civis | A | Archive DM92 | — | self |
| 14 | OmniRoute | A | Archive DM92 | — | self |
| 15 | KWatch | A | Archive DM92 | — | self |
| 16 | phenotype-ops | E (identical) | Archive DM92 | — | self |
| 17 | phenotype-otel | A | Archive DM92 | — | self |
| 18 | Nanovms | D (KP archived) | Archive DM92 with note | — | self |
| 19 | PhenoContracts | A | Archive DM92 | — | self |
| 20 | phenotype-teamcomm | E (identical) | Archive DM92 | — | self |

**Totals:** 2 substantive migrations (dispatch-mcp → substrate, pheno → substrate), 18 archives, 1 verification (AgilePlus).

---

## 5. Stale / warnings

- **Dmouse92 read-only collaborator** — `gh` is currently KooshaPari. Archive commands against Dmouse92 repos require Dmouse92 auth. Recommend `GH_TOKEN=$(gh auth token --user Dmouse92) gh repo archive ...` per-command to avoid global auth switch (safer than `gh auth switch`).
- **Archive ≠ delete** — initial action is archive (read-only marker). Delete only after 90-day archive retention (GitHub policy).
- **pheno-mcp-router substrate publication required first** — currently local-only; `gh repo create KooshaPari/pheno-mcp-router --public --source=repos/pheno-mcp-router --push` needed before module porting.
- **Bulk archive requires care** — 18 repos to archive in a single batch; one mistaken command archives a KooshaPari repo. Mitigation: explicit `--repo Dmouse92/<name>` flag on every command.
- **macOS case-insensitive filesystem** — Dmouse92 pheno clone shows case-collision warning; analysis must use `git diff --name-only` instead of `git diff` for full-tree diffs.
- **Civis full fetch timed out** — KP/Civis main is too large to clone within 90s (`fetch-pack: unexpected disconnect`). Used `--depth=1` for verdict; full clone deferred to next session if needed.
- **ADR-022 substrate readiness** — `phenotype-config` Rust core and `Conft` TS edge both exist on KooshaPari; substrate is mature. CANONICAL.md marker re-pointing is a small but important governance fix.

---

## 6. Related artifacts

- `findings/2026-06-17-L5-104-dispatch-mcp-migration-plan.md` (527 lines) — sub-agent A
- `findings/2026-06-17-L5-104-pheno-adr012-migration-plan.md` (414 lines) — sub-agent B
- `findings/2026-06-17-L5-104-bulk-rust-ts-migration.md` (999 lines) — sub-agent C
- `findings/2026-06-17-L5-104-forgecode-migration.md` (305 lines) — sub-agent D
- `docs/adr/2026-06-15/ADR-013-pheno-mcp-router-substrate.md` — substrate for dispatch-mcp
- `docs/adr/2026-06-15/ADR-022-config-consolidation-two-crate-split.md` — substrate for pheno ADR-012
- `docs/adr/2026-06-15/ADR-023-agent-effort-governance.md` — device-fit gate for execution
- `docs/adr/2026-06-15/ADR-006-cheap-llm-mcp-deprecation.md` — already-archived example
- `docs/adr/2026-06-15/ADR-007-cheap-llm-mcp-deprecation-followup.md` — already-archived example

---

**End of L5-104 parent audit (consolidated 2026-06-17 19:35 PDT).**