package mcp

import (
	"bufio"
	"context"
	"encoding/json"
	"io"
	"strings"
	"testing"
	"time"

	"kwatch/runner"
)

// helper: build an MCPServer that reads from r and writes to w (instead of
// os.Stdin / os.Stdout). NewMCPServer wires those fields internally, but
// tests need to drive the JSON-RPC loop with deterministic I/O.
func newTestMCPServer(t *testing.T, r io.Reader, w io.Writer) *MCPServer {
	t.Helper()
	s := NewMCPServer(t.TempDir())
	s.reader = bufio.NewScanner(r)
	s.writer = w
	return s
}

// ------------------------------------------------------------------
// JSON-RPC types — round-trip serialisation
// ------------------------------------------------------------------

func TestJSONRPCRequest_Marshaling(t *testing.T) {
	r := JSONRPCRequest{
		JSONRPC: "2.0",
		ID:      1,
		Method:  "tools/list",
	}
	data, err := json.Marshal(r)
	if err != nil {
		t.Fatalf("Marshal: %v", err)
	}
	got := string(data)
	if !strings.Contains(got, `"jsonrpc":"2.0"`) {
		t.Errorf("missing jsonrpc field: %s", got)
	}
	if !strings.Contains(got, `"method":"tools/list"`) {
		t.Errorf("missing method field: %s", got)
	}
}

func TestJSONRPCResponse_Marshaling(t *testing.T) {
	r := JSONRPCResponse{
		JSONRPC: "2.0",
		ID:      "abc",
		Result:  map[string]string{"hello": "world"},
	}
	data, err := json.Marshal(r)
	if err != nil {
		t.Fatalf("Marshal: %v", err)
	}
	if !strings.Contains(string(data), `"id":"abc"`) {
		t.Errorf("missing id field: %s", string(data))
	}
	if !strings.Contains(string(data), `"result":`) {
		t.Errorf("missing result field: %s", string(data))
	}
}

// JSON serialises numbers unquoted, so a marshaled error code shows up as
// `code":-32601` (no surrounding quotes around the digits). The test was
// originally written to look for `"-32601"` (quoted as a JSON string),
// which can never appear in the output.
func TestJSONRPCResponse_ErrorMarshaling(t *testing.T) {
	r := JSONRPCResponse{
		JSONRPC: "2.0",
		ID:      1,
		Error:   &JSONRPCError{Code: -32601, Message: "Method not found"},
	}
	data, err := json.Marshal(r)
	if err != nil {
		t.Fatalf("Marshal: %v", err)
	}
	if !strings.Contains(string(data), `"error":`) {
		t.Errorf("missing error field: %s", string(data))
	}
	if !strings.Contains(string(data), `-32601`) {
		t.Errorf("missing error code: %s", string(data))
	}
	if !strings.Contains(string(data), `"Method not found"`) {
		t.Errorf("missing error message: %s", string(data))
	}
}

// ------------------------------------------------------------------
// formatCommandResults
// ------------------------------------------------------------------

func TestFormatCommandResults_AllTypes(t *testing.T) {
	now := time.Now()
	results := map[runner.CommandType]runner.CommandResult{
		runner.TypescriptCheck: {
			Command: "tsc", Passed: true, IssueCount: 0, FileCount: 0,
			Duration: 100 * time.Millisecond, Timestamp: now,
		},
		runner.LintCheck: {
			Command: "eslint", Passed: false, IssueCount: 3, FileCount: 2,
			Duration: 200 * time.Millisecond, Timestamp: now,
		},
		runner.TestRunner: {
			Command: "npm test", Passed: false, IssueCount: 1,
			TotalTests: 10, PassedTests: 7, FailedTests: 3,
			Duration: 500 * time.Millisecond, Timestamp: now,
		},
		runner.CommandType("custom_check"): {
			Command: "custom", Passed: true, IssueCount: 0,
			Duration: 50 * time.Millisecond, Timestamp: now,
		},
	}

	got := formatCommandResults(results)
	if _, ok := got["tsc"]; !ok {
		t.Error("tsc missing from formatted results")
	}
	if _, ok := got["lint"]; !ok {
		t.Error("lint missing from formatted results")
	}
	if _, ok := got["test"]; !ok {
		t.Error("test missing from formatted results")
	}
	if _, ok := got["custom_check"]; !ok {
		t.Error("custom_check missing from formatted results")
	}

	// Test-specific fields are only present on the test entry.
	testEntry, ok := got["test"].(map[string]interface{})
	if !ok {
		t.Fatal("test entry is not a map")
	}
	if _, ok := testEntry["total_tests"]; !ok {
		t.Error("total_tests should be present on test entry")
	}
	if _, ok := testEntry["passed_tests"]; !ok {
		t.Error("passed_tests should be present on test entry")
	}
	if _, ok := testEntry["failed_tests"]; !ok {
		t.Error("failed_tests should be present on test entry")
	}

	// TSC entry should not have test-specific fields.
	tscEntry, _ := got["tsc"].(map[string]interface{})
	if _, ok := tscEntry["total_tests"]; ok {
		t.Error("total_tests should NOT be present on tsc entry")
	}
}

