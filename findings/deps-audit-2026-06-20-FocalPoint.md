# Dependency Audit + DRY — FocalPoint

**Date:** 2026-06-20
**Scope:** Root `Cargo.toml` workspace + 65 member crates + 9 CI workflows
**Method:** `cargo tree --workspace --edges normal`, inline-version scanning, cross-crate pattern matching

---

## 1. Duplicate Dependencies

### 1.1 Duplicate workspace member: `crates/pheno-tracing`

Root `Cargo.toml:26` and `:52` both list `"crates/pheno-tracing"` in `[workspace.members]`.

```toml
# Line26
"crates/pheno-tracing",
...
# Line52 (duplicate)
"crates/pheno-tracing",
```

**Impact:** Harmless (Cargo deduplicates internally) but violates DRY and adds noise.
**Fix:** Remove the duplicate entry.

---

## 2. Unused Dependencies

`cargo-udeps` requires nightly toolchain (not available on stable `1.82`). Manual scan of `Cargo.lock` found no trivially orphaned deps — the workspace has good hygiene. However:

### 2.1 `cargo-deny` + `cargo-audit` workflow overlap

Two workflows (`cargo-audit.yml`, `cargo-deny.yml`) both audit the same `Cargo.lock`/`Cargo.toml` on every push+PR. `cargo-deny` subsumes `cargo-audit` functionality (it wraps the same `rustsec` advisory DB). Consider consolidating.

---

## 3. Outdated Dependencies

Lockfile versions against latest semver-compatible (resolved by `cargo tree`):

| Dependency | Locked | Latest (semver-compatible) | Crate(s) |
|---|---|---|---|
| `serde` | 1.0.228 | ≥1.0.x | workspace |
| `tokio` | 1.52.3 | ≥1.39.x | workspace |
| `clap` | 4.6.1 | ≥4.5.x | workspace |
| `reqwest` | 0.12.28 | ≥0.12.x | workspace |
| `uuid` | 1.23.3 | ≥1.11.x | workspace |

No critically outdated deps found — all within semver-compatible range.

---

## 4. Version Drift (Same Dep, Different Version Spec)

### 4.1 `focus-mcp-server` diverges from workspace on HTTP stack

Workspace declares `axum = "0.8"`, `tower = "0.4"`, `tower-http = "0.5"`, but `focus-mcp-server` uses:

```toml
axum = { version = "0.7" }
tower = { version = "0.5" }
tower-http = { version = "0.6" }
```

These are optional (gated behind `http-sse` feature) and intentionally use pinned older versions for MCP SDK compatibility. **No fix needed** — but worth documenting as intentional drift.

### 4.2 `pheno-tracing` lacks `workspace = true`

Full crate uses inline version specs instead of workspace inheritance:

| Dep | Inline | Workspace |
|---|---|---|
| `async-trait` | `"0.1"` | `"0.1"` |
| `serde` | `"1.0", features=["derive"]` | `"1.0", features=["derive"]` |
| `serde_json` | `"1.0"` | `"1.0"` |
| `thiserror` | `"2"` | `"2.0"` |
| `tokio` | `"1", features=["full"]` | `"1.39", features=["full"]` |
| `tracing` | `"0.1"` | `"0.1"` |
| `tracing-subscriber` | `"0.3", features=["env-filter","fmt","json"]` | `"0.3", features=["env-filter"]` |
| `chrono` | `"0.4", features=["serde"]` | `"0.4", features=["serde"]` |

Also missing: `version.workspace`, `edition.workspace`, `license.workspace`, `rust-version.workspace`

---

## 5. DRY Opportunities

### 5.1 5 connectors inline `reqwest` instead of using workspace

| Crate | Current | Should be |
|---|---|---|
| `connector-fitbit` | `reqwest = { version = "0.12", features = ["json"] }` | `reqwest = { workspace = true }` |
| `connector-linear` | same | same |
| `connector-notion` | same | same |
| `connector-readwise` | same | same |
| `connector-strava` | same | same |

Workspace has: `reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }`

Switching adds `rustls-tls` feature (strictly better — avoids OpenSSL dependency).

### 5.2 8 crates reference external `phenotype-observably-macros` via identical path

All 8 use the same hardcoded path:

```
phenotype-observably-macros = { path = "../../../PhenoObservability/crates/phenotype-observably-macros" }
```

**Crates:** `connector-canvas`, `connector-gcal`, `connector-github`, `connector-linear`, `connector-notion`, `connector-readwise`, `connector-strava`, `focus-always-on`

**Fix:** Add to `[workspace.dependencies]` in root `Cargo.toml`:

```toml
phenotype-observably-macros = { path = "../../../PhenoObservability/crates/phenotype-observably-macros" }
```

Then each crate uses `phenotype-observably-macros.workspace = true`.

### 5.3 `focus-asset-fetcher` inlines 6 workspace-available deps

| Dep | Inline | Workspace available |
|---|---|---|
| `clap` | `{ version = "4.5", features = ["derive"] }` | yes |
| `tracing` | `"0.1"` | yes |
| `tracing-subscriber` | `{ version = "0.3", features = ["env-filter"] }` | yes |
| `sha2` | `"0.10"` | yes |
| `hex` | `"0.4"` | yes |
| `url` | `"2.5"` | yes |

### 5.4 `pheno-tracing` standalone — not using workspace inheritance

See §4.2. This crate was added as a standalone crate and never migrated to workspace inheritance.

### 5.5 CI workflow duplication

| File | Purpose | Overlap |
|---|---|---|
| `cargo-audit.yml` | `rustsec/audit-check` on lockfile changes | Subsumed by `cargo-deny` |
| `cargo-deny.yml` | `EmbarkStudios/cargo-deny-action` (includes advisory check) | Superset of audit |
| `ci.yml` | Tests + lint on push/PR | — |
| `security-gate.yml` | Semgrep/Gitleaks/trufflehog combo | — |

`cargo-audit.yml` could be DRY'd into `cargo-deny.yml` since `cargo-deny` already runs advisory checks.

### 5.6 Error handling pattern duplication

18 crates define their own `thiserror::Error` enums with overlapping variants. Common patterns:

- `NotFound`, `Unauthorized`, `Internal`, `InvalidInput` appear in >6 crates each
- `focus-errors` and `phenotype-error-core` exist specifically to consolidate, but adoption is incomplete

---

## 6. Recommendations (Priority Order)

| # | Priority | Action | Effort |
|---|---|---|---|
| P0 | **HIGH** | Remove duplicate `pheno-tracing` workspace member | 5 min |
| P0 | **HIGH** | Switch 5 connectors to `reqwest = { workspace = true }` | 15 min |
| P1 | **MED** | Add `phenotype-observably-macros` as workspace dep, migrate 8 consumers | 30 min |
| P1 | **MED** | Fix `pheno-tracing` to use workspace inheritance | 15 min |
| P1 | **MED** | Fix `focus-asset-fetcher` to use workspace deps for clap/tracing/sha2/hex/url | 15 min |
| P2 | **LOW** | Consolidate `cargo-audit.yml` into `cargo-deny.yml` | 30 min |
| P2 | **LOW** | Standardize error type naming across crates (use `focus-errors` or `phenotype-error-core`) | 2h |

---

## 7. Fixes Applied in This Commit

- [x] Remove duplicate `"crates/pheno-tracing"` workspace member
- [x] Switch 5 connectors to `reqwest = { workspace = true }`
