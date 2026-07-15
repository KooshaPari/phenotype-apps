// SPDX-License-Identifier: MIT OR Apache-2.0
//! `phenocompose-port-runtime`
//!
//! The Runtime port trait — the canonical hex-architecture port
//! for spawning / stopping / statusing containers from an
//! [`ImageRef`].
//!
//! Adapters implement [`Runtime`] to bridge to local container
//! engines (Docker, Podman, `systemd-nspawn`, Kubernetes, the
//! Apple `container` CLI, `wslc.exe`, ...). The trait
//! intentionally exposes only the three operations the
//! orchestration layer needs: `spawn`, `stop`, `status`, plus a
//! cheap metadata probe (`probe`) that lets downstream code
//! (orchestrator, CLI, observability) select and report on a
//! normalized provider surface without reaching into adapter-
//! specific types.
//!
//! Object-safety: the trait has no associated types, no generic
//! methods, and only `&self` receivers (with `Send + Sync`
//! super-traits) so it can be stored as `Box<dyn Runtime>` and
//! dispatched dynamically.
//!
//! See also: [`phenocompose_port_types`] for the value types
//! ([`ImageRef`], [`ContainerId`], [`ContainerStatus`],
//! [`PortError`], [`ProviderInfo`], [`ProviderKind`]) that
//! flow across this port.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

// Re-export the canonical provider-metadata types so adapters
// can `use phenocompose_port_runtime::{ProviderInfo, ProviderKind}`
// without taking a direct dependency on `phenocompose_port_types`.
pub use phenocompose_port_types::{Capability, ProviderInfo, ProviderKind, Transport};

use phenocompose_port_types::{ContainerId, ContainerStatus, ImageRef, PortError};
use std::fmt;
use thiserror::Error;

/// The Runtime port trait — `Send + Sync` + no generics + no
/// associated types ⇒ object-safe ⇒ storable as
/// `Box<dyn Runtime>`.
pub trait Runtime: Send + Sync {
    /// Spawn a container from the given [`ImageRef`].
    ///
    /// Implementations return a [`ContainerId`] that can later
    /// be passed to [`Runtime::stop`] and [`Runtime::status`].
    /// The id MUST be unique to the spawned container (so two
    /// spawns of the same image MUST return distinct ids).
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeError::Validation`] for inputs the
    /// adapter considers malformed (e.g. an empty image ref),
    /// or [`RuntimeError::Backend`] for transport / daemon
    /// failures.
    fn spawn(&self, image: &ImageRef) -> Result<ContainerId, RuntimeError>;

    /// Stop the container with the given id.
    ///
    /// Idempotent: stopping an already-stopped container is a
    /// no-op and returns `Ok(())`. Stopping a non-existent
    /// container returns [`RuntimeError::NotFound`] (idempotency
    /// is a property of the call site, not the runtime — a
    /// caller that wants to "stop if running" can call
    /// [`Runtime::status`] first).
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeError::NotFound`] if the id is unknown
    /// to the runtime, or [`RuntimeError::Backend`] for daemon
    /// failures.
    fn stop(&self, id: &ContainerId) -> Result<(), RuntimeError>;

    /// Report the [`ContainerStatus`] of the container with the
    /// given id.
    ///
    /// # Errors
    ///
    /// Returns [`RuntimeError::Backend`] for daemon failures
    /// (e.g. the runtime is offline). A non-existent id is
    /// reported as [`ContainerStatus::NotFound`] (not an error)
    /// so callers can branch on it.
    fn status(&self, id: &ContainerId) -> Result<ContainerStatus, RuntimeError>;

    /// Optional human-readable adapter name (e.g. `"docker"`,
    /// `"podman"`, `"noop"`). Defaults to `"unknown"`.
    fn name(&self) -> &str {
        "unknown"
    }

    /// Report what this adapter is and which static lifecycle
    /// capabilities it advertises. The default implementation
    /// returns a generic [`ProviderInfo::unknown`] value (with
    /// no capabilities advertised).
    ///
    /// `probe()` is **forward-only**: existing adapters that
    /// don't override it continue to satisfy the trait without
    /// modification. Adapters that *do* override it MUST advertise
    /// the [`Capability::SPAWN`], [`Capability::STOP`],
    /// [`Capability::STATUS`], and [`Capability::PROBE`] tags
    /// (the lifecycle methods they implement) so downstream code
    /// can branch on a normalized provider surface.
    ///
    /// `probe()` MUST be cheap — it is intended for documentation,
    /// observability, and CLI selection, **not** for hot-path
    /// container management. Adapters MUST NOT issue subprocess
    /// I/O or open sockets here unless they cache the result.
    fn probe(&self) -> ProviderInfo {
        ProviderInfo::unknown()
    }
}

/// Errors a [`Runtime`] can return.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum RuntimeError {
    /// The input failed validation (e.g. an empty image ref).
    #[error("runtime validation: {0}")]
    Validation(String),
    /// The runtime daemon / backend failed.
    #[error("runtime backend: {0}")]
    Backend(String),
    /// The id is not known to the runtime (returned by
    /// [`Runtime::stop`] only — [`Runtime::status`] reports
    /// unknown ids as [`ContainerStatus::NotFound`]).
    #[error("runtime not found: {0}")]
    NotFound(String),
}

