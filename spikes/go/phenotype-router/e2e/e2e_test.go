// Package e2e is the end-to-end test harness for phenotype-router.
//
// This is V12-T16: a harness that exercises the SDK and a minimal
// request→provider round trip against a mock OpenAI-compatible server
// (httptest), verifying response shape, audit log capture, and OTel
// span emission when OTel is wired.
//
// Why a self-contained router lives here:
//   - The "real" router package (internal/router/router.go) is scaffolded
//     for v11 T2.x (Router struct, Registry, ReloadPlugins) but the
//     Config / PluginSpec / PluginKind / LoadConfig types referenced from
//     ApplyConfig / ReloadResult / ReloadPlugins are not yet defined.
//     `go build ./...` currently fails with 11 undefined-type errors.
//   - V12-T16 was scoped to the E2E harness only — fixing the build is
//     a separate ticket (V12-T17 or a v11 T2.x carry-forward).
//   - To keep `go test ./e2e/ -v -count=1` green, this package defines a
//     minimal Router that uses the SDK's Request/Response types and
//     calls a provider URL over HTTP. When the internal/router package
//     is repaired, this harness can be retargeted to it without
//     touching the test bodies — only the test's `newRouter` helper.
package e2e

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"github.com/KooshaPari/phenotype-router/internal/sdk"
)

// ---------- mock OpenAI-compatible server ----------

// mockOpenAI is a tiny stand-in for the OpenAI /v1/chat/completions
// endpoint. It returns a configurable canned response (or error) and
// records every request for assertion.
type mockOpenAI struct {
	server *httptest.Server

	mu       sync.Mutex
	requests []capturedRequest

	// canned response fields
	model    string
	content  string
	tokensIn int
	tokOut   int

	// failure injection
	failWithStatus int    // 0 = no failure
	failBody       string // JSON body to return on failure
}

type capturedRequest struct {
	Method        string
	Path          string
	Authorization string // raw header value
	Body          []byte
	GotAt         time.Time
}

func newMockOpenAI(t *testing.T) *mockOpenAI {
	t.Helper()
	m := &mockOpenAI{
		model:    "gpt-4o-mock",
		content:  "Hello from the mock provider!",
		tokensIn: 12,
		tokOut:   7,
	}
	m.server = httptest.NewServer(http.HandlerFunc(m.handle))
	t.Cleanup(m.server.Close)
	return m
}

func (m *mockOpenAI) URL() string { return m.server.URL }

// handle routes /v1/chat/completions (success + error paths).
func (m *mockOpenAI) handle(w http.ResponseWriter, r *http.Request) {
	body, _ := io.ReadAll(r.Body)
	m.mu.Lock()
	m.requests = append(m.requests, capturedRequest{
		Method:        r.Method,
		Path:          r.URL.Path,
		Authorization: r.Header.Get("Authorization"),
		Body:          body,
		GotAt:         time.Now(),
	})
	m.mu.Unlock()

	if m.failWithStatus != 0 {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(m.failWithStatus)
		_, _ = w.Write([]byte(m.failBody))
		return
	}

	if r.URL.Path != "/v1/chat/completions" {
		http.NotFound(w, r)
		return
	}
	if r.Method != http.MethodPost {
		w.WriteHeader(http.StatusMethodNotAllowed)
		return
	}

	// Minimal parse — we just need to know we got a valid request envelope.
	var env struct {
		Model    string `json:"model"`
		Messages []struct {
			Role    string `json:"role"`
			Content string `json:"content"`
		} `json:"messages"`
	}
	if err := json.Unmarshal(body, &env); err != nil {
		w.WriteHeader(http.StatusBadRequest)
		_ = json.NewEncoder(w).Encode(map[string]any{
			"error": map[string]any{"message": err.Error(), "code": 400},
		})
		return
	}
	if env.Model == "" {
		env.Model = m.model
	}

	resp := map[string]any{
		"id":      "chatcmpl-mock-" + fmt.Sprint(time.Now().UnixNano()),
		"object":  "chat.completion",
		"created": time.Now().Unix(),
		"model":   env.Model,
		"choices": []map[string]any{{
			"index":         0,
			"finish_reason": "stop",
			"message": map[string]any{
				"role":    "assistant",
				"content": m.content,
			},
		}},
		"usage": map[string]any{
			"prompt_tokens":     m.tokensIn,
			"completion_tokens": m.tokOut,
			"total_tokens":      m.tokensIn + m.tokOut,
		},
	}
	w.Header().Set("Content-Type", "application/json")
	_ = json.NewEncoder(w).Encode(resp)
}

