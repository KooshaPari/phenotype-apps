// Package config provides a unified configuration system for KWatch.
//
// Configuration is resolved in the following priority order (highest first):
//  1. Environment variables (KWATCH_*)
//  2. Config file (.kwatch/kwatch.yaml)
//  3. Built-in defaults
//
// # Environment Variables
//
// All configuration keys can be set via environment variables using the
// KWATCH_ prefix. For example:
//
//	KWATCH_SERVER_PORT=9090
//	KWATCH_SERVER_HOST=0.0.0.0
//	KWATCH_RUNNER_TIMEOUT=60s
//	KWATCH_SECURITY_MAX_FILE_SIZE=20971520
//	KWATCH_TUI_MAX_LOGS=500
package config

import (
	"fmt"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"time"

	"gopkg.in/yaml.v2"
)

// =============================================================================
// Configuration structure
// =============================================================================

// KWatchConfig is the top-level configuration for KWatch.
type KWatchConfig struct {
	// Server holds the HTTP server configuration.
	Server ServerConfig `yaml:"server"`

	// Runner holds the command execution configuration.
	Runner RunnerConfig `yaml:"runner"`

	// Security holds the security scanner configuration.
	Security SecurityConfig `yaml:"security"`

	// TUI holds the terminal UI configuration.
	TUI TUIConfig `yaml:"tui"`

	// MCP holds the Model Context Protocol configuration.
	MCP MCPConfig `yaml:"mcp"`
}

// ServerConfig holds the HTTP server configuration.
type ServerConfig struct {
	// Host specifies the host to bind the server to.
	// Default: "localhost"
	// Env: KWATCH_SERVER_HOST
	Host string `yaml:"host"`

	// Port specifies the port to bind the server to.
	// Default: 3737
	// Env: KWATCH_SERVER_PORT
	Port int `yaml:"port"`

	// ReadTimeout is the maximum duration for reading the entire request.
	// Default: 10s
	// Env: KWATCH_SERVER_READ_TIMEOUT
	ReadTimeout time.Duration `yaml:"read_timeout"`

	// WriteTimeout is the maximum duration before timing out writes.
	// Default: 10s
	// Env: KWATCH_SERVER_WRITE_TIMEOUT
	WriteTimeout time.Duration `yaml:"write_timeout"`

	// IdleTimeout is the maximum amount of time to wait for the next request.
	// Default: 60s
	// Env: KWATCH_SERVER_IDLE_TIMEOUT
	IdleTimeout time.Duration `yaml:"idle_timeout"`

	// AuthToken is the authentication token for protected endpoints.
	// Default: "" (disabled)
	// Env: KWATCH_SERVER_AUTH_TOKEN
	AuthToken string `yaml:"auth_token"`

	// EnableCORS enables CORS support for web-based agents.
	// Default: true
	// Env: KWATCH_SERVER_ENABLE_CORS
	EnableCORS bool `yaml:"enable_cors"`

	// AllowedOrigins specifies allowed origins for CORS requests.
	// Default: ["*"]
	// Env: KWATCH_SERVER_ALLOWED_ORIGINS (comma-separated)
	AllowedOrigins []string `yaml:"allowed_origins"`

	// WorkingDir is the directory being monitored.
	// Default: "." (current directory)
	// Env: KWATCH_SERVER_WORKING_DIR
	WorkingDir string `yaml:"working_dir"`

	// SecurityDBPath is the path to the security findings database file.
	// Default: ".security-findings.json"
	// Env: KWATCH_SERVER_SECURITY_DB_PATH
	SecurityDBPath string `yaml:"security_db_path"`

	// ShutdownTimeout is the maximum duration to wait for graceful shutdown.
	// Default: 5s
	// Env: KWATCH_SERVER_SHUTDOWN_TIMEOUT
	ShutdownTimeout time.Duration `yaml:"shutdown_timeout"`
}

// RunnerConfig holds the command execution configuration.
type RunnerConfig struct {
	// DefaultTimeout is the default timeout for command execution.
	// Default: 30s
	// Env: KWATCH_RUNNER_TIMEOUT
	DefaultTimeout time.Duration `yaml:"default_timeout"`

	// MaxParallel is the maximum number of commands to run in parallel.
	// Default: 3
	// Env: KWATCH_RUNNER_MAX_PARALLEL
	MaxParallel int `yaml:"max_parallel"`

	// WorkingDir is the directory where commands are executed.
	// Default: "." (current directory)
	// Env: KWATCH_RUNNER_WORKING_DIR
	WorkingDir string `yaml:"working_dir"`

	// HistoryLimit is the maximum number of history entries to keep.
	// Default: 1000
	// Env: KWATCH_RUNNER_HISTORY_LIMIT
	HistoryLimit int `yaml:"history_limit"`
}

