#!/usr/bin/env python3
"""
MCP Desktop Interface for KVirtualStage
Provides MCP interface for Claude Code/Cursor integration with desktop interaction validation

This interface allows Claude Code and Cursor to control desktop interaction validation
through the Model Context Protocol (MCP), enabling seamless integration with AI workflows.
"""

import asyncio
import json
import logging
import time
from typing import Dict, List, Optional, Any, Union
from dataclasses import dataclass, asdict
from desktop_interaction_validator import DesktopInteractionValidator, VisualIntentConfig

logger = logging.getLogger(__name__)

@dataclass
class MCPResponse:
    """Standardized MCP response format"""
    success: bool
    data: Optional[Dict[str, Any]] = None
    error: Optional[str] = None
    metadata: Optional[Dict[str, Any]] = None

class MCPDesktopInterface:
    """MCP interface for desktop interaction validation"""
    
    def __init__(self):
        self.validator = DesktopInteractionValidator()
        self.active_sessions: Dict[str, Dict[str, Any]] = {}
        self.interface_version = "1.0.0"
        
        # MCP tool definitions
        self.mcp_tools = {
            "create_validation_session": {
                "description": "Create a new desktop interaction validation session",
                "parameters": {
                    "session_name": {"type": "string", "description": "Name for the validation session"},
                    "visual_intent": {"type": "boolean", "description": "Enable visual intent demonstration", "default": True},
                    "apps_to_test": {"type": "array", "description": "List of desktop apps to test", "default": ["calculator", "text_editor"]}
                }
            },
            "execute_desktop_scenario": {
                "description": "Execute specific desktop interaction scenario",
                "parameters": {
                    "session_id": {"type": "string", "description": "Validation session ID"},
                    "scenario_name": {"type": "string", "description": "Name of scenario to execute"},
                    "parameters": {"type": "object", "description": "Scenario-specific parameters"}
                }
            },
            "launch_app_with_intent": {
                "description": "Launch desktop application with visible user intent",
                "parameters": {
                    "app_name": {"type": "string", "description": "Application to launch (calculator, text_editor, file_manager, browser)"},
                    "intent_description": {"type": "string", "description": "Description of user intent"},
                    "wait_for_launch": {"type": "boolean", "description": "Wait for application to fully load", "default": True}
                }
            },
            "visual_intent_click": {
                "description": "Perform click with visible user intent (slow cursor movement, hover)",
                "parameters": {
                    "target": {"type": "string", "description": "Target element to click"},
                    "coordinates": {"type": "array", "description": "X,Y coordinates if target not found", "optional": True},
                    "intent_message": {"type": "string", "description": "Description of user intent"},
                    "hover_duration": {"type": "number", "description": "Seconds to hover before clicking", "default": 1.0}
                }
            },
            "natural_type_text": {
                "description": "Type text with natural character-by-character timing",
                "parameters": {
                    "text": {"type": "string", "description": "Text to type"},
                    "intent_message": {"type": "string", "description": "Description of user intent"},
                    "typing_speed": {"type": "number", "description": "Seconds between characters", "default": 0.15},
                    "allow_mistakes": {"type": "boolean", "description": "Include realistic typing mistakes", "default": True}
                }
            },
            "navigate_menu_system": {
                "description": "Navigate application menu system with visible intent",
                "parameters": {
                    "menu_path": {"type": "array", "description": "Array of menu items to navigate"},
                    "intent_message": {"type": "string", "description": "Description of user intent"},
                    "exploration_delay": {"type": "number", "description": "Seconds to explore each menu", "default": 0.8}
                }
            },
            "fill_form_with_intent": {
                "description": "Fill form fields with visible user intent and realistic behavior",
                "parameters": {
                    "form_fields": {"type": "object", "description": "Object mapping field names to values"},
                    "intent_message": {"type": "string", "description": "Description of user intent"},
                    "include_validation": {"type": "boolean", "description": "Include form validation testing", "default": True}
                }
            },
            "handle_login_scenario": {
                "description": "Handle authentication/login scenario with visible intent",
                "parameters": {
                    "username": {"type": "string", "description": "Username for authentication"},
                    "password": {"type": "string", "description": "Password for authentication"},
                    "auth_type": {"type": "string", "description": "Authentication type (basic, form, dialog)", "default": "form"},
                    "intent_message": {"type": "string", "description": "Description of user intent"}
                }
            },
            "start_screen_recording": {
                "description": "Start screen recording of desktop interactions",
                "parameters": {
                    "recording_name": {"type": "string", "description": "Name for the recording file"},
                    "quality": {"type": "string", "description": "Recording quality (high, medium, low)", "default": "high"},
                    "include_audio": {"type": "boolean", "description": "Include audio in recording", "default": False}
                }
            },
            "stop_screen_recording": {
                "description": "Stop active screen recording and save file",
                "parameters": {
                    "generate_gif": {"type": "boolean", "description": "Also generate GIF version", "default": True}
                }
            },
            "take_validation_screenshot": {
                "description": "Take screenshot for validation documentation",
                "parameters": {
                    "screenshot_name": {"type": "string", "description": "Name for the screenshot"},
                    "add_annotations": {"type": "boolean", "description": "Add intent annotations to screenshot", "default": False}
                }
            },
            "get_session_status": {
                "description": "Get status of validation session",
                "parameters": {
                    "session_id": {"type": "string", "description": "Validation session ID"}
                }
            },
            "generate_validation_report": {
                "description": "Generate comprehensive validation report",
                "parameters": {
                    "session_id": {"type": "string", "description": "Validation session ID"},
                    "include_screenshots": {"type": "boolean", "description": "Include screenshots in report", "default": True},
                    "include_recordings": {"type": "boolean", "description": "Include recording links in report", "default": True}
                }
            },
            "configure_visual_intent": {
                "description": "Configure visual intent system parameters",
                "parameters": {
                    "cursor_speed": {"type": "number", "description": "Cursor movement speed (seconds between moves)", "default": 0.02},
                    "typing_speed": {"type": "number", "description": "Typing speed (seconds between characters)", "default": 0.15},
                    "hover_duration": {"type": "number", "description": "Hover duration before clicks", "default": 1.0},
                    "intent_visibility": {"type": "boolean", "description": "Enable intent visibility features", "default": True}
                }
            }
        }

    async def handle_mcp_call(self, tool_name: str, parameters: Dict[str, Any]) -> MCPResponse:
        """Handle MCP tool call and return standardized response"""
        
        logger.info(f"🔧 MCP Call: {tool_name}")
        logger.info(f"   Parameters: {parameters}")
        
        try:
            if tool_name == "create_validation_session":
                return await self._create_validation_session(parameters)
            
            elif tool_name == "execute_desktop_scenario":
                return await self._execute_desktop_scenario(parameters)
            
            elif tool_name == "launch_app_with_intent":
                return await self._launch_app_with_intent(parameters)
            
            elif tool_name == "visual_intent_click":
                return await self._visual_intent_click(parameters)
            
            elif tool_name == "natural_type_text":
                return await self._natural_type_text(parameters)
            
            elif tool_name == "navigate_menu_system":
                return await self._navigate_menu_system(parameters)
            
            elif tool_name == "fill_form_with_intent":
                return await self._fill_form_with_intent(parameters)
            
            elif tool_name == "handle_login_scenario":
                return await self._handle_login_scenario(parameters)
            
            elif tool_name == "start_screen_recording":
                return await self._start_screen_recording(parameters)
            
            elif tool_name == "stop_screen_recording":
                return await self._stop_screen_recording(parameters)
            
            elif tool_name == "take_validation_screenshot":
                return await self._take_validation_screenshot(parameters)
            
            elif tool_name == "get_session_status":
                return await self._get_session_status(parameters)
            
            elif tool_name == "generate_validation_report":
                return await self._generate_validation_report(parameters)
            
            elif tool_name == "configure_visual_intent":
                return await self._configure_visual_intent(parameters)
            
            else:
                return MCPResponse(
                    success=False,
                    error=f"Unknown MCP tool: {tool_name}"
                )
        
        except Exception as e:
            logger.error(f"MCP call failed: {e}")
            return MCPResponse(
                success=False,
                error=str(e)
            )

    async def _create_validation_session(self, params: Dict[str, Any]) -> MCPResponse:
        """Create new validation session"""
        
        session_name = params.get("session_name", f"mcp_session_{int(time.time())}")
        visual_intent = params.get("visual_intent", True)
        apps_to_test = params.get("apps_to_test", ["calculator", "text_editor"])
        
        # Configure visual intent
        config = VisualIntentConfig()
        config.intent_visibility = visual_intent
        
        # Update validator config
        self.validator.config = config
        
        # Create session record
        session_id = f"mcp_{session_name}_{int(time.time())}"
        session_data = {
            "session_id": session_id,
            "session_name": session_name,
            "created_at": time.time(),
            "visual_intent_enabled": visual_intent,
            "apps_to_test": apps_to_test,
            "status": "created",
            "scenarios_completed": 0,
            "total_scenarios": len(apps_to_test),
            "screenshots": [],
            "recordings": []
        }
        
        self.active_sessions[session_id] = session_data
        
        logger.info(f"✅ Created validation session: {session_id}")
        
        return MCPResponse(
            success=True,
            data={
                "session_id": session_id,
                "session_info": session_data,
                "available_apps": list(self.validator.desktop_apps.keys()),
                "mcp_interface_version": self.interface_version
            },
            metadata={
                "session_created": True,
                "visual_intent_configured": visual_intent
            }
        )

    async def _execute_desktop_scenario(self, params: Dict[str, Any]) -> MCPResponse:
        """Execute specific desktop interaction scenario"""
        
        session_id = params.get("session_id")
        scenario_name = params.get("scenario_name")
        scenario_params = params.get("parameters", {})
        
        if session_id not in self.active_sessions:
            return MCPResponse(
                success=False,
                error=f"Session not found: {session_id}"
            )
        
        session = self.active_sessions[session_id]
        
        # Find and execute scenario
        matching_scenario = None
        for scenario in self.validator.validation_scenarios:
            if scenario["name"].lower() == scenario_name.lower():
                matching_scenario = scenario
                break
        
        if not matching_scenario:
            return MCPResponse(
                success=False,
                error=f"Scenario not found: {scenario_name}"
            )
        
        logger.info(f"🎯 Executing scenario: {scenario_name} for session {session_id}")
        
        # Execute scenario
        scenario_result = await self.validator.execute_validation_scenario(matching_scenario)
        
        # Update session
        if scenario_result["success"]:
            session["scenarios_completed"] += 1
        
        session["status"] = "running"
        
        return MCPResponse(
            success=scenario_result["success"],
            data={
                "scenario_name": scenario_name,
                "scenario_result": scenario_result,
                "session_progress": f"{session['scenarios_completed']}/{session['total_scenarios']}"
            },
            metadata={
                "scenario_executed": True,
                "steps_completed": scenario_result["steps_completed"],
                "total_steps": scenario_result["total_steps"]
            }
        )

    async def _launch_app_with_intent(self, params: Dict[str, Any]) -> MCPResponse:
        """Launch application with visible user intent"""
        
        app_name = params.get("app_name")
        intent_description = params.get("intent_description", f"Testing {app_name} functionality")
        wait_for_launch = params.get("wait_for_launch", True)
        
        logger.info(f"🚀 Launching {app_name} with intent: {intent_description}")
        
        # Launch app with intent
        success = await self.validator.launch_app_with_intent(app_name)
        
        result_data = {
            "app_name": app_name,
            "intent_description": intent_description,
            "launch_successful": success
        }
        
        if wait_for_launch and success:
            app_info = self.validator.desktop_apps.get(app_name)
            if app_info:
                await asyncio.sleep(app_info.launch_delay)
                window_found = await self.validator.wait_for_window(app_info.window_title, 10)
                result_data["window_ready"] = window_found
        
        return MCPResponse(
            success=success,
            data=result_data,
            metadata={
                "visual_intent_demonstrated": True,
                "app_launched": success
            }
        )

    async def _visual_intent_click(self, params: Dict[str, Any]) -> MCPResponse:
        """Perform click with visible user intent"""
        
        target = params.get("target")
        coordinates = params.get("coordinates")
        intent_message = params.get("intent_message", f"Clicking on {target}")
        hover_duration = params.get("hover_duration", 1.0)
        
        logger.info(f"🎯 Visual intent click: {target} - {intent_message}")
        
        # Update hover duration in config
        original_hover = self.validator.config.hover_duration
        self.validator.config.hover_duration = hover_duration
        
        # Perform click with intent
        success = await self.validator.visual_intent_click(target, intent_message)
        
        # Restore original config
        self.validator.config.hover_duration = original_hover
        
        return MCPResponse(
            success=success,
            data={
                "target": target,
                "intent_message": intent_message,
                "click_successful": success,
                "hover_duration": hover_duration
            },
            metadata={
                "visual_intent_demonstrated": True,
                "cursor_movement_shown": True
            }
        )

    async def _natural_type_text(self, params: Dict[str, Any]) -> MCPResponse:
        """Type text with natural character-by-character timing"""
        
        text = params.get("text")
        intent_message = params.get("intent_message", f"Typing: {text[:50]}...")
        typing_speed = params.get("typing_speed", 0.15)
        allow_mistakes = params.get("allow_mistakes", True)
        
        logger.info(f"⌨️ Natural typing: {intent_message}")
        
        # Update typing speed in config
        original_speed = self.validator.config.typing_speed
        self.validator.config.typing_speed = typing_speed
        
        # Perform natural typing
        success = await self.validator.visual_intent_type(text, intent_message)
        
        # Restore original config
        self.validator.config.typing_speed = original_speed
        
        return MCPResponse(
            success=success,
            data={
                "text_length": len(text),
                "intent_message": intent_message,
                "typing_successful": success,
                "typing_speed": typing_speed,
                "mistakes_allowed": allow_mistakes
            },
            metadata={
                "natural_typing_demonstrated": True,
                "character_by_character": True
            }
        )

    async def _navigate_menu_system(self, params: Dict[str, Any]) -> MCPResponse:
        """Navigate menu system with visible intent"""
        
        menu_path = params.get("menu_path", [])
        intent_message = params.get("intent_message", f"Navigating to {' → '.join(menu_path)}")
        exploration_delay = params.get("exploration_delay", 0.8)
        
        logger.info(f"📋 Menu navigation: {intent_message}")
        
        # Update menu exploration delay
        original_delay = self.validator.config.menu_explore_delay
        self.validator.config.menu_explore_delay = exploration_delay
        
        # Navigate menu with intent
        success = await self.validator.navigate_menu_with_intent(menu_path, intent_message)
        
        # Restore original config
        self.validator.config.menu_explore_delay = original_delay
        
        return MCPResponse(
            success=success,
            data={
                "menu_path": menu_path,
                "intent_message": intent_message,
                "navigation_successful": success,
                "exploration_delay": exploration_delay
            },
            metadata={
                "menu_navigation_demonstrated": True,
                "exploration_shown": True
            }
        )

    async def _fill_form_with_intent(self, params: Dict[str, Any]) -> MCPResponse:
        """Fill form fields with visible user intent"""
        
        form_fields = params.get("form_fields", {})
        intent_message = params.get("intent_message", "Filling form with realistic behavior")
        include_validation = params.get("include_validation", True)
        
        logger.info(f"📝 Form filling: {intent_message}")
        
        results = {}
        overall_success = True
        
        for field_name, field_value in form_fields.items():
            logger.info(f"   Filling field: {field_name} = {field_value}")
            
            # Fill each field with intent
            field_success = await self.validator.fill_form_field(
                field_name, 
                str(field_value), 
                f"Entering {field_name}"
            )
            
            results[field_name] = {
                "value": field_value,
                "success": field_success
            }
            
            if not field_success:
                overall_success = False
            
            # Brief pause between fields
            await asyncio.sleep(self.validator.config.form_field_delay)
        
        return MCPResponse(
            success=overall_success,
            data={
                "form_fields_filled": len(form_fields),
                "field_results": results,
                "intent_message": intent_message,
                "validation_included": include_validation
            },
            metadata={
                "form_interaction_demonstrated": True,
                "realistic_behavior_shown": True
            }
        )

    async def _handle_login_scenario(self, params: Dict[str, Any]) -> MCPResponse:
        """Handle authentication/login scenario"""
        
        username = params.get("username")
        password = params.get("password")
        auth_type = params.get("auth_type", "form")
        intent_message = params.get("intent_message", f"Authenticating user {username}")
        
        logger.info(f"🔐 Login scenario: {intent_message}")
        
        # Handle authentication with intent
        success = await self.validator.handle_auth_dialog(username, password, intent_message)
        
        return MCPResponse(
            success=success,
            data={
                "username": username,
                "auth_type": auth_type,
                "intent_message": intent_message,
                "authentication_successful": success
            },
            metadata={
                "login_scenario_demonstrated": True,
                "auth_flow_completed": success
            }
        )

    async def _start_screen_recording(self, params: Dict[str, Any]) -> MCPResponse:
        """Start screen recording"""
        
        recording_name = params.get("recording_name", f"mcp_recording_{int(time.time())}")
        quality = params.get("quality", "high")
        include_audio = params.get("include_audio", False)
        
        logger.info(f"📹 Starting recording: {recording_name}")
        
        success = await self.validator.start_screen_recording()
        
        if success:
            # Add recording to session if available
            for session in self.active_sessions.values():
                if session["status"] in ["created", "running"]:
                    session["recordings"].append({
                        "name": recording_name,
                        "started_at": time.time(),
                        "quality": quality,
                        "include_audio": include_audio
                    })
                    break
        
        return MCPResponse(
            success=success,
            data={
                "recording_name": recording_name,
                "recording_started": success,
                "quality": quality,
                "include_audio": include_audio,
                "recording_file": getattr(self.validator, 'recording_file', None)
            },
            metadata={
                "screen_recording_active": success
            }
        )

    async def _stop_screen_recording(self, params: Dict[str, Any]) -> MCPResponse:
        """Stop screen recording"""
        
        generate_gif = params.get("generate_gif", True)
        
        logger.info("📹 Stopping screen recording")
        
        success = await self.validator.stop_screen_recording()
        recording_file = getattr(self.validator, 'recording_file', None)
        
        return MCPResponse(
            success=success,
            data={
                "recording_stopped": success,
                "recording_file": recording_file,
                "gif_generated": generate_gif and success
            },
            metadata={
                "screen_recording_complete": success
            }
        )

    async def _take_validation_screenshot(self, params: Dict[str, Any]) -> MCPResponse:
        """Take screenshot for validation"""
        
        screenshot_name = params.get("screenshot_name", f"mcp_screenshot_{int(time.time())}")
        add_annotations = params.get("add_annotations", False)
        
        logger.info(f"📸 Taking screenshot: {screenshot_name}")
        
        screenshot_path = await self.validator.take_screenshot(screenshot_name)
        
        # Add to session if available
        for session in self.active_sessions.values():
            if session["status"] in ["created", "running"]:
                session["screenshots"].append({
                    "name": screenshot_name,
                    "path": screenshot_path,
                    "taken_at": time.time(),
                    "annotations": add_annotations
                })
                break
        
        return MCPResponse(
            success=bool(screenshot_path),
            data={
                "screenshot_name": screenshot_name,
                "screenshot_path": screenshot_path,
                "annotations_added": add_annotations
            },
            metadata={
                "screenshot_captured": bool(screenshot_path)
            }
        )

    async def _get_session_status(self, params: Dict[str, Any]) -> MCPResponse:
        """Get validation session status"""
        
        session_id = params.get("session_id")
        
        if session_id not in self.active_sessions:
            return MCPResponse(
                success=False,
                error=f"Session not found: {session_id}"
            )
        
        session = self.active_sessions[session_id]
        
        return MCPResponse(
            success=True,
            data={
                "session_info": session,
                "session_duration": time.time() - session["created_at"],
                "progress_percentage": (session["scenarios_completed"] / session["total_scenarios"]) * 100 if session["total_scenarios"] > 0 else 0
            },
            metadata={
                "session_active": session["status"] in ["created", "running"]
            }
        )

    async def _generate_validation_report(self, params: Dict[str, Any]) -> MCPResponse:
        """Generate comprehensive validation report"""
        
        session_id = params.get("session_id")
        include_screenshots = params.get("include_screenshots", True)
        include_recordings = params.get("include_recordings", True)
        
        if session_id not in self.active_sessions:
            return MCPResponse(
                success=False,
                error=f"Session not found: {session_id}"
            )
        
        session = self.active_sessions[session_id]
        
        # Generate comprehensive report
        report = {
            "session_id": session_id,
            "validation_type": "mcp_desktop_interaction",
            "generated_at": time.time(),
            "session_info": session,
            "validation_summary": {
                "scenarios_completed": session["scenarios_completed"],
                "total_scenarios": session["total_scenarios"],
                "success_rate": (session["scenarios_completed"] / session["total_scenarios"]) * 100 if session["total_scenarios"] > 0 else 0,
                "session_duration": time.time() - session["created_at"]
            },
            "visual_intent_features": {
                "slow_cursor_movement": session["visual_intent_enabled"],
                "character_by_character_typing": session["visual_intent_enabled"],
                "hover_before_click": session["visual_intent_enabled"],
                "menu_exploration": session["visual_intent_enabled"],
                "form_realistic_behavior": session["visual_intent_enabled"]
            },
            "mcp_interface": {
                "version": self.interface_version,
                "tools_available": len(self.mcp_tools),
                "integration_ready": True
            }
        }
        
        if include_screenshots:
            report["screenshots"] = session.get("screenshots", [])
        
        if include_recordings:
            report["recordings"] = session.get("recordings", [])
        
        # Save report
        report_file = f"/tmp/mcp_validation_report_{session_id}.json"
        with open(report_file, 'w') as f:
            json.dump(report, f, indent=2, default=str)
        
        logger.info(f"📊 Validation report generated: {report_file}")
        
        return MCPResponse(
            success=True,
            data={
                "report_file": report_file,
                "report_summary": report["validation_summary"],
                "report_data": report
            },
            metadata={
                "report_generated": True,
                "screenshots_included": include_screenshots,
                "recordings_included": include_recordings
            }
        )

    async def _configure_visual_intent(self, params: Dict[str, Any]) -> MCPResponse:
        """Configure visual intent system parameters"""
        
        cursor_speed = params.get("cursor_speed", self.validator.config.cursor_speed)
        typing_speed = params.get("typing_speed", self.validator.config.typing_speed)
        hover_duration = params.get("hover_duration", self.validator.config.hover_duration)
        intent_visibility = params.get("intent_visibility", self.validator.config.intent_visibility)
        
        # Update configuration
        self.validator.config.cursor_speed = cursor_speed
        self.validator.config.typing_speed = typing_speed
        self.validator.config.hover_duration = hover_duration
        self.validator.config.intent_visibility = intent_visibility
        
        logger.info("⚙️ Visual intent configuration updated")
        
        return MCPResponse(
            success=True,
            data={
                "cursor_speed": cursor_speed,
                "typing_speed": typing_speed,
                "hover_duration": hover_duration,
                "intent_visibility": intent_visibility,
                "configuration_updated": True
            },
            metadata={
                "visual_intent_configured": True
            }
        )

    def get_mcp_tools_schema(self) -> Dict[str, Any]:
        """Get MCP tools schema for integration"""
        
        return {
            "interface_version": self.interface_version,
            "interface_name": "KVirtualStage Desktop Interaction Validator",
            "description": "MCP interface for desktop interaction validation with visible user intent",
            "tools": self.mcp_tools,
            "capabilities": [
                "desktop_app_interaction",
                "visual_intent_demonstration", 
                "form_filling_validation",
                "menu_navigation_testing",
                "login_scenario_handling",
                "screen_recording",
                "screenshot_capture",
                "validation_reporting"
            ],
            "supported_platforms": ["Linux", "macOS", "Windows"],
            "required_dependencies": ["xdotool", "ffmpeg", "import"],
            "integration_examples": {
                "claude_code": "Use MCP tools to validate desktop interactions in development workflow",
                "cursor": "Integrate desktop validation into coding sessions",
                "ai_workflows": "Automate desktop interaction testing with AI oversight"
            }
        }

