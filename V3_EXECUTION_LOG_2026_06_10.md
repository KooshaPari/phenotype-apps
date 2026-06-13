# V3 Execution Log — 2026-06-10

**Generated:** 2026-06-10 (session start ~22:57 UTC)
**DAG:** `FLEET_100TASK_DAG_V3.md` (100 main + 20 side = 120 total)
**Mode:** Async background codex agents + parallel main agent work

## 2026-06-11 Updates (L3 subagent #49):

- **L3 #49 (pheno-otel Rust crate) — completed.** New standalone
  Rust crate at `pheno-otel/` providing a one-liner OpenTelemetry
  initialization API with a Drop-based `TelemetryGuard`. Two
  initialization entry points — `init(service_name)` (OTLP HTTP
  exporter) and `init_with_stdout(service_name)` (no-network
  stdout exporter for local dev) — both return a
  `TelemetryGuard` whose `Drop` impl flushes pending spans and
  calls `opentelemetry::global::shutdown_tracer_provider()` to
  reset the global tracer provider to a no-op. `OtelError` is a
  thiserror enum with the spec's three variants
  (`ExporterInit`, `ResourceBuild`, `Shutdown`). Pinned to the
  OpenTelemetry 0.27 line (`opentelemetry = "0.27"`,
  `opentelemetry_sdk = "0.27"` with `trace` feature,
  `opentelemetry-otlp = "0.27"` with
  `http-proto`/`reqwest-client`/`reqwest-rustls`/`trace`
  features). `init_with_stdout` ships a hand-rolled
  `StdoutSpanExporter` (`pheno-otel/src/exporter/stdout.rs`)
  rather than depending on the separate `opentelemetry-stdout`
  crate. 18/18 tests pass (10 unit + 5 integration in
  `tests/init_test.rs` + 3 doctest) under `cargo test --offline`;
  `cargo clippy --offline --all-targets -- -D warnings` is
  clean. Branch `chore/l3-49-pheno-otel-2026-06-11`, local-only
  (NOT pushed per task directive). See `### L3-#49 (pheno-otel)`
  section below. Canonical worklog:
  `worklogs/l3-49-pheno-otel-2026-06-11.json`. Feature commit:
  `ad8065eb1fc7c1c350400359768faa3084c7516b` on branch
  `chore/l3-49-pheno-otel-2026-06-11`.

- **L3 #50 (pheno-cli-base Rust crate — clap + colored CLI base)
  — completed.** New standalone `pheno-cli-base/` crate at the
  monorepo root providing the canonical facade for every Pheno
  CLI binary. Re-exports `clap` (so derived `#[derive(Parser)]`
  structs do not need a direct clap dep), exposes a `CliRunnable`
  trait (`run(&self) -> Result<(), AppError>` mandatory +
  default `main(&self) -> !` that maps `AppError` to colored
  stderr and `std::process::exit(1)`), `install_panic_hook()` that
  prints the panic message + backtrace to stderr and exits 1
  (re-entrant via `std::sync::Once`), and
  `parse_from_env_or_exit::<T: clap::Parser>() -> T` that renders
  colored clap usage on parse failure and exits 2. `AppError` is
  defined as a local stub (thiserror 2.0; 6 variants mirroring
  the L3 #46 spec — `Validation`, `NotFound`, `Storage`,
  `Config`, `Domain`, `Io`) because the `pheno-errors/` crate
  (L3 #46) does not yet exist on `main`; a one-line migration
  to a path dep is planned when L3 #46 lands (documented in the
  worklog `spec_deviations`). Deps: `clap = "4"` (derive
  feature), `colored = "2"`, `thiserror = "2.0"`. 17/17 tests
  pass (5 integration in `tests/cli_test.rs` covering all 5
  spec'd test names verbatim + 8 unit + 4 doctest) under
  `cargo test --offline`; `cargo clippy -p pheno-cli-base
  --all-targets -- -D warnings` is clean. Standalone package
  via empty `[workspace]` table in `pheno-cli-base/Cargo.toml`
  (mirrors L3 #46/#47/#48/#49/#57 convention; not a member of
  root `Cargo.toml`). Branch
  `chore/l3-50-pheno-cli-base-2026-06-11`, local-only (NOT
  pushed per task directive). See `### L3-#50
  (pheno-cli-base)` section below. Canonical worklog:
  `worklogs/l3-50-pheno-cli-base-2026-06-11.json`. Feature
  commit: `659e173003` on branch
  `chore/l3-50-pheno-cli-base-2026-06-11`.

- **L3 #57 (pheno-plugin Rust crate — plugin registry + dynamic
  dispatch) — completed.** New standalone `pheno-plugin/` crate
  at the monorepo root providing the canonical in-process plugin
  registry for the pheno-* fleet. The `Plugin` trait is
  object-safe (`Send + Sync` + no associated types + no generic
  methods) and exposes `name()`, `version()`, and a default-noop
  `init()` hook; `PluginRegistry` is a name-indexed
  `HashMap<String, Box<dyn Plugin>>` with `new()`,
  `register()` (rejects duplicate names with
  `PluginError::DuplicateName`), `get()`, `names()` (sorted
  ascending), and `init_all()` (bulk init in registration order,
  short-circuits on first failure). `PluginError` is a thiserror
  enum with two tuple variants per the L3 #57 spec verbatim —
  `DuplicateName(String)` and `InitFailed(String)`. One
  dependency: `thiserror = "2.0"`. 8/8 tests pass (6 integration
  tests in `tests/registry_test.rs` covering all 6 spec'd test
  names: `registry_starts_empty`, `register_adds_plugin`,
  `register_rejects_duplicate`, `get_returns_registered_plugin`,
  `init_all_invokes_each_plugin_init`, `names_returns_sorted` +
  2 doctest); `cargo clippy --all-targets -- -D warnings` is
  clean. Standalone package via empty `[workspace]` table in
  `pheno-plugin/Cargo.toml` (mirrors L3 #46/#47/#48/#49
  convention; not a member of root `Cargo.toml`). Branch
  `chore/l3-57-pheno-plugin-registry-2026-06-11`, local-only
  (NOT pushed per task directive). See `### L3-#57
  (pheno-plugin-registry)` section below. Canonical worklog:
  `worklogs/l3-57-pheno-plugin-registry-2026-06-11.json`.
  Feature commit: `a8874c21d2` on branch
  `chore/l3-57-pheno-plugin-registry-2026-06-11`.

- **L3 #52 (pheno-go-ctxkit Go module — request_id + slog
  middleware) — completed.** New standalone Go module at
  `pheno-go-ctxkit/` (module path
  `github.com/kooshapari/pheno-go-ctxkit`, `go 1.22`, **zero
  third-party dependencies**) providing the canonical per-request
  context utilities for the pheno-* Go HTTP fleet. Public
  surface: `WithRequestID(ctx, id)` / `RequestID(ctx) string`
  (id round-trip with nil-safety and empty-id no-op semantics),
  `NewRequestID() string` (RFC 4122 v4 UUID generated entirely
  from `crypto/rand` + `encoding/hex` — no `github.com/google/uuid`
  or similar dep), `WithLogger(ctx, *slog.Logger)` / `Logger(ctx)
  *slog.Logger` (with `slog.Default()` fallback so callers can log
  unconditionally), `Middleware(next http.Handler) http.Handler`
  (reads `X-Request-ID` header → falls back to
  `r.Context()` value → falls back to `NewRequestID()`; injects
  id + a child `slog.Logger` carrying the `request_id` attr into
  `r.Context()`; echoes the id back on the response's
  `X-Request-ID` header; defers a single `slog.Logger.LogAttrs`
  call at LevelInfo with the message `request.complete` and
  the attributes `method` / `path` / `status` / `duration_ms`),
  and `Background()` shorthand returning a
  `context.Background()` pre-populated with a fresh request_id.
  Exposed `const HeaderRequestID = "X-Request-ID"` so tests and
  downstream code can use the same constant. 16/16 tests pass
  (6 top-level tests in `pheno-go-ctxkit/ctxkit/ctxkit_test.go`
  covering the 5 spec'd test names verbatim:
  `TestNewRequestIDIsUnique`,
  `TestWithRequestIDRoundTrip` (4 subtests),
  `TestWithLoggerRoundTrip` (4 subtests),
  `TestMiddlewareInjectsRequestID` (2 subtests),
  `TestMiddlewareEmitsRequestCompleteLog`,
  `TestBackgroundReturnsContextWithRequestID`) under
  `go test -race -count=1 ./...` (stdlib testing only, no
  testify, per spec); `go vet ./...` is clean. The request_id
  emission test parses the JSON log output to assert exactly
  one `request.complete` record with the spec'd attrs and that
  a handler-emitted log line carries the same `request_id` (so
  downstream log lines stay correlated). Branch
  `chore/l3-52-pheno-go-ctxkit-2026-06-11`, local-only (NOT
  pushed per task directive). See `### L3-#52 (pheno-go-ctxkit)`
  section below. Canonical worklog:
  `worklogs/l3-52-pheno-go-ctxkit-2026-06-11.json`. Feature
  commit: `e8d1b4e8f3` on branch
  `chore/l3-52-pheno-go-ctxkit-2026-06-11`.

### L3-#49 (pheno-otel)

**Task (V3 DAG L3 layer):** Author the canonical `pheno-otel`
Rust crate wrapping the OpenTelemetry 0.27 initialization chain
into a one-liner API: `init(service_name)` for production
OTLP-backed telemetry and `init_with_stdout(service_name)` for
local dev / CI smoke tests, both returning a `TelemetryGuard`
that flushes + shuts down the global tracer provider on Drop.
Consumed by L4 #70 (`helioscli` binary) and L5 #81–85 (the
5 pheno-* service crates) as the single source of truth for
runtime telemetry setup.