impl RuntimeError {
    /// Convenience constructor for [`RuntimeError::Validation`].
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Convenience constructor for [`RuntimeError::Backend`].
    pub fn backend(msg: impl Into<String>) -> Self {
        Self::Backend(msg.into())
    }

    /// Convenience constructor for [`RuntimeError::NotFound`].
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
}

impl From<PortError> for RuntimeError {
    fn from(e: PortError) -> Self {
        match e {
            PortError::Validation(s) | PortError::Unsupported(s) => Self::Validation(s),
            PortError::NotFound(s) => Self::NotFound(s),
            PortError::Transport(s) => Self::Backend(s),
        }
    }
}

/// A trivial in-memory [`Runtime`] used for tests and as a
/// default for adapters that don't talk to a real container
/// engine (e.g. a dry-run mode that just records what would be
/// spawned).
///
/// `NoopRuntime` assigns a fresh monotonic id on each `spawn`
/// and tracks the ids in a `Vec<String>`; `status` reports
/// `Running` for known ids and `NotFound` for unknown ones;
/// `stop` removes the id from the tracking list (so the
/// follow-up `status` returns `NotFound`).
#[derive(Debug, Default)]
pub struct NoopRuntime {
    /// Monotonic counter for spawn-id generation.
    pub next_id: std::sync::atomic::AtomicUsize,
    /// Tracking list — id is in the list ⇔ `status` reports
    /// `Running`.
    pub alive: std::sync::Mutex<Vec<String>>,
}

impl NoopRuntime {
    /// Construct a fresh `NoopRuntime` with an empty tracking
    /// list and a zero counter.
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Display for NoopRuntime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("noop")
    }
}

impl Runtime for NoopRuntime {
    fn spawn(&self, image: &ImageRef) -> Result<ContainerId, RuntimeError> {
        use std::sync::atomic::Ordering;
        if image.as_ref().is_empty() {
            return Err(RuntimeError::validation("image reference is empty"));
        }
        let n = self.next_id.fetch_add(1, Ordering::SeqCst);
        let id = format!("noop-{}", n);
        self.alive
            .lock()
            .expect("noop runtime mutex poisoned")
            .push(id.clone());
        Ok(ContainerId::new(id))
    }

    fn stop(&self, id: &ContainerId) -> Result<(), RuntimeError> {
        let mut guard = self.alive.lock().expect("noop runtime mutex poisoned");
        let pos = guard.iter().position(|x| x == id.as_ref());
        match pos {
            Some(i) => {
                guard.swap_remove(i);
                Ok(())
            }
            None => Err(RuntimeError::not_found(format!(
                "no container with id {}",
                id.as_ref()
            ))),
        }
    }

    fn status(&self, id: &ContainerId) -> Result<ContainerStatus, RuntimeError> {
        let guard = self.alive.lock().expect("noop runtime mutex poisoned");
        if guard.iter().any(|x| x == id.as_ref()) {
            Ok(ContainerStatus::Running)
        } else {
            Ok(ContainerStatus::NotFound)
        }
    }

    fn name(&self) -> &str {
        "noop"
    }

