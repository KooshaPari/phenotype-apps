# 0003 — Three-directory monorepo: upstream/ + site/ + infra/

- **Status:** accepted
- **Date:** 2026-07-07
- **Supersedes:** none

## Context

Planify is a fork of an existing upstream project (Plane.so) while also
introducing Phenotype-specific additions. The repository must satisfy several
constraints:

1. **Clean upstream sync**: The Plane.so seed must remain as a verbatim
   snapshot to enable `git diff`-based syncs when upstream releases new
   versions. Modifications to upstream code would create merge conflicts.

2. **Phenotype additions**: Planify needs custom infrastructure (Docker
   compose files tuned for the Phenotype stack), a marketing landing page,
   and potentially Phenotype-specific UI overlays — none of which belong
   inside the upstream tree.

3. **Monorepo tooling**: Upstream Plane.so already ships as a pnpm + Turbo
   monorepo. Planify must not create nested monorepo complexity or tooling
   conflicts.

4. **Deployment separation**: The landing page (planify.space) and the PM
   app (served via upstream Plane's web app) are deployed independently:
   the landing page goes to Vercel as a static Astro build; the PM app
   requires a Docker Compose stack with Postgres and Dragonfly.

The following structures were considered:

| Option                               | Upstream Sync | Isolation | Tooling Simplicity |
|--------------------------------------|---------------|-----------|--------------------|
| Flat monorepo (single package.json)  | Conflict risk | Low       | Simple             |
| **Three-top-level dirs (chosen)**    | Clean (isolate)| High     | Simple             |
| Git submodule for upstream           | Clean         | Highest   | Complex            |
| Separate repos for each surface      | Clean         | Highest   | Complex (cross-ref)|
| Vendored upstream in vendor/         | Clean         | High      | Moderate           |

## Decision

Organize the repo as three top-level directories, each with independent tooling:

```
planify/
├── upstream/            # Verbatim Plane.so seed — DO NOT MODIFY
│   ├── apps/            # admin, api, live, proxy, space, web
│   ├── packages/        # 15 shared TS packages
│   ├── package.json     # pnpm workspace root
│   ├── pnpm-workspace.yaml
│   ├── turbo.json
│   └── LICENSE.txt      # AGPL-3.0 (inherited)
├── site/                # planify.space landing page (Astro + Bun + Tailwind)
│   ├── astro.config.mjs
│   ├── package.json     # Bun-managed
│   └── vercel.json      # Vercel deployment config
└── infra/               # Phenotype-specific deployment additions
    └── docker-compose.plane.yml
```

### Directory policies

| Directory  | Modify? | Sync mechanism                    | Package manager |
|------------|---------|-----------------------------------|-----------------|
| `upstream/`| Never   | `git fetch/merge upstream/preview`| pnpm            |
| `site/`    | Freely  | N/A (owned)                       | bun             |
| `infra/`   | Freely  | Mirrored from `AgilePlus` repo    | N/A             |

### Key invariants

- `upstream/` is never modified in Planify-specific commits. If a bugfix is
  needed in upstream code, the fix must be sent to `makeplane/plane` upstream
  first, then synced via a merge commit.
- `upstream/` keeps its own package.json, tsconfig, and tooling configurations.
  Root-level configs (eslint, prettier) serve repo-level concerns only.
- Infra files in `infra/` reference the Plane stack from `upstream/` via
  relative docker-compose paths or image names, never by modifying upstream
  manifests.

## Consequences

### Positive

- Upstream syncs are trivial: `git fetch upstream && git merge upstream/preview`
  into `upstream/`. Zero merge conflicts because no customizations touch
  upstream files.
- Each directory has the right package manager and build tool for its context:
  pnpm for the heavy Plane monorepo, Bun for the lightweight landing page.
- Independent deployment pipelines: landing deploys to Vercel on its own
  schedule; the PM app stack deploys via Docker Compose.
- Clear ownership boundaries for contributors: "if it's in `upstream/`, file
  the fix upstream."

### Negative

- No shared node_modules or TypeScript project references between `upstream/`
  and `site/` — shared types must be duplicated or published as separate
  packages.
- Two package managers (pnpm + bun) must be installed in CI/CD environments,
  increasing setup time.
- The three-directory structure means `cd upstream && pnpm run dev` starts a
  different dev server than `cd site && bun run dev`. Contributors must learn
  which to use for their task.

### Neutral

- `infra/` currently contains only a single docker-compose file (mirrored from
  `KooshaPari/AgilePlus`). As deployment complexity grows, subdirectories for
  Helm charts, Terraform modules, and CI configs may be added.
- Git submodules were rejected for upstream because they add an extra `git
  submodule update` step to clone workflows and complicate branch switching.
  The verbatim-copy approach is simpler at the cost of slightly larger repo
  size (~200 MB from upstream).