func TestFormatCommandResults_Empty(t *testing.T) {
	got := formatCommandResults(map[runner.CommandType]runner.CommandResult{})
	if len(got) != 0 {
		t.Errorf("empty results: len = %d, want 0", len(got))
	}
}

// ------------------------------------------------------------------
// handleMessage dispatch
//
// handleMessage always returns the error from sendError/sendResponse; on
// success those helpers return nil. The tests assert on the response written
// to the buffer rather than the error value, because the only thing we can
// observe from a client perspective is what hits the wire.
// ------------------------------------------------------------------

func TestHandleMessage_ParseError(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	if err := s.handleMessage("not json"); err != nil {
		t.Fatalf("handleMessage parse error: %v", err)
	}
	if !strings.Contains(buf.String(), `"code":-32700`) {
		t.Errorf("expected parse error -32700, got: %s", buf.String())
	}
	if !strings.Contains(buf.String(), `"Parse error"`) {
		t.Errorf("expected parse error message, got: %s", buf.String())
	}
}

func TestHandleMessage_UnknownMethod(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	req := JSONRPCRequest{JSONRPC: "2.0", ID: 1, Method: "totally/unknown"}
	data, _ := json.Marshal(req)
	if err := s.handleMessage(string(data)); err != nil {
		t.Fatalf("handleMessage unknown method: %v", err)
	}
	if !strings.Contains(buf.String(), `"code":-32601`) {
		t.Errorf("expected method-not-found -32601, got: %s", buf.String())
	}
}

func TestHandleMessage_NotificationsInitialized(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	req := JSONRPCRequest{JSONRPC: "2.0", Method: "notifications/initialized"}
	data, _ := json.Marshal(req)
	if err := s.handleMessage(string(data)); err != nil {
		t.Errorf("handleMessage notifications/initialized: %v", err)
	}
	// No response is sent for notifications
	if buf.Len() != 0 {
		t.Errorf("notification should produce no output, got: %s", buf.String())
	}
}

func TestHandleMessage_Initialize(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	params := InitializeParams{
		ProtocolVersion: "2025-03-26",
		ClientInfo:      ClientInfo{Name: "test", Version: "0.0.1"},
	}
	paramBytes, _ := json.Marshal(params)
	req := JSONRPCRequest{
		JSONRPC: "2.0", ID: 1, Method: "initialize",
		Params: paramBytes,
	}
	data, _ := json.Marshal(req)
	if err := s.handleMessage(string(data)); err != nil {
		t.Fatalf("handleMessage initialize: %v", err)
	}
	if !strings.Contains(buf.String(), `"protocolVersion":"2025-03-26"`) {
		t.Errorf("initialize response missing protocolVersion: %s", buf.String())
	}
}

// Pass a params value of the wrong type (a number) so the outer JSON parses
// but the handler's struct-decode fails with the documented -32602.
func TestHandleMessage_Initialize_BadParams(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	req := JSONRPCRequest{JSONRPC: "2.0", ID: 1, Method: "initialize", Params: json.RawMessage(`42`)}
	data, _ := json.Marshal(req)
	if err := s.handleMessage(string(data)); err != nil {
		t.Fatalf("handleMessage bad params: %v", err)
	}
	if !strings.Contains(buf.String(), `"code":-32602`) {
		t.Errorf("expected invalid-params -32602, got: %s", buf.String())
	}
}

