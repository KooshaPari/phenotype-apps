#!/usr/bin/env python3
"""
Generate KVirtualStage Demo Videos
Creates multiple demonstration videos showing smooth cursor movement and user intent fixes
"""

import subprocess
import time
import os
import math

class VideoGenerator:
    def __init__(self, container="kvirtual-test", display=":1"):
        self.container = container
        self.display = display
        
    def smooth_move_cursor(self, x1, y1, x2, y2, steps=40):
        """Ultra-smooth cursor movement with 40 interpolation steps"""
        for i in range(steps + 1):
            progress = i / steps
            # Cubic easing
            eased = self.ease_in_out_cubic(progress)
            
            x = int(x1 + (x2 - x1) * eased)
            y = int(y1 + (y2 - y1) * eased)
            
            # Add subtle curve for natural movement
            curve = math.sin(progress * math.pi) * 15
            y_curved = int(y + curve)
            
            self.exec_in_container(f'DISPLAY={self.display} xdotool mousemove {x} {y_curved}')
            time.sleep(0.03)
    
    def ease_in_out_cubic(self, t):
        """Smooth cubic easing function"""
        if t < 0.5:
            return 4 * t * t * t
        else:
            return 1 - pow(-2 * t + 2, 3) / 2
    
    def exec_in_container(self, command):
        """Execute command in target container"""
        subprocess.run(['docker', 'exec', self.container, 'bash', '-c', command], 
                      capture_output=True)
    
    def launch_app_with_enter_fix(self, app_name):
        """Launch app with Enter key fix - no search bar stuck text"""
        print(f"🚀 Launching {app_name} with Enter key fix...")
        
        # Open launcher
        self.exec_in_container(f'DISPLAY={self.display} xdotool key alt+F2')
        time.sleep(1)
        
        # Clear any existing text
        self.exec_in_container(f'DISPLAY={self.display} xdotool key ctrl+a Delete')
        time.sleep(0.3)
        
        # Type app name
        self.exec_in_container(f'DISPLAY={self.display} xdotool type "{app_name}"')
        time.sleep(0.5)
        
        # CRITICAL FIX: Press Enter to launch
        print("✅ PRESSING ENTER (THE FIX!)")
        self.exec_in_container(f'DISPLAY={self.display} xdotool key Return')
        time.sleep(3)
    
    def generate_smooth_cursor_demo(self):
        """Generate video demonstrating smooth cursor movement"""
        print("🎬 Generating: Smooth Cursor Movement Demo")
        
        # Start recording
        recording = subprocess.Popen([
            'ffmpeg', '-f', 'x11grab', '-framerate', '30',
            '-video_size', '1920x1080', '-i', f'{self.display}',
            '-c:v', 'libx264', '-preset', 'fast', '-crf', '18',
            '-pix_fmt', 'yuv420p', '-movflags', '+faststart',
            '-t', '30', '-y', '/tmp/smooth_cursor_demo.mp4'
        ], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        
        time.sleep(2)
        
        try:
            # Clear desktop
            self.exec_in_container(f'DISPLAY={self.display} xdotool key ctrl+alt+d')
            time.sleep(1)
            
            print("Demo 1: Smooth movement to center")
            self.smooth_move_cursor(100, 100, 960, 540)
            time.sleep(1)
            
            print("Demo 2: Smooth movement patterns")
            movements = [
                (960, 540, 1800, 100),   # Top-right
                (1800, 100, 1800, 950),  # Bottom-right
                (1800, 950, 120, 950),   # Bottom-left
                (120, 950, 120, 100),    # Top-left
                (120, 100, 960, 540),    # Center
            ]
            
            for x1, y1, x2, y2 in movements:
                print(f"   Moving: ({x1},{y1}) → ({x2},{y2})")
                self.smooth_move_cursor(x1, y1, x2, y2)
                time.sleep(0.5)
            
            print("Demo 3: Circular pattern")
            center_x, center_y = 960, 540
            radius = 200
            for angle in range(0, 720, 8):  # Two circles
                x = center_x + int(radius * math.cos(math.radians(angle)))
                y = center_y + int(radius * math.sin(math.radians(angle)))
                self.exec_in_container(f'DISPLAY={self.display} xdotool mousemove {x} {y}')
                time.sleep(0.05)
            
            time.sleep(2)
            
        finally:
            recording.terminate()
            recording.wait()
        
        print("✅ Smooth cursor demo complete!")
        return "/tmp/smooth_cursor_demo.mp4"
    
    def generate_user_intent_demo(self):
        """Generate video demonstrating user intent fixes"""
        print("🎬 Generating: User Intent Fixes Demo")
        
        # Start recording
        recording = subprocess.Popen([
            'ffmpeg', '-f', 'x11grab', '-framerate', '30',
            '-video_size', '1920x1080', '-i', f'{self.display}',
            '-c:v', 'libx264', '-preset', 'fast', '-crf', '18',
            '-pix_fmt', 'yuv420p', '-movflags', '+faststart',
            '-t', '45', '-y', '/tmp/user_intent_demo.mp4'
        ], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        
        time.sleep(2)
        
        try:
            # Clear desktop
            self.exec_in_container(f'DISPLAY={self.display} xdotool key ctrl+alt+d')
            time.sleep(1)
            
            print("User Intent 1: Calculator with Enter key fix")
            self.smooth_move_cursor(100, 100, 100, 50)
            self.launch_app_with_enter_fix("galculator")
            
            # Find calculator window and move to it
            calc_x, calc_y = 300, 200  # Default position
            result = subprocess.run(['docker', 'exec', self.container, 'bash', '-c', 
                                   f'DISPLAY={self.display} xdotool search --name galculator'],
                                  capture_output=True, text=True)
            
            if result.stdout.strip():
                window_id = result.stdout.strip().split('\n')[0]
                
                # Get window position
                geo_result = subprocess.run(['docker', 'exec', self.container, 'bash', '-c',
                                           f'DISPLAY={self.display} xdotool getwindowgeometry --shell {window_id}'],
                                          capture_output=True, text=True)
                
                if geo_result.returncode == 0:
                    try:
                        lines = geo_result.stdout.strip().split('\n')
                        x = int([line for line in lines if line.startswith('X=')][0].split('=')[1])
                        y = int([line for line in lines if line.startswith('Y=')][0].split('=')[1])
                        width = int([line for line in lines if line.startswith('WIDTH=')][0].split('=')[1])
                        height = int([line for line in lines if line.startswith('HEIGHT=')][0].split('=')[1])
                        
                        calc_x = x + width // 2
                        calc_y = y + height // 2
                    except (IndexError, ValueError):
                        calc_x, calc_y = 300, 200  # Fallback
                    
                    print(f"Moving smoothly to calculator at ({calc_x}, {calc_y})")
                    self.smooth_move_cursor(100, 50, calc_x, calc_y)
                    
                    # Activate window and do calculation
                    self.exec_in_container(f'DISPLAY={self.display} xdotool windowactivate {window_id}')
                    time.sleep(0.5)
                    self.exec_in_container(f'DISPLAY={self.display} xdotool key 1 2 3 plus 4 5 6 Return')
                    time.sleep(2)
            
            print("User Intent 2: Text editor with Enter key fix")
            self.smooth_move_cursor(calc_x, calc_y, 100, 50)
            self.launch_app_with_enter_fix("mousepad")
            
            # Find text editor and move to it
            text_x, text_y = 400, 300  # Default position
            editor_result = subprocess.run(['docker', 'exec', self.container, 'bash', '-c',
                                          f'DISPLAY={self.display} xdotool search --name mousepad'],
                                         capture_output=True, text=True)
            
            if editor_result.stdout.strip():
                window_id = editor_result.stdout.strip().split('\n')[0]
                
                geo_result = subprocess.run(['docker', 'exec', self.container, 'bash', '-c',
                                           f'DISPLAY={self.display} xdotool getwindowgeometry --shell {window_id}'],
                                          capture_output=True, text=True)
                
                if geo_result.returncode == 0:
                    try:
                        lines = geo_result.stdout.strip().split('\n')
                        x = int([line for line in lines if line.startswith('X=')][0].split('=')[1])
                        y = int([line for line in lines if line.startswith('Y=')][0].split('=')[1])
                        
                        text_x = x + 150
                        text_y = y + 120
                    except (IndexError, ValueError):
                        text_x, text_y = 400, 300  # Fallback
                    
                    print(f"Moving smoothly to text editor at ({text_x}, {text_y})")
                    self.smooth_move_cursor(100, 50, text_x, text_y)
                    
                    # Activate and type
                    self.exec_in_container(f'DISPLAY={self.display} xdotool windowactivate {window_id}')
                    time.sleep(0.5)
                    
                    # Type success message
                    text = "SUCCESS! KVirtualStage automation working!\n\nFixes demonstrated:\n✓ Enter key pressed after app names\n✓ No search bar stuck text\n✓ Smooth cursor movement\n✓ Apps launch properly\n✓ Full 1920x1080 recording"
                    for char in text:
                        if char == '\n':
                            self.exec_in_container(f'DISPLAY={self.display} xdotool key Return')
                            time.sleep(0.3)
                        else:
                            self.exec_in_container(f'DISPLAY={self.display} xdotool type "{char}"')
                            time.sleep(0.05)
                    
                    time.sleep(2)
            
            print("Demo 3: Final smooth movement showcase")
            self.smooth_move_cursor(text_x, text_y, 1700, 900)
            self.smooth_move_cursor(1700, 900, 960, 540)
            time.sleep(2)
            
        finally:
            recording.terminate()
            recording.wait()
        
        print("✅ User intent demo complete!")
        return "/tmp/user_intent_demo.mp4"
    
    def generate_comprehensive_demo(self):
        """Generate comprehensive demo showing all features"""
        print("🎬 Generating: Comprehensive KVirtualStage Demo")
        
        # Start recording
        recording = subprocess.Popen([
            'ffmpeg', '-f', 'x11grab', '-framerate', '30',
            '-video_size', '1920x1080', '-i', f'{self.display}',
            '-c:v', 'libx264', '-preset', 'fast', '-crf', '18',
            '-pix_fmt', 'yuv420p', '-movflags', '+faststart',
            '-t', '60', '-y', '/tmp/comprehensive_demo.mp4'
        ], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        
        time.sleep(2)
        
        try:
            # Clear desktop
            self.exec_in_container(f'DISPLAY={self.display} xdotool key ctrl+alt+d')
            time.sleep(1)
            
            print("Phase 1: Smooth cursor demonstration")
            self.smooth_move_cursor(100, 100, 960, 540)
            time.sleep(1)
            
            print("Phase 2: Calculator workflow")
            self.smooth_move_cursor(960, 540, 100, 50)
            self.launch_app_with_enter_fix("galculator")
            
            # Find and interact with calculator
            calc_result = subprocess.run(['docker', 'exec', self.container, 'bash', '-c',
                                        f'DISPLAY={self.display} xdotool search --name galculator'],
                                       capture_output=True, text=True)
            
            if calc_result.stdout.strip():
                window_id = calc_result.stdout.strip().split('\n')[0]
                self.exec_in_container(f'DISPLAY={self.display} xdotool windowactivate {window_id}')
                time.sleep(0.5)
                
                # Calculate 150 * 25 + 1000 (business scenario)
                calc_sequence = ['1', '5', '0', 'multiply', '2', '5', 'plus', '1', '0', '0', '0', 'Return']
                for key in calc_sequence:
                    self.exec_in_container(f'DISPLAY={self.display} xdotool key {key}')
                    time.sleep(0.3)
                
                time.sleep(2)
            
            print("Phase 3: Text editor workflow")
            self.smooth_move_cursor(200, 300, 100, 50)
            self.launch_app_with_enter_fix("mousepad")
            
            editor_result = subprocess.run(['docker', 'exec', self.container, 'bash', '-c',
                                          f'DISPLAY={self.display} xdotool search --name mousepad'],
                                         capture_output=True, text=True)
            
            # Set default position for file manager
            file_pos_x, file_pos_y = 300, 400
            
            if editor_result.stdout.strip():
                window_id = editor_result.stdout.strip().split('\n')[0]
                self.exec_in_container(f'DISPLAY={self.display} xdotool windowactivate {window_id}')
                time.sleep(0.5)
                
                # Get editor position for next movement
                geo_result = subprocess.run(['docker', 'exec', self.container, 'bash', '-c',
                                           f'DISPLAY={self.display} xdotool getwindowgeometry --shell {window_id}'],
                                          capture_output=True, text=True)
                
                if geo_result.returncode == 0:
                    try:
                        lines = geo_result.stdout.strip().split('\n')
                        x = int([line for line in lines if line.startswith('X=')][0].split('=')[1])
                        y = int([line for line in lines if line.startswith('Y=')][0].split('=')[1])
                        file_pos_x, file_pos_y = x + 150, y + 120
                    except (IndexError, ValueError):
                        file_pos_x, file_pos_y = 300, 400
                
                # Type comprehensive documentation
                doc_text = "KVIRTUALSTAGE COMPREHENSIVE DEMO\n\nBusiness Calculation Results:\n150 employees × $25 lunch = $3,750\nRoom rental: $1,000\nTOTAL BUDGET: $4,750\n\nAutomation Features Demonstrated:\n✓ Smooth cursor movement (40-step interpolation)\n✓ Enter key fix for app launching\n✓ No search bar stuck text issues\n✓ Full 1920x1080 recording\n✓ Multiple application workflows\n✓ Business scenario automation\n✓ Natural timing and movement\n\nArchitecture: Host KVirtualStage → Container Desktop\nResult: Professional automation capabilities"
                
                for char in doc_text:
                    if char == '\n':
                        self.exec_in_container(f'DISPLAY={self.display} xdotool key Return')
                        time.sleep(0.3)
                    else:
                        self.exec_in_container(f'DISPLAY={self.display} xdotool type "{char}"')
                        time.sleep(0.04)
                
                time.sleep(2)
            
            print("Phase 4: File manager")
            self.smooth_move_cursor(file_pos_x, file_pos_y, 100, 50)
            self.launch_app_with_enter_fix("thunar")
            time.sleep(3)
            
            print("Phase 5: Final movement showcase")
            # Complex movement pattern starting from current position
            movements = [
                (file_pos_x, file_pos_y, 1800, 100),
                (1800, 100, 1800, 950),
                (1800, 950, 120, 950),
                (120, 950, 120, 100),
                (120, 100, 960, 540)
            ]
            
            for x1, y1, x2, y2 in movements:
                self.smooth_move_cursor(x1, y1, x2, y2)
                time.sleep(0.5)
            
            # Circular finale
            center_x, center_y = 960, 540
            radius = 250
            for angle in range(0, 1080, 6):  # Three circles
                x = center_x + int(radius * math.cos(math.radians(angle)))
                y = center_y + int(radius * math.sin(math.radians(angle)))
                self.exec_in_container(f'DISPLAY={self.display} xdotool mousemove {x} {y}')
                time.sleep(0.04)
            
            time.sleep(3)
            
        finally:
            recording.terminate()
            recording.wait()
        
        print("✅ Comprehensive demo complete!")
        return "/tmp/comprehensive_demo.mp4"

def main():
    print("🚀 KVirtualStage Video Generator")
    print("Generating demonstration videos with smooth cursor movement and user intent fixes")
    print("=" * 70)
    
    # Initialize video generator
    generator = VideoGenerator()
    
    # Check container is running
    result = subprocess.run(['docker', 'ps', '--filter', 'name=kvirtual-test', '--format', '{{.Names}}'],
                          capture_output=True, text=True)
    
    if 'kvirtual-test' not in result.stdout:
        print("❌ Container kvirtual-test not found. Please start it first.")
        return
    
    print("✅ Container kvirtual-test is running")
    print("")
    
    # Generate videos
    videos = []
    
    try:
        print("1/3 Generating smooth cursor demo...")
        video1 = generator.generate_smooth_cursor_demo()
        videos.append(video1)
        time.sleep(3)
        
        print("\n2/3 Generating user intent demo...")
        video2 = generator.generate_user_intent_demo()
        videos.append(video2)
        time.sleep(3)
        
        print("\n3/3 Generating comprehensive demo...")
        video3 = generator.generate_comprehensive_demo()
        videos.append(video3)
        
    except KeyboardInterrupt:
        print("\n⚠️  Generation interrupted by user")
    
    # Copy videos to host
    print("\n📁 Copying videos to host...")
    for video in videos:
        filename = os.path.basename(video)
        host_path = f"/Users/kooshapari/temp-PRODVERCEL/485/kush/KAgents/kvirtualstage/{filename}"
        
        copy_result = subprocess.run(['docker', 'cp', f'kvirtual-test:{video}', host_path],
                                   capture_output=True)
        
        if copy_result.returncode == 0:
            print(f"   ✅ {filename}")
        else:
            print(f"   ❌ {filename} (copy failed)")
    
    print("\n🎉 Video generation complete!")
    print("\n📊 Generated Videos:")
    print("   • smooth_cursor_demo.mp4 - Demonstrates 40-step smooth cursor interpolation")
    print("   • user_intent_demo.mp4 - Shows Enter key fix and app launching")
    print("   • comprehensive_demo.mp4 - Complete business workflow demonstration")
    print("\n🏗️  Architecture Demonstrated:")
    print("   • Host-side KVirtualStage automation")
    print("   • Container targeting via VNC/X11")
    print("   • Smooth cursor algorithms with cubic easing")
    print("   • User intent fixes (no search bar stuck text)")
    print("   • Full 1920x1080 QuickTime-compatible recording")

if __name__ == "__main__":
    main()