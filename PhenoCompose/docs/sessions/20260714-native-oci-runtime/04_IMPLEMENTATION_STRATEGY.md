# Implementation Strategy

- **Forward-only.** `probe()` is added to the `Runtime` trait
  with a default implementation that returns
  `ProviderInfo::unknown()`. Pre-probe adapters still satisfy
  the trait without modification — no `pub trait Runtime`
  breakage.
- **Single canonical surface.** The probe contract reuses the
  existing `ProviderInfo` / `ProviderKind` / `Transport` /
  `Capability` types in `phenocompose_port_types` rather than
  introducing a parallel taxonomy. `probe()` is the one
  supported way to learn an adapter's kind / transport /
  capabilities from outside the adapter crate.
- **Re-export through port-runtime.** `pub use` of the
  provider-metadata types from `phenocompose_port_runtime`
  means adapters and downstream services can keep their
  single import:
  ```rust
  use phenocompose_port_runtime::{Runtime, ProviderInfo, ProviderKind};
  ```
  No direct `phenocompose_port_types` dep needed for adapter
  authors.
- **Adapters use factory constructors.** `AppleContainerRuntime`
  and `WslcRuntime` use `ProviderInfo::apple_container(...)`
  and `ProviderInfo::wslc(...)` rather than a generic `new`,
  so the canonical kind + capability set is owned by the port
  crate. Adding a new capability to "every Apple container
  adapter" becomes a single port-types change.
- **probe() is metadata-only.** No subprocess I/O, no socket
  I/O. The adapter returns its kind + transport + capabilities
  from in-memory data. Health-check / version inspection is
  out of scope and should land as a separate explicit
  `Runtime::health()` method (future work).
- **Cherry-pick is the source of truth.** The upstream
  `14e3f83` commit is what the runtime-adapters worktree was
  built on. We recover its substance by cherry-picking onto
  `feat/PhenoCompose-pillar-docs-landing` rather than
  duplicating the adapter bodies from the worktree's diff
  (which only differed by formatting).