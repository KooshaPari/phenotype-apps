# AGENTS.md — planify-wt

> Repo-level instruction file for any AI agent working in this directory.

## What this is
A Plane.so fork focused on the public landing site (`planify.space`). The upstream runtime is **never modified** in this repo — only the `site/` landing, `infra/` docker-compose, `docs/`, and root files are ours.

## Hard rules
1. **Never modify `upstream/`.** That directory is the verbatim snapshot of `makeplane/plane@preview v1.3.1` and serves as our reference for upstream PRs.
2. **Never modify `site/src/components/*` to match a different upstream version** without first rebasing on upstream. We diverge intentionally in `site/`.
3. **Don't add secrets to `site/.env`.** Use Vercel env vars for any non-public keys.
4. **Bun for site** (Astro's recommendation). Don't switch to Node pnpm here.
5. **Update `CHANGELOG.md` for every site/ change.** Use the [Unreleased] section.

## Workflow rules
- Use `just` (canonical) or `task` (fallback) at `repos/planify-wt/justfile`.
- Tests: `just test`. Build: `just build`. Deploy: `vercel --prod`.
- The PM web runtime (sprints, kanban) is a future sibling — it lives in `AgilePlus/crates/agileplus-plane` and is built separately.

## Compliance
- Phenotype governance applies. See `../PHENOTYPE_DOGFOOD_COMPLIANCE_AUDIT.md` for the cross-project scorecard.
- ADRs in `docs/adrs/`, specs in `docs/specs/`.

## Common tasks

| Task | Recipe |
|---|---|
| Dev server | `just dev` |
| Build | `just build` |
| Deploy to Vercel | `just deploy-vercel` |
| Pull upstream | see `UPSTREAM.md` |
