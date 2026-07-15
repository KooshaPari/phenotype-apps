# ADR-001: Automation Engine Architecture

**Document ID:** PHENOTYPE_KDESKTOPVIRT_ADR_001  
**Status:** Accepted  
**Last Updated:** 2026-04-03  
**Author:** Phenotype Architecture Team  
**Supersedes:** N/A  
**Related:** ADR-002, ADR-003, SPEC.md

---

## Context

KDesktopVirt requires a core automation engine capable of interacting with desktop environments in a way that is both programmatically precise and visually natural. The engine must serve two distinct audiences:

1. **AI Agents**: Need structured, reliable APIs for programmatic control through MCP and REST interfaces
2. **Human Observers**: Need to see automation that looks natural (not robotic) when watching recordings or live sessions

### Technical Constraints

- Must support X11 as the primary display server (Wayland support planned)
- Must integrate with Docker-based containerized desktop sessions
- Must provide both coordinate-based and element-based interaction
- Must support recording synchronization (FFmpeg pipeline)
- Must be async-first (Tokio runtime)
- Must be memory-safe (Rust)

### Design Alternatives Considered

```
Alternative 1: Pure Coordinate-Based (xdotool wrapper)
┌─────────────────────────────────────────────────────┐
│  Pros: Simple, proven, fast                         │
│  Cons: Robotic movement, no element awareness       │
│  Result: Rejected - fails natural interaction req.  │
└─────────────────────────────────────────────────────┘

Alternative 2: Pure AI-Based (vision model only)
┌─────────────────────────────────────────────────────┐
│  Pros: Self-healing, natural language tasks         │
│  Cons: Slow (2s+ latency), cloud-dependent, costly  │
│  Result: Rejected - fails performance/self-host req.│
└─────────────────────────────────────────────────────┘

Alternative 3: Hybrid Engine (Physics + AI)
┌─────────────────────────────────────────────────────┐
│  Pros: Natural movement, fast execution, extensible │
│  Cons: More complex implementation                  │
│  Result: ACCEPTED                                   │
└─────────────────────────────────────────────────────┘
```

---

## Decision

We adopt a **Hybrid Automation Engine** with three layers:

```
┌─────────────────────────────────────────────────────────────┐
│              KDesktopVirt Automation Engine                  │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  Layer 1: Physics Engine (WindMouse 2.0)                    │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  WindMouseEngine                                    │   │
│  │  ├── Gravity force (adaptive target attraction)     │   │
│  │  ├── Wind force (controlled randomness)             │   │
│  │  ├── Tremor force (8-12 Hz physiological tremor)    │   │
│  │  ├── Context force (obstacle avoidance)             │   │
│  │  └── User profile (speed, precision, fatigue)       │   │
│  │                                                     │   │
│  │  NaturalTypingEngine                                │   │
│  │  ├── Character-specific timing                      │   │
│  │  ├── Burst typing for common words                  │   │
│  │  ├── Adjacent-key error simulation                  │   │
│  │  ├── Fatigue-based slowdown                         │   │
│  │  └── Natural pauses at word boundaries              │   │
│  └─────────────────────────────────────────────────────┘   │
│                          │                                  │
│  Layer 2: Automation Engine                                 │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  AutomationEngine                                   │   │
│  │  ├── move_cursor_naturally()                        │   │
│  │  ├── click_naturally()                              │   │
│  │  ├── type_text_naturally()                          │   │
│  │  ├── execute_workflow()                             │   │
│  │  └── Performance metrics tracking                   │   │
│  └─────────────────────────────────────────────────────┘   │
│                          │                                  │
│  Layer 3: Platform Abstraction                              │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Platform-specific implementations (trait-based)    │   │
│  │  ├── X11: xdotool, XTest, x11 crate                 │   │
│  │  ├── Wayland: virtual-keyboard, virtual-pointer     │   │
│  │  ├── Windows: SendInput (future)                    │   │
│  │  └── macOS: CGEvent (future)                        │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

### Core Data Structures

```rust
// Point and Vector2 for geometry
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

// WindMouse 2.0 engine with user profiles
pub struct WindMouseEngine {
    pub gravity: f64,          // 12.0 - gravitational pull
    pub wind: f64,             // 4.0  - random wind force
    pub friction: f64,         // 0.95 - velocity damping
    pub target_awareness: f64, // 15.0 - proximity threshold
    pub user_profile: UserMovementProfile,
    trajectory_cache: HashMap<String, Vec<MovementFrame>>,
}

