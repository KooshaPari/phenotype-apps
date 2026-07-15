# KVirtualStage - Build and Installation Makefile

.PHONY: build install install-user uninstall clean test deps dev help check run package

# Variables
BINARY_NAME := kvirtualstage
BUILD_DIR := target/release
INSTALL_DIR := /usr/local/bin
VERSION := $(shell git describe --tags --always --dirty 2>/dev/null || echo "dev")

# Default target - build release version
build:
	@echo "Building $(BINARY_NAME)..."
	cargo build --release
	@echo "Binary built: $(BUILD_DIR)/$(BINARY_NAME)"

# Install globally (requires sudo)
install: build
	@echo "Installing $(BINARY_NAME) to $(INSTALL_DIR)..."
	@sudo cp $(BUILD_DIR)/$(BINARY_NAME) $(INSTALL_DIR)/$(BINARY_NAME)
	@sudo chmod +x $(INSTALL_DIR)/$(BINARY_NAME)
	@echo "$(BINARY_NAME) installed successfully!"
	@echo "Usage: $(BINARY_NAME) --help"

# User install (no sudo required)
install-user: build
	@echo "Installing $(BINARY_NAME) to ~/bin..."
	@mkdir -p ~/bin
	@cp $(BUILD_DIR)/$(BINARY_NAME) ~/bin/$(BINARY_NAME)
	@chmod +x ~/bin/$(BINARY_NAME)
	@echo "$(BINARY_NAME) installed to ~/bin/"
	@echo "Make sure ~/bin is in your PATH:"
	@echo "Add this to your shell config: export PATH=\"\$$HOME/bin:\$$PATH\""

# Uninstall
uninstall:
	@echo "Uninstalling $(BINARY_NAME)..."
	@sudo rm -f $(INSTALL_DIR)/$(BINARY_NAME)
	@rm -f ~/bin/$(BINARY_NAME)
	@echo "$(BINARY_NAME) uninstalled."

# Install dependencies
deps:
	@echo "Installing Rust dependencies..."
	cargo fetch

# Development build (with debug info)
dev:
	@echo "Building development version..."
	cargo build
	@echo "Development binary built: target/debug/$(BINARY_NAME)"

# Run tests
test:
	@echo "Running tests..."
	cargo test

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	cargo clean
	@echo "Clean complete."

# Check if installed correctly
check:
	@echo "Checking installation..."
	@which $(BINARY_NAME) || echo "$(BINARY_NAME) not found in PATH"
	@$(BINARY_NAME) --help >/dev/null 2>&1 || echo "$(BINARY_NAME) not executable"

# Development server
run: build
	@echo "Starting $(BINARY_NAME)..."
	@$(BUILD_DIR)/$(BINARY_NAME) --help

# Docker build for virtual desktop
docker-build:
	@echo "Building virtual desktop Docker image..."
	docker build -f Dockerfile.simple-desktop -t kvirtualstage-desktop .
	@echo "Docker image built: kvirtualstage-desktop"

# Start virtual desktop container
docker-run:
	@echo "Starting virtual desktop container..."
	docker run -d --name kvirtualstage-desktop-test -p 5901:5901 kvirtualstage-desktop
	@echo "Virtual desktop running on VNC port 5901"

# Stop virtual desktop container
docker-stop:
	@echo "Stopping virtual desktop container..."
	docker stop kvirtualstage-desktop-test 2>/dev/null || true
	docker rm kvirtualstage-desktop-test 2>/dev/null || true

# Package for distribution
package: build
	@echo "Creating distribution package..."
	@mkdir -p dist
	@tar -czf dist/$(BINARY_NAME)-$(VERSION)-$(shell uname -s | tr '[:upper:]' '[:lower:]')-$(shell uname -m).tar.gz -C $(BUILD_DIR) $(BINARY_NAME)
	@echo "Package created: dist/$(BINARY_NAME)-$(VERSION)-$(shell uname -s | tr '[:upper:]' '[:lower:]')-$(shell uname -m).tar.gz"

# Help
help:
	@echo "KVirtualStage - Build and Installation Commands"
	@echo ""
	@echo "Usage:"
	@echo "  make build         - Build the release binary"
	@echo "  make install       - Install globally (requires sudo)"
	@echo "  make install-user  - Install to ~/bin (no sudo required)"
	@echo "  make uninstall     - Uninstall globally and from ~/bin"
	@echo "  make clean         - Clean build artifacts"
	@echo "  make test          - Run tests"
	@echo "  make dev           - Build development version"
	@echo "  make run           - Build and show help"
	@echo "  make docker-build  - Build virtual desktop Docker image"
	@echo "  make docker-run    - Start virtual desktop container"
	@echo "  make docker-stop   - Stop virtual desktop container"
	@echo "  make package       - Create distribution package"
	@echo "  make check         - Check if installed correctly"
	@echo "  make help          - Show this help"
	@echo ""
	@echo "Quick start:"
	@echo "  make install-user  # Install to ~/bin (recommended)"
	@echo "  kvirtualstage --help  # Show available commands"
	@echo "  make docker-build  # Build virtual desktop environment"
	@echo "  make docker-run    # Start virtual desktop for testing"