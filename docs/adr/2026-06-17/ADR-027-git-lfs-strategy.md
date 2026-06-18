# ADR-027: Git LFS strategy — 3-tier policy for binary assets

**Status:** Accepted 2026-06-17 · **Deciders:** Phenotype governance circle
**Closes:** L66 (DX: git LFS guidance) — root cause of the 2026-06-17 monorepo strand (3 governance commits + l4-68 pheno-context crate stranded by LFS rejects).
**Supersedes:** ad-hoc "LFS if it's big" guidance; the 3-tier policy is now canonical for all Phenotype repos.

## Context

The 2026-06-17 monorepo wrap-up (`audit-71-pillar-2026-06-17-wrapup.md` § 6.3) stranded **3 governance commits** + the l4-68 pheno-context crate (286 LOC) because `git push` to `argis` (KooshaPari/argis-extensions) was rejected: submodules checked out from Dmouse92/AgilePlus referenced LFS objects not in the local cache, and `git config lfs.allowincompletepush true` is not sufficient. L66 has been `⚠ blocked` since the 30-pillar audit landed. The 71-pillar expansion (ADR-024) kept L66 blocked.

The fix is a 3-tier policy that names the asset, the action, and the LFS cache management on heavy-runner devices.

## Decision

**3-tier LFS policy for binary assets.** A `.gitattributes` at the repo root declares the tier per pattern; the policy is enforced by pre-commit + CI smoke.

### Tier 1 — ALWAYS track (mandatory LFS)

Assets that **must** be in LFS; pushing without LFS fails the L6 health audit.

| Asset | Pattern | Why |
|---|---|---|
| **iOS `.xcframework`** | `*.xcframework/** filter=lfs diff=lfs merge=lfs -text` | Single bundle can be 100 MB–2 GB; multiple frameworks push repos past GitHub's 5 GB soft cap. **This is the asset that blocked the 2026-06-17 monorepo push.** |
| **iOS `.framework` bundles** | `*.framework/** filter=lfs diff=lfs merge=lfs -text` | Same reason; smaller but still MB-scale. |
| **Compiled `.a` / `.dylib` / `.so` static libs** | `*.a filter=lfs diff=lfs merge=lfs -text`, `*.dylib filter=…`, `*.so filter=…` | Compiled artifacts; not diffable. |
| **Unity / Unreal asset bundles** | `*.unitypackage filter=lfs diff=lfs merge=lfs -text`, `*.uasset filter=lfs diff=lfs merge=lfs -text` | Binary blobs; diffs are meaningless. |
| **Disk images (`.dmg`, `.iso`)** | `*.dmg filter=lfs diff=lfs merge=lfs -text`, `*.iso filter=…` | Single-file GB-scale. |

### Tier 2 — ON-DEMAND track (LFS only when > 5 MB or > 50 files in a dir)

Assets tracked in LFS **only when** size or count justifies it. Default: not in LFS. Add LFS only if any of:

- single file > 5 MB, OR
- directory contains > 50 files of the same type, OR
- the asset is a build output, vendored dep, or 3rd-party SDK.

| Asset | Pattern (default — not in LFS) | Trigger to LFS |
|---|---|---|
| **Rust release binaries** | `target/release/**/*.exe filter=lfs diff=lfs merge=lfs -text` (in `.gitignore` normally) | If a CI workflow publishes a binary to a release branch, push to LFS or (preferred) use `gh release upload` with the asset, not git. |
| **Python wheels** | `dist/*.whl filter=lfs diff=lfs merge=lfs -text`, `dist/*.tar.gz filter=…` | Only if publishing wheels via git (rare; usually PyPI). |
| **Go release binaries** | `bin/* filter=lfs diff=lfs merge=lfs -text` | Same as Rust — prefer `gh release upload`. |
| **Vendored deps with binaries** | `vendor/** filter=lfs diff=lfs merge=lfs -text` | If the vendored dep tree is > 50 MB, track in LFS. |
| **Test fixtures (images, audio, video)** | `tests/fixtures/**/*.png filter=lfs …`, `*.jpg`, `*.mp4`, `*.wav` | If a single fixture > 5 MB OR a fixture dir > 50 MB. |
| **Generated docs (`cargo doc` HTML, PDF)** | `docs/**/api.html filter=…`, `*.pdf filter=…` | Only if > 5 MB. |