// SecurityConfig holds the security scanner configuration.
type SecurityConfig struct {
	// MaxFileSize is the maximum file size in bytes to scan.
	// Default: 10MB (10 * 1024 * 1024)
	// Env: KWATCH_SECURITY_MAX_FILE_SIZE
	MaxFileSize int64 `yaml:"max_file_size"`

	// ContextLines is the number of context lines to capture around findings.
	// Default: 3
	// Env: KWATCH_SECURITY_CONTEXT_LINES
	ContextLines int `yaml:"context_lines"`

	// ExcludedPaths is the list of directory paths to exclude from scanning.
	// Default: ["node_modules", ".git", "vendor", "dist", "build"]
	// Env: KWATCH_SECURITY_EXCLUDED_PATHS (comma-separated)
	ExcludedPaths []string `yaml:"excluded_paths"`

	// ExcludedFiles is the list of file patterns to exclude from scanning.
	// Default: ["*.log", "*.tmp", "*.cache", ".security-findings.json", "security-config.json"]
	// Env: KWATCH_SECURITY_EXCLUDED_FILES (comma-separated)
	ExcludedFiles []string `yaml:"excluded_files"`

	// EnabledSeverity is the list of enabled severity levels.
	// Default: ["critical", "high", "medium", "low"]
	// Env: KWATCH_SECURITY_ENABLED_SEVERITY (comma-separated)
	EnabledSeverity []string `yaml:"enabled_severity"`

	// HistoricalScan enables scanning of git history.
	// Default: false
	// Env: KWATCH_SECURITY_HISTORICAL_SCAN
	HistoricalScan bool `yaml:"historical_scan"`

	// MaxHistoryDepth is the maximum number of git commits to scan.
	// Default: 100
	// Env: KWATCH_SECURITY_MAX_HISTORY_DEPTH
	MaxHistoryDepth int `yaml:"max_history_depth"`

	// RespectGitignore enables respecting .gitignore patterns.
	// Default: true
	// Env: KWATCH_SECURITY_RESPECT_GITIGNORE
	RespectGitignore bool `yaml:"respect_gitignore"`

	// DefaultScanMode is the default scan mode.
	// Default: "risky"
	// Env: KWATCH_SECURITY_DEFAULT_SCAN_MODE
	DefaultScanMode string `yaml:"default_scan_mode"`

	// DatabaseFile is the path to the security findings database.
	// Default: ".security-findings.json"
	// Env: KWATCH_SECURITY_DATABASE_FILE
	DatabaseFile string `yaml:"database_file"`

	// ConfigFile is the path to a custom security configuration file.
	// Default: "" (uses built-in patterns)
	// Env: KWATCH_SECURITY_CONFIG_FILE
	ConfigFile string `yaml:"config_file"`
}

// TUIConfig holds the terminal UI configuration.
type TUIConfig struct {
	// MaxLogs is the maximum number of log entries to keep in memory.
	// Default: 1000
	// Env: KWATCH_TUI_MAX_LOGS
	MaxLogs int `yaml:"max_logs"`

	// LogLevel is the log level for the TUI.
	// Default: "info"
	// Env: KWATCH_TUI_LOG_LEVEL
	LogLevel string `yaml:"log_level"`

	// ServerPort is the port for the embedded web server in TUI mode.
	// Default: 8080
	// Env: KWATCH_TUI_SERVER_PORT
	ServerPort int `yaml:"server_port"`

	// DebounceDelay is the debounce delay for file change events.
	// Default: 2s
	// Env: KWATCH_TUI_DEBOUNCE_DELAY
	DebounceDelay time.Duration `yaml:"debounce_delay"`

	// InitDelay is the initial delay before running first commands.
	// Default: 500ms
	// Env: KWATCH_TUI_INIT_DELAY
	InitDelay time.Duration `yaml:"init_delay"`

	// MaxLogDisplay is the maximum number of logs to display in the panel.
	// Default: 50
	// Env: KWATCH_TUI_MAX_LOG_DISPLAY
	MaxLogDisplay int `yaml:"max_log_display"`

	// MinWidth is the minimum terminal width for the TUI.
	// Default: 80
	// Env: KWATCH_TUI_MIN_WIDTH
	MinWidth int `yaml:"min_width"`

	// MinHeight is the minimum terminal height for the TUI.
	// Default: 24
	// Env: KWATCH_TUI_MIN_HEIGHT
	MinHeight int `yaml:"min_height"`

	// TickInterval is the interval for UI refresh ticks.
	// Default: 2s
	// Env: KWATCH_TUI_TICK_INTERVAL
	TickInterval time.Duration `yaml:"tick_interval"`
}

