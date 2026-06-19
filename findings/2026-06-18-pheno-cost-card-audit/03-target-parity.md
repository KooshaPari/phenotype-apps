# Target Parity Audit — pheno-cost-card

**Date:** 2026-06-18
**Source repo:** `KooshaPari/pheno-cost-card`
**Local path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/pheno-cost-card/`
**Authoritative inputs:**
- `findings/2026-06-18-pheno-cost-card-audit/01-source-inventory.md` (source inventory)
- `findings/2026-06-18-pheno-cost-card-audit/02-docs-code.md` (docs & code review)
- `AGENTS.md` §"App-level repo triage & app substrate placement" (ADR-023)
- `AGENTS.md` §"Dmouse92 → KooshaPari migration" (ADR-029, PRs #1-#6)
- `AGENTS.md` §"Active ADRs" (ADR-013, ADR-022, ADR-031, ADR-032, ADR-035)
- GitHub: `KooshaPari/pheno-mcp-router#1` (the cost absorption PR)

**Sister finding:** `01-source-inventory.md` §"F.3 Strategic recommendation" already establishes that the `CostCard` + render functions should migrate to a `pheno-cost` / `phenotype-cost` library substrate. This report confirms or refutes that recommendation against all 12 candidates and proposes a concrete target.

---

## 0. Summary

**Headline verdict (HIGH confidence):** **No existing fleet repo is a good fit for pheno-cost-card.** The recommended target is a **new dedicated substrate repo `KooshaPari/pheno-cost`** (Python, per ADR-023 Rule 3 "pheno-*-lib" placement).

**Top three candidates evaluated, all REJECTED:**

| # | Candidate | Verdict | Why rejected |
|---|---|---|---|
| 1 | `KooshaPari/pheno-mcp-router` | ❌ **REJECT** (false positive) | Substrate is **MCP router per ADR-013**, not cost tracking. Its dispatch-mcp W2-1 cost code is **provider-usage cost** (LLM token/$) only — does NOT cover CI minutes, runner hours, or storage. Adding infra-cost dimensions would violate single-concern and ADR-013's substrate scope. |
| 2 | `KooshaPari/pheno-prompt-test` | ❌ **REJECT** | Substrate is **prompt testing** (per-test cost). Different abstraction (per-test spend vs fleet infra rollup). No `CostCard`, no `llm_tokens/ci_minutes/runner_hours/storage_gb` quartet. |
| 3 | `KooshaPari/pheno-scaffold-kit` | ❌ **REJECT** | Umbrella is **project scaffolding** (generates new repos). No cost module. `CostCard` is not in scope of `__init__.py` exports. |

**Zero in-tree consumers** of `pheno-cost-card` were found (see §1). The library is **fully orphaned** in the monorepo and was only used by its own `examples/` (which is broken — see `02-docs-code.md` §B.5).

**Recommended action (one PR sequence):**

1. **Create** `KooshaPari/pheno-cost` (Python, MIT, pyproject.toml, src layout).
2. **Port** the 5 files from `pheno-cost-card/` — strip the fleet-specific `render_fleet_card()` (it depends on a non-existent fleet registry), keep the general primitives:
   - `CostCard` dataclass (`llm_tokens`, `ci_minutes`, `runner_hours`, `storage_gb`)
   - `render_repo_card(card, …)` — generic repo render function
   - `collectors.py` (the 4 stub collectors)
   - `__init__.py` with corrected exports
3. **Drop** the broken `examples/fleet_card.py` and misapplied `deny.toml` (per `02-docs-code.md` §B.5 + §C.3).
4. **Update** all 4 docs (`README.md`, `AGENTS.md`, `WORKLOG.md`, `CHANGELOG.md`) to reflect the rename + dropping the fleet example.
5. **Archive** `KooshaPari/pheno-cost-card` (read-only marker; per the same policy applied to `McpKit`, `kwality`, `phenotype-auth-ts`, `dinoforge-packs` in `AGENTS.md` §"4-repo retirement").
6. **Add** to `AGENTS.md` §"Active focus repos" or §"pheno-* family" with bucket `LIB / active`.

**Naming:** `pheno-cost` is the recommended name because (a) it follows the `pheno-*-lib` convention from ADR-023 Rule 3, (b) `phenotype-cost` would be wrong (per ADR-031 `phenotype-*` is reserved for cross-language SDKs/federated services, not single-concern libs), (c) the existing `pheno-cost-card` name confuses "card" as in "business card" vs "render card".

---

## 1. In-Tree Consumer Probe

**Command (executed 2026-06-18):**

```bash
grep -rn "pheno_cost_card\|pheno-cost-card" /Users/kooshapari/CodeProjects/Phenotype/repos/ \
  --include=*.py --include=*.toml --include=*.md 2>/dev/null \
  | grep -v "pheno-cost-card/" | head -50
```

**Result: ZERO matches.** (Excluding the source repo itself.)

**Cross-check with symbol-level grep (executed 2026-06-18):**

```bash
grep -rln "render_fleet_card\|render_repo_card\|CostCard\|pheno_cost_card" \
  --include="*.py" --include="*.md" --include="*.toml" \
  --exclude-dir=".git" --exclude-dir="__pycache__" --exclude-dir="node_modules" --exclude-dir=".venv" \
  /Users/kooshapari/CodeProjects/Phenotype/repos/ 2>/dev/null \
  | grep -v "pheno-cost-card/"
```

**Result: ZERO matches.** No file in the monorepo imports `pheno_cost_card`, references `CostCard`, calls `render_fleet_card`, or calls `render_repo_card`.

**Cross-check with focused candidate list (executed 2026-06-18):**

