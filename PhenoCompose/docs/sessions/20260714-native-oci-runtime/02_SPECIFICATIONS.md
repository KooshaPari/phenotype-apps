# Specifications

## Runtime trait (port-runtime)

- `Runtime::spawn(&self, image: &ImageRef) -> Result<ContainerId, RuntimeError>`
  — unchanged.
- `Runtime::stop(&self, id: &ContainerId) -> Result<(), RuntimeError>`
  — unchanged.
- `Runtime::status(&self, id: &ContainerId) -> Result<ContainerStatus, RuntimeError>`
  — unchanged.
- `Runtime::name(&self) -> &str` — unchanged, defaults to
  `"unknown"`.
- `Runtime::probe(&self) -> ProviderInfo` — **NEW**; default
  implementation returns `ProviderInfo::unknown()`. Override to
  advertise real kind / transport / endpoint / capabilities.
- Trait remains object-safe (`Send + Sync`, no generics, no
  associated types) so `Box<dyn Runtime>` keeps working.

## ProviderInfo / ProviderKind / Transport / Capability (port-types)

- `ProviderInfo::unknown()` — generic default (kind `Unknown`,
  transport `Subprocess`, empty endpoint, no capabilities).
- `ProviderInfo::apple_container(version)` — kind
  `AppleContainer`, transport `Subprocess`, capabilities
  `[spawn, stop, status, probe]`.
- `ProviderInfo::wslc(version)` — kind `Wslc`, transport
  `Subprocess`, capabilities `[spawn, stop, status, probe]`.
- `ProviderInfo::socktainer(endpoint, version)` — kind
  `Socktainer`, transport `UnixSocket`, capabilities
  `[spawn, stop, status, probe]`. Reserved for the future
  Apple-container-over-socket adapter.
- `ProviderInfo::docker_socket(endpoint, version)` — kind
  `DockerSocket`, transport `UnixSocket` (Unix) /
  `NamedPipe` (Windows), capabilities
  `[spawn, stop, status, probe]`. Reserved for any future
  generic Docker daemon bridge.
- `ProviderInfo::noop()` — kind `Noop`, transport `Subprocess`,
  capabilities `[spawn, stop, status, probe]`.
- `Capability` namespace pins stable string tags
  (`spawn`, `stop`, `status`, `probe`, `pause`, `exec`,
  `logs`).

## Adapter overrides

- `NoopRuntime::probe()` → `ProviderInfo::noop()`.
- `AppleContainerRuntime::probe()` →
  `ProviderInfo::apple_container(None::<String>)`.
- `WslcRuntime::probe()` →
  `ProviderInfo::wslc(None::<String>)`.

## Conformance invariants

- Every adapter that overrides `probe()` MUST advertise the
  base lifecycle capabilities it implements (SPAWN / STOP /
  STATUS) plus `PROBE`.
- `probe()` MUST be cheap (no subprocess I/O, no socket I/O).
- `probe()` MUST be idempotent — successive calls return
  equivalent metadata.
- The default `probe()` (adapters that haven't been ported)
  MUST remain available and MUST report `ProviderInfo::is_default()`.