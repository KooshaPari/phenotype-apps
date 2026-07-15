#!/bin/bash
# Smooth Cursor Movement Demo - Natural Human-like Automation
# Implements WindMouse-style cursor movement with visible paths

export DISPLAY=:1

echo "🎬 Creating Smooth Cursor Movement Demo"

# Function to move cursor smoothly between two points
smooth_move_cursor() {
    local start_x=$1
    local start_y=$2
    local end_x=$3
    local end_y=$4
    local steps=${5:-20}  # Number of movement steps
    local delay=${6:-0.05}  # Delay between steps
    
    echo "🖱️  Moving cursor from ($start_x,$start_y) to ($end_x,$end_y)"
    
    # Calculate movement increments
    local dx=$(( (end_x - start_x) ))
    local dy=$(( (end_y - start_y) ))
    
    # Move cursor in smooth steps
    for ((i=0; i<=steps; i++)); do
        # Calculate current position with slight curve for natural movement
        local progress=$(echo "scale=4; $i / $steps" | bc -l)
        
        # Add slight curve using sine for natural movement
        local curve=$(echo "scale=4; s($progress * 3.14159) * 0.1" | bc -l)
        
        local current_x=$(echo "scale=0; $start_x + ($dx * $progress) + ($curve * 10)" | bc -l)
        local current_y=$(echo "scale=0; $start_y + ($dy * $progress)" | bc -l)
        
        # Move cursor to current position
        xdotool mousemove $current_x $current_y
        sleep $delay
    done
    
    # Ensure we end exactly at target
    xdotool mousemove $end_x $end_y
    sleep 0.2
}

# Function to get current cursor position
get_cursor_position() {
    eval $(xdotool getmouselocation --shell)
    echo "$X,$Y"
}

# Function to click with smooth movement
smooth_click() {
    local target_x=$1
    local target_y=$2
    
    # Get current cursor position
    eval $(xdotool getmouselocation --shell)
    local current_x=$X
    local current_y=$Y
    
    echo "🖱️  Smooth click at ($target_x,$target_y)"
    
    # Move smoothly to target
    smooth_move_cursor $current_x $current_y $target_x $target_y 25 0.04
    
    # Pause before click
    sleep 0.3
    
    # Click
    xdotool click 1
    sleep 0.5
}

