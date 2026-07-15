package tui

import (
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	tea "github.com/charmbracelet/bubbletea"
	"github.com/charmbracelet/lipgloss"
	"github.com/fsnotify/fsnotify"
	"kwatch/config"
	"kwatch/runner"
)

// ------------------------------------------------------------------
// helpers
// ------------------------------------------------------------------

// newTestModel returns a Model backed by a fresh temp dir so the runner and
// kwatchConfig can be constructed without touching real project files.
func newTestModel(t *testing.T) Model {
	t.Helper()
	return NewModel(t.TempDir())
}

// ------------------------------------------------------------------
// Model construction
// ------------------------------------------------------------------

func TestNewModel_Defaults(t *testing.T) {
	dir := t.TempDir()
	m := NewModel(dir)

	if m.watchDir != dir {
		t.Errorf("watchDir = %q, want %q", m.watchDir, dir)
	}
	if m.viewMode != ViewMain {
		t.Errorf("viewMode = %d, want ViewMain (%d)", m.viewMode, ViewMain)
	}
	if m.serverPort != 8080 {
		t.Errorf("serverPort = %d, want 8080", m.serverPort)
	}
	if m.maxLogs != 100 {
		t.Errorf("maxLogs = %d, want 100", m.maxLogs)
	}
	if m.runner == nil {
		t.Error("runner is nil")
	}
	if m.kwatchConfig == nil {
		t.Error("kwatchConfig is nil")
	}
	if m.history == nil {
		t.Error("history is nil")
	}
	if m.running == nil {
		t.Error("running map is nil")
	}
	if m.logs == nil {
		t.Error("logs slice is nil")
	}
}

func TestModelInit(t *testing.T) {
	m := newTestModel(t)
	cmd := m.Init()
	if cmd == nil {
		t.Error("Init() returned nil tea.Cmd")
	}
}

func TestModelUpdateSize(t *testing.T) {
	m := newTestModel(t)
	if m.ready {
		t.Error("fresh model is already ready")
	}
	m.UpdateSize(120, 40)
	if m.width != 120 || m.height != 40 {
		t.Errorf("size = (%d, %d), want (120, 40)", m.width, m.height)
	}
	if !m.ready {
		t.Error("UpdateSize did not set ready=true")
	}
}

// ------------------------------------------------------------------
// Logs
// ------------------------------------------------------------------

func TestModel_AddLog_AndGetRecent(t *testing.T) {
	m := newTestModel(t)

	for i := 0; i < 5; i++ {
		m.AddLog(LogInfo, "msg", "file.go", "modified")
	}
	if got := len(m.GetRecentLogs(100)); got != 5 {
		t.Errorf("len(GetRecentLogs) = %d, want 5", got)
	}
	if got := len(m.GetRecentLogs(3)); got != 3 {
		t.Errorf("len(GetRecentLogs(3)) = %d, want 3", got)
	}
	// Recent logs should be the last 3 entries.
	for i, e := range m.GetRecentLogs(3) {
		if e.Type != LogInfo {
			t.Errorf("entry %d: type = %d, want LogInfo", i, e.Type)
		}
	}
}

func TestModel_AddLog_TrimsToMaxLogs(t *testing.T) {
	m := newTestModel(t)
	// maxLogs is 100. Add 200 entries.
	for i := 0; i < 200; i++ {
		m.AddLog(LogInfo, "x", "", "")
	}
	// After trimming, the second-pass guard trims to <= 50.
	if got := len(m.logs); got > 50 {
		t.Errorf("len(logs) = %d, want <= 50 (post-trim guard)", got)
	}
}

func TestModel_GetRecentLogs_Empty(t *testing.T) {
	m := newTestModel(t)
	if got := m.GetRecentLogs(10); len(got) != 0 {
		t.Errorf("empty logs: len = %d, want 0", len(got))
	}
}

// ------------------------------------------------------------------
// Command status / running state
// ------------------------------------------------------------------

func TestModel_GetCurrentCommandStatuses_Defaults(t *testing.T) {
	m := newTestModel(t)
	statuses := m.GetCurrentCommandStatuses()
	if len(statuses) != 3 {
		t.Fatalf("len(statuses) = %d, want 3", len(statuses))
	}
	wantOrder := []runner.CommandType{runner.TypescriptCheck, runner.LintCheck, runner.TestRunner}
	for i, s := range statuses {
		if s.Type != wantOrder[i] {
			t.Errorf("statuses[%d].Type = %q, want %q", i, s.Type, wantOrder[i])
		}
		if s.Running {
			t.Errorf("statuses[%d].Running = true, want false", i)
		}
		if s.Result != nil {
			t.Errorf("statuses[%d].Result should be nil before any run", i)
		}
	}
}

