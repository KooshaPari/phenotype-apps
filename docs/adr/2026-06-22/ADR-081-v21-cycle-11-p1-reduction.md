# ADR-081: v21 Cycle 11 P1 Reduction Plan

- **Status**: ACCEPTED
- **Date**: 2026-06-22
- **Author**: KooshaPari
- **Deciders**: orchestrator
- **Cycle**: 11
- **Wave**: v21
- **Branch**: feat/v21-71-pillar-cycle-11-p1-2026-06-22

## Context

v9 (2026-06-18) → v20 (2026-06-22) closed **47/47 P0 pillars** (100%) and deepened the
P1 pillar pool. The fleet now sits at **2.94/3.00 mean** with two domains below 3.00:

- **Security (SEC, L46-L55)**: 2.70 (5 P1 gaps: L48 SBOM diff, L51 SOC2 evidence, L50 rotation, L54 OIDC, L52 encryption-at-rest)
- **Performance (PERF, L13-L19)**: 2.85 (3 P1 gaps: L19 perf gate, L13 latency budget, L17 soak tests)

Cycle 11 (v21) targets these two domains by deepening 5 P1 pillars to 3/3 with concrete
implementation, not just plans.

## Decision

Dispatch 5 parallel subagents in `orch-v21-T{1..5}-L{nn}-{slug}` mode. Each agent:

1. Reads the corresponding v20 finding for context
2. Implements the gate/automation in a target crate
3. Adds 1 proptest or 1 criterion bench or 1 doc-test for coverage
4. Commits + pushes + opens PR to argis
5. Updates the corresponding finding with closure status

| Track | Pillar | Target Crate | Subagent |
|-------|--------|--------------|----------|
| T1 | L48 SBOM diff gate | pheno-port-adapter | orch-v21-T1 |
| T2 | L51 SOC2 evidence | pheno-evidence-collector (new) | orch-v21-T2 |
| T3 | L33 SIGHUP hot-reload | pheno-config | orch-v21-T3 |
| T4 | L37 devcontainer × 3 | pheno-events, pheno-predict, pheno-secret-scan | orch-v21-T4 |
| T5 | L34 release.yml × 5 | 5 fleet-critical crates | orch-v21-T5 |

## Consequences

**Positive:**
- 5 P1 pillars → 3/3 (target: 47/47 P0 + 5/5 P1 = 52/52 "saturated" pillars)
- Brings SEC domain to 3.00 and PERF to 3.00
- Fleet mean: 2.94 → ~3.00
- Workload distributed across 5 subagents (no single agent overload)

**Negative:**
- Subagents may collide on shared resources (cargo lock, network)
- 5 parallel agents may push load to 300+ temporarily
- New crate `pheno-evidence-collector` requires GitHub repo creation first

## Compliance

- Device: **macbook** (orchestration only, no cargo)
- Heavy work: subagents (`device: heavy-runner`)
- Fork-guardian: **MUST stay UP** (2h48m uptime as of this ADR)
- Protected processes: forge, ghostty, claude, zsh, login, nvim, cargo
