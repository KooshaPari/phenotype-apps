# ADR-0002: Astro 6 + Bun + Tailwind 4 for the landing

- **Status**: Accepted
- **Date**: 2026-07-14

## Context
The landing site needs to be fast (LCP < 1s), SEO-friendly (full SSR/SSG), and easy to maintain by a small team.

## Decision
Astro 6 + Bun + Tailwind 4.

## Consequences
- Astro's island architecture = mostly static HTML + selective JS hydration.
- Bun = faster install/build than Node 22.
- Tailwind 4 = consistent with grapheon-frontend, planify-wt, and the rest of the new monorepo.

## Alternatives considered
- **Next.js**: rejected — heavier bundle, RSC overhead for what is essentially a marketing site.
- **Plain Vite SPA**: rejected — worse SEO.
- **Eleventy**: rejected — no first-class TS support.