func TestModel_SetCommandRunning_FlipsRunning(t *testing.T) {
	m := newTestModel(t)
	m.SetCommandRunning(runner.TypescriptCheck, true)
	if !m.running[runner.TypescriptCheck] {
		t.Error("SetCommandRunning(true) did not set running flag")
	}
	if !m.IsAnyCommandRunning() {
		t.Error("IsAnyCommandRunning = false after SetCommandRunning(true)")
	}
	if !m.IsRunning() {
		t.Error("IsRunning = false after SetCommandRunning(true)")
	}

	m.SetCommandRunning(runner.TypescriptCheck, false)
	if m.running[runner.TypescriptCheck] {
		t.Error("SetCommandRunning(false) did not clear running flag")
	}
	if m.IsAnyCommandRunning() {
		t.Error("IsAnyCommandRunning = true after all commands stopped")
	}
}

func TestModel_IsAnyCommandRunning_Empty(t *testing.T) {
	m := newTestModel(t)
	if m.IsAnyCommandRunning() {
		t.Error("fresh model reports a command running")
	}
	if m.IsRunning() {
		t.Error("fresh model IsRunning() = true")
	}
}

func TestModel_AddCommandResult_RecordsHistory(t *testing.T) {
	m := newTestModel(t)
	res := runner.CommandResult{
		Command:   "tsc",
		Passed:    true,
		Timestamp: time.Now(),
		Duration:  10 * time.Millisecond,
	}
	m.AddCommandResult(res)
	latest := m.history.GetLatest()
	if got, ok := latest[runner.TypescriptCheck]; !ok {
		t.Error("history does not contain tsc result")
	} else if !got.Passed {
		t.Error("history tsc result Passed = false")
	}
	if m.IsAnyCommandRunning() {
		t.Error("AddCommandResult should clear the running flag for the command type")
	}
}

func TestModel_GetHistoryForView_SortedNewestFirst(t *testing.T) {
	m := newTestModel(t)
	base := time.Now()
	old := runner.CommandResult{Command: "tsc", Passed: true, Timestamp: base.Add(-2 * time.Hour)}
	med := runner.CommandResult{Command: "tsc", Passed: false, Timestamp: base.Add(-1 * time.Hour)}
	latest := runner.CommandResult{Command: "tsc", Passed: true, Timestamp: base}
	m.AddCommandResult(old)
	m.AddCommandResult(med)
	m.AddCommandResult(latest)

	view := m.GetHistoryForView()
	if len(view) != 3 {
		t.Fatalf("len(view) = %d, want 3", len(view))
	}
	if !view[0].Timestamp.Equal(latest.Timestamp) {
		t.Errorf("view[0] = %v, want %v (newest first)", view[0].Timestamp, latest.Timestamp)
	}
	if !view[2].Timestamp.Equal(old.Timestamp) {
		t.Errorf("view[2] = %v, want %v (oldest last)", view[2].Timestamp, old.Timestamp)
	}
}

// ------------------------------------------------------------------
// Status summary / error metrics
// ------------------------------------------------------------------

func TestModel_GetStatusSummary(t *testing.T) {
	cases := []struct {
		name string
		seed func(*Model)
		want string
	}{
		{
			name: "no results",
			seed: func(*Model) {},
			want: "Ready",
		},
		{
			name: "all passed",
			seed: func(m *Model) {
				for _, ct := range []runner.CommandType{runner.TypescriptCheck, runner.LintCheck, runner.TestRunner} {
					m.AddCommandResult(runner.CommandResult{Command: string(ct), Passed: true, Timestamp: time.Now()})
				}
			},
			want: "All Passed",
		},
		{
			name: "one failed",
			seed: func(m *Model) {
				m.AddCommandResult(runner.CommandResult{Command: string(runner.TypescriptCheck), Passed: true, Timestamp: time.Now()})
				m.AddCommandResult(runner.CommandResult{Command: string(runner.LintCheck), Passed: false, Timestamp: time.Now()})
				m.AddCommandResult(runner.CommandResult{Command: string(runner.TestRunner), Passed: true, Timestamp: time.Now()})
			},
			want: "Failed",
		},
		{
			name: "running overrides all",
			seed: func(m *Model) {
				m.AddCommandResult(runner.CommandResult{Command: string(runner.TypescriptCheck), Passed: true, Timestamp: time.Now()})
				m.SetCommandRunning(runner.LintCheck, true)
			},
			want: "Running",
		},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			m := newTestModel(t)
			tc.seed(&m)
			if got := m.GetStatusSummary(); got != tc.want {
				t.Errorf("GetStatusSummary() = %q, want %q", got, tc.want)
			}
		})
	}
}

