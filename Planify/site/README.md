# planify.space — landing site

Static landing page for [Planify](https://github.com/KooshaPari/Planify).

## Stack

- Astro 6 + Bun + Tailwind 4 (matches sibling Phenotype landings in `phenotype-landing`)
- Three.js for the hero 3D scene (placeholder keyboard geometry; will swap to `keyboard.glb` once added)

## Develop

```bash
bun install
bun run dev          # http://localhost:4321
bun run build
```

## Deploy

Vercel — see `vercel.json`. Domain: `planify.space` (or `planify.kooshapari.com`).

## File map

```
site/
├── astro.config.mjs
├── vercel.json
├── tsconfig.json
├── package.json
├── data/
│   └── config.json          # single source of truth for the page
├── public/
│   └── favicon.svg
└── src/
    ├── pages/
    │   └── index.astro      # landing page
    ├── components/
    │   └── HeroScene.astro  # 3D canvas (Three.js)
    └── styles/              # (Tailwind handles globals via @tailwindcss/vite)
```

## Asset TODO

- `public/keyboard.glb` — drop the keyboard `.glb` here and the HeroScene component will
  pick it up via GLTFLoader. Until then the placeholder geometry renders.