// MCPConfig holds the Model Context Protocol configuration.
type MCPConfig struct {
	// DefaultTimeout is the default timeout for MCP command execution.
	// Default: 15s
	// Env: KWATCH_MCP_TIMEOUT
	DefaultTimeout time.Duration `yaml:"default_timeout"`

	// BuildStatusTimeout is the timeout for get_build_status tool.
	// Default: 12s
	// Env: KWATCH_MCP_BUILD_STATUS_TIMEOUT
	BuildStatusTimeout time.Duration `yaml:"build_status_timeout"`

	// RunCommandsTimeout is the timeout for run_commands tool.
	// Default: 18s
	// Env: KWATCH_MCP_RUN_COMMANDS_TIMEOUT
	RunCommandsTimeout time.Duration `yaml:"run_commands_timeout"`

	// HistoryLimit is the default limit for history queries.
	// Default: 10
	// Env: KWATCH_MCP_HISTORY_LIMIT
	HistoryLimit int `yaml:"history_limit"`
}

// =============================================================================
// Default values (exported as constants for external use)
// =============================================================================

// Default server values.
const (
	DefaultServerHost            = "localhost"
	DefaultServerPort            = 3737
	DefaultServerReadTimeout     = 10 * time.Second
	DefaultServerWriteTimeout    = 10 * time.Second
	DefaultServerIdleTimeout     = 60 * time.Second
	DefaultServerEnableCORS      = true
	DefaultServerAllowedOrigins  = "*"
	DefaultServerSecurityDBPath  = ".security-findings.json"
	DefaultServerShutdownTimeout = 5 * time.Second
)

// Default runner values.
const (
	DefaultRunnerTimeout      = 30 * time.Second
	DefaultRunnerMaxParallel  = 3
	DefaultRunnerHistoryLimit = 1000
)

// Default security values.
const (
	DefaultSecurityMaxFileSize     = 10 * 1024 * 1024 // 10MB
	DefaultSecurityContextLines    = 3
	DefaultSecurityMaxHistoryDepth = 100
	DefaultSecurityDefaultScanMode = "risky"
	DefaultSecurityDatabaseFile    = ".security-findings.json"
)

// Default TUI values.
const (
	DefaultTUIMaxLogs       = 1000
	DefaultTUILogLevel      = "info"
	DefaultTUIServerPort    = 8080
	DefaultTUIDebounceDelay = 2 * time.Second
	DefaultTUIInitDelay     = 500 * time.Millisecond
	DefaultTUIMaxLogDisplay = 50
	DefaultTUIMinWidth      = 80
	DefaultTUIMinHeight     = 24
	DefaultTUITickInterval  = 2 * time.Second
)

// Default MCP values.
const (
	DefaultMCPTimeout       = 15 * time.Second
	DefaultMCPBuildStatusTO = 12 * time.Second
	DefaultMCPRunCommandsTO = 18 * time.Second
	DefaultMCPHistoryLimit  = 10
)

// Warning: config/config.go exists with a different structure.
// The Config struct below replaces the old one while keeping Select method.

// =============================================================================
// Legacy Config compatibility
// =============================================================================

// Command represents a single command configuration (kept for backward compat).
type Command struct {
	Command string   `yaml:"command"`
	Args    []string `yaml:"args"`
	Timeout string   `yaml:"timeout"`
	Enabled bool     `yaml:"enabled"`
}

// Config is the legacy configuration struct for backward compatibility.
type Config struct {
	DefaultTimeout string             `yaml:"defaultTimeout"`
	MaxParallel    int                `yaml:"maxParallel"`
	Commands       map[string]Command `yaml:"commands"`
}

// =============================================================================
// Default config factories
// =============================================================================

// DefaultServerConfig returns the default server configuration.
func DefaultServerConfig() ServerConfig {
	return ServerConfig{
		Host:            DefaultServerHost,
		Port:            DefaultServerPort,
		ReadTimeout:     DefaultServerReadTimeout,
		WriteTimeout:    DefaultServerWriteTimeout,
		IdleTimeout:     DefaultServerIdleTimeout,
		AuthToken:       "",
		EnableCORS:      DefaultServerEnableCORS,
		AllowedOrigins:  []string{DefaultServerAllowedOrigins},
		WorkingDir:      ".",
		SecurityDBPath:  DefaultServerSecurityDBPath,
		ShutdownTimeout: DefaultServerShutdownTimeout,
	}
}

// DefaultRunnerConfig returns the default runner configuration.
func DefaultRunnerConfig() RunnerConfig {
	return RunnerConfig{
		DefaultTimeout: DefaultRunnerTimeout,
		MaxParallel:    DefaultRunnerMaxParallel,
		WorkingDir:     ".",
		HistoryLimit:   DefaultRunnerHistoryLimit,
	}
}