func TestModel_GetErrorMetrics(t *testing.T) {
	m := newTestModel(t)
	now := time.Now()
	// TSC failed with 3 issues, 1 file.
	m.AddCommandResult(runner.CommandResult{
		Command: string(runner.TypescriptCheck), Passed: false,
		IssueCount: 3, FileCount: 1, Timestamp: now,
	})
	// Lint passed.
	m.AddCommandResult(runner.CommandResult{
		Command: string(runner.LintCheck), Passed: true,
		IssueCount: 0, FileCount: 0, Timestamp: now,
	})
	// Test failed with 2 failed tests, no IssueCount/FileCount.
	m.AddCommandResult(runner.CommandResult{
		Command: string(runner.TestRunner), Passed: false,
		FailedTests: 2, Timestamp: now,
	})

	errs, files := m.GetErrorMetrics()
	if errs != 5 { // 3 tsc + 2 test
		t.Errorf("errs = %d, want 5", errs)
	}
	if files != 1 {
		t.Errorf("files = %d, want 1", files)
	}
}

func TestModel_GetErrorMetrics_AllPass(t *testing.T) {
	m := newTestModel(t)
	now := time.Now()
	for _, ct := range []runner.CommandType{runner.TypescriptCheck, runner.LintCheck, runner.TestRunner} {
		m.AddCommandResult(runner.CommandResult{Command: string(ct), Passed: true, Timestamp: now})
	}
	errs, files := m.GetErrorMetrics()
	if errs != 0 || files != 0 {
		t.Errorf("all pass: errs=%d, files=%d, want 0,0", errs, files)
	}
}

// ------------------------------------------------------------------
// Watcher / server state setters
// ------------------------------------------------------------------

func TestModel_SetWatcherActive_LogsTransitions(t *testing.T) {
	m := newTestModel(t)
	before := len(m.logs)
	m.SetWatcherActive(true)
	if !m.watcherActive {
		t.Error("SetWatcherActive(true) did not flip flag")
	}
	if len(m.logs) <= before {
		t.Error("SetWatcherActive(true) should log a transition")
	}
	// Setting the same value again is a no-op for logging.
	beforeRepeat := len(m.logs)
	m.SetWatcherActive(true)
	if len(m.logs) != beforeRepeat {
		t.Error("repeating SetWatcherActive(true) should not log again")
	}
}

func TestModel_SetServerActive(t *testing.T) {
	m := newTestModel(t)
	m.SetServerActive(true)
	if !m.serverActive {
		t.Error("SetServerActive(true) did not flip flag")
	}
	m.SetServerActive(false)
	if m.serverActive {
		t.Error("SetServerActive(false) did not flip flag")
	}
}

// ------------------------------------------------------------------
// Error state
// ------------------------------------------------------------------

func TestModel_ErrorState(t *testing.T) {
	m := newTestModel(t)
	if m.HasError() {
		t.Error("fresh model reports an error")
	}
	m.SetError("boom")
	if !m.HasError() {
		t.Error("SetError did not set error flag")
	}
	if m.GetError() != "boom" {
		t.Errorf("GetError() = %q, want %q", m.GetError(), "boom")
	}
	m.ClearError()
	if m.HasError() {
		t.Error("ClearError did not clear error")
	}
}

// ------------------------------------------------------------------
// Navigation
// ------------------------------------------------------------------

func TestModel_NavigateUp_Down(t *testing.T) {
	m := newTestModel(t)
	m.viewMode = ViewHistory
	// Seed at least 3 history entries.
	base := time.Now()
	for i := 0; i < 3; i++ {
		m.AddCommandResult(runner.CommandResult{Command: "tsc", Passed: true, Timestamp: base.Add(time.Duration(i) * time.Second)})
	}

	// Start at 0, navigate down twice, then up once.
	m.NavigateDown()
	m.NavigateDown()
	if m.selectedRow != 2 {
		t.Errorf("after 2x down: selectedRow = %d, want 2", m.selectedRow)
	}
	m.NavigateDown()
	if m.selectedRow != 2 {
		t.Errorf("down past max: selectedRow = %d, want 2 (clamped)", m.selectedRow)
	}
	m.NavigateUp()
	if m.selectedRow != 1 {
		t.Errorf("after up: selectedRow = %d, want 1", m.selectedRow)
	}
	m.NavigateUp()
	m.NavigateUp()
	if m.selectedRow != 0 {
		t.Errorf("up past 0: selectedRow = %d, want 0 (clamped)", m.selectedRow)
	}
}

func TestGetCommandType_Mapping(t *testing.T) {
	cases := map[string]runner.CommandType{
		"tsc":        runner.TypescriptCheck,
		"typescript": runner.TypescriptCheck,
		"lint":       runner.LintCheck,
		"test":       runner.TestRunner,
		"custom_x":   runner.CommandType("custom_x"),
	}
	for in, want := range cases {
		if got := getCommandType(in); got != want {
			t.Errorf("getCommandType(%q) = %q, want %q", in, got, want)
		}
	}
}

// ------------------------------------------------------------------
// Update / key handling
// ------------------------------------------------------------------

