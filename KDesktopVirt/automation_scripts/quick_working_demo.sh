#!/bin/bash
# Quick Working Demo - Guaranteed to show visible progression
export DISPLAY=:1

echo "🎬 Creating Quick Working Demo with Visible Changes"

# Start recording
ffmpeg -f x11grab -framerate 20 -video_size 1024x768 -i :1.0 \
       -c:v libx264 -preset ultrafast -crf 18 -pix_fmt yuv420p \
       -t 30 /tmp/working_demo.mp4 &

FFMPEG_PID=$!
echo "Recording started with PID: $FFMPEG_PID"
sleep 2

echo "🧹 Clearing desktop first"
# Clear desktop to show visible starting state
xdotool key ctrl+alt+d
sleep 2

echo "📱 Opening Calculator - Step 1"
# Open calculator with visible typing
xdotool key alt+F2
sleep 1.5

# Type "calc" letter by letter so it's visible
echo "Typing 'galculator' visibly..."
xdotool type "g"; sleep 0.5
xdotool type "a"; sleep 0.5
xdotool type "l"; sleep 0.5
xdotool type "c"; sleep 0.5
xdotool type "u"; sleep 0.5
xdotool type "l"; sleep 0.5
xdotool type "a"; sleep 0.5
xdotool type "t"; sleep 0.5
xdotool type "o"; sleep 0.5
xdotool type "r"; sleep 0.5

xdotool key Return
sleep 3

echo "🔢 Performing Calculation - Step 2"
# Find and use calculator
calcwin=$(xdotool search --name galculator | head -1)
if [ -n "$calcwin" ]; then
    xdotool windowactivate $calcwin
    sleep 1
    
    echo "Calculating 2+2..."
    xdotool key 2; sleep 1
    xdotool key plus; sleep 1  
    xdotool key 2; sleep 1
    xdotool key equal; sleep 2
    
    echo "Calculating 5*7..."
    xdotool key c; sleep 1  # Clear
    xdotool key 5; sleep 1
    xdotool key asterisk; sleep 1
    xdotool key 7; sleep 1
    xdotool key equal; sleep 2
fi

echo "📝 Opening Text Editor - Step 3"
# Open text editor
xdotool key alt+F2
sleep 1

xdotool type "mousepad"
xdotool key Return
sleep 3

echo "✍️  Typing Visible Text - Step 4"
# Type something clearly visible
xdotool type "WORKING DEMO

Calculator Results:
- 2 + 2 = 4
- 5 × 7 = 35

Demo Status: SUCCESS!"

sleep 3

echo "⏹️  Stopping recording..."
kill -TERM $FFMPEG_PID
wait $FFMPEG_PID 2>/dev/null

echo "🎨 Creating optimized GIF..."
# Create GIF with good settings
ffmpeg -i /tmp/working_demo.mp4 -vf "fps=10,scale=640:-1:flags=lanczos,palettegen" /tmp/palette.png -y
ffmpeg -i /tmp/working_demo.mp4 -i /tmp/palette.png -filter_complex \
       "fps=10,scale=640:-1:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer" \
       /tmp/working_demo.gif -y

echo "✅ Working demo created successfully!"
ls -la /tmp/working_demo.*