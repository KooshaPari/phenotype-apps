#!/usr/bin/env python3
"""
MCP Claude Code and Cursor Integration Interface

This module provides direct MCP interfaces that allow Claude Code and Cursor
to manipulate desktop environments for testing and automation. It implements
the core MCP functions required for seamless AI-driven desktop interaction.

Key Features:
- Direct desktop control via MCP protocol
- Real-time interaction validation
- Visual feedback for natural user simulation
- Test generation and execution
- Multi-format export capabilities

MCP Functions Implemented:
- kvirtualstage_interact: Perform user actions (click, type, navigate)
- kvirtualstage_record: Start/stop recording with format options
- kvirtualstage_screenshot: Capture current state
- kvirtualstage_script: Execute automation scripts
- kvirtualstage_test: Generate and run automated tests
"""

import asyncio
import json
import logging
import time
import subprocess
import os
from typing import Dict, List, Optional, Any, Union
from dataclasses import dataclass, asdict
from pathlib import Path

# Import existing automation capabilities
from kvirtualstage_mcp_server import KVirtualStageMCPServer
from mcp_desktop_interface import MCPDesktopInterface
from desktop_interaction_validator import DesktopInteractionValidator

logger = logging.getLogger(__name__)

@dataclass
class MCPToolDefinition:
    """Standard MCP tool definition"""
    name: str
    description: str
    input_schema: Dict[str, Any]

@dataclass
class MCPResponse:
    """Standard MCP response format"""
    success: bool
    content: List[Dict[str, Any]]
    error: Optional[str] = None
    metadata: Optional[Dict[str, Any]] = None

