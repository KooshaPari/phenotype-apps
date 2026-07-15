# VIBECODING_GUARD_BASELINE_2026_06_12.md

Protected paths for KWatch (vibecoding-guard do-not-touch zones).

1. `go.mod` — Go module manifest; changes affect the dependency graph and minimum Go version
2. `go.sum` — Go dependency checksums; changes silently alter the transitive dependency graph
3. `AGENTS.md` — Agent governance file; protects working conventions and quality gates
4. `cmd/security.go` — Security-critical CLI surface; changes must be reviewed for security implications
5. `config/config.go` — Configuration schema and defaults; changes affect runtime behavior for all users
