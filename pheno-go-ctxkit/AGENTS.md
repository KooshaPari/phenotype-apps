# pheno-go-ctxkit

## Project type
Go module (single package: `ctxkit`).

## Public surface
```go
package ctxkit

func Inject(ctx context.Context, h http.Header) error
func Extract(h http.Header) (context.Context, error)
func WithTraceparent(ctx context.Context, tp string) context.Context
func TraceparentFromContext(ctx context.Context) string
```

## Conventions
- Go 1.22+
- `gofmt` strict (CI gates)
- `go vet` strict (CI gates)
- 80% test coverage required (per ADR-040 Tier 0 gate)
- No external deps — stdlib only (per ADR-023 Rule 3.1 substrate quality)
- `internal/` allowed for test helpers; everything else is `package ctxkit`

## Build/test
```bash
go build ./...
go test ./...
go test -cover ./...
```

## CI
- `test` job: go test on ubuntu + macos
- `lint` job: gofmt -l + go vet
- `coverage` job: codecov upload

## Related
- `pheno-context` (Rust) — same W3C trace-context contract
- `pheno-otel` — OTLP collector (downstream of ctxkit propagation)
- ADR-023, ADR-036, ADR-040

## License
MIT OR Apache-2.0 (dual).
