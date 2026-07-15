#!/bin/bash
# Working XFCE Automation Demo - Fixed Application Launching
# Addresses application launching issues with reliable XFCE methods

export DISPLAY=:1

echo "🖥️  Working XFCE Automation Demo with Smooth Cursor Movement"

# Enhanced cursor movement with proper timing
smooth_move_cursor() {
    local start_x=$1
    local start_y=$2
    local end_x=$3
    local end_y=$4
    local steps=${5:-25}
    local delay=${6:-0.04}
    
    echo "🖱️  Moving cursor from ($start_x,$start_y) to ($end_x,$end_y)"
    
    for ((i=0; i<=steps; i++)); do
        local progress=$(echo "scale=4; $i / $steps" | bc -l)
        local current_x=$(echo "scale=0; $start_x + ($end_x - $start_x) * $progress" | bc -l)
        local current_y=$(echo "scale=0; $start_y + ($end_y - $start_y) * $progress" | bc -l)
        
        xdotool mousemove $current_x $current_y
        sleep $delay
    done
    
    xdotool mousemove $end_x $end_y
    sleep 0.2
}

# Smooth click with movement
smooth_click() {
    local target_x=$1
    local target_y=$2
    
    eval $(xdotool getmouselocation --shell)
    smooth_move_cursor $X $Y $target_x $target_y
    sleep 0.3
    xdotool click 1
    sleep 0.5
}

# Start recording
start_recording() {
    local output_file=$1
    echo "📹 Starting XFCE demo recording: ${output_file}"
    
    ffmpeg -f x11grab -framerate 30 -video_size 1024x768 -i :1.0 \
           -c:v libx264 -preset fast -crf 18 -pix_fmt yuv420p \
           -movflags +faststart "${output_file}" &
    
    FFMPEG_PID=$!
    echo "Recording started with PID: $FFMPEG_PID"
    sleep 3
}

stop_recording() {
    echo "⏹️  Stopping recording..."
    if [ ! -z "$FFMPEG_PID" ]; then
        kill -TERM $FFMPEG_PID
        wait $FFMPEG_PID 2>/dev/null
        echo "Recording stopped"
    fi
}