func TestUpdate_Quit(t *testing.T) {
	m := newTestModel(t)
	// 'q'
	mm, cmd := m.Update(tea.KeyMsg{Type: tea.KeyRunes, Runes: []rune{'q'}})
	if cmd == nil {
		t.Error("Update(q) returned nil cmd; want tea.Quit")
	}
	_ = mm
	// ctrl+c
	mm, cmd = m.Update(tea.KeyMsg{Type: tea.KeyCtrlC})
	if cmd == nil {
		t.Error("Update(ctrl+c) returned nil cmd; want tea.Quit")
	}
	_ = mm
}

func TestUpdate_Key_SwitchesViewMode(t *testing.T) {
	m := newTestModel(t)
	cases := map[string]ViewMode{
		"1": ViewMain,
		"2": ViewHistory,
		"3": ViewLogs,
		"h": ViewHelp,
	}
	for key, want := range cases {
		t.Run(key, func(t *testing.T) {
			mm, _ := m.Update(tea.KeyMsg{Type: tea.KeyRunes, Runes: []rune{rune(key[0])}})
			got := mm.(Model)
			if got.viewMode != want {
				t.Errorf("key %q: viewMode = %d, want %d", key, got.viewMode, want)
			}
			if got.selectedRow != 0 {
				t.Errorf("key %q: selectedRow = %d, want 0", key, got.selectedRow)
			}
		})
	}
}

func TestUpdate_Key_Navigation(t *testing.T) {
	m := newTestModel(t)
	m.viewMode = ViewHistory
	base := time.Now()
	for i := 0; i < 3; i++ {
		m.AddCommandResult(runner.CommandResult{Command: "tsc", Passed: true, Timestamp: base.Add(time.Duration(i) * time.Second)})
	}
	mm, _ := m.Update(tea.KeyMsg{Type: tea.KeyDown})
	if mm.(Model).selectedRow != 1 {
		t.Errorf("down: selectedRow = %d, want 1", mm.(Model).selectedRow)
	}
	mm, _ = m.Update(tea.KeyMsg{Type: tea.KeyUp})
	if mm.(Model).selectedRow != 0 {
		t.Errorf("up: selectedRow = %d, want 0", mm.(Model).selectedRow)
	}
	// 'j' / 'k' are vim-style aliases.
	mm, _ = m.Update(tea.KeyMsg{Type: tea.KeyRunes, Runes: []rune("j")})
	if mm.(Model).selectedRow != 1 {
		t.Errorf("j: selectedRow = %d, want 1", mm.(Model).selectedRow)
	}
	mm, _ = m.Update(tea.KeyMsg{Type: tea.KeyRunes, Runes: []rune("k")})
	if mm.(Model).selectedRow != 0 {
		t.Errorf("k: selectedRow = %d, want 0", mm.(Model).selectedRow)
	}
}

func TestUpdate_Esc_ReturnsToMain(t *testing.T) {
	m := newTestModel(t)
	m.viewMode = ViewLogs
	mm, _ := m.Update(tea.KeyMsg{Type: tea.KeyEsc})
	if got := mm.(Model).viewMode; got != ViewMain {
		t.Errorf("esc: viewMode = %d, want ViewMain", got)
	}
	// From main, esc is a no-op.
	mm, _ = m.Update(tea.KeyMsg{Type: tea.KeyEsc})
	if got := mm.(Model).viewMode; got != ViewMain {
		t.Errorf("esc in main: viewMode = %d, want ViewMain", got)
	}
}

func TestUpdate_C_ClearsError(t *testing.T) {
	m := newTestModel(t)
	m.SetError("boom")
	mm, _ := m.Update(tea.KeyMsg{Type: tea.KeyRunes, Runes: []rune{'c'}})
	updated := mm.(Model)
	if updated.HasError() {
		t.Error("'c' should clear error")
	}
	// 'c' with no error is a no-op.
	mm, _ = m.Update(tea.KeyMsg{Type: tea.KeyRunes, Runes: []rune{'c'}})
	updated = mm.(Model)
	if updated.HasError() {
		t.Error("'c' on clean state should not produce an error")
	}
}

func TestUpdate_WindowSizeMsg(t *testing.T) {
	m := newTestModel(t)
	mm, _ := m.Update(tea.WindowSizeMsg{Width: 100, Height: 30})
	got := mm.(Model)
	if got.width != 100 || got.height != 30 {
		t.Errorf("WindowSizeMsg: size = (%d, %d), want (100, 30)", got.width, got.height)
	}
	if !got.ready {
		t.Error("WindowSizeMsg should mark ready=true")
	}
}

func TestUpdate_TickMsg(t *testing.T) {
	m := newTestModel(t)
	mm, _ := m.Update(tickMsg(time.Now()))
	if mm == nil {
		t.Error("tickMsg returned nil model")
	}
}

