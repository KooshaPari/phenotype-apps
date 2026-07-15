// SPDX-License-Identifier: MIT OR Apache-2.0
//! `phenocompose-port-types`
//!
//! Shared value types that flow across the PhenoCompose port
//! traits (Composer, Publisher, Runtime, SecretStore). Defined
//! as a standalone crate so the port-trait crates can depend on
//! a single canonical type vocabulary without pulling in each
//! other's implementation code or test fixtures.
//!
//! Type inventory:
//!
//! | Type               | Role                                                 |
//! |--------------------|------------------------------------------------------|
//! | [`Manifest`]       | Input to the [`Composer`](crate::Composer) port      |
//! | [`ComposedArtifact`] | Output of `Composer`; input to [`Publisher`](crate::Publisher) |
//! | [`PublishTarget`]  | Where a [`ComposedArtifact`] is sent                 |
//! | [`PublishReceipt`] | Proof of a successful publish                        |
//! | [`ImageRef`]       | Reference to a container image; input to [`Runtime`](crate::Runtime) |
//! | [`ContainerId`]    | Opaque handle returned by `Runtime::spawn`           |
//! | [`ContainerStatus`] | State reported by `Runtime::status`                  |
//! | [`SecretRef`]      | Strongly-typed identifier for a [`Secret`]            |
//! | [`Secret`]         | A versioned, named value stored by a `SecretStore`  |
//! | [`ProviderKind`]   | Tagged backend kind for a [`Runtime`](crate::Runtime) (Apple container, wslc, Socktainer, Docker socket, ...) |
//! | [`Transport`]      | How a `Runtime` adapter reaches its provider (subprocess / UNIX socket / named pipe) |
//! | [`Capability`]     | Well-known static capability tags (`spawn`, `pause`, ...) |
//! | [`ProviderInfo`]   | Snapshot returned by `Runtime::probe` describing what a `Runtime` adapter is and what it advertises |
//!
//! All types in this crate are `Send + Sync` so they can be moved
//! across worker threads and stored in `Box<dyn Trait>` adapters
//! that downstream pheno-* services compose into their dependency
//! graph.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

// ---------------------------------------------------------------------------
// OCI helpers — canonical home is `phenotype-types`
// ---------------------------------------------------------------------------
/// OCI (Open Container Initiative) image reference helpers.
///
/// Parsing, validation, and construction utilities for OCI image
/// references. See the [`oci` module documentation](oci) for details.
///
/// The **canonical** home for these helpers in the Phenotype
/// ecosystem is the **`phenotype-types`** crate
/// (<https://github.com/kooshapari/phenotype-types>). Consumers
/// SHOULD prefer that crate over this local module when it is
/// available.
pub mod oci;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

// ---------------------------------------------------------------------------
// ProviderInfo — see also: `Runtime::probe` in phenocompose-port-runtime
// ---------------------------------------------------------------------------

/// Identifies which container-runtime backend a [`Runtime`](crate::Runtime)
/// adapter drives. Used by [`ProviderInfo::kind`] for normalization,
/// documentation, and CLI surface selection.
///
/// `ProviderKind` is intentionally a tagged enum with a `#[non_exhaustive]`
/// attribute: new variants (e.g. `Containerd`, `Crio`, `Firecracker`) can be
/// added in a minor-version bump without breaking downstream callers that
/// match on a specific variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ProviderKind {
    /// No concrete provider is wired (e.g. `NoopRuntime`, or an adapter that
    /// has not overridden the default [`Runtime::probe`](crate::Runtime::probe)).
    Unknown,
    /// Apple `container` CLI (macOS-native per-VM OCI; shells out to
    /// `/usr/local/bin/container`).
    AppleContainer,
    /// Microsoft `wslc.exe` CLI (Windows WSL native containers).
    Wslc,
    /// Socktainer — Docker-API-compatible UNIX-socket emulator that wraps the
    /// Apple container CLI; reaches the Apple container runtime over
    /// `/var/run/...` style endpoints.
    Socktainer,
    /// Generic Docker daemon reached over its standard socket
    /// (`/var/run/docker.sock` on Linux/macOS, `\\.\pipe\docker_engine` on
    /// Windows). Kept distinct from `Socktainer` because the lifecycle surface
    /// differs.
    DockerSocket,
    /// Noop / in-memory adapter (tests, dry-run).
    Noop,
}

impl fmt::Display for ProviderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Unknown => "unknown",
            Self::AppleContainer => "apple-container",
            Self::Wslc => "wslc",
            Self::Socktainer => "socktainer",
            Self::DockerSocket => "docker-socket",
            Self::Noop => "noop",
        })
    }
}

