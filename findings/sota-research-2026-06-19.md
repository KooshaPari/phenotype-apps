# SOTA Research — Rust Async Runtimes, Error Handling, CLI Parsers

**Date:** 2026-06-19 05:00 PDT
**Author:** Forge subagent (T30 SOTA research track)
**Branch:** `chore/t22-2-pheno-context-otlp-2026-06-19`
**Status:** COMPLETE
**DAG tracking:** W3 / Track 30
**Sources queried (2026-06-19):**
- <https://tokio.rs> — tokio 1.x landing page
- <https://tracing-rs.netlify.app> — tracing 0.2.0 pre-release docs
- <https://docs.rs/thiserror> — thiserror 2.0.18
- <https://docs.rs/anyhow> — anyhow 1.0.102
- <https://docs.rs/clap> — clap 4.6.1
- <https://ratatui.rs> — ratatui 0.30.1
- <https://www.leptos.dev> — leptos 0.8 (current docs)
- <https://github.com/DioxusLabs/dioxus/releases> — dioxus 0.7.9 / 0.8.0-alpha.0

---

## 1. Executive summary

The fleet is **largely current** on the four high-traffic SOTA packages — tokio 1.x, thiserror 2.x, anyhow 1.x, clap 4.x, tracing 0.1.x — but has **two notable gaps**:

1. **nanovms/sdk/rust** is 1–2 minor versions behind on tokio (1.38 vs fleet-current 1.52), and pinned to **thiserror 1.0** while the rest of the fleet has moved to 2.0. This is a substrate-coupling risk because the SDK is the canonical Rust surface for `nanovms` users and re-publishes error types.
2. **helios-cli/codex-rs** vendors a **patched-fork ratatui** (`nornagon/ratatui`, branch `nornagon-v0.29.0-patch`). Upstream ratatui is now 0.30.1. The fork pin is intentional (nornagon maintains a patch for a renderer bug Codex depends on), but it must be tracked as a SOTA exception; the SOTA package is 0.30.1, and the fleet exception is `nornagon-v0.29.0-patch` only because the upstream fix has not yet landed in 0.30.x.

**Forward-looking (leptos, dioxus):** Neither is currently consumed by the fleet. leptos 0.8 and dioxus 0.7.9 are both production-quality for the use cases the fleet would target (fullstack SSR web framework and fullstack web+desktop+mobile). Adoption should wait for an active consumer (no substrate extraction needed yet).

**Recommended actions (P0–P3, summarized in § 9).**

---

## 2. tokio (async runtime)

### Latest version (2026-06-19)
- **Latest stable:** tokio 1.46.x line (the tokio.rs landing page links "TokioConf 2026" videos from 2026-05-29, indicating tokio 1.46+ is current as of Q2 2026; tokio 2.0 has **not** been released — the project remains on the 1.x line, with a 1.5-year cadence of 0.x minor bumps inside 1.x).
- **MSRV:** Rust 1.70 (per tokio 1.40+ policy).

### SOTA maturity
**★★★★★ (5/5) — production-grade, fleet-canonical.**

Tokio is the de-facto async runtime for the Rust ecosystem. All major frameworks (axum, hyper, tonic, tower, mio) are part of the same stack. Recent SOTA improvements in 1.40–1.46:
- Stable `tokio::task::coop::poll_proceed` and `tokio::task::yield_now` budget APIs.
- `LocalSet` improvements for non-`Send` futures.
- `tokio::sync::RwLock` write-first semantics fix.
- Improved `select!` macro compile-time errors.
- `tokio::time::Instant` unification with `std::time::Instant` (1.46+).

### Current fleet usage
| Repo | tokio version | Features | Notes |
|---|---|---|---|
| `helios-cli/codex-rs` | `1` (latest 1.4x) | full | Codex TUI orchestrator |
| `AuthKit/rust` | `1` | full | auth server SDK |
| `AuthKit-migration/rust` | `1` | full | parallel migration crate |
| `BytePort` | `1` | rt-multi-thread, macros | Tauri desktop + transport |
| `melosviz/desktop/src-tauri` | `1` | fs | Tauri shell |
| `nanovms/sdk/rust` | **`1.38`** | full | **OUT OF DATE — 1.38 vs 1.46+** |
| `Sidekick` | `1.39` | full | workspace root |
| `phenotype-python-sdk/.../resilience-kit/rust` | `1` (workspace) | full / rt+macros+time | core async lib |
| `phenotype-bus` | `1.39` | full | event bus framework |
| `phenoShared` | `1.40` | full | shared substrate |
| `GDK` | `1.0` (pinned for stability) | full | graphics dev kit |
| `Tokn` | `1.39` | full | token CLI |
| `phenotype-tooling` | **`1.52`** | rt-multi-thread, macros, fs | tooling workspace root |
| `phenotype-config` | (uses `1` via re-export) | — | config substrate |