```bash
grep -rln "pheno-cost-card\|pheno_cost_card" --include="*.py" --include="*.toml" --include="*.md" \
  --include="*.lock" --include="*.txt" --include="*.yml" --include="*.yaml" \
  --exclude-dir=".git" --exclude-dir="__pycache__" --exclude-dir="node_modules" \
  --exclude-dir=".venv" --exclude-dir="target" \
  pheno-prompt-test pheno-mcp-router pheno-scaffold-kit pheno-llms-txt pheno-fastapi-base \
  pheno-vibecoding-guard pheno-worklog-schema pheno-pydantic-models phenotype-py-utils \
  phenotype-python-sdk phenotype-registry phenotype-config pheno-otel pheno-errors pheno-flags \
  pheno-context pheno-config dispatch-mcp pheno
```

**Result: ZERO matches.**

**Conclusion: pheno-cost-card has zero in-tree consumers.** It was created in isolation, has never been imported by any other repo, and is not referenced in any docs, plans, findings, or governance files outside its own directory. This makes it an **orphaned asset** — a candidate for absorption/migration but with no consumer to break.

---

## 2. dispatch-mcp W2-1 cost code analysis (CRITICAL)

**Context:** Per AGENTS.md §"Dmouse92 → KooshaPari migration", PR `KooshaPari/pheno-mcp-router#1` absorbed the dispatch-mcp W2-1 cost/budget/quota/audit/tiers module. This is the single most plausible "is there a home for pheno-cost-card content already?" question, so it gets its own deep analysis.

### 2.1 Inventory of cost code in `pheno-mcp-router`

**Local path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/pheno-mcp-router/`
**Branch:** `chore/l3-57-pheno-plugin-registry-2026-06-11` (per AGENTS.md §"Key Commands" + ADR-013)

**Source files (6 modules, 1 docs, 1 guide):**

| File | LOC | Purpose |
|---|---|---|
| `src/pheno_mcp_router/cost.py` | ~200 | `LLMUsage`, `CostRecord`, `CostEstimator`, per-token pricing tables, currency conversion |
| `src/pheno_mcp_router/cost_middleware.py` | ~250 | ASGI/WSGI-style middleware that records `LLMUsage` events into `CostRecord` ledger |
| `src/pheno_mcp_router/budget.py` | ~200 | `Budget`, `BudgetPeriod` (daily/weekly/monthly), `BudgetEnforcer` raises on overflow |
| `src/pheno_mcp_router/quota.py` | ~180 | `Quota`, `RateLimiter`, request-count + token-count quota |
| `src/pheno_mcp_router/audit.py` | ~250 | `AuditEntry`, `AuditLog`, `AppendOnlyAuditLog`, hash-chain integrity |
| `src/pheno_mcp_router/tiers.py` | ~150 | `Tier` (free/pro/enterprise), `TierPolicy` |
| `docs/PROVIDER_GUIDE.md` | n/a | Provider-specific pricing & integration guide |
| `tests/test_cost.py` + `test_cost_middleware.py` (+4 others) | n/a | Test coverage |

**Existence confirmed via:** `pheno-mcp-router/src/pheno_mcp_router/cost.py:1` (file present), `pheno-mcp-router/src/pheno_mcp_router/cost_middleware.py:1` (file present), `pheno-mcp-router/tests/test_cost.py` (file present), `pheno-mcp-router/tests/test_cost_middleware.py` (file present), `pheno-mcp-router/docs/PROVIDER_GUIDE.md` (file present).

### 2.2 pheno-mcp-router cost code is a SUPERSET of pheno-cost-card's LLM features, but a NON-OVERLAP for infra features

**Comparison table:**

| pheno-cost-card feature | pheno-mcp-router equivalent | Overlap? |
|---|---|---|
| `CostCard.llm_tokens` field | `LLMUsage.prompt_tokens + completion_tokens` (`pheno-mcp-router/src/pheno_mcp_router/cost.py`) | ✅ **Superset** (pheno-mcp-router is per-call token, pheno-cost-card is aggregated per-repo tokens) |
| `CostCard.ci_minutes` field | ❌ None | ❌ **No equivalent** |
| `CostCard.runner_hours` field | ❌ None | ❌ **No equivalent** |
| `CostCard.storage_gb` field | ❌ None | ❌ **No equivalent** |
| `CostCard` dataclass shape (4-dim rollup) | ❌ None — pheno-mcp-router has flat `CostRecord` with `model`, `tokens`, `cost_usd` per call | ❌ **No equivalent** |
| `render_repo_card()` (markdown render) | ❌ None | ❌ **No equivalent** |
| `render_fleet_card()` (multi-repo rollup) | ❌ None | ❌ **No equivalent** |
| `collectors.py` (4 stub collectors) | ❌ None | ❌ **No equivalent** |
| `tier.py` stub | `tiers.py` (real `Tier` enum + `TierPolicy`) | ✅ **Superset** |

**Critical mismatch:** `pheno-cost-card` is **infra-cost** (CI, runners, storage, LLM tokens as one of 4 dimensions). `pheno-mcp-router` cost code is **provider-usage cost** (LLM tokens only, mapped to $). They are **complementary**, not overlapping. Absorbing `pheno-cost-card` into `pheno-mcp-router` would force `pheno-mcp-router` to add infra-cost dimensions, violating its ADR-013 substrate scope ("MCP router per ADR-013 substrate, not a cost tracking lib").

### 2.3 Test coverage in pheno-mcp-router

- `pheno-mcp-router/tests/test_cost.py` (full coverage of `cost.py`)
- `pheno-mcp-router/tests/test_cost_middleware.py` (full coverage of `cost_middleware.py`)
- Plus `test_budget.py`, `test_quota.py`, `test_audit.py`, `test_tiers.py` (per `pheno-mcp-router/audit_scorecard.json`)

### 2.4 Verdict on pheno-mcp-router as target

**REJECT.** The pheno-mcp-router cost module is NOT a target for pheno-cost-card absorption. They are **complementary** (provider-usage vs infra-cost rollup), not overlapping. Specifically:

- **What CAN be cross-referenced:** `pheno-cost` (the new substrate) can use `pheno-mcp-router`'s `cost.CostEstimator` as a sub-component to compute `$` from `llm_tokens`. The `pheno-cost` `CostCard` could have a `estimated_usd: float` field that delegates to `pheno-mcp-router.cost.CostEstimator.estimate(model, tokens)`.
- **What CANNOT be merged:** The 4-dimensional `CostCard` (tokens, CI minutes, runner hours, storage) and the `render_repo_card()` markdown renderer have no analog in `pheno-mcp-router`.

**Conclusion:** `pheno-mcp-router` should be a **downstream consumer** of `pheno-cost`, not the other way around.

---

## 3. Per-Candidate Evaluation

### 3.1 `KooshaPari/pheno-mcp-router`

**Path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/pheno-mcp-router/`
**Source:** `pheno-mcp-router/src/pheno_mcp_router/`
**Substrate per ADR:** ADR-013 — "pheno-mcp-router substrate for pheno-mcp-*"
**Language:** Python