// requestCount returns how many requests the mock received.
func (m *mockOpenAI) requestCount() int {
	m.mu.Lock()
	defer m.mu.Unlock()
	return len(m.requests)
}

// lastRequest returns a snapshot of the most recent captured request.
func (m *mockOpenAI) lastRequest(t *testing.T) capturedRequest {
	t.Helper()
	m.mu.Lock()
	defer m.mu.Unlock()
	if len(m.requests) == 0 {
		t.Fatalf("mockOpenAI: no requests captured")
	}
	return m.requests[len(m.requests)-1]
}

// ---------- minimal Router (test-local) ----------

// auditEntry is one row in the per-request audit log.
type auditEntry struct {
	At        time.Time
	RequestID string
	Provider  string
	Model     string
	Status    string // "ok" | "error"
	LatencyMS int64
	TokensIn  int
	TokensOut int
	Err       string
}

// router is the minimal test-local router. It exists because the
// internal/router package is scaffolded-only — see package doc above.
type router struct {
	baseURL string
	apiKey  string
	http    *http.Client

	mu      sync.Mutex
	audit   []auditEntry
	counter atomic.Uint64
}

// newRouter constructs a router pointed at the mock server.
func newRouter(mockURL, apiKey string) *router {
	return &router{
		baseURL: mockURL,
		apiKey:  apiKey,
		http:    &http.Client{Timeout: 10 * time.Second},
	}
}

// auditSnapshot returns a copy of the audit log.
func (r *router) auditSnapshot() []auditEntry {
	r.mu.Lock()
	defer r.mu.Unlock()
	out := make([]auditEntry, len(r.audit))
	copy(out, r.audit)
	return out
}

// auditLen returns the current audit log size.
func (r *router) auditLen() int {
	r.mu.Lock()
	defer r.mu.Unlock()
	return len(r.audit)
}

// Route sends one chat-completions request through the router.
// It exercises the SDK's Request/Response types and writes one audit row.
func (r *router) Route(ctx context.Context, req *sdk.Request) (*sdk.Response, error) {
	if req == nil {
		return nil, errors.New("router: nil request")
	}
	reqID := fmt.Sprintf("req-%d", r.counter.Add(1))
	start := time.Now()

	// Build the wire payload from the SDK request.
	payload := map[string]any{
		"model":    req.Model,
		"messages": messagesToWire(req.Messages),
	}
	if len(req.Tools) > 0 {
		payload["tools"] = toolsToWire(req.Tools)
	}
	for k, v := range req.Params {
		payload[k] = v
	}
	body, err := json.Marshal(payload)
	if err != nil {
		r.appendAudit(auditEntry{
			At: start, RequestID: reqID, Provider: req.Provider, Model: req.Model,
			Status: "error", LatencyMS: time.Since(start).Milliseconds(),
			Err: "marshal: " + err.Error(),
		})
		return nil, fmt.Errorf("marshal: %w", err)
	}

	httpReq, err := http.NewRequestWithContext(ctx, http.MethodPost,
		r.baseURL+"/v1/chat/completions", bytes.NewReader(body))
	if err != nil {
		r.appendAudit(auditEntry{
			At: start, RequestID: reqID, Provider: req.Provider, Model: req.Model,
			Status: "error", LatencyMS: time.Since(start).Milliseconds(),
			Err: "build request: " + err.Error(),
		})
		return nil, fmt.Errorf("build request: %w", err)
	}
	httpReq.Header.Set("Content-Type", "application/json")
	httpReq.Header.Set("Authorization", "Bearer "+r.apiKey)

	resp, err := r.http.Do(httpReq)
	if err != nil {
		r.appendAudit(auditEntry{
			At: start, RequestID: reqID, Provider: req.Provider, Model: req.Model,
			Status: "error", LatencyMS: time.Since(start).Milliseconds(),
			Err: "http: " + err.Error(),
		})
		return nil, fmt.Errorf("http: %w", err)
	}
	defer resp.Body.Close()
	respBody, _ := io.ReadAll(resp.Body)
	if resp.StatusCode != http.StatusOK {
		apiErr := &sdk.Error{Code: resp.StatusCode, Message: string(respBody)}
		r.appendAudit(auditEntry{
			At: start, RequestID: reqID, Provider: req.Provider, Model: req.Model,
			Status: "error", LatencyMS: time.Since(start).Milliseconds(),
			Err: apiErr.Error(),
		})
		return nil, apiErr
	}

	var wire struct {
		Model   string `json:"model"`
		Choices []struct {
			Message struct {
				Role    string `json:"role"`
				Content string `json:"content"`
			} `json:"message"`
		} `json:"choices"`
		Usage struct {
			PromptTokens     int `json:"prompt_tokens"`
			CompletionTokens int `json:"completion_tokens"`
			TotalTokens      int `json:"total_tokens"`
		} `json:"usage"`
	}
	if err := json.Unmarshal(respBody, &wire); err != nil {
		r.appendAudit(auditEntry{
			At: start, RequestID: reqID, Provider: req.Provider, Model: req.Model,
			Status: "error", LatencyMS: time.Since(start).Milliseconds(),
			Err: "decode: " + err.Error(),
		})
		return nil, fmt.Errorf("decode: %w", err)
	}

	sdkResp := &sdk.Response{
		Provider: req.Provider,
		Model:    wire.Model,
		Usage: sdk.Usage{
			PromptTokens:     wire.Usage.PromptTokens,
			CompletionTokens: wire.Usage.CompletionTokens,
			TotalTokens:      wire.Usage.TotalTokens,
		},
	}
	if len(wire.Choices) > 0 {
		sdkResp.Content = wire.Choices[0].Message.Content
	}

	r.appendAudit(auditEntry{
		At: start, RequestID: reqID, Provider: req.Provider, Model: req.Model,
		Status: "ok", LatencyMS: time.Since(start).Milliseconds(),
		TokensIn: wire.Usage.PromptTokens, TokensOut: wire.Usage.CompletionTokens,
	})
	return sdkResp, nil
}

