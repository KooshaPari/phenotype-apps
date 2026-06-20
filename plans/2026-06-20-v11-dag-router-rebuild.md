# v11 DAG — Router Architecture Rebuild — 2026-06-20

## Tracks

### T34: Bifrost bump
- Bump bifrost-extensions to upstream v1.5.21
- Resolve API drift (if any)
- Run smoke tests across all 9 plugins
- Update version pin in go.mod

### T35: Token router consolidation
- Tokn canonical substrate (ADR-001)
- Migrate helios-router router code → Tokn (where reuse exists)
- Document router contract in SPEC.md

### T36: cliproxyapi-plusplus health
- Run on macOS — verify mac targets
- Run on Linux — verify Linux targets
- Audit for Windows-specific code that should NOT be on macOS
- Update CI matrix

### T37: Argis adapter integration
- Define adapter contract
- Implement Argis-as-Bifrost-plugin
- E2E test: Argis → Bifrost → provider

### Side-DAG Fillers (T33 remainder, 9 tasks)
- SOTA sweep across 9 Rust substrate crates
- Guardrail hardening (deny.toml, CI checks)
- Coverage analysis

### T0.5: Wrap-up
- AGENTS.md closure
- Final verification
