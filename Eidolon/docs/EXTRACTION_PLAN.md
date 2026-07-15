# Extraction Plan — Eidolon

Per KDesktopVirt audit: design fresh with trait-based core. Selectively extract salvageable modules from sibling projects.

## Source Projects

### KDesktopVirt (macOS/Windows/Linux automation)

**Status**: Core infrastructure broken; FFmpeg pipeline + security framework salvageable.

| Module | Status | Target in Eidolon | Notes |
|--------|--------|-------------------|-------|
| FFmpeg screenshot pipeline | ✅ Working | `eidolon-desktop` | Handles encoding, performance profiling |
| Security framework (sandboxing) | ✅ Working | `eidolon-core` error/permissions layer | Rate limiting, capability checks |
| Pointer/keyboard input system | ❌ Broken | Design fresh | XCTest/UiAutomator pattern preferred |
| Native API bindings (macOS NSScreen, etc.) | ⚠️ Partial | `eidolon-desktop` | Salvage working bits; modernize |
| tts_audio_system | ❌ Broken | Do NOT extract | Replace with external TTS service |
| ffmpeg_pipeline_broken | ❌ Broken | Do NOT extract | Use working FFmpeg pipeline instead |

### kmobile (iOS/Android automation)

**Status**: Working interfaces; ready for direct consumption.

| Module | Status | Target in Eidolon | Notes |
|--------|--------|-------------------|-------|
| XCTest adapter | ✅ Working | `eidolon-mobile` | Adapt to `MobileAutomator` trait |
| UiAutomator adapter | ✅ Working | `eidolon-mobile` | Adapt to `MobileAutomator` trait |
| Device discovery | ✅ Working | `eidolon-mobile` | List connected iOS/Android devices |
| Screenshot capture | ✅ Working | `eidolon-mobile` | Consistent with desktop pipeline |

### KVirtualStage (container/VM automation)

**Status**: VM orchestration patterns; nanoVMs integration pending.

| Module | Status | Target in Eidolon | Notes |
|--------|--------|-------------------|-------|
| Docker adapter | ✅ Working | `eidolon-sandbox` | `start()`, `stop()`, `exec()` |
| Resource monitoring | ✅ Working | `eidolon-sandbox` | CPU, memory, disk usage snapshots |
| nanoVMs integration | ⚠️ In progress | `eidolon-sandbox` | Low-footprint VM pattern |

### PlayCua (desktop sandbox / virtual display)

**Status**: Virtual display patterns; Xvfb/Wayland integration.

| Module | Status | Target in Eidolon | Notes |
|--------|--------|-------------------|-------|
| Virtual display manager | ✅ Working | `eidolon-sandbox` | Xvfb, VNC, Wayland isolation |
| Display resolution/DPI config | ✅ Working | `eidolon-core::Viewport` | Reusable abstraction |

### bare-cua (lightweight containerization)

**Status**: Security isolation patterns; namespace/cgroup abstractions.

| Module | Status | Target in Eidolon | Notes |
|--------|--------|-------------------|-------|
| Namespace/cgroup wrappers | ✅ Working | `eidolon-sandbox` security layer | Privilege isolation |
| Resource limits | ✅ Working | `eidolon-sandbox::SandboxMetadata` | CPU, memory, disk caps |

## Extraction Order (Phased)

### Phase 1: Foundation (High confidence, no breaking changes)
1. Extract kmobile XCTest/UiAutomator → `eidolon-mobile`
2. Extract KVirtualStage Docker adapter → `eidolon-sandbox`
3. Extract PlayCua Viewport/DPI logic → `eidolon-core`

### Phase 2: Desktop Platform (Medium confidence, requires refactoring)
4. Extract KDesktopVirt FFmpeg pipeline → `eidolon-desktop`
5. Extract KDesktopVirt security framework → `eidolon-core`
6. Modernize native API bindings (macOS/Windows/Linux)

### Phase 3: Advanced Features (Lower priority, future)
7. Integrate KVirtualStage nanoVMs patterns
8. Integrate bare-cua namespace/cgroup abstractions
9. Add event serialization/playback (from agileplus-event-sourcing?)

## Dependency Graph

```
eidolon-core (no deps)
├── eidolon-desktop
├── eidolon-mobile
└── eidolon-sandbox
```

No cross-crate dependencies; each implementation consumes only `eidolon-core` traits.

## Quality Gates

- ✅ All stubs implement their trait (no unimplemented!() calls)
- ✅ Extraction target code compiles without warnings (`cargo clippy`)
- ✅ New trait methods have matching implementations in all stubs
- ✅ Event serialization round-trips (AutomationEvent → JSON → AutomationEvent)
- ✅ Each extraction has ≥1 integration test verifying trait behavior
