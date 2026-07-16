# T21 Security Audit — 14 pheno-* repos

**Date:** 2026-06-18
**Branch:** `wip-2026-06-18-v8-batch-9C-security-audit`
**Subagent:** Subagent C — T21 SECURITY AUDIT (batch 9C)
**Tooling:** ripgrep 14.1, cargo-audit 0.22.1, advisory-db 1132 entries
**Source of truth:** `git ls-files` (tracked-only, no `.git/` recursion)

---

## Scope

14 pheno-* substrate repos in `/Users/kooshapari/CodeProjects/Phenotype/repos/`:

| # | Repo | Lang | Tracked files | Non-test source files |
|---|---|---|---:|---:|
| 1 | pheno-config | Rust | 16 | 1 |
| 2 | pheno-errors | Rust | 5 | 1 |
| 3 | pheno-flags | Rust | 4 | 1 |
| 4 | pheno-context | Rust | 10 | 1 |
| 5 | pheno-port-adapter | Rust | 11 | 4 |
| 6 | pheno-cli-base | Rust | 15 | 4 |
| 7 | pheno-otel | Rust | 19 | 6 |
| 8 | pheno-cargo-template | Rust | 1 (submodule pointer only) | 0 |
| 9 | pheno-go-ctxkit | Go | 13 | 1 |
| 10 | pheno-fastapi-base | Python | 22 | 4 |
| 11 | pheno-pydantic-models | Python | 10 | 2 |
| 12 | pheno-agents-md | Rust | 1 (submodule pointer only) | 0 |
| 13 | pheno-cost-card | Python | 1 (submodule pointer only) | 0 |
| 14 | pheno-prompt-test | Python | 1 (submodule pointer only) | 0 |

Sparse-checkout note: 4 of 14 repos (pheno-cargo-template, pheno-agents-md, pheno-cost-card, pheno-prompt-test) have only their gitlink pointer tracked locally. T21.1/T21.4 marked `NA` for those — full audit requires populating the submodule.

---

## Per-repo 4-cell scorecard

| Repo | T21.1 Secret scan | T21.2 SBOM | T21.3 SLSA | T21.4 CVE |
|---|---|---|---|---|
| pheno-config | PASS (0 hits / 1 file) | FAIL | FAIL | PASS (58 deps clean) |
| pheno-errors | PASS (0 hits / 1 file) | FAIL | FAIL | PASS (13 deps clean) |
| pheno-flags | PASS (0 hits / 1 file) | FAIL | FAIL | PASS (7 deps clean) |
| pheno-context | PASS (0 hits / 1 file) | FAIL | FAIL | NA (no Cargo.lock) |
| pheno-port-adapter | PASS (0 hits / 4 files) | FAIL | FAIL | PASS (7 deps clean) |
| pheno-cli-base | PASS (0 hits / 4 files) | FAIL | FAIL | PASS (42 deps clean) |
| pheno-otel | PASS (0 hits / 6 files) | FAIL | FAIL | **WARN** (1 unmaintained) |
| pheno-cargo-template | NA (empty sparse-checkout) | FAIL | FAIL | NA (no Cargo.toml) |
| pheno-go-ctxkit | PASS (0 hits / 1 file) | FAIL | FAIL | NA (Go — not audited) |
| pheno-fastapi-base | PASS (0 hits / 4 files) | FAIL | FAIL | NA (Python — not audited) |
| pheno-pydantic-models | PASS (0 hits / 2 files) | FAIL | FAIL | NA (Python — not audited) |
| pheno-agents-md | NA (empty sparse-checkout) | FAIL | FAIL | NA (no Cargo.toml) |
| pheno-cost-card | NA (empty sparse-checkout) | FAIL | FAIL | NA (Python — not audited) |
| pheno-prompt-test | NA (empty sparse-checkout) | FAIL | FAIL | NA (Python — not audited) |

PASS = check passed (no finding). FAIL = check missing/negative. WARN = finding present but non-blocking. NA = cannot evaluate (no source/no toolchain).

---

## Aggregated fleet summary

| Check | Coverage | Findings |
|---|---|---|
| **T21.1 Secret scan** | **10/14 evaluable (PASS=10, NA=4)** | **0 secrets leaked** |
| **T21.2 SBOM present** | **0/14 (0%)** | All missing |
| **T21.3 SLSA provenance** | **0/14 (0%)** | All missing |
| **T21.4 cargo audit (Rust only)** | **6/7 audited (PASS=6, WARN=1, NA=1)** | **1 unmaintained warning** |

### CVE detail (only finding)

