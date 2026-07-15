# KWatch — Substrate Audit DAG Remediation Plan

> **Generated:** 2026-07-09
> **Audit Score:** ~70–75 % (B–/B)
> **Total Effort:** ~20 h (spread across 4 phases)
>
> Dependencies between items are declared as **blocks** (↓) or **weak-ordering** (→).
> Blocking = must finish before dependent starts. Weak-ordering = strong recommendation but not a hard gate.

---

## Phase 0 — Quick Wins (~2 h, 6 items)

Low-risk, high-visibility fixes that unblock later phases and eliminate noise from the scorecard.

| ID | Pillar | Task | Effort | Risk | Blocks | Weak-Order |
|----|--------|------|--------|------|--------|------------|
| P0.1 | `license-file` | Create `LICENSE` (MIT) — declared in `package.json` but missing on disk. | 5 min | none | — | — |
| P0.2 | `docs-accuracy` | Fix `SSOT.md` — says "Built with Cargo" but project is pure Go. | 5 min | none | — | — |
| P0.3 | `gosec` | Add `gosec` runner to `.github/workflows/audit.yml` alongside `govulncheck`. | 10 min | none | — | P2.4 |
| P0.4 | `housekeeping` | Delete root `deny.toml` — dormant Rust cargo-deny config, not used. | 2 min | none | — | — |
| P0.5 | `golangci-lint` | Create `.golangci.yml` config (presets, govet, errcheck, ineffassign, etc.) + add to `audit.yml`. | 45 min | low | — | P1.3, P2.4 |
| P0.6 | `release-version` | Fix `release.yml` Go version from `1.21` → `1.25` to match `go.mod`. | 5 min | none | — | — |

**Phase-0 blockers:** None. All items are independent.

**Acceptance:** Each item's CI check (or manual verify) passes before declaring done.

---

## Phase 1 — Foundational Improvements (~4 h, 4 items)

Address the three largest scoring gaps: observability, error hygiene, and test gating.

| ID | Pillar | Task | Effort | Risk | Blocks | Weak-Order |
|----|--------|------|--------|------|--------|------------|
| P1.1 | `structured-logging` | Replace `fmt.Printf` / `log.Print` with `log/slog` across the codebase. Introduce central `slog.Logger` (or `slog.NewJSONHandler`), structured fields (`component=`, `req_id=`, `duration=`), and decision-aware verbosity. | 2 h | medium (wide touch) | — | P2.1 |
| P1.2 | `error-wrapping` | Replace bare `fmt.Errorf("...")` and ad-hoc string errors with custom error types + `%w` wrapping. Define sentinel errors (`ErrNotFound`, `ErrPermission`, `ErrTimeout`) in `internal/errors/`. | 1 h | low (contained) | — | P2.1 |
| P1.3 | `coverage-gate` | Wire `Makefile` coverage target into CI (`ci.yml`): run `go test -coverprofile=coverage.out ./...` and enforce threshold (≥ 60 %). Add `cover.html` artifact upload on failure. | 30 min | low | — | — |
| P1.4 | `pre-commit` | Install `lefthook` (already present in repo root? check) or add `.pre-commit-config.yaml` with `gofmt`, `go vet`, `go mod tidy`, `golangci-lint`, `gitleaks`. | 30 min | low | — | P1.1 (avoid fmt→slog churn on staged lines) |

**Phase-1 blockers:** None. P1.1 has wide surface area but no hard block on other phases.

**Acceptance:**
- P1.1: No bare `fmt.Printf`/`log.Print` remains in library code (main/test CLIs may keep minimal `fmt.Println` for user-facing output).
- P1.2: `internal/errors/` package exists; `go vet` passes on `%w` usage.
- P1.3: CI fails when coverage drops below threshold.
- P1.4: `git commit` runs formatting + lint on staged files.

---

## Phase 2 — Structural Improvements (~6 h, 4 items)

Medium-effort changes that strengthen the security posture, test hygiene, and architectural consistency.

| ID | Pillar | Task | Effort | Risk | Blocks | Weak-Order |
|----|--------|------|--------|------|--------|------------|
| P2.1 | `codeql-sbom` | Add CodeQL workflow (`.github/workflows/codeql.yml` — `security-extended` + `security-and-quality` query suites) and SBOM generation (`.github/workflows/sbom.yml` — `CycloneDX` or `syft`). | 2 h | low | — | P3.2 |
| P2.2 | `test-split` | Split `cmd/cmd_test.go` (1 306 lines) into domain-aligned test files: `cmd/runner_test.go`, `cmd/watch_test.go`, `cmd/config_test.go`, `cmd/output_test.go`. Keep shared helpers in `cmd/testutil_test.go`. | 1.5 h | medium (merge conflicts if branch is long-lived) | — | — |
| P2.3 | `interface-alignment` | Fix Server Runner interface to match `runner.Runner` signatures. Audit all interface consumers. | 1 h | medium | — | — |
| P2.4 | `adrs` | Create `docs/adr/` directory with 2–3 seed ADRs (e.g., ADR-001: why `log/slog` over zap/logrus; ADR-002: CLI framework choice; ADR-003: architecture overview). Template from `docs/adr/template.md`. | 1.5 h | none | P0.5 (lint choices) | — |