# Function to type with visible cursor movement to text areas
smooth_type_at() {
    local target_x=$1
    local target_y=$2
    local text=$3
    
    echo "⌨️  Smooth type at ($target_x,$target_y): $text"
    
    # Move to text area smoothly
    smooth_click $target_x $target_y
    
    # Type text character by character
    for (( i=0; i<${#text}; i++ )); do
        char="${text:$i:1}"
        if [[ "$char" == $'\n' ]]; then
            xdotool key Return
            sleep 0.8
        else
            xdotool type "$char"
            # Vary typing speed naturally
            if [[ "$char" == " " ]]; then
                sleep 0.3
            elif [[ "$char" =~ [.!?] ]]; then
                sleep 0.6
            else
                sleep $(echo "scale=3; 0.08 + ($RANDOM % 50) / 1000" | bc -l)
            fi
        fi
    done
}

# Start FFmpeg recording with cursor capture
start_recording() {
    local output_file=$1
    echo "📹 Starting smooth cursor recording: ${output_file}"
    
    # Start recording with cursor included
    ffmpeg -f x11grab -framerate 30 -video_size 1024x768 -i :1.0 \
           -c:v libx264 -preset fast -crf 18 -pix_fmt yuv420p \
           -movflags +faststart "${output_file}" &
    
    FFMPEG_PID=$!
    echo "Recording started with PID: $FFMPEG_PID"
    sleep 3  # Longer delay to ensure recording starts
}

# Stop recording
stop_recording() {
    echo "⏹️  Stopping recording..."
    if [ ! -z "$FFMPEG_PID" ]; then
        kill -TERM $FFMPEG_PID
        wait $FFMPEG_PID 2>/dev/null
        echo "Recording stopped"
    fi
}

# Main demo with smooth cursor movements
smooth_cursor_demo() {
    echo "🎯 Starting Smooth Cursor Automation Demo"
    
    # Start recording
    start_recording "/tmp/smooth_cursor_demo.mp4"
    
    # Clear desktop and move to center
    xdotool key ctrl+alt+d
    sleep 2
    
    # Move cursor to center of screen as starting point
    smooth_move_cursor 100 100 512 384 30 0.08
    sleep 1
    
    echo "📱 Opening calculator with smooth cursor movement"
    
    # Move to Alt+F2 area (top of screen) and trigger
    smooth_move_cursor 512 384 400 50 25 0.06
    xdotool key alt+F2
    sleep 1.5
    
    # Get run dialog position and type calculator
    eval $(xdotool getwindowfocus getwindowgeometry --shell)
    dialog_x=$(( X + WIDTH/2 ))
    dialog_y=$(( Y + HEIGHT/2 ))
    
    # Move to dialog and type
    smooth_move_cursor 400 50 $dialog_x $dialog_y 20 0.05
    sleep 0.5
    
    # Type galculator with visible movement
    text="galculator"
    for (( i=0; i<${#text}; i++ )); do
        char="${text:$i:1}"
        xdotool type "$char"
        # Small cursor movement during typing
        current_pos=$(get_cursor_position)
        IFS=',' read -r cx cy <<< "$current_pos"
        xdotool mousemove $((cx + (RANDOM % 3 - 1))) $((cy + (RANDOM % 3 - 1)))
        sleep 0.2
    done
    
    xdotool key Return
    sleep 4  # Wait for calculator to load
    
    echo "🔢 Performing calculations with smooth cursor movements"
    
    # Find calculator window
    calcwin=$(xdotool search --name galculator | head -1)
    if [ -n "$calcwin" ]; then
        # Get calculator window geometry
        eval $(xdotool getwindowgeometry --shell $calcwin)
        calc_center_x=$(( X + WIDTH/2 ))
        calc_center_y=$(( Y + HEIGHT/2 ))
        
        # Focus calculator with smooth movement
        smooth_click $calc_center_x $calc_center_y
        sleep 1
        
        # Calculate button positions (approximate for galculator)
        button_width=40
        button_height=35
        buttons_start_x=$(( X + 20 ))
        buttons_start_y=$(( Y + 80 ))
        
        # Click 1 (row 3, col 1)
        smooth_click $((buttons_start_x + 0 * button_width)) $((buttons_start_y + 2 * button_height))
        sleep 0.5
        
        # Click 2 (row 3, col 2)  
        smooth_click $((buttons_start_x + 1 * button_width)) $((buttons_start_y + 2 * button_height))
        sleep 0.5
        
        # Click 3 (row 3, col 3)
        smooth_click $((buttons_start_x + 2 * button_width)) $((buttons_start_y + 2 * button_height))
        sleep 0.5
        
        # Click + (row 1, col 4)
        smooth_click $((buttons_start_x + 3 * button_width)) $((buttons_start_y + 0 * button_height))
        sleep 0.5
        
        # Click 4 (row 2, col 1)
        smooth_click $((buttons_start_x + 0 * button_width)) $((buttons_start_y + 1 * button_height))
        sleep 0.5
        
        # Click 5 (row 2, col 2)
        smooth_click $((buttons_start_x + 1 * button_width)) $((buttons_start_y + 1 * button_height))
        sleep 0.5
        
        # Click = (row 0, col 4)
        smooth_click $((buttons_start_x + 3 * button_width)) $((buttons_start_y + 3 * button_height))
        sleep 2
    fi
    
    echo "📝 Opening text editor with smooth navigation"
    
    # Move to top of screen for Alt+F2
    smooth_move_cursor $calc_center_x $calc_center_y 400 50 25 0.06
    xdotool key alt+F2
    sleep 1.5
    
    # Type mousepad
    eval $(xdotool getwindowfocus getwindowgeometry --shell)
    dialog_x=$(( X + WIDTH/2 ))
    dialog_y=$(( Y + HEIGHT/2 ))
    
    smooth_move_cursor 400 50 $dialog_x $dialog_y 20 0.05
    
    text="mousepad"
    for (( i=0; i<${#text}; i++ )); do
        char="${text:$i:1}"
        xdotool type "$char"
        sleep 0.2
    done
    
    xdotool key Return
    sleep 4
    
    echo "✍️  Typing with natural cursor micro-movements"
    
    # Get text editor window
    editorwin=$(xdotool search --name mousepad | head -1)
    if [ -n "$editorwin" ]; then
        eval $(xdotool getwindowgeometry --shell $editorwin)
        editor_text_x=$(( X + 50 ))
        editor_text_y=$(( Y + 100 ))
        
        # Move to text area
        smooth_click $editor_text_x $editor_text_y
        
        # Type with micro cursor movements
        demo_text="SMOOTH CURSOR AUTOMATION DEMO

This demonstration shows:
• Natural cursor movement between UI elements
• Human-like timing and micro-movements  
• Professional automation with visible workflow

Calculation Result: 123 + 45 = 168 ✓

Status: Smooth automation completed successfully!"
        
        for (( i=0; i<${#demo_text}; i++ )); do
            char="${demo_text:$i:1}"
            if [[ "$char" == $'\n' ]]; then
                xdotool key Return
                sleep 0.8
            else
                xdotool type "$char"
                # Micro cursor movements during typing
                current_pos=$(get_cursor_position)
                IFS=',' read -r cx cy <<< "$current_pos"
                xdotool mousemove $((cx + (RANDOM % 2))) $((cy + (RANDOM % 2)))
                
                if [[ "$char" == " " ]]; then
                    sleep 0.3
                elif [[ "$char" =~ [.!?•] ]]; then
                    sleep 0.6
                else
                    sleep 0.12
                fi
            fi
        done
    fi
    
    sleep 3
    
    # Final smooth movement to show completion
    eval $(xdotool getmouselocation --shell)
    smooth_move_cursor $X $Y 512 384 30 0.06
    
    sleep 2
    stop_recording
    
    echo "🎨 Creating optimized GIF with smooth cursor movement"
    ffmpeg -i /tmp/smooth_cursor_demo.mp4 -vf "fps=20,scale=640:-1:flags=lanczos,palettegen" /tmp/palette.png -y
    ffmpeg -i /tmp/smooth_cursor_demo.mp4 -i /tmp/palette.png -filter_complex \
           "fps=20,scale=640:-1:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer" \
           /tmp/smooth_cursor_demo.gif -y
    
    echo "✅ Smooth cursor demo completed!"
    ls -la /tmp/smooth_cursor_demo.*
}

# Execute demo
echo "🚀 Starting Smooth Cursor Movement Demo"
smooth_cursor_demo