class ClaudeCursorMCPInterface:
    """
    Primary MCP interface for Claude Code and Cursor integration
    
    Provides the critical MCP functions that enable AI agents to directly
    control desktop environments for testing and automation workflows.
    """
    
    def __init__(self):
        self.kvs_server = KVirtualStageMCPServer()
        self.desktop_interface = MCPDesktopInterface()
        self.validator = DesktopInteractionValidator()
        
        # Active recording state
        self.recording_active = False
        self.recording_file = None
        self.recording_process = None
        
        # Session management
        self.active_sessions = {}
        self.current_session_id = None
        
        # Tool definitions for MCP protocol
        self.mcp_tools = self._create_mcp_tool_definitions()
        
        logger.info("Claude Code/Cursor MCP Interface initialized")
    
    def _create_mcp_tool_definitions(self) -> List[MCPToolDefinition]:
        """Create MCP tool definitions for Claude Code/Cursor integration"""
        return [
            # Core Desktop Control
            MCPToolDefinition(
                name="kvirtualstage_interact",
                description="Perform user actions on desktop (click, type, navigate) with visual feedback",
                input_schema={
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["click", "type", "key", "move", "scroll", "drag"],
                            "description": "Type of interaction to perform"
                        },
                        "target": {
                            "type": "string",
                            "description": "Target element, text, or description"
                        },
                        "coordinates": {
                            "type": "array",
                            "items": {"type": "number"},
                            "minItems": 2,
                            "maxItems": 2,
                            "description": "X,Y coordinates if target not found"
                        },
                        "text": {
                            "type": "string",
                            "description": "Text to type (for type action)"
                        },
                        "key": {
                            "type": "string",
                            "description": "Key to press (for key action)"
                        },
                        "visual_feedback": {
                            "type": "boolean",
                            "default": True,
                            "description": "Show visual feedback during interaction"
                        },
                        "natural_timing": {
                            "type": "boolean",
                            "default": True,
                            "description": "Use natural human-like timing"
                        }
                    },
                    "required": ["action"]
                }
            ),
            
            # Recording Control
            MCPToolDefinition(
                name="kvirtualstage_record",
                description="Start/stop screen recording with multiple format options",
                input_schema={
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["start", "stop", "pause", "resume"],
                            "description": "Recording action to perform"
                        },
                        "filename": {
                            "type": "string",
                            "description": "Output filename (auto-generated if not provided)"
                        },
                        "format": {
                            "type": "string",
                            "enum": ["mp4", "webm", "gif", "mov"],
                            "default": "mp4",
                            "description": "Output format"
                        },
                        "quality": {
                            "type": "string",
                            "enum": ["low", "medium", "high", "lossless"],
                            "default": "high",
                            "description": "Recording quality"
                        },
                        "fps": {
                            "type": "integer",
                            "minimum": 15,
                            "maximum": 60,
                            "default": 30,
                            "description": "Frames per second"
                        },
                        "region": {
                            "type": "array",
                            "items": {"type": "number"},
                            "minItems": 4,
                            "maxItems": 4,
                            "description": "Recording region [x, y, width, height]"
                        },
                        "show_cursor": {
                            "type": "boolean",
                            "default": True,
                            "description": "Include cursor in recording"
                        },
                        "show_clicks": {
                            "type": "boolean",
                            "default": True,
                            "description": "Highlight clicks in recording"
                        }
                    },
                    "required": ["action"]
                }
            ),
            
            # Screenshot Capture
            MCPToolDefinition(
                name="kvirtualstage_screenshot",
                description="Capture screenshots with annotation and validation capabilities",
                input_schema={
                    "type": "object",
                    "properties": {
                        "filename": {
                            "type": "string",
                            "description": "Screenshot filename (auto-generated if not provided)"
                        },
                        "region": {
                            "type": "array",
                            "items": {"type": "number"},
                            "minItems": 4,
                            "maxItems": 4,
                            "description": "Region to capture [x, y, width, height]"
                        },
                        "annotate": {
                            "type": "boolean",
                            "default": False,
                            "description": "Add annotations to screenshot"
                        },
                        "highlight_cursor": {
                            "type": "boolean",
                            "default": True,
                            "description": "Highlight cursor position"
                        },
                        "format": {
                            "type": "string",
                            "enum": ["png", "jpg", "bmp"],
                            "default": "png",
                            "description": "Image format"
                        },
                        "quality": {
                            "type": "integer",
                            "minimum": 1,
                            "maximum": 100,
                            "default": 95,
                            "description": "Image quality (for jpg)"
                        }
                    }
                }
            ),
            
            # Script Execution
            MCPToolDefinition(
                name="kvirtualstage_script",
                description="Execute automation scripts with validation and error handling",
                input_schema={
                    "type": "object",
                    "properties": {
                        "script_type": {
                            "type": "string",
                            "enum": ["json", "python", "shell", "workflow"],
                            "description": "Type of script to execute"
                        },
                        "script_content": {
                            "type": "string",
                            "description": "Script content or JSON workflow"
                        },
                        "script_path": {
                            "type": "string",
                            "description": "Path to script file"
                        },
                        "parameters": {
                            "type": "object",
                            "description": "Parameters to pass to script"
                        },
                        "validate_before_run": {
                            "type": "boolean",
                            "default": True,
                            "description": "Validate script before execution"
                        },
                        "timeout": {
                            "type": "integer",
                            "default": 300,
                            "description": "Script timeout in seconds"
                        },
                        "visual_feedback": {
                            "type": "boolean",
                            "default": True,
                            "description": "Show visual feedback during execution"
                        }
                    },
                    "required": ["script_type"]
                }
            ),
            
            # Test Generation and Execution
            MCPToolDefinition(
                name="kvirtualstage_test",
                description="Generate and run automated tests for desktop applications",
                input_schema={
                    "type": "object",
                    "properties": {
                        "test_action": {
                            "type": "string",
                            "enum": ["generate", "run", "validate", "report"],
                            "description": "Test action to perform"
                        },
                        "app_name": {
                            "type": "string",
                            "description": "Application to test"
                        },
                        "test_scenarios": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "List of test scenarios to execute"
                        },
                        "test_type": {
                            "type": "string",
                            "enum": ["functional", "ui", "integration", "performance", "accessibility"],
                            "default": "functional",
                            "description": "Type of testing to perform"
                        },
                        "output_format": {
                            "type": "string",
                            "enum": ["json", "html", "junit", "tap"],
                            "default": "json",
                            "description": "Test report format"
                        },
                        "include_screenshots": {
                            "type": "boolean",
                            "default": True,
                            "description": "Include screenshots in test results"
                        },
                        "include_recordings": {
                            "type": "boolean",
                            "default": False,
                            "description": "Include recordings in test results"
                        }
                    },
                    "required": ["test_action"]
                }
            ),
            
            # Session Management
            MCPToolDefinition(
                name="kvirtualstage_session",
                description="Manage desktop automation sessions for Claude Code/Cursor",
                input_schema={
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["create", "destroy", "list", "status", "switch"],
                            "description": "Session action to perform"
                        },
                        "session_id": {
                            "type": "string",
                            "description": "Session identifier"
                        },
                        "session_name": {
                            "type": "string",
                            "description": "Descriptive session name"
                        },
                        "config": {
                            "type": "object",
                            "description": "Session configuration options"
                        }
                    },
                    "required": ["action"]
                }
            ),
            
            # Application Control
            MCPToolDefinition(
                name="kvirtualstage_app",
                description="Launch and control desktop applications for testing",
                input_schema={
                    "type": "object",
                    "properties": {
                        "action": {
                            "type": "string",
                            "enum": ["launch", "close", "focus", "list", "wait"],
                            "description": "Application action to perform"
                        },
                        "app_name": {
                            "type": "string",
                            "description": "Application name or command"
                        },
                        "app_command": {
                            "type": "string",
                            "description": "Command to launch application"
                        },
                        "window_title": {
                            "type": "string",
                            "description": "Expected window title"
                        },
                        "launch_timeout": {
                            "type": "integer",
                            "default": 15,
                            "description": "Timeout for application launch"
                        },
                        "wait_for_ready": {
                            "type": "boolean",
                            "default": True,
                            "description": "Wait for application to be ready"
                        }
                    },
                    "required": ["action"]
                }
            ),
            
            # Export and Reporting
            MCPToolDefinition(
                name="kvirtualstage_export",
                description="Export automation results in multiple formats (GIF, MP4, HTML)",
                input_schema={
                    "type": "object",
                    "properties": {
                        "export_type": {
                            "type": "string",
                            "enum": ["gif", "video", "report", "screenshots", "all"],
                            "description": "Type of export to generate"
                        },
                        "source_files": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Source files to include in export"
                        },
                        "output_path": {
                            "type": "string",
                            "description": "Output directory or file path"
                        },
                        "format_options": {
                            "type": "object",
                            "description": "Format-specific options"
                        },
                        "include_metadata": {
                            "type": "boolean",
                            "default": True,
                            "description": "Include metadata in export"
                        },
                        "compress": {
                            "type": "boolean",
                            "default": False,
                            "description": "Compress output files"
                        }
                    },
                    "required": ["export_type"]
                }
            )
        ]
    
    async def handle_mcp_tool_call(self, tool_name: str, arguments: Dict[str, Any]) -> MCPResponse:
        """Handle MCP tool calls from Claude Code/Cursor"""
        
        logger.info(f"🔧 Claude/Cursor MCP Tool Call: {tool_name}")
        logger.info(f"   Arguments: {arguments}")
        
        try:
            # Route to appropriate handler
            if tool_name == "kvirtualstage_interact":
                return await self._handle_interact(arguments)
            elif tool_name == "kvirtualstage_record":
                return await self._handle_record(arguments)
            elif tool_name == "kvirtualstage_screenshot":
                return await self._handle_screenshot(arguments)
            elif tool_name == "kvirtualstage_script":
                return await self._handle_script(arguments)
            elif tool_name == "kvirtualstage_test":
                return await self._handle_test(arguments)
            elif tool_name == "kvirtualstage_session":
                return await self._handle_session(arguments)
            elif tool_name == "kvirtualstage_app":
                return await self._handle_app(arguments)
            elif tool_name == "kvirtualstage_export":
                return await self._handle_export(arguments)
            else:
                return MCPResponse(
                    success=False,
                    content=[{"type": "text", "text": f"Unknown tool: {tool_name}"}],
                    error=f"Tool '{tool_name}' not implemented"
                )
                
        except Exception as e:
            logger.error(f"MCP tool call failed: {e}")
            return MCPResponse(
                success=False,
                content=[{"type": "text", "text": f"Tool execution failed: {str(e)}"}],
                error=str(e)
            )
    
    async def _handle_interact(self, args: Dict[str, Any]) -> MCPResponse:
        """Handle desktop interaction requests"""
        
        action = args["action"]
        target = args.get("target")
        coordinates = args.get("coordinates")
        text = args.get("text")
        key = args.get("key")
        visual_feedback = args.get("visual_feedback", True)
        natural_timing = args.get("natural_timing", True)
        
        results = []
        
        try:
            if action == "click":
                # Perform click with visual feedback
                if target:
                    success = await self.desktop_interface.handle_mcp_call("visual_intent_click", {
                        "target": target,
                        "coordinates": coordinates,
                        "intent_message": f"Claude Code clicking on {target}",
                        "hover_duration": 1.0 if natural_timing else 0.1
                    })
                elif coordinates:
                    # Use direct coordinates
                    await self.validator.visual_intent_click_coordinates(
                        coordinates[0], coordinates[1], 
                        f"Claude Code clicking at ({coordinates[0]}, {coordinates[1]})"
                    )
                    success = MCPResponse(success=True, data={})
                else:
                    raise ValueError("Either target or coordinates must be provided for click action")
                
                results.append({
                    "type": "text",
                    "text": f"Clicked {'on ' + target if target else 'at coordinates ' + str(coordinates)}: {'Success' if success.success else 'Failed'}"
                })
                
            elif action == "type":
                if not text:
                    raise ValueError("Text is required for type action")
                
                # Perform natural typing
                success = await self.desktop_interface.handle_mcp_call("natural_type_text", {
                    "text": text,
                    "intent_message": f"Claude Code typing: {text[:50]}...",
                    "typing_speed": 0.15 if natural_timing else 0.05,
                    "allow_mistakes": natural_timing
                })
                
                results.append({
                    "type": "text",
                    "text": f"Typed text '{text}': {'Success' if success.success else 'Failed'}"
                })
                
            elif action == "key":
                if not key:
                    raise ValueError("Key is required for key action")
                
                # Press key
                if natural_timing:
                    await asyncio.sleep(0.1)
                
                subprocess.run(['xdotool', 'key', key])
                
                results.append({
                    "type": "text",
                    "text": f"Pressed key '{key}': Success"
                })
                
            elif action == "move":
                if not coordinates:
                    raise ValueError("Coordinates are required for move action")
                
                # Move cursor with visual feedback
                success = await self.desktop_interface.handle_mcp_call("kvirtualstage_cursor_move", {
                    "x": coordinates[0],
                    "y": coordinates[1],
                    "movement_style": "human" if natural_timing else "direct",
                    "show_path": visual_feedback
                })
                
                results.append({
                    "type": "text",
                    "text": f"Moved cursor to ({coordinates[0]}, {coordinates[1]}): {'Success' if success.success else 'Failed'}"
                })
                
            else:
                raise ValueError(f"Unknown action: {action}")
            
            return MCPResponse(
                success=True,
                content=results,
                metadata={
                    "action": action,
                    "visual_feedback_enabled": visual_feedback,
                    "natural_timing_enabled": natural_timing
                }
            )
            
        except Exception as e:
            return MCPResponse(
                success=False,
                content=[{"type": "text", "text": f"Interaction failed: {str(e)}"}],
                error=str(e)
            )
    
    async def _handle_record(self, args: Dict[str, Any]) -> MCPResponse:
        """Handle recording requests"""
        
        action = args["action"]
        filename = args.get("filename")
        format_type = args.get("format", "mp4")
        quality = args.get("quality", "high")
        fps = args.get("fps", 30)
        region = args.get("region")
        show_cursor = args.get("show_cursor", True)
        show_clicks = args.get("show_clicks", True)
        
        try:
            if action == "start":
                if self.recording_active:
                    return MCPResponse(
                        success=False,
                        content=[{"type": "text", "text": "Recording already active"}],
                        error="Recording already in progress"
                    )
                
                # Generate filename if not provided
                if not filename:
                    timestamp = int(time.time())
                    filename = f"/tmp/claude_cursor_recording_{timestamp}.{format_type}"
                
                # Start recording with ffmpeg
                cmd = self._build_recording_command(filename, format_type, quality, fps, region, show_cursor)
                self.recording_process = subprocess.Popen(cmd)
                self.recording_active = True
                self.recording_file = filename
                
                return MCPResponse(
                    success=True,
                    content=[{"type": "text", "text": f"Recording started: {filename}"}],
                    metadata={
                        "filename": filename,
                        "format": format_type,
                        "recording_active": True
                    }
                )
                
            elif action == "stop":
                if not self.recording_active:
                    return MCPResponse(
                        success=False,
                        content=[{"type": "text", "text": "No recording active"}],
                        error="No recording in progress"
                    )
                
                # Stop recording
                if self.recording_process:
                    self.recording_process.terminate()
                    await asyncio.sleep(2)  # Allow graceful termination
                    if self.recording_process.poll() is None:
                        self.recording_process.kill()
                
                recording_file = self.recording_file
                self.recording_active = False
                self.recording_file = None
                self.recording_process = None
                
                return MCPResponse(
                    success=True,
                    content=[{"type": "text", "text": f"Recording stopped: {recording_file}"}],
                    metadata={
                        "filename": recording_file,
                        "recording_active": False
                    }
                )
                
            else:
                return MCPResponse(
                    success=False,
                    content=[{"type": "text", "text": f"Unknown recording action: {action}"}],
                    error=f"Action '{action}' not supported"
                )
                
        except Exception as e:
            return MCPResponse(
                success=False,
                content=[{"type": "text", "text": f"Recording operation failed: {str(e)}"}],
                error=str(e)
            )
    
    async def _handle_screenshot(self, args: Dict[str, Any]) -> MCPResponse:
        """Handle screenshot requests"""
        
        filename = args.get("filename")
        region = args.get("region")
        annotate = args.get("annotate", False)
        highlight_cursor = args.get("highlight_cursor", True)
        format_type = args.get("format", "png")
        quality = args.get("quality", 95)
        
        try:
            # Generate filename if not provided
            if not filename:
                timestamp = int(time.time())
                filename = f"/tmp/claude_cursor_screenshot_{timestamp}.{format_type}"
            
            # Take screenshot
            success = await self.desktop_interface.handle_mcp_call("take_validation_screenshot", {
                "screenshot_name": os.path.basename(filename),
                "add_annotations": annotate
            })
            
            if success.success:
                screenshot_path = success.data["screenshot_path"]
                
                return MCPResponse(
                    success=True,
                    content=[
                        {"type": "text", "text": f"Screenshot captured: {screenshot_path}"},
                        {"type": "image", "url": f"file://{screenshot_path}"}
                    ],
                    metadata={
                        "filename": screenshot_path,
                        "format": format_type,
                        "annotated": annotate,
                        "cursor_highlighted": highlight_cursor
                    }
                )
            else:
                return MCPResponse(
                    success=False,
                    content=[{"type": "text", "text": "Screenshot capture failed"}],
                    error="Failed to capture screenshot"
                )
                
        except Exception as e:
            return MCPResponse(
                success=False,
                content=[{"type": "text", "text": f"Screenshot operation failed: {str(e)}"}],
                error=str(e)
            )
    
    async def _handle_script(self, args: Dict[str, Any]) -> MCPResponse:
        """Handle script execution requests"""
        
        script_type = args["script_type"]
        script_content = args.get("script_content")
        script_path = args.get("script_path")
        parameters = args.get("parameters", {})
        validate_before_run = args.get("validate_before_run", True)
        timeout = args.get("timeout", 300)
        visual_feedback = args.get("visual_feedback", True)
        
        try:
            if script_type == "json":
                # Execute JSON workflow
                if script_content:
                    workflow = json.loads(script_content)
                elif script_path:
                    with open(script_path, 'r') as f:
                        workflow = json.load(f)
                else:
                    raise ValueError("Either script_content or script_path must be provided")
                
                # Execute workflow steps
                results = await self._execute_json_workflow(workflow, parameters, visual_feedback)
                
                return MCPResponse(
                    success=True,
                    content=[{"type": "text", "text": f"Workflow executed: {len(results)} steps completed"}],
                    metadata={
                        "script_type": script_type,
                        "steps_executed": len(results),
                        "results": results
                    }
                )
                
            elif script_type == "python":
                # Execute Python script (limited for security)
                return MCPResponse(
                    success=False,
                    content=[{"type": "text", "text": "Python script execution not supported for security reasons"}],
                    error="Python execution disabled"
                )
                
            else:
                return MCPResponse(
                    success=False,
                    content=[{"type": "text", "text": f"Unsupported script type: {script_type}"}],
                    error=f"Script type '{script_type}' not supported"
                )
                
        except Exception as e:
            return MCPResponse(
                success=False,
                content=[{"type": "text", "text": f"Script execution failed: {str(e)}"}],
                error=str(e)
            )
    
    async def _handle_test(self, args: Dict[str, Any]) -> MCPResponse:
        """Handle test generation and execution requests"""
        
        test_action = args["test_action"]
        app_name = args.get("app_name")
        test_scenarios = args.get("test_scenarios", [])
        test_type = args.get("test_type", "functional")
        output_format = args.get("output_format", "json")
        include_screenshots = args.get("include_screenshots", True)
        include_recordings = args.get("include_recordings", False)
        
        try:
            if test_action == "generate":
                # Generate test scenarios for application
                if not app_name:
                    raise ValueError("app_name is required for test generation")
                
                generated_tests = await self._generate_test_scenarios(app_name, test_type)
                
                return MCPResponse(
                    success=True,
                    content=[{"type": "text", "text": f"Generated {len(generated_tests)} test scenarios for {app_name}"}],
                    metadata={
                        "app_name": app_name,
                        "test_type": test_type,
                        "generated_tests": generated_tests
                    }
                )
                
            elif test_action == "run":
                # Execute test scenarios
                if not test_scenarios:
                    raise ValueError("test_scenarios is required for test execution")
                
                test_results = await self._execute_test_scenarios(test_scenarios, app_name, include_screenshots)
                
                return MCPResponse(
                    success=True,
                    content=[{"type": "text", "text": f"Executed {len(test_results)} test scenarios"}],
                    metadata={
                        "test_results": test_results,
                        "app_name": app_name,
                        "screenshots_included": include_screenshots
                    }
                )
                
            else:
                return MCPResponse(
                    success=False,
                    content=[{"type": "text", "text": f"Unknown test action: {test_action}"}],
                    error=f"Test action '{test_action}' not supported"
                )
                
        except Exception as e:
            return MCPResponse(
                success=False,
                content=[{"type": "text", "text": f"Test operation failed: {str(e)}"}],
                error=str(e)
            )
    
    async def _handle_session(self, args: Dict[str, Any]) -> MCPResponse:
        """Handle session management requests"""
        
        action = args["action"]
        session_id = args.get("session_id")
        session_name = args.get("session_name")
        config = args.get("config", {})
        
        try:
            if action == "create":
                # Create new session
                success = await self.desktop_interface.handle_mcp_call("create_validation_session", {
                    "session_name": session_name or f"claude_cursor_session_{int(time.time())}",
                    "visual_intent": True,
                    "apps_to_test": config.get("apps_to_test", ["calculator", "text_editor"])
                })
                
                if success.success:
                    new_session_id = success.data["session_id"]
                    self.current_session_id = new_session_id
                    
                    return MCPResponse(
                        success=True,
                        content=[{"type": "text", "text": f"Session created: {new_session_id}"}],
                        metadata={
                            "session_id": new_session_id,
                            "session_name": session_name,
                            "active": True
                        }
                    )
                else:
                    return MCPResponse(
                        success=False,
                        content=[{"type": "text", "text": "Failed to create session"}],
                        error="Session creation failed"
                    )
                    
            elif action == "list":
                # List active sessions
                sessions = list(self.active_sessions.keys())
                
                return MCPResponse(
                    success=True,
                    content=[{"type": "text", "text": f"Active sessions: {', '.join(sessions) if sessions else 'None'}"}],
                    metadata={
                        "sessions": sessions,
                        "current_session": self.current_session_id
                    }
                )
                
            else:
                return MCPResponse(
                    success=False,
                    content=[{"type": "text", "text": f"Unknown session action: {action}"}],
                    error=f"Session action '{action}' not supported"
                )
                
        except Exception as e:
            return MCPResponse(
                success=False,
                content=[{"type": "text", "text": f"Session operation failed: {str(e)}"}],
                error=str(e)
            )
    
    async def _handle_app(self, args: Dict[str, Any]) -> MCPResponse:
        """Handle application control requests"""
        
        action = args["action"]
        app_name = args.get("app_name")
        app_command = args.get("app_command")
        window_title = args.get("window_title")
        launch_timeout = args.get("launch_timeout", 15)
        wait_for_ready = args.get("wait_for_ready", True)
        
        try:
            if action == "launch":
                if not app_name:
                    raise ValueError("app_name is required for launch action")
                
                # Launch application with intent
                success = await self.desktop_interface.handle_mcp_call("launch_app_with_intent", {
                    "app_name": app_name,
                    "intent_description": f"Claude Code launching {app_name} for automation",
                    "wait_for_launch": wait_for_ready
                })
                
                return MCPResponse(
                    success=success.success,
                    content=[{"type": "text", "text": f"{'Successfully launched' if success.success else 'Failed to launch'} {app_name}"}],
                    metadata={
                        "app_name": app_name,
                        "launched": success.success,
                        "window_ready": success.data.get("window_ready", False) if success.success else False
                    }
                )
                
            elif action == "list":
                # List running applications
                window_result = await self.kvs_server.handle_tool_call("kvs_window_manage", {
                    "action": "list"
                })
                
                windows = window_result.get("windows", []) if window_result.get("success") else []
                
                return MCPResponse(
                    success=True,
                    content=[{"type": "text", "text": f"Running applications: {len(windows)} windows found"}],
                    metadata={
                        "windows": windows,
                        "window_count": len(windows)
                    }
                )
                
            else:
                return MCPResponse(
                    success=False,
                    content=[{"type": "text", "text": f"Unknown app action: {action}"}],
                    error=f"Application action '{action}' not supported"
                )
                
        except Exception as e:
            return MCPResponse(
                success=False,
                content=[{"type": "text", "text": f"Application operation failed: {str(e)}"}],
                error=str(e)
            )
    
    async def _handle_export(self, args: Dict[str, Any]) -> MCPResponse:
        """Handle export requests"""
        
        export_type = args["export_type"]
        source_files = args.get("source_files", [])
        output_path = args.get("output_path", "/tmp")
        format_options = args.get("format_options", {})
        include_metadata = args.get("include_metadata", True)
        compress = args.get("compress", False)
        
        try:
            exported_files = []
            
            if export_type == "gif":
                # Convert video to GIF
                if self.recording_file and os.path.exists(self.recording_file):
                    gif_path = self._convert_to_gif(self.recording_file, output_path, format_options)
                    exported_files.append(gif_path)
                
            elif export_type == "video":
                # Process video file
                if self.recording_file and os.path.exists(self.recording_file):
                    processed_path = self._process_video(self.recording_file, output_path, format_options)
                    exported_files.append(processed_path)
                
            elif export_type == "report":
                # Generate HTML report
                report_path = await self._generate_html_report(output_path, include_metadata)
                exported_files.append(report_path)
                
            else:
                return MCPResponse(
                    success=False,
                    content=[{"type": "text", "text": f"Unknown export type: {export_type}"}],
                    error=f"Export type '{export_type}' not supported"
                )
            
            return MCPResponse(
                success=True,
                content=[{"type": "text", "text": f"Export completed: {len(exported_files)} files created"}],
                metadata={
                    "export_type": export_type,
                    "exported_files": exported_files,
                    "output_path": output_path
                }
            )
            
        except Exception as e:
            return MCPResponse(
                success=False,
                content=[{"type": "text", "text": f"Export operation failed: {str(e)}"}],
                error=str(e)
            )
    
    # Helper methods
    
    def _build_recording_command(self, filename: str, format_type: str, quality: str, 
                               fps: int, region: Optional[List[int]], show_cursor: bool) -> List[str]:
        """Build ffmpeg command for screen recording"""
        
        cmd = ["ffmpeg", "-y"]  # -y to overwrite output files
        
        # Input options
        cmd.extend(["-f", "x11grab"])
        cmd.extend(["-r", str(fps)])
        
        if region:
            cmd.extend(["-s", f"{region[2]}x{region[3]}"])
            cmd.extend(["-i", f":0.0+{region[0]},{region[1]}"])
        else:
            cmd.extend(["-i", ":0.0"])
        
        # Video codec options
        if format_type == "mp4":
            cmd.extend(["-c:v", "libx264"])
            if quality == "high":
                cmd.extend(["-crf", "18"])
            elif quality == "medium":
                cmd.extend(["-crf", "23"])
            else:
                cmd.extend(["-crf", "28"])
        
        # Output
        cmd.append(filename)
        
        return cmd
    
    def _convert_to_gif(self, video_path: str, output_path: str, options: Dict[str, Any]) -> str:
        """Convert video to optimized GIF"""
        
        gif_path = os.path.join(output_path, f"animation_{int(time.time())}.gif")
        
        # Use ffmpeg to create optimized GIF
        cmd = [
            "ffmpeg", "-y", "-i", video_path,
            "-vf", "fps=10,scale=800:-1:flags=lanczos,palettegen",
            f"{gif_path}_palette.png"
        ]
        subprocess.run(cmd)
        
        cmd = [
            "ffmpeg", "-y", "-i", video_path, "-i", f"{gif_path}_palette.png",
            "-filter_complex", "fps=10,scale=800:-1:flags=lanczos[x];[x][1:v]paletteuse",
            gif_path
        ]
        subprocess.run(cmd)
        
        # Clean up palette file
        os.remove(f"{gif_path}_palette.png")
        
        return gif_path
    
    def _process_video(self, video_path: str, output_path: str, options: Dict[str, Any]) -> str:
        """Process and optimize video file"""
        
        processed_path = os.path.join(output_path, f"processed_{int(time.time())}.mp4")
        
        # Process with ffmpeg
        cmd = [
            "ffmpeg", "-y", "-i", video_path,
            "-c:v", "libx264", "-crf", "23",
            "-preset", "medium",
            processed_path
        ]
        subprocess.run(cmd)
        
        return processed_path
    
    async def _generate_html_report(self, output_path: str, include_metadata: bool) -> str:
        """Generate HTML report of automation session"""
        
        report_path = os.path.join(output_path, f"automation_report_{int(time.time())}.html")
        
        html_content = f"""
        <!DOCTYPE html>
        <html>
        <head>
            <title>Claude Code/Cursor Automation Report</title>
            <style>
                body {{ font-family: Arial, sans-serif; margin: 20px; }}
                .header {{ background: #f0f0f0; padding: 20px; border-radius: 5px; }}
                .section {{ margin: 20px 0; }}
                .screenshot {{ max-width: 800px; margin: 10px 0; }}
                .metadata {{ background: #f9f9f9; padding: 15px; border-radius: 5px; }}
            </style>
        </head>
        <body>
            <div class="header">
                <h1>Desktop Automation Report</h1>
                <p>Generated by Claude Code/Cursor MCP Interface</p>
                <p>Timestamp: {time.strftime('%Y-%m-%d %H:%M:%S')}</p>
            </div>
            
            <div class="section">
                <h2>Session Summary</h2>
                <p>Session ID: {self.current_session_id or 'None'}</p>
                <p>Recording Active: {self.recording_active}</p>
                <p>Recording File: {self.recording_file or 'None'}</p>
            </div>
            
            <div class="section">
                <h2>Automation Capabilities</h2>
                <ul>
                    <li>Desktop Interaction (Click, Type, Key Press)</li>
                    <li>Screen Recording (MP4, WebM, GIF)</li>
                    <li>Screenshot Capture (PNG, JPG)</li>
                    <li>Script Execution (JSON Workflows)</li>
                    <li>Test Generation and Execution</li>
                    <li>Application Control</li>
                    <li>Visual Feedback and Natural Timing</li>
                </ul>
            </div>
            
            <div class="section">
                <h2>MCP Tools Available</h2>
                <ul>
        """
        
        for tool in self.mcp_tools:
            html_content += f"<li><strong>{tool.name}</strong>: {tool.description}</li>\n"
        
        html_content += """
                </ul>
            </div>
        </body>
        </html>
        """
        
        with open(report_path, 'w') as f:
            f.write(html_content)
        
        return report_path
    
    async def _execute_json_workflow(self, workflow: Dict[str, Any], 
                                   parameters: Dict[str, Any], 
                                   visual_feedback: bool) -> List[Dict[str, Any]]:
        """Execute JSON workflow steps"""
        
        results = []
        steps = workflow.get("steps", [])
        
        for step in steps:
            step_type = step.get("type")
            step_args = step.get("args", {})
            
            # Merge parameters into step args
            step_args.update(parameters)
            
            if step_type == "click":
                result = await self._handle_interact({"action": "click", **step_args})
            elif step_type == "type":
                result = await self._handle_interact({"action": "type", **step_args})
            elif step_type == "screenshot":
                result = await self._handle_screenshot(step_args)
            elif step_type == "wait":
                wait_time = step_args.get("duration", 1.0)
                await asyncio.sleep(wait_time)
                result = MCPResponse(success=True, content=[{"type": "text", "text": f"Waited {wait_time} seconds"}])
            else:
                result = MCPResponse(success=False, content=[{"type": "text", "text": f"Unknown step type: {step_type}"}])
            
            results.append({
                "step": step,
                "success": result.success,
                "result": result.content
            })
            
            if not result.success:
                logger.warning(f"Workflow step failed: {step}")
        
        return results
    
    async def _generate_test_scenarios(self, app_name: str, test_type: str) -> List[Dict[str, Any]]:
        """Generate test scenarios for application"""
        
        scenarios = []
        
        if app_name.lower() == "calculator":
            scenarios = [
                {
                    "name": "Basic Addition Test",
                    "description": "Test basic addition functionality",
                    "steps": [
                        {"type": "click", "target": "7"},
                        {"type": "click", "target": "+"},
                        {"type": "click", "target": "3"},
                        {"type": "click", "target": "="},
                        {"type": "screenshot"}
                    ]
                },
                {
                    "name": "Clear Function Test",
                    "description": "Test calculator clear functionality",
                    "steps": [
                        {"type": "click", "target": "5"},
                        {"type": "click", "target": "C"},
                        {"type": "screenshot"}
                    ]
                }
            ]
        elif app_name.lower() == "text_editor":
            scenarios = [
                {
                    "name": "Text Input Test",
                    "description": "Test text input functionality",
                    "steps": [
                        {"type": "type", "text": "Hello from Claude Code!"},
                        {"type": "screenshot"}
                    ]
                },
                {
                    "name": "Save Document Test",
                    "description": "Test document saving",
                    "steps": [
                        {"type": "type", "text": "Test document content"},
                        {"type": "key", "key": "ctrl+s"},
                        {"type": "screenshot"}
                    ]
                }
            ]
        
        return scenarios
    
    async def _execute_test_scenarios(self, scenarios: List[str], app_name: Optional[str], 
                                    include_screenshots: bool) -> List[Dict[str, Any]]:
        """Execute test scenarios"""
        
        results = []
        
        for scenario_name in scenarios:
            try:
                # Generate or find scenario
                if app_name:
                    generated_scenarios = await self._generate_test_scenarios(app_name, "functional")
                    scenario = next((s for s in generated_scenarios if s["name"] == scenario_name), None)
                else:
                    scenario = None
                
                if not scenario:
                    results.append({
                        "scenario": scenario_name,
                        "success": False,
                        "error": "Scenario not found"
                    })
                    continue
                
                # Execute scenario steps
                step_results = await self._execute_json_workflow(scenario, {}, True)
                
                success = all(step["success"] for step in step_results)
                
                results.append({
                    "scenario": scenario_name,
                    "success": success,
                    "steps_executed": len(step_results),
                    "step_results": step_results if include_screenshots else None
                })
                
            except Exception as e:
                results.append({
                    "scenario": scenario_name,
                    "success": False,
                    "error": str(e)
                })
        
        return results
    
    def get_tool_definitions(self) -> List[Dict[str, Any]]:
        """Get MCP tool definitions for Claude Code/Cursor"""
        return [
            {
                "name": tool.name,
                "description": tool.description,
                "inputSchema": tool.input_schema
            }
            for tool in self.mcp_tools
        ]