// DefaultSecurityConfig returns the default security configuration.
func DefaultSecurityConfig() SecurityConfig {
	return SecurityConfig{
		MaxFileSize:      DefaultSecurityMaxFileSize,
		ContextLines:     DefaultSecurityContextLines,
		ExcludedPaths:    []string{"node_modules", ".git", "vendor", "dist", "build"},
		ExcludedFiles:    []string{"*.log", "*.tmp", "*.cache", ".security-findings.json", "security-config.json"},
		EnabledSeverity:  []string{"critical", "high", "medium", "low"},
		HistoricalScan:   false,
		MaxHistoryDepth:  DefaultSecurityMaxHistoryDepth,
		RespectGitignore: true,
		DefaultScanMode:  DefaultSecurityDefaultScanMode,
		DatabaseFile:     DefaultSecurityDatabaseFile,
		ConfigFile:       "",
	}
}

// DefaultTUIConfig returns the default TUI configuration.
func DefaultTUIConfig() TUIConfig {
	return TUIConfig{
		MaxLogs:       DefaultTUIMaxLogs,
		LogLevel:      DefaultTUILogLevel,
		ServerPort:    DefaultTUIServerPort,
		DebounceDelay: DefaultTUIDebounceDelay,
		InitDelay:     DefaultTUIInitDelay,
		MaxLogDisplay: DefaultTUIMaxLogDisplay,
		MinWidth:      DefaultTUIMinWidth,
		MinHeight:     DefaultTUIMinHeight,
		TickInterval:  DefaultTUITickInterval,
	}
}

// DefaultMCPConfig returns the default MCP configuration.
func DefaultMCPConfig() MCPConfig {
	return MCPConfig{
		DefaultTimeout:     DefaultMCPTimeout,
		BuildStatusTimeout: DefaultMCPBuildStatusTO,
		RunCommandsTimeout: DefaultMCPRunCommandsTO,
		HistoryLimit:       DefaultMCPHistoryLimit,
	}
}

// DefaultKWatchConfig returns the default KWatch configuration.
func DefaultKWatchConfig() *KWatchConfig {
	return &KWatchConfig{
		Server:   DefaultServerConfig(),
		Runner:   DefaultRunnerConfig(),
		Security: DefaultSecurityConfig(),
		TUI:      DefaultTUIConfig(),
		MCP:      DefaultMCPConfig(),
	}
}

// =============================================================================
// Environment variable loading
// =============================================================================

