# PhenoMCP — Action Plan

> **Generated 2026-06-17.** Score grid: [`FLEET-AUDIT-30-PILLAR.md`](../FLEET-AUDIT-30-PILLAR.md). Source: [`PhenoMCP.json`](../../audits_data/PhenoMCP.json).

## Current state

- **Language:** Rust + Python + TypeScript + Go (polyglot)
- **Mean score:** 0.99 (median 1)
- **Zero-pillar count:** 45 of 109
- **Three-pillar count:** 11 of 109
- **Blockers:** S7/SC2-SC4: no threat model/SBOM/attestation, OB2-OB4: no metrics/traces/SLOs, T1-T6: light testing, G1: CODEOWNERS thin

## Notes

Polyglot MCP server. Strong ADRs, decent governance, weak testing.

## Pillar distribution

| Score | Count | % |
|----|----:|----:|
| 3 (measured) | 11 | 10.1% |
| 2 (wired) | 22 | 20.2% |
| 1 (ad-hoc) | 31 | 28.4% |
| 0 (absent) | 45 | 41.3% |

## Phased WBS

### Phase 1: Discovery (≤3 tool calls per task)

- [ ] Read existing pillar evidence for each 0/1 score below
- [ ] Confirm scope of remediation with code owner

### Phase 2: Design (≤5 tool calls per task)

- [ ] Write ADR/decision record for any architectural change (A1-A5)
- [ ] Document coverage/SLO targets before writing the CI gate

### Phase 3: Build (≤15 tool calls per task)

**Tasks by role:**

#### agentic (1 tasks)

- [ ] **PHE-006** `AS2` (Agentic safety) — score 1 → target 2: Lift AS2 (Agentic safety) from 1 to ≥2. Evidence: dry-run partial

#### api (2 tasks)

- [ ] **PHE-004** `AP1` (API surface) — score 0 → target 2: Lift AP1 (API surface) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-005** `AP2` (API surface) — score 0 → target 2: Lift AP2 (API surface) from 0 to ≥2. Evidence: N/A

#### ci-ops (3 tasks)

- [ ] **PHE-046** `Q2` (Quality eng) — score 0 → target 2: Lift Q2 (Quality eng) from 0 to ≥2. Evidence: no ratchet
- [ ] **PHE-047** `Q3` (Quality eng) — score 0 → target 2: Lift Q3 (Quality eng) from 0 to ≥2. Evidence: no allowlist
- [ ] **PHE-048** `Q4` (Quality eng) — score 1 → target 2: Lift Q4 (Quality eng) from 1 to ≥2. Evidence: 1 coverage workflow

#### data (3 tasks)

- [ ] **PHE-022** `DA1` (Data/contracts) — score 0 → target 2: Lift DA1 (Data/contracts) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-023** `DA2` (Data/contracts) — score 0 → target 2: Lift DA2 (Data/contracts) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-024** `DA3` (Data/contracts) — score 0 → target 2: Lift DA3 (Data/contracts) from 0 to ≥2. Evidence: N/A

#### docs (4 tasks)

- [ ] **PHE-018** `D2` (Documentation) — score 0 → target 2: Lift D2 (Documentation) from 0 to ≥2. Evidence: no journeys
- [ ] **PHE-019** `D1` (Documentation) — score 1 → target 2: Lift D1 (Documentation) from 1 to ≥2. Evidence: limited spec tracker
- [ ] **PHE-020** `D3` (Documentation) — score 1 → target 2: Lift D3 (Documentation) from 1 to ≥2. Evidence: rustdoc sparse
- [ ] **PHE-021** `D5` (Documentation) — score 1 → target 2: Lift D5 (Documentation) from 1 to ≥2. Evidence: cargo doc not deployed

#### frontend (11 tasks)

- [ ] **PHE-007** `AT1` (Accessibility & i18n) — score 0 → target 2: Lift AT1 (Accessibility & i18n) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-008** `AT2` (Accessibility & i18n) — score 0 → target 2: Lift AT2 (Accessibility & i18n) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-009** `AT3` (Accessibility & i18n) — score 0 → target 2: Lift AT3 (Accessibility & i18n) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-010** `AT4` (Accessibility & i18n) — score 0 → target 2: Lift AT4 (Accessibility & i18n) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-011** `AT5` (Accessibility & i18n) — score 0 → target 2: Lift AT5 (Accessibility & i18n) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-067** `U1` (UX/Frontend) — score 0 → target 2: Lift U1 (UX/Frontend) from 0 to ≥2. Evidence: N/A — backend server
- [ ] **PHE-068** `U2` (UX/Frontend) — score 0 → target 2: Lift U2 (UX/Frontend) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-069** `U3` (UX/Frontend) — score 0 → target 2: Lift U3 (UX/Frontend) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-070** `U4` (UX/Frontend) — score 1 → target 2: Lift U4 (UX/Frontend) from 1 to ≥2. Evidence: logs use monospace
- [ ] **PHE-071** `UX2` (User experience) — score 0 → target 2: Lift UX2 (User experience) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-072** `UX3` (User experience) — score 0 → target 2: Lift UX3 (User experience) from 0 to ≥2. Evidence: N/A

