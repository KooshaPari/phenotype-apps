#!/bin/bash
# Professional Video Demo Script with FFmpeg Recording
# Creates smooth, high-quality automation demonstrations

export DISPLAY=:1

echo "🎬 Starting Professional Video Automation Demo"

# Function to start FFmpeg screen recording
start_recording() {
    local output_file=$1
    echo "📹 Starting screen recording: ${output_file}"
    
    # Start FFmpeg screen recording in background
    ffmpeg -f x11grab -framerate 30 -video_size 1024x768 -i :1.0 \
           -c:v libx264 -preset fast -crf 18 -pix_fmt yuv420p \
           -movflags +faststart "${output_file}" &
    
    # Store FFmpeg process ID
    FFMPEG_PID=$!
    echo "FFmpeg recording started with PID: $FFMPEG_PID"
    
    # Give FFmpeg time to start
    sleep 2
}

# Function to stop recording
stop_recording() {
    echo "⏹️  Stopping screen recording..."
    if [ ! -z "$FFMPEG_PID" ]; then
        kill -TERM $FFMPEG_PID
        wait $FFMPEG_PID 2>/dev/null
        echo "Recording stopped and saved"
    fi
}

# Function to create optimized GIF from video
create_optimized_gif() {
    local input_video=$1
    local output_gif=$2
    local fps=${3:-10}
    local scale=${4:-640:-1}
    
    echo "🎨 Creating optimized GIF: ${output_gif}"
    
    # Generate palette for high-quality GIF
    ffmpeg -i "${input_video}" -vf "fps=${fps},scale=${scale}:flags=lanczos,palettegen" /tmp/palette.png -y
    
    # Create GIF with custom palette
    ffmpeg -i "${input_video}" -i /tmp/palette.png -filter_complex \
           "fps=${fps},scale=${scale}:flags=lanczos[x];[x][1:v]paletteuse=dither=bayer:bayer_scale=2" \
           "${output_gif}" -y
    
    echo "✅ GIF created: ${output_gif}"
}