func TestHandleMessage_Initialize_UnsupportedVersion(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	params := InitializeParams{ProtocolVersion: "9999-99-99"}
	paramBytes, _ := json.Marshal(params)
	req := JSONRPCRequest{
		JSONRPC: "2.0", ID: 1, Method: "initialize",
		Params: paramBytes,
	}
	data, _ := json.Marshal(req)
	if err := s.handleMessage(string(data)); err != nil {
		t.Fatalf("handleMessage unsupported version: %v", err)
	}
	if !strings.Contains(buf.String(), `"code":-32602`) {
		t.Errorf("expected invalid-params -32602, got: %s", buf.String())
	}
	if !strings.Contains(buf.String(), `"Unsupported protocol version"`) {
		t.Errorf("expected unsupported version message, got: %s", buf.String())
	}
}

func TestHandleMessage_ToolsList(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	req := JSONRPCRequest{JSONRPC: "2.0", ID: 1, Method: "tools/list"}
	data, _ := json.Marshal(req)
	if err := s.handleMessage(string(data)); err != nil {
		t.Fatalf("handleMessage tools/list: %v", err)
	}
	if !strings.Contains(buf.String(), `"get_build_status"`) {
		t.Errorf("tools/list missing get_build_status: %s", buf.String())
	}
	if !strings.Contains(buf.String(), `"run_commands"`) {
		t.Errorf("tools/list missing run_commands: %s", buf.String())
	}
	if !strings.Contains(buf.String(), `"get_command_history"`) {
		t.Errorf("tools/list missing get_command_history: %s", buf.String())
	}
}

func TestHandleMessage_ToolsCall_UnknownTool(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	args := map[string]interface{}{"name": "no_such_tool"}
	argBytes, _ := json.Marshal(args)
	req := JSONRPCRequest{
		JSONRPC: "2.0", ID: 1, Method: "tools/call",
		Params: argBytes,
	}
	data, _ := json.Marshal(req)
	if err := s.handleMessage(string(data)); err != nil {
		t.Fatalf("handleMessage unknown tool: %v", err)
	}
	if !strings.Contains(buf.String(), `"code":-32602`) {
		t.Errorf("expected invalid-params -32602, got: %s", buf.String())
	}
	if !strings.Contains(buf.String(), `"Unknown tool"`) {
		t.Errorf("expected unknown-tool message, got: %s", buf.String())
	}
}

// Sending a params value of the wrong type (a JSON string instead of the
// expected object) is a valid outer JSON message that the parser accepts
// but the handler's struct-decode rejects with the documented -32602.
func TestHandleMessage_ToolsCall_BadParams(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	req := JSONRPCRequest{JSONRPC: "2.0", ID: 1, Method: "tools/call", Params: json.RawMessage(`"not an object"`)}
	data, _ := json.Marshal(req)
	if err := s.handleMessage(string(data)); err != nil {
		t.Fatalf("handleMessage bad params: %v", err)
	}
	if !strings.Contains(buf.String(), `"code":-32602`) {
		t.Errorf("expected invalid-params -32602, got: %s", buf.String())
	}
	if !strings.Contains(buf.String(), `"Invalid params"`) {
		t.Errorf("expected invalid-params message, got: %s", buf.String())
	}
}

// A truly malformed wire message (truncated JSON) is rejected at the
// outermost parse step with the JSON-RPC standard -32700.
func TestHandleMessage_ToolsCall_MalformedRequest(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	if err := s.handleMessage(`{"jsonrpc":"2.0","id":1,"method":"tools/call","params":`); err != nil {
		t.Fatalf("handleMessage malformed: %v", err)
	}
	if !strings.Contains(buf.String(), `"code":-32700`) {
		t.Errorf("expected parse error -32700, got: %s", buf.String())
	}
}

func TestHandleMessage_ToolsCall_GetBuildStatus(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	args := map[string]interface{}{"name": "get_build_status", "arguments": map[string]interface{}{}}
	argBytes, _ := json.Marshal(args)
	req := JSONRPCRequest{
		JSONRPC: "2.0", ID: 1, Method: "tools/call",
		Params: argBytes,
	}
	data, _ := json.Marshal(req)
	if err := s.handleMessage(string(data)); err != nil {
		t.Fatalf("handleMessage tools/call: %v", err)
	}
	if !strings.Contains(buf.String(), `"content"`) {
		t.Errorf("expected content block in response: %s", buf.String())
	}
}

