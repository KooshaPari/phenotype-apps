# Eidolon — Audit DAG Plan & WBS

**Audit date:** 2026-07-08
**Baseline score:** 61.3% (Grade C) — 94 satisfied / 34 partial / 53 missing / 3 N/A
**Taxonomy:** pillar-taxonomy-v2-140
**Repo type:** Rust device-automation framework (CLI + library + MCP server, 5 crates, 15.6K LOC)

---

## 1. Domain breakdown

| Domain | Score | Grade | Sat | Par | Mis | N/A | Priority trend |
|---|---|---|---|---|---|---|---|
| Code Quality | 82.1% | B | 9 | 5 | 0 | 0 | → |
| Architecture | 67.6% | C | 7 | 5 | 5 | 0 | ↓ |
| Testing | 44.2% | D | 9 | 5 | 12 | 0 | ↓↓↓ |
| Observability | 38.9% | F | 3 | 3 | 12 | 0 | ↓↓↓ |
| Security | 81.3% | B | 21 | 5 | 5 | 1 | → |
| Documentation | 47.1% | D | 6 | 3 | 8 | 0 | ↓↓ |
| CI/CD | 78.6% | C+ | 9 | 2 | 3 | 0 | → |
| Supply Chain | 69.4% | C | 11 | 5 | 3 | 1 | → |
| Release Engineering | 23.1% | F | 2 | 1 | 9 | 1 | ↓↓↓ |
| Developer Experience | 44.2% | D | 4 | 0 | 8 | 1 | ↓↓ |

---

## 2. Critical gaps (D/F domains — 6 domains)

### Release Engineering (23.1% F) — 9 missing

| # | Item | Est. | Priority |
|---|---|---|---|
| RE-01 | Document semver policy | 15 min | P1 |
| RE-02 | Cut v0.0.1 release tag | 5 min | P1 |
| RE-03 | Add git-cliff changelog automation | 30 min | P1 |
| RE-04 | Add pre-release CI testing | 15 min | P1 |
| RE-05 | Publish to crates.io | 15 min | P1 |
| RE-06 | Add release SLO documentation | 30 min | P2 |
| RE-07 | Write rollback plan | 30 min | P2 |
| RE-08 | Add BREAKING_CHANGES.md | 15 min | P2 |
| RE-09 | Create release checklist | 30 min | P2 |

### Observability (38.9% F) — 12 missing

| # | Item | Est. | Priority |
|---|---|---|---|
| OBS-01 | Add OpenTelemetry OTLP exporter | 2h | P1 |
| OBS-02 | Add request/operation ID to handlers | 1h | P1 |
| OBS-03 | Add /metrics + /healthz endpoints | 1h | P1 |
| OBS-04 | Add metrics counters for dispatch | 1h | P2 |
| OBS-05 | Add W3C trace context propagation | 1.5h | P2 |
| OBS-06 | Add custom panic hook | 15 min | P3 |
| OBS-07 | Add span context on errors | 30 min | P2 |
| OBS-08 | Add latency histograms | 1h | P2 |
| OBS-09 | Add sensitive data redaction | 30 min | P3 |
| OBS-10 | Audit trail for device ops | 1h | P2 |
| OBS-11 | Obs integration tests | 1h | P3 |
| OBS-12 | JSON error output for API | 30 min | P2 |

### Documentation (47.1% D) — 8 missing

| # | Item | Est. | Priority |
|---|---|---|---|
| DOC-01 | Create ARCHITECTURE.md | 1h | P1 |
| DOC-02 | Add crate-level rustdoc to all 5 | 30 min | P1 |
| DOC-03 | Generate OpenAPI spec | 1h | P2 |
| DOC-04 | Add migration guide | 30 min | P2 |
| DOC-05 | Add examples/ directory | 1h | P1 |
| DOC-06 | Add module-level docs | 45 min | P2 |
| DOC-07 | Add GLOSSARY.md | 30 min | P2 |
| DOC-08 | Add FAQ.md + RFC process | 30 min | P3 |

### Developer Experience (44.2% D) — 8 missing

| # | Item | Est. | Priority |
|---|---|---|---|
| DX-01 | Add nextest.toml | 10 min | P1 |
| DX-02 | Add sccache CI caching | 10 min | P1 |
| DX-03 | Add lefthook pre-commit hooks | 15 min | P1 |
| DX-04 | Add rust-project.json | 10 min | P2 |
| DX-05 | Add .devcontainer/devcontainer.json | 10 min | P1 |
| DX-06 | Add .env.example | 5 min | P1 |
| DX-07 | Add commit-msg hook for conventional commits | 10 min | P2 |
| DX-08 | Add bug_report.yml + PULL_REQUEST_TEMPLATE.md | 15 min | P1 |

### Testing (44.2% D) — 12 missing

| # | Item | Est. | Priority |
|---|---|---|---|
| TST-01 | Add property-based tests (proptest) | 2h | P1 |
| TST-02 | Add fuzz targets (cargo-fuzz for MCP) | 3h | P2 |
| TST-03 | Add nextest CI integration | 30 min | P1 |
| TST-04 | Add coverage gate (cargo-llvm-cov 70%) | 1h | P1 |
| TST-05 | Add doc tests | 1h | P2 |
| TST-06 | Add edge case tests for device disconnect | 1h | P2 |
| TST-07 | Fix timing-sensitive tests | 1h | P2 |
| TST-08 | Standardize AAA test organization | 1h | P3 |
| TST-09 | Add perf regression tests | 2h | P3 |
| TST-10 | Add E2E smoke test | 2h | P2 |
| TST-11 | Tag slow tests with #[ignore] | 15 min | P2 |
| TST-12 | Add coverage badges | 15 min | P3 |

---

## 3. DAG

```
Phase 0: Quick wins (parallel, ~2h total)
  [nextest.toml] [devcontainer.json] [.env.example]
  [issue/PR templates] [lefthook hooks] [sccache CI]
  [semver policy doc] [v0.0.1 tag]
  [crate-level rustdoc] [module-level docs]

       │
       ▼

Phase 1: Testing + observability backbone
  [proptest] [fuzz targets] [nextest CI + coverage gate]
  [OTel exporter] [/metrics + /healthz] [request IDs]
  [arch docs + ARCHITECTURE.md] [examples/]

       │
       ▼

Phase 2: Security + hardening
  [W3C trace context] [latency histograms] [panic hook]
  [span context on errors] [edge case tests] [E2E smoke]
  [OpenAPI spec] [migration guide] [glossary]

       │
       ▼

Phase 3: Release readiness
  [crates.io publish] [pre-release CI] [rollback plan]
  [BREAKING_CHANGES.md] [release checklist]
  [perf regression tests] [FAQ + RFC process]
```

## 4. Projected scores

| Phase | Target | Delta |
|---|---|---|
| Phase 0 | 68% | +7 pts |
| Phase 1 | 79% | +11 pts |
| Phase 2 | 86% | +7 pts |
| Phase 3 | 90% | +4 pts |
