package ctxkit

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"io"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"regexp"
	"strings"
	"sync"
	"testing"
)

// uuidV4Pattern matches the canonical 8-4-4-4-12 hex form and verifies
// the version (4) and variant (8/9/a/b) nibbles. Anything that parses
// against this regex is, by construction, an RFC 4122 v4 UUID.
var uuidV4Pattern = regexp.MustCompile(`^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$`)

// TestNewRequestIDIsUnique asserts that successive calls to
// [NewRequestID] produce values that (a) are syntactically valid UUID
// v4 strings and (b) never collide. Running the loop 256 times gives
// a comfortable margin for catching accidental use of math/rand or a
// fixed seed.
func TestNewRequestIDIsUnique(t *testing.T) {
	const n = 256
	seen := make(map[string]struct{}, n)
	for i := 0; i < n; i++ {
		id := NewRequestID()
		if !uuidV4Pattern.MatchString(id) {
			t.Fatalf("NewRequestID() = %q, does not match UUID v4 shape", id)
		}
		if _, dup := seen[id]; dup {
			t.Fatalf("NewRequestID() produced duplicate id %q at iteration %d", id, i)
		}
		seen[id] = struct{}{}
	}
}

// TestWithRequestIDRoundTrip covers the [WithRequestID] / [RequestID]
// pair, the "empty id is a no-op" contract, the "missing key returns
// empty string" contract, and the nil-context guard.
func TestWithRequestIDRoundTrip(t *testing.T) {
	t.Run("stores_and_retrieves", func(t *testing.T) {
		ctx := WithRequestID(context.Background(), "abc-123")
		if got := RequestID(ctx); got != "abc-123" {
			t.Fatalf("RequestID = %q, want %q", got, "abc-123")
		}
	})

	t.Run("empty_id_is_a_noop", func(t *testing.T) {
		ctx := WithRequestID(context.Background(), "seed")
		if got := RequestID(WithRequestID(ctx, "")); got != "seed" {
			t.Fatalf("empty WithRequestID clobbered existing id: got %q, want %q", got, "seed")
		}
	})

	t.Run("missing_key_returns_empty_string", func(t *testing.T) {
		if got := RequestID(context.Background()); got != "" {
			t.Fatalf("RequestID on bare ctx = %q, want empty", got)
		}
	})

	t.Run("nil_ctx_returns_empty_string", func(t *testing.T) {
		//nolint:staticcheck // intentional nil-context test
		if got := RequestID(nil); got != "" {
			t.Fatalf("RequestID on nil ctx = %q, want empty", got)
		}
	})
}

// TestWithLoggerRoundTrip covers the [WithLogger] / [Logger] pair, the
// nil-guard, and the [slog.Default] fallback for contexts that don't
// carry a logger.
func TestWithLoggerRoundTrip(t *testing.T) {
	t.Run("stores_and_retrieves", func(t *testing.T) {
		buf := &bytes.Buffer{}
		custom := slog.New(slog.NewJSONHandler(buf, nil))
		ctx := WithLogger(context.Background(), custom)
		if got := Logger(ctx); got != custom {
			t.Fatalf("Logger = %p, want %p", got, custom)
		}
	})

	t.Run("nil_logger_is_a_noop", func(t *testing.T) {
		original := slog.New(slog.NewJSONHandler(io.Discard, nil))
		ctx := WithLogger(context.Background(), original)
		if got := Logger(WithLogger(ctx, nil)); got != original {
			t.Fatalf("nil WithLogger clobbered existing logger: got %p, want %p", got, original)
		}
	})

	t.Run("missing_logger_falls_back_to_slog_default", func(t *testing.T) {
		if got := Logger(context.Background()); got != slog.Default() {
			t.Fatalf("Logger on bare ctx = %p, want slog.Default() = %p", got, slog.Default())
		}
	})

	t.Run("nil_ctx_falls_back_to_slog_default", func(t *testing.T) {
		//nolint:staticcheck // intentional nil-context test
		if got := Logger(nil); got != slog.Default() {
			t.Fatalf("Logger on nil ctx = %p, want slog.Default() = %p", got, slog.Default())
		}
	})
}

// TestBackgroundReturnsContextWithRequestID confirms that
// [Background] pre-populates the context with a non-empty,
// UUID-v4-shaped id and that the context is fresh (no inherited
// cancellation).
func TestBackgroundReturnsContextWithRequestID(t *testing.T) {
	ctx := Background()
	id := RequestID(ctx)
	if !uuidV4Pattern.MatchString(id) {
		t.Fatalf("Background().RequestID = %q, does not match UUID v4 shape", id)
	}
	if err := ctx.Err(); err != nil && !errors.Is(err, context.Canceled) {
		t.Fatalf("Background() returned a non-fresh context: err = %v", err)
	}
}