# CLI Integration for MCP Interface
async def demo_claude_cursor_integration():
    """Demonstrate Claude Code/Cursor MCP integration"""
    
    interface = ClaudeCursorMCPInterface()
    
    print("🚀 Claude Code/Cursor MCP Integration Demo")
    print("===========================================")
    
    # Create session
    session_result = await interface.handle_mcp_tool_call("kvirtualstage_session", {
        "action": "create",
        "session_name": "claude_cursor_demo"
    })
    print(f"Session Creation: {session_result.success}")
    
    if session_result.success:
        # Launch calculator
        app_result = await interface.handle_mcp_tool_call("kvirtualstage_app", {
            "action": "launch",
            "app_name": "calculator"
        })
        print(f"App Launch: {app_result.success}")
        
        # Perform interaction
        if app_result.success:
            click_result = await interface.handle_mcp_tool_call("kvirtualstage_interact", {
                "action": "click",
                "target": "7",
                "visual_feedback": True,
                "natural_timing": True
            })
            print(f"Interaction: {click_result.success}")
            
            # Take screenshot
            screenshot_result = await interface.handle_mcp_tool_call("kvirtualstage_screenshot", {
                "annotate": True,
                "highlight_cursor": True
            })
            print(f"Screenshot: {screenshot_result.success}")
    
    print("\n✅ Claude Code/Cursor MCP Integration Demo Complete!")
    print(f"Available tools: {len(interface.mcp_tools)}")
    
    # Print tool summary
    print("\n📋 Available MCP Tools:")
    for tool in interface.mcp_tools:
        print(f"  • {tool.name}: {tool.description}")

if __name__ == "__main__":
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(levelname)s - %(message)s'
    )
    
    asyncio.run(demo_claude_cursor_integration())
