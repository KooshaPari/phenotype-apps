# Operations

Day-to-day runbook for building, testing, releasing, and maintaining KWatch.

## Build

```bash
# Standard build (CGO disabled, trimmed binary)
make build
# → ./build/kwatch

# Development build (race detector, full debug info)
make dev
# → ./build/kwatch-dev

# Cross-platform release
make build-all
# → ./build/kwatch-{linux,darwin,amd64,arm64}
```

`make build` runs `go mod tidy` then `CGO_ENABLED=0 go build` with embedded `version`, `commit`, and `buildTime` set via `-ldflags`.

## Test

```bash
go test ./...
```

KWatch currently has **no Go test files** (`go test ./...` is informational and exits 0). Adding tests is tracked as future work. Do not introduce a Go test framework dependency without justification — the test surface is small and the runtime is exercised end-to-end via the daemon and CLI subcommands.

## Format & Lint

```bash
gofmt -w .
go vet ./...
```

CI does not run `gofmt`/`go vet` as a hard gate today. Run them locally before opening a PR.

## Continuous Integration

- **Platform:** GitHub Actions on `ubuntu-latest` (standard Linux runner; macOS/Windows skipped per Phenotype billing policy — see `~/.claude/CLAUDE.md`).
- **Workflows:** to be added under `.github/workflows/` as Go test/lint gates are introduced.
- **OpenSSF Scorecard:** `.github/scorecard.yml` declares the policy gate; enforced by a future `scorecard.yml` workflow.
- **CodeQL:** not yet wired. Tracked as a follow-up.

## `node_modules` Untrack Decision

The historical `node_modules/` directory contained ~7,815 tracked files after the TypeScript layer was removed in commit `e7eff93`. The decision to **not untrack** these files was made to avoid a forced git history rewrite and to preserve bisectability across the TypeScript→Go transition.

Implications:

- Clone size is large (~7,800 extra files in the worktree).
- No runtime impact: `package.json` is not invoked, `node_modules/` is not used.
- Future cleanup: a dedicated `chore: untrack node_modules` commit can be made when the project is ready to absorb the history rewrite.

## Dependabot

`.github/dependabot.yml` is configured to monitor:

- **`gomod`** ecosystem — Go module updates, weekly, grouped by minor version.
- **`github-actions`** ecosystem — Action pin updates, weekly, grouped.

Direct dependencies, dev dependencies, and Action references are all covered. The `npm` ecosystem is **not** enabled (no live Node.js surface).

## Release

Releases are cut by tagging `main` and pushing the tag:

```bash
git tag -a vX.Y.Z -m "vX.Y.Z"
git push origin vX.Y.Z
```

`make build-all` produces the per-platform binaries that get attached to the GitHub release.

## Local Dev Tips

- Use `make dev` for the race-detector build during development.
- The `.kwatch/` directory holds runtime state — safe to delete to reset.
- Logs are written to `.kwatch/kwatch.log` and to stderr.
- The `history` subcommand dumps the process-event log to stdout.