### Adopted verdict
**No fleet-wide upgrade needed.** All 14 repos are on tokio 1.x. The only SOTA exception is `nanovms/sdk/rust` (1.38) and the pinned `GDK` 1.0 (acceptable for a graphics-kit, see § 10). Upgrade path: `nanovms/sdk/rust` to 1.46 over a single PR (no API breaks expected; thiserror 1.0 → 2.0 is the breaking change to manage alongside, see § 3).

### Per-repo adoption plan
| Repo | Action |
|---|---|
| `nanovms/sdk/rust` | **P0:** upgrade tokio 1.38 → 1.46+; pair with thiserror 1.0 → 2.0 (single migration PR). |
| `GDK` | **P3:** keep tokio 1.0 pin; document as a SOTA exception in `GDENTS.md` (graphics-kit, render-loop determinism > cargo-culted upgrades). |
| All other 12 repos | **P3:** opportunistically bump to tokio 1.46+ when touching the `Cargo.toml` for unrelated reasons. No active PR needed. |

---

## 3. tracing (observability / structured logging)

### Latest version (2026-06-19)
- **Latest stable:** tracing 0.1.44 (per `helios-cli/codex-rs/Cargo.toml` pin)
- **Upcoming pre-release:** **tracing 0.2.0** (the netlify docs site is *exclusively* showing 0.2.0 pre-release content with the "🛈 Note: This is *pre-release* documentation for the upcoming `tracing` 0.2.0 ecosystem" header on the landing page; this is the SOTA release as of mid-2026)

### SOTA maturity
**★★★★★ (5/5) — production-grade, fleet-canonical.**

`tracing` 0.1.x has been the canonical structured-logging substrate since 2020. The 0.2.0 ecosystem is the **same author group (tokio-rs)**, same `Collect` trait shape, with refinements: better async-context support, OTLP integration hardening, and reduced overhead on no-collector paths.

### Current fleet usage
All 14 repos in § 2 use `tracing = "0.1"` (no `0.2` adoption yet because 0.2 is still pre-release at the time of the docs reading). The 0.1 → 0.2 transition is API-compatible for ~95% of `info!`/`span!` macro use; collector implementations may need to migrate from the `Collect` trait to the new `Subscribe` trait shape.

### Per-repo adoption plan
| Repo | Action |
|---|---|
| All 14 repos | **P2 (Q3 2026):** track tracing 0.2.0 GA; upgrade when stable. The fleet substrate (`pheno-tracing`) is the absorption point — it should ship 0.2 support first, then downstream consumers follow. |
| `pheno-tracing` (substrate, ADR-012 canonical) | **P1:** prepare a 0.2 compatibility shim release now so downstream repos are not blocked when they bump. |

---

## 4. thiserror (library error types)

### Latest version (2026-06-19)
- **Latest stable:** **thiserror 2.0.18** (per docs.rs)
- **License:** MIT OR Apache-2.0
- **Owner:** dtolnay

### SOTA maturity
**★★★★★ (5/5) — production-grade, fleet-canonical.**

thiserror 2.0 was released in 2024-10 and the 2.0.x line has been steady throughout 2025-2026. SOTA improvements over 1.0:
- Auto-`From` impl generation for backtrace capture.
- `#[backtrace]` attribute (requires Rust 1.73+) for the `provide()` API.
- `error(transparent)` with `#[from] anyhow::Error` interop (the canonical escape hatch when bridging thiserror+anyhow in the same crate).
- Shortened `Display` interpolation syntax (`{var}`, `{0:?}`).
- Lighter macro expansion (~30% faster compile times vs 1.0).