# Function for smooth, visible automation
smooth_automation() {
    echo "🖱️  Performing smooth automation with visible changes"
    
    # Clear desktop first
    xdotool key ctrl+alt+d
    sleep 1
    
    # Demo 1: Open calculator with smooth progression
    echo "📱 Opening calculator application"
    xdotool key alt+F2
    sleep 1
    
    # Type calculator slowly so it's visible
    for char in g a l c u l a t o r; do
        xdotool type "$char"
        sleep 0.3
    done
    
    xdotool key Return
    sleep 3  # Wait for calculator to fully load
    
    # Find calculator window
    calcwin=$(xdotool search --name galculator | head -1)
    if [ -n "$calcwin" ]; then
        echo "🔢 Performing visible calculations"
        
        # Focus calculator
        xdotool windowactivate $calcwin
        sleep 1
        
        # Perform calculation: 123 + 456 = 579
        xdotool key 1; sleep 0.5
        xdotool key 2; sleep 0.5  
        xdotool key 3; sleep 0.5
        xdotool key plus; sleep 0.5
        xdotool key 4; sleep 0.5
        xdotool key 5; sleep 0.5
        xdotool key 6; sleep 0.5
        xdotool key equal; sleep 2
        
        # Clear and do another calculation: 789 * 3 = 2367
        xdotool key c; sleep 1
        xdotool key 7; sleep 0.5
        xdotool key 8; sleep 0.5
        xdotool key 9; sleep 0.5
        xdotool key asterisk; sleep 0.5
        xdotool key 3; sleep 0.5
        xdotool key equal; sleep 2
    fi
    
    # Demo 2: Open text editor and type visible content
    echo "📝 Opening text editor application"
    xdotool key alt+F2
    sleep 1
    
    for char in m o u s e p a d; do
        xdotool type "$char"
        sleep 0.3
    done
    
    xdotool key Return
    sleep 3  # Wait for text editor to load
    
    # Type content with visible progression
    echo "✍️  Typing visible content"
    
    text="AUTOMATED DEMO DOCUMENTATION

This demonstration showcases:
1. Calculator automation with visible calculations
2. Text editor automation with natural typing
3. Professional screen recording capabilities

Results:
- 123 + 456 = 579 ✓
- 789 × 3 = 2367 ✓

Status: Automation completed successfully!"
    
    # Type with realistic delays
    for (( i=0; i<${#text}; i++ )); do
        char="${text:$i:1}"
        if [[ "$char" == $'\n' ]]; then
            xdotool key Return
            sleep 0.8  # Longer pause for new lines
        else
            xdotool type "$char"
            if [[ "$char" == " " ]]; then
                sleep 0.2  # Pause for spaces
            elif [[ "$char" =~ [.!?] ]]; then
                sleep 0.6  # Pause for punctuation
            else
                sleep 0.1  # Normal character delay
            fi
        fi
    done
    
    sleep 2
    
    # Demo 3: Open file manager to show workflow completion
    echo "📁 Opening file manager"
    xdotool key alt+F2
    sleep 1
    
    for char in t h u n a r; do
        xdotool type "$char"
        sleep 0.3
    done
    
    xdotool key Return
    sleep 3
    
    echo "✅ Automation workflow completed"
    sleep 2
}

# Main execution
echo "🎯 Professional Automation Demo Starting"

# Demo 1: Quick Calculator Demo
start_recording "/tmp/calculator_demo.mp4"
echo "🔢 Calculator Demo Recording"

# Clear desktop and start fresh
xdotool key ctrl+alt+d
sleep 2

# Open calculator
xdotool key alt+F2
sleep 1
for char in g a l c u l a t o r; do
    xdotool type "$char"
    sleep 0.2
done
xdotool key Return
sleep 3

# Perform visible calculation
calcwin=$(xdotool search --name galculator | head -1)
if [ -n "$calcwin" ]; then
    xdotool windowactivate $calcwin
    sleep 1
    # 50 + 75 = 125
    xdotool key 5 0; sleep 0.7
    xdotool key plus; sleep 0.7
    xdotool key 7 5; sleep 0.7
    xdotool key equal; sleep 2
fi

sleep 3
stop_recording

# Demo 2: Complete Workflow Demo
start_recording "/tmp/complete_workflow_demo.mp4"
echo "🔄 Complete Workflow Demo Recording"

smooth_automation

stop_recording

# Demo 3: Text Editor Focus Demo
start_recording "/tmp/text_editor_demo.mp4"
echo "📝 Text Editor Demo Recording"

# Clear desktop
xdotool key ctrl+alt+d
sleep 2

# Open text editor
xdotool key alt+F2
sleep 1
for char in m o u s e p a d; do
    xdotool type "$char"
    sleep 0.2
done
xdotool key Return
sleep 3

# Type with very visible progression
demo_text="Professional Automation Demo

This text appears with realistic typing speed.

Each character shows natural human-like timing."

for (( i=0; i<${#demo_text}; i++ )); do
    char="${demo_text:$i:1}"
    if [[ "$char" == $'\n' ]]; then
        xdotool key Return
        sleep 1.0
    else
        xdotool type "$char"
        sleep 0.15
    fi
done

sleep 3
stop_recording

echo "🎬 Creating optimized GIFs from videos"

# Create high-quality GIFs
create_optimized_gif "/tmp/calculator_demo.mp4" "/tmp/calculator_automation.gif" 15 640:-1
create_optimized_gif "/tmp/complete_workflow_demo.mp4" "/tmp/complete_automation_workflow.gif" 12 640:-1
create_optimized_gif "/tmp/text_editor_demo.mp4" "/tmp/text_editor_automation.gif" 15 640:-1

echo "✅ Professional video demonstrations completed!"
echo "📁 Generated files:"
echo "   - calculator_demo.mp4"
echo "   - complete_workflow_demo.mp4"
echo "   - text_editor_demo.mp4"
echo "   - calculator_automation.gif"
echo "   - complete_automation_workflow.gif"
echo "   - text_editor_automation.gif"