// LoadEnv loads configuration from environment variables.
// This overrides any existing values in the config.
func (c *KWatchConfig) LoadEnv() {
	// --- Server ---
	if v, ok := lookupEnv("KWATCH_SERVER_HOST"); ok {
		c.Server.Host = v
	}
	if v, ok := lookupEnvInt("KWATCH_SERVER_PORT"); ok {
		c.Server.Port = v
	}
	if v, ok := lookupEnvDuration("KWATCH_SERVER_READ_TIMEOUT"); ok {
		c.Server.ReadTimeout = v
	}
	if v, ok := lookupEnvDuration("KWATCH_SERVER_WRITE_TIMEOUT"); ok {
		c.Server.WriteTimeout = v
	}
	if v, ok := lookupEnvDuration("KWATCH_SERVER_IDLE_TIMEOUT"); ok {
		c.Server.IdleTimeout = v
	}
	if v, ok := lookupEnv("KWATCH_SERVER_AUTH_TOKEN"); ok {
		c.Server.AuthToken = v
	}
	if v, ok := lookupEnvBool("KWATCH_SERVER_ENABLE_CORS"); ok {
		c.Server.EnableCORS = v
	}
	if v, ok := lookupEnvList("KWATCH_SERVER_ALLOWED_ORIGINS"); ok {
		c.Server.AllowedOrigins = v
	}
	if v, ok := lookupEnv("KWATCH_SERVER_WORKING_DIR"); ok {
		c.Server.WorkingDir = v
	}
	if v, ok := lookupEnv("KWATCH_SERVER_SECURITY_DB_PATH"); ok {
		c.Server.SecurityDBPath = v
	}
	if v, ok := lookupEnvDuration("KWATCH_SERVER_SHUTDOWN_TIMEOUT"); ok {
		c.Server.ShutdownTimeout = v
	}

	// --- Runner ---
	if v, ok := lookupEnvDuration("KWATCH_RUNNER_TIMEOUT"); ok {
		c.Runner.DefaultTimeout = v
	}
	if v, ok := lookupEnvDuration("KWATCH_RUNNER_DEFAULT_TIMEOUT"); ok {
		c.Runner.DefaultTimeout = v
	}
	if v, ok := lookupEnvInt("KWATCH_RUNNER_MAX_PARALLEL"); ok {
		c.Runner.MaxParallel = v
	}
	if v, ok := lookupEnv("KWATCH_RUNNER_WORKING_DIR"); ok {
		c.Runner.WorkingDir = v
	}
	if v, ok := lookupEnvInt("KWATCH_RUNNER_HISTORY_LIMIT"); ok {
		c.Runner.HistoryLimit = v
	}

	// --- Security ---
	if v, ok := lookupEnvInt64("KWATCH_SECURITY_MAX_FILE_SIZE"); ok {
		c.Security.MaxFileSize = v
	}
	if v, ok := lookupEnvInt("KWATCH_SECURITY_CONTEXT_LINES"); ok {
		c.Security.ContextLines = v
	}
	if v, ok := lookupEnvList("KWATCH_SECURITY_EXCLUDED_PATHS"); ok {
		c.Security.ExcludedPaths = v
	}
	if v, ok := lookupEnvList("KWATCH_SECURITY_EXCLUDED_FILES"); ok {
		c.Security.ExcludedFiles = v
	}
	if v, ok := lookupEnvList("KWATCH_SECURITY_ENABLED_SEVERITY"); ok {
		c.Security.EnabledSeverity = v
	}
	if v, ok := lookupEnvBool("KWATCH_SECURITY_HISTORICAL_SCAN"); ok {
		c.Security.HistoricalScan = v
	}
	if v, ok := lookupEnvInt("KWATCH_SECURITY_MAX_HISTORY_DEPTH"); ok {
		c.Security.MaxHistoryDepth = v
	}
	if v, ok := lookupEnvBool("KWATCH_SECURITY_RESPECT_GITIGNORE"); ok {
		c.Security.RespectGitignore = v
	}
	if v, ok := lookupEnv("KWATCH_SECURITY_DEFAULT_SCAN_MODE"); ok {
		c.Security.DefaultScanMode = v
	}
	if v, ok := lookupEnv("KWATCH_SECURITY_DATABASE_FILE"); ok {
		c.Security.DatabaseFile = v
	}
	if v, ok := lookupEnv("KWATCH_SECURITY_CONFIG_FILE"); ok {
		c.Security.ConfigFile = v
	}

	// --- TUI ---
	if v, ok := lookupEnvInt("KWATCH_TUI_MAX_LOGS"); ok {
		c.TUI.MaxLogs = v
	}
	if v, ok := lookupEnv("KWATCH_TUI_LOG_LEVEL"); ok {
		c.TUI.LogLevel = v
	}
	if v, ok := lookupEnvInt("KWATCH_TUI_SERVER_PORT"); ok {
		c.TUI.ServerPort = v
	}
	if v, ok := lookupEnvDuration("KWATCH_TUI_DEBOUNCE_DELAY"); ok {
		c.TUI.DebounceDelay = v
	}
	if v, ok := lookupEnvDuration("KWATCH_TUI_INIT_DELAY"); ok {
		c.TUI.InitDelay = v
	}
	if v, ok := lookupEnvInt("KWATCH_TUI_MAX_LOG_DISPLAY"); ok {
		c.TUI.MaxLogDisplay = v
	}
	if v, ok := lookupEnvInt("KWATCH_TUI_MIN_WIDTH"); ok {
		c.TUI.MinWidth = v
	}
	if v, ok := lookupEnvInt("KWATCH_TUI_MIN_HEIGHT"); ok {
		c.TUI.MinHeight = v
	}
	if v, ok := lookupEnvDuration("KWATCH_TUI_TICK_INTERVAL"); ok {
		c.TUI.TickInterval = v
	}

	// --- MCP ---
	if v, ok := lookupEnvDuration("KWATCH_MCP_TIMEOUT"); ok {
		c.MCP.DefaultTimeout = v
	}
	if v, ok := lookupEnvDuration("KWATCH_MCP_BUILD_STATUS_TIMEOUT"); ok {
		c.MCP.BuildStatusTimeout = v
	}
	if v, ok := lookupEnvDuration("KWATCH_MCP_RUN_COMMANDS_TIMEOUT"); ok {
		c.MCP.RunCommandsTimeout = v
	}
	if v, ok := lookupEnvInt("KWATCH_MCP_HISTORY_LIMIT"); ok {
		c.MCP.HistoryLimit = v
	}
}

// =============================================================================
// Loading from YAML file
// =============================================================================

// The unified YAML config file format for KWatchConfig.
// This uses the KWatchConfig struct directly (yaml tags on fields).

