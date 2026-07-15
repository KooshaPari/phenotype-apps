#!/bin/bash
# Simple App Launch Test - Minimal script to verify fix
export DISPLAY=:1

echo "🧪 Simple App Launch Test - Verifying Fix"

# Wait for window function
wait_for_app() {
    local app_name=$1
    local max_wait=8
    local count=0
    
    while [ $count -lt $max_wait ]; do
        window=$(xdotool search --name "$app_name" 2>/dev/null | head -1)
        if [ -n "$window" ]; then
            echo "✅ Found $app_name"
            return 0
        fi
        sleep 1
        count=$((count + 1))
    done
    echo "❌ $app_name not found"
    return 1
}

# Start short recording
ffmpeg -f x11grab -framerate 20 -video_size 1024x768 -i :1.0 \
       -c:v libx264 -preset ultrafast -crf 20 \
       -t 30 /tmp/simple_app_test.mp4 &
FFMPEG_PID=$!
sleep 2

echo "🧹 Clear desktop"
xdotool key ctrl+alt+d
sleep 2

echo "🧮 Test 1: Calculator via direct command"
galculator &
sleep 4

if wait_for_app "galculator"; then
    calcwin=$(xdotool search --name galculator | head -1)
    echo "Calculator window ID: $calcwin"
    
    # Focus and do simple calculation
    xdotool windowactivate $calcwin
    sleep 1
    xdotool key 5 plus 3 Return
    sleep 3
    echo "✅ Calculator test PASSED"
else
    echo "❌ Calculator test FAILED"
fi

echo "📝 Test 2: Text editor via direct command"
mousepad &
sleep 4

if wait_for_app "mousepad"; then
    editorwin=$(xdotool search --name mousepad | head -1)
    echo "Text editor window ID: $editorwin"
    
    # Focus and type
    xdotool windowactivate $editorwin
    sleep 1
    xdotool type "Test successful! Apps are launching properly."
    sleep 2
    echo "✅ Text editor test PASSED"
else
    echo "❌ Text editor test FAILED"
fi

sleep 3

# Stop recording
kill -TERM $FFMPEG_PID
wait $FFMPEG_PID 2>/dev/null

echo "✅ Simple test complete"
ls -la /tmp/simple_app_test.mp4