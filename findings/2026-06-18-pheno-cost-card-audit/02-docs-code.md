# Docs/Spec/Intent + Code Features - pheno-cost-card

**Repo:** `KooshaPari/pheno-cost-card`
**Local path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/pheno-cost-card/`
**Audit date:** 2026-06-18
**Mode:** Read-only. No files modified.
**Test environment:** Python 3.14.5, pytest 9.0.3, macOS arm64
**Test result:** **2 passed / 0 failed** in ~2.9s

---

## Part A: Docs / Spec / Intent

Each file in the repo is enumerated. For every claim, a row in the
"Evidence" sub-table cites `file:line`, quotes the exact claim, notes what
it promises the user, and classifies implementation status.

Status legend (cross-referenced with Part B):

- **implemented_and_tested** — feature is in the source tree and exercised
  by `pytest`.
- **implemented_untested** — feature is in the source tree but not
  exercised by any test.
- **docs_only** — claim exists in docs but no corresponding code.
- **scaffold_placeholder** — code exists but the body is a TODO/`pass`/
  `NotImplementedError`/stub return.
- **broken** — code exists and is wrong / will crash / references a
  non-existent symbol.
- **branch_only** — claim lives on a remote branch that is not in the
  default working set (i.e., cannot be observed in the local clone).
- **n/a** — pure boilerplate, not a feature claim.

---

### A.1 `README.md` (73 lines, 1,624 bytes, last-modified 2026-06-11)

#### A.1.1 Evidence

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| R-1 | `README.md:1` | `# pheno-cost-card` | Repo name | n/a |
| R-2 | `README.md:3` | `Track per-repo monthly burn in CI minutes + LLM tokens + storage. Produces a cost card in markdown for each repo, rolled up to a fleet card.` | Tracks 3 metrics, emits per-repo + fleet markdown | implemented_and_tested for `ci_minutes` + `llm_tokens_usd` + `storage_gb`; **does NOT** track raw LLM tokens (only USD) — see BUG-R1 |
| R-3 | `README.md:7` | `Each repo gets a one-page markdown report, and all repo cards can be aggregated into a one-page fleet card.` | 1-page repo card + 1-page fleet card | implemented_and_tested |
| R-4 | `README.md:11-14` | `- GitHub Actions CI minutes / - LLM token spend in USD / - Repository storage in GB / - Contributors active in the reporting window` | 4 tracked inputs | CI minutes: implemented_untested; USD spend: implemented_untested; Storage GB: implemented_untested; Contributors: implemented_and_tested (via `card.contributors`) |
| R-5 | `README.md:19` | `pip install pheno-cost-card` | Distributable on PyPI | docs_only (no `pyproject.toml` `[project.urls]`; no tag; no `dist/`; verify-only) |
| R-6 | `README.md:25` | `pip install -e .` | Editable install works locally | implemented (deps are zero, `pip install -e .[dev]` is what `just dev` runs; verified by `just dev` recipe at `justfile:4-5`) |
| R-7 | `README.md:31` | `pheno-cost-card repo /path/to/repo --month 2026-06` | CLI subcommand `repo` | **docs_only — broken**: no CLI exists, see BUG-2 |
| R-8 | `README.md:32` | `pheno-cost-card fleet /Users/kooshapari/CodeProjects/Phenotype/repos --month 2026-06` | CLI subcommand `fleet` | **docs_only — broken**: no CLI exists, see BUG-2 |
| R-9 | `README.md:39-51` | Markdown table example showing `# Cost Card: example-repo` with CI minutes / LLM token spend / Storage / Contributors + `Trend: up` + `Computed: <ISO>` | Output format | implemented_and_tested (matches `render_repo_card` output exactly — see Part B.2) |
| R-10 | `README.md:55-65` | Fleet card table with `Repositories / CI minutes / LLM token spend / Storage / Contributors` | Fleet output format | implemented_and_tested (matches `render_fleet_card`) |
| R-11 | `README.md:69-71` | `Collectors: gh_actions_minutes: collects CI minutes for a repository and month. / lfm_token_ledger: reads local LLM token ledger spend. / du_storage: measures repository storage using disk usage.` | 3 named collectors | all 3 implemented_untested (no per-collector test exists) |
| R-12 | `README.md:73` | `Collectors are intentionally small and replaceable so the project can adapt to local ledgers, GitHub exports, or future billing APIs.` | Plug-and-play design | docs_only design intent; no plugin protocol/`Protocol`/registry is declared — see Part F |

#### A.1.2 Traceability — README → code

| README claim | Resolves to | File:line | Verdict |
|---|---|---|---|
| R-3 (per-repo + fleet card) | `render_repo_card` + `render_fleet_card` | `src/pheno_cost_card/render.py:18,41` | OK |
| R-7/R-8 (CLI `repo`/`fleet`) | — | — | MISSING |
| R-11 (3 collectors) | `gh_actions_minutes`, `lfm_token_ledger`, `du_storage` | `src/pheno_cost_card/collectors.py:8,21,37` | OK (functions exist; names match) |
| R-9 (per-repo markdown shape) | `render_repo_card` output | `src/pheno_cost_card/render.py:23-38` | OK (matches byte-for-byte when defaults used) |

---

### A.2 `SPEC.md` (47 lines, 1,566 bytes, last-modified 2026-06-18)

#### A.2.1 Evidence

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| S-1 | `SPEC.md:3-7` | `Compute and render fleet cost cards: per-repo monthly cost (CI minutes, storage, egress, LLM tokens) and a fleet-wide aggregate.` | 4 metrics: CI minutes, storage, egress, LLM tokens | **PARTIAL** — `ci_minutes` + `storage_gb` are present; `egress_gb` and `llm_tokens` are NOT fields of `CostCard`; `llm_tokens_usd` is the actual field. See BUG-4. |
| S-2 | `SPEC.md:8` | `The output is a Markdown card suitable for pasting into the weekly health inventory or a status dashboard.` | Output is Markdown | implemented_and_tested |
| S-3 | `SPEC.md:12-14` | `pheno_cost_card.CostCard — frozen dataclass with fields: repo, ci_minutes, storage_gb, egress_gb, llm_tokens, month (str, ISO YYYY-MM).` | Exact field list | **PARTIAL** — actual fields are `repo, ci_minutes, llm_tokens_usd, storage_gb, computed_at, contributors`. `egress_gb`, `llm_tokens`, `month` are missing. `llm_tokens_usd`/`computed_at`/`contributors` are present but undocumented in SPEC. See BUG-4. |
| S-4 | `SPEC.md:15-16` | `pheno_cost_card.collectors.gh_actions_minutes(repo: Path, month: str) -> float — query the GitHub Actions API for monthly minutes used.` | A collector that **queries the GitHub Actions API** | **DOCS MISLEADING** — the implementation does NOT query the API. It reads a checked-in JSON ledger at `.cost-card/gh-actions-minutes.json`. See BUG-S1. |
| S-5 | `SPEC.md:17-18` | `pheno_cost_card.collectors.local_storage_gb(repo: Path) -> float — sum .git size + artifacts on disk.` | A collector named `local_storage_gb` summing `.git` + artifacts | **MISSING** — actual collector is named `du_storage` and uses `du -sk` on the whole repo. No `.git`/artifacts separation. See BUG-4. |
| S-6 | `SPEC.md:19-20` | `pheno_cost_card.render.render_repo_card(card: CostCard) -> str — Markdown table for one repo.` | Function name + signature | implemented_and_tested — actual signature is `render_repo_card(card, previous_total_usd=None)` (extra kwarg) |
| S-7 | `SPEC.md:21-22` | `pheno_cost_card.render.render_fleet_card(cards: list[CostCard]) -> str — Markdown table for the fleet aggregate.` | Function name + signature | implemented_and_tested — actual signature is `render_fleet_card(cards, previous_total_usd=None)` (extra kwarg; type widened to `Iterable[CostCard]`) |
| S-8 | `SPEC.md:28-33` | `5-line quickstart: from pheno_cost_card import CostCard, render_fleet_card / cards = [CostCard(repo="pheno-config", ci_minutes=120.0, ...)] / print(render_fleet_card(cards))` | 5-line quickstart runs as-is | **BROKEN** — `from pheno_cost_card import CostCard, render_fleet_card` raises `ImportError: cannot import name 'render_fleet_card' from 'pheno_cost_card'`. The function lives in `pheno_cost_card.render`. See BUG-3. |
| S-9 | `SPEC.md:37` | `71-pillar score: 22/71 (Tier 0)` | Self-graded 22/71 | docs_only — scorecard in repo (`audit_scorecard.json:3`) reports overall 56 (30-pillar scheme) — different rubric, different denominator. See Part F. |
| S-10 | `SPEC.md:38` | `Test matrix: 5 unit tests (smoke + per-collector + render)` | 5 unit tests covering smoke + per-collector + render | **FALSE** — actual count is 2 tests, both render-smoke; no per-collector tests exist. See BUG-5. |
| S-11 | `SPEC.md:39` | `CI: pytest on push` | pytest runs on push | implemented (`.github/workflows/ci.yml:1-24` runs `pytest -v` on push and PR to `main` and across 8 OS×Python matrix combos) |
| S-12 | `SPEC.md:40` | `License: dual (MIT + Apache-2.0)` | Dual-license | implemented (`LICENSE-MIT` and `LICENSE-APACHE` both present; `pyproject.toml:11` declares only `MIT` — see BUG-6) |
| S-13 | `SPEC.md:43-45` | `See also: L6 fleet health inventory / ADR-024 (71-pillar framework) / ADR-039 (pheno-flake template)` | Cross-references exist | docs_only — neither `L6 fleet health inventory` nor `ADR-039` is referenced anywhere else in the repo |

#### A.2.2 Traceability — SPEC → code

