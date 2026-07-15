# Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for the Planify
project. Each ADR documents a significant architectural decision, its context,
the options considered, and the resulting consequences.

ADR format follows [MADR](https://adr.github.io/madr/) (Markdown ADR)
conventions. See [`TEMPLATE.md`](TEMPLATE.md) for the template and instructions.

## ADR Index

| ID    | Title                                                        | Status     | Date       |
|-------|--------------------------------------------------------------|------------|------------|
| 0001  | [Use Plane.so fork](0001-use-plane-so-fork.md)               | accepted   | 2026-07-07 |
| 0002  | [Hybrid frontend stack](0002-frontend-stack.md)              | accepted   | 2026-07-07 |
| 0003  | [Three-directory monorepo](0003-monorepo-structure.md)       | accepted   | 2026-07-07 |
| 0004  | [CI workflow strategy](0004-ci-workflow-strategy.md)         | accepted   | 2026-07-08 |
| 0005  | [Upstream sync strategy](0005-upstream-sync-strategy.md)     | accepted   | 2026-07-08 |

## Status Legend

| Status         | Meaning                                                    |
|----------------|------------------------------------------------------------|
| `proposed`     | Under discussion; not yet accepted                         |
| `accepted`     | Approved and adopted                                       |
| `deprecated`   | No longer recommended; kept for historical record          |
| `superseded`   | Replaced by a newer ADR; kept for historical record        |

## Quick Reference

- **[ADR-0001](0001-use-plane-so-fork.md)** — Why Planify is a fork of
  `makeplane/plane` rather than a from-scratch build, including license
  implications and upstream sync strategy.
- **[ADR-0002](0002-frontend-stack.md)** — The hybrid stack decision: React +
  Next.js for the PM application (inherited from Plane.so) + Astro + Bun +
  Tailwind for the planify.space landing page.
- **[ADR-0003](0003-monorepo-structure.md)** — The three-directory layout
  (`upstream/` + `site/` + `infra/`), package manager separation, and
  upstream sync isolation policy.
- **[ADR-0004](0004-ci-workflow-strategy.md)** — GitHub Actions CI strategy
  with two workflow files (site build + upstream check), Dependabot config,
  and auto-merge for patch-level dependency updates.
- **[ADR-0005](0005-upstream-sync-strategy.md)** — Weekly automated upstream
  sync from `makeplane/plane` via GitHub Actions, including conflict handling
  strategy and the sync PR workflow.

## Adding a New ADR

1. Choose the next sequential number
2. Copy [`TEMPLATE.md`](TEMPLATE.md) as `docs/adr/NNNN-title.md`
3. Fill in all sections with concrete, repo-specific reasoning
4. Add the entry to this index
5. If superseding an existing ADR, update that ADR's `Superseded by` field

## Maintenance

ADRs are living documents. When a decision is revisited or reversed:

- Mark the old ADR as `superseded` with a `Superseded by` field pointing to the
  new ADR
- Create the new ADR with its number, status `accepted`, and a `Supersedes`
  field pointing to the old ADR
- Update this index for both entries
