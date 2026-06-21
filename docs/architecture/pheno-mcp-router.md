# pheno-mcp-router — Architecture Overview (L1)

**Tier:** `pheno-*-lib` (per ADR-023 substrate placement; `linter-output.json`
  `inferred_tier: pheno-*-lib`, with `PROMOTION.md` documenting sdk-tier
  readiness: 2+ polyglot consumers, stable API since 0.1.0)
**Pillar:** 71-pillar L1 (Architecture Overview), Cycle 7 closure
**Language:** Python (FastMCP-based)
**Source:** `src/pheno_mcp_router/` (per `KooshaPari/pheno-mcp-router`
  meta-bundle patch `0001-chore-meta-bundle-batch-with-config-module.patch`)
**Date:** 2026-06-21

## L1 Context

`pheno-mcp-router` is the **canonical MCP (Model Context Protocol) router
substrate** for the Phenotype fleet. It is a Python package that wraps a
typed `McpRouter` dataclass around `fastmcp.FastMCP`, applying per-tier
cost / budget / quota enforcement, payload sanitization, and response
allowlisting before dispatching to a backend LLM.

The default backend is **OmniRoute** (`http://localhost:20128/v1/chat/completions`,
overridable at router construction time). Two polyglot consumers exist
today: `dispatch-mcp` (Python) and `phenotype-config` (Rust-side via
`LlmPort` trait re-use).

```
Application / MCP server
        │  McpRouter(name=..., backend_url=...)
        ▼
┌──────────────────────────────────────────────────────────┐
│  pheno-mcp-router  (this package)                        │
│   __init__.py     McpRouter dataclass + serve()          │
│   ports.py        LlmAdapter / StorageAdapter / Tool…    │
│   adapters.py     OpenAIAdapter, AnthropicAdapter,       │
│                    LlamaAdapter, OpenAICompatAdapter     │
│   config.py       Centralized defaults (env-overridable) │
│   cost.py         Pre-dispatch cost calculator           │
│   budget.py       Per-tier budget enforcer               │
│   quota.py        Rolling-window quota tracker           │
│   tiers.py        TierRegistry + TierPricing             │
│   cli.py          `pheno-mcp-router init` scaffold        │
└──────────────────────────────────────────────────────────┘
        │
        ▼  httpx POST {backend_url}/v1/chat/completions
   OmniRoute / OpenAI / Anthropic / llama.cpp / OpenAI-compat
```

## C4 Container view

