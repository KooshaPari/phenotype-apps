# Spec 0001: planify.space landing page

## Sections
1. **Hero** — single headline + sub + CTA → "Get early access"
2. **Features grid** — 6 cards (3×2):
   - Cycles
   - Modules
   - Pages
   - Views
   - Analytics
   - AI (auto-archive, drafting, etc.)
3. **3D keyboard** — the marquee interactive element (`.glb` model)
4. **Use cases** — DevOps / Product / Design / Personal
5. **Pricing** — 3 tiers
6. **Footer** — links + brand

## Performance budget
- LCP < 1.0s
- TTI < 2.0s
- Total bundle < 200kB gzipped

## Hosting
- `planify.space` and `www.planify.space` → Vercel
- API at `pm.phenotype.tailnet` → Caddy reverse proxy → AgilePlus PM server
