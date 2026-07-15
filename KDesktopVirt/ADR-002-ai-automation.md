# ADR-002: AI-Native UI Automation with UI-TARS Integration

Date: 2025-04-04

## Context

KDesktopVirt requires a UI automation system capable of controlling desktop applications programmatically. We must choose between traditional selector-based automation and AI-powered approaches.

Traditional automation uses explicit selectors:

```python
# Traditional selector-based automation (brittle)
driver.find_element(By.ID, "submit-button").click()  # Breaks if ID changes
driver.find_element(By.XPATH, "//input[@name='username']").send_keys("admin")
```

AI-powered automation uses computer vision and reasoning:

```python
# AI-powered automation (resilient)
screenshot = capture_screen()
analysis = model.predict(screenshot, "Click the blue submit button")
click(analysis.coordinates)  # Adapts to UI changes
```

The landscape of AI agents for UI automation has evolved rapidly:

| Approach | Year | Capabilities | Availability |
|----------|------|--------------|--------------|
| Sikuli | 2010 | Image matching | Open source |
| Selenium + Heuristics | 2015 | DOM + ML | Open source |
| GPT-4V + Scripts | 2023 | Vision + LLM | Cloud API |
| Claude Computer Use | 2024 | Vision + Control | Cloud API |
| UI-TARS | 2025 | End-to-end trained | Model weights |

## Decision

We will adopt a hybrid automation architecture with UI-TARS as the primary AI engine, supplemented by traditional X11 control for deterministic operations.

### Architecture

```
Automation Engine Architecture:

┌─────────────────────────────────────────┐
│  Automation Interface Layer             │
│  ├─ Normal Scripting (JSON/YAML)       │
│  ├─ MCP Live Scripting (real-time)     │
│  ├─ ACI Agent Interface (autonomous)   │
│  └─ Desktop Recording (with capture)   │
├─────────────────────────────────────────┤
│  AI Reasoning Layer (UI-TARS)          │
│  ├─ Screenshot analysis                  │
│  ├─ Element detection                    │
│  ├─ Action prediction                    │
│  └─ Task planning                        │
├─────────────────────────────────────────┤
│  Control Layer                           │
│  ├─ X11/xdotool (mouse, keyboard)      │
│  ├─ wmctrl (window management)           │
│  └─ xwininfo (element queries)           │
├─────────────────────────────────────────┤
│  Application Layer (desktop session)     │
└─────────────────────────────────────────┘
```

### Modes of Operation

Four automation modes serve different use cases:

#### 1. Normal Scripting

Sequential execution with explicit steps:

```json
{
  "steps": [
    {"action": "launch_app", "params": {"name": "firefox"}},
    {"action": "wait", "params": {"duration": 2000}},
    {"action": "click_element", "params": {"description": "New Tab button"}},
    {"action": "type", "params": {"text": "https://example.com"}},
    {"action": "press_key", "params": {"key": "Return"}}
  ]
}
```

Use case: Predictable workflows, CI/CD integration, regression testing.

#### 2. MCP Live Scripting

Real-time tool execution via Model Context Protocol:

```javascript
// MCP tool call
{
  "name": "kdesktopvirt_click_element",
  "arguments": {
    "session_id": "session-123",
    "description": "Submit button in the payment form"
  }
}
```

Use case: Interactive agent control, Claude Desktop integration.

#### 3. ACI Agent Interface

Autonomous agent control with goal-based tasking:

```json
{
  "goal": "Generate monthly report from spreadsheet data",
  "constraints": {
    "max_steps": 100,
    "allowed_apps": ["libreoffice", "firefox"],
    "timeout_seconds": 300
  }
}
```

Use case: Complex multi-step tasks, research automation, data extraction.

#### 4. Desktop Recording

Video capture with synchronized automation:

```json
{
  "record": true,
  "automation": {
    "mode": "normal",
    "script_id": "demo-workflow"
  },
  "output": {
    "format": "mp4",
    "quality": "high",
    "fps": 30
  }
}
```

Use case: Documentation, demonstrations, audit trails.

### UI-TARS Integration

UI-TARS is integrated as a modular component:

```rust
pub struct UITarsEngine {
    model: ModelInstance,
    config: AutomationConfig,
}

impl UITarsEngine {
    /// Analyze screenshot and predict next action
    pub async fn predict_action(
        &self,
        screenshot: &Image,
        task: &str,
        history: &[Action],
    ) -> Result<Action, Error> {
        // Vision encoding + LLM reasoning + action decoding
    }
    
    /// Execute predicted action via X11
    pub async fn execute_action(
        &self,
        action: Action,
        display: &Display,
    ) -> Result<(), Error> {
        // Convert to xdotool/wmctrl commands
    }
}
```

**Key features**:
- Self-healing selectors (adapts to UI changes)
- Natural language task description
- Multi-step reasoning
- Confidence scoring for validation

## Status

Accepted

## Consequences

### Positive

- **Resilient automation**: Adapts to UI changes without selector updates
- **Natural language interface**: Non-developers can create automation
- **Reduced maintenance**: No brittle XPath/CSS selectors
- **Future-proof**: Improves as AI models advance

### Negative

- **Inference latency**: 500-2000ms per action (vs <50ms for scripted)
- **Model dependency**: Requires UI-TARS or similar model availability
- **Resource overhead**: GPU recommended for real-time inference
- **Non-deterministic**: May produce different results for same input

### Mitigations

| Concern | Mitigation |
|---------|------------|
| Latency | Cache common predictions; hybrid with scripted for hot paths |
| Availability | Support multiple models (UI-TARS, GPT-4V, Claude) |
| Resources | Optional CPU inference; cloud API fallback |
| Determinism | Confidence threshold; retry with escalation |

### Neutral

- **Training data**: Benefits from proprietary interaction logs
- **Versioning**: Model versions may produce different results

## Alternatives Considered

### Pure Selector-Based (Selenium/Puppeteer)

Rejected due to:
- Brittleness to UI changes
- High maintenance burden
- No desktop application support

### GPT-4V via API

Rejected as primary due to:
- Cloud dependency (no offline)
- API costs at scale
- Rate limiting
- Kept as fallback option

### Claude Computer Use

Rejected as primary due to:
- API-only (no local deployment)
- Session state externalized
- Rate limiting
- Kept as alternative integration

### Proprietary Training

Deferred due to:
- High training cost
- Data collection requirements
- Revisit if UI-TARS proves insufficient

## Related Decisions

- ADR-001: Container-Based Desktop Virtualization
- ADR-003: MCP as Primary Integration Interface

## References

1. Yao, D., et al. (2025). "UI-TARS: Vision-Language Model for GUI Agents". ByteDance Research.
2. OpenAI. (2023). "GPT-4V(ision) System Card". OpenAI Research.
3. Anthropic. (2024). "Claude 3.5 Sonnet Model Card". Anthropic Documentation.
4. Shi, W., et al. (2017). "Deep Learning for GUI Testing". ASE.
