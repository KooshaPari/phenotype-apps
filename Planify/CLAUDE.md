# planify-wt — Plane.so fork PM landing

## What this is
A fork of **makeplane/plane@preview v1.3.1** with our house branding (`planify.space`). Astro 6 + Bun + Tailwind 4 landing site. Lives next to a future `apps/web` workspace for the full PM web (sprints, kanban, issue boards).

**Provenance**: see `MERGES.md` — the entire `upstream/` directory is verbatim `makeplane/plane@preview v1.3.1` (1456 entries). We are explicitly NOT forking the runtime; just the landing + design language.

## Quickstart

```bash
cd site
bun install            # or pnpm install
bun dev                # http://localhost:4321
bun run build          # → dist/
```

## Structure

```
planify-wt/
├── MERGES.md           # provenance: makeplane/plane@preview v1.3.1
├── UPSTREAM.md         # how to pull upstream changes (rerun merge)
├── CHANGELOG.md        # [Unreleased] tracks our additions
├── upstream/           # VERBATIM plane@preview v1.3.1 (do not move)
├── site/               # OUR landing — Astro 6 + Bun + Tailwind 4
│   ├── src/
│   ├── public/
│   ├── astro.config.mjs
│   ├── package.json
│   └── vercel.json
├── infra/              # docker-compose.plane.yml (mirror of AgilePlus')
├── README.md
├── LICENSE
├── AGENTS.md           # this file
└── docs/               # our governance + specs
```

## Hard rules

1. **`upstream/` is READ-ONLY.** Never edit. To pull upstream changes, see `UPSTREAM.md`.
2. **All OUR work happens in `site/`, `infra/`, `docs/`, and the root files** (`CLAUDE.md`, `AGENTS.md`, `CHANGELOG.md`, `MERGES.md`, `UPSTREAM.md`).
3. **TypeScript strict mode** for `site/` (Astro defaults). We don't downgrade to JS.
4. **Bun for site** — but the rest of the monorepo uses pnpm. Don't try to unify.

## Relationship to other repos

- **AgilePlus** hosts the multi-tenant PM server (`crane-mcp` workspace). planify-wt is the *public landing* for that, not a runtime.
- **Phenotype monorepo root** tracks cross-project compliance — see `../PHENOTYPE_DOGFOOD_COMPLIANCE_AUDIT.md`.
- **Grapheon** is the claims-graph store that plane.so will eventually be wired to (AI-DD: every Plane issue gets a Grapheon claim behind it).

## Deploy

`site/vercel.json` deploys to **planify.space** via Vercel.

## Compliance
- This repo follows the Phenotype governance program. ADRs in `docs/adrs/`, specs in `docs/specs/`, and the cross-project scorecard in `../PHENOTYPE_DOGFOOD_COMPLIANCE_AUDIT.md`.