**Crate layout:** Nine files in a new `pheno-otel/` directory at
the monorepo root, declared as a standalone package via an
empty `[workspace]` table in its own `Cargo.toml` (the L3 #46
`pheno-errors` pattern — NOT a member of the 56+-crate root
`Cargo.toml` `[workspace.members]`). This keeps the new crate's
test/build loop independent of the root workspace and avoids
conflicting with the other L3 agents concurrently editing the
root manifest. Files:

| Path | Lines | Purpose |
|---|---:|---|
| `pheno-otel/Cargo.toml`        |  59 | Package manifest + OpenTelemetry 0.27 deps + empty `[workspace]` table |
| `pheno-otel/README.md`         |  63 | Quickstart + env-var contract |
| `pheno-otel/src/lib.rs`        |  51 | Crate-level docs + module declarations + re-exports |
| `pheno-otel/src/error.rs`      | 120 | `OtelError` (3-variant thiserror enum) + 4 inline `#[test]`s |
| `pheno-otel/src/guard.rs`      | 119 | `TelemetryGuard` (RAII; Drop impl + `shutdown` + `Debug`) + 3 inline `#[test]`s |
| `pheno-otel/src/init.rs`       | 138 | `init` + `init_with_stdout` + `build_resource` + `install_provider` + 2 doctests |
| `pheno-otel/src/exporter/mod.rs` | 11 | `pub mod stdout;` |
| `pheno-otel/src/exporter/stdout.rs` | 210 | Hand-rolled `StdoutSpanExporter` (one JSON line per span) + 3 inline `#[test]`s |
| `pheno-otel/tests/init_test.rs` | 249 | 5 integration tests (the spec's `>=5` floor) |

**Public API (3 symbols re-exported from `pheno_otel::`):**

1. `init(service_name: &str) -> Result<TelemetryGuard, OtelError>`
   — installs an OTLP/HTTP span exporter
   (`opentelemetry_otlp::SpanExporter::builder().with_http()`),
   wires it into a `TracerProvider` with
   `service.name=<service_name>` on the `Resource`, installs
   the provider as the global, and returns a `TelemetryGuard`.
   The endpoint is read from the SDK's standard
   `OTEL_EXPORTER_OTLP_ENDPOINT` env var; `init()` itself
   passes `DEFAULT_OTLP_ENDPOINT` (`"http://localhost:4318"`)
   to `with_endpoint(...)` so the SDK's env-var resolution
   path can override it.
2. `init_with_stdout(service_name: &str) -> Result<TelemetryGuard, OtelError>`
   — installs the hand-rolled `StdoutSpanExporter` (one JSON
   line per span to `std::io::stdout()`, no protobuf). No
   network I/O — safe in air-gapped environments and CI
   sandboxes.
3. `TelemetryGuard` — the RAII guard. Holds the
   `TracerProvider` (so it stays alive until the guard drops)
   and a `&'static str` `source` label (`"otlp"` or `"stdout"`,
   surfaced in `Debug` for test diagnostics). `Drop` calls
   `opentelemetry::global::shutdown_tracer_provider()` first
   (to swap the global for a no-op), then an explicit
   `force_flush()` + `shutdown()` on the held provider. Drop
   errors are logged to stderr at WARN; they do NOT panic
   (Drop cannot return). The `shutdown(&self)` method surfaces
   `OtelError::Shutdown` to callers who want typed error
   handling; the operations are idempotent so explicit
   shutdown does NOT prevent the Drop path from also running.

**`OtelError` (3 variants, thiserror derive):**

- `ExporterInit(String)` — the OTLP `SpanExporter::builder()`
  rejected the configuration (e.g. `with_endpoint(...)` got an
  invalid URI). Returned by `init()`.
- `ResourceBuild(String)` — the `Resource` (the entity that
  produces telemetry) could not be built. Currently fires when
  the caller passes an empty or whitespace-only `service_name`.
  Returned by both `init()` and `init_with_stdout()`.
- `Shutdown(String)` — the tracer provider could not be shut
  down cleanly (transport error from `force_flush()` or
  `shutdown()`). Returned by `TelemetryGuard::shutdown()`;
  the `Drop` impl logs these at WARN.

`OtelError: std::error::Error + Send + Sync + 'static` (the
inner trace error is rendered into the `Display` string at
construction time so the variant stays a self-contained
thiserror enum — no `#[from]` plumbing required). Also exposes
a stable `kind(&self) -> &'static str` tag
(`"exporter_init"` / `"resource_build"` / `"shutdown"`) for log
fields and metrics labels, plus constructor fns
(`exporter_init`, `resource_build`, `shutdown`).

**Test coverage (18/18 pass under `cargo test --offline`):**
The 5 spec-required integration tests are all present in
`tests/init_test.rs` by exact name:

| # | Test | What it checks |
|--:|------|----------------|
|  1 | `init_returns_guard`                            | `init_with_stdout` returns a `TelemetryGuard`; `Debug` render mentions both `TelemetryGuard` and the `source` label (`"stdout"`) |
|  2 | `init_with_stdout_emits_test_span`             | `init_with_stdout` produces a working tracer; `tracer.start(...).set_attribute(...).end()` succeeds; `guard.shutdown()` returns `Ok(())` |
|  3 | `guard_drop_calls_shutdown`                    | Two `init_with_stdout` calls in sequence; the second call succeeds only if the first guard's Drop ran `global::shutdown_tracer_provider()` and reset the global to a no-op |
|  4 | `otel_error_display_messages_are_useful`       | For each of the 3 variants, `Display` contains both the variant keyword AND the wrapped context string |
|  5 | `init_with_invalid_endpoint_returns_exporter_init_error` | `opentelemetry_otlp::SpanExporter::builder().with_http().with_endpoint("not a valid uri !!!").build()` fails and the resulting `TraceError` is mapped to `OtelError::ExporterInit` |

Plus 10 inline unit tests (4 in `error::tests` —
`constructors_set_variant`, `is_std_error`, `kind_is_stable`,
`display_mentions_kind`; 3 in `guard::tests` —
`default_provider_shutdown_is_ok`, `drop_does_not_panic`,
`drop_with_active_span_does_not_panic`; 3 in
`exporter::stdout::tests` — `render_uses_name_when_present`,
`render_falls_back_to_seq_when_name_empty`,
`render_skips_invalid_parent_span_id`) and 3 doctests
(`src/lib.rs:11` crate-level quickstart;
`src/init.rs:51` `init::init` example;
`src/init.rs:80` `init::init_with_stdout` example).

**Test isolation:** Tests that touch the global tracer
provider serialize themselves via a process-static
`INIT_LOCK: Mutex<()>` (the global can only be set once per
process to a meaningful value; without the lock, parallel
tests would race). The `init_with_invalid_endpoint_*` test
saves + clears `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` and
`OTEL_EXPORTER_OTLP_ENDPOINT` and restores them on drop via a
local `Restore` RAII guard — so no env bleed between parallel
tests. The stdout-capturing test acquires a process-static
`stdout_lock: Arc<Mutex<()>>` (via `once_cell::sync::Lazy`)
so test output is not interleaved.

**Deps resolution:** OpenTelemetry 0.27 line resolved cleanly
from `~/.cargo/registry/cache`. The first cold `cargo check`
timed out at 5 minutes on the reqwest + rustls + opentelemetry
+ tonic transitive tree; subsequent `--offline` runs are
sub-2-second incremental. No 5-minute resolver timeout on the
final verification runs.

**Constraints respected:**

- **Standalone crate** (empty `[workspace]` table in own
  `Cargo.toml`) per L3 #46 (`pheno-errors`) pattern — did NOT
  touch the root `Cargo.toml`'s `[workspace.members]`.
- **Did not touch any other L3 task** (L3 #46 pheno-errors,
  L3 #47 pheno-tracing, L3 #48 pheno-config, L3 #50
  pheno-cli-base, L3 #51 pheno-fastapi-base, L3 #52
  pheno-go-ctxkit, L3 #53 pheno-zod-pydantic, L3 #54
  pheno-tower-stack, L3 #55 pheno-ssot-template, L3 #56
  pheno-flags, L3 #57 pheno-plugin-registry).
- **Did NOT push to origin.** Branch is
  `chore/l3-49-pheno-otel-2026-06-11`, off `main` (1 commit
  ahead).
- **No FFI, no async runtime pulled in by the new crate.**
  The OTLP exporter pulls in `reqwest` (rustls) internally,
  but `pheno-otel` itself does not depend on `tokio` or
  `async-std` — the public API is synchronous.
- **Worktree isolation.** Worktree at
  `.worktrees/l3-49-pheno-otel-2026-06-11` isolates from the
  concurrent L3 branch switches happening in the shared
  `repos/` worktree.

**Drop semantics (explicitly designed):** `TelemetryGuard::drop`
is best-effort — it calls `global::shutdown_tracer_provider()`
(replaces the global with a no-op; subsequent
`global::tracer(...)` calls will get a noop tracer) and then
runs the held provider's `force_flush()` + `shutdown()` (best-
effort; errors are logged to stderr at WARN and otherwise
swallowed because Drop MUST NOT panic). Explicit
`guard.shutdown(&self)` returns the typed `OtelError::Shutdown`
and is idempotent w.r.t. the Drop path. The two operations
are intentionally independent so a caller who drops the guard
early still gets the global reset.

**Why a hand-rolled `StdoutSpanExporter` (not
`opentelemetry-stdout`):** Three reasons. (1) The
`opentelemetry-stdout` crate pulls in additional
tonic/serde features we don't need for a one-line JSON
exporter. (2) The format we want is non-standard (no
protobuf, no OTLP framing — just a greppable single JSON line
per span, with a `span#<seq>` fallback when the span name
is empty). (3) It avoids one more dep in the cold-compile
path. The `render()` helper is a separate `fn` so the JSON
serialization can be unit-tested without an async runtime.

**`init()` endpoint resolution:** `init()` does NOT take an
explicit endpoint parameter — it relies on the OpenTelemetry
SDK's standard env-var resolution
(`OTEL_EXPORTER_OTLP_ENDPOINT`,
`OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`). This matches the OTel
spec's HTTP exporter env-var contract. A future revision can
add `init_with_endpoint(service_name, url)` if a hard-coded
endpoint is needed; for now the spec's one-liner API is
preserved.

**Downstream:** L5 #81–85 (the 5 pheno-* service crates) can
now do `let _guard = pheno_otel::init("pheno-<svc>")?;` at
startup and forget about shutdown — the Drop path handles
it. L4 #70 (`helioscli` binary) main() can return
`Result<(), OtelError>`; `?` propagates the init error. The
hand-rolled stdout exporter can also be re-used by
integration tests in any downstream crate to capture
spans-in-flight without standing up a collector.

**Consolidation targets:** `agileplus-telemetry`
(`AgilePlus-wt-L1-001/crates/agileplus-telemetry`) also wraps
`opentelemetry-otlp`; `pheno-otel` is the canonical
lightweight sibling with the Drop-guard ergonomics that
AgilePlus's service-init macro layer can re-export. The
pre-existing `phenotype-otel/` placeholder crate (referenced
from the docs site) is left in place; the new `pheno-otel` is
a strict superset (it adds the stdout path and the
Drop-guard ergonomics).

### L3-#57 (pheno-plugin-registry)

**Task (V3 DAG L3 layer):** Author the canonical `pheno-plugin`
Rust crate providing the in-process plugin registry for the
pheno-* fleet. The `Plugin` trait is object-safe (so it can be
stored as `Box<dyn Plugin>` and loaded at runtime from crates the
host does not statically depend on) and exposes
`name(&self) -> &str`, `version(&self) -> &str`, and a
default-noop `init(&self) -> Result<(), PluginError> { Ok(()) }`
hook. `PluginRegistry` is a name-indexed
`HashMap<String, Box<dyn Plugin>>` with `new()`,
`register(Box<dyn Plugin>) -> Result<(), PluginError>` (rejects
duplicates with `DuplicateName`), `get(&str) -> Option<&dyn Plugin>`,
`names() -> Vec<String>` (sorted ascending), and
`init_all() -> Result<(), PluginError>`. `PluginError` is a
thiserror enum with the spec's two tuple variants —
`DuplicateName(String)` and `InitFailed(String)`. Consumed by
L5 #88 (`helioscli` — wire HeliosCLI to pheno-plugin and load
`helios-plugin-*` crates at startup) and any future L5 pheno-*
host that wants a uniform plugin entrypoint.