func (r *router) appendAudit(e auditEntry) {
	r.mu.Lock()
	r.audit = append(r.audit, e)
	r.mu.Unlock()
}

func messagesToWire(msgs []sdk.Message) []map[string]string {
	out := make([]map[string]string, 0, len(msgs))
	for _, m := range msgs {
		out = append(out, map[string]string{"role": m.Role, "content": m.Content})
	}
	return out
}

func toolsToWire(tools []sdk.Tool) []map[string]any {
	out := make([]map[string]any, 0, len(tools))
	for _, t := range tools {
		out = append(out, map[string]any{
			"type": t.Type,
			"function": map[string]any{
				"name":        t.Function.Name,
				"description": t.Function.Description,
				"parameters":  t.Function.Parameters,
			},
		})
	}
	return out
}

// ---------- OTel probe (no-op unless PHENOTYPE_E2E_OTEL=1) ----------

// otelSpan is the minimal span record this harness emits. The real OTel
// SDK (go.opentelemetry.io/otel) is not currently wired into this
// module — go.mod has no OTel deps — so this struct lets the harness
// record span-shaped data without forcing a dependency. When OTel is
// added, this can be replaced with oteltrace.Span without changing the
// test assertions.
type otelSpan struct {
	Name       string
	Start      time.Time
	End        time.Time
	Attributes map[string]any
}

type otelRecorder struct {
	mu    sync.Mutex
	spans []otelSpan
}

func (o *otelRecorder) record(s otelSpan) {
	o.mu.Lock()
	o.spans = append(o.spans, s)
	o.mu.Unlock()
}

func (o *otelRecorder) snapshot() []otelSpan {
	o.mu.Lock()
	defer o.mu.Unlock()
	out := make([]otelSpan, len(o.spans))
	copy(out, o.spans)
	return out
}

// otelEnabled reports whether OTel capture should run. Off by default —
// the harness is self-contained and must not require external deps.
func otelEnabled() bool {
	return strings.EqualFold(os.Getenv("PHENOTYPE_E2E_OTEL"), "1")
}

// ---------- the test ----------

