# FocalPoint Code Consolidation Extension Plan

Extends the consolidation work from `2026-06-13-focalpoint-consolidation.md` (Phases 1-10) with Phases 11-15.

---

## Phase 11: Storage & Backend Abstraction (P2)
**Objective**: Unify storage backends to eliminate duplicate DB/file/memory patterns.

**Current State**: Multiple crates likely have their own storage implementations (`focus-storage`, `focus-cache`, `focus-persistence`, `focus-index` etc.) with overlapping concerns.

**Work Items**:
- [ ] Create `Storage` trait with `get`, `put`, `delete`, `list`, `exists` methods
- [ ] Create `StorageBackend` enum: `File`, `Memory`, `Sqlite`, `Redis`
- [ ] Create `StorageConfig` struct for backend selection
- [ ] Create `StorageTransaction` trait for atomic operations
- [ ] Create `ContentAddressableStorage` using canonical hashing (Phase 5)
- [ ] Migrate all storage implementations to use shared traits
- [ ] Create `focus-storage` crate as the unified storage layer
- [ ] Add migration utilities for data format transitions

**Files to Modify**:
- `crates/focus-storage/src/lib.rs` (new or refactor)
- `crates/focus-cache/src/lib.rs` (if exists)
- `crates/focus-persistence/src/lib.rs` (if exists)
- `crates/focus-index/src/lib.rs` (if exists)
- All crates with file/db access

**Dependencies**: Phase 1 (errors), Phase 4 (serde), Phase 5 (crypto)

---

## Phase 12: Configuration & Settings Unification (P2)
**Objective**: Merge configuration systems into a single typed config layer.

**Current State**: `phenotype-config` may exist, but crates may also have their own `Config` structs. Settings may be scattered across files, env vars, and CLI args.

**Work Items**:
- [ ] Audit all `Config` / `Settings` / `Options` types across crates
- [ ] Create unified `Config` trait with `load()`, `save()`, `validate()` methods
- [ ] Create `ConfigSource` enum: `File`, `Env`, `Cli`, `Default`
- [ ] Create `ConfigBuilder` using Phase 3 builder pattern
- [ ] Create `ConfigSchema` for validation with `Validator` trait
- [ ] Merge `phenotype-config` into `focus-config` crate
- [ ] Add env var prefixing (`FOCUS_*`, `PHENOTYPE_*`)
- [ ] Create `ConfigWatcher` for hot-reload support
- [ ] Migrate all crates to use unified config system

**Files to Modify**:
- `crates/focus-config/src/lib.rs` (new or refactor)
- `crates/phenotype-config/src/lib.rs` (if exists, merge)
- All crates with config loading
- `Cargo.toml` (workspace dependencies)

**Dependencies**: Phase 1 (errors), Phase 3 (builders), Phase 4 (serde)

---

## Phase 13: Telemetry & Observability Consolidation (P3)
**Objective**: Unify metrics, logging, tracing, and health checks.

**Current State**: `focus-telemetry`, `focus-audit`, `focus-metrics`, `pheno-tracing` may overlap or use different instrumentation libraries.

**Work Items**:
- [ ] Audit all telemetry crates for overlap
- [ ] Create `Telemetry` trait with `record_metric`, `log_event`, `trace_span`
- [ ] Create `Metric` trait with `counter`, `gauge`, `histogram`, `timer` types
- [ ] Create `HealthCheck` trait for service health reporting
- [ ] Create `AuditLogger` for security/audit events (using Phase 9 event system)
- [ ] Create `TracingExporter` for OpenTelemetry/OTLP integration
- [ ] Merge `pheno-tracing` into `focus-telemetry` or create unified crate
- [ ] Standardize all `#[instrument]` usage across crates
- [ ] Create `RequestId` propagation across all async boundaries
- [ ] Add structured logging with `tracing-subscriber` layers

**Files to Modify**:
- `crates/focus-telemetry/src/lib.rs` (new or refactor)
- `crates/focus-metrics/src/lib.rs` (if exists, merge)
- `crates/focus-audit/src/lib.rs` (if exists, merge)
- `crates/pheno-tracing/src/lib.rs` (if exists, merge)
- All crates with `log::`, `tracing::`, `metrics::` usage

**Dependencies**: Phase 1 (errors), Phase 9 (event system), Phase 12 (config)

---

## Phase 14: Security & Authentication Consolidation (P3)
**Objective**: Unify auth patterns, token management, and permission systems.

**Current State**: Each connector may have its own OAuth2/token handling. No unified permission/authorization layer.

**Work Items**:
- [ ] Create `Auth` trait for authentication (OAuth2, API key, Bearer token)
- [ ] Create `TokenManager` for token refresh, storage, and revocation
- [ ] Create `Permission` trait for RBAC/ABAC authorization
- [ ] Create `SecureStore` trait for credential storage (using Phase 11 storage)
- [ ] Create `Session` trait for session management
- [ ] Create `RateLimiter` for API key usage (using Phase 7 connectors)
- [ ] Create `focus-auth` crate with all auth types
- [ ] Migrate all connector auth to use `TokenManager`
- [ ] Add mTLS support for internal service communication
- [ ] Create `Secret` wrapper type for zeroized credential storage