// Natural typing with fatigue model
pub struct NaturalTypingEngine {
    pub base_wpm: f64,             // 65.0 average
    pub keystroke_variance: f64,   // 0.3 timing variation
    pub error_probability: f64,    // 0.02 (2% error rate)
    pub fatigue_model: TypingFatigue,
    pub correction_behavior: CorrectionStyle,
}

// Main engine combining both
pub struct AutomationEngine {
    pub windmouse: WindMouseEngine,
    pub typing_engine: NaturalTypingEngine,
    pub session_id: Option<String>,
    pub is_recording: bool,
    performance_metrics: PerformanceMetrics,
}
```

### Workflow Execution

```rust
pub struct AutomationWorkflow {
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub continue_on_error: bool,
}

pub enum StepAction {
    MoveCursor { to: Point },
    Click { position: Point, button: MouseButton },
    Type { text: String },
    Wait { duration: Duration },
}
```

### Platform Abstraction Trait

```rust
#[async_trait]
pub trait PlatformInput {
    async fn move_cursor(&self, x: f64, y: f64) -> Result<()>;
    async fn click(&self, button: MouseButton) -> Result<()>;
    async fn type_character(&self, char: char) -> Result<()>;
    async fn press_key(&self, key: &str) -> Result<()>;
    async fn screenshot(&self) -> Result<Vec<u8>>;
}
```

---

## Consequences

### Positive

1. **Natural-Looking Automation**: WindMouse 2.0 produces cursor movement indistinguishable from human interaction, with gravity-based attraction, wind-based randomness, and physiological tremor simulation.

2. **Extensible Architecture**: The three-layer design allows adding new platforms (Wayland, Windows, macOS) without modifying the physics or automation logic.

3. **Performance**: Physics-based movement is computed locally (<1ms per frame), avoiding the 2-3 second latency of cloud-based AI vision models.

4. **Trajectory Caching**: Repeated movements between the same coordinates are cached, reducing computation for common automation patterns.

5. **User Profiles**: Configurable movement profiles (speed, precision, jitter, hesitation, fatigue) enable realistic variation between automation sessions.

6. **Recording Synchronization**: The engine tracks timing at the frame level, enabling precise synchronization with FFmpeg recording pipelines.

7. **Error Simulation**: NaturalTypingEngine simulates realistic typing errors (adjacent-key mistakes) with correction behavior, making demos and training data more authentic.

8. **Async-First**: Full Tokio integration enables concurrent session management, parallel workflow execution, and non-blocking I/O.

### Negative

1. **Implementation Complexity**: Three-layer architecture requires more code than a simple xdotool wrapper. The physics engine alone is 400+ lines.

2. **X11 Dependency**: Primary implementation depends on xdotool and X11, creating a migration path challenge when Wayland becomes dominant.

3. **No Semantic Understanding**: The physics engine handles movement but not element detection. AI integration (UI-TARS) is required for semantic UI understanding.

4. **Platform-Specific Gaps**: Platform abstraction traits are defined but Windows/macOS implementations are future work, creating a completeness gap.

5. **Tuning Required**: WindMouse parameters (gravity, wind, friction, tremor) require tuning for different desktop environments and use cases.

### Neutral

1. **Rust Implementation**: The engine is implemented in Rust, requiring developers to understand Rust's ownership model and async patterns.

2. **Feature Flags**: X11, Wayland, and audio support are gated behind feature flags, requiring careful build configuration.

3. **Cache Memory**: Trajectory caching improves performance but consumes memory proportional to unique movement patterns.

---

## Cross-References

- **ADR-002**: Cross-Platform Strategy - defines how the platform abstraction layer extends to Wayland, Windows, and macOS
- **ADR-003**: AI Agent Integration - defines how the automation engine integrates with AI models (UI-TARS, GPT-4V, Claude)
- **SPEC.md**: Section 4 (Automation Engine) - detailed specification of the engine's behavior
- **src/automation_engine.rs**: WindMouse 2.0 and NaturalTypingEngine implementation
- **src/ui_automation.rs**: UiAutomationEngine with gesture-based interaction

---

## Appendix A: WindMouse 2.0 Physics Model

```
Force Equation:
  F_total = F_gravity + F_wind + F_tremor + F_context

  F_gravity = direction_to_target * gravity * adaptive_strength(progress)
  F_wind    = previous_wind * decay + random_noise * wind_strength
  F_tremor  = sin(phase * frequency) * amplitude * fatigue_multiplier
  F_context = obstacle_avoidance_force + user_preference_force

  velocity = (velocity + F_total * dt) * friction
  position = position + velocity * dt
