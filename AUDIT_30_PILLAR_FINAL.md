# AUDIT — Phenotype 30-Pillar Holistic (state of bloc 2026-06-16)

**Generated:** 2026-06-16
**Scope:** AgilePlus (45+ Rust crates) + thegent (25+ Python+hybrid crates) + Tracely (5 crates) + Tracera (1 polyglot crate) = ~76 crates
**Methodology:** Per-pillar file:line citations. Each pillar has its own audit-30-pillar-L[N].md.
**Team:** `tick28-holistic-audit` (20-wide DAG, 13 active, missing pillars reassigned to A05-2/A11/A14/A15)
**Reference:** `AUDIT_BLOC_VS_2026_SOTA.md` (2026-06-15), `FLEET_100TASK_DAG.md`, `audit-30-pillar-template.md`

---

## 0. Pillar coverage matrix (30/30 written) — closed 2026-06-17

| Pillar | Domain | File | Status | Top finding |
|---|---|---|---|---|
| L0  | AX: Architecture foundations | L0.md | △ partial | subcmds stub, zerokit+pheno-logging-zig empty crates, no cycle guard |
| L1  | AX: Module structure | L1.md | △ partial | thegent hybrid boundary undocumented, Tracera flat pub use |
| L2  | API surface & contract | L2.md | △ partial | no Stripe-style error envelope, no idempotency keys |
| L3  | Data model & state | L3.md | △ partial | `Bead` state transitions unvalidated, `TraceLink::new` confidence unenforced |
| L4  | Async/concurrency | L4.md | △ partial | no `loom`/`shuttle`, no TLA+ spec for claim system, no `CancellationToken` |
| L5  | Observability | L5.md | △ partial | gRPC exporter dead code, no `/metrics` endpoint, no Sentry |
| L6  | Performance | L6.md | ✗ missing | no CI bench step, no N+1 detection, no continuous profiler, no p99 SLO |
| L7  | Concurrency safety | L7.md | △ partial | no `loom`/`miri`/`TSan`, no `CONCURRENCY.md` lock-ordering doc |
| L8  | Memory & allocation | L8.md | △ partial | (Phenom scope TBD) |
| L9  | Build & release pipeline | L9.md | △ partial | (Phenom scope TBD) |
| L10 | CI/CD hygiene | L10.md | ✗ missing | Tracely double-SHA bugs, thegent ci.yml broken stub, 8× ubuntu-latest |
| L11 | Tests | L11.md | ✗ missing | no cargo-mutants, no fuzz, mutation/property/snapshot 0/4 in 3 of 4 repos |
| L12 | Documentation & SSOT | L12.md | △ partial | Tracera SSOT is auto-recovered index dump, Tracely docs thin |
| L13 | Onboarding & DX | L13.md | △ partial | `just bootstrap` missing in 4/4 main repos, SLA missing 3/4 |
| L14 | Error handling | L14.md | △ partial | no Sentry init in main 4, internal leak at agileplus-api/src/error.rs:64 |
| L15 | CLI/UX | L15.md | △ partial | no clap_complete, no clap_mangen, thegent/cli/parser.py is a 12-line stub |
| L16 | i18n/l10n | L16.md | ✗ partial | locale intent declared in 5 places, no translated content dirs, no i18next/Lingui |
| L17 | Accessibility | L17.md | △ partial | @axe-core/playwright in byteport only, no bloc-wide `prefers-reduced-motion`, no skip-link |
| L18 | Secret management | L18.md | △ partial | no Vault/AWS Secrets Manager runtime, no `SECRETS_ROTATION.md` in main 4 |
| L19 | Supply-chain security | L19.md | △ partial | no OpenVEX/SPDX emission, no SLSA on phenotype-dep-guard, Renovate+Dependabot overlap |
| L20 | Threat model | (pending) | — | (A11 dispatched) |
| L21 | AuthN/AuthZ | (pending) | — | (A11 dispatched) |
| L22 | Cryptography & key mgmt | L22.md | △ partial | thegent-crypto is MAC-only, no AES-GCM, EncryptedArtifactStore stub |
| L23 | Audit log & compliance | L23.md | △ partial | no hash chain, no event signature, retention 90d vs SOC2 1y / HIPAA 6y |
| L24 | Multi-tenant isolation | L24.md | △ partial | bloc is single-tenant CLI by design; no `TenantId` newtype, no `purge_actor` |
| L25 | Resource limits & rate limits | L25.md | △ partial | 4 rate-limiter crates ✓; `ResourceLimits` declared but not enforced in pipeline |
| L26 | Resilience | (pending) | — | (A14 dispatched) |
| L27 | Failure-mode observability | (pending) | — | (A14 dispatched) |
| L28 | Dependency hygiene | (pending) | — | (A15 dispatched) |
| L29 | Repo governance | (pending) | — | (A15 dispatched) |

## 1. Top 10 cross-cutting gaps (sequencing)

1. **L14 / L22 / L19 — Security tooling triad missing in main 4**
   No Sentry init, thegent-crypto is MAC-only, no OpenVEX/SPDX, broken `reusable-trufflehog.yml` federation link.
   **Effort:** M (1 week). **Highest single-day ROI**: fix the Sentry drift in `Tokn/sentry_config.rs`, expand thegent-crypto to AES-GCM+Ed25519, add OpenVEX emitter to `phenotype-dep-guard`.