**Phase-2 blockers:**
- P2.4 weakly ordered after P0.5 (ADR-003 can reference golangci-lint decisions).

**Acceptance:**
- P2.1: CodeQL passes on `main`; SBOM artifact appears in release assets.
- P2.2: Each `cmd/*_test.go` is ≤ 400 lines; `go test ./cmd/` still passes.
- P2.3: `go build ./...` passes; no interface mismatch at runtime.
- P2.4: `docs/adr/0001-*.md`, `0002-*.md`, `0003-*.md` exist and are linked from root `README`.

---

## Phase 3 — Advanced Hardening (~8 h, 3 items)

Higher-effort items that push the project toward A-grade maturity.

| ID | Pillar | Task | Effort | Risk | Blocks | Weak-Order |
|----|--------|------|--------|------|--------|------------|
| P3.1 | `fuzz-bench` | Add fuzz tests (`go test -fuzz=FuzzFoo -fuzztime=30s`) for core parse / watch / config paths. Add benchmark tests (`go test -bench=. -benchmem`) for hot loops. Wire into CI as optional (non-blocking, nightly). | 4 h | medium | — | — |
| P3.2 | `slsa-provenance` | Add SLSA provenance generation to `release.yml` (`slsa-github-generator`). Sign binaries with `cosign` or `minisign`. Publish checksum signatures. | 3 h | medium | P2.1 (SBOM feeds into provenance) | — |
| P3.3 | `devcontainer-lint` | Create `.devcontainer/devcontainer.json` (Go 1.25 + gopls + golangci-lint + gitleaks). Tune `.golangci.yml` with custom `linters-settings` and `issues.exclude-rules`. Add `staticcheck` presets. | 1 h | low | — | — |

**Phase-3 blockers:**
- P3.2 weakly ordered after P2.1 (SBOM is a natural input to provenance attestation).

**Acceptance:**
- P3.1: CI has a nightly fuzz workflow; `go test -bench=./...` runs without error.
- P3.2: Release artifacts include `*.intoto.jsonl` attestation; SHA256SUMS are signed.
- P3.3: Dev container boots with `gopls` and all linters pre-configured.

---

## DAG Summary

```
Phase 0                        Phase 1               Phase 2               Phase 3
───────                        ───────               ───────               ───────
P0.1 ──→ (done)
P0.2 ──→ (done)
P0.3 ──→ ─ ─ ─ ─ ─ ─ ─ ─ ─ → P2.4
P0.4 ──→ (done)
P0.5 ──→ ─ ─ ─ ─ ─ ─ ─ ─ ─ → P2.4
P0.6 ──→ (done)

                               P1.1 ──→ ─ ─ → P2.1
                               P1.2 ──→ ─ ─ → P2.1
                               P1.3 ──→ (done)
                               P1.4 ──→ (done)

                                                 P2.1 ──→ ─ ─ → P3.2
                                                 P2.2 ──→ (done)
                                                 P2.3 ──→ (done)
                                                 P2.4 ──→ (done)

                                                                      P3.1 ──→ (done)
                                                                      P3.2 ──→ (done)
                                                                      P3.3 ──→ (done)
```

| Symbol | Meaning |
|--------|---------|
| `──→`  | Hard block (must finish before dependent) |
| `─ ─ →`| Weak ordering (recommended but not blocking) |
| `(done)`| No downstream dependencies within plan |

---

## Risk Register

| Risk | Phase | Likelihood | Impact | Mitigation |
|------|-------|------------|--------|------------|
| P1.1 slog migration breaks tests | 1 | medium | medium | Do in isolated PR; run full test suite before merge |
| P2.2 test-split merge conflict | 2 | high | low | Do early in sprint; coordinate with other PRs |
| P2.3 interface changes break plugin consumers | 2 | low | medium | Tag interface as `Experimental` for one release |
| P3.2 SLSA provenance adds 5 min to release | 3 | high | low | Run in parallel with cross-compile; acceptable trade-off |

---

## Ordering Recommendation

1. Sprint 1: **P0.1–P0.6**, **P1.1**, **P1.2** (~3 h) — quick wins + observability
2. Sprint 2: **P1.3**, **P1.4**, **P2.2**, **P2.3** (~3 h) — gating + test hygiene
3. Sprint 3: **P2.1**, **P2.4** (~3.5 h) — security + ADRs
4. Sprint 4: **P3.1**, **P3.2**, **P3.3** (~8 h) — advanced hardening

Total: ~17.5 h implementation + ~2.5 h review/QA ≈ **20 h**.