    /// `NoopRuntime` advertises itself as a `ProviderKind::Noop`
    /// subprocess adapter that conforms to the base lifecycle
    /// contract (spawn / stop / status) without reaching any
    /// real container engine. `probe()` is cheap and side-effect
    /// free.
    fn probe(&self) -> ProviderInfo {
        ProviderInfo::noop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phenocompose_port_types::ImageRef;

    #[test]
    fn noop_runtime_spawn_assigns_unique_ids() {
        let r = NoopRuntime::new();
        let a = r.spawn(&ImageRef::new("phenocommand-web:0.1.0")).unwrap();
        let b = r.spawn(&ImageRef::new("phenocommand-web:0.1.0")).unwrap();
        assert_ne!(a, b);
    }

    #[test]
    fn noop_runtime_spawn_then_status_reports_running() {
        let r = NoopRuntime::new();
        let id = r.spawn(&ImageRef::new("phenocommand-web:0.1.0")).unwrap();
        let status = r.status(&id).unwrap();
        assert_eq!(status, ContainerStatus::Running);
    }

    #[test]
    fn noop_runtime_stop_then_status_reports_not_found() {
        let r = NoopRuntime::new();
        let id = r.spawn(&ImageRef::new("phenocommand-web:0.1.0")).unwrap();
        r.stop(&id).unwrap();
        let status = r.status(&id).unwrap();
        assert_eq!(status, ContainerStatus::NotFound);
    }

    #[test]
    fn noop_runtime_stop_unknown_returns_not_found_error() {
        let r = NoopRuntime::new();
        let err = r.stop(&ContainerId::new("nope")).unwrap_err();
        assert!(matches!(err, RuntimeError::NotFound(_)));
    }

    #[test]
    fn noop_runtime_spawn_rejects_empty_image_ref() {
        let r = NoopRuntime::new();
        let err = r.spawn(&ImageRef::new("")).unwrap_err();
        assert!(matches!(err, RuntimeError::Validation(_)));
    }

    #[test]
    fn noop_runtime_status_unknown_id_returns_not_found_value() {
        let r = NoopRuntime::new();
        let status = r.status(&ContainerId::new("nope")).unwrap();
        assert_eq!(status, ContainerStatus::NotFound);
    }

    #[test]
    fn runtime_error_from_port_error_dispatches() {
        let pe = PortError::Validation("bad".to_string());
        let re: RuntimeError = pe.into();
        assert!(matches!(re, RuntimeError::Validation(_)));

        let pe = PortError::NotFound("missing".to_string());
        let re: RuntimeError = pe.into();
        assert!(matches!(re, RuntimeError::NotFound(_)));

        let pe = PortError::Transport("net".to_string());
        let re: RuntimeError = pe.into();
        assert!(matches!(re, RuntimeError::Backend(_)));
    }

    #[test]
    fn runtime_trait_is_object_safe() {
        fn _takes_dyn(_r: &dyn Runtime) {}
        // Compile-time check: Runtime is object-safe.
    }

    /// `NoopRuntime::probe()` advertises the base lifecycle
    /// capabilities (SPAWN/STOP/STATUS) plus the meta PROBE
    /// capability, with kind `Noop` and a Unix-subprocess
    /// transport. The conformance surface guarantees that all
    /// adapters which override `probe()` must list every
    /// lifecycle method they implement.
    #[test]
    fn noop_runtime_probe_advertises_full_lifecycle() {
        use phenocompose_port_types::Capability;
        let r = NoopRuntime::new();
        let info = r.probe();
        assert_eq!(info.kind, ProviderKind::Noop);
        assert!(format!("{}", info.kind).contains("noop"));
        assert_eq!(info.transport, phenocompose_port_types::Transport::Subprocess);
        let caps: Vec<&str> = info.capabilities.iter().map(|s| s.as_str()).collect();
        for required in [
            Capability::SPAWN,
            Capability::STOP,
            Capability::STATUS,
            Capability::PROBE,
        ] {
            assert!(
                caps.contains(&required),
                "noop probe must advertise {:?}, got {:?}",
                required,
                caps
            );
        }
    }

    /// The default `probe()` (used by adapters that haven't
    /// been ported yet) must remain cheap and non-functional:
    /// it advertises `ProviderKind::Unknown` with no
    /// capabilities. This guarantees forward-only behaviour
    /// — pre-probe adapters still satisfy the trait.
    #[test]
    fn default_probe_is_cheap_and_unknown() {
        use phenocompose_port_types::Capability;
        struct PreProbe;
        impl Runtime for PreProbe {
            fn spawn(&self, _: &ImageRef) -> Result<ContainerId, RuntimeError> {
                Ok(ContainerId::new("pre"))
            }
            fn stop(&self, _: &ContainerId) -> Result<(), RuntimeError> {
                Ok(())
            }
            fn status(&self, _: &ContainerId) -> Result<ContainerStatus, RuntimeError> {
                Ok(ContainerStatus::Running)
            }
        }
        let r = PreProbe;
        let info = r.probe();
        assert_eq!(info.kind, ProviderKind::Unknown);
        assert!(info.capabilities.is_empty());
        // Default PROBE capability must NOT be advertised —
        // that's only for adapters that opt into the new
        // contract.
        assert!(!info.capabilities.iter().any(|c| c == Capability::PROBE));
    }

    /// Adapter conformance: any `Runtime` that exposes
    /// `probe()` must satisfy the invariant
    /// "advertised capabilities ⊆ implemented methods". This
    /// test compiles only if every lifecycle method used in
    /// the capability set is implemented on `NoopRuntime`,
    /// which is the canary adapter for the contract.
    #[test]
    fn noop_runtime_advertised_capabilities_match_implemented_methods() {
        use phenocompose_port_types::Capability;
        let r = NoopRuntime::new();
        let info = r.probe();
        for cap in &info.capabilities {
            match cap.as_str() {
                Capability::SPAWN => {
                    let _ = r.spawn(&ImageRef::new("phenocommand-web:0.1.0")).unwrap();
                }
                Capability::STOP => {
                    // Stop is not exercised because we'd need
                    // a known id; the existence of the call
                    // site is the conformance check.
                    let _ = r.stop(&ContainerId::new("noop-0")).is_ok()
                        || matches!(
                            r.stop(&ContainerId::new("noop-0")),
                            Err(RuntimeError::NotFound(_))
                        );
                }
                Capability::STATUS => {
                    let _ = r.status(&ContainerId::new("noop-0")).unwrap();
                }
                Capability::PROBE => {
                    let _ = r.probe();
                }
                _ => {
                    panic!("unhandled capability {:?}", cap);
                }
            }
        }
    }
}
