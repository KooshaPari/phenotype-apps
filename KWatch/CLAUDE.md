# KWatch

Fast, lightweight project monitoring tool for TypeScript / JavaScript projects. Exposes real-time build status through a TUI panel and an HTTP API optimized for AI agent polling. Go-only stack: daemon (TUI, server, runner, MCP, security) is Go; the historic TypeScript client / CLI bridge (the `proc-service` npm package, with source files `proc-service.ts` / `proc-cli.ts`) was removed in commit `e7eff93` and is not part of the current build. KWatch itself is Go-only — the "TypeScript / JavaScript projects" wording in the lead refers to the *projects KWatch monitors*, not to KWatch's own source.

## Stack

| Layer | Technology |
|-------|------------|
| Daemon (TUI, server, runner, MCP, security) | Go 1.21 (charmbracelet/bubbletea, fsnotify, cobra) |
| (historic) Client / CLI bridge | TypeScript / Node.js — removed in `e7eff93`; `find . -name '*.ts' -not -path '*/node_modules/*'` returns no source files. The `package.json` `dependencies` block, `dev`/`cli`/`proc:*` scripts, and the `.eslintrc.json` are leftovers; see "Useful entry points" below. |
| Testing | Go `go test ./...` is the meaningful test surface; Jest is gated (no `jest.config.mjs`, no test files) — see `CONTRIBUTING.md` "Test framework status" |
| Lint | ESLint (`.eslintrc.json`) — current source is Go only, so the lint step is effectively a no-op until either the TS layer is restored or the script is rewritten for the Go-only stack |
| Build | `go build ./...` (via Makefile → `build/kwatch`); `tsc` is a no-op (no `.ts` source files) |
| Format | `gofmt -l .` (Go only; enforced by CI) |

## Key Commands

```bash
# Go daemon
go build ./...
go test  ./...
gofmt -l .       # must be empty (CI-enforced)

# TypeScript client / CLI
npm install
npm run build
npm run lint
# npm test — see CONTRIBUTING.md "Test framework status" before running
```

Useful entry points: `main.go` (daemon bootstrap), `cmd/` (subcommands), `kwatch` (compiled binary), `runner/`, `server/`, `tui/`, `mcp/`, `security/` (Go daemon modules). The TypeScript client/CLI layer (`proc-service.ts`, `proc-cli.ts`) referenced by older revisions was removed in commit `e7eff93`; `package.json` still carries the `dev`/`cli`/`proc:*` scripts that pointed at those files and they will fail at runtime until the TS layer is restored or the scripts are rewritten for the Go-only stack. The `package.json` `dependencies` block (4 packages: `@ffmpeg-installer/ffmpeg`, `blessed`, `chokidar`, `playwright`) is also a holdover from the deleted TS layer — no source file in the repo imports any of them, so `npm install` pulls ~4 unused packages and their transitive deps for no reason. Same fix: restore the TS layer, or strip the dead deps as part of the Go-only migration. Finally, `node_modules/` is also a holdover from that era — `git ls-files node_modules | wc -l` returns **7815** tracked files despite `.gitignore` line 6 listing `node_modules/`. The files were added in the historical commit `d689e8e` ("tmp") before the gitignore was tightened in `e7eff93`; the untrack would need a `git rm -r --cached node_modules/` (preserves the on-disk copy, untracks the index) and is a large 7815-file diff that needs explicit owner authorization before running. The gitignore itself is correct — new `npm install` output is ignored, and `git check-ignore -v node_modules/foo` returns the expected match. See CHANGELOG.md.

## Key Files

- `main.go` — Daemon bootstrap and config wiring
- `cmd/` — Subcommand entry points
- `kwatch` — Compiled daemon binary (Linux ELF, built from `main.go` via the Makefile)
- `runner/`, `server/`, `tui/`, `mcp/`, `security/` — Go daemon modules
- `package.json` — Node scripts (`build`, `lint` are the only ones that still work; `dev`/`cli`/`proc:start`/`proc:build` reference `proc-service.ts` / `proc-cli.ts` that no longer exist and will fail at runtime — see the Useful entry points note above)
- `Makefile` — Build / install helpers
- `screenshots/` — TUI / web demo gifs and PNGs
- `AGENTS.md` — Local agent governance (canonical for working conventions)
- `CONTRIBUTING.md` — Contributor guide (AgilePlus mandate, branch conventions, PR expectations)
- `SECURITY.md` — Vulnerability disclosure path
- `.github/dependabot.yml` — gomod / github-actions weekly updates (npm intentionally not enabled — see the YAML comment for the rationale and re-enable condition)
- `.github/ISSUE_TEMPLATE/` and `.github/pull_request_template.md` — issue templates and PR template
- `.editorconfig`, `.gitattributes` — formatting / line-ending rules

## Reference

- **Local source of truth for agent behavior:** `AGENTS.md`
- **Global Phenotype rules:** `~/.claude/CLAUDE.md` or `/Users/kooshapari/CodeProjects/Phenotype/repos/CLAUDE.md`
- **AgilePlus work tracking:** `cd /repos/AgilePlus && agileplus <command>` (required for non-trivial work per the CONTRIBUTING mandate)
