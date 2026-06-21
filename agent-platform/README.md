# agent-platform

Federated runtime for long-lived AI agents — session lifecycle, tool-call
arbitration, and telemetry aggregation across the Phenotype fleet.

> **Substrate role:** `phenotype-*-framework` (ADR-023). Federated service that
> hosts agent workloads and exposes a stable REST contract for orchestrators.

## REST API examples

This service follows the fleet-wide REST conventions at
[`docs/conventions/rest-api.md`](../../docs/conventions/rest-api.md)
(v16 T2 / L9). The examples below show the canonical surface; OpenAPI 3.1 is
generated from these contracts at build time.

### 1. List agent sessions (cursor pagination + Bearer auth)

```bash
curl -sS https://api.agent-platform.dev/v1/sessions?limit=50 \
  -H "Authorization: Bearer ${AGENT_PLATFORM_TOKEN}" \
  -H "Accept: application/json"
```

`200 OK`:

```json
{
  "data": [
    { "id": "01HXYZSESS001ABCDEFGHIJKLMN0", "status": "running", "agent_type": "researcher", "created_at": "2026-06-21T18:42:00Z" },
    { "id": "01HXYZSESS002ABCDEFGHIJKLMN0", "status": "idle",    "agent_type": "coder",      "created_at": "2026-06-21T18:39:11Z" },
    { "id": "01HXYZSESS003ABCDEFGHIJKLMN0", "status": "stopped", "agent_type": "reviewer",   "created_at": "2026-06-21T18:30:02Z" }
  ],
  "next_cursor": "eyJpZCI6IjAxSFhZWlNFU1MwMDNBQkNERUZHSElKSktMTU4wIn0=",
  "has_more": true
}
```

### 2. Open a new session (Idempotency-Key + 201 Created)

```bash
curl -sS -X POST https://api.agent-platform.dev/v1/sessions \
  -H "Authorization: Bearer ${AGENT_PLATFORM_TOKEN}" \
  -H "Content-Type: application/json" \
  -H "Idempotency-Key: 01HXYZOPEN0000000000000000A" \
  -d '{
    "agent_type": "researcher",
    "system_prompt_ref": "prompts/researcher-v3",
    "tool_policy": { "allow": ["web.search", "files.read"] }
  }'
```

`201 Created`:

```http
HTTP/1.1 201 Created
Location: https://api.agent-platform.dev/v1/sessions/01HXYZOPENSESS0000000000000
Content-Type: application/json
```

```json
{
  "id": "01HXYZOPENSESS0000000000000",
  "status": "starting",
  "agent_type": "researcher",
  "created_at": "2026-06-21T18:55:14Z"
}
```

A retry of the same request with the same `Idempotency-Key` and identical body
returns the stored `201 Created` response without opening a second session.

### 3. Record a tool-call event (RFC 7807 problem+json on conflict)

```bash
curl -sS -X POST https://api.agent-platform.dev/v1/sessions/01HXYZOPENSESS0000000000000/events \
  -H "Authorization: Bearer ${AGENT_PLATFORM_TOKEN}" \
  -H "Content-Type: application/json" \
  -H "If-Match: \"01HXYZOPENSESS0000000000000:r3\"" \
  -d '{
    "type": "tool.call",
    "tool_name": "web.search",
    "args": { "query": "RFC 7807 problem+json" },
    "occurred_at": "2026-06-21T18:56:02Z"
  }'
```

`409 Conflict` with the canonical `application/problem+json` envelope
(optimistic-concurrency mismatch on `If-Match`):

```json
{
  "type": "https://api.phenotype.dev/probs/optimistic-concurrency-conflict",
  "title": "Session version does not match the supplied If-Match.",
  "detail": "Current version is r4; supplied If-Match was r3. Re-read the session and retry.",
  "instance": "/v1/sessions/01HXYZOPENSESS0000000000000/events",
  "status": 409,
  "code": "OPTIMISTIC_CONCURRENCY_CONFLICT",
  "trace_id": "01HXYZCONFLICTABCDEF0123456789"
}
```

### 4. Rate-limit envelope (`Retry-After` required on 429)

```bash
curl -sS -i https://api.agent-platform.dev/v1/sessions?limit=200 \
  -H "Authorization: Bearer ${AGENT_PLATFORM_TOKEN}"
```

`429 Too Many Requests`:

```http
HTTP/1.1 429 Too Many Requests
Content-Type: application/problem+json
RateLimit-Limit: 1000
RateLimit-Remaining: 0
RateLimit-Reset: 18
Retry-After: 18
```

```json
{
  "type": "https://api.phenotype.dev/probs/rate-limit-exceeded",
  "title": "Rate limit exceeded.",
  "detail": "You have used 1000 of 1000 requests in the last 60 seconds.",
  "instance": "/v1/sessions",
  "status": 429,
  "code": "RATE_LIMIT_EXCEEDED",
  "trace_id": "01HXYZRATEAGENTPLATFORMABCDEF012",
  "retry_after_seconds": 18
}
```

## See also

- Fleet convention: `docs/conventions/rest-api.md` (canonical)
- Session protocol: `docs/protocols/session-lifecycle.md`
- Federation auth: ADR-046 (mTLS + OIDC for service-to-service)