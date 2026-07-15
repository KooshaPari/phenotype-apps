# FAQ

## Why is `package.json` still in the repo?

`package.json` is a **vestigial manifest** from the historical TypeScript layer that was removed in commit `e7eff93`. The Go build does not consult it, and no `npm`/`bun`/`yarn` command is invoked at build, test, or runtime.

It is kept in the tree (rather than deleted) for two reasons:

1. **Auditability** — preserves evidence of the prior stack for anyone reviewing git history.
2. **No-op safety** — `package.json` defines `scripts.test` as `jest --config jest.config.mjs`, but no `jest.config.mjs` exists, so `npm test` is a no-op (it errors with "no such file" if invoked). This is intentional: a stray `npm test` in a CI script cannot accidentally execute Go tests or change the Go build.

If a future cleanup PR deletes `package.json` entirely, no functional change is expected. Track the decision in `CHANGELOG.md`.

## Why is `npm test` a no-op?

Because the TypeScript client and CLI bridge were removed in commit `e7eff93`, the `jest.config.mjs` referenced by `package.json` no longer exists. Invoking `npm test` will fail with a file-not-found error from Jest, not from KWatch. This is **expected** — KWatch's test surface is the Go test suite (`go test ./...`), which is itself informational today (see `docs/OPERATIONS.md`).

If you see `npm test` in a script or doc, treat it as a stale reference and replace with `go test ./...` or `make test` (when wired up).

## Was the TypeScript client removed?

**Yes.** Commit `e7eff93` removed the TypeScript client (`proc-service.ts`), the CLI bridge (`proc-cli.ts`), the build config (`tsconfig.json`, `jest.config.mjs`), the package manifest dependencies, and the `node_modules/` contents. The only TypeScript artifacts remaining in the working tree are inside the untracked-vestigial `node_modules/typescript/` directory (see `docs/OPERATIONS.md` for the untrack decision).

The current KWatch codebase is **100% Go** for the runtime. There is no `tsc`, `jest`, `ts-node`, or `eslint` invocation in any active code path.

## How do I add a new subcommand?

See `docs/ARCHITECTURE.md` → "Extension Points". Short version: add a file under `cmd/`, register it in `cmd/root.go`, and document it in `README.md`.

## How do I run the daemon?

```bash
make build
./build/kwatch daemon
```

The daemon writes state to `.kwatch/` in the working directory. Inspect with `./build/kwatch status` or `./build/kwatch history`.