```

### Adaptive Gravity Curve

```
Gravity Strength vs Progress:
  Progress 0.0-0.1:  120%+ (strong initial pull)
  Progress 0.1-0.9:  100%  (normal gravity)
  Progress 0.9-1.0:  30-100% (gentle final approach)

  Graph:
  1.4 |  *
      |   *
  1.2 |    *
      |     *
  1.0 |      ──────────────
      |                    *
  0.5 |                     *
      |                      *
  0.3 |                       *
      +──────────────────────────
      0.0   0.1   0.5   0.9   1.0
                Progress
```

---

## Appendix B: Rust Code Example - Complete Workflow

```rust
use kvirtualstage::automation_engine::{
    AutomationEngine, WindMouseEngine, NaturalTypingEngine,
    AutomationWorkflow, WorkflowStep, StepAction, Point, MouseButton,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the automation engine
    let mut engine = AutomationEngine::new()?;

    // Configure a user profile for natural movement
    engine.set_user_profile(UserMovementProfile {
        movement_speed: 1.0,
        precision_level: 0.8,
        jitter_amount: 0.1,
        hesitation_factor: 0.2,
        fatigue_level: 0.0,
        path_curvature: 0.3,
    });

    // Define a workflow
    let workflow = AutomationWorkflow {
        name: "Calculator Demo".into(),
        description: "Open calculator and perform a calculation".into(),
        continue_on_error: false,
        steps: vec![
            WorkflowStep {
                name: "Move to calculator".into(),
                action: StepAction::MoveCursor {
                    to: Point::new(100.0, 100.0),
                },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Click calculator".into(),
                action: StepAction::Click {
                    position: Point::new(100.0, 100.0),
                    button: MouseButton::Left,
                },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Type calculation".into(),
                action: StepAction::Type {
                    text: "123 * 456".into(),
                },
                timeout: Some(Duration::from_secs(10)),
            },
        ],
    };

    // Execute the workflow
    let result = engine.execute_workflow(workflow).await?;

    println!(
        "Workflow completed: {}/{} steps in {:?}",
        result.successful_steps,
        result.total_steps,
        result.total_execution_time
    );

    Ok(())
}
```

---

## Appendix C: Performance Optimization Strategies

### Trajectory Caching

The WindMouseEngine caches computed trajectories to avoid redundant physics calculations:

```rust
// Cache key format: "startX,startY-targetX,targetY"
let cache_key = format!("{:.0},{:.0}-{:.0},{:.0}", start.x, start.y, target.x, target.y);
if let Some(cached) = self.trajectory_cache.get(&cache_key) {
    return cached.clone();  // Hit: return cached trajectory
}
// Miss: compute new trajectory and cache it
```

**Cache Policy**:
- Only cache trajectories under 1000 frames (prevents memory bloat)
- Cache key uses rounded coordinates (integer precision)
- No eviction strategy (bounded by unique movement patterns)

### Frame Timing

```rust
const DELTA_TIME: f64 = 1.0 / 60.0; // 60 FPS target

for frame in trajectory {
    self.execute_cursor_position(frame.position).await?;
    sleep(Duration::from_secs_f64(1.0 / 60.0)).await;
}
```

**Considerations**:
- Tokio's `sleep` has ~1ms granularity on most platforms
- Actual frame rate may vary slightly from 60 FPS
- For production, consider a dedicated frame scheduler

## Appendix D: Integration with UI Automation Engine

The AutomationEngine integrates with UiAutomationEngine for gesture-based interaction:

```
AutomationEngine                    UiAutomationEngine
      │                                    │
      ├── move_cursor_naturally() ───────> │
      │     (WindMouse 2.0)                │
      │                                    │
      ├── click_naturally() ─────────────> │
      │     (move + delay + click)         │
      │                                    │
      ├── type_text_naturally() ─────────> │
      │     (NaturalTypingEngine)          │
      │                                    │
      │<── gesture execution ──────────────┤
      │     (hover_click, drag, scroll)    │
```

**Gesture Types**:
- `PreciseClick`: Direct click with minimal movement
- `HoverClick`: Hover with micro-movements, then click
- `DoubleClick`: Two rapid clicks with natural interval (100-200ms)
- `RightClick`: Context menu activation
- `DragDrop`: Mouse down, natural drag, release
- `Scroll`: Mouse wheel in specified direction

---

## Document History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0 | 2026-04-03 | Phenotype Architecture Team | Initial ADR |
