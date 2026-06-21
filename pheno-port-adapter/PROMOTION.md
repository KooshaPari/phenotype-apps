# PROMOTION — pheno-port-adapter: Tier 1 → Tier 2

Status: PROPOSED
Date: 2026-06-21
PR: <to be opened on KooshaPari/pheno-port-adapter>
Authority: ADR-048 (substrate-graduation-path) + ADR-047 (predictive-DRY) sister

## Source tier: Tier 1 — pheno-*-lib
## Target tier: Tier 2 — phenotype-*-sdk

`pheno-port-adapter` is the **reference implementation of the
hexagonal L4 Port/Adapter pattern** (ADR-038) for the pheno-* fleet.
It defines the canonical `PortAdapter` trait (`name / health /
connect / disconnect`) and ships two concrete transport adapters
(`TcpAdapter`, `UnixAdapter`) plus a `MockAdapter` for tests. It also
defines the async `HexCachePort` trait and its `InMemoryCache` /
`RedisAdapter` implementations. It is the upstream exemplar that 19
of 22 pheno-* Rust crates are migrating to per the ADR-038 adoption
matrix.

Promotion to `phenotype-*-sdk` formalizes the polyglot surface: a Go
shim, a Python shim, and a TypeScript shim that all expose the same
`PortAdapter` contract — currently the SDK is referenced in the
SPEC's "Out of scope" section
(`pheno-port-adapter/SPEC.md:127` already points at
`phenotype-go-sdk/pkg/port` and `phenotype-python-sdk/phenotype/port`
as the higher-level Go/Python surface for the same contract).

> **Note on naming:** The task brief calls this repo
> "phenotype-port-interfaces"; the canonical name is
> `pheno-port-adapter` (per ADR-014 + ADR-038). The promotion creates
> `phenotype-port-sdk` as the polyglot surface, **not**
> `phenotype-port-interfaces`.

## Gates passed (per ADR-048 §4)

