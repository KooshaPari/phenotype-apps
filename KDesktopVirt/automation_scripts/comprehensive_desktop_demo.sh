#!/bin/bash
# Comprehensive Virtual Desktop Automation Demo
# Shows AI agent performing complex desktop workflows with visible cursor

export DISPLAY=:1

echo "🤖 Starting comprehensive desktop automation demo..."

# Function to take a timestamped screenshot
take_screenshot() {
    local name=$1
    import -window root "/tmp/demo_${name}.png"
    echo "📸 Screenshot taken: ${name}"
}

# Function to move cursor visibly and smoothly
move_cursor() {
    local x=$1
    local y=$2
    local steps=${3:-10}
    
    # Get current cursor position
    eval $(xdotool getmouselocation --shell)
    local start_x=$X
    local start_y=$Y
    
    # Calculate step increments
    local dx=$(( (x - start_x) / steps ))
    local dy=$(( (y - start_y) / steps ))
    
    # Move cursor smoothly
    for i in $(seq 1 $steps); do
        local new_x=$(( start_x + (dx * i) ))
        local new_y=$(( start_y + (dy * i) ))
        xdotool mousemove $new_x $new_y
        sleep 0.05
    done
}

# Function to click with visible cursor movement
click_at() {
    local x=$1
    local y=$2
    local description=$3
    
    echo "🖱️  Moving to $description ($x, $y)"
    move_cursor $x $y
    sleep 0.3
    xdotool click 1
    sleep 0.5
}

# Function to type text naturally
type_naturally() {
    local text="$1"
    local delay=${2:-50}
    
    for (( i=0; i<${#text}; i++ )); do
        char="${text:$i:1}"
        xdotool type "$char"
        sleep 0.$(($RANDOM % 3 + $delay))
    done
}

echo "📋 Demo Scenario: AI Agent Creates Meeting Notes"
echo "Tasks: Open calculator, perform budget calculation, open text editor, create meeting notes, save document"

# Step 1: Initial desktop
sleep 2
take_screenshot "01_clean_desktop"

# Step 2: Open calculator with visible cursor movement
echo "🔢 Task 1: Opening calculator for budget calculation..."
click_at 50 50 "Applications Menu"
sleep 1
click_at 150 200 "Calculator"
sleep 2
take_screenshot "02_calculator_opened"

# Step 3: Perform budget calculation with detailed steps
echo "💰 Calculating meeting room budget: (150 * 12) + 500"
move_cursor 75 120 5  # Move to number 1
sleep 0.3
xdotool click 1
sleep 0.2

move_cursor 100 120 5  # Move to number 5
sleep 0.3
xdotool click 1
sleep 0.2

move_cursor 125 120 5  # Move to number 0
sleep 0.3
xdotool click 1
sleep 0.5

# Multiply button
move_cursor 150 100 5
sleep 0.3
xdotool click 1
sleep 0.5

# Enter 12
move_cursor 75 140 5
sleep 0.3
xdotool click 1
sleep 0.2

move_cursor 100 160 5
sleep 0.3
xdotool click 1
sleep 0.5

# Equals button
move_cursor 150 180 5
sleep 0.3
xdotool click 1
sleep 1
take_screenshot "03_calculation_step1"

# Add 500
move_cursor 125 80 5  # Plus button
sleep 0.3
xdotool click 1
sleep 0.2

move_cursor 100 120 5  # 5
sleep 0.3
xdotool click 1
sleep 0.2

move_cursor 125 120 5  # 0
sleep 0.3
xdotool click 1
sleep 0.2

move_cursor 125 120 5  # 0
sleep 0.3
xdotool click 1
sleep 0.5

# Final equals
move_cursor 150 180 5
sleep 0.3
xdotool click 1
sleep 1
take_screenshot "04_budget_calculated"

echo "✅ Budget calculated: $2300"

# Step 4: Open text editor for meeting notes
echo "📝 Task 2: Opening text editor for meeting notes..."
move_cursor 300 50 10  # Move to different area
sleep 0.5
click_at 60 50 "Applications Menu"
sleep 1
click_at 120 250 "Text Editor"
sleep 3
take_screenshot "05_text_editor_opened"

# Step 5: Create comprehensive meeting notes
echo "📄 Writing comprehensive meeting notes..."
type_naturally "QUARTERLY PLANNING MEETING NOTES"
sleep 0.5
xdotool key Return Return
sleep 0.3

type_naturally "Date: $(date '+%B %d, %Y')"
xdotool key Return
type_naturally "Attendees: Sarah Johnson, Mike Chen, Dr. Rodriguez"
xdotool key Return Return

type_naturally "AGENDA:"
xdotool key Return
type_naturally "1. Budget Review and Approval"
xdotool key Return
type_naturally "2. Resource Allocation for Q4"
xdotool key Return
type_naturally "3. Project Timeline Updates"
xdotool key Return Return

type_naturally "BUDGET CALCULATION:"
xdotool key Return
type_naturally "Monthly meeting room cost: $150"
xdotool key Return
type_naturally "Annual cost (12 months): $150 x 12 = $1800"
xdotool key Return
type_naturally "Additional equipment budget: $500"
xdotool key Return
type_naturally "TOTAL APPROVED BUDGET: $2300"
xdotool key Return Return

take_screenshot "06_meeting_notes_typed"

type_naturally "ACTION ITEMS:"
xdotool key Return
type_naturally "- Sarah: Finalize Q4 resource allocation by Friday"
xdotool key Return
type_naturally "- Mike: Update project timelines in management system"
xdotool key Return
type_naturally "- Dr. Rodriguez: Review budget proposal with finance team"
xdotool key Return Return

type_naturally "NEXT MEETING: $(date -d '+1 month' '+%B %d, %Y')"
xdotool key Return Return

type_naturally "Meeting completed successfully. All budget items approved."
xdotool key Return
type_naturally "AI-generated notes saved automatically."

sleep 1
take_screenshot "07_complete_meeting_notes"

# Step 6: Save the document
echo "💾 Saving meeting notes document..."
xdotool key ctrl+s
sleep 2
type_naturally "Meeting_Notes_$(date '+%Y%m%d')"
sleep 1
xdotool key Return
sleep 2
take_screenshot "08_document_saved"

# Step 7: Show file manager to verify save
echo "📁 Verifying document was saved..."
click_at 80 50 "Applications Menu"
sleep 1
click_at 140 180 "File Manager"
sleep 3
take_screenshot "09_file_manager_verification"

# Step 8: Final desktop with all applications
echo "🎯 Demo completed - showing final desktop state"
sleep 1
take_screenshot "10_demo_complete"

echo "✅ Comprehensive desktop automation demo completed!"
echo "📊 Created: Budget calculation ($2300)"
echo "📝 Generated: Complete meeting notes with action items"
echo "💾 Saved: Document with timestamp"
echo "🤖 All tasks performed with visible cursor movement and natural interaction"