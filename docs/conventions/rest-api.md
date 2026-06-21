# REST API design conventions (v16 T2 / L9)
# Date: 2026-06-21
# Pillar: L9 (REST + OpenAPI) — Industry: Microsoft REST API Guidelines, Zalando RESTful
#         API Guidelines, Google AIP, RFC 7807 (Problem Details for HTTP APIs),
#         RFC 8594 (Sunset), RFC 9457 (Problem Details, 2023 successor)

This document is the **canonical contract** for every HTTP API published under a
`KooshaPari/*` repo. All 5 service READMEs (phenotype-router, pheno-mcp-router,
nanovms, Eidolon, agent-platform) carry an `## REST API examples` section that
conforms to this spec.

**Status:** adopted 2026-06-21 by v16 cycle-6 Track T2.

---

## 1. Resource naming

URL paths describe **resources**, not actions or RPC names.

| Rule | Example (good) | Example (bad) |
|------|----------------|---------------|
| Plural noun for collections | `GET /v1/orders` | `GET /v1/order` |
| Kebab-case for multi-word | `GET /v1/order-items` | `GET /v1/orderItems` |
| Hierarchical ownership via nesting | `GET /v1/orders/{order_id}/line-items` | `GET /v1/line-items?order_id=...` |
| IDs are opaque strings (UUIDv4 or ULID) | `/v1/users/01HXYZ...` | `/v1/users/12345` |
| Verbs only appear on actions (rare) | `POST /v1/orders/{id}:cancel` | `POST /v1/cancelOrder` |
| Lowercase only (no camelCase, no snake_case in paths) | `/v1/billing-accounts` | `/v1/billingAccounts` |

**Sub-resources** (e.g., line-items under an order) are nested **at most one
level deep**. If a second nesting is needed, promote the sub-resource to a
top-level collection and reference the parent via query parameter.

**Singleton resources** (the user's profile, the workspace's billing settings)
are addressable without an ID when there is exactly one for the principal:
`GET /v1/me`, `GET /v1/workspaces/{ws_id}/billing`.

---

## 2. HTTP status codes

Use the smallest status code that conveys the full meaning. Never return `200 OK`
with an error payload — clients must be able to dispatch on the status line alone.

### 2xx — Success

| Code | When | Response body |
|------|------|---------------|
| `200 OK` | Standard success with body | resource representation |
| `201 Created` | A new resource was created (POST, PUT that creates) | created resource + `Location` header |
| `202 Accepted` | Request queued for async processing | `{ "job_id": "...", "status": "queued" }` |
| `204 No Content` | Success with no body (DELETE, PUT-with-no-echo) | empty |

### 3xx — Redirection

| Code | When |
|------|------|
| `301 Moved Permanently` | Resource renamed; clients must update bookmarks |
| `304 Not Modified` | Conditional GET (`If-None-Match` / `If-Modified-Since`) hit |
| `308 Permanent Redirect` | Permanent redirect preserving method + body |

### 4xx — Client errors (RFC 7807 envelope, see §3)

| Code | When |
|------|------|
| `400 Bad Request` | Malformed JSON, missing required field, schema violation |
| `401 Unauthorized` | Missing or invalid credentials (no `Authorization` header, or expired token) |
| `403 Forbidden` | Authenticated but not authorized (RBAC denial, quota exceeded) |
| `404 Not Found` | Resource does not exist OR caller may not see it (avoid information leak) |
| `409 Conflict` | Optimistic concurrency conflict (`If-Match` mismatch), unique-constraint violation |
| `410 Gone` | Resource permanently deleted (RFC 8594 `Sunset` header recommended) |
| `412 Precondition Failed` | `If-Match` / `If-Unmodified-Since` precondition failed |
| `415 Unsupported Media Type` | `Content-Type` not accepted by this endpoint |
| `422 Unprocessable Entity` | Schema-valid but semantically invalid (e.g., end-date before start-date) |
| `429 Too Many Requests` | Rate-limit exceeded; respond with `Retry-After` (see §8) |

### 5xx — Server errors

| Code | When |
|------|------|
| `500 Internal Server Error` | Unhandled exception; **never** expose stack traces in body |
| `502 Bad Gateway` | Upstream service returned invalid response |
| `503 Service Unavailable` | Planned maintenance OR overload; respond with `Retry-After` |
| `504 Gateway Timeout` | Upstream service did not respond within the deadline |

---

## 3. Error envelope: RFC 7807 `application/problem+json`

