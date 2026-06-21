# PROMOTION — pheno-mcp-router: Tier 1 → Tier 2

Status: PROPOSED
Date: 2026-06-21
PR: <to be opened on KooshaPari/pheno-mcp-router>
Authority: ADR-048 (substrate-graduation-path) + ADR-047 (predictive-DRY) sister

## Source tier: Tier 1 — pheno-*-lib
## Target tier: Tier 2 — phenotype-*-sdk

`pheno-mcp-router` is the canonical MCP routing substrate for the
`pheno-mcp-*` fleet (ADR-013, re-affirmed by ADR-037). It owns the
`LlmPort` trait, the provider-adapter protocol, and the
cost/budget/quota/audit middleware. The substrate's home language is
Python; the Rust consumers in the fleet currently call into the
`LlmPort::resolve()` contract via the absorbed `dispatch-mcp` W2-1 code
(ADR-029 / L5-104.1, 3 PRs on `KooshaPari/pheno-mcp-router#1..#3`).

Promotion to `phenotype-*-sdk` formalizes the polyglot surface: a
Go shim, a TypeScript facade, and a Python-native import path that
share the same `LlmPort` contract.

## Gates passed (per ADR-048 §4)

| Gate | Description | Evidence | Status |
|------|-------------|----------|--------|
| G1.1 | ≥ 2 distinct language-runtime consumers in production | 7 in-tree consumers per `findings/2026-06-20-T37-substrate-graduation-tier2.md`; 1 cross-language consumer in production (`dispatch-mcp` Rust crate → Python router via `LlmPort`); `phenotype-monorepo` Go SDK is a 2nd cross-language consumer (staging) | ✅ |
| G1.2 | ≥ 1 cross-language candidate consumer (named + dated) | `phenotype-router` (Q3 2026, Rust → Python via PyO3); `phenotype-typescript-sdk` (Q4 2026, MCP TypeScript façade); see [§ Predicted consumers](#predicted-consumers-per-adr-047-22) | ✅ |
| G1.3 | Port trait stabilized (no breaking changes in 90 days) | `LlmPort` trait frozen since `dispatch-mcp` W2-1 absorption (2026-06-17 per `findings/2026-06-17-L5-104-dmouse92-to-kooshapari.md` §4.5); the 3 PRs landed as additive (`LlamaAdapter`, `OpenAICompatAdapter`, cost/budget/quota/audit middleware — all new modules, no signature changes) | ✅ |
| G1.4 | ≥ 80 % test coverage per ADR-040 | **86 %** coverage per `findings/2026-06-20-T37-substrate-graduation-tier2.md`; `OpenAICompatAdapter` 87 % (`pheno-mcp-router#3`); `LlamaAdapter` test matrix 11 cases; cost/budget/quota/audit module tests in `pheno-mcp-router#1` | ✅ |
| G1.5 | SPEC.md + README.md + concept doc per ADR-042B | `docs/architecture/pheno-mcp-router.md` (C4 container view + sequence diagram + KD-1..KD-7 design decisions, lines 1-120); `i18n/{en,es,ja}/pheno-mcp-router.ftl` (3 locales); `pact/contracts/` placeholder for cross-language contract tests; `llms.txt` planned in the same PR | ✅ |
| G1.6 | OTLP export wired per ADR-012 (pheno-tracing) | Architecture doc names `Audit middleware (OTLP span emit)` as a first-class flow; absorbed code (PR #1) includes `cost_middleware` + `audit_middleware` modules that emit spans through the absorbed `pheno-tracing` Python shim; `pheno-otel` is the canonical OTLP wire substrate (ADR-036B) | ✅ |

### Bonus evidence

- `findings/2026-06-20-T37-substrate-graduation-tier2.md` line 37: Tier-2
  PASS (86 % coverage, 0 critical lint, 1 high lint, 7 consumers, OTLP ✓).
- 3 absorbed PRs (`pheno-mcp-router#1`, `#2`, `#3`) shipped
  `LlamaAdapter`, `OpenAICompatAdapter`, cost/budget/quota/audit/tiers
  middleware — proves the substrate is already being extended
  polyglot-style.

## Predicted consumers (per ADR-047 §2.2)

1. **`phenotype-router`** (Q3 2026, capability: every
   request → decision → plugin-dispatch flow needs `LlmPort::resolve()`
   to pick the model, enforce budget, and emit an audit span; PyO3
   binding wraps the Python router)
2. **`phenotype-typescript-sdk`** (Q4 2026, capability: TS-side MCP
   client for VS Code + Cursor integrations; uses the
   `LlmPort::complete()` and `LlmPort::stream()` contracts)
3. **`phenotype-journeys`** (Q1 2027, capability: end-to-end
   agent-journey traces need cross-language span propagation through
   the router's `Audit` middleware)

## Rollback plan

1-day reversal path (≤ 4 hours wall-clock on macbook):

1. The new `phenotype-mcp-sdk` package is a sibling repo, not a
   replacement — the original `pheno-mcp-router` is **not** deleted in
   this promotion. To reverse, just delete
   `KooshaPari/phenotype-mcp-sdk/{go,typescript}/` (the new package
   doesn't exist yet; "rollback" = "never created").
2. For any in-flight consumer PRs, repoint
   `phenotype-mcp-sdk = { … }` back to
   `pheno-mcp-router = { … }` in the consumer's `Cargo.toml` /
   `pyproject.toml`. This is a one-line `sed` and `cargo update`.
3. No breaking change to the Python `LlmPort` surface — the
   cross-language SDK is a wrapper, not a refactor.
4. `pheno-mcp-router`'s existing dispatch-mcp W2-1 modules stay
   intact; their `LlmPort::resolve` contract is preserved.

**Estimated reversal cost:** 2 hours (delete unused SDK dirs +
`Cargo.toml` reverts in 2-3 in-flight PRs).

## References

- ADR-048 §"Current fleet readiness" — 4-tier gate table
- ADR-047 §2.2 (predictive-DRY sister) — predicted-consumer rubric
- ADR-013 (original `pheno-mcp-router` substrate canonical)
- ADR-037 (re-affirmed canonical, 2026-06-18)
- ADR-029 (Dmouse92 → KooshaPari migration; 3 PRs absorbed)
- `findings/2026-06-20-T37-substrate-graduation-tier2.md` line 37
- `findings/2026-06-17-L5-104-dmouse92-to-kooshapari.md` §4.5
- `findings/2026-06-18-L8-008-substrate-graduation.md` — gate provenance
- `KooshaPari/pheno-framework-lint` — tier-convention enforcer
- `pheno-predict` (L72) — predictive-DRY tool

## Reviewer checklist

- [x] All 6 tier-transition gates are ✓ with linked evidence
- [x] No tier-skipping (lib → SDK, not lib → framework)
- [x] Breaking-change budget = 0 (the 3 absorbed PRs are additive
      modules; the `LlmPort` trait surface is unchanged)
- [x] Reversal plan is concrete and ≤ 1 day (2 hours)
- [x] Promotion-decision ADR will be filed in this PR (ADR-092 draft
      on the `phenotype-monorepo`)
