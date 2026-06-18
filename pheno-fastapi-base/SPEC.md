# pheno-fastapi-base — SPEC

## Scope

Canonical FastAPI base for the pheno-* service fleet. Every pheno-* HTTP
service is expected to share these building blocks:

- `/healthz` and `/readyz` probes
- structlog-based JSON access-log middleware
- startup hook emitting `app.started` event
- framework-agnostic `AppError` exception type
- `AsyncTestClient` for in-process testing

## Public API

- `create_app(title: str, version: str) -> FastAPI`
  Factory that wires up all standard middleware + handlers.
- `class AppError(Exception)`
  - `code: str` — stable wire-code (`"NotFound"`, `"Validation"`, …)
  - `message: str` — human-readable
  - `details: dict | None`
- `code_to_status: dict[str, int]` — canonical code → HTTP status map.
- `register_error_handlers(app: FastAPI) -> None`
- `class AsyncTestClient(httpx.AsyncClient)`
  Drives FastAPI app in-process via `httpx.ASGITransport`.

## Conventions

- **When to use:** any new pheno-* HTTP service.
- **When NOT to use:** non-HTTP services (use plain Pydantic models only).
- **5-line quickstart:**
  ```python
  from pheno_fastapi_base import create_app, AppError
  app = create_app(title="my-service", version="0.1.0")
  @app.get("/items/{id}")
  async def get_item(id: str):
      raise AppError("NotFound", f"no item {id}")
  ```

## Probe contracts

- `GET /healthz` → 200 `{"status": "ok"}` always.
- `GET /readyz` → 200 `{"status": "ready"}` once startup hook completed.

## Quality bar

- 71-pillar score: 23/71 (Tier 0)
- Test matrix: 5 contract tests + smoke (`tests/test_app.py`)
- Coverage: pending measurement
- License: dual (MIT + Apache-2.0)

## See also

- ADR-039 (pheno-flake template)
- ADR-040 (coverage gates)