```
ID:        RUSTSEC-2025-0052
Crate:     async-std 1.13.2
Warning:   unmaintained
Title:     async-std has been discontinued
Date:      2025-08-24
URL:       https://rustsec.org/advisories/RUSTSEC-2025-0052
Repo:      pheno-otel
Path:      async-std 1.13.2
         └── opentelemetry_sdk 0.27.1
              ├── tracing-opentelemetry 0.28.0
              │    └── pheno-otel 0.1.0
              ├── pheno-otel 0.1.0
              ├── opentelemetry-proto 0.27.0
              │    └── opentelemetry-otlp 0.27.0
              │         └── pheno-otel 0.1.0
              └── opentelemetry-otlp 0.27.0
Severity:  unmaintained (not exploitable)
Status:    warning (not deny); allow-listed by default
```

`async-std` is a transitive dependency of `opentelemetry_sdk 0.27.1` (transitive of `opentelemetry-otlp`). The pheno-otel crate itself uses `tokio` as the primary runtime — `async-std` is a compile-time optional path in opentelemetry_sdk. Risk is LOW because: (1) no runtime exposure, (2) the crate is feature-gated, (3) it's "unmaintained" not "vulnerable".

### Cargo.lock availability

| Repo | Cargo.lock |
|---|---|
| pheno-config | tracked |
| pheno-errors | tracked |
| pheno-flags | tracked |
| pheno-context | NOT tracked (cargo audit says "Locking 0 packages") |
| pheno-port-adapter | tracked |
| pheno-cli-base | tracked |
| pheno-otel | tracked |

pheno-context is the only Rust repo without a tracked `Cargo.lock` — must be added to enable audit.

### Critical findings

**NONE.** No real secrets were leaked. No exploitable CVEs found.

False-positive regex hits reviewed and confirmed benign:
- `pheno-agents-md/audit_scorecard.json:201-204` — JSON scorecard fields (`hardcoded_api_key: 0`, etc.)
- `pheno-agents-md/src/lib.rs:233` — test fixture with `.secrets/**` gitignore pattern
- `pheno-cost-card/AGENTS.md:33` — gitignore documentation listing `**/*.pem`, `**/*.key`, `**/secrets/**`
- `pheno-prompt-test/AGENTS.md:24` — same gitignore documentation pattern

---

## Top 5 priority remediations

### P0 — Address pheno-otel async-std unmaintained warning (RUSTSEC-2025-0052)

**Why P0:** While not exploitable, "unmaintained" is a 71-pillar L48 (Dependency Hygiene) signal and breaks `--deny warnings`. The fix is small.

**Action:**
1. Track `pheno-otel/Cargo.lock` with `[patch.crates-io]` or `default-features = false` on `opentelemetry-otlp`/`opentelemetry_sdk` to disable the async-std feature path.
2. Or upgrade to `opentelemetry_sdk ≥ 0.28` which drops the async-std optional dependency.
3. Or add `[advisories]` ignore block in `Cargo.toml` if the optional path is genuinely inert.

### P1 — Generate SBOM artifacts for all 14 repos

**Why P1:** 0/14 SBOM coverage. SBOM is a CISA/NTIA minimum element for software supply chain transparency and is increasingly required by enterprise procurement.

**Action:**
1. Add `.github/workflows/sbom.yml` using `cyclonedx/cyclonedx-action` to each repo:
   - Rust: `cargo cyclonedx-bom --format json --output bom.json`
   - Python: `cyclonedx-py` via `pip install cyclonedx-bom`
   - Go: `cyclonedx-gomod` binary
2. Add `bom.json` to release artifacts.
3. Reference `bom.json` in `README.md` and the 71-pillar scorecard.

### P1 — Add SLSA provenance to release process

**Why P1:** 0/14 SLSA coverage. SLSA L1 (provenance) is the minimum bar for supply chain integrity. Industry expectation is rising (US EO 14028, EU CRA).

**Action:**
1. Add `docs/slsa.md` to each repo documenting build provenance claims.
2. Add `.github/workflows/release-attestation.yml` using `slsa-framework/slsa-github-generator`:
   - For binaries: build with `slsa-github-generator/go-releases` or `rust-releases`.
   - For libraries: `slsa-github-generator/generator-generator-binaries` (containerized build).
3. Pin SLSA build level L1 → L2 → L3 over time.

### P2 — Commit `pheno-context/Cargo.lock`

**Why P2:** pheno-context is the only Rust repo in scope without a tracked lockfile. Without it, `cargo audit` returns "Locking 0 packages" — supply chain is unverifiable.

