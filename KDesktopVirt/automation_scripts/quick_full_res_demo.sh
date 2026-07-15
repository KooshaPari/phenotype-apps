#!/bin/bash
# Quick Full Resolution Demo - QuickTime Compatible
export DISPLAY=:1

echo "🎬 QUICK FULL RESOLUTION DEMO"

# Kill any existing recordings
pkill -f ffmpeg 2>/dev/null || true
sleep 1

# Start recording with QuickTime-compatible settings and full resolution
ffmpeg -f x11grab -framerate 25 -video_size 1920x1080 -i :1.0 \
       -c:v libx264 -preset fast -crf 23 \
       -pix_fmt yuv420p -movflags +faststart \
       -t 45 /tmp/quick_full_demo.mp4 &
FFMPEG_PID=$!
sleep 2

echo "🧹 Clear desktop"
xdotool key ctrl+alt+d
sleep 2

# Smooth cursor movement function
smooth_move() {
    local x1=$1 y1=$2 x2=$3 y2=$4
    local steps=15
    
    for i in $(seq 0 $steps); do
        local progress=$(echo "$i * 100 / $steps" | awk '{print $1}')
        local x=$(echo "$x1 + ($x2 - $x1) * $progress / 100" | awk '{print int($1)}')
        local y=$(echo "$y1 + ($y2 - $y1) * $progress / 100" | awk '{print int($1)}')
        xdotool mousemove $x $y
        sleep 0.05
    done
    sleep 0.2
}

echo "📊 DEMO: App launching with Enter key fix"

# Move cursor to show we're starting
smooth_move 100 100 960 540
sleep 1

# Test 1: Calculator with launcher fix
echo "Opening calculator with ENTER KEY FIX..."
xdotool key alt+F2
sleep 2
xdotool key ctrl+a Delete
sleep 0.5
xdotool type "galculator"
sleep 1
echo "PRESSING ENTER (THE FIX!)"
xdotool key Return
sleep 4

calcwin=$(xdotool search --name galculator 2>/dev/null | head -1)
if [ -n "$calcwin" ]; then
    echo "✅ Calculator launched!"
    xdotool windowactivate $calcwin
    sleep 1
    
    # Get position and move cursor there
    eval $(xdotool getwindowgeometry --shell $calcwin)
    calc_x=$(echo "$X + $WIDTH / 2" | awk '{print int($1)}')
    calc_y=$(echo "$Y + $HEIGHT / 2" | awk '{print int($1)}')
    smooth_move 960 540 $calc_x $calc_y
    
    # Do calculation
    xdotool key 1 2 3 plus 4 5 6 Return
    sleep 2
fi

# Test 2: Text editor
echo "Opening text editor with ENTER KEY FIX..."
xdotool key alt+F2
sleep 2
xdotool key ctrl+a Delete
sleep 0.5
xdotool type "mousepad"
sleep 1
echo "PRESSING ENTER (THE FIX!)"
xdotool key Return
sleep 4

editorwin=$(xdotool search --name mousepad 2>/dev/null | head -1)
if [ -n "$editorwin" ]; then
    echo "✅ Text editor launched!"
    xdotool windowactivate $editorwin
    sleep 1
    
    # Get position and move cursor
    eval $(xdotool getwindowgeometry --shell $editorwin)
    text_x=$(echo "$X + 200" | awk '{print int($1)}')
    text_y=$(echo "$Y + 150" | awk '{print int($1)}')
    smooth_move $calc_x $calc_y $text_x $text_y
    
    # Type text
    xdotool type "SUCCESS! Apps launch properly now.

Enter key fix working:
✓ Calculator launched 
✓ Text editor launched
✓ No search bar stuck text
✓ Smooth cursor movement

Full 1920x1080 resolution captured!"
    sleep 3
fi

echo "🎨 Cursor movement demo across full screen"
eval $(xdotool getmouselocation --shell)
start_x=$X
start_y=$Y

# Show movement across full 1920x1080 screen
smooth_move $start_x $start_y 200 200      # Top left
smooth_move 200 200 1720 200               # Top right  
smooth_move 1720 200 1720 880             # Bottom right
smooth_move 1720 880 200 880              # Bottom left
smooth_move 200 880 960 540               # Center

sleep 2

# Stop recording gracefully
echo "🎬 Stopping recording..."
kill -TERM $FFMPEG_PID
wait $FFMPEG_PID 2>/dev/null

echo "✅ DEMO COMPLETE!"
echo "📁 Generated files:"
ls -la /tmp/quick_full_demo.mp4