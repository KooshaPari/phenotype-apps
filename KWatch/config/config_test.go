package config

import (
	"os"
	"path/filepath"
	"testing"
	"time"
)

// =============================================================================
// Existing tests (kept for backward compatibility)
// =============================================================================

func TestDefaultConfig(t *testing.T) {
	cfg := DefaultConfig()

	if cfg == nil {
		t.Fatal("DefaultConfig() returned nil")
	}

	if cfg.DefaultTimeout != "30s" {
		t.Errorf("DefaultConfig().DefaultTimeout = %q, want %q", cfg.DefaultTimeout, "30s")
	}

	if cfg.MaxParallel != 3 {
		t.Errorf("DefaultConfig().MaxParallel = %d, want 3", cfg.MaxParallel)
	}

	if len(cfg.Commands) == 0 {
		t.Error("DefaultConfig() returned config with no commands")
	}

	// Verify expected default commands are present
	required := []string{"typescript", "lint", "test"}
	for _, name := range required {
		cmd, ok := cfg.Commands[name]
		if !ok {
			t.Errorf("DefaultConfig() missing required command %q", name)
			continue
		}
		if !cmd.Enabled {
			t.Errorf("DefaultConfig() command %q should be enabled", name)
		}
		if cmd.Command == "" {
			t.Errorf("DefaultConfig() command %q has empty Command", name)
		}
		if cmd.Timeout == "" {
			t.Errorf("DefaultConfig() command %q has empty Timeout", name)
		}
	}
}

func TestConfigLoad_NoConfigFile(t *testing.T) {
	dir := t.TempDir()

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() returned unexpected error: %v", err)
	}
	if cfg == nil {
		t.Fatal("Load() returned nil config")
	}

	if cfg.MaxParallel != 3 {
		t.Errorf("Load() with no file: MaxParallel = %d, want 3", cfg.MaxParallel)
	}
	if cfg.DefaultTimeout != "30s" {
		t.Errorf("Load() with no file: DefaultTimeout = %q, want %q", cfg.DefaultTimeout, "30s")
	}
}

func TestConfigLoad_InvalidYAML(t *testing.T) {
	dir := t.TempDir()
	kwatchDir := filepath.Join(dir, ".kwatch")
	if err := os.MkdirAll(kwatchDir, 0755); err != nil {
		t.Fatalf("failed to create .kwatch dir: %v", err)
	}

	invalidYAML := "commands: [invalid: yaml: here"
	if err := os.WriteFile(filepath.Join(kwatchDir, "kwatch.yaml"), []byte(invalidYAML), 0644); err != nil {
		t.Fatalf("failed to write invalid yaml: %v", err)
	}

	// With unified config, invalid YAML should fall back to defaults
	unifiedCfg, err := LoadKWatchConfig(dir)
	if err != nil {
		t.Fatalf("LoadKWatchConfig() with invalid YAML should fall back: %v", err)
	}
	if unifiedCfg == nil {
		t.Fatal("LoadKWatchConfig() returned nil")
	}
	// Should use defaults
	if unifiedCfg.Server.Port != 3737 {
		t.Errorf("default server port = %d, want 3737", unifiedCfg.Server.Port)
	}
}

func TestConfigLoad_ValidFile(t *testing.T) {
	dir := t.TempDir()
	kwatchDir := filepath.Join(dir, ".kwatch")
	if err := os.MkdirAll(kwatchDir, 0755); err != nil {
		t.Fatalf("failed to create .kwatch dir: %v", err)
	}

	yamlContent := `defaultTimeout: 45s
maxParallel: 5
commands:
  typescript:
    command: npx
    args: [tsc, --noEmit]
    timeout: 60s
    enabled: true
  custom:
    command: ./script.sh
    args: []
    timeout: 10s
    enabled: false
`
	if err := os.WriteFile(filepath.Join(kwatchDir, "kwatch.yaml"), []byte(yamlContent), 0644); err != nil {
		t.Fatalf("failed to write yaml: %v", err)
	}

	cfg, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() returned error: %v", err)
	}

	if cfg.DefaultTimeout != "45s" {
		t.Errorf("DefaultTimeout = %q, want 45s", cfg.DefaultTimeout)
	}
	if cfg.MaxParallel != 5 {
		t.Errorf("MaxParallel = %d, want 5", cfg.MaxParallel)
	}
	if len(cfg.Commands) != 2 {
		t.Errorf("got %d commands, want 2", len(cfg.Commands))
	}
	ts := cfg.Commands["typescript"]
	if !ts.Enabled {
		t.Error("typescript command should be enabled")
	}
	if ts.Args[0] != "tsc" {
		t.Errorf("typescript args[0] = %q, want tsc", ts.Args[0])
	}
}

