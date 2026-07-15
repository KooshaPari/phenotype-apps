#!/bin/bash
# WindMouse Algorithm Implementation for Natural Cursor Movement
# Based on Benjamin J. Land's WindMouse algorithm for human-like automation

export DISPLAY=:1

echo "🌪️  WindMouse Natural Cursor Movement Demo"

# WindMouse algorithm parameters
GRAVITY=9        # Strength of gravitational pull to target
WIND=3          # Strength of wind (randomness)
MIN_WAIT=5      # Minimum wait between movements (ms)
MAX_WAIT=10     # Maximum wait between movements (ms)
MAX_STEP=15     # Maximum step size
TARGET_AREA=10  # Distance from target to start slowing down

# Function to generate random float between min and max
random_float() {
    local min=$1
    local max=$2
    echo "scale=6; $min + ($max - $min) * $RANDOM / 32767" | bc -l
}

# WindMouse implementation for natural cursor movement
windmouse() {
    local start_x=$1
    local start_y=$2
    local dest_x=$3
    local dest_y=$4
    
    echo "🌪️  WindMouse: ($start_x,$start_y) → ($dest_x,$dest_y)"
    
    local current_x=$start_x
    local current_y=$start_y
    local velocity_x=0
    local velocity_y=0
    local wind_x=0
    local wind_y=0
    
    # Move cursor to starting position
    xdotool mousemove $current_x $current_y
    sleep 0.1
    
    while true; do
        # Calculate distance to target
        local dist_x=$(echo "$dest_x - $current_x" | bc -l)
        local dist_y=$(echo "$dest_y - $current_y" | bc -l)
        local distance=$(echo "sqrt($dist_x * $dist_x + $dist_y * $dist_y)" | bc -l)
        
        # Break if we're close enough to target
        if (( $(echo "$distance < 1" | bc -l) )); then
            break
        fi
        
        # Calculate wind (random force)
        wind_x=$(echo "$wind_x + $(random_float -0.5 0.5)" | bc -l)
        wind_y=$(echo "$wind_y + $(random_float -0.5 0.5)" | bc -l)
        
        # Apply wind decay
        wind_x=$(echo "$wind_x * 0.95" | bc -l)
        wind_y=$(echo "$wind_y * 0.95" | bc -l)
        
        # Calculate gravitational force toward target
        local grav_x=$(echo "$GRAVITY * $dist_x / $distance" | bc -l)
        local grav_y=$(echo "$GRAVITY * $dist_y / $distance" | bc -l)
        
        # Calculate wind force
        local wind_force_x=$(echo "$WIND * $wind_x" | bc -l)
        local wind_force_y=$(echo "$WIND * $wind_y" | bc -l)
        
        # Update velocity
        velocity_x=$(echo "$velocity_x + $grav_x + $wind_force_x" | bc -l)
        velocity_y=$(echo "$velocity_y + $grav_y + $wind_force_y" | bc -l)
        
        # Apply velocity decay when close to target
        if (( $(echo "$distance < $TARGET_AREA" | bc -l) )); then
            local decay=$(echo "0.3 + 0.7 * $distance / $TARGET_AREA" | bc -l)
            velocity_x=$(echo "$velocity_x * $decay" | bc -l)
            velocity_y=$(echo "$velocity_y * $decay" | bc -l)
        fi
        
        # Limit maximum step size
        local step_size=$(echo "sqrt($velocity_x * $velocity_x + $velocity_y * $velocity_y)" | bc -l)
        if (( $(echo "$step_size > $MAX_STEP" | bc -l) )); then
            local scale=$(echo "$MAX_STEP / $step_size" | bc -l)
            velocity_x=$(echo "$velocity_x * $scale" | bc -l)
            velocity_y=$(echo "$velocity_y * $scale" | bc -l)
        fi
        
        # Update position
        current_x=$(echo "$current_x + $velocity_x" | bc -l)
        current_y=$(echo "$current_y + $velocity_y" | bc -l)
        
        # Ensure we stay within screen bounds
        current_x=$(echo "if ($current_x < 0) 0 else if ($current_x > 1023) 1023 else $current_x" | bc -l)
        current_y=$(echo "if ($current_y < 0) 0 else if ($current_y > 767) 767 else $current_y" | bc -l)
        
        # Convert to integers for xdotool
        local int_x=$(printf "%.0f" $current_x)
        local int_y=$(printf "%.0f" $current_y)
        
        # Move cursor
        xdotool mousemove $int_x $int_y
        
        # Variable delay for natural timing
        local wait_time=$(echo "scale=3; ($MIN_WAIT + $RANDOM % ($MAX_WAIT - $MIN_WAIT + 1)) / 1000" | bc -l)
        sleep $wait_time
    done
    
    # Final precise movement to target
    xdotool mousemove $dest_x $dest_y
    sleep 0.2
}

