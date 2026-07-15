> 📋 Read [`CONTRIBUTING.md`](./CONTRIBUTING.md) and [`AGENTS.md`](./AGENTS.md) before opening this PR. Non-trivial features require a linked AgilePlus spec.

## Summary

<!-- What does this PR do? -->

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Security fix

## Affected surface

- [ ] Go daemon (TUI, server, runner, MCP, security)
- [ ] TUI panel
- [ ] HTTP API / MCP server
- [ ] Docs / screenshots

Note: KWatch itself is Go-only. The "TypeScript client / CLI bridge" line that used to be in this checklist was removed when the TS source layer was deleted in `e7eff93` (along with `proc-service.ts` / `proc-cli.ts`); if you want to bring the TS layer back, the checkbox can be re-added once the source is restored. See `AGENTS.md` and `CLAUDE.md` for the full picture.

## Testing

- [ ] `go build ./...`
- [ ] `go test ./...` (currently vacuous — `find . -name '*_test.go'` returns no files in the repo, so the command exits 0 with no test signal; check this box only when your PR actually adds Go tests, otherwise treat as informational)
- [ ] `gofmt -l .` (must be empty)
- [ ] `go vet ./...`
- [ ] `npm run build` (no-op on current source tree — no `.ts` source files since `e7eff93`; check this box only if your PR restores the TS layer or rewrites the script for the Go-only stack)
- [ ] `npm run lint` (no-op on current source tree — same caveat as `npm run build`)
- [ ] `npm test` (only meaningful once a jest config + tests exist; see `CONTRIBUTING.md` "Test framework status" — currently the script fails fast with "ENOENT: no such file or directory, open 'jest.config.mjs'" on a fresh checkout, so check this box only when the framework is actually wired up in this PR)
- [ ] Manual smoke test (TUI and/or web) (if applicable)
- [ ] Updated TUI/web screenshots in `screenshots/` (if UI changed)

## Spec / Traceability

<!-- Link the AgilePlus spec, FR IDs, or ADR that this change implements -->
- Spec:
- FR / NFR:

## Risks & Rollback

<!-- Known risks, breaking changes, and how to roll back if needed -->

## Related

<!-- Issues this PR closes; PRs/specs this depends on -->
Closes #
