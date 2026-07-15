package runner

import (
	"encoding/json"
	"testing"
	"time"
)

func TestCommandResult_JSON(t *testing.T) {
	r := CommandResult{
		Command:     "npx tsc",
		Passed:      true,
		IssueCount:  0,
		Duration:    1234 * time.Millisecond,
		Timestamp:   time.Unix(1700000000, 0).UTC(),
		TotalTests:  5,
		PassedTests: 5,
		FailedTests: 0,
	}

	data, err := json.Marshal(r)
	if err != nil {
		t.Fatalf("Marshal: %v", err)
	}

	// Decode back to verify roundtrip
	var got CommandResult
	if err := json.Unmarshal(data, &got); err != nil {
		t.Fatalf("Unmarshal: %v", err)
	}

	if got.Command != r.Command {
		t.Errorf("Command = %q, want %q", got.Command, r.Command)
	}
	if got.Passed != r.Passed {
		t.Errorf("Passed = %v, want %v", got.Passed, r.Passed)
	}
	if got.IssueCount != r.IssueCount {
		t.Errorf("IssueCount = %d, want %d", got.IssueCount, r.IssueCount)
	}
	if got.TotalTests != r.TotalTests {
		t.Errorf("TotalTests = %d, want %d", got.TotalTests, r.TotalTests)
	}
}

func TestCommandTypeConstants(t *testing.T) {
	if string(TypescriptCheck) != "typescript" {
		t.Errorf("TypescriptCheck = %q, want \"typescript\"", TypescriptCheck)
	}
	if string(LintCheck) != "lint" {
		t.Errorf("LintCheck = %q, want \"lint\"", LintCheck)
	}
	if string(TestRunner) != "test" {
		t.Errorf("TestRunner = %q, want \"test\"", TestRunner)
	}
	if string(SecurityCheck) != "security" {
		t.Errorf("SecurityCheck = %q, want \"security\"", SecurityCheck)
	}
}

func TestResultHistory_Empty(t *testing.T) {
	h := &ResultHistory{}

	if got := len(h.GetAll()); got != 0 {
		t.Errorf("empty history: GetAll() = %d, want 0", got)
	}
	latest := h.GetLatest()
	if len(latest) != 0 {
		t.Errorf("empty history: GetLatest() = %d, want 0", len(latest))
	}
}
