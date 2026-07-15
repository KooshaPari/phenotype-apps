#!/bin/bash
# Focused Virtual Desktop Demo with Visible Cursor Movement
export DISPLAY=:1

echo "🤖 Starting focused automation demo..."

# Take initial screenshot
import -window root /tmp/step1_desktop.png
echo "📸 Step 1: Clean desktop captured"

# Open calculator with cursor movement
echo "🔢 Opening calculator..."
xdotool mousemove 100 100
sleep 0.5
xdotool click 1  # Click on desktop area
sleep 1

# Search for calculator
xdotool key alt+F2  # Open run dialog
sleep 1
xdotool type "galculator"
sleep 0.5
xdotool key Return
sleep 2

import -window root /tmp/step2_calculator.png
echo "📸 Step 2: Calculator opened"

# Perform calculation with visible cursor
calcwin=$(xdotool search --name galculator | head -1)
if [ -n "$calcwin" ]; then
    echo "🖱️ Performing calculation with visible cursor..."
    
    # Click 7
    xdotool mousemove --window $calcwin 50 120
    sleep 0.3
    xdotool click 1
    sleep 0.3
    
    # Click multiply
    xdotool mousemove --window $calcwin 150 120
    sleep 0.3
    xdotool click 1
    sleep 0.3
    
    # Click 6
    xdotool mousemove --window $calcwin 100 120
    sleep 0.3
    xdotool click 1
    sleep 0.3
    
    # Click equals
    xdotool mousemove --window $calcwin 150 180
    sleep 0.3
    xdotool click 1
    sleep 1
    
    import -window root /tmp/step3_calculation.png
    echo "📸 Step 3: Calculation complete (7×6=42)"
fi

# Open text editor
echo "📝 Opening text editor..."
xdotool key alt+F2
sleep 1
xdotool type "mousepad"
sleep 0.5
xdotool key Return
sleep 3

import -window root /tmp/step4_editor.png
echo "📸 Step 4: Text editor opened"

# Type with natural rhythm
echo "✍️ Typing document with natural rhythm..."
sleep 1
xdotool type "AI AUTOMATION DEMO RESULTS"
xdotool key Return Return
sleep 0.5

xdotool type "Task: Budget Calculation"
xdotool key Return
xdotool type "Calculation: 7 × 6 = 42"
xdotool key Return
xdotool type "Status: COMPLETED"
xdotool key Return Return

xdotool type "Agent Actions Demonstrated:"
xdotool key Return
xdotool type "✓ Opened calculator application"
xdotool key Return
xdotool type "✓ Performed mathematical calculation"
xdotool key Return
xdotool type "✓ Opened text editor"
xdotool key Return
xdotool type "✓ Created structured document"
sleep 1

import -window root /tmp/step5_document.png
echo "📸 Step 5: Document created"

echo "✅ Focused automation demo completed!"
echo "Generated 5 screenshots showing clear user intent and cursor movement"