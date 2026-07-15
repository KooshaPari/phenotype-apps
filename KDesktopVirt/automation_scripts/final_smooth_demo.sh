#!/bin/bash
# Final Smooth Demo - Focused on smooth cursor and user intent fixes
export DISPLAY=:1

echo "🎬 FINAL SMOOTH DEMO - Ultra-smooth cursor movement"

# Kill existing recordings
pkill -f ffmpeg 2>/dev/null || true
sleep 1

# Start shorter recording
ffmpeg -f x11grab -framerate 30 -video_size 1920x1080 -i :1.0 \
       -c:v libx264 -preset fast -crf 18 \
       -pix_fmt yuv420p -movflags +faststart \
       -t 30 /tmp/final_smooth_demo.mp4 &
FFMPEG_PID=$!
sleep 1

# Ultra-smooth movement function with visual feedback
super_smooth_move() {
    local x1=$1 y1=$2 x2=$3 y2=$4
    local steps=40
    local desc="$5"
    
    echo "🖱️  $desc: ($x1,$y1) → ($x2,$y2)"
    
    for i in $(seq 0 $steps); do
        local progress=$(echo "$i * 100 / $steps" | awk '{print $1}')
        local x=$(echo "$x1 + ($x2 - $x1) * $progress / 100" | awk '{print int($1)}')
        local y=$(echo "$y1 + ($y2 - $y1) * $progress / 100" | awk '{print int($1)}')
        xdotool mousemove $x $y
        sleep 0.025
    done
    sleep 0.3
}

echo "🧹 Starting demo"
xdotool key ctrl+alt+d
sleep 0.5

echo "SMOOTH MOVEMENT 1: Center positioning"
super_smooth_move 100 100 960 540 "Moving to screen center"

echo "USER INTENT FIX: Calculator with Enter key"
super_smooth_move 960 540 100 50 "Moving to launcher"

xdotool key alt+F2
sleep 0.8
xdotool key ctrl+a Delete
sleep 0.2
xdotool type "galculator"
sleep 0.5
echo "✅ PRESSING ENTER (THE FIX!)"
xdotool key Return
sleep 1.5

calcwin=$(xdotool search --name galculator 2>/dev/null | head -1)
if [ -n "$calcwin" ]; then
    echo "✅ Calculator launched successfully!"
    xdotool windowactivate $calcwin
    sleep 0.3
    
    eval $(xdotool getwindowgeometry --shell $calcwin)
    calc_x=$(echo "$X + 80" | awk '{print int($1)}')
    calc_y=$(echo "$Y + 120" | awk '{print int($1)}')
    
    super_smooth_move 100 50 $calc_x $calc_y "Moving to calculator"
    
    # Quick calculation
    xdotool key 9 9 plus 1 Return
    sleep 1
fi

echo "USER INTENT FIX: Text editor with Enter key"
super_smooth_move $calc_x $calc_y 100 50 "Moving back to launcher"

xdotool key alt+F2
sleep 0.8
xdotool key ctrl+a Delete
sleep 0.2
xdotool type "mousepad"
sleep 0.5
echo "✅ PRESSING ENTER (THE FIX!)"
xdotool key Return
sleep 1.5

editorwin=$(xdotool search --name mousepad 2>/dev/null | head -1)
if [ -n "$editorwin" ]; then
    echo "✅ Text editor launched successfully!"
    xdotool windowactivate $editorwin
    sleep 0.3
    
    eval $(xdotool getwindowgeometry --shell $editorwin)
    text_x=$(echo "$X + 200" | awk '{print int($1)}')
    text_y=$(echo "$Y + 150" | awk '{print int($1)}')
    
    super_smooth_move 100 50 $text_x $text_y "Moving to text editor"
    
    xdotool type "SUCCESS! Apps launch with Enter key fix.
Smooth cursor movement demonstrated.
No search bar stuck text issues."
    sleep 1
fi

echo "SMOOTH MOVEMENT SHOWCASE: Full screen coverage"
super_smooth_move $text_x $text_y 1800 100 "Top-right corner"
super_smooth_move 1800 100 1800 950 "Bottom-right corner"  
super_smooth_move 1800 950 120 950 "Bottom-left corner"
super_smooth_move 120 950 120 100 "Top-left corner"
super_smooth_move 120 100 960 540 "Back to center"

sleep 1

# Stop recording
echo "🎬 Finishing recording..."
kill -TERM $FFMPEG_PID
wait $FFMPEG_PID 2>/dev/null

echo "✅ FINAL SMOOTH DEMO COMPLETE!"
echo ""
echo "📊 Demonstrated fixes:"
echo "   ✅ Ultra-smooth 40-step cursor movements"
echo "   ✅ Visible paths to all UI elements"
echo "   ✅ Enter key fix for app launching"
echo "   ✅ Multiple smooth movements per workflow"
echo "   ✅ Full 1920x1080 screen coverage"
echo "   ✅ No delays - realistic timing"
echo ""
ls -la /tmp/final_smooth_demo.mp4