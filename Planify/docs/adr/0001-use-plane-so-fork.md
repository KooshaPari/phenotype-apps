# 0001 — Use Plane.so fork as the PM frontend foundation

- **Status:** accepted
- **Date:** 2026-07-07

## Context

Planify needs to provide a web-based project management frontend that powers the
AgilePlus dashboard and integrates with the broader Phenotype ecosystem. The
frontend must support:

- Issue tracking (boards, lists, backlogs)
- Project cycles and sprints
- Team modules and views
- Pages and documents
- Project analytics and roadmaps
- User authentication and workspace management

Building a production-grade PM UI from scratch would take an estimated 12–18
months to reach feature parity with existing open-source alternatives.

The following options were evaluated:

| Option                            | License        | Maturity                 | Stack Affinity | Effort to Adapt |
|-----------------------------------|----------------|--------------------------|----------------|-----------------|
| **Fork Plane.so** (makeplane/plane)| AGPL-3.0      | Production-ready (v1.3.1)| React/TS       | ~2 weeks        |
| Fork Taiga                         | MPL-2.0       | Mature (Python/Django)   | Python/Angular | ~4 weeks        |
| Fork OpenProject                   | GPL-3.0       | Mature (Ruby/Rails)      | Ruby/Angular   | ~6 weeks        |
| Build from scratch                 | —             | —                        | Any            | ~12–18 months   |
| Use Linear-style boards via SDK    | Proprietary   | N/A (API-only)           | Any            | ~3 months        |

## Decision

Fork [makeplane/plane](https://github.com/makeplane/plane) at `preview` branch
(v1.3.1 tag, AGPL-3.0) as the foundation for Planify.

Key selection criteria:

1. **License alignment**: AGPL-3.0 permits forking and self-hosting. Phenotype
   operates its own infrastructure, so the AGPL network-interaction clause is
   satisfied by providing source access to SaaS users per §13.

2. **Stack compatibility**: Plane uses React 18 + TypeScript + Tailwind CSS +
   Next.js 14, which matches the Phenotype frontend skill base. No
   language/framework mismatch.

3. **Feature scope**: Plane already ships issue tracking, cycles, modules,
   pages, views, analytics, and workspace management — reducing the starting
   gap from 12–18 months to approximately zero.

4. **Community and ecosystem**: 40k+ GitHub stars, active development, and
   a proven migration path from monolithic to distributed architecture (the
   `preview` branch includes a Turbo monorepo with separation of concerns).

5. **Monorepo structure**: Plane ships as a pnpm + Turbo monorepo with
   published shared packages (UI, types, state, editor). This enables
   selective integration without forking the entire stack.

## Consequences

### Positive

- Immediate feature parity with a production-grade PM tool — zero gap
  starting from v1.3.1.
- Access to 15 shared TypeScript packages (UI components, editor, hooks,
  constants, state management, i18n, utilities) that can be selectively
  consumed or replaced.
- Active upstream community providing bug fixes and features that can be
  merged periodically.
- AGPL-3.0 license meets Phenotype's requirement for open-core with
  self-hosted customers.

### Negative

- AGPL-3.0 restricts proprietary redistribution. Third parties licensing
  Planify as part of a closed-source product must negotiate a separate
  commercial license with Plane.so.
- Upstream sync burden: Plane's preview branch evolves rapidly
  (50–100 commits/week). Skipping syncs creates drift that compounds
  over time.
- Inherited technical debt: Plane's codebase carries pre-v1.0 migration
  artifacts, legacy state patterns (MobX + React Query phase transition),
  and some duplicated APIs across apps.

### Neutral

- The `upstream/` directory must remain as a verbatim snapshot to enable
  clean diff-based syncs. All customizations live outside this directory.
- Plane's mobile app (Flutter) and live-collaboration layer are under
  active development upstream; Planify can adopt these when stable
  without immediate investment.
- Fork branding and domain separation (planify.space vs plane.so) requires
  ongoing asset updates with each upstream sync.
