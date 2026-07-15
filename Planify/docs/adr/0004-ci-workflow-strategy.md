# 0004 — CI workflow strategy: GitHub Actions with two-path pipelines

- **Status:** accepted
- **Date:** 2026-07-08
- **Supersedes:** none

## Context

Planify is a multi-tool monorepo with two independent build surfaces and a strict
upstream isolation policy. Before this ADR, there was no automated CI — all
verification was manual:

1. **Site surface** (`site/`): Astro + Bun + Tailwind. Must type-check and build
   to ensure planify.space deploys without errors.

2. **Upstream surface** (`upstream/`): pnpm + Turbo monorepo inherited from
   Plane.so. Must continue to type-check and lint after sync merges. ADR-0003
   forbids modifying upstream files, but syncing upstream Plane releases can
   introduce type regressions that would go undetected without CI.

3. **Tooling divergence**: `site/` uses Bun; `upstream/` uses pnpm + Node.js.
   A single CI runner cannot install dependencies for both without separate
   setup steps.

4. **Velocity constraints**: Without CI, every merge to `main` risks breaking
   the landing page build or introducing type errors inherited from upstream
   Plane changes. A CI gate is required before merge for all contributors.

The following CI orchestration options were considered:

| Option                                        | Complexity | Parallelism | Build Time | Maintenance |
|-----------------------------------------------|------------|-------------|------------|-------------|
| **Two separate workflows (chosen)**           | Low        | Full        | ~5 min     | Low         |
| Single workflow with two jobs                 | Low        | Full        | ~5 min     | Low         |
| Monorepo-aware tool (Turborepo, Nx, Bazel)    | High       | Full        | ~3 min     | High        |
| Single matrix-job workflow                    | Moderate   | Sequential  | ~7 min     | Moderate    |

Using two separate workflow files was chosen over a single workflow because:

- **Clear ownership**: Each workflow file is scoped to its surface (`site` vs.
  `upstream`). A contributor working on the landing page can see the site CI
  config without sifting through upstream job definitions.
- **Independent evolution**: If one surface later adds deploy steps or
  integration tests, its workflow file grows independently.
- **Shared concurrency rules**: Both workflows benefit from GitHub Actions'
  concurrency cancellation, but they target different paths so they don't need
  to share a group.

Using full monorepo tooling (Turborepo remote caching, Nx affected detection)
was rejected as over-engineering for a repo with two independent surfaces that
already have their own build systems.

## Decision

Adopt **two GitHub Actions workflow files** — one per build surface — plus
a Dependabot auto-merge workflow:

```
.github/
├── dependabot.yml              # Weekly npm updates for site/ and upstream/
└── workflows/
    ├── ci.yml                  # Site build + upstream type-check on push/PR
    └── dependabot-auto-merge.yml  # Auto-approve + merge for patch deps
```

### CI pipeline (`ci.yml`)

| Job            | Trigger              | Steps                                                                 | Agent     |
|----------------|----------------------|-----------------------------------------------------------------------|-----------|
| `site-build`   | push/PR to `main`    | Setup Bun → `bun install --frozen-lockfile` → `bun check` → `bun build` | ubuntu-latest |
| `upstream-check`| push/PR to `main`   | Setup Node + pnpm → `pnpm install --frozen-lockfile` → `pnpm check`   | ubuntu-latest |

Key behaviors:

- **Concurrency**: Both jobs use `concurrency` groups keyed on `github.ref` to
  cancel in-progress runs when a new commit is pushed to the same branch.
- **Caching**: `actions/cache` for both Bun's `~/.bun/install/cache` and
  pnpm's `~/.local/share/pnpm/store` to speed up installs across runs.
- **Fail-fast**: Jobs run in parallel; a failure in either job blocks the merge.

### Dependabot auto-merge (`dependabot-auto-merge.yml`)

Triggered by Dependabot PR activity. Auto-approves and auto-merges patch-level
dependency updates to reduce noise while keeping security fixes fast.

### Dependabot configuration (`dependabot.yml`)

Two weekly npm update groups:

| Package ecosystem | Location | Schedule | Reviewer       |
|-------------------|----------|----------|----------------|
| `npm`             | `site/`  | Weekly   | @KooshaPari    |
| `npm`             | `upstream/`| Weekly | @KooshaPari    |

## Consequences

### Positive

- **CI gate before merge**: Every push and PR to `main` is verified. Broken
  builds or type errors are caught before they land.
- **Parallel execution**: Site build and upstream check run concurrently,
  completing in ~5 minutes total.
- **Zero upstream modification**: CI runs `pnpm check` on upstream verbatim;
  no Planify-specific changes touch `upstream/`.
- **Automatic patch updates**: Dependabot auto-merge reduces manual triage
  for safe dependency bumps.
- **Cancellation of stale runs**: Concurrency groups prevent wasted CI minutes
  on outdated commits.

### Negative

- **Two setup steps**: CI must install both Bun (for site) and Node.js + pnpm
  (for upstream), doubling the per-run setup time versus a single-tool repo.
- **No integration tests**: The current CI only covers type-checking and
  builds. End-to-end tests (e.g., landing page visual regression, API smoke
  tests) are not included.
- **No matrix coverage**: Node/bun version matrices are skipped for speed.
  If a contributor uses a different Node version locally, CI may behave
  differently.

### Neutral

- **Dependabot weekly cadence**: Daily or monthly were considered; weekly
  balances freshness against noise for a project at this maturity stage.
- **Workflow separation**: Two files means two places to check for CI status.
  A dashboard or status badge could be added later.
- **Upstream CI scope**: `pnpm check` runs lint + format + type-checks via
  Plane's Turborepo pipeline. If upstream adds new checks, they are picked
  up automatically — no Planify workflow change needed.