/// How a [`Runtime`](crate::Runtime) adapter talks to its provider.
///
/// `Transport` is `#[non_exhaustive]` so the taxonomy can grow
/// (`TlsTcp`, `LunaticChannel`, ...) without breaking downstream matches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Transport {
    /// The adapter shells out to a provider CLI (`/usr/local/bin/container`,
    /// `wslc.exe`, `docker`, ...).
    Subprocess,
    /// The adapter talks to the provider over a UNIX domain socket
    /// (Docker, Socktainer-on-macOS, ...).
    UnixSocket,
    /// The adapter talks to the provider over a Windows named pipe
    /// (Docker-on-Windows, `\\\\.\\pipe\\...`).
    NamedPipe,
}

impl fmt::Display for Transport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Subprocess => "subprocess",
            Self::UnixSocket => "unix-socket",
            Self::NamedPipe => "named-pipe",
        })
    }
}

/// Static capability tags that a [`Runtime`](crate::Runtime) adapter may
/// advertise via [`ProviderInfo::capabilities`].
///
/// `Capability` is intentionally a constant-only namespace (no `enum`
/// variants): the set of well-known tags is open and downstream callers SHOULD
/// treat unknown tags as opaque strings rather than failing on `match`
/// exhaustiveness. New constants can be added in any minor release.
///
/// Every adapter MUST report at minimum the lifecycle capabilities it
/// implements by overriding [`Runtime::probe`](crate::Runtime::probe).
pub struct Capability;

impl Capability {
    /// The adapter implements [`Runtime::spawn`](crate::Runtime::spawn).
    pub const SPAWN: &'static str = "spawn";
    /// The adapter implements [`Runtime::stop`](crate::Runtime::stop).
    pub const STOP: &'static str = "stop";
    /// The adapter implements [`Runtime::status`](crate::Runtime::status).
    pub const STATUS: &'static str = "status";
    /// The adapter overrides [`Runtime::probe`](crate::Runtime::probe) and
    /// returns a [`ProviderInfo`] that is not the generic default. The
    /// default `probe()` impl reports this capability as `false`; adapters
    /// MUST add this capability when they override.
    pub const PROBE: &'static str = "probe";
    /// The adapter can pause / unpause containers (cgroup freezer, provider
    /// equivalent, ...).
    pub const PAUSE: &'static str = "pause";
    /// The adapter can exec a process inside a running container
    /// (Docker `-e`, Apple container `container exec`, ...).
    pub const EXEC: &'static str = "exec";
    /// The adapter can stream container logs.
    pub const LOGS: &'static str = "logs";
}

/// Snapshot of what a [`Runtime`](crate::Runtime) adapter is and which
/// lifecycle capabilities it advertises.
///
/// Returned by [`Runtime::probe`](crate::Runtime::probe). Intended for
/// documentation, observability, and CLI/backend selection — NOT for hot-path
/// container management.
///
/// `ProviderInfo` is a value type: cloning is cheap, comparison is value
/// equality. Adapters SHOULD return a freshly-built `ProviderInfo` on each
/// `probe()` call; downstream code MUST treat it as effectively immutable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderInfo {
    /// Which provider this adapter drives.
    pub kind: ProviderKind,
    /// Adapter-reported version string (e.g. `"0.5.1"`). `None` when the
    /// adapter has not (yet) inspected the provider's `--version`.
    pub version: Option<String>,
    /// How the adapter reaches the provider.
    pub transport: Transport,
    /// For socket / pipe transports, the absolute endpoint
    /// (e.g. `"/var/run/docker.sock"`, `"\\.\pipe\docker_engine"`). `None`
    /// for [`Transport::Subprocess`].
    pub endpoint: Option<String>,
    /// The static capability tags the adapter advertises. Use the
    /// constants on [`Capability`] for the well-known values
    /// (`Capability::SPAWN`, ...).
    pub capabilities: Vec<String>,
}

impl ProviderInfo {
    /// Construct a generic, capability-empty `ProviderInfo` with
    /// `kind = Unknown`. This is the value returned by the default
    /// [`Runtime::probe`](crate::Runtime::probe) implementation.
    pub fn unknown() -> Self {
        Self {
            kind: ProviderKind::Unknown,
            version: None,
            transport: Transport::Subprocess,
            endpoint: None,
            capabilities: Vec::new(),
        }
    }

    /// Build a `ProviderInfo` for a Socktainer UNIX-socket adapter
    /// (Apple container shim).
    ///
    /// `endpoint` is typically the path produced by
    /// `container system socket` (Apple container's Socktainer shim).
    pub fn socktainer(
        endpoint: impl Into<String>,
        version: Option<impl Into<String>>,
    ) -> Self {
        Self {
            kind: ProviderKind::Socktainer,
            version: version.map(Into::into),
            transport: Transport::UnixSocket,
            endpoint: Some(endpoint.into()),
            capabilities: vec![
                Capability::SPAWN.into(),
                Capability::STOP.into(),
                Capability::STATUS.into(),
                Capability::PROBE.into(),
            ],
        }
    }