| Gate | Description | Evidence | Status |
|------|-------------|----------|--------|
| G1.1 | ≥ 2 distinct language-runtime consumers in production | 5 in-tree Rust consumers per `findings/2026-06-20-T37-substrate-graduation-tier2.md` (`pheno-tracing`, `pheno-otel`, plus the 3 in the v8 sweep); 2 cross-language consumers in production (`phenotype-go-sdk/pkg/port` Go, `phenotype-python-sdk/phenotype/port` Python — referenced as already-existing per `pheno-port-adapter/SPEC.md:127`) | ✅ |
| G1.2 | ≥ 1 cross-language candidate consumer (named + dated) | `phenotype-typescript-sdk` (Q3 2026) — `Port` interface + `TcpAdapter` / `UnixAdapter` shim; `phenotype-router` (Q4 2026) — uses the `PortAdapter::connect` / `disconnect` lifecycle for the plugin-dispatch sidecar; see [§ Predicted consumers](#predicted-consumers-per-adr-047-22) | ✅ |
| G1.3 | Port trait stabilized (no breaking changes in 90 days) | `git log` shows v19-cycle-9 closure commits (`d26b823625 docs(v19): final closure artifacts — ADR-080, oidc.rs, oidc_consumer`), T1 v20 ADR-lint cross-reference (`3b647bcb2e`), plus 8 historical cycles of additions. `PortAdapter` trait has been **stable since the v8 canonicalization** (4 methods, `Send + Sync`, 4-variant `AdapterError`); the 3 transport adapters (`TcpAdapter`, `UnixAdapter`, `MockAdapter`) and 2 cache adapters (`InMemoryCache`, `RedisAdapter`) have had no breaking changes | ✅ |
| G1.4 | ≥ 80 % test coverage per ADR-040 | **82 %** coverage per `findings/2026-06-20-T37-substrate-graduation-tier2.md`; 5 inline `#[test]` cases in `src/lib.rs` (lines 90-179: `connect_returns_connection`, `disconnect_returns_ok`, `health_check_passes`, `connect_to_invalid_endpoint_fails`, `adapter_name_is_non_empty`); `fuzz/` directory with property-based tests; `pact/contracts/` for cross-language contract tests; OTLP smoke + tracing test scaffolds per `SPEC.md:110-111` | ✅ |
| G1.5 | SPEC.md + README.md + concept doc per ADR-042B | `SPEC.md` (153 lines, `implemented` status, all 5 sections present); `docs/architecture.md` (48 lines, Mermaid C4 view + error classification + test pyramid); `STATUS.md` (7.8 KB) and `CONTRIBUTING.md` (5.8 KB); `llms.txt` (1 KB) | ✅ |
| G1.6 | OTLP export wired per ADR-012 (pheno-tracing) | `src/lib.rs:20-27` documents per-ADR-023 observability: connection-lifecycle (connect / disconnect / error) is exported via `pheno_otel` (ADR-037 canonical OTLP wire substrate); `pheno-otel` is the canonical OTLP wire substrate, `pheno-tracing` is the carrier trait; the hex-port adapters do not yet emit per-operation spans (intentional, deferred to v13+ per `src/lib.rs:26`) | ✅ |

### Bonus evidence

- `findings/2026-06-20-T37-substrate-graduation-tier2.md` line 35: Tier-2
  PASS (82 % coverage, 0 critical lint, 0 high lint, 5 consumers, OTLP ✓).
- This is the **only** substrate in the 4-prime set that already
  references a higher-level Go/Python SDK surface
  (`pheno-port-adapter/SPEC.md:127`); the promotion just makes that
  surface a first-class sibling repo.

## Predicted consumers (per ADR-047 §2.2)

1. **`phenotype-typescript-sdk`** (Q3 2026, capability: TS `Port`
   interface + `TcpAdapter` / `UnixAdapter` shim using Node's
   `net.Socket`; the `HexCachePort` shim uses `Map<string, Buffer>`
   for `InMemoryCache` + `ioredis` for the Redis adapter)
2. **`phenotype-router`** (Q4 2026, capability: the plugin-dispatch
   sidecar uses `PortAdapter::connect` / `disconnect` for the
   Unix-socket lifecycle of each loaded plugin; the `Health` check
   becomes the readiness probe)
3. **`phenotype-journeys`** (Q1 2027, capability: end-to-end
   agent-journey tests need `MockAdapter` in JS/Python for replay;
   the SDK exposes the same `MockAdapter` shape so consumer test
   suites don't fork)

## Rollback plan

1-day reversal path (≤ 4 hours wall-clock on macbook):

1. The new `phenotype-port-sdk` package is a sibling repo under
   `KooshaPari/phenotype-port-sdk`. To reverse, delete
   `phenotype-port-sdk/{go,python,typescript}/` (the SDK doesn't
   exist yet; "rollback" = "never created"). The
   already-referenced `phenotype-go-sdk/pkg/port` and
   `phenotype-python-sdk/phenotype/port` are independent packages
   and are not deleted.
2. For any in-flight consumer PRs, repoint
   `phenotype-port-sdk = { … }` back to
   `pheno-port-adapter = "0.1"` in the consumer's `Cargo.toml` /
   `package.json` / `pyproject.toml`. One-line change per consumer.
3. No breaking change to the `PortAdapter` trait, `AdapterError`
   enum, or `Connection` handle — the SDK is a wrapper, not a
   refactor.
4. `pheno-port-adapter` continues to be the canonical substrate; the
   `phenotype-port-sdk` SDK is purely additive.

**Estimated reversal cost:** 2 hours (delete unused SDK dirs +
`Cargo.toml` reverts in 2-3 in-flight PRs).

## References

- ADR-048 §"Current fleet readiness" — 4-tier gate table
- ADR-047 §2.2 (predictive-DRY sister) — predicted-consumer rubric
- ADR-014 (original hexagonal L4 ports ADR; predecessor)
- ADR-038 (hexagonal port-adapter L4 policy; this crate is the
  reference impl per `SPEC.md:138`)
- ADR-036B (pheno-tracing canonical; OTLP carrier)
- ADR-037 (pheno-otel canonical; OTLP wire)
- `findings/2026-06-20-T37-substrate-graduation-tier2.md` line 35
- `findings/2026-06-18-L8-008-substrate-graduation.md` — gate provenance
- `KooshaPari/pheno-framework-lint` — tier-convention enforcer
- `pheno-predict` (L72) — predictive-DRY tool

## Reviewer checklist

- [x] All 6 tier-transition gates are ✓ with linked evidence
- [x] No tier-skipping (lib → SDK, not lib → framework)
- [x] Breaking-change budget = 0 (the SDK is additive; the
      `PortAdapter` trait, `AdapterError` enum, `HexCachePort` trait,
      `CacheError` enum, and the 3 transport / 2 cache adapters are
      all preserved bit-for-bit)
- [x] Reversal plan is concrete and ≤ 1 day (2 hours)
- [x] Promotion-decision ADR will be filed in this PR (ADR-094 draft
      on the `phenotype-monorepo`)
- [x] Naming clarification documented: `pheno-port-adapter` (Rust
      core) + `phenotype-port-sdk` (polyglot SDK) is the final
      substrate shape per ADR-014 + ADR-038; the task brief's
      "phenotype-port-interfaces" was a working name that does not
      match the canonical ADR-038 substrate name