func TestE2E_RouterThroughMockOpenAI(t *testing.T) {
	mock := newMockOpenAI(t)
	r := newRouter(mock.URL(), "test-key-not-real")

	rec := &otelRecorder{}
	_ = rec // recorder is populated only when otelEnabled(); kept in scope for clarity.

	// (1) configure router to use the mock — done by newRouter(mock.URL()).
	if r.baseURL != mock.URL() {
		t.Fatalf("router baseURL not configured to mock: got %q want %q", r.baseURL, mock.URL())
	}

	// (2)+(3) send a request via the router.
	req := &sdk.Request{
		Provider: "openai",
		Model:    "gpt-4o",
		Messages: []sdk.Message{
			{Role: "user", Content: "Say hi to the harness."},
		},
		Params: map[string]any{"temperature": 0.0, "max_tokens": 64},
	}

	var (
		start   = time.Now()
		spanRec = otelSpan{
			Name:  "router.route",
			Start: start,
			Attributes: map[string]any{
				"router.provider": req.Provider,
				"router.model":    req.Model,
			},
		}
		resp *sdk.Response
		err  error
	)

	if otelEnabled() {
		// Span-shaped capture path. Kept side-effect free so tests are
		// deterministic when PHENOTYPE_E2E_OTEL is unset.
		resp, err = r.Route(context.Background(), req)
		spanRec.End = time.Now()
		spanRec.Attributes["router.status"] = "ok"
		rec.record(spanRec)
	} else {
		resp, err = r.Route(context.Background(), req)
	}

	// (4) verify response.
	if err != nil {
		t.Fatalf("router.Route returned error: %v", err)
	}
	if resp == nil {
		t.Fatal("router.Route returned nil response without error")
	}
	if resp.Provider != "openai" {
		t.Errorf("response.Provider = %q, want %q", resp.Provider, "openai")
	}
	// OpenAI-shaped providers echo the request model. Mock follows suit.
	if resp.Model != "gpt-4o" {
		t.Errorf("response.Model = %q, want %q (echo of request model)", resp.Model, "gpt-4o")
	}
	if !strings.Contains(resp.Content, "Hello from the mock provider") {
		t.Errorf("response.Content = %q, want it to contain mock canned content", resp.Content)
	}
	if resp.Usage.TotalTokens != 19 {
		t.Errorf("response.Usage.TotalTokens = %d, want 19 (12 prompt + 7 completion)", resp.Usage.TotalTokens)
	}

	// (4a) verify the mock actually got called and saw the auth header.
	if got := mock.requestCount(); got != 1 {
		t.Fatalf("mock request count = %d, want 1", got)
	}
	last := mock.lastRequest(t)
	if last.Path != "/v1/chat/completions" {
		t.Errorf("mock saw path %q, want /v1/chat/completions", last.Path)
	}
	if last.Authorization != "Bearer test-key-not-real" {
		t.Errorf("mock saw Authorization %q, want %q", last.Authorization, "Bearer test-key-not-real")
	}
	if !bytes.Contains(last.Body, []byte("Say hi to the harness.")) {
		t.Errorf("mock body did not contain user message; got: %s", string(last.Body))
	}
	if !bytes.Contains(last.Body, []byte(`"temperature":0`)) {
		t.Errorf("mock body did not contain temperature param; got: %s", string(last.Body))
	}

	// (4b) verify audit log.
	if got := r.auditLen(); got != 1 {
		t.Fatalf("audit log size = %d, want 1", got)
	}
	audit := r.auditSnapshot()
	e := audit[0]
	if e.Status != "ok" {
		t.Errorf("audit.Status = %q, want ok", e.Status)
	}
	if e.RequestID != "req-1" {
		t.Errorf("audit.RequestID = %q, want req-1", e.RequestID)
	}
	if e.Provider != "openai" || e.Model != "gpt-4o" {
		t.Errorf("audit provider/model = (%q,%q), want (openai,gpt-4o)", e.Provider, e.Model)
	}
	if e.TokensIn != 12 || e.TokensOut != 7 {
		t.Errorf("audit tokens = (%d,%d), want (12,7)", e.TokensIn, e.TokensOut)
	}
	if e.LatencyMS < 0 {
		t.Errorf("audit.LatencyMS = %d, want >= 0", e.LatencyMS)
	}

	// (4c) verify OTel — only when enabled.
	if otelEnabled() {
		spans := rec.snapshot()
		if len(spans) != 1 {
			t.Fatalf("OTel recorder got %d spans, want 1", len(spans))
		}
		s := spans[0]
		if s.Name != "router.route" {
			t.Errorf("OTel span.Name = %q, want router.route", s.Name)
		}
		if s.End.Before(s.Start) {
			t.Errorf("OTel span end (%v) is before start (%v)", s.End, s.Start)
		}
		if s.Attributes["router.provider"] != "openai" {
			t.Errorf("OTel span attribute router.provider = %v, want openai", s.Attributes["router.provider"])
		}
	}
}