# CLI integration for MCP interface
async def run_mcp_server():
    """Run MCP server for desktop interaction validation"""
    
    mcp_interface = MCPDesktopInterface()
    
    print("🚀 KVirtualStage MCP Desktop Interface Started")
    print("Available tools:")
    for tool_name, tool_info in mcp_interface.mcp_tools.items():
        print(f"  • {tool_name}: {tool_info['description']}")
    
    # Example usage demonstration
    print("\n📋 Example MCP Tool Usage:")
    
    # Create validation session
    session_response = await mcp_interface.handle_mcp_call("create_validation_session", {
        "session_name": "mcp_demo_session",
        "visual_intent": True,
        "apps_to_test": ["calculator", "text_editor"]
    })
    
    if session_response.success:
        session_id = session_response.data["session_id"]
        print(f"✅ Session created: {session_id}")
        
        # Launch calculator with intent
        launch_response = await mcp_interface.handle_mcp_call("launch_app_with_intent", {
            "app_name": "calculator",
            "intent_description": "Testing calculator functionality with MCP interface",
            "wait_for_launch": True
        })
        
        if launch_response.success:
            print("✅ Calculator launched with visual intent")
            
            # Perform visual intent click
            click_response = await mcp_interface.handle_mcp_call("visual_intent_click", {
                "target": "7",
                "intent_message": "User wants to enter number 7",
                "hover_duration": 1.0
            })
            
            if click_response.success:
                print("✅ Visual intent click demonstrated")
        
        # Generate report
        report_response = await mcp_interface.handle_mcp_call("generate_validation_report", {
            "session_id": session_id,
            "include_screenshots": True,
            "include_recordings": False
        })
        
        if report_response.success:
            print(f"✅ Validation report generated: {report_response.data['report_file']}")
    
    print("\n🏆 MCP Desktop Interface Demo Complete")
    return mcp_interface

if __name__ == "__main__":
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(levelname)s - %(message)s'
    )
    
    asyncio.run(run_mcp_server())