func TestConfigLoad_UnreadableFile(t *testing.T) {
	if os.Getuid() == 0 {
		t.Skip("running as root; cannot test permission denial")
	}
	dir := t.TempDir()
	kwatchDir := filepath.Join(dir, ".kwatch")
	if err := os.MkdirAll(kwatchDir, 0755); err != nil {
		t.Fatalf("failed to create .kwatch dir: %v", err)
	}
	configPath := filepath.Join(kwatchDir, "kwatch.yaml")
	if err := os.WriteFile(configPath, []byte("defaultTimeout: 30s\nmaxParallel: 1\ncommands: {}\n"), 0000); err != nil {
		t.Fatalf("failed to write yaml: %v", err)
	}
	t.Cleanup(func() { _ = os.Chmod(configPath, 0644) })

	_, err := Load(dir)
	if err == nil {
		t.Error("Load() with unreadable file: expected error, got nil")
	}
}

func TestConfigSave(t *testing.T) {
	dir := t.TempDir()

	cfg := DefaultConfig()
	cfg.DefaultTimeout = "1m"
	cfg.MaxParallel = 7

	if err := cfg.Save(dir); err != nil {
		t.Fatalf("Save() returned error: %v", err)
	}

	configPath := filepath.Join(dir, ".kwatch", "kwatch.yaml")
	if _, err := os.Stat(configPath); os.IsNotExist(err) {
		t.Fatal("Save() did not create config file")
	}

	loaded, err := Load(dir)
	if err != nil {
		t.Fatalf("Load() after Save() returned error: %v", err)
	}
	if loaded.DefaultTimeout != "1m" {
		t.Errorf("after save/reload: DefaultTimeout = %q, want 1m", loaded.DefaultTimeout)
	}
	if loaded.MaxParallel != 7 {
		t.Errorf("after save/reload: MaxParallel = %d, want 7", loaded.MaxParallel)
	}
}

func TestConfigValidate(t *testing.T) {
	tests := []struct {
		name    string
		cfg     *Config
		wantErr bool
	}{
		{
			name: "valid default config",
			cfg:  DefaultConfig(),
		},
		{
			name: "invalid default timeout",
			cfg: &Config{
				DefaultTimeout: "not-a-duration",
				MaxParallel:    1,
				Commands:       map[string]Command{},
			},
			wantErr: true,
		},
		{
			name: "zero max parallel",
			cfg: &Config{
				DefaultTimeout: "30s",
				MaxParallel:    0,
				Commands:       map[string]Command{},
			},
			wantErr: true,
		},
		{
			name: "negative max parallel",
			cfg: &Config{
				DefaultTimeout: "30s",
				MaxParallel:    -1,
				Commands:       map[string]Command{},
			},
			wantErr: true,
		},
		{
			name: "command with empty Command field",
			cfg: &Config{
				DefaultTimeout: "30s",
				MaxParallel:    1,
				Commands: map[string]Command{
					"bad": {Command: "", Args: nil, Timeout: "1s", Enabled: true},
				},
			},
			wantErr: true,
		},
		{
			name: "command with invalid timeout",
			cfg: &Config{
				DefaultTimeout: "30s",
				MaxParallel:    1,
				Commands: map[string]Command{
					"bad": {Command: "echo", Args: []string{"hi"}, Timeout: "forever", Enabled: true},
				},
			},
			wantErr: true,
		},
		{
			name: "command with empty timeout is allowed",
			cfg: &Config{
				DefaultTimeout: "30s",
				MaxParallel:    1,
				Commands: map[string]Command{
					"ok": {Command: "echo", Args: []string{"hi"}, Timeout: "", Enabled: true},
				},
			},
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.cfg.Validate()
			if (err != nil) != tt.wantErr {
				t.Errorf("Validate() error = %v, wantErr = %v", err, tt.wantErr)
			}
		})
	}
}

