# Session Overview

- Goal: move PhenoCompose's Runtime port toward a normalized
  provider-capabilities / probe / lifecycle / conformance surface
  for native-OCI backends (Apple container via Socktainer / Docker
  socket, Windows `wslc.exe`) without forking Podman and without
  rebuilding NanoVMs / BytePort.
- Scope:
  - Cherry-pick the upstream AppleContainer + wslc adapter commit
    (`14e3f83`) onto the current `feat/PhenoCompose-pillar-docs-landing`
    branch.
  - Make `Runtime::probe()` an additive, default-implemented
    method that returns a normalized `ProviderInfo` snapshot
    (kind + transport + endpoint + capability tags).
  - Override `probe()` on `AppleContainerRuntime`, `WslcRuntime`,
    and `NoopRuntime` so downstream selection code can branch on a
    single canonical surface.
  - Preserve the existing `Box<dyn Runtime>` object-safety and the
    existing ad-hoc-subprocess / per-OS-gated adapter bodies.
  - Add probe-conformance unit tests in each affected crate.
- Success criteria:
  - `cargo check -p phenocompose-port-types -p phenocompose-port-runtime -p phenocompose-apple-container-adapter -p phenocompose-wslc-adapter` exits 0.
  - `cargo test` for those four crates is green on every test the
    P0 slice adds or touches. (One pre-existing test in
    `port-types` is left untouched; see `05_KNOWN_ISSUES.md`.)
  - Cherry-pick preserves the upstream commit's substance; the
    diff in this branch only adds `probe()` overrides, tests, and
    crate-doc tweaks.
  - PhenoCompose remains the orchestration layer; NanoVMs / BytePort
    are not touched.