# pheno-config — Architecture Overview (L1)

**Tier:** `pheno-*-lib` (per ADR-023 substrate placement)
**Pillar:** 71-pillar L1 (Architecture Overview), Cycle 7 closure
**Source:** `pheno-config/src/cascade.rs`, `pheno-config/tests/cascade_test.rs`
**Date:** 2026-06-21

## L1 Context

`pheno-config` is the **canonical layered-configuration substrate** for the
Phenotype fleet. It wraps [`figment`](https://docs.rs/figment) into a single
four-provider cascade with a strict 12-factor priority order. Every other
`pheno-*-lib` and `phenotype-*-sdk` that needs typed configuration depends on
this crate rather than re-implementing the cascade.

The cascade is opinionated and non-negotiable:

```
Jetbrains::default()    ──▶  highest priority (developer-machine override)
        ▲
        │  Figment::merge (last merge wins)
        │
Env::prefixed("PHENO_") ──▶  12-factor env override
Toml::file("config.toml")──▶  checked-in TOML (soft-miss if absent)
Toml::string(DEFAULT_TOML)─▶  embedded compile-time defaults (always present)
```

## C4 Container view

```
┌─────────────────────────────────────────────────────────────────────────┐
│                     Application / Binary                                │
│                  (any pheno-*-lib consumer)                             │
└───────────────────────────────────┬─────────────────────────────────────┘
                                    │ use pheno_config::cascade::build_cascade
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                       pheno-config (this crate)                         │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │ cascade.rs                                                        │  │
│  │   pub fn build_cascade() -> Figment                               │  │
│  │   pub fn build_cascade_from_str(toml: &str) -> Figment            │  │
│  │   pub const DEFAULT_TOML: &str                                    │  │
│  └───────────────────────────────────────────────────────────────────┘  │
│                                    │                                    │
│                                    ▼                                    │
│  ┌───────────────────────────────────────────────────────────────────┐  │
│  │                       figment 0.10 (dep)                          │  │
│  │   providers: Env, Jetbrains, Toml, Format, Data                   │  │
│  └───────────────────────────────────────────────────────────────────┘  │
└───────────────────────────────────┬─────────────────────────────────────┘
                                    │ Figment::find_value("server.port")
                                    ▼
                ┌──────────────────────────────────────┐
                │  External providers (read at runtime)│
                ├──────────────────────────────────────┤
                │  $PHENO_* environment variables      │
                │  ./config.toml (optional)            │
                │  .idea/runConfigurations/*.xml       │
                └──────────────────────────────────────┘
```

**Container responsibilities:**

| Container | Responsibility |
|-----------|----------------|
| `cascade.rs` | Compose the 4-provider Figment in the documented priority order; expose `DEFAULT_TOML` as a `pub const`. |
| `tests/cascade_test.rs` | Integration-test the priority order end-to-end against a real `Figment` (no mocking of providers). |
| `figment` (transitive) | Provider trait, `merge` semantics, `find_value` path resolution. |

## Key decisions

1. **Single file, single concern.** `pheno-config` is intentionally a
   one-source-file crate (`src/cascade.rs`, 93 LoC, 1 module). All priority
   logic is in `build_cascade()` and `build_cascade_from_str()`. Anything
   beyond a 4-provider cascade is out of scope and belongs in
   `phenotype-configd` (federated config server).
2. **Jetbrains layer is non-optional.** `Jetbrains::default()` is always
   the topmost provider so a developer can shadow a checked-in value from
   IntelliJ run-configurations without touching env vars or TOML. This is
   codified in `cascade.rs:55-65`.
3. **TOML file is soft-miss.** `Toml::file("config.toml")` does NOT abort
   on missing file; the cascade continues. Documented at `cascade.rs:60`.
   This lets `pheno-config` work in minimal-binary contexts (CLI tools,
   fuzzers, examples) where no TOML file is checked in.
4. **Test pyramid uses integration tests, not unit tests.** Per
   `tests/cascade_test.rs:1-9` (top-level comment), the cascade is verified
   end-to-end via `cargo test --test cascade_test`. The four cases are:
   default-only, toml-overrides-default, env-overrides-toml, env-only key.
   Mocking providers is explicitly avoided — the cascade *is* the contract.
5. **Tier `pheno-*-lib`, not `phenotype-*-sdk`.** Per ADR-023 Rule 3.1
   substrate placement + ADR-048 substrate graduation path, `pheno-config`
   is a primitive library — no business logic, no I/O beyond reading the
   cascade, no domain knowledge. The 80%-coverage lib gate (ADR-040) is the
   applicable quality bar.

## Future-state

- **v18 (Cycle 8) — hot-reload.** Promote `pheno-config` to a 2-tier shape:
  in-process `Cascade` (today) + out-of-process `phenotype-configd` (federated
  service per ADR-023 substrate placement) for hot-reload. The cascade
  shape stays identical; `phenotype-configd` becomes a 5th provider whose
  values trump Jetbrains. Tracked in ADR-022.
- **v19 — config schema validation.** Add `pheno-config-schema` (typed
  `Figment::validate<T: DeserializeOwned>`) without changing the cascade
  contract. Consumers opt in by calling `find_value::<MyConfig>()` instead
  of `find_value::<u16>("server.port")`.
- **v20+ — Configra absorb.** Per ADR-031, all `phenotype-config*` repos
  fold into `KooshaPari/Configra` as the canonical name; `pheno-config`
  becomes the Rust core and `Configra` becomes the TypeScript edge.
  Track T19 in the v8 DAG.

## Cross-references

- **ADR-022** (Config consolidation — two-crate split)
- **ADR-023** (Agent-effort governance — device + substrate policy)
- **ADR-031** (Configra absorb — `phenotype-config` → `Configra`)
- **ADR-038** (Hexagonal port-adapter L4 policy)
- **ADR-040** (Test coverage gates per tier — 80 % lib)
- **ADR-048** (Substrate graduation path — 4-tier gate table)
- **L1 cycle-7 closure finding:** `findings/2026-06-21-v17-L1-architecture-overview.md`
- **Test:** `pheno-config/tests/cascade_test.rs:42-109` (4 cases)
- **Source:** `pheno-config/src/cascade.rs:55-77` (cascade builders)
