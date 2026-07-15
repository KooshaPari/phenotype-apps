#!/bin/bash
set -e

echo "🖥️  Starting KVirtualStage Virtual Desktop Environment"

# Create necessary directories
mkdir -p /tmp/.X11-unix /tmp/.ICE-unix
chmod 1777 /tmp/.X11-unix /tmp/.ICE-unix

# Set environment variables
export DISPLAY=:1
export USER=kvirtualstage
export HOME=/app
export XDG_SESSION_TYPE=x11
export XDG_CURRENT_DESKTOP=XFCE

# Start VNC server with XFCE desktop
echo "Starting VNC server with XFCE desktop on display :1"
vncserver :1 -geometry 1920x1080 -depth 24 -dpi 96 \
    -localhost no -desktop "KVirtualStage-XFCE" \
    -xstartup $HOME/.vnc/xstartup

# Wait for desktop to start
sleep 5

# Start PulseAudio for virtual desktop audio
echo "Starting audio system"
pulseaudio --start --log-target=syslog

# Create automation demo scripts in virtual desktop
mkdir -p $HOME/automation_demos
cat > $HOME/automation_demos/calculator_demo.sh << 'EOF'
#!/bin/bash
# Demo script to run inside virtual desktop
galculator &
sleep 2
echo "Calculator opened in virtual desktop"
EOF
chmod +x $HOME/automation_demos/calculator_demo.sh

# Start KVirtualStage automation service
echo "Starting KVirtualStage automation service"
cd $HOME
exec kvirtualstage start --ui --port 3000 --host 0.0.0.0