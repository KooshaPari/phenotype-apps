# Findings: OTel-piglet tracing options comparison

**Date:** 2026-06-25
**Author:** forge subagent (L5-201)
**Context:** v28 T3 — choose tracing approach for piglet gRPC services (pillar L60, Cycle 18)
**Cross-ref:** ADR-103 (decision record), ADR-036 (pheno-tracing substrate)

---

## Candidate approaches

### A. OpenTelemetry Operator auto-instrumentation

Uses the `opentelemetry-operator` mutating webhook to inject instrumentation
into pods at deployment time. The operator patches pods with a pre-configured
OpenTelemetry SDK sidecar (or init container) that auto-wraps supported libraries
(gRPC, HTTP, etc.) via `OTEL_INSTRUMENTATION_*` environment variables.

**Pros:**
- Zero application code changes — injection happens at the Kubernetes level
- Consistent across all languages (operator handles Java, Python, Node.js, Go, .NET)
- Automatic inject version management (operator upgrades SDKs across the fleet)
- W3C TraceContext propagation built-in

**Cons:**
- Limited custom attribute support — cannot add domain-specific span attributes
  (tenant-id, workflow-step) without custom SDK configuration
- Operator must be installed and configured (cluster-level RBAC)
- Not all gRPC frameworks are covered by auto-instrumentation (e.g., custom `tonic`
  interceptors may need manual wiring)
- Tight coupling to Kubernetes — doesn't work for bare-metal piglet deployments

**Scoring (1-5):** Consistency: 5, Effort: 5, Flexibility: 2, Portability: 2,
Observability: 4 → **Weighted: 3.6**

### B. Piglet's built-in OTel SDK integration

Each piglet service imports its language's OpenTelemetry SDK directly and registers
gRPC interceptors in the application startup code. Span creation, context propagation,
and OTLP export are handled in-process. Rust services use `tracing-opentelemetry`
bridged through `pheno-tracing` init (per ADR-036).

**Pros:**
- Full control over span attributes — add tenant-id, workflow-step, user-id per request
- No Kubernetes dependency — works in any deployment environment
- Familiar to teams already using pheno-tracing (Rust) or `opentelemetry-api` (Go/Python/TS)
- Compatible with custom gRPC interceptors (tonic, grpc-go, etc.)

**Cons:**
- Per-service code change required — each of 20+ endpoints needs interceptor registration
- Inconsistent implementation across languages unless strict conventions are enforced
- Export backpressure in-process can affect request latency
- Teams must manage OTLP exporter config themselves (endpoint, batch size, retry)

**Scoring (1-5):** Consistency: 2, Effort: 2, Flexibility: 5, Portability: 5,
Observability: 3 → **Weighted: 3.4**

### C. Sidecar collector pattern (Envoy / OTel Collector)

Each piglet pod runs a dedicated OTel Collector (or Envoy) sidecar that receives OTLP
data from the application over localhost. The sidecar buffers, batches, retries, and
exports to the central observability backend. The application itself can use either
auto-instrumentation or SDK integration for span creation, but the export path is
always through the sidecar.

**Pros:**
- Export reliability decoupled from application — retries, backpressure, and
  credential management live in the sidecar, not the service
- Centralized export config — update one collector config instead of 20+ services
- Supports tail-based sampling for high-volume services
- Can run as a DaemonSet for node-level sharing (resource-efficient)

**Cons:**
- Additional container per pod (or per node) — resource overhead
- Adds network hop (localhost gRPC) even when export is the only concern
- Collector config is another thing to manage and version
- Adds 50-150ms of tail latency when collector buffers are full

**Scoring (1-5):** Consistency: 4, Effort: 4, Flexibility: 3, Portability: 3,
Observability: 5 → **Weighted: 3.8**

---

## Scoring matrix

| Criterion | Weight | A. Auto-instr | B. SDK native | C. Sidecar |
|-----------|--------|---------------|---------------|-------------|
| Polyglot consistency | 25% | 5 (1.25) | 2 (0.50) | 4 (1.00) |
| Adoption effort | 20% | 5 (1.00) | 2 (0.40) | 4 (0.80) |
| Custom attribute flexibility | 20% | 2 (0.40) | 5 (1.00) | 3 (0.60) |
| Deployment portability | 15% | 2 (0.30) | 5 (0.75) | 3 (0.45) |
| Export reliability | 20% | 4 (0.80) | 3 (0.60) | 5 (1.00) |
| **Weighted total** | **100%** | **3.75** | **3.25** | **3.85** |

---

## Recommendation

**Adopt C + A as a composite strategy: auto-instrumentation for span creation
(A) + sidecar collector for export (C).** This combination scores highest (3.85)
while providing the zero-code span creation of A and the export isolation of C.

Fall back to B (SDK integration) only for the subset of services that require
custom span attributes beyond what auto-instrumentation provides. See ADR-103
for the full decision and compliance timeline.

---

## References

- ADR-103: `docs/adr/2026-06-25/ADR-103-otel-piglet-tracing.md`
- ADR-036: `docs/adr/2026-06-18/ADR-036-pheno-tracing-substrate-canonical.md`
- ADR-088: grafonnet dashboard pipeline (downstream trace consumer)
- `plans/2026-06-25-v28-dag-stable.md` § 3.3 T3
- OpenTelemetry Operator docs: https://opentelemetry.io/docs/kubernetes/operator/
- OpenTelemetry Collector: https://opentelemetry.io/docs/collector/
