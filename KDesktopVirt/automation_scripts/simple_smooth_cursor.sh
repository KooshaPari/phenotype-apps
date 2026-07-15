#!/bin/bash
# Simple Smooth Cursor Demo - Focus on cursor movement, reliable app launching
export DISPLAY=:1

echo "🎯 Simple Smooth Cursor Movement Demo"

# Simple smooth movement function
move_smooth() {
    local x1=$1 y1=$2 x2=$3 y2=$4
    local steps=30
    
    echo "Moving cursor from ($x1,$y1) to ($x2,$y2)"
    
    for ((i=0; i<=steps; i++)); do
        local progress=$(echo "scale=3; $i / $steps" | bc -l)
        local x=$(echo "scale=0; $x1 + ($x2 - $x1) * $progress" | bc -l)
        local y=$(echo "scale=0; $y1 + ($y2 - $y1) * $progress" | bc -l)
        xdotool mousemove $x $y
        sleep 0.03
    done
    sleep 0.3
}

# Click with smooth movement
click_smooth() {
    local x=$1 y=$2
    eval $(xdotool getmouselocation --shell)
    move_smooth $X $Y $x $y
    xdotool click 1
    sleep 0.5
}

# Start recording
ffmpeg -f x11grab -framerate 30 -video_size 1024x768 -i :1.0 \
       -c:v libx264 -preset fast -crf 18 -pix_fmt yuv420p \
       -t 60 /tmp/simple_smooth_demo.mp4 &
FFMPEG_PID=$!
sleep 3

echo "🖥️  Starting demo"

# Clear desktop
xdotool key ctrl+alt+d
sleep 2

# Move cursor to center smoothly
move_smooth 100 100 512 384
sleep 1

echo "📱 Opening calculator directly"
# Launch calculator directly (most reliable)
galculator &
sleep 4

# Find calculator window
calcwin=$(xdotool search --name galculator | head -1)
if [ -n "$calcwin" ]; then
    echo "✅ Calculator found"
    eval $(xdotool getwindowgeometry --shell $calcwin)
    
    # Click on calculator with smooth movement
    click_smooth $(( X + WIDTH/2 )) $(( Y + HEIGHT/2 ))
    
    # Use keyboard for calculation (more reliable than button clicking)
    echo "Calculating 123 + 456"
    sleep 1
    xdotool key 1; sleep 0.3
    xdotool key 2; sleep 0.3  
    xdotool key 3; sleep 0.3
    xdotool key plus; sleep 0.5
    xdotool key 4; sleep 0.3
    xdotool key 5; sleep 0.3
    xdotool key 6; sleep 0.3
    xdotool key Return; sleep 2
    
    # Move cursor around the calculator to show movement
    eval $(xdotool getwindowgeometry --shell $calcwin)
    move_smooth $(( X + WIDTH/2 )) $(( Y + HEIGHT/2 )) $(( X + 50 )) $(( Y + 50 ))
    move_smooth $(( X + 50 )) $(( Y + 50 )) $(( X + WIDTH - 50 )) $(( Y + 50 ))
    move_smooth $(( X + WIDTH - 50 )) $(( Y + 50 )) $(( X + WIDTH/2 )) $(( Y + HEIGHT/2 ))
fi

echo "📝 Opening text editor"
mousepad &
sleep 4

editorwin=$(xdotool search --name mousepad | head -1)
if [ -n "$editorwin" ]; then
    echo "✅ Text editor found"
    eval $(xdotool getwindowgeometry --shell $editorwin)
    
    # Click in text area with smooth movement
    click_smooth $(( X + 100 )) $(( Y + 100 ))
    
    # Type text with small cursor movements
    text="SMOOTH CURSOR DEMO

Cursor movement is now visible and smooth!
Watch the cursor travel naturally between elements.

Calculator result: 123 + 456 = 579 ✓

This demonstrates natural automation patterns."
    
    for (( i=0; i<${#text}; i++ )); do
        char="${text:$i:1}"
        if [[ "$char" == $'\n' ]]; then
            xdotool key Return
            sleep 0.5
        else
            xdotool type "$char"
            # Tiny cursor movements during typing
            eval $(xdotool getmouselocation --shell)
            xdotool mousemove $((X + (RANDOM % 3 - 1))) $((Y + (RANDOM % 3 - 1)))
            sleep 0.1
        fi
    done
fi

echo "🎨 Final cursor movement pattern"
# Create a visible pattern
eval $(xdotool getmouselocation --shell)
start_x=$X
start_y=$Y

move_smooth $start_x $start_y 200 200
move_smooth 200 200 800 200
move_smooth 800 200 800 600  
move_smooth 800 600 200 600
move_smooth 200 600 512 384

sleep 3

# Stop recording
kill -TERM $FFMPEG_PID
wait $FFMPEG_PID 2>/dev/null

echo "🎬 Creating GIF"
ffmpeg -i /tmp/simple_smooth_demo.mp4 -vf "fps=15,scale=640:-1:flags=lanczos,palettegen" /tmp/simple_palette.png -y
ffmpeg -i /tmp/simple_smooth_demo.mp4 -i /tmp/simple_palette.png -filter_complex \
       "fps=15,scale=640:-1:flags=lanczos[x];[x][1:v]paletteuse" \
       /tmp/simple_smooth_demo.gif -y

echo "✅ Simple smooth cursor demo complete!"
ls -la /tmp/simple_smooth_demo.*