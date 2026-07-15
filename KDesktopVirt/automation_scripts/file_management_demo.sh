#!/bin/bash
# File Management Demo - Shows AI Agent Organizing Files
export DISPLAY=:1

echo "🤖 Starting file management automation demo..."

# Take initial screenshot
import -window root /tmp/file_demo_01_desktop.png
echo "📸 Step 1: Initial desktop"

# Open file manager
echo "📁 Opening file manager..."
xdotool key alt+F2
sleep 1
xdotool type "thunar"
sleep 0.5
xdotool key Return
sleep 3

import -window root /tmp/file_demo_02_filemanager.png
echo "📸 Step 2: File manager opened"

# Create a new folder
echo "📂 Creating project folder..."
xdotool key ctrl+shift+n
sleep 1
xdotool type "AI_Projects"
sleep 0.5
xdotool key Return
sleep 2

import -window root /tmp/file_demo_03_folder_created.png
echo "📸 Step 3: Project folder created"

# Navigate into the folder
echo "🔍 Navigating into project folder..."
xdotool key Return
sleep 2

# Create subfolders
echo "📁 Creating organized subfolder structure..."
xdotool key ctrl+shift+n
sleep 1
xdotool type "Documentation"
sleep 0.5
xdotool key Return
sleep 1

xdotool key ctrl+shift+n
sleep 1
xdotool type "Source_Code"
sleep 0.5
xdotool key Return
sleep 1

xdotool key ctrl+shift+n
sleep 1
xdotool type "Testing"
sleep 0.5
xdotool key Return
sleep 2

import -window root /tmp/file_demo_04_subfolders.png
echo "📸 Step 4: Organized folder structure created"

# Open text editor to create a README file
echo "📝 Creating project README file..."
xdotool key alt+F2
sleep 1
xdotool type "mousepad"
sleep 0.5
xdotool key Return
sleep 3

# Type README content
xdotool type "# AI Projects Directory"
xdotool key Return Return
xdotool type "This directory contains AI automation projects."
xdotool key Return Return
xdotool type "Structure:"
xdotool key Return
xdotool type "- Documentation/ - Project documentation"
xdotool key Return
xdotool type "- Source_Code/ - Application source files"
xdotool key Return
xdotool type "- Testing/ - Test files and results"
xdotool key Return Return
xdotool type "Created by AI automation on $(date)"
sleep 2

import -window root /tmp/file_demo_05_readme.png
echo "📸 Step 5: README file created"

# Save the file
echo "💾 Saving README file..."
xdotool key ctrl+s
sleep 2
xdotool type "README.md"
sleep 1
xdotool key Return
sleep 2

import -window root /tmp/file_demo_06_saved.png
echo "📸 Step 6: File saved and organized"

echo "✅ File management demo completed!"
echo "🎯 User Intent Demonstrated: Organizing project files"
echo "📊 Actions: Created folders, subfolders, and documentation"