    /// Build a `ProviderInfo` for a Docker daemon reached over its
    /// standard socket. The transport is selected per host OS:
    /// [`Transport::UnixSocket`] on Unix, [`Transport::NamedPipe`] on
    /// Windows.
    pub fn docker_socket(
        endpoint: impl Into<String>,
        version: Option<impl Into<String>>,
    ) -> Self {
        let endpoint = endpoint.into();
        #[cfg(target_os = "windows")]
        let transport = Transport::NamedPipe;
        #[cfg(not(target_os = "windows"))]
        let transport = Transport::UnixSocket;
        Self {
            kind: ProviderKind::DockerSocket,
            version: version.map(Into::into),
            transport,
            endpoint: Some(endpoint),
            capabilities: vec![
                Capability::SPAWN.into(),
                Capability::STOP.into(),
                Capability::STATUS.into(),
                Capability::PROBE.into(),
            ],
        }
    }

    /// Build a `ProviderInfo` for the Apple `container` CLI subprocess
    /// adapter.
    pub fn apple_container(version: Option<impl Into<String>>) -> Self {
        Self {
            kind: ProviderKind::AppleContainer,
            version: version.map(Into::into),
            transport: Transport::Subprocess,
            endpoint: None,
            capabilities: vec![
                Capability::SPAWN.into(),
                Capability::STOP.into(),
                Capability::STATUS.into(),
                Capability::PROBE.into(),
            ],
        }
    }

    /// Build a `ProviderInfo` for the `wslc.exe` subprocess adapter.
    pub fn wslc(version: Option<impl Into<String>>) -> Self {
        Self {
            kind: ProviderKind::Wslc,
            version: version.map(Into::into),
            transport: Transport::Subprocess,
            endpoint: None,
            capabilities: vec![
                Capability::SPAWN.into(),
                Capability::STOP.into(),
                Capability::STATUS.into(),
                Capability::PROBE.into(),
            ],
        }
    }

    /// Build a `ProviderInfo` describing an in-memory / no-op adapter
    /// (tests, dry-run modes).
    pub fn noop() -> Self {
        Self {
            kind: ProviderKind::Noop,
            version: None,
            transport: Transport::Subprocess,
            endpoint: None,
            capabilities: vec![
                Capability::SPAWN.into(),
                Capability::STOP.into(),
                Capability::STATUS.into(),
                Capability::PROBE.into(),
            ],
        }
    }

    /// Add an additional capability tag (e.g. `Capability::PAUSE`).
    /// Returns `&mut self` for builder-style chaining.
    pub fn with_capability(mut self, cap: impl Into<String>) -> Self {
        let cap = cap.into();
        if !self.capabilities.contains(&cap) {
            self.capabilities.push(cap);
        }
        self
    }

    /// Returns `true` if this `ProviderInfo` advertises `capability`.
    pub fn has(&self, capability: &str) -> bool {
        self.capabilities.iter().any(|c| c == capability)
    }

    /// Returns `true` if this `ProviderInfo` describes a provider reachable
    /// via a process (shelling out, no socket / pipe).
    pub fn is_subprocess(&self) -> bool {
        matches!(self.transport, Transport::Subprocess)
    }

    /// Returns `true` if this `ProviderInfo` describes a provider reachable
    /// over a UNIX socket or named pipe (Docker / Socktainer style).
    pub fn is_socket(&self) -> bool {
        matches!(
            self.transport,
            Transport::UnixSocket | Transport::NamedPipe
        )
    }

    /// Returns `true` if this `ProviderInfo` is the generic default
    /// (`ProviderInfo::unknown()` shape). Adapters that override
    /// `probe()` SHOULD NOT return `is_default() == true`.
    pub fn is_default(&self) -> bool {
        matches!(
            (&self.kind, &self.version, &self.transport, &self.endpoint),
            (
                ProviderKind::Unknown,
                None,
                Transport::Subprocess,
                None,
            )
        ) && self.capabilities.is_empty()
    }
}