| SPEC claim | Resolves to | File:line | Verdict |
|---|---|---|---|
| S-3 (CostCard fields) | `CostCard` dataclass | `src/pheno_cost_card/__init__.py:7-14` | MISMATCH (3 fields invented, 3 fields missing) |
| S-4 (gh_actions_minutes via API) | `gh_actions_minutes` body | `src/pheno_cost_card/collectors.py:8-18` | SEMANTIC MISMATCH (reads file, doesn't query API) |
| S-5 (local_storage_gb) | — | — | NAME MISMATCH + MISSING (actual: `du_storage` at `collectors.py:37`) |
| S-6 (render_repo_card) | `render.render_repo_card` | `src/pheno_cost_card/render.py:18` | OK (extra kwarg `previous_total_usd` not in SPEC) |
| S-7 (render_fleet_card) | `render.render_fleet_card` | `src/pheno_cost_card/render.py:41` | OK (extra kwarg `previous_total_usd`; type widened to `Iterable`) |
| S-8 (5-line quickstart) | `render.render_fleet_card` | `src/pheno_cost_card/render.py:41` | BROKEN (top-level import fails) |
| S-10 (5 unit tests) | `tests/test_smoke.py` | `tests/test_smoke.py:7,27` | FALSE (2 tests, no per-collector coverage) |

---

### A.3 `AGENTS.md` (40 lines, 1,469 bytes, last-modified 2026-06-17)

#### A.3.1 Evidence

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| AG-1 | `AGENTS.md:3-7` | `Per-repo and fleet cost card. Tracks monthly CI minutes, LLM token spend in USD, and storage GB. Renders as 1-page markdown per repo + 1-page fleet card aggregating all repos.` | Repo purpose | implemented_and_tested |
| AG-2 | `AGENTS.md:11-14` | `just dev # pip install -e ".[dev]" / just test # pytest -v` | Two task recipes exist | implemented — `justfile:4-5` (`dev`) and `justfile:8-9` (`test`) match verbatim |
| AG-3 | `AGENTS.md:18-19` | `Standard Python src/ layout with hatchling` | Build backend is hatchling | implemented (`pyproject.toml:2-3` declares `hatchling`) |
| AG-4 | `AGENTS.md:19-20` | `CostCard is a frozen dataclass (immutable)` | `CostCard` is `@dataclass(frozen=True)` | implemented (`src/pheno_cost_card/__init__.py:7`) — verified: assigning to `card.repo` raises `FrozenInstanceError` |
| AG-5 | `AGENTS.md:20-21` | `render.render_repo_card and render.render_fleet_card are pure functions` | Render funcs are pure | implemented — both take args, return `str`, no I/O, no globals mutated |
| AG-6 | `AGENTS.md:21-22` | `collectors.* functions read checked-in .cost-card/*.json ledgers (so the card is reproducible in CI without API access)` | Reproducible, no API access | implemented — `gh_actions_minutes` reads `repo/.cost-card/gh-actions-minutes.json` (`collectors.py:14`); `lfm_token_ledger` reads `repo/.cost-card/lfm-token-ledger.json` (`collectors.py:27`) |
| AG-7 | `AGENTS.md:25-29` | `The CostCard dataclass fields — they're the wire format for .cost-card/ JSON exports consumed by downstream dashboards. / The render_* function signatures — they're called from external scripts (e.g. weekly fleet reports).` | Do not break the public surface | policy / docs_only |
| AG-8 | `AGENTS.md:30-32` | `Lockfiles and submodule pins: **/Cargo.lock, **/package-lock.json, **/yarn.lock, **/pnpm-lock.yaml, **/poetry.lock, **/.gitmodules (enforced by pheno-vibecoding-guard pre-commit hook).` | Pre-commit enforces no-touch list | implemented — `.pre-commit-config.yaml:4-17` declares `pheno-vibecoding-guard` hook; the upstream `scan` CLI is known broken in v0.1.0 (see `.pre-commit-config.yaml:6-9` comment) so the hook invokes the Python API directly. This is **self-acknowledged fragile**. |
| AG-9 | `AGENTS.md:33-34` | `Common secrets: **/*.pem, **/*.key, **/*.p12, **/secrets/**, **/.env, **/.envrc (enforced by pheno-vibecoding-guard).` | Secret scanning | implemented (same hook, same fragility) |
| AG-10 | `AGENTS.md:37-39` | `See /Users/kooshapari/CodeProjects/Phenotype/repos/FLEET_100TASK_DAG_V4.md §64 Side Y (Cost / Economics) and §78.6 (V13 grand-total). / See pheno-worklog-schema for the related fleet-wide tracking schema.` | Cross-references | docs_only — neither `FLEET_100TASK_DAG_V4.md` §64/§78.6 nor `pheno-worklog-schema` is linked or importable from this repo |

#### A.3.2 Traceability — AGENTS → code

| AGENTS claim | Resolves to | File:line | Verdict |
|---|---|---|---|
| AG-4 (frozen) | `@dataclass(frozen=True)` | `src/pheno_cost_card/__init__.py:7` | OK |
| AG-5 (pure render) | `render_repo_card`/`render_fleet_card` | `src/pheno_cost_card/render.py:18,41` | OK |
| AG-6 (`.cost-card/*.json` ledger) | `gh_actions_minutes` / `lfm_token_ledger` | `src/pheno_cost_card/collectors.py:14,27` | OK |

---

### A.4 `CHANGELOG.md` (32 lines, 936 bytes, last-modified 2026-06-14)

#### A.4.1 Evidence

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| CL-1 | `CHANGELOG.md:1-2` | `# Changelog — pheno-cost-card / All notable changes to this project will be documented in this file.` | Keep a Changelog convention | implemented |
| CL-2 | `CHANGELOG.md:5` | `The format is based on Keep a Changelog (https://keepachangelog.com/en/1.1.0/)` | Format is v1.1.0 | implemented (matches the standard layout) |
| CL-3 | `CHANGELOG.md:6` | `this project adheres to Semantic Versioning (https://semver.org/spec/v2.0.0.html)` | SemVer | implemented — `pyproject.toml:7` declares `0.1.0` |
| CL-4 | `CHANGELOG.md:8-21` | `## [Unreleased] / ### Added / ### Changed / ### Deprecated / ### Removed / ### Fixed / ### Security` (all empty) | Empty placeholder for next release | scaffold_placeholder — kept honest, all subsections empty |
| CL-5 | `CHANGELOG.md:22` | `## [0.1.0] - 2026-06-11` | 0.1.0 release | implemented |
| CL-6 | `CHANGELOG.md:26` | `Initial scaffold from V11 prep agent design (V4 §64 Side Y).` | Provenance | docs_only — references internal V11 design; same "V4 §64 Side Y" reference as `AGENTS.md:38` |
| CL-7 | `CHANGELOG.md:27` | `CostCard frozen dataclass: repo, ci_minutes, llm_tokens_usd, storage_gb, computed_at, contributors` | Field list at 0.1.0 | **OK** — this matches the actual `CostCard` fields at `__init__.py:9-14`. The CHANGELOG is the **only doc that lists the real fields correctly** |
| CL-8 | `CHANGELOG.md:28` | `render.render_repo_card(card, previous_total_usd) and render.render_fleet_card(cards, previous_total_usd)` | Render signatures with trend arg | implemented (matches `render.py:18,41` signatures) |
| CL-9 | `CHANGELOG.md:29` | `3 collectors: gh_actions_minutes, lfm_token_ledger, du_storage` | 3 collector names | implemented (`collectors.py:8,21,37`) |
| CL-10 | `CHANGELOG.md:30` | `2 smoke tests (repo card + fleet card)` | 2 tests | **OK** — matches actual 2 tests at `tests/test_smoke.py:7,27` |
| CL-11 | `CHANGELOG.md:31` | `pyproject.toml, justfile, ci.yml, AGENTS.md, llms.txt, .gitignore` | Files shipped at 0.1.0 | **PARTIAL** — at 0.1.0 the tree had: `pyproject.toml`, `justfile`, `.github/workflows/ci.yml`, `AGENTS.md`, `llms.txt`, `.gitignore`. The CHANGELOG omits `CODEOWNERS`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `SECURITY.md`, `LICENSE`, `LICENSE-MIT`, `requirements-dev.txt` (added later per git log: `a9ddff6`, `cbfd3b1`, `632693a`). Those landed in `[Unreleased]` but were never moved into `[0.2.0]` or equivalent. |

#### A.4.2 Traceability — CHANGELOG → code

| CHANGELOG claim | Resolves to | File:line | Verdict |
|---|---|---|---|
| CL-7 (CostCard fields) | `CostCard` | `src/pheno_cost_card/__init__.py:7-14` | OK |
| CL-8 (render signatures) | `render_repo_card` / `render_fleet_card` | `src/pheno_cost_card/render.py:18,41` | OK |
| CL-9 (3 collectors) | `gh_actions_minutes` / `lfm_token_ledger` / `du_storage` | `src/pheno_cost_card/collectors.py:8,21,37` | OK |
| CL-10 (2 smoke tests) | `tests/test_smoke.py` | `tests/test_smoke.py:7,27` | OK |

---

### A.5 `WORKLOG.md` (8 lines, 803 bytes, last-modified 2026-06-17)

#### A.5.1 Evidence

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| WL-1 | `WORKLOG.md:3` | `\|task_id\|date\|repo\|category\|title\|commit_sha\|pr_number\|status\|author\|device\|notes\|` | 11-column v2.1 schema (ADR-015 v2.1) | implemented (the header has the `device:` column) |
| WL-2 | `WORKLOG.md:5` | `V11-CC-Y3 \| 2026-06-11 \| pheno-cost-card \| L16-AX \| initial scaffold (per-repo + fleet cost card) \| (this commit) \| (none) \| merged \| koosha-ai \|  \| V10 Side Y from V4 §64` | Initial scaffold entry; `device:` blank | broken — see BUG-7 (the v2.1 schema requires the `device:` field per ADR-025; this cell is empty) |
| WL-3 | `WORKLOG.md:6` | `V14-AA-3 \| 2026-06-11 \| pheno-cost-card \| L14-UX \| smoke tests added \| (this commit) \| (none) \| merged \| koosha-ai \|  \| 2/2 tests pass` | 2 tests passing entry | implemented (matches current 2/2 pass) |
| WL-4 | `WORKLOG.md:7` | `V14-AA-4 \| 2026-06-11 \| pheno-cost-card \| L15-DX \| justfile + ci.yml + AGENTS.md + llms.txt \| (this commit) \| (none) \| merged \| koosha-ai \|  \| standard convention` | Standard DX files added | implemented (all 4 files present) |
| WL-5 | `WORKLOG.md:8` | `V20-1.7 \| 2026-06-12 \| pheno-cost-card \| V20 \| crutch verification complete \| (this commit) \| (none) \| merged \| koosha-ai \|  \| crutch verification complete` | Crutch verification entry | docs_only — no definition of "crutch verification" exists; this is a meaningless placeholder string repeated in `title` and `notes` |

#### A.5.2 Traceability — WORKLOG → code

| WORKLOG claim | Resolves to | Verdict |
|---|---|---|
| WL-2 / WL-3 / WL-4 / WL-5 (4 entries) | git log shows commits: `630efb8` initial scaffold, `cbfd3b1` governance docs, `3541fc6` issue templates, `764bafe` PR template, `a9ddff6` CODEOWNERS, `50c1ac0` CHANGELOG, `632693a` pre-commit, `27b12d7` wip snapshot, `16bb08c` worklog v2.1, `885f419` meta-bundle | The 4 worklog rows correspond to a subset of the 11 commits; many later commits (issue templates, PR template, CODEOWNERS, CHANGELOG, pre-commit, meta-bundle) are NOT in WORKLOG.md |

---

### A.6 `llms.txt` (44 lines, 938 bytes, last-modified 2026-06-11)

#### A.6.1 Evidence

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| LLM-1 | `llms.txt:3` | `Per-repo and fleet cost card for CI minutes, LLM token spend, and storage.` | 3-metric scope | implemented_and_tested |
| LLM-2 | `llms.txt:8` | `pip install pheno-cost-card` | Distributable | docs_only (no PyPI release) |
| LLM-3 | `llms.txt:13-27` | `from datetime import datetime, timezone / from pheno_cost_card import CostCard / from pheno_cost_card.render import render_repo_card, render_fleet_card / card = CostCard(repo="my-repo", ci_minutes=120.0, llm_tokens_usd=4.25, storage_gb=1.5, contributors=("alice", "bob")) / print(render_repo_card(card, previous_total_usd=3.0))` | Code sample works | implemented — verified by manual run (output matches expected Markdown) |
| LLM-4 | `llms.txt:31-37` | `from pheno_cost_card.collectors import gh_actions_minutes, lfm_token_ledger, du_storage / gh_actions_minutes(Path("/path/to/repo"), "2026-06") / lfm_token_ledger(Path("/path/to/repo"), "2026-06") / du_storage(Path("/path/to/repo"))` | Collector imports work | implemented — all 3 importable; function signatures match |
| LLM-5 | `llms.txt:40-41` | `Collectors read checked-in .cost-card/*.json ledgers so the card is reproducible in CI without API access.` | Reproducible, no API | implemented (`collectors.py:14,27`) |
| LLM-6 | `llms.txt:43` | `## License / MIT` | License = MIT | implemented (LICENSE-MIT exists) — but `LICENSE-APACHE` is also present (dual), so this section is incomplete |

#### A.6.2 Traceability — llms → code

| llms claim | Resolves to | Verdict |
|---|---|---|
| LLM-3 (Code sample) | `CostCard(...)` / `render_repo_card` | OK (verified) |
| LLM-4 (3 collector imports) | `pheno_cost_card/collectors.py` | OK (all 3 exist with matching signatures) |
| LLM-6 (MIT only) | `LICENSE-MIT` + `LICENSE-APACHE` | INCOMPLETE (dual-license reality, not MIT-only) |

---

### A.7 `SECURITY.md` (24 lines, 669 bytes, last-modified 2026-06-14)

#### A.7.1 Evidence

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| SE-1 | `SECURITY.md:1-9` | `# Security Policy / ## Reporting Vulnerabilities / Please report security vulnerabilities via GitHub Security Advisories / Open a private security advisory / For sensitive issues, contact the repository owner directly` | Reporting flow | policy / docs_only |
| SE-2 | `SECURITY.md:10-12` | `## Supported Versions / Latest main branch. Older versions are not supported.` | Only main is supported | policy |
| SE-3 | `SECURITY.md:14-16` | `We follow coordinated disclosure with reporters. Once an issue is patched, an advisory will be published.` | Coordinated disclosure | policy |
| SE-4 | `SECURITY.md:18-20` | `## Cargo-deny / Rust projects in this org enforce a zero-advisory floor via cargo-deny.yml workflow (Monday cron + on-demand).` | A `cargo-deny.yml` workflow exists in this org | docs_only for **this repo** — pheno-cost-card is a Python project; no `cargo-deny.yml` workflow exists here; the `deny.toml` config at the repo root is a **misapplied** cargo-deny config (see BUG-8). The claim refers to a fleet-wide workflow, not this repo. |
| SE-5 | `SECURITY.md:22-24` | `## CodeQL / Static analysis runs Tuesday weekly via codeql-rust.yml workflow.` | A `codeql-rust.yml` workflow exists in this org | docs_only for **this repo** — no CodeQL workflow exists in `.github/workflows/`. Pheno-cost-card is Python, so the workflow's language binding is also wrong for this project |

#### A.7.2 Traceability — SECURITY → code

| SECURITY claim | Resolves to | Verdict |
|---|---|---|
| SE-4 (cargo-deny workflow) | `deny.toml` + no `cargo-deny.yml` workflow | INCONSISTENT (file present, workflow absent; language mismatch) |
| SE-5 (CodeQL workflow) | `.github/workflows/` | MISSING (no CodeQL file in this repo's workflow directory) |

---

### A.8 `CONTRIBUTING.md` (43 lines, 2,215 bytes, last-modified 2026-06-14)

#### A.8.1 Evidence

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| CO-1 | `CONTRIBUTING.md:5-13` | Standard Fork / Branch / Test / Commit / Push / PR steps | Standard contribution flow | policy |
| CO-2 | `CONTRIBUTING.md:17-24` | Conventional commits prefix list (`feat:`, `fix:`, etc.) | Conventional Commits | policy |
| CO-3 | `CONTRIBUTING.md:28-31` | `All submissions require review. / CI checks pass / Code is documented / Tests cover new functionality` | Review + CI gate | policy |
| CO-4 | `CONTRIBUTING.md:34-42` | `Project-wide rules live under docs/governance/. The canonical background-agent policy that this repository and sibling repos (such as thegent and thegent-clean) point at is: docs/governance/background_agent_policy.md` | Background-agent policy file exists in-repo | **BROKEN** — `docs/governance/background_agent_policy.md` does not exist in this repo (verified by `find`); the policy is not available as a clickable link. See BUG-CO1 |

#### A.8.2 Traceability — CONTRIBUTING → code

| CONTRIBUTING claim | Resolves to | Verdict |
|---|---|---|
| CO-4 (governance doc) | `docs/governance/background_agent_policy.md` | MISSING (file does not exist) |

---

### A.9 `CODE_OF_CONDUCT.md` (39 lines, last-modified 2026-06-14)

Contributor Covenant v2.1 — boilerplate. No feature claims. **n/a** for
traceability.

---

### A.10 `LICENSE` (21 lines) / `LICENSE-MIT` (21 lines) / `LICENSE-APACHE` (15 lines)

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| LI-1 | `LICENSE:1-2` | `MIT License / Copyright (c) 2026 Koosha Pari` | MIT under Koosha Pari | implemented |
| LI-2 | `LICENSE-MIT:1-2` | `MIT License / Copyright (c) 2026 Phenotype` | MIT under Phenotype | implemented |
| LI-3 | `LICENSE-APACHE:1-15` | `Apache License Version 2.0, January 2004` | Apache-2.0 | implemented |
| LI-4 | `pyproject.toml:11` | `license = { text = "MIT" }` | PyPI metadata says MIT only | **CONTRADICTS** dual-license reality — only `MIT` is declared in metadata despite `LICENSE-APACHE` being present. See BUG-6. |

---

### A.11 `pyproject.toml` (25 lines, 540 bytes, last-modified 2026-06-17)

#### A.11.1 Evidence

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| PP-1 | `pyproject.toml:2-3` | `requires = ["hatchling"] / build-backend = "hatchling.build"` | hatchling build | implemented |
| PP-2 | `pyproject.toml:6-7` | `name = "pheno-cost-card" / version = "0.1.0"` | Package name + version | implemented |
| PP-3 | `pyproject.toml:8` | `description = "Per-repo and fleet cost card for CI minutes, LLM token spend, and storage."` | 3-metric scope | implemented (matches `llms.txt:3` and `README.md:3`) |
| PP-4 | `pyproject.toml:9` | `readme = "README.md"` | README is the long description | implemented |
| PP-5 | `pyproject.toml:10` | `requires-python = ">=3.10"` | Supports Python 3.10+ | implemented — CI matrix at `ci.yml:15` tests 3.10, 3.11, 3.12, 3.13 (no 3.14; the dev env runs 3.14 successfully) |
| PP-6 | `pyproject.toml:11` | `license = { text = "MIT" }` | License = MIT | implemented but **incomplete** — see BUG-6 |
| PP-7 | `pyproject.toml:12-14` | `authors = [{ name = "Phenotype" }]` | Author = Phenotype | implemented |
| PP-8 | `pyproject.toml:15` | `dependencies = []` | Zero runtime deps | implemented (no `import` outside stdlib: `json`, `subprocess`, `pathlib`, `dataclasses`, `datetime`, `collections.abc`) |
| PP-9 | `pyproject.toml:17-18` | `dev = ["pytest"]` | Dev dep = pytest | implemented |
| PP-10 | `pyproject.toml:20-21` | `[tool.hatch.build.targets.wheel] / packages = ["src/pheno_cost_card"]` | Build targets the src layout | implemented (verified by `pip install -e .[dev]` succeeding) |
| PP-11 | `pyproject.toml:23-24` | `[tool.pytest.ini_options] / testpaths = ["tests"] / pythonpath = ["src"]` | pytest picks up tests from `tests/` with `src` on path | implemented (verified by `pytest` collecting `tests/test_smoke.py`) |
| PP-12 | (MISSING) | No `[project.scripts]` block | — | broken — see BUG-2 (no CLI entry points despite README documenting CLI usage) |
| PP-13 | (MISSING) | No `[project.urls]` block | — | scaffold_placeholder — no homepage / repository / documentation / changelog URLs |

#### A.11.2 Traceability — pyproject → code

| pyproject claim | Resolves to | Verdict |
|---|---|---|
| PP-2 (version 0.1.0) | `CHANGELOG.md:22` | OK |
| PP-8 (zero runtime deps) | `src/pheno_cost_card/*.py` | OK (all stdlib imports) |
| PP-10 (hatchling src layout) | `src/pheno_cost_card/__init__.py` | OK |
| PP-11 (pytest testpaths + pythonpath) | `tests/test_smoke.py` | OK |
| (missing) CLI scripts | — | MISSING (README documents CLI; no entry point) |

---

### A.12 `justfile` (33 lines, 599 bytes, last-modified 2026-06-17)

#### A.12.1 Evidence

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| JF-1 | `justfile:1` | `default := just --list` | `just` with no args lists recipes | implemented (justfile standard convention) |
| JF-2 | `justfile:4-5` | `dev: /     pip install -e ".[dev]"` | `just dev` installs editable | implemented — matches `AGENTS.md:11` |
| JF-3 | `justfile:8-9` | `test: /     pytest -v` | `just test` runs tests verbose | implemented — matches `AGENTS.md:12` |
| JF-4 | `justfile:12-13` | `build: /     python -m build` | `just build` runs `python -m build` | broken — `build` is **not in dev deps** (`pyproject.toml:18` only lists `pytest`); running `just build` will fail with `No module named build`. See BUG-JF1. |
| JF-5 | `justfile:16-17` | `lint: /     ruff check src tests \|\| true` | `just lint` runs ruff (with `\|\| true` to never fail) | scaffold_placeholder — `ruff` is not in dev deps; the `\|\| true` defeats the purpose of linting; no `[tool.ruff]` config in `pyproject.toml`. See BUG-JF2. |
| JF-6 | `justfile:20-21` | `typecheck: /     mypy src \|\| true` | `just typecheck` runs mypy | scaffold_placeholder — `mypy` not in dev deps; `\|\| true` swallows errors; no `[tool.mypy]` config. See BUG-JF2. |
| JF-7 | `justfile:24-25` | `clean: /     rm -rf build dist src/*.egg-info .pytest_cache .ruff_cache .mypy_cache` | `just clean` removes build artifacts | implemented — paths match the `.gitignore` excludes |
| JF-8 | `justfile:28-29` | `release: clean build /     twine check dist/*` | `just release` builds + twine-checks (no upload) | broken — chained on `build` (which fails per JF-4); `twine` not in dev deps. See BUG-JF1. |
| JF-9 | `justfile:32-33` | `audit: /     pip list --outdated` | `just audit` lists outdated deps | implemented (will work since there are zero runtime deps) |

#### A.12.2 Traceability — justfile → pyproject

| justfile recipe | Resolves to | Verdict |
|---|---|---|
| JF-2 (dev) | `pyproject.toml:18` `dev = ["pytest"]` | OK |
| JF-3 (test) | `pyproject.toml:23-24` | OK |
| JF-4 (build) | `pyproject.toml:18` | MISSING (`build` not declared) |
| JF-5 (lint) | `pyproject.toml:18` | MISSING (`ruff` not declared; no config) |
| JF-6 (typecheck) | `pyproject.toml:18` | MISSING (`mypy` not declared; no config) |
| JF-8 (release) | JF-4 + twine | BROKEN (chain fails on JF-4; `twine` not declared) |

---

### A.13 `deny.toml` (37 lines, 703 bytes, last-modified 2026-06-18)

#### A.13.1 Evidence

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| DN-1 | `deny.toml:1-2` | `# cargo-deny config for pheno-cost-card / # Per ADR-023 Rule 3.1: every substrate ships with cargo-deny config.` | cargo-deny config | **MISAPPLIED** — `pheno-cost-card` is a Python project; no `Cargo.toml` exists in the repo (verified by `find`). `cargo-deny` requires a `Cargo.lock`, neither of which is present. The config cannot run. See BUG-8. |
| DN-2 | `deny.toml:4-5` | `[graph] / all-features = false` | Default graph settings | n/a (never runs) |
| DN-3 | `deny.toml:7-11` | `[advisories] / db-path = "~/.cargo/advisory-db" / db-urls = ["https://github.com/rustsec/advisory-db"] / yanked = "warn" / ignore = []` | Advisory DB config | n/a (never runs) |
| DN-4 | `deny.toml:13-26` | `[licenses] / allow = ["MIT", "Apache-2.0", ...]` | Allow-list of 9 licenses | n/a (never runs) — note: the allow-list matches the dual-license intent of the repo |
| DN-5 | `deny.toml:28-31` | `[bans] / multiple-versions = "warn" / wildcards = "deny" / deny = []` | No wildcard deps, no duplicate crate versions | n/a (never runs) |
| DN-6 | `deny.toml:33-36` | `[sources] / unknown-registry = "deny" / unknown-git = "warn" / allow-registry = ["https://github.com/rust-lang/crates.io"] / allow-git = []` | Source allow-list | n/a (never runs) |

#### A.13.2 Traceability — deny.toml → repo state

| deny.toml claim | Resolves to | Verdict |
|---|---|---|
| DN-1 (cargo-deny) | `Cargo.toml` in repo | MISSING (no `Cargo.toml`; this file is decorative/misapplied) |

---

### A.14 `.pre-commit-config.yaml` (24 lines, 861 bytes, last-modified 2026-06-17)

#### A.14.1 Evidence

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| PC-1 | `.pre-commit-config.yaml:1-17` | `repos: / - repo: local / hooks: / - id: pheno-vibecoding-guard / entry: python -c "import sys; from pheno_vibecoding_guard import scan_repo; r = scan_repo('.'); sys.exit(0 if r.clean else 1)"` | pheno-vibecoding-guard hook | **partially broken** — the hook runs the Python API directly because the upstream `scan` CLI is broken in v0.1.0 (per the comment at `.pre-commit-config.yaml:6-9`). If `pheno-vibecoding-guard` is not installed (`requirements-dev.txt:1` lists it as `>=0.1.0`, so it is a dev dep), the hook will fail with `ModuleNotFoundError`. `requirements-dev.txt` is NOT picked up by `pip install -e .[dev]`, so the guard is a **manual install** requirement, not a transitive one. See BUG-PC1. |
| PC-2 | `.pre-commit-config.yaml:18-23` | `pre-commit-hooks v5.0.0 / check-yaml / check-toml / end-of-file-fixer / trailing-whitespace` | Standard pre-commit hooks | implemented — all 4 hooks are well-known, run against staged files |

#### A.14.2 Traceability — pre-commit → dev deps

| pre-commit claim | Resolves to | Verdict |
|---|---|---|
| PC-1 (pheno-vibecoding-guard) | `requirements-dev.txt:1` | OK (declared), but `requirements-dev.txt` is not auto-installed by `pip install -e .[dev]` |

---

### A.15 `requirements-dev.txt` (1 line, 30 bytes)

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| RD-1 | `requirements-dev.txt:1` | `pheno-vibecoding-guard>=0.1.0` | A dev-only dep | **ORPHAN** — this file is not referenced by `pyproject.toml:18` (`dev = ["pytest"]`). Installing via `pip install -e ".[dev]"` does NOT install `pheno-vibecoding-guard`. The pre-commit hook (PC-1) assumes it is installed. See BUG-RD1. |

---

### A.16 `.gitignore` (21 lines, 194 bytes)

Standard Python ignores plus `.cost-card/`. The `.cost-card/` entry is
the conventional location for the collector JSON ledgers per
`AGENTS.md:21-22` and `collectors.py:14,27`.

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| GI-1 | `.gitignore:21` | `.cost-card/` | Local ledger dir is gitignored | implemented — matches AGENTS.md intent |

---

### A.17 `.github/workflows/ci.yml` (24 lines)

#### A.17.1 Evidence

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| GH-1 | `ci.yml:1` | `name: ci` | Workflow name | implemented |
| GH-2 | `ci.yml:3-6` | `on: / push: / branches: [main] / pull_request:` | Triggers on push to main and on any PR | implemented — matches `SPEC.md:39` ("CI: pytest on push") |
| GH-3 | `ci.yml:8-15` | `jobs: / test: / runs-on: ${{ matrix.os }} / strategy: / fail-fast: false / matrix: / os: [ubuntu-latest, macos-latest] / python-version: ["3.10", "3.11", "3.12", "3.13"]` | 8-cell matrix: 2 OS × 4 Python | implemented — but **missing Python 3.14** (which is what runs in the dev env where the audit was performed); also `windows-latest` is missing (inconsistent with fleet norms) |
| GH-4 | `ci.yml:17-20` | `- uses: actions/checkout@v4 / - uses: actions/setup-python@v5 / with: / python-version: ${{ matrix.python-version }}` | Pinned checkout + setup-python | implemented (current pinned versions) |
| GH-5 | `ci.yml:21-22` | `- name: Install / run: pip install -e ".[dev]"` | Editable install | implemented |
| GH-6 | `ci.yml:23-24` | `- name: Test / run: pytest -v` | Run pytest verbose | implemented |

#### A.17.2 Traceability — CI → code

| CI claim | Resolves to | Verdict |
|---|---|---|
| GH-3 (Python 3.10-3.13) | `pyproject.toml:10` `>=3.10` | OK (3.10 floor respected; 3.14 floor-cap not declared) |
| GH-5 (pip install -e ".[dev]") | `pyproject.toml:18` | OK |
| GH-6 (pytest -v) | `pyproject.toml:23-24` | OK |

---

### A.18 `.github/CODEOWNERS` (5 lines)

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| CO-1 | `.github/CODEOWNERS:5` | `* @kooshapari` | @kooshapari is default reviewer for all files | implemented (GitHub-recognized syntax) |

---

### A.19 `.github/PULL_REQUEST_TEMPLATE.md` (114 lines)

Boilerplate PR template with sections: What, Why, How, Type of Change,
Testing, Checklist, Risk & Rollout, Affected Surfaces, Related, Reviewer
Notes. All standard. **n/a** for traceability.

---

### A.20 `.github/ISSUE_TEMPLATE/{bug_report,feature_request,question,security_report}.md` + `config.yml`

Four GitHub issue forms (bug, feature, question, security) plus a
`config.yml` that disables blank issues and links to the KooshaPari
GitHub org. All standard. **n/a** for traceability.

---

### A.21 `audit_scorecard.json` (392 lines, 9,020 bytes, last-modified 2026-06-17)

#### A.21.1 Evidence — top-level

| # | File:line | Claim (verbatim) | Promises | Status |
|---|---|---|---|---|
| AS-1 | `audit_scorecard.json:2-4` | `"repo": "pheno-cost-card", "overall": 56, "grade": "C-"` | Overall score 56 / grade C- | docs_only — single-snapshot score |
| AS-2 | `audit_scorecard.json:6-35` | 30 L-numbers (L1-L30) with per-pillar scores | L1-L30 30-pillar rubric (legacy) | **MIXED** — the repo also has the 71-pillar framework (ADR-024) per `SPEC.md:37` and `AGENTS.md:38`; this scorecard is the older 30-pillar scheme. See Part F. |
| AS-3 | `audit_scorecard.json:38-158` | Per-pillar `details` strings and empty `raw: {}` | Score rationale per pillar | docs_only — the `raw: {}` objects are empty (no raw measurements), so the score is a heuristic not a measurement |
| AS-4 | `audit_scorecard.json:161-164` | `"source": { "total": 3, "over_500": 0, "over_350": 0, "oversized_files": [] }` | 3 source files, none oversized | **OK** — matches the 3 files in `src/pheno_cost_card/` |
| AS-5 | `audit_scorecard.json:166-179` | `"tests": { "total": 1, "unit": 1, "integration": 0, "e2e": 0, ... }` `collection: { "collected": 2, "errors": 0, ... }` | 1 test file, 2 collected tests, 0 errors | **OK** — matches `tests/test_smoke.py` (1 file, 2 tests) |
| AS-6 | `audit_scorecard.json:181-186` | `"cli": { "exists": false, "cmd": null, "has_subcommands": false, "help_length": 0 }` | No CLI exists | **OK** — confirms BUG-2 (no CLI) |
| AS-7 | `audit_scorecard.json:188-198` | `"docs": { "has_docs_dir": false, "files": { "README": true, "ARCHITECTURE": false, "SSOT": false, "CLAUDE": false, "AGENTS": true, "CONTRIBUTING": true, "CHANGELOG": true, "LICENSE": true } }` | Docs checklist | **OK** — `README`, `AGENTS`, `CONTRIBUTING`, `CHANGELOG`, `LICENSE` all present; no `ARCHITECTURE.md`, `SSOT.md`, or `CLAUDE.md` (consistent with the small repo) |
| AS-8 | `audit_scorecard.json:200-205` | `"security": { "hardcoded_api_key": 0, "hardcoded_secret": 0, "hardcoded_password": 0, "hardcoded_token": 0 }` | No secrets in source | **OK** — manually verified, no `*.pem`/`*.key`/tokens in `src/` |
| AS-9 | `audit_scorecard.json:212-218` | `"async": { "async_def": 0, "await": 0, "asyncio_import": 0, "httpx_import": 0, "aiohttp_import": 0 }` | No async code | **OK** — verified by inspection |
| AS-10 | `audit_scorecard.json:219-228` | `"pyproject": { "exists": true, "has_black": false, "has_ruff": false, "has_mypy": false, "has_pytest": true, "has_uv": false, "has_hatch": true, "has_poetry": false }` | pyproject has pytest + hatch | **OK** — matches `pyproject.toml:18,2` |
| AS-11 | `audit_scorecard.json:229-233` | `"git": { "has_git": true, "recent_commits": 8, "has_merge_commits": false }` | 8 recent commits, no merge commits | OK at the time of scorecard generation (2026-06-17); current HEAD is at 11 reachable commits |
| AS-12 | `audit_scorecard.json:234-240` | `"ci": { "has_github_actions": true, "workflow_files": ["ci.yml"], "has_precommit": true }` | 1 workflow + pre-commit | **OK** — matches `.github/workflows/ci.yml` + `.pre-commit-config.yaml` |
| AS-13 | `audit_scorecard.json:246-253` | `"type_safety": { "annotated_funcs": 6, "total_funcs": 6, "dataclasses": 0, "protocols": 0, "typeddicts": 0, "generics": 0 }` | 6/6 funcs annotated; no dataclasses | **FALSE** — there is a `@dataclass(frozen=True)` at `__init__.py:7`, so `dataclasses` should be ≥ 1. The scorecard under-counts dataclasses. See BUG-AS1. |
| AS-14 | `audit_scorecard.json:254-259` | `"dependencies": { "has_lock": false, "has_requirements": false, "has_constraints": false, "dep_count": 0 }` | No lock / requirements / constraints | **PARTIAL** — `requirements-dev.txt:1` exists (`pheno-vibecoding-guard>=0.1.0`); scorecard says `has_requirements: false` — this is a false negative. |
| AS-15 | `audit_scorecard.json:306-311` | `"config": { "env_refs": 0, "dotenv_refs": 0, "pydantic_settings": 0, "config_files": 4 }` | 4 config files | **OK** — `pyproject.toml`, `justfile`, `.pre-commit-config.yaml`, `deny.toml` = 4 |
| AS-16 | `audit_scorecard.json:312-316` | `"testing_depth": { "parametrize": 0, "fixtures": 0, "mock": 0, "patch": 0 }` | No test fixtures, no parametrize, no mocks | **OK** — verified by reading `tests/test_smoke.py` |
| AS-17 | `audit_scorecard.json:323-328` | `"release": { "has_version_file": false, "tag_count": 0, "semver_tags": 0, "has_changelog": true }` | No tags, changelog present | **OK** — `pyproject.toml:7` has the version inline (no `_version.py`); `CHANGELOG.md:22` exists |
| AS-18 | `audit_scorecard.json:366-371` | `"onboarding": { "makefile": 0, "devcontainer": 0, "setup_scripts": 0, "readme_setup": 1 }` | README has setup steps | **OK** — `README.md:18-26` has install + dev-install blocks |

#### A.21.2 L-pillar score summary

| L# | Domain | Score | Notes |
|---|---|---:|---|
| L1 | Architecture | 100 | 3 src files, none oversized |
| L2 | Dev Loop | 90 | 1 test file, 2 collected, 0 errors |
| L3 | Agent Loop | 40 | CLI missing; CI present |
| L4 | Observability | 55 | 5/8 canonical docs |
| L5 | Security | 100 | 0 secret patterns |
| L6 | Performance | 25 | No async, no awaits |
| L7 | Extensibility | 60 | 3 src files, 3 features in config |
| L8 | Compliance | 30 | 8 commits, SSOT: false |
| L9 | Complexity | 100 | 0 long funcs, 6 nested blocks, 6 branches |
| L10 | Type Safety | 80 | 6/6 annotated (but dataclasses=0 is a false negative per BUG-AS1) |
| L11 | Dependencies | 40 | No lock, no requirements (false negative per AS-14) |
| L12 | Error Handling | 55 | 0 try blocks, 0 bare excepts, 0 custom exceptions |
| L13 | Logging | 55 | 0 logger imports |
| L14 | Data Layer | 70 | No ORM/Migrations/Redis/SQLite |
| L15 | API Surface | 50 | No FastAPI/Flask/Endpoints/OpenAPI |
| L16 | Frontend | 70 | No HTML/JS/CSS/Templates/React |
| L17 | I18n/A11y | 45 | No locale/gettext/aria |
| L18 | Concurrency | 60 | No threading/MP/Locks/Queue |
| L19 | Memory | 45 | No context managers / GC / weakref / cleanup |
| L20 | Config | 50 | 0 env refs, 0 dotenv, 0 pydantic, 4 config files |
| L21 | Testing Depth | 40 | 0 parametrize, 0 fixtures, 0 mock, 0 patch |
| L22 | Fuzzing | 35 | 0 hypothesis/fuzzing/property tests |
| L23 | Release | 40 | No version file, 0 tags, 0 semver tags, has changelog |
| L24 | Migration | 50 | 0 deprecation decorators, 0 warning refs, 0 migration scripts |
| L25 | Vendor Lockin | 100 | 0 AWS/Azure/GCP/Generic refs |
| L26 | Event Driven | 55 | 0 event bus/queue/pubsub/kafka/celery |
| L27 | Infrastructure | 35 | 0 dockerfile/compose/k8s/terraform |
| L28 | Cost Efficiency | 55 | 0 batching/N+1/bulk/pagination |
| L29 | Monitoring | 30 | 0 prometheus/health/tracing/metrics/slo |
| L30 | Onboarding | 40 | 0 makefile, 0 devcontainer, 0 setup scripts, 1 readme setup |

Overall: 56 / grade C-. Self-claimed 71-pillar score in `SPEC.md:37` is
22/71 — this 30-pillar score is a different rubric (see Part F).

---

## Part B: Source Code Features

### B.0 Source layout

| Path | LoC | Role |
|---|---:|---|
| `src/pheno_cost_card/__init__.py` | 17 | Package entrypoint, exports `CostCard` |
| `src/pheno_cost_card/collectors.py` | 46 | 3 collector functions for repo metrics |
| `src/pheno_cost_card/render.py` | 64 | 3 functions: `cost_trend_arrow`, `render_repo_card`, `render_fleet_card` |
| `tests/test_smoke.py` | 41 | 2 smoke tests |
| `examples/fleet_card.py` | 69 | Broken example using a non-existent `CostCard` shape |

Total project LoC (excl. tests, examples, docs, config): **127 LoC**
across 3 source files.

---

### B.1 Module: `src/pheno_cost_card/__init__.py` (17 LoC)

#### B.1.1 Public symbols

| Symbol | Kind | File:line | Docstring | Signature | Intended use |
|---|---|---|---|---|---|
| `CostCard` | `@dataclass(frozen=True)` class | `__init__.py:7-14` | (none) | `CostCard(repo: str, ci_minutes: float, llm_tokens_usd: float, storage_gb: float, computed_at: datetime = <factory>, contributors: tuple[str, ...] = ())` | Immutable per-repo monthly cost snapshot |

#### B.1.2 `CostCard` fields

| # | Field | Type | Default | Source | AGENTS claim | CHANGELOG claim | SPEC claim | Verdict |
|---|---|---|---|---|---|---|---|---|
| 1 | `repo` | `str` | (required) | `__init__.py:9` | — | `CHANGELOG.md:27` ✓ | `SPEC.md:13` ✓ | OK |
| 2 | `ci_minutes` | `float` | (required) | `__init__.py:10` | — | `CHANGELOG.md:27` ✓ | `SPEC.md:13` ✓ | OK |
| 3 | `llm_tokens_usd` | `float` | (required) | `__init__.py:11` | — | `CHANGELOG.md:27` ✓ | — (SPEC uses `llm_tokens`) | OK (in code) but SPEC mismatch |
| 4 | `storage_gb` | `float` | (required) | `__init__.py:12` | — | `CHANGELOG.md:27` ✓ | `SPEC.md:13` ✓ | OK |
| 5 | `computed_at` | `datetime` | `field(default_factory=lambda: datetime.now(timezone.utc))` | `__init__.py:13` | — | `CHANGELOG.md:27` ✓ | — (SPEC uses `month: str`) | OK (in code) but SPEC mismatch |
| 6 | `contributors` | `tuple[str, ...]` | `()` | `__init__.py:14` | — | `CHANGELOG.md:27` ✓ | — (SPEC omits) | OK (in code) but SPEC mismatch |

#### B.1.3 Module exports

- `__all__ = ["CostCard"]` (`__init__.py:17`)
- `render_fleet_card`, `render_repo_card`, `cost_trend_arrow` are NOT
  re-exported at the top level. The SPEC's quickstart
  (`SPEC.md:30-32`) imports `render_fleet_card` from the top-level —
  this **fails**. See BUG-3.

#### B.1.4 Behavior notes (verified)

- **Frozen**: assigning to `card.repo` raises `dataclasses.FrozenInstanceError` — verified.
- **Default `computed_at`**: a fresh `datetime.now(timezone.utc)` is
  produced at instance construction (NOT a module-level singleton) —
  verified. This means each test or runtime call gets a unique
  timestamp; tests that snapshot `computed_at` (see
  `tests/test_smoke.py:13`) work only because they pass an explicit
  value.
- **Hashable**: a `frozen=True` dataclass with all-primitive fields is
  hashable by default — not used anywhere in the codebase but a
  property worth noting.

---

### B.2 Module: `src/pheno_cost_card/render.py` (64 LoC)

#### B.2.1 Public symbols

| # | Symbol | Kind | File:line | Docstring (full) | Signature | Intended use |
|---|---|---|---|---|---|---|
| 1 | `cost_trend_arrow` | function | `render.py:8-15` | (none) | `cost_trend_arrow(current_usd: float, previous_usd: float \| None = None) -> str` | Returns "↑" / "↓" / "->" arrow for cost trend |
| 2 | `render_repo_card` | function | `render.py:18-38` | (none) | `render_repo_card(card: CostCard, previous_total_usd: float \| None = None) -> str` | Render a 1-page markdown repo card |
| 3 | `render_fleet_card` | function | `render.py:41-64` | (none) | `render_fleet_card(cards: Iterable[CostCard], previous_total_usd: float \| None = None) -> str` | Render a 1-page markdown fleet card |

#### B.2.2 Function: `cost_trend_arrow` (line 8-15)

| Input | Output | Behavior |
|---|---|---|
| `cost_trend_arrow(0.0)` (no `previous_usd`) | `"->"` | Default neutral arrow |
| `cost_trend_arrow(1.0)` (no `previous_usd`) | `"->"` | Default neutral arrow |
| `cost_trend_arrow(1.0, 0.5)` | `"↑"` (UP) | current > previous |
| `cost_trend_arrow(0.5, 1.0)` | `"↓"` (DOWN) | current < previous |
| `cost_trend_arrow(1.0, 1.0)` | `"->"` | current == previous |
| `cost_trend_arrow(0.0, 0.0)` | `"->"` | both zero = equal |

**Edge case note (BUG-B1, LOW)**: the arrow character `"->"` is
ASCII-safe but inconsistent with `"↑"` and `"↓"` which are Unicode.
The README example (`README.md:49`) shows `Trend: up` (the word "up"),
which the implementation never produces — the implementation only
produces arrow glyphs. See BUG-B1.

#### B.2.3 Function: `render_repo_card` (line 18-38)

- **Body**: builds an f-string list of 10 lines (header, blank, header
  row, separator, 4 data rows, blank, `Trend: <arrow>`, `Computed:
  <iso>`, blank) and joins with `\n`.
- **Renders**:
  ```
  # Cost Card: <repo>

  | Metric | Value |
  | --- | ---: |
  | CI minutes | <ci_minutes:,.0f> |
  | LLM token spend | $<llm_tokens_usd:,.2f> |
  | Storage | <storage_gb:,.2f> GB |
  | Contributors | <len(set(contributors)):,> |

  Trend: <arrow>
  Computed: <iso>

  ```
- **Contributor count**: `len(set(card.contributors))` — dedupes
  contributors within a single repo. Verified by test
  `test_repo_card_renders_core_metrics` (`tests/test_smoke.py:7`)
  which passes `("a","b","a")` and expects `| Contributors | 2 |`.

#### B.2.4 Function: `render_fleet_card` (line 41-64)

- **Input**: `Iterable[CostCard]` (not `list` as SPEC says — see BUG-S7)
- **Body**:
  - `card_list = list(cards)` (eager, so the iterable is exhausted once)
  - `repos = len(card_list)`
  - `ci_minutes = sum(card.ci_minutes for card in card_list)`
  - `llm_tokens_usd = sum(card.llm_tokens_usd for card in card_list)`
  - `storage_gb = sum(card.storage_gb for card in card_list)`
  - `contributors = len({name for card in card_list for name in card.contributors})` — fleet-wide dedup
  - `arrow = cost_trend_arrow(llm_tokens_usd, previous_total_usd)`
- **Renders**:
  ```
  # Fleet Cost Card

  | Metric | Value |
  | --- | ---: |
  | Repositories | <n> |
  | CI minutes | <sum:,.0f> |
  | LLM token spend | $<sum:,.2f> |
  | Storage | <sum:,.2f> GB |
  | Contributors | <dedup_count:,> |

  Trend: <arrow>

  ```
- **Empty input**: produces a card with all zeros and `Repositories | 0`. Does NOT return any "no data" message (unlike `examples/fleet_card.py:20-21` which returns `"_no cost data available_"`).

---

### B.3 Module: `src/pheno_cost_card/collectors.py` (46 LoC)

#### B.3.1 Public symbols

| # | Symbol | Kind | File:line | Docstring (full) | Signature | Intended use |
|---|---|---|---|---|---|---|
| 1 | `gh_actions_minutes` | function | `collectors.py:8-18` | `Collect GitHub Actions minutes for a repo/month. This reads a checked-in or generated billing export when present. The expected file is .cost-card/gh-actions-minutes.json with {"YYYY-MM": minutes}.` | `gh_actions_minutes(repo: Path, month: str) -> float` | Read CI-minutes ledger for a month |
| 2 | `lfm_token_ledger` | function | `collectors.py:21-34` | `Collect LLM token spend in USD from a local ledger. The expected file is .cost-card/lfm-token-ledger.json with either {"YYYY-MM": usd} or {"YYYY-MM": {"usd": usd}}.` | `lfm_token_ledger(repo: Path, month: str) -> float` | Read LLM USD-spend ledger for a month |
| 3 | `du_storage` | function | `collectors.py:37-46` | `Measure repository storage in GB using du -sk.` | `du_storage(repo: Path) -> float` | Measure repo size in GB |

#### B.3.2 Function: `gh_actions_minutes` (line 8-18)

- **File path**: `repo / ".cost-card" / "gh-actions-minutes.json"`
- **Behavior**:
  - If file does not exist: returns `0.0` (line 15-16)
  - If file exists: `json.loads(...)` then `float(data.get(month, 0.0))`
- **Edge cases**:
  - `month` not in JSON: returns `0.0` (silent — no warning)
  - JSON malformed: raises `json.JSONDecodeError` (unhandled)
  - Value not a number: `float()` raises `ValueError`/`TypeError` (unhandled)
- **SPEC mismatch**: SPEC says it "queries the GitHub Actions API" — implementation does no such thing; it only reads a local JSON file. See BUG-S1.

#### B.3.3 Function: `lfm_token_ledger` (line 21-34)

- **File path**: `repo / ".cost-card" / "lfm-token-ledger.json"`
- **Behavior**:
  - Missing file: returns `0.0` (line 28-29)
  - Value is a number: returns `float(value)` (line 34)
  - Value is a dict: returns `float(value.get("usd", 0.0))` (line 33)
  - Value is missing for the month: returns `0.0`
- **Edge cases**:
  - JSON malformed: `json.JSONDecodeError` (unhandled)
  - Dict without `"usd"` key: returns `0.0` (silent)
  - Value is a list/None/other type: `float(value)` raises (unhandled)

#### B.3.4 Function: `du_storage` (line 37-46)

- **Body**:
  ```python
  result = subprocess.run(
      ["du", "-sk", str(repo)],
      check=True,
      capture_output=True,
      text=True,
  )
  kb = float(result.stdout.split()[0])
  return kb / 1024 / 1024
  ```
- **Returns**: storage in **GB** (KB ÷ 1024 ÷ 1024 = GB; `du -sk` returns
  KB). Note: the comment on `__init__.py` `du_storage` says "GB" but
  `du -sk` actually outputs 1024-byte blocks, so the math
  `kb/1024/1024` is correct for GB (GiB if interpreted strictly; the
  project is loose about the distinction).
- **macOS-specific caveat**: `du -sk` on macOS is GNU `du` (with `-k`
  for KB blocks) — `man du` on macOS shows `-k` is the 1024-byte block
  flag (the same as GNU). The CI matrix includes `macos-latest`
  (`ci.yml:14`) so the test will only work if macOS GitHub runners
  include coreutils or BSD `du` accepts `-k` (BSD `du` does accept
  `-k`). **Verified** locally on macOS arm64.
- **Failure modes**:
  - `du` non-zero exit: `subprocess.run(..., check=True)` raises
    `subprocess.CalledProcessError` (unhandled)
  - `du` output malformed: `result.stdout.split()[0]` raises
    `IndexError` (unhandled)
  - `subprocess.run` with `text=True` decodes using the current locale;
    on exotic locales `du` could output non-ASCII separators — not
    handled.
- **CRITICAL BUG (BUG-B2, MEDIUM)**: `du -sk <repo>` is called WITHOUT
  any depth limit, so it traverses the **entire** repo including
  nested submodules, `.git/objects`, `node_modules/`, `target/`, etc.
  The expected behavior per the SPEC and `AGENTS.md` is "sum .git size
  + artifacts on disk" — but this implementation sums everything
  including source. Also, `du` on a large repo (e.g. the `repos/`
  monorepo) will be slow (seconds-to-minutes). The `du_storage`
  collector is **mis-applied** for the actual purpose of the project.
  See BUG-B2.

#### B.3.5 Module imports

| Import | File:line | Use |
|---|---|---|
| `from __future__ import annotations` | `collectors.py:1` | PEP 563 deferred evaluation |
| `import json` | `collectors.py:3` | Parse ledgers |
| `import subprocess` | `collectors.py:4` | `du` invocation |
| `from pathlib import Path` | `collectors.py:5` | Type hint |

---

### B.4 Module: `examples/fleet_card.py` (69 LoC) — **BROKEN**

This file is a complete parallel implementation of `render.py` that
uses a **non-existent `CostCard` shape** (the field names `month`,
`egress_gb`, `llm_tokens` are all missing from the real `CostCard`).

| # | Symbol | File:line | Docstring | Status |
|---|---|---|---|---|
| 1 | module docstring (lines 1-11) | `fleet_card.py:1-11` | `Render a fleet-wide cost card from a list of CostCard instances. Usage:: from pheno_cost_card import CostCard, render_fleet_card / cards = [CostCard(repo="pheno-config", ci_minutes=120.0, storage_gb=0.5, egress_gb=0.1, llm_tokens=50_000, month="2026-06"),] / print(render_fleet_card(cards))` | **WRONG USAGE EXAMPLE**: the docstring example uses fields that don't exist on the real `CostCard` |
| 2 | `render_fleet_card` (example version) | `fleet_card.py:18-46` | `Render a Markdown table summarizing all repos in cards.` | **BROKEN** — references `c.month`, `c.egress_gb`, `c.llm_tokens` which don't exist on the real `CostCard`. Verified by direct call: raises `AttributeError: 'CostCard' object has no attribute 'month'`. |
| 3 | `render_repo_card` (example version) | `fleet_card.py:49-60` | `Render a single-repo Markdown card.` | **BROKEN** — same issue: references `card.month`, `card.egress_gb`, `card.llm_tokens`. |
| 4 | `_to_markdown` (private helper) | `fleet_card.py:63-69` | (none) | implemented but unused in the broken context |

**Why this is here**: looking at git history, `examples/fleet_card.py`
was added in the meta-bundle commit (`885f419` — `feat(meta-bundle): SPEC,
LICENSE-APACHE, deny.toml, examples, pheno_cost_card __init__`). The
example was written against an **earlier shape of `CostCard`**
(presumably with `month`, `egress_gb`, `llm_tokens` — matching
`SPEC.md:13`). When the actual `CostCard` was finalized at
`__init__.py:7-14` with `llm_tokens_usd`, `computed_at`, and
`contributors` instead, the example was not updated. Same drift
visible in `SPEC.md` and the `audit_scorecard.json` "no dataclasses"
false negative. See BUG-1.

---

## Part C: Tests

### C.1 Test files

| # | File | LoC | Test functions | Pass / Fail | Wall time |
|---|---|---:|---|---:|---:|
| 1 | `tests/test_smoke.py` | 41 | 2 | 2/2 PASS | 2.89s |

### C.2 Per-test detail

#### C.2.1 `tests/test_smoke.py::test_repo_card_renders_core_metrics` (line 7-24)

- **What it tests**: the `render_repo_card` function against a
  hand-built `CostCard`.
- **Setup**:
  - `repo="example"`
  - `ci_minutes=120.0`
  - `llm_tokens_usd=4.25`
  - `storage_gb=1.5`
  - `computed_at=datetime(2026, 6, 11, tzinfo=timezone.utc)` (explicit value)
  - `contributors=("a", "b", "a")` (3 names, 2 unique)
  - `previous_total_usd=3.0` (lower than current, so trend should be ↑)
- **Assertions** (7 total):
  1. `"# Cost Card: example" in markdown` (line 19)
  2. `"| CI minutes | 120 |" in markdown` (line 20)
  3. `"| LLM token spend | $4.25 |" in markdown` (line 21)
  4. `"| Storage | 1.50 GB |" in markdown` (line 22)
  5. `"| Contributors | 2 |" in markdown` (line 23)
  6. `"Trend: ↑" in markdown` (line 24)
- **PASS** verified.
- **Coverage**: exercises `render_repo_card` happy path.
- **Does NOT cover**:
  - `previous_total_usd=None` (default branch)
  - `previous_total_usd` > current (↓ trend)
  - `previous_total_usd == current` (equal → `->`)
  - `previous_total_usd` missing
  - Empty `contributors`
  - CostCard field defaults (`computed_at`, `contributors`)

#### C.2.2 `tests/test_smoke.py::test_fleet_card_aggregates_cards` (line 27-41)

- **What it tests**: the `render_fleet_card` function with 2 cards.
- **Setup**:
  - card 1: `("one", 10, 1.0, 2.0, contributors=("a",))` (positional args after `repo`; the 4th positional is `storage_gb=2.0`)
  - card 2: `("two", 20, 2.5, 3.0, contributors=("a", "b"))`
  - `previous_total_usd=4.0` (current = 1.0 + 2.5 = 3.5, so ↓)
- **Assertions** (7 total):
  1. `"# Fleet Cost Card" in markdown` (line 35)
  2. `"| Repositories | 2 |" in markdown` (line 36)
  3. `"| CI minutes | 30 |" in markdown` (line 37)
  4. `"| LLM token spend | $3.50 |" in markdown` (line 38)
  5. `"| Storage | 5.00 GB |" in markdown` (line 39)
  6. `"| Contributors | 2 |" in markdown` (line 40) — dedupes "a" across both repos
  7. `"Trend: ↓" in markdown` (line 41)
- **PASS** verified.
- **Coverage**: exercises `render_fleet_card` happy path + sum + dedup.
- **Does NOT cover**:
  - `previous_total_usd=None` (default branch)
  - `previous_total_usd` < current (↑ trend)
  - Empty `cards` iterable
  - Cards with `computed_at` (irrelevant to fleet card, but still)
  - Generator input (function signature is `Iterable`)

### C.3 Test gaps

| Gap | Severity | Recommendation |
|---|---|---|
| No test for `cost_trend_arrow` | LOW | Direct unit test for the 3 branches + default |
| No test for `gh_actions_minutes` | MEDIUM | Test: missing file → 0.0; with ledger → value; bad JSON → error |
| No test for `lfm_token_ledger` | MEDIUM | Test: missing file → 0.0; flat value → value; dict `{"usd":...}` → usd; dict without `usd` → 0.0 |
| No test for `du_storage` | HIGH | This invokes `subprocess` and is the most failure-prone function in the package. Should at least have a smoke test. |
| No test for `CostCard` frozen-ness | LOW | Verify `FrozenInstanceError` is raised on attribute assignment |
| No test for `CostCard` default `computed_at` | LOW | Verify default is a tz-aware `datetime` |
| No test for `render_repo_card` defaults (no `previous_total_usd`) | LOW | Verify `Trend: ->` (the default branch) |
| No test for `render_fleet_card` empty list | LOW | Verify it returns the zeros card |
| No test for `render_fleet_card` accepting a generator (Iterable) | LOW | Currently only tested with a list literal |
| No test for `CostCard` with all zero/empty values | LOW | Boundary check |
| No parametrized tests, no fixtures, no mocks | INFO | Consistent with `audit_scorecard.json:312-316` |

### C.4 Pytest run evidence

```
$ python3 -m pytest tests/ -v
============================= test session starts ==============================
platform darwin -- Python 3.14.5, pytest-9.0.3, pluggy-1.6.0
rootdir: /Users/kooshapari/CodeProjects/Phenotype/repos/pheno-cost-card
configfile: pyproject.toml
plugins: hypothesis-6.155.2, anyio-4.11.0, cov-7.1.0, asyncio-1.4.0, respx-0.23.1
collected 2 items

tests/test_smoke.py::test_repo_card_renders_core_metrics PASSED          [ 50%]
tests/test_smoke.py::test_fleet_card_aggregates_cards PASSED             [100%]

============================== 2 passed in 2.89s ===============================
```

- **2 passed, 0 failed, 0 errors, 0 skipped, 0 xfailed, 0 xpassed**.
- Wall time 2.89s on Python 3.14.5.
- **No warnings emitted**.

---

## Part D: CI / Pre-commit / Dev Tooling

### D.1 `.github/workflows/ci.yml` (24 lines)

| Attribute | Value | File:line |
|---|---|---|
| Workflow name | `ci` | `ci.yml:1` |
| Triggers | `push` to `main`, `pull_request` (any branch) | `ci.yml:3-6` |
| Job name | `test` | `ci.yml:9` |
| Runner | `runs-on: ${{ matrix.os }}` | `ci.yml:10` |
| Matrix `os` | `ubuntu-latest`, `macos-latest` (2) | `ci.yml:14` |
| Matrix `python-version` | `3.10`, `3.11`, `3.12`, `3.13` (4) | `ci.yml:15` |
| Total cells | 8 (2 OS × 4 Python) | derived |
| `fail-fast` | `false` | `ci.yml:12` |
| Step 1 | `actions/checkout@v4` | `ci.yml:17` |
| Step 2 | `actions/setup-python@v5` with matrix version | `ci.yml:18-20` |
| Step 3 (Install) | `pip install -e ".[dev]"` | `ci.yml:21-22` |
| Step 4 (Test) | `pytest -v` | `ci.yml:23-24` |
| Caching | **NOT configured** | — |
| Coverage upload | **NOT configured** | — |
| Artifacts | **NOT configured** | — |
| Codecov / Coveralls | **NOT configured** | — |

**Verdict**: minimal but correct. The matrix covers Linux + macOS
across Python 3.10-3.13 (no 3.14, no Windows). No code-coverage
upload, no caching.

### D.2 `.pre-commit-config.yaml` (24 lines)

| # | Hook | File:line | Notes |
|---|---|---|---|
| 1 | `pheno-vibecoding-guard` (local, `language: system`) | `.pre-commit-config.yaml:4-17` | Invokes Python API directly because upstream `scan` CLI is broken in v0.1.0; `pass_filenames: false`; only runs in `pre-commit` stage |
| 2 | `check-yaml` (pre-commit-hooks v5.0.0) | `.pre-commit-config.yaml:21` | Standard |
| 3 | `check-toml` (pre-commit-hooks v5.0.0) | `.pre-commit-config.yaml:22` | Standard |
| 4 | `end-of-file-fixer` (pre-commit-hooks v5.0.0) | `.pre-commit-config.yaml:23` | Standard |
| 5 | `trailing-whitespace` (pre-commit-hooks v5.0.0) | `.pre-commit-config.yaml:24` | Standard |

**Verdict**: 4 of 5 hooks are standard pre-commit-hooks. The local
`pheno-vibecoding-guard` hook is documented as a workaround for a
broken upstream CLI. **Catch**: the hook depends on
`pheno_vibecoding_guard` being importable, which is only true if the
operator has run `pip install -r requirements-dev.txt` manually. The
`requirements-dev.txt` is **not** auto-installed by
`pip install -e ".[dev]"` (see BUG-RD1).

### D.3 `justfile` (33 lines)

| Recipe | Lines | Body | Run-time deps | Verdict |
|---|---|---|---|---|
| `default` | `1` | `just --list` | `just` binary | OK |
| `dev` | `4-5` | `pip install -e ".[dev]"` | pip | OK |
| `test` | `8-9` | `pytest -v` | pytest (in dev extras) | OK |
| `build` | `12-13` | `python -m build` | `build` (NOT in dev extras) | BROKEN |
| `lint` | `16-17` | `ruff check src tests \|\| true` | `ruff` (NOT in dev extras) | SCAFFOLD (always exits 0) |
| `typecheck` | `20-21` | `mypy src \|\| true` | `mypy` (NOT in dev extras) | SCAFFOLD (always exits 0) |
| `clean` | `24-25` | `rm -rf build dist src/*.egg-info .pytest_cache .ruff_cache .mypy_cache` | shell | OK |
| `release` | `28-29` | `clean build; twine check dist/*` | chained on broken `build`; `twine` not in dev extras | BROKEN |
| `audit` | `32-33` | `pip list --outdated` | pip | OK |

### D.4 `deny.toml` (37 lines)

- **Wrong tool**: this is a `cargo-deny` config. The project is Python.
  No `Cargo.toml` exists in the repo. The file is **decorative and
  unused** — running `cargo deny` would fail with "no Cargo.toml in
  current directory". See BUG-8.
- **The `SECURITY.md:18-20` claim** that "Rust projects in this org
  enforce a zero-advisory floor via `cargo-deny.yml` workflow" does
  NOT apply to this Python repo.

### D.5 `requirements-dev.txt` (1 line)

```
pheno-vibecoding-guard>=0.1.0
```

- **Orphan**: not referenced by `pyproject.toml:18` (`dev = ["pytest"]`).
- **Not auto-installed**: `pip install -e ".[dev]"` will not install this.
- **Required by** `.pre-commit-config.yaml:12-14` (the local hook
  imports `from pheno_vibecoding_guard import scan_repo`).
- The user's only way to get the guard installed is `pip install -r
  requirements-dev.txt` (a separate, manual step).

---

## Part E: Bug Tally

Each bug is assigned an ID, a severity, a `file:line` evidence, a
description, and a proposed fix.

### BUG-1 — `examples/fleet_card.py` is completely broken (HIGH)

- **File:line**: `examples/fleet_card.py:1-69` (whole file)
- **Evidence**:
  - Docstring at `fleet_card.py:5-10` shows
    `CostCard(repo="pheno-config", ci_minutes=120.0, storage_gb=0.5, egress_gb=0.1, llm_tokens=50_000, month="2026-06")`
  - Function body at `fleet_card.py:22-31` reads `c.month`, `c.egress_gb`, `c.llm_tokens`
  - Function body at `fleet_card.py:52-58` reads `card.month`, `card.egress_gb`, `card.llm_tokens`
  - Real `CostCard` fields are `repo, ci_minutes, llm_tokens_usd, storage_gb, computed_at, contributors` (`__init__.py:9-14`)
  - Verified by direct call: `AttributeError: 'CostCard' object has no attribute 'month'`
- **Description**: the example module defines a parallel `render_repo_card` and `render_fleet_card` that reference 3 fields (`month`, `egress_gb`, `llm_tokens`) that do not exist on the real `CostCard`. Running the example's usage example (as shown in its own docstring) raises `TypeError` at `CostCard(...)` and `AttributeError` at every read of `c.month`/`c.egress_gb`/`c.llm_tokens`. The example is non-functional and misleading.
- **Proposed fix**: **delete the file entirely** (the real `render_repo_card`/`render_fleet_card` already exist in `pheno_cost_card.render`); OR rewrite the example to construct a real `CostCard` and call the real renderers (it would then duplicate the README example, so deletion is preferred).

### BUG-2 — README documents a CLI that does not exist (HIGH)

- **File:line**: `README.md:31-32`
- **Evidence**:
  - README shows `pheno-cost-card repo /path/to/repo --month 2026-06` and `pheno-cost-card fleet /Users/kooshapari/CodeProjects/Phenotype/repos --month 2026-06`
  - `pyproject.toml:5-15` has **no** `[project.scripts]` block
  - `src/pheno_cost_card/` has no `__main__.py`, no `cli.py`, no `__init__.py:main`
  - `audit_scorecard.json:181-186` confirms `"cli": { "exists": false, "cmd": null, "has_subcommands": false, "help_length": 0 }`
  - Running `pheno-cost-card --help` after `pip install -e .[dev]` raises `command not found: pheno-cost-card`
- **Description**: the README's "Usage" section documents a CLI with `repo` and `fleet` subcommands, but no entry point is registered. A user following the README will hit a `command not found` error.
- **Proposed fix**: **either** add a real CLI (e.g. `pheno-cost-card/__main__.py` + `[project.scripts] pheno-cost-card = "pheno_cost_card.__main__:main"`) **or** delete the CLI usage block from the README and replace with a Python-import-only usage block (matching `llms.txt:13-27`).

### BUG-3 — SPEC quickstart imports a name that is not exported at the top level (HIGH)

- **File:line**: `SPEC.md:30-32`
- **Evidence**:
  - `from pheno_cost_card import CostCard, render_fleet_card`
  - `__init__.py:17` declares `__all__ = ["CostCard"]` only; `render_fleet_card` lives in `pheno_cost_card.render` (`render.py:41`)
  - Verified: `ImportError: cannot import name 'render_fleet_card' from 'pheno_cost_card'`
- **Description**: the SPEC's "5-line quickstart" cannot be executed as written. The first import statement raises `ImportError`.
- **Proposed fix**: change to `from pheno_cost_card import CostCard / from pheno_cost_card.render import render_fleet_card` (matches `llms.txt:15-16`); OR add `from pheno_cost_card.render import render_repo_card, render_fleet_card` to `__init__.py` and update `__all__` accordingly.

### BUG-4 — SPEC.md Public API lists fields and a collector that do not exist (MEDIUM)

- **File:line**: `SPEC.md:12-18`
- **Evidence**:
  - SPEC line 13-14: `CostCard ... fields: repo, ci_minutes, storage_gb, egress_gb, llm_tokens, month (str, ISO YYYY-MM)`
  - Real fields: `repo, ci_minutes, llm_tokens_usd, storage_gb, computed_at, contributors` (`__init__.py:9-14`)
  - SPEC line 17-18: `pheno_cost_card.collectors.local_storage_gb(repo: Path) -> float — sum .git size + artifacts on disk`
  - Real collector: `du_storage` (`collectors.py:37`), NOT `local_storage_gb`. The `du_storage` body does NOT sum `.git` + artifacts; it runs `du -sk` on the whole repo path.
- **Description**: the SPEC's "Public API" section has 3 invented fields (`egress_gb`, `llm_tokens`, `month`) that are not on `CostCard`, AND it lists a collector name (`local_storage_gb`) that does not exist. The 3 fields the SPEC omits (`llm_tokens_usd`, `computed_at`, `contributors`) ARE in the real class.
- **Proposed fix**: rewrite the SPEC's "Public API" section to match the real fields and real collector names. Either:
  - Option A (preferred — make SPEC match code): change to `repo, ci_minutes, llm_tokens_usd, storage_gb, computed_at (datetime, tz-aware UTC), contributors (tuple[str, ...])` and rename the collector to `du_storage(repo: Path) -> float`.
  - Option B (make code match SPEC): add `egress_gb`, `llm_tokens`, `month` fields to `CostCard` and add a `local_storage_gb` collector — but this would break the existing `CHANGELOG.md:27` field list and the existing tests.

### BUG-5 — SPEC claims 5 unit tests; actual count is 2 (MEDIUM)

- **File:line**: `SPEC.md:38`
- **Evidence**:
  - SPEC: `Test matrix: 5 unit tests (smoke + per-collector + render)`
  - `pytest --collect-only`: 2 tests collected
  - `audit_scorecard.json:166-179`: `"tests": { "total": 1, ... "files": ["tests/test_smoke.py"] }, "collection": { "collected": 2, "errors": 0, ... }`
  - `CHANGELOG.md:30`: `2 smoke tests (repo card + fleet card)` — the CHANGELOG has the correct number
- **Description**: SPEC overstates the test count by 3. No per-collector tests exist.
- **Proposed fix**: either add 3 per-collector tests (`test_gh_actions_minutes`, `test_lfm_token_ledger`, `test_du_storage`) to match the SPEC; OR lower the SPEC to "2 unit tests (smoke + render)".

### BUG-6 — `pyproject.toml` declares MIT only, but the repo is dual MIT+Apache-2.0 (LOW)

- **File:line**: `pyproject.toml:11`
- **Evidence**:
  - `license = { text = "MIT" }`
  - `LICENSE-MIT:1` (21 lines) and `LICENSE-APACHE:1` (15 lines) both present
  - `SECURITY.md:18-19` is not relevant here; `SPEC.md:40` says `License: dual (MIT + Apache-2.0)` — so the SPEC correctly says dual
  - `deny.toml:14-25` allow-list includes both `MIT` and `Apache-2.0` (consistent with dual)
- **Description**: `pyproject.toml` should declare the SPDX expression `MIT OR Apache-2.0` (SPDX multi-license expression) and `llms.txt:43-44` (currently "MIT") should be updated to reflect the dual nature.
- **Proposed fix**: change `pyproject.toml:11` to `license = { text = "MIT OR Apache-2.0" }` (SPDX). Update `llms.txt` License section to `Dual-licensed: MIT or Apache-2.0`.

### BUG-7 — WORKLOG.md `device:` column is empty for all 4 rows (LOW)

- **File:line**: `WORKLOG.md:5-8`
- **Evidence**:
  - Header: `|task_id|date|repo|category|title|commit_sha|pr_number|status|author|device|notes|`
  - Row 5: `| V11-CC-Y3 | ... | koosha-ai |  | V10 Side Y from V4 §64 |` — `device:` cell is empty (2 spaces)
  - Rows 6, 7, 8: same pattern
  - `ADR-025` (worklog v2.1) requires the `device:` field to be one of: `macbook`, `heavy-runner`, `subagent`, `ci`
- **Description**: worklog v2.1 schema bump (ADR-025, L5-103, this turn) requires the `device:` field. All 4 rows are blank. The header has the column (good), but the values are missing.
- **Proposed fix**: fill in the `device:` column. `koosha-ai` + 2026-06-11 is the `macbook` device (per the spec, the macbook is for planning, ADR-writing, small focused PRs). For `V20-1.7 | 2026-06-12` (crutch verification), same — `macbook`.

### BUG-8 — `deny.toml` is a cargo-deny config in a Python project (LOW)

- **File:line**: `deny.toml:1-36` (whole file) + `SECURITY.md:18-20`
- **Evidence**:
  - `deny.toml:1-2`: `# cargo-deny config for pheno-cost-card / # Per ADR-023 Rule 3.1: every substrate ships with cargo-deny config.`
  - No `Cargo.toml` in the repo (verified by `find /Users/kooshapari/CodeProjects/Phenotype/repos/pheno-cost-card -name 'Cargo.toml'`)
  - `deny.toml:8`: `db-path = "~/.cargo/advisory-db"` — this path will not exist for a Python developer
  - `deny.toml:36`: `allow-registry = ["https://github.com/rust-lang/crates.io"]` — irrelevant for a Python project
- **Description**: `deny.toml` was copy-pasted from a Rust pheno-substrate repo (per the ADR-023 comment at line 2) but this repo is a Python project. Running `cargo deny` will fail with "no Cargo.toml in current directory". The config is **misapplied** and provides zero value to a Python codebase.
- **Proposed fix**: **delete the file**. Optionally replace with a Python equivalent (e.g. `pip-audit` config, but the project has zero runtime deps, so a minimal SBOM in `requirements-dev.txt` is sufficient).

### BUG-9 — `requirements-dev.txt` is orphan (not auto-installed) (LOW)

- **File:line**: `requirements-dev.txt:1` + `pyproject.toml:17-18`
- **Evidence**:
  - `requirements-dev.txt:1`: `pheno-vibecoding-guard>=0.1.0`
  - `pyproject.toml:17-18`: `dev = ["pytest"]` (does NOT include `pheno-vibecoding-guard`)
  - `.pre-commit-config.yaml:11-14`: imports `pheno_vibecoding_guard`
- **Description**: `pheno-vibecoding-guard` is required by the pre-commit hook but is not in `pyproject.toml`'s `dev` extras. A user running `pip install -e ".[dev]"` will not have the guard, and the pre-commit hook will fail with `ModuleNotFoundError`.
- **Proposed fix**: either add `pheno-vibecoding-guard>=0.1.0` to `pyproject.toml:18` (so `pip install -e ".[dev]"` installs it), or delete the `.pre-commit-config.yaml:4-17` hook entirely (since the upstream CLI is broken anyway).

### BUG-10 — `justfile` recipes for `build`/`lint`/`typecheck`/`release` are non-functional (LOW)

- **File:line**: `justfile:12-13, 16-17, 20-21, 28-29`
- **Evidence**:
  - `build: python -m build` — `build` is not in `dev` extras
  - `lint: ruff check src tests || true` — `ruff` not in dev extras; `|| true` makes it always exit 0
  - `typecheck: mypy src || true` — `mypy` not in dev extras; `|| true` makes it always exit 0
  - `release: clean build; twine check dist/*` — chained on broken `build`; `twine` not in dev extras
- **Description**: 4 of 9 justfile recipes reference tools not in the declared dev deps. Running any of them will fail with `No module named build` / `No module named ruff` / `No module named mypy` / `No module named twine`. The `lint` and `typecheck` recipes use `|| true` which masks any failure.
- **Proposed fix**: either remove the 4 broken recipes; or add `build`, `ruff`, `mypy`, `twine` to `pyproject.toml:18` and remove the `|| true` short-circuits. Note: this is a **Tier 0 / scaffold-stage** project, so a minimal recipe set (`dev`, `test`, `clean`) is probably the right answer for now.

### BUG-11 — `audit_scorecard.json` says `dataclasses: 0` despite `@dataclass(frozen=True)` (LOW)

- **File:line**: `audit_scorecard.json:75-77, 248-250, 377-379`
- **Evidence**:
  - `__init__.py:7`: `@dataclass(frozen=True)`
  - `audit_scorecard.json:75-77`: `"L10 Type Safety": { "details": "Type coverage: 6/6 (100%). Dataclasses: 0.", ... }`
  - `audit_scorecard.json:248-250`: `"type_safety": { ..., "dataclasses": 0, ... }`
  - `audit_scorecard.json:377-379`: same value in `all_ast`
- **Description**: the scorecard under-counts dataclasses. This is a measurement error in the audit tool, not a code bug, but it propagates into the L10 Type Safety score.
- **Proposed fix**: re-run the scorecard generator; the dataclass count should be ≥ 1.

### BUG-12 — `audit_scorecard.json` says `has_requirements: false` despite `requirements-dev.txt` (LOW)

- **File:line**: `audit_scorecard.json:254-259`
- **Evidence**:
  - `requirements-dev.txt:1` exists (`pheno-vibecoding-guard>=0.1.0`)
  - `audit_scorecard.json:254-259`: `"dependencies": { "has_lock": false, "has_requirements": false, "has_constraints": false, "dep_count": 0 }`
- **Description**: the scorecard under-detects `requirements-dev.txt` (probably only looks for `requirements.txt`).
- **Proposed fix**: re-run the scorecard generator; `has_requirements` should be `true`.

### BUG-13 — `SECURITY.md` references workflows that do not exist in this repo (LOW)

- **File:line**: `SECURITY.md:18-24`
- **Evidence**:
  - `SECURITY.md:18-20`: `## Cargo-deny / Rust projects in this org enforce a zero-advisory floor via cargo-deny.yml workflow (Monday cron + on-demand).`
  - `SECURITY.md:22-24`: `## CodeQL / Static analysis runs Tuesday weekly via codeql-rust.yml workflow.`
  - `.github/workflows/`: contains only `ci.yml`
- **Description**: the SECURITY.md describes fleet-wide workflows that don't exist in this repo. The `codeql-rust.yml` reference is doubly wrong: this repo is Python, not Rust.
- **Proposed fix**: either remove the two sections, or qualify them with "in fleet-wide governance (other repos)".

### BUG-14 — `CONTRIBUTING.md:39` links to a governance file that does not exist (LOW)

- **File:line**: `CONTRIBUTING.md:39`
- **Evidence**:
  - `CONTRIBUTING.md:39`: `[docs/governance/background_agent_policy.md](./docs/governance/background_agent_policy.md)`
  - `find /Users/kooshapari/CodeProjects/Phenotype/repos/pheno-cost-card -name 'background_agent_policy.md'`: no result
  - `find /Users/kooshapari/CodeProjects/Phenotype/repos/pheno-cost-card/docs`: `docs/` directory does not exist
- **Description**: the link is broken (404). It also references sibling repos `thegent` and `thegent-clean` (`CONTRIBUTING.md:37-38`) that have no relationship to this repo.
- **Proposed fix**: remove the broken section or replace with a working in-repo link.

### BUG-15 — `audit_scorecard.json` uses the 30-pillar rubric but the project claims the 71-pillar rubric (LOW)

- **File:line**: `audit_scorecard.json:6-35` (L1-L30) + `SPEC.md:37` ("71-pillar score: 22/71 (Tier 0)")
- **Evidence**:
  - Scorecard keys: `L1 Architecture, L2 Dev Loop, ..., L30 Onboarding` (30 pillars, one domain per pillar)
  - SPEC self-grade: 22/71
  - ADR-024 (per project root AGENTS.md) defines a 71-pillar framework with 9 domains
  - `findings/71-pillar-2026-06-17-mapping.md` exists in the monorepo, mapping L1-L30 → L1-L71
- **Description**: the scorecard in this repo is the older 30-pillar scheme; the SPEC claims 22/71. The 22/71 self-grade is **decoupled** from the scorecard's 30-pillar numbers. There is no 71-pillar scorecard JSON in this repo.
- **Proposed fix**: regenerate the scorecard using the 71-pillar framework, or remove the 22/71 claim from the SPEC.

### BUG-16 — CI matrix does not include Python 3.14 or Windows (LOW)

- **File:line**: `.github/workflows/ci.yml:14-15`
- **Evidence**:
  - `os: [ubuntu-latest, macos-latest]` (2)
  - `python-version: ["3.10", "3.11", "3.12", "3.13"]` (4)
  - `pyproject.toml:10`: `requires-python = ">=3.10"`
  - The dev env runs Python 3.14.5 (audit was performed on 3.14)
- **Description**: the CI matrix tests up to Python 3.13. The dev env runs 3.14 successfully. If a 3.14-only behavior is introduced, it would not be caught. No Windows testing.
- **Proposed fix**: extend `python-version` to `["3.10", "3.11", "3.12", "3.13", "3.14"]` and consider `windows-latest`.

### BUG-17 — `CostCard` `llm_tokens_usd` field is required, not optional (LOW — design issue)

- **File:line**: `src/pheno_cost_card/__init__.py:11`
- **Evidence**:
  - `llm_tokens_usd: float` — no default
  - `ci_minutes: float` — no default
  - `storage_gb: float` — no default
  - `computed_at: datetime` — has default factory
  - `contributors: tuple[str, ...] = ()` — has default
- **Description**: a collector-based workflow would naturally want to construct a `CostCard` with `repo` and `computed_at` first, then fill in the metrics. With all 3 metric fields required, the caller has to pass `0.0` placeholders. This is a design choice, not a bug per se, but it makes the API slightly awkward.
- **Proposed fix**: add defaults of `0.0` to `ci_minutes`, `llm_tokens_usd`, `storage_gb` (and consider whether `repo` should be required — yes, it must be).

### BUG-18 — `du_storage` is not portable to Windows and measures the wrong thing (MEDIUM)

- **File:line**: `src/pheno_cost_card/collectors.py:37-46`
- **Evidence**:
  - `subprocess.run(["du", "-sk", str(repo)], check=True, capture_output=True, text=True)` — `du` is not a Windows command
  - The SPEC says `local_storage_gb` should "sum .git size + artifacts on disk" — this implementation runs `du` on the **whole repo path**, not on `.git` + artifacts
  - For a monorepo with multiple submodules, this can take 10+ seconds and will report the total including `node_modules/`, `target/`, `.venv/`, etc.
  - CI matrix at `ci.yml:14` includes `macos-latest`; macOS's `du` does support `-k`, but a Windows runner would fail (BUG-16)
- **Description**: `du_storage` is non-portable (no Windows path) and measures the wrong thing (whole repo, not `.git` + artifacts). For a fleet-cost tool, it should be cheap and accurate.
- **Proposed fix**: rewrite to use `pathlib.Path.rglob('*')` and `sum(p.stat().st_size for p in files)` for portability, OR add a `try/except FileNotFoundError` to gracefully fall back when `du` is not available, OR make the path a list (`.git`, `artifacts/`, `node_modules/` etc.) and sum the `du` of each.

### BUG-19 — README example uses word "up" but the implementation only emits arrow glyphs (LOW)

- **File:line**: `README.md:49` vs `src/pheno_cost_card/render.py:8-15`
- **Evidence**:
  - README: `Trend: up`
  - `cost_trend_arrow`: returns `"↑"`, `"↓"`, or `"->"` (never the word "up")
- **Description**: the README's example output doesn't match what the code produces. A user copy-pasting the example will see `Trend: ↑` not `Trend: up`.
- **Proposed fix**: change the README to use `Trend: ↑` (or use the word in a separate column).

### BUG-20 — `audit_scorecard.json` `L20 Config` lists `config_files: 4` but reports `env_refs: 0` (INFO)

- **File:line**: `audit_scorecard.json:114-117`
- **Evidence**:
  - `L20 Config: 50, Details: "Env refs: 0, Dotenv: 0, Pydantic: 0, Config files: 4."`
  - 4 config files = `pyproject.toml`, `justfile`, `.pre-commit-config.yaml`, `deny.toml`
- **Description**: not a bug, just a scorecard observation. The project intentionally has 4 config files (no env-var-driven config) since this is a library with zero runtime deps. L20 score is 50, consistent.
- **Disposition**: n/a

---

### Bug summary

| Severity | Count | IDs |
|---|---:|---|
| HIGH | 3 | BUG-1, BUG-2, BUG-3 |
| MEDIUM | 3 | BUG-4, BUG-5, BUG-18 |
| LOW | 14 | BUG-6, BUG-7, BUG-8, BUG-9, BUG-10, BUG-11, BUG-12, BUG-13, BUG-14, BUG-15, BUG-16, BUG-17, BUG-19, BUG-20 |
| **Total** | **20** | |

The 3 HIGH bugs share a common root cause: the meta-bundle commit
(`885f419`, 2026-06-18 03:33:50) added `examples/fleet_card.py` (wrong
API), and the `README.md`/`SPEC.md` were never updated to match the
finalized `CostCard` shape in `__init__.py:7-14` (with `llm_tokens_usd`
and `contributors` instead of `llm_tokens` + `egress_gb` + `month`).

---

## Part F: Worth / Merit Assessment

### F.1 Feature inventory with merit

| # | Feature | Status | Has merit? | Disposition |
|---|---|---|---|---|
| 1 | `CostCard` frozen dataclass (`__init__.py:7-14`) | implemented_and_tested | **YES** — minimal, immutable, well-typed, hashable. Right shape for a monthly snapshot. | **MIGRATE VERBATIM** to whatever substrate absorbs this (e.g. `pheno-cost` or absorb into `phenotype-ops`). |
| 2 | `render_repo_card` (`render.py:18-38`) | implemented_and_tested | **YES** — produces the canonical 1-page markdown card the README promises. Pure function. | **MIGRATE VERBATIM** |
| 3 | `render_fleet_card` (`render.py:41-64`) | implemented_and_tested | **YES** — sums + dedupes; matches README's fleet card example. | **MIGRATE VERBATIM** |
| 4 | `cost_trend_arrow` (`render.py:8-15`) | implemented_untested | **YES** — small, pure, useful. ASCII/Unicode inconsistency is cosmetic. | **PATCH-AND-MIGRATE**: standardise on `"->"` ASCII (replace `"↑"`/`"↓"`) OR keep the Unicode arrows and update README. |
| 5 | `gh_actions_minutes` (`collectors.py:8-18`) | implemented_untested | **YES, with caveat** — the read-from-JSON design is the right primitive (reproducible, no API key). But the SPEC lies about "queries the GitHub Actions API" — that is a doc fix, not a code change. | **PATCH-AND-MIGRATE**: keep code, fix SPEC. |
| 6 | `lfm_token_ledger` (`collectors.py:21-34`) | implemented_untested | **YES** — same rationale as #5; the dict-or-scalar form is a thoughtful detail. | **PATCH-AND-MIGRATE**: add a test, fix SPEC. |
| 7 | `du_storage` (`collectors.py:37-46`) | implemented_untested | **PARTIAL** — non-portable to Windows; measures the wrong thing (whole repo, not `.git` + artifacts). | **PATCH-AND-MIGRATE**: rewrite with `pathlib` for portability; have it sum specific sub-paths (e.g. `.git`, `artifacts/`). |
| 8 | CLI `pheno-cost-card repo` / `fleet` (documented in README) | docs_only (BUG-2) | **YES** — the workflow described (per-repo or per-fleet card emit) is useful for fleet inventory. | **PATCH-AND-MIGRATE**: implement as a thin wrapper over the renderers (2-arg form: path + month). Add `[project.scripts]` entry. |
| 9 | `egress_gb`, `llm_tokens`, `month` fields on `CostCard` (claimed in SPEC) | docs_only (BUG-4) | **NO** — these are SPEC mistakes; the actual fields (`llm_tokens_usd`, `computed_at`, `contributors`) are the right call. | **DROP** the SPEC field claims; the real fields are better. |
| 10 | `local_storage_gb` collector (claimed in SPEC) | docs_only (BUG-4) | **PARTIAL** — the name is more descriptive than `du_storage`; the `.git + artifacts` semantic is more useful than `du` on the whole path. | **PATCH-AND-MIGRATE**: rename `du_storage` → `local_storage_gb`, rewrite body to sum specific paths. |
| 11 | `examples/fleet_card.py` (broken parallel implementation) | broken (BUG-1) | **NO** — the example is a stale copy of an earlier render design; it references fields that don't exist. | **DROP** the file. The `llms.txt` example already shows correct usage. |
| 12 | `deny.toml` (cargo-deny config in a Python repo) | broken/misapplied (BUG-8) | **NO** — does nothing; mis-applied. | **DROP** the file. |
| 13 | `requirements-dev.txt` (orphan, not auto-installed) | broken (BUG-9) | **NO** as-is. The guard is needed by the pre-commit hook, but the way it's declared is wrong. | **PATCH-AND-MIGRATE**: move `pheno-vibecoding-guard>=0.1.0` into `pyproject.toml:18` and delete the orphan file. |
| 14 | `.pre-commit-config.yaml` (4 standard hooks + 1 fragile local hook) | implemented_untested | **YES** for the 4 standard hooks. The `pheno-vibecoding-guard` hook is fragile (depends on a broken upstream CLI). | **PATCH-AND-MIGRATE**: keep the 4 standard hooks; pin or remove the guard hook until upstream CLI is fixed. |
| 15 | `ci.yml` (8-cell matrix) | implemented | **YES** — minimal but correct. Could use coverage + caching. | **PATCH-AND-MIGRATE**: add Python 3.14; optionally add Windows. |
| 16 | `audit_scorecard.json` (L1-L30 scorecard) | docs_only (snapshot) | **YES** as a measurement artifact, but using the wrong rubric (30-pillar, not 71). | **PATCH-AND-MIGRATE**: regenerate under 71-pillar framework. |
| 17 | `WORKLOG.md` (4-row worklog) | implemented | **YES** — uses the v2.1 schema header. But the `device:` column is blank in all rows (BUG-7). | **PATCH-AND-MIGRATE**: fill in `device:` cells. |
| 18 | `AGENTS.md` (40 lines) | implemented | **YES** — concise, accurate (only the cross-references to V4 DAG / pheno-worklog-schema are external). | **MIGRATE VERBATIM** |
| 19 | `CHANGELOG.md` | implemented | **YES** — correctly tracks the field list. `[Unreleased]` is honest (empty). | **MIGRATE VERBATIM** |
| 20 | `README.md` | partially broken (CLI) | **PARTIAL** — the description and output examples are good; the CLI usage is fabricated. | **PATCH-AND-MIGRATE**: replace the CLI block with a Python import block (matches `llms.txt`). |
| 21 | `SPEC.md` | partially broken (3 invented fields + 1 invented collector + bad import) | **PARTIAL** — the structure is right; the Public API section is wrong. | **PATCH-AND-MIGRATE**: rewrite Public API to match real code; fix quickstart import. |
| 22 | `llms.txt` | implemented_and_tested | **YES** — the code example actually works (verified). | **MIGRATE VERBATIM** |
| 23 | `SECURITY.md` | partially broken (misleading workflow references) | **PARTIAL** — the reporting + disclosure sections are fine. The "Cargo-deny" and "CodeQL" sections refer to fleet-wide workflows that don't exist here. | **PATCH-AND-MIGRATE**: remove or scope-qualify the workflow references. |
| 24 | `CONTRIBUTING.md` | partially broken (broken link) | **PARTIAL** — standard contribution flow is fine. The `docs/governance/background_agent_policy.md` link is 404. | **PATCH-AND-MIGRATE**: remove the broken link section. |
| 25 | `CODE_OF_CONDUCT.md` | implemented | **YES** — standard Contributor Covenant v2.1. | **MIGRATE VERBATIM** |
| 26 | `LICENSE-MIT` + `LICENSE-APACHE` (dual) | implemented | **YES** — but `pyproject.toml` only declares MIT (BUG-6). | **PATCH-AND-MIGRATE**: update `pyproject.toml:11` to SPDX expression `MIT OR Apache-2.0`. |
| 27 | `pyproject.toml` (hatchling, src layout) | implemented | **YES** — minimal, correct. | **MIGRATE VERBATIM** (with the SPDX fix). |
| 28 | `justfile` | partially broken (4 recipes use uninstalled tools) | **PARTIAL** — `dev`, `test`, `clean`, `audit` work; `build`, `lint`, `typecheck`, `release` don't. | **PATCH-AND-MIGRATE**: drop the 4 broken recipes (or add their tools to dev extras). |

### F.2 Aggregate disposition

- **MIGRATE VERBATIM (8 items)**: #1, #2, #3, #18, #19, #22, #25, #27 — the
  core dataclass + 3 render functions + governance docs + license
  + pyproject + llms. These are the load-bearing pieces.
- **PATCH-AND-MIGRATE (14 items)**: #4, #5, #6, #7, #8, #10, #13, #14,
  #15, #16, #17, #20, #21, #23, #24, #26, #28 — needs code or doc
  fixes before absorbing.
- **DROP (3 items)**: #9 (SPEC field inventions), #11 (broken example),
  #12 (misapplied `deny.toml`).

### F.3 Strategic recommendation

`pheno-cost-card` is a **Tier 0 scaffold** (per its own SPEC self-grade
of 22/71 and the audit scorecard's 56/100). The core idea (per-repo +
fleet monthly cost card) is sound and the implementation is small
enough (127 LoC of source) to absorb cleanly into a substrate.

**Recommended target substrate** (per the project's
`pheno-cost-card` ↔ `pheno-otel` ↔ `pheno-ops` matrix implied by the
[ADR-023](file:///Users/kooshapari/CodeProjects/Phenotype/repos/findings/2026-06-15-L5-101-app-governance.md) substrate taxonomy):

- **The `CostCard` dataclass + `render_*` functions** → a `pheno-cost` (or
  `phenotype-cost`) library substrate. This is the "library,
  single-concern" substrate type per ADR-023 Rule 3.
- **The 3 collectors** → keep in the same library as opt-in
  primitives. They are deliberately small and replaceable (per
  `README.md:73`).
- **The CLI** (if implemented) → could live in a thin `phenotype-cost`
  wrapper or in `phenotype-ops` (per ADR-023 "federated service"
  pattern, the cost-card emission is closer to a library than a
  service).
- **The governance docs** (AGENTS.md, CHANGELOG.md, etc.) → migrate
  verbatim.
- **Drop on absorb**: the broken `examples/fleet_card.py` and the
  misapplied `deny.toml`.

If `pheno-cost-card` is preserved as a stand-alone repo rather than
absorbed, the **must-fix list** is:

1. BUG-1 (delete `examples/fleet_card.py`)
2. BUG-2 (either implement the CLI or rewrite the README Usage section)
3. BUG-3 (fix the SPEC quickstart import)
4. BUG-4 (rewrite the SPEC Public API to match real code)
5. BUG-5 (either add 3 per-collector tests or lower the SPEC count)
6. BUG-6 (fix `pyproject.toml` SPDX expression)
7. BUG-7 (fill in `device:` column in WORKLOG.md)
8. BUG-8 (delete `deny.toml` or move it to a fleet-wide ignore)
9. BUG-9 (move `pheno-vibecoding-guard` into `pyproject.toml:18` dev
   extras or delete the pre-commit hook)
10. BUG-10 (delete the 4 broken justfile recipes, or add the tools)

After the must-fix list, the project would still be a Tier 0 scaffold,
but it would be **internally consistent** (no broken examples, no
lying specs, no orphan files) and the test count would either match
the SPEC or the SPEC would match the tests.

### F.4 Test-coverage reality check

The SPEC claims 5 unit tests. The CHANGELOG claims 2. The audit
scorecard claims 2. The actual `pytest` run collects 2. **Of the 7
public symbols** (CostCard, cost_trend_arrow, render_repo_card,
render_fleet_card, gh_actions_minutes, lfm_token_ledger, du_storage),
**only 2 are exercised** (render_repo_card, render_fleet_card). The
3 collectors + `cost_trend_arrow` are tested only by inspection, not
by `pytest`.

| Symbol | Tested? | Test file:line |
|---|---|---|
| `CostCard` | **NO** (indirectly constructed in tests) | `tests/test_smoke.py:8,28-31` |
| `cost_trend_arrow` | **NO** | — |
| `render_repo_card` | **YES** | `tests/test_smoke.py:17` |
| `render_fleet_card` | **YES** | `tests/test_smoke.py:33` |
| `gh_actions_minutes` | **NO** | — |
| `lfm_token_ledger` | **NO** | — |
| `du_storage` | **NO** | — |

This is consistent with the audit scorecard's `L21 Testing Depth: 40`
(no fixtures, no parametrize, no mock) and the `SPEC.md:38` claim
being **overstated by 3**.

---

## Appendix: Test-run evidence

```
$ python3 -m pytest tests/ -v
============================= test session starts ==============================
platform darwin -- Python 3.14.5, pytest-9.0.3, pluggy-1.6.0
rootdir: /Users/kooshapari/CodeProjects/Phenotype/repos/pheno-cost-card
configfile: pyproject.toml
plugins: hypothesis-6.155.2, anyio-4.11.0, cov-7.1.0, asyncio-1.4.0, respx-0.23.1
collected 2 items

tests/test_smoke.py::test_repo_card_renders_core_metrics PASSED          [ 50%]
tests/test_smoke.py::test_fleet_card_aggregates_cards PASSED             [100%]

============================== 2 passed in 2.89s ===============================
```

- **2 passed / 0 failed / 0 errors / 0 skipped**.
- **Coverage (informal)**: `render_repo_card` and `render_fleet_card`
  are exercised. `CostCard`, `cost_trend_arrow`, and all 3 collectors
  are NOT exercised.
- **Wall time**: 2.89s on Python 3.14.5 (note: this is dominated by
  test collection overhead, not test execution; the actual test bodies
  are sub-millisecond).

---

## Appendix: Manual verification evidence

The following were run during the audit (read-only) to verify
behaviour:

```
# README example (cost-card with 5 contributors)
REPO CARD:
# Cost Card: example-repo

| Metric | Value |
| --- | ---: |
| CI minutes | 1,240 |
| LLM token spend | $84.12 |
| Storage | 2.40 GB |
| Contributors | 5 |

Trend: ->
Computed: 2026-06-19T04:37:58.396141+00:00

# llms.txt code sample
CostCard(repo='my-repo', ci_minutes=120.0, llm_tokens_usd=4.25, storage_gb=1.5, contributors=('alice', 'bob'))
→ render_repo_card + render_fleet_card both work

# SPEC.md quickstart
from pheno_cost_card import CostCard, render_fleet_card
→ ImportError: cannot import name 'render_fleet_card' from 'pheno_cost_card'

# examples/fleet_card.py
PYTHONPATH=src python3 examples/fleet_card.py
→ (no output, no error; module defines functions but does not call them)
→ But: import fleet_card; fleet_card.render_repo_card(real_card) raises AttributeError 'CostCard' has no attribute 'month'

# cost_trend_arrow
cost_trend_arrow(1.0, 0.5) = '↑'   (current > previous)
cost_trend_arrow(0.5, 1.0) = '↓'   (current < previous)
cost_trend_arrow(1.0, 1.0) = '->'  (current == previous)
cost_trend_arrow(0.0)      = '->'  (no previous)

# collectors
gh_actions_minutes(empty_dir, '2026-06') = 0.0     (file missing → 0)
gh_actions_minutes(dir_with_ledger, '2026-06') = 123.0  (with JSON)
lfm_token_ledger(dir_with_ledger, '2026-06') = 4.25  (scalar value)
lfm_token_ledger(dir_with_dict_ledger, '2026-06') = 9.99  (dict {"usd":9.99})
du_storage(temp_dir) = 0.0  (empty dir)

# CostCard frozen
card = CostCard(repo='x', ci_minutes=0, llm_tokens_usd=0, storage_gb=0)
card.repo = 'y' → FrozenInstanceError cannot assign to field 'repo'
```

---

**End of audit. No files in pheno-cost-card/ were modified.**
