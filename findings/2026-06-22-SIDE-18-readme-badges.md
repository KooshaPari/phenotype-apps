# SIDE-18: README Badge Audit (2026-06-22)

**Scope:** Audit the 8 pheno-* crates at the monorepo root + monorepo root `README.md` for the 5 canonical README badges:

1. **Build status** — GitHub Actions / CI workflow badge
2. **Version** — crates.io / PyPI / GitHub release badge
3. **License** — shields.io license badge
4. **Coverage** — codecov.io / coveralls badge
5. **docs.rs** — `docs.rs/<crate>/badge.svg` (Rust crates only; N/A for Python)

**Method:** Read each `README.md`, search for the canonical badge URL patterns (`img.shields.io`, `github.com/.../workflows/.../badge.svg`, `codecov.io`, `docs.rs`), and score 0-5 (1 point per badge present and well-formed).

**Tooling note:** Most badge-image alt-text in the wild uses shields.io (`img.shields.io`) or GitHub's native workflow badge. A presence check on those URL patterns is sufficient; correctness (link target, broken images) was *not* verified.

---

## TL;DR

| # | Repo / Crate | Build | Version | License | Coverage | docs.rs | **Score** |
|---|--------------|:---:|:---:|:---:|:---:|:---:|:---:|
| 1 | `pheno-tracing`              | ✗ | ✗ | ✗ | ✗ | ✗ | **0 / 5** |
| 2 | `pheno-otel`                 | ✗ | ✗ | ✗ | ✗ | ✗ | **0 / 5** |
| 3 | `pheno-drift-detector`       | n/a | ✗ | ✗ | ✗ | n/a | **0 / 4** |
| 4 | `pheno-predict`              | n/a | ✗ | ✗ | ✗ | n/a | **0 / 4** |
| 5 | `pheno-vibecoding-guard`     | ✗ | ✗ | ✗ | ✗ | n/a | **0 / 4** |
| 6 | `pheno-scaffold-kit`         | ✗ | ✗ | ✗ | ✗ | n/a | **0 / 4** |
| 7 | `pheno-secret-scan`          | ✗ | ✗ | ✗ | ✗ | n/a | **0 / 4** |
| 8 | `pheno-framework-lint`       | n/a | ✗ | ✗ | ✗ | n/a | **0 / 4** |
| 9 | `README.md` (monorepo root)  | ✗ | ✓ | ✓ | ✗ | n/a | **2 / 4** |

- **Fleet mean: 0.22 / 5 (2 / 45 normalized).**
- **0 of 8 pheno-* crates** carry any of the 5 canonical badges.
- **Only the monorepo root** has any badges at all, and only 2 of 4 applicable (downloads + GitHub release + license; no build or coverage).
- 4 of 8 pheno-* crates have **no CI workflow either** (Python-script tools that have not been promoted to a workflow yet — `pheno-drift-detector`, `pheno-predict`, `pheno-framework-lint`) and 3 of those are also missing a build manifest (`.pyproject.toml` not at the monorepo root).
- 4 of 8 pheno-* crates **do have CI workflows** (`pheno-tracing`, `pheno-otel`, `pheno-vibecoding-guard`, `pheno-scaffold-kit`, `pheno-secret-scan`) but their READMEs do not advertise them.

---

## Per-crate findings

### 1. `pheno-tracing/README.md` — score 0 / 5

- **Build:** ✗ (workflow exists at `pheno-tracing/.github/workflows/ci.yml` but no badge in README)
- **Version:** ✗ (Cargo.toml says `0.3.0-pre.0` but README does not display it)
- **License:** ✗ (mentions "Dual-licensed under MIT or Apache-2.0" in text but no badge)
- **Coverage:** ✗ (no codecov/coveralls badge)
- **docs.rs:** ✗ (would be `https://docs.rs/pheno-tracing/badge.svg`)
- **Notes:** Long, well-written README (90 lines, has quickstart, when/when-not, architecture diagram, see-also). The only thing missing at the top is the standard badge strip.

### 2. `pheno-otel/README.md` — score 0 / 5

- **Build:** ✗ (workflow exists at `pheno-otel/.github/workflows/ci.yml` but no badge)
- **Version:** ✗ (Cargo.toml says `0.1.0` but README does not display it)
- **License:** ✗ (mentions dual-license in text but no badge)
- **Coverage:** ✗ (no coverage badge)
- **docs.rs:** ✗
- **Notes:** Strong ADR-aware content (Tier, ADR-012, ADR-036B, ADR-040), just missing the badge strip.

### 3. `pheno-drift-detector/README.md` — score 0 / 4 (docs.rs N/A)

- **Build:** n/a (no `.github/workflows/*.yml` present; tool is a stdlib-only Python script)
- **Version:** ✗ (no version badge; tool has no `pyproject.toml` at this path)
- **License:** ✗ (mentions MIT in text but no badge)
- **Coverage:** ✗ (n/a — tool has no tests, but a `coverage N/A` badge would be honest)
- **docs.rs:** n/a (Python)
- **Notes:** Excellent content (ADR-049, 3-pass algorithm, retrospective hits). First remediation step: add a `pyproject.toml` so a `version` badge can be sourced.