#### perf (6 tasks)

- [ ] **PHE-013** `C3` (Cost) — score 1 → target 2: Lift C3 (Cost) from 1 to ≥2. Evidence: no ratchet
- [ ] **PHE-037** `P1` (Performance) — score 0 → target 2: Lift P1 (Performance) from 0 to ≥2. Evidence: no benches
- [ ] **PHE-038** `P2` (Performance) — score 0 → target 2: Lift P2 (Performance) from 0 to ≥2. Evidence: no profiling
- [ ] **PHE-039** `P3` (Performance) — score 0 → target 2: Lift P3 (Performance) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-040** `P4` (Performance) — score 0 → target 2: Lift P4 (Performance) from 0 to ≥2. Evidence: no SLOs
- [ ] **PHE-041** `P5` (Performance) — score 0 → target 2: Lift P5 (Performance) from 0 to ≥2. Evidence: N/A

#### qa (6 tasks)

- [ ] **PHE-061** `T3` (Testing) — score 0 → target 2: Lift T3 (Testing) from 0 to ≥2. Evidence: no E2E
- [ ] **PHE-062** `T4` (Testing) — score 0 → target 2: Lift T4 (Testing) from 0 to ≥2. Evidence: no contract tests
- [ ] **PHE-063** `T1` (Testing) — score 1 → target 2: Lift T1 (Testing) from 1 to ≥2. Evidence: 42 test files; no coverage gate
- [ ] **PHE-064** `T2` (Testing) — score 1 → target 2: Lift T2 (Testing) from 1 to ≥2. Evidence: basic integration
- [ ] **PHE-065** `T5` (Testing) — score 1 → target 2: Lift T5 (Testing) from 1 to ≥2. Evidence: ad-hoc bug repro
- [ ] **PHE-066** `T6` (Testing) — score 1 → target 2: Lift T6 (Testing) from 1 to ≥2. Evidence: Linux only

#### rust-dev (17 tasks)

- [ ] **PHE-001** `A1` (Architecture) — score 1 → target 2: Lift A1 (Architecture) from 1 to ≥2. Evidence: MCP server; minimal hex
- [ ] **PHE-002** `A3` (Architecture) — score 1 → target 2: Lift A3 (Architecture) from 1 to ≥2. Evidence: Rust workspace; mixed langs
- [ ] **PHE-003** `A5` (Architecture) — score 1 → target 2: Lift A5 (Architecture) from 1 to ≥2. Evidence: basic types
- [ ] **PHE-015** `CN1` (Concurrency) — score 0 → target 2: Lift CN1 (Concurrency) from 0 to ≥2. Evidence: no race detection
- [ ] **PHE-016** `CN2` (Concurrency) — score 1 → target 2: Lift CN2 (Concurrency) from 1 to ≥2. Evidence: tokio cancellation partial
- [ ] **PHE-017** `CN3` (Concurrency) — score 1 → target 2: Lift CN3 (Concurrency) from 1 to ≥2. Evidence: tool call idempotency
- [ ] **PHE-025** `DM1` (Domain model) — score 1 → target 2: Lift DM1 (Domain model) from 1 to ≥2. Evidence: basic types
- [ ] **PHE-026** `DM2` (Domain model) — score 1 → target 2: Lift DM2 (Domain model) from 1 to ≥2. Evidence: newtypes used
- [ ] **PHE-027** `EH2` (Error handling) — score 1 → target 2: Lift EH2 (Error handling) from 1 to ≥2. Evidence: partial sanitization
- [ ] **PHE-044** `PS1` (Persistence) — score 0 → target 2: Lift PS1 (Persistence) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-045** `PS2` (Persistence) — score 0 → target 2: Lift PS2 (Persistence) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-051** `RT1` (Runtime compat) — score 1 → target 2: Lift RT1 (Runtime compat) from 1 to ≥2. Evidence: Rust version pinned in Cargo.toml
- [ ] **PHE-052** `RT2` (Runtime compat) — score 1 → target 2: Lift RT2 (Runtime compat) from 1 to ≥2. Evidence: Linux only
- [ ] **PHE-073** `X3` (Code quality) — score 0 → target 2: Lift X3 (Code quality) from 0 to ≥2. Evidence: no complexity
- [ ] **PHE-074** `X4` (Code quality) — score 0 → target 2: Lift X4 (Code quality) from 0 to ≥2. Evidence: no duplication
- [ ] **PHE-075** `X5` (Code quality) — score 0 → target 2: Lift X5 (Code quality) from 0 to ≥2. Evidence: no dead code
- [ ] **PHE-076** `X2` (Code quality) — score 1 → target 2: Lift X2 (Code quality) from 1 to ≥2. Evidence: Rust; not strict mode enforced

