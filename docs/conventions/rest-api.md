# REST API design conventions (v16 T2 / L9)
# Date: 2026-06-21
# Pillar: L9 (Architecture — REST API design conventions)
# Adopters: phenotype-router, pheno-mcp-router, Eidolon, agent-platform, pheno-port-adapter

## Overview

All fleet services that expose HTTP/REST APIs follow these conventions. The goals
are predictability (clients can guess endpoints), debuggability (errors carry
machine-readable context), and operability (rate-limit and auth are first-class).
Every section below is enforced at code review and is a precondition for the
L9 score in the 71-pillar audit.

## 1. Resource naming

Resources are **plural nouns** in **kebab-case**, with optional hierarchical
sub-resources separated by `/`. The HTTP verb expresses the action; the URL
expresses only the noun.

| Pattern | Example | Notes |
|---------|---------|-------|
| Collection | `GET /v1/agents` | list/create |
| Item | `GET /v1/agents/{id}` | read/update/delete |
| Sub-collection | `GET /v1/agents/{id}/sessions` | nested resource under an item |
| Action | `POST /v1/agents/{id}:restart` | verb-style endpoint using `:` (gRPC convention) — use sparingly |

Rules:
- **Nouns, not verbs** in path segments (`/agents`, not `/getAgents`).
- **Plural** for collections (`/sessions`, not `/session`).
- **kebab-case** for multi-word resources (`/api-keys`, not `/apiKeys` or `/api_keys`).
- **Lowercase** throughout; uppercase is reserved for query-param enums.
- **No file extensions** in the path (`/agents/123.json` is forbidden — use `Accept: application/json`).
- **Trailing slashes are forbidden** — redirect to canonical form at the gateway.

## 2. HTTP status codes

Use the most specific code that matches the outcome. Do not blanket-return `200`
with a body-encoded status.

### Success (2xx)

| Code | When | Example |
|------|------|---------|
| `200 OK` | Read, update, or non-idempotent action that returned a body | `GET /v1/agents/abc` |
| `201 Created` | Resource was created; include `Location: <url>` header | `POST /v1/agents` |
| `202 Accepted` | Request is queued or scheduled; processing is async | `POST /v1/agents/abc:restart` |
| `204 No Content` | Success with no body (delete, ack) | `DELETE /v1/agents/abc` |
| `206 Partial Content` | Range / paginated subset | `GET /v1/agents?limit=10` (with cursor) |

### Client error (4xx)

| Code | When |
|------|------|
| `400 Bad Request` | Malformed JSON, missing required field, schema violation |
| `401 Unauthorized` | Missing or invalid `Authorization` header |
| `403 Forbidden` | Authenticated but not permitted (RBAC denial) |
| `404 Not Found` | Resource does not exist OR caller is not allowed to know it exists |
| `409 Conflict` | State conflict (idempotency-key reuse with different payload, version mismatch) |
| `410 Gone` | Resource was permanently removed (deletion tombstone) |
| `412 Precondition Failed` | `If-Match` / `If-Unmodified-Since` ETag check failed |
| `422 Unprocessable Entity` | Syntactically valid JSON but semantically wrong (e.g. cron expression) |
| `429 Too Many Requests` | Rate limit exceeded — include `Retry-After` header |

### Server error (5xx)

| Code | When |
|------|------|
| `500 Internal Server Error` | Unhandled exception — log a trace ID, do not leak internals to client |
| `502 Bad Gateway` | Upstream service (LLM, MCP server) returned an invalid response |
| `503 Service Unavailable` | Dependency down or graceful-shutdown in progress; include `Retry-After` |
| `504 Gateway Timeout` | Upstream service exceeded its SLO |

## 3. Error envelope: RFC 7807 `application/problem+json`

