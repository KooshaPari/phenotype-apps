# pheno-capacity → collection-repo merge plan (L5-117)

**Date:** 2026-06-19
**Author:** fleet-arch subagent
**Status:** PROPOSAL — review pending
**Scope:** single migration, no code changes
**Tracks:** SOTA pattern (`phenotype-gfx` 4-repo absorb, L5-109..L5-112) applied to `pheno-capacity`
**Authority chain:** ADR-023 (substrate placement), ADR-035A (extraction rationale), ADR-ECO-014 (gateway charter), this doc L5-117 (home selection)

---

## 1. Context

`KooshaPari/pheno-capacity` is a standalone Rust math library created
2026-06-19 (~24 h old at time of writing). v0.2.0 (L5-115, 2026-06-18)
ships 60 unit tests + 6 doc tests, `no_std` compatible, zero
dependencies, deterministic pure functions. Five source files
(`lib.rs`, `math.rs`, `estimate.rs`, `attention.rs`, `policy.rs`)
totaling ~80 KB. Full meta-bundle present: `AGENTS.md`, `CHANGELOG.md`,
`Cargo.toml`, `LICENSE-{MIT,APACHE}`, `README.md`, `SECURITY.md`,
`WORKLOG.md`, `llms.txt`, `llvm-cov.toml`, `.github/{CODEOWNERS,
workflows/ci.yml}` (3 jobs: test, coverage ≥ 80 %, `no_std` build
smoke), `docs/SPEC.md` (4.9 KB), `docs/methodology.md` (11 KB).
The crate is `pheno-*-lib` per ADR-023, lib tier quality bar.

Extracted from HwLedger per ADR-035A (L5-105) — the math was previously
`apps/streamlit/lib/cost_model.py` + `apps/streamlit/lib/perf_model.py`
(Python, git @ `8bf878ca`). The crate is intentionally a **standalone
package** (empty `[workspace]` table in `Cargo.toml:31-35`):

> "This crate is intentionally NOT a member of the root monorepo
> workspace (which contains 3000+ files including submodules, worktrees,
> and 50+ sub-repos). Consumers add it to their own workspace or depend
> on it via path/git dep. This mirrors the standalone pattern used by
> `pheno-config`, `pheno-errors`, `pheno-context`, `pheno-port-adapter`."

### User directive (2026-06-19, paraphrased)

> "pheno-capacity exists but must now find its way to a collection
> repo, NOT a new repo. I do not want new repos unless it provably
> prevents further new repos and/or cuts down on existing."

### Task constraints

- Pick an **existing** collection repo as the new home.
- 3 candidates were pre-screened by the orchestrator:

  | Candidate | Path | Verdict |
  |---|---|---|
  | **A** | `KooshaPari/phenotype-gateway` (polyglot monorepo, `spikes/rust/<name>/` subcrate pattern) | **STRONGEST FIT** (see §3) |
  | B | local `repos/pheno-capacity/` next to `pheno-config/`, `pheno-context/` (12-crate substrate pattern) | Fits the `pheno-*` family but contradicts the explicit "standalone, not a workspace member" intent in the existing `Cargo.toml` |
  | C | `KooshaPari/bifrost` as sub-crate | Worst fit (bifrost is itself a gateway submodule, not a collection repo) |

- This plan covers (1) the home recommendation, (2) the migration
  steps, (3) the risk analysis, (4) the effort estimate, and (5) the
  open questions. **No code changes are made by this plan.**

---

## 2. Pattern reference — `KooshaPari/phenotype-gfx`

`KooshaPari/phenotype-gfx` is the proven collection-repo absorb
pattern. It absorbed 4 standalone repos in the L5-109..L5-112 wave
(2026-06-18): `phenotype-voxel`, `phenotype-terrain`, `phenotype-water`,
`phenotype-postfx`. All source repos are now archived; the absorbing
PRs merged to `main` of `phenotype-gfx` with single commits
(`9a7c05a` for voxel, parallel commits for the others). Verdict
captured per-source in `phenotype-gfx/findings/2026-06-16-{voxel,
terrain,water}-block-c.md` and `2026-06-18-postfx-block-c.md`;
integration log in `findings/2026-06-18-sister-repo-block-c-summary.md`.

### 2.1 The absorb pattern (single Rust core + thin FFI edges, ADR-004)

| Element | How `phenotype-gfx` does it | `pheno-capacity` analogue |
|---|---|---|
| Repo shape | Single Cargo package, **not a Cargo workspace**. Empty `[workspace]` table in `Cargo.toml:25-26`. Multi-module, single-crate core. | Same: `pheno-capacity` already has an empty `[workspace]` table (Cargo.toml:31-35) and is single-crate. **Already aligned.** |
| Source location | Each absorbed repo becomes a sub-module of the single crate: `phenotype-gfx/src/voxel/`, `src/terrain/`, `src/water/`, `src/postfx/`. Top-level siblings (e.g. `lod.rs`, `streaming.rs`, `voxelizer.rs`) hold cross-cutting algorithms. | `pheno-capacity` has 5 sibling modules (`math`, `attention`, `policy`, `estimate`, `lib`). Two absorb options: (a) inline as `spikes/rust/capacity/src/{math,attention,policy,estimate,lib}.rs` (sibling-files), or (b) inline as `spikes/rust/capacity/src/pheno_capacity.rs` (single-file mod), or (c) preserve the 5-file structure as `spikes/rust/capacity/src/pheno_capacity/{math,attention,policy,estimate,lib}.rs` (sub-module). **Recommended: option (c)** — preserves the v0.2.0 module boundaries. |
| Versioning | `VERSION.toml` is the umbrella manifest. Each module pinned: `voxel = "0.1.0"`, `terrain = "0.1.0"`, etc. | Recommended: add `spikes/rust/capacity/VERSION.toml` (or extend an existing one) with `pheno_capacity = "0.2.0"`. |
| Re-exports / compat | `pub use voxel as kernel; pub use voxel::*;` re-exports the absorbed module under its old name (`phenotype_voxel::Chunk` API still works). | Not needed (no consumer yet depends on `pheno_capacity::Chunk`-style re-exports; the v0.2.0 API is the canonical surface). |
| Specs / docs | Each absorbed repo's `docs/` is migrated into the umbrella's `docs/` (e.g. `phenotype-gfx/docs/spec/`, `phenotype-gfx/docs/UPSTREAM.md`). | `pheno-capacity/docs/SPEC.md` + `docs/methodology.md` migrate to `phenotype-gateway/docs/spikes/capacity/` (or similar). |
| CI | `phenotype-gfx` inherits a single CI matrix. | The gateway does NOT currently have a Cargo CI workflow (Taskfile.yml is PowerShell + bash smoke for Go submodules). A new `.github/workflows/cargo.yml` is required (see §4 step 5). |
| Governance | `CODEOWNERS` updated; per-source-repo audit findings preserved in `phenotype-gfx/findings/`. | Per-source audit (this doc) + a `2026-06-19-L5-117-capacity-absorption.md` finding in `phenotype-gateway/findings/` (or umbrella `repos/findings/`). |
| Archive | Source repo set to **archived** (read-only) once PR merges. | `KooshaPari/pheno-capacity` archived after the absorb PR merges (90-day GitHub retention before hard-delete is possible). |

