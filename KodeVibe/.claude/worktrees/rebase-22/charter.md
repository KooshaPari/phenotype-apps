# Charter — KodeVibe

> **Boundary class:** tooling  
> **Role:** quality  
> **Lifecycle:** active  
> **Genesis template:** HexaKit `templates/genesis/` v1.0.0

## Mission

Code quality guardian for Phenotype-org — deterministic static analysis (vibes, hooks, CI) via Go engine, shell CLI, MCP/daemon, and linked genesis governance.

## Scope

### In scope

- `engine/` — Go static-analysis runtime (scanner, daemon, MCP hooks, scoring) migrated from KodeVibeGo
- Shell CLI (`kodevibe` / vibecheck) delegating to Go binary when present
- Git hooks, `.vibecheck.yaml` / `.kodevibe.yaml` configuration
- MCP and daemon integration for agent workflows
- Genesis governance: intent, charter, review, SOTA, OKF
- Quality platform docs (`docs/architecture/quality-platform.md`)

### Out of scope

| Boundary | Owner repo |
|----------|------------|
| LLM output validation, test-quality assessment | `kwality` |
| Governance rule schema only (not runtime linting) | `HexaKit` `phenotype-compliance-scanner` |
| Rust developer tooling / CI wrappers | `phenotype-tooling` |
| Go platform SDK modules | `phenotype-go-sdk` |
| Genesis templates | `HexaKit` |
| KodeVibeGo archive tombstone | `KodeVibeGo` (protected; do not delete) |

## Governance artifacts

| Artifact | Path |
|----------|------|
| Intent | [intent.md](intent.md) |
| Review (Kilo Code Stand) | [review.md](review.md) |
| SOTA | [SOTA.md](SOTA.md) |
| OKF manifest | [okf/manifest.okf.yaml](okf/manifest.okf.yaml) |

Specs: [HexaKit docs/genesis/STANDARD.md](https://github.com/KooshaPari/HexaKit/blob/main/docs/genesis/STANDARD.md)

## Decision rights

| Action | Authority |
|--------|-----------|
| Merge to `main` | KooshaPari + 1 reviewer |
| Agent-authored PR | Allowed per [review.md](review.md) |
| Scope expansion into LLM validation | **Blocked** — kwality owns that boundary |
| New language runtime without SOTA | **Blocked** — requires `docs/sota/technical.md` justification |

**Agent autonomy:** Level 2 — agents may edit engine/CLI/docs within charter; kwality boundary is Block tier.

## Dependencies

- Genesis bootstrap: HexaKit templates version `v1.0.0`
- Predecessor: KodeVibeGo (archived; engine migrated)
- Fleet registry: `phenotype-registry` (`quality` role)
- Complement: `kwality` for LLM validation layer

## Retirement

KodeVibeGo remains archived tombstone. This repo is canonical for static analysis runtime. Do not delete without 100% boundary coverage in successor + registry update.

## Changelog

| Date | Change | Author |
|------|--------|--------|
| 2026-06-16 | Initial charter from genesis template | agent |

## Attestation

This charter supersedes informal README scope claims. On conflict, charter wins.
