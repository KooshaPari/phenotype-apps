# pheno-port-adapter

[![Crates.io](https://img.shields.io/crates/v/pheno-port-adapter.svg)](https://crates.io/crates/pheno-port-adapter)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE-MIT)
[![71-pillar: L9](https://img.shields.io/badge/71--pillar-L9-blue.svg)](https://phenotype.dev/pillars)

Hexagonal port-adapter substrate for the Phenotype fleet. Provides the canonical
`PortAdapter` trait, `Connection` handle, and `AdapterError` envelope.

This crate is **library-only** — it does not expose HTTP endpoints directly. It is the
substrate that HTTP services (phenotype-router, Civis, etc.) build on. The curl blocks
below illustrate the typical HTTP wrapper integration following the
[L9 REST API conventions](../../docs/conventions/rest-api.md) (RFC 7807 error envelope).

## Quickstart (Rust library)

```rust
use pheno_port_adapter::{PortAdapter, adapters::TcpAdapter};

let adapter = TcpAdapter::new("pheno-port-adapter");
adapter.health()?;
let conn = adapter.connect("tcp://localhost:8080")?;
// ... use conn ...
adapter.disconnect()?;
```

## HTTP wrapper curl examples (illustrative)

When `pheno-port-adapter` is wired into an HTTP service (e.g., `phenotype-router`), the
following curl blocks demonstrate the [L9 REST API conventions](../../docs/conventions/rest-api.md).
The `<host>`, `<port>`, and `<resource>` placeholders must be replaced with the deploying
service's values.

### Health check (GET 200)

```bash
curl -sS -X GET \
  -H "Accept: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  "https://<host>/api/v1/pheno-port-adapter/connectors/<id>/health"
```

Successful response (HTTP 200):

```json
{
  "status": "healthy",
  "adapter": "pheno-port-adapter",
  "checked_at": "2026-06-21T10:00:00Z"
}
```

### Validation error — RFC 7807 Problem Details (HTTP 400)

```bash
curl -sS -X POST \
  -H "Accept: application/problem+json" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer $TOKEN" \
  -d '{}' \
  "https://<host>/api/v1/pheno-port-adapter/connectors"
```

Error response (HTTP 400):

```json
{
  "type": "https://phenotype.dev/errors/validation-failed",
  "title": "Validation Failed",
  "status": 400,
  "detail": "Field 'endpoint' is required",
  "instance": "/api/v1/pheno-port-adapter/connectors",
  "code": "VALIDATION_FAILED",
  "errors": [
    {"field": "endpoint", "code": "REQUIRED", "message": "Field is required"}
  ]
}
```

### Rate-limited (HTTP 429)

```bash
curl -sS -i -X GET \
  -H "Accept: application/json" \
  "https://<host>/api/v1/pheno-port-adapter/connectors"
```

Response headers:

```
HTTP/1.1 429 Too Many Requests
Content-Type: application/problem+json
Retry-After: 60
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1718947200
```

## API surface

| Item | Kind | Description |
|------|------|-------------|
| `PortAdapter` | trait | Canonical port-adapter contract |
| `Connection` | struct | Opaque handle for an active connection |
| `AdapterError` | enum | Error envelope (`ConnectFailed`, `DisconnectFailed`, `HealthCheckFailed`, `Timeout`) |
| `adapters::TcpAdapter` | struct | TCP transport adapter |
| `adapters::UnixAdapter` | struct | Unix-domain socket adapter |

## Error envelope mapping to RFC 7807

When HTTP wrappers convert `AdapterError` into HTTP responses, the mapping is:

| `AdapterError` variant | HTTP status | `code` (RFC 7807) |
|------------------------|------------:|-------------------|
| `ConnectFailed(_)`     | 502 | `CONNECT_FAILED` |
| `DisconnectFailed(_)`  | 502 | `DISCONNECT_FAILED` |
| `HealthCheckFailed(_)` | 503 | `HEALTH_CHECK_FAILED` |
| `Timeout`              | 504 | `ADAPTER_TIMEOUT` |

## Conventions

This crate follows the [Phenotype REST API conventions](../../docs/conventions/rest-api.md):

- L9 — RFC 7807 Problem Details error envelope.
- L9.5 — `/openapi.json` published by HTTP wrappers.
- L9.6 — Cursor-based pagination.
- L9.7 — `Idempotency-Key` header on POST.

## Development

```bash
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
```

## License

MIT — see [LICENSE-MIT](LICENSE-MIT).
