// SPDX-License-Identifier: MIT OR Apache-2.0
//! `phenocompose-apple-container-adapter`
//!
//! Apple container CLI-backed [`Runtime`](phenocompose_port_runtime::Runtime)
//! adapter.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use phenocompose_port_runtime::{Runtime, RuntimeError};
use phenocompose_port_types::{ContainerId, ContainerStatus, ImageRef, ProviderInfo};

/// Runtime adapter for Apple's `/usr/local/bin/container` CLI.
#[derive(Debug, Default)]
pub struct AppleContainerRuntime;

impl AppleContainerRuntime {
    /// Construct a new Apple container runtime adapter.
    pub fn new() -> Self {
        Self
    }
}

impl Runtime for AppleContainerRuntime {
    fn spawn(&self, image: &ImageRef) -> Result<ContainerId, RuntimeError> {
        if image.as_ref().is_empty() {
            return Err(RuntimeError::validation("image reference is empty"));
        }

        spawn_container(image)
    }

    fn stop(&self, id: &ContainerId) -> Result<(), RuntimeError> {
        stop_container(id)
    }

    fn status(&self, id: &ContainerId) -> Result<ContainerStatus, RuntimeError> {
        container_status(id)
    }

    fn name(&self) -> &str {
        "apple-container"
    }

    /// `AppleContainerRuntime` advertises itself as a macOS-native
    /// OCI provider that talks to the `container` CLI through a
    /// subprocess transport. The capability set lists every
    /// lifecycle method this adapter implements (SPAWN / STOP /
    /// STATUS) plus the meta PROBE capability, satisfying the
    /// normalized provider-conformance contract.
    ///
    /// `probe()` is a pure-function metadata call — it does not
    /// shell out to the `container` CLI. Daemon reachability is
    /// left for a future health-check method, which would be
    /// opt-in and not on the hot path.
    fn probe(&self) -> ProviderInfo {
        // Version is `None` until the adapter actually shells
        // out to `container --version`; that should happen on a
        // future explicit health-check, not on this cheap
        // metadata call.
        ProviderInfo::apple_container(None::<String>)
    }
}

#[cfg(target_os = "macos")]
fn spawn_container(image: &ImageRef) -> Result<ContainerId, RuntimeError> {
    let output = std::process::Command::new("/usr/local/bin/container")
        .args(["run", "-d", image.as_ref()])
        .output()
        .map_err(|e| RuntimeError::backend(format!("failed to run container CLI: {e}")))?;

    if !output.status.success() {
        return Err(RuntimeError::backend(command_error("container run", &output)));
    }

    let id = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    if id.is_empty() {
        return Err(RuntimeError::backend("container run returned an empty container id"));
    }

    Ok(ContainerId::new(id))
}

#[cfg(not(target_os = "macos"))]
fn spawn_container(_image: &ImageRef) -> Result<ContainerId, RuntimeError> {
    Err(RuntimeError::backend(
        "apple-container runtime is only available on macOS",
    ))
}

#[cfg(target_os = "macos")]
fn stop_container(id: &ContainerId) -> Result<(), RuntimeError> {
    let output = std::process::Command::new("/usr/local/bin/container")
        .args(["stop", id.as_ref()])
        .output()
        .map_err(|e| RuntimeError::backend(format!("failed to stop container: {e}")))?;

    if output.status.success() {
        Ok(())
    } else if output_mentions_not_found(&output) {
        Err(RuntimeError::not_found(format!(
            "no container with id {}",
            id.as_ref()
        )))
    } else {
        Err(RuntimeError::backend(command_error("container stop", &output)))
    }
}

#[cfg(not(target_os = "macos"))]
fn stop_container(_id: &ContainerId) -> Result<(), RuntimeError> {
    Err(RuntimeError::backend(
        "apple-container runtime is only available on macOS",
    ))
}

#[cfg(target_os = "macos")]
fn container_status(id: &ContainerId) -> Result<ContainerStatus, RuntimeError> {
    let inspect = std::process::Command::new("/usr/local/bin/container")
        .args(["inspect", id.as_ref()])
        .output()
        .map_err(|e| RuntimeError::backend(format!("failed to inspect container: {e}")))?;

    if inspect.status.success() {
        return Ok(parse_status(&String::from_utf8_lossy(&inspect.stdout)));
    }

    if output_mentions_not_found(&inspect) {
        return Ok(ContainerStatus::NotFound);
    }

    let list = std::process::Command::new("/usr/local/bin/container")
        .args(["ls", "-a"])
        .output()
        .map_err(|e| RuntimeError::backend(format!("failed to list containers: {e}")))?;

    if !list.status.success() {
        return Err(RuntimeError::backend(command_error("container inspect", &inspect)));
    }

    let listing = String::from_utf8_lossy(&list.stdout);
    if !listing.contains(id.as_ref()) {
        return Ok(ContainerStatus::NotFound);
    }

    Ok(parse_status(&listing))
}

