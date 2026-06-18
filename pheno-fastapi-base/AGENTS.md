# pheno-fastapi-base — AGENTS

## Project Overview

Canonical FastAPI base for the pheno-* service fleet. Provides:

- `pheno_fastapi_base.create_app` — FastAPI factory wiring up `/healthz` and
  `/readyz` probes, structlog access-log middleware, `app.started` startup
  hook, and the `AppError` exception handler.
- `pheno_fastapi_base.errors.AppError` — framework-agnostic exception with
  stable string `code` mapped to HTTP status via
  `pheno_fastapi_base.errors.code_to_status`.
- `pheno_fastapi_base.testing.AsyncTestClient` — thin wrapper over
  `httpx.AsyncClient` driving the FastAPI app in-process via
  `httpx.ASGITransport`.

## Conventions

- Branch naming: `feat/<req-id>-<slug>-<date>`, `chore/<req-id>-<slug>-<date>`
- Commit messages: Conventional Commits (`feat:`, `fix:`, `chore:`, etc.)
- PR labels: `governance`, `L<n>-#<n>` for tracking against DAG level
- License: dual (MIT + Apache-2.0)

## Quality bar

- 71-pillar score: 23/71 (Tier 0)
- Test matrix: 5 contract tests + smoke (`tests/test_app.py`)
- Coverage: pending measurement
- License: dual

## See also

- [ADR-039](docs/adr/2026-06-18/ADR-039-pheno-flake-refresh-template.md) — pheno-flake template
- [ADR-040](docs/adr/2026-06-18/ADR-040-test-coverage-gates-per-tier.md) — coverage gates
- L6 fleet health inventory