func TestConfigGetTimeout(t *testing.T) {
	cfg := &Config{
		DefaultTimeout: "30s",
		MaxParallel:    1,
		Commands: map[string]Command{
			"custom":  {Command: "x", Timeout: "5s"},
			"inherit": {Command: "y", Timeout: ""},
			"broken":  {Command: "z", Timeout: "bogus"},
		},
	}

	tests := []struct {
		name    string
		cmdName string
		want    time.Duration
	}{
		{"command-specific timeout", "custom", 5 * time.Second},
		{"falls back to default", "inherit", 30 * time.Second},
		{"falls back when command timeout is invalid", "broken", 30 * time.Second},
		{"unknown command uses default", "unknown", 30 * time.Second},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := cfg.GetTimeout(tt.cmdName)
			if got != tt.want {
				t.Errorf("GetTimeout(%q) = %v, want %v", tt.cmdName, got, tt.want)
			}
		})
	}
}

func TestConfigGetTimeout_InvalidDefault(t *testing.T) {
	cfg := &Config{
		DefaultTimeout: "not-a-duration",
		MaxParallel:    1,
		Commands:       map[string]Command{},
	}

	got := cfg.GetTimeout("anything")
	if got != 30*time.Second {
		t.Errorf("GetTimeout with invalid default = %v, want 30s", got)
	}
}

func TestConfigGetEnabledCommands(t *testing.T) {
	cfg := &Config{
		DefaultTimeout: "30s",
		MaxParallel:    1,
		Commands: map[string]Command{
			"a": {Command: "x", Enabled: true},
			"b": {Command: "y", Enabled: false},
			"c": {Command: "z", Enabled: true},
		},
	}

	enabled := cfg.GetEnabledCommands()
	if len(enabled) != 2 {
		t.Errorf("GetEnabledCommands() returned %d, want 2", len(enabled))
	}
	if _, ok := enabled["a"]; !ok {
		t.Error("GetEnabledCommands() missing 'a'")
	}
	if _, ok := enabled["b"]; ok {
		t.Error("GetEnabledCommands() should not include disabled 'b'")
	}
	if _, ok := enabled["c"]; !ok {
		t.Error("GetEnabledCommands() missing 'c'")
	}
}

func TestConfigGetEnabledCommands_Empty(t *testing.T) {
	cfg := &Config{Commands: map[string]Command{}}
	enabled := cfg.GetEnabledCommands()
	if len(enabled) != 0 {
		t.Errorf("GetEnabledCommands() on empty config = %d, want 0", len(enabled))
	}
}

func TestConfigExists(t *testing.T) {
	dir := t.TempDir()

	if ConfigExists(dir) {
		t.Error("ConfigExists() = true for empty dir, want false")
	}

	kwatchDir := filepath.Join(dir, ".kwatch")
	if err := os.MkdirAll(kwatchDir, 0755); err != nil {
		t.Fatalf("failed to create .kwatch dir: %v", err)
	}
	configPath := filepath.Join(kwatchDir, "kwatch.yaml")
	if err := os.WriteFile(configPath, []byte("defaultTimeout: 30s\nmaxParallel: 1\ncommands: {}\n"), 0644); err != nil {
		t.Fatalf("failed to write yaml: %v", err)
	}

	if !ConfigExists(dir) {
		t.Error("ConfigExists() = false after writing config, want true")
	}
}

// =============================================================================
// NEW TESTS: Unified KWatchConfig
// =============================================================================

