# pheno-go-ctxkit

> W3C trace-context propagation for Go services, in the Phenotype monorepo.

## What
Lightweight Go package that parses and emits W3C trace-context (`traceparent`, `tracestate`) HTTP headers, with first-class interop with `pheno-context` (Rust) and `pheno-context-py` (Python).

## When
Use it when you have Go services that need to participate in a distributed trace across the fleet (e.g., a Go MCP server calls a Rust substrate service, and the trace must propagate).

## When **not**
Don't use it for trace *collection* — that's `pheno-otel` (OTLP pipeline). This is propagation only.

## 5-line quickstart
```go
import "github.com/KooshaPari/pheno-go-ctxkit/ctxkit"

func main() {
    headers := http.Header{}
    ctxkit.Inject(r.Context(), headers)        // outbound
    parent, _ := ctxkit.Extract(headers)       // inbound
    _ = parent
}
```

## Tier
0 — meta-bundle (SPEC, README, AGENTS, llms, CHANGELOG, LICENSE) required by ADR-023.

## License
MIT OR Apache-2.0 (dual).
