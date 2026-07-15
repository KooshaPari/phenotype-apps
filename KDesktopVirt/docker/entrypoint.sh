#!/bin/bash
set -e

# KVirtualStage Docker Entrypoint Script
# Handles initialization and service startup

echo "🎭 Starting KVirtualStage Container"
echo "=================================="

# Function to check if Docker daemon is accessible
check_docker() {
    if docker info >/dev/null 2>&1; then
        echo "✅ Docker daemon accessible"
        return 0
    else
        echo "⚠️  Docker daemon not accessible"
        echo "   Container orchestration features will be limited"
        return 1
    fi
}

# Function to setup audio system
setup_audio() {
    echo "🔊 Setting up audio system..."
    
    # Check if PipeWire is available
    if command -v pipewire >/dev/null 2>&1; then
        echo "✅ PipeWire available"
        if ! pgrep -x pipewire >/dev/null; then
            pipewire &
            sleep 2
        fi
        if ! pgrep -x wireplumber >/dev/null; then
            wireplumber &
            sleep 1
        fi
    elif command -v pulseaudio >/dev/null 2>&1; then
        echo "✅ PulseAudio available"
        if ! pgrep -x pulseaudio >/dev/null; then
            pulseaudio --start --exit-idle-time=-1
        fi
    else
        echo "⚠️  No audio system detected"
    fi
}

# Function to setup display
setup_display() {
    if [ -n "$DISPLAY" ]; then
        echo "✅ Display environment: $DISPLAY"
    else
        echo "⚠️  No display environment set"
        export DISPLAY=:0
    fi
}

# Function to initialize configuration
init_config() {
    echo "⚙️  Initializing configuration..."
    
    if [ ! -f "/app/.kvirtualstage/config.toml" ]; then
        echo "📁 Creating default configuration..."
        kvirtualstage config init
    else
        echo "✅ Configuration already exists"
    fi
}

# Function to check system requirements
check_system() {
    echo "🔍 Checking system requirements..."
    
    # Check memory
    MEM_TOTAL=$(free -m | grep '^Mem:' | awk '{print $2}')
    if [ "$MEM_TOTAL" -lt 1024 ]; then
        echo "⚠️  Low memory: ${MEM_TOTAL}MB (recommended: 2GB+)"
    else
        echo "✅ Memory: ${MEM_TOTAL}MB"
    fi
    
    # Check disk space
    DISK_AVAIL=$(df /app | tail -1 | awk '{print $4}')
    if [ "$DISK_AVAIL" -lt 1048576 ]; then  # 1GB in KB
        echo "⚠️  Low disk space: ${DISK_AVAIL}KB (recommended: 5GB+)"
    else
        echo "✅ Disk space: ${DISK_AVAIL}KB available"
    fi
}

# Function to start services
start_services() {
    echo "🚀 Starting KVirtualStage services..."
    
    case "$1" in
        "start")
            shift
            echo "Starting full KVirtualStage service..."
            exec kvirtualstage start "$@"
            ;;
        "mcp")
            shift
            echo "Starting MCP server..."
            exec kvirtualstage mcp start "$@"
            ;;
        "desktop")
            echo "Starting desktop environment..."
            # This would be used by the desktop variant
            setup_vnc
            exec kvirtualstage start --ui --host 0.0.0.0 --port 3000
            ;;
        *)
            echo "Starting KVirtualStage with args: $@"
            exec kvirtualstage "$@"
            ;;
    esac
}

# Function to setup VNC (for desktop variant)
setup_vnc() {
    echo "🖥️  Setting up VNC server..."
    
    if [ ! -f "/app/.vnc/passwd" ]; then
        # Generate secure random VNC password
        VNC_PASSWORD=$(openssl rand -base64 32 | tr -d "=+/" | cut -c1-25)
        echo "🔐 Generated secure VNC password: $VNC_PASSWORD"
        echo "$VNC_PASSWORD" | vncpasswd -f > /app/.vnc/passwd
        chmod 600 /app/.vnc/passwd
        
        # Store password securely for application access
        echo "$VNC_PASSWORD" > /app/.vnc/password.txt
        chmod 600 /app/.vnc/password.txt
    fi
    
    # Start VNC server
    vncserver :1 -geometry ${VNC_RESOLUTION:-1920x1080} -depth ${VNC_DEPTH:-24} \
        -SecurityTypes None &
    
    export DISPLAY=:1
    sleep 3
}

# Signal handlers for graceful shutdown
handle_signal() {
    echo ""
    echo "🛑 Received shutdown signal..."
    echo "Stopping KVirtualStage services..."
    
    # Stop any background processes
    if pgrep -x pipewire >/dev/null; then
        pkill pipewire
    fi
    if pgrep -x pulseaudio >/dev/null; then
        pulseaudio --kill
    fi
    if pgrep -x Xvnc >/dev/null; then
        vncserver -kill :1 2>/dev/null || true
    fi
    
    echo "✅ Shutdown complete"
    exit 0
}

# Setup signal handlers
trap handle_signal SIGTERM SIGINT

# Main execution
main() {
    echo "🏠 Working directory: $(pwd)"
    echo "👤 Running as: $(whoami)"
    echo "🐳 Container ID: $(hostname)"
    echo ""
    
    # System checks
    check_system
    check_docker
    setup_display
    setup_audio
    init_config
    
    echo ""
    echo "🎬 Ready to start KVirtualStage!"
    echo ""
    
    # Start services based on command
    start_services "$@"
}

# Execute main function with all arguments
main "$@"