func TestUpdate_FileChange_TriggersCommandRun(t *testing.T) {
	m := newTestModel(t)
	// runCommandsOnChange returns a tea.Batch; it should be non-nil.
	mm, cmd := m.Update(fileChangeMsg{file: "src/foo.ts", action: "modified"})
	if cmd == nil {
		t.Error("fileChangeMsg: expected non-nil command when not running")
	}
	_ = mm
	// When something is already running, the change is logged but no new
	// command is dispatched.
	m.SetCommandRunning(runner.TypescriptCheck, true)
	mm, cmd = m.Update(fileChangeMsg{file: "src/foo.ts", action: "modified"})
	if cmd != nil {
		t.Error("fileChangeMsg: expected nil command while another is running")
	}
	_ = mm
}

func TestUpdate_RefreshMsg(t *testing.T) {
	m := newTestModel(t)
	m.SetError("boom")
	mm, cmd := m.Update(refreshMsg{})
	updated := mm.(Model)
	if updated.HasError() {
		t.Error("refreshMsg should clear error")
	}
	if cmd == nil {
		t.Error("refreshMsg: expected non-nil runAllCommands cmd")
	}
}

func TestUpdate_StatusUpdateMsg(t *testing.T) {
	m := newTestModel(t)
	mm, _ := m.Update(statusUpdateMsg{watcherActive: true, serverActive: false})
	got := mm.(Model)
	if !got.watcherActive {
		t.Error("statusUpdateMsg: watcherActive not propagated")
	}
	if got.serverActive {
		t.Error("statusUpdateMsg: serverActive should be false")
	}
}

func TestUpdate_ErrorMsg(t *testing.T) {
	m := newTestModel(t)
	mm, _ := m.Update(errorMsg{err: "boom"})
	updated := mm.(Model)
	if !updated.HasError() {
		t.Error("errorMsg: not propagated to model")
	}
}

func TestUpdate_CommandStartMsg(t *testing.T) {
	m := newTestModel(t)
	mm, _ := m.Update(commandStartMsg{cmdType: runner.LintCheck})
	if !mm.(Model).running[runner.LintCheck] {
		t.Error("commandStartMsg: did not mark command as running")
	}
}

func TestUpdate_CommandResultMsg(t *testing.T) {
	m := newTestModel(t)
	mm, _ := m.Update(commandResultMsg{
		result: runner.CommandResult{
			Command: "tsc", Passed: true, Timestamp: time.Now(),
		},
	})
	latest := mm.(Model).history.GetLatest()
	if _, ok := latest[runner.TypescriptCheck]; !ok {
		t.Error("commandResultMsg: result not added to history")
	}
}

func TestUpdate_UnknownMsg_NoOp(t *testing.T) {
	m := newTestModel(t)
	mm, cmd := m.Update(struct{ foo int }{foo: 1})
	if cmd != nil {
		t.Error("unknown message: expected nil cmd")
	}
	if mm == nil {
		t.Error("unknown message: returned nil model")
	}
}

func TestUpdate_HandleEnter_LogsViewClears(t *testing.T) {
	m := newTestModel(t)
	m.viewMode = ViewLogs
	m.AddLog(LogInfo, "hello", "", "")
	mm, _ := m.Update(tea.KeyMsg{Type: tea.KeyEnter})
	// handleEnterKey on ViewLogs: adds a "Logs cleared" log entry, then
	// wipes the slice, so the final state is an empty log list.
	if len(mm.(Model).logs) != 0 {
		t.Errorf("enter in ViewLogs: len(logs) = %d, want 0 (logs cleared)", len(mm.(Model).logs))
	}
}

// ------------------------------------------------------------------
// handleEnterKey in ViewMain triggers runSpecificCommand on the
// selected row.
// ------------------------------------------------------------------

func TestHandleEnter_ViewMain_DispatchesSelectedCommand(t *testing.T) {
	m := newTestModel(t)
	m.viewMode = ViewMain
	m.selectedRow = 0 // TypescriptCheck
	mm, cmd := m.Update(tea.KeyMsg{Type: tea.KeyEnter})
	if cmd == nil {
		t.Error("enter in ViewMain: expected non-nil cmd to run the selected command")
	}
	_ = mm
}

// ------------------------------------------------------------------
// Utility functions exposed by update.go
// ------------------------------------------------------------------

func TestIsValidCommand(t *testing.T) {
	// We can't assert truth values for a real PATH, but we can verify
	// the contract: the function returns the same value as commandExists
	// for each named tool.
	if isValidCommand(runner.TypescriptCheck) != commandExists("npx") {
		t.Error("isValidCommand(TypescriptCheck) disagrees with commandExists(\"npx\")")
	}
	if isValidCommand(runner.LintCheck) != commandExists("npm") {
		t.Error("isValidCommand(LintCheck) disagrees with commandExists(\"npm\")")
	}
	if isValidCommand(runner.TestRunner) != commandExists("npm") {
		t.Error("isValidCommand(TestRunner) disagrees with commandExists(\"npm\")")
	}
	if isValidCommand(runner.CommandType("bogus")) {
		t.Error("isValidCommand(bogus) should be false")
	}
}