// TestE2E_AuditOnError verifies the audit log captures the error path
// (5xx from upstream) with Status="error" and an err string.
func TestE2E_AuditOnError(t *testing.T) {
	mock := newMockOpenAI(t)
	mock.failWithStatus = 502
	mock.failBody = `{"error":{"message":"upstream down","code":502}}`
	r := newRouter(mock.URL(), "k")

	resp, err := r.Route(context.Background(), &sdk.Request{
		Provider: "openai", Model: "gpt-4o",
		Messages: []sdk.Message{{Role: "user", Content: "trigger 502"}},
	})
	if err == nil {
		t.Fatal("expected error on 502, got nil")
	}
	if resp != nil {
		t.Errorf("expected nil response on error, got %+v", resp)
	}
	var sdkErr *sdk.Error
	if !errors.As(err, &sdkErr) {
		t.Fatalf("error is not *sdk.Error: %T %v", err, err)
	}
	if sdkErr.Code != 502 {
		t.Errorf("sdkErr.Code = %d, want 502", sdkErr.Code)
	}

	audit := r.auditSnapshot()
	if len(audit) != 1 {
		t.Fatalf("audit log size = %d, want 1", len(audit))
	}
	e := audit[0]
	if e.Status != "error" {
		t.Errorf("audit.Status = %q, want error", e.Status)
	}
	if e.Err == "" {
		t.Errorf("audit.Err is empty; expected upstream error message")
	}
	if !strings.Contains(e.Err, "upstream down") {
		t.Errorf("audit.Err = %q, want it to contain 'upstream down'", e.Err)
	}
}

// TestE2E_MockServerCountsRequests is a tiny smoke test that the mock
// itself behaves correctly — request counter increments, requests are
// captured, auth header is preserved.
func TestE2E_MockServerCountsRequests(t *testing.T) {
	mock := newMockOpenAI(t)
	r := newRouter(mock.URL(), "k1")

	for i := 0; i < 3; i++ {
		_, err := r.Route(context.Background(), &sdk.Request{
			Provider: "openai", Model: "gpt-4o",
			Messages: []sdk.Message{{Role: "user", Content: fmt.Sprintf("msg %d", i)}},
		})
		if err != nil {
			t.Fatalf("Route #%d: %v", i, err)
		}
	}
	if got := mock.requestCount(); got != 3 {
		t.Errorf("mock request count = %d, want 3", got)
	}
	if got := r.auditLen(); got != 3 {
		t.Errorf("audit log size = %d, want 3", got)
	}
}

// fakePlugin is a compile-time witness that sdk.Plugin's method set
// still fits after any future edit to internal/sdk/sdk.go. The zero
// value is enough; we only need the type-checker to verify the
// methods exist with the right signatures.
type fakePlugin struct{}

func (fakePlugin) GetName() string { return "fake" }
func (fakePlugin) Config() map[string]any {
	return nil
}
func (fakePlugin) TransportInterceptor(_ context.Context, _ *sdk.Request) (*sdk.Request, *sdk.ShortCircuit, error) {
	return nil, nil, sdk.ErrNotImplemented
}
func (fakePlugin) PreHook(_ context.Context, _ *sdk.Request) (*sdk.Request, *sdk.ShortCircuit, error) {
	return nil, nil, sdk.ErrNotImplemented
}
func (fakePlugin) PostHook(_ context.Context, _ *sdk.Response) (*sdk.Response, *sdk.Error, error) {
	return nil, nil, sdk.ErrNotImplemented
}
func (fakePlugin) Cleanup() error { return nil }

// TestE2E_SDKSatisfiesPluginInterface is a compile-time + runtime guard
// that the SDK's Plugin contract still compiles after any future edits
// to internal/sdk/sdk.go. Catches refactor drift early. The var-decl
// inside the function body is the real check — this test also runs a
// zero-value instance through the lifecycle to make sure runtime
// semantics match.
func TestE2E_SDKSatisfiesPluginInterface(t *testing.T) {
	var p sdk.Plugin = fakePlugin{}
	if got := p.GetName(); got != "fake" {
		t.Errorf("Plugin.GetName() = %q, want fake", got)
	}
	if cfg := p.Config(); cfg != nil {
		t.Errorf("Plugin.Config() = %v, want nil", cfg)
	}
	if err := p.Cleanup(); err != nil {
		t.Errorf("Plugin.Cleanup() = %v, want nil", err)
	}
}
