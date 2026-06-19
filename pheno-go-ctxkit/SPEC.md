# SPEC — pheno-go-ctxkit

## Public API

### `Inject`
```go
func Inject(ctx context.Context, h http.Header) error
```
Writes the current trace's `traceparent` (and `tracestate` if present) into the given `http.Header` map. Returns an error if the context has no traceparent (i.e., no parent span).

### `Extract`
```go
func Extract(h http.Header) (context.Context, error)
```
Reads `traceparent` from the given headers and returns a new `context.Context` with the parent span attached. Returns the original context (unchanged) if no `traceparent` is present.

### `WithTraceparent`
```go
func WithTraceparent(ctx context.Context, tp string) context.Context
```
Forces a specific `traceparent` value on the context. Used in tests and for synthetic-span scenarios.

### `TraceparentFromContext`
```go
func TraceparentFromContext(ctx context.Context) string
```
Returns the `traceparent` string for the context, or empty string if none.

## W3C trace-context
Implements [W3C Trace Context Level 2](https://www.w3.org/TR/trace-context/):
- `traceparent`: `00-{trace_id:32hex}-{span_id:16hex}-{flags:2hex}`
- `tracestate`: vendor-specific, passed through verbatim

## Interop
- **pheno-context (Rust):** byte-identical `traceparent` format. Verified by golden tests in both repos.
- **pheno-context-py (Python):** same format; tests round-trip a span end-to-end.

## Error model
- `Inject` returns `ErrNoTraceparent` if the context has no parent.
- `Extract` returns `ErrInvalidTraceparent` if the header is malformed (does not drop the context; caller decides).
- All other errors are `ErrInternal` wrapping the underlying cause.

## Versioning
- `v0.1.0` — initial release (2026-06-18)
- Backward-compatible within `0.x` per Go semver

## License
MIT OR Apache-2.0 (dual).
