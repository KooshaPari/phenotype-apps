# AGENTS.md — Phenotype monorepo

**Date:** 2026-06-17 12:00 PDT
**Status:** ACTIVE (this file supersedes the prior FocalPoint template that lived here 2026-06-12 → 2026-06-15, and the 2026-06-15 18:42 PDT version that lived here 2026-06-15 → 2026-06-17)

---

## Project Overview

The `repos/` directory is a **monorepo of sub-repos** for the Phenotype organization (`KooshaPari` on GitHub). It is the top-level coordination point for ~50+ Rust crates, Python packages, Go modules, and TypeScript packages, organized as either git submodules, worktree containers, or as worktrees of other repos.

**It is NOT a single project.** It is a meta-repo that aggregates sibling repos. Each `pheno-*`, `phenotype-*`, `phenodocs-*`, etc. subdirectory is its own repository (or a worktree of one) with its own `Cargo.toml` / `pyproject.toml` / `go.mod` / `package.json` and its own release cadence.

---

## Stack

- **Languages:** Rust (primary), Python, Go, TypeScript, Swift (iOS)
- **Build systems:** Cargo, Cargo workspaces, Poetry/pyproject, Go modules, npm/pnpm, Xcode
- **Orchestration:** `just` (Justfile), sparse-checkout cone, git worktrees, forge/muse subagent dispatch

---

## Key Commands

```bash
# Repo state
git status --short                                    # All changes (incl. submodule pointer drift)
git log --oneline -10                                 # Last 10 commits on current branch
git rev-list --left-right --count main...HEAD         # Real divergence from main
git submodule status                                  # Submodule pointer health

# Sparse-checkout (this branch uses cone mode)
cat .git/info/sparse-checkout                         # Current cone pattern
git config core.sparseCheckout                        # true = sparse enabled
git config core.sparseCheckoutCone                    # true = cone mode

# Dispatch
gh --version && gh auth status                        # GitHub CLI (KooshaPari active as of 2026-06-15 18:40 PDT)
                                                      # Dmouse92 account still in keyring (read-only collaborator) — DO NOT push as Dmouse92.
                                                      # Owner account is KooshaPari — push target for ALL repos under github.com/KooshaPari/*
                                                      # If gh auth status shows Dmouse92 active, run: gh auth switch --user KooshaPari
curl -sf -m 3 http://localhost:20128/v1/models        # OmniRoute liveness
forge -p "<prompt>" -C /path/to/repo                  # Subagent dispatch (proven working 2026-06-15)
                                                      # (task tool had JSON errors; forge CLI works)

# Branch management
git worktree list                                     # Active worktrees
git stash list                                        # Stash backups
git branch --show-current                             # Current branch

# Audit doc + work DAG (live locations)
ls findings/71-pillar-2026-06-17*.md                  # 71-pillar industry-standard audit (this turn)
ls plans/2026-06-17-v7-dag-stable.md                  # v7 DAG (this turn; supersedes v6)
```

---

## Sub-repos at a Glance (sparse-checkout-visible, 2026-06-17)

### Active focus repos (5)
`AgilePlus`, `PhenoCompose`, `PlayCua`, `BytePort`, `nanovms` — coordinated via `chore/l5-87-focus-repo-specs-2026-06-11` branch.

### pheno-* family (22 visible)
- **Rust (11):** pheno-agents-md, pheno-cargo-template, pheno-cli-base, pheno-config, pheno-context, pheno-errors, pheno-flags, pheno-otel, pheno-port-adapter, pheno-tracing
- **Python (10):** pheno-cost-card, pheno-fastapi-base, pheno-llms-txt, pheno-mcp-router, pheno-prompt-test, pheno-pydantic-models, pheno-scaffold-kit, pheno-vibecoding-guard, pheno-worklog-schema
- **Go (1):** pheno-go-ctxkit
- **TypeScript (1):** pheno-zod-schemas (out of scope for cargo/pytest runs)
- **Container (1):** pheno-wtrees (git worktree container; not buildable)

See `L6_PHENO_REPOS_HEALTH_2026_06_14.md` for full health inventory (136 tests pass, 4 fail in pheno-agents-md). See `L6_PHENO_REPOS_HEALTH_2026_06_15_DELTA.md` for the 4 new crates added since. See `findings/71-pillar-2026-06-17.md` for the 71-pillar industry-standard audit (this turn).

### Submodule-style repos (~30 in submodules, e.g.)
- `AuthKit`, `Civis`, `Eidolon`, `Eventra`, `HeliosLab`, `KWatch`, `KodeVibe`, `KlipDot`, `McpKit`, `NetScript`, `PhenoKits`, `PhenoMCP`, `PhenoProc`, `Pyron`, `Tasken`, `Tracera`, `Tracely`, etc.