// The MCP tool responses are wrapped in {"content":[{"text":"<json string>"}]}.
// The `text` field is itself a JSON string, so inner keys like "history" and
// "results" are written as `\"history\"` in the outer response. Searching for
// the unescaped key (no quotes) is the right invariant.
func TestHandleMessage_ToolsCall_GetCommandHistory(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	// Pre-seed the runner with a couple of history entries
	s.runner.RunCommand(context.Background(), runner.Command{
		Type:    runner.TypescriptCheck,
		Command: "echo tsc",
		Timeout: 2 * time.Second,
	})

	args := map[string]interface{}{
		"name":      "get_command_history",
		"arguments": map[string]interface{}{"limit": 5},
	}
	argBytes, _ := json.Marshal(args)
	req := JSONRPCRequest{
		JSONRPC: "2.0", ID: 1, Method: "tools/call",
		Params: argBytes,
	}
	data, _ := json.Marshal(req)
	if err := s.handleMessage(string(data)); err != nil {
		t.Fatalf("handleMessage: %v", err)
	}
	if !strings.Contains(buf.String(), `history`) {
		t.Errorf("expected history block in response: %s", buf.String())
	}
}

func TestHandleMessage_ToolsCall_GetCommandHistory_WithFilter(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	s.runner.RunCommand(context.Background(), runner.Command{
		Type:    runner.TypescriptCheck,
		Command: "npx tsc --noEmit",
		Timeout: 2 * time.Second,
	})
	s.runner.RunCommand(context.Background(), runner.Command{
		Type:    runner.LintCheck,
		Command: "npx eslint .",
		Timeout: 2 * time.Second,
	})

	args := map[string]interface{}{
		"name":      "get_command_history",
		"arguments": map[string]interface{}{"filter": "tsc"},
	}
	argBytes, _ := json.Marshal(args)
	req := JSONRPCRequest{
		JSONRPC: "2.0", ID: 1, Method: "tools/call",
		Params: argBytes,
	}
	data, _ := json.Marshal(req)
	if err := s.handleMessage(string(data)); err != nil {
		t.Fatalf("handleMessage: %v", err)
	}
	// Response should only contain tsc, not eslint
	if !strings.Contains(buf.String(), `history`) {
		t.Errorf("expected history block: %s", buf.String())
	}
	// eslint should not appear in filtered history
	if strings.Contains(buf.String(), "eslint") {
		t.Errorf("filtered history should not contain eslint: %s", buf.String())
	}
}

func TestHandleMessage_ToolsCall_RunCommands_All(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	args := map[string]interface{}{
		"name":      "run_commands",
		"arguments": map[string]interface{}{"command": "all"},
	}
	argBytes, _ := json.Marshal(args)
	req := JSONRPCRequest{
		JSONRPC: "2.0", ID: 1, Method: "tools/call",
		Params: argBytes,
	}
	data, _ := json.Marshal(req)
	if err := s.handleMessage(string(data)); err != nil {
		t.Fatalf("handleMessage: %v", err)
	}
	if !strings.Contains(buf.String(), `results`) {
		t.Errorf("expected results block: %s", buf.String())
	}
}

func TestHandleMessage_ToolsCall_RunCommands_Invalid(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	args := map[string]interface{}{
		"name":      "run_commands",
		"arguments": map[string]interface{}{"command": "bogus"},
	}
	argBytes, _ := json.Marshal(args)
	req := JSONRPCRequest{
		JSONRPC: "2.0", ID: 1, Method: "tools/call",
		Params: argBytes,
	}
	data, _ := json.Marshal(req)
	if err := s.handleMessage(string(data)); err != nil {
		t.Fatalf("handleMessage: %v", err)
	}
	if !strings.Contains(buf.String(), `"code":-32602`) {
		t.Errorf("expected invalid command error, got: %s", buf.String())
	}
	if !strings.Contains(buf.String(), `"Invalid command"`) {
		t.Errorf("expected invalid command message, got: %s", buf.String())
	}
}