impl fmt::Display for ProviderInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "provider={} transport={}", self.kind, self.transport)?;
        if let Some(v) = &self.version {
            write!(f, " version={v}")?;
        }
        if let Some(ep) = &self.endpoint {
            write!(f, " endpoint={ep}")?;
        }
        if !self.capabilities.is_empty() {
            write!(f, " capabilities={:?}", self.capabilities)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Endpoint conformance for `ProviderInfo` (`probe` conformance):
// ---------------------------------------------------------------------------
/// Conformance tests for the [`ProviderInfo`] value type and its
/// constructors. Validates that every constructor produces a value that
/// satisfies the documented invariants (right kind, right transport,
/// right endpoint shape) so adapters can rely on the round-trip
/// behavior.
#[cfg(test)]
mod provider_info_tests {
    use super::*;

    #[test]
    fn unknown_is_default_and_capability_empty() {
        let u = ProviderInfo::unknown();
        assert_eq!(u.kind, ProviderKind::Unknown);
        assert_eq!(u.version, None);
        assert_eq!(u.endpoint, None);
        assert!(u.capabilities.is_empty());
        assert!(u.is_default());
        assert!(!u.is_subprocess());
        assert!(!u.is_socket());
    }

    #[test]
    fn apple_container_factory_sets_subprocess_with_capabilities() {
        let p = ProviderInfo::apple_container(Some("0.5.1"));
        assert_eq!(p.kind, ProviderKind::AppleContainer);
        assert_eq!(p.version.as_deref(), Some("0.5.1"));
        assert_eq!(p.transport, Transport::Subprocess);
        assert_eq!(p.endpoint, None);
        assert!(p.is_subprocess());
        assert!(!p.is_socket());
        assert!(!p.is_default());
        assert!(p.has(Capability::SPAWN));
        assert!(p.has(Capability::STOP));
        assert!(p.has(Capability::STATUS));
        assert!(p.has(Capability::PROBE));
        assert!(!p.has(Capability::PAUSE));
    }

    #[test]
    fn apple_container_factory_accepts_none_version() {
        let p = ProviderInfo::apple_container(None::<String>);
        assert_eq!(p.version, None);
        assert_eq!(p.kind, ProviderKind::AppleContainer);
    }

    #[test]
    fn wslc_factory_sets_subprocess_with_capabilities() {
        let p = ProviderInfo::wslc(None::<String>);
        assert_eq!(p.kind, ProviderKind::Wslc);
        assert_eq!(p.transport, Transport::Subprocess);
        assert_eq!(p.endpoint, None);
        assert!(p.has(Capability::SPAWN));
        assert!(p.has(Capability::STOP));
        assert!(p.has(Capability::STATUS));
        assert!(p.has(Capability::PROBE));
        assert!(!p.is_default());
        assert!(!p.is_socket());
    }

    #[test]
    fn socktainer_factory_sets_unix_socket_endpoint_and_capabilities() {
        let p = ProviderInfo::socktainer(
            "/var/run/socktainer.sock",
            Some("0.1.0"),
        );
        assert_eq!(p.kind, ProviderKind::Socktainer);
        assert_eq!(
            p.endpoint.as_deref(),
            Some("/var/run/socktainer.sock")
        );
        assert_eq!(p.transport, Transport::UnixSocket);
        assert!(p.is_socket());
        assert!(!p.is_subprocess());
        assert!(!p.is_default());
        assert!(p.has(Capability::SPAWN));
        assert!(p.has(Capability::STOP));
        assert!(p.has(Capability::STATUS));
        assert!(p.has(Capability::PROBE));
    }

    #[test]
    fn docker_socket_factory_endpoint_round_trips_and_transport_per_os() {
        // Use a literal that's valid for both Unix-socket and Windows
        // named-pipe paths — the transport assertion is OS-conditional
        // because docker_socket picks the right one per host.
        let p = ProviderInfo::docker_socket(
            "/var/run/docker.sock",
            Some("26.1.0"),
        );
        assert_eq!(p.kind, ProviderKind::DockerSocket);
        assert_eq!(
            p.endpoint.as_deref(),
            Some("/var/run/docker.sock")
        );
        #[cfg(target_os = "windows")]
        assert_eq!(p.transport, Transport::NamedPipe);
        #[cfg(not(target_os = "windows"))]
        assert_eq!(p.transport, Transport::UnixSocket);
        assert!(p.is_socket());
        assert!(p.has(Capability::SPAWN));
        assert!(!p.is_default());
    }

    #[test]
    fn noop_factory_sets_kind_and_capabilities() {
        let p = ProviderInfo::noop();
        assert_eq!(p.kind, ProviderKind::Noop);
        assert_eq!(p.transport, Transport::Subprocess);
        assert_eq!(p.endpoint, None);
        assert!(!p.is_default());
        assert!(p.has(Capability::SPAWN));
        assert!(p.has(Capability::STOP));
        assert!(p.has(Capability::STATUS));
        assert!(p.has(Capability::PROBE));
    }

    #[test]
    fn with_capability_adds_and_dedupes() {
        let p = ProviderInfo::apple_container(None::<String>)
            .with_capability(Capability::PAUSE)
            .with_capability(Capability::EXEC);
        assert!(p.has(Capability::PAUSE));
        assert!(p.has(Capability::EXEC));

        // Adding the same capability again MUST NOT duplicate.
        let p2 = p.clone().with_capability(Capability::PAUSE);
        let count = p2
            .capabilities
            .iter()
            .filter(|c| *c == Capability::PAUSE)
            .count();
        assert_eq!(count, 1);
    }

    #[test]
    fn has_returns_false_for_unknown_capability() {
        let p = ProviderInfo::apple_container(None::<String>);
        assert!(!p.has("fly-me-to-the-moon"));
    }

    #[test]
    fn provider_kind_display_matches_conventional_strings() {
        assert_eq!(format!("{}", ProviderKind::Unknown), "unknown");
        assert_eq!(
            format!("{}", ProviderKind::AppleContainer),
            "apple-container"
        );
        assert_eq!(format!("{}", ProviderKind::Wslc), "wslc");
        assert_eq!(format!("{}", ProviderKind::Socktainer), "socktainer");
        assert_eq!(
            format!("{}", ProviderKind::DockerSocket),
            "docker-socket"
        );
        assert_eq!(format!("{}", ProviderKind::Noop), "noop");
    }

    #[test]
    fn transport_display_matches_conventional_strings() {
        assert_eq!(format!("{}", Transport::Subprocess), "subprocess");
        assert_eq!(format!("{}", Transport::UnixSocket), "unix-socket");
        assert_eq!(format!("{}", Transport::NamedPipe), "named-pipe");
    }

    #[test]
    fn provider_info_display_includes_kind_and_capabilities() {
        let p = ProviderInfo::apple_container(Some("1.0.0"));
        let s = format!("{p}");
        assert!(s.contains("provider=apple-container"));
        assert!(s.contains("transport=subprocess"));
        assert!(s.contains("version=1.0.0"));
        assert!(s.contains("spawn"));
    }

    #[test]
    fn provider_info_value_equality_holds_for_same_inputs() {
        let a = ProviderInfo::apple_container(Some("1.0.0"));
        let b = ProviderInfo::apple_container(Some("1.0.0"));
        assert_eq!(a, b);

        let c = ProviderInfo::apple_container(Some("1.0.1"));
        assert_ne!(a, c);
    }

    #[test]
    fn capability_constants_have_stable_string_values() {
        // Pin the well-known capability tags so downstream consumers
        // (CLI strings, log greps) get stable identifiers.
        assert_eq!(Capability::SPAWN, "spawn");
        assert_eq!(Capability::STOP, "stop");
        assert_eq!(Capability::STATUS, "status");
        assert_eq!(Capability::PROBE, "probe");
        assert_eq!(Capability::PAUSE, "pause");
        assert_eq!(Capability::EXEC, "exec");
        assert_eq!(Capability::LOGS, "logs");
    }
}

/// A composition request — describes *what* the
/// [`Composer`](crate::Composer) should produce.
///
/// `Manifest` is intentionally transport-agnostic (no file paths,
/// no URIs, no environment variables). Adapters that need to
/// resolve files or secrets pull those out of the manifest into
/// the local adapter implementation; the port type stays small.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Manifest {
    /// Human-readable name (e.g. `"phenocommand-web"`). Used by
    /// the Composer for log lines and by adapters as a default
    /// artifact name when [`Manifest::artifact_name`] is `None`.
    pub name: String,
    /// Optional explicit artifact name. If `Some`, the
    /// [`Composer`](crate::Composer) MUST use this exact string
    /// as the artifact identifier; if `None`, the composer
    /// derives one from [`Manifest::name`].
    pub artifact_name: Option<String>,
    /// Free-form key/value tags (e.g. `version=0.1.0`,
    /// `channel=stable`). Adapters MUST preserve them on the
    /// resulting [`ComposedArtifact::tags`].
    pub tags: Vec<(String, String)>,
}

