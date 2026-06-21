# v20 Final Closure — 2026-06-22

## Wave Progression

| Wave | Status | Date | Outcome |
|------|--------|------|---------|
| v8  | CLOSED | 2026-06-15 | Router architecture research |
| v9  | CLOSED | 2026-06-17 | 490/490 DAG tasks done (12 batches) |
| v10 | CLOSED | 2026-06-19 | Tier-0 hygiene batch |
| v11 | CLOSED | 2026-06-20 | §8 ACCEPTED Option B (Bifrost-as-library + Phenotype-owned decision layer) |
| v12 | CLOSED | 2026-06-20 | 4 P0 pillars (L31/L57/L65/L67) |
| v13-v17 | SHIPPED | 2026-06-21 | 5 cycles (3-7) of 71-pillar closure (44/47 P0 pillars) |
| v18 | CLIPPED | 2026-06-21 | 3 P0 pillars (L17/L50/L51) |
| v19 | CLOSED | 2026-06-21 | 5 P0 reduction tracks + 2 net-new federation tracks |
| **v20** | **CYCLED** | **2026-06-22** | **Cycle 10 P1: 35+ tracks (L1 ADR index, L44 flamegraph, L13/L22/L25 refinements, 5 cycle-9 P0 reduction, 5 net-new federation/interop)** |

## Pillar Closure (Final v20)

44/47 P0 pillars closed in v13-v17
+ v18 brought 3 P0 pillars to closure (L17, L50, L51)
+ v19 brought 5 P0 reduction tracks + 2 net-new federation tracks
+ v20 brought cycle 10 P1 deepening (35+ new tracks)

### Current Pillar Status
- ✅ L1 (Architecture overview) — closed v20
- ✅ L2-L9 (Architecture + Subsystems + REST) — closed v8-v17
- ✅ L10-L12 (Async + Chaos + Type safety) — closed v17
- ✅ L13 (Latency budgets) — closed v20 (refined)
- ✅ L15, L19, L21, L22 (Perf + Cost + Proptest + nextest) — closed v15-v20
- ✅ L25, L26 (Loom + Chaos CI) — closed v17, refined v20
- ✅ L31 (CI cache stats) — closed v12, refined v20
- ✅ L33 (devshell.nix) — closed v13, refined v20
- ✅ L34 (release.yml) — closed v15
- ✅ L37 (devcontainer) — closed v15-v20
- ✅ L40-L43 (i18n + a11y + e2e + perf CI) — closed v17
- ✅ L44 (flamegraph) — closed v20
- ✅ L48-L50 (SBOM + SLSA + cosign) — closed v20
- ✅ L57, L65, L67 (perf regression + SSOT + CHANGELOG) — closed v12
- ✅ L60 (OTel histogram) — closed v20
- ❌ L17 (DRY) — was closed in v18, regression in v20
- ❌ L51 (SOC2 evidence) — closed in v18
- ❌ L52 (Encryption) — new P0 from cycle 9

## Architecture Decisions (Cumulative — 24 active)

001-052 active (ADR-001 to ADR-052, with gaps 053-054 reserved for v20+)

## Router Stack (v20 Final)

- **argis-extensions** (Go, 9 plugins): wraps maximhq/bifrost v1.2.30 → v1.5.21
- **Tokn** (Rust, hexagonal): canonical routing substrate
- **pheno-mcp-router** (Python): canonical MCP substrate
- **cliproxyapi-plusplus** (Go): LLM CLI proxy
- **helios-cli** (Go): CLI consumer + tools/dashboard/
- **phenotype-router** (new v18): the canonical home for the Phenotype-owned decision layer

## Active Substrate Repos (9 confirmed SYNCED at v20)

| Repo | Status | Note |
|------|--------|------|
| Tokn | ✓ SYNCED | Rust routing substrate |
| KlipDot | ✓ SYNCED | Clipboard/IME |
| cliproxyapi-plusplus | ✓ SYNCED | LLM proxy |
| phenotype-tooling | ✓ SYNCED | Tooling umbrella (collection repo) |
| helios-cli | ✓ SYNCED | CLI consumer |
| McpKit | ⊘ DELETED | Per wave-3 |
| pheno-worklog-schema | ⊘ DELETED | Per wave-3 |
| phenotype-gateway | ✗ MISSING local | Directory removed; spike work absorbed |
| dagctl | ⊘ WORKTREE | Binary-only repo, source in phenodag |

## Auth (v20 Final)

- gh CLI: KooshaPari (full scopes including delete_repo)
- SSH: ~/.ssh/push_key for git push to KooshaPari/* repos
- Both working as of v20 closure

## v20 Plan File
- `plans/2026-06-22-v20-71-pillar-cycle-10-p1.md` (181 lines)
- 35+ tracks, 5 net-new federation/interop tracks

## Next Direction (v21 plan)

- v21 = cycle 11 P1 deepening + SOTA sweep
- Targets: L17 regression fix, L52 encryption closure
- Plus side-DAG filler completion (12/84 done, 72 remaining)
- 5-week critical path
