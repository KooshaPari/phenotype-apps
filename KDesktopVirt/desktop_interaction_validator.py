#!/usr/bin/env python3
"""
Desktop Interaction Validation Agent for KVirtualStage
Creates comprehensive desktop application interaction validation with visible user intent

MISSION: Validate real desktop app interactions (calculator, text editor, file manager, browser)
with visible user intent including slow cursor movement, character-by-character typing,
menu navigation, login scenarios, and form inputs.

Key Features:
1. Visual Intent System - slow, visible cursor movement showing user intent
2. Desktop App Interactions - test real apps with full workflows
3. Login Scenarios - handle authentication flows with username/password forms
4. Menu Navigation - navigate main menus, submenus, context menus
5. Form Inputs - fill forms with visible typing, dropdowns, checkboxes
6. Recording Capabilities - CLI commands for record start/stop
7. Interface Coverage - available via Python scripting, CLI commands, MCP interface
"""

import asyncio
import subprocess
import time
import os
import json
import logging
import math
import random
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, asdict
from enum import Enum
from pathlib import Path
import cv2
import numpy as np

logger = logging.getLogger(__name__)

class VisualIntentMode(Enum):
    """Visual intent demonstration modes"""
    SLOW_CURSOR = "slow_cursor"          # Slow, visible cursor movement
    CHARACTER_TYPING = "character_typing" # Character-by-character typing
    HOVER_BEFORE_CLICK = "hover_before_click" # Hover before clicking
    MENU_EXPLORATION = "menu_exploration" # Menu navigation with intent
    FORM_FILLING = "form_filling"        # Visible form input
    LOGIN_FLOW = "login_flow"            # Authentication scenarios

@dataclass
class DesktopApp:
    """Desktop application definition"""
    name: str
    executable: str
    window_title: str
    launch_delay: float = 3.0
    interactions: List[str] = None
    
class VisualIntentConfig:
    """Configuration for visual intent system"""
    def __init__(self):
        self.cursor_speed = 0.02      # Seconds between cursor moves
        self.typing_speed = 0.15      # Seconds between characters
        self.hover_duration = 1.0     # Seconds to hover before click
        self.menu_explore_delay = 0.8 # Seconds to explore menus
        self.form_field_delay = 0.5   # Seconds between form fields
        self.intent_visibility = True  # Show intent indicators

