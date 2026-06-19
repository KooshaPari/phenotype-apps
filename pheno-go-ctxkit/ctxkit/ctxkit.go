// Package ctxkit provides the canonical context utilities used by every
// HTTP service in the Pheno Go fleet.
//
// ctxkit standardises three values that every Pheno handler needs:
//
//  1. A per-request identifier (a UUID v4) — see [WithRequestID],
//     [RequestID], and [NewRequestID].
//  2. A *slog.Logger that is always reachable — see [WithLogger] and
//     [Logger] (with a [slog.Default] fallback).
//  3. A small net/http middleware that wires the two together and emits
//     a structured "request.complete" log line on the way out — see
//     [Middleware].
//
// The package depends only on the Go standard library. In particular,
// [NewRequestID] generates a UUID v4 from [crypto/rand] and
// [encoding/hex] and introduces no third-party dependency.
package ctxkit

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"log/slog"
	"net/http"
	"time"
)

// HeaderRequestID is the canonical HTTP header used to propagate a
// request identifier across process boundaries. Inbound requests that
// already carry this header keep their existing id; otherwise
// [Middleware] generates a fresh one.
const HeaderRequestID = "X-Request-ID"

// requestIDKey is the unexported context key under which a request id
// is stored. Using an unexported empty struct prevents collisions with
// keys defined in other packages.
type requestIDKey struct{}

// loggerKey is the unexported context key under which a *slog.Logger
// is stored. Using an unexported empty struct prevents collisions with
// keys defined in other packages.
type loggerKey struct{}

// WithRequestID returns a copy of ctx that carries the supplied request
// id.
//
// An empty id is treated as "no override" and ctx is returned unchanged
// so downstream code can call [RequestID] on the result without
// checking. This matches the convention used by the rest of the
// stdlib's context helpers.
func WithRequestID(ctx context.Context, id string) context.Context {
	if id == "" {
		return ctx
	}
	return context.WithValue(ctx, requestIDKey{}, id)
}

// RequestID extracts the request id stored in ctx by [WithRequestID] or
// [Middleware]. If no id is present (or ctx is nil) it returns the
// empty string.
func RequestID(ctx context.Context) string {
	if ctx == nil {
		return ""
	}
	if v, ok := ctx.Value(requestIDKey{}).(string); ok {
		return v
	}
	return ""
}

// NewRequestID returns a freshly-generated RFC 4122 v4 UUID rendered as
// a 36-character hyphenated string, for example
// "f47ac10b-58cc-4372-a567-0e02b2c3d479".
//
// The implementation reads 16 bytes from [crypto/rand], sets the
// version (4) and variant (RFC 4122) nibbles per RFC 4122 §4.4 /
// §4.1.1, and formats the result with [encoding/hex]. No third-party
// dependency is introduced. If [crypto/rand.Read] ever fails (which
// should not happen on a healthy system), a time-derived fallback is
// used so the function always returns a non-empty, non-constant id.
func NewRequestID() string {
	var b [16]byte
	if _, err := rand.Read(b[:]); err != nil {
		// crypto/rand should never fail on a healthy system. Fall back
		// to a time-derived value so callers always receive a
		// non-empty, non-constant id; collisions are still vanishingly
		// unlikely.
		ts := time.Now().UTC().UnixNano()
		for i := 0; i < 8; i++ {
			b[i] = byte(ts >> (8 * i))
		}
		for i := 8; i < 16; i++ {
			b[i] = byte(ts >> (8 * (i - 8)))
		}
	}

	// RFC 4122 §4.4: set version to 4 (random) in the high nibble of
	// byte 6.
	b[6] = (b[6] & 0x0f) | 0x40
	// RFC 4122 §4.1.1: set variant to RFC 4122 in the high two bits of
	// byte 8.
	b[8] = (b[8] & 0x3f) | 0x80

	hexBuf := hex.EncodeToString(b[:])
	// 8-4-4-4-12 layout
	return fmt.Sprintf("%s-%s-%s-%s-%s",
		hexBuf[0:8], hexBuf[8:12], hexBuf[12:16], hexBuf[16:20], hexBuf[20:32])
}