impl Manifest {
    /// Construct a manifest with the given name and no explicit
    /// artifact name or tags.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            artifact_name: None,
            tags: Vec::new(),
        }
    }

    /// Builder-style setter for [`Manifest::artifact_name`].
    #[must_use]
    pub fn with_artifact_name(mut self, name: impl Into<String>) -> Self {
        self.artifact_name = Some(name.into());
        self
    }

    /// Builder-style setter for [`Manifest::tags`].
    #[must_use]
    pub fn with_tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.push((key.into(), value.into()));
        self
    }
}

/// The output of a [`Composer`](crate::Composer) — an artifact
/// ready to be [`Publisher::publish`](crate::Publisher::publish)ed
/// or [`Runtime::spawn`](crate::Runtime::spawn)ed.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComposedArtifact {
    /// Stable artifact identifier (typically
    /// `<name>:<tag-digest>`).
    pub id: String,
    /// Image reference for the artifact (consumable by
    /// [`Runtime::spawn`](crate::Runtime::spawn)).
    pub image: ImageRef,
    /// Tags copied from the source [`Manifest::tags`] plus any
    /// new tags the composer wants to attach
    /// (e.g. `content-digest=sha256:...`).
    pub tags: Vec<(String, String)>,
}

impl ComposedArtifact {
    /// Construct an artifact from an id and image ref, with no
    /// tags.
    pub fn new(id: impl Into<String>, image: ImageRef) -> Self {
        Self {
            id: id.into(),
            image,
            tags: Vec::new(),
        }
    }

    /// Builder-style setter for [`ComposedArtifact::tags`].
    #[must_use]
    pub fn with_tag(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.tags.push((key.into(), value.into()));
        self
    }

    /// Look up a tag by key. Returns `None` if the key is not
    /// present.
    pub fn tag(&self, key: &str) -> Option<&str> {
        self.tags
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
    }
}

