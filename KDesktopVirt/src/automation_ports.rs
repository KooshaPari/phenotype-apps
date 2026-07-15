//! Automation port traits — the hexagonal contract for device automation.
//!
//! These trait definitions and their supporting value types were **migrated from
//! the Eidolon repository** (`eidolon-core`), which defined a clean trait-based
//! automation abstraction but contained no concrete implementations — every
//! Eidolon impl was a stub deferring to KDesktopVirt. KDesktopVirt already owns
//! the real implementations (see [`crate::ui_automation`], [`crate::automation`],
//! [`crate::ffmpeg_pipeline`], [`crate::recording`]); this module gives those
//! implementations a published, trait-abstracted port boundary so callers can
//! depend on the abstraction rather than concrete engines.
//!
//! Consolidation note: with this contract present here, the Eidolon repository is
//! fully superseded by KDesktopVirt (implementations + contract).
//!
//! Error handling uses the crate-wide [`anyhow::Result`] rather than Eidolon's
//! external `phenotype_errors` dependency, so no new dependency is introduced.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

// ============================================================================
// Value types
// ============================================================================

/// Viewport dimensions and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    /// Display width in pixels.
    pub width: u32,
    /// Display height in pixels.
    pub height: u32,
    /// Device pixel ratio (DPI scaling).
    pub dpr: f64,
    /// Orientation: `"portrait"` or `"landscape"`.
    pub orientation: String,
}

impl Viewport {
    pub fn new(width: u32, height: u32, dpr: f64) -> Self {
        let orientation = if width > height {
            "landscape".to_string()
        } else {
            "portrait".to_string()
        };
        Self {
            width,
            height,
            dpr,
            orientation,
        }
    }

    /// Desktop standard: 1920x1080 @ 1.0 DPI.
    pub fn desktop_fhd() -> Self {
        Self::new(1920, 1080, 1.0)
    }

    /// Mobile standard: 1080x1920 @ 2.0 DPI (portrait).
    pub fn mobile_fhd() -> Self {
        Self::new(1080, 1920, 2.0)
    }

    /// Tablet standard: 2560x1440 @ 1.5 DPI.
    pub fn tablet_qhd() -> Self {
        Self::new(2560, 1440, 1.5)
    }
}

/// Pointer (mouse/touch) input action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerInput {
    /// X coordinate.
    pub x: i32,
    /// Y coordinate.
    pub y: i32,
    /// Button: `"left"`, `"right"`, `"middle"`, or `None` for movement.
    pub button: Option<String>,
    /// Action: `"press"`, `"release"`, `"move"`, `"tap"`, `"long_press"`.
    pub action: String,
    /// Duration in milliseconds for long press / hold.
    pub duration_ms: Option<u32>,
}

impl PointerInput {
    pub fn click(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            button: Some("left".to_string()),
            action: "press".to_string(),
            duration_ms: None,
        }
    }

    pub fn move_to(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            button: None,
            action: "move".to_string(),
            duration_ms: None,
        }
    }
}

/// Text input action.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextInput {
    /// Text to input.
    pub text: String,
    /// Type of input: `"keystroke"`, `"paste"`, `"clear"`.
    pub input_type: String,
    /// Delay between keystrokes (ms).
    pub delay_ms: Option<u32>,
}

impl TextInput {
    pub fn keystroke(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            input_type: "keystroke".to_string(),
            delay_ms: None,
        }
    }

    pub fn paste(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            input_type: "paste".to_string(),
            delay_ms: None,
        }
    }
}

/// Unified automation event for the audit log.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationEvent {
    /// Event identifier.
    pub id: String,
    /// Event type: `"pointer"`, `"text"`, `"screenshot"`, `"assertion"`, `"navigate"`.
    pub event_type: String,
    /// Platform: `"desktop"`, `"mobile"`, `"sandbox"`.
    pub platform: String,
    /// Event payload.
    pub payload: EventPayload,
    /// Timestamp (Unix seconds).
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventPayload {
    Pointer(PointerInput),
    Text(TextInput),
    Screenshot { path: String },
    Assertion { condition: String, expected: String },
    Navigate { url: String },
    Custom { data: serde_json::Value },
}

fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl AutomationEvent {
    /// Create a new pointer event.
    pub fn pointer(platform: &str, input: PointerInput) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            event_type: "pointer".to_string(),
            platform: platform.to_string(),
            payload: EventPayload::Pointer(input),
            timestamp: now_unix_secs(),
        }
    }

    /// Create a new text input event.
    pub fn text(platform: &str, input: TextInput) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            event_type: "text".to_string(),
            platform: platform.to_string(),
            payload: EventPayload::Text(input),
            timestamp: now_unix_secs(),
        }
    }

    /// Create a screenshot event.
    pub fn screenshot(platform: &str, path: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            event_type: "screenshot".to_string(),
            platform: platform.to_string(),
            payload: EventPayload::Screenshot { path: path.into() },
            timestamp: now_unix_secs(),
        }
    }
}

