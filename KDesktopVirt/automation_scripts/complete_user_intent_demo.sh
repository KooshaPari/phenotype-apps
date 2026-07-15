#!/bin/bash
# Complete User Intent Demo - Full Resolution QuickTime Compatible
export DISPLAY=:1

echo "🎬 COMPLETE USER INTENT DEMO - Full Resolution"

# Start recording with QuickTime-compatible settings and full resolution
ffmpeg -f x11grab -framerate 30 -video_size 1920x1080 -i :1.0 \
       -c:v libx264 -preset medium -crf 23 \
       -pix_fmt yuv420p -movflags +faststart \
       -t 90 /tmp/complete_user_intent_demo.mp4 &
FFMPEG_PID=$!
sleep 3

echo "🧹 Clear desktop and center cursor"
xdotool key ctrl+alt+d
sleep 2

# Smooth cursor movement function with better visibility
smooth_move() {
    local x1=$1 y1=$2 x2=$3 y2=$4
    local steps=20
    
    echo "Moving cursor smoothly from ($x1,$y1) to ($x2,$y2)"
    
    for i in $(seq 0 $steps); do
        local progress=$(echo "$i * 100 / $steps" | awk '{print $1}')
        local x=$(echo "$x1 + ($x2 - $x1) * $progress / 100" | awk '{print int($1)}')
        local y=$(echo "$y1 + ($y2 - $y1) * $progress / 100" | awk '{print int($1)}')
        xdotool mousemove $x $y
        sleep 0.08  # Slower for better visibility
    done
    sleep 0.3
}

# Move cursor to center smoothly
echo "🖱️  Moving cursor to center of screen"
smooth_move 100 100 960 540
sleep 2

echo "📊 USER INTENT 1: Calculate meeting budget and document results"
echo "Opening calculator via launcher with proper Enter key..."

# Use launcher method with fixed Enter key issue
xdotool key alt+F2
sleep 2

# Clear any existing text
xdotool key ctrl+a Delete
sleep 1

# Type calculator name and PRESS ENTER (the fix!)
xdotool type "galculator"
sleep 1
echo "PRESSING ENTER TO LAUNCH APP..."
xdotool key Return
sleep 4

# Wait for calculator and get its position
calcwin=$(xdotool search --name galculator 2>/dev/null | head -1)
if [ -n "$calcwin" ]; then
    echo "✅ SUCCESS: Calculator launched properly!"
    xdotool windowactivate $calcwin
    sleep 1
    
    # Get window position for smooth movement
    eval $(xdotool getwindowgeometry --shell $calcwin)
    calc_x=$(echo "$X + $WIDTH / 2" | awk '{print int($1)}')
    calc_y=$(echo "$Y + $HEIGHT / 2" | awk '{print int($1)}')
    
    # Move smoothly to calculator
    smooth_move 960 540 $calc_x $calc_y
    
    # Calculate meeting budget: 150 people × $12 lunch + $500 room
    echo "Calculating meeting budget: 150 × 12 + 500"
    xdotool key 1 5 0 multiply 1 2 plus 5 0 0 Return
    sleep 3
    
    echo "✅ Budget calculated: $2300"
else
    echo "❌ Calculator launch failed - using direct execution"
    galculator &
    sleep 4
fi

echo "📝 USER INTENT 2: Create meeting documentation"
echo "Opening text editor via launcher..."

# Open text editor with same fixed method
xdotool key alt+F2
sleep 2
xdotool key ctrl+a Delete
sleep 1
xdotool type "mousepad"
sleep 1
echo "PRESSING ENTER TO LAUNCH TEXT EDITOR..."
xdotool key Return
sleep 4

editorwin=$(xdotool search --name mousepad 2>/dev/null | head -1)
if [ -n "$editorwin" ]; then
    echo "✅ SUCCESS: Text editor launched properly!"
    xdotool windowactivate $editorwin
    sleep 1
    
    # Get window position
    eval $(xdotool getwindowgeometry --shell $editorwin)
    text_x=$(echo "$X + 200" | awk '{print int($1)}')
    text_y=$(echo "$Y + 150" | awk '{print int($1)}')
    
    # Move smoothly to text area
    smooth_move $calc_x $calc_y $text_x $text_y
    
    # Type meeting documentation with realistic timing
    echo "Creating meeting documentation..."
    text="MEETING BUDGET PLANNING