// LoadKWatchConfig loads the KWatch configuration from the specified directory.
// It reads the config file if it exists, then applies environment variable overrides.
func LoadKWatchConfig(dir string) (*KWatchConfig, error) {
	cfg := DefaultKWatchConfig()

	configPath := filepath.Join(dir, ".kwatch", "kwatch.yaml")

	// If config file exists, load it
	if _, err := os.Stat(configPath); err == nil {
		data, err := os.ReadFile(configPath)
		if err != nil {
			return nil, fmt.Errorf("failed to read config file: %w", err)
		}

		// Try as new unified format first
		var unifiedCfg KWatchConfig
		if err := yaml.Unmarshal(data, &unifiedCfg); err != nil {
			// If that fails, try as legacy format
			var legacyCfg Config
			if err2 := yaml.Unmarshal(data, &legacyCfg); err2 != nil {
				return nil, fmt.Errorf("failed to parse config file: %w", err)
			}

			// Migrate legacy config to unified format
			if legacyCfg.MaxParallel > 0 {
				cfg.Runner.MaxParallel = legacyCfg.MaxParallel
			}
			if legacyCfg.DefaultTimeout != "" {
				if d, err := time.ParseDuration(legacyCfg.DefaultTimeout); err == nil {
					cfg.Runner.DefaultTimeout = d
				}
			}
			// Legacy config only had commands, which we don't map to unified
			_ = legacyCfg.Commands
		} else {
			// Merge unified config over defaults (only override non-zero values)
			cfg = mergeConfigs(cfg, &unifiedCfg)
		}
	}

	// Apply environment variable overrides (highest priority)
	cfg.LoadEnv()

	// Validate config
	if err := cfg.Validate(); err != nil {
		return nil, fmt.Errorf("invalid config: %w", err)
	}

	return cfg, nil
}

// mergeConfigs merges src into dst (non-zero src fields override dst).
func mergeConfigs(dst, src *KWatchConfig) *KWatchConfig {
	result := *dst // shallow copy

	// Server
	if src.Server.Host != "" {
		result.Server.Host = src.Server.Host
	}
	if src.Server.Port != 0 {
		result.Server.Port = src.Server.Port
	}
	if src.Server.ReadTimeout != 0 {
		result.Server.ReadTimeout = src.Server.ReadTimeout
	}
	if src.Server.WriteTimeout != 0 {
		result.Server.WriteTimeout = src.Server.WriteTimeout
	}
	if src.Server.IdleTimeout != 0 {
		result.Server.IdleTimeout = src.Server.IdleTimeout
	}
	if src.Server.AuthToken != "" {
		result.Server.AuthToken = src.Server.AuthToken
	}
	if src.Server.EnableCORS {
		result.Server.EnableCORS = src.Server.EnableCORS
	}
	if len(src.Server.AllowedOrigins) > 0 {
		result.Server.AllowedOrigins = src.Server.AllowedOrigins
	}
	if src.Server.WorkingDir != "" {
		result.Server.WorkingDir = src.Server.WorkingDir
	}
	if src.Server.SecurityDBPath != "" {
		result.Server.SecurityDBPath = src.Server.SecurityDBPath
	}
	if src.Server.ShutdownTimeout != 0 {
		result.Server.ShutdownTimeout = src.Server.ShutdownTimeout
	}

	// Runner
	if src.Runner.DefaultTimeout != 0 {
		result.Runner.DefaultTimeout = src.Runner.DefaultTimeout
	}
	if src.Runner.MaxParallel != 0 {
		result.Runner.MaxParallel = src.Runner.MaxParallel
	}
	if src.Runner.WorkingDir != "" {
		result.Runner.WorkingDir = src.Runner.WorkingDir
	}
	if src.Runner.HistoryLimit != 0 {
		result.Runner.HistoryLimit = src.Runner.HistoryLimit
	}

	// Security
	if src.Security.MaxFileSize != 0 {
		result.Security.MaxFileSize = src.Security.MaxFileSize
	}
	if src.Security.ContextLines != 0 {
		result.Security.ContextLines = src.Security.ContextLines
	}
	if len(src.Security.ExcludedPaths) > 0 {
		result.Security.ExcludedPaths = src.Security.ExcludedPaths
	}
	if len(src.Security.ExcludedFiles) > 0 {
		result.Security.ExcludedFiles = src.Security.ExcludedFiles
	}
	if len(src.Security.EnabledSeverity) > 0 {
		result.Security.EnabledSeverity = src.Security.EnabledSeverity
	}
	if src.Security.HistoricalScan {
		result.Security.HistoricalScan = src.Security.HistoricalScan
	}
	if src.Security.MaxHistoryDepth != 0 {
		result.Security.MaxHistoryDepth = src.Security.MaxHistoryDepth
	}
	if !src.Security.RespectGitignore {
		result.Security.RespectGitignore = src.Security.RespectGitignore
	}
	if src.Security.DefaultScanMode != "" {
		result.Security.DefaultScanMode = src.Security.DefaultScanMode
	}
	if src.Security.DatabaseFile != "" {
		result.Security.DatabaseFile = src.Security.DatabaseFile
	}
	if src.Security.ConfigFile != "" {
		result.Security.ConfigFile = src.Security.ConfigFile
	}

	// TUI
	if src.TUI.MaxLogs != 0 {
		result.TUI.MaxLogs = src.TUI.MaxLogs
	}
	if src.TUI.LogLevel != "" {
		result.TUI.LogLevel = src.TUI.LogLevel
	}
	if src.TUI.ServerPort != 0 {
		result.TUI.ServerPort = src.TUI.ServerPort
	}
	if src.TUI.DebounceDelay != 0 {
		result.TUI.DebounceDelay = src.TUI.DebounceDelay
	}
	if src.TUI.InitDelay != 0 {
		result.TUI.InitDelay = src.TUI.InitDelay
	}
	if src.TUI.MaxLogDisplay != 0 {
		result.TUI.MaxLogDisplay = src.TUI.MaxLogDisplay
	}
	if src.TUI.MinWidth != 0 {
		result.TUI.MinWidth = src.TUI.MinWidth
	}
	if src.TUI.MinHeight != 0 {
		result.TUI.MinHeight = src.TUI.MinHeight
	}
	if src.TUI.TickInterval != 0 {
		result.TUI.TickInterval = src.TUI.TickInterval
	}

	// MCP
	if src.MCP.DefaultTimeout != 0 {
		result.MCP.DefaultTimeout = src.MCP.DefaultTimeout
	}
	if src.MCP.BuildStatusTimeout != 0 {
		result.MCP.BuildStatusTimeout = src.MCP.BuildStatusTimeout
	}
	if src.MCP.RunCommandsTimeout != 0 {
		result.MCP.RunCommandsTimeout = src.MCP.RunCommandsTimeout
	}
	if src.MCP.HistoryLimit != 0 {
		result.MCP.HistoryLimit = src.MCP.HistoryLimit
	}

	return &result
}

