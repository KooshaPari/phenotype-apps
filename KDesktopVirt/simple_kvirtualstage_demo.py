#!/usr/bin/env python3
"""
Simple KVirtualStage Demo - Focused on smooth cursor movement
"""

import time
import subprocess
import math
import os
import random

class SmoothCursor:
    def __init__(self, display=":1"):
        self.display = display
        os.environ['DISPLAY'] = display
    
    def smooth_move(self, start_x, start_y, end_x, end_y, duration_ms=1000):
        """Ultra-smooth cursor movement with 50 interpolation steps"""
        steps = 50
        delay = duration_ms / (steps * 1000)  # Convert to seconds
        
        print(f"🖱️  Smooth move: ({start_x},{start_y}) → ({end_x},{end_y})")
        
        for i in range(steps + 1):
            # Smooth easing function
            progress = i / steps
            eased = self.ease_in_out_cubic(progress)
            
            # Calculate position with easing
            x = int(start_x + (end_x - start_x) * eased)
            y = int(start_y + (end_y - start_y) * eased)
            
            # Add slight natural curve
            curve = math.sin(progress * math.pi) * 20
            y_curved = int(y + curve)
            
            subprocess.run(['xdotool', 'mousemove', str(x), str(y_curved)], 
                          capture_output=True)
            time.sleep(delay)
    
    def ease_in_out_cubic(self, t):
        """Smooth cubic easing"""
        if t < 0.5:
            return 4 * t * t * t
        else:
            return 1 - pow(-2 * t + 2, 3) / 2

def main():
    print("🎬 KVirtualStage Smooth Cursor Demo")
    print("Demonstrating smooth movement and user intent fixes")
    
    cursor = SmoothCursor()
    
    # Start recording
    print("📹 Starting recording...")
    rec_cmd = [
        'ffmpeg', '-f', 'x11grab', '-framerate', '30',
        '-video_size', '1920x1080', '-i', ':1.0',
        '-c:v', 'libx264', '-preset', 'fast', '-crf', '18',
        '-pix_fmt', 'yuv420p', '-movflags', '+faststart',
        '-t', '45', '-y', '/tmp/kvirtualstage_smooth_demo.mp4'
    ]
    recording = subprocess.Popen(rec_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    time.sleep(2)
    
    try:
        # Demo 1: Smooth movement to center
        print("Demo 1: Moving to screen center")
        cursor.smooth_move(100, 100, 960, 540, 1500)
        time.sleep(1)
        
        # Demo 2: Open calculator with ENTER KEY FIX
        print("Demo 2: Opening calculator with Enter key fix")
        cursor.smooth_move(960, 540, 100, 50, 1200)
        time.sleep(0.5)
        
        subprocess.run(['xdotool', 'key', 'alt+F2'])
        time.sleep(1)
        subprocess.run(['xdotool', 'key', 'ctrl+a', 'Delete'])
        time.sleep(0.3)
        subprocess.run(['xdotool', 'type', 'galculator'])
        time.sleep(0.8)
        print("✅ PRESSING ENTER (THE FIX!)")
        subprocess.run(['xdotool', 'key', 'Return'])
        time.sleep(3)
        
        # Demo 3: Find calculator and move to it
        print("Demo 3: Finding calculator window")
        result = subprocess.run(['xdotool', 'search', '--name', 'galculator'], 
                              capture_output=True, text=True)
        
        if result.returncode == 0 and result.stdout.strip():
            window_id = result.stdout.strip().split('\n')[0]
            
            # Get window position
            geo_result = subprocess.run(['xdotool', 'getwindowgeometry', '--shell', window_id],
                                      capture_output=True, text=True)
            
            if geo_result.returncode == 0:
                lines = geo_result.stdout.strip().split('\n')
                x = int([line for line in lines if line.startswith('X=')][0].split('=')[1])
                y = int([line for line in lines if line.startswith('Y=')][0].split('=')[1])
                width = int([line for line in lines if line.startswith('WIDTH=')][0].split('=')[1])
                height = int([line for line in lines if line.startswith('HEIGHT=')][0].split('=')[1])
                
                calc_x = x + width // 2
                calc_y = y + height // 2
                
                print(f"Moving smoothly to calculator at ({calc_x}, {calc_y})")
                cursor.smooth_move(100, 50, calc_x, calc_y, 1500)
                
                subprocess.run(['xdotool', 'windowactivate', window_id])
                time.sleep(0.5)
                
                # Quick calculation
                subprocess.run(['xdotool', 'key', '1', '2', '3', 'plus', '4', '5', '6', 'Return'])
                time.sleep(2)
        
        # Demo 4: Text editor with smooth movement
        print("Demo 4: Opening text editor")
        cursor.smooth_move(calc_x, calc_y, 100, 50, 1200)
        time.sleep(0.5)
        
        subprocess.run(['xdotool', 'key', 'alt+F2'])
        time.sleep(1)
        subprocess.run(['xdotool', 'key', 'ctrl+a', 'Delete'])
        time.sleep(0.3)
        subprocess.run(['xdotool', 'type', 'mousepad'])
        time.sleep(0.8)
        print("✅ PRESSING ENTER (THE FIX!)")
        subprocess.run(['xdotool', 'key', 'Return'])
        time.sleep(3)
        
        # Demo 5: Multiple smooth movements across screen
        print("Demo 5: Smooth movement showcase")
        movements = [
            (100, 50, 1800, 100),    # Top-right
            (1800, 100, 1800, 950),  # Bottom-right
            (1800, 950, 120, 950),   # Bottom-left
            (120, 950, 120, 100),    # Top-left
            (120, 100, 960, 540),    # Center
        ]
        
        for i, (x1, y1, x2, y2) in enumerate(movements):
            print(f"Movement {i+1}: ({x1},{y1}) → ({x2},{y2})")
            cursor.smooth_move(x1, y1, x2, y2, 1500)
            time.sleep(0.3)
        
        # Demo 6: Circular pattern
        print("Demo 6: Circular movement pattern")
        center_x, center_y = 960, 540
        radius = 200
        
        for angle in range(0, 720, 10):  # Two full circles
            x = center_x + int(radius * math.cos(math.radians(angle)))
            y = center_y + int(radius * math.sin(math.radians(angle)))
            subprocess.run(['xdotool', 'mousemove', str(x), str(y)])
            time.sleep(0.05)
        
        time.sleep(2)
        
    finally:
        # Stop recording
        print("🛑 Stopping recording...")
        recording.terminate()
        recording.wait()
    
    print("✅ KVirtualStage smooth demo complete!")
    print("")
    print("📊 Demonstrated features:")
    print("   ✅ 50-step smooth cursor interpolation")
    print("   ✅ No cursor jumping - smooth paths visible")
    print("   ✅ Enter key fix for app launching")
    print("   ✅ Multiple smooth movements per workflow")
    print("   ✅ Full 1920x1080 screen coverage")
    print("   ✅ Natural easing and curved movements")
    print("   ✅ Circular motion patterns")
    
    # Check if file was created
    if os.path.exists('/tmp/kvirtualstage_smooth_demo.mp4'):
        size = os.path.getsize('/tmp/kvirtualstage_smooth_demo.mp4')
        print(f"📁 Video created: /tmp/kvirtualstage_smooth_demo.mp4 ({size} bytes)")
    else:
        print("❌ Video file not found")

if __name__ == "__main__":
    main()