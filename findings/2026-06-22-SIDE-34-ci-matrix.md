# SIDE-34: GitHub Actions CI Matrix Coverage for pheno-* Crates

**Date:** 2026-06-21
**Scope:** All 25 `pheno-*` crates visible under `repos/` (sparse-checkout cone)
**Mode:** Read-only audit
**Source of truth:** `<crate>/.github/workflows/*.yml` (locally checked-in copies)

---

## 1. Summary

| Bucket | Count | % of 25 |
|---|---:|---:|
| **Full matrix (Ubuntu + macOS + Windows)** | **0** | **0 %** |
| **Ubuntu + macOS** | 3 | 12 % |
| **Ubuntu only** | 10 | 40 % |
| **Workflows present but NO test job** (perf-only / deny / secret-scan / scorecard / etc.) | 3 | 12 % |
| **No `.github/workflows/` at all** | 9 | 36 % |
| **Reusable CI template repo (not a tested crate)** | 1 (pheno-ci-templates) | 4 % |
| **macOS coverage** | 3 / 25 | 12 % |
| **Windows coverage** | 0 / 25 | 0 % |

**Headline finding:** **0 of 25 pheno-* crates test on `windows-latest`.** Cross-platform regressions on Windows will not be caught by CI.

---

## 2. Per-crate matrix

Legend:
- ✅ = runner present in test job matrix
- ❌ = runner absent
- ⚪ = not applicable (no test job, or no workflows)
- `lang` = primary language (R=Rust, Py=Python, Go=Go, TS=TypeScript, etc.)
- `n/a-win?` = "is Windows a meaningfully-applicable runner for this crate's tests?" (Rust/Python/Go all run on Windows; pure-curl/POSIX-shell test suites are usually Linux-only)

| # | Crate | Lang | `.github/workflows/*.yml` | Test job present? | Ubuntu | macOS | Windows | n/a-win? | Notes |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `pheno-chaos` | Rust | *(none)* | ⚪ | ❌ | ❌ | ❌ | yes | **No CI.** |
| 2 | `pheno-ci-templates` | n/a | `ci.yml`, `release.yml` | — (reusable template) | — | — | — | — | Shared template repo; matrix lives in **consumer** `ci.yml`s (DRY per ADR-022). |
| 3 | `pheno-cli-base` | Rust | `ci.yml` | yes | ✅ | ❌ | ❌ | yes | Single `runs-on: ubuntu-latest` test job. |
| 4 | `pheno-config` | Rust | *(none)* | ⚪ | ❌ | ❌ | ❌ | yes | **No CI.** |
| 5 | `pheno-context` | Rust | *(none)* | ⚪ | ❌ | ❌ | ❌ | yes | **No CI.** |
| 6 | `pheno-drift-detector` | Rust | *(none)* | ⚪ | ❌ | ❌ | ❌ | yes | **No CI.** |
| 7 | `pheno-errors` | Rust | `ci.yml` + 4 others (audit/deny/codeql/governance) | yes | ✅ | ❌ | ❌ | yes | Single `runs-on: ubuntu-latest`. |
| 8 | `pheno-events` | Rust | *(none)* | ⚪ | ❌ | ❌ | ❌ | yes | **No CI.** |
| 9 | `pheno-fastapi-base` | Py | `ci.yml` | yes | ✅ | ❌ | ❌ | yes | FastAPI: Windows runner would apply but not configured. |
| 10 | `pheno-flags` | Rust | *(none)* | ⚪ | ❌ | ❌ | ❌ | yes | **No CI.** |
| 11 | `pheno-framework-lint` | Rust | *(none)* | ⚪ | ❌ | ❌ | ❌ | yes | **No CI.** |
| 12 | `pheno-go-ctxkit` | Go | `ci.yml` | yes | ✅ | ✅ | ❌ | yes | Matrix `os: [ubuntu-latest, macos-latest]`. |
| 13 | `pheno-llms-txt` | Py | `ci.yml` | yes | ✅ | ❌ | ❌ | yes | pytest + 80 % coverage gate per ADR-040. |
| 14 | `pheno-mcp-router` | Rust | `perf-gate.yml` | **NO test job** | ⚪ | ⚪ | ⚪ | yes | **Only `perf-gate.yml` — no `cargo test` job.** |
| 15 | `pheno-otel` | Rust | `ci.yml` + 5 others (audit/deny/lint/scorecard/release) | yes | ✅ | ✅ | ❌ | yes | Matrix `os: [ubuntu-latest, macos-latest]`. |
| 16 | `pheno-port-adapter` | Rust | `ci.yml` + 3 others (audit/lint/scorecard) | yes | ✅ | ❌ | ❌ | yes | Single `runs-on: ubuntu-latest`. |
| 17 | `pheno-predict` | Rust | *(none)* | ⚪ | ❌ | ❌ | ❌ | yes | **No CI.** |
| 18 | `pheno-pydantic-models` | Py | `ci.yml` | yes | ✅ | ✅ | ❌ | yes | Matrix `os: [ubuntu-latest, macos-latest]`. |
| 19 | `pheno-scaffold-kit` | Py | `ci.yml` | yes | ✅ | ❌ | ❌ | yes | pytest + 70 % coverage gate per ADR-040 (framework tier). |
| 20 | `pheno-secret-scan` | Shell | `deny.yml`, `secret-scan.yml` | **NO test job** | ⚪ | ⚪ | ⚪ | n/a | tooling/script repo; no unit tests by design. |
| 21 | `pheno-ssot-template` | Rust | `ci.yml` + 3 others (audit/lint/scorecard) | yes | ✅ | ❌ | ❌ | yes | MSRV cargo test on Ubuntu. |
| 22 | `pheno-tracing` | Rust | `ci.yml` + `sbom.yml` | yes | ✅ | ❌ | ❌ | yes | test + OTLP smoke test on Ubuntu. |
| 23 | `pheno-vibecoding-guard` | Py | `ci.yml` | yes | ✅ | ❌ | ❌ | yes | pytest + 80 % coverage gate per ADR-040. |
| 24 | `pheno-worklog-schema` | Py | `ci.yml` | yes | ✅ | ❌ | ❌ | yes | pytest + 80 % coverage gate per ADR-040. |
| 25 | `pheno-zod-schemas` | TS | *(none)* | ⚪ | ❌ | ❌ | ❌ | yes | **No CI.** |