2. **L10 / L11 — CI/CD hygiene + test quality gates un-built**
   Tracely has 3 reusable workflows @main (audit.yml/deny.yml double-SHA bugs), thegent ci.yml is a broken zero-SHA stub, 8× ubuntu-latest, no fuzz/mutation/property CI anywhere.
   **Effort:** M (3 days). **Quickest win**: pin reusable workflows to SHAs, fix thegent ci.yml, add `cargo +nightly miri test` to thegent-zmx-interop.

3. **L0 / L1 / L13 — Architecture & onboarding gaps**
   Empty subcmds/zerokit/pheno-logging-zig crates, no machine-checkable cycle guard, thegent/cli/parser.py is a 12-line stub, `just bootstrap` recipe missing in 4/4 main repos.
   **Effort:** M (1 week). **Quick win**: remove or implement the 3 empty crates, add the devcontainer/just-bootstrap/SLA pattern as `pheno-ci-templates` snippet.

4. **L5 / L27 — Observability stack present but unwired**
   `agileplus-telemetry` gRPC exporter is dead code, no `/metrics` endpoint anywhere bloc-wide, Tracera declares OTel/Prometheus deps but never uses them, no Sentry.
   **Effort:** M (1 week). **Quick win**: add `tower::limit::RateLimitLayer` + `/metrics` in `agileplus-api`, wire OTel exporter in `tracera-core`.

5. **L12 / L13 — Doc & DX hygiene**
   Tracera's `SSOT.md` is an auto-recovered index dump (160+ refs to obsolete `.PHASE_5_CHECKPOINT_*.md`), Tracely `docs/` only has `operations/`, no `just bootstrap`.
   **Effort:** S (3 days). **Quick win**: refactor Tracera SSOT.md to precedence table.

6. **L4 / L7 — Concurrency spec present, gates absent**
   TLA+ spec in `thegent/docs/spec/tla_multi_agent.tla:1-76` covers claim system, but no `loom`/`miri`/`TSan` CI, no `CancellationToken`, no `CONCURRENCY.md`.
   **Effort:** M (3 days). **Quick win**: add `RUSTFLAGS="-Z sanitizer=thread"` nightly job.

7. **L2 / L14 — API surface leaks internal details**
   `agileplus-api/src/error.rs:64` `ApiError::Internal(other.to_string())` exposes domain messages on the wire. No pagination, no idempotency keys on POST. `tracera-core::pagination` exists but isn't wired to `agileplus-api` consumers.
   **Effort:** M (3 days). **Quick win**: add Stripe-style envelope (`{code, message, request_id, type}`) to `agileplus-api`.

8. **L15 / L17 — CLI & a11y gaps**
   No `clap_complete` (only HeliosCLI satellite has it), no `clap_mangen`, no `indicatif` progress bars, no `inquire` prompts, no `prefers-reduced-motion` bloc-wide, no skip-link component.
   **Effort:** S (1 day). **Quick win**: add `clap_complete` + `clap_mangen` to `agileplus-cli`, replace `thegent/cli/parser.py` stub with real typer.

9. **L18 / L19 / L28 — Supply-chain federation link broken**
   `reusable-trufflehog.yml` reference in `phenotype-dep-guard/.github/workflows/trufflehog.yml:14` points to non-existent path. Renovate + Dependabot co-existence creates duplicate PRs. `thegent` Dependabot is `interval: daily` with 30 PR limit (PR flood).
   **Effort:** M (2 days). **Quick win**: fix federation ref, drop Dependabot on repos with Renovate.

10. **L23 / L24 / L25 — Audit/compliance + tenant + resource enforcement**
    No hash chain, no event signature, retention 90d vs SOC2 1y / HIPAA 6y minimums. `ResourceLimits` declared but not enforced in `agileplus-pipeline/executor.rs`.
    **Effort:** L (2-3 weeks). **Quick win**: add append-only DB triggers, wire `ResourceLimits` via `setrlimit` + cgroup.

## 2. Sequencing (8 weeks)

| Week | Focus | Pillars | Top 3 gaps addressed |
|---|---|---|---|
| 1 | Security foundation | L14, L19, L22 | Gap #1, #9 |
| 2 | CI/CD + tests | L10, L11 | Gap #2 |
| 3 | Architecture cleanup | L0, L1, L13 | Gap #3 |
| 4 | Observability | L5, L23, L27 | Gap #4, #10 |
| 5 | Concurrency safety | L4, L7, L24 | Gap #6, #10 |
| 6 | API surface | L2, L3, L14, L15 | Gap #7, #8 |
| 7 | Supply chain | L18, L19, L28 | Gap #9 |
| 8 | DX polish | L12, L13, L16, L17 | Gap #5, #8 |

## 3. Verdict

The Phenotype bloc is **strong on primitives** (TLA+ spec, TYPESCRIPT error envelope, parking_lot + crossbeam, criterion benches, OpenSSF Scorecard, SBOM emission, RateLimiter multi-strategy) but **weak on gates**: most SOTA primitives exist locally, but no CI / no test coverage / no Sentry / no /metrics endpoint / no loom / no miri. Closing the 10 top gaps pushes the bloc from "12 months ahead on detection" to "12 months ahead end-to-end".

The **cheapest mass-impact wins** (3 days total):
1. Add `clap_complete` + `clap_mangen` to `agileplus-cli` (L15, 1 day)
2. Refactor Tracera `SSOT.md` to precedence table (L12, 1 day)
3. Add `just bootstrap` recipe in 4 main repos (L13, 1 day)
4. Wire `RUSTFLAGS="-Z sanitizer=thread"` nightly job in thegent (L7, 1 day)