### 2.2 What was absorbed (voxel as the closest analogue)

`phenotype-voxel` was the largest of the 4 absorbs (7,704 lines,
94 tests → 122 on the `feat/port-sister-repos-2026-06-18` branch).
The `findings/2026-06-16-voxel-block-c.md` doc re-issued the
verdict on 2026-06-18 to point at the absorb rather than the
original "umbrella-sister" plan. Same pattern is appropriate for
`pheno-capacity`: the "umbrella-sister" framing (separate repo +
git dep) is **superseded** by the "absorb into the gateway"
decision captured here.

### 2.3 Proven roll-out checklist (4-step from `2026-06-18-L5-109-4-repo-retirement.md`)

1. Per-source absorption PR with per-file log in the PR body.
2. CI matrix update (the new crate is now in the umbrella's CI).
3. `CHANGELOG.md` bump + ADR reference.
4. Source repo `gh repo archive` (read-only marker); 90-day
   retention before GitHub UI "Delete this repository" is possible
   (the active `gh` token has scopes `'gist', 'read:org', 'repo',
   'workflow'` — **no `delete_repo`**, archive is the terminal state).

This same 4-step pattern + the gateway-specific Cargo workspace
shim (§4) is the recommended roll-out for `pheno-capacity`.

---

## 3. Home recommendation — **A: `KooshaPari/phenotype-gateway` at `spikes/rust/capacity/`**

### 3.1 Why A wins

| Dimension | A (phenotype-gateway, spikes/rust/capacity/) | B (local monorepo, repos/pheno-capacity/) | C (bifrost as sub-crate) |
|---|---|---|---|
| **Domain fit** | Gateway is "agent API + LLM proxy + enterprise gateway + router revamp" per its README. Capacity math answers "does LLM X fit on device Y?" — directly relevant to LLM proxy fit-checks, intelligent routing, and provider capacity planning. The router spike (`spikes/rust/router/`) is in the same parent already. | Local monorepo is meta-coordination only; no production code is built there. Hosting a published lib at this layer mixes concerns. | Bifrost is a single-purpose enterprise gateway (submodule of A). Capacity math is foreign to its charter. |
| **Pattern fit** | `spikes/rust/router/` is an existing subcrate pattern (Cargo.toml, src/lib.rs, README.md, .gitkeep parent). Adding a sibling `spikes/rust/capacity/` is a literal copy of the pattern, with capacity as the second Rust spike. | `repos/pheno-config/`, `repos/pheno-context/` etc. are siblings but each is also a *separately published* crate on GitHub (`KooshaPari/pheno-config` etc.). The local dir is a checkout, not the canonical home. The published canonical is the GitHub repo. | bifrost does not have a sub-crate pattern; it is a single Go module. |
| **No new repo** | ✅ Yes — pre-existing repo. | ✅ Yes — uses existing local monorepo. | ✅ Yes — pre-existing repo. |
| **No new repos downstream** | Gateway absorbs; if HwLedger Phase 2 or `phenotype-mcp-router` later need capacity, they import from the gateway (or the still-published `pheno-capacity = "0.2"` if we keep the crates.io path). | Each consumer still needs to add a `pheno-capacity` Cargo dep; no consolidation. | Same as A. |
| **Cargo shape compatibility** | Gateway's `spikes/rust/router/Cargo.toml` is a sub-crate (not a workspace member); a `spikes/rust/capacity/Cargo.toml` matching that pattern preserves the spike-as-evaluation context. | The `pheno-capacity/Cargo.toml` already has an empty `[workspace]` table that says "intentionally NOT a member of the root monorepo workspace" — adding to the local monorepo would force a workspace re-evaluation, contradicting the author's explicit choice. | bifrost is a Go submodule; mixing Rust sub-crate is type-incoherent. |
| **Discoverability for the gateway's actual consumers** | `phenotype-mcp-router`, HwLedger Phase 2, and the future "router revamp" all live in the github.com/KooshaPari/ namespace. They can `pheno_capacity = { path = "../phenotype-gateway/spikes/rust/capacity" }` or rely on the still-published crate. | Discoverable via the monorepo's sparse-checkout cone, but not via the github.com namespace. | Worst — bifrost is a submodule; consumers do not depend on a submodule path. |
| **Pre-existing precedent in the fleet** | The `phenotype-gfx` 4-repo absorb is the exact same pattern; this is "do the same thing in the gateway". | None — pheno-* are siblings, but they are not an "absorb into a parent" pattern. | None. |
| **New repo count: net delta** | 0 new; 1 archived (`KooshaPari/pheno-capacity` → archived). | 0 new; 1 archived. | 0 new; 1 archived. |
| **Future expansion** | Gateway can absorb future Rust math (e.g. `pheno-throughput` per `pheno-capacity/docs/SPEC.md` §7) at the same `spikes/rust/<name>/` path. | Less clear. | Least extensible. |

### 3.2 The pairing argument (router + capacity = the gateway's H6+ vision)

`spikes/rust/router/` is the **H13** scaffold (per its README.md):
replacing the interim `OmniRoute` TypeScript router with a Rust
implementation. `spikes/rust/capacity/` would be the **H14+**
scaffold: a Rust capacity math lib feeding the router with
"will this model fit on this device?" decisions. Together they
become the foundation for "intelligent LLM routing" in the
gateway's H6+ roadmap. The two spikes are complementary, not
duplicative:

- **Router** — given a request, which model endpoint serves it?
- **Capacity** — given a model + a target device, does it fit?

The pairing is exactly what the gateway will need to do
provider-aware fit-checked routing (e.g. "route to deepseek-r1
because the request needs a 70B model and only 2x A100-80 has
the VRAM").

### 3.3 Naming

| Asset | Recommended value | Rationale |
|---|---|---|
| GitHub repo | `KooshaPari/phenotype-gateway` (unchanged) | Pre-existing; no new repo. |
| Path within repo | `spikes/rust/capacity/` | Matches the `spikes/rust/router/` parent. |
| Cargo crate name | `phenotype-capacity-spike` | Mirrors `phenotype-router-spike` (the `phenotype-` prefix for collection-anchored crates, `-spike` suffix for evaluation-stage crates). |
| Rust library name | `phenotype_capacity_spike` | `Cargo.toml:9 [lib] name = "..."` follows Cargo's snake_case convention. |
| Initial version | `0.0.0`, `publish = false` | Mirror router spike's Cargo.toml (publish=false; not on crates.io from the gateway; consumers can use `path = "..."` or `git = "https://github.com/KooshaPari/phenotype-gateway"`). |
| `pheno_capacity` import path (for consumers) | **Dual path during transition**: (a) `pheno_capacity = "0.2"` from crates.io (the existing published crate, kept as a stand-alone), (b) `phenotype_capacity_spike = { path = "../phenotype-gateway" }` for gateway-local consumers. **Decision deferred to Open Question OQ-1** — see §7. |
| VERSION manifest | `spikes/rust/capacity/VERSION.toml` (or extend the gateway's `VERSION.toml` if it has one — it does not, so a new one) | Mirrors `phenotype-gfx/VERSION.toml`. |

### 3.4 What stays in `KooshaPari/pheno-capacity` after the absorb

Until the absorb PR merges, **`KooshaPari/pheno-capacity` remains the
canonical published source**. Recommended sequence:

1. Open the absorb PR against `phenotype-gateway:master`.
2. Cut a `pheno-capacity` v0.2.x patch release (or v0.3.0) **before**
   archiving — captures the published artifact for crates.io consumers.
3. Merge absorb PR.
4. Add a `DEPRECATION.md` to `KooshaPari/pheno-capacity` pointing at
   `phenotype-gateway/spikes/rust/capacity/`.
5. Archive `KooshaPari/pheno-capacity` (read-only).
6. (Optional, 90+ days later) Hard-delete via GitHub UI.

The published crates.io path is **not** removed by this plan — it
can stay as a stable shim that re-exports from the gateway (or as
a frozen artifact). See OQ-2.

---

## 4. Migration plan — 8 steps

The plan executes in one PR against `phenotype-gateway:master`,
plus a follow-up archive action on `KooshaPari/pheno-capacity`.
No code in `pheno-capacity` itself is modified — the migration is
purely a copy + archive.

### Step 1 — Fork the absorb PR branch

```bash
# Local: clone + branch
gh repo clone KooshaPari/phenotype-gateway /tmp/pg-mig
cd /tmp/pg-mig
git checkout -b feat/l5-117-absorb-pheno-capacity-2026-06-19
```

Base: `KooshaPari/phenotype-gateway:master` (HEAD verified
2026-06-19 03:44:48Z). Confirm: `git rev-parse HEAD` ==
the `updated_at` from the repo metadata (§3.4).

### Step 2 — Scaffold the subcrate

Copy the `spikes/rust/router/` shape into `spikes/rust/capacity/`:

```bash
mkdir -p spikes/rust/capacity/src
```

Create `spikes/rust/capacity/Cargo.toml` mirroring the router
spike's `Cargo.toml` (120 bytes; just package name, edition, lib
path, publish=false):

```toml
[package]
name = "phenotype-capacity-spike"
version = "0.0.0"
edition = "2021"
publish = false

[lib]
path = "src/lib.rs"
```

Create `spikes/rust/capacity/README.md` mirroring the router
spike's README (558 bytes), with the analogous "interim MVP:
KooshaPari/pheno-capacity" framing.

Create `spikes/rust/capacity/VERSION.toml`:

```toml
# Absorbed from KooshaPari/pheno-capacity (v0.2.0, L5-115) on
# 2026-06-19 per L5-117. Single source of truth for the absorbed
# version going forward.
pheno_capacity = "0.2.0"
```

### Step 3 — Ingest the `pheno-capacity` source tree

The source files are 5 small files (~80 KB total). Two layout
options:

- **Option (a) — flatten**: copy all 5 `.rs` files directly into
  `spikes/rust/capacity/src/` (`lib.rs`, `math.rs`, `attention.rs`,
  `policy.rs`, `estimate.rs`). The new `src/lib.rs` is a
  re-export: `pub mod math; pub mod attention; pub mod policy;
  pub mod estimate; pub use math::...; pub use attention::...; etc.`
  (lifting all the `pub use` lines from the source `lib.rs`).
- **Option (b) — preserve module structure**: copy into
  `spikes/rust/capacity/src/pheno_capacity/{math,attention,policy,
  estimate}.rs` + `src/pheno_capacity/mod.rs` (= source `lib.rs`).
  The new `src/lib.rs` is a thin re-export module:
  `pub mod pheno_capacity; pub use pheno_capacity::*;`.

**Recommended: option (b)** — preserves the v0.2.0 module
boundaries, makes future upstream syncs (if `pheno-capacity` ever
gets a v0.3.0 from a side branch) a clean `git subtree` operation,
and matches the `phenotype-gfx/src/voxel/{mod.rs, ...}` pattern
(where the absorbed crate's `mod.rs` retains its own
re-export surface). The new crate is `phenotype_capacity_spike`
externally; the internal module is `pheno_capacity` to keep the
upstream file paths and rustdoc anchors identical.

### Step 4 — Ingest the meta-bundle

Copy into `spikes/rust/capacity/`:

| Source file (from `KooshaPari/pheno-capacity`) | Destination (under `spikes/rust/capacity/`) |
|---|---|
| `Cargo.toml` (the `[package]` block only; merge with the new `Cargo.toml` from step 2) | Merged into `spikes/rust/capacity/Cargo.toml` (preserve `description`, `license`, `repository`, `keywords`, `categories`, `rust-version`, `readme`). Drop the empty `[workspace]` table. |
| `AGENTS.md` | `spikes/rust/capacity/AGENTS.md` (update header to reflect new home). |
| `CHANGELOG.md` | `spikes/rust/capacity/CHANGELOG.md` (prepend a "## Absorbed into phenotype-gateway 2026-06-19" section with L5-117 reference). |
| `README.md` | `spikes/rust/capacity/README.md` (update the "See also" section to point at the gateway; keep the rest verbatim). |
| `WORKLOG.md` | `spikes/rust/capacity/WORKLOG.md` (prepend a row for L5-117 with action "absorb into phenotype-gateway"; mark the previous L5-105 row with the new home). |
| `llms.txt` | `spikes/rust/capacity/llms.txt` (update the source location to "spikes/rust/capacity/ in KooshaPari/phenotype-gateway"). |
| `LICENSE-MIT` + `LICENSE-APACHE` | `spikes/rust/capacity/LICENSE-{MIT,APACHE}` (verbatim; the gateway LICENSE is unchanged, dual-license for this sub-crate only). |
| `SECURITY.md` | `spikes/rust/capacity/SECURITY.md` (verbatim; SECURITY contact paths unchanged). |
| `llvm-cov.toml` | `spikes/rust/capacity/llvm-cov.toml` (verbatim). |
| `.github/CODEOWNERS` | Merged into the gateway's `.github/CODEOWNERS` (add `spikes/rust/capacity/ @KooshaPari`). |
| `docs/SPEC.md` + `docs/methodology.md` | `spikes/rust/capacity/docs/{SPEC,methodology}.md` (verbatim; update header to reflect the new home). |
| `.github/workflows/ci.yml` (3 jobs: test, coverage, no_std) | **Translated** to a gateway-anchored CI workflow (see step 5). |

### Step 5 — CI integration

The gateway currently has only a PowerShell + bash `smoke` task
(`Taskfile.yml:1-12`) that runs `go build` against the
`cliproxyapi-plusplus` submodule. It has **no Rust CI yet**. Two
options:

- **Option (a) — add a new `cargo.yml` workflow**: creates
  `.github/workflows/cargo.yml` with the same 3 jobs as the
  pheno-capacity CI (test, coverage ≥ 80 %, no_std build smoke).
  Triggered on changes to `spikes/rust/**`. The router spike
  benefits retroactively (it inherits CI for free once any
  `spikes/rust/*` change triggers the workflow).
- **Option (b) — extend the existing `ci.yml` workflow** (if the
  gateway has one — needs verification; the repo metadata says
  `.github/` exists but the contents were not enumerated in this
  scan).

**Recommended: option (a)**. The pheno-capacity CI is
self-contained, no external Cargo workspace coupling (the
`no_std` smoke build creates a `target/no_std_check/` sub-crate
that depends on `pheno-capacity = { path = "../.." }`; the
new path would be `path = "../../../.."` from the gateway
root, which is awkward but works).

### Step 6 — Documentation cross-links

In the gateway root `README.md`, add a row to the "spikes/"
sub-table for `spikes/rust/capacity/` pointing at the absorb
PR. In `docs/SPEC.md` (the gateway's unified spec, if it
exists — verify in the merge), add a "Capacity math" section
referencing the absorbed subcrate. In
`docs/UPSTREAM.md` (if it exists), add a row for
`pheno-capacity` pointing at the absorbed location.

### Step 7 — Open the absorb PR

```bash
git add spikes/rust/capacity/
git add .github/CODEOWNERS
git add README.md
# (if applicable) git add docs/SPEC.md docs/UPSTREAM.md
git add .github/workflows/cargo.yml
git commit -m "feat(spikes/capacity): absorb pheno-capacity v0.2.0 (L5-117, ADR-035A)

Pattern reference: phenotype-gfx 4-repo absorb (L5-109..L5-112).
Home: spikes/rust/capacity/ in phenotype-gateway.
Absorbed: 5 .rs files (~80 KB) + meta-bundle + CI.

Migration notes: see repos/findings/2026-06-19-L5-117-pheno-capacity-collection-merge.md
Migration PR: <link>
"
gh pr create --base master --head feat/l5-117-absorb-pheno-capacity-2026-06-19 \
  --title "feat(spikes/capacity): absorb pheno-capacity v0.2.0 (L5-117)" \
  --body "## Migration matrix

| Source | Destination | Notes |
|---|---|---|
| src/{lib,math,attention,policy,estimate}.rs | spikes/rust/capacity/src/pheno_capacity/*.rs | 5 files, ~80 KB, verbatim |
| Cargo.toml | spikes/rust/capacity/Cargo.toml (merged) | description, license, keywords preserved |
| README.md, CHANGELOG.md, WORKLOG.md, AGENTS.md, llms.txt | spikes/rust/capacity/* | verbatim with home-update |
| docs/SPEC.md, docs/methodology.md | spikes/rust/capacity/docs/ | verbatim |
| LICENSE-{MIT,APACHE} | spikes/rust/capacity/LICENSE-{MIT,APACHE} | verbatim |
| SECURITY.md, llvm-cov.toml | spikes/rust/capacity/{SECURITY.md,llvm-cov.toml} | verbatim |
| .github/workflows/ci.yml (3 jobs) | .github/workflows/cargo.yml (new) | rewritten for gateway paths |

## Integration verification

- (a) All 60 unit tests pass under `cargo test --all-features`
  from the gateway root.
- (b) All 6 doc tests pass under `cargo test --doc --all-features`.
- (c) `cargo llvm-cov --all-features --summary-only` reports
  ≥ 80 % line coverage for the `phenotype-capacity-spike` crate
  (the lib tier per ADR-023 Rule 3.1).
- (d) `cargo +stable build --release` in
  `spikes/rust/capacity/target/no_std_check/` (the structural
  `no_std` smoke consumer) exits 0.
- (e) `cargo fmt --all -- --check` and `cargo clippy --all-targets
  -- -D warnings` clean.

## Policy notes

- (1) `pheno-capacity` source repo retains its published crates.io
  artifact as a stable shim. See OQ-1.
- (2) `pheno-capacity` is archived (read-only) after this PR merges
  per the 4-repo retirement pattern (L5-109).
- (3) Substrate tier is preserved: this remains a `pheno-*-lib`
  per ADR-023, hosted inside a larger collection repo.

## Cross-references

- Migration plan: repos/findings/2026-06-19-L5-117-pheno-capacity-collection-merge.md
- ADR-035A: HwLedger reclassification (L5-105, 2026-06-18)
- ADR-004: single-core + thin FFI edges (phenotype-gfx pattern)
- ADR-023: substrate placement rules
- ADR-ECO-014: gateway charter (the absorbing repo)
- Pattern reference: L5-109..L5-112 (phenotype-gfx 4-repo absorb)"
```

### Step 8 — Archive source repo + ADRs

Once the PR merges (and `cargo test` + coverage are green on
`phenotype-gateway:master`):

```bash
gh repo archive KooshaPari/pheno-capacity --yes
```

Then drop a one-line `DEPRECATION.md` into the archived repo
pointing at the new home (via the GitHub UI; the active `gh`
token cannot push to archived repos).

Finally, add **ADR-036** to `repos/docs/adr/2026-06-19/` (or
the next batch) recording:

> **ADR-036** — `pheno-capacity` absorb into `phenotype-gateway` (L5-117).
> Source repo archived; absorbed as `phenotype-gateway/spikes/rust/capacity/`
> (crate name `phenotype-capacity-spike`). The gateway is now the
> canonical home for the LLM capacity math lib; the published
> `pheno-capacity = "0.2"` crates.io path remains a stable shim
> pending OQ-1 resolution.

---

## 5. Risk analysis

### 5.1 Technical risks

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| **R1 — `no_std` build smoke fails on the gateway path** | Low | Medium | The `no_std` check consumer in `.github/workflows/cargo.yml` uses `pheno-capacity = { path = "../.." }` from `target/no_std_check/`. The new path is from `spikes/rust/capacity/target/no_std_check/` to `spikes/rust/capacity/Cargo.toml`, which is `path = "../.."`. Identical mechanism. Verify in the PR's CI run. |
| **R2 — Cargo workspace inheritance collisions** | Low | High | The gateway's `spikes/rust/router/Cargo.toml` does not declare a parent workspace (it has no `[workspace]` table, no parent `Cargo.toml` exists at the gateway root). The capacity subcrate will follow the same shape. Verified in the repo scan: the gateway has no `Cargo.toml` at the root. |
| **R3 — `cargo fmt` / `clippy` churn in the absorbed source** | Low | Low | The pheno-capacity source is already `cargo fmt --check` + `clippy -D warnings` clean (CI enforces it). The absorb is a verbatim copy; no reformatting needed unless the gateway adds a project-level `rustfmt.toml` that diverges (none present). |
| **R4 — Test count regression** | Low | Medium | 60 unit + 6 doc tests must pass under the new path. If any test references a relative path or `env!("CARGO_MANIFEST_DIR")` (none in the source per scan), the path shifts. Mitigation: run `cargo test --all-features` in the PR's CI before merge. |
| **R5 — Module name collision with router spike** | Very low | Low | The router spike is `phenotype-router-spike` (`phenotype_router_spike` lib); the capacity spike is `phenotype-capacity-spike` (`phenotype_capacity_spike` lib). No collision. The two subcrates can coexist; both are `publish = false`. |
| **R6 — Coverage gate (80 %) fails on the new path** | Low | Medium | The existing `llvm-cov` job enforces ≥ 80 % lib coverage. The pheno-capacity crate already meets it (per its CI); the absorb is a verbatim copy. Verify in PR CI. |
| **R7 — `pheno-capacity` consumers break (crates.io path)** | Medium | High | Any consumer depending on `pheno-capacity = "0.2"` from crates.io is unaffected by the absorb (the crates.io artifact remains). Any consumer depending on `pheno-capacity = { git = "https://github.com/KooshaPari/pheno-capacity" }` will be unaffected as long as the source repo is `archived` (not deleted) and the default branch stays reachable. **Archive, do not delete** in the immediate term. |
| **R8 — Mvn-style "two homes" confusion** | Medium | Medium | During the transition window, `pheno-capacity` exists in two places: the archived `KooshaPari/pheno-capacity` repo (source of truth during the 90-day retention) and the absorbed `phenotype-gateway/spikes/rust/capacity/`. The `DEPRECATION.md` in the archived repo (step 8) is the disambiguator. After 90 days, the archived repo is hard-deleted. |
| **R9 — `pheno-capacity` is re-used as a name elsewhere** | Very low | Low | None of the fleet's 200+ repos use the bare name `pheno-capacity` for anything else (verified via registry ECOSYSTEM_MAP per L5-109 audit pattern). |
| **R10 — `git subtree` complications if a side-branch emerges** | Low | Low | If `pheno-capacity` ever gets a v0.3.0 (e.g. from HwLedger Phase 2 work), the absorb's option-(b) layout (`spikes/rust/capacity/src/pheno_capacity/`) makes a `git subtree pull` from the v0.3.0 branch a 1-commit operation. |

### 5.2 Process / governance risks

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| **R11 — User rejects the home (prefers B or a new option)** | Low | High | This plan is a recommendation; the final decision is the user's. The plan is reversible: the absorb PR can be closed; `pheno-capacity` remains unchanged. Effort sunk is ≤ 4 h of code review + migration. |
| **R12 — The 71-pillar audit (ADR-024) flags the gateway's spike as "speculative"** | Low | Low | The router spike is already accepted; the capacity spike is identical in shape. The 71-pillar scoring lives at the umbrella-repo level; the spike tiers reasonably. The absorb is a "in-process to in-process" move (both are spikes, both are non-published, both are ADR-023 lib tier). |
| **R13 — Conflict with the `phenotype-gfx` precedent (the absorb pattern is now used twice)** | Very low | Low | The gfx 4-repo absorb (L5-109) was the precedent; the gateway absorb (L5-117) is the second instance. **This is the goal**: the absorb pattern becomes a fleet-wide SOTA. ADR-036 should reference the gfx precedent explicitly. |
| **R14 — `pheno-throughput` (future) needs a different home** | Low | Low | The gateway can host `pheno-throughput` at `spikes/rust/throughput/` following the same pattern. The absorb is the precedent, not a one-off. The `pheno-capacity/docs/SPEC.md` §7 already calls out `pheno-throughput` as a future crate — when it lands, the gateway is the natural home. |
| **R15 — The `pheno-*` family is now split (some in monorepo, some in gateways)** | Low | Low | The `pheno-*` family is already a hybrid: `pheno-config` lives at `KooshaPari/pheno-config` (and is sparse-checkout-visible at `repos/pheno-config/`); `pheno-context` lives at `KooshaPari/pheno-context`; `pheno-port-adapter` lives at `KooshaPari/pheno-port-adapter`. The "monorepo checkout vs GitHub canonical" duality is already established. `pheno-capacity` becoming a gateway-anchored `pheno-*-lib` is **consistent with** the existing pattern (each pheno-*-lib is its own repo on GitHub, just like the others). |

### 5.3 Risk summary

The highest-impact risk is **R7** (crates.io consumer breakage) —
mitigated by the archive-not-delete approach. The most likely
operational risk is **R1** (`no_std` path-shift in the CI smoke
consumer) — mitigated by running the CI before merge. **No
risk rises to the level of blocking the recommendation.** The
absorb can be executed with the standard fleet CI gates (test,
clippy, fmt, coverage, no_std smoke) and the per-PR review that
all L5-109..L5-112 PRs received.

---

## 6. Effort estimate — 2 to 4 hours

| Step | Time | Device (per ADR-023 / ADR-025 v2.1) | Notes |
|---|---|---|---|
| 1 — Fork branch | 5 min | `device: macbook` | Pure git. |
| 2 — Scaffold subcrate | 10 min | `device: macbook` | 3 files (Cargo.toml, README.md, VERSION.toml). |
| 3 — Ingest source | 15 min | `device: macbook` | `cp` 5 .rs files + restructure into `pheno_capacity/` sub-module. |
| 4 — Ingest meta-bundle | 20 min | `device: macbook` | 11 files to copy + minor header updates. |
| 5 — CI workflow | 30 min | `device: macbook` | New `.github/workflows/cargo.yml`; translate the 3 pheno-capacity jobs to gateway paths. |
| 6 — Doc cross-links | 10 min | `device: macbook` | 2-3 README/SPEC updates. |
| 7 — Open PR | 10 min | `device: macbook` | `gh pr create` with the migration matrix in the body. |
| 8 — Archive + ADR-036 | 15 min (post-merge) | `device: macbook` | `gh repo archive` + ADR-036 file. |
| **Subtotal (steps 1-7)** | **~1h 40min** | | Pre-merge effort. |
| **PR review iteration** | 1-2 h | `device: macbook` or `device: subagent` | Fleet standard. |
| **Total (with review)** | **2h 40min — 3h 40min** | | Comfortably under the "~2-4 h" envelope. |

**Heavy work gate (ADR-023 Rule 1):** None of the steps above
require `cargo test --workspace` against a multi-100-crate
workspace, an iOS Simulator boot, a Docker-in-Docker test, or a
single build/test cycle > 10 min. The only Cargo invocations are
`cargo test --all-features` against a single subcrate
(`phenotype-capacity-spike`), which is sub-30-second wall time.
**All steps run on `device: macbook`; no subagent dispatch
required.** No risk of hitting the heavy-work gate.

---

## 7. Open questions

The following are decision-deferred items that block the absorb
PR's full landing, in order of urgency.

### OQ-1 — What happens to the published crates.io artifact `pheno-capacity = "0.2"`?

**Three options:**

- **(a) Keep publishing from the archived repo.** The archive is
  read-only on GitHub, but `cargo publish` uses crates.io as the
  source of truth (not GitHub). A v0.2.x patch can still be
  published to crates.io from the archived repo. This is the
  **least churn for existing consumers** and matches the fleet
  convention (e.g. `pheno-config` is a published crate; absorbed
  crates' crates.io paths stay stable).
- **(b) Yank v0.2.0 from crates.io.** Replace with a 1-line shim
  that re-exports from the gateway (`pub use phenotype_capacity_spike::*;`).
  Causes a SemVer-major bump; breaks every existing consumer.
  Not recommended.
- **(c) Yank the crate entirely; force consumers to depend on the
  gateway via `git = "..."`.** Rejected; this is "the new-repo
  problem" the user directive is trying to avoid.

**Recommendation: (a) keep publishing from the archived repo.**
The archive-vs-delete distinction is what makes this safe. The
`DEPRECATION.md` (step 8) makes the migration discoverable.

**Decision needed from:** user + ADR-036 author.

### OQ-2 — Should the `pheno-capacity` GitHub repo be hard-deleted after 90 days?

The active `gh` token has scopes `'gist', 'read:org', 'repo',
'workflow'` — **no `delete_repo`**. Hard-delete requires the
GitHub UI (Settings → General → Danger Zone → Delete this
repository). The 4-repo retirement (L5-109) is currently in the
"archived" state for all 4 sources; hard-delete has not yet been
performed for any of them.

**Recommendation:** Apply the same 90-day grace period; do not
hard-delete. Hard-delete is a user-action item (UI only) and
should be batched with the other 4 repos in a single UI session.

**Decision needed from:** user.

### OQ-3 — Does the `pheno-*` family need a fleet-wide ADR-023 update?

`pheno-capacity` is currently a `pheno-*-lib` (per `AGENTS.md`).
After the absorb, it is a `pheno-*-lib` hosted inside a
collection repo (the gateway). Is this still a "pure reusable
library" per ADR-023, or is it now a "federated service
sub-crate"? The behavior is identical (no I/O, no panics, no
runtime); the only thing that changed is the GitHub host.

**Recommendation:** Add a one-paragraph clarification to ADR-023
(distinguishing "pheno-*-lib hosted in a collection repo" from
"pheno-*-lib hosted as a standalone repo"). No rule change; the
hosting is an organizational decision, not an architectural one.

**Decision needed from:** ADR-023 author (deferred to the next
ADR batch).

### OQ-4 — Does the absorbed `phenotype-capacity-spike` need to participate in the `phenotype-gateway` Cargo workspace?

The router spike (`spikes/rust/router/Cargo.toml`) is a
**standalone subcrate** (no parent workspace at the gateway
root; no `members = [...]` array anywhere). The capacity spike
follows the same shape.

**Trade-off:**

- **Keep standalone (recommended):** matches the router spike;
  preserves the "spike" semantics (evaluating, not promoting);
  minimizes coupling; the gateway's `Cargo.toml` is not needed.
- **Convert to workspace member:** add a root
  `phenotype-gateway/Cargo.toml` with
  `members = ["spikes/rust/router", "spikes/rust/capacity"]`;
  enables `cargo test --workspace` from the gateway root. The
  cost is a new `Cargo.toml` at the gateway root, which the
  gateway's `Taskfile.yml` and `.gitmodules` did not anticipate.

**Recommendation:** Keep standalone. The router spike has been
standalone since 2026-06-18 (H13); changing the shape now
introduces churn. When the gateway's H6+ roadmap stabilizes
(i.e. when both router + capacity are promoted from "spike" to
"promoted package"), introduce the workspace at that point.

**Decision needed from:** gateway owner (charter ADR-ECO-014).

### OQ-5 — Does the absorbed crate need a `packages/` slot?

The gateway's `README.md` has this promotion rule:

> "Submodule → `packages/<name>` when component passes checklist
> in `GATEWAY_FEATURE_PARITY.md`."

`spikes/rust/capacity/` is a **spike** (per the same README);
promotion to `packages/capacity/` would require passing
`GATEWAY_FEATURE_PARITY.md`. That document does not yet exist
in the visible tree (verify during the PR review). Promotion is
**a future concern**, not part of this absorb.

**Recommendation:** Land the absorb in `spikes/rust/capacity/`;
defer the promotion question to a follow-up PR after
`GATEWAY_FEATURE_PARITY.md` is authored.

**Decision needed from:** gateway owner; deferred to H14+.

### OQ-6 — Should the absorb also fold the `pheno-throughput` (future) and `pheno-predict` (existing) libs into the same gateway spike path?

`pheno-capacity/docs/SPEC.md` §7 lists future crates that will
share the `AttentionKind` + `KvContext` types:
`pheno-throughput` (future) and the `phenotype-mcp-router` (which
already consumes `pheno-capacity` for fit checks).

`pheno-predict` is an existing `pheno-*-lib` per the monorepo
sparse-checkout cone.

**Recommendation:** Out of scope for L5-117. Each absorb is a
single PR; the fleet's batched L5-109..L5-112 absorb is the
precedent (one PR per source). When `pheno-throughput` is created
in the future, it lands at `spikes/rust/throughput/` in a
separate PR.

**Decision needed from:** future L5-x; not blocking.

### OQ-7 — Does the `pheno-throughput` mention in `pheno-capacity/docs/SPEC.md` need updating?

After the absorb, the `docs/SPEC.md` §7 "Consumers (planned)"
section names "pheno-throughput (future)" as a future crate.
This is unchanged by the absorb (the consumers are the same
whether `pheno-capacity` is in its own repo or in the gateway).
No update needed.

**Decision needed from:** none; informational.

---

## 8. Appendix A — File inventory (pre-absorb)

### A.1 `KooshaPari/pheno-capacity` (source, 5 source files + 16 meta files)

```
.github/CODEOWNERS                              296 B
.github/workflows/ci.yml                      2,772 B  (3 jobs: test, coverage, no_std)
.gitignore                                      435 B
AGENTS.md                                     2,435 B
CHANGELOG.md                                  4,929 B
Cargo.toml                                    1,526 B  (empty [workspace] — standalone)
LICENSE-APACHE                                  795 B
LICENSE-MIT                                   1,079 B
README.md                                     7,295 B
SECURITY.md                                   1,165 B
WORKLOG.md                                    1,687 B
docs/SPEC.md                                  4,925 B
docs/methodology.md                          11,252 B
llms.txt                                      2,391 B
llvm-cov.toml                                   216 B
src/attention.rs                             16,653 B
src/estimate.rs                              29,822 B
src/lib.rs                                   11,296 B
src/math.rs                                  12,854 B
src/policy.rs                                11,704 B
                                              --------
                                  Total:   ~120 KB / 21 files
```

### A.2 `KooshaPari/phenotype-gateway` (target, current shape)

```
.github/                                        (dir; contents not enumerated in this scan)
.gitmodules                                      541 B  (4 submodules: agentapi++, cliproxy++, argis-ext, bifrost)
README.md                                      1,727 B
Taskfile.yml                                     259 B  (smoke task; PowerShell + bash)
docs/                                           (dir; gateway's unified SPEC.md is here)
packages/                                       (dir; promotion targets)
scripts/                                        (dir; smoke-go.sh/ps1)
spikes/
  go/                                           (dir)
  mojo/                                         (dir)
  rust/
    .gitkeep                                       0 B
    router/
      Cargo.toml                                  120 B  (phenotype-router-spike, publish=false)
      README.md                                   558 B
      src/lib.rs                                  658 B  (RouterPlane trait + ComboRouter + 1 test)
  zig/                                          (dir)
third_party/                                    (4 submodules: agentapi++, cliproxy++, argis-ext, bifrost)
```

### A.3 `KooshaPari/phenotype-gfx` (proven pattern reference, for comparison)

```
.github/, .gitignore, Cargo.lock, Cargo.toml, README.md, VERSION.toml
benches/, bindings/, docs/, examples/, findings/, spec/, src/
src/
  lib.rs                                       1,808 B
  lod.rs                                      21,757 B
  postfx/                                            (dir)
  streaming.rs                                12,683 B
  terrain/                                           (dir)
  voxel/                                       18 files, largest 32,877 B
  voxelizer.rs                                       (file)
findings/
  2026-06-16-terrain-block-c.md               17,452 B
  2026-06-16-voxel-block-c.md                 20,898 B
  2026-06-16-water-block-c.md                 12,055 B
  2026-06-18-postfx-block-c.md                27,485 B
  2026-06-18-sister-repo-block-c-summary.md    9,894 B
  71-pillar-2026-06-18-delta.md               15,403 B
  71-pillar-2026-06-18-summary.md              8,794 B
  71-pillar-2026-06-18.md                     50,727 B
VERSION.toml                                     445 B  (umbrella version manifest)
```

---

## 9. Appendix B — Proven absorb precedent (L5-109..L5-112, summary)

For traceability; this is the exact pattern L5-117 follows.

| Source repo | Source size | Target | Absorb PR | Status (2026-06-19) |
|---|---|---|---|---|
| `KooshaPari/phenotype-voxel` | 17,317 lines / 94 tests | `KooshaPari/phenotype-gfx` (`src/voxel/`) | [phenotype-gfx#10](https://github.com/KooshaPari/phenotype-gfx/pull/10) | Merged (commit `9a7c05a`); source archived. |
| `KooshaPari/phenotype-terrain` | (C# → ported to Rust) | `KooshaPari/phenotype-gfx` (`src/terrain/`) | (L5-110) | Merged; source archived. |
| `KooshaPari/phenotype-water` | (C# → ported to Rust) | `KooshaPari/phenotype-gfx` (`src/water/`) | (L5-111) | Merged; source archived. |
| `KooshaPari/phenotype-postfx` | (C# + HLSL → ported to Rust) | `KooshaPari/phenotype-gfx` (`src/postfx/`) | (L5-112) | Merged; source archived. |
| `KooshaPari/dagctl` | 62 KB | `KooshaPari/phenodag` | [phenodag#13](https://github.com/KooshaPari/phenodag/pull/13) | OPEN (+93). |
| `KooshaPari/kwality` | 6.6 MB / 93 files | `KooshaPari/phenotype-tooling` (`docs/absorbed-from-kwality/`) | [phenotype-tooling#158](https://github.com/KooshaPari/phenotype-tooling/pull/158) | OPEN (+29,422). |
| `KooshaPari/phenotype-auth-ts` | 16 KB | `KooshaPari/AuthKit` (`typescript/packages/auth-ts/`) | [AuthKit#120](https://github.com/KooshaPari/AuthKit/pull/120) | OPEN (+1,901). |
| `KooshaPari/dinoforge-packs` | 744 KB | `KooshaPari/Dino` (`packs/`) | [Dino#297](https://github.com/KooshaPari/Dino/pull/297) | OPEN (+2,329). |

(All from `repos/findings/2026-06-18-L5-109-4-repo-retirement.md`.)

`pheno-capacity` (L5-117) is the **9th absorb** in this wave and
the **first `pheno-*-lib`** in the pattern. The pattern is
established enough that the absorb is mechanical: copy + restructure
+ CI + archive. **No new architectural decision is required by
L5-117 — only the home selection (A vs B vs C), which §3 resolves.**

---

## 10. Appendix C — Related ADRs and findings

- **ADR-023** (2026-06-15): agent-effort governance, device-fit
  gate, app substrate placement rules, lib tier quality bar (80 %
  coverage, 100 % public-API doc-tested, `no_std` compatible where
  applicable).
- **ADR-035A** (2026-06-18, L5-105): HwLedger reclassification +
  `pheno-capacity` extraction rationale.
- **ADR-035** (2026-06-18, L5-105): HwLedger bucket PAUSED →
  CONDITIONAL; capability inventory pending.
- **ADR-004** (the `phenotype-gfx` precedent): single-core + thin
  FFI edges; one Cargo package, multi-module.
- **ADR-ECO-014** (gateway charter): canonical home for agent API,
  LLM proxy, enterprise gateway, router revamp.
- **ADR-022** (config consolidation): two-crate canonical split;
  precedent for "absorb, do not coexist."
- **ADR-031** (Configra absorb, L5-104.7): same pattern applied
  to `phenotype-config` → `Configra`; precedent for "absorb + archive
  + ADR-036."
- **`repos/findings/2026-06-18-L5-109-4-repo-retirement.md`**:
  4-repo retirement pattern (the 4-step roll-out in §2.3).
- **`repos/findings/2026-06-17-L5-104-dmouse92-to-kooshapari.md`**:
  Dmouse92 → KooshaPari migration; same "absorb, do not duplicate"
  principle.
- **`KooshaPari/phenotype-gfx/findings/2026-06-18-sister-repo-block-c-summary.md`**:
  the integration log for the 4-repo absorb; template for the
  `2026-06-19-L5-117-capacity-absorption.md` finding that should
  be authored post-merge (companion to this plan).

---

## 11. Sign-off

- [ ] **User approval** of home recommendation A
  (`phenotype-gateway/spikes/rust/capacity/`).
- [ ] **Resolution of OQ-1** (crates.io artifact fate).
- [ ] **Resolution of OQ-4** (Cargo workspace participation).
- [ ] **Open absorb PR** against `phenotype-gateway:master`.
- [ ] **CI green** on the absorb PR (test, coverage ≥ 80 %,
  no_std smoke).
- [ ] **Merge absorb PR** + bump `phenotype-gateway` minor
  version (per Taskfile.yml versioning).
- [ ] **Archive `KooshaPari/pheno-capacity`** (read-only).
- [ ] **Author ADR-036** capturing the absorb + new home.
- [ ] **Update `phenotype-gateway/README.md`** spike table.
- [ ] **(OQ-2, 90+ days later) Hard-delete** `KooshaPari/pheno-capacity`
  via GitHub UI.
