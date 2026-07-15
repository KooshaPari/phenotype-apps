# Contributing to KWatch

Thanks for your interest in contributing to **KWatch**, part of the [Phenotype](https://github.com/KooshaPari) ecosystem.

## AgilePlus spec mandate

All non-trivial work in this organization is tracked in **AgilePlus**. Before opening a PR for a feature or substantive change:

1. Check the [AgilePlus](https://github.com/KooshaPari/AgilePlus) spec registry for an existing spec.
2. If none exists, open one (`agileplus specify --title "<feature>" --description "<desc>"`) and link it from your PR description.
3. Trivial fixes (typos, dependency bumps, doc tweaks, governance file additions) do not require a spec.

## Build & test

KWatch is a **Go-only** project. The historic TypeScript client/CLI bridge (the `proc-service` npm package, with source files `proc-service.ts` / `proc-cli.ts`) was removed in commit `e7eff93` and is not part of the current build. The npm-related commands below are present in `package.json` but operate on a no-op source tree; they exit 0 vacuously and are not load-bearing. See `AGENTS.md` "Quality gates" and `CLAUDE.md` "Useful entry points" for the full picture.

```bash
# Go daemon (1.21+) — these are the load-bearing checks
go build ./...
go vet   ./...
gofmt -l .   # must be empty (CI-enforced)

# go test ./... is also part of the workflow, but currently
# vacuous — find . -name '*_test.go' returns no files in the
# repo, so the command exits 0 with no test signal. Treat a
# clean go test as informational only, not as evidence the
# code is tested. A future PR that adds Go tests will have
# the gate run automatically.

# Node-side (NOT load-bearing — no .ts source files since
# e7eff93). Listed for completeness; running these is optional
# and they will succeed vacuously.
npm install        # pulls deps but no source uses them
npm run build      # tsc with no .ts files = no-op
npm run lint       # eslint with no .ts/.js files = no-op
npm test           # gated; see "Test framework status" below
```

The CI workflow at `.github/workflows/ci.yml` runs both jobs on every push, PR, and manual dispatch. The Go job also runs `gofmt -l .` and fails the build if any file is not gofmt-clean. Local checks should be green before pushing.

### Test framework status

KWatch's `package.json` declares `jest` and `@types/jest` as devDependencies, and the `test` script is `jest --config jest.config.mjs`, but the repository currently has no `jest.config.mjs` and no test files (no `*.test.ts`, no `*.spec.ts`, no `__tests__/` directory). To keep CI green, the Node test step is **gated** on both a jest config and at least one test file being present. When either is missing, the step prints a `::notice::` and exits 0; when both are present, `npm test` runs normally.

Note on the current state: the TypeScript source layer that the Node test infrastructure was designed for was removed in commit `e7eff93` (along with `proc-service.ts` / `proc-cli.ts`). `find . -name '*.ts' -not -path '*/node_modules/*'` returns no source files, so the recipe below is **forward-looking guidance** for a future PR that re-introduces a Node-side surface (or fully strips the test framework as part of a Go-only migration). Right now, the meaningful test surface is **Go only** — run `go test ./...` from the repo root, which uses the standard Go toolchain. The recipe below should not be followed verbatim until the TS layer is restored, or until the recipe is rewritten to point at the Go test surface.

If you're adding Node tests (only after the TS layer is restored):

1. Add a `jest.config.mjs`. Jest 30 has no built-in TypeScript transformer, so you'll also need one of `ts-jest`, `@swc/jest`, or `babel-jest` plus `@babel/preset-typescript`. The `tsc` step in `npm run build` already produces `dist/`, so a simple config that points jest at compiled JS is also viable.
2. Add at least one test file under any of the standard globs.
3. The CI test step will then run automatically with no workflow changes.

## Branch naming

Use kebab-case prefixed by intent:

- `feat/<scope>-<short-desc>`     — new feature
- `fix/<scope>-<short-desc>`      — bug fix
- `chore/<scope>-<short-desc>`    — tooling, deps, infra
- `docs/<scope>-<short-desc>`     — docs only
- `refactor/<scope>-<short-desc>` — non-behavioral change

## Commit messages

Follow [Conventional Commits](https://www.conventionalcommits.org/). Examples:

- `feat(tui): add keyboard shortcut to toggle panel sections`
- `fix(server): handle malformed JSON in build webhook`
- `chore(deps): bump bubbletea to v0.25`

If a `commitlint.config.*` exists in the repo, it is enforced; otherwise the convention above is the floor.

## Pull request expectations

- Keep PRs focused and small; split unrelated changes.
- Ensure the build, tests, lint, and format checks above pass locally before pushing.
- Describe **what** changed and **why**. Link the AgilePlus spec, issue, or ADR.
- Touched UI surfaces: refresh screenshots in `screenshots/` so the README demos stay accurate.
- Expect review from a maintainer; be responsive to feedback.
- Squash-merge is the default; the PR title becomes the commit subject.

## Quality gates

This repo participates in the Phenotype quality regime: zero new lint suppressions without justification, traceability to FR IDs where applicable, and 0-error CI on Linux runners. See `AGENTS.md` for repo-specific governance.

## Reporting issues

Open a GitHub issue with reproduction steps, expected vs. actual behavior, and environment details (OS, Go version, Node version, watched-project stack and version, TUI vs. web API).

## Security

Do not open public issues for security findings. See `SECURITY.md` for the private disclosure path.