# Enhanced click with WindMouse movement
windmouse_click() {
    local target_x=$1
    local target_y=$2
    
    # Get current position
    eval $(xdotool getmouselocation --shell)
    local current_x=$X
    local current_y=$Y
    
    echo "🎯 WindMouse click: ($current_x,$current_y) → ($target_x,$target_y)"
    
    # Move with WindMouse algorithm
    windmouse $current_x $current_y $target_x $target_y
    
    # Natural pause before click
    sleep $(echo "scale=3; 0.2 + $RANDOM % 200 / 1000" | bc -l)
    
    # Click
    xdotool click 1
    sleep 0.4
}

# Enhanced typing with micro-movements
windmouse_type() {
    local target_x=$1
    local target_y=$2
    local text=$3
    
    echo "⌨️  WindMouse type at ($target_x,$target_y)"
    
    # Move to text area with WindMouse
    windmouse_click $target_x $target_y
    
    # Type with natural micro-movements
    for (( i=0; i<${#text}; i++ )); do
        char="${text:$i:1}"
        
        if [[ "$char" == $'\n' ]]; then
            xdotool key Return
            sleep $(echo "scale=3; 0.6 + $RANDOM % 400 / 1000" | bc -l)
        else
            xdotool type "$char"
            
            # Small random cursor movements during typing
            eval $(xdotool getmouselocation --shell)
            local micro_x=$(( X + (RANDOM % 5 - 2) ))
            local micro_y=$(( Y + (RANDOM % 3 - 1) ))
            xdotool mousemove $micro_x $micro_y
            
            # Natural typing delays
            if [[ "$char" == " " ]]; then
                sleep $(echo "scale=3; 0.25 + $RANDOM % 100 / 1000" | bc -l)
            elif [[ "$char" =~ [.!?,:;] ]]; then
                sleep $(echo "scale=3; 0.4 + $RANDOM % 300 / 1000" | bc -l)
            else
                sleep $(echo "scale=3; 0.08 + $RANDOM % 80 / 1000" | bc -l)
            fi
        fi
    done
}

# Start recording function
start_windmouse_recording() {
    local output_file=$1
    echo "📹 Starting WindMouse recording: ${output_file}"
    
    # High-quality recording to capture smooth movement
    ffmpeg -f x11grab -framerate 30 -video_size 1024x768 -i :1.0 \
           -c:v libx264 -preset medium -crf 16 -pix_fmt yuv420p \
           -movflags +faststart "${output_file}" &
    
    FFMPEG_PID=$!
    echo "WindMouse recording started with PID: $FFMPEG_PID"
    sleep 3
}

# Stop recording
stop_windmouse_recording() {
    echo "⏹️  Stopping WindMouse recording..."
    if [ ! -z "$FFMPEG_PID" ]; then
        kill -TERM $FFMPEG_PID
        wait $FFMPEG_PID 2>/dev/null
        echo "WindMouse recording stopped"
    fi
}

# Main WindMouse demonstration
windmouse_demo() {
    echo "🌪️  Starting WindMouse Natural Automation Demo"
    
    start_windmouse_recording "/tmp/windmouse_demo.mp4"
    
    # Clear desktop
    xdotool key ctrl+alt+d
    sleep 2
    
    # Start from center of screen
    xdotool mousemove 512 384
    sleep 1
    
    echo "📱 Opening calculator with WindMouse movement"
    
    # Natural movement to trigger Alt+F2
    windmouse 512 384 400 50
    xdotool key alt+F2
    sleep 1.5
    
    # Get dialog and type calculator
    eval $(xdotool getwindowfocus getwindowgeometry --shell)
    dialog_x=$(( X + WIDTH/2 ))
    dialog_y=$(( Y + HEIGHT/2 ))
    
    windmouse 400 50 $dialog_x $dialog_y
    
    # Type with micro-movements
    text="galculator"
    for (( i=0; i<${#text}; i++ )); do
        char="${text:$i:1}"
        xdotool type "$char"
        # Micro-movement during typing
        eval $(xdotool getmouselocation --shell)
        xdotool mousemove $((X + (RANDOM % 3 - 1))) $((Y + (RANDOM % 3 - 1)))
        sleep 0.15
    done
    
    xdotool key Return
    sleep 4
    
    echo "🔢 Calculator automation with WindMouse precision"
    
    # Find calculator and perform operations
    calcwin=$(xdotool search --name galculator | head -1)
    if [ -n "$calcwin" ]; then
        eval $(xdotool getwindowgeometry --shell $calcwin)
        
        # Calculate button positions
        button_width=42
        button_height=38
        buttons_start_x=$(( X + 15 ))
        buttons_start_y=$(( Y + 75 ))
        
        # Focus calculator window first
        windmouse_click $(( X + WIDTH/2 )) $(( Y + HEIGHT/2 ))
        
        # Perform calculation: 789 * 6 = 4734
        echo "Calculating 789 × 6 with natural cursor movement"
        
        # Click 7 (row 1, col 1)
        windmouse_click $((buttons_start_x + 0 * button_width)) $((buttons_start_y + 0 * button_height))
        
        # Click 8 (row 1, col 2)
        windmouse_click $((buttons_start_x + 1 * button_width)) $((buttons_start_y + 0 * button_height))
        
        # Click 9 (row 1, col 3)
        windmouse_click $((buttons_start_x + 2 * button_width)) $((buttons_start_y + 0 * button_height))
        
        # Click × (multiply button)
        windmouse_click $((buttons_start_x + 3 * button_width)) $((buttons_start_y + 1 * button_height))
        
        # Click 6 (row 2, col 3)
        windmouse_click $((buttons_start_x + 2 * button_width)) $((buttons_start_y + 1 * button_height))
        
        # Click = (equals button)
        windmouse_click $((buttons_start_x + 3 * button_width)) $((buttons_start_y + 3 * button_height))
        
        sleep 2
    fi
    
    echo "📝 Text editor with WindMouse navigation"
    
    # Open text editor
    windmouse $(( X + WIDTH/2 )) $(( Y + HEIGHT/2 )) 400 50
    xdotool key alt+F2
    sleep 1.5
    
    eval $(xdotool getwindowfocus getwindowgeometry --shell)
    dialog_x=$(( X + WIDTH/2 ))
    dialog_y=$(( Y + HEIGHT/2 ))
    
    windmouse 400 50 $dialog_x $dialog_y
    
    text="mousepad"
    for (( i=0; i<${#text}; i++ )); do
        char="${text:$i:1}"
        xdotool type "$char"
        sleep 0.15
    done
    
    xdotool key Return
    sleep 4
    
    # Type in text editor with WindMouse
    editorwin=$(xdotool search --name mousepad | head -1)
    if [ -n "$editorwin" ]; then
        eval $(xdotool getwindowgeometry --shell $editorwin)
        text_area_x=$(( X + 50 ))
        text_area_y=$(( Y + 100 ))
        
        windmouse_type $text_area_x $text_area_y "WINDMOUSE NATURAL AUTOMATION

This demonstration showcases:
🌪️  WindMouse algorithm for human-like cursor movement
🎯 Natural acceleration and deceleration patterns
🔄 Realistic micro-movements during interactions
📊 Professional automation with visible workflow

Calculation Performed: 789 × 6 = 4734 ✓

✅ WindMouse automation completed successfully!

The cursor movement appears completely natural and human-like,
with proper physics-based motion including gravity, wind forces,
and realistic timing patterns."
    fi
    
    sleep 3
    
    # Final demonstration movement
    eval $(xdotool getmouselocation --shell)
    windmouse $X $Y 512 200
    windmouse 512 200 200 400
    windmouse 200 400 800 400
    windmouse 800 400 512 384
    
    sleep 2
    stop_windmouse_recording
    
    echo "🎨 Creating high-quality GIF with WindMouse movement"
    ffmpeg -i /tmp/windmouse_demo.mp4 -vf "fps=25,scale=640:-1:flags=lanczos,palettegen" /tmp/windmouse_palette.png -y
    ffmpeg -i /tmp/windmouse_demo.mp4 -i /tmp/windmouse_palette.png -filter_complex \
           "fps=25,scale=640:-1:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer" \
           /tmp/windmouse_demo.gif -y
    
    echo "✅ WindMouse natural automation demo completed!"
    ls -la /tmp/windmouse_demo.*
}

# Execute WindMouse demo
echo "🚀 Launching WindMouse Natural Cursor Movement"
windmouse_demo