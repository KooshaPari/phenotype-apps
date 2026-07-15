package main

import (
	"os"
	"os/signal"
	"syscall"

	"github.com/sirupsen/logrus"
	"github.com/spf13/cobra"

	"kodevibe/pkg/config"
	"kodevibe/pkg/server"
)

var (
	configFile string
	host       string
	port       int
	tlsEnabled bool
	certFile   string
	keyFile    string
	verbose    bool
	logger     *logrus.Logger
)

func main() {
	logger = logrus.New()
	logger.SetFormatter(&logrus.JSONFormatter{})

	var rootCmd = &cobra.Command{
		Use:   "kodevibe-server",
		Short: "KodeVibe HTTP Server",
		Long: `KodeVibe HTTP Server provides REST API and WebSocket endpoints
for remote code quality scanning and real-time monitoring.

Features:
• REST API for scanning, configuration, and reporting
• WebSocket for real-time updates
• Dashboard UI for project monitoring
• Integration webhooks (Slack, GitHub, etc.)
• Performance profiling and metrics`,
		Version: "1.0.0",
		RunE:    runServer,
	}

	// Flags
	rootCmd.Flags().StringVarP(&configFile, "config", "c", "", "Configuration file path")
	rootCmd.Flags().StringVar(&host, "host", "localhost", "Server host")
	rootCmd.Flags().IntVarP(&port, "port", "p", 8080, "Server port")
	rootCmd.Flags().BoolVar(&tlsEnabled, "tls", false, "Enable TLS")
	rootCmd.Flags().StringVar(&certFile, "cert", "", "TLS certificate file")
	rootCmd.Flags().StringVar(&keyFile, "key", "", "TLS private key file")
	rootCmd.Flags().BoolVarP(&verbose, "verbose", "v", false, "Verbose logging")

	if err := rootCmd.Execute(); err != nil {
		logger.WithError(err).Error("server command execution failed")
		os.Exit(1)
	}
}

func runServer(cmd *cobra.Command, args []string) error {
	// Set log level
	if verbose {
		logger.SetLevel(logrus.DebugLevel)
	} else {
		logger.SetLevel(logrus.InfoLevel)
	}

	logger.Info("🌊 Starting KodeVibe Server")

	// Load configuration
	configMgr := config.NewManager()
	if err := configMgr.LoadConfig(configFile); err != nil {
		logger.Warnf("Failed to load config: %v, using defaults", err)
	}

	cfg := configMgr.GetConfig()

	// Create and start server
	srv := server.NewServer(cfg, logger)

	// Handle graceful shutdown
	go func() {
		sigChan := make(chan os.Signal, 1)
		signal.Notify(sigChan, syscall.SIGINT, syscall.SIGTERM)
		<-sigChan

		logger.Info("Shutdown signal received, stopping server...")
		// TODO: Implement graceful shutdown
		os.Exit(0)
	}()

	logger.Infof("🚀 Server starting on %s:%d", host, port)
	if tlsEnabled {
		logger.Info("🔒 TLS enabled")
	}

	return srv.Start(host, port, tlsEnabled, certFile, keyFile)
}