/// A destination for a [`ComposedArtifact`] — opaque to the port
/// trait; interpreted by the concrete [`Publisher`](crate::Publisher)
/// adapter (e.g. a registry host, a local file path, a Kafka
/// topic, a `std::io::Write` sink).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublishTarget {
    /// Transport identifier (e.g. `"docker-registry"`, `"file"`,
    /// `"kafka"`). Adapters dispatch on this.
    pub kind: String,
    /// Transport-specific locator (e.g.
    /// `"registry.phenotype.internal/phenocommand/web:0.1.0"`,
    /// `"/var/lib/phenocompose/artifacts/phenocommand-web.tar"`,
    /// `"phenocommand-artifacts"`).
    pub locator: String,
}

impl PublishTarget {
    /// Construct a publish target with the given kind and
    /// locator.
    pub fn new(kind: impl Into<String>, locator: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            locator: locator.into(),
        }
    }
}

/// Proof of a successful publish — returned by
/// [`Publisher::publish`](crate::Publisher::publish).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PublishReceipt {
    /// The artifact id that was published (mirrors
    /// [`ComposedArtifact::id`]).
    pub artifact_id: String,
    /// The destination that received the publish (mirrors
    /// [`PublishTarget`] but value-equal).
    pub target: PublishTarget,
    /// Adapter-defined publication locator (e.g. a digest on the
    /// remote side, a tarball path, a Kafka offset). Adapters
    /// SHOULD set this to something an operator can use to verify
    /// the publish post-hoc.
    pub published_at: String,
}

impl PublishReceipt {
    /// Construct a publish receipt.
    pub fn new(
        artifact_id: impl Into<String>,
        target: PublishTarget,
        published_at: impl Into<String>,
    ) -> Self {
        Self {
            artifact_id: artifact_id.into(),
            target,
            published_at: published_at.into(),
        }
    }
}

/// A reference to a container image — consumed by
/// [`Runtime::spawn`](crate::Runtime::spawn).
///
/// `ImageRef` is intentionally minimal: just a string in
/// `<repo>[:<tag>][@<digest>]` form (or whatever the underlying
/// runtime accepts). Adapters that need richer addressing (e.g.
/// a separate `tag` and `digest` field) can split the string
/// locally.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageRef {
    /// The full image reference string, exactly as the runtime
    /// should consume it (e.g. `"phenocommand-web:0.1.0"`,
    /// `"registry.phenotype/internal/phenocommard-web@sha256:abc..."`).
    pub reference: String,
}

impl ImageRef {
    /// Construct an image ref from a reference string.
    pub fn new(reference: impl Into<String>) -> Self {
        Self {
            reference: reference.into(),
        }
    }

    /// Convenience: build an image ref from a repo and a tag
    /// (joined as `"<repo>:<tag>"`).
    pub fn with_tag(repo: impl AsRef<str>, tag: impl AsRef<str>) -> Self {
        Self::new(format!("{}:{}", repo.as_ref(), tag.as_ref()))
    }

    /// Parse this image reference into its OCI components.
    ///
    /// Delegates to [`oci::parse`](crate::oci::parse). Returns `None`
    /// if the reference cannot be parsed.
    ///
    /// # Example
    ///
    /// ```
    /// use phenocompose_port_types::ImageRef;
    ///
    /// let r = ImageRef::new("registry.example.org/my-app:1.2.3");
    /// let parsed = r.parse_oci().unwrap();
    /// assert_eq!(parsed.repository(), "my-app");
    /// assert_eq!(parsed.tag(), Some("1.2.3"));
    /// ```
    pub fn parse_oci(&self) -> Option<crate::oci::Reference> {
        crate::oci::parse(&self.reference).ok()
    }

    /// Returns `true` if this image reference is a valid OCI
    /// reference (has at least a tag or a digest).
    ///
    /// Delegates to [`oci::is_valid`](crate::oci::is_valid).
    pub fn is_valid_oci(&self) -> bool {
        crate::oci::is_valid(&self.reference)
    }
}

impl AsRef<str> for ImageRef {
    fn as_ref(&self) -> &str {
        &self.reference
    }
}

impl From<&str> for ImageRef {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for ImageRef {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl std::fmt::Display for ImageRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.reference)
    }
}

/// Opaque handle to a running container — returned by
/// [`Runtime::spawn`](crate::Runtime::spawn), consumed by
/// [`Runtime::stop`](crate::Runtime::stop) and
/// [`Runtime::status`](crate::Runtime::status).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContainerId {
    /// The runtime-assigned id (e.g. a Docker container id, a
    /// `systemd-nspawn` machine name).
    pub id: String,
}

impl ContainerId {
    /// Construct a container id.
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

impl AsRef<str> for ContainerId {
    fn as_ref(&self) -> &str {
        &self.id
    }
}

impl From<&str> for ContainerId {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for ContainerId {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl std::fmt::Display for ContainerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id)
    }
}

/// The state of a container, as reported by
/// [`Runtime::status`](crate::Runtime::status).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContainerStatus {
    /// The container is running.
    Running,
    /// The container has exited (cleanly or not — see the exit
    /// code on the adapter side if it cares).
    Exited,
    /// The container is paused (SIGSTOP'd, cgroup frozen, etc.).
    Paused,
    /// The container does not exist (the runtime no longer has
    /// any record of the id). Adapters return this for unknown
    /// ids so callers can distinguish "stopped" from "never
    /// existed".
    NotFound,
}