// TestMiddlewareInjectsRequestID checks that the middleware (a) reads
// X-Request-ID from the inbound request when present, (b) generates a
// UUID v4 when it is not, and (c) exposes the id to the downstream
// handler via the request's context, (d) echoes the id back on the
// response.
func TestMiddlewareInjectsRequestID(t *testing.T) {
	t.Run("honours_inbound_X_Request_ID", func(t *testing.T) {
		const want = "client-supplied-42"
		var seen string
		handler := Middleware(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			seen = RequestID(r.Context())
			w.WriteHeader(http.StatusNoContent)
		}))

		req := httptest.NewRequest(http.MethodGet, "/ping", nil)
		req.Header.Set(HeaderRequestID, want)
		rr := httptest.NewRecorder()
		handler.ServeHTTP(rr, req)

		if seen != want {
			t.Fatalf("handler saw request_id = %q, want %q", seen, want)
		}
		if got := rr.Header().Get(HeaderRequestID); got != want {
			t.Fatalf("response %s = %q, want %q", HeaderRequestID, got, want)
		}
	})

	t.Run("generates_UUID_v4_when_header_missing", func(t *testing.T) {
		var seen string
		handler := Middleware(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
			seen = RequestID(r.Context())
			w.WriteHeader(http.StatusOK)
		}))

		req := httptest.NewRequest(http.MethodGet, "/ping", nil)
		rr := httptest.NewRecorder()
		handler.ServeHTTP(rr, req)

		if !uuidV4Pattern.MatchString(seen) {
			t.Fatalf("generated request_id %q is not a UUID v4", seen)
		}
		if got := rr.Header().Get(HeaderRequestID); got != seen {
			t.Fatalf("response %s = %q, want %q (handler context value)", HeaderRequestID, got, seen)
		}
	})
}

// TestMiddlewareEmitsRequestCompleteLog asserts that exactly one
// "request.complete" log record is emitted per request and that it
// carries the spec'd attributes: method, path, status, duration_ms.
// The request_id on the log line must also match the value visible to
// the downstream handler (proves the request-scoped logger is what the
// middleware uses).
func TestMiddlewareEmitsRequestCompleteLog(t *testing.T) {
	var buf safeBuffer
	logger := slog.New(slog.NewJSONHandler(&buf, &slog.HandlerOptions{Level: slog.LevelDebug}))

	// Pre-seed slog.Default so the middleware's fallback path also
	// exercises the test logger when the handler does not touch it.
	prev := slog.Default()
	slog.SetDefault(logger)
	t.Cleanup(func() { slog.SetDefault(prev) })

	handler := Middleware(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Emit a child log to prove the request-scoped logger is
		// reachable from the downstream handler.
		Logger(r.Context()).Info("handler.tick", slog.String("path", r.URL.Path))
		w.WriteHeader(http.StatusTeapot) // 418 — must precede Write.
		_, _ = w.Write([]byte("hello"))
	}))

	req := httptest.NewRequest(http.MethodPost, "/widgets", strings.NewReader(""))
	req.Header.Set(HeaderRequestID, "log-test-id")
	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)

	// Decode one JSON object per line.
	raw := buf.Bytes()

	var records []map[string]any
	for _, line := range bytes.Split(bytes.TrimRight(raw, "\n"), []byte("\n")) {
		if len(line) == 0 {
			continue
		}
		var rec map[string]any
		if err := json.Unmarshal(line, &rec); err != nil {
			t.Fatalf("log line is not JSON: %q (%v)", line, err)
		}
		records = append(records, rec)
	}

	var complete map[string]any
	for _, rec := range records {
		if rec["msg"] == "request.complete" {
			if complete != nil {
				t.Fatalf("more than one request.complete record emitted: %+v", records)
			}
			complete = rec
		}
	}
	if complete == nil {
		t.Fatalf("no request.complete record found in: %+v", records)
	}

	if got := complete["method"]; got != "POST" {
		t.Errorf("method = %v, want POST", got)
	}
	if got := complete["path"]; got != "/widgets" {
		t.Errorf("path = %v, want /widgets", got)
	}
	// JSON numbers decode to float64.
	if got, _ := complete["status"].(float64); int(got) != http.StatusTeapot {
		t.Errorf("status = %v, want 418", complete["status"])
	}
	if _, ok := complete["duration_ms"]; !ok {
		t.Errorf("duration_ms missing from record: %+v", complete)
	}

	// The handler-tick record must also carry the request_id attribute
	// so downstream log lines stay correlated.
	var tick map[string]any
	for _, rec := range records {
		if rec["msg"] == "handler.tick" {
			tick = rec
			break
		}
	}
	if tick == nil {
		t.Fatalf("handler.tick record missing; middleware did not expose scoped logger")
	}
	if got := tick["request_id"]; got != "log-test-id" {
		t.Errorf("handler.tick request_id = %v, want log-test-id", got)
	}
}

// safeBuffer is a [bytes.Buffer] with an embedded mutex so the slog
// JSON handler stays race-free under `go test -race`.
type safeBuffer struct {
	mu  sync.Mutex
	buf bytes.Buffer
}

func (s *safeBuffer) Write(p []byte) (int, error) {
	s.mu.Lock()
	defer s.mu.Unlock()
	return s.buf.Write(p)
}

func (s *safeBuffer) Bytes() []byte {
	s.mu.Lock()
	defer s.mu.Unlock()
	// Return a copy so callers can read without holding the lock.
	out := make([]byte, s.buf.Len())
	copy(out, s.buf.Bytes())
	return out
}