**Files to Modify**:
- `crates/focus-auth/src/lib.rs` (new)
- `crates/focus-auth/src/token.rs` (new)
- `crates/focus-auth/src/permission.rs` (new)
- `crates/focus-auth/src/session.rs` (new)
- All connector crates (Phase 7)
- `crates/focus-ffi/src/lib.rs` (if FFI auth)
- `crates/focus-mcp-server/src/lib.rs` (if MCP auth)

**Dependencies**: Phase 1 (errors), Phase 7 (connectors), Phase 11 (storage), Phase 12 (config)

---

## Phase 15: Workspace & Repository Hygiene (P3)
**Objective**: Final cleanup — remove unused crates, standardize structure, and lock the consolidation.

**Current State**: Workspace may have stale crates, unused dependencies, or inconsistent file layouts.

**Work Items**:
- [ ] Audit all crates for zero usage (unused modules, dead code)
- [ ] Remove or merge crates with <100 lines of unique code
- [ ] Standardize crate naming: `focus-*` for runtime, `phenotype-*` for shared libraries
- [ ] Standardize module structure: `src/lib.rs`, `src/error.rs`, `src/tests/` (using Phase 6)
- [ ] Update workspace `Cargo.toml` to reflect final crate list
- [ ] Run `cargo-deny` for license and security audit
- [ ] Run `cargo-bloat` to verify binary size reduction
- [ ] Update `ARCHITECTURE.md` with final crate map
- [ ] Update `README.md` with consolidated crate descriptions
- [ ] Create `crates/README.md` with crate dependency graph
- [ ] Lock all dependency versions in `Cargo.lock`
- [ ] Final `cargo test --workspace` and `cargo clippy --workspace -- -D warnings`
- [ ] Tag release: `v0.2.0-consolidated`

**Files to Modify**:
- `Cargo.toml` (workspace)
- `Cargo.lock` (locked)
- `README.md`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- All crate `Cargo.toml` files (standardize metadata)
- Any unused crate directories (remove)

**Dependencies**: Phase 1-14 (all prior work)

---

## Implementation Order (Extended DAG)

```
Phase 1 (Error Unification)
    |
    +---> Phase 2 (Transpiler Traits)
    |         |
    |         +---> Phase 8 (IR/Domain Model)
    |
    +---> Phase 3 (Builder Pattern)
    |
    +---> Phase 4 (Serde Utilities)
    |         |
    |         +---> Phase 5 (Crypto/Hashing)
    |         |         |
    |         |         +---> Phase 11 (Storage)
    |         |
    |         +---> Phase 8 (IR/Domain Model)
    |         |
    |         +---> Phase 9 (Event System)
    |         |         |
    |         |         +---> Phase 13 (Telemetry)
    |         |
    |         +---> Phase 10 (CLI/FFI/Plugin)
    |         |
    |         +---> Phase 12 (Config)
    |                   |
    |                   +---> Phase 13 (Telemetry)
    |                   |
    |                   +---> Phase 14 (Security)
    |
    +---> Phase 6 (Test Infrastructure)
    |
    +---> Phase 7 (Connector Pattern)
              |
              +---> Phase 9 (Event System)
              |
              +---> Phase 14 (Security)
              |
              +---> Phase 11 (Storage)

Phase 11 + Phase 12 + Phase 13 + Phase 14
    |
    +---> Phase 15 (Workspace Hygiene)
```

---

## Verification Checklist

- [ ] All crates compile with `cargo check --workspace`
- [ ] All tests pass with `cargo test --workspace`
- [ ] No clippy warnings with `cargo clippy --workspace -- -D warnings`
- [ ] No duplicate code detected by `cargo-deduplicate` or similar
- [ ] Binary size reduction verified with `cargo bloat`
- [ ] Documentation updated for all public APIs
- [ ] `CHANGELOG.md` updated with consolidation summary
- [ ] `ARCHITECTURE.md` updated with final crate map
- [ ] All commits pushed to remote
- [ ] Release tagged: `v0.2.0-consolidated`

## Metrics

**Estimated Code Reduction (Phases 11-15)**:
- Phase 11: ~600 lines (storage backends)
- Phase 12: ~400 lines (config types)
- Phase 13: ~500 lines (telemetry instrumentation)
- Phase 14: ~400 lines (auth/token handling)
- Phase 15: ~300 lines (dead code removal)

**Total Estimated Reduction (Phases 11-15)**: ~2200 lines
**Combined Total (Phases 1-15)**: ~6000 lines
**Estimated Binary Size Reduction**: 25-35%
**Estimated Build Time Reduction**: 20-30%

## Risk Assessment

| Phase | Risk | Mitigation |
|-------|------|------------|
| Phase 11 | Data migration breakage | Create migration tests with rollback |
| Phase 12 | Config loss during merge | Back up all config files before merge |
| Phase 13 | Telemetry loss during merge | Add dual-write period during transition |
| Phase 14 | Auth disruption | Staged rollout with feature flags |
| Phase 15 | Accidental crate deletion | Git history review before removal |

## Rollback Plan

If any phase causes breakage:
1. Revert to last known-good commit
2. Apply fixes in isolation
3. Re-run verification checklist
4. Tag release only after full verification