/// Sandbox metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxMetadata {
    pub id: String,
    pub image: String,
    pub cpu_limit: u32,
    pub memory_limit_mb: u32,
    pub disk_limit_mb: Option<u32>,
}

/// Resource usage snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUsage {
    pub cpu_percent: f64,
    pub memory_mb: u32,
    pub disk_mb: Option<u32>,
}

// ============================================================================
// Port traits
// ============================================================================

/// Desktop automation port.
/// Implemented by: macOS (native), Windows (native), Linux (X11/Wayland).
#[async_trait::async_trait]
pub trait DesktopAutomator: Send + Sync {
    /// Get current viewport dimensions.
    async fn get_viewport(&self) -> Result<Viewport>;
    /// Take a screenshot to `path`.
    async fn screenshot(&self, path: &str) -> Result<()>;
    /// Execute pointer input.
    async fn pointer(&self, event: &PointerInput) -> Result<()>;
    /// Execute text input.
    async fn text(&self, event: &TextInput) -> Result<()>;
    /// Record an automation event for the audit log.
    async fn record_event(&self, event: AutomationEvent) -> Result<()>;
}

/// Mobile automation port.
/// Implemented by: iOS (via XCTest), Android (via UiAutomator).
#[async_trait::async_trait]
pub trait MobileAutomator: Send + Sync {
    /// Get current viewport (screen dimensions).
    async fn get_viewport(&self) -> Result<Viewport>;
    /// Take a screenshot to `path`.
    async fn screenshot(&self, path: &str) -> Result<()>;
    /// Tap screen at coordinates.
    async fn tap(&self, x: i32, y: i32) -> Result<()>;
    /// Swipe from `(x1, y1)` to `(x2, y2)`.
    async fn swipe(&self, x1: i32, y1: i32, x2: i32, y2: i32) -> Result<()>;
    /// Input text.
    async fn input_text(&self, text: &str) -> Result<()>;
    /// Record an automation event for the audit log.
    async fn record_event(&self, event: AutomationEvent) -> Result<()>;
}

/// Sandbox / container automation port.
/// Implemented by: nanoVMs, Docker, Firecracker, KVM VMs.
#[async_trait::async_trait]
pub trait SandboxAutomator: Send + Sync {
    /// Get sandbox metadata (image, resource limits).
    async fn get_metadata(&self) -> Result<SandboxMetadata>;
    /// Start the sandbox.
    async fn start(&self) -> Result<()>;
    /// Stop the sandbox.
    async fn stop(&self) -> Result<()>;
    /// Execute a command inside the sandbox.
    async fn exec(&self, cmd: &str) -> Result<String>;
    /// Get current resource usage (CPU, memory, disk).
    async fn resource_usage(&self) -> Result<ResourceUsage>;
    /// Record an automation event for the audit log.
    async fn record_event(&self, event: AutomationEvent) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn viewport_orientation_is_derived() {
        assert_eq!(Viewport::desktop_fhd().orientation, "landscape");
        assert_eq!(Viewport::mobile_fhd().orientation, "portrait");
        assert_eq!(Viewport::tablet_qhd().width, 2560);
    }

    #[test]
    fn pointer_and_text_constructors() {
        let p = PointerInput::click(10, 20);
        assert_eq!(p.button.as_deref(), Some("left"));
        assert_eq!(PointerInput::move_to(1, 2).action, "move");
        assert_eq!(TextInput::paste("hi").input_type, "paste");
    }

    #[test]
    fn automation_event_round_trips_through_serde() {
        let ev = AutomationEvent::pointer("desktop", PointerInput::click(5, 5));
        let json = serde_json::to_string(&ev).expect("serialize");
        let back: AutomationEvent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.event_type, "pointer");
        assert_eq!(back.platform, "desktop");
    }

    /// A concrete impl can be written against the port (compile-time proof the
    /// contract is usable). Real impls live in the crate's automation engines.
    struct DummyDesktop;
    #[async_trait::async_trait]
    impl DesktopAutomator for DummyDesktop {
        async fn get_viewport(&self) -> Result<Viewport> {
            Ok(Viewport::desktop_fhd())
        }
        async fn screenshot(&self, _path: &str) -> Result<()> {
            Ok(())
        }
        async fn pointer(&self, _event: &PointerInput) -> Result<()> {
            Ok(())
        }
        async fn text(&self, _event: &TextInput) -> Result<()> {
            Ok(())
        }
        async fn record_event(&self, _event: AutomationEvent) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn port_is_implementable_and_object_safe() {
        let d: Box<dyn DesktopAutomator> = Box::new(DummyDesktop);
        assert_eq!(d.get_viewport().await.unwrap().width, 1920);
    }
}
