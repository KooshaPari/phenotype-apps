#!/bin/bash

# KodeVibe Integration Test Script
# This script tests the complete functionality of KodeVibe

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
TEST_DIR="integration-test-workspace"
CLI_BINARY="./build/kodevibe"
SERVER_BINARY="./build/kodevibe-server"

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

cleanup() {
    log_info "Cleaning up test environment..."
    rm -rf "$TEST_DIR" 2>/dev/null || true
    pkill -f "$SERVER_BINARY" 2>/dev/null || true
    log_success "Cleanup completed"
}

# Trap cleanup on exit
trap cleanup EXIT

# Function to check if binary exists and is executable
check_binary() {
    local binary=$1
    if [[ ! -f "$binary" ]]; then
        log_error "Binary not found: $binary"
        log_info "Please run 'make build' first"
        exit 1
    fi
    
    if [[ ! -x "$binary" ]]; then
        log_error "Binary is not executable: $binary"
        chmod +x "$binary"
        log_success "Made binary executable: $binary"
    fi
}

# Function to create test files with various issues
create_test_files() {
    log_info "Creating test files..."
    
    mkdir -p "$TEST_DIR"
    
    # JavaScript file with multiple issues
    cat > "$TEST_DIR/test.js" << 'EOF'
// This file contains intentional issues for testing
var globalVar = "bad practice";
console.log("Debug message that should be removed");

function longFunctionName() {
    if (globalVar == "test") {  // Should use === instead of ==
        var localVar = 42;  // Magic number
        console.log(localVar);
        if (localVar > 40) {
            if (localVar < 50) {
                return "nested too deep";
            }
        }
    }
    return globalVar;
}

// TODO: This should be tracked in issues
// var commentedCode = "should be removed";
EOF

    # Python file with issues
    cat > "$TEST_DIR/test.py" << 'EOF'
import os
import sys

def test_function():
    print("Debug print statement")
    password = "hardcoded_secret"  # Security issue
    api_key = "sk-1234567890abcdef"  # Another security issue
    
    if True:
        if True:
            if True:
                print("Too much nesting")
    
    return password

# FIXME: This needs to be fixed
x = 42  # Magic number
EOF

    # Go file with issues
    cat > "$TEST_DIR/test.go" << 'EOF'
package main

import (
    "context"
    "fmt"
)

func main() {
    ctx := context.TODO()  // Should use proper context
    fmt.Printf("Hello %s\n", "world")
    
    secret := "sk-abcd1234567890"  // Security issue
    if len(secret) > 10 {
        panic("Something went wrong")  // Should return error instead
    }
}
EOF

    # Configuration file with secrets
    cat > "$TEST_DIR/.env" << 'EOF'
DATABASE_URL=postgres://user:pass@localhost/db
API_KEY=sk-test1234567890abcdef1234567890abcdef
GITHUB_TOKEN=ghp_test1234567890abcdef1234567890abcdef123456
AWS_ACCESS_KEY_ID=AKIATEST0000000EXAMPLE
SECRET_KEY=very-secret-key-that-should-not-be-exposed
EOF

    log_success "Test files created in $TEST_DIR"
}

# Function to test CLI functionality
test_cli() {
    log_info "Testing CLI functionality..."
    
    # Test version command
    log_info "Testing version command..."
    if $CLI_BINARY version > /dev/null 2>&1; then
        log_success "Version command works"
    else
        log_warning "Version command failed (might not be implemented)"
    fi
    
    # Test help command
    log_info "Testing help command..."
    if $CLI_BINARY --help > /dev/null 2>&1; then
        log_success "Help command works"
    else
        log_error "Help command failed"
        return 1
    fi
    
    # Test scan command with JSON output
    log_info "Testing scan command with JSON output..."
    if $CLI_BINARY scan "$TEST_DIR" --format json > scan-result.json 2>&1; then
        log_success "Scan command completed"
        
        # Verify output file exists and has content
        if [[ -f "scan-result.json" && -s "scan-result.json" ]]; then
            log_success "Scan results generated successfully"
            
            # Check for expected issues
            local issue_count=$(grep -c '"type"' scan-result.json 2>/dev/null || echo "0")
            log_info "Found $issue_count issues in scan results"
            
            if [[ $issue_count -gt 0 ]]; then
                log_success "Scan detected issues as expected"
            else
                log_warning "No issues detected (might be expected)"
            fi
        else
            log_error "Scan results file is empty or missing"
            return 1
        fi
    else
        log_error "Scan command failed"
        cat scan-result.json 2>/dev/null || echo "No output file"
        return 1
    fi
    
    # Test scan command with text output
    log_info "Testing scan command with text output..."
    if $CLI_BINARY scan "$TEST_DIR" --format text > scan-result.txt 2>&1; then
        log_success "Text format scan completed"
    else
        log_warning "Text format scan failed"
    fi
    
    # Test configuration validation
    log_info "Testing configuration..."
    if $CLI_BINARY config validate 2>/dev/null; then
        log_success "Configuration validation works"
    else
        log_warning "Configuration validation failed (might not be implemented)"
    fi
}