All 4xx and 5xx responses use [RFC 7807 Problem Details for HTTP APIs](https://datatracker.ietf.org/doc/html/rfc7807).
The `Content-Type` header MUST be `application/problem+json`.

```json
{
  "type": "https://phenotype.example/probs/out-of-credit",
  "title": "You do not have enough credit.",
  "detail": "Your current balance is 30, but that costs 50.",
  "instance": "/account/12345/msgs/abc",
  "status": 403
}
```

Field semantics:

| Field | Required | Type | Meaning |
|-------|----------|------|---------|
| `type` | recommended | URI | Identifier for the problem class. Either a URL to docs or `about:blank` |
| `title` | required | string | Short, human-readable summary — MUST NOT change for the same `type` |
| `status` | required | int | The HTTP status code, duplicated in the body for log correlation |
| `detail` | optional | string | Human-readable explanation specific to this occurrence |
| `instance` | required | string | The request path that produced the error (not the resource URI) |
| `code` (extension) | recommended | string | Stable machine-readable code, e.g. `OUT_OF_CREDIT` — recommended for clients |
| `trace_id` (extension) | required | string | W3C `traceparent` value for log correlation across services |

Extension fields (`code`, `trace_id`) are non-normative but recommended. Clients
SHOULD branch on `type` (stable) or `code` (machine), NOT on `title` or `detail`.

## 4. Pagination

Cursor-based pagination is the default for any collection that may grow
unbounded. Offset/limit is acceptable only for small bounded collections and
admin UIs.

Request:
```
GET /v1/agents?limit=20&cursor=eyJpZCI6ImFiYyJ9
```

Response: a top-level `items` array and a `next_cursor` field. `next_cursor`
being `null` or absent means "end of collection".

```json
{
  "items": [ ... ],
  "next_cursor": "eyJpZCI6ImRlZiJ9",
  "limit": 20
}
```

Rules:
- `limit` is capped server-side (default 50, max 200). Values above the cap return `400`.
- `cursor` is opaque to the client; do not parse.
- Stable ordering is the server's responsibility (typically `created_at DESC, id DESC`).
- `X-Total-Count` is **NOT** included — it is unreliable on unbounded sets and forces a count query.

## 5. Versioning

Versioning is **URL-path-based**: `/v1/`, `/v2/`, etc. The major version is
embedded in the path, not the header. Breaking changes require a new major
version; the prior version MUST be supported for at least 6 months after
deprecation.

```
GET /v1/agents         ← current
GET /v2/agents         ← new
```

When a route is deprecated, the response carries a `Deprecation: true` header
and a `Sunset: <RFC 1123 date>` header per RFC 8594. The `Deprecation` and
`Sunset` headers MUST also be set on a `Sunset` warn-page returned by
`GET /v1/deprecations`.

Additive changes (new optional fields, new endpoints, new enum values) are
non-breaking and may ship in the current major version. Removing or renaming a
field, changing a type, or changing semantics of an existing field is breaking.

## 6. Idempotency

`POST` endpoints that create or mutate state MUST accept an
`Idempotency-Key` header per [IETF draft-ietf-httpapi-idempotency-key-header](https://datatracker.ietf.org/doc/draft-ietf-httpapi-idempotency-key-header/).
The key is a client-generated opaque string up to 255 characters.

```
POST /v1/agents
Idempotency-Key: 7c4e8a3b-2d5f-4a1e-9c8b-6e7f8a9b0c1d
Content-Type: application/json
```

Server behavior:
- First request with a given key: process normally, store the response (status + body) keyed on the hash of `(key, user, body)`.
- Repeat request with the same key AND same body: replay the stored response.
- Repeat request with the same key BUT a different body: return `409 Conflict` with `type: about:blank`, `code: IDEMPOTENCY_KEY_REUSE`.
- The key is scoped to the authenticated user; cross-user replay returns the original user's response only if the original is owned by the same caller (otherwise `409`).

Keys expire 24 hours after first use.

## 7. Authentication

`Authorization: Bearer <jwt>` is the default scheme. The JWT is signed with
the platform's RS256 key, carries `sub`, `org`, `scopes`, and `exp` claims, and
is verified by the gateway before reaching the service.

```
GET /v1/agents
Authorization: Bearer eyJhbGciOiJSUzI1NiIs...
```

Failure modes:
- Missing header → `401` with `code: AUTH_MISSING`
- Expired token → `401` with `code: AUTH_EXPIRED`
- Invalid signature → `401` with `code: AUTH_INVALID`
- Token valid but scope missing → `403` with `code: AUTHZ_INSUFFICIENT_SCOPE`

Service-to-service calls use the same `Authorization` header with a machine
identity token from the federation mTLS issuer (see ADR-046).

## 8. Rate limiting

Every endpoint advertises its rate-limit budget via `X-RateLimit-*` response
headers per [RFC 9239 draft-polli-ratelimit-headers](https://datatracker.ietf.org/doc/draft-polli-ratelimit-headers/).

| Header | Sent when | Meaning |
|--------|-----------|---------|
| `X-RateLimit-Limit` | always | Total budget for the current window |
| `X-RateLimit-Remaining` | always | Budget remaining in the current window |
| `X-RateLimit-Reset` | always | Seconds until the window resets (epoch seconds in the IETF draft; we use delta-seconds for readability) |
| `Retry-After` | only on `429` | Seconds the client should wait before retrying |

Limits are token-bucket per `(user, endpoint-class)`. Endpoint classes:
- `read` — `GET`/`HEAD` (200/min default)
- `write` — `POST`/`PUT`/`PATCH` (60/min default)
- `delete` — `DELETE` (30/min default)
- `auth` — `/v1/auth/*` (10/min, hard)

When a client exceeds its budget, the response is `429` with the same
`application/problem+json` envelope and `code: RATE_LIMITED`.

## Adoption checklist

For every service that exposes an HTTP API:

1. **Naming audit**: scan routes for verb segments, singular collections, and snake_case; rename or refactor.
2. **Status code sweep**: replace any `200`-with-error-body with the correct `4xx`/`5xx`.
3. **RFC 7807 envelope**: implement a single `problem()` helper that all error paths funnel through.
4. **Cursor pagination**: refactor any `?offset=&limit=` collection into cursor form.
5. **Versioned path**: every route lives under `/v{N}/`.
6. **Idempotency-Key** on `POST` that creates state; document which endpoints accept it.
7. **`Authorization: Bearer <jwt>`** at the gateway; service trusts the gateway.
8. **`X-RateLimit-*` headers** on every response; `Retry-After` on `429`.

## References

- [RFC 7807 — Problem Details for HTTP APIs](https://datatracker.ietf.org/doc/html/rfc7807)
- [RFC 8594 — The Sunset HTTP Header Field](https://datatracker.ietf.org/doc/html/rfc8594)
- [draft-ietf-httpapi-idempotency-key-header](https://datatracker.ietf.org/doc/draft-ietf-httpapi-idempotency-key-header/)
- [draft-polli-ratelimit-headers](https://datatracker.ietf.org/doc/draft-polli-ratelimit-headers/)
- [Microsoft REST API Guidelines](https://github.com/microsoft/api-guidelines)
- [Google API Design Guide](https://cloud.google.com/apis/design)
- [ADR-046 — Federation mTLS + OIDC](../../adr/2026-06-18/ADR-046-federation-mtls-oidc.md)
