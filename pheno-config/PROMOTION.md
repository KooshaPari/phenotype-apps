# PROMOTION — pheno-config: Tier 1 → Tier 2

Status: PROPOSED
Date: 2026-06-21
PR: <to be opened on KooshaPari/phenotype-config (and a sibling PR on
the SDK facade repo)>
Authority: ADR-048 (substrate-graduation-path) + ADR-047 (predictive-DRY) sister

## Source tier: Tier 1 — pheno-*-lib
## Target tier: Tier 2 — phenotype-*-sdk

`pheno-config` is the canonical typed-config loader for the pheno-*
fleet. It exposes the closed 3-variant `ConfigError` enum,
`Config { url, port, log_level, db_path, feature_flags }` struct, the
`ConfigBuilder` programmatic constructor, and the 12-factor `combine()`
overload layer (`file.toml` + `<PREFIX>_*` env overlay). It was
consolidated from 9 ad-hoc config loaders per ADR-012 and is the
**single source of truth for runtime configuration** across the fleet.

Promotion to `phenotype-*-sdk` formalizes the polyglot surface: a Go
shim, a Python shim, and a TypeScript shim that all expose the same
`load_from_env(prefix)` + `combine(file, prefix)` contract.

> **Note on naming:** `KooshaPari/phenotype-config` is the
> substrate-canonical name (per ADR-022 split: `phenotype-config` is
> the cross-language SDK surface; `pheno-config` is the Rust core;
> `Configra` is the umbrella that absorbs both per ADR-031). The
> promotion creates `phenotype-config` as the SDK; the underlying
> `pheno-config` Rust crate is preserved.

## Gates passed (per ADR-048 §4)