// TestDefaultKWatchConfig verifies the default unified config values.
func TestDefaultKWatchConfig(t *testing.T) {
	cfg := DefaultKWatchConfig()

	if cfg == nil {
		t.Fatal("DefaultKWatchConfig() returned nil")
	}

	// Server defaults
	if cfg.Server.Host != "localhost" {
		t.Errorf("Server.Host = %q, want %q", cfg.Server.Host, "localhost")
	}
	if cfg.Server.Port != 3737 {
		t.Errorf("Server.Port = %d, want %d", cfg.Server.Port, 3737)
	}
	if cfg.Server.ReadTimeout != 10*time.Second {
		t.Errorf("Server.ReadTimeout = %v, want 10s", cfg.Server.ReadTimeout)
	}
	if cfg.Server.WriteTimeout != 10*time.Second {
		t.Errorf("Server.WriteTimeout = %v, want 10s", cfg.Server.WriteTimeout)
	}
	if cfg.Server.IdleTimeout != 60*time.Second {
		t.Errorf("Server.IdleTimeout = %v, want 60s", cfg.Server.IdleTimeout)
	}
	if !cfg.Server.EnableCORS {
		t.Error("Server.EnableCORS should be true by default")
	}
	if len(cfg.Server.AllowedOrigins) != 1 || cfg.Server.AllowedOrigins[0] != "*" {
		t.Errorf("Server.AllowedOrigins = %v, want [\"*\"]", cfg.Server.AllowedOrigins)
	}
	if cfg.Server.WorkingDir != "." {
		t.Errorf("Server.WorkingDir = %q, want \".\"", cfg.Server.WorkingDir)
	}
	if cfg.Server.ShutdownTimeout != 5*time.Second {
		t.Errorf("Server.ShutdownTimeout = %v, want 5s", cfg.Server.ShutdownTimeout)
	}

	// Runner defaults
	if cfg.Runner.DefaultTimeout != 30*time.Second {
		t.Errorf("Runner.DefaultTimeout = %v, want 30s", cfg.Runner.DefaultTimeout)
	}
	if cfg.Runner.MaxParallel != 3 {
		t.Errorf("Runner.MaxParallel = %d, want 3", cfg.Runner.MaxParallel)
	}
	if cfg.Runner.WorkingDir != "." {
		t.Errorf("Runner.WorkingDir = %q, want \".\"", cfg.Runner.WorkingDir)
	}
	if cfg.Runner.HistoryLimit != 1000 {
		t.Errorf("Runner.HistoryLimit = %d, want 1000", cfg.Runner.HistoryLimit)
	}

	// Security defaults
	if cfg.Security.MaxFileSize != 10*1024*1024 {
		t.Errorf("Security.MaxFileSize = %d, want 10MB", cfg.Security.MaxFileSize)
	}
	if cfg.Security.ContextLines != 3 {
		t.Errorf("Security.ContextLines = %d, want 3", cfg.Security.ContextLines)
	}
	if cfg.Security.MaxHistoryDepth != 100 {
		t.Errorf("Security.MaxHistoryDepth = %d, want 100", cfg.Security.MaxHistoryDepth)
	}
	if cfg.Security.DefaultScanMode != "risky" {
		t.Errorf("Security.DefaultScanMode = %q, want \"risky\"", cfg.Security.DefaultScanMode)
	}
	if !cfg.Security.RespectGitignore {
		t.Error("Security.RespectGitignore should be true by default")
	}

	// TUI defaults
	if cfg.TUI.MaxLogs != 1000 {
		t.Errorf("TUI.MaxLogs = %d, want 1000", cfg.TUI.MaxLogs)
	}
	if cfg.TUI.LogLevel != "info" {
		t.Errorf("TUI.LogLevel = %q, want \"info\"", cfg.TUI.LogLevel)
	}
	if cfg.TUI.ServerPort != 8080 {
		t.Errorf("TUI.ServerPort = %d, want 8080", cfg.TUI.ServerPort)
	}
	if cfg.TUI.DebounceDelay != 2*time.Second {
		t.Errorf("TUI.DebounceDelay = %v, want 2s", cfg.TUI.DebounceDelay)
	}
	if cfg.TUI.InitDelay != 500*time.Millisecond {
		t.Errorf("TUI.InitDelay = %v, want 500ms", cfg.TUI.InitDelay)
	}
	if cfg.TUI.MaxLogDisplay != 50 {
		t.Errorf("TUI.MaxLogDisplay = %d, want 50", cfg.TUI.MaxLogDisplay)
	}
	if cfg.TUI.MinWidth != 80 {
		t.Errorf("TUI.MinWidth = %d, want 80", cfg.TUI.MinWidth)
	}
	if cfg.TUI.MinHeight != 24 {
		t.Errorf("TUI.MinHeight = %d, want 24", cfg.TUI.MinHeight)
	}
	if cfg.TUI.TickInterval != 2*time.Second {
		t.Errorf("TUI.TickInterval = %v, want 2s", cfg.TUI.TickInterval)
	}

	// MCP defaults
	if cfg.MCP.DefaultTimeout != 15*time.Second {
		t.Errorf("MCP.DefaultTimeout = %v, want 15s", cfg.MCP.DefaultTimeout)
	}
	if cfg.MCP.BuildStatusTimeout != 12*time.Second {
		t.Errorf("MCP.BuildStatusTimeout = %v, want 12s", cfg.MCP.BuildStatusTimeout)
	}
	if cfg.MCP.RunCommandsTimeout != 18*time.Second {
		t.Errorf("MCP.RunCommandsTimeout = %v, want 18s", cfg.MCP.RunCommandsTimeout)
	}
	if cfg.MCP.HistoryLimit != 10 {
		t.Errorf("MCP.HistoryLimit = %d, want 10", cfg.MCP.HistoryLimit)
	}
}

