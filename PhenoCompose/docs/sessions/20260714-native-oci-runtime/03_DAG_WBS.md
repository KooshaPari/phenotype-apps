# DAG / WBS

1. Inspect repo governance, branch state, and runtime-adapters
   worktree. (Done — see `01_RESEARCH.md`.)
2. Confirm upstream commit `14e3f83` is the AppleContainer +
   wslc cherry-pick candidate and that `4e00d86` is its unmerged
   twin. (Done.)
3. Cherry-pick `14e3f83` onto `feat/PhenoCompose-pillar-docs-landing`
   (resulting in commit `8fe2595`). (Done.)
4. Surface `Capability`, `ProviderInfo`, `ProviderKind`, and
   `Transport` from `phenocompose_port_types` through
   `phenocompose_port_runtime` so adapters can `use`
   them without a direct port-types dependency.
   (Done — `pub use` re-exports in `crates/port-runtime/src/lib.rs:34`.)
5. Override `Runtime::probe()` on `NoopRuntime`,
   `AppleContainerRuntime`, and `WslcRuntime`. (Done.)
6. Add probe-conformance unit tests in each affected crate.
   (Done — see test list in `06_TESTING_STRATEGY.md`.)
7. Run `cargo check`, `cargo fmt --check`, and `cargo test`
   for the four affected crates. (Done.)
8. Document the slice in `docs/sessions/20260714-native-oci-runtime/`.
   (This file.)