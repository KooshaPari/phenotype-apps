# Wave-2 Shard Manifest — 2026-06-18 (EXECUTING)

## Wave-1 Status (L1-L3): COMPLETE
| Shard | Status | SHA |
|---|---|---|
| S01 Tokn | ✅ | 13871e8 |
| S02 argis-extensions | ✅ | 0ab5953 |
| S03 McpKit | ✅ | 28eef29 |
| S04 KlipDot | ✅ | e874dac |
| S05 helios-router PR | ⏳ | #230 OPEN |
| S06 Metron | ✅ | 77dfc33 |
| S07 NetScript (deleted) | ❌ | absorbed S12 |
| S08 cheap-llm-mcp (deleted) | ❌ | per ADR-007/008 |
| S09 dispatch-mcp | ✅ | e1138dc |
| S10 KodeVibe | ✅ | af1223e |
| S11 shard manifest | ✅ | (this file) |

## Wave-2 Status (L5-L8): IN PROGRESS

| Shard | Lane | Target | Status | Owner | Notes |
|---|---|---|---|---|---|
| S12 | L5 | NetScript→phenotype-lexer-rs (new) | in_progress | forge-2 | Rust lexer (was misclassified as Go in DAG) |
| S13 | L5 | McpKit→PhenoFastMCP-rust | in_progress | forge-2 | Already deprecated per ADR-017 |
| S14 | L6 | phenotype-sdk scaffold | in_progress | forge-3 | AtomsBot core lib |
| S15 | L6 | phenotype-bot-framework | in_progress | forge-3 | AtomsBot framework |
| S16 | L6 | phenotype-discord-adapter | in_progress | forge-3 | AtomsBot adapter |
| S17 | L6 | phenotype-github-adapter | in_progress | forge-3 | AtomsBot adapter |
| S18 | L7 | KodeVibe→phenotype-tooling | in_progress | forge-4 | HexaKit Genesis scaffold |
| S19 | L8 | helios-router→helios-cli | in_progress | forge-4 | UI components |
| S20 | L9 | Final verification | pending | forge-1 | After all L5-L8 done |

## Concurrency
- forge-1: orchestrator + verification
- forge-2: S12-S13 (Rust absorptions)
- forge-3: S14-S17 (AtomsBot decomp)
- forge-4: S18-S19 (template + UI migration)