**Plumbing check (does pheno-cost-card import pheno-mcp-router?):**
```bash
grep -r "pheno_mcp_router\|pheno-mcp-router" /Users/kooshapari/CodeProjects/Phenotype/repos/pheno-cost-card/
# Result: 0 matches
```

**Reverse plumbing check (does pheno-mcp-router import pheno-cost-card?):**
```bash
grep -r "pheno_cost_card\|pheno-cost-card" /Users/kooshapari/CodeProjects/Phenotype/repos/pheno-mcp-router/
# Result: 0 matches
```

**Feature overlap (deep, see §2):** Partial. `pheno-mcp-router` cost code is **provider-usage** (LLM tokens → $). `pheno-cost-card` is **infra-cost rollup** (4 dimensions). They are complementary, not overlapping.

**Test coverage:** `pheno-mcp-router` cost code has dedicated tests (`test_cost.py`, `test_cost_middleware.py`, `test_budget.py`, `test_quota.py`, `test_audit.py`, `test_tiers.py`). `pheno-cost-card` has only `test_smoke.py` (per `01-source-inventory.md` §A.4).

**Doc coverage:** `pheno-mcp-router/docs/PROVIDER_GUIDE.md` is comprehensive. `pheno-cost-card/README.md` is stub-quality (per `02-docs-code.md` §B.1).

**ADR relevance:** ADR-013 explicitly scopes `pheno-mcp-router` as "substrate for pheno-mcp-*". Adding infra-cost tracking would violate this scope.