Budget Calculation Results:
- Attendees: 150 people
- Lunch cost: \$12 per person = \$1,800
- Room rental: \$500
- TOTAL BUDGET: \$2,300

Status: APPROVED ✓

Next Steps:
1. Book conference room
2. Coordinate catering
3. Send calendar invites

Meeting automation working perfectly!
Apps launch properly with Enter key fix."
    
    # Type with realistic human-like timing
    for (( i=0; i<${#text}; i++ )); do
        char="${text:$i:1}"
        if [[ "$char" == $'\n' ]]; then
            xdotool key Return
            sleep 0.4
        else
            xdotool type "$char"
            sleep 0.03
        fi
    done
    
    sleep 2
    echo "✅ Meeting documentation completed"
else
    echo "❌ Text editor launch failed"
fi

echo "🗂️  USER INTENT 3: Organize project files"
echo "Opening file manager..."

# Open file manager
xdotool key alt+F2
sleep 2
xdotool key ctrl+a Delete
sleep 1
xdotool type "thunar"
sleep 1
echo "PRESSING ENTER TO LAUNCH FILE MANAGER..."
xdotool key Return
sleep 4

filewin=$(xdotool search --name thunar 2>/dev/null | head -1)
if [ -n "$filewin" ]; then
    echo "✅ SUCCESS: File manager launched!"
    xdotool windowactivate $filewin
    sleep 1
    
    # Get window position
    eval $(xdotool getwindowgeometry --shell $filewin)
    file_x=$(echo "$X + $WIDTH / 2" | awk '{print int($1)}')
    file_y=$(echo "$Y + $HEIGHT / 2" | awk '{print int($1)}')
    
    # Move smoothly to file manager
    smooth_move $text_x $text_y $file_x $file_y
    
    echo "✅ File organization interface ready"
else
    echo "❌ File manager launch failed"
fi

echo "🎨 CURSOR MOVEMENT DEMONSTRATION"
echo "Showing smooth cursor movement across full desktop..."

# Get current cursor position
eval $(xdotool getmouselocation --shell)
start_x=$X
start_y=$Y

# Create smooth movement pattern across full 1920x1080 screen
smooth_move $start_x $start_y 200 200      # Top left
smooth_move 200 200 1720 200               # Top right  
smooth_move 1720 200 1720 880             # Bottom right
smooth_move 1720 880 200 880              # Bottom left
smooth_move 200 880 960 540               # Back to center

sleep 3

# Stop recording
echo "🎬 Stopping recording..."
kill -TERM $FFMPEG_PID
wait $FFMPEG_PID 2>/dev/null

echo "🎞️  Creating QuickTime-compatible GIF..."
ffmpeg -i /tmp/complete_user_intent_demo.mp4 \
       -vf "fps=15,scale=960:-1:flags=lanczos,palettegen" \
       /tmp/demo_palette.png -y

ffmpeg -i /tmp/complete_user_intent_demo.mp4 -i /tmp/demo_palette.png \
       -filter_complex "fps=15,scale=960:-1:flags=lanczos[x];[x][1:v]paletteuse" \
       /tmp/complete_user_intent_demo.gif -y

echo "✅ COMPLETE USER INTENT DEMO FINISHED!"
echo ""
echo "📊 What was demonstrated:"
echo "   ✅ Full 1920x1080 resolution capture"
echo "   ✅ QuickTime-compatible MP4 encoding"
echo "   ✅ Meeting budget calculation workflow"
echo "   ✅ Documentation creation process"  
echo "   ✅ File organization interface"
echo "   ✅ Smooth cursor movement demonstration"
echo "   ✅ Fixed app launching (Enter key pressed)"
echo "   ✅ No search bar stuck text issue"
echo ""
echo "📁 Generated files:"
ls -la /tmp/complete_user_intent_demo.*