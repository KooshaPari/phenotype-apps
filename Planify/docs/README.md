# Planify Documentation

> **Planify** is the web-based project management UI for the Phenotype platform,
> derived from [Plane](https://github.com/makeplane/plane) (AGPL-3.0).

## Repository Layout

```
planify/
├── upstream/          # Verbatim Plane.so seed (DO NOT MODIFY)
├── site/              # planify.space landing page (Astro + Bun + Tailwind)
├── infra/             # Phenotype-specific deployment additions
├── docs/              # Planify documentation
│   ├── README.md      # This file
│   └── adr/           # Architecture Decision Records
├── AGENTS.md          # Agent development guide
├── CONTRIBUTING.md    # Contribution guidelines
├── CHANGELOG.md       # Release history
├── SECURITY.md        # Security policies
├── MERGES.md          # Consolidation provenance
└── UPSTREAM.md        # Upstream seeding notes
```

## Architecture Decision Records

The [`docs/adr/`](adr/) directory contains Architecture Decision Records (ADRs)
that document significant architectural choices. Each ADR describes the context,
options considered, the decision made, and its consequences.

| ID   | Title                                                  | Status     | Date       |
|------|--------------------------------------------------------|------------|------------|
| 0001 | [Use Plane.so fork](adr/0001-use-plane-so-fork.md)     | accepted   | 2026-07-07 |
| 0002 | [Hybrid frontend stack](adr/0002-frontend-stack.md)    | accepted   | 2026-07-07 |
| 0003 | [Three-directory monorepo](adr/0003-monorepo-structure.md) | accepted | 2026-07-07 |

For the full index and contribution guidelines, see [`docs/adr/INDEX.md`](adr/INDEX.md).

## Getting Started

```bash
# Upstream Plane.so stack (PM application)
cd upstream
pnpm install
pnpm dev

# Landing page (planify.space)
cd site
bun install
bun run dev

# Infrastructure
cd infra
docker compose -f docker-compose.plane.yml up -d
```

## Related Documentation

| File                 | Purpose                          |
|----------------------|----------------------------------|
| `AGENTS.md`          | Development workflow for AI agents |
| `CONTRIBUTING.md`    | PR process, commit conventions   |
| `CHANGELOG.md`       | Version history                  |
| `SECURITY.md`        | Vulnerability reporting          |
| `MERGES.md`          | Code consolidation provenance    |
| `UPSTREAM.md`        | Upstream sync instructions       |