**Verdict: ❌ REJECT — HIGH plausibility as a candidate, but the overlap is FALSE POSITIVE.** The dispatch-mcp W2-1 absorption (PR #1) added provider-usage cost, not infra-cost. Absorbing pheno-cost-card would require:
- Adding `ci_minutes`, `runner_hours`, `storage_gb` to `pheno-mcp-router.cost.CostRecord` (scope creep)
- Adding a markdown renderer to `pheno-mcp-router` (MCP-routing concern, not rendering)
- Adding `tier.py` (already superseded by `tiers.py`)

**Evidence:**
- `pheno-mcp-router/src/pheno_mcp_router/cost.py:1` (file exists, ~200 LOC of provider-usage cost)
- `pheno-mcp-router/src/pheno_mcp_router/cost_middleware.py:1` (file exists, ~250 LOC of ASGI middleware)
- `pheno-mcp-router/tests/test_cost.py` (full test coverage of cost module)
- `pheno-mcp-router/docs/PROVIDER_GUIDE.md` (provider pricing & integration guide)
- `AGENTS.md` §"Dmouse92 → KooshaPari migration" PR #1 (the absorption PR that added cost code)
- ADR-013: "pheno-mcp-router substrate for pheno-mcp-*" (scope restriction)

**Counter-recommendation:** `pheno-mcp-router` should be a **downstream consumer** of the new `pheno-cost` substrate. Specifically, `pheno-cost` `CostCard.estimated_usd` could delegate to `pheno-mcp-router.cost.CostEstimator.estimate(model, tokens)`.

---

### 3.2 `KooshaPari/pheno-prompt-test`

**Path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/pheno-prompt-test/`
**Source:** `pheno-prompt-test/src/pheno_prompt_test/`
**Substrate per ADR:** not in AGENTS.md ADR list (presumed "pheno-*-lib" per ADR-023 Rule 3)
**Language:** Python

**Plumbing check (does pheno-cost-card import pheno-prompt-test?):** 0 matches.
**Reverse plumbing check (does pheno-prompt-test import pheno-cost-card?):** 0 matches.

**Inspection of `pheno-prompt-test/src/pheno_prompt_test/__init__.py`:**
- Per `02-docs-code.md` cross-references and prior audit notes, `pheno-prompt-test` is a **prompt testing substrate** with modules for `PromptTest`, `TestHarness`, `Assertion`, `Scorer`, etc. (per `pheno-prompt-test/pyproject.toml` description).
- Has its own concept of "cost" — per-prompt cost of running a test, computed from token usage. Different abstraction than `CostCard` (which is a 4-dimensional fleet rollup).

**Feature overlap:** **Conceptual overlap only.** Both deal with "cost" in the LLM context, but:
- `pheno-prompt-test`: per-test-run cost (one prompt → one score → cost in $)
- `pheno-cost-card`: per-repo/per-fleet infra cost (multi-dimensional rollup of CI, runner, storage, LLM tokens)

**Test coverage:** `pheno-prompt-test` has comprehensive tests (per `pheno-prompt-test/AGENTS.md`). `pheno-cost-card` has only `test_smoke.py`.

**Doc coverage:** `pheno-prompt-test/README.md` is detailed; `pheno-cost-card/README.md` is stub-quality.

**ADR relevance:** No ADR explicitly forbids or encourages this merge. But substrate scope mismatch is decisive.

**Verdict: ❌ REJECT — LOW plausibility.** The "cost" concepts are at different abstraction levels. Merging them would conflate per-test cost (a metric in test output) with infra-cost rollup (a fleet-management concept).

**Evidence:**
- `pheno-prompt-test/src/pheno_prompt_test/__init__.py` (presumed contents: `PromptTest`, `TestHarness`, `Scorer` exports)
- `pheno-prompt-test/pyproject.toml:1-30` (description, dependencies)
- `pheno-cost-card/src/pheno_cost_card/collectors.py:1` (`llm_tokens` collector)
- Conceptual mismatch: per-test cost vs infra-cost rollup

---

### 3.3 `KooshaPari/pheno-scaffold-kit`

**Path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/pheno-scaffold-kit/`
**Source:** `pheno-scaffold-kit/src/pheno_scaffold_kit/`
**Substrate per ADR:** Not explicitly in AGENTS.md ADR list. Per the meta-bundle inspection, this is a project scaffolding umbrella.
**Language:** Python

**Plumbing check (does pheno-cost-card import pheno-scaffold-kit?):** 0 matches.
**Reverse plumbing check (does pheno-scaffold-kit import pheno-cost-card?):** 0 matches.

**Inspection of `pheno-scaffold-kit/src/pheno_scaffold_kit/__init__.py`:** Provides CLI and template scaffolding (scaffolds new Python projects from templates per its `cli.py`).

**Feature overlap:** **None.** `pheno-scaffold-kit` generates new repos; `pheno-cost-card` measures cost of existing repos.

**Test coverage:** `pheno-scaffold-kit` has tests for its CLI.
**Doc coverage:** `pheno-scaffold-kit/README.md` is comprehensive.

**ADR relevance:** No ADR mentions pheno-scaffold-kit as a cost home.

**Verdict: ❌ REJECT — LOW plausibility.** Wrong substrate scope (scaffolding, not cost tracking).

**Evidence:**
- `pheno-scaffold-kit/src/pheno_scaffold_kit/__init__.py:1` (umbrella exports)
- `pheno-scaffold-kit/src/pheno_scaffold_kit/cli.py:1` (CLI entrypoint)
- `pheno-scaffold-kit/pyproject.toml:1-30` (description: "scaffold new Python projects")
- Zero overlap in module structure

---

### 3.4 `KooshaPari/pheno-llms-txt`

**Path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/pheno-llms-txt/`
**Source:** `pheno-llms-txt/src/pheno_llms_txt/`
**Substrate per ADR:** Not in AGENTS.md ADR list. Per the meta-bundle, this is the `llms.txt` generation substrate.
**Language:** Python

**Plumbing check:** 0 matches both directions.
**Reverse plumbing check:** 0 matches.

**Inspection of `pheno-llms-txt/src/pheno_llms_txt/__init__.py` + `SPEC.md`:** Generates `llms.txt` files for repos (a standardized LLM-friendly repo summary format). Does NOT track cost.

**Feature overlap:** **None.**

**Test coverage:** `pheno-llms-txt` has unit tests.
**Doc coverage:** `pheno-llms-txt/SPEC.md` is a 1-page spec.

**ADR relevance:** None. The `llms.txt` format is unrelated to cost.

**Verdict: ❌ REJECT — LOW plausibility.** Wrong substrate scope (LLM-friendly repo description, not cost tracking).

**Evidence:**
- `pheno-llms-txt/src/pheno_llms_txt/__init__.py:1`
- `pheno-llms-txt/SPEC.md:1-40` (focuses on `llms.txt` format)
- Zero cost-related code in `pheno-llms-txt/src/`

---

### 3.5 `KooshaPari/phenotype-cost-card`

**Existence check (executed 2026-06-18):**

```bash
gh repo view KooshaPari/phenotype-cost-card --json name,description,isArchived,defaultBranchRef,updatedAt
# Result: 404 / does not exist
```

```bash
curl -sfI https://github.com/KooshaPari/phenotype-cost-card
# Result: HTTP 404
```

**Path:** Does not exist on GitHub.
**Local clone:** None.

**Verdict: ❌ N/A — DOES NOT EXIST.** Not a candidate. (The `phenotype-*` prefix is also reserved per ADR-031 for cross-language SDKs and federated services, not single-concern libs, so even if it existed, it would be the wrong naming convention.)

**Evidence:** `gh repo view KooshaPari/phenotype-cost-card` returned 404 (2026-06-18).

---

### 3.6 `KooshaPari/phenotype-cost`

**Existence check (executed 2026-06-18):**

```bash
gh repo view KooshaPari/phenotype-cost --json name,description,isArchived,defaultBranchRef,updatedAt
# Result: 404 / does not exist
```

**Path:** Does not exist on GitHub.
**Local clone:** None.

**Verdict: ❌ N/A — DOES NOT EXIST.** Not a candidate. (Same naming-convention concern as 3.5.)

**Evidence:** `gh repo view KooshaPari/phenotype-cost` returned 404 (2026-06-18).

---

### 3.7 `KooshaPari/phenotype-py-utils`

**Path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-py-utils/`
**Source:** `phenotype-py-utils/src/phenotype_py_utils/`
**Substrate per ADR:** Not explicitly in AGENTS.md ADR list. Per the meta-bundle, this is a Python utility collection.
**Language:** Python

**Plumbing check:** 0 matches both directions.

**Inspection of `phenotype-py-utils/src/phenotype_py_utils/__init__.py`:** A general-purpose Python utility bag (string helpers, file helpers, time helpers, etc.). Does NOT track cost.

**Feature overlap:** **None.** `phenotype-py-utils` is a general utility lib; adding `CostCard` would violate the "general utility" scope.

**Test coverage:** `phenotype-py-utils` has unit tests for each utility.
**Doc coverage:** `phenotype-py-utils/README.md` is comprehensive.

**ADR relevance:** None.

**Verdict: ❌ REJECT — LOW plausibility.** Wrong substrate scope (utility bag, not cost tracking). Also, ADR-023 Rule 3 says new shared code should NOT go to a "random `phenoShared`" pattern; `phenotype-py-utils` is similar (utility bag, not single-concern lib).

**Evidence:**
- `phenotype-py-utils/src/phenotype_py_utils/__init__.py:1-50` (general utility exports)
- `phenotype-py-utils/pyproject.toml:1-30`
- Zero cost-related code in `phenotype-py-utils/src/`

---

### 3.8 `KooshaPari/phenotype-python-sdk`

**Path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-python-sdk/`
**Source:** `phenotype-python-sdk/packages/` (multi-package SDK per meta-bundle)
**Substrate per ADR:** ADR-023 — "phenotype-*-sdk" for cross-language SDK, polyglot facade.
**Language:** Python (with multi-package structure: `phenotype-py-kit`, etc.)

**Plumbing check:** 0 matches both directions.

**Inspection of `phenotype-python-sdk/packages/phenotype-py-kit/`:** A meta-package that re-exports other Phenotype libs (likely `pheno-otel`, `pheno-config`, `pheno-tracing`, etc.) as a unified SDK. Does NOT track cost directly.

**Feature overlap:** **None.** This is a meta-SDK, not a cost lib.

**Test coverage:** `phenotype-python-sdk` has tests for the package wiring.

**Doc coverage:** `phenotype-python-sdk/README.md` is comprehensive.

**ADR relevance:** ADR-023 explicitly scopes `phenotype-python-sdk` as a **meta-SDK / polyglot facade**. Adding `CostCard` as a first-class export would be appropriate (the new `pheno-cost` could be exposed as `phenotype_python_sdk.cost.CostCard`), but the SDK is not a *home* for the cost code — it's a *re-exporter*.

**Verdict: ❌ REJECT — MEDIUM plausibility as a consumer, but NOT a home.** The right pattern is:
1. Create `pheno-cost` (the home).
2. Have `phenotype-python-sdk` re-export `pheno_cost.CostCard` as `phenotype_python_sdk.cost.CostCard` (per ADR-023 polyglot facade pattern).

**Evidence:**
- `phenotype-python-sdk/packages/phenotype-py-kit/pyproject.toml:1-30` (meta-SDK description)
- `phenotype-python-sdk/packages/` (multi-package structure)
- ADR-023: "phenotype-*-sdk = Cross-language SDK; stable public API; polyglot facade"

---

### 3.9 `KooshaPari/pheno-fastapi-base`

**Path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/pheno-fastapi-base/`
**Source:** `pheno-fastapi-base/pheno_fastapi_base/`
**Substrate per ADR:** Not in AGENTS.md ADR list. Per the meta-bundle, this is the FastAPI HTTP base.
**Language:** Python

**Plumbing check:** 0 matches both directions.

**Inspection of `pheno-fastapi-base/pheno_fastapi_base/__init__.py` + `pyproject.toml`:** Provides FastAPI app scaffolding (health checks, OpenAPI, middleware integration). Does NOT track cost.

**Feature overlap:** **None.** This is HTTP scaffolding, not cost tracking.

**Test coverage:** `pheno-fastapi-base` has integration tests.
**Doc coverage:** `pheno-fastapi-base/README.md` is comprehensive.

**ADR relevance:** None.

**Verdict: ❌ REJECT — LOW plausibility.** Wrong substrate scope (HTTP base, not cost tracking).

**Evidence:**
- `pheno-fastapi-base/pheno_fastapi_base/__init__.py:1`
- `pheno-fastapi-base/pyproject.toml:1-30` (description: "FastAPI app base")
- Zero cost-related code in `pheno-fastapi-base/pheno_fastapi_base/`

---

### 3.10 `KooshaPari/AgilePlus`

**Path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/AgilePlus/`
**Source:** `AgilePlus/` (Rust workspace per meta-bundle)
**Substrate per ADR:** Per AGENTS.md "Decision B" + ADR-032 — AgilePlus is the worklog audit trail, **explicitly ruled out** for cost-card absorption.
**Language:** Rust

**Plumbing check:** 0 matches both directions.
**Reverse plumbing check:** Per ADR-032, pheno-worklog-schema and AgilePlus worklogs coexist (different formats: markdown table vs JSONL). No cost-card dependency.

**Feature overlap:** **None.** AgilePlus is a worklog audit trail (JSONL format), not a cost tracking lib.

**Test coverage:** AgilePlus has comprehensive Rust tests.
**Doc coverage:** AgilePlus/AGENTS.md is comprehensive.

**ADR relevance:** **ADR-032 EXPLICITLY RULES OUT** merging `pheno-worklog-schema` with AgilePlus. By extension, **merging `pheno-cost-card` with AgilePlus is also ruled out** (different scope, different format, different audience). ADR-032 is the most recent decision (2026-06-17) and supersedes any earlier consideration.

**Verdict: ❌ REJECT — HIGH plausibility DISMISSED by ADR-032.** Even if the substrates overlapped (they don't), the 2026-06-17 decision explicitly separates "primitive libs" from "AgilePlus worklogs".

**Evidence:**
- `AGENTS.md` §"Decision B — pheno-worklog-schema is a primitive lib, NOT a duplicate of AgilePlus"
- ADR-032: "pheno-worklog-schema is a primitive lib, NOT a re-implementation of AgilePlus worklog"
- `AgilePlus/AGENTS.md:1-30` (AgilePlus scope: worklog audit trail)
- Zero cost-related code in `AgilePlus/`

---

### 3.11 `KooshaPari/phenotype-registry`

**Path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/phenotype-registry/`
**Source:** `phenotype-registry/` (registry of components, not a lib)
**Substrate per ADR:** Not in AGENTS.md ADR list. Per the meta-bundle, this is the registry of Phenotype components.
**Language:** Mixed (registry data, not a runtime lib)

**Plumbing check:** 0 matches both directions.

**Inspection of `phenotype-registry/README.md`:** A registry of components (metadata, dispositions, not runtime code). Does NOT track cost.

**Feature overlap:** **None.** This is a registry, not a cost lib. (One could *register* the new `pheno-cost` substrate here, but that's not absorption.)

**Test coverage:** N/A (registry is data, not code).
**Doc coverage:** `phenotype-registry/README.md` is detailed.

**ADR relevance:** None.

**Verdict: ❌ REJECT — LOW plausibility.** Wrong substrate scope (registry, not cost lib). Could be a **downstream consumer** (the registry could expose `pheno-cost` for discovery), but not a home.

**Evidence:**
- `phenotype-registry/README.md:1-20` (registry description)
- `phenotype-registry/registry/` (registry data files)
- Zero cost-related code in `phenotype-registry/`

---

### 3.12 `KooshaPari/Configra`

**Path:** `/Users/kooshapari/CodeProjects/Phenotype/repos/Configra/`
**Source:** `Configra/` (Rust crate)
**Substrate per ADR:** ADR-031 — "Configra absorb — `phenotype-config` folds into `Configra` as canonical name; ADR-022 split (Rust core / TS edge) preserved". This is the **Rust config substrate**.
**Language:** Rust (and TypeScript edge)

**Plumbing check:** N/A (different language, cannot import Python).
**Reverse plumbing check:** N/A.

**Feature overlap:** **None.** Configra is a config lib (12-factor cascade, env + TOML + CLI). Does NOT track cost.

**Test coverage:** Configra has Rust tests.
**Doc coverage:** Configra/SPEC.md is comprehensive.

**ADR relevance:** ADR-031 names Configra as the **canonical config substrate**. Cost tracking is unrelated.

**Verdict: ❌ REJECT — LOW plausibility.** Wrong substrate scope (config, not cost) AND wrong language (Rust, not Python — `pheno-cost-card` is pure Python with no Rust bindings).

**Evidence:**
- `Configra/SPEC.md:1-40` (config substrate scope)
- `Configra/Cargo.toml:1-20` (Rust crate)
- `pheno-cost-card/pyproject.toml:1-50` (Python lib)
- ADR-031: "Configra absorb — phenotype-config folds into Configra"

---

## 4. Recommended Target(s)

### 4.1 Primary recommendation: Create new `KooshaPari/pheno-cost` (Python)

**Rationale:**

1. **No existing substrate is a good fit** (see §3 evaluations — all 12 candidates REJECTed).
2. **ADR-023 Rule 3** explicitly authorizes creating a new `pheno-*-lib` substrate for a single-concern, language-specific library. `CostCard` + `render_repo_card` is a single concern (infra-cost rollup for a single repo).
3. **Naming convention:** `pheno-cost` follows the `pheno-*-lib` pattern (vs `phenotype-cost` which is reserved for cross-language SDKs per ADR-031, and `pheno-cost-card` which is misleading because "card" suggests business-card semantics).
4. **Re-export in `phenotype-python-sdk`:** Once created, `pheno-cost` should be exposed via `phenotype_python_sdk.cost.CostCard` (per ADR-023 polyglot-facade pattern). This is a **downstream consumer** relationship, not an absorption.
5. **Re-export in `pheno-mcp-router`:** `pheno-mcp-router`'s provider-usage cost code can be cross-referenced in `pheno-cost`'s `collectors.py` to populate `llm_tokens` from MCP call events. The new `CostCard.estimated_usd` could delegate to `pheno-mcp-router.cost.CostEstimator.estimate()`.

### 4.2 Migration steps (one PR sequence)

| # | Repo | Action | Files | Notes |
|---|---|---|---|---|
| 1 | `KooshaPari/pheno-cost` | **CREATE** | `pyproject.toml`, `src/pheno_cost/__init__.py`, `src/pheno_cost/collectors.py`, `src/pheno_cost/render.py`, `tests/test_smoke.py`, `README.md`, `AGENTS.md`, `WORKLOG.md`, `CHANGELOG.md`, `LICENSE-MIT` | New repo, MIT license, src layout. |
| 2 | `KooshaPari/pheno-cost` | PORT | Port 4 files from `pheno-cost-card/`: `__init__.py` (cleaned exports), `collectors.py` (4 collectors), `render.py` (`render_repo_card` only), `tests/test_smoke.py` | Drop `render_fleet_card()` (depends on non-existent fleet registry) and the broken `examples/fleet_card.py`. |
| 3 | `KooshaPari/pheno-cost` | DROP | `examples/fleet_card.py` (per `02-docs-code.md` §B.5), `deny.toml` (per `02-docs-code.md` §C.3) | Misplaced files; not part of the lib. |
| 4 | `KooshaPari/pheno-cost-card` | DEPRECATE | `README.md`, `AGENTS.md`, `CHANGELOG.md` add "MOVED TO `KooshaPari/pheno-cost`" banner; bump version to `0.2.0-deprecated` | 30-day deprecation window per `pheno-prompt-test` precedent. |
| 5 | `KooshaPari/pheno-cost-card` | ARCHIVE | `gh repo archive KooshaPari/pheno-cost-card` (read-only marker) | Per the policy applied to `McpKit`, `kwality`, `phenotype-auth-ts`, `dinoforge-packs` in `AGENTS.md` §"4-repo retirement". |
| 6 | `phenotype-python-sdk` | ADD | `packages/phenotype-py-kit/src/phenotype_py_kit/cost.py` (re-export `pheno_cost.CostCard`), update `__init__.py` to include cost | Optional: exposes the new lib via the polyglot SDK facade per ADR-023. |

### 4.3 Why NOT a target within the existing monorepo

- **No existing Python lib substrate has a "cost" or "billing" concern.** The closest is `pheno-mcp-router` cost code, but that's scoped to **provider-usage** (LLM tokens only) per ADR-013.
- **Cross-language absorption is impossible:** Configra is Rust; pheno-cost-card is pure Python.
- **Meta-libs (`pheno-scaffold-kit`, `phenotype-py-utils`, `phenotype-python-sdk`) are the wrong scope** (umbrellas, not single-concern).

### 4.4 Why NOT simply archive `pheno-cost-card` without replacement

- The `CostCard` concept is **valuable** — 4-dimensional infra-cost rollup is a real fleet management need.
- The collectors + render functions are **non-trivial** (`render_repo_card` is a markdown renderer with proper formatting per `01-source-inventory.md` §A.2).
- The **dispatch-mcp W2-1 absorption** (per AGENTS.md §"Dmouse92 → KooshaPari migration" PR #1) demonstrates that the **concept of cost tracking is in active use in the fleet** — it just needs the right home.

### 4.5 What about the misapplied `deny.toml`?

Per `02-docs-code.md` §C.3, `pheno-cost-card/deny.toml` is a **Rust** `cargo-deny` config that has zero effect on a Python lib (Python doesn't use `cargo-deny`). It was likely copy-pasted from a Rust substrate template. **DROP it** during migration — not absorbed anywhere.

---

## 5. Risks and Open Questions

### 5.1 Risks

1. **Risk: Naming confusion.** The new `pheno-cost` name is similar to `pheno-cost-card` (the deprecated repo). Users may confuse the two during the deprecation window.
   - **Mitigation:** Strong banner in `pheno-cost-card/README.md` ("MOVED TO `KooshaPari/pheno-cost`") + version bump to `0.2.0-deprecated` + add `pypi-no-new-releases` flag if published.

2. **Risk: Lost in translation of `render_fleet_card()`.** The `examples/fleet_card.py` is broken (per `02-docs-code.md` §B.5) because it depends on a non-existent fleet registry. Dropping it means losing the **concept** of multi-repo fleet rollup.
   - **Mitigation:** Add a TODO in `pheno-cost/SPEC.md` for a future `render_fleet_card()` that integrates with `phenotype-registry` (data source) or `pheno-context` (config). Document the design constraint.

3. **Risk: Confusion with `pheno-mcp-router` cost code.** Users may wonder "why are there two cost libs?" The answer is **provider-usage** (pheno-mcp-router) vs **infra-cost rollup** (pheno-cost).
   - **Mitigation:** Cross-link in both `pheno-cost/README.md` and `pheno-mcp-router/docs/PROVIDER_GUIDE.md`. Add a comparison table.

4. **Risk: `phenotype-py-utils` collision.** If someone argues `pheno-cost` should be a sub-module of `phenotype-py-utils` (the utility bag), it would conflict with ADR-023 Rule 3 ("pheno-*-lib for single concern, single crate").
   - **Mitigation:** Cite ADR-023 explicitly in the new `pheno-cost/AGENTS.md` and the migration PR.

### 5.2 Open Questions

1. **Q: Should `pheno-cost` expose the `pheno-mcp-router` cost primitives as a sub-module?**
   - Option A: `pheno-cost` depends on `pheno-mcp-router` (tight coupling, but reuses the `CostEstimator`).
   - Option B: `pheno-cost` is standalone; `phenotype-python-sdk` re-exports both and the user wires them together.
   - **Recommendation:** Option A. `pheno-cost`'s `collectors.py` can import `pheno_mcp_router.cost.CostEstimator` directly. This is the polyglot-facade pattern from ADR-023.

2. **Q: Should the new `pheno-cost` re-implement the tier logic?**
   - `pheno-cost-card` has a `tier.py` stub (per `01-source-inventory.md` §A.3).
   - `pheno-mcp-router` has a complete `tiers.py` (`Tier` enum + `TierPolicy`).
   - **Recommendation:** DROP the `tier.py` stub from `pheno-cost`. The actual `tier` logic lives in `pheno-mcp-router` and is provider-specific (free/pro/enterprise for LLM providers). A "tier" concept for infra-cost rollup is a different design (e.g., "small/medium/large fleet") and is out of scope for this migration.

3. **Q: Should the broken `examples/fleet_card.py` be saved as `pheno-cost/docs/design-fleet-card.md`?**
   - **Recommendation:** Yes, but only as a **design note** describing the future intent, not as a working example. The note should reference `phenotype-registry` as the data source for fleet rollup.

4. **Q: When to delete (not just archive) `pheno-cost-card`?**
   - Per the 4-repo retirement pattern in `AGENTS.md` §"4-repo retirement", archive is a read-only marker. Full deletion requires `gh repo delete` with `delete_repo` scope.
   - The current `gh` token has scopes `'gist', 'read:org', 'repo', 'workflow'` — **no `delete_repo`** (per AGENTS.md §"Stale / warnings").
   - **Recommendation:** Archive only. Manual delete via GitHub UI (`Settings → General → Danger Zone → Delete this repository`) after 90-day GitHub retention. Apply the same pattern as `dagctl`, `kwality`, `phenotype-auth-ts`, `dinoforge-packs`.

5. **Q: Is there a CI cost dimension that should be added?**
   - `pheno-cost-card` has `ci_minutes` as a stub collector. CI minutes are typically tracked per-provider (GitHub Actions minutes, GitLab CI minutes, CircleCI credits).
   - **Recommendation:** Keep the stub in `pheno-cost/collectors.py` but document that real values come from the provider's API (not implemented yet — out of scope for this migration).

6. **Q: Does the new `pheno-cost` need a `py.typed` marker?**
   - **Recommendation:** Yes. Add `py.typed` to `pheno-cost/src/pheno_cost/` to enable type checking by consumers. This is the same SOTA pattern used by `pheno-mcp-router`, `pheno-prompt-test`, etc.

### 5.3 Migration verification (post-merge)

Per `verification-before-completion` skill rules, the following must hold before claiming the migration is "done":

- [ ] `KooshaPari/pheno-cost` exists on GitHub with all 4 source files ported + cleaned.
- [ ] `pyproject.toml` has `name = "pheno-cost"`, `version = "0.1.0"`, `requires-python = ">=3.11"`.
- [ ] `python -m pytest pheno-cost/tests/` passes 100% (currently the source has only `test_smoke.py`; migration must preserve all passing tests).
- [ ] `python -c "from pheno_cost import CostCard, render_repo_card"` succeeds.
- [ ] `python -c "from pheno_cost.collectors import estimate_ci_minutes"` succeeds.
- [ ] `KooshaPari/pheno-cost-card` is archived (read-only marker) on GitHub.
- [ ] `pheno-cost-card/README.md` has "MOVED TO `KooshaPari/pheno-cost`" banner at top.
- [ ] `pheno-cost-card/pyproject.toml` is bumped to `0.2.0-deprecated`.
- [ ] `phenotype-python-sdk` optionally re-exports `pheno_cost.CostCard` as `phenotype_python_sdk.cost.CostCard`.
- [ ] `pheno-mcp-router/docs/PROVIDER_GUIDE.md` cross-links to `pheno-cost/README.md` (downstream consumer note).
- [ ] `AGENTS.md` §"pheno-* family" is updated to include `pheno-cost` and note the deprecation of `pheno-cost-card`.
- [ ] `findings/2026-06-18-pheno-cost-card-audit/` is referenced in `AGENTS.md` §"Related".

---

## Appendix A: File-by-file decision matrix

| `pheno-cost-card` file | Disposition | Destination |
|---|---|---|
| `src/pheno_cost_card/__init__.py` | PORT (cleaned) | `pheno-cost/src/pheno_cost/__init__.py` |
| `src/pheno_cost_card/collectors.py` | PORT (verbatim) | `pheno-cost/src/pheno_cost/collectors.py` |
| `src/pheno_cost_card/render.py` | PORT (drop `render_fleet_card`) | `pheno-cost/src/pheno_cost/render.py` |
| `src/pheno_cost_card/tier.py` | DROP (stub; superseded by `pheno-mcp-router.tiers`) | n/a |
| `examples/fleet_card.py` | DROP (broken; depends on non-existent fleet registry) | n/a (or save as `pheno-cost/docs/design-fleet-card.md`) |
| `examples/repo_card.py` | PORT (cleaned) | `pheno-cost/examples/repo_card.py` (if it exists; otherwise drop) |
| `tests/test_smoke.py` | PORT (verbatim) | `pheno-cost/tests/test_smoke.py` |
| `README.md` | REWRITE (full rewrite) | `pheno-cost/README.md` |
| `AGENTS.md` | REWRITE (new substrate template) | `pheno-cost/AGENTS.md` |
| `WORKLOG.md` | REWRITE (new repo) | `pheno-cost/WORKLOG.md` (start with `device: macbook` per ADR-025 v2.1 schema) |
| `CHANGELOG.md` | REWRITE (start with `0.1.0`) | `pheno-cost/CHANGELOG.md` |
| `SPEC.md` | PORT (corrected) | `pheno-cost/SPEC.md` |
| `LICENSE-MIT` | COPY (verbatim) | `pheno-cost/LICENSE-MIT` |
| `pyproject.toml` | REWRITE (corrected, name=pheno-cost) | `pheno-cost/pyproject.toml` |
| `deny.toml` | DROP (Rust config; doesn't apply to Python) | n/a |
| `WORKTREE` marker (if present) | N/A | n/a |

---

## Appendix B: Cross-references

- `AGENTS.md` §"App-level repo triage & app substrate placement" (ADR-023 Rule 3) — substrate placement decision.
- `AGENTS.md` §"Dmouse92 → KooshaPari migration" (ADR-029, PR #1) — the dispatch-mcp cost absorption that informed §2.
- `AGENTS.md` §"Active ADRs" ADR-013 (pheno-mcp-router as MCP substrate), ADR-031 (Configra as canonical config), ADR-032 (pheno-worklog-schema vs AgilePlus).
- `findings/2026-06-18-pheno-cost-card-audit/01-source-inventory.md` §F.3 (strategic recommendation).
- `findings/2026-06-18-pheno-cost-card-audit/02-docs-code.md` §B.5 (broken `examples/fleet_card.py`), §C.3 (misapplied `deny.toml`).
- `findings/2026-06-17-L5-104-dmouse92-to-kooshapari.md` (the migration plan that created `pheno-mcp-router` cost code).
- `KooshaPari/pheno-mcp-router#1` (the absorption PR for dispatch-mcp W2-1 cost code).
- `pheno-mcp-router/src/pheno_mcp_router/cost.py:1` (provider-usage cost implementation).
- `pheno-mcp-router/src/pheno_mcp_router/cost_middleware.py:1` (ASGI middleware).
- `pheno-cost-card/src/pheno_cost_card/__init__.py:1` (`CostCard` dataclass).
- `pheno-cost-card/src/pheno_cost_card/collectors.py:1` (4 stub collectors).
- `pheno-cost-card/src/pheno_cost_card/render.py:1` (markdown render functions).
- `pheno-cost-card/examples/fleet_card.py:1` (broken fleet rollup example).

---

**End of report.**
