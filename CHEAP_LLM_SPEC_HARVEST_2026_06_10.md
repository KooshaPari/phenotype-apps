# CHEAP-LLM-MCP Spec Harvest - 2026-06-10

> **Source provenance (W5-5 re-harvest, 2026-06-18):** The original V4 launch
> agent output file `/tmp/dispatch-batch/agent_06_cheapllm_spec.out` is **no
> longer present on this machine** (`/tmp/dispatch-batch/` directory does not
> exist as of 2026-06-18). The verified content below was originally extracted
> from that source file in the prior V4 harvest cycle and committed at
> `954252e4ee3105c6febfa56f39416085e5706042` as
> `agent_06_cheapllm_spec_2026_06_10.md` (79 lines, blob `e5a14fda`).
> This document re-lifts that **prior-verified content** under the W5-5
> filename convention `CHEAP_LLM_SPEC_HARVEST_2026_06_10.md`. Content is
> **high-quality / verified** (not hallucinated) — provenance is git, not
> fresh agent re-run.

**Lifted blob:** `e5a14fda6741ce37ced8f90f46b0a890182efb7d`
**Source commit:** `954252e4ee3105c6febfa56f39416085e5706042`
**Original filename in source commit:** `agent_06_cheapllm_spec_2026_06_10.md`
**Refs:** FLEET_DAG_v3.md §100 (V21 deferred: land V4 launch agent outputs);
W5-5 task slot in FLEET_DAG_v3.db; V3_EXECUTION_LOG_2026_06_10.md.

---

## Verified content (verbatim, from blob e5a14fda)

```
>>> dispatch-worker tier=worker model=Worker endpoint=http://localhost:20128/v1
>>> routed to backend: gemini-3-flash-preview
### 1. Plan Summary

#### Current Public Surface
* `src/index.ts`: Entry point for the MCP server.
* `src/server.ts`: Implementation of the `CheapLLMServer` class and MCP tool handlers.
* `src/llm-provider.ts`: Abstract base and concrete implementations for LLM providers (OpenAI, Anthropic).
* `src/types.ts`: Internal type definitions for configuration and API responses.

#### Re-Export List
* `src/index.ts` -> `CheapLLMServer` (from `src/server.ts`)
* `src/index.ts` -> `LLMProvider`, `OpenAIProvider`, `AnthropicProvider` (from `src/llm-provider.ts`)
* `src/index.ts` -> `ConfigSchema` (from `src/types.ts`)

#### Exact PR Title / Summary / Validation
* **Title**: `refactor: modularize codebase and expose public API surface`
* **Summary**: This PR restructures the project to support programmatic usage as a library while maintaining its functionality as a standalone MCP server. It introduces a clean export structure in `index.ts` and decouples the server logic from the CLI entry point.
* **Validation**:
    * Run `npm run build` to ensure types are generated correctly.
    * Execute `npm test` to verify tool handlers and provider logic.
    * Link the package locally and import `CheapLLMServer` in a test script.

#### Exact Diff
diff --git a/src/index.ts b/src/index.ts
--- a/src/index.ts
+++ b/src/index.ts
@@ -1,15 +1,6 @@
 #!/usr/bin/env node
-import { Server } from "@modelcontextprotocol/sdk/server/index.js";
-import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
-import {
-  CallToolRequestSchema,
-  ListToolsRequestSchema,
-} from "@modelcontextprotocol/sdk/types.js";
-import { CheapLLMServer } from "./server.js";
+export { CheapLLMServer } from "./server.js";
+export { OpenAIProvider, AnthropicProvider } from "./llm-provider.js";
+export * from "./types.js";

-const server = new CheapLLMServer();
-const transport = new StdioServerTransport();
-server.connect(transport).catch((error) => {
-  console.error("Server error:", error);
-  process.exit(1);
-});
+import { main } from "./cli.js";
+if (require.main === module) { main(); }

### 2. Implementation Checklist

- [ ] `src/index.ts` (Modify: ~15 lines)
- [ ] `src/cli.ts` (Create: ~20 lines)
- [ ] `src/server.ts` (Modify: ~10 lines)
- [ ] `src/llm-provider.ts` (Modify: ~5 lines)
- [ ] `package.json` (Modify: ~5 lines)

### 3. Validation Commands

bash
# Clean and build the project
npm run clean && npm run build

# Run unit tests
npm test

# Verify CLI execution
node build/index.js --help

# Verify programmatic import (requires local link or manual check)
node -e 'require("./build/index.js").CheapLLMServer ? process.exit(0) : process.exit(1)'


### 4. Risks / Blockers

* **Circular Dependencies**: Moving `main()` logic to a new `cli.ts` might create issues if `server.ts` imports from `index.ts`. Ensure unidirectional flow.
* **Breaking Changes**: External consumers using the CLI via `npx` must ensure the `bin` field in `package.json` still points to the transpiled `index.js`.
* **Environment Variables**: Programmatic usage may bypass standard `.env` loading if not handled explicitly in the `CheapLLMServer` constructor.
```

## Notes

- `gemini-3-flash-preview` was the model that produced the original plan (per
  the dispatch-worker header).
- The diff block is the **literal** agent output. It is unverified-as-still-applicable
  to the current `cheap-llm-mcp` source tree (the crate was later absorbed into
  `dispatch-mcp` per the W1-2 archive work, and `dispatch-mcp` is itself
  deprecated per ADR-008 — see ADR-007 in `docs/adr/2026-06-15/`). The
  archival purpose of this harvest is traceability, not live applicability.
- The agent's `[ ]` checklist is **un-actioned**: this harvest records the
  plan, it does not execute it.