---

## 3. Aggregated matrix coverage

| Runner | Crates covering | Crates (%) |
|---|---:|---:|
| `ubuntu-latest` | 13 / 25 | 52 % |
| `macos-latest` | 3 / 25 | 12 % |
| `windows-latest` | 0 / 25 | 0 % |

**Cross-platform floor (Ubuntu ∩ macOS ∩ Windows) coverage: 0 / 25 (0 %).**

---

## 4. Gap analysis

### 4.1 Crates with NO `.github/workflows/` directory (9)

`pheno-chaos`, `pheno-config`, `pheno-context`, `pheno-drift-detector`, `pheno-events`, `pheno-flags`, `pheno-framework-lint`, `pheno-predict`, `pheno-zod-schemas`.

All 9 are Rust (except `pheno-zod-schemas` which is TypeScript). Windows runner applies to all of them.

### 4.2 Crates with workflows but NO test job (3)

- `pheno-mcp-router` — only `perf-gate.yml`. **Missing `ci.yml` with `cargo test`.** This is the most critical gap given `pheno-mcp-router` is on the substrate canonicals list (ADR-013 / ADR-037).
- `pheno-secret-scan` — `deny.yml` + `secret-scan.yml` (intentional — pure shell tooling). Acceptable.
- `pheno-ci-templates` — reusable template, not a tested crate. Acceptable.

### 4.3 Windows runner never configured

Zero crates include `windows-latest`. Many of these crates use only portable stdlib APIs (Rust `std::fs`, Go `os` package, Python `pathlib`), but several have POSIX-specific behavior that would break on Windows:

- `pheno-config` (TOML paths with `~/` expansion)
- `pheno-cli-base` (`clap` argument parsing — known to differ on Windows path handling)
- `pheno-errors` (any error chains referencing paths)
- `pheno-events` (FS event watchers)
- `pheno-fastapi-base` (uvicorn worker spawn — Windows uses `SelectReader` not `epoll`)

### 4.4 macOS-only coverage gap (22 / 25)

Most crates test on Linux only. Substrate canonicals should at minimum test on Linux + macOS to catch BSD-vs-Linux syscall differences (e.g. `SO_REUSEADDR`, file-locking semantics in `pheno-config`).

The 3 crates that do test on macOS:
- `pheno-go-ctxkit` (Go — matrix is idiomatic)
- `pheno-otel` (Rust OTel exporter)
- `pheno-pydantic-models` (Python)

---

## 5. Recommendations (out of scope, this turn is read-only)

Listed for traceability; no actions taken.

1. **`pheno-mcp-router`** — add `ci.yml` with `cargo test --all-features --locked` on `ubuntu-latest` + `macos-latest`. (ADR-013 / ADR-037 substrate canonical.)
2. **Windows runner** — add to matrix for substrate canonicals (`pheno-otel`, `pheno-tracing`, `pheno-config`, `pheno-port-adapter`, `pheno-errors`). Cost: ~3× runner-minutes per workflow; mitigation: use `fail-fast: false` and limit to substrate tier.
3. **9 crates with no CI** — bootstrap with `pheno-ci-templates/ci.yml` pattern (copied via reusable workflow reference, not fork). Each adds ~50 lines of YAML.
4. **macOS runner** — extend from 3/25 → at minimum 12/25 (substrate canonicals + framework tier per ADR-040).
5. **Cross-cutting matrix policy** — codify as ADR (suggested ADR-081 candidate for a future wave): "Every `pheno-*-lib` substrate MUST test on Ubuntu + macOS minimum; federated services + framework tier MUST additionally test on Windows."

---

## 6. Verification commands (reproducible)

```bash
# Inventory
ls -d pheno-*/                                      # 25 dirs

# Per-crate workflow listing
for d in pheno-*/; do
  echo "=== $d ==="
  ls "$d/.github/workflows" 2>/dev/null || echo "(none)"
done

# Matrix extraction (Ubuntu/macOS/Windows)
for f in pheno-*/.github/workflows/ci.yml; do
  echo "=== $f ==="
  grep -E "(runs-on:|matrix:|os:|ubuntu-latest|macos-latest|windows-latest)" "$f" | head -10
done
```

---

## 7. Side-task metadata

| Field | Value |
|---|---|
| Side-task ID | **SIDE-34** |
| Wave | v19 (cycle 9) |
| Owner | orch-w1-a |
| Device | macbook (read-only, no builds) |
| Worklog | (none — task is single-shot read-only audit, not multi-step) |
| Inputs | AGENTS.md (SIDE-task pattern), `findings/2026-06-22-*.md` (existing naming convention) |
| Outputs | `findings/2026-06-22-SIDE-34-ci-matrix.md` (this file) |
| Estimated LoC | 0 (no code changes) |
| ADR affected | none this turn (recommendation #5 proposes ADR-081 for a future wave) |
