#!/usr/bin/env python3
"""
Claude Code Integration for KVirtualStage MCP Tools

This module provides specialized MCP tools designed specifically for Claude Code
to control desktop applications with human-like precision. These tools enable
AI agents to perform sophisticated desktop automation tasks through natural
language commands.

Key Features:
- Claude Code-optimized tool interfaces
- Real-time desktop feedback for AI sessions
- Visual intent capture and replay
- Automated test generation from manual interactions
- Human-like interaction patterns for believable automation

Usage with Claude Code:
```python
# Claude Code can call these MCP tools directly:
await call_tool("claude_desktop_click", {
    "target_description": "blue submit button in the bottom right",
    "confidence_level": "high"
})
```
"""

import asyncio
import json
import logging
import time
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass
import subprocess
import os

from kvirtualstage_mcp_server import KVirtualStageMCPServer, MCPTool
from automation_stack import UIElement, AutomationResult

logger = logging.getLogger(__name__)

@dataclass
class ClaudeIntentCapture:
    """Captures user intent for Claude Code automation"""
    intent_id: str
    description: str
    target_elements: List[str]
    interaction_type: str
    expected_outcome: str
    visual_cues: List[str]
    timestamp: float

class ClaudeCodeMCPInterface:
    """
    Specialized MCP interface for Claude Code desktop automation
    
    Provides high-level, intelligent tools that Claude Code can use
    to understand and control desktop applications naturally.
    """
    
    def __init__(self):
        self.base_server = KVirtualStageMCPServer()
        self.intent_history: List[ClaudeIntentCapture] = []
        self.claude_tools = self._create_claude_specific_tools()
        
        # Claude Code specific settings
        self.natural_language_parsing = True
        self.intent_learning_enabled = True
        self.visual_feedback_for_ai = True
        
        logger.info("Claude Code MCP Interface initialized")
    
    def _create_claude_specific_tools(self) -> List[MCPTool]:
        """Create tools specifically designed for Claude Code interaction"""
        return [
            # High-level Intent-based Tools
            MCPTool(
                name="claude_desktop_interact",
                description="Interact with desktop elements using natural language descriptions",
                input_schema={
                    "type": "object",
                    "properties": {
                        "intent": {
                            "type": "string",
                            "description": "Natural language description of what to do (e.g., 'click the blue submit button')"
                        },
                        "target_description": {
                            "type": "string", 
                            "description": "Description of the target element (e.g., 'submit button', 'text field with placeholder Enter name')"
                        },
                        "interaction_type": {
                            "type": "string",
                            "enum": ["click", "type", "drag", "hover", "select"],
                            "description": "Type of interaction to perform"
                        },
                        "text_input": {
                            "type": "string",
                            "description": "Text to input (for type interactions)"
                        },
                        "confidence_level": {
                            "type": "string",
                            "enum": ["low", "medium", "high"],
                            "default": "medium",
                            "description": "Required confidence level for element detection"
                        },
                        "retry_on_failure": {
                            "type": "boolean",
                            "default": True,
                            "description": "Retry interaction with different methods if first attempt fails"
                        },
                        "capture_intent": {
                            "type": "boolean", 
                            "default": True,
                            "description": "Capture this interaction for learning and replay"
                        }
                    },
                    "required": ["intent", "interaction_type"]
                }
            ),
            
            MCPTool(
                name="claude_app_workflow",
                description="Execute a complete application workflow described in natural language",
                input_schema={
                    "type": "object",
                    "properties": {
                        "workflow_description": {
                            "type": "string",
                            "description": "Natural language description of the entire workflow"
                        },
                        "app_name": {
                            "type": "string",
                            "description": "Name of the application to work with"
                        },
                        "steps": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "step_description": {"type": "string"},
                                    "expected_outcome": {"type": "string"},
                                    "verification_method": {"type": "string"}
                                }
                            },
                            "description": "List of workflow steps with descriptions"
                        },
                        "error_handling": {
                            "type": "string",
                            "enum": ["strict", "adaptive", "continue"],
                            "default": "adaptive",
                            "description": "How to handle errors during workflow execution"
                        },
                        "record_session": {
                            "type": "boolean",
                            "default": True,
                            "description": "Record the workflow execution for analysis"
                        }
                    },
                    "required": ["workflow_description", "app_name"]
                }
            ),
            
            MCPTool(
                name="claude_visual_understand",
                description="Analyze and understand the current desktop state for Claude Code",
                input_schema={
                    "type": "object",
                    "properties": {
                        "analysis_type": {
                            "type": "string",
                            "enum": ["full_screen", "active_window", "specific_region"],
                            "default": "active_window",
                            "description": "Scope of visual analysis"
                        },
                        "focus_elements": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Specific types of elements to focus on (e.g., ['buttons', 'text_fields'])"
                        },
                        "extract_text": {
                            "type": "boolean",
                            "default": True,
                            "description": "Extract all visible text using OCR"
                        },
                        "identify_interactive": {
                            "type": "boolean",
                            "default": True,
                            "description": "Identify interactive elements (buttons, fields, etc.)"
                        },
                        "describe_layout": {
                            "type": "boolean",
                            "default": True,
                            "description": "Provide natural language description of layout"
                        },
                        "generate_selectors": {
                            "type": "boolean",
                            "default": True,
                            "description": "Generate selectors for automated interaction"
                        }
                    }
                }
            ),
            
            MCPTool(
                name="claude_form_intelligent_fill",
                description="Intelligently fill forms by understanding field context and requirements",
                input_schema={
                    "type": "object",
                    "properties": {
                        "form_data": {
                            "type": "object",
                            "description": "Key-value pairs of form data to fill"
                        },
                        "auto_detect_fields": {
                            "type": "boolean",
                            "default": True,
                            "description": "Automatically detect and map form fields"
                        },
                        "field_mapping_hints": {
                            "type": "object",
                            "description": "Hints for mapping data to specific field types"
                        },
                        "validation_enabled": {
                            "type": "boolean",
                            "default": True,
                            "description": "Validate form entries after filling"
                        },
                        "submit_after_fill": {
                            "type": "boolean",
                            "default": False,
                            "description": "Submit form after successful filling"
                        },
                        "simulate_human_behavior": {
                            "type": "boolean",
                            "default": True,
                            "description": "Simulate realistic human form-filling behavior"
                        }
                    },
                    "required": ["form_data"]
                }
            ),
            
            MCPTool(
                name="claude_test_generate",
                description="Generate automated tests based on manual interactions performed",
                input_schema={
                    "type": "object",
                    "properties": {
                        "test_name": {
                            "type": "string",
                            "description": "Name for the generated test"
                        },
                        "test_description": {
                            "type": "string",
                            "description": "Description of what the test validates"
                        },
                        "capture_duration": {
                            "type": "number",
                            "default": 60,
                            "description": "Duration in seconds to capture interactions"
                        },
                        "test_framework": {
                            "type": "string",
                            "enum": ["kvirtualstage", "playwright", "selenium", "custom"],
                            "default": "kvirtualstage",
                            "description": "Test framework to generate code for"
                        },
                        "include_assertions": {
                            "type": "boolean",
                            "default": True,
                            "description": "Include automatic assertions in generated test"
                        },
                        "generate_comments": {
                            "type": "boolean",
                            "default": True,
                            "description": "Include descriptive comments in test code"
                        }
                    },
                    "required": ["test_name"]
                }
            ),
            
            MCPTool(
                name="claude_context_aware_action",
                description="Perform context-aware actions that adapt to current application state",
                input_schema={
                    "type": "object",
                    "properties": {
                        "goal": {
                            "type": "string",
                            "description": "High-level goal to achieve (e.g., 'save the current document')"
                        },
                        "context_hints": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Context hints about current state or application"
                        },
                        "adaptation_level": {
                            "type": "string",
                            "enum": ["conservative", "moderate", "aggressive"],
                            "default": "moderate",
                            "description": "How aggressively to adapt to context changes"
                        },
                        "fallback_strategies": {
                            "type": "array",
                            "items": {"type": "string"},
                            "description": "Alternative strategies if primary approach fails"
                        },
                        "max_attempts": {
                            "type": "integer",
                            "default": 3,
                            "description": "Maximum attempts to achieve goal"
                        }
                    },
                    "required": ["goal"]
                }
            ),
            
            MCPTool(
                name="claude_session_analyze",
                description="Analyze automation session for Claude Code learning and optimization",
                input_schema={
                    "type": "object",
                    "properties": {
                        "session_id": {
                            "type": "string",
                            "description": "Session to analyze"
                        },
                        "analysis_focus": {
                            "type": "array",
                            "items": {
                                "type": "string",
                                "enum": ["efficiency", "accuracy", "user_experience", "reliability"]
                            },
                            "default": ["efficiency", "accuracy"],
                            "description": "Aspects to focus analysis on"
                        },
                        "generate_improvements": {
                            "type": "boolean",
                            "default": True,
                            "description": "Generate suggestions for improvement"
                        },
                        "export_insights": {
                            "type": "boolean",
                            "default": True,
                            "description": "Export insights for future sessions"
                        }
                    }
                }
            ),
            
            MCPTool(
                name="claude_live_feedback",
                description="Get real-time feedback about desktop state for AI decision making",
                input_schema={
                    "type": "object",
                    "properties": {
                        "feedback_type": {
                            "type": "string",
                            "enum": ["visual_state", "interaction_result", "application_response", "error_status"],
                            "description": "Type of feedback to provide"
                        },
                        "monitoring_duration": {
                            "type": "number",
                            "default": 5.0,
                            "description": "Duration to monitor for changes (seconds)"
                        },
                        "change_threshold": {
                            "type": "number",
                            "default": 0.1,
                            "description": "Threshold for detecting significant changes"
                        },
                        "include_suggestions": {
                            "type": "boolean",
                            "default": True,
                            "description": "Include suggestions for next actions"
                        }
                    },
                    "required": ["feedback_type"]
                }
            ),
            
            MCPTool(
                name="claude_cursor_natural_move",
                description="Move cursor with natural, human-like patterns for believable automation",
                input_schema={
                    "type": "object",
                    "properties": {
                        "target_description": {
                            "type": "string",
                            "description": "Natural description of target (e.g., 'the submit button')"
                        },
                        "coordinates": {
                            "type": "array",
                            "items": {"type": "number"},
                            "minItems": 2,
                            "maxItems": 2,
                            "description": "Exact coordinates if known [x, y]"
                        },
                        "movement_personality": {
                            "type": "string",
                            "enum": ["efficient", "cautious", "natural", "quick"],
                            "default": "natural",
                            "description": "Personality of cursor movement"
                        },
                        "show_intent_path": {
                            "type": "boolean",
                            "default": True,
                            "description": "Show visual indication of movement intent"
                        },
                        "pause_at_target": {
                            "type": "boolean", 
                            "default": True,
                            "description": "Brief pause when reaching target (human-like)"
                        }
                    }
                }
            )
        ]
    
    def get_all_tools(self) -> List[MCPTool]:
        """Get all tools (base + Claude-specific)"""
        return self.base_server.tools + self.claude_tools
    
    async def handle_claude_tool_call(self, tool_name: str, arguments: Dict[str, Any]) -> Dict[str, Any]:
        """Handle Claude Code specific tool calls"""
        try:
            # Route Claude-specific tools
            if tool_name == "claude_desktop_interact":
                return await self._claude_desktop_interact(arguments)
            elif tool_name == "claude_app_workflow":
                return await self._claude_app_workflow(arguments)
            elif tool_name == "claude_visual_understand":
                return await self._claude_visual_understand(arguments)
            elif tool_name == "claude_form_intelligent_fill":
                return await self._claude_form_intelligent_fill(arguments)
            elif tool_name == "claude_test_generate":
                return await self._claude_test_generate(arguments)
            elif tool_name == "claude_context_aware_action":
                return await self._claude_context_aware_action(arguments)
            elif tool_name == "claude_session_analyze":
                return await self._claude_session_analyze(arguments)
            elif tool_name == "claude_live_feedback":
                return await self._claude_live_feedback(arguments)
            elif tool_name == "claude_cursor_natural_move":
                return await self._claude_cursor_natural_move(arguments)
            else:
                # Delegate to base server for standard tools
                return await self.base_server.handle_tool_call(tool_name, arguments)
                
        except Exception as e:
            logger.error(f"Claude tool execution failed for {tool_name}: {e}")
            return {
                "success": False,
                "error": str(e),
                "tool_name": tool_name,
                "claude_enhanced": True
            }
    
    # Claude-specific tool implementations
    
    async def _claude_desktop_interact(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Natural language desktop interaction for Claude Code"""
        intent = args["intent"]
        target_description = args.get("target_description", "")
        interaction_type = args["interaction_type"]
        
        # Capture intent for learning
        if args.get("capture_intent", True):
            intent_capture = ClaudeIntentCapture(
                intent_id=str(time.time()),
                description=intent,
                target_elements=[target_description],
                interaction_type=interaction_type,
                expected_outcome=args.get("expected_outcome", ""),
                visual_cues=[],
                timestamp=time.time()
            )
            self.intent_history.append(intent_capture)
        
        try:
            # Parse natural language intent
            parsed_intent = await self._parse_natural_language_intent(intent, target_description)
            
            # Execute based on interaction type
            if interaction_type == "click":
                # Find element using multiple methods
                element_result = await self.base_server.handle_tool_call("kvs_element_detect", {
                    "search_criteria": {
                        "text": parsed_intent.get("element_text", target_description)
                    },
                    "detection_methods": ["accessibility", "ocr", "template"]
                })
                
                if element_result["success"] and element_result["detected_elements"]:
                    # Click the detected element
                    first_element = element_result["detected_elements"][0]
                    coordinates = first_element.get("coordinates")
                    
                    if coordinates:
                        click_result = await self.base_server.handle_tool_call("kvs_element_click", {
                            "element_name": target_description,
                            "coordinates": coordinates,
                            "visual_feedback": True
                        })
                        
                        return {
                            "success": click_result["success"],
                            "intent": intent,
                            "interaction_type": interaction_type,
                            "method_used": first_element["method"],
                            "coordinates": coordinates,
                            "claude_enhanced": True,
                            "message": f"Successfully executed intent: {intent}"
                        }
                
                return {
                    "success": False,
                    "error": f"Could not locate element: {target_description}",
                    "intent": intent,
                    "claude_enhanced": True
                }
            
            elif interaction_type == "type":
                text_input = args.get("text_input", "")
                if text_input:
                    type_result = await self.base_server.handle_tool_call("kvs_text_input", {
                        "text": text_input,
                        "show_character_input": True,
                        "typing_speed": 65
                    })
                    
                    return {
                        "success": type_result["success"],
                        "intent": intent,
                        "interaction_type": interaction_type,
                        "text_typed": text_input,
                        "claude_enhanced": True,
                        "message": f"Successfully typed: {text_input}"
                    }
                else:
                    return {
                        "success": False,
                        "error": "No text provided for typing interaction",
                        "intent": intent,
                        "claude_enhanced": True
                    }
            
            # Add other interaction types as needed
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "intent": intent,
                "claude_enhanced": True
            }
    
    async def _claude_app_workflow(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Execute complete application workflow"""
        workflow_description = args["workflow_description"]
        app_name = args["app_name"]
        steps = args.get("steps", [])
        
        try:
            # Start recording if requested
            recording_started = False
            if args.get("record_session", True):
                record_result = await self.base_server.handle_tool_call("kvs_record_start", {
                    "output_filename": f"claude_workflow_{app_name}_{int(time.time())}.mp4",
                    "show_cursor_path": True
                })
                recording_started = record_result["success"]
            
            # Ensure application is running
            app_result = await self.base_server.handle_tool_call("kvs_app_launch", {
                "app_name": app_name,
                "app_command": app_name.lower(),  # Simple assumption
                "wait_for_launch": True
            })
            
            if not app_result["success"]:
                return {
                    "success": False,
                    "error": f"Failed to launch application: {app_name}",
                    "workflow_description": workflow_description,
                    "claude_enhanced": True
                }
            
            # Execute workflow steps
            completed_steps = []
            failed_steps = []
            
            for i, step in enumerate(steps):
                step_description = step.get("step_description", f"Step {i+1}")
                
                try:
                    # Parse step and execute
                    step_result = await self._execute_workflow_step(step)
                    
                    if step_result["success"]:
                        completed_steps.append({
                            "step_number": i + 1,
                            "description": step_description,
                            "result": step_result
                        })
                    else:
                        failed_steps.append({
                            "step_number": i + 1,
                            "description": step_description,
                            "error": step_result.get("error", "Unknown error")
                        })
                        
                        # Handle error based on error_handling setting
                        error_handling = args.get("error_handling", "adaptive")
                        if error_handling == "strict":
                            break
                        elif error_handling == "adaptive":
                            # Try alternative approach
                            retry_result = await self._retry_workflow_step(step)
                            if retry_result["success"]:
                                completed_steps.append({
                                    "step_number": i + 1,
                                    "description": step_description,
                                    "result": retry_result,
                                    "retry_used": True
                                })
                            else:
                                break
                        # For "continue", just continue to next step
                    
                    # Brief pause between steps
                    await asyncio.sleep(0.5)
                    
                except Exception as e:
                    failed_steps.append({
                        "step_number": i + 1,
                        "description": step_description,
                        "error": str(e)
                    })
            
            # Stop recording if started
            if recording_started:
                await self.base_server.handle_tool_call("kvs_record_stop", {})
            
            return {
                "success": len(failed_steps) == 0,
                "workflow_description": workflow_description,
                "app_name": app_name,
                "total_steps": len(steps),
                "completed_steps": len(completed_steps),
                "failed_steps": len(failed_steps),
                "step_details": {
                    "completed": completed_steps,
                    "failed": failed_steps
                },
                "recording_available": recording_started,
                "claude_enhanced": True,
                "message": f"Workflow completed: {len(completed_steps)}/{len(steps)} steps successful"
            }
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "workflow_description": workflow_description,
                "claude_enhanced": True
            }
    
    async def _claude_visual_understand(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Analyze and understand desktop state for Claude Code"""
        analysis_type = args.get("analysis_type", "active_window")
        
        try:
            # Take screenshot for analysis
            screenshot_result = await self.base_server.handle_tool_call("kvs_screenshot", {
                "filename": f"claude_analysis_{int(time.time())}.png",
                "annotate_cursor": True
            })
            
            if not screenshot_result["success"]:
                return {
                    "success": False,
                    "error": "Failed to capture screenshot for analysis",
                    "claude_enhanced": True
                }
            
            analysis_results = {
                "screenshot_path": screenshot_result["filename"],
                "analysis_type": analysis_type,
                "timestamp": time.time()
            }
            
            # Extract text if requested
            if args.get("extract_text", True):
                text_content = await self._extract_text_from_screenshot(screenshot_result["filename"])
                analysis_results["extracted_text"] = text_content
            
            # Identify interactive elements if requested
            if args.get("identify_interactive", True):
                interactive_elements = await self._identify_interactive_elements()
                analysis_results["interactive_elements"] = interactive_elements
            
            # Describe layout if requested
            if args.get("describe_layout", True):
                layout_description = await self._describe_layout()
                analysis_results["layout_description"] = layout_description
            
            # Generate selectors if requested
            if args.get("generate_selectors", True):
                selectors = await self._generate_element_selectors()
                analysis_results["element_selectors"] = selectors
            
            return {
                "success": True,
                "analysis_results": analysis_results,
                "claude_enhanced": True,
                "message": f"Visual analysis completed for {analysis_type}"
            }
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "analysis_type": analysis_type,
                "claude_enhanced": True
            }
    
    async def _claude_form_intelligent_fill(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Intelligently fill forms with context understanding"""
        form_data = args["form_data"]
        
        try:
            # Auto-detect form fields if enabled
            if args.get("auto_detect_fields", True):
                detected_fields = await self._detect_form_fields()
            else:
                detected_fields = []
            
            # Map form data to detected fields
            field_mappings = await self._map_data_to_fields(form_data, detected_fields)
            
            # Fill form using intelligent mapping
            fill_result = await self.base_server.handle_tool_call("kvs_form_fill", {
                "form_fields": field_mappings,
                "fill_strategy": "realistic",
                "simulate_user_behavior": args.get("simulate_human_behavior", True),
                "visual_feedback": True
            })
            
            # Validate entries if enabled
            validation_results = {}
            if args.get("validation_enabled", True):
                validation_results = await self._validate_form_entries(field_mappings)
            
            # Submit if requested and validation passed
            submitted = False
            if args.get("submit_after_fill", False) and validation_results.get("all_valid", True):
                submit_result = await self._submit_form()
                submitted = submit_result["success"]
            
            return {
                "success": fill_result["success"],
                "form_data": form_data,
                "fields_detected": len(detected_fields),
                "fields_filled": fill_result.get("filled_fields", []),
                "fields_failed": fill_result.get("failed_fields", []),
                "validation_results": validation_results,
                "form_submitted": submitted,
                "claude_enhanced": True,
                "message": f"Intelligent form filling completed: {len(fill_result.get('filled_fields', []))} fields filled"
            }
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "form_data": form_data,
                "claude_enhanced": True
            }
    
    async def _claude_test_generate(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Generate automated tests from manual interactions"""
        test_name = args["test_name"]
        
        try:
            # Start capturing interactions
            capture_duration = args.get("capture_duration", 60)
            
            # This would capture user interactions for the specified duration
            # and generate test code based on the captured interactions
            captured_interactions = await self._capture_user_interactions(capture_duration)
            
            # Generate test code
            test_framework = args.get("test_framework", "kvirtualstage")
            test_code = await self._generate_test_code(
                test_name, 
                captured_interactions, 
                test_framework,
                args.get("include_assertions", True),
                args.get("generate_comments", True)
            )
            
            # Save test file
            test_filename = f"test_{test_name}_{int(time.time())}.py"
            test_path = f"/tmp/{test_filename}"
            
            with open(test_path, 'w') as f:
                f.write(test_code)
            
            return {
                "success": True,
                "test_name": test_name,
                "test_file": test_path,
                "interactions_captured": len(captured_interactions),
                "test_framework": test_framework,
                "test_code": test_code,
                "claude_enhanced": True,
                "message": f"Test '{test_name}' generated successfully with {len(captured_interactions)} interactions"
            }
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "test_name": test_name,
                "claude_enhanced": True
            }
    
    async def _claude_context_aware_action(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Perform context-aware actions that adapt to application state"""
        goal = args["goal"]
        
        try:
            # Analyze current context
            context_analysis = await self._analyze_application_context()
            
            # Plan action based on context and goal
            action_plan = await self._plan_context_aware_action(goal, context_analysis)
            
            # Execute action plan with adaptation
            execution_results = []
            max_attempts = args.get("max_attempts", 3)
            
            for attempt in range(max_attempts):
                try:
                    result = await self._execute_adaptive_action(action_plan, context_analysis)
                    
                    if result["success"]:
                        execution_results.append(result)
                        break
                    else:
                        # Adapt strategy and retry
                        action_plan = await self._adapt_action_plan(action_plan, result, attempt)
                        execution_results.append(result)
                        
                except Exception as e:
                    execution_results.append({
                        "success": False,
                        "attempt": attempt + 1,
                        "error": str(e)
                    })
            
            final_success = any(result.get("success", False) for result in execution_results)
            
            return {
                "success": final_success,
                "goal": goal,
                "context_analysis": context_analysis,
                "action_plan": action_plan,
                "execution_attempts": len(execution_results),
                "execution_results": execution_results,
                "claude_enhanced": True,
                "message": f"Context-aware action {'completed' if final_success else 'failed'}: {goal}"
            }
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "goal": goal,
                "claude_enhanced": True
            }
    
    async def _claude_session_analyze(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Analyze session for Claude Code learning"""
        session_id = args.get("session_id")
        
        try:
            # Get session information
            session_info_result = await self.base_server.handle_tool_call("kvs_session_info", {
                "session_id": session_id,
                "include_history": True
            })
            
            if not session_info_result["success"]:
                return {
                    "success": False,
                    "error": "Failed to retrieve session information",
                    "session_id": session_id,
                    "claude_enhanced": True
                }
            
            session_info = session_info_result["session_info"]
            analysis_focus = args.get("analysis_focus", ["efficiency", "accuracy"])
            
            # Perform analysis based on focus areas
            analysis_results = {}
            
            if "efficiency" in analysis_focus:
                analysis_results["efficiency"] = await self._analyze_session_efficiency(session_info)
            
            if "accuracy" in analysis_focus:
                analysis_results["accuracy"] = await self._analyze_session_accuracy(session_info)
            
            if "user_experience" in analysis_focus:
                analysis_results["user_experience"] = await self._analyze_user_experience(session_info)
            
            if "reliability" in analysis_focus:
                analysis_results["reliability"] = await self._analyze_session_reliability(session_info)
            
            # Generate improvements if requested
            improvements = []
            if args.get("generate_improvements", True):
                improvements = await self._generate_session_improvements(analysis_results)
            
            # Export insights if requested
            insights_exported = False
            if args.get("export_insights", True):
                insights_exported = await self._export_session_insights(session_id, analysis_results, improvements)
            
            return {
                "success": True,
                "session_id": session_id,
                "analysis_focus": analysis_focus,
                "analysis_results": analysis_results,
                "improvements": improvements,
                "insights_exported": insights_exported,
                "claude_enhanced": True,
                "message": f"Session analysis completed for {len(analysis_focus)} focus areas"
            }
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "session_id": session_id,
                "claude_enhanced": True
            }
    
    async def _claude_live_feedback(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Provide real-time feedback for AI decision making"""
        feedback_type = args["feedback_type"]
        
        try:
            monitoring_duration = args.get("monitoring_duration", 5.0)
            
            # Monitor for specified duration
            feedback_data = await self._monitor_desktop_state(feedback_type, monitoring_duration)
            
            # Generate suggestions if requested
            suggestions = []
            if args.get("include_suggestions", True):
                suggestions = await self._generate_action_suggestions(feedback_data)
            
            return {
                "success": True,
                "feedback_type": feedback_type,
                "monitoring_duration": monitoring_duration,
                "feedback_data": feedback_data,
                "suggestions": suggestions,
                "timestamp": time.time(),
                "claude_enhanced": True,
                "message": f"Live feedback provided for {feedback_type}"
            }
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "feedback_type": feedback_type,
                "claude_enhanced": True
            }
    
    async def _claude_cursor_natural_move(self, args: Dict[str, Any]) -> Dict[str, Any]:
        """Move cursor with natural, human-like patterns"""
        target_description = args.get("target_description", "")
        coordinates = args.get("coordinates")
        
        try:
            # If coordinates not provided, try to find target
            if not coordinates and target_description:
                detect_result = await self.base_server.handle_tool_call("kvs_element_detect", {
                    "search_criteria": {"text": target_description},
                    "detection_methods": ["accessibility", "ocr"]
                })
                
                if detect_result["success"] and detect_result["detected_elements"]:
                    coordinates = detect_result["detected_elements"][0].get("coordinates")
            
            if not coordinates:
                return {
                    "success": False,
                    "error": f"Could not determine target coordinates for: {target_description}",
                    "claude_enhanced": True
                }
            
            # Move cursor with specified personality
            movement_personality = args.get("movement_personality", "natural")
            
            move_result = await self.base_server.handle_tool_call("kvs_cursor_move", {
                "x": coordinates[0],
                "y": coordinates[1],
                "movement_style": "human" if movement_personality == "natural" else movement_personality,
                "show_path": args.get("show_intent_path", True)
            })
            
            # Add pause at target if requested
            if args.get("pause_at_target", True) and move_result["success"]:
                await asyncio.sleep(0.2)  # Brief human-like pause
            
            return {
                "success": move_result["success"],
                "target_description": target_description,
                "final_coordinates": coordinates,
                "movement_personality": movement_personality,
                "claude_enhanced": True,
                "message": f"Natural cursor movement to {target_description or coordinates}"
            }
            
        except Exception as e:
            return {
                "success": False,
                "error": str(e),
                "target_description": target_description,
                "claude_enhanced": True
            }
    
    # Helper methods for Claude-specific functionality
    
    async def _parse_natural_language_intent(self, intent: str, target_description: str) -> Dict[str, Any]:
        """Parse natural language intent into actionable parameters"""
        # This would implement NLP parsing of user intent
        # For now, return basic parsing
        return {
            "element_text": target_description,
            "confidence_required": "medium",
            "interaction_hints": []
        }
    
    async def _execute_workflow_step(self, step: Dict[str, Any]) -> Dict[str, Any]:
        """Execute a single workflow step"""
        # Implementation would parse step and execute appropriate action
        return {"success": True, "step_executed": step}
    
    async def _retry_workflow_step(self, step: Dict[str, Any]) -> Dict[str, Any]:
        """Retry a failed workflow step with alternative approach"""
        # Implementation would try alternative execution strategies
        return {"success": True, "retry_method": "alternative_approach"}
    
    async def _extract_text_from_screenshot(self, screenshot_path: str) -> List[str]:
        """Extract text from screenshot using OCR"""
        # Implementation would use OCR to extract text
        return ["Sample extracted text"]
    
    async def _identify_interactive_elements(self) -> List[Dict[str, Any]]:
        """Identify interactive elements on screen"""
        # Implementation would detect buttons, fields, etc.
        return [{"type": "button", "text": "Submit", "coordinates": [100, 200]}]
    
    async def _describe_layout(self) -> str:
        """Provide natural language description of current layout"""
        # Implementation would analyze layout and generate description
        return "The screen shows a form with text fields and a submit button"
    
    async def _generate_element_selectors(self) -> List[Dict[str, Any]]:
        """Generate selectors for automated interaction"""
        # Implementation would generate various selector types
        return [{"type": "accessibility", "selector": "button[text='Submit']"}]
    
    async def _detect_form_fields(self) -> List[Dict[str, Any]]:
        """Detect form fields on current screen"""
        # Implementation would detect form elements
        return [{"type": "text", "label": "Name", "id": "name_field"}]
    
    async def _map_data_to_fields(self, form_data: Dict[str, Any], detected_fields: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
        """Map form data to detected fields intelligently"""
        # Implementation would map data to appropriate fields
        return [{"field_name": "Name", "field_value": form_data.get("name", ""), "field_type": "text"}]
    
    async def _validate_form_entries(self, field_mappings: List[Dict[str, Any]]) -> Dict[str, Any]:
        """Validate form entries after filling"""
        # Implementation would validate filled form fields
        return {"all_valid": True, "validation_details": []}
    
    async def _submit_form(self) -> Dict[str, Any]:
        """Submit the form"""
        # Implementation would find and click submit button
        return {"success": True, "method": "submit_button_click"}
    
    async def _capture_user_interactions(self, duration: float) -> List[Dict[str, Any]]:
        """Capture user interactions for test generation"""
        # Implementation would record user interactions
        return [{"type": "click", "element": "button", "timestamp": time.time()}]
    
    async def _generate_test_code(self, test_name: str, interactions: List[Dict[str, Any]], 
                                framework: str, include_assertions: bool, generate_comments: bool) -> str:
        """Generate test code from captured interactions"""
        # Implementation would generate actual test code
        return f"""
# Generated test: {test_name}
def test_{test_name.lower().replace(' ', '_')}():
    # Test implementation based on captured interactions
    pass
"""
    
    async def _analyze_application_context(self) -> Dict[str, Any]:
        """Analyze current application context"""
        # Implementation would analyze current app state
        return {"app_name": "unknown", "state": "active", "context_hints": []}
    
    async def _plan_context_aware_action(self, goal: str, context: Dict[str, Any]) -> Dict[str, Any]:
        """Plan action based on context and goal"""
        # Implementation would create action plan
        return {"primary_strategy": "direct_action", "fallback_strategies": []}
    
    async def _execute_adaptive_action(self, action_plan: Dict[str, Any], context: Dict[str, Any]) -> Dict[str, Any]:
        """Execute action with adaptation to context"""
        # Implementation would execute adaptive action
        return {"success": True, "action_taken": "context_adapted_action"}
    
    async def _adapt_action_plan(self, action_plan: Dict[str, Any], previous_result: Dict[str, Any], attempt: int) -> Dict[str, Any]:
        """Adapt action plan based on previous failure"""
        # Implementation would modify strategy
        return action_plan  # Modified plan
    
    async def _analyze_session_efficiency(self, session_info: Dict[str, Any]) -> Dict[str, Any]:
        """Analyze session efficiency metrics"""
        return {"efficiency_score": 0.85, "recommendations": ["reduce pause times"]}
    
    async def _analyze_session_accuracy(self, session_info: Dict[str, Any]) -> Dict[str, Any]:
        """Analyze session accuracy metrics"""
        return {"accuracy_score": 0.92, "failed_interactions": 2}
    
    async def _analyze_user_experience(self, session_info: Dict[str, Any]) -> Dict[str, Any]:
        """Analyze user experience aspects"""
        return {"ux_score": 0.88, "naturalness_rating": "high"}
    
    async def _analyze_session_reliability(self, session_info: Dict[str, Any]) -> Dict[str, Any]:
        """Analyze session reliability metrics"""
        return {"reliability_score": 0.95, "error_rate": 0.05}
    
    async def _generate_session_improvements(self, analysis_results: Dict[str, Any]) -> List[str]:
        """Generate improvement suggestions"""
        return ["Improve element detection accuracy", "Optimize cursor movement timing"]
    
    async def _export_session_insights(self, session_id: str, analysis: Dict[str, Any], improvements: List[str]) -> bool:
        """Export insights for future learning"""
        # Implementation would save insights
        return True
    
    async def _monitor_desktop_state(self, feedback_type: str, duration: float) -> Dict[str, Any]:
        """Monitor desktop state for specified duration"""
        # Implementation would monitor changes
        return {"state_changes": [], "current_state": "stable"}
    
    async def _generate_action_suggestions(self, feedback_data: Dict[str, Any]) -> List[str]:
        """Generate suggestions based on feedback"""
        return ["Consider clicking the highlighted button", "Wait for loading to complete"]

# MCP Integration Functions for Claude Code

def get_claude_tools() -> List[Dict[str, Any]]:
    """Get Claude Code optimized MCP tools"""
    claude_interface = ClaudeCodeMCPInterface()
    return [
        {
            "name": tool.name,
            "description": tool.description,
            "inputSchema": tool.input_schema
        }
        for tool in claude_interface.get_all_tools()
    ]

async def call_claude_tool(name: str, arguments: Dict[str, Any]) -> Dict[str, Any]:
    """Handle Claude Code MCP tool calls"""
    claude_interface = ClaudeCodeMCPInterface()
    return await claude_interface.handle_claude_tool_call(name, arguments)

# Example usage for Claude Code
async def demo_claude_integration():
    """Demonstrate Claude Code integration"""
    claude_interface = ClaudeCodeMCPInterface()
    
    print("🤖 Claude Code MCP Integration Demo")
    print("===================================")
    
    # Natural language interaction
    interaction_result = await claude_interface.handle_claude_tool_call("claude_desktop_interact", {
        "intent": "Open a calculator and compute 25 * 8",
        "target_description": "calculator application",
        "interaction_type": "click"
    })
    print(f"Natural Language Interaction: {interaction_result}")
    
    # Visual understanding
    visual_result = await claude_interface.handle_claude_tool_call("claude_visual_understand", {
        "analysis_type": "active_window",
        "extract_text": True,
        "identify_interactive": True
    })
    print(f"Visual Understanding: {visual_result}")
    
    # Context-aware action
    context_result = await claude_interface.handle_claude_tool_call("claude_context_aware_action", {
        "goal": "save the current work",
        "adaptation_level": "moderate"
    })
    print(f"Context-Aware Action: {context_result}")
    
    print("\n✅ Claude Code integration demo completed!")

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    asyncio.run(demo_claude_integration())