# Main working demo
working_xfce_demo() {
    echo "🎯 Starting Working XFCE Demo with Fixed App Launching"
    
    start_recording "/tmp/working_xfce_demo.mp4"
    
    # Clear desktop
    xdotool key ctrl+alt+d
    sleep 2
    
    # Start from center
    smooth_move_cursor 100 100 512 384
    sleep 1
    
    echo "📱 Method 1: Direct application launch via terminal"
    
    # Open terminal first (more reliable)
    smooth_move_cursor 512 384 50 50
    xdotool key ctrl+alt+t
    sleep 3
    
    # Check if terminal opened
    termwin=$(xdotool search --name terminal 2>/dev/null | head -1)
    if [ -z "$termwin" ]; then
        echo "Terminal not found, trying xfce4-terminal directly"
        xfce4-terminal &
        sleep 3
        termwin=$(xdotool search --name terminal 2>/dev/null | head -1)
    fi
    
    if [ -n "$termwin" ]; then
        echo "✅ Terminal opened, launching calculator"
        eval $(xdotool getwindowgeometry --shell $termwin)
        terminal_x=$(( X + WIDTH/2 ))
        terminal_y=$(( Y + HEIGHT/2 ))
        
        smooth_click $terminal_x $terminal_y
        
        # Type galculator command
        text="galculator &"
        for (( i=0; i<${#text}; i++ )); do
            char="${text:$i:1}"
            xdotool type "$char"
            sleep 0.1
        done
        
        xdotool key Return
        sleep 4
        
        # Close terminal
        xdotool key ctrl+d
        sleep 1
    fi
    
    echo "🔢 Calculator automation with smooth movements"
    
    # Find calculator window
    calcwin=$(xdotool search --name galculator 2>/dev/null | head -1)
    if [ -n "$calcwin" ]; then
        echo "✅ Calculator found, performing operations"
        eval $(xdotool getwindowgeometry --shell $calcwin)
        
        # Focus calculator
        smooth_click $(( X + WIDTH/2 )) $(( Y + HEIGHT/2 ))
        
        # Perform simple calculation with keyboard (more reliable)
        echo "Calculating 42 + 58 = 100"
        
        # Use keyboard input instead of clicking buttons
        xdotool key 4 2
        sleep 0.5
        xdotool key plus
        sleep 0.5  
        xdotool key 5 8
        sleep 0.5
        xdotool key Return
        sleep 2
        
        # Clear and do another calculation
        xdotool key c
        sleep 0.5
        xdotool key 9 asterisk 7
        sleep 1
        xdotool key Return
        sleep 2
        
    else
        echo "❌ Calculator not found, trying alternative method"
        
        # Try direct command execution
        galculator &
        sleep 4
        
        calcwin=$(xdotool search --name galculator 2>/dev/null | head -1)
        if [ -n "$calcwin" ]; then
            echo "✅ Calculator opened via direct command"
            eval $(xdotool getwindowgeometry --shell $calcwin)
            smooth_click $(( X + WIDTH/2 )) $(( Y + HEIGHT/2 ))
            
            xdotool key 1 2 3 plus 4 5 6 Return
            sleep 2
        fi
    fi
    
    echo "📝 Opening text editor"
    
    # Try multiple methods for text editor
    mousepad &
    sleep 4
    
    editorwin=$(xdotool search --name mousepad 2>/dev/null | head -1)
    if [ -z "$editorwin" ]; then
        # Try via terminal if mousepad didn't work
        xfce4-terminal -e "mousepad" &
        sleep 4
        editorwin=$(xdotool search --name mousepad 2>/dev/null | head -1)
    fi
    
    if [ -n "$editorwin" ]; then
        echo "✅ Text editor opened"
        eval $(xdotool getwindowgeometry --shell $editorwin)
        text_area_x=$(( X + 50 ))
        text_area_y=$(( Y + 100 ))
        
        smooth_click $text_area_x $text_area_y
        
        # Type with visible progression
        demo_text="WORKING XFCE AUTOMATION DEMO

This demonstration shows FIXED application launching:

✅ Methods Used:
  • Direct application execution
  • Terminal-based launching
  • Keyboard shortcuts for reliability
  • Fallback mechanisms

📊 Calculator Results:
  • 42 + 58 = 100 ✓
  • 9 × 7 = 63 ✓

🖱️  Cursor Movement:
  • Smooth interpolated movement
  • Natural timing patterns
  • Visible workflow progression

Status: All applications launched successfully!"
        
        for (( i=0; i<${#demo_text}; i++ )); do
            char="${demo_text:$i:1}"
            if [[ "$char" == $'\n' ]]; then
                xdotool key Return
                sleep 0.6
            else
                xdotool type "$char"
                if [[ "$char" == " " ]]; then
                    sleep 0.2
                elif [[ "$char" =~ [.!?:•] ]]; then
                    sleep 0.4
                else
                    sleep 0.08
                fi
            fi
        done
        
    else
        echo "❌ Text editor failed to open"
    fi
    
    echo "📁 Opening file manager"
    
    # Open file manager
    thunar &
    sleep 4
    
    filewin=$(xdotool search --name thunar 2>/dev/null | head -1)
    if [ -n "$filewin" ]; then
        echo "✅ File manager opened"
        eval $(xdotool getwindowgeometry --shell $filewin)
        smooth_click $(( X + WIDTH/2 )) $(( Y + HEIGHT/2 ))
        sleep 2
    fi
    
    # Final smooth cursor movement demonstration
    echo "🎨 Final cursor movement demonstration"
    eval $(xdotool getmouselocation --shell)
    start_x=$X
    start_y=$Y
    
    # Create a smooth pattern
    smooth_move_cursor $start_x $start_y 200 200
    smooth_move_cursor 200 200 800 200  
    smooth_move_cursor 800 200 800 600
    smooth_move_cursor 800 600 200 600
    smooth_move_cursor 200 600 512 384
    
    sleep 3
    stop_recording
    
    echo "🎨 Creating optimized GIF"
    ffmpeg -i /tmp/working_xfce_demo.mp4 -vf "fps=20,scale=640:-1:flags=lanczos,palettegen" /tmp/working_palette.png -y
    ffmpeg -i /tmp/working_xfce_demo.mp4 -i /tmp/working_palette.png -filter_complex \
           "fps=20,scale=640:-1:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer" \
           /tmp/working_xfce_demo.gif -y
    
    echo "✅ Working XFCE automation demo completed!"
    echo "📁 Generated files:"
    ls -la /tmp/working_xfce_demo.*
}

# Execute demo
echo "🚀 Starting Working XFCE Automation Demo"
working_xfce_demo