```
┌──────────────────────────────────────────────────────────────────────────┐
│  Application / MCP server (consumer of pheno-mcp-router)                │
│   router = McpRouter(name="x", backend_url=OMNIROUTE)                   │
│   router.add_tier("default", {"model": "minimax-m2p7"})                 │
│   router.serve()                                                        │
└────────────────────────────────┬─────────────────────────────────────────┘
                                 │
                                 ▼
┌──────────────────────────────────────────────────────────────────────────┐
│  pheno-mcp-router (this package — src/pheno_mcp_router/)                │
│                                                                          │
│  ┌────────────────────────────┐    ┌─────────────────────────────────┐    │
│  │ __init__.py                │    │ config.py                       │    │
│  │  @dataclass McpRouter      │───▶│   SECURITY-SENSITIVE:           │    │
│  │  ToolFn, serve(),          │    │     DEFAULT_SANITIZE_KEYS       │    │
│  │  add_tier()                │    │     DEFAULT_RESPONSE_KEYS       │    │
│  │  Tool / TierRoute          │    │   Env-overridable:              │    │
│  │                            │    │     PHENO_TIMEOUT_SECONDS       │    │
│  │  Uses:                     │    │     PHENO_MAX_MESSAGE_BYTES     │    │
│  │    httpx (HTTP client)     │    │     PHENO_MAX_RESPONSE_BYTES    │    │
│  │    fastmcp.FastMCP         │    │   LLM defaults:                 │    │
│  │                            │    │     OPENAI_BASE_URL, etc.       │    │
│  └────────────────────────────┘    └─────────────────────────────────┘    │
│                                                                          │
│  ┌────────────────────────────┐    ┌─────────────────────────────────┐    │
│  │ ports.py (abstract)        │    │ adapters.py (concrete)          │    │
│  │   LlmAdapter   ──abstract──┼───▶│   OpenAIAdapter                 │    │
│  │   StorageAdapter            │    │   AnthropicAdapter              │    │
│  │   ToolAdapter               │    │   LlamaAdapter (PR #2)          │    │
│  └────────────────────────────┘    │   OpenAICompatAdapter (PR #3)   │    │
│                                    └─────────────────────────────────┘    │
│                                                                          │
│  ┌──────────────────────────────────────────────────────────────────┐    │
│  │ Cross-cutting concerns                                           │    │
│  │   cost.py      CHARS_PER_TOKEN=4, output-token fallback          │    │
│  │   budget.py    UNPRICED_FLOOR_USD=1.00  (BudgetExceeded)         │    │
│  │   quota.py     MIN_WINDOW_SECONDS=1.0, DEFAULT_WINDOW=60.0       │    │
│  │   tiers.py     TierRegistry + UNKNOWN_PRICING + DEFAULT_REGISTRY │    │
│  │   cli.py       `pheno-mcp-router init` (click) scaffolds server  │    │
│  └──────────────────────────────────────────────────────────────────┘    │
│                                                                          │
│  Tests: test_ports.py, test_adapters.py, test_budget.py, ...             │
└──────────────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼  HTTP POST {backend_url}/v1/chat/completions
                ┌─────────────────────────────────────────────────┐
                │  External LLM backends                          │
                │   - OmniRoute (default, http://localhost:20128) │
                │   - OpenAI, Anthropic, llama.cpp, OpenAI-compat │
                └─────────────────────────────────────────────────┘
```

**Container responsibilities:**