---

## Active ADRs (23 total, +ADR-024 this turn)

**2026-06-14 wave (6 ADRs at `docs/adr/2026-06-14/`):**

| ADR | Repo | Disposition |
|---|---|---|
| ADR-001 | NetScript | **DELETE** (Rust→Go port abandoned; use `phenotype-go-sdk/pkg/lexer` instead) |
| ADR-002 | KlipDot | KEEP-archived (governance: do not delete) |
| ADR-003 | McpKit | MERGE into `PhenoMCP` |
| ADR-004 | Metron | KEEP (sole prod Prometheus library) |
| ADR-005 | KodeVibe | KEEP (1.7k LOC Go engine; schema already in HexaKit) |
| ADR-006 | cheap-llm-mcp | archive verified (2 cherry-picks, merge `a1612805`) |

**2026-06-15 wave (11 ADRs at `docs/adr/2026-06-15/`):**

| ADR | Subject | Notes |
|---|---|---|
| ADR-007 | cheap-llm-mcp deprecation | Triggers W1-2 archive work |
| ADR-008 | dispatch-mcp as sole MCP server | consolidation decision |
| ADR-009..011 | (DAG-V5 reconciliation) | added 2026-06-15 by subagent |
| ADR-012 | `pheno-tracing` canonical across pheno-* repos | V5 SOTA sweep |
| ADR-013 | `pheno-mcp-router` substrate for pheno-mcp-* | V5 SOTA sweep |
| ADR-014 | Hexagonal L4 ports: `Port` trait + `Adapter` impl | V5 SOTA sweep |
| ADR-015 | V2 10-column WORKLOG.md schema (canonical) | V5 SOTA sweep (v2.1 bump pending — `device:` field) |
| ADR-016 | Fork-only-not-rewrite policy for SOTA libraries | V5 SOTA sweep |

**2026-06-15 evening wave (V6 closure, 6 ADRs at `docs/adr/2026-06-15/`):**