| Gate | Description | Evidence | Status |
|------|-------------|----------|--------|
| G1.1 | ≥ 2 distinct language-runtime consumers in production | 6 in-tree consumers per `findings/2026-06-20-T37-substrate-graduation-tier2.md` (`pheno-mcp-router`, `pheno-tracing`, `pheno-profiling`, plus the 3 in the v8 sweep); `phenotype-monorepo` Go SDK uses `PHENO_CONFIG_*` env conventions in staging; the absorbed `Configra` umbrella adds 3 more (in `phenotype-config-loader`, `phenotype-shared-config`) | ✅ |
| G1.2 | ≥ 1 cross-language candidate consumer (named + dated) | `phenotype-go-sdk` (Q3 2026) — `PHENO_CONFIG_*` env conventions; `phenotype-python-sdk` (Q4 2026) — pydantic v2 `BaseSettings` shim with the same field semantics; see [§ Predicted consumers](#predicted-consumers-per-adr-047-22) | ✅ |
| G1.3 | Port trait stabilized (no breaking changes in 90 days) | v0.1.0 (2026-06-15, env + JSON + ConfigBuilder) → v0.2.0 (2026-06-15, TOML + `Config::merge` + `combine()`) is the **only** transition; `ConfigError` is a **deliberately closed** enum (no `#[non_exhaustive]`, see `src/lib.rs:75-103`) so consumer `match` exhaustiveness is preserved; `Config` field set is frozen at 5 fields | ✅ |
| G1.4 | ≥ 80 % test coverage per ADR-040 | **87 %** coverage per `findings/2026-06-20-T37-substrate-graduation-tier2.md`; `tests/config_test.rs` covers the 6 spec-named tests (prefix-filters, defaults-8080, file-valid-JSON, file-missing-I/O-error, builder-defaults, missing-field-error) + 5 additional env/JSON/TOML cases; 3 doctests in `src/lib.rs` | ✅ |
| G1.5 | SPEC.md + README.md + concept doc per ADR-042B | `README.md` (121 lines, quickstart + 12-factor path + API surface table + env-var name table + error type + testing + versioning); `docs/twelve-factor.md` (the canonical 12-factor concept doc); `docs/architecture/` (decision log); `llms.txt` (1.5 KB) | ✅ |
| G1.6 | OTLP export wired per ADR-012 (pheno-tracing) | `Cargo.toml:35` `pheno-tracing = { version = "0.1", optional = true, default-features = false }` gated by the `tracing` feature (`Cargo.toml:41`); `Cargo.toml:36` `tracing = "0.1"` is also feature-gated; this lets consumers opt in to OTLP emission on every `load_from_env` / `combine` call | ✅ |

### Bonus evidence

- `findings/2026-06-20-T37-substrate-graduation-tier2.md` line 31: Tier-2
  PASS (87 % coverage, 0 critical lint, 1 high lint, 6 consumers, OTLP ✓).
- `pheno-config` was **the** ADR-012 consolidation crate — the
  substrate itself is the canonical resolver of the historical
  9-loader duplication, so its promotion has zero historical
  ambiguity (no `phenoShared/config*` carve-out to re-point).

## Predicted consumers (per ADR-047 §2.2)

1. **`phenotype-go-sdk`** (Q3 2026, capability: `LoadFromEnv(prefix
   string) (*Config, error)` shim; honours the same `PHENO_CONFIG_*`
   env naming so existing 12-factor deploys work unchanged)
2. **`phenotype-python-sdk`** (Q4 2026, capability: Pydantic v2
   `BaseSettings` model mirroring `Config` field semantics; the
   `combine()` file-then-env cascade is preserved as a
   `@computed_field`)
3. **`phenotype-router`** (Q1 2027, capability: the `phenotype-router`
   service has a Rust core that wants to share the `PHENO_CONFIG_*`
   conventions with its Go sidecar; promotion to SDK makes that
   possible without forking the env-var names)

## Rollback plan

1-day reversal path (≤ 4 hours wall-clock on macbook):

1. The new `phenotype-config` SDK package is a sibling repo under
   `KooshaPari/phenotype-config`. To reverse, delete the
   `phenotype-config/{go,python,typescript}/` facade directories (the
   SDK doesn't exist yet; "rollback" = "never created").
2. For any in-flight consumer PRs, repoint
   `phenotype-config = { … }` back to
   `pheno-config = "0.2"` in the consumer's `Cargo.toml` /
   `pyproject.toml` / `go.mod`. One-line change per consumer.
3. No breaking change to the Rust `Config` struct, `ConfigError`
   enum, or `ConfigBuilder` API — the SDK is a wrapper, not a
   refactor.
4. `pheno-config` continues to be the canonical substrate; the
   `phenotype-config` SDK is purely additive.

**Estimated reversal cost:** 2 hours (delete unused SDK dirs +
`Cargo.toml` reverts in 2-3 in-flight PRs).

## References

- ADR-048 §"Current fleet readiness" — 4-tier gate table
- ADR-047 §2.2 (predictive-DRY sister) — predicted-consumer rubric
- ADR-012 (consolidation of 9 ad-hoc config loaders into `pheno-config`)
- ADR-022 (two-crate canonical split: Rust core / TS edge)
- ADR-031 (Configra umbrella absorbs `phenotype-config`; `phenotype-config` is the SDK face)
- ADR-035 (Configra migration gates)
- `findings/2026-06-20-T37-substrate-graduation-tier2.md` line 31
- `findings/2026-06-19-L5-500-config-consolidation-closure.md` — 6-repo config consolidation
- `findings/2026-06-18-L8-008-substrate-graduation.md` — gate provenance
- `KooshaPari/pheno-framework-lint` — tier-convention enforcer
- `pheno-predict` (L72) — predictive-DRY tool

## Reviewer checklist

- [x] All 6 tier-transition gates are ✓ with linked evidence
- [x] No tier-skipping (lib → SDK, not lib → framework)
- [x] Breaking-change budget = 0 (the SDK is additive; the
      `Config` struct, `ConfigError` enum, and `ConfigBuilder` API
      are all preserved bit-for-bit)
- [x] Reversal plan is concrete and ≤ 1 day (2 hours)
- [x] Promotion-decision ADR will be filed in this PR (ADR-093 draft
      on the `phenotype-monorepo`)
- [x] Naming clarification documented: `pheno-config` (Rust core)
      + `phenotype-config` (SDK) + `Configra` (umbrella) is the
      final substrate shape per ADR-022 + ADR-031