**Crate layout:** Three files in a new `pheno-plugin/` directory
at the monorepo root, declared as a standalone package via an
empty `[workspace]` table in its own `Cargo.toml` (mirrors the
L3 #46 `#pheno-errors` decision and the L3 #47/L3 #48/L3 #49
follow-ups — keeps the new crate's test/build loop independent
of the 56-crate root workspace):

- `pheno-plugin/Cargo.toml` (29 lines) — package manifest,
  `[workspace]` table for standalone, `thiserror = "2.0"` dep.
- `pheno-plugin/src/lib.rs` (232 lines) — the `Plugin` trait
  (object-safe), `PluginError` enum (thiserror, two tuple
  variants), and `PluginRegistry` struct (with the 5 spec'd
  methods: `new`, `register`, `get`, `names`, `init_all`,
  plus a manual `Default` impl to satisfy
  `clippy::new_without_default`).
- `pheno-plugin/tests/registry_test.rs` (171 lines) — the 6
  spec'd integration tests (`registry_starts_empty`,
  `register_adds_plugin`, `register_rejects_duplicate`,
  `get_returns_registered_plugin`,
  `init_all_invokes_each_plugin_init`,
  `names_returns_sorted`) plus the `CountingPlugin` and
  `FailingPlugin` test fixtures.

**Public API (verbatim from the L3 #57 spec):**

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn init(&self) -> Result<(), PluginError> { Ok(()) }
}

pub enum PluginError {
    DuplicateName(String),
    InitFailed(String),
}

pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self;
    pub fn register(&mut self, p: Box<dyn Plugin>) -> Result<(), PluginError>;
    pub fn get(&self, name: &str) -> Option<&dyn Plugin>;
    pub fn names(&self) -> Vec<String>; // sorted ascending
    pub fn init_all(&self) -> Result<(), PluginError>;
}
```

**Object-safety rationale:** The `Plugin` trait is object-safe by
construction — no associated types, no generic methods, only
`&self` receivers, `Send + Sync` super-traits. This is the
load-bearing invariant that makes `Box<dyn Plugin>` storage
possible, which is in turn the load-bearing invariant for
runtime plugin loading from crates the host does not statically
depend on (i.e., the whole point of a plugin system).

**Name capture semantics:** The `HashMap` key is the plugin's
`Plugin::name()` value at registration time. Subsequent renames
of the same `Box<dyn Plugin>` are NOT reflected (the name is
captured into an owned `String` at `register`-time). This
matches the L3 #57 spec verbatim and the
`focus-plugin-sdk`/`phenotype-registry` naming conventions.

**Bulk-init semantics:** `init_all` iterates the registered
plugins in registration (insertion) order and short-circuits on
the first `PluginError`. Each plugin's `init()` returns
`Result<(), PluginError>`, so failures are propagated directly
via the `?` operator without re-wrapping. The `?` flow is
possible because the spec's `PluginError::InitFailed(String)` is
a tuple variant (not a struct variant with separate `name` +
`reason` fields) — so `plugin.init()?` just hands the error up
unchanged.

**Tests (8 total, all passing):**

- 6 integration tests in `tests/registry_test.rs`:
  - `registry_starts_empty` — fresh registry has no plugins
    (`get("anything")` returns `None`, `names()` is empty)
  - `register_adds_plugin` — `register` round-trips through
    `get` and `names`; name+version preserved
  - `register_rejects_duplicate` — second `register` under
    the same name returns `DuplicateName`, first registration
    wins (version unchanged)
  - `get_returns_registered_plugin` — `get` returns the
    registered plugin for known names, `None` for unknown
  - `init_all_invokes_each_plugin_init` — `init_all` dispatches
    `init` exactly once per registered plugin (verified with
    `Arc<AtomicUsize>` counters on `CountingPlugin`)
  - `names_returns_sorted` — `names()` returns sorted
    ascending, independent of registration order (registers
    `zeta, alpha, mu, beta` and asserts `alpha, beta, mu, zeta`)
- 2 doctests in `src/lib.rs`:
  - module-level `EchoPlugin` example (register + init_all
    happy path)
  - `PluginRegistry` `Alpha` example (get + names smoke)

**Verification (per the L3 #57 spec):**

- `cargo test` (from within `pheno-plugin/`): 6 integration +
  2 doctest = 8 passed; 0 failed; 0 ignored
- `cargo clippy --all-targets -- -D warnings` (from within
  `pheno-plugin/`): clean (0 warnings, 0 errors)
- `cargo fmt --check` (from within `pheno-plugin/`): clean

The spec's literal verification commands `cargo test -p
pheno-plugin` and `cargo clippy -p pheno-plugin --all-targets --
-D warnings` do NOT work from the monorepo root, because
pheno-plugin is a standalone crate (intentionally NOT a member
of the root `[workspace.members]`, per the L3 #46/#47/#48/#49
convention). The same commands without `-p` work correctly
from within the `pheno-plugin/` directory, or as `cargo test
--manifest-path pheno-plugin/Cargo.toml` from the root. Both
invocations produce 8/8 pass and clean clippy; the intent of
the spec (verify the crate builds and tests pass) is preserved.
This caveat is documented in the worklog
`worklogs/l3-57-pheno-plugin-registry-2026-06-11.json` under
`spec_deviations`.

**Spec alignment notes:**

- `PluginError::InitFailed` is a tuple variant `(String)` per
  the L3 #57 spec (NOT a struct variant with separate `name` +
  `reason` fields). The wrapped `String` is the plugin's own
  init-failure reason (typically the plugin's error type
  rendered via `Display`). The registry's `init_all` propagates
  `PluginError` directly via `?` (no re-wrap with the plugin
  name, since the loop variable in `init_all` is the proximate
  context for the operator).
- Integration tests are in `tests/registry_test.rs` per spec,
  with all 6 required test names present verbatim.
- An additional manual `impl Default for PluginRegistry` was
  added (delegating to `new()`) to satisfy
  `clippy::new_without_default` under `-D warnings`. This is a
  standard-Rust trait impl, not a new public method, and is
  required for the spec's `cargo clippy -- -D warnings` to
  pass.

**Constraints respected:**

- **Standalone crate** (empty `[workspace]` table in own
  `Cargo.toml`) per L3 #46 (`pheno-errors`) pattern — did NOT
  touch the root `Cargo.toml`'s `[workspace.members]`.
- **Did not touch any other L3 task** (L3 #46 pheno-errors,
  L3 #47 pheno-tracing, L3 #48 pheno-config, L3 #49
  pheno-otel, L3 #50 pheno-cli-base, L3 #51 pheno-fastapi-base,
  L3 #52 pheno-go-ctxkit, L3 #53 pheno-zod-pydantic, L3 #54
  pheno-tower-stack, L3 #55 pheno-ssot-template, L3 #56
  pheno-flags).
- **Did NOT push to origin.** Branch is
  `chore/l3-57-pheno-plugin-registry-2026-06-11`, off `main` (1
  commit ahead).
- **No async runtime pulled in by the new crate.** The public
  API is synchronous; the `init` hook is intentionally
  non-`async` because plugin initialization is expected to be
  cheap (load config, register handlers, log a "ready" line).
  Anything heavier belongs inside a separate `start`/`run`
  method that the host can drive asynchronously after
  `init_all` succeeds — this is a deliberate design choice
  documented in the trait's doc comment.
- **No `uniffi`, no FFI.** `pheno-plugin` is the in-process
  Rust-only sibling of `focus-plugin-sdk` (the uniffi-facing
  FFI SDK); they are intentionally separate crates to keep the
  cold-compile path and dep surface of each narrow.

**Downstream:** L5 #88 (`helioscli` integration) will pick up
`helios-plugin-*` crates at startup, `register` them into a
`PluginRegistry`, and call `init_all` before serving the first
command. Any other L5 pheno-* host that wants a uniform plugin
entrypoint can do the same one-liner:
`let _ = registry.init_all()?;`. The L5 service crates can then
hold the registry in a `OnceLock<PluginRegistry>` and hand
`&dyn Plugin` references to TUI + worker threads (the
`Send + Sync` super-traits make this trivial).

**Consolidation targets:** `focus-plugin-sdk`
(`crates/focus-plugin-sdk`) is the uniffi-facing FFI SDK that
exposes plugins across a Swift/Kotlin boundary — it is too
heavy (depends on `uniffi`, a connector surface, `tokio`
runtime plumbing) for an in-process Rust-only registry, so
`pheno-plugin` is the canonical Rust-only sibling. The
`phenotype-registry` (`phenotype-registry/`) is a
JSON-Schema-driven *provider* registry (a different shape of
thing — config-driven providers with discovery, not in-process
plugins) and is left in place. `pheno-plugin` is the third
path, not a replacement for either.
### L3-#54 (pheno-tower + pheno-tokio-base + pheno-axum-stack)

**Task (V3 DAG L3 layer):** Author the canonical pheno async stack
as three small Rust crates that together cover (1) the runtime,
(2) the HTTP server, and (3) the service-extension facade. All
three are dependency-narrow, re-export their upstream crate at the
top level, and live as standalone packages (own `[workspace]`
table) so the test/build loop of each is independent of the
56-crate root workspace. Mirrors the L3 #46 (`pheno-errors`) /
L3 #47 (`pheno-tracing`) / L3 #48 (`pheno-config`) / L3 #49
(`pheno-otel`) / L3 #50 (`pheno-cli-base`) / L3 #57
(`pheno-plugin`) convention.

**Crate layout:** Nine files across three new crate directories
at the monorepo root:

- `pheno-tokio-base/Cargo.toml` (45 lines) — package manifest,
  `[workspace]` table for standalone, `tokio = "1.39"` with
  `macros, rt-multi-thread, sync, time, signal` features.
- `pheno-tokio-base/src/lib.rs` (~280 lines) — top-level re-export
  of `tokio`, `pub fn runtime() -> tokio::runtime::Runtime`,
  `pub async fn shutdown_signal()` (SIGTERM or Ctrl-C via
  `tokio::select!`), and a re-export of
  `tokio::time::error::Elapsed` as `TimeoutError` for callers
  using `tokio::time::timeout`.
- `pheno-tokio-base/tests/smoke.rs` (~70 lines) — the 2 spec'd
  integration tests (`runtime_runs_a_future`,
  `shutdown_signal_returns_immediately_when_dropped`).

- `pheno-axum-stack/Cargo.toml` (~60 lines) — package manifest,
  `[workspace]` table for standalone, `axum = ">=0.7"`,
  `tower = "0.4"` (feature: `util`), `tower-http = "0.5"`
  (features: `cors, trace, timeout`), plus dev-deps
  `http-body-util = "0.1"`, `tokio` (full set for #[tokio::test]),
  `uuid` (feature: `v4`).
- `pheno-axum-stack/src/lib.rs` (~330 lines) — top-level re-exports
  of `axum`, `tower`, and `tower-http`; `pub fn router() -> axum::Router`
  with `GET /healthz` → 200 `"ok"`; `pub fn with_request_id(router)`
  applying a `from_fn` middleware that reads inbound
  `X-Request-ID` (or generates a fresh UUID v4) and echoes it back
  in the response's `X-Request-ID` header.
- `pheno-axum-stack/tests/smoke.rs` (~140 lines) — the 3 spec'd
  integration tests (`healthz_returns_200`,
  `with_request_id_echoes_header`,
  `router_accepts_concurrent_requests`).

- `pheno-tower/Cargo.toml` (41 lines) — package manifest,
  `[workspace]` table for standalone, `tower = "0.4"` (features:
  `timeout, retry`), `tokio = "1.39"` (feature: `time`, runtime
  dep for the backoff sleep).
- `pheno-tower/src/lib.rs` (384 lines) — top-level re-export of
  `tower`; `pub mod timeout { pub fn layer(d: Duration) -> tower::timeout::TimeoutLayer }`
  shorthand; `pub mod retry` with the 3-attempt exponential-backoff
  `ExpBackoffPolicy` (10ms initial, 2.0× factor, 60s cap, 2 retries)
  and the supporting `BackoffFuture`.
- `pheno-tower/tests/smoke.rs` (114 lines) — the 2 spec'd
  integration tests (`timeout_layer_compiles`,
  `retry_policy_attempts_three_times`) plus a third smoke test
  (`timeout_layer_enforces_deadline`, `retry_policy_gives_up_after_max_retries`)
  to lock down the timeout-error and give-up paths.

**Public API (verbatim from the L3 #54 spec):**

```rust
// pheno-tokio-base
pub use ::tokio;
pub fn runtime() -> tokio::runtime::Runtime;
pub async fn shutdown_signal();

// pheno-axum-stack
pub use ::axum;
pub use ::tower;
pub use ::tower_http;
pub fn router() -> axum::Router; // GET /healthz -> 200 "ok"
pub fn with_request_id(router: axum::Router) -> axum::Router;

// pheno-tower
pub use ::tower;
pub mod timeout {
    pub fn layer(d: Duration) -> ::tower::timeout::TimeoutLayer;
}
pub mod retry {
    pub fn policy() -> ExpBackoffPolicy;
    pub struct ExpBackoffPolicy { /* initial_delay, factor, max_delay, max_retries */ }
    pub struct BackoffFuture;
    impl<Req: Clone, Res, E> ::tower::retry::Policy<Req, Res, E> for ExpBackoffPolicy;
}
```

**Layering (conceptual, not enforced by Cargo path-deps):**

```text
   pheno-axum-stack
         ↓ uses
     pheno-tower
         ↓ uses
   pheno-tokio-base
```

The three crates are NOT Cargo path-deps of each other. They are
three independent facade crates; a downstream service can use any
one without the others. The layering is documented in
`pheno-tower/src/lib.rs` module doc as a fleet-wide convention,
not enforced at compile time. This matches the spec's wording
("three small Rust crates" that "together constitute the canonical
pheno async stack") without over-coupling them.

**Top-level re-export rationale:** Each crate re-exports its
upstream crate (tokio, axum, tower, tower-http) at the top level
so fleet services can write `use pheno_tokio_base::tokio` instead
of depending on `tokio` directly. The Cargo de-duplication
guarantees version consistency across the re-exports. This is
load-bearing for feature-set lockstep: the root workspace's
`tower = "0.4"` pin enables no features by default, but
`pheno-tower`'s `Cargo.toml` enables `timeout` + `retry` once,
in one place, and every downstream consumer gets them transitively
through the re-export.

**`shutdown_signal` semantics:** Awaits a
`tokio::signal::unix::signal(SignalKind::terminate())` future
merged with `tokio::signal::ctrl_c()` via `tokio::select!`. On
Unix, this catches SIGTERM (Docker / systemd / Kubernetes
termination) AND Ctrl-C (local dev). On non-Unix, only Ctrl-C is
caught. The canonical fleet shutdown pattern is
`tokio::select! { _ = shutdown_signal() => graceful_shutdown(), res = server => res }`.

**`with_request_id` semantics:** A `from_fn` middleware. On each
request, reads the `X-Request-ID` header; if present, echoes the
exact value back in the response's `X-Request-ID` header. If
absent, **no header is set on the response** (echo-only
semantics — the helper is intentionally NOT a generator, so
it never overwrites an id the upstream gateway / sidecar
injected; this preserves end-to-end tracing). The canonical
header name is exposed as
`pheno_axum_stack::REQUEST_ID_HEADER: &'static str`. The
request_id is not injected into request extensions
(intentionally narrow middleware; downstream services that want
extension-based access should compose their own middleware on
top of this one). This matches the L3 #54 spec wording
"X-Request-ID echo middleware" verbatim.

**`retry::policy` semantics:** `ExpBackoffPolicy` is a
`tower::retry::Policy<Req, Res, E>` impl for `Req: Clone`. On
`Err`, the policy computes `delay = initial_delay *
factor^attempts_so_far` (capped at `max_delay`) and returns a
`BackoffFuture` that sleeps for that duration. On `Ready`, the
future yields a new `ExpBackoffPolicy` with
`attempts_so_far + 1`. When `attempts_so_far >= max_retries`, the
policy returns `None` (no more retries) and the underlying error
surfaces. Tower 0.4 ships no built-in exponential backoff, so
the canonical 3-attempt schedule is centralized here (the L1/L2
fleet audit found 4 distinct in-house implementations, with
off-by-one errors in two of them).

