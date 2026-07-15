# 0002 — Hybrid frontend stack: Plane.so React/Next.js apps + Astro landing page

- **Status:** accepted
- **Date:** 2026-07-07
- **Supersedes:** none

## Context

Planify serves two distinct surfaces:

1. **The PM application** — issue tracking, cycles, modules, views, pages,
   analytics, admin. This is the core product and must ship with full
   interactivity, real-time updates, and server-side rendering.

2. **planify.space landing page** — marketing site (hero, feature grid, CTA,
   pricing, docs). This is a static-content surface that prioritizes fast
   first-contentful paint, SEO, and low operational cost.

The upstream Plane.so monorepo ships its app surfaces (apps/web, apps/space,
apps/admin) built on React 18 + Next.js 14 + TypeScript + Tailwind CSS,
communicating with a Python (Django) API backend.

For the landing page, two paths were evaluated:

| Option                    | Build Speed | SEO      | Bundle Size | Learning Curve |
|---------------------------|-------------|----------|-------------|----------------|
| **Astro 6**               | Fast        | Native   | Tiny (~4 KB)| Low            |
| Next.js 14 (pages router) | Moderate    | SSR/ISR  | ~80 KB      | Already known  |
| Plain HTML + Tailwind     | Fast        | Native   | Minimal     | None needed    |

Astro was selected over Next.js for the landing page because:

- Zero JS by default — the landing page ships no JavaScript for pages that
  don't need interactivity, driving Lighthouse scores into the high 90s.
- `.astro` component syntax is React-adjacent (JSX-like) so team members
  comfortable with React can contribute immediately.
- Built-in support for Tailwind 4 via `@tailwindcss/vite` plugin.
- Multiple Phenotype sibling landing pages (`phenotype-landing/`) already
  follow this pattern, enabling component reuse and shared build tooling
  understanding.

For the PM application, React + Next.js is inherited from the Plane.so fork
and remains the correct choice: the apps already exist in Plane and rewriting
them would defeat the purpose of the fork.

## Decision

Adopt a hybrid frontend stack:

| Surface            | Framework       | Rationale                                    |
|--------------------|-----------------|----------------------------------------------|
| PM apps            | Next.js + React | Inherited from Plane.so fork; no rewrite     |
| Landing page       | Astro + Bun     | Zero-JS default, Fast, sibling pattern match |

### Additional tooling decisions

| Tool          | Choice     | Rationale                                  |
|---------------|------------|--------------------------------------------|
| CSS framework | Tailwind 4 | Shared across apps and landing, inherited  |
| Runtime       | Bun        | Fast installs, TS-native, sibling pattern  |
| 3D rendering  | Three.js   | Interactive hero, GLTF placeholder support |
| Build system  | Turbo      | Inherited from Plane.so monorepo           |
| Linting       | TypeScript | Strict mode across all surfaces            |

## Consequences

### Positive

- Landing page is fully static — deploys to Vercel with zero cold starts,
  sub-100ms TTFB, and perfect Lighthouse scores.
- Astro's island architecture allows gradual hydration of interactive
  elements (Three.js hero) without loading a JS framework for static content.
- Astro + Tailwind 4 matches the pattern used by other Phenotype landing
  sites, enabling shared component libraries and deployment pipelines.
- The PM app inherits Plane's existing Next.js optimizations (ISR for
  workspace pages, RSC for data grids).

### Negative

- Two frameworks to maintain (Astro + Next.js) instead of one. Developers
  working across both surfaces must context-switch between `.astro` and
  `.tsx` component syntax.
- Component sharing between landing and app is limited — Astro cannot
  directly render React components without a client:load directive (works
  but adds JS overhead).
- Three.js bundle (172 KB minified) is loaded on the landing page even for
  users who don't interact with the hero, unless lazy-loaded.

### Neutral

- If the PM app later adopts Astro for specific static routes (docs,
  changelog, pricing), the infrastructure and expertise already exist.
- Bun as the runtime is exclusive to the landing page; the upstream Plane
  stack uses Node.js + pnpm for the app monorepo.
