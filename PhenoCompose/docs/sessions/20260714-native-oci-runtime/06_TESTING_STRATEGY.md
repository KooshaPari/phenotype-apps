# Testing Strategy

## Cargo commands run

```sh
cargo check  -p phenocompose-port-types \
             -p phenocompose-port-runtime \
             -p phenocompose-apple-container-adapter \
             -p phenocompose-wslc-adapter

cargo test   -p phenocompose-port-types \
             -p phenocompose-port-runtime \
             -p phenocompose-apple-container-adapter \
             -p phenocompose-wslc-adapter
```

`cargo fmt --check` was also run on the four affected files via
`rustfmt --check`; no formatting diffs are introduced by this
slice (pre-existing drift in unrelated files is documented in
`05_KNOWN_ISSUES.md`).

## Tests added by this slice

### `crates/port-runtime/src/lib.rs`

- `noop_runtime_probe_advertises_full_lifecycle` —
  `NoopRuntime::probe()` reports `ProviderKind::Noop`,
  `Transport::Subprocess`, and the SPAWN / STOP / STATUS /
  PROBE capability tags.
- `default_probe_is_cheap_and_unknown` — a synthetic
  pre-probe `Runtime` impl returns `ProviderInfo::unknown()`
  with no capabilities. Forward-only guarantee.
- `noop_runtime_advertised_capabilities_match_implemented_methods`
  — every advertised capability tag maps to a real method
  call site on `NoopRuntime` (compile-time conformance).

### `crates/apple-container-adapter/src/lib.rs`

- `apple_container_runtime_probe_advertises_capabilities` —
  `AppleContainerRuntime::probe()` reports
  `ProviderKind::AppleContainer` and the SPAWN / STOP /
  STATUS / PROBE tags.
- `apple_container_runtime_probe_is_idempotent` — two
  successive `probe()` calls return equivalent metadata.
- `apple_container_runtime_capabilities_match_methods` —
  every advertised capability tag maps to a real method
  call site on the adapter.

### `crates/wslc-adapter/src/lib.rs`

- `wslc_runtime_probe_advertises_capabilities` —
  `WslcRuntime::probe()` reports `ProviderKind::Wslc` and
  the SPAWN / STOP / STATUS / PROBE tags.
- `wslc_runtime_probe_is_idempotent` — two successive
  `probe()` calls return equivalent metadata.
- `wslc_runtime_capabilities_match_methods` — every
  advertised capability tag maps to a real method call
  site on the adapter.

## Test counts after this slice

- `phenocompose-port-runtime`: **11 passed** (8 pre-existing
  lifecycle tests + 3 new probe-conformance tests).
- `phenocompose-apple-container-adapter`: **6 passed**
  (3 pre-existing + 3 new probe-conformance tests).
- `phenocompose-wslc-adapter`: **6 passed** (3 pre-existing
  + 3 new probe-conformance tests).
- `phenocompose-port-types`: 47 passed, **1 pre-existing
  failure** (`unknown_is_default_and_capability_empty` —
  see `05_KNOWN_ISSUES.md`).

## Build / runtime safety

- No containers were built.
- No runtimes were installed or deployed.
- `NanoVMs` / `BytePort` repos were not touched.
- The runtime-adapters worktree's dirty state was preserved.