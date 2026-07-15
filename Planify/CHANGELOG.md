# Changelog

All notable changes to Planify will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `upstream/` — seeded with verbatim snapshot of `makeplane/plane@preview` v1.3.1 (AGPL-3.0)
  - Plane apps: admin, api, live, proxy, space, web
  - Plane packages (15): codemods, constants, decorators, editor, hooks, i18n, logger, propel, services, shared-state, tailwind-config, types, typescript-config, ui, utils
  - Root pnpm workspace, Turbo config, Docker Compose manifests
- `site/` — Astro 6 + Bun + Tailwind 4 landing page scaffolded (planify.space)
  - Hero section with Three.js 3D canvas (placeholder keyboard geometry)
  - Feature grid, CTA section, footer
  - Vercel deployment config
- `infra/` — Docker Compose for Plane stack mirrored from AgilePlus
  - Postgres 16 + Dragonfly + plane-api/worker/beat + plane-web
- Root README, UPSTREAM.md, MERGES.md — project overview, seeding notes, consolidation provenance
- Root LICENSE (Apache 2.0), AGENTS.md, CONTRIBUTING.md, CHANGELOG.md, SECURITY.md — foundational repo docs
- `.gitignore` — patterns for build artifacts, secrets, and OS files

### Known Gaps

- `.glb` keyboard model for hero 3D scene — missing from assets; placeholder geometry renders
- `pnpm install` and `bun install` deferred due to disk pressure (42 GiB free at seed time)
- No custom Phenotype features beyond scaffolding — upstream Plane code is unmodified

## [0.1.0] - Unreleased

### Added

- Initial repository seeding and scaffolding
- Plane.so fork structure with upstream/ subtree
- Astro landing page
- Docker infra
- Foundational documentation and tooling