Every 4xx and 5xx response **must** use the `application/problem+json` media type
and follow the [RFC 7807 / RFC 9457](https://www.rfc-editor.org/rfc/rfc7807)
shape. Clients parse the envelope, not a bespoke error schema.

```http
HTTP/1.1 403 Forbidden
Content-Type: application/problem+json

{
  "type": "https://api.example.com/probs/out-of-credit",
  "title": "You do not have enough credit.",
  "detail": "Your current balance is 30, but that costs 50.",
  "instance": "/account/12345/msgs/abc",
  "status": 403,
  "code": "OUT_OF_CREDIT",
  "trace_id": "01HXYZABCDEF...",
  "errors": [
    { "field": "amount", "code": "MIN_VALUE", "message": "must be ≤ 30" }
  ]
}
```

### Field rules

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `type` | URI string | yes | Resolvable URL to the problem class; use the API's docs domain |
| `title` | string | yes | Short human-readable summary, **stable per `type`** |
| `detail` | string | recommended | Per-occurrence human-readable detail |
| `instance` | URI string | recommended | URI of the specific occurrence (request path or correlation ID) |
| `status` | integer | recommended | Mirror of the HTTP status code; helps log aggregation |
| `code` | string | recommended | Machine-readable, stable per `type` (e.g., `OUT_OF_CREDIT`); UPPER_SNAKE_CASE |
| `trace_id` | string | recommended | W3C `traceparent` or equivalent; join key for log search |
| `errors` | array | optional | Per-field validation errors (subset of `type`) |

### Conventions specific to the fleet

- `type` URLs are rooted at `https://api.phenotype.dev/probs/<kebab-case-class>`.
- `code` is **stable across versions** — clients branch on it. Renaming a code is a breaking change.
- `instance` is the request path (`/account/12345/msgs/abc`), not a fully-qualified URL.
- `trace_id` is a ULID, lowercase, 26 characters.

---

## 4. Pagination

Two pagination styles are supported; **the API may pick one but must document
which**. Clients SHOULD NOT assume a default.

### 4.1 Cursor-based (preferred for large / append-only collections)

```http
GET /v1/orders?limit=50&cursor=eyJpZCI6IjAxSFhZW... HTTP/1.1
```

Response body:

```json
{
  "data": [ { "id": "01HXYZ...", "total_cents": 4500 }, ... ],
  "next_cursor": "eyJpZCI6IjAxSFhZWjI...",
  "has_more": true
}
```

- `limit` is the page size; default 50, max 200.
- `next_cursor` is opaque (base64-encoded JSON or signed token). Clients MUST NOT parse it.
- `has_more: false` and an absent `next_cursor` signal the terminal page.

### 4.2 Offset-based (only for small bounded collections, e.g., admin tables)

```http
GET /v1/admin/audit-log?limit=50&offset=200 HTTP/1.1
```

- `offset` defaults to 0, `limit` defaults to 50, max 200.
- Total count is returned as a header: `X-Total-Count: 12345`.

### Sort

`?sort=<field>` (single field) or `?sort=<field1>,<field2>` (multi). Prefix `-`
for descending: `?sort=-created_at`. Multi-field sorts must be a stable order.

---

## 5. Versioning

**URL-path versioning** with a major version segment. The version segment is
**the major version only** — `v1`, `v2` — never `v1.2` or `v1.2.3`.

```http
GET /v1/users/01HXYZ...        # current major
GET /v2/users/01HXYZ...        # next major (breaking changes)
```

### Lifecycle

- A new major version ships alongside the old one for **at least 6 months**.
- The old version emits a `Sunset: Sat, 01 Jan 2028 00:00:00 GMT` header (RFC 8594) at least 90 days before retirement.
- The old version emits a `Deprecation: true` header on every response at least 180 days before retirement.
- A `Link: <https://docs.example.com/api/migration/v1-to-v2>; rel="successor-version"` header points to the migration guide.

### Breaking-change policy

A change is breaking if it alters any of:
- request schema (added required field, removed field, type change)
- response schema (removed field, type change)
- status codes (success path → error path, or vice versa)
- error envelope field meanings (§3)
- authentication mechanism
- URL paths (renames count; pure additions do not)

Non-breaking: adding optional request fields, adding response fields, adding
new endpoints, adding new error codes in the envelope, deprecating (but not
removing) endpoints.

---

## 6. Idempotency

For any non-idempotent verb (`POST` and `DELETE` on custom actions), clients
**should** send an `Idempotency-Key` header to make the operation safe to
retry.

```http
POST /v1/payments HTTP/1.1
Idempotency-Key: 01HXYZABCDEF...             # ULID, ≤ 255 chars
Content-Type: application/json

{ "amount_cents": 4500, "currency": "USD" }
```

### Server rules

1. The server stores `(Idempotency-Key, request body hash, response)` for 24 hours.
2. A retry with the same `Idempotency-Key` and the **same body** returns the
   stored response without re-executing the operation.
3. A retry with the same `Idempotency-Key` and a **different body** returns
   `409 Conflict` with `application/problem+json` (RFC 7807) and `code: IDEMPOTENCY_KEY_REUSED`.
4. The server echoes the key back in the response header `Idempotency-Key` so
   the client can verify.

### Scope

The `Idempotency-Key` is scoped to the API key (or bearer subject). Two tenants
using the same key value never collide.

---

## 7. Authentication

Bearer-token auth on every endpoint that touches tenant data.

```http
GET /v1/orders HTTP/1.1
Authorization: Bearer eyJhbGciOiJSUzI1NiIs...
```

### Token format

- JWT signed with RS256; key published at `https://api.example.com/.well-known/jwks.json`.
- Required claims: `sub` (subject = user id), `iss` (must equal the API issuer),
  `aud` (must equal the API audience), `exp`, `iat`, `tenant_id`.
- Clock skew tolerance: 60 seconds.

### Refresh

- Short-lived access tokens (≤ 15 min) + longer-lived refresh tokens (≤ 30 d).
- Refresh tokens rotate on every use; the old token is invalidated.

### Service-to-service

- Federated services (per ADR-046) use **mTLS + OIDC client credentials**:
  client ID + signed JWT (`client_credentials` flow) exchanged for an access
  token at the federation issuer.

### Errors

- Missing token → `401 Unauthorized`, `code: AUTH_TOKEN_MISSING`.
- Expired token → `401 Unauthorized`, `code: AUTH_TOKEN_EXPIRED`,
  `WWW-Authenticate: Bearer error="invalid_token"`.
- Insufficient scope → `403 Forbidden`, `code: AUTH_INSUFFICIENT_SCOPE`,
  `WWW-Authenticate: Bearer error="insufficient_scope"`.

---

## 8. Rate limiting

Every endpoint returns rate-limit headers per
[IETF draft-ietf-httpapi-ratelimit-headers](https://datatracker.ietf.org/doc/draft-ietf-httpapi-ratelimit-headers/).

```http
HTTP/1.1 200 OK
RateLimit-Limit: 1000
RateLimit-Remaining: 742
RateLimit-Reset: 73

# On 429 responses:
HTTP/1.1 429 Too Many Requests
RateLimit-Limit: 1000
RateLimit-Remaining: 0
RateLimit-Reset: 27
Retry-After: 27
```

### Field rules

| Field | Type | Meaning |
|-------|------|---------|
| `RateLimit-Limit` | integer | Total requests permitted in the current window |
| `RateLimit-Remaining` | integer | Requests left in the current window |
| `RateLimit-Reset` | integer | Seconds until the window resets |
| `Retry-After` | integer (seconds) or HTTP-date | Sent only on `429`; **required** |

### Policy

- **Per principal** (token `sub` + `tenant_id`): default 1000 req / 60 s sliding window.
- **Per IP** for unauthenticated endpoints: 100 req / 60 s.
- **Burst allowance** is **none** — the limit is enforced strictly. Use jitter on the client side.
- The window is **sliding**, not fixed, so `RateLimit-Reset` is the time until the oldest request in the window ages out.

### 429 envelope

```json
{
  "type": "https://api.phenotype.dev/probs/rate-limit-exceeded",
  "title": "Rate limit exceeded.",
  "detail": "You have used 1000 of 1000 requests in the last 60 seconds.",
  "instance": "/v1/orders",
  "status": 429,
  "code": "RATE_LIMIT_EXCEEDED",
  "trace_id": "01HXYZABCDEF...",
  "retry_after_seconds": 27
}
```

---

## Quick checklist for new endpoints

Before opening a PR, confirm:

- [ ] Path uses plural nouns, kebab-case, hierarchical nesting (max 1 deep)
- [ ] HTTP status code matches the smallest semantic outcome
- [ ] All 4xx/5xx responses carry `Content-Type: application/problem+json`
- [ ] `type`, `title`, `status`, `code` are present in the error envelope
- [ ] Pagination style is documented (cursor preferred)
- [ ] URL path includes the major version (`/v1/...`)
- [ ] Non-idempotent verbs accept `Idempotency-Key`
- [ ] `Authorization: Bearer` required (or endpoint is explicitly public)
- [ ] `RateLimit-*` headers present on every response; `Retry-After` on 429
- [ ] OpenAPI 3.1 spec committed alongside (per L9 → OpenAPI generation plan)
- [ ] Example curl block in the repo README under `## REST API examples`

---

## References

- RFC 7807 — Problem Details for HTTP APIs (2016)
- RFC 9457 — Problem Details for HTTP APIs (2023, successor)
- RFC 8594 — The Sunset HTTP Header Field
- RFC 7231 — HTTP/1.1 Semantics and Content (status code definitions)
- Zalando RESTful API Guidelines — https://opensource.zalando.com/restful-api-guidelines/
- Microsoft REST API Guidelines — https://github.com/microsoft/api-guidelines
- Google API Improvement Proposals — https://google.aip.dev/
- IETF draft-ietf-httpapi-ratelimit-headers — RateLimit header fields
- ADR-046 — Federation mTLS + OIDC (internal)
- ADR-088 — Async runtime decision (internal; references this doc for I/O surface)

## Change history

| Date | Version | Author | Notes |
|------|---------|--------|-------|
| 2026-06-21 | v1.0 | Forge (v16 T2 L9) | Initial publication; 8 sections + checklist + references |