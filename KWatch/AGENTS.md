# AGENTS.md — KWatch

This file governs work inside the KWatch repository.

## Identity

KWatch is a fast, lightweight project monitoring tool for TypeScript / JavaScript projects. It exposes real-time build status through a TUI panel and an HTTP API optimized for AI agent polling. KWatch itself is **Go-only** — the daemon (TUI, server, runner, MCP, security) is Go, and that is the entire runtime surface. The historic TypeScript client/CLI bridge (the `proc-service` npm package, with source files `proc-service.ts` / `proc-cli.ts`) was removed in commit `e7eff93` and is not part of the current build. The "TypeScript / JavaScript projects" wording refers to the *projects KWatch monitors*, not to KWatch's own source.

Do not apply parent shelf instructions (e.g. `/Users/kooshapari/CodeProjects/Phenotype/repos/AGENTS.md` or `~/.claude/AGENTS.md`) unless explicitly referenced. Work from this directory and treat paths as local to KWatch.

## Quick Links

- **Local CLAUDE.md:** Present (`./CLAUDE.md`); this AGENTS.md is the source of truth for cross-cutting rules, CLAUDE.md is the Claude-specific entry point mirroring the McpKit stack template.
- **Phenotype org governance:** `/Users/kooshapari/CodeProjects/Phenotype/repos/CLAUDE.md` (consult when touching cross-repo contracts).
- **Global agent guidance:** `~/.claude/AGENTS.md` (consult for global defaults).
- **AgilePlus work tracking:** `cd /repos/AgilePlus && agileplus <command>` — required for non-trivial work per the CONTRIBUTING mandate.

## Working Conventions

- **Branch naming:** `<type>/<topic>` in kebab-case, conventional commits (`feat:`, `fix:`, `chore:`, `refactor:`). See `CONTRIBUTING.md`.
- **PR expectations:** Use `.github/pull_request_template.md`. Each PR links an AgilePlus spec or issue, runs the Go checks (Go is the only meaningful test surface — see Quality gates below), and updates TUI/web screenshots if the UI changed.
- **Quality gates:** `go build ./...`, `gofmt -l .` (must be empty), `go vet ./...` — all green before requesting review. These are the load-bearing gates: build catches compile errors, gofmt catches formatting drift, go vet catches suspicious constructs. `go test ./...` is listed for completeness but **currently vacuous** — `find . -name '*_test.go'` returns no files in the repo, so the command exits 0 with no test signal. A future PR that adds Go tests will have the gate run automatically; until then, treat `go test ./...` as informational and **do not** count a clean `go test` as evidence the code is tested. The `npm run build` and `npm run lint` scripts are present in `package.json` but operate on a no-op source tree (no `.ts` source files since `e7eff93`); they will exit 0 vacuously and are not load-bearing for review. `npm test` only applies once a test framework is wired up (jest config + transformer + at least one test file); see `CONTRIBUTING.md` "Test framework status" for the recipe (which itself is forward-looking — see CLAUDE.md "Useful entry points" for the full picture).
- **Security disclosures:** Follow `SECURITY.md`; never open public issues for security findings.
- **Traceability:** Substantive work links FR IDs or an ADR. JSDoc/GoDoc on public surfaces.

## Do / Don't

- **Do** keep changes focused; split unrelated work into separate PRs.
- **Do** prefer Go for new features; the TypeScript client/CLI surface is gone and the package is Go-only. If a future PR brings the TS layer back, prefer TypeScript only for client/CLI surfaces and Go for everything else.
- **Do** re-run the demo (`make dev`) and refresh screenshots if the TUI or web UI visibly changed. (`npm run dev` is broken — it invokes `ts-node proc-service.ts`, but `proc-service.ts` was deleted in `e7eff93`; use `make dev` for the working Go daemon dev build.)
- **Don't** add new lint suppressions without justification in the PR body.
- **Don't** introduce new top-level dependencies without first checking the existing `go.mod` / `package.json` and proposing the addition in the PR.
- **Don't** bypass the security policy in `SECURITY.md` for any reason; if a finding is sensitive, follow the private reporting path.

## Status

This AGENTS.md is living governance for KWatch. Update it when the working conventions change, and link any new tooling, scripts, or process notes here.
