# Technical — SOTA (KodeVibe)

## Use case

Deliver deterministic static code-quality analysis (multi-vibe scanner, hooks, CI, MCP/daemon) as the canonical `quality` role owner — complementary to kwality's LLM validation layer.

**AgilePlus / FR trace:** quality-platform architecture; KodeVibeGo migration

## Requirements

| Requirement | Weight |
|-------------|--------|
| Fast file-tree scanning on medium/large repos | must |
| Git hook + CI integration with stable exit codes | must |
| MCP/daemon for agent workflows | should |
| Go engine Tier 3 justification documented | must |
| Clear boundary vs kwality (LLM) and HexaKit (schema) | must |

## Language placement

| Component | Lang | Tier | Rationale |
|-----------|------|------|-----------|
| `engine/` scanner, daemon, MCP | Go | 3 | Migrated from KodeVibeGo; existing analyzer; walk/parser performance; stdlib + ecosystem for static analysis |
| Shell CLI / install scripts | Shell | 3 | Developer UX, hook installation, delegate-to-binary pattern |
| Future core rewrite | Rust | 1 | Only if SOTA shows benchmark + feature parity + migration plan |

### Why Go at this edge (Tier 3)

1. **Migration cost:** KodeVibeGo engine already implements scanner, scoring, daemon, and MCP hooks — rewrite would block fleet rollout.
2. **Ecosystem:** Go static analysis tooling (`go vet`, `staticcheck`, `gosec` patterns) aligns with multi-language file walking and fast CLI binaries.
3. **Performance:** Single static binary for daemon mode; acceptable memory for org-scale repos per internal dogfood.
4. **Boundary clarity:** Go is confined to `engine/`; shell is UX only; LLM validation stays in kwality (Tier 2 Python/TS as applicable).

Per [phenotype-registry LANGUAGE_PLACEMENT](https://github.com/KooshaPari/phenotype-registry/blob/main/LANGUAGE_PLACEMENT.md): *"KodeVibe `engine/` | Go | Tier 3 — existing analyzer; documented in KodeVibe SOTA"*

## Alternatives considered

| Alternative | Type | Pros | Cons | Verdict |
|-------------|------|------|------|---------|
| Keep KodeVibeGo standalone | internal | zero migration | archived; split governance | rejected |
| Rust rewrite (rust-analyzer patterns) | internal | Tier 1 alignment | high cost; delays MCP/hooks | rejected — no migration budget |
| Python-only linter (ruff-style) | OSS pattern | fast to script | slower cold start; weak daemon story | rejected |
| ESLint-only JS stack | OSS | huge plugin ecosystem | weak multi-language vibes | rejected |
| kwality absorbs static checks | internal | one quality repo | mixes LLM + deterministic; charter conflict | rejected |
| **Go engine + shell CLI + kwality complement** | chosen | migration done; clear boundaries | Tier 3 maintenance | **chosen** |

Research sources: [docs/migration-from-kodevibego.md](../../docs/migration-from-kodevibego.md), [docs/architecture/quality-platform.md](../../docs/architecture/quality-platform.md), LANGUAGE_PLACEMENT.

## Chosen strategy

`engine/` holds the Go runtime (`make engine-build`). Shell `kodevibe` CLI delegates to the Go binary when present. Configuration via `.vibecheck.yaml`. HexaKit `phenotype-compliance-scanner` remains schema-only. kwality owns LLM output validation.

Link: [charter.md](../../../charter.md) · [intent.md](../../../intent.md)

## Evolution triggers

Re-open when:

- Engine test coverage drops on critical vibe classes
- Rust analyzer prototype beats Go engine on ≥2 fleet repos
- kwality scope creep into deterministic checks

Update [alternatives.md](alternatives.md) and [../../../SOTA.md](../../../SOTA.md) when verdict changes.