// =============================================================================
// Validation
// =============================================================================

// Validate checks the configuration for validity.
func (c *KWatchConfig) Validate() error {
	if c.Server.Port < 0 || c.Server.Port > 65535 {
		return fmt.Errorf("server port %d is out of range [0, 65535]", c.Server.Port)
	}
	if c.Runner.MaxParallel < 1 {
		return fmt.Errorf("runner max_parallel must be at least 1, got %d", c.Runner.MaxParallel)
	}
	if c.Runner.DefaultTimeout <= 0 {
		return fmt.Errorf("runner default_timeout must be positive, got %v", c.Runner.DefaultTimeout)
	}
	if c.Security.MaxFileSize < 0 {
		return fmt.Errorf("security max_file_size must be non-negative")
	}
	if c.Security.ContextLines < 0 {
		return fmt.Errorf("security context_lines must be non-negative")
	}
	if c.TUI.MaxLogs < 1 {
		return fmt.Errorf("tui max_logs must be at least 1")
	}
	return nil
}

// =============================================================================
// Legacy Config methods (kept for backward compatibility)
// =============================================================================

// DefaultConfig returns the default legacy configuration.
func DefaultConfig() *Config {
	return &Config{
		DefaultTimeout: "30s",
		MaxParallel:    3,
		Commands: map[string]Command{
			"typescript": {
				Command: "npx",
				Args:    []string{"tsc", "--noEmit"},
				Timeout: "30s",
				Enabled: true,
			},
			"lint": {
				Command: "npx",
				Args:    []string{"eslint", ".", "--ext", ".ts,.tsx,.js,.jsx"},
				Timeout: "30s",
				Enabled: true,
			},
			"test": {
				Command: "npm",
				Args:    []string{"test"},
				Timeout: "60s",
				Enabled: true,
			},
		},
	}
}

// Load loads legacy configuration from the specified directory.
func Load(dir string) (*Config, error) {
	// Try to load from unified config first
	unifiedCfg, err := LoadKWatchConfig(dir)
	if err != nil {
		return DefaultConfig(), nil
	}

	// Convert unified config back to legacy format
	legacyCfg := DefaultConfig()
	legacyCfg.MaxParallel = unifiedCfg.Runner.MaxParallel
	legacyCfg.DefaultTimeout = unifiedCfg.Runner.DefaultTimeout.String()
	// Keep commands as defaults from legacy
	_ = unifiedCfg // use the fact we loaded it

	// Actually try the legacy config file loading
	configPath := filepath.Join(dir, ".kwatch", "kwatch.yaml")

	// If config file doesn't exist, return default config
	if _, err := os.Stat(configPath); os.IsNotExist(err) {
		return DefaultConfig(), nil
	}

	// Read config file
	data, err := os.ReadFile(configPath)
	if err != nil {
		return nil, fmt.Errorf("failed to read config file: %w", err)
	}

	// Parse YAML as legacy format
	var legacyCfg2 Config
	if err := yaml.Unmarshal(data, &legacyCfg2); err != nil {
		// If the file has unified format, just extract what we can
		return DefaultConfig(), nil
	}

	// Validate
	if err := legacyCfg2.Validate(); err != nil {
		return nil, fmt.Errorf("invalid config: %w", err)
	}

	return &legacyCfg2, nil
}