func TestCommandExists(t *testing.T) {
	// The "go" tool is essentially guaranteed to be on PATH during
	// `go test`, and there is no "definitely-missing" command.
	if !commandExists("go") {
		t.Error("commandExists(\"go\") = false; want true (go is on PATH)")
	}
	if commandExists("kwatch-totally-not-a-binary-zzz") {
		t.Error("commandExists(<bogus>) = true; want false")
	}
}

func TestParseCommandOutput(t *testing.T) {
	cases := []struct {
		name    string
		cmdType runner.CommandType
		output  string
		minCnt  int
		summary string
	}{
		{"typescript", runner.TypescriptCheck, "src/a.ts(1,1): error TS2304: Cannot find name 'foo'.\n", 1, "1 errors"},
		{"lint", runner.LintCheck, "1 problem (1 error, 0 warnings)\n", 1, "1 problems"},
		{"test", runner.TestRunner, "PASS src/foo.test.ts\nFAIL src/bar.test.ts\n", 2, "2 tests"},
		{"unknown", runner.CommandType("custom"), "anything", 0, "Unknown"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			cnt, summary := parseCommandOutput(tc.cmdType, tc.output)
			if cnt < tc.minCnt {
				t.Errorf("count = %d, want >= %d", cnt, tc.minCnt)
			}
			if summary != tc.summary {
				t.Errorf("summary = %q, want %q", summary, tc.summary)
			}
		})
	}
}

func TestFormatError(t *testing.T) {
	if formatError(nil) != "" {
		t.Errorf("formatError(nil) should be empty")
	}
	short := formatError(errSentinel("boom"))
	if short != "boom" {
		t.Errorf("formatError(short) = %q, want %q", short, "boom")
	}
	// Long errors are truncated to 100 chars (97 + "...").
	long := formatError(errSentinel(strings.Repeat("x", 200)))
	if !strings.HasSuffix(long, "...") {
		t.Errorf("formatError(long) should end with '...'; got %q", long)
	}
	if len(long) != 100 {
		t.Errorf("formatError(long) length = %d, want 100", len(long))
	}
}

type errSentinel string

func (e errSentinel) Error() string { return string(e) }

func TestFileWatchCmd_IsTeaCmd(t *testing.T) {
	cmd := fileWatchCmd(t.TempDir())
	if cmd == nil {
		t.Error("fileWatchCmd returned nil")
	}
}

func TestRefreshCmd_IsTeaCmd(t *testing.T) {
	cmd := refreshCmd()
	if cmd == nil {
		t.Error("refreshCmd returned nil")
	}
}

// ------------------------------------------------------------------
// View rendering
// ------------------------------------------------------------------

func TestView_NotReady(t *testing.T) {
	m := newTestModel(t)
	if got := m.View(); got != "Loading..." {
		t.Errorf("View() before ready = %q, want %q", got, "Loading...")
	}
}

func TestView_AllModes_NonEmpty(t *testing.T) {
	m := newTestModel(t)
	m.UpdateSize(120, 40)
	modes := []ViewMode{ViewMain, ViewHistory, ViewLogs, ViewHelp}
	for _, mode := range modes {
		t.Run(modeName(mode), func(t *testing.T) {
			mm := m
			mm.viewMode = mode
			if got := mm.View(); got == "" {
				t.Errorf("View() in %s returned empty string", modeName(mode))
			}
		})
	}
}

func modeName(m ViewMode) string {
	switch m {
	case ViewMain:
		return "ViewMain"
	case ViewHistory:
		return "ViewHistory"
	case ViewLogs:
		return "ViewLogs"
	case ViewHelp:
		return "ViewHelp"
	default:
		return "Unknown"
	}
}

func TestView_MainView_ShowsExpected(t *testing.T) {
	m := newTestModel(t)
	m.UpdateSize(120, 40)
	view := m.View()
	// CommandType constants are "typescript", "lint", "test" — those are
	// the strings the main view table renders for each command row.
	for _, want := range []string{"KWatch", "typescript", "lint", "test"} {
		if !strings.Contains(view, want) {
			t.Errorf("View() missing %q", want)
		}
	}
}

func TestView_Help_ContainsKeys(t *testing.T) {
	m := newTestModel(t)
	m.UpdateSize(120, 40)
	m.viewMode = ViewHelp
	view := m.View()
	for _, want := range []string{"KWATCH", "Quit", "Refresh", "Help", "Main", "History", "Logs"} {
		if !strings.Contains(view, want) {
			t.Errorf("help view missing %q", want)
		}
	}
}

func TestView_Logs_EmptyMessage(t *testing.T) {
	m := newTestModel(t)
	m.UpdateSize(120, 40)
	m.viewMode = ViewLogs
	view := m.View()
	if !strings.Contains(view, "No logs available") {
		t.Error("empty logs view should show empty-state message")
	}
}