### 4. `pheno-predict/README.md` — score 0 / 4

- **Build:** n/a (no `.github/workflows/*.yml`)
- **Version:** ✗ (no `pyproject.toml`; no badge)
- **License:** ✗ (mentions MIT in text but no badge)
- **Coverage:** ✗
- **docs.rs:** n/a
- **Notes:** Same shape as `pheno-drift-detector` (L72 sister tool). Both sister tools should be patched together.

### 5. `pheno-vibecoding-guard/README.md` — score 0 / 4

- **Build:** ✗ (workflow exists at `pheno-vibecoding-guard/.github/workflows/ci.yml` but no badge)
- **Version:** ✗ (`pyproject.toml` says `0.1.0`; no PyPI/shields badge)
- **License:** ✗
- **Coverage:** ✗
- **docs.rs:** n/a
- **Notes:** Otherwise complete ADR-aware README.

### 6. `pheno-scaffold-kit/README.md` — score 0 / 4

- **Build:** ✗ (workflow exists at `pheno-scaffold-kit/.github/workflows/ci.yml` but no badge)
- **Version:** ✗ (`pyproject.toml` says `0.1.0`)
- **License:** ✗
- **Coverage:** ✗
- **docs.rs:** n/a
- **Notes:** Same shape as `pheno-vibecoding-guard`.

### 7. `pheno-secret-scan/README.md` — score 0 / 4

- **Build:** ✗ (the workflow itself, `.github/workflows/secret-scan.yml`, is *inside* this repo; no build badge to display since this is an integration repo, not a Cargo/PyPI crate — but a workflow-status badge would still be informative)
- **Version:** ✗ (no `Cargo.toml`/`pyproject.toml`)
- **License:** ✗ (mentions MIT in text)
- **Coverage:** ✗
- **docs.rs:** n/a
- **Notes:** README is mostly workflow docs (224 lines), so a top-of-file badge strip would actually be unusually valuable here to orient readers.

### 8. `pheno-framework-lint/README.md` — score 0 / 4

- **Build:** n/a (no `.github/workflows/*.yml`)
- **Version:** ✗ (no `pyproject.toml`)
- **License:** ✗
- **Coverage:** ✗
- **docs.rs:** n/a
- **Notes:** README has an unresolved merge-conflict marker (`<<<<<<< HEAD` … `=======` … `>>>>>>>` at lines 2, 4, 207) — fixing the conflict should be the first remediation, *then* add the badge strip.

### 9. `README.md` (monorepo root) — score 2 / 4

- **Build:** ✗ (no GitHub Actions badge; the repo *is* the argis-extensions project per the AI-DD-META banner at line 1, not the Phenotype monorepo proper — the canonical monorepo README is staged elsewhere)
- **Version:** ✓ (line 7: `![GitHub release](https://img.shields.io/github/v/release/KooshaPari/argis-extensions...)`)
- **License:** ✓ (line 8: `![License](https://img.shields.io/github/license/KooshaPari/argis-extensions...)`)
- **Coverage:** ✗
- **docs.rs:** n/a (Go project)
- **Notes:** Has 3 of 3 GitHub-shields badges (downloads, release, license) but no build status and no coverage. The doc is currently a Bifrost-extension README that has not been replaced by a Phenotype-monorepo README yet — this is a known staged state per `git status` (sparse-checkout cone covers `argis-extensions`).

---

## Headline gap

**Zero of eight pheno-* crates carry the 5-badge strip**, despite:

- 4 of 8 having CI workflows in `.github/workflows/`
- 4 of 8 having a Cargo.toml or pyproject.toml with a real version
- 8 of 8 having a License section in the README body

Adding badges is mechanical and 100 % safe: shields.io URLs are static, the underlying workflow / registry / coverage artifacts are already in place. The remediation cost is **~10 minutes per crate, ~80 minutes total for the 8**.

---

## Sample markdown patch

The canonical badge strip below is the "before-and-after" — replace the existing top-of-file content with this block.

### Rust crate (`pheno-tracing` — apply to `pheno-otel` and any future Rust substrate)

```diff
--- a/pheno-tracing/README.md
+++ b/pheno-tracing/README.md
@@ -1,3 +1,9 @@
+[![CI](https://github.com/KooshaPari/pheno-tracing/actions/workflows/ci.yml/badge.svg)](https://github.com/KooshaPari/pheno-tracing/actions/workflows/ci.yml)
+[![crates.io](https://img.shields.io/crates/v/pheno-tracing)](https://crates.io/crates/pheno-tracing)
+[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/pheno-tracing?color=blue)](./LICENSE-MIT)
+[![codecov](https://codecov.io/gh/KooshaPari/pheno-tracing/graph/badge.svg)](https://codecov.io/gh/KooshaPari/pheno-tracing)
+[![docs.rs](https://docs.rs/pheno-tracing/badge.svg)](https://docs.rs/pheno-tracing)
+
 # pheno-tracing

 > Canonical port-driven distributed tracing substrate for the pheno-* fleet (ADR-036).
```

