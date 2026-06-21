# pheno-port-adapter — Architecture Overview (L1)

**Tier:** `pheno-*-lib` (per ADR-023 substrate placement)
**Pillar:** 71-pillar L1 (Architecture Overview), Cycle 7 closure
**Source:** `pheno-port-adapter/src/lib.rs`, `src/ports/`, `src/adapters/`, `tests/`
**Date:** 2026-06-21

## L1 Context

`pheno-port-adapter` is the **canonical hexagonal port-adapter substrate**
for the Phenotype fleet (ADR-038). It provides two surfaces in one crate:

1. **Sync transport adapters** — [`PortAdapter`] trait + concrete TCP and
   Unix-domain implementations, intended for connection-lifecycle plumbing.
2. **Async hex-port traits + adapters** — `HexCachePort`, `HexTimePort`,
   and their concrete adapters (`InMemoryCache`, `RedisAdapter`,
   `SystemClock`, `MockClock`).

Application code depends on the **trait**, never the adapter. Adapter
choice is a wiring concern (DI, factory, builder) — typically configured
once at process start.

```
                  ┌──────────────────────────────┐
                  │   Application / Binary       │
                  │   depends on PortAdapter /   │
                  │   HexCachePort / HexTimePort │
                  └───────────────┬──────────────┘
                                  │
                                  ▼
              ┌─────────────────────────────────────┐
              │   pheno-port-adapter (this crate)   │
              │                                     │
              │   pub trait PortAdapter (sync)      │  ◀── tcp, unix adapters
              │   pub mod ports::HexCachePort       │  ◀── in_memory, redis
              │   pub mod ports::HexTimePort        │  ◀── system_clock, mock
              │                                     │
              │   pheno-otel (dep, OTLP wire)       │
              └─────────────────────────────────────┘
```

## C4 Container view

```
┌──────────────────────────────────────────────────────────────────────────────┐
│                   Application / Binary (consumer crate)                      │
└────────────────────────────────────────┬─────────────────────────────────────┘
                                         │
                                         ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│                          pheno-port-adapter (this crate)                     │
│                                                                              │
│  ┌────────────────────────────────────────────────────────────────────────┐  │
│  │ src/lib.rs                                                             │  │
│  │   #![deny(missing_docs)] #[deny(unsafe_code)] #[deny(rust_2018_idioms)]│  │
│  │   pub trait PortAdapter { fn name(), health(), connect(), disconnect() }│  │
│  │   pub enum AdapterError { ConnectFailed, DisconnectFailed, ... }       │  │
│  │   pub struct Connection { id: String }                                 │  │
│  │   pub mod ports;  pub mod adapters;                                    │  │
│  └────────────────────────────────────────────────────────────────────────┘  │
│                          │                          │                         │
│                          ▼                          ▼                         │
│  ┌─────────────────────────────────┐  ┌──────────────────────────────────┐    │
│  │ src/ports/  (the "P" in hex)    │  │ src/adapters/  (the "A" in hex)  │    │
│  │   cache.rs   HexCachePort        │  │   in_memory_cache.rs             │    │
│  │   time.rs    HexTimePort         │  │   redis_cache.rs (tokio-comp)    │    │
│  │   mod.rs     re-exports          │  │   system_clock.rs                │    │
│  │                                 │  │   mock_clock.rs                  │    │
│  │                                 │  │   tcp.rs, unix.rs (sync)         │    │
│  └─────────────────────────────────┘  └──────────────────────────────────┘    │
│                          │                          │                         │
│                          ▼                          ▼                         │
│  ┌────────────────────────────────────────────────────────────────────────┐  │
│  │ External runtime / drivers                                             │  │
│  │   tokio 1.x (rt-multi-thread, macros, sync, time)                      │  │
│  │   async-trait 0.1 (object-safe `dyn HexCachePort`)                     │  │
│  │   redis 0.27 (tokio-comp)                                              │  │
│  │   pheno-otel (ADR-037 OTLP wire — connection-lifecycle spans)          │  │
│  └────────────────────────────────────────────────────────────────────────┘  │
│                                                                              │
│  Tests:                                                                      │
│   tests/hex_cache.rs   (contract test against HexCachePort)                  │
│   tests/hex_time.rs    (contract test against HexTimePort)                   │
│   tests/loom.rs        (Loom concurrent-exploration tests)                   │
└──────────────────────────────────────────────────────────────────────────────┘
```

**Container responsibilities:**