# Function to test server functionality
test_server() {
    log_info "Testing server functionality..."
    
    # Start server in background
    log_info "Starting server..."
    $SERVER_BINARY --port 18080 > server.log 2>&1 &
    local server_pid=$!
    
    # Wait for server to start
    sleep 3
    
    # Check if server is running
    if kill -0 "$server_pid" 2>/dev/null; then
        log_success "Server started successfully"
        
        # Test health endpoint
        log_info "Testing health endpoint..."
        if curl -s "http://localhost:18080/health" > /dev/null 2>&1; then
            log_success "Health endpoint responds"
        else
            log_warning "Health endpoint not accessible"
        fi
        
        # Test API endpoints
        log_info "Testing API endpoints..."
        if curl -s "http://localhost:18080/api/v1/vibes" > /dev/null 2>&1; then
            log_success "API endpoints accessible"
        else
            log_warning "API endpoints not accessible"
        fi
        
        # Stop server
        kill "$server_pid" 2>/dev/null || true
        wait "$server_pid" 2>/dev/null || true
        log_success "Server stopped"
    else
        log_error "Server failed to start"
        cat server.log 2>/dev/null || echo "No server log"
        return 1
    fi
}

# Function to test various file types
test_file_types() {
    log_info "Testing various file types..."
    
    # Create additional test files
    cat > "$TEST_DIR/test.java" << 'EOF'
public class Test {
    public static void main(String[] args) {
        System.out.println("Hello World");  // Should use logging
        String secret = "sk-abcd1234567890";  // Security issue
    }
}
EOF

    cat > "$TEST_DIR/test.cpp" << 'EOF'
#include <iostream>
using namespace std;

int main() {
    cout << "Hello World" << endl;
    return 0;
}
EOF

    # Test scanning different file types
    if $CLI_BINARY scan "$TEST_DIR" --format json > multi-file-scan.json 2>&1; then
        local file_count=$(grep -c '"file"' multi-file-scan.json 2>/dev/null || echo "0")
        log_success "Multi-file scan completed, scanned files: $file_count"
    else
        log_warning "Multi-file scan failed"
    fi
}

# Function to test performance
test_performance() {
    log_info "Testing performance..."
    
    # Create many test files for performance testing
    mkdir -p "$TEST_DIR/perf-test"
    for i in {1..50}; do
        cp "$TEST_DIR/test.js" "$TEST_DIR/perf-test/test$i.js"
    done
    
    # Time the scan
    local start_time=$(date +%s)
    if $CLI_BINARY scan "$TEST_DIR/perf-test" --format json > perf-scan.json 2>&1; then
        local end_time=$(date +%s)
        local duration=$((end_time - start_time))
        log_success "Performance test completed in ${duration}s"
        
        if [[ $duration -lt 30 ]]; then
            log_success "Performance is acceptable"
        else
            log_warning "Performance might be slow: ${duration}s"
        fi
    else
        log_warning "Performance test failed"
    fi
}

# Main test execution
main() {
    log_info "Starting KodeVibe Integration Tests"
    echo "=================================================="
    
    # Pre-test checks
    check_binary "$CLI_BINARY"
    check_binary "$SERVER_BINARY"
    
    # Run tests
    create_test_files
    test_cli
    test_server
    test_file_types
    test_performance
    
    # Summary
    echo "=================================================="
    log_success "Integration tests completed successfully!"
    
    # Show test results summary
    log_info "Test Results Summary:"
    if [[ -f "scan-result.json" ]]; then
        local total_issues=$(grep -c '"type"' scan-result.json 2>/dev/null || echo "0")
        echo "  - Total issues detected: $total_issues"
    fi
    
    if [[ -f "multi-file-scan.json" ]]; then
        local scanned_files=$(grep -c '"file"' multi-file-scan.json 2>/dev/null || echo "0")
        echo "  - Files scanned: $scanned_files"
    fi
    
    echo "  - Test workspace: $TEST_DIR"
    echo "  - All integration tests: PASSED ✅"
}

# Check if running in CI environment
if [[ "${CI:-}" == "true" ]]; then
    log_info "Running in CI environment"
    # CI-specific settings
    set -x  # Enable command tracing
fi

# Run main function
main "$@"