// ------------------------------------------------------------------
// Start() / Stop()
// ------------------------------------------------------------------

func TestStop_CancelsContext(t *testing.T) {
	s := NewMCPServer(t.TempDir())
	if s.ctx.Err() != nil {
		t.Fatal("fresh server's ctx already cancelled")
	}
	s.Stop()
	if s.ctx.Err() == nil {
		t.Error("after Stop(), ctx should be cancelled")
	}
}

// Start() runs an infinite read loop on a bufio.Scanner. Test that it
// returns when stdin is closed and does not error on a clean EOF.
func TestStart_CleanExit(t *testing.T) {
	r, w := io.Pipe()
	s := NewMCPServer(t.TempDir())
	s.reader = bufio.NewScanner(r)
	s.writer = io.Discard

	done := make(chan error, 1)
	go func() { done <- s.Start() }()

	// Close the writer to signal EOF on the reader.
	if err := w.Close(); err != nil {
		t.Fatalf("pipe close: %v", err)
	}
	select {
	case err := <-done:
		if err != nil {
			t.Errorf("Start returned %v on clean EOF", err)
		}
	case <-time.After(2 * time.Second):
		t.Fatal("Start did not return within 2s after EOF")
	}
}

// TestStart_HandleSingleRequest exercises the full read → write loop with
// a single initialize request, end-to-end.
func TestStart_HandleSingleRequest(t *testing.T) {
	var buf strings.Builder
	r, w := io.Pipe()

	s := NewMCPServer(t.TempDir())
	s.reader = bufio.NewScanner(r)
	s.writer = &buf

	done := make(chan struct{})
	go func() {
		defer close(done)
		_ = s.Start()
	}()

	params := InitializeParams{ProtocolVersion: "2025-03-26"}
	pb, _ := json.Marshal(params)
	req := JSONRPCRequest{JSONRPC: "2.0", ID: 7, Method: "initialize", Params: pb}
	rb, _ := json.Marshal(req)
	if _, err := io.WriteString(w, string(rb)+"\n"); err != nil {
		t.Fatalf("write: %v", err)
	}
	// Give Start a moment to process the line.
	time.Sleep(50 * time.Millisecond)
	if err := w.Close(); err != nil {
		t.Fatalf("close: %v", err)
	}
	select {
	case <-done:
	case <-time.After(2 * time.Second):
		t.Fatal("Start did not return")
	}
	if !strings.Contains(buf.String(), `"id":7`) {
		t.Errorf("expected id:7 in response, got: %s", buf.String())
	}
}

// ------------------------------------------------------------------
// writeMessage / sendResponse / sendError
// ------------------------------------------------------------------

func TestWriteMessage_MarshalsJSON(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)

	if err := s.writeMessage(map[string]string{"hello": "world"}); err != nil {
		t.Fatalf("writeMessage: %v", err)
	}
	if !strings.HasSuffix(buf.String(), "\n") {
		t.Error("writeMessage should end with newline")
	}
}

func TestSendResponse_WrapsResult(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	if err := s.sendResponse(42, map[string]int{"x": 1}); err != nil {
		t.Fatalf("sendResponse: %v", err)
	}
	if !strings.Contains(buf.String(), `"id":42`) {
		t.Errorf("id missing: %s", buf.String())
	}
	if !strings.Contains(buf.String(), `"result":`) {
		t.Errorf("result missing: %s", buf.String())
	}
}

func TestSendError_WrapsError(t *testing.T) {
	var buf strings.Builder
	s := newTestMCPServer(t, strings.NewReader(""), &buf)
	if err := s.sendError(99, -32600, "boom", "extra"); err != nil {
		t.Fatalf("sendError: %v", err)
	}
	if !strings.Contains(buf.String(), `"id":99`) {
		t.Errorf("id missing: %s", buf.String())
	}
	// Numbers serialize unquoted: -32600, not "-32600".
	if !strings.Contains(buf.String(), `-32600`) {
		t.Errorf("code missing: %s", buf.String())
	}
	if !strings.Contains(buf.String(), `"message":"boom"`) {
		t.Errorf("message missing: %s", buf.String())
	}
	if !strings.Contains(buf.String(), `"data":"extra"`) {
		t.Errorf("data missing: %s", buf.String())
	}
}