| Container | Responsibility |
|-----------|----------------|
| `src/lib.rs` | Define `PortAdapter` trait, `AdapterError`, `Connection` handle; expose flat re-exports (`HexCachePort`, `HexTimePort`); enforce `#![deny(missing_docs)]` + `#![deny(unsafe_code)]` + `#![deny(rust_2018_idioms)]` (`lib.rs:31-33`). |
| `src/ports/` | Async hex-port traits. Each port has its own error type per ADR-038 (e.g. `CacheError`, `TimeError`) — no shared global `AdapterError`. |
| `src/adapters/` | Concrete impls. Sync transport adapters implement `PortAdapter`; async cache/clock adapters implement `HexCachePort` / `HexTimePort`. |
| `tests/` | Contract tests against the trait, not the concrete type. Loom tests for concurrency correctness. |

## Key decisions

1. **Two surfaces, one crate.** Sync `PortAdapter` (TCP/Unix lifecycle)
   lives alongside async `HexCachePort` / `HexTimePort`. The split is
   explicit at `lib.rs:6-19`. They serve different runtimes (sync vs
   `async-trait`), and the crate forbids port-trait implementations from
   leaking into the transport layer.
2. **`#![deny(missing_docs)]` + `#![deny(unsafe_code)]` at the crate root.**
   Enforced at `lib.rs:31-33`. New ports cannot land without doc-comments
   on every public item — codifies ADR-023 Rule 3.1 + ADR-040 quality bar.
3. **Each port owns its error type.** Per ADR-038, the hex-port traits
   (`cache.rs:50-62`, `time.rs`) define per-port error enums rather than
   sharing a global `AdapterError`. Adapters map their backend-specific
   errors into the port's enum variants.
4. **Flat re-exports, not nested paths.** `HexCachePort` is reachable as
   `pheno_port_adapter::HexCachePort`, not
   `pheno_port_adapter::ports::HexCachePort`. Documented at `lib.rs:99-102`
   so adding a new port doesn't break import paths.
5. **OTLP observability via `pheno-otel` (ADR-037).** Connection
   lifecycle (connect / disconnect / error) emits spans via the canonical
   OTLP wire substrate. Per-operation spans on cache hits are explicitly
   deferred to v13+ so high-QPS cache paths aren't blanketed.
6. **Loom concurrency tests, not just unit tests.** `tests/loom.rs`
   explores concurrent adapter interactions under Loom's permutation
   explorer — catches races that hand-rolled multi-thread tests miss.

## Future-state

- **v18 (Cycle 8) — more hex-port traits.** Add `HexSecretPort` (vault
  backend) and `HexKvPort` (key-value, separate from cache). Both will
  land under `src/ports/` with their own error enums per ADR-038. The
  `HexKvPort` is a strict superset of `HexCachePort` semantics (added
  scan/iterate) — tracked in ADR-047 Predictive-DRY for whether they
  should merge.
- **v19 — blanket OTel spans.** Promote per-operation cache spans from
  v13-deferred to shipped. Adapter emits a span per `get`/`put`/
  `invalidate` call; callers can downsample via the OTLP samplers. Gate:
  L57 perf budget must still pass (ADR-040 lib 80 % coverage + L57
  metrics/red).
- **v20+ — substrate graduation to `phenotype-port-adapter-sdk`.** Per
  ADR-048 graduation path, once 2+ polyglot consumers (Rust + Python
  UniFFI, or Rust + Go bridge) ship, the trait surface forks to a
  polyglot SDK while `pheno-port-adapter` stays the Rust core.

## Cross-references

- **ADR-014** (Hexagonal L4 ports — Port trait + Adapter impl, superseded)
- **ADR-023** (Agent-effort governance — substrate placement)
- **ADR-037** (pheno-tracing substrate canonical — OTLP wire)
- **ADR-038** (Hexagonal port-adapter L4 policy — formal, supersedes ADR-014)
- **ADR-040** (Test coverage gates per tier — 80 % lib)
- **ADR-047** (Predictive DRY discipline — 4-criterion rule)
- **ADR-048** (Substrate graduation path — 4-tier gate table)
- **Source:** `pheno-port-adapter/src/lib.rs:70-89` (`PortAdapter` trait)
- **Tests:** `pheno-port-adapter/tests/hex_cache.rs`, `tests/hex_time.rs`, `tests/loom.rs`
- **Cargo deps:** `pheno-port-adapter/Cargo.toml` (tokio, async-trait, redis, pheno-otel)
- **Existing doc:** `pheno-port-adapter/docs/architecture.md` (mermaid diagrams)
