# Known Issues

- **Pre-existing test contradiction in `port-types`.**
  `provider_info_tests::unknown_is_default_and_capability_empty`
  asserts `!u.is_subprocess()` for `ProviderInfo::unknown()`,
  but `ProviderInfo::unknown()` is implemented with
  `transport: Transport::Subprocess`. This test was added in
  the prior session's uncommitted work and fails on every
  run regardless of this P0 slice's edits:
  ```
  thread 'provider_info_tests::unknown_is_default_and_capability_empty' panicked at crates/port-types/src/lib.rs:392:9:
  assertion failed: !u.is_subprocess()
  ```
  Resolution options (out of scope for this P0):
  - Change `ProviderInfo::unknown()` to use a new
    `Transport::Unknown` variant and implement `is_subprocess()`
    to return `false` for it.
  - Change the test assertion to `assert!(u.is_subprocess())`.
  - Both. Document the contract that `ProviderInfo::unknown()`
    means "no transport claim, capability-empty, kind Unknown".
- **Pre-existing formatting drift in unrelated files.**
  `cargo fmt --check` reports drift in
  `windows/scm-service/src/scm.rs` and
  `windows/scm-service/src/service_main.rs`, plus a small
  number of long-form method signatures in `crates/port-types`
  that this P0 didn't introduce (e.g. `ProviderInfo::socktainer`
  and `ProviderInfo::docker_socket`). These are pre-existing
  on `feat/PhenoCompose-pillar-docs-landing` and not touched
  by this slice.
- **`Cargo.lock` updated by cargo check / test.**
  Running `cargo check` and `cargo test` pulled in
  `phenocompose-port-runtime` and `phenocompose-port-types`
  into `Cargo.lock` for the first time. This is expected and
  matches the new crate graph; no manual lock edits.
- **Runtime-adapters worktree left dirty.**
  The worktree at
  `/Users/kooshapari/CodeProjects/Phenotype/repos/PhenoCompose/.claude/worktrees/runtime-adapters/`
  carries pre-existing uncommitted formatting differences
  against the cherry-picked upstream commit. Per task
  directive ("without resetting or cleaning"), this worktree
  was not touched and its dirty state is preserved.
- **`probe()` is metadata-only by design.** It does not
  perform a health check or surface the provider's reported
  version. Adapters currently return `version = None`. A
  future opt-in `Runtime::health()` method is the right
  place for that — out of scope here.