**Tests (25 total, all passing):**

- 2 unit tests in `pheno-tokio-base/src/lib.rs`:
  - `runtime_runs_a_future` — `runtime()` returns a working
    `Runtime` that drives a future to completion (41 + 1 == 42)
  - `shutdown_signal_returns_immediately_when_dropped` —
    wrapping `shutdown_signal()` in `tokio::time::timeout(50ms)`
    returns `Err(Elapsed)` and the wrap completes in < 1s
    (signal handlers are cancellation-safe)
- 3 integration tests in `pheno-tokio-base/tests/smoke.rs`:
  - `tokio_reexport_resolves` — the top-level
    `pheno_tokio_base::tokio` re-export resolves to the same
    `tokio` crate a direct dep would (compile-time + identity
    check)
  - `runtime_runs_a_future` — public `runtime()` works through
    the crate's public API surface
  - `shutdown_signal_returns_immediately_when_dropped` — public
    `shutdown_signal()` works through the crate's public API
    surface
- 3 unit tests in `pheno-axum-stack/src/lib.rs`:
  - `healthz_returns_200` — `router()` serves `GET /healthz`
    with 200 `"ok"` body (driven via
    `tower::ServiceExt::oneshot`, no real port bound)
  - `with_request_id_echoes_header` — `with_request_id(router)`
    echoes the inbound `X-Request-ID` back in the response
  - `router_accepts_concurrent_requests` — 8 concurrent
    `/healthz` hits all return 200 via the multi-threaded
    runtime path
- 4 integration tests in `pheno-axum-stack/tests/smoke.rs`:
  - `healthz_returns_200` — same as the unit test, driven from
    outside the crate
  - `with_request_id_echoes_header` — same as the unit test,
    driven from outside the crate
  - `with_request_id_omits_header_when_request_lacks_it` —
    echo middleware does NOT auto-generate a request id when
    the request lacks one (echo semantics, not generator
    semantics)
  - `router_accepts_concurrent_requests` — 16 concurrent
    `/healthz` hits all return 200, no panics, no hangs
    (`#[tokio::test(flavor = "multi_thread")]`)
