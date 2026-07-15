FROM ubuntu:22.04

# Avoid timezone prompts
ENV DEBIAN_FRONTEND=noninteractive
ENV TZ=UTC

# Install KDE Plasma desktop environment and VNC server
RUN apt-get update && apt-get install -y \
    kubuntu-desktop-minimal \
    tigervnc-standalone-server \
    tigervnc-common \
    dbus-x11 \
    xfce4-terminal \
    firefox \
    xdotool \
    imagemagick \
    ffmpeg \
    pulseaudio \
    pipewire \
    espeak \
    curl \
    wget \
    git \
    vim \
    nano \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Create VNC user
RUN useradd -m -s /bin/bash kvirtualstage
RUN echo 'kvirtualstage:kvirtualstage' | chpasswd

# Set up VNC
USER kvirtualstage
WORKDIR /home/kvirtualstage

# Create VNC password
RUN mkdir -p ~/.vnc
RUN echo 'kvirtualstage' | vncpasswd -f > ~/.vnc/passwd
RUN chmod 600 ~/.vnc/passwd

# VNC startup script
RUN echo '#!/bin/bash\n\
export DISPLAY=:0\n\
startplasma-x11 &\n\
while true; do sleep 1000; done' > ~/.vnc/xstartup
RUN chmod +x ~/.vnc/xstartup

# Set up audio
RUN mkdir -p ~/.config/pulse
RUN echo 'default-server = unix:/tmp/pulse-socket' > ~/.config/pulse/client.conf

# Create startup script
RUN echo '#!/bin/bash\n\
pulseaudio --start --exit-idle-time=-1\n\
vncserver :0 -geometry 1920x1080 -depth 24\n\
tail -f ~/.vnc/*.log' > ~/start.sh
RUN chmod +x ~/start.sh

EXPOSE 5900

CMD ["./start.sh"]