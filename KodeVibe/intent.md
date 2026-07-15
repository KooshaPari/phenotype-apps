# Intent — KodeVibe

## Problem statement

Code quality tooling was split between an archived Go repo (KodeVibeGo), shell wrappers, and overlapping claims with kwality and HexaKit compliance schema. Agents could not reliably distinguish static analysis runtime from LLM validation or governance-only schemas.

## Success criteria

- [ ] Go `engine/` is canonical static-analysis runtime; shell CLI delegates cleanly
- [ ] Quality platform map documents KodeVibe vs kwality vs HexaKit schema
- [ ] Genesis doc set linked and agent-readable
- [ ] Go engine choice documented in SOTA (Tier 3 justified edge)

## Non-goals

See [charter.md](charter.md#out-of-scope). Key exclusions:

- LLM validation pipelines (owned by `kwality`)
- Governance rule schema without runtime (owned by `HexaKit`)
- Replacing KodeVibeGo archive (tombstone preserved)

## Originating prompts

Deterministic provenance in [docs/intent/prompts/](docs/intent/prompts/README.md).

| Date | Tool | Session | Summary |
|------|------|---------|---------|
| 2026-06-16 | cursor | genesis-rollout | [quality role + Go engine SOTA](docs/intent/prompts/.gitkeep) |

Refresh: `python scripts/extract-intent-prompts.py --out-dir docs/intent/prompts --repo KodeVibe`

## Synthesized goals

Full synthesis: [docs/intent/synthesis.md](docs/intent/synthesis.md)

**Confirmed (user-stated):**

1. Own `quality` domain role for deterministic static analysis
2. Go engine justified in SOTA per LANGUAGE_PLACEMENT Tier 3
3. Bootstrap genesis governance from HexaKit `templates/genesis/`

**Inferred (needs validation):**

1. Shell CLI remains UX layer; engine/ is long-term core
2. MCP/daemon paths are first-class for agent integration

## Agent assumptions log

| Assumption | Action taken | Validated? |
|------------|--------------|------------|
| Genesis rollout on feat/genesis-docs-rollout | Copied and customized genesis template | pending |
| Go engine migration from KodeVibeGo is complete enough for SOTA claim | SOTA technical.md language table | pending |

Details: [docs/intent/assumptions.md](docs/intent/assumptions.md)