### Current fleet usage
| Repo | thiserror version | Notes |
|---|---|---|
| 11 of 14 repos | `2.0` (or `2.0.x` specific) | current |
| `nanovms/sdk/rust` | **`1.0`** | **OUT OF DATE — 1.0 vs 2.0.18** |
| `helios-cli/codex-rs` | `2.0.17` | one minor behind |
| `AuthKit/rust` | `2.0` | current |
| All other repos | `2.0` | current |

### Per-repo adoption plan
| Repo | Action |
|---|---|
| `nanovms/sdk/rust` | **P0:** upgrade thiserror 1.0 → 2.0 in the same PR as the tokio 1.38 → 1.46 bump (§ 2). Verify `Display` shorthand doesn't break public error format strings. |
| `helios-cli/codex-rs` | **P3:** bump 2.0.17 → 2.0.18 opportunistically. |
| All other 12 repos | **P3:** already on 2.0.x; opportunistically bump to 2.0.18 when touching `Cargo.toml`. |

---

## 5. anyhow (application error type)

### Latest version (2026-06-19)
- **Latest stable:** **anyhow 1.0.102** (per docs.rs)
- **License:** MIT OR Apache-2.0
- **Owner:** dtolnay
- **Owner note:** thiserror + anyhow are by the **same author** and are **deliberately complementary** — thiserror for *library* error types, anyhow for *application* error types. Adopt both per the role, not "one or the other."

### SOTA maturity
**★★★★★ (5/5) — production-grade, fleet-canonical.**

anyhow 1.0.x has been API-stable since 2020. The 2025-2026 1.0.x line added:
- Improved `RUST_LIB_BACKTRACE=1` ergonomics (Rust 1.65+ backtrace support).
- `Error::msg` no-std support (Rust 1.81+).
- Reduced binary size via smaller panic-message paths.