| ADR | Subject | Notes |
|---|---|---|
| ADR-017 | `settly-*` archive — full deprecation | V6 Track 5 closure |
| ADR-018 | PRCP pattern (Polyglot Reuse via Canonical Ports) | V6 Track 5 closure |
| ADR-019 | `pheno-vessel-*` full deprecation | V6 Track 5 closure |
| ADR-020 | `pheno-types-*` full deprecation | V6 Track 5 closure |
| ADR-021 | `pheno-profiling` replaces `Profila` | V6 Track 5 closure |
| ADR-022 | Config consolidation — two-crate canonical split | Subagent-B 11-PR plan |
| **ADR-023** | **Agent-effort governance — device + dogfood + app substrate policy** | **L5-101, 2026-06-15** — see [§ App-level repo triage & substrate](#app-level-repo-triage--app-substrate-placement-adr-023) below |

**2026-06-17 wave (this turn):**

| ADR | Subject | Notes |
|---|---|---|
| **ADR-024** | **71-pillar industry-standard audit framework (L1-L71, 9 domains)** | **L5-102, 2026-06-17** — see `findings/71-pillar-2026-06-17-schema.md` |
| **ADR-025** | **ADR-015 v2.1 worklog schema bump (11th column `device:`)** | **L5-103, 2026-06-17** — supersedes v2.0; deprecation 2026-06-22 |

---

## Wave Plan (v7 — current, supersedes v6)

See `plans/2026-06-17-v7-dag-stable.md`. **~7 tracks, 30+ PRs, orchestrator + parallel forge subagent dispatch.**

- Track 1: Triage (P0, ~5min, this turn) — drop empty commits, drop stashes, commit meta-bundle, refresh governance docs
- Track 2: 5 PR reviews (P1, parallel, ~10min) — PRs #129-#133 from W5 batch
- Track 3: 71-pillar audit (P1, ~30min) — schema + probe + score + render + crosswalk
- Track 4: ADR-015 v2.1 schema bump (P0, due 2026-06-22, ~15min)
- Track 5: HwLedger reclassification (P0, ADR-023 Rule 3 deliverable, ~30min)
- Track 6: Rebase + push cleaned branch (~5min)
- Track 7: Work DAG maintenance (ongoing) — keep `findings/71-pillar-2026-06-17*.md` and `plans/2026-06-17-v7-dag-stable.md` updated

---

## Conventions

- **Branch naming:** `chore/<req-id>-<slug>-<date>` for chore work; `feat/<req-id>-<slug>-<date>` for features
- **Commit messages:** Conventional Commits (`feat:`, `fix:`, `chore:`, `docs:`, `refactor:`, `test:`, `build:`, `ci:`) with optional scope
- **PR labels:** `governance` for cleanup, `L<n>-#<n>` for tracking against DAG level
- **SOTA artifacts:** `findings/`, `plans/`, `worklogs/`, `docs/adr/<date>/`
- **Meta-bundle for a release-ready crate:** `AGENTS.md` + `llms.txt` + `WORKLOG.md` + `CHANGELOG.md` + `LICENSE-MIT`

---

## App-level repo triage & app substrate placement (ADR-023)

Source of truth: `docs/adr/2026-06-15/ADR-023-agent-effort-governance.md`.
Decision log: `findings/2026-06-15-L5-101-app-governance.md`.
Worklog: `worklogs/L5-101-app-governance-2026-06-15.json`.

### Device-fit gate

The MacBook is **not** a heavy-work device. Heavy work is defined as anything that requires a full `cargo test --workspace` against a multi-100-crate workspace, an iOS Simulator boot, a Docker-in-Docker test, a Unity/Unreal editor head, or any single build/test cycle > 10 min wall on the MacBook. Heavy work runs on a self-hosted runner or a dispatched subagent (`device: heavy-runner`); the MacBook is reserved for planning, ADR-writing, small focused PRs, code review, and dogfooding (`device: macbook`). The `device:` field is in the worklog v2.1 schema (ADR-015 bump pending — see ADR-025).

### Active / Paused app-level repos (triage by dogfood use)

| Repo         | Bucket         | Allowed work                                                                                            |
| :----------- | :------------- | :------------------------------------------------------------------------------------------------------ |
| `Civis`      | **ACTIVE**     | Any. Full SWE process.                                                                                  |
| `focalpoint` | **PAUSED**     | Read-only. The prior AGENTS.md template is shelved.                                                     |
| `Dino`       | **CONDITIONAL** | Engine / non-frontend only (heavy visual engine, asset pipeline, deterministic sim). No UI / HUD / UX work right now. |
| `WSM`        | **CONDITIONAL** | None right now. Re-evaluate when an active consumer appears.                                            |
| `QuadSGM`    | **PAUSED**     | Read-only.                                                                                              |
| `AtomsBot*`  | **PAUSED (capstone)** | Read-only as a *target* of new work. **May be legally mined** (code, concepts, schema, docs, tests) — capstone project's sponsor is not in good standing; the public repo is fair-game reference material. |
| `HwLedger` + every other app-level repo not in this list | **RECLASSIFY** (default PAUSED) | Underlying parts to be moved to one of `pheno-*-lib` / `phenotype-*-sdk` / `phenotype-*-framework` / federated service per Rule 3 below. |

A new repo defaults to **PAUSED** until it is added to this table with a bucket. A bucket change requires a one-line worklog entry (`bucket_change: from=... to=... reason=...`).

### App substrate placement (no "random `phenoShared`")

When an app-level repo needs a reusable underlying capability, that capability is placed in **exactly one** of:

| Substrate type             | When to use                                                                                                  | Examples in fleet                              |
| :------------------------- | :----------------------------------------------------------------------------------------------------------- | :--------------------------------------------- |
| **`pheno-*-lib` / `pheno-*-core`** | Pure reusable library; language-specific; single concern, single crate.                                     | `pheno-config`, `pheno-context`, `pheno-port-adapter` |
| **`phenotype-*-sdk`**        | Cross-language SDK; stable public API; polyglot facade.                                                      | `phenotype-go-sdk`, `phenotype-python-sdk`     |
| **`phenotype-*-framework`**  | Inversion-of-control framework; opinionated lifecycle, ports, adapters, conventions.                        | `phenotype-hub`, `phenotype-bus`               |
| **Federated service**        | Stateful, long-running, independently scalable.                                                              | `phenoMCP`, `phenoObservability`, `phenoEvents` |

The "random `phenoShared`" pattern (and `crates/`, `libs/`, per-app `lib/`) is **forbidden** for new shared code. Existing "random `phenoShared`" placements are migrated per-capability; tracked in the L6 health-audit delta.

### Quality bar for new substrate (Rule 3.1)

Every new `pheno-*-lib`, `phenotype-*-sdk`, `phenotype-*-framework`, or federated service ships with:

- **Spec** (`SPEC.md` or equiv) — 1-page max.
- **Docs** (`README.md` + 1 concept doc) — what, when, when **not**, 5-line quickstart.
- **Test matrix** — unit + integ minimum; e2e + perf + chaos strongly preferred for the 4 fleet-critical substrates (config, tracing, MCP-router, observability).
- **Observability** — OTLP export via `pheno-tracing` (ADR-012), info-level minimum.
- **Coverage gate** — 80 % lib/SDK, 70 % framework, 60 % federated service.
- **CI gate** — `pheno-ci-templates` runs the test matrix, coverage gate, OTLP smoke test.
- **Worklog v2.1** — including the new `device:` field.

The goal: **HITL-less dev from base intent**. A one-line intent ("I need a `Config` struct for my service that reads from env and a TOML file, with a 12-factor cascade") produces a PR that already has spec + docs + tests + coverage + observability + CI gate, without the human specifying each one.

---

## 71-pillar audit (this turn)

See `findings/71-pillar-2026-06-17-schema.md` for the full schema doc (industry references, scoring rubric, pillar definitions). See `findings/71-pillar-2026-06-17.md` for the latest scorecard across 10 existing repos. See `findings/71-pillar-2026-06-17-mapping.md` for the L1-L30 → L1-L71 crosswalk (so the older 30-pillar audit at `findings/30-pillar-2026-06-16.md` is not orphaned).

**Domains (9 total, 71 pillars):**

- **Architecture (AX)** L1-L12 (12)
- **Performance** L13-L19 (7)
- **Quality / Correctness** L20-L27 (8)
- **Developer Experience (DX)** L28-L37 (10)
- **User Experience (UX)** L38-L45 (8)
- **Security** L46-L55 (10)
- **Observability & Ops** L56-L63 (8)
- **Documentation & SSOT** L64-L68 (5)
- **Governance & Sustainability** L69-L71 (3)

**Industry references:** AWS Well-Architected Framework, Azure Well-Architected Framework, Google Cloud Architecture Framework, ISO 25010, OWASP ASVS, NIST SSDF, Microsoft SDL, DORA 2023 capabilities, Google SRE Book, CNCF Cloud Native Definition, OpenSSF Best Practices, Divio documentation system.

**Scoring:** 0-3 per pillar per repo (0=absent, 1=minimal, 2=adequate, 3=strong/SOTA). N/A=3 (per `audit-30-pillar-template.md` rule) for UI pillars (L40 i18n, L41 a11y) on headless backend/CLI libraries.

**Refresh cadence:** weekly (every Monday 09:00 PDT). Owner: worklog-schema circle. Diff against previous week is logged in `findings/71-pillar-{date}-delta.md`.

---

## Stale / warnings

- **Root `Cargo.toml` workspace** lists `crates/phenotype-error-core` as a member but the directory does NOT exist on this branch's sparse-checkout cone. **This is an intentional sparse-checkout artifact**, not a real bug. The crate exists in `phenoShared/crates/`, `FocalPoint/crates/`, `HexaKit/crates/`, `ResilienceKit/rust/`, etc. as workspace-local sub-paths.
- **Melosviz submodule** is `-dirty` (3 uncommitted files in the submodule). Do not commit the parent pointer until the submodule is clean.
- **Working tree shows 170+ "M" entries** for submodules — these are submodule pointer drifts from prior sessions, not modifications in this repo.
- **2 unapplied stashes (pre-2026-06-17)** — DROPPED this turn (WIP pheno-tracing fix already in HEAD via W5 batch).
- **4 empty `gate1-0..3` local branches** — DELETED this turn (probe commits, no content, not on any pushed branch).
- **ADR-015 v2.1 deprecation in 5 days** (2026-06-22) — see ADR-025 for the bump.

---

## Related

- `STATUS.md` — current state of the monorepo
- `SSOT.md` — single source of truth for repo conventions
- `SPEC.md` — top-level specification
- `L6_PHENO_REPOS_HEALTH_2026_06_14.md` — health inventory of pheno-* crates
- `L6_PHENO_REPOS_HEALTH_2026_06_15_DELTA.md` — 2026-06-15 delta
- `findings/30-pillar-2026-06-16.md` — 30-pillar audit (superseded by 71-pillar)
- `findings/71-pillar-2026-06-17-schema.md` — 71-pillar schema doc
- `findings/71-pillar-2026-06-17.md` — 71-pillar scorecard (live)
- `findings/71-pillar-2026-06-17-mapping.md` — L1-L30 → L1-L71 crosswalk
- `FLEET_DAG_v3.md` — FLEET DAG shape (180 tasks, all done)
- `plans/2026-06-15-v6-dag-stable.md` — superseded v6 plan
- `plans/2026-06-17-v7-dag-stable.md` — current v7 plan (this turn)
- `findings/SESSION_STATUS_2026_06_15_0105.md` — last session status (pre-W5-batch)
- `findings/2026-06-15-L5-101-app-governance.md` — ADR-023 decision log
- `findings/2026-06-17-L5-102-71-pillar-audit.md` — ADR-024 decision log (this turn)
- `findings/2026-06-17-L5-103-adr-015-v2-1.md` — ADR-025 decision log (this turn)