func TestView_History_EmptyMessage(t *testing.T) {
	m := newTestModel(t)
	m.UpdateSize(120, 40)
	m.viewMode = ViewHistory
	view := m.View()
	if !strings.Contains(view, "No command history") {
		t.Error("empty history view should show empty-state message")
	}
}

func TestView_Logs_Populated(t *testing.T) {
	m := newTestModel(t)
	m.UpdateSize(120, 40)
	m.AddLog(LogInfo, "hello", "src/foo.ts", "modified")
	m.AddLog(LogError, "boom", "", "error")
	m.viewMode = ViewLogs
	view := m.View()
	if !strings.Contains(view, "hello") {
		t.Error("logs view missing 'hello'")
	}
	if !strings.Contains(view, "boom") {
		t.Error("logs view missing 'boom'")
	}
}

func TestView_DefaultModeFallsThroughToMain(t *testing.T) {
	m := newTestModel(t)
	m.UpdateSize(120, 40)
	m.viewMode = ViewMode(999) // bogus mode
	view := m.View()
	if !strings.Contains(view, "KWatch") {
		t.Error("unknown view mode should fall back to main view")
	}
}

// ------------------------------------------------------------------
// Style helpers
//
// lipgloss.Style contains function-typed fields and is not directly
// comparable with ==, so we render a small payload through both the
// helper and the named style and assert that the bytes match.
// ------------------------------------------------------------------

func TestGetStatusStyle(t *testing.T) {
	const sample = "X"
	cases := []struct {
		desc string
		got  lipgloss.Style
		want lipgloss.Style
	}{
		{"passed+not running", GetStatusStyle(true, false), statusPassStyle},
		{"failed+not running", GetStatusStyle(false, false), statusFailStyle},
		{"running (failed)", GetStatusStyle(false, true), statusRunningStyle},
		{"running overrides passed", GetStatusStyle(true, true), statusRunningStyle},
	}
	for _, tc := range cases {
		t.Run(tc.desc, func(t *testing.T) {
			if tc.got.Render(sample) != tc.want.Render(sample) {
				t.Errorf("GetStatusStyle rendered output mismatch: %q vs %q",
					tc.got.Render(sample), tc.want.Render(sample))
			}
		})
	}
}

func TestGetStatusIcon(t *testing.T) {
	if got := GetStatusIcon(true, false); got != "✓" {
		t.Errorf("GetStatusIcon(passed,running=false) = %q, want %q", got, "✓")
	}
	if got := GetStatusIcon(false, false); got != "✗" {
		t.Errorf("GetStatusIcon(failed,running=false) = %q, want %q", got, "✗")
	}
	if got := GetStatusIcon(false, true); got != "⟳" {
		t.Errorf("GetStatusIcon(running) = %q, want %q", got, "⟳")
	}
}

func TestGetCommandStyle(t *testing.T) {
	const sample = "X"
	cases := []struct {
		input string
		want  lipgloss.Style
	}{
		{"typescript", commandTSCStyle},
		{"lint", commandLintStyle},
		{"test", commandTestStyle},
		{"custom", normalTextStyle},
	}
	for _, tc := range cases {
		t.Run(tc.input, func(t *testing.T) {
			if got := GetCommandStyle(tc.input).Render(sample); got != tc.want.Render(sample) {
				t.Errorf("GetCommandStyle(%q) rendered output mismatch: %q vs %q",
					tc.input, got, tc.want.Render(sample))
			}
		})
	}
}

func TestFormatDuration(t *testing.T) {
	cases := []struct {
		ms       int64
		contains string
	}{
		{0, "0ms"},
		{500, "ms"},
		{1500, "1.5s"},
		{20000, "20.0s"},
	}
	for _, tc := range cases {
		got := FormatDuration(tc.ms)
		if !strings.Contains(got, tc.contains) {
			t.Errorf("FormatDuration(%d) = %q, want substring %q", tc.ms, got, tc.contains)
		}
	}
}

func TestTruncate(t *testing.T) {
	if got := Truncate("hello", 10); got != "hello" {
		t.Errorf("Truncate(short) = %q, want %q", got, "hello")
	}
	if got := Truncate("hello world", 8); got != "hello..." {
		t.Errorf("Truncate(long) = %q, want %q", got, "hello...")
	}
	if got := Truncate("hello", 3); got != "hel" {
		t.Errorf("Truncate(width<=3) = %q, want %q", got, "hel")
	}
	if got := Truncate("hi", 0); got != "" {
		t.Errorf("Truncate(width=0) = %q, want %q", got, "")
	}
}

func TestCenter(t *testing.T) {
	got := Center("hi", 10)
	if got == "" {
		t.Error("Center should not be empty")
	}
}

// ------------------------------------------------------------------
// TUI: NewTUI / setupLogging
// ------------------------------------------------------------------

func TestNewTUI_NonExistentDir(t *testing.T) {
	if _, err := NewTUI(filepath.Join(t.TempDir(), "no-such-dir")); err == nil {
		t.Error("NewTUI(nonexistent) should error")
	}
}

