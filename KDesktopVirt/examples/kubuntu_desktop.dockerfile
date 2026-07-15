# Optimized Kubuntu Desktop Container for KVirtualStage
# Based on research recommendations for KDE Plasma 6 automation
FROM ubuntu:24.04

LABEL maintainer="KVirtualStage Team"
LABEL description="Optimized Kubuntu Desktop with KDE Plasma 6 for desktop automation"
LABEL version="1.0"

# Environment variables for non-interactive installation
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC
ENV DISPLAY=:0
ENV VNC_RESOLUTION=1920x1080
ENV VNC_COL_DEPTH=24

# Create automation user
RUN useradd -m -s /bin/bash kvs && \
    echo 'kvs:kvs' | chpasswd && \
    usermod -aG sudo kvs && \
    echo 'kvs ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

# Update and install essential packages
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Core system
    ca-certificates \
    curl \
    wget \
    gnupg \
    software-properties-common \
    # Desktop environment
    kubuntu-desktop-minimal \
    plasma-desktop \
    kde-standard \
    kwin-x11 \
    # Essential KDE applications
    dolphin \
    kate \
    konsole \
    kcalc \
    gwenview \
    okular \
    # VNC and remote access
    tigervnc-standalone-server \
    tigervnc-common \
    novnc \
    websockify \
    # Automation tools
    xdotool \
    wmctrl \
    xvfb \
    scrot \
    imagemagick \
    # Development tools
    git \
    vim \
    nano \
    htop \
    # Media support
    ubuntu-restricted-extras \
    ffmpeg \
    # Fonts
    fonts-liberation \
    fonts-dejavu \
    fonts-noto \
    fonts-roboto \
    && apt-get autoremove -y \
    && apt-get autoclean \
    && rm -rf /var/lib/apt/lists/*

# Configure VNC
USER kvs
WORKDIR /home/kvs

# Create VNC directory and configuration
RUN mkdir -p /home/kvs/.vnc && \
    mkdir -p /home/kvs/.config/autostart

# VNC startup script optimized for automation
RUN cat > /home/kvs/.vnc/xstartup << 'EOF'
#!/bin/bash
export XKL_XMODMAP_DISABLE=1
unset SESSION_MANAGER
unset DBUS_SESSION_BUS_ADDRESS

# Start D-Bus
eval `dbus-launch --sh-syntax`

# KDE Plasma optimized for automation
export DESKTOP_SESSION=plasma
export XDG_SESSION_DESKTOP=KDE
export XDG_CURRENT_DESKTOP=KDE
export KDE_SESSION_VERSION=5

# Disable compositor for better automation performance
kwriteconfig5 --file kwinrc --group Compositing --key Enabled false

# Set single-click mode for easier automation
kwriteconfig5 --file kdeglobals --group KDE --key SingleClick false

# Start Plasma session
exec startplasma-x11
EOF

# Make startup script executable
RUN chmod +x /home/kvs/.vnc/xstartup

# Plasma configuration for automation
RUN mkdir -p /home/kvs/.config/plasma-org.kde.plasma.desktop-appletsrc && \
    cat > /home/kvs/.config/plasma-org.kde.plasma.desktop-appletsrc << 'EOF'
[ActionPlugins][0]
RightButton;NoModifier=org.kde.contextmenu

[Containments][1]
activity=automation-session
formfactor=0
immutability=1
lastScreen=0
location=0
plugin=org.kde.plasma.folder
wallpaperplugin=org.kde.image

[Containments][1][Wallpaper][org.kde.image][General]
Image=file:///usr/share/wallpapers/Next/contents/images/1920x1080.png

[Containments][2]
activityId=
formfactor=2
immutability=1
lastScreen=0
location=3
plugin=org.kde.panel

[Containments][2][Applets][3]
immutability=1
plugin=org.kde.plasma.kickoff

[Containments][2][Applets][4]
immutability=1
plugin=org.kde.plasma.digitalclock

[Containments][2][General]
AppletOrder=3;4
EOF

# Create automation helper scripts
RUN cat > /home/kvs/start-desktop.sh << 'EOF'
#!/bin/bash
# KVirtualStage desktop startup script

echo "Starting KVirtualStage Kubuntu Desktop..."

# Set VNC password if provided
if [ ! -z "$VNC_PASSWORD" ]; then
    echo "$VNC_PASSWORD" | vncpasswd -f > /home/kvs/.vnc/passwd
    chmod 600 /home/kvs/.vnc/passwd
fi

# Start VNC server
vncserver :0 -geometry $VNC_RESOLUTION -depth $VNC_COL_DEPTH -localhost no

# Start noVNC for web access
websockify --web=/usr/share/novnc/ 6080 localhost:5900 &

echo "Desktop started successfully!"
echo "VNC: localhost:5900"
echo "Web: http://localhost:6080"

# Keep container running
tail -f /home/kvs/.vnc/*.log
EOF

RUN chmod +x /home/kvs/start-desktop.sh

# Optimization script for automation performance
RUN cat > /home/kvs/optimize-for-automation.sh << 'EOF'
#!/bin/bash
# Optimization script for desktop automation

echo "Optimizing desktop for automation..."

# Disable visual effects
kwriteconfig5 --file kwinrc --group Compositing --key Enabled false
kwriteconfig5 --file kwinrc --group Effect-Blur --key Enabled false
kwriteconfig5 --file kwinrc --group Effect-DesktopGrid --key Enabled false

# Optimize theme for automation
kwriteconfig5 --file plasmarc --group Theme --key name default
kwriteconfig5 --file kdeglobals --group Icons --key Theme breeze

# Disable screensaver and power management
kwriteconfig5 --file powermanagementprofilesrc --group AC --group SuspendSession --key idleTime 0
kwriteconfig5 --file kscreenlockerrc --group Daemon --key Autolock false

# Configure window behavior for automation
kwriteconfig5 --file kwinrc --group Windows --key FocusPolicy FocusFollowsMouse
kwriteconfig5 --file kwinrc --group MouseBindings --key CommandActiveTitlebar1 Raise

echo "Optimization completed!"
EOF

RUN chmod +x /home/kvs/optimize-for-automation.sh

# Switch back to root for final setup
USER root

# Create systemd service for VNC (if systemd is available)
RUN cat > /etc/systemd/system/kvs-desktop.service << 'EOF'
[Unit]
Description=KVirtualStage Desktop Session
After=network.target

[Service]
Type=forking
User=kvs
ExecStart=/home/kvs/start-desktop.sh
ExecStop=/usr/bin/vncserver -kill :0
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

# Security hardening
RUN chmod 700 /home/kvs && \
    chmod 700 /root

# Final cleanup
RUN apt-get autoremove -y && \
    apt-get autoclean && \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

# Expose VNC and noVNC ports
EXPOSE 5900 6080

# Set working directory and default user
WORKDIR /home/kvs
USER kvs

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD pgrep -f "Xvnc :0" || exit 1

# Default command
CMD ["/home/kvs/start-desktop.sh"]