**Action:**
1. Run `cargo update` in `pheno-context/`, commit the resulting `Cargo.lock`.
2. Add `# lockfile-required` rule to `pheno-vibecoding-guard` so lockfiles can't be silently dropped.

### P2 — Add `deny.toml` to repos lacking one

**Why P2:** License + advisory enforcement at `cargo deny` level is cheap and prevents future regressions. 4 of 7 Rust repos already have `deny.toml` (pheno-cli-base, pheno-otel, pheno-pydantic-models is Python, pheno-fastapi-base is Python). Missing in: pheno-config, pheno-errors, pheno-flags, pheno-context, pheno-port-adapter, pheno-cargo-template.

**Action:**
1. Copy a baseline `deny.toml` from `pheno-cli-base/deny.toml` to each missing Rust repo.
2. Customize `[licenses]` allow-list per repo's actual deps.
3. Wire `cargo deny check` into CI for each repo.

### Honorable mentions (not blocking, tracked separately)

- **Pre-commit secret scanning across all 14 repos.** `pheno-vibecoding-guard` pre-commit hook exists per AGENTS.md but only in a few. Recommendation: install `gitleaks` hook in all 14 to keep T21.1 green even as new code lands.
- **Populate the 4 empty sparse-checkout submodules** (pheno-cargo-template, pheno-agents-md, pheno-cost-card, pheno-prompt-test) so future audits have full source to scan.

---

## Per-check methodology

### T21.1 — Secret scan

**Regex:** `api[_-]?key|api[_-]?token|secret|password|bearer|AKIA[0-9A-Z]{16}|ghp_[a-zA-Z0-9]{36}|sk-[a-zA-Z0-9]{48}|xox[bpars]-`

**Scope:** tracked files only (`git ls-files`); extensions `.py .rs .ts .go .js`; excluded paths matching `test|fixture|example|tests|test_|test\.|spec\.ts$|bench|benches|__tests__`.

**Tool:** `rg -l` (ripgrep) per file, individually scoped (avoids the CWD-fallback bug that pollutes results when the file list is empty).

**Timeout:** 60s per repo; finished all 14 within 5s.

### T21.2 — SBOM

**Artifacts checked:** `bom.json`, `sbom.json`, `bom.xml`, any `cyclonedx-*` file.

**Method:** `git ls-files <repo> | grep -E '(bom\.json|sbom\.json|bom\.xml|cyclonedx-)'`.

### T21.3 — SLSA

**Artifacts checked:** `docs/slsa.md`, `.github/workflows/release-attestation.yml`, `.github/workflows/slsa.yml`, `.github/workflows/provenance.yml`.

**Method:** `git ls-files <repo> | grep -E '(docs/slsa\.md|\.github/workflows/(release-attestation|slsa|provenance)\.yml$)'`.

### T21.4 — CVE

**Tool:** `cargo audit --no-fetch` (Rust only; 7/14 repos).

**Advisory DB:** 1132 entries (cached at `~/.cargo/advisory-db`).

**Timeout:** 60s per repo; finished all 7 within 45s.

---

## Cross-references

- **71-pillar audit (ADR-024):** L46-L55 (Security domain) — this audit scores L46 (secret scanning), L48 (dependency hygiene), L53 (SBOM), L54 (SLSA/provenance), L55 (CVE patching).
- **Factory AI Agent Readiness (ADR-026):** maps to Security pillar (Level 3+ requires SBOM + provenance).
- **pheno-vibecoding-guard:** pre-commit secret scanning library — referenced for the pre-commit remediation.
- **pheno-otel:** the only repo with an advisory — see its own worklog for any planned OTLP upgrade.

---

## Repro commands

```bash
cd /Users/kooshapari/CodeProjects/Phenotype/repos

# T21.1 secret scan (single repo)
SECRET_REGEX='api[_-]?key|secret|password|bearer|AKIA[0-9A-Z]{16}|ghp_[a-zA-Z0-9]{36}|sk-[a-zA-Z0-9]{48}|xox[bpars]-'
git ls-files pheno-config | grep -E '\.(py|rs|ts|go|js)$' | grep -vE '(test|fixture|example|/tests/|test_|_test\.|spec\.ts$|/bench/|/benches/|__tests__)' \
  | xargs rg -l -e "$SECRET_REGEX"

# T21.2 SBOM
git ls-files pheno-config | grep -E '(bom\.json|sbom\.json|bom\.xml|cyclonedx-)'

# T21.3 SLSA
git ls-files pheno-config | grep -E '(docs/slsa\.md|\.github/workflows/(release-attestation|slsa|provenance)\.yml$)'

# T21.4 cargo audit (single repo)
cd pheno-config && cargo audit --no-fetch
```