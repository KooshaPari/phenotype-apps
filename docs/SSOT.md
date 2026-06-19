# SSOT (Single Source of Truth)

This document is the canonical reference for **architectural decisions** in this
repository. When in doubt, this document is authoritative; code may be
aspirational until it catches up.

## Purpose

- Record the **why** behind non-obvious technical choices
- Pin **invariants** the codebase must respect
- Provide a stable reference for cross-team and cross-repo work
- Reduce duplicate explanations across ADRs, READMEs, and code comments

## Scope

This SSOT covers:

- **Language & toolchain versions** (e.g., Rust stable, MSRV)
- **Module layout & hexagonal architecture boundaries**
- **Naming conventions** (files, modules, crates, types)
- **Error handling contract** (which crate owns which error type)
- **Testing strategy** (unit, integration, property, doc)
- **Dependency policy** (allowed licenses, security advisories, deprecations)
- **CI gates** (what must pass before merge)
- **Release & versioning policy** (semver, breaking-change protocol)
- **Observability contract** (logging, metrics, traces, OTEL)
- **Persistence & data ownership** (where each entity lives)

## Non-Goals

This SSOT is **not**:

- A tutorial — see the README
- An API reference — see rustdoc / docs.rs
- A changelog — see CHANGELOG.md
- A roadmap — see the issue tracker

## Living Document

Whenever an architectural decision is made (in an ADR, PR, or review), update
this document in the **same PR**. Out-of-date SSOTs are a worse signal than no
SSOT.

## How to use this document

1. Before opening a PR, skim the relevant section
2. If your PR changes an architectural decision, update the corresponding
   section and call it out in the PR description
3. If you find a contradiction, open an issue titled `SSOT drift: <topic>`
