# Repo Inventory — KooshaPari / Phenotype

> **Generated 2026-06-16** via `gh repo list KooshaPari --limit 999`. Source: `gh api repos/KooshaPari/...` (cached locally in `/tmp/repo-inv.tsv`). All 201 KooshaPari repos enumerated; **197 to-score**, **4 skipped** per user instruction.

## Skip-list (do not score)

Per user instruction (2026-06-16), the following 4 repos are skipped from the audit:

| Repo | Reason |
|------|--------|
| `FocalPoint` | user skip-list |
| `AtomsBot` | user skip-list |
| `QuadSGM` | user skip-list |
| `Parpoura` | user skip-list |

The skip-list is enforced in two places: (1) the per-repo gate in `FLEET-AUDIT-30-PILLAR.md`, (2) every Phase-4 subagent prompt instructs the agent to abort with `{"skipped": true}` if the repo is on this list.

## Counts

- **Total KooshaPari repos**: 201
- **Skipped**: 4
- **Active (to-score)**: 197
  - Of which 140 are not archived, 57 are archived
- **Visibility mix** (active): 142 PUBLIC, 55 PRIVATE
- **Top languages** (active): Rust ~78, Python ~52, TypeScript ~32, Go ~14, others (C/C++/C#/Svelte/Astro/HTML/Kotlin/Shell/MATLAB/Swift/JavaScript) ~21

## Reference repo

**OmniRoute** is the single reference repo (the only one scored in Phase 3 before fan-out). It is the most mature repo in the fleet (35 quality gates, SLSA Build L2, resilience guide, 160+ LLM providers, 87 MCP tools, 30 quality gates, 60/60/60/60 coverage gate).

## All 197 to-score repos (sorted by last push, descending)

| # | Repo | Language | Visibility | Archived | Last push | Description |
|---|------|----------|------------|----------|-----------|-------------|
| 1 | Civis | Rust | PUBLIC |    | 2026-06-16T06:01:02Z | Phenotype-org governance |
| 2 | phenodocs | Rust | PUBLIC |    | 2026-06-16T05:33:35Z | Phenotype documentation system with VitePress |
| 3 | OmniRoute | TypeScript | PUBLIC |    | 2026-06-16T05:28:33Z | OmniRoute — AI gateway for multi-provider LLMs: OpenAI-compatible endpoint with smart routing across Anthropic, OpenAI,  |
| 4 | Tracera | Rust | PUBLIC |    | 2026-06-16T05:23:44Z | Agent-native multi-view requirements traceability and project management system |
| 5 | AgilePlus | Rust | PUBLIC |    | 2026-06-16T05:23:34Z | Phenotype-org spec-driven development |
| 6 | nanovms | Go | PUBLIC |    | 2026-06-16T05:16:09Z | Lightweight headless VM abstraction for AI agents — three-tier isolation (WASM ~1ms / gVisor ~90ms / Firecracker microVM |
| 7 | Httpora | — | PUBLIC |    | 2026-06-16T05:13:25Z | HTTP client/server framework with middleware support |
| 8 | MCPForge | Go | PUBLIC |    | 2026-06-16T05:13:12Z | mcp-language-server gives MCP enabled clients access semantic tools like get definition, references, rename, and diagnos |
| 9 | HexaKit | Rust | PUBLIC |    | 2026-06-16T05:13:04Z | Phenotype-org hexagonal architecture toolkit |
| 10 | phenotype-bus | Rust | PRIVATE |    | 2026-06-16T05:12:43Z | Phenotype event bus infrastructure |
| 11 | phenotype-org-audits | Rust | PRIVATE |    | 2026-06-16T05:12:30Z | Org-wide audit tooling and compliance tracking |
| 12 | PolicyStack | Python | PUBLIC |    | 2026-06-16T05:12:17Z | Phenotype PolicyStack — policy federation CLI and contracts |
| 13 | thegent | Python | PUBLIC |    | 2026-06-16T05:11:59Z | Python agent runtime with tool registry, LLM provider abstraction, and agent orchestration |
| 14 | phenotype-pm-core | Rust | PRIVATE |    | 2026-06-16T05:09:05Z |  |
| 15 | pheno | Rust | PUBLIC |    | 2026-06-16T05:08:14Z | Pheno Rust monorepo workspace — 170K+ LOC, 11 workspace members (AgilePlus, phenoDesign, PhenoLibs, etc.) |
| 16 | KlipDot | Rust | PUBLIC |    | 2026-06-16T04:26:51Z | STRICTLY DO NOT DELETE NOR UNARCHIVE - Legacy Archived AI-DD Project - Universal terminal image interceptor |
| 17 | PlayCua | Rust | PUBLIC |    | 2026-06-16T04:19:16Z | Bare-metal computer-use agent with Rust + Python |
| 18 | PhenoAgent | Rust | PUBLIC |    | 2026-06-16T04:18:48Z | Phenotype agent core, daemon, and CLI — extracted from phenotype-infra |
| 19 | heliosApp | TypeScript | PUBLIC |    | 2026-06-16T04:18:46Z | An internal tool or component for the Phenotype ecosystem. |
| 20 | dispatch-mcp | Python | PRIVATE |    | 2026-06-16T04:18:44Z | FastMCP dispatch service (extracted from AgilePlus) |
| 21 | rich-cli-kit | Rust | PUBLIC |    | 2026-06-16T04:18:19Z | Rich CLI toolkit for terminal UX |
| 22 | sharecli | Rust | PUBLIC |    | 2026-06-16T04:18:18Z | Shared CLI process manager for multi-project agent orchestration |
| 23 | clap-ext | Rust | PUBLIC |    | 2026-06-16T04:17:44Z | Shared Rust CLI extension library: common subcommands, config flags, error display for clap-based CLIs |
| 24 | Eidolon | Rust | PRIVATE |    | 2026-06-16T04:15:18Z | Phenotype-org agent runtime |
| 25 | Agentora | Rust | PUBLIC |    | 2026-06-16T04:12:57Z | Rust hexagonal-architecture framework for AI agents — skill system, tool registry, two-tier memory (ring + persistent),  |
| 26 | thegent-dispatch | Rust | PRIVATE |    | 2026-06-16T04:12:41Z | Phenotype-org dispatch CLI |
| 27 | helioscope | Rust | PUBLIC |    | 2026-06-16T04:10:37Z | SUPERSEDED — use https://github.com/KooshaPari/helios-cli (canonical Phenotype-org Helios CLI fork). Former heliosCLI re |
| 28 | heliosBench | Python | PUBLIC |    | 2026-06-16T04:10:02Z | An internal tool or component for the Phenotype ecosystem. |
| 29 | AgentMCP | Python | PRIVATE |    | 2026-06-16T04:07:23Z | Namespace placeholder for future Agentic Model Context Protocol work — no implementation yet, scope and stack undecided |
| 30 | KDesktopVirt | Rust | PRIVATE |    | 2026-06-16T04:00:52Z | KDesktopVirt - AI Agent Desktop Automation with container orchestration |
| 31 | thegent-sharecli | Python | PUBLIC |    | 2026-06-16T03:57:24Z | Shared CLI process manager for multi-project agent orchestration |
| 32 | substrate | Rust | PRIVATE |    | 2026-06-16T03:44:50Z |  |
| 33 | PhenoProject | TypeScript | PUBLIC |    | 2026-06-16T03:04:44Z | Phenotype PhenoProject domain workspace |
| 34 | BytePort | Svelte | PUBLIC |    | 2026-06-16T02:07:22Z | Phenotype-org infrastructure tooling |
| 35 | HeliosLab | TypeScript | PUBLIC |    | 2026-06-16T02:03:14Z | Phenotype-org research lab |
| 36 | HeliosCLI | — | PUBLIC |    | 2026-06-16T01:53:55Z | Phenotype HeliosCLI |
| 37 | phenotype-teamcomm | — | PUBLIC |    | 2026-06-16T01:53:50Z | Phenotype phenotype-teamcomm |
| 38 | slickport | TypeScript | PUBLIC |    | 2026-06-16T01:29:38Z | Strictly-personal TypeScript port of Slicker HTTP/socket utility suite — type-safe fetch, middleware chains, and SSR-fri |
| 39 | Benchora | Rust | PUBLIC |    | 2026-06-16T01:15:51Z | Rust benchmarking and performance-testing framework for the Phenotype ecosystem — criterion-based load testing, regressi |
| 40 | Tasken | Rust | PUBLIC |    | 2026-06-16T00:33:58Z | Phenotype-org task orchestration |
| 41 | phenoShared | Rust | PUBLIC |    | 2026-06-16T00:07:37Z | Phenotype-org shared crates |
| 42 | phenotype-journeys | Rust | PUBLIC |    | 2026-06-16T00:03:17Z | Reusable journey harness (Rust CLI + Vue components + Playwright helper) for the Phenotype ecosystem |
| 43 | phenotype-ops-mcp | Python | PUBLIC |    | 2026-06-16T00:00:55Z | mcp server for working with unikernels using the nanos/ops toolchain |
| 44 | phenotype-landing | Astro | PUBLIC |    | 2026-06-16T00:00:46Z | Monorepo: all Phenotype org landing pages (odin, thegent, projects, byteport, agileplus, phenokits, hwledger) |
| 45 | phenotype-ts-utils | TypeScript | PUBLIC |    | 2026-06-16T00:00:38Z | Shared TypeScript utility library: cn, truncate, formatDate, debounce, deepMerge |
| 46 | PhenoContracts | TypeScript | PUBLIC |    | 2026-06-16T00:00:24Z | Phenotype contract verification — formal verification port + Prusti/Kani adapters + contract test bundle |
| 47 | phenotype-request-id | Python | PUBLIC |    | 2026-06-16T00:00:07Z | Shared Python request-id middleware (FastAPI/structlog/contextvars) |
| 48 | phenotype-py-utils | Python | PUBLIC |    | 2026-06-16T00:00:01Z | Shared Python utility library for the Phenotype org: load_config, setup_logging, parse_args, iso_now, truncate |
| 49 | phenotype-py-extras | Python | PUBLIC |    | 2026-06-15T23:59:55Z | Shared Python extras (cli/mcp/web/testing/observability) for the Phenotype ecosystem |
| 50 | localbase3 | HTML | PUBLIC |    | 2026-06-15T23:44:09Z | localbase3 — Localhost development environment bootstrap |
| 51 | Tracely | HTML | PRIVATE |    | 2026-06-15T23:43:11Z | Tracely observability primitives |
| 52 | Planify | TypeScript | PUBLIC |    | 2026-06-15T23:37:48Z | Canonical Plane fork for Phenotype/AgilePlus — project management UI (candidate frontend #1). Consolidated from main+mas |
| 53 | PhenoMCP | Python | PUBLIC |    | 2026-06-15T23:36:50Z | Phenotype-org MCP server |
| 54 | McpKit | Go | PUBLIC |    | 2026-06-15T23:35:57Z | MCP (Model Context Protocol) framework SDK for the Phenotype ecosystem |
| 55 | cheap-llm-mcp | Python | PRIVATE |    | 2026-06-15T23:29:30Z | MCP server for routing to budget LLM providers (Minimax, Kimi, Fireworks) |
| 56 | kmobile | Rust | PRIVATE |    | 2026-06-15T23:28:08Z | KMobile - Kompass for Mobile Development: Comprehensive CLI, API, and MCP server for mobile app development and testing  |
| 57 | phenotype-tooling | Svelte | PUBLIC |    | 2026-06-15T23:17:54Z | Phenotype org internal tooling: usage-poll, agent-forecast, temporal-grounding |
| 58 | Pine | Rust | PUBLIC |    | 2026-06-15T23:15:36Z | Wine-equivalent for Phenotype — Windows and cross-platform application compatibility layer |
| 59 | NetScript | Rust | PUBLIC | 🗄 | 2026-06-15T23:07:57Z | Rust lexer for NetScript |
| 60 | Sidekick | Python | PRIVATE |    | 2026-06-15T23:07:49Z | Phenotype-org agent presence |
| 61 | Tokn | Rust | PUBLIC |    | 2026-06-15T23:07:46Z | TokenLedger - LLM cost and usage tracking |
| 62 | phenoRouterMonitor | Rust | PUBLIC |    | 2026-06-15T23:01:42Z | LLM router with monitoring backend (Rust) and Pareto analysis dashboard (Streamlit) |
| 63 | KodeVibe | Go | PRIVATE |    | 2026-06-15T22:55:19Z | 🌊 The Ultimate Code Quality Guardian - Comprehensive tool that prevents bad vibes from entering your codebase |
| 64 | argis-extensions | Go | PUBLIC |    | 2026-06-15T22:55:10Z | Argis gateway extensions - routing, SLM, embeddings, plugin architecture |
| 65 | phenoData | Rust | PUBLIC |    | 2026-06-15T22:54:48Z | Pheno data-layer workspace: SurrealDB embedded bridge, Postgres+pgvector bridge, and unified pheno-query planner (Rust). |
| 66 | cliproxyapi-plusplus | HTML | PUBLIC |    | 2026-06-15T22:54:47Z | The Plus version of CLIProxyAPI |
| 67 | phenotype-water | C# | PUBLIC |    | 2026-06-15T22:53:36Z |  |
| 68 | phenotype-terrain | C# | PUBLIC |    | 2026-06-15T22:53:34Z |  |
| 69 | PhenoObservability | Rust | PUBLIC |    | 2026-06-15T19:48:44Z | Phenotype-org observability primitives |
| 70 | phenotype-postfx | C# | PUBLIC |    | 2026-06-15T19:46:37Z | Reusable BRP post-processing stack for Unity: SSAO, SSGI, Bloom, ACES, LUT |
| 71 | PhenoCompose | Rust | PRIVATE |    | 2026-06-15T08:22:23Z | Unified Process Compose with NVMS Support |
| 72 | AuthKit | Rust | PUBLIC |    | 2026-06-15T07:02:13Z | Authentication and security SDK for the Phenotype ecosystem |
| 73 | Apisync | Rust | PUBLIC |    | 2026-06-15T07:01:48Z | API synchronization and integration platform — auto-syncs OpenAPI/GraphQL schemas with versioning, conflict resolution,  |
| 74 | Dino | C# | PUBLIC |    | 2026-06-15T06:58:46Z | Dino — DINOForge general-purpose mod platform for AI agents |
| 75 | phenoUtils | Rust | PUBLIC |    | 2026-06-15T06:57:23Z | Foundational Rust utility crates for the Phenotype ecosystem: CLI shells, async filesystem, cryptography (ring/argon2),  |
| 76 | eyetracker | Kotlin | PUBLIC |    | 2026-06-15T06:56:09Z | Phenotype eye-tracking framework — Rust core + UniFFI bindings |
| 77 | PhenoVCS | Rust | PUBLIC |    | 2026-06-15T06:53:10Z | Version control primitives registry with Git worktree management |
| 78 | PhenoDevOps | Rust | PUBLIC |    | 2026-06-15T06:52:51Z | Phenotype PhenoDevOps domain workspace |
| 79 | phenoDesign | TypeScript | PUBLIC |    | 2026-06-15T06:27:34Z | Design tokens and VitePress theme package (@kooshapari/design); CSS keycap palette, components, and W3C DTCG tokens |
| 80 | phenotype-e2e-base | TypeScript | PUBLIC |    | 2026-06-15T06:27:31Z | Phenotype E2E testing harness — Playwright-driven landing page render tests for the fleet |
| 81 | phenotype-auth-ts | TypeScript | PUBLIC |    | 2026-06-15T06:27:29Z | TypeScript OAuth2/OIDC authentication patterns |
| 82 | portage | Python | PUBLIC |    | 2026-06-15T06:26:46Z | Harbor framework for agent evaluations and RL environments |
| 83 | helios-router | TypeScript | PRIVATE |    | 2026-06-15T06:13:38Z | Streamlit dashboard for Pareto analysis of LLM provider/model selection and ledger management |
| 84 | forgecode | Rust | PUBLIC |    | 2026-06-15T05:59:15Z | AI enabled pair programmer for Claude, GPT, O Series, Grok, Deepseek, Gemini and 300+ models |
| 85 | phenotype-voxel | Rust | PUBLIC |    | 2026-06-15T05:17:33Z | Adaptive voxel substrate for Phenotype-org games: SVO + dense leaf chunks, deterministic dirty queue, BRP-friendly strea |
| 86 | pheno-harness | Python | PRIVATE |    | 2026-06-15T02:29:28Z | Pheno compression stack, eval, RLVR harness (3090 Ti) |
| 87 | pheno-specs | — | PRIVATE |    | 2026-06-15T02:29:02Z | AgilePlus specs for pheno-harness (SPEC-001-016) |
| 88 | Melosviz | Python | PUBLIC |    | 2026-06-14T21:06:56Z | Music-to-visual generation toolkit: FastAPI backend, desktop (Tauri/Electrobun), Python + Rust SDKs, web client. |
| 89 | phenoAI | Rust | PUBLIC |    | 2026-06-14T20:37:06Z | Phenotype AI agent workspace and tooling |
| 90 | phenoResearchEngine | Python | PRIVATE |    | 2026-06-14T20:35:50Z | Phenotype research and investigation engine |
| 91 | KWatch | Go | PRIVATE |    | 2026-06-14T10:15:40Z | Kubernetes cluster monitoring and alerting system |
| 92 | Authvault | Rust | PUBLIC |    | 2026-06-14T08:52:51Z | Authentication and authorization framework with OAuth2, JWT, RBAC/ABAC, and multi-tenant support |
| 93 | WorldSphereMod | C# | PUBLIC |    | 2026-06-14T08:14:45Z | The 3D Worldbox Mod |
| 94 | Quillr | Shell | PUBLIC |    | 2026-06-13T17:42:32Z | Restored: Quillr |
| 95 | PhenoSpecs | TypeScript | PUBLIC |    | 2026-06-13T15:42:25Z | Phenotype Specification Registry - Central source of truth for specs, ADRs, and API contracts |
| 96 | Stashly | Rust | PUBLIC |    | 2026-06-13T10:26:59Z | Universal caching abstraction with TTL, multi-tier caching, singleflight, and multi-backend support |
| 97 | Pyron | Rust | PRIVATE |    | 2026-06-13T09:46:58Z | Python middleware and utilities library |
| 98 | agentapi-plusplus | Go | PUBLIC |    | 2026-06-13T08:35:37Z | HTTP API for Claude Code, Goose, Aider, Gemini, Amp, and Codex |
| 99 | phenoEvents | Rust | PUBLIC |    | 2026-06-13T08:05:57Z | PhenoEvents - EventBus port with hexagonal architecture |
| 100 | phenotype-registry | TypeScript | PUBLIC |    | 2026-06-13T06:53:48Z | Phenotype Registry System - Master index connecting specs, patterns, and templates |
| 101 | DataKit | — | PUBLIC |    | 2026-06-13T06:47:47Z | Storage and events SDK for the Phenotype ecosystem |
| 102 | phenodag | Go | PUBLIC |    | 2026-06-13T01:38:33Z | Multi-agent multi-project DAG (single-file Go, SQLite + flock) — built-in v3-180 preset, hybrid similarity, atomic claim |
| 103 | PhenoRuntime | Rust | PRIVATE | 🗄 | 2026-06-13T01:24:19Z | ARCHIVED: placeholder only |
| 104 | PhenoProc | Python | PUBLIC | 🗄 | 2026-06-13T01:23:31Z | Phenotype processor workspace for AI agent infrastructure and tools |
| 105 | PhenoKits | Python | PUBLIC | 🗄 | 2026-06-13T01:22:43Z | PhenoKits — Phenotype-org Python toolkit collection and template registry |
| 106 | Metron | Rust | PUBLIC |    | 2026-06-13T01:21:56Z | Metrics collection and reporting with Prometheus support |
| 107 | GDK | Rust | PUBLIC | 🗄 | 2026-06-13T01:10:43Z | GDK - General Development Kit (Rust monorepo) |
| 108 | Conft | TypeScript | PUBLIC |    | 2026-06-13T00:51:56Z | Phenotype configuration workspace (standalone) |
| 109 | phenotype-python-sdk | Python | PUBLIC |    | 2026-06-13T00:33:00Z | Phenotype-org Python SDK — consolidates McpKit/TestingKit/AuthKit/ResilienceKit/PhenoKits Python packages |
| 110 | Eventra | Rust | PRIVATE |    | 2026-06-13T00:32:17Z | Event-driven architecture framework with CQRS and Event Sourcing |
| 111 | phenoXdd | — | PUBLIC |    | 2026-06-12T23:58:33Z | 150+ software engineering methodologies and xDD patterns |
| 112 | PhenoHandbook | Shell | PUBLIC |    | 2026-06-12T23:27:19Z | Phenotype Handbook - Patterns, guidelines, anti-patterns, and methodologies for building software |
| 113 | Configra | — | PRIVATE |    | 2026-06-12T23:27:11Z | Phenotype-org configuration framework |
| 114 | phenoForge | Shell | PUBLIC | 🗄 | 2026-06-12T23:26:59Z | A Rust-native task runner with parallel execution, dependency graph resolution, hot reload, and a powerful plugin system |
| 115 | phenotype-infra | Rust | PUBLIC |    | 2026-06-12T23:26:09Z | Phenotype compute mesh: OCI + CF + GCP + AWS + Vercel + TS — ADRs, specs, IaC, runbooks |
| 116 | DevHex | Go | PRIVATE |    | 2026-06-12T23:23:56Z | Hexagonal Go library for dev environment abstractions (Docker, Podman, Nix, process-compose) |
| 117 | PhenoPlugins | Rust | PUBLIC |    | 2026-06-12T23:23:30Z | Phenotype-org plugin framework |
| 118 | phenotype-go-sdk | Go | PUBLIC |    | 2026-06-12T23:18:08Z | Phenotype-org Go SDK — consolidates PlatformKit/DevHex/McpKit Go packages |
| 119 | phenotype-dep-guard | Python | PUBLIC |    | 2026-06-12T17:54:11Z | Dependency guard toolchain and audit utility |
| 120 | phenotype-org-governance | Shell | PRIVATE |    | 2026-06-12T11:26:41Z | Phenotype-org governance, audits, dashboards, and policy |
| 121 | netweave-final2 | HTML | PRIVATE |    | 2026-06-12T11:26:07Z |  |
| 122 | Logify | Rust | PRIVATE |    | 2026-06-12T11:26:02Z | Structured logging framework with zero-cost abstraction and multiple sinks |
| 123 | agent-user-status | Python | PRIVATE |    | 2026-06-12T11:25:06Z | An internal tool or component for the Phenotype ecosystem. |
| 124 | DINOForge-UnityDoorstop | C | PUBLIC |    | 2026-06-12T11:17:19Z | Doorstop -- run C# before Unity does! |
| 125 | Zerokit | — | PUBLIC |    | 2026-06-12T10:06:34Z | Restored: Zerokit |
| 126 | spec-kitty | — | PUBLIC |    | 2026-06-12T10:06:21Z | Spec-driven development workflow for Phenotype — 14 slash commands, kitty-specs lifecycle |
| 127 | services | — | PUBLIC |    | 2026-06-12T10:06:17Z | Phenotype services registry — CycloneDX SBOMs for fleet third-party services |
| 128 | phenotype-zod-schemas | TypeScript | PUBLIC |    | 2026-06-12T10:06:00Z | Phenotype Zod schema bundle — shared TypeScript runtime validators for fleet frontends |
| 129 | phenotype-templates | — | PUBLIC |    | 2026-06-12T10:05:53Z | Phenotype org template — use this as the base for new repos |
| 130 | phenotype-runs | — | PUBLIC |    | 2026-06-12T10:05:52Z | Universal CI/job observability substrate for the Phenotype org |
| 131 | phenotype-otel | Rust | PUBLIC |    | 2026-06-12T10:05:43Z | Phenotype OpenTelemetry bridge — single-call OTLP + tracing-subscriber init |
| 132 | phenotype-gates | — | PUBLIC |    | 2026-06-12T10:05:35Z | Policy-as-code gate engine for the Phenotype org |
| 133 | phenoShared-niche | Rust | PUBLIC |    | 2026-06-12T10:05:28Z | Niche sub-crates from phenoShared — only consumed by 1-2 dependents |
| 134 | phenokits-commons | Python | PUBLIC |    | 2026-06-12T10:05:18Z | Phenotype org cross-cutting commons: governance templates, hexagon ADRs, shared libs (go/ts) — extracted from the Python |
| 135 | kwality | Go | PUBLIC |    | 2026-06-12T10:04:51Z | STRICTLY DO NOT DELETE NOR UNARCHIVE - Personal Project - LLM validation platform |
| 136 | dagctl | Go | PUBLIC |    | 2026-06-12T10:04:34Z | Multi-agent multi-project DAG (v3): SQLite + flock, 20×6 + side-DAGs, hybrid similarity, atomic claims, mangled-git scan |
| 137 | Compound-Spheres-3D | C# | PUBLIC |    | 2026-06-12T10:04:31Z | WSM3D fork of MelvinShwuaner/Compound-Spheres with height-field renderer + perf fixes |
| 138 | AppGen | TypeScript | PUBLIC |    | 2026-06-12T10:04:22Z | STRICTLY DO NOT DELETE NOR UNARCHIVE - Personal project template for rapid app prototyping |
| 139 | agileplus-spec-harmonizer | Rust | PUBLIC |    | 2026-06-12T10:04:19Z | Harmonizer for GSD, OpenSpec, BMAD-Method, and Spec-Kitty spec formats → unified WorkPackage shape. 12/12 tests pass. Pu |
| 140 | phenoXddLib | Rust | PUBLIC |    | 2026-06-12T09:24:45Z | Cross-cutting xDD utilities library: property testing, contract verification, mutation coverage |
| 141 | Paginary | Rust | PRIVATE | 🗄 | 2026-06-12T07:48:20Z | Paginary — pagination and caching utilities |
| 142 | chatta | Svelte | PUBLIC | 🗄 | 2026-06-12T07:47:19Z | chatta — Svelte-based chat application and UI components |
| 143 | ResilienceKit | Python | PUBLIC | 🗄 | 2026-06-12T05:55:03Z | Circuit breakers and retry SDK for the Phenotype ecosystem |
| 144 | ObservabilityKit | Python | PUBLIC | 🗄 | 2026-06-12T05:54:40Z | Logging & metrics SDK for Phenotype |
| 145 | vibeproxy | Swift | PUBLIC |    | 2026-06-11T11:07:23Z | Deprecated fork of automaze.io VibeProxy — native macOS menu-bar app routing AI coding tools through Claude/ChatGPT/Gemi |
| 146 | hwLedger | — | PUBLIC |    | 2026-06-11T09:47:01Z | Phenotype-org hardware ledger |
| 147 | KaskMan | JavaScript | PRIVATE | 🗄 | 2026-06-11T05:37:14Z | KaskManager R&D Platform - Persistent, Always-On, Self-Improving Utility Platform with CLI, API, MCP, and Advanced R&D C |
| 148 | dinoforge-packs | Go | PUBLIC |    | 2026-06-10T07:34:08Z | Resource packs for DINOForge platform |
| 149 | foqos-private | Swift | PRIVATE |    | 2026-06-08T09:13:35Z | Private mirror of awaseem/foqos for FocalPoint donor use (MIT) |
| 150 | helios-cli | Python | PUBLIC |    | 2026-06-05T03:35:33Z | Phenotype-org multi-runtime CLI |
| 151 | KommandLineAutomation | Rust | PRIVATE |    | 2026-06-02T23:55:29Z | KLA - A Playwright equivalent for beautiful CLI recordings |
| 152 | bifrost | Go | PUBLIC |    | 2026-06-02T23:22:20Z | Fastest enterprise AI gateway (50x faster than LiteLLM) with adaptive load balancer, cluster mode, guardrails, 1000+ mod |
| 153 | agslag-docs | — | PRIVATE | 🗄 | 2026-06-02T23:19:24Z |  |
| 154 | argisexec | — | PRIVATE | 🗄 | 2026-05-30T02:40:15Z |  |
| 155 | acp | — | PUBLIC | 🗄 | 2026-05-30T02:40:07Z |  |
| 156 | Settly | Rust | PUBLIC | 🗄 | 2026-05-29T05:16:57Z | Universal configuration management with layered configs, validation, and environment-aware settings |
| 157 | phenotype-omlx | Python | PUBLIC | 🗄 | 2026-05-29T01:05:07Z | LLM inference server with continuous batching & SSD caching for Apple Silicon — managed from the macOS menu bar |
| 158 | vibeproxy-monitoring-unified | — | PUBLIC | 🗄 | 2026-05-28T08:36:09Z | Unified monitoring scaffold for VibeProxy ecosystem — pre-implementation governance/spec stubs (AGENTS, CLAUDE, FR, SPEC |
| 159 | phenotype-hub | JavaScript | PUBLIC | 🗄 | 2026-05-28T08:36:02Z | Phenotype org hub repository — governance scaffold (AGENTS, CLAUDE, FR templates); pre-foundational, no implementation y |
| 160 | thegent-workspace | Rust | PRIVATE | 🗄 | 2026-05-28T08:35:13Z | Phenotype-org agent workspace |
| 161 | TestingKit | Python | PUBLIC | 🗄 | 2026-05-28T08:29:22Z | Test utilities SDK for Phenotype |
| 162 | PlatformKit | Go | PUBLIC | 🗄 | 2026-05-21T00:29:00Z | Phenotype PlatformKit — Go devenv and devhex tooling |
| 163 | KodeVibeGo | Go | PUBLIC | 🗄 | 2026-05-07T19:05:29Z | STRICTLY DO NOT DELETE NOR UNARCHIVE - Personal Project - Phenotype ecosystem component (deprecated, consolidated into H |
| 164 | phenoStandards | — | PUBLIC | 🗄 | 2026-05-07T19:04:02Z | DEPRECATED: Empty skeleton - standards in KooshaPari/HexaKit/governance/ |
| 165 | Traceon | Rust | PUBLIC | 🗄 | 2026-05-07T19:03:12Z | Distributed tracing framework with OpenTelemetry support |
| 166 | agentapi | — | PRIVATE | 🗄 | 2026-05-07T19:02:58Z | AgentAPI - Unified API gateway for AI agent orchestration and tool integration |
| 167 | pheno-sdk | — | PRIVATE | 🗄 | 2026-05-07T19:02:18Z | ATOMS-PHENO SDK for infrastructure migration and operations - Private Package Repository |
| 168 | odin-landing | HTML | PUBLIC | 🗄 | 2026-05-07T05:55:01Z |  |
| 169 | worktree-manager | Rust | PUBLIC | 🗄 | 2026-05-04T18:42:36Z | Git worktree management tool |
| 170 | projects-landing | Astro | PUBLIC | 🗄 | 2026-05-03T21:22:58Z | Phenotype org portfolio (auto-generated from gh repo list) |
| 171 | helios-cli-backup | Rust | PRIVATE | 🗄 | 2026-05-03T03:48:44Z | DEPRECATED: Backup/old version - use KooshaPari/HexaKit/helios-cli |
| 172 | phenotype-colab-extensions | — | PRIVATE | 🗄 | 2026-05-03T03:48:43Z | Phenotype extensions for colab fork - AgilePlus specs, webflow-plugin |
| 173 | Prismal | TypeScript | PRIVATE | 🗄 | 2026-05-03T03:48:42Z | Minimal, accessible React component library |
| 174 | KVirtualStage | Go | PRIVATE | 🗄 | 2026-05-03T03:48:40Z | STRICTLY DO NOT DELETE NOR UNARCHIVE - Personal Project - Desktop automation platform for AI agents |
| 175 | PhenoLang | Rust | PRIVATE | 🗄 | 2026-05-02T22:22:20Z | ARCHIVED: superseded by KooshaPari/phenoUtils |
| 176 | atoms.tech | TypeScript | PRIVATE | 🗄 | 2026-04-30T18:07:01Z |  |
| 177 | Profila | Python | PUBLIC | 🗄 | 2026-04-04T07:05:16Z | Unified system and code profiling toolkit for the Phenotype agent ecosystem - CPU, memory, and I/O analysis |
| 178 | .github | — | PUBLIC | 🗄 | 2026-04-04T07:02:43Z |  |
| 179 | phenoVessel | HTML | PRIVATE | 🗄 | 2026-04-03T07:53:19Z | DEPRECATED: Merged into KooshaPari/PhenoPlugins as pheno-plugin-vessel crate |
| 180 | phenoTypes | Python | PRIVATE | 🗄 | 2026-04-03T03:56:02Z | DEPRECATED: Empty skeleton - type definitions in KooshaPari/HexaKit/python/ |
| 181 | phenoPatch | — | PRIVATE | 🗄 | 2026-03-27T11:16:20Z | Phenotype phenotype-patch library |
| 182 | forge | Rust | PRIVATE | 🗄 | 2026-03-25T16:19:35Z | Modern CLI task runner and build orchestrator |
| 183 | Diffuse | — | PRIVATE | 🗄 | 2026-03-25T16:19:33Z | Unified diff and patch library |
| 184 | Cryptora | — | PRIVATE | 🗄 | 2026-03-25T16:19:30Z | Simple, safe cryptography |
| 185 | Servion | — | PRIVATE | 🗄 | 2026-03-25T16:19:28Z | Service registry and discovery for microservices |
| 186 | Guardrail | — | PRIVATE | 🗄 | 2026-03-25T16:19:27Z | Rate limiting, circuit breaking, and bulkhead isolation |
| 187 | tehgent | — | PUBLIC | 🗄 | 2026-02-24T08:42:55Z | TehGent - AI-powered code review and development assistant |
| 188 | router-docs | — | PRIVATE | 🗄 | 2025-12-01T02:01:33Z |  |
| 189 | P2 | Python | PRIVATE | 🗄 | 2025-11-25T07:31:53Z |  |
| 190 | 472-P2-Flame-War | Python | PRIVATE | 🗄 | 2025-11-25T03:54:09Z |  |
| 191 | RIP-Fitness-App | Kotlin | PUBLIC | 🗄 | 2025-07-10T10:15:03Z | 🏋️‍♂️ Complete fitness app combining MacroFactor adaptive nutrition, Strong workout tracking, and comprehensive health i |
| 192 | go-nippon | — | PRIVATE | 🗄 | 2025-04-18T22:57:35Z |  |
| 193 | agslag | JavaScript | PRIVATE | 🗄 | 2025-04-15T02:17:30Z |  |
| 194 | agslag-dash | TypeScript | PRIVATE | 🗄 | 2025-04-13T19:32:38Z |  |
| 195 | model-conductor-hub | TypeScript | PRIVATE | 🗄 | 2025-04-12T09:56:01Z |  |
| 196 | canvasApp | Python | PRIVATE | 🗄 | 2024-08-26T04:21:00Z |  |
| 197 | Project-Spyn | MATLAB | PUBLIC | 🗄 | 2023-11-28T20:18:13Z | ASU FSE100 Project |

## Phase plan

- **Phase 3**: Score OmniRoute (this reference repo) with cited evidence per pillar.
- **Phase 4**: Fan out subagent scoring to the other 196 repos (5 in parallel, ≤30 concurrent max).
- **Phase 5**: Emit `docs/audits/<repo>/ACTION-PLAN.md` for each.
- **Phase 6**: Roll-up report + backlog.
- **Phase 7**: Wire audit ratchet (CI gate + quarterly cron).

## Source

```
gh repo list KooshaPari --limit 999 --json name,visibility,isArchived,pushedAt,description,primaryLanguage
```

Captured 2026-06-16, cached at `/tmp/repo-inv.tsv`.