**Rule of thumb:** "Generated artifacts belong in CI release artifacts, not in git." If the file is reproducible from source, it does not belong in git at all (`.gitignore` > LFS).

### Tier 3 — NEVER track (explicit non-LFS)

Assets that **must not** be in LFS; the audit flags any violation.

| Asset | Why |
|---|---|
| **Source code (`.rs`, `.py`, `.go`, `.ts`, `.swift`)** | Source is text; LFS would lose diff-ability. |
| **Small text configs (`.toml`, `.yaml`, `.json`)** | Same; LFS would corrupt the diff workflow. |
| **Lockfiles (`Cargo.lock`, `package-lock.json`, `go.sum`)** | Deterministic, small, diff-able. |
| **Markdown / docs source (`.md`)** | Diff-able, small. |
| **SPDX headers, LICENSE files** | Trivial size. |

### LFS cache management on heavy-runner devices

Heavy-runner devices (self-hosted runners, dispatched subagents — see ADR-023 Rule 1, ADR-025 `device: heavy-runner`) must:

1. **Pre-warm LFS on checkout** — `git lfs fetch --all` runs as a setup step before any `cargo test` / `pytest` / `go test` that touches a Tier-1 asset.
2. **Pin LFS object versions in CI** — the CI workflow checks `git lfs ls-files | sha256sum` against a `LFS_LOCKSUM` file (committed in repo root) to detect silent LFS regressions.
3. **Clean LFS on worktree close** — `git worktree remove` runs `git lfs prune --local` to prevent cache bloat.
4. **Quota on heavy-runner** — 50 GB LFS cache ceiling; `git lfs prune --warn-when-above 50G` is the CI guard.

**Local-dev override (MacBook, `device: macbook`):** Tier-1 assets may be checked out **without** LFS if the local task does not need them (sparse-checkout the dir, leave the LFS pointer in place). The build will then fail locally for any task that needs the asset — which is the signal to dispatch to a heavy-runner (ADR-023 Rule 1).

## Consequences

*Positive:*
- L66 promoted from `⚠` to `△ partial → ✓ addressed`. The monorepo strand from 2026-06-17 (3 governance commits + l4-68 pheno-context crate) becomes recoverable: pre-warm LFS on the heavy-runner, push to argis, or cherry-pick to a non-LFS host.
- A single `.gitattributes` per repo is the source of truth; `cat .gitattributes | grep lfs` is the L6 audit check.
- Heavy-runners are explicit (not implicit) about LFS; the MacBook stays small.

*Negative:*
- 3 repos with no `.gitattributes` (yet) must adopt the policy: cost ≈ 1 PR per repo (the .gitattributes + 1 `LFS_LOCKSUM` update).
- Heavy-runner cache management adds 2 CI steps (~5–10 s).
- Tier-2 "on-demand" requires judgment; L6 audit cannot programmatically detect "should this be LFS?".

*Mitigation:*
- `.gitattributes.example` (shipped with this ADR) is a copy-paste starting point; the first PR per repo is purely additive.
- LFS_LOCKSUM is auto-generated by a `pheno-ci-templates` step; no manual maintenance.

## Alternatives considered

- **LFS for everything binary.** Rejected: LFS adds a network round-trip to every checkout; over-tracking hurts small repos.
- **No LFS, no binary assets, ever.** Rejected: iOS / Unity / Unreal / cross-platform native work requires LFS; forbidding it would block those repos entirely.
- **LFS only on the heavy-runner, not in the repo.** Rejected: LFS is a server-side protocol; the `.gitattributes` is what enables it; without the file, the asset is always inline.

## References

- `audit-71-pillar-2026-06-17-wrapup.md` § 6.3 (LFS root cause of the 2026-06-17 monorepo strand).
- `audit-71-pillar-2026-06-17-wrapup.md` § 8.2 row L66 (was `⚠`, now `△ → ✓`).
- `.gitattributes.example` (at repo root) — copy-paste Tier-1/2/3 policy.
- ADR-023 (device-fit gate), ADR-025 (worklog v2.1 `device:` field).
- L66 (DX: git LFS guidance) — the pillar this ADR addresses.