// WithLogger returns a copy of ctx that carries the supplied logger.
//
// A nil logger is treated as "no override" and ctx is returned
// unchanged, mirroring the [WithRequestID] contract. Storing a nil
// logger in the context would cause [Logger] to fall back to
// [slog.Default] and is therefore never what callers want.
func WithLogger(ctx context.Context, logger *slog.Logger) context.Context {
	if logger == nil {
		return ctx
	}
	return context.WithValue(ctx, loggerKey{}, logger)
}

// Logger extracts the *slog.Logger stored in ctx by [WithLogger] or
// [Middleware]. If no logger is present (or the stored value is the
// wrong type, or ctx is nil) it returns [slog.Default] so that callers
// can log unconditionally without a nil check.
func Logger(ctx context.Context) *slog.Logger {
	if ctx != nil {
		if l, ok := ctx.Value(loggerKey{}).(*slog.Logger); ok && l != nil {
			return l
		}
	}
	return slog.Default()
}

// Background returns a [context.Background] pre-populated with a fresh
// request id. Use it as a shorthand for code paths that need a request
// id outside of an HTTP request (CLI entry points, queue workers,
// tests).
func Background() context.Context {
	return WithRequestID(context.Background(), NewRequestID())
}

// Middleware returns an [http.Handler] middleware that:
//
//  1. Extracts a request id from the X-Request-ID header, falling back
//     to any id already present on r.Context() and finally to a freshly
//     generated UUID v4 from [NewRequestID].
//  2. Stores the id in the request's context via [WithRequestID].
//  3. Wraps the inbound logger (or [slog.Default]) with a
//     "request_id" attribute and stores it in the context via
//     [WithLogger].
//  4. Echoes the id back on the response's X-Request-ID header so
//     downstream observability tooling can correlate log lines with
//     the HTTP transaction without needing access to the server log.
//  5. Defers a single [slog.Logger.LogAttrs] call at level Info with
//     the message "request.complete" and the attributes method, path,
//     status, and duration_ms.
//
// The middleware does not log request bodies and never panics on
// malformed input.
func Middleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()

		// 1. Resolve request id: header > pre-set context value > new.
		id := r.Header.Get(HeaderRequestID)
		if id == "" {
			id = RequestID(r.Context())
		}
		if id == "" {
			id = NewRequestID()
		}

		// 2. Build a child logger with the request_id attribute and
		//    stash both values in the request's context.
		baseLogger := Logger(r.Context())
		reqLogger := baseLogger.With(slog.String("request_id", id))

		ctx := WithLogger(WithRequestID(r.Context(), id), reqLogger)

		// 3. Wrap the ResponseWriter so we can observe status & bytes.
		rw := &statusRecorder{ResponseWriter: w, status: http.StatusOK}

		// 4. Echo the id on the response for cross-process correlation.
		w.Header().Set(HeaderRequestID, id)

		// 5 (deferred). Emit exactly one structured log line per
		//    request via slog.Logger.LogAttrs.
		defer func() {
			reqLogger.LogAttrs(ctx, slog.LevelInfo, "request.complete",
				slog.String("method", r.Method),
				slog.String("path", r.URL.Path),
				slog.Int("status", rw.status),
				slog.Int64("duration_ms", time.Since(start).Milliseconds()),
			)
		}()

		next.ServeHTTP(rw, r.WithContext(ctx))
	})
}

// statusRecorder is a minimal [http.ResponseWriter] decorator that
// captures the status code and the number of bytes written. Every
// other method is delegated to the wrapped writer so it is transparent
// to handlers.
type statusRecorder struct {
	http.ResponseWriter
	status      int
	bytes       int
	wroteHeader bool
}

func (s *statusRecorder) WriteHeader(code int) {
	if !s.wroteHeader {
		s.status = code
		s.wroteHeader = true
	}
	s.ResponseWriter.WriteHeader(code)
}

func (s *statusRecorder) Write(b []byte) (int, error) {
	if !s.wroteHeader {
		// Per net/http contract, Write triggers an implicit 200 if no
		// WriteHeader has been issued.
		s.status = http.StatusOK
		s.wroteHeader = true
	}
	n, err := s.ResponseWriter.Write(b)
	s.bytes += n
	return n, err
}