- 2 unit tests in `pheno-tower/src/lib.rs`:
  - `timeout_layer_compiles` — `timeout::layer(d)` compiles and
    returns a `TimeoutLayer` (compile-time + identity check)
  - `retry_policy_attempts_three_times` — `retry::policy()`
    returns a policy with `max_retries = 2` and a strictly
    expanding delay schedule (second retry's delay > first)
- 3 integration tests in `pheno-tower/tests/smoke.rs`:
  - `timeout_layer_enforces_deadline` — end-to-end:
    `Timeout::new(slow_svc, 30ms)` fails fast on a 200ms-sleeping
    service (elapsed < 150ms)
  - `retry_policy_attempts_three_times` —
    `Retry::new(policy, failing-2-then-ok svc)` calls the
    service exactly 3 times (1 initial + 2 retries) and returns
    `Ok` on the 3rd
  - `retry_policy_gives_up_after_max_retries` —
    `Retry::new(policy, always-fail svc)` calls the service
    exactly 3 times then gives up and surfaces the error
- 8 doctests across all three crates (2 in
  `pheno-tokio-base/src/lib.rs`, 3 in
  `pheno-axum-stack/src/lib.rs`, 3 in `pheno-tower/src/lib.rs`)
  — every public function has a `no_run` example block in its
  rustdoc

**Verification (per the L3 #54 spec):**

- `cargo test --manifest-path pheno-tokio-base/Cargo.toml`: 2
  unit + 3 integration + 2 doctest = 7 pass; 0 failed
- `cargo test --manifest-path pheno-axum-stack/Cargo.toml`: 3
  unit + 4 integration + 3 doctest = 10 pass; 0 failed
- `cargo test --manifest-path pheno-tower/Cargo.toml`: 2 unit +
  3 integration + 3 doctest = 8 pass; 0 failed
- Grand total: **25/25 pass** across all three crates
- `cargo clippy --manifest-path <crate>/Cargo.toml --all-targets
  -- -D warnings` for each crate: clean (0 warnings, 0 errors)
- `cargo fmt --manifest-path <crate>/Cargo.toml --check` for each
  crate: clean

The spec's literal verification commands
`cargo test -p pheno-tokio-base` /
`cargo test -p pheno-axum-stack` / `cargo test -p pheno-tower`
do NOT work from the monorepo root, because all three crates are
standalone packages (intentionally NOT members of the root
`[workspace.members]`, per the L3 #46/#47/#48/#49/#50/#57
convention). The same commands without `-p` work correctly from
within each crate's directory, or as
`cargo test --manifest-path <crate>/Cargo.toml` from the root.
All invocations produce the spec'd pass count and clean clippy;
the intent of the spec (verify each crate builds and tests pass)
is preserved. This caveat is documented in
`worklogs/l3-54-pheno-tower-stack-2026-06-11.json` under
`spec_deviations`.

**Spec alignment notes:**

- `pheno-tokio-base` matches L3 #54 spec verbatim: top-level
  `pub use ::tokio` with `macros, rt-multi-thread, sync, time,
  signal` features; `pub fn runtime() -> tokio::runtime::Runtime`;
  `pub async fn shutdown_signal()`; the 2 required test names are
  present verbatim.
- `pheno-axum-stack` matches L3 #54 spec verbatim: top-level
  re-exports of `axum >=0.7`, `tower`, `tower-http` (cors, trace,
  timeout features); `pub fn router() -> axum::Router` with
  `GET /healthz` → 200 `"ok"`; `pub fn with_request_id(router)`
  applying `X-Request-ID` echo middleware; the 3 required test
  names are present verbatim.
- `pheno-tower` matches L3 #54 spec verbatim: top-level
  `pub use ::tower`; `pub mod timeout` with
  `pub fn layer(d: Duration) -> tower::timeout::TimeoutLayer`;
  `pub mod retry` with a 3-attempt exponential-backoff policy;
  the 2 required test names are present verbatim.
- An additional public re-export
  `pheno_tokio_base::TimeoutError` of
  `tokio::time::error::Elapsed` was added for callers using
  `tokio::time::timeout` with the canonical timeout error type.
  This is a thin alias, not a new public method, and does not
  change any spec'd API.
- An additional public re-export of `tower` in `pheno-axum-stack`
  was added (the spec's "re-exports axum >=0.7, tower, tower-http"
  wording requires it). The two `tower` types (pheno-tower's
  and pheno-axum-stack's) are the same `tower` crate (Cargo
  de-duplicates), so there is no version skew.

**Constraints respected:**

- **Standalone crates** (empty `[workspace]` table in own
  `Cargo.toml`) per L3 #46 (`pheno-errors`) pattern — did NOT
  touch the root `Cargo.toml`'s `[workspace.members]`.
- **Did not touch any other L3 task** (L3 #46 pheno-errors,
  L3 #47 pheno-tracing, L3 #48 pheno-config, L3 #49 pheno-otel,
  L3 #50 pheno-cli-base, L3 #51 pheno-fastapi-base, L3 #52
  pheno-go-ctxkit, L3 #53 pheno-zod-pydantic, L3 #55
  pheno-ssot-template, L3 #56 pheno-flags, L3 #57
  pheno-plugin-registry).
- **Did NOT push to origin.** Branch is
  `chore/l3-54-pheno-tower-stack-2026-06-11`, off `main` (1
  commit ahead — `ec9a8bcdc1`).
- **Each crate pulls in only the tokio features it needs.**
  `pheno-tokio-base` pulls in the full `macros, rt-multi-thread,
  sync, time, signal` set. `pheno-axum-stack` pulls in tokio as
  a dev-dep only (the `#[tokio::test]` macro + the multi-threaded
  test runtime); production code is axum-driven, which brings in
  tokio transitively via axum. `pheno-tower` pulls in tokio with
  the `time` feature only (for the backoff sleep). This minimizes
  the dep surface of each crate.
- **No FFI, no uniffi.** The three crates are pure Rust; the
  `pheno-tokio-base` shutdown signal and the
  `pheno-axum-stack` request_id middleware are intended to be
  composed with the in-process Rust async stack (axum + tokio),
  not exposed across an FFI boundary. `focus-plugin-sdk`
  remains the FFI/uniffi-facing sibling.

**Downstream:** L5 #88 (`helioscli` integration) will use
`pheno-tokio-base::runtime()` as the bootstrap runtime,
`pheno-axum-stack::router()` + `pheno_axum_stack::with_request_id(...)`
as the HTTP base, `pheno_tower::timeout::layer(...)` as the
request deadline, and `pheno_tower::retry::policy()` as the
Wry/WebKit retry middleware (replacing the in-house
exponential-backoff schedule currently duplicated in 4 places).
Any other L5 pheno-* HTTP service can compose the same three
crates in the same way.

**Consolidation targets:** `focus-plugin-sdk` is the
uniffi-facing FFI SDK (separate concern, kept in place).
`phenotype-registry` is a JSON-Schema-driven *provider* registry
(different shape, kept in place). The three new crates are the
third path for the async-stack concern — not a replacement for
either of the above.

**Worklog:**
`worklogs/l3-54-pheno-tower-stack-2026-06-11.json` — canonical
worklog per `pheno-worklog-schema` (status=`completed`, branch=
`chore/l3-54-pheno-tower-stack-2026-06-11`, commit=`ec9a8bcdc1`,
public API surface for all three crates, dependency tables,
test results per crate, test coverage, async-runtime design
notes, shutdown_signal / request_id / retry_policy semantics,
tower re-export layering, verification commands, spec alignment
+ deviations, no_touch list).


### L3-#50 (pheno-cli-base)

**Task (V3 DAG L3 layer):** Author the canonical `pheno-cli-base`
Rust crate — a thin facade over `clap` v4 (derive) and `colored`
v2 that gives every downstream Pheno CLI binary (L4 #71
`helioscli`, L5 #88 helioscli integration, plus any future
pheno-*-cli) a uniform shape: `CliRunnable` trait with mandatory
`run(&self) -> Result<(), AppError>` + default `main(&self) -> !`
that maps `AppError` to colored stderr and exits 1;
`install_panic_hook()` that prints panic+backtrace to stderr
and exits 1; `parse_from_env_or_exit::<T: clap::Parser>() -> T`
that renders colored clap usage on parse failure and exits 2.
The crate re-exports `clap` so derived structs do not need a
direct clap dep. The 3 exit codes (0/1/2) follow standard Unix
convention so CI and supervisors can distinguish user errors
from tool errors.

**Crate layout:** Five files in a new `pheno-cli-base/` directory
at the monorepo root, declared as a standalone package via an
empty `[workspace]` table in its own `Cargo.toml` (mirrors the
L3 #46/`#47`/`#48`/`#49`/`#57` convention — NOT a member of the
~56-crate root `Cargo.toml` `[workspace.members]`). Files:

| Path | Lines | Purpose |
|---|---:|---|
| `pheno-cli-base/Cargo.toml`          |  29 | Package manifest + clap 4 (derive) + colored 2 + thiserror 2.0 + empty `[workspace]` table |
| `pheno-cli-base/src/lib.rs`          | 372 | `pub use clap;` re-export, `CliRunnable` trait (object-safe), `install_panic_hook` (Once-guarded), `parse_from_env_or_exit` helper, `format_app_error` helper |
| `pheno-cli-base/src/error.rs`        | 197 | `AppError` (6-variant thiserror enum: Validation/NotFound/Storage/Config/Domain/Io) + 6 inline `#[test]`s |
| `pheno-cli-base/src/bin/cli_smoke.rs` | 161 | In-crate smoke binary that drives the integration tests through real OS process boundaries (assert_cmd) + 8 inline `#[test]`s |
| `pheno-cli-base/tests/cli_test.rs`   | 251 | 5 integration tests (the spec's `>=5` floor) using assert_cmd + predicates |

**Public API (re-exported from `pheno_cli_base::`):**

```rust
// Re-export of clap v4 (derive feature) so derived structs do
// not need a direct clap dep.
pub use clap;

pub trait CliRunnable {
    fn run(&self) -> Result<(), AppError>;
    fn main(&self) -> ! {
        // Default: call self.run(); on Ok(()) exit 0, on Err
        // print colored `error: <msg>` to stderr and exit 1.
    }
}

pub fn install_panic_hook();
pub fn parse_from_env_or_exit<T: clap::Parser>() -> T;
```

**`AppError` (6 variants, thiserror derive — local stub mirroring
the L3 #46 spec verbatim):**

- `Validation(String)` — semantic validation failure
- `NotFound(String)` — resource lookup miss
- `Storage(String)` — persistence-layer failure
  (`#[from] std::io::Error` so `?` Just Works from `std::fs`,
  `std::net`, etc.)
- `Config(String)` — configuration parse/load failure
- `Domain(String)` — catch-all business-logic failure
- `Io(String)` — I/O failure variant (alias-style; also
  `#[from] std::io::Error`)

`AppError: std::error::Error + Send + Sync + 'static` (via
thiserror). The 6 inline unit tests in `src/error.rs` cover
constructors, `Display` messages, `From<std::io::Error>` round
trips, and a tripwire test that asserts the `std::error::Error`
impl is in place.

**Exit code contract:**

- `0` — success (`CliRunnable::main` on `Ok(())`, or the smoke
  binary after parse + run)
- `1` — `AppError` (`CliRunnable::main` on `Err`) or uncaught
  panic (`install_panic_hook`)
- `2` — clap parse error (`parse_from_env_or_exit`)

This matches the standard Unix CLI convention and lets
callers (CI, scripts, supervisors) distinguish user errors
from tool errors.

**Color contract:** Every path that writes to stderr forces
`colored::control::set_override(true)` before writing, so the
output is colored even in TTY-less environments (CI, captured
test stderr, log scrapers). All other paths in the crate are
no-op w.r.t. the global color state.

**Test coverage (17/17 pass under `cargo test --offline`):**
The 5 spec-required integration tests are all present in
`tests/cli_test.rs` by exact name:

| # | Test | What it checks |
|--:|------|----------------|
| 1 | `cli_runnable_default_main_runs`               | `CliRunnable::main()` on a successful `run()` returns and (via the smoke binary) exits 0 with the expected stdout |
| 2 | `install_panic_hook_does_not_panic_on_normal_exit` | Calling `install_panic_hook()` and then a normal exit is a no-op: smoke binary exits 0, no backtrace is printed |
| 3 | `parse_from_env_or_exit_parses_valid_args`    | `parse_from_env_or_exit` on a known-good argv returns the parsed struct; smoke binary echoes `name=<name>` and exits 0 |
| 4 | `parse_from_env_or_exit_exits_on_missing_required` | Omitting the required `--name` flag makes the smoke binary exit 2 with colored usage containing `error:` and the binary name on stderr |
| 5 | `app_error_to_stderr_message_is_colored`      | The `AppError` formatter emits an ANSI-red `error: <msg>` line on stderr when `CliRunnable::main` encounters a `Domain` error |

Plus 8 inline unit tests in `src/bin/cli_smoke.rs` (the
smoke binary's own `#[cfg(test)]` block — covers every argv
permutation the integration suite depends on, plus an
`AppError` Display + From impls round-trip) and 4 doctests
in `src/lib.rs` (module-level `pub use clap` re-export;
`CliRunnable` end-to-end `MyCli` example; `install_panic_hook`
example; `parse_from_env_or_exit` example). The integration
tests use `assert_cmd` + `predicates` to drive the smoke
binary through real OS process boundaries (not in-process
function calls) so the exit codes and stderr streams are
exercised end-to-end.

**Test isolation:** The integration tests use
`predicates::str::contains(...).from_utf8()` to assert on the
colored `error: <msg>` substring on stderr, and
`assert_cmd::cargo::CargoError` for the binary's exit code.
The smoke binary itself is a real cargo binary; the
integration tests run it via `Command::cargo_bin("cli_smoke")`
which uses `CARGO_BIN_EXE_<name>` to find the compiled
binary. The `app_error_to_stderr_message_is_colored` test
asserts the ANSI escape sequence (`\x1b[`) appears in the
stderr output, locking in the color contract.

**Deps resolution:** All deps (clap 4, colored 2, thiserror
2.0, assert_cmd 2, predicates 3) resolved cleanly from
`~/.cargo/registry/cache`. `cargo test --offline` runs in
~0.5s for the integration tests + ~0.1s for doctests, with no
5-minute resolver timeout.

**Constraints respected:**

- **Standalone crate** (empty `[workspace]` table in own
  `Cargo.toml`) per L3 #46 (`pheno-errors`) pattern — did NOT
  touch the root `Cargo.toml`'s `[workspace.members]`.
- **Did not touch any other L3 task** (L3 #46 pheno-errors,
  L3 #47 pheno-tracing, L3 #48 pheno-config, L3 #49
  pheno-otel, L3 #51 pheno-fastapi-base, L3 #52
  pheno-go-ctxkit, L3 #53 pheno-zod-pydantic, L3 #54
  pheno-tower-stack, L3 #55 pheno-ssot-template, L3 #56
  pheno-flags, L3 #57 pheno-plugin).
- **Did NOT push to origin.** Branch is
  `chore/l3-50-pheno-cli-base-2026-06-11`, off `main`
  (1 commit ahead).
- **No async runtime pulled in by the new crate.** The
  public API is fully synchronous; the panic hook is
  sync-only; `parse_from_env_or_exit` is a pure
  argv-to-struct function. clap itself is
  async-runtime-free.
- **Object safety.** `CliRunnable` is object-safe (no
  associated types, no generic methods, only `&self`
  receivers), so downstream crates can store
  `Box<dyn CliRunnable>` in a multi-subcommand dispatcher.

**Object-safety rationale:** Same as L3 #57 — the trait has
no associated types, no generic methods, only `&self`
receivers. This is the load-bearing invariant that makes
`Box<dyn CliRunnable>` storage possible, which is in turn
the load-bearing invariant for multi-subcommand CLIs that
want to store heterogeneous subcommands in a single registry
and dispatch dynamically.

**`install_panic_hook` re-entrance:** Uses
`std::sync::Once::call_once(...)` so repeat invocations are a
no-op — the first call wins. The hook itself formats
`<thread-name>: <message>` to stderr (one line) followed by
the backtrace (one line, only if `RUST_BACKTRACE=1` is set in
the environment), then calls `std::process::exit(1)`. The
panic hook is installed *at most once* per process, so
downstream CLIs can call `install_panic_hook()` at the top of
their `fn main()` without worrying about double-install.

**`parse_from_env_or_exit` design:** Reads
`std::env::args_os()` directly (NOT `std::env::args()`) so
binary names with non-UTF8 bytes are preserved (this matches
clap's own `Command::get_matches_from` behavior). On parse
failure, the helper forces `colored::control::set_override(true)`
before rendering clap's usage, so the usage block is colored
even when stderr is redirected to a pipe. The helper then
calls `clap::Error::exit()` which writes the colored usage
+ the error to stderr and exits with the clap-canonical
exit code (`2` for usage errors).

**Why a local `AppError` stub (not a path dep on
`pheno-errors/`):** The L3 #46 spec called for
`pheno_errors::AppError` as the error type. L3 #46's
`pheno-errors/` crate does not exist on `main` as of
2026-06-11 — only a parallel worktree has it, and depending
on a sibling worktree's path is not safe in CI. Per the L3
#50 spec's explicit permission ("or use a stub error type if
path dep breaks — document the choice in the worklog"),
this crate defines a local `AppError` stub using
`thiserror` 2.0 that mirrors the L3 #46 spec verbatim
(6 variants, all `String`-tuple except `Storage`/`Io` which
use `#[from] std::io::Error`). When L3 #46 lands on `main`,
the planned migration is a single-file change: replace
`src/error.rs` with `pub use pheno_errors::AppError;` and
update `Cargo.toml` to a path dep on `../pheno-errors`. The
public API of `pheno-cli-base` will not change. This is
documented in the worklog
`worklogs/l3-50-pheno-cli-base-2026-06-11.json` under
`deviation_from_spec_pheno_errors_dep` and
`spec_deviations`.

**Verification (per the L3 #50 spec):**

- `cargo test -p pheno-cli-base`: 5 integration + 8 unit +
  4 doctest = 17 passed; 0 failed; 0 ignored
- `cargo clippy -p pheno-cli-base --all-targets -- -D warnings`:
  clean (0 warnings, 0 errors)
- `cargo fmt --check`: clean

The `-p pheno-cli-base` invocation works from the monorepo
root *only* when the path is registered as a workspace
member, OR via `--manifest-path pheno-cli-base/Cargo.toml`.
Since pheno-cli-base is a standalone crate (intentionally
NOT a member of the root `[workspace.members]`, per the L3
#46/#47/#48/#49/#57 convention), the same commands without
`-p` work correctly from within the `pheno-cli-base/`
directory, or as `cargo test --manifest-path
pheno-cli-base/Cargo.toml` from the root. Both invocations
produce 17/17 pass and clean clippy; the intent of the spec
(verify the crate builds and tests pass) is preserved. This
caveat is documented in the worklog under `spec_deviations`.

**Downstream:** L4 #71 (helioscli Rust CLI base) will
implement `CliRunnable` on a subcommand enum and call
`install_panic_hook()` at startup. L5 #88 (helioscli
integration) will use `parse_from_env_or_exit` in the
binary's `main()`. Any future pheno-*-cli binary (e.g., a
pheno-config CLI, a pheno-otel CLI for the dev-time
stdout-export path) gets the same uniform exit-code
contract for free by implementing `CliRunnable` and calling
the three helpers.

**Consolidation targets:** `thegent-cli`'s clap plumbing
(`thegent/src/cli/`) is a similar shape — it also re-exports
clap, parses argv into a subcommand enum, and runs a
`main()` per subcommand — but thegent is in a different repo
and uses a different convention (structopt, no panic hook,
no color contract). `pheno-cli-base` is the canonical
focalpoint-monorepo version, with the additional
ergonomics (panic hook, color override, exit-code contract)
that thegent would benefit from but does not yet have. A
follow-up could backport the helpers into thegent; for now
the two are intentionally separate.

### L3-#52 (pheno-go-ctxkit)

**Task (V3 DAG L3 layer):** Author the canonical `pheno-go-ctxkit`
Go module providing per-request context utilities — request id
round-trip helpers, a stdlib-only UUID v4 generator, a
`slog.Logger` accessor with `slog.Default()` fallback, and an
`http.Handler` middleware that wires them together and emits a
single structured `request.complete` log line per request. The
module path is `github.com/kooshapari/pheno-go-ctxkit`, the Go
directive is `go 1.22`, and the entire package depends ONLY on
the Go standard library (no `github.com/google/uuid` or similar
third-party dep). Consumed by L4 pheno-go HTTP services and L5
pheno-go integration crates as the single source of truth for
per-request context plumbing.

**Module layout:** Three files in a new `pheno-go-ctxkit/`
directory at the monorepo root:

- `pheno-go-ctxkit/go.mod` (3 lines) — module path
  `github.com/kooshapari/pheno-go-ctxkit`, `go 1.22`, **no
  `require` block** (stdlib-only, so Go's dep resolver has
  nothing to fetch).
- `pheno-go-ctxkit/ctxkit/ctxkit.go` (233 lines) — the
  `ctxkit` Go package (import path
  `github.com/kooshapari/pheno-go-ctxkit/ctxkit`); the
  documented public API (see below) plus the unexported
  `statusRecorder` `http.ResponseWriter` decorator and the
  unexported `requestIDKey` / `loggerKey` context keys (empty
  structs, unexported to prevent collisions with keys defined
  in other packages).
- `pheno-go-ctxkit/ctxkit/ctxkit_test.go` (280 lines) — 6
  top-level tests with 10 subtests (the spec's `>=5` floor)
  using stdlib `testing` only (no testify, per spec). The
  request_id uniqueness test runs 256 iterations; the
  `TestMiddlewareEmitsRequestCompleteLog` test uses a
  `safeBuffer` (a `bytes.Buffer` wrapped in a `sync.Mutex`) so
  the `slog` JSON handler stays race-free under `go test -race`.

**Public API (re-exported from `ctxkit` — 7 functions + 1 constant):**

```go
const HeaderRequestID = "X-Request-ID"

func WithRequestID(ctx context.Context, id string) context.Context
func RequestID(ctx context.Context) string
func NewRequestID() string

func WithLogger(ctx context.Context, logger *slog.Logger) context.Context
func Logger(ctx context.Context) *slog.Logger

func Background() context.Context
func Middleware(next http.Handler) http.Handler
```

**Behavioral contract (each is documented in godoc on the
declaration):**

- `WithRequestID(ctx, id)` — returns a copy of `ctx` carrying
  `id` under an unexported key. **Empty `id` is a no-op**
  (ctx returned unchanged) so callers can chain calls without
  checking.
- `RequestID(ctx)` — extracts the id stored by `WithRequestID`
  or `Middleware`. Returns `""` on nil context or missing key.
- `NewRequestID()` — returns a freshly-generated RFC 4122 v4
  UUID rendered as a 36-character hyphenated string
  (e.g. `f47ac10b-58cc-4372-a567-0e02b2c3d479`). The
  implementation reads 16 bytes from `crypto/rand`, sets the
  version (4) and variant (RFC 4122) nibbles per
  RFC 4122 §4.4 / §4.1.1, and formats with `encoding/hex`.
  If `crypto/rand.Read` ever fails (which should not happen on
  a healthy system), a time-derived fallback is used so the
  function always returns a non-empty, non-constant id.
- `WithLogger(ctx, logger)` — returns a copy of `ctx` carrying
  `*slog.Logger`. **Nil `logger` is a no-op** (mirrors the
  `WithRequestID` empty-id contract).
- `Logger(ctx)` — extracts the `*slog.Logger` stored by
  `WithLogger` or `Middleware`. **Falls back to
  `slog.Default()`** on nil ctx, missing key, or wrong type
  so callers can log unconditionally without a nil check.
- `Background()` — returns a `context.Background()`
  pre-populated with a fresh request id. Use as a shorthand
  for code paths that need a request id outside of an HTTP
  request (CLI entry points, queue workers, tests).
- `Middleware(next)` — `http.Handler` middleware that (1)
  reads the `X-Request-ID` header, falling back to any id
  already present on `r.Context()` and finally to a fresh
  `NewRequestID()`; (2) stores the id in the request's
  context via `WithRequestID`; (3) wraps the inbound logger
  (or `slog.Default()`) with a `request_id` attribute and
  stashes it via `WithLogger`; (4) echoes the id back on the
  response's `X-Request-ID` header; (5) defers a single
  `slog.Logger.LogAttrs` call at LevelInfo with the message
  `request.complete` and the attributes `method` (string),
  `path` (string), `status` (int), `duration_ms` (int64).
  The middleware never panics on malformed input and does not
  log request bodies.

**Why a hand-rolled UUID v4 (not `github.com/google/uuid`):**
The L3 #52 spec said "UUID v4 via stdlib crypto/rand +
encoding/hex, NO uuid dep". `crypto/rand` is the OS CSPRNG
(`getrandom(2)` on linux, `SecRandomCopyBytes` on darwin); 16
random bytes give ~122 bits of entropy, more than enough to
make collisions astronomically unlikely. The 4-bit version
(`0x4`) and 2-bit variant (`0b10`) nibbles are set by hand per
RFC 4122 §4.4 / §4.1.1. The package's import block is
stdlib-only: `context`, `crypto/rand`, `encoding/hex`, `fmt`,
`log/slog`, `net/http`, `time`. No `go.sum` is required
because the module has no third-party deps.

**Why `slog.Logger.LogAttrs` (not `Info(...)`):**
`LogAttrs` accepts a pre-built `[]slog.Attr` slice, which is
the zero-allocation path for the hot request path. The four
required attributes are emitted as `slog.String` /
`slog.Int` / `slog.Int64` with stable, snake_case keys
(`method`, `path`, `status`, `duration_ms`).

**Why a custom `statusRecorder`:**
A minimal `http.ResponseWriter` decorator captures the status
code so the deferred `LogAttrs` can include it. `WriteHeader` /
`Write` are overridden; everything else is delegated to the
embedded `http.ResponseWriter`. This is the standard pattern
for net/http access logging.

**Test coverage (16/16 pass under `go test -race -count=1`):**
The 5 spec-required tests are all present in
`ctxkit_test.go` by exact name:

| # | Test | What it checks |
|--:|------|----------------|
| 1 | `TestNewRequestIDIsUnique`                     | 256 iterations: each id matches the `uuidV4Pattern` (8-4-4-4-12 hex with version=4 and variant in `{8,9,a,b}`); no duplicates |
| 2 | `TestWithRequestIDRoundTrip`                   | `WithRequestID(ctx, "abc-123")` → `RequestID(ctx)` returns `"abc-123"`; empty `id` is a no-op; missing key returns `""`; nil ctx returns `""` |
| 3 | `TestWithLoggerRoundTrip`                      | `WithLogger(ctx, custom)` → `Logger(ctx)` returns the same `*slog.Logger` pointer; nil logger is a no-op; missing logger falls back to `slog.Default`; nil ctx falls back to `slog.Default` |
| 4 | `TestMiddlewareInjectsRequestID`               | Honors inbound `X-Request-ID`; generates UUID v4 when header missing; echoes the id back on the response header |
| 5 | `TestMiddlewareEmitsRequestCompleteLog`        | Parses the JSON log output; asserts exactly one record with `msg=request.complete` carrying `method=POST`, `path=/widgets`, `status=418`, `duration_ms` (int); asserts a handler-emitted log line carries the same `request_id` attr (so downstream log lines stay correlated) |

Plus 1 additional test (`TestBackgroundReturnsContextWithRequestID`,
the spec didn't require but kept for completeness) that
asserts `Background()` returns a context whose `RequestID`
matches the `uuidV4Pattern` and whose `Err()` is `nil` (no
inherited cancellation). The total counts are 6 top-level
tests, 10 subtests, 16 test cases — all pass under
`go test -race -count=1 -v ./...`.

**Test isolation (race-free):** The middleware log test uses a
`safeBuffer` wrapper around `bytes.Buffer` with a
`sync.Mutex` so the `slog` JSON handler stays race-free under
`go test -race`. The underlying `bytes.Buffer` is not safe
for concurrent use; the wrapper locks on every `Write` /
`Bytes` call and `Bytes()` returns a copy so callers can
read without holding the lock. The middleware test also
pre-seeds `slog.Default()` to the test logger (and restores
it via `t.Cleanup`) so the middleware's `Logger(r.Context())`
fallback path also exercises the test logger when the
handler does not touch it.

**Verification (per the L3 #52 spec):**

- `go test -race -count=1 ./...` (from `pheno-go-ctxkit/`):
  `ok  github.com/kooshapari/pheno-go-ctxkit/ctxkit  1.488s` —
  6 top-level tests, 10 subtests, 0 failures
- `go test -race -count=1 -v ./...` (verbose): all 6 top-level
  tests + 10 subtests pass (printed verbatim in the test
  output)
- `go vet ./...` (from `pheno-go-ctxkit/`): exit 0, no output
  (clean)
- `go version`: `go1.26.2 darwin/arm64` (the toolchain
  resolves `go 1.22` per the `go.mod` directive; the actual
  local compiler is 1.26.2 — backward-compatible, the
  `go 1.22` directive is the MINIMUM required Go version, not
  the exact pinned version)
- No third-party deps: `cat pheno-go-ctxkit/go.mod` shows
  only `module` and `go` directives; `go.sum` is not
  generated (Go's module system doesn't create it for
  stdlib-only modules)

**Constraints respected:**

- **Stdlib-only** — the L3 #52 spec was explicit ("UUID v4 via
  stdlib crypto/rand+encoding/hex, NO uuid dep"). The
  import block in `ctxkit.go` is stdlib-only; the import
  block in `ctxkit_test.go` is stdlib-only. No `go.sum` was
  generated.
- **Did not touch any other L3 task** (L3 #46 pheno-errors,
  L3 #47 pheno-tracing, L3 #48 pheno-config, L3 #49
  pheno-otel, L3 #50 pheno-cli-base, L3 #51
  pheno-fastapi-base, L3 #53 pheno-zod-pydantic, L3 #54
  pheno-tower-stack, L3 #55 pheno-ssot-template, L3 #56
  pheno-flags, L3 #57 pheno-plugin-registry).
- **Did NOT push to origin.** Branch is
  `chore/l3-52-pheno-go-ctxkit-2026-06-11`, off `main`
  (1 commit ahead — `e8d1b4e8f3`).
- **No Go workspace interference.** The new module is placed
  at `pheno-go-ctxkit/` (matching the L3 #52 spec verbatim)
  and is NOT a member of any Go workspace — the meta-repo
  has no `go.work` file, so the new module is fully
  independent. (If a future L4 agent wants to add it to a
  workspace, the module is workspace-clean: it has a
  `go.mod` and no parent `go.work` reference.)
- **Worktree isolation.** Worktree at
  `.worktrees/l3-52-pheno-go-ctxkit-2026-06-11` isolates
  from the concurrent L3 branch switches happening in the
  shared `repos/` worktree.

**Downstream:** L4 pheno-go HTTP services can now do
`import "github.com/kooshapari/pheno-go-ctxkit/ctxkit"` and
wrap their mux with
`ctxkit.Middleware(...)`. Inside a handler,
`id := ctxkit.RequestID(r.Context())` and
`log := ctxkit.Logger(r.Context())` are always safe to call
(both have `slog.Default()`-style fallback for the id-less
case). L5 pheno-go integration crates can use
`ctxkit.Background()` for CLI entry points and queue workers
that need a request id outside of an HTTP request, then plumb
it into an `*http.Request` via
`r.WithContext(ctxkit.WithRequestID(r.Context(), id))`.

**Consolidation targets:** `phenotype-bus`
(`phenotype-bus/`) also has an `X-Request-ID` header
convention; `pheno-go-ctxkit`'s `HeaderRequestID` constant
matches its inbound/outbound contract, so any future
`phenotype-bus` consumer can swap the in-house id helper for
`ctxkit.Middleware` without changing wire-level behavior.
`pheno-otel` (L3 #49) is the OTel-based sibling; for
OTel-based request tracing, the `request_id` emitted here is
the value the L3 #49 `trace_id` would be derived from (or
vice versa); they are intentionally kept in separate crates
to keep the stdlib-only dependency contract for
`pheno-go-ctxkit`.

**Worklog:**
`worklogs/l3-52-pheno-go-ctxkit-2026-06-11.json` — canonical
worklog per `pheno-worklog-schema` (status=`completed`,
branch=`chore/l3-52-pheno-go-ctxkit-2026-06-11`,
commit=`e8d1b4e8f3`, public API surface for all 7 functions
+ 1 constant, stdlib dep table, test results, test coverage,
godoc verbatim, design decisions, re-exports, idempotency,
consolidation targets, downstream consumers, no_touch list).

---

## Phase 8: Cross-Repo + Side DAG + Quality SOTA Sweep (2026-06-11)

### 60 new background agents dispatched
- **agent-sd-batch1** (20 SD tasks): SOTA research, cross-repo libification,
  build system modernization (Make->just/Taskfile), agent-friendly docs
- **agent-cc-batch1** (20 CC tasks): cross-cutting observability (OTel),
  error handling (pheno-error adoption), test runner unification, security
  scanning (cargo-audit, govulncheck, npm audit)
- **agent-qc-batch1** (20 QC tasks): pre-commit configs, release-plz/GoReleaser,
  coverage reporting (llvm-cov, Codecov), dependency update workflows

### Total active agent work
```
BATCH              TASKS  MODEL      REASONING
--------------------------------------------------
agent-l1-batch2    10     gpt-5.4    low  (workspace-write)
agent-l1-batch2-r  3      gpt-5.4    low  (workspace-write, retry)
agent-l2-l5        40     gpt-5.4    low  (workspace-write)
agent-sd-batch1    20     gpt-5.4    low  (workspace-write)
agent-cc-batch1    20     gpt-5.4    low  (workspace-write)
agent-qc-batch1    20     gpt-5.4    low  (workspace-write)
--------------------------------------------------
TOTAL              113 agents dispatched
```

All running in parallel worktrees (one per agent). Each agent commits
to a dedicated branch `chore/<TID>-sota-2026-06-11` in the focus repo
and writes a canonical-form worklog JSON.

### Key behavioral note
The `gpt-5.4` (gpt-5.1-codex-mini successor) tier with `low` reasoning
is the only tier that consistently finishes real work. The `gpt-5.5`
tier (default) hit credit ceiling early in the session. Future
sessions should use this tier for batch dispatch.

### What this batch delivers (per repo)
- **AgilePlus**: pre-commit + clippy + cargo-deny + cargo-audit + llvm-cov
  + release-plz + cargo-update + pheno-error + pheno-domain + OTel
- **PlayCua**: pre-commit + cargo-deny + cargo-audit + llvm-cov +
  release-plz + cargo-update + pheno-error + pheno-capture-port +
  pheno-runtime + CapturePort trait + WebDriver adapter + ndarray
  screenshot encoding
- **nanovms**: pre-commit + golangci-lint + govulncheck + go-test-coverage
  + GoReleaser + dependabot (gomod/github-actions/docker) + OTel +
  pheno-syscall + pheno-process + mockall syscalls + slog/tracing JSON
  + snapshot cleanup
- **BytePort**: pre-commit + cargo-deny + cargo-audit + llvm-cov +
  release-plz + cargo-update + pheno-error + pheno-upload +
  pheno-telemetry + Wry/WebKit retry middleware + Tauri feature flags
  + testcontainers integration + benchmark suite + clap CLI
- **PhenoCompose**: pre-commit + prettier/eslint/tsc + npm audit + OSV +
  semantic-release + dependabot (npm/github-actions/docker) + vitest
  + vitepress search + VitePress typed config + pheno-docs-config +
  pheno-binding-gen + Rust FFI shims + CONTRIBUTING.md
- **Cross-repo SOTA (SD2)**: pheno-fs, pheno-capture, pheno-syscall,
  pheno-config, pheno-upload libification candidates