// TestLoadKWatchConfig_UnifiedYAML tests loading a unified YAML config file.
func TestLoadKWatchConfig_UnifiedYAML(t *testing.T) {
	dir := t.TempDir()
	kwatchDir := filepath.Join(dir, ".kwatch")
	if err := os.MkdirAll(kwatchDir, 0755); err != nil {
		t.Fatalf("failed to create .kwatch dir: %v", err)
	}

	unifiedYAML := `
server:
  host: "0.0.0.0"
  port: 9090
  read_timeout: 30s
  write_timeout: 15s
  idle_timeout: 120s
  enable_cors: false
  working_dir: "/projects/myapp"
  security_db_path: "/data/security.json"
  shutdown_timeout: 10s

runner:
  default_timeout: 60s
  max_parallel: 5
  working_dir: "/projects/myapp"
  history_limit: 500

security:
  max_file_size: 20971520
  context_lines: 5
  excluded_paths: ["node_modules", ".git", ".aws"]
  excluded_files: ["*.log", "*.secret"]
  enabled_severity: ["critical", "high"]
  historical_scan: true
  max_history_depth: 50
  respect_gitignore: false
  default_scan_mode: "comprehensive"
  database_file: "/data/security-findings.json"

tui:
  max_logs: 500
  log_level: "debug"
  server_port: 3000
  debounce_delay: 1s
  init_delay: 200ms
  max_log_display: 100
  min_width: 100
  min_height: 30
  tick_interval: 1s

mcp:
  default_timeout: 30s
  build_status_timeout: 25s
  run_commands_timeout: 35s
  history_limit: 25
`

	if err := os.WriteFile(filepath.Join(kwatchDir, "kwatch.yaml"), []byte(unifiedYAML), 0644); err != nil {
		t.Fatalf("failed to write unified yaml: %v", err)
	}

	cfg, err := LoadKWatchConfig(dir)
	if err != nil {
		t.Fatalf("LoadKWatchConfig() returned error: %v", err)
	}
	if cfg == nil {
		t.Fatal("LoadKWatchConfig() returned nil")
	}

	// Server
	if cfg.Server.Host != "0.0.0.0" {
		t.Errorf("Server.Host = %q, want %q", cfg.Server.Host, "0.0.0.0")
	}
	if cfg.Server.Port != 9090 {
		t.Errorf("Server.Port = %d, want 9090", cfg.Server.Port)
	}
	if cfg.Server.ReadTimeout != 30*time.Second {
		t.Errorf("Server.ReadTimeout = %v, want 30s", cfg.Server.ReadTimeout)
	}
	if cfg.Server.WriteTimeout != 15*time.Second {
		t.Errorf("Server.WriteTimeout = %v, want 15s", cfg.Server.WriteTimeout)
	}
	if cfg.Server.IdleTimeout != 120*time.Second {
		t.Errorf("Server.IdleTimeout = %v, want 120s", cfg.Server.IdleTimeout)
	}
	if cfg.Server.ReadTimeout != 30*time.Second {
		t.Errorf("Server.ReadTimeout = %v, want 30s", cfg.Server.ReadTimeout)
	}
	if cfg.Server.EnableCORS {
		t.Error("Server.EnableCORS should be false")
	}
	if cfg.Server.WorkingDir != "/projects/myapp" {
		t.Errorf("Server.WorkingDir = %q, want \"/projects/myapp\"", cfg.Server.WorkingDir)
	}
	if cfg.Server.ShutdownTimeout != 10*time.Second {
		t.Errorf("Server.ShutdownTimeout = %v, want 10s", cfg.Server.ShutdownTimeout)
	}

	// Runner
	if cfg.Runner.DefaultTimeout != 60*time.Second {
		t.Errorf("Runner.DefaultTimeout = %v, want 60s", cfg.Runner.DefaultTimeout)
	}
	if cfg.Runner.MaxParallel != 5 {
		t.Errorf("Runner.MaxParallel = %d, want 5", cfg.Runner.MaxParallel)
	}
	if cfg.Runner.HistoryLimit != 500 {
		t.Errorf("Runner.HistoryLimit = %d, want 500", cfg.Runner.HistoryLimit)
	}

	// Security
	if cfg.Security.MaxFileSize != 20971520 {
		t.Errorf("Security.MaxFileSize = %d, want 20971520", cfg.Security.MaxFileSize)
	}
	if cfg.Security.ContextLines != 5 {
		t.Errorf("Security.ContextLines = %d, want 5", cfg.Security.ContextLines)
	}
	if cfg.Security.MaxHistoryDepth != 50 {
		t.Errorf("Security.MaxHistoryDepth = %d, want 50", cfg.Security.MaxHistoryDepth)
	}
	if cfg.Security.RespectGitignore {
		t.Error("Security.RespectGitignore should be false")
	}
	if cfg.Security.DefaultScanMode != "comprehensive" {
		t.Errorf("Security.DefaultScanMode = %q, want \"comprehensive\"", cfg.Security.DefaultScanMode)
	}

	// TUI
	if cfg.TUI.MaxLogs != 500 {
		t.Errorf("TUI.MaxLogs = %d, want 500", cfg.TUI.MaxLogs)
	}
	if cfg.TUI.LogLevel != "debug" {
		t.Errorf("TUI.LogLevel = %q, want \"debug\"", cfg.TUI.LogLevel)
	}
	if cfg.TUI.ServerPort != 3000 {
		t.Errorf("TUI.ServerPort = %d, want 3000", cfg.TUI.ServerPort)
	}
	if cfg.TUI.DebounceDelay != 1*time.Second {
		t.Errorf("TUI.DebounceDelay = %v, want 1s", cfg.TUI.DebounceDelay)
	}
	if cfg.TUI.InitDelay != 200*time.Millisecond {
		t.Errorf("TUI.InitDelay = %v, want 200ms", cfg.TUI.InitDelay)
	}

	// MCP
	if cfg.MCP.DefaultTimeout != 30*time.Second {
		t.Errorf("MCP.DefaultTimeout = %v, want 30s", cfg.MCP.DefaultTimeout)
	}
	if cfg.MCP.BuildStatusTimeout != 25*time.Second {
		t.Errorf("MCP.BuildStatusTimeout = %v, want 25s", cfg.MCP.BuildStatusTimeout)
	}
	if cfg.MCP.RunCommandsTimeout != 35*time.Second {
		t.Errorf("MCP.RunCommandsTimeout = %v, want 35s", cfg.MCP.RunCommandsTimeout)
	}
	if cfg.MCP.HistoryLimit != 25 {
		t.Errorf("MCP.HistoryLimit = %d, want 25", cfg.MCP.HistoryLimit)
	}
}