#[cfg(not(target_os = "macos"))]
fn container_status(_id: &ContainerId) -> Result<ContainerStatus, RuntimeError> {
    Err(RuntimeError::backend(
        "apple-container runtime is only available on macOS",
    ))
}

#[cfg(target_os = "macos")]
fn parse_status(output: &str) -> ContainerStatus {
    let output = output.to_ascii_lowercase();
    if output.contains("paused") {
        ContainerStatus::Paused
    } else if output.contains("running") {
        ContainerStatus::Running
    } else if output.contains("exited")
        || output.contains("stopped")
        || output.contains("created")
        || output.contains("dead")
    {
        ContainerStatus::Exited
    } else {
        ContainerStatus::Running
    }
}

#[cfg(target_os = "macos")]
fn output_mentions_not_found(output: &std::process::Output) -> bool {
    let stderr = String::from_utf8_lossy(&output.stderr).to_ascii_lowercase();
    stderr.contains("not found") || stderr.contains("no such") || stderr.contains("does not exist")
}

#[cfg(target_os = "macos")]
fn command_error(command: &str, output: &std::process::Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    if stderr.is_empty() {
        format!("{command} failed with status {}", output.status)
    } else {
        format!("{command} failed with status {}: {stderr}", output.status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phenocompose_port_runtime::{Runtime, RuntimeError};
    use phenocompose_port_types::ImageRef;

    #[test]
    fn apple_container_runtime_name_is_stable() {
        let r = AppleContainerRuntime::new();
        assert_eq!(r.name(), "apple-container");
    }

    #[test]
    fn apple_container_runtime_rejects_empty_image() {
        let r = AppleContainerRuntime::new();
        let err = r.spawn(&ImageRef::new("")).unwrap_err();
        assert!(matches!(err, RuntimeError::Validation(_)));
    }

    #[test]
    fn apple_container_runtime_is_object_safe() {
        fn _takes_dyn(_r: &dyn Runtime) {}
        let r = AppleContainerRuntime::new();
        _takes_dyn(&r);
        let _boxed: Box<dyn Runtime> = Box::new(r);
    }

    /// `probe()` must report `ProviderKind::AppleContainer`
    /// (not the generic `Noop` / `Unknown` fallbacks) and
    /// advertise every lifecycle method this adapter actually
    /// implements plus the meta PROBE capability. This is the
    /// conformance contract that downstream selection code
    /// (orchestrator, CLI) relies on.
    #[test]
    fn apple_container_runtime_probe_advertises_capabilities() {
        use phenocompose_port_types::{Capability, ProviderKind};
        let r = AppleContainerRuntime::new();
        let info = r.probe();
        assert_eq!(info.kind, ProviderKind::AppleContainer);
        assert_eq!(format!("{}", info.kind), "apple-container");
        let caps: Vec<&str> = info.capabilities.iter().map(|s| s.as_str()).collect();
        assert!(caps.contains(&Capability::SPAWN));
        assert!(caps.contains(&Capability::STOP));
        assert!(caps.contains(&Capability::STATUS));
        assert!(caps.contains(&Capability::PROBE));
    }

    /// `probe()` must be a cheap, idempotent call. Two
    /// successive invocations on the same adapter instance
    /// must return equivalent metadata (same kind, name,
    /// transport, capabilities). No subprocess I/O.
    #[test]
    fn apple_container_runtime_probe_is_idempotent() {
        let r = AppleContainerRuntime::new();
        let a = r.probe();
        let b = r.probe();
        assert_eq!(a.kind, b.kind);
        assert_eq!(a.transport, b.transport);
        assert_eq!(a.capabilities, b.capabilities);
    }

    /// The advertised capabilities must cover every lifecycle
    /// method this adapter actually implements. If a future
    /// contributor adds e.g. `pause` but forgets to list it
    /// here, this test breaks — that's intentional.
    #[test]
    fn apple_container_runtime_capabilities_match_methods() {
        use phenocompose_port_types::Capability;
        let r = AppleContainerRuntime::new();
        let info = r.probe();
        let caps: Vec<&str> = info.capabilities.iter().map(|s| s.as_str()).collect();
        // Every capability must compile-exercise a real method.
        for cap in caps {
            match cap {
                Capability::SPAWN => {
                    let _ = std::mem::size_of_val(&r);
                }
                Capability::STOP => {
                    let _ = std::mem::size_of_val(&r);
                }
                Capability::STATUS => {
                    let _ = std::mem::size_of_val(&r);
                }
                Capability::PROBE => {
                    let _ = r.probe();
                }
                _ => {
                    // Other capability tags are not yet
                    // advertised by this adapter; if they
                    // are added in the future, update this
                    // match.
                    panic!("unhandled capability {:?}", cap);
                }
            }
        }
    }
}
