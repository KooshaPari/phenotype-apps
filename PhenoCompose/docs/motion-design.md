# Motion Design — PhenoCompose

## Principles

PhenoCompose motion is **purposeful, restrained, and functional**. Every
animation serves a user goal — orienting, confirming, or transitioning —
rather than decoration.

| Principle | Rationale |
|-----------|-----------|
| **Fast** — all animations complete in ≤300ms | User waiting on deployment/reconfiguration; motion must not delay action. |
| **Flat** — favour opacity/transform, avoid scaling | Deployment UIs render container topology; scale animations create misleading spatial metaphors for abstract infrastructure. |
| **Determinate** — progress indicates duration | Deployments have real completion times; indeterminate spinners used only when duration is genuinely unknown. |
| **Accessible** — respect `prefers-reduced-motion` | Operators with vestibular disorders must not be excluded. |

## Duration Tokens

| Token | Value | Use |
|-------|-------|-----|
| `--motion-instant` | 0ms | State toggles (theme, language) |
| `--motion-fast` | 100ms | Hover, focus, micro-interactions |
| `--motion-normal` | 200ms | Panel open/close, status transitions |
| `--motion-slow` | 300ms | Page transitions, modal enter/exit |
| `--motion-deploy` | dynamic | Deployment progress bar (matches real ETA from backend) |

## Easing Tokens

| Token | Curve | Use |
|-------|-------|-----|
| `--ease-in` | `cubic-bezier(0.4, 0, 1, 1)` | Exit animations (fade out, slide out) |
| `--ease-out` | `cubic-bezier(0, 0, 0.2, 1)` | Entry animations (fade in, slide in) |
| `--ease-in-out` | `cubic-bezier(0.4, 0, 0.2, 1)` | Panel expand/collapse, status badge transitions |

## Key Scenes

### 1. Composition status transitions

When a composition transitions between `pending → running` or
`running → failed`, the status badge animates:

- **pending → running**: `--motion-fast` fade from yellow to green with a 4px
  ambient glow that resolves to a 1px solid border.
- **running → failed**: `--motion-normal` shake (translateX ±3px, 3 cycles)
  before settling on the failed state. The shake duration is 150ms so it
  reads as urgency, not error.
- Disabled when `prefers-reduced-motion: reduce` — status colour changes
  instantly with no glow or shake.

### 2. Panel expand / collapse

The deployment detail panel uses a height-based expand:

```
panel.expand(200ms ease-in-out, max-height: 0 → max-height: 2000px)
```

- Content opacity fades in over the last 100ms so the panel feels
  open before the text is visible.
- `overflow: hidden` on the container during animation; after transition
  the overflow resets to `auto` for scrollable content.

### 3. Deployment progress bar

The primary deployment progress bar:

- **Determinate mode** by default — width transitions to the backend-reported
  percentage at `--motion-normal` speed.
- **Indeterminate mode** only when the backend returns `eta: null`.
- Striped gradient that pulses left-to-right at 1s intervals, 50% opaque
  white over the primary colour.
- On completion, the bar fills to 100% over 300ms, then a check-mark icon
  fades in over 100ms.

### 4. Toast notifications

Transient success/error toasts:

- **Entry**: slide in from bottom-right (translateY: 40px → 0, 200ms ease-out)
- **Dismiss**: fade out (opacity: 1 → 0, 150ms ease-in)
- **Auto-dismiss**: success toasts dismiss after 4s; error toasts persist
  until manual dismiss.
- Stack: max 3 visible; overflow queues with a 100ms staggered delay.

## Implementation

Motion tokens are exposed as CSS custom properties on `:root`:

```css
:root {
  --motion-instant: 0ms;
  --motion-fast: 100ms;
  --motion-normal: 200ms;
  --motion-slow: 300ms;

  --ease-in: cubic-bezier(0.4, 0, 1, 1);
  --ease-out: cubic-bezier(0, 0, 0.2, 1);
  --ease-in-out: cubic-bezier(0.4, 0, 0.2, 1);
}

@media (prefers-reduced-motion: reduce) {
  :root {
    --motion-instant: 0ms;
    --motion-fast: 0ms;
    --motion-normal: 0ms;
    --motion-slow: 0ms;
  }
}
```

All animations use `will-change: transform, opacity` on the animating
element to encourage GPU compositing. Avoid `will-change: auto` for
animations — it defeats the purpose.

## Testing

| Check | Criterion |
|-------|-----------|
| Visual regression | Playwright snapshot diff: each key scene captured at start, mid, and end of animation |
| Reduced motion | Verify all animations collapse to 0ms when `prefers-reduced-motion: reduce` is active |
| Duration limits | No animation exceeds 300ms (measured via `performance.now()` in Playwright) |
| Stack overflow | 10 rapid-fire toast dispatches: max 3 visible, no layout thrash |