// TestLoadKWatchConfig_EnvOverrides tests that environment variables correctly
// override config file values.
func TestLoadKWatchConfig_EnvOverrides(t *testing.T) {
	dir := t.TempDir()
	kwatchDir := filepath.Join(dir, ".kwatch")
	if err := os.MkdirAll(kwatchDir, 0755); err != nil {
		t.Fatalf("failed to create .kwatch dir: %v", err)
	}

	// Write a unified YAML with some values
	unifiedYAML := `
server:
  host: "0.0.0.0"
  port: 9090
runner:
  default_timeout: 60s
  max_parallel: 5
`
	if err := os.WriteFile(filepath.Join(kwatchDir, "kwatch.yaml"), []byte(unifiedYAML), 0644); err != nil {
		t.Fatalf("failed to write yaml: %v", err)
	}

	// Set environment variable overrides
	t.Setenv("KWATCH_SERVER_PORT", "8080")
	t.Setenv("KWATCH_RUNNER_MAX_PARALLEL", "10")
	t.Setenv("KWATCH_SERVER_HOST", "127.0.0.1")
	t.Setenv("KWATCH_RUNNER_TIMEOUT", "45s")
	t.Setenv("KWATCH_TUI_LOG_LEVEL", "debug")
	t.Setenv("KWATCH_MCP_HISTORY_LIMIT", "50")

	cfg, err := LoadKWatchConfig(dir)
	if err != nil {
		t.Fatalf("LoadKWatchConfig() returned error: %v", err)
	}

	// Environment should override YAML values
	if cfg.Server.Host != "127.0.0.1" {
		t.Errorf("Server.Host = %q (env override), want %q", cfg.Server.Host, "127.0.0.1")
	}
	// Env KWATCH_SERVER_PORT=8080 overrides YAML port 9090
	if cfg.Server.Port != 8080 {
		t.Errorf("Server.Port = %d (env override), want 8080", cfg.Server.Port)
	}
	if cfg.Runner.MaxParallel != 10 {
		t.Errorf("Runner.MaxParallel = %d (env override), want 10", cfg.Runner.MaxParallel)
	}
	if cfg.Runner.DefaultTimeout != 45*time.Second {
		t.Errorf("Runner.DefaultTimeout = %v (env override), want 45s", cfg.Runner.DefaultTimeout)
	}

	// Environment-only values should also be set
	if cfg.TUI.LogLevel != "debug" {
		t.Errorf("TUI.LogLevel = %q (env only), want \"debug\"", cfg.TUI.LogLevel)
	}
	if cfg.MCP.HistoryLimit != 50 {
		t.Errorf("MCP.HistoryLimit = %d (env only), want 50", cfg.MCP.HistoryLimit)
	}

	// Non-overridden values should remain from YAML or defaults
	if cfg.Server.ReadTimeout != DefaultServerReadTimeout {
		t.Errorf("Server.ReadTimeout should remain default, got %v", cfg.Server.ReadTimeout)
	}
}