func TestNewTUI_OK(t *testing.T) {
	dir := t.TempDir()
	tui, err := NewTUI(dir)
	if err != nil {
		t.Fatalf("NewTUI: %v", err)
	}
	if tui == nil {
		t.Fatal("NewTUI returned nil")
	}
	if tui.watchDir == "" {
		t.Error("watchDir should be populated")
	}
	if tui.watcher == nil {
		t.Error("watcher should be set")
	}
	if tui.logFile == nil {
		t.Error("logFile should be set")
	}
	if tui.model.watchDir == "" {
		t.Error("model.watchDir should be populated")
	}
	t.Cleanup(func() {
		if err := tui.Stop(); err != nil {
			t.Errorf("Stop: %v", err)
		}
	})
}

func TestTUI_Stop_ClosesResources(t *testing.T) {
	dir := t.TempDir()
	tui, err := NewTUI(dir)
	if err != nil {
		t.Fatalf("NewTUI: %v", err)
	}
	if err := tui.Stop(); err != nil {
		t.Errorf("Stop: %v", err)
	}
}

// ------------------------------------------------------------------
// File-watcher helpers
// ------------------------------------------------------------------

func TestTUI_IsRelevantFile(t *testing.T) {
	dir := t.TempDir()
	tui, err := NewTUI(dir)
	if err != nil {
		t.Fatalf("NewTUI: %v", err)
	}
	t.Cleanup(func() { _ = tui.Stop() })

	cases := []struct {
		path   string
		want   bool
		reason string
	}{
		{"src/foo.ts", true, "typescript file"},
		{"src/foo.tsx", true, "tsx file"},
		{"package.json", true, "config file"},
		{"tsconfig.json", true, "config file"},
		{"src/foo.css", true, "css file"},
		{"src/foo.html", true, "html file"},

		{".kwatch/kwatch.log", false, "ignored directory"},
		{"node_modules/foo.ts", false, "ignored directory"},
		{"dist/foo.js", false, "ignored directory"},
		{"build/foo.js", false, "ignored directory"},
		{".git/HEAD", false, "ignored directory"},
		{".hidden/foo.ts", false, "hidden file"},
		{"src/foo.log", false, "ignored extension"},
		{"src/foo.tmp", false, "ignored extension"},
		{"src/foo.swp", false, "vim swap"},
		{"src/foo.py", false, "not a watched extension"},
		{"src/foo", false, "no extension, not a known config name"},
	}
	for _, tc := range cases {
		t.Run(tc.path, func(t *testing.T) {
			if got := tui.isRelevantFile(filepath.Join(dir, tc.path)); got != tc.want {
				t.Errorf("isRelevantFile(%q) = %v, want %v (%s)", tc.path, got, tc.want, tc.reason)
			}
		})
	}
}

func TestTUI_GetFileAction(t *testing.T) {
	tui := &TUI{}
	cases := map[fsnotify.Op]string{
		fsnotify.Create: "created",
		fsnotify.Write:  "modified",
		fsnotify.Remove: "deleted",
		fsnotify.Rename: "renamed",
		fsnotify.Chmod:  "chmod",
		fsnotify.Op(0):  "changed",
	}
	for op, want := range cases {
		if got := tui.getFileAction(op); got != want {
			t.Errorf("getFileAction(%v) = %q, want %q", op, got, want)
		}
	}
}

func TestTUI_ShouldIgnoreEvent(t *testing.T) {
	tui := &TUI{}
	if !tui.shouldIgnoreEvent(fsnotify.Chmod) {
		t.Error("Chmod should be ignored")
	}
	if tui.shouldIgnoreEvent(fsnotify.Write) {
		t.Error("Write should not be ignored")
	}
}

// ------------------------------------------------------------------
// DefaultConfig / StartTUI / RunWithConfig behavior
// ------------------------------------------------------------------

func TestDefaultConfig_Defaults(t *testing.T) {
	c := DefaultConfig("/foo")
	if c.WatchDir != "/foo" {
		t.Errorf("WatchDir = %q, want %q", c.WatchDir, "/foo")
	}
	if c.ServerPort != 8080 {
		t.Errorf("ServerPort = %d, want 8080", c.ServerPort)
	}
	if c.MaxLogs != 1000 {
		t.Errorf("MaxLogs = %d, want 1000", c.MaxLogs)
	}
	if c.LogLevel != "info" {
		t.Errorf("LogLevel = %q, want %q", c.LogLevel, "info")
	}
}

// ------------------------------------------------------------------
// Sanity: ensure runner types are importable and Config package is reachable
// from the tui test scope.
// ------------------------------------------------------------------

func TestImportedPackagesUsable(t *testing.T) {
	if (config.DefaultConfig()) == nil {
		t.Error("config.DefaultConfig() returned nil")
	}
	if os.Getenv("PATH") == "" {
		t.Error("PATH is empty in test env")
	}
}
