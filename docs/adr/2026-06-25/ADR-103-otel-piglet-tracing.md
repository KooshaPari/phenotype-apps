# ADR-103: OpenTelemetry tracing strategy for piglet gRPC services

**Status:** PROPOSED
**Date:** 2026-06-25
**Author:** orchestrator (forge subagent)
**L5-201** (v28 track T3, pillar L60 Cycle 18)
**Refs:**
- ADR-036 (pheno-tracing substrate canonical, tracing substrate for Rust services)
- ADR-088 (grafonnet dashboard pipeline, downstream consumer of OTLP traces)
- `findings/otel-piglet-tracing-options.md` (this ADR's options comparison)
- `plans/2026-06-25-v28-dag-stable.md` § 3.3 T3
- 71-pillar L60 (distributed tracing coverage for gRPC services)

---

## Context

Piglet is a gRPC-based service mesh node within the Phenotype federated architecture.
It handles inter-service RPC calls across substrate boundaries (Rust ↔ Go ↔ Python ↔
TypeScript). Currently, piglet services emit unstructured logs but lack distributed
tracing context propagation and OTLP export.

The 71-pillar audit (Cycle 18, pillar L60) requires that all gRPC services in the
fleet have **end-to-end distributed tracing** with:

- OTLP-compliant trace export to the centralized observability backend
- W3C TraceContext propagation across service boundaries
- Span annotations for gRPC method, status code, latency, and error metadata
- Integration with the existing pheno-tracing substrate (ADR-036) where the service
  is implemented in Rust

Three candidate approaches exist. Each is evaluated in the companion finding
(`findings/otel-piglet-tracing-options.md`). This ADR records the architectural
decision and the rationale for the chosen approach.

## Decision

**Piglet gRPC services adopt the OpenTelemetry Operator auto-instrumentation approach
(pattern A) as the primary strategy, with fallback to piglet's built-in OTLP SDK
integration (pattern B) for services that require custom span attributes beyond what
auto-instrumentation provides.**

### Binding scope

| Layer | Binding | Rationale |
|---|---|---|
| Rust gRPC services | Auto-instrumentation via `tracing-opentelemetry` + `tonic` interceptor | Integrates with `pheno-tracing` (ADR-036) substrate init, automatic span creation on each gRPC call |
| Go gRPC services | OpenTelemetry-Go SDK `grpc.Interceptor` | Native `otelgrpc` package provides client/server interceptors with TraceContext propagation |
| Python gRPC services | OpenTelemetry Python SDK `grpc.aio_interceptor` | Python's `opentelemetry-instrumentation-grpc` auto-wraps `grpc.aio` server/client |
| TypeScript gRPC services | `@opentelemetry/instrumentation-grpc` package | Auto-loads via Node.js `--require` instrumentation hook |
| Sidecar collector (Envoy) | OTLP exporter + batch processor | All services export to a local OTel Collector sidecar that buffers, batches, and retries to the central observability backend |

### Rationale

1. **Auto-instrumentation** provides consistent span creation across polyglot services
   without per-service code changes — critical for the 20+ piglet gRPC endpoints.
2. **Envoy sidecar** decouples export reliability from application code:
   retries, backpressure, and batching are handled by the collector, not the service.
3. The **dual-path** (A primary, B fallback) ensures teams can add custom attributes
   (e.g., tenant-id, workflow-step) where auto-instrumentation is insufficient, without
   forking the standard approach.

### Approach A details (primary — auto-instrumentation)

```
piglet-service
  └─ gRPC handler (tonic / grpc-go / grpc-py / grpc-ts)
       └─ auto-instrumentation interceptor
            └─ OTLP export (gRPC or HTTP)
                 └─ OTel Collector sidecar (127.0.0.1:4317)
                      └─ batch export
                           └─ Central observability backend
```

### Approach B details (fallback — SDK integration)

For services that need custom span attributes:

```rust
// Rust example — integrates with pheno-tracing init (ADR-036)
use pheno_tracing::init_otlp;
use opentelemetry::global;
use tonic::{Request, Status};

fn piglet_trace_interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    let span = global::tracer("piglet-grpc")
        .start("piglet.handle_request");
    span.set_attribute("piglet.tenant_id", extract_tenant(&req));
    req.extensions_mut().insert(span.context());
    Ok(req)
}
```

## Consequences

### Positive

- **Polyglot consistency:** All four language runtimes follow the same interceptor-based
  auto-instrumentation pattern. Trace context propagates correctly across service
  boundaries regardless of implementation language.
- **Zero-code span creation:** Teams adopt tracing by deploying the OTel Collector sidecar
  and registering the auto-instrumentation package; no gRPC handler changes needed.
- **Observability backend decoupling:** The sidecar handles retries and backpressure,
  preventing tracing backpressure from affecting request latency.
- **Integration with ADR-036:** Rust services on pheno-tracing get OTLP export for free
  via the `tracing-opentelemetry` bridge; no separate init needed.

### Negative

- **Sidecar resource overhead:** Each piglet pod runs an OTel Collector sidecar container
  (approximately 128 MiB RAM, 0.1 CPU per instance). Estimated fleet-wide overhead:
  2.5 GiB RAM total across 20 replicas.
- **Custom attribute gap:** Services that need per-request custom attributes (tenant,
  workflow-id, user-id) must implement approach B for those specific spans. The project
  should maintain a shared interceptor library to avoid per-team replication.

## Compliance

1. All new piglet gRPC services SHALL deploy an OTel Collector sidecar as of
   2026-07-07 (Cycle 18 end).
2. The sidecar SHALL listen on `127.0.0.1:4317` (gRPC) with a batch export interval
   of 5 seconds and a maximum export batch size of 512 spans.
3. Each service language runtime SHALL register the corresponding auto-instrumentation
   package (see binding scope table).
4. Services that need custom span attributes SHALL use the fallback SDK integration
   (approach B) and SHALL place shared attribute logic in `/observability/piglet-tracing-core`.
5. Compliance is verified via the 71-pillar audit cycle: pillar L60 score must reach
   3/3 (full) by Cycle 18 close.

## Cross-references

- ADR-036 (pheno-tracing substrate canonical — tracing init for Rust services)
- ADR-088 (grafonnet dashboard pipeline — downstream trace consumer via OTLP)
- `findings/otel-piglet-tracing-options.md` — options comparison and scoring matrix
- `plans/2026-06-25-v28-dag-stable.md` § 3.3 T3
- `docs/architecture/OTEL-OPERATOR.md` — operator version pin and configuration
