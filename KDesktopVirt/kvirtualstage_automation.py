#!/usr/bin/env python3
"""
KVirtualStage-style Automation Script
Implements proper smooth cursor movement and user intent workflows
following the KVirtualStage JSON automation format and algorithms.
"""

import json
import time
import subprocess
import math
import os
from typing import Dict, List, Tuple, Optional
from dataclasses import dataclass

@dataclass
class Point:
    x: int
    y: int

class SmoothCursorEngine:
    """Implements KVirtualStage-style smooth cursor movement algorithms"""
    
    def __init__(self, display=":1"):
        self.display = display
        os.environ['DISPLAY'] = display
    
    def windmouse_movement(self, start: Point, end: Point, steps: int = 50) -> List[Point]:
        """
        WindMouse algorithm for natural cursor movement
        Based on KVirtualStage's advanced cursor movement
        """
        points = []
        
        # Calculate distance and direction
        dx = end.x - start.x
        dy = end.y - start.y
        distance = math.sqrt(dx*dx + dy*dy)
        
        # Natural movement parameters
        gravity = 9
        wind = 3
        
        for i in range(steps + 1):
            # Progress from 0 to 1
            progress = i / steps
            
            # Apply easing (smooth acceleration/deceleration)
            eased_progress = self._ease_in_out_cubic(progress)
            
            # Add natural variation
            wind_x = (random_gauss() * wind) if i > 0 else 0
            wind_y = (random_gauss() * wind) if i > 0 else 0
            
            # Calculate position with gravity and wind
            x = start.x + (dx * eased_progress) + wind_x
            y = start.y + (dy * eased_progress) + wind_y
            
            # Add subtle curve for natural movement
            curve_offset = math.sin(progress * math.pi) * (distance / 20)
            y += curve_offset
            
            points.append(Point(int(x), int(y)))
        
        return points
    
    def _ease_in_out_cubic(self, t: float) -> float:
        """Smooth cubic easing function"""
        if t < 0.5:
            return 4 * t * t * t
        else:
            return 1 - pow(-2 * t + 2, 3) / 2
    
    def smooth_move_cursor(self, start: Point, end: Point, duration_ms: int = 1000):
        """Execute smooth cursor movement"""
        steps = max(30, duration_ms // 20)  # Adaptive step count
        points = self.windmouse_movement(start, end, steps)
        
        step_delay = duration_ms / (len(points) * 1000)  # Convert to seconds
        
        print(f"🖱️  Smooth movement: ({start.x},{start.y}) → ({end.x},{end.y}) in {duration_ms}ms")
        
        for i, point in enumerate(points):
            self._xdotool_move(point.x, point.y)
            time.sleep(step_delay)
        
        # Ensure we end exactly at target
        self._xdotool_move(end.x, end.y)
    
    def _xdotool_move(self, x: int, y: int):
        """Move cursor using xdotool"""
        subprocess.run(['xdotool', 'mousemove', str(x), str(y)], 
                      capture_output=True)

class KVirtualStageAutomation:
    """
    KVirtualStage-style automation engine that follows the JSON workflow format
    """
    
    def __init__(self, display=":1"):
        self.display = display
        self.cursor_engine = SmoothCursorEngine(display)
        self.current_position = Point(100, 100)
        os.environ['DISPLAY'] = display
    
    def execute_workflow(self, workflow_file: str):
        """Execute a KVirtualStage JSON workflow"""
        with open(workflow_file, 'r') as f:
            workflow = json.load(f)
        
        print(f"🎬 Executing workflow: {workflow['name']}")
        print(f"📝 Description: {workflow['description']}")
        
        # Start recording if specified
        recording_process = None
        if workflow.get('recording_settings', {}).get('enable_recording', False):
            recording_process = self._start_recording(workflow['recording_settings'])
        
        try:
            # Execute each workflow step
            for step in workflow['workflow']:
                self._execute_step(step)
                time.sleep(0.5)  # Brief pause between steps
        
        finally:
            # Stop recording
            if recording_process:
                self._stop_recording(recording_process)
        
        print("✅ Workflow execution completed!")
    
    def _execute_step(self, step: Dict):
        """Execute a single workflow step"""
        step_num = step.get('step', 0)
        name = step.get('name', 'Unknown')
        
        print(f"\n🔄 Step {step_num}: {name}")
        
        if 'action' in step:
            self._execute_action(step)
        elif 'actions' in step:
            for action in step['actions']:
                self._execute_action(action)
                time.sleep(0.2)
    
    def _execute_action(self, action: Dict):
        """Execute a single action with KVirtualStage-style smooth movement"""
        action_type = action.get('action', '')
        
        if action_type == 'smooth_move_cursor':
            start = Point(action['from']['x'], action['from']['y']) if action['from'] != 'current_position' else self.current_position
            end = Point(action['to']['x'], action['to']['y'])
            duration = action.get('duration', 1000)
            
            self.cursor_engine.smooth_move_cursor(start, end, duration)
            self.current_position = end
        
        elif action_type == 'take_screenshot':
            self._take_screenshot(action['output'])
        
        elif action_type == 'key_combination':
            self._press_key_combination(action['keys'])
        
        elif action_type == 'key_press':
            self._press_key(action['key'])
        
        elif action_type == 'type_text':
            self._type_text(action['text'], action.get('typing_speed', 'normal'))
        
        elif action_type == 'wait':
            duration = action.get('duration', 1000)
            print(f"⏳ Waiting {duration}ms...")
            time.sleep(duration / 1000)
        
        elif action_type == 'smooth_click_sequence':
            self._execute_click_sequence(action['clicks'])
        
        elif action_type == 'find_and_move_to_window':
            self._find_and_move_to_window(action['target'], action.get('duration', 1000))
        
        elif action_type == 'circular_cursor_pattern':
            self._circular_cursor_pattern(action)
        
        # Print action description if available
        if 'description' in action:
            print(f"   {action['description']}")
    
    def _take_screenshot(self, output_path: str):
        """Take screenshot using scrot"""
        print(f"📸 Taking screenshot: {output_path}")
        subprocess.run(['scrot', output_path])
    
    def _press_key_combination(self, keys: List[str]):
        """Press key combination"""
        key_combo = '+'.join(keys)
        print(f"⌨️  Pressing: {key_combo}")
        subprocess.run(['xdotool', 'key', key_combo])
    
    def _press_key(self, key: str):
        """Press single key"""
        print(f"⌨️  Pressing: {key}")
        subprocess.run(['xdotool', 'key', key])
    
    def _type_text(self, text: str, speed: str = 'normal'):
        """Type text with realistic timing"""
        print(f"📝 Typing: {text[:50]}{'...' if len(text) > 50 else ''}")
        
        # Determine typing speed
        if speed == 'human_realistic' or speed == 'realistic_human':
            base_delay = 0.05
            variation = 0.03
        else:
            base_delay = 0.02
            variation = 0.01
        
        for char in text:
            if char == '\n':
                subprocess.run(['xdotool', 'key', 'Return'])
                time.sleep(0.3)
            else:
                subprocess.run(['xdotool', 'type', char])
                # Add realistic timing variation
                delay = base_delay + (random_gauss() * variation)
                time.sleep(max(0.01, delay))
    
    def _execute_click_sequence(self, clicks: List[Dict]):
        """Execute sequence of clicks with smooth cursor movement"""
        print("🖱️  Executing smooth click sequence...")
        
        for click in clicks:
            # Calculate absolute coordinates (relative to current window)
            target_x = self.current_position.x + click['x']
            target_y = self.current_position.y + click['y']
            
            # Smooth move to click position
            self.cursor_engine.smooth_move_cursor(
                self.current_position, 
                Point(target_x, target_y),
                click.get('duration', 300)
            )
            
            # Perform click
            subprocess.run(['xdotool', 'click', '1'])
            time.sleep(0.1)
            
            self.current_position = Point(target_x, target_y)
    
    def _find_and_move_to_window(self, window_name: str, duration: int):
        """Find window and move cursor to it smoothly"""
        print(f"🔍 Finding window: {window_name}")
        
        # Find window using xdotool
        result = subprocess.run(['xdotool', 'search', '--name', window_name], 
                              capture_output=True, text=True)
        
        if result.returncode == 0 and result.stdout.strip():
            window_id = result.stdout.strip().split('\n')[0]
            
            # Get window geometry
            geo_result = subprocess.run(['xdotool', 'getwindowgeometry', '--shell', window_id],
                                      capture_output=True, text=True)
            
            if geo_result.returncode == 0:
                # Parse geometry
                lines = geo_result.stdout.strip().split('\n')
                x = int([line for line in lines if line.startswith('X=')][0].split('=')[1])
                y = int([line for line in lines if line.startswith('Y=')][0].split('=')[1])
                width = int([line for line in lines if line.startswith('WIDTH=')][0].split('=')[1])
                height = int([line for line in lines if line.startswith('HEIGHT=')][0].split('=')[1])
                
                # Move to window center
                center_x = x + width // 2
                center_y = y + height // 2
                
                self.cursor_engine.smooth_move_cursor(
                    self.current_position,
                    Point(center_x, center_y),
                    duration
                )
                
                self.current_position = Point(center_x, center_y)
                
                # Activate window
                subprocess.run(['xdotool', 'windowactivate', window_id])
                print(f"✅ Found and moved to {window_name} at ({center_x}, {center_y})")
            else:
                print(f"❌ Could not get geometry for {window_name}")
        else:
            print(f"❌ Window {window_name} not found")
    
    def _circular_cursor_pattern(self, action: Dict):
        """Create circular cursor movement pattern"""
        center = Point(action['center']['x'], action['center']['y'])
        radius = action['radius']
        revolutions = action.get('revolutions', 1.0)
        duration = action.get('duration', 3000)
        
        print(f"🌀 Creating circular pattern: center=({center.x},{center.y}), radius={radius}")
        
        steps = int(duration / 50)  # 20 FPS for smooth movement
        angle_step = (2 * math.pi * revolutions) / steps
        
        for i in range(steps):
            angle = i * angle_step
            x = center.x + int(radius * math.cos(angle))
            y = center.y + int(radius * math.sin(angle))
            
            self.cursor_engine._xdotool_move(x, y)
            time.sleep(0.05)
        
        self.current_position = center
    
    def _start_recording(self, settings: Dict) -> subprocess.Popen:
        """Start screen recording"""
        output_file = settings.get('output_file', '/tmp/recording.mp4')
        framerate = settings.get('framerate', 30)
        resolution = settings.get('resolution', '1920x1080')
        
        print(f"🎬 Starting recording: {output_file}")
        
        cmd = [
            'ffmpeg', '-f', 'x11grab', 
            '-framerate', str(framerate),
            '-video_size', resolution,
            '-i', self.display,
            '-c:v', 'libx264', '-preset', 'fast', '-crf', '18',
            '-pix_fmt', 'yuv420p', '-movflags', '+faststart',
            '-y', output_file
        ]
        
        return subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    
    def _stop_recording(self, process: subprocess.Popen):
        """Stop screen recording"""
        print("🛑 Stopping recording...")
        process.terminate()
        process.wait()

def random_gauss() -> float:
    """Simple Gaussian random number generator"""
    import random
    return random.gauss(0, 1)

def main():
    """Main execution function"""
    print("🚀 KVirtualStage Automation Engine")
    print("Implementing smooth cursor movement and user intent workflows")
    
    # Initialize automation engine
    automation = KVirtualStageAutomation(display=":1")
    
    # Execute the workflow
    workflow_file = "/app/smooth_user_intent_workflow.json"
    
    if os.path.exists(workflow_file):
        automation.execute_workflow(workflow_file)
    else:
        print(f"❌ Workflow file not found: {workflow_file}")

if __name__ == "__main__":
    main()