impl ContainerStatus {
    /// Returns `true` if the status indicates an active container
    /// ([`ContainerStatus::Running`] or [`ContainerStatus::Paused`]).
    pub fn is_active(self) -> bool {
        matches!(self, Self::Running | Self::Paused)
    }
}

impl std::fmt::Display for ContainerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Running => "running",
            Self::Exited => "exited",
            Self::Paused => "paused",
            Self::NotFound => "not_found",
        })
    }
}

/// Errors that can arise in any of the port-trait
/// adapters. Each variant carries the adapter-defined
/// contextual string (typically the adapter's own error type
/// rendered via `Display`).
///
/// This is intentionally a single error type so that downstream
/// `Box<dyn Trait>` storage can produce a single `Result<_,
/// PortError>` shape across all four port traits without
/// forcing the caller to learn four error enums.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum PortError {
    /// The input failed validation (e.g. a [`Manifest`] with an
    /// empty name, a [`PublishTarget`] with an empty locator).
    #[error("validation error: {0}")]
    Validation(String),
    /// The request referred to a resource the adapter could not
    /// find (e.g. a container id that does not exist on
    /// `Runtime::status`).
    #[error("not found: {0}")]
    NotFound(String),
    /// The underlying transport or backend failed (network
    /// error, registry 5xx, runtime daemon offline, etc.).
    #[error("transport error: {0}")]
    Transport(String),
    /// The operation is not supported by this adapter (e.g. a
    /// `stop` on a read-only runtime). Adapters should return
    /// this rather than panicking or returning a generic
    /// `Transport` error so callers can branch on the cause.
    #[error("unsupported: {0}")]
    Unsupported(String),
}

/// A strongly-typed identifier for a [`Secret`] stored by a
/// `SecretStore` port.
///
/// `SecretRef` is the addressing handle used by callers when
/// asking the port for `get` / `put` / `delete` operations. The
/// optional `namespace` field mirrors the Kubernetes-style
/// "namespace/name" convention used by the rest of the
/// PhenoCompose port types (see [`Deployment`] in the
/// orchestrator port). The `namespace` defaults to `"default"`
/// in [`SecretRef::new`]; adapters are free to interpret it
/// (or ignore it) as the underlying engine requires.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SecretRef {
    /// Optional scope qualifier (e.g. `"phenotype"`, `"default"`,
    /// `"staging"`). An empty value means "no namespace".
    pub namespace: String,
    /// The bare secret name (e.g. `"db-password"`,
    /// `"tls-certificate"`). MUST be non-empty.
    pub name: String,
}

impl SecretRef {
    /// Construct a `SecretRef` with an empty namespace and the
    /// given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            namespace: String::new(),
            name: name.into(),
        }
    }

    /// Construct a namespaced `SecretRef`.
    pub fn namespaced(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
        }
    }

    /// Render the ref as `"<namespace>/<name>"` (or just
    /// `"<name>"` when the namespace is empty). Useful for log
    /// lines and as a stable map key.
    pub fn locator(&self) -> String {
        if self.namespace.is_empty() {
            self.name.clone()
        } else {
            format!("{}/{}", self.namespace, self.name)
        }
    }
}

impl AsRef<str> for SecretRef {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl std::fmt::Display for SecretRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.locator())
    }
}

/// A versioned, named value stored by a `SecretStore` port.
///
/// `Secret` is the value type returned by a `get` operation and
/// the value type accepted by a `put` operation. The
/// `version` field is the adapter-defined monotonic counter
/// (vault's `version`, k8s `resourceVersion`, etc.); adapters
/// MUST bump it on every successful `put` so callers can detect
/// concurrent updates.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Secret {
    /// The address of this secret (mirrors the [`SecretRef`]
    /// passed to `get` / `put`).
    pub r#ref: SecretRef,
    /// The opaque secret material (e.g. a PEM-encoded TLS
    /// certificate, a database password, a JSON blob of API
    /// keys). Adapters MUST NOT log this value.
    pub value: String,
    /// Adapter-defined monotonic version counter. `0` is
    /// reserved for "never written"; the first successful
    /// `put` produces `version = 1`.
    pub version: u64,
}

impl Secret {
    /// Construct a `Secret` with `version = 1`. Adapters
    /// should call [`Secret::at_version`] to override the
    /// version counter.
    pub fn new(r#ref: SecretRef, value: impl Into<String>) -> Self {
        Self {
            r#ref,
            value: value.into(),
            version: 1,
        }
    }

