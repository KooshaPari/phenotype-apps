# pheno-mcp-router

Polyglot Model Context Protocol (MCP) router substrate. Routes LLM tool calls and
prompt traffic across heterogeneous model adapters with a single canonical
contract.

> **Substrate role:** `phenotype-*-sdk` (ADR-023). Cross-language routing substrate
> for `pheno-mcp-router` and any downstream consumer that needs MCP-style
> request dispatch.

## REST API examples

This service follows the fleet-wide REST conventions at
[`docs/conventions/rest-api.md`](../../docs/conventions/rest-api.md)
(v16 T2 / L9). The examples below show the canonical surface; OpenAPI 3.1 is
generated from these contracts at build time.

### 1. List routes (cursor pagination + Bearer auth)

```bash
curl -sS https://api.pheno-mcp-router.dev/v1/routes?limit=50 \
  -H "Authorization: Bearer ${PHENO_MCP_ROUTER_TOKEN}" \
  -H "Accept: application/json"
```

`200 OK`:

```json
{
  "data": [
    { "id": "route_openai_gpt4o",   "adapter": "openai",   "model": "gpt-4o",   "enabled": true },
    { "id": "route_llama3_local",   "adapter": "llama",    "model": "llama3",   "enabled": true },
    { "id": "route_anthropic_opus", "adapter": "anthropic","model": "claude-opus-4", "enabled": false }
  ],
  "next_cursor": "eyJpZCI6InJvdXRlX2FudGhyb3BpY19vcHVzIn0=",
  "has_more": true
}
```

### 2. Make a routing decision (Idempotency-Key + 201 Created)

```bash
curl -sS -X POST https://api.pheno-mcp-router.dev/v1/decisions \
  -H "Authorization: Bearer ${PHENO_MCP_ROUTER_TOKEN}" \
  -H "Content-Type: application/json" \
  -H "Idempotency-Key: 01HXYZABCDEF1234567890ABCD" \
  -d '{
    "prompt": "Summarize the architecture of this monorepo.",
    "constraints": { "max_tokens": 1024, "prefer_local": true }
  }'
```

`201 Created`:

```json
{
  "id": "01HXYZDECTEST00000000000000",
  "selected_route": "route_llama3_local",
  "reason": "prefer_local=true matched a local adapter with sufficient context window",
  "location": "https://api.pheno-mcp-router.dev/v1/decisions/01HXYZDECTEST00000000000000"
}
```

A retry of the same request with the same `Idempotency-Key` and identical body
returns the stored `201 Created` response without re-executing the dispatch.

### 3. Invoke a tool (RFC 7807 problem+json on missing scope)

```bash
curl -sS -X POST https://api.pheno-mcp-router.dev/v1/tools/filesystem.read/invocations \
  -H "Authorization: Bearer ${PHENO_MCP_ROUTER_TOKEN}" \
  -H "Content-Type: application/json" \
  -d '{ "path": "/var/data/secrets/api.key" }'
```

`403 Forbidden` with the canonical `application/problem+json` envelope:

```json
{
  "type": "https://api.phenotype.dev/probs/auth-insufficient-scope",
  "title": "Authenticated principal lacks the required scope.",
  "detail": "Scope 'files:read:secrets' is not granted to sub 'svc-9421'.",
  "instance": "/v1/tools/filesystem.read/invocations",
  "status": 403,
  "code": "AUTH_INSUFFICIENT_SCOPE",
  "trace_id": "01HXYZRATE01ABCDEF0123456789"
}
```

### 4. Rate-limit envelope (`Retry-After` required on 429)

```bash
curl -sS -i https://api.pheno-mcp-router.dev/v1/routes?limit=200 \
  -H "Authorization: Bearer ${PHENO_MCP_ROUTER_TOKEN}"
```

`429 Too Many Requests`:

```http
HTTP/1.1 429 Too Many Requests
Content-Type: application/problem+json
RateLimit-Limit: 1000
RateLimit-Remaining: 0
RateLimit-Reset: 27
Retry-After: 27
```

```json
{
  "type": "https://api.phenotype.dev/probs/rate-limit-exceeded",
  "title": "Rate limit exceeded.",
  "detail": "You have used 1000 of 1000 requests in the last 60 seconds.",
  "instance": "/v1/routes",
  "status": 429,
  "code": "RATE_LIMIT_EXCEEDED",
  "trace_id": "01HXYZRATE02ABCDEF0123456789",
  "retry_after_seconds": 27
}
```

## See also

- Fleet convention: `docs/conventions/rest-api.md` (canonical)
- Adapter contract: `docs/contracts/lm-port.md` (per ADR-013, ADR-014)
- Federation auth: ADR-046 (mTLS + OIDC for service-to-service)