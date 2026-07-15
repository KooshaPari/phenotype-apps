# Research

## Repo / container governance

- `AGENTS.md` and `CLAUDE.md` are present at the repo root.
- Parent governance: `/Users/kooshapari/CodeProjects/Phenotype/repos/CLAUDE.md`.
- ADR table at `docs/adr/` lists ADR-001..ADR-014 with explicit
  language, runtime, networking, security, and observability
  decisions.
- `SPEC.md`, `PLAN.md`, `CONSOLIDATION.md`, `TEST_COVERAGE_MATRIX.md`
  define the contract surface for this P0.

## Pre-existing branch / worktree state

- Current branch: `feat/PhenoCompose-pillar-docs-landing`.
- HEAD: `8fe2595` — the cherry-picked form of upstream
  `14e3f83` (`feat(runtime): AppleContainer + wslc Runtime
  adapters (native-OCI backends) (#79)`).
- Twin unmerged commit: `4e00d86` (same commit, different
  branch tip — left untouched).
- Active worktree with the same branch tip:
  `/Users/kooshapari/CodeProjects/Phenotype/repos/PhenoCompose/.claude/worktrees/runtime-adapters/`.
  Dirty state there is left untouched per task directive.

## Port / type / DI landscape

- `crates/port-types` — value types (`Manifest`,
  `ComposedArtifact`, `PublishTarget`, `PublishReceipt`,
  `ImageRef`, `ContainerId`, `ContainerStatus`, `SecretRef`,
  `Secret`, `PortError`, `ProviderKind`, `Transport`,
  `Capability`, `ProviderInfo`).
- `crates/port-runtime` — `Runtime` trait (`spawn` / `stop` /
  `status` + `name` + new `probe`), `NoopRuntime`, `RuntimeError`.
- `crates/port-di` — DI container wiring Composer / Publisher /
  Runtime / SecretStore together (no native-OCI specific surface
  yet; covered by orchestrator selection on
  `ProviderInfo::kind`).
- `crates/apple-container-adapter` — Apple `container` CLI
  subprocess adapter, gated on `target_os = "macos"`.
- `crates/wslc-adapter` — Windows `wslc.exe` subprocess adapter,
  gated on `target_os = "windows"`.

## Apple container → Socktainer / Docker socket (direction)

- Apple container exposes `/usr/local/bin/container run|stop|inspect|ls`
  (current adapter surface) and ships a Docker-API-compatible
  Socktainer shim reachable over a UNIX domain socket
  (`/var/run/...` style) via `container system socket`.
- The current `AppleContainerRuntime` still uses the CLI;
  `ProviderInfo::apple_container(...)` advertises
  `ProviderKind::AppleContainer` and `Transport::Subprocess` so
  the orchestrator can later swap to a `ProviderInfo::socktainer(...)`
  shape (`Transport::UnixSocket`, endpoint at the socket path)
  without touching the trait.

## wslc.exe surface (direction)

- `wslc.exe` is the Windows WSL native container CLI.
  Current adapter uses `run` / `stop` / `inspect` subcommands
  with subprocess transport (`Transport::Subprocess`).
- The future "Windows native bridge" call-out in the task brief
  can land as either a `wslc.exe` extension or a separate
  named-pipe adapter; both will plug into `Runtime::probe()` via
  `ProviderInfo::wslc(...)` (subprocess) or a new
  `ProviderInfo::wsl_named_pipe(...)` factory.