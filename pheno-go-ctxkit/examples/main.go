// Quickstart example for pheno-go-ctxkit.
//
// Run with:
//   go run ./examples
//
// Demonstrates a round-trip: Inject a traceparent into outbound headers,
// then Extract it back on the receiving side.

package main

import (
	"fmt"
	"net/http"
	"net/http/httptest"

	"github.com/KooshaPari/pheno-go-ctxkit/ctxkit"
)

func main() {
	// Server side: extract the traceparent from the request
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		ctx, err := ctxkit.Extract(r.Header)
		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		tp := ctxkit.TraceparentFromContext(ctx)
		fmt.Fprintf(w, "server received: %s\n", tp)
	}))
	defer server.Close()

	// Client side: inject the traceparent into the outgoing request
	req, _ := http.NewRequest("GET", server.URL, nil)
	if err := ctxkit.Inject(req.Context(), req.Header); err != nil {
		fmt.Println("inject error:", err)
		return
	}

	resp, _ := http.DefaultClient.Do(req)
	defer resp.Body.Close()

	fmt.Println("client sent:", req.Header.Get("Traceparent"))
	buf := make([]byte, 256)
	n, _ := resp.Body.Read(buf)
	fmt.Print(string(buf[:n]))
}