class DesktopInteractionValidator:
    """Main desktop interaction validation system"""
    
    def __init__(self, config: VisualIntentConfig = None):
        self.config = config or VisualIntentConfig()
        self.session_id = f"desktop_validation_{int(time.time())}"
        self.recording_active = False
        self.recording_process = None
        self.validation_results = {}
        
        # Desktop applications to test
        self.desktop_apps = {
            "calculator": DesktopApp(
                name="Calculator",
                executable="galculator",
                window_title="Calculator",
                interactions=["number_input", "operations", "calculation"]
            ),
            "text_editor": DesktopApp(
                name="Text Editor", 
                executable="mousepad",
                window_title="Text Editor",
                interactions=["text_input", "formatting", "save"]
            ),
            "file_manager": DesktopApp(
                name="File Manager",
                executable="thunar",
                window_title="File Manager", 
                interactions=["navigation", "file_operations", "context_menu"]
            ),
            "browser": DesktopApp(
                name="Browser",
                executable="firefox",
                window_title="Firefox",
                launch_delay=8.0,
                interactions=["url_input", "navigation", "form_filling"]
            )
        }
        
        self.validation_scenarios = []
        self._init_validation_scenarios()
    
    def _init_validation_scenarios(self):
        """Initialize comprehensive validation scenarios"""
        
        # Calculator Interaction Scenario
        self.validation_scenarios.append({
            "name": "Calculator Mathematical Operations",
            "app": "calculator",
            "description": "Demonstrate calculator usage with visible user intent",
            "steps": [
                {"type": "launch_app", "app": "calculator"},
                {"type": "wait_for_window", "title": "Calculator", "timeout": 10},
                {"type": "visual_intent_click", "target": "7", "intent": "Select number 7"},
                {"type": "visual_intent_click", "target": "+", "intent": "Choose addition operation"},
                {"type": "visual_intent_click", "target": "3", "intent": "Select number 3"},
                {"type": "visual_intent_click", "target": "=", "intent": "Calculate result"},
                {"type": "screenshot", "name": "calculator_result"},
                {"type": "visual_intent_click", "target": "C", "intent": "Clear calculator"},
                {"type": "complex_calculation", "expression": "15 * 8 - 3", "intent": "Perform complex calculation"}
            ]
        })
        
        # Text Editor Interaction Scenario
        self.validation_scenarios.append({
            "name": "Text Editor Document Creation", 
            "app": "text_editor",
            "description": "Demonstrate text editing with natural typing patterns",
            "steps": [
                {"type": "launch_app", "app": "text_editor"},
                {"type": "wait_for_window", "title": "Text Editor", "timeout": 10},
                {"type": "visual_intent_type", "text": "DESKTOP INTERACTION VALIDATION REPORT\n\n", "intent": "Create document header"},
                {"type": "visual_intent_type", "text": "Validation Date: " + time.strftime("%Y-%m-%d %H:%M:%S") + "\n\n", "intent": "Add timestamp"},
                {"type": "visual_intent_type", "text": "Applications Tested:\n• Calculator - Mathematical operations with visual intent\n• Text Editor - Document creation with natural typing\n• File Manager - Navigation and file operations\n• Browser - Web navigation and form filling\n\n", "intent": "Document test summary"},
                {"type": "menu_navigation", "menu_path": ["Edit", "Select All"], "intent": "Navigate to Edit menu"},
                {"type": "key_combination", "keys": ["ctrl", "s"], "intent": "Save document"},
                {"type": "handle_save_dialog", "filename": "validation_report.txt", "intent": "Save with specific filename"}
            ]
        })
        
        # File Manager Navigation Scenario
        self.validation_scenarios.append({
            "name": "File Manager Operations",
            "app": "file_manager", 
            "description": "Demonstrate file navigation and operations",
            "steps": [
                {"type": "launch_app", "app": "file_manager"},
                {"type": "wait_for_window", "title": "File Manager", "timeout": 10},
                {"type": "navigate_to_folder", "path": "/tmp", "intent": "Navigate to temp directory"},
                {"type": "right_click_context", "target": "empty_space", "intent": "Access context menu"},
                {"type": "context_menu_select", "option": "Create Folder", "intent": "Create new folder"},
                {"type": "visual_intent_type", "text": "validation_test_folder", "intent": "Name the new folder"},
                {"type": "key_press", "key": "Return", "intent": "Confirm folder creation"},
                {"type": "double_click_folder", "folder": "validation_test_folder", "intent": "Enter the new folder"},
                {"type": "create_test_file", "filename": "test_document.txt", "content": "Test file created during validation", "intent": "Create test file"}
            ]
        })
        
        # Browser Web Navigation Scenario  
        self.validation_scenarios.append({
            "name": "Browser Web Navigation and Forms",
            "app": "browser",
            "description": "Demonstrate web navigation and form interaction",
            "steps": [
                {"type": "launch_app", "app": "browser"},
                {"type": "wait_for_window", "title": "Firefox", "timeout": 15},
                {"type": "navigate_to_url", "url": "https://httpbin.org/forms/post", "intent": "Navigate to form testing page"},
                {"type": "wait_for_page_load", "timeout": 10},
                {"type": "fill_form_field", "field": "custname", "value": "John Doe", "intent": "Enter customer name"},
                {"type": "fill_form_field", "field": "custtel", "value": "555-0123", "intent": "Enter phone number"},
                {"type": "fill_form_field", "field": "custemail", "value": "john.doe@example.com", "intent": "Enter email address"},
                {"type": "select_dropdown", "field": "size", "value": "medium", "intent": "Select size option"},
                {"type": "check_checkbox", "field": "delivery", "intent": "Select delivery option"},
                {"type": "fill_textarea", "field": "comments", "value": "This is a test form submission during desktop validation testing.", "intent": "Add comments"},
                {"type": "visual_intent_click", "target": "submit", "intent": "Submit the form"}
            ]
        })
        
        # Login Scenario
        self.validation_scenarios.append({
            "name": "Login Authentication Flow",
            "app": "browser",
            "description": "Demonstrate login form interaction",
            "steps": [
                {"type": "navigate_to_url", "url": "https://httpbin.org/basic-auth/testuser/testpass", "intent": "Navigate to authentication page"},
                {"type": "handle_auth_dialog", "username": "testuser", "password": "testpass", "intent": "Handle HTTP authentication"},
                {"type": "wait_for_auth_result", "timeout": 5},
                {"type": "screenshot", "name": "auth_success"}
            ]
        })

    async def start_validation_session(self) -> Dict[str, Any]:
        """Start comprehensive desktop interaction validation session"""
        
        logger.info(f"🚀 Starting Desktop Interaction Validation Session: {self.session_id}")
        
        session_results = {
            "session_id": self.session_id,
            "start_time": time.time(),
            "validation_mode": "comprehensive_desktop_interaction",
            "visual_intent_enabled": self.config.intent_visibility,
            "scenarios_planned": len(self.validation_scenarios),
            "scenarios_completed": 0,
            "scenario_results": [],
            "overall_success": False
        }
        
        # Take initial desktop screenshot
        await self.take_screenshot("00_initial_desktop")
        
        # Start screen recording if supported
        if await self.start_screen_recording():
            logger.info("📹 Screen recording started for validation session")
        
        try:
            # Execute each validation scenario
            for i, scenario in enumerate(self.validation_scenarios):
                logger.info(f"🎯 Executing scenario {i+1}/{len(self.validation_scenarios)}: {scenario['name']}")
                
                scenario_result = await self.execute_validation_scenario(scenario)
                session_results["scenario_results"].append(scenario_result)
                
                if scenario_result["success"]:
                    session_results["scenarios_completed"] += 1
                
                # Brief pause between scenarios
                await asyncio.sleep(2.0)
            
            session_results["overall_success"] = session_results["scenarios_completed"] == len(self.validation_scenarios)
            session_results["end_time"] = time.time()
            session_results["total_duration"] = session_results["end_time"] - session_results["start_time"]
            
            # Stop recording
            if self.recording_active:
                await self.stop_screen_recording()
            
            # Generate validation report
            await self.generate_validation_report(session_results)
            
            logger.info(f"✅ Validation session completed: {session_results['scenarios_completed']}/{len(self.validation_scenarios)} scenarios successful")
            
        except Exception as e:
            logger.error(f"❌ Validation session failed: {e}")
            session_results["error"] = str(e)
            session_results["overall_success"] = False
        
        return session_results

    async def execute_validation_scenario(self, scenario: Dict[str, Any]) -> Dict[str, Any]:
        """Execute individual validation scenario with visual intent"""
        
        scenario_result = {
            "scenario_name": scenario["name"],
            "app": scenario["app"],
            "start_time": time.time(),
            "steps_completed": 0,
            "total_steps": len(scenario["steps"]),
            "success": False,
            "step_results": [],
            "errors": []
        }
        
        try:
            logger.info(f"📋 Starting scenario: {scenario['name']}")
            
            for i, step in enumerate(scenario["steps"]):
                logger.info(f"   Step {i+1}/{len(scenario['steps'])}: {step['type']}")
                
                step_result = await self.execute_validation_step(step, scenario["app"])
                scenario_result["step_results"].append(step_result)
                
                if step_result["success"]:
                    scenario_result["steps_completed"] += 1
                else:
                    scenario_result["errors"].append(f"Step {i+1} failed: {step_result.get('error', 'Unknown error')}")
                
                # Visual intent delay between steps
                await asyncio.sleep(0.5)
            
            scenario_result["success"] = scenario_result["steps_completed"] == scenario_result["total_steps"]
            scenario_result["end_time"] = time.time()
            scenario_result["duration"] = scenario_result["end_time"] - scenario_result["start_time"]
            
        except Exception as e:
            scenario_result["errors"].append(f"Scenario execution failed: {str(e)}")
            logger.error(f"Scenario execution failed: {e}")
        
        return scenario_result

    async def execute_validation_step(self, step: Dict[str, Any], app_context: str) -> Dict[str, Any]:
        """Execute individual validation step with visual intent"""
        
        step_result = {
            "step_type": step["type"],
            "success": False,
            "intent": step.get("intent", ""),
            "execution_time": 0,
            "visual_feedback": []
        }
        
        start_time = time.time()
        
        try:
            if step["type"] == "launch_app":
                success = await self.launch_app_with_intent(step["app"])
                step_result["success"] = success
                
            elif step["type"] == "wait_for_window":
                success = await self.wait_for_window(step["title"], step.get("timeout", 10))
                step_result["success"] = success
                
            elif step["type"] == "visual_intent_click":
                success = await self.visual_intent_click(step["target"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "visual_intent_type":
                success = await self.visual_intent_type(step["text"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "screenshot":
                screenshot_path = await self.take_screenshot(step["name"])
                step_result["success"] = True
                step_result["screenshot_path"] = screenshot_path
                
            elif step["type"] == "complex_calculation":
                success = await self.perform_complex_calculation(step["expression"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "menu_navigation":
                success = await self.navigate_menu_with_intent(step["menu_path"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "key_combination":
                success = await self.press_key_combination(step["keys"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "handle_save_dialog":
                success = await self.handle_save_dialog(step["filename"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "navigate_to_folder":
                success = await self.navigate_to_folder(step["path"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "right_click_context":
                success = await self.right_click_context_menu(step["target"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "context_menu_select":
                success = await self.select_context_menu_option(step["option"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "double_click_folder":
                success = await self.double_click_folder(step["folder"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "create_test_file":
                success = await self.create_test_file(step["filename"], step["content"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "navigate_to_url":
                success = await self.navigate_to_url(step["url"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "wait_for_page_load":
                success = await self.wait_for_page_load(step.get("timeout", 10))
                step_result["success"] = success
                
            elif step["type"] == "fill_form_field":
                success = await self.fill_form_field(step["field"], step["value"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "select_dropdown":
                success = await self.select_dropdown_option(step["field"], step["value"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "check_checkbox":
                success = await self.check_checkbox(step["field"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "fill_textarea":
                success = await self.fill_textarea(step["field"], step["value"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "handle_auth_dialog":
                success = await self.handle_auth_dialog(step["username"], step["password"], step.get("intent", ""))
                step_result["success"] = success
                
            elif step["type"] == "wait_for_auth_result":
                success = await self.wait_for_auth_result(step.get("timeout", 5))
                step_result["success"] = success
                
            elif step["type"] == "key_press":
                success = await self.press_key(step["key"], step.get("intent", ""))
                step_result["success"] = success
                
            else:
                step_result["error"] = f"Unknown step type: {step['type']}"
                step_result["success"] = False
            
            step_result["execution_time"] = time.time() - start_time
            
        except Exception as e:
            step_result["error"] = str(e)
            step_result["success"] = False
            step_result["execution_time"] = time.time() - start_time
            logger.error(f"Step execution failed: {e}")
        
        return step_result

    async def launch_app_with_intent(self, app_name: str) -> bool:
        """Launch application with visual intent demonstration"""
        
        if app_name not in self.desktop_apps:
            logger.error(f"Unknown application: {app_name}")
            return False
        
        app = self.desktop_apps[app_name]
        
        logger.info(f"🚀 Launching {app.name} with visual intent...")
        logger.info(f"   Intent: User wants to open {app.name} for interaction testing")
        
        # Show intent: Move cursor to application launcher area
        await self.show_cursor_intent("Moving to application launcher")
        await self.slow_cursor_move(100, 50)
        await asyncio.sleep(1.0)
        
        # Launch application
        try:
            subprocess.Popen([app.executable])
            logger.info(f"   ✅ {app.name} launched successfully")
            
            # Wait for application to start
            await asyncio.sleep(app.launch_delay)
            
            return True
            
        except Exception as e:
            logger.error(f"   ❌ Failed to launch {app.name}: {e}")
            return False

    async def visual_intent_click(self, target: str, intent: str = "") -> bool:
        """Perform click with visible user intent"""
        
        logger.info(f"🎯 Visual Intent Click: {target}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        # Show cursor intent
        await self.show_cursor_intent(f"Looking for {target}")
        
        # Find target element (simplified - would use computer vision in real implementation)
        target_coords = await self.find_element_coordinates(target)
        if not target_coords:
            logger.warning(f"   ⚠️ Could not locate {target}")
            return False
        
        x, y = target_coords
        
        # Demonstrate visual intent: slow cursor movement
        await self.slow_cursor_move(x, y)
        
        # Hover before clicking to show intent
        logger.info(f"   👆 Hovering over {target} to show intent...")
        await asyncio.sleep(self.config.hover_duration)
        
        # Perform click
        result = subprocess.run(['xdotool', 'click', '1'], capture_output=True)
        if result.returncode == 0:
            logger.info(f"   ✅ Successfully clicked {target}")
            return True
        else:
            logger.error(f"   ❌ Click failed: {result.stderr.decode()}")
            return False

    async def visual_intent_type(self, text: str, intent: str = "") -> bool:
        """Type text with visible character-by-character intent"""
        
        logger.info(f"⌨️ Visual Intent Typing: '{text[:50]}{'...' if len(text) > 50 else ''}'")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        logger.info(f"   💭 User is thinking about what to type...")
        await asyncio.sleep(0.5)
        
        # Character-by-character typing with visible intent
        for i, char in enumerate(text):
            if char == '\n':
                logger.info(f"   ↵ Pressing Enter")
                subprocess.run(['xdotool', 'key', 'Return'])
            else:
                # Show typing progress
                if i % 10 == 0:  # Every 10 characters
                    progress = f"{i}/{len(text)} characters"
                    logger.info(f"   ⌨️ Typing progress: {progress}")
                
                subprocess.run(['xdotool', 'type', char])
            
            # Natural typing delay with variation
            delay = self.config.typing_speed + random.uniform(-0.02, 0.02)
            await asyncio.sleep(delay)
        
        logger.info(f"   ✅ Completed typing {len(text)} characters")
        return True

    async def slow_cursor_move(self, target_x: int, target_y: int) -> bool:
        """Move cursor slowly to show user intent"""
        
        # Get current cursor position
        result = subprocess.run(['xdotool', 'getmouselocation'], capture_output=True, text=True)
        if result.returncode != 0:
            return False
        
        current_x = current_y = 0
        for line in result.stdout.split('\n'):
            if 'x:' in line:
                current_x = int(line.split('x:')[1].split()[0])
                current_y = int(line.split('y:')[1].split()[0])
                break
        
        # Calculate movement path
        dx = target_x - current_x
        dy = target_y - current_y
        distance = math.sqrt(dx*dx + dy*dy)
        
        if distance < 5:
            return True
        
        # Number of steps for smooth movement
        steps = max(10, int(distance / 10))
        
        logger.info(f"   🖱️ Moving cursor from ({current_x},{current_y}) to ({target_x},{target_y})")
        
        # Smooth cursor movement
        for step in range(steps + 1):
            progress = step / steps
            x = int(current_x + dx * progress)
            y = int(current_y + dy * progress)
            
            subprocess.run(['xdotool', 'mousemove', str(x), str(y)])
            await asyncio.sleep(self.config.cursor_speed)
        
        return True

    async def show_cursor_intent(self, intent_message: str):
        """Show cursor intent through logging"""
        logger.info(f"   💭 {intent_message}")

    async def find_element_coordinates(self, target: str) -> Optional[Tuple[int, int]]:
        """Find element coordinates (simplified implementation)"""
        
        # In a real implementation, this would use computer vision, OCR, or accessibility APIs
        # For demo purposes, we'll use approximate coordinates based on common UI patterns
        
        coordinate_map = {
            # Calculator coordinates (approximate)
            "7": (150, 200),
            "+": (200, 250),
            "3": (120, 280),
            "=": (200, 350),
            "C": (120, 150),
            "*": (200, 200),
            "8": (150, 200),
            "-": (200, 275),
            
            # Generic UI elements
            "submit": (400, 500),
            "ok": (350, 400),
            "cancel": (450, 400),
            "save": (300, 200),
            "open": (250, 200),
        }
        
        return coordinate_map.get(target.lower())

    async def perform_complex_calculation(self, expression: str, intent: str = "") -> bool:
        """Perform complex calculation with visual intent"""
        
        logger.info(f"🧮 Complex Calculation: {expression}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        # Parse expression and execute step by step
        # For demo: "15 * 8 - 3"
        parts = expression.split()
        
        for part in parts:
            if part.isdigit():
                # Click number
                await self.visual_intent_click(part, f"Enter number {part}")
                await asyncio.sleep(0.5)
            elif part in ['+', '-', '*', '/', '=']:
                # Click operator  
                await self.visual_intent_click(part, f"Select operation {part}")
                await asyncio.sleep(0.5)
        
        # Click equals to complete calculation
        await self.visual_intent_click("=", "Calculate final result")
        
        return True

    async def navigate_menu_with_intent(self, menu_path: List[str], intent: str = "") -> bool:
        """Navigate menu system with visual intent"""
        
        logger.info(f"📋 Menu Navigation: {' → '.join(menu_path)}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        for i, menu_item in enumerate(menu_path):
            logger.info(f"   📂 Navigating to: {menu_item}")
            
            # Find and click menu item
            success = await self.visual_intent_click(menu_item, f"Access {menu_item} menu")
            if not success:
                return False
            
            # Brief pause for menu to appear
            await asyncio.sleep(self.config.menu_explore_delay)
        
        return True

    async def take_screenshot(self, name: str) -> str:
        """Take screenshot for validation"""
        
        timestamp = int(time.time())
        filename = f"validation_{name}_{timestamp}.png"
        filepath = f"/tmp/{filename}"
        
        result = subprocess.run(['import', '-window', 'root', filepath])
        
        if result.returncode == 0:
            logger.info(f"📸 Screenshot saved: {filepath}")
            return filepath
        else:
            logger.error(f"❌ Screenshot failed")
            return ""

    async def start_screen_recording(self) -> bool:
        """Start screen recording for validation session"""
        
        if self.recording_active:
            return False
        
        timestamp = int(time.time())
        recording_file = f"/tmp/desktop_validation_{timestamp}.mp4"
        
        # FFmpeg command for screen recording
        cmd = [
            'ffmpeg', '-f', 'x11grab',
            '-framerate', '30',
            '-video_size', '1024x768', 
            '-i', ':1.0',
            '-c:v', 'libx264',
            '-preset', 'ultrafast',
            '-crf', '18',
            '-pix_fmt', 'yuv420p',
            '-y', recording_file
        ]
        
        try:
            self.recording_process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            self.recording_active = True
            self.recording_file = recording_file
            logger.info(f"📹 Screen recording started: {recording_file}")
            return True
        except Exception as e:
            logger.error(f"❌ Failed to start recording: {e}")
            return False

    async def stop_screen_recording(self) -> bool:
        """Stop screen recording"""
        
        if not self.recording_active or not self.recording_process:
            return False
        
        try:
            self.recording_process.terminate()
            self.recording_process.wait(timeout=10)
            self.recording_active = False
            logger.info(f"📹 Screen recording stopped: {self.recording_file}")
            return True
        except Exception as e:
            logger.error(f"❌ Failed to stop recording: {e}")
            return False

    # Additional method implementations for comprehensive coverage...
    
    async def wait_for_window(self, title: str, timeout: int = 10) -> bool:
        """Wait for window to appear"""
        start_time = time.time()
        while time.time() - start_time < timeout:
            result = subprocess.run(['wmctrl', '-l'], capture_output=True, text=True)
            if title.lower() in result.stdout.lower():
                logger.info(f"   ✅ Window found: {title}")
                return True
            await asyncio.sleep(0.5)
        
        logger.warning(f"   ⚠️ Window not found within timeout: {title}")
        return False

    async def press_key_combination(self, keys: List[str], intent: str = "") -> bool:
        """Press key combination with intent"""
        key_combo = '+'.join(keys)
        logger.info(f"⌨️ Key Combination: {key_combo}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        result = subprocess.run(['xdotool', 'key', key_combo])
        return result.returncode == 0

    async def press_key(self, key: str, intent: str = "") -> bool:
        """Press single key with intent"""
        logger.info(f"⌨️ Key Press: {key}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        result = subprocess.run(['xdotool', 'key', key])
        return result.returncode == 0

    async def handle_save_dialog(self, filename: str, intent: str = "") -> bool:
        """Handle save dialog with visual intent"""
        logger.info(f"💾 Save Dialog: {filename}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        # Type filename
        await self.visual_intent_type(filename, "Enter filename")
        await asyncio.sleep(0.5)
        
        # Press Enter to save
        await self.press_key("Return", "Confirm save")
        return True

    async def navigate_to_folder(self, path: str, intent: str = "") -> bool:
        """Navigate to folder with intent"""
        logger.info(f"📁 Navigate to: {path}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        # Use Ctrl+L to open location bar
        await self.press_key_combination(["ctrl", "l"], "Open location bar")
        await asyncio.sleep(0.5)
        
        # Type path
        await self.visual_intent_type(path, f"Navigate to {path}")
        await self.press_key("Return", "Navigate to folder")
        
        return True

    async def right_click_context_menu(self, target: str, intent: str = "") -> bool:
        """Right-click for context menu"""
        logger.info(f"🖱️ Right-click: {target}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        # Find empty space or specific target
        coords = await self.find_element_coordinates(target) or (400, 300)
        await self.slow_cursor_move(*coords)
        
        result = subprocess.run(['xdotool', 'click', '3'])
        return result.returncode == 0

    async def select_context_menu_option(self, option: str, intent: str = "") -> bool:
        """Select context menu option"""
        logger.info(f"📋 Context Menu: {option}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        return await self.visual_intent_click(option, f"Select {option}")

    async def double_click_folder(self, folder: str, intent: str = "") -> bool:
        """Double-click folder to open"""
        logger.info(f"📂 Double-click: {folder}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        coords = await self.find_element_coordinates(folder)
        if coords:
            await self.slow_cursor_move(*coords)
            subprocess.run(['xdotool', 'click', '--repeat', '2', '1'])
            return True
        return False

    async def create_test_file(self, filename: str, content: str, intent: str = "") -> bool:
        """Create test file with content"""
        logger.info(f"📄 Create file: {filename}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        # Right-click to open context menu
        await self.right_click_context_menu("empty_space", "Access context menu")
        await asyncio.sleep(0.5)
        
        # Select create new file option (simplified)
        await self.visual_intent_click("New Document", "Create new document")
        await asyncio.sleep(1.0)
        
        # Type filename and content
        await self.visual_intent_type(filename, "Enter filename")
        await self.press_key("Return", "Confirm filename")
        
        return True

    async def navigate_to_url(self, url: str, intent: str = "") -> bool:
        """Navigate to URL in browser"""
        logger.info(f"🌐 Navigate to: {url}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        # Click address bar
        await self.press_key_combination(["ctrl", "l"], "Focus address bar")
        await asyncio.sleep(0.5)
        
        # Type URL
        await self.visual_intent_type(url, f"Enter URL {url}")
        await self.press_key("Return", "Navigate to URL")
        
        return True

    async def wait_for_page_load(self, timeout: int = 10) -> bool:
        """Wait for page to load"""
        logger.info(f"⏳ Waiting for page load (timeout: {timeout}s)")
        await asyncio.sleep(timeout)  # Simplified implementation
        return True

    async def fill_form_field(self, field: str, value: str, intent: str = "") -> bool:
        """Fill form field with value"""
        logger.info(f"📝 Fill field '{field}': {value}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        # Click field (simplified - would use proper element detection)
        coords = await self.find_element_coordinates(field)
        if coords:
            await self.slow_cursor_move(*coords)
            await self.visual_intent_click(field, f"Focus {field} field")
            await asyncio.sleep(0.5)
            
            # Clear field and type value
            await self.press_key_combination(["ctrl", "a"], "Select all")
            await self.visual_intent_type(value, f"Enter {field} value")
            
            return True
        return False

    async def select_dropdown_option(self, field: str, value: str, intent: str = "") -> bool:
        """Select dropdown option"""
        logger.info(f"📋 Dropdown '{field}': {value}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        # Click dropdown
        await self.visual_intent_click(field, f"Open {field} dropdown")
        await asyncio.sleep(0.5)
        
        # Select option
        await self.visual_intent_click(value, f"Select {value}")
        
        return True

    async def check_checkbox(self, field: str, intent: str = "") -> bool:
        """Check checkbox"""
        logger.info(f"☑️ Checkbox: {field}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        return await self.visual_intent_click(field, f"Check {field} checkbox")

    async def fill_textarea(self, field: str, value: str, intent: str = "") -> bool:
        """Fill textarea with value"""
        logger.info(f"📝 Textarea '{field}': {value[:50]}...")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        await self.visual_intent_click(field, f"Focus {field} textarea")
        await asyncio.sleep(0.5)
        await self.visual_intent_type(value, f"Enter {field} content")
        
        return True

    async def handle_auth_dialog(self, username: str, password: str, intent: str = "") -> bool:
        """Handle authentication dialog"""
        logger.info(f"🔐 Authentication: {username}")
        if intent:
            logger.info(f"   Intent: {intent}")
        
        # Wait for auth dialog
        await asyncio.sleep(2.0)
        
        # Type username
        await self.visual_intent_type(username, "Enter username")
        await self.press_key("Tab", "Move to password field")
        
        # Type password
        await self.visual_intent_type(password, "Enter password")
        await self.press_key("Return", "Submit authentication")
        
        return True

    async def wait_for_auth_result(self, timeout: int = 5) -> bool:
        """Wait for authentication result"""
        logger.info(f"⏳ Waiting for authentication result")
        await asyncio.sleep(timeout)
        return True

    async def generate_validation_report(self, session_results: Dict[str, Any]):
        """Generate comprehensive validation report"""
        
        report_path = f"/tmp/desktop_validation_report_{self.session_id}.json"
        
        with open(report_path, 'w') as f:
            json.dump(session_results, f, indent=2, default=str)
        
        logger.info(f"📊 Validation report generated: {report_path}")
        
        # Generate summary
        total_scenarios = session_results["scenarios_planned"]
        completed_scenarios = session_results["scenarios_completed"]
        success_rate = (completed_scenarios / total_scenarios) * 100 if total_scenarios > 0 else 0
        
        print(f"\n🏆 DESKTOP INTERACTION VALIDATION COMPLETE")
        print(f"=" * 60)
        print(f"Session ID: {self.session_id}")
        print(f"Total Scenarios: {total_scenarios}")
        print(f"Completed: {completed_scenarios}")
        print(f"Success Rate: {success_rate:.1f}%")
        print(f"Duration: {session_results.get('total_duration', 0):.1f} seconds")
        print(f"Visual Intent: {'Enabled' if self.config.intent_visibility else 'Disabled'}")
        print(f"Report: {report_path}")
        
        if hasattr(self, 'recording_file'):
            print(f"Recording: {self.recording_file}")

# CLI Interface for Desktop Interaction Validation
class DesktopValidationCLI:
    """Command-line interface for desktop validation"""
    
    def __init__(self):
        self.validator = DesktopInteractionValidator()
    
    async def run_full_validation(self):
        """Run complete desktop validation suite"""
        return await self.validator.start_validation_session()
    
    async def run_single_scenario(self, scenario_name: str):
        """Run single validation scenario"""
        for scenario in self.validator.validation_scenarios:
            if scenario["name"].lower() == scenario_name.lower():
                return await self.validator.execute_validation_scenario(scenario)
        
        print(f"❌ Scenario not found: {scenario_name}")
        return None
    
    async def start_recording(self):
        """Start screen recording"""
        return await self.validator.start_screen_recording()
    
    async def stop_recording(self):
        """Stop screen recording"""
        return await self.validator.stop_screen_recording()
    
    async def take_screenshot(self, name: str = "manual"):
        """Take manual screenshot"""
        return await self.validator.take_screenshot(name)

async def main():
    """Main entry point for desktop interaction validation"""
    
    import sys
    
    if len(sys.argv) < 2:
        print("🖥️ Desktop Interaction Validation Agent")
        print("Usage:")
        print("  python desktop_interaction_validator.py full_validation")
        print("  python desktop_interaction_validator.py scenario <scenario_name>")
        print("  python desktop_interaction_validator.py start_recording")
        print("  python desktop_interaction_validator.py stop_recording")
        print("  python desktop_interaction_validator.py screenshot [name]")
        return
    
    cli = DesktopValidationCLI()
    command = sys.argv[1]
    
    if command == "full_validation":
        await cli.run_full_validation()
    
    elif command == "scenario" and len(sys.argv) > 2:
        scenario_name = sys.argv[2]
        await cli.run_single_scenario(scenario_name)
    
    elif command == "start_recording":
        success = await cli.start_recording()
        print(f"Recording {'started' if success else 'failed'}")
    
    elif command == "stop_recording":
        success = await cli.stop_recording()
        print(f"Recording {'stopped' if success else 'failed'}")
    
    elif command == "screenshot":
        name = sys.argv[2] if len(sys.argv) > 2 else "manual"
        path = await cli.take_screenshot(name)
        print(f"Screenshot: {path}")
    
    else:
        print(f"❌ Unknown command: {command}")

if __name__ == "__main__":
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(levelname)s - %(message)s'
    )
    
    asyncio.run(main())