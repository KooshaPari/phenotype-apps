# ADR-023: Agent-effort governance — device + dogfood + app substrate policy

**Status:** Accepted 2026-06-15
**Deciders:** PhenoRust 1.0 architecture circle + repo owner
**Supersedes:** ad-hoc "work on whatever is loud" triage; `STATUS.md` "Active focus repos" listing as the sole authority on what agents touch.
**Effective:** 2026-06-15 (immediate; affects all future PRCP manifests, worklogs, and orchestrator-direct sessions).

## Context

Three recurring frictions were identified at the close of V6 (2026-06-15):

1. **Device mismatch.** The orchestrator runs on a MacBook (macOS, 16 GB RAM class,
   8–10 cores). Some Phenotype sub-repos are *heavy* in a way that is incompatible
   with the device: large native build matrices, OS-specific build breaks, full
   monorepo `cargo test --workspace` runs, GUI/X11-only E2E, simulator boots,
   container-in-container tests, GPU/Metal-shader harnesses. Working on those
   repos on this device produces noise (slow CI, truncated logs, timeout failures
   in the `task` tool's 60–120 s wall clock) without producing real signal.

2. **App-level dogfood gap.** Several app-level repos (`focalpoint`, `QuadSGM`,
   `AtomsBot*`) consume agent time but do **not** consume the SWE process the
   same way `Civis` / `Dino` / `WSM` / the focus repos do. Specifically:
   - `focalpoint` is the prior AGENTS.md template; effectively shelved.
   - `QuadSGM` and `AtomsBot*` are not on the SWE critical path; `AtomsBot*`
     originated as a capstone project whose sponsor we are not on good terms
     with — we can legally lift code/concepts from it but should not invest
     new effort into it.
   - `Civis`, `Dino`, `WSM` are real active work, but the "frontend" slice of
     each is currently low-iteration. The *engine / non-frontend* slice of
     `Dino` (heavy visual engine work that can be quickly iterated) is
     acceptable. `WSM` has no acceptable work right now.
   - The pattern: **dogfood use in the SWE process is the only signal that
     earns agent time.** A repo only earns effort if it is something we
     actually use to do our own work.

3. **Substrate placement for app-level repos (e.g. `HwLedger`).** `HwLedger`
   and similar apps currently have their "underlying parts" scattered across
   ad-hoc `phenoShared/` directories and per-app `lib/` folders. That
   placement produces:
   - **Context waste** — agents waste tokens re-loading the same `phenoShared/`
     bits across many apps because there is no canonical place to look.
   - **Quality / maintainability decay** — without an owning crate, the
     parts have no spec, no docs, no test gate, no release cadence; drift
     is the default.
   - **Dev / user / agent satisfaction cliff** — the three stakeholders
     want three different things (dev: clean code; user: working app; agent:
     low context cost), and the "random `phenoShared`" placement satisfies
     none of them well.
   - **LOC bloat** — duplication across apps inflates LoC, slows review,
     and dilutes coverage.
   - **Coverage under-investment** — specs, docs, unit / e2e / integration
     / perf / chaos tests, and observability are what makes the codebase
     maintainable long-term, and none of that exists for the "random
     `phenoShared`" parts.

The unifying insight: **agent time is the only scarce resource in this
fleet** (humans are bottlenecked, CI is not). The job of governance is to
direct that time at work that *pays back*: dogfooded, well-substrated, well-
covered, fully observable, and HITL-friendly.

## Decision

Three coupled rules. Each is enforced via PRCP manifests, worklog v2 schema
(ADR-015), and the L6 health-audit delta cadence.

### Rule 1 — Device-fit gate (MacBook, current)

The MacBook (current orchestrator device) is **not** a heavy-work device.
A repo / task is "heavy" if any of:

- The local dev cycle includes a full `cargo test --workspace` against a
  multi-100-crate workspace, an iOS Simulator boot, a Docker-in-Docker
  test, a Unity / Unreal editor head, or a browser-automation E2E that
  needs a live backend with seeded data.
- A single build / test cycle exceeds **10 minutes wall** on the MacBook.
- The repo requires Linux-only or Windows-only tooling (e.g. `WPF`,
  `Win32`, `systemd`, `iproute2`-only network shims) to do its core work.

**Heavy work does not happen on the MacBook.** The worklog entry
(`pr_number`, `category=chore|feat|fix`, `notes`) must include
`device: macbook` *or* `device: heavy-runner` (a self-hosted runner or
dispatched subagent) — and a "heavy" task with `device: macbook` is a
policy violation that the L6 health audit flags.

This rule does not stop the agent from *thinking* about heavy repos
(reading, planning, ADR-writing, finding-drafting) on the MacBook; it
stops the agent from *executing* builds / tests / E2Es on it.

### Rule 2 — App-level repo triage by dogfood use

The app-level sub-repos are placed into one of three buckets. The bucket
determines what (if anything) the agent is allowed to do.

| Repo         | Bucket         | Allowed work on this device                                                                                                                |
| :----------- | :------------- | :----------------------------------------------------------------------------------------------------------------------------------------- |
| `Civis`      | **ACTIVE**     | Any. Full SWE process.                                                                                                                     |
| `focalpoint` | **PAUSED**     | Read-only. The prior AGENTS.md template is shelved. Do not write code; do not create branches.                                              |
| `Dino`       | **CONDITIONAL** | **Engine / non-frontend** work that can be quickly iterated (heavy visual engine, game-runtime, asset-pipeline, deterministic sim). No UI / HUD / UX / asset work right now. |
| `WSM`        | **CONDITIONAL** | **None right now.** Re-evaluate when an active consumer appears.                                                                            |
| `QuadSGM`    | **PAUSED**     | Read-only. Not on SWE critical path.                                                                                                       |
| `AtomsBot*`  | **PAUSED (capstone)** | Read-only as a *target* of new work. **May be legally mined** — code, concepts, schema, docs, tests may be lifted into new pheno-* / phenotype-* / hwLedger-adjacent repos as needed. The capstone project's sponsor is not in good standing; we treat its public repo as fair-game reference material. |
| `HwLedger` + every other "app-level" repo not in this list | **RECLASSIFY** | See Rule 3. Until reclassified, default to **PAUSED**.                                                                                      |

The bucket list above is the source of truth for orchestrator-direct
sessions. A new repo must be added to this table (in `STATUS.md` §
"App-level repo triage" and in `AGENTS.md` § "Active / Paused app-level
repos") before any work is opened on it; otherwise it is PAUSED by default.

A repo can move buckets. A move requires a one-line entry in the worklog
(`bucket_change: from=PAUSED to=ACTIVE reason=...`) and an updated
`STATUS.md` row.

### Rule 3 — App substrate placement (no "random `phenoShared`")

When an app-level repo (e.g. `HwLedger`) needs an underlying capability
that is reusable across apps, that capability is **not** dropped into
"some random `phenoShared`." It is placed in exactly one of:

| Substrate type             | When to use                                                                                                  | Owner             | Examples in fleet                              |
| :------------------------- | :----------------------------------------------------------------------------------------------------------- | :---------------- | :--------------------------------------------- |
| **`pheno-*-lib` / `pheno-*-core`** | Pure reusable library; language-specific (Rust `pheno-*`, Python `pheno-*`, TS `pheno-zod-schemas`). Single concern, single crate. | The crate's named owner | `pheno-config`, `pheno-context`, `pheno-port-adapter` |
| **`phenotype-*-sdk`**        | Cross-language SDK; multiple `pheno-*` libraries plus a polyglot facade. Stable public API.                 | The SDK repo's owner | `phenotype-go-sdk`, `phenotype-python-sdk`     |
| **`phenotype-*-framework`**  | Inversion-of-control framework; opinionated lifecycle, ports, adapters, conventions.                        | The framework repo's owner | `phenotype-hub`, `phenotype-bus`            |
| **Federated service**        | The capability is stateful, long-running, or has independent scaling / availability. Run as a service, not a lib. | The service repo's owner | `phenoMCP`, `phenoObservability`, `phenoEvents` |

The decision tree for new shared code:

1. Is it a single language, single concern, no I/O? → `pheno-*-lib`.
2. Is it cross-language and exposes a stable public API? → `phenotype-*-sdk`.
3. Does it dictate application lifecycle / DI / config wiring? → `phenotype-*-framework`.
4. Is it stateful, scalable, or long-running? → Federated service.

A capability is **never** dropped into `phenoShared/`, `crates/`, `libs/`,
or any per-app `lib/` directory *as a side effect* of an app-level feature
request. If a "random `phenoShared`" placement already exists for a
capability, that capability is migrated to one of the four canonical
placements on a per-capability basis; the migration is tracked in the
L6 health audit delta.

### Rule 3.1 — Quality bar for new substrate

Every new `pheno-*-lib`, `phenotype-*-sdk`, `phenotype-*-framework`, or
federated service ships with:

- **Spec** (`SPEC.md` or equivalent) — 1-page max, audience = the
  app-level repo owners, format = `SSOT.md` linked.
- **Docs** (`README.md` + 1 concept doc) — what it is, when to use it,
  when **not** to use it, a 5-line "first 5 minutes" quickstart.
- **Test matrix** — unit + integration at minimum; e2e + perf + chaos
  **strongly preferred** for the four fleet-critical substrates
  (config, tracing, MCP-router, observability).
- **Observability** — OTLP export via `pheno-tracing` (ADR-012) at
  info-level minimum; `error` and `warn` are always emitted; `debug`
  behind an env flag.
- **Coverage gate** — 80 % line coverage for libs / SDKs, 70 % for
  frameworks, 60 % for federated services (the lower for the latter
  because integration coverage is what matters there).
- **CI gate** — `pheno-ci-templates` runs the test matrix, the coverage
  gate, and the OTLP smoke test on every PR.
- **Worklog v2 entry** — per ADR-015.

The goal is **HITL-less dev from base intent**: a single one-line
description of intent ("I need a `Config` struct for my service that
reads from env and a TOML file, with a 12-factor cascade") produces
a PR that already has the spec, the docs, the tests, the coverage, the
observability, and the CI gate — without the human in the loop having
to specify any of those one by one.

### Rule 4 — Agent-execution quality bar

The bar for "agent did its job well" is **not** "the PR merged." It is:

- **LOC was reduced** (or, for new code, was *justified* by spec).
- **Coverage of spec / docs / tests was maximized** (not maximized = the
  same set of unit tests + 1 happy-path integration test that V2 shipped).
- **Observability is automated** — the new code emits the same structured
  logs / metrics / traces as the substrate around it. A human reading
  Grafana should not be able to tell which lines were written by a human
  and which by an agent.
- **Dev / user / agent satisfaction** — measurable, not aspirational.
  Each PR worklog entry ends with a one-line self-rating on each axis
  (1–5) and the dev/user/agent are explicitly listed in the PR template.

This is the L5 (governance) layer's answer to the L3 (substrate) layer's
PRCP pattern (ADR-018): PRCP ships the work, ADR-023 governs *which* work
ships and *how well*.

## Consequences

**Positive**

- Heavy work is moved to the right device (self-hosted runner / dispatched
  subagent) and the MacBook is reserved for the work it can actually do
  well: planning, ADR-writing, small focused PRs, code review, dogfooding.
- Agent time is redirected to repos that *pay back* — `Civis` and the
  engine / non-frontend slice of `Dino` — and away from repos that
  consume time without producing value (`focalpoint`, `QuadSGM`,
  `AtomsBot*` as a target, `WSM` for now).
- The "random `phenoShared`" sprawl stops growing. New shared code lands
  in a canonical place (`pheno-*-lib` / `phenotype-*-sdk` /
  `phenotype-*-framework` / federated service) with a spec, docs, tests,
  coverage, observability, and CI gate as a single package.
- Context waste drops: an agent looking for "how do I do X" finds the
  one canonical repo and the one canonical docs page, not 5 near-duplicate
  `phenoShared/` shims.
- The fleet's quality bar (LOC reduced, coverage maximized, observability
  automated, HITL-less dev) becomes enforceable as a PR-template field,
  not an aspirational paragraph.
- `AtomsBot*` is now explicitly a *resource* (we can legally lift from
  it) rather than a *target* (we sink time into it). This is the
  capstone-sponsor consequence made explicit.

**Negative**

- The "Active focus repos" listing in `STATUS.md` and `AGENTS.md` is no
  longer the source of truth for what agents touch. The new "App-level
  repo triage" table is. Drift between the two is a real risk; the
  L6 health-audit delta is the canary.
- Device-fit gate requires a `device:` field in worklog v2 rows. This is
  a schema change to ADR-015 and breaks the v2 validator. The schema is
  bumped to v2.1 (additive) and the migration is mechanical.
- "Paused" repos that are *not* in the table can be re-opened by an
  over-eager agent. The default-PAUSED rule is a guardrail but not a
  lock; `CODEOWNERS` is the lock, and it needs to be reviewed per repo.
- `Civis` becomes the de facto "everything else" target. If `Civis` ever
  bloats or stops dogfooding, the whole governance premise breaks. This
  is mitigated by the bucket-change worklog entry — `Civis` is reviewed
  at every V-cycle.

**Mitigation**

- The L6 health-audit delta runs weekly and includes a "bucket drift"
  check: any active PR / branch in a PAUSED repo, or a `device: macbook`
  flag on a heavy task, is a `P1` finding that the next session addresses.
- The schema bump (ADR-015 v2.0 → v2.1) ships with a 1-week deprecation
  period; the validator logs a warning for the old shape and errors
  after 2026-06-22.
- The `findings/` directory gets a new `L6_APP_TRIAGE_BUCKET_DRIFT-*.md`
  template that any session can drop a finding into in 60 seconds.

## Alternatives considered

- **"Just don't work on the heavy stuff" as a soft guideline, not a gate.**
  Rejected: the 2026-06-14 RESUME wave demonstrated that soft guidelines
  are routinely violated when the orchestrator is under time pressure.
  The gate (the `device:` field in worklog v2.1) is what makes the rule
  survive contact with reality.
- **Bucket repos by `CODEOWNERS` rather than by dogfood use.** Rejected:
  `CODEOWNERS` answers "who reviews" not "is it worth doing." The two
  questions are independent; the dogfood table is the *what* and
  `CODEOWNERS` is the *who* and they compose.
- **Keep the "random `phenoShared`" pattern but add a registry.** Rejected:
  the V4 → V5 audit showed that 4 of 9 "config" crates had zero consumers
  in a registry. A registry does not fix context waste; the canonical
  placement does.
- **Forbid app-level repos from sharing code entirely (force them to
  copy-paste).** Rejected: this is the *opposite* problem (LOC bloat
  via duplication) and contradicts the rest of the fleet's substrate
  work (ADR-012, ADR-013, ADR-014).
- **Treat `Civis` as the only ACTIVE app-level repo permanently.**
  Rejected: this is the current de-facto state and the cause of the
  context waste we're trying to fix. `Civis` is the *floor* of ACTIVE,
  not the ceiling. `Dino`'s engine slice can earn ACTIVE via the
  bucket-change worklog rule.

## References

- `STATUS.md` § "App-level repo triage" (added by this ADR).
- `AGENTS.md` § "Active / Paused app-level repos" + "App substrate placement"
  (added by this ADR).
- `SSOT.md` row "Agent-effort governance" (added by this ADR; points
  back to this ADR-023 file).
- `ADR-015-v2-worklog-schema.md` — to be bumped to v2.1 to add the
  `device:` field.
- `ADR-012-pheno-tracing-canonical.md` — observability substrate that
  the new `pheno-*-lib` quality bar assumes.
- `ADR-014-hexagonal-l4-ports.md` — `pheno-port-adapter` pattern that
  the federated-service row of the substrate table assumes.
- `ADR-018-prcp-pattern.md` — the cross-repo-coordination pattern that
  this governance rule composes with (PRCP decides *how*; ADR-023
  decides *what* and *when*).
- `findings/2026-06-15-L5-101-app-governance.md` — the L5 worklog
  finding for this decision.
- `worklogs/L5-101-app-governance-2026-06-15.json` — the L5 worklog
  JSON (v2.1 schema).