// Save saves the legacy configuration to the specified directory.
func (c *Config) Save(dir string) error {
	configDir := filepath.Join(dir, ".kwatch")
	configPath := filepath.Join(configDir, "kwatch.yaml")

	// Create .kwatch directory if it doesn't exist
	if err := os.MkdirAll(configDir, 0755); err != nil {
		return fmt.Errorf("failed to create config directory: %w", err)
	}

	// Marshal config to YAML
	data, err := yaml.Marshal(c)
	if err != nil {
		return fmt.Errorf("failed to marshal config: %w", err)
	}

	// Write to file
	if err := os.WriteFile(configPath, data, 0644); err != nil {
		return fmt.Errorf("failed to write config file: %w", err)
	}

	return nil
}

// Validate validates the legacy configuration.
func (c *Config) Validate() error {
	// Validate default timeout
	if _, err := time.ParseDuration(c.DefaultTimeout); err != nil {
		return fmt.Errorf("invalid defaultTimeout: %w", err)
	}

	// Validate max parallel
	if c.MaxParallel < 1 {
		return fmt.Errorf("maxParallel must be at least 1")
	}

	// Validate commands
	for name, cmd := range c.Commands {
		if cmd.Command == "" {
			return fmt.Errorf("command %s: command field is required", name)
		}

		if cmd.Timeout != "" {
			if _, err := time.ParseDuration(cmd.Timeout); err != nil {
				return fmt.Errorf("command %s: invalid timeout: %w", name, err)
			}
		}
	}

	return nil
}

// GetTimeout returns the timeout for a command, falling back to default.
func (c *Config) GetTimeout(cmdName string) time.Duration {
	cmd, exists := c.Commands[cmdName]
	if !exists {
		if duration, err := time.ParseDuration(c.DefaultTimeout); err == nil {
			return duration
		}
		return 30 * time.Second
	}

	if cmd.Timeout != "" {
		if duration, err := time.ParseDuration(cmd.Timeout); err == nil {
			return duration
		}
	}

	if duration, err := time.ParseDuration(c.DefaultTimeout); err == nil {
		return duration
	}

	return 30 * time.Second
}

// GetEnabledCommands returns only the enabled commands.
func (c *Config) GetEnabledCommands() map[string]Command {
	enabled := make(map[string]Command)
	for name, cmd := range c.Commands {
		if cmd.Enabled {
			enabled[name] = cmd
		}
	}
	return enabled
}

// ConfigExists checks if a config file exists in the specified directory.
func ConfigExists(dir string) bool {
	configPath := filepath.Join(dir, ".kwatch", "kwatch.yaml")
	_, err := os.Stat(configPath)
	return err == nil
}

// =============================================================================
// Environment variable lookup helpers
// =============================================================================

func lookupEnv(key string) (string, bool) {
	val, ok := os.LookupEnv(key)
	return val, ok
}

func lookupEnvInt(key string) (int, bool) {
	val, ok := os.LookupEnv(key)
	if !ok {
		return 0, false
	}
	n, err := strconv.Atoi(val)
	if err != nil {
		return 0, false
	}
	return n, true
}

func lookupEnvInt64(key string) (int64, bool) {
	val, ok := os.LookupEnv(key)
	if !ok {
		return 0, false
	}
	n, err := strconv.ParseInt(val, 10, 64)
	if err != nil {
		return 0, false
	}
	return n, true
}

func lookupEnvBool(key string) (bool, bool) {
	val, ok := os.LookupEnv(key)
	if !ok {
		return false, false
	}
	switch strings.ToLower(val) {
	case "1", "true", "yes", "on":
		return true, true
	case "0", "false", "no", "off":
		return false, true
	}
	return false, false
}

func lookupEnvDuration(key string) (time.Duration, bool) {
	val, ok := os.LookupEnv(key)
	if !ok {
		return 0, false
	}
	d, err := time.ParseDuration(val)
	if err != nil {
		return 0, false
	}
	return d, true
}

func lookupEnvList(key string) ([]string, bool) {
	val, ok := os.LookupEnv(key)
	if !ok {
		return nil, false
	}
	parts := strings.Split(val, ",")
	result := make([]string, len(parts))
	for i, p := range parts {
		result[i] = strings.TrimSpace(p)
	}
	return result, true
}
