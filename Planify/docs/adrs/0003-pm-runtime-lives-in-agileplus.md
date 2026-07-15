# ADR-0003: PM runtime lives in AgilePlus, not in planify-wt

- **Status**: Accepted
- **Date**: 2026-07-14

## Context
We have a Plane fork (planify-wt) AND a Rust PM workspace member (`agileplus-plane`) in AgilePlus. Where does the actual PM runtime live?

## Decision
**The PM runtime lives in `AgilePlus/crates/agileplus-plane` (Rust). planify-wt is a public landing site + a frozen upstream reference.**

## Consequences
- Customization of the runtime happens in AgilePlus (Rust + SQLx + axum).
- The planify.space landing site is what users hit first; the actual PM tool is at `pm.phenotype.tailnet` (or similar).
- No duplicated code — single source of truth for PM logic.
- **AI-DD**: every Plane issue can be linked to a Grapheon claim via `agileplus-plane`'s integration layer.

## Alternatives considered
- **Build runtime in planify-wt**: rejected — duplicates AgilePlus's PM server.
- **Use plane.so as a hosted service**: rejected — costs scale linearly with users; we want self-hosted for the multi-tenant Phenotype platform.