// TestLoadKWatchConfig_Validate tests validation of the unified config.
func TestLoadKWatchConfig_Validate(t *testing.T) {
	tests := []struct {
		name    string
		cfg     *KWatchConfig
		wantErr bool
	}{
		{
			name: "valid default config",
			cfg:  DefaultKWatchConfig(),
		},
		{
			name: "invalid port (negative)",
			cfg: &KWatchConfig{
				Server: ServerConfig{Port: -1},
				Runner: RunnerConfig{
					DefaultTimeout: 30 * time.Second,
					MaxParallel:    1,
				},
				TUI: TUIConfig{MaxLogs: 100},
			},
			wantErr: true,
		},
		{
			name: "invalid port (>65535)",
			cfg: &KWatchConfig{
				Server: ServerConfig{Port: 70000},
				Runner: RunnerConfig{
					DefaultTimeout: 30 * time.Second,
					MaxParallel:    1,
				},
				TUI: TUIConfig{MaxLogs: 100},
			},
			wantErr: true,
		},
		{
			name: "invalid max parallel (0)",
			cfg: &KWatchConfig{
				Server: ServerConfig{Port: 3737},
				Runner: RunnerConfig{
					DefaultTimeout: 30 * time.Second,
					MaxParallel:    0,
				},
				TUI: TUIConfig{MaxLogs: 100},
			},
			wantErr: true,
		},
		{
			name: "invalid timeout (zero)",
			cfg: &KWatchConfig{
				Server: ServerConfig{Port: 3737},
				Runner: RunnerConfig{
					DefaultTimeout: 0,
					MaxParallel:    1,
				},
				TUI: TUIConfig{MaxLogs: 100},
			},
			wantErr: true,
		},
		{
			name: "invalid max logs (zero)",
			cfg: &KWatchConfig{
				Server: ServerConfig{Port: 3737},
				Runner: RunnerConfig{
					DefaultTimeout: 30 * time.Second,
					MaxParallel:    1,
				},
				TUI: TUIConfig{MaxLogs: 0},
			},
			wantErr: true,
		},
		{
			name: "negative max file size",
			cfg: &KWatchConfig{
				Server: ServerConfig{Port: 3737},
				Runner: RunnerConfig{
					DefaultTimeout: 30 * time.Second,
					MaxParallel:    1,
				},
				Security: SecurityConfig{
					MaxFileSize: -1,
				},
				TUI: TUIConfig{MaxLogs: 100},
			},
			wantErr: true,
		},
		{
			name: "negative context lines",
			cfg: &KWatchConfig{
				Server: ServerConfig{Port: 3737},
				Runner: RunnerConfig{
					DefaultTimeout: 30 * time.Second,
					MaxParallel:    1,
				},
				Security: SecurityConfig{
					ContextLines: -5,
				},
				TUI: TUIConfig{MaxLogs: 100},
			},
			wantErr: true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			err := tt.cfg.Validate()
			if (err != nil) != tt.wantErr {
				t.Errorf("Validate() error = %v, wantErr = %v", err, tt.wantErr)
			}
		})
	}
}