#### security (12 tasks)

- [ ] **PHE-012** `AU1` (Auditability) — score 1 → target 2: Lift AU1 (Auditability) from 1 to ≥2. Evidence: structured logs
- [ ] **PHE-014** `CF2` (Config) — score 1 → target 2: Lift CF2 (Config) from 1 to ≥2. Evidence: secrets in env
- [ ] **PHE-042** `PR1` (Privacy) — score 0 → target 2: Lift PR1 (Privacy) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-043** `PR2` (Privacy) — score 0 → target 2: Lift PR2 (Privacy) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-053** `S7` (Security) — score 0 → target 2: Lift S7 (Security) from 0 to ≥2. Evidence: no threat model
- [ ] **PHE-054** `S4` (Security) — score 1 → target 2: Lift S4 (Security) from 1 to ≥2. Evidence: auth minimal
- [ ] **PHE-055** `S5` (Security) — score 1 → target 2: Lift S5 (Security) from 1 to ≥2. Evidence: CODEOWNERS gate (2 lines)
- [ ] **PHE-056** `S6` (Security) — score 1 → target 2: Lift S6 (Security) from 1 to ≥2. Evidence: input validation partial
- [ ] **PHE-057** `S8` (Security) — score 1 → target 2: Lift S8 (Security) from 1 to ≥2. Evidence: 1 provenance workflow
- [ ] **PHE-058** `SC2` (Supply chain) — score 0 → target 2: Lift SC2 (Supply chain) from 0 to ≥2. Evidence: no SBOM
- [ ] **PHE-059** `SC3` (Supply chain) — score 0 → target 2: Lift SC3 (Supply chain) from 0 to ≥2. Evidence: no attestation
- [ ] **PHE-060** `SC4` (Supply chain) — score 0 → target 2: Lift SC4 (Supply chain) from 0 to ≥2. Evidence: no provenance

#### sre (11 tasks)

- [ ] **PHE-028** `O2` (Operations) — score 0 → target 2: Lift O2 (Operations) from 0 to ≥2. Evidence: no runbooks
- [ ] **PHE-029** `O3` (Operations) — score 0 → target 2: Lift O3 (Operations) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-030** `O4` (Operations) — score 0 → target 2: Lift O4 (Operations) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-031** `O5` (Operations) — score 0 → target 2: Lift O5 (Operations) from 0 to ≥2. Evidence: N/A
- [ ] **PHE-032** `O1` (Operations) — score 1 → target 2: Lift O1 (Operations) from 1 to ≥2. Evidence: release flow minimal
- [ ] **PHE-033** `OB2` (Observability) — score 0 → target 2: Lift OB2 (Observability) from 0 to ≥2. Evidence: no metrics
- [ ] **PHE-034** `OB3` (Observability) — score 0 → target 2: Lift OB3 (Observability) from 0 to ≥2. Evidence: no traces
- [ ] **PHE-035** `OB4` (Observability) — score 0 → target 2: Lift OB4 (Observability) from 0 to ≥2. Evidence: no SLOs
- [ ] **PHE-036** `OB1` (Observability) — score 1 → target 2: Lift OB1 (Observability) from 1 to ≥2. Evidence: tracing crate
- [ ] **PHE-049** `RL3` (Resilience) — score 0 → target 2: Lift RL3 (Resilience) from 0 to ≥2. Evidence: no bulkheads
- [ ] **PHE-050** `RL1` (Resilience) — score 1 → target 2: Lift RL1 (Resilience) from 1 to ≥2. Evidence: downstream call retries

### Phase 4: Test/Validate (≤5 tool calls per task)

- [ ] Run the new CI gate; verify it fails when evidence is removed
- [ ] Re-score the lifted pillars; confirm the audit JSON reflects the change

### Phase 5: Deploy/Handoff (≤3 tool calls per task)

- [ ] Commit + push the gate
- [ ] Open a PR with the action plan referenced in the body

## DAG (mermaid)

```mermaid
graph TD
  P1[Phase 1: Discovery] --> P2[Phase 2: Design]
  P2 --> P3[Phase 3: Build]
  P3 --> P4[Phase 4: Test/Validate]
  P4 --> P5[Phase 5: Deploy/Handoff]
```

## Top 5 biggest deltas (pillars to lift first)

1. **AP1** — N/A
1. **AP2** — N/A
1. **AT1** — N/A
1. **AT2** — N/A
1. **AT3** — N/A

## Backlog of unaddressed items

Total 76 tasks across 11 roles. See "Build" phase above for the full list.