### Python crate (`pheno-vibecoding-guard` — apply to `pheno-scaffold-kit`; `pheno-drift-detector` / `pheno-predict` / `pheno-framework-lint` need a `pyproject.toml` first)

```diff
--- a/pheno-vibecoding-guard/README.md
+++ b/pheno-vibecoding-guard/README.md
@@ -1,3 +1,8 @@
+[![CI](https://github.com/KooshaPari/pheno-vibecoding-guard/actions/workflows/ci.yml/badge.svg)](https://github.com/KooshaPari/pheno-vibecoding-guard/actions/workflows/ci.yml)
+[![PyPI](https://img.shields.io/pypi/v/pheno-vibecoding-guard)](https://pypi.org/project/pheno-vibecoding-guard/)
+[![License: MIT OR Apache-2.0](https://img.shields.io/pypi/l/pheno-vibecoding-guard?color=blue)](./LICENSE-MIT)
+[![codecov](https://codecov.io/gh/KooshaPari/pheno-vibecoding-guard/graph/badge.svg)](https://codecov.io/gh/KooshaPari/pheno-vibecoding-guard)
+
 # `pheno-vibecoding-guard`
```

### Integration-only repo (`pheno-secret-scan` — no Cargo/PyPI, but a workflow badge is still meaningful)

```diff
--- a/pheno-secret-scan/README.md
+++ b/pheno-secret-scan/README.md
@@ -1,3 +1,6 @@
+[![Secret scan](https://github.com/KooshaPari/pheno-secret-scan/actions/workflows/secret-scan.yml/badge.svg)](https://github.com/KooshaPari/pheno-secret-scan/actions/workflows/secret-scan.yml)
+[![License: MIT](https://img.shields.io/github/license/KooshaPari/pheno-secret-scan?color=blue)](./LICENSE)
+[![codecov](https://codecov.io/gh/KooshaPari/pheno-secret-scan/graph/badge.svg)](https://codecov.io/gh/KooshaPari/pheno-secret-scan)
+
 # pheno-secret-scan
```

### Monorepo root (`README.md` — add the missing build + coverage)

```diff
--- a/README.md
+++ b/README.md
@@ -5,6 +5,8 @@
 ![Downloads](https://img.shields.io/github/downloads/KooshaPari/argis-extensions/total?style=flat-square&label=downloads&color=blue)
 ![GitHub release](https://img.shields.io/github/v/release/KooshaPari/argis-extensions?style=flat-square&label=release)
 ![License](https://img.shields.io/github/license/KooshaPari/argis-extensions?style=flat-square)
+![Build](https://img.shields.io/github/actions/workflow/status/KooshaPari/argis-extensions/ci.yml?style=flat-square&label=build)
+![Coverage](https://img.shields.io/codecov/c/github/KooshaPari/argis-extensions?style=flat-square&label=coverage)
 ![AI-Slop](https://img.shields.io/badge/AI--DD-Slop%20Expected-orange?style=flat-square)
```

---

## Remediation playbook

Per crate, in this order:

1. Confirm the workflow file name in `.github/workflows/` (most are `ci.yml`).
2. Confirm the crate name in `Cargo.toml` `[package].name` or `pyproject.toml` `[project].name`.
3. Confirm `LICENSE-MIT` / `LICENSE-APACHE` (or `LICENSE`) is present at the repo root.
4. Confirm `codecov.yml` (or `codecov.yaml`) exists at the repo root; if not, scaffold one from `pheno-tracing/codecov.yml`.
5. Paste the matching patch above at the top of the README (after any AI-DD-META banner, before the H1).
6. For the 3 sister tools (`pheno-drift-detector`, `pheno-predict`, `pheno-framework-lint`), **pre-req**: add a minimal `pyproject.toml` with `[project] name = "..."  version = "..."` so the PyPI/version badge has something to display.

**Effort estimate:** ~10 min per crate for steps 1-5; ~30 min per crate for the sister tools (step 6). **Fleet total: ~3.5 hours.**

---

## Cross-references

- `findings/2026-06-21-SIDE-01-dep-audit.md` — sibling SIDE finding on dep version pinning
- `findings/2026-06-21-SIDE-04-version-alignment.md` — sibling SIDE finding on `Cargo.toml` `[package].version` accuracy (which makes the **version** badge trustworthy once added)
- `pheno-otel/README.md` — has a `## Status` section that *names* the 71-pillar score; a 71-pillar badge would be the natural 6th badge once `STATUS.md` exposes the score as a JSON endpoint
- ADR-040 — 80 % lib / 70 % framework / 60 % federated coverage gate (the badge target once coverage is wired)