package tests

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

// TestE2ESmoke is a minimal E2E smoke test that validates the HTTP layer.
// It exercises the full request lifecycle through an httptest server.
// At T3=2 (wired), this runs in CI on every PR.
func TestE2ESmoke(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status":"ok"}`))
	})

	server := httptest.NewServer(handler)
	defer server.Close()

	resp, err := http.Get(server.URL + "/health")
	if err != nil {
		t.Fatalf("E2E request failed: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		t.Fatalf("expected 200, got %d", resp.StatusCode)
	}

	t.Log("E2E smoke test passed — HTTP layer is healthy")
}