// TestLoadKWatchConfig_NoFile verifies default config is returned when no
// config file exists.
func TestLoadKWatchConfig_NoFile(t *testing.T) {
	dir := t.TempDir()

	cfg, err := LoadKWatchConfig(dir)
	if err != nil {
		t.Fatalf("LoadKWatchConfig() with no file returned error: %v", err)
	}
	if cfg == nil {
		t.Fatal("LoadKWatchConfig() returned nil")
	}

	// Should have full defaults
	if cfg.Server.Port != DefaultServerPort {
		t.Errorf("default port = %d, want %d", cfg.Server.Port, DefaultServerPort)
	}
	if cfg.Runner.DefaultTimeout != DefaultRunnerTimeout {
		t.Errorf("default runner timeout = %v, want %v", cfg.Runner.DefaultTimeout, DefaultRunnerTimeout)
	}
	if cfg.Security.MaxFileSize != DefaultSecurityMaxFileSize {
		t.Errorf("default max file size = %d, want %d", cfg.Security.MaxFileSize, DefaultSecurityMaxFileSize)
	}
}

// TestKWatchConfig_LegacyBackwardCompat verifies that the legacy config format
// still loads correctly through the unified system.
func TestKWatchConfig_LegacyBackwardCompat(t *testing.T) {
	dir := t.TempDir()
	kwatchDir := filepath.Join(dir, ".kwatch")
	if err := os.MkdirAll(kwatchDir, 0755); err != nil {
		t.Fatalf("failed to create .kwatch dir: %v", err)
	}

	legacyYAML := `defaultTimeout: 45s
maxParallel: 5
commands:
  typescript:
    command: npx
    args: [tsc, --noEmit]
    timeout: 60s
    enabled: true
  lint:
    command: npx
    args: [eslint, ".", "--ext", ".ts,.tsx,.js,.jsx"]
    timeout: 45s
    enabled: true
  test:
    command: npm
    args: [test]
    timeout: 90s
    enabled: false
`

	if err := os.WriteFile(filepath.Join(kwatchDir, "kwatch.yaml"), []byte(legacyYAML), 0644); err != nil {
		t.Fatalf("failed to write legacy yaml: %v", err)
	}

	// Unified config loader should handle legacy format gracefully
	unifiedCfg, err := LoadKWatchConfig(dir)
	if err != nil {
		t.Fatalf("LoadKWatchConfig() with legacy format should not error: %v", err)
	}
	if unifiedCfg == nil {
		t.Fatal("LoadKWatchConfig() returned nil")
	}

	// maxParallel should be extracted from legacy config
	if unifiedCfg.Runner.MaxParallel != 5 {
		t.Errorf("Runner.MaxParallel from legacy config = %d, want 5", unifiedCfg.Runner.MaxParallel)
	}
}