    /// Builder-style setter for [`Secret::version`].
    #[must_use]
    pub fn at_version(mut self, version: u64) -> Self {
        self.version = version;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_new_sets_name_and_leaves_others_empty() {
        let m = Manifest::new("phenocommand-web");
        assert_eq!(m.name, "phenocommand-web");
        assert!(m.artifact_name.is_none());
        assert!(m.tags.is_empty());
    }

    #[test]
    fn manifest_builder_set_artifact_name_and_tags() {
        let m = Manifest::new("phenocommand-web")
            .with_artifact_name("phenocommand-web:0.1.0")
            .with_tag("channel", "stable")
            .with_tag("version", "0.1.0");
        assert_eq!(m.artifact_name.as_deref(), Some("phenocommand-web:0.1.0"));
        assert_eq!(m.tags.len(), 2);
        assert_eq!(m.tags[0], ("channel".to_string(), "stable".to_string()));
        assert_eq!(m.tags[1], ("version".to_string(), "0.1.0".to_string()));
    }

    #[test]
    fn composed_artifact_tag_lookup() {
        let a = ComposedArtifact::new("phenocommand-web:0.1.0", ImageRef::new("phenocommand-web:0.1.0"))
            .with_tag("content-digest", "sha256:abc");
        assert_eq!(a.tag("content-digest"), Some("sha256:abc"));
        assert_eq!(a.tag("missing"), None);
    }

    #[test]
    fn image_ref_with_tag_joins_correctly() {
        let r = ImageRef::with_tag("phenocommand-web", "0.1.0");
        assert_eq!(r.reference, "phenocommand-web:0.1.0");
        assert_eq!(r.as_ref(), "phenocommand-web:0.1.0");
        assert_eq!(format!("{r}"), "phenocommand-web:0.1.0");
    }

    #[test]
    fn image_ref_from_str_and_string() {
        let from_str: ImageRef = "phenocommand-web:0.1.0".into();
        let from_string: ImageRef = String::from("phenocommand-web:0.2.0").into();
        assert_eq!(from_str.reference, "phenocommand-web:0.1.0");
        assert_eq!(from_string.reference, "phenocommand-web:0.2.0");
    }

    #[test]
    fn container_id_display_and_as_ref() {
        let id = ContainerId::new("abc123");
        assert_eq!(id.as_ref(), "abc123");
        assert_eq!(format!("{id}"), "abc123");
    }

    #[test]
    fn container_status_is_active_and_display() {
        assert!(ContainerStatus::Running.is_active());
        assert!(ContainerStatus::Paused.is_active());
        assert!(!ContainerStatus::Exited.is_active());
        assert!(!ContainerStatus::NotFound.is_active());

        assert_eq!(format!("{}", ContainerStatus::Running), "running");
        assert_eq!(format!("{}", ContainerStatus::Exited), "exited");
        assert_eq!(format!("{}", ContainerStatus::Paused), "paused");
        assert_eq!(format!("{}", ContainerStatus::NotFound), "not_found");
    }

    #[test]
    fn publish_target_new() {
        let t = PublishTarget::new("docker-registry", "registry.phenotype/phenocommand/web:0.1.0");
        assert_eq!(t.kind, "docker-registry");
        assert_eq!(t.locator, "registry.phenotype/phenocommand/web:0.1.0");
    }

    #[test]
    fn publish_receipt_new() {
        let t = PublishTarget::new("file", "/var/lib/phenocompose/x.tar");
        let r = PublishReceipt::new("phenocommand-web:0.1.0", t.clone(), "/var/lib/phenocompose/x.tar");
        assert_eq!(r.artifact_id, "phenocommand-web:0.1.0");
        assert_eq!(r.target, t);
        assert_eq!(r.published_at, "/var/lib/phenocompose/x.tar");
    }

    #[test]
    fn port_error_display_mentions_kind_and_context() {
        let e = PortError::Validation("empty name".to_string());
        let s = format!("{e}");
        assert!(s.contains("validation"));
        assert!(s.contains("empty name"));
    }

    #[test]
    fn secret_ref_new_uses_empty_namespace() {
        let r = SecretRef::new("db-password");
        assert_eq!(r.name, "db-password");
        assert_eq!(r.namespace, "");
        assert_eq!(r.locator(), "db-password");
        assert_eq!(r.as_ref(), "db-password");
        assert_eq!(format!("{r}"), "db-password");
    }

    #[test]
    fn secret_ref_namespaced_renders_as_namespace_slash_name() {
        let r = SecretRef::namespaced("phenotype", "tls-cert");
        assert_eq!(r.namespace, "phenotype");
        assert_eq!(r.name, "tls-cert");
        assert_eq!(r.locator(), "phenotype/tls-cert");
        assert_eq!(format!("{r}"), "phenotype/tls-cert");
    }

    #[test]
    fn secret_new_defaults_to_version_one() {
        let r = SecretRef::namespaced("phenotype", "api-key");
        let s = Secret::new(r.clone(), "s3cr3t");
        assert_eq!(s.r#ref, r);
        assert_eq!(s.value, "s3cr3t");
        assert_eq!(s.version, 1);
    }

    #[test]
    fn secret_at_version_overrides_counter() {
        let r = SecretRef::new("db-password");
        let s = Secret::new(r, "hunter2").at_version(7);
        assert_eq!(s.version, 7);
    }
}
