# Eidolon → KDesktopVirt consolidation (proof of redundancy)

**Decision (org steward + lead, 2026-06-02):** fold the Eidolon agent-automation
framework into KDesktopVirt (the most complete desktop-automation tier), then
archive Eidolon. PlayCua stays separate (distinct bare-metal layer); KVirtualStage
is untouched (protected).

## What Eidolon actually contained

Eidolon's `eidolon-desktop`, `eidolon-mobile`, and `eidolon-sandbox` crates had
**no real implementations** — every `impl ...Automator` body was a log line plus a
`// TODO: Integrate ...` comment that deferred to KDesktopVirt / kmobile /
KVirtualStage. Examples:

| Eidolon file | stub body |
|---|---|
| `eidolon-desktop/src/lib.rs` `screenshot()` | `// TODO: Integrate FFmpeg pipeline from KDesktopVirt` |
| `eidolon-mobile/src/native/mod.rs` | `// TODO: integrate kmobile XCTest/UiAutomator wrappers` |
| `eidolon-sandbox/src/docker/mod.rs` | `// TODO: integrate KVirtualStage patterns` |

The only substance was the **trait contract**: `DesktopAutomator`,
`MobileAutomator`, `SandboxAutomator`, plus the `Viewport`, `AutomationEvent`,
`PointerInput`, `TextInput`, `SandboxMetadata`, and `ResourceUsage` value types.

## Why KDesktopVirt already supersedes it

KDesktopVirt owns the **real implementations** Eidolon only stubbed:

- screenshots + FFmpeg recording: `src/api.rs` (`take_screenshot`), `src/audio.rs`,
  `src/ffmpeg_pipeline.rs`, `src/recording*.rs`
- UI automation engine: `src/ui_automation.rs` (`UiAutomationEngine`)
- automation orchestration: `src/automation/` module
- container/sandbox + virtualization: `src/virtualization.rs`,
  `src/containerization.rs`, `src/podman_integration.rs`

## What this PR migrates

KDesktopVirt previously had no trait-abstracted port boundary for automation.
This PR ports Eidolon's trait contract verbatim into
[`src/automation_ports.rs`](../src/automation_ports.rs) (re-homed onto the
crate-wide `anyhow::Result`, no new dependency), giving KDesktopVirt the
hexagonal port boundary it lacked. With this landed, **both** the
implementations and the contract now live in KDesktopVirt.

## Conclusion

Nothing unique remains in Eidolon. It is fully superseded and is an **archive
candidate**. Archival is escalated to the lead with this PR as proof of migration;
the steward does not archive.
