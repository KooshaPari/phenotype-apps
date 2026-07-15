# ADR-0001: Fork, don't rewrite plane.so

- **Status**: Accepted
- **Date**: 2026-07-14
- **Deciders**: KooshaPari

## Context
We need a Plane-like PM tool. The natural temptation is to rewrite from scratch. Should we?

## Decision
**Fork plane@preview v1.3.1 verbatim, write only our landing/infra, and let AgilePlus's `agileplus-plane` crate (Rust) handle the runtime.**

## Consequences
- `upstream/` is read-only — preserves the option to merge future upstream PRs.
- We don't waste engineering on a 200k-LOC rewrite.
- The landing site (`site/`) is the only part we actively maintain.
- The actual PM runtime lives in **AgilePlus** as `agileplus-plane` (Rust workspace member). We can customize via that surface.

## Alternatives considered
- **Rewrite**: rejected — 6+ months of work for a marginal feature win.
- **Vendor as dependency**: rejected — plane.so's Apache license allows it, but our theme system would require theme patches anyway.
- **Build a new lightweight tool**: deferred — when claim count >50k, we'll re-evaluate.