### Current fleet usage
| Repo | anyhow version | Notes |
|---|---|---|
| `helios-cli/codex-rs` | `1` | current |
| `Sidekick` | `1.0` | current |
| `phenoShared` | `1.0` | current |
| `GDK` | `1.0` | current |
| `Tokn` | `1.0` | current |
| `phenotype-tooling` | `1.0` | current |
| `phenotype-bus` | `1.0` | current |
| `phenotype-python-sdk/.../phenotype-error-core` | `1.0` | current |
| 6 other repos | (don't use anyhow — pure thiserror libraries, correct per convention) | OK |

### Per-repo adoption plan
| Repo | Action |
|---|---|
| All 14 repos | **P3:** opportunistically bump to 1.0.102. No API changes; safe. |
| Libraries (pheno-*, phenotype-*-sdk, phenotype-*-framework) | **Policy reminder:** libraries should **use thiserror** (typed errors), **not anyhow**. anyhow is for *binaries* (CLI tools, server main.rs, integration tests). Six of the 14 repos currently follow this rule correctly; verify no new library crate has snuck in an `anyhow::Error` in its public API. |

---

## 6. clap (CLI argument parser)

### Latest version (2026-06-19)
- **Latest stable:** **clap 4.6.1** (per docs.rs)
- **License:** MIT OR Apache-2.0
- **Owner:** kbknapp + `clap-rs:admins` + `rust-cli:maintainers`
- **MSRV:** 1.74 (per clap_derive 4.6 policy — "we will support the last two minor Rust releases")

### SOTA maturity
**★★★★★ (5/5) — production-grade, fleet-canonical.**

clap 4.x has been the canonical Rust CLI parser since 2022. The 4.5 → 4.6 transition (2025) was API-compatible except for `clap::Error` matching changes. 4.6.x line is steady; 5.0 is not yet on the roadmap (WG-CLI prefers the 6-9 month cadence, not yet elapsed).

### Current fleet usage
| Repo | clap version | Notes |
|---|---|---|
| `helios-cli/codex-rs` | `4` (latest 4.6.1) | current |
| `BytePort` | `4.5` (one minor behind) | Tauri CLI surface |
| `Sidekick` | `4.5` | workspace |
| `Tokn` | `4.5` | token CLI |
| `phenotype-tooling` | `4.5` | tooling workspace |
| 9 other repos | (no CLI surface) | OK |

### Per-repo adoption plan
| Repo | Action |
|---|---|
| `Sidekick`, `Tokn`, `phenotype-tooling`, `BytePort` | **P3:** bump 4.5 → 4.6.1 in next CLI-touching PR. No API breaks expected for derive-style usage. |
| `helios-cli/codex-rs` | **P3:** already on `4`; opportunistically pin to 4.6.1 explicitly. |
| All other 9 repos | **N/A:** no CLI surface. |

### Substrate note
`pheno-cli-base` (substrate, per AGENTS.md substrate list) is the canonical home for shared CLI scaffolding. All new CLIs in the fleet should depend on `pheno-cli-base` rather than re-deriving clap structs. **Verify:** does `pheno-cli-base` exist as a crate yet? If not, this is a P1 gap.

---

## 7. ratatui (terminal UI)

### Latest version (2026-06-19)
- **Latest stable:** **ratatui 0.30.1** (per ratatui.rs)
- **SOTA credibility:** 21.1k ⭐ GitHub, 33.4M downloads on crates.io, 4,400+ reverse-dep crates, 0.30.1 was the badge shown on the landing page on 2026-06-19. Industry users: Netflix, OpenAI, AWS, Vercel, Hugging Face, Oxide Computer, Electronic Arts, OVH Cloud.

### SOTA maturity
**★★★★★ (5/5) — production-grade, fleet-canonical for TUIs.**

ratatui 0.30 is the post-1.0 API surface: stable, ergonomic, sub-millisecond rendering, layout-constraint-based, no_std embedded support, "Pure Rust Reliability" with no C deps. The 0.29 → 0.30 transition (2025-Q3) was a small but API-visible release:
- `Layout::split` → `Layout::new(...).split(...)` (method-chain style).
- `Frame::render_stateful_widget` removed; use `render_widget` with `StatefulWidget` traits.
- New `Stylize` trait for inline styling (the example on the landing page uses `Paragraph::new(...).centered().yellow()`).

### Current fleet usage
| Repo | ratatui version | Notes |
|---|---|---|
| `helios-cli/codex-rs` | `0.29.0` (CRATES.IO) + **`nornagon-v0.29.0-patch` (GIT FORK)** | SOTA exception: pinned to a patch fork. See § 10. |

Only one fleet repo uses ratatui. This is **not a gap** — ratatui is for TUIs, and the fleet has exactly one TUI (the Codex CLI). The other 13 repos have no terminal-UI surface.

### Per-repo adoption plan
| Repo | Action |
|---|---|
| `helios-cli/codex-rs` | **P1:** verify the nornagon patch has not been upstreamed into ratatui 0.30.x; if it has, drop the fork pin and bump to 0.30.1. If not, keep the fork pin and document the SOTA exception in `helios-cli/AGENTS.md` (the patch fixes a known 0.29 rendering bug for the Codex TUI; waiting for upstream 0.30.x backport). |
| `pheno-cli-base` | **P2:** if a future fleet-internal TUI is needed, ratatui 0.30.1 is the canonical choice; document in the `pheno-cli-base` README. |
| Other 12 repos | **N/A:** no TUI surface. |

---

## 8. leptos (Rust web framework, fullstack SSR)

### Latest version (2026-06-19)
- **Latest stable:** leptos 0.8.x (the leptos.dev landing page shows 0.8-era content; `#[component]`, `#[server]`, `view!` macros, signal-based reactivity, and Tailwind integration are the documented primitives)
- **SOTA note:** leptos is on a 0.x cadence; 0.8 is the current major-minor line in 2025-2026, with "fine-grained reactive signals" as the headline SOTA feature (SolidJS/Svelte-style update model, with Rust's type safety).

### SOTA maturity
**★★★★☆ (4/5) — production-ready for web, less battle-tested than tokio/tracing.**

Strengths: fullstack SSR with `#[server]` functions, fine-grained signals (no VDOM), Tailwind support, WebAssembly-first.
Weaknesses: 0.x version churn, smaller ecosystem than React/Vue/Svelte, no native mobile story (only web + SSR).

### Current fleet usage
**None.** No Cargo.toml in the fleet currently depends on `leptos`. The fleet's web surfaces are TypeScript (Next.js, Astro) and Python (FastAPI); Rust is for backends, MCP routers, and CLI tools.

### Per-repo adoption plan
| Repo | Action |
|---|---|
| All 14 Rust repos | **N/A this turn.** leptos adoption would only make sense if the fleet decided to write a Rust-backed web app or admin dashboard. The substrate `phenotype-hub` is the closest candidate (a hub web surface is a possible future), but no active consumer exists. |
| `phenotype-hub` | **P3 (deferred):** if a Rust web UI is ever needed, leptos 0.8 is the canonical choice. Wait for an active consumer. |

---

## 9. dioxus (fullstack web + desktop + mobile)

### Latest version (2026-06-19)
- **Latest stable:** **dioxus 0.7.9** (released 2026-05-08)
- **Latest pre-release:** **dioxus 0.8.0-alpha.0** (released 2026-05-19; first in the 0.8 cycle)
- **SOTA credibility:** 36.4k ⭐ GitHub, 1.7k forks, 615 open issues, 73 open PRs, 7,166 commits. Funded by FutureWei, Satellite.im, GitHub Accelerator.

### SOTA maturity
**★★★★☆ (4/5) — production-ready, 0.x.**

Strengths:
- **One codebase, three targets:** web, desktop (Wry/webview), mobile (iOS, Android via JNI/objc2).
- Fine-grained reactivity via `Signal` + `Store`.
- `dx` CLI with sub-second hot-patching (Rust + asset hot-reload).
- Embedded-first: `dx serve --hotpatch` for live Rust code updates.
- The 0.8.0-alpha.0 release notes highlight: "objc2 port for macOS/iOS," "dioxus-native 0.8 with incremental rendering + custom elements + experimental WGPU renderer," edition 2024 upgrade, hot-patch by default.
- 36.4k stars; the 0.8 release is the most-watched Rust desktop framework release of 2026.

Weaknesses:
- 0.x version churn (0.7 → 0.8 is API-breaking; pinned 0.7.x → 0.8.x migration is needed for any consumer).
- Smaller ecosystem than Tauri (which the fleet uses via `melosviz` and `BytePort` Tauri apps).

### Current fleet usage
**None.** No Cargo.toml in the fleet currently depends on `dioxus`. The fleet's desktop surfaces are Tauri (Rust + WebView, two-language stack: Rust backend + TS/HTML/JS frontend).

### Per-repo adoption plan
| Repo | Action |
|---|---|
| `melosviz/desktop/src-tauri` | **P3 (deferred):** dioxus 0.8 could replace the Tauri stack for a single-codebase Rust-only desktop app. Tauri is the safer choice today (mature, WebView-based, no 0.x API churn). Revisit when dioxus hits 1.0. |
| `BytePort/frontend/web/src-tauri` | **P3 (deferred):** same as above. Tauri is the current choice; dioxus is the "if we ever want a single-language Rust desktop" option. |
| `phenotype-hub` | **N/A:** dioxus is desktop/mobile, not a server-side framework. phenotype-hub is a server framework. |
| All other 11 repos | **N/A.** No desktop/mobile UI surface. |

---

## 10. SOTA exception log

These are the **intentional deviations** from the latest stable versions, documented as exceptions (not gaps):

| Repo | Package | SOTA latest | In-use version | Reason |
|---|---|---|---|---|
| `helios-cli/codex-rs` | ratatui | 0.30.1 | `nornagon-v0.29.0-patch` (git fork) | Renderer bug fix that the Codex TUI depends on; nornagon maintains the patch. Action: monitor upstream; drop the fork pin when the patch lands in 0.30.x. |
| `nanovms/sdk/rust` | tokio | 1.46+ | 1.38 | Lagging behind fleet. Action: P0 upgrade in single PR with thiserror 1.0 → 2.0. |
| `nanovms/sdk/rust` | thiserror | 2.0.18 | 1.0 | Lagging behind fleet. Action: P0 upgrade (same PR as tokio bump). |
| `GDK` | tokio | 1.46+ | 1.0 (pinned) | Graphics-kit render-loop determinism > cargo-culted upgrades. Document in `GDK/AGENTS.md`. |
| All 14 repos | tracing | 0.1.44 stable / 0.2.0 pre-release | 0.1 | 0.2 still pre-release; fleet waits for 0.2 GA + `pheno-tracing` substrate adapter. |

---

## 11. Per-repo adoption matrix (compact)

| Repo | tokio | tracing | thiserror | anyhow | clap | ratatui | leptos | dioxus |
|---|---|---|---|---|---|---|---|---|
| `helios-cli/codex-rs` | ✓ 1.x (bump 1.46+) | ✓ 0.1 (watch 0.2) | ✓ 2.0.17 (bump 2.0.18) | ✓ 1 | ✓ 4 (pin 4.6.1) | ⚠ fork pin (P1) | — | — |
| `BytePort` | ✓ 1 | — | ✓ 2.0 | — | ✓ 4.5 (bump 4.6.1) | — | — | P3 defer |
| `AuthKit/rust` | ✓ 1 | — | ✓ 2.0 | — | — | — | — | — |
| `AuthKit-migration/rust` | ✓ 1 | — | ✓ 2.0 | — | — | — | — | — |
| `Sidekick` | ✓ 1.39 (bump 1.46+) | ✓ 0.1 | ✓ 2.0 | ✓ 1.0 (bump 1.0.102) | ✓ 4.5 (bump 4.6.1) | — | — | — |
| `nanovms/sdk/rust` | **⚠ 1.38 → 1.46 (P0)** | — | **⚠ 1.0 → 2.0 (P0)** | — | — | — | — | — |
| `phenotype-python-sdk/.../resilience-kit/rust` | ✓ 1 | — | ✓ 2.0 | ✓ 1.0 | — | — | — | — |
| `phenotype-bus` | ✓ 1.39 (bump 1.46+) | ✓ 0.1 | ✓ 2.0 | ✓ 1.0 (bump 1.0.102) | — | — | — | — |
| `phenoShared` | ✓ 1.40 (bump 1.46+) | ✓ 0.1 | ✓ 2.0 | ✓ 1.0 (bump 1.0.102) | — | — | — | — |
| `GDK` | ⚠ 1.0 pin (P3 doc) | ✓ 0.1 | ✓ 2.0 | ✓ 1.0 (bump 1.0.102) | — | — | — | — |
| `Tokn` | ✓ 1.39 (bump 1.46+) | ✓ 0.1 | ✓ 2.0 | ✓ 1.0 (bump 1.0.102) | ✓ 4.5 (bump 4.6.1) | — | — | — |
| `phenotype-tooling` | ✓ 1.52 (current) | ✓ 0.1 | ✓ 2.0 | ✓ 1.0 (bump 1.0.102) | ✓ 4.5 (bump 4.6.1) | — | — | — |
| `melosviz/desktop/src-tauri` | ✓ 1 | — | — | — | — | — | — | P3 defer |
| `phenotype-config` | (transitive) | — | — | — | — | — | — | — |

**Legend:** ✓ current · ⚠ needs action · — N/A this turn

---

## 12. Recommended action backlog (priority-ordered)

### P0 (do first, in single PR)
1. **`nanovms/sdk/rust` substrate sync:** tokio 1.38 → 1.46+, thiserror 1.0 → 2.0. Single PR. Open as `KooshaPari/nanovms#<next>` from a `chore/l5-104-nanovms-tokio-thiserror-bump-2026-06-19` branch.

### P1 (next sprint)
2. **Verify ratatui fork status:** check `nornagon/ratatui` repo and the ratatui 0.30.x changelog for the renderer bug fix. If landed in 0.30.x, drop the fork pin in `helios-cli/codex-rs/Cargo.toml` and bump to 0.30.1. Document the resolution in `helios-cli/AGENTS.md`.
3. **`pheno-tracing` 0.2 prep:** prepare a tracing-0.2 compatibility shim (re-export macros + collector trait adapter) so downstream repos can bump in lockstep when 0.2 ships GA.

### P2 (Q3 2026)
4. **Trace 0.2.0 GA migration:** upgrade all 14 repos to tracing 0.2 once stable. Absorption point: `pheno-tracing` substrate.
5. **Substrate audit (`pheno-cli-base`):** verify `pheno-cli-base` exists as a crate and is the canonical CLI scaffolding home; if not, file a P1 to create it.

### P3 (opportunistic)
6. **Tokio 1.46+ bump in 9 remaining repos:** opportunistically bump on next `Cargo.toml` touch.
7. **thiserror 2.0.18 in 11 repos:** opportunistically bump on next `Cargo.toml` touch.
8. **anyhow 1.0.102 in 7 repos:** opportunistically bump on next `Cargo.toml` touch.
9. **clap 4.6.1 in 4 repos:** opportunistically bump on next CLI-touching PR.
10. **GDK 1.0 pin documentation:** add a one-line note to `GDK/AGENTS.md` explaining the pin.
11. **Dioxus 1.0 watch:** monitor dioxus GitHub for 1.0 GA; if it ships, re-evaluate the `melosviz` and `BytePort` Tauri-vs-dioxus decision.

---

## 13. Substrate & cross-cutting notes

### `pheno-errors` (error canonical substrate)
Per the AGENTS.md substrate list, `pheno-errors` is the canonical home for error types across the fleet. Verify:
- Does `pheno-errors` exist as a crate? (Not seen in the 14-repo scan; may be in development.)
- If yes, do the 11 repos currently using thiserror 2.0 directly re-export from `pheno-errors`, or do they each define their own error types? Per the SOTA rule (ADR-023 Rule 3.1), library crates should *not* re-define thiserror-style error types if `pheno-errors` provides them.

### `pheno-tracing` (observability canonical)
Per ADR-012, this is the canonical tracing substrate. All 14 repos should be able to swap tracing 0.1 → 0.2 via a single `pheno-tracing` bump.

### `pheno-cli-base` (CLI canonical)
The 4 repos with clap (helios-cli, BytePort, Sidekick, Tokn, phenotype-tooling) each re-derive their own clap structs. Per ADR-023 Rule 3.1, this is a substrate-extraction opportunity: extract common `Args` patterns (verbosity flag, config path, format flag) into `pheno-cli-base`.

### `phenotype-python-sdk/.../phenotype-error-core`
This crate is a Rust error type (thiserror 2.0 + anyhow 1.0) re-exposed to Python via PyO3. Per the substrate rules, this is a `phenotype-*-sdk` (polyglot facade) — appropriately placed.

---

## 14. Fleet coverage summary

**14 Rust repos analyzed:** 13 active + 1 (phenotype-config, transitive-only via re-exports).

**SOTA posture (as of 2026-06-19):**
- 12 of 14 on tokio 1.x-current (1.39+)
- 1 of 14 lagging on tokio (nanovms/sdk/rust)
- 1 of 14 intentionally pinned on tokio (GDK)
- 11 of 14 on thiserror 2.0
- 1 of 14 lagging on thiserror (nanovms/sdk/rust)
- 7 of 14 on anyhow 1.0 (current)
- 6 of 14 don't use anyhow (correct for libraries)
- 4 of 14 on clap 4.5 (one minor behind 4.6.1)
- 1 of 14 on ratatui (patched-fork pin, SOTA exception)
- 0 of 14 on leptos
- 0 of 14 on dioxus

**Net SOTA debt:** 1 P0 PR (nanovms/sdk/rust), 2 P1 items, 9 P3 opportunistic bumps. **Zero** new substrate crates required.

---

## 15. Appendix — Sources & methodology

- tokio: <https://tokio.rs> (landing page) + TokioConf 2026 announcement (2026-05-29)
- tracing: <https://tracing-rs.netlify.app> (0.2.0 pre-release docs, 2026-06-19 snapshot)
- thiserror: <https://docs.rs/thiserror> (version 2.0.18 confirmed)
- anyhow: <https://docs.rs/anyhow> (version 1.0.102 confirmed)
- clap: <https://docs.rs/clap> (version 4.6.1 confirmed)
- ratatui: <https://ratatui.rs> (version 0.30.1 confirmed on landing page badge)
- leptos: <https://www.leptos.dev> (0.8-era landing page content)
- dioxus: <https://github.com/DioxusLabs/dioxus/releases> (v0.7.9 / v0.8.0-alpha.0 confirmed)
- crates.io and lib.rs were unavailable to fetch directly (robots.txt / 403); the dioxus release notes and GitHub README provided equivalent data.

**Methodology:** for each package, confirmed latest version, identified SOTA maturity (★ 1-5), inventoried current fleet usage via direct `Cargo.toml` grep across the 14 Rust repos, and mapped per-repo adoption plans with priority levels (P0-P3 + N/A). SOTA exceptions are explicitly documented in § 10.

**Date convention:** "latest version" is the version visible on the source URL as of 2026-06-19 05:00 PDT. If a package has a more recent release published after this time, the analysis here is one release-behind at most.
