# Phenotype Quality Platform

Canonical homes for code quality, validation, and profiling across the KooshaPari org.

## Architecture

| Layer | Repo | Role |
|-------|------|------|
| **Static analysis & vibes** | [KodeVibe](https://github.com/KooshaPari/KodeVibe) | Go engine (`engine/`): scanner, daemon, MCP, scoring, auto-fix |
| **LLM output validation** | [kwality](https://github.com/KooshaPari/kwality) | LLM validation platform (personal; do not delete) |
| **Runtime profiling** | [phenotype-tooling](https://github.com/KooshaPari/phenotype-tooling) `packages/profila/` | CPU/memory/I/O profilers (migrated from Profila) |
| **Governance rules only** | [HexaKit](https://github.com/KooshaPari/HexaKit) `phenotype-compliance-scanner` | YAML rule schema + policy types — **not** a linter runtime |
| **Tombstone** | [KodeVibeGo](https://github.com/KooshaPari/KodeVibeGo) | Archived lineage; runtime absorbed into KodeVibe `engine/` |

## What does NOT live in HexaKit

- Vibe checkers, scoring engines, MCP fix loops → **KodeVibe**
- LLM response validation → **kwality**
- System/code profilers → **phenotype-tooling/profila**

HexaKit `phenotype-compliance-scanner` holds federation rule types for CI policy gates, not the full scanner stack.

## Integration points

- KodeVibe MCP payloads can feed kwality validation pipelines for AI-generated fixes.
- Profila metrics complement KodeVibe scan reports in agent observability dashboards.
- HexaKit governance rules reference `.kodevibe.yaml` schema types; execution stays in KodeVibe.

## Migration lineage

- KodeVibeGo → KodeVibe `engine/` (2026-05)
- Profila → phenotype-tooling `packages/profila/` (2026-06)