| Container | Responsibility |
|-----------|----------------|
| `__init__.py` | `McpRouter` dataclass; `add_tier()` tier allowlist enforcement; `serve()` glue to `fastmcp.FastMCP`. |
| `config.py` | Single source of truth for tunable defaults. Security-sensitive (`DEFAULT_SANITIZE_KEYS`, `DEFAULT_RESPONSE_KEYS`) are frozen sets and NOT env-overridable; tunables (`TIMEOUT_SECONDS`, `MAX_MESSAGE_BYTES`, `MAX_RESPONSE_BYTES`) are env-overridable via `PHENO_*`. |
| `ports.py` | Abstract port traits: `LlmAdapter`, `StorageAdapter`, `ToolAdapter`. The `LlmAdapter` is the polyglot contract — both `dispatch-mcp` and `phenotype-config` consume it. |
| `adapters.py` | Concrete `LlmAdapter` impls: `OpenAIAdapter`, `AnthropicAdapter`, `LlamaAdapter` (PR #2), `OpenAICompatAdapter` (PR #3). |
| `cost.py` | Pre-dispatch cost estimator. CHARS_PER_TOKEN=4 conservative. |
| `budget.py` | Per-tier budget enforcer; UNPRICED_FLOOR_USD=1.00 stops unregistered tiers from bypassing with cost=0. |
| `quota.py` | Rolling-window quota tracker; MIN_WINDOW_SECONDS=1.0 clamp prevents callers from disabling via absurdly short window. |
| `cli.py` | `click`-based CLI; `init` subcommand scaffolds a runnable server from a template. |

## Key decisions

1. **Centralized config module.** All hardcoded magic numbers were
   migrated to `config.py` in patch `0001-chore-meta-bundle-batch-with-
   config-module.patch` (`__init__.py:30-42`, `adapters.py:50-53`,
   `budget.py`, `cost.py`, `quota.py`). Operators have one place to
   inspect, document, or override defaults.
2. **Two-tier config sensitivity.** `DEFAULT_SANITIZE_KEYS` and
   `DEFAULT_RESPONSE_KEYS` are **security-sensitive** — frozen sets, NOT
   env-overridable, gated by `AGENTS.md §"Do Not Touch"` per the SPEC.md
   configuration section. Other values are env-overridable at import
   time. This split is the substrate's most important contract.
3. **Hexagonal ports, not inheritance.** Adapters implement abstract
   `LlmAdapter` (in `ports.py`); consumers depend only on the trait.
   `OpenAICompatAdapter` (PR #3, 17 tests, 87 % coverage) was added
   specifically to validate this shape against a heterogeneous backend
   set.
4. **Tier allowlist enforced at construction.** Per the SPEC.md
   security model, the router requires `add_tier()` to be called
   before any dispatch; unregistered tiers cannot bypass the budget
   enforcer (UNPRICED_FLOOR_USD = $1.00 default floor).
5. **OmniRoute as default backend.** Default `backend_url` is
   `http://localhost:20128/v1/chat/completions` — a deliberate
   dependency on the federated OmniRoute router (ADR-023 federated
   service). Overridable per-router at construction time.
6. **Tier = `pheno-*-lib`, with sdk promotion documented.** The
   `linter-output.json` notes (in archived patch) flag PROMOTION.md
   evidence: 2+ polyglot consumers (dispatch-mcp + phenotype-config),
   stable API since 0.1.0 (2026-06-11), 0 breaking changes, CHANGELOG
   per Keep-a-Changelog 1.1.0 + SemVer 2.0.0. Promotion to
   `phenotype-*-sdk` is on the v18+ roadmap (ADR-048 graduation path).

## Future-state

- **v18 (Cycle 8) — sdk graduation.** Per ADR-048, fork the polyglot
  surface (`LlmAdapter` port + central `config.py`) into a stable
  `phenotype-mcp-router-sdk` while `pheno-mcp-router` stays the
  Python core. Rust consumer (`phenotype-config`) bridges via UniFFI.
- **v19 — OpenTelemetry spans.** Add `pheno-tracing` (Rust) / OTLP
  Python exporter for: `add_tier` events, per-dispatch cost calc,
  `BudgetExceeded` / `QuotaExceeded` rejections, and adapter-level
  request/response durations. Required for L8 (inter-service
  contracts) closure.
- **v20+ — Tier promo to framework.** If a router-level lifecycle
  (startup banner, graceful shutdown, plugin SDK) accretes, promote
  to `phenotype-*-framework` per ADR-048. Today the lifecycle is too
  thin to warrant the framework tier.

## Cross-references

- **ADR-013** (pheno-mcp-router substrate canonical, superseded by ADR-037)
- **ADR-023** (Agent-effort governance — substrate placement)
- **ADR-037** (pheno-mcp-router substrate canonical — re-affirmed for v8 sweep)
- **ADR-040** (Test coverage gates per tier — 80 % lib)
- **ADR-047** (Predictive DRY discipline)
- **ADR-048** (Substrate graduation path)
- **L5-104 absorption PRs:** `pheno-mcp-router#1` (cost/budget/quota/audit/tiers),
  `#2` (LlamaAdapter), `#3` (OpenAICompatAdapter)
- **Source patches:**
  `findings/archived-repo-local-patches/2026-06-20/pheno-mcp-router/0001-chore-meta-bundle-batch-with-config-module.patch`
  (config centralization + 3 module imports + SPEC.md updates),
  `0002-chore-orch-v12-s1-010-full-tier-0-hygiene-governance.patch`
  (`.editorconfig`, `.gitattributes`, 5 GitHub Actions workflows)
- **Default backend:** `pheno-mcp-router/src/pheno_mcp_router/config.py:390`
  (`DEFAULT_BACKEND_URL = "http://localhost:20128/v1/chat/completions"`)
- **Repo:** `KooshaPari/pheno-mcp-router` (main branch, single README.md
  currently; full source on the 3 absorbed feature branches per L5-104)
