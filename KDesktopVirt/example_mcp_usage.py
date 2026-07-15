#!/usr/bin/env python3
"""
Example Usage of KVirtualStage MCP Server

This file demonstrates comprehensive usage of the KVirtualStage MCP server
for desktop automation with AI agents like Claude Code.

Run these examples to understand the capabilities and see the server in action.
"""

import asyncio
import json
import logging
import time
from typing import Dict, Any

# Import MCP server components
from kvirtualstage_mcp_server import KVirtualStageMCPServer
from mcp_tools_claude_integration import ClaudeCodeMCPInterface
from mcp_protocol_handler import MCPProtocolHandler

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class MCPExampleRunner:
    """
    Demonstrates various MCP server capabilities through practical examples
    """
    
    def __init__(self):
        self.server = KVirtualStageMCPServer()
        self.claude_interface = ClaudeCodeMCPInterface()
        self.session_id = None
        
    async def run_all_examples(self):
        """Run all example demonstrations"""
        print("🚀 KVirtualStage MCP Server - Comprehensive Examples")
        print("=" * 60)
        
        try:
            # Basic examples
            await self.example_01_session_management()
            await self.example_02_application_launching()
            await self.example_03_element_interaction()
            await self.example_04_text_input()
            await self.example_05_cursor_movement()
            
            # Advanced examples
            await self.example_06_form_filling()
            await self.example_07_menu_navigation()
            await self.example_08_screenshot_recording()
            
            # Claude Code specific examples
            await self.example_09_natural_language_interaction()
            await self.example_10_workflow_automation()
            await self.example_11_visual_understanding()
            await self.example_12_intelligent_form_filling()
            
            # Real-world scenarios
            await self.example_13_calculator_workflow()
            await self.example_14_document_creation()
            await self.example_15_system_monitoring()
            
            print("\n✅ All examples completed successfully!")
            
        except Exception as e:
            logger.error(f"Examples failed: {e}")
            raise
    
    async def example_01_session_management(self):
        """Example 1: Session Management"""
        print("\n📋 Example 1: Session Management")
        print("-" * 40)
        
        # Create a new automation session
        result = await self.server.handle_tool_call("kvs_session_create", {
            "user_id": "example_user",
            "session_name": "Demo Session",
            "enable_recording": True,
            "enable_cursor_path": True,
            "enable_visual_feedback": True
        })
        
        if result["success"]:
            self.session_id = result["session_id"]
            print(f"✅ Session created: {self.session_id}")
            print(f"   Recording enabled: {result['session_info']['recording_active']}")
            print(f"   Visual feedback: {result['session_info']['visual_feedback_enabled']}")
        else:
            print(f"❌ Session creation failed: {result.get('error', 'Unknown error')}")
        
        # List active sessions
        list_result = await self.server.handle_tool_call("kvs_session_list", {
            "include_details": True
        })
        
        if list_result["success"]:
            print(f"📊 Active sessions: {list_result['session_count']}")
            for session in list_result["sessions"]:
                print(f"   - {session['session_id']}: {session['status']}")
        
        # Get session info
        if self.session_id:
            info_result = await self.server.handle_tool_call("kvs_session_info", {
                "session_id": self.session_id,
                "include_history": False
            })
            
            if info_result["success"]:
                session_info = info_result["session_info"]
                print(f"📄 Session details:")
                print(f"   Created: {time.ctime(session_info['created_at'])}")
                print(f"   Status: {session_info['status']}")
    
    async def example_02_application_launching(self):
        """Example 2: Application Launching"""
        print("\n🚀 Example 2: Application Launching")
        print("-" * 40)
        
        # Launch calculator application
        result = await self.server.handle_tool_call("kvs_app_launch", {
            "session_id": self.session_id,
            "app_name": "Calculator",
            "app_command": "galculator",
            "wait_for_launch": True,
            "launch_timeout": 15,
            "focus_after_launch": True
        })
        
        if result["success"]:
            print(f"✅ Calculator launched successfully")
            print(f"   Process ID: {result.get('process_id', 'N/A')}")
            print(f"   Launch method: Direct command execution")
        else:
            print(f"❌ Calculator launch failed: {result.get('error', 'Unknown error')}")
        
        await asyncio.sleep(2)  # Allow application to fully load
    
    async def example_03_element_interaction(self):
        """Example 3: Element Interaction"""
        print("\n🖱️ Example 3: Element Interaction")
        print("-" * 40)
        
        # Click calculator buttons using different detection methods
        buttons_to_click = [
            {"name": "8", "type": "button"},
            {"name": "×", "type": "operator"},
            {"name": "7", "type": "button"},
            {"name": "=", "type": "operator"}
        ]
        
        for button in buttons_to_click:
            result = await self.server.handle_tool_call("kvs_element_click", {
                "session_id": self.session_id,
                "element_name": button["name"],
                "element_type": button["type"],
                "detection_methods": ["accessibility", "ocr", "template"],
                "confidence_threshold": 0.7,
                "click_type": "left",
                "visual_feedback": True
            })
            
            if result["success"]:
                print(f"✅ Clicked '{button['name']}' using {result.get('method_used', 'unknown')} detection")
                if result.get("coordinates"):
                    print(f"   Coordinates: {result['coordinates']}")
            else:
                print(f"❌ Failed to click '{button['name']}': {result.get('error', 'Unknown error')}")
            
            await asyncio.sleep(0.5)  # Natural delay between clicks
        
        # Verify calculation result
        await asyncio.sleep(1)
        screenshot_result = await self.server.handle_tool_call("kvs_screenshot", {
            "session_id": self.session_id,
            "filename": "calculator_result.png",
            "annotate_cursor": True
        })
        
        if screenshot_result["success"]:
            print(f"📸 Screenshot saved: {screenshot_result['filename']}")
    
    async def example_04_text_input(self):
        """Example 4: Text Input"""
        print("\n⌨️ Example 4: Text Input")
        print("-" * 40)
        
        # Launch text editor
        launch_result = await self.server.handle_tool_call("kvs_app_launch", {
            "session_id": self.session_id,
            "app_name": "Text Editor",
            "app_command": "mousepad",
            "wait_for_launch": True
        })
        
        if launch_result["success"]:
            await asyncio.sleep(2)  # Allow editor to load
            
            # Type demonstration text
            text_content = """🤖 KVirtualStage MCP Server Demo

This text was typed using human-like simulation:
• Natural typing rhythm with variations
• Character-by-character visual feedback
• Realistic pause patterns
• Error correction simulation

Current timestamp: """ + time.strftime("%Y-%m-%d %H:%M:%S")
            
            result = await self.server.handle_tool_call("kvs_text_input", {
                "session_id": self.session_id,
                "text": text_content,
                "typing_speed": 65,  # WPM
                "char_delay_variation": 0.3,
                "clear_field_first": False,
                "send_enter": True,
                "show_character_input": True
            })
            
            if result["success"]:
                print(f"✅ Text input completed")
                print(f"   Characters typed: {result['text_length']}")
                print(f"   Typing speed: {result.get('typing_speed', 'default')} WPM")
            else:
                print(f"❌ Text input failed: {result.get('error', 'Unknown error')}")
        else:
            print(f"❌ Text editor launch failed: {launch_result.get('error', 'Unknown error')}")
    
    async def example_05_cursor_movement(self):
        """Example 5: Cursor Movement"""
        print("\n🖱️ Example 5: Cursor Movement")
        print("-" * 40)
        
        # Demonstrate different cursor movement styles
        movement_patterns = [
            {"style": "smooth", "description": "Linear interpolation"},
            {"style": "curved", "description": "Natural arc movement"},
            {"style": "human", "description": "Human-like with micro-movements"}
        ]
        
        start_positions = [(100, 100), (300, 200), (500, 150)]
        end_positions = [(800, 400), (200, 500), (600, 300)]
        
        for i, pattern in enumerate(movement_patterns):
            start_x, start_y = start_positions[i]
            end_x, end_y = end_positions[i]
            
            result = await self.server.handle_tool_call("kvs_cursor_move", {
                "session_id": self.session_id,
                "x": end_x,
                "y": end_y,
                "movement_style": pattern["style"],
                "movement_speed": 1.0,
                "show_path": True
            })
            
            if result["success"]:
                print(f"✅ {pattern['style'].title()} movement: ({start_x},{start_y}) → ({end_x},{end_y})")
                print(f"   Description: {pattern['description']}")
                print(f"   Path shown: {result.get('show_path', False)}")
            else:
                print(f"❌ Cursor movement failed: {result.get('error', 'Unknown error')}")
            
            await asyncio.sleep(1)  # Allow movement to complete
    
    async def example_06_form_filling(self):
        """Example 6: Form Filling"""
        print("\n📝 Example 6: Form Filling")
        print("-" * 40)
        
        # Simulate form filling (this would work with actual form applications)
        form_data = [
            {"field_name": "First Name", "field_value": "John", "field_type": "text"},
            {"field_name": "Last Name", "field_value": "Doe", "field_type": "text"},
            {"field_name": "Email", "field_value": "john.doe@example.com", "field_type": "email"},
            {"field_name": "Phone", "field_value": "555-0123", "field_type": "tel"},
            {"field_name": "Company", "field_value": "Example Corp", "field_type": "text"}
        ]
        
        result = await self.server.handle_tool_call("kvs_form_fill", {
            "session_id": self.session_id,
            "form_fields": form_data,
            "fill_strategy": "realistic",
            "simulate_user_behavior": True,
            "visual_feedback": True
        })
        
        if result["success"]:
            print(f"✅ Form filling completed")
            print(f"   Fields filled: {len(result.get('filled_fields', []))}")
            print(f"   Fields failed: {len(result.get('failed_fields', []))}")
            print(f"   Strategy used: {result.get('fill_strategy', 'default')}")
        else:
            print(f"❌ Form filling failed: {result.get('error', 'Unknown error')}")
    
    async def example_07_menu_navigation(self):
        """Example 7: Menu Navigation"""
        print("\n🧭 Example 7: Menu Navigation")
        print("-" * 40)
        
        # Simulate menu navigation (would work with actual menu-driven applications)
        menu_paths = [
            ["File", "New", "Document"],
            ["Edit", "Preferences", "General"],
            ["View", "Zoom", "Fit to Page"],
            ["Help", "About"]
        ]
        
        for menu_path in menu_paths:
            result = await self.server.handle_tool_call("kvs_menu_navigate", {
                "session_id": self.session_id,
                "menu_path": menu_path,
                "navigation_method": "mixed",
                "hover_delay": 0.5,
                "show_hover_path": True
            })
            
            if result["success"]:
                print(f"✅ Navigated menu: {' > '.join(menu_path)}")
                print(f"   Method: {result.get('navigation_method', 'default')}")
            else:
                print(f"❌ Menu navigation failed for {' > '.join(menu_path)}: {result.get('error', 'Unknown error')}")
            
            await asyncio.sleep(1)
    
    async def example_08_screenshot_recording(self):
        """Example 8: Screenshot and Recording"""
        print("\n📸 Example 8: Screenshot and Recording")
        print("-" * 40)
        
        # Take various screenshots
        screenshot_types = [
            {"name": "current_desktop", "description": "Full desktop capture"},
            {"name": "active_window", "description": "Active window only"},
            {"name": "annotated", "description": "With cursor and highlights"}
        ]
        
        for screenshot in screenshot_types:
            result = await self.server.handle_tool_call("kvs_screenshot", {
                "session_id": self.session_id,
                "filename": f"{screenshot['name']}.png",
                "annotate_cursor": True,
                "highlight_elements": ["active_button", "text_field"]
            })
            
            if result["success"]:
                print(f"✅ Screenshot '{screenshot['name']}': {result['filename']}")
                print(f"   Description: {screenshot['description']}")
            else:
                print(f"❌ Screenshot failed: {result.get('error', 'Unknown error')}")
        
        # Start recording
        record_start = await self.server.handle_tool_call("kvs_record_start", {
            "session_id": self.session_id,
            "output_filename": "demo_recording.mp4",
            "quality": "high",
            "include_audio": False,
            "fps": 30,
            "show_cursor_path": True
        })
        
        if record_start["success"]:
            print(f"🎥 Recording started: {record_start.get('output_filename', 'N/A')}")
            
            # Perform some actions while recording
            await asyncio.sleep(3)
            
            # Stop recording
            record_stop = await self.server.handle_tool_call("kvs_record_stop", {
                "session_id": self.session_id
            })
            
            if record_stop["success"]:
                print(f"🎥 Recording stopped: {record_stop.get('recording_path', 'N/A')}")
            else:
                print(f"❌ Recording stop failed: {record_stop.get('error', 'Unknown error')}")
        else:
            print(f"❌ Recording start failed: {record_start.get('error', 'Unknown error')}")
    
    async def example_09_natural_language_interaction(self):
        """Example 9: Natural Language Interaction (Claude Code)"""
        print("\n🗣️ Example 9: Natural Language Interaction")
        print("-" * 40)
        
        # Natural language interactions
        interactions = [
            {
                "intent": "Click the blue submit button in the bottom right corner",
                "target": "submit button",
                "type": "click"
            },
            {
                "intent": "Type my name in the first text field",
                "target": "name field",
                "type": "type",
                "text": "Demo User"
            },
            {
                "intent": "Close the current window",
                "target": "close button",
                "type": "click"
            }
        ]
        
        for interaction in interactions:
            params = {
                "intent": interaction["intent"],
                "target_description": interaction["target"],
                "interaction_type": interaction["type"],
                "confidence_level": "medium",
                "capture_intent": True
            }
            
            if interaction["type"] == "type":
                params["text_input"] = interaction.get("text", "")
            
            result = await self.claude_interface.handle_claude_tool_call("claude_desktop_interact", params)
            
            if result["success"]:
                print(f"✅ Natural language: {interaction['intent']}")
                print(f"   Method used: {result.get('method_used', 'N/A')}")
                print(f"   Intent captured: {result.get('claude_enhanced', False)}")
            else:
                print(f"❌ Natural language failed: {result.get('error', 'Unknown error')}")
            
            await asyncio.sleep(1)
    
    async def example_10_workflow_automation(self):
        """Example 10: Workflow Automation (Claude Code)"""
        print("\n🔄 Example 10: Workflow Automation")
        print("-" * 40)
        
        # Define a complete workflow
        workflow_definition = {
            "workflow_description": "Create a simple document with formatted text and save it",
            "app_name": "Text Editor",
            "steps": [
                {
                    "step_description": "Launch text editor application",
                    "expected_outcome": "Text editor window opens successfully"
                },
                {
                    "step_description": "Type document title and content",
                    "expected_outcome": "Text appears in the editor"
                },
                {
                    "step_description": "Format text with basic styling",
                    "expected_outcome": "Text formatting is applied"
                },
                {
                    "step_description": "Save document with specific filename",
                    "expected_outcome": "Document is saved successfully"
                }
            ],
            "error_handling": "adaptive",
            "record_session": True
        }
        
        result = await self.claude_interface.handle_claude_tool_call("claude_app_workflow", workflow_definition)
        
        if result["success"]:
            print(f"✅ Workflow automation completed")
            print(f"   Total steps: {result.get('total_steps', 0)}")
            print(f"   Completed steps: {result.get('completed_steps', 0)}")
            print(f"   Failed steps: {result.get('failed_steps', 0)}")
            print(f"   Recording available: {result.get('recording_available', False)}")
            
            # Show step details
            step_details = result.get('step_details', {})
            if step_details.get('completed'):
                print("   Completed steps:")
                for step in step_details['completed']:
                    print(f"     - Step {step['step_number']}: {step['description']}")
        else:
            print(f"❌ Workflow automation failed: {result.get('error', 'Unknown error')}")
    
    async def example_11_visual_understanding(self):
        """Example 11: Visual Understanding (Claude Code)"""
        print("\n👁️ Example 11: Visual Understanding")
        print("-" * 40)
        
        # Analyze current desktop state
        analysis_params = {
            "analysis_type": "active_window",
            "extract_text": True,
            "identify_interactive": True,
            "describe_layout": True,
            "generate_selectors": True
        }
        
        result = await self.claude_interface.handle_claude_tool_call("claude_visual_understand", analysis_params)
        
        if result["success"]:
            analysis = result.get("analysis_results", {})
            print(f"✅ Visual analysis completed")
            print(f"   Screenshot: {analysis.get('screenshot_path', 'N/A')}")
            print(f"   Analysis type: {analysis.get('analysis_type', 'N/A')}")
            
            # Show extracted text
            extracted_text = analysis.get("extracted_text", [])
            if extracted_text:
                print(f"   Extracted text ({len(extracted_text)} items):")
                for i, text in enumerate(extracted_text[:3]):  # Show first 3
                    print(f"     - {text}")
                if len(extracted_text) > 3:
                    print(f"     ... and {len(extracted_text) - 3} more")
            
            # Show interactive elements
            interactive = analysis.get("interactive_elements", [])
            if interactive:
                print(f"   Interactive elements ({len(interactive)} found):")
                for element in interactive[:3]:  # Show first 3
                    print(f"     - {element.get('type', 'unknown')}: {element.get('text', 'N/A')}")
            
            # Show layout description
            layout = analysis.get("layout_description", "")
            if layout:
                print(f"   Layout: {layout[:100]}{'...' if len(layout) > 100 else ''}")
        else:
            print(f"❌ Visual understanding failed: {result.get('error', 'Unknown error')}")
    
    async def example_12_intelligent_form_filling(self):
        """Example 12: Intelligent Form Filling (Claude Code)"""
        print("\n🤖 Example 12: Intelligent Form Filling")
        print("-" * 40)
        
        # Demonstrate intelligent form filling
        form_data = {
            "personal_info": {
                "first_name": "Alice",
                "last_name": "Johnson",
                "email": "alice.johnson@email.com",
                "phone": "555-0199",
                "date_of_birth": "1990-05-15"
            },
            "address": {
                "street": "123 Main Street",
                "city": "Anytown",
                "state": "CA",
                "zip_code": "12345",
                "country": "United States"
            },
            "preferences": {
                "newsletter": True,
                "notifications": False,
                "language": "English"
            }
        }
        
        # Flatten form data for the tool
        flattened_data = {}
        for category, fields in form_data.items():
            for key, value in fields.items():
                flattened_data[key] = str(value)
        
        result = await self.claude_interface.handle_claude_tool_call("claude_form_intelligent_fill", {
            "form_data": flattened_data,
            "auto_detect_fields": True,
            "field_mapping_hints": {
                "email": ["email", "e-mail", "email_address"],
                "phone": ["phone", "telephone", "mobile"],
                "first_name": ["first_name", "fname", "given_name"]
            },
            "validation_enabled": True,
            "submit_after_fill": False,
            "simulate_human_behavior": True
        })
        
        if result["success"]:
            print(f"✅ Intelligent form filling completed")
            print(f"   Fields detected: {result.get('fields_detected', 0)}")
            print(f"   Fields filled: {len(result.get('fields_filled', []))}")
            print(f"   Fields failed: {len(result.get('fields_failed', []))}")
            
            validation = result.get('validation_results', {})
            if validation:
                print(f"   Validation: {'✅ All valid' if validation.get('all_valid') else '❌ Some invalid'}")
        else:
            print(f"❌ Intelligent form filling failed: {result.get('error', 'Unknown error')}")
    
    async def example_13_calculator_workflow(self):
        """Example 13: Real-world Calculator Workflow"""
        print("\n🧮 Example 13: Real-world Calculator Workflow")
        print("-" * 40)
        
        # Complete calculator workflow
        calculations = [
            {"expression": "15 + 28", "expected": "43"},
            {"expression": "100 - 37", "expected": "63"},
            {"expression": "8 × 9", "expected": "72"},
            {"expression": "144 ÷ 12", "expected": "12"}
        ]
        
        for calc in calculations:
            print(f"Calculating: {calc['expression']}")
            
            # Parse expression and click buttons
            tokens = calc['expression'].replace('×', '*').replace('÷', '/').split()
            
            for token in tokens:
                if token.strip():
                    # Click the calculator button
                    click_result = await self.server.handle_tool_call("kvs_element_click", {
                        "session_id": self.session_id,
                        "element_name": token,
                        "element_type": "button",
                        "visual_feedback": True
                    })
                    
                    if click_result["success"]:
                        print(f"   ✅ Clicked '{token}'")
                    else:
                        print(f"   ❌ Failed to click '{token}'")
                    
                    await asyncio.sleep(0.3)  # Natural clicking rhythm
            
            # Take screenshot of result
            screenshot_result = await self.server.handle_tool_call("kvs_screenshot", {
                "session_id": self.session_id,
                "filename": f"calc_result_{calc['expression'].replace(' ', '_')}.png"
            })
            
            if screenshot_result["success"]:
                print(f"   📸 Result captured: {screenshot_result['filename']}")
            
            # Clear calculator for next calculation
            await self.server.handle_tool_call("kvs_element_click", {
                "session_id": self.session_id,
                "element_name": "C",
                "element_type": "button"
            })
            
            await asyncio.sleep(1)
    
    async def example_14_document_creation(self):
        """Example 14: Document Creation Workflow"""
        print("\n📄 Example 14: Document Creation Workflow")
        print("-" * 40)
        
        # Create a structured document
        document_content = {
            "title": "KVirtualStage MCP Server Demo Report",
            "sections": [
                {
                    "heading": "Executive Summary",
                    "content": "This document demonstrates the capabilities of the KVirtualStage MCP Server for automated desktop interaction."
                },
                {
                    "heading": "Test Results",
                    "content": "All automation tests completed successfully with high accuracy and natural user simulation."
                },
                {
                    "heading": "Performance Metrics",
                    "content": "Element detection: 95% accuracy, Cursor movement: Smooth 60fps, Text input: Natural rhythm"
                },
                {
                    "heading": "Conclusion",
                    "content": "KVirtualStage MCP Server provides sophisticated desktop automation capabilities for AI agents."
                }
            ]
        }
        
        # Type the document content
        full_content = f"{document_content['title']}\n\n"
        for section in document_content['sections']:
            full_content += f"{section['heading']}\n{'-' * len(section['heading'])}\n{section['content']}\n\n"
        
        result = await self.server.handle_tool_call("kvs_text_input", {
            "session_id": self.session_id,
            "text": full_content,
            "typing_speed": 80,
            "char_delay_variation": 0.2,
            "show_character_input": True
        })
        
        if result["success"]:
            print(f"✅ Document created successfully")
            print(f"   Total characters: {result['text_length']}")
            print(f"   Document sections: {len(document_content['sections'])}")
        else:
            print(f"❌ Document creation failed: {result.get('error', 'Unknown error')}")
    
    async def example_15_system_monitoring(self):
        """Example 15: System Monitoring and Analysis"""
        print("\n📊 Example 15: System Monitoring and Analysis")
        print("-" * 40)
        
        # Get session analysis
        if self.session_id:
            analysis_result = await self.claude_interface.handle_claude_tool_call("claude_session_analyze", {
                "session_id": self.session_id,
                "analysis_focus": ["efficiency", "accuracy", "user_experience"],
                "generate_improvements": True,
                "export_insights": True
            })
            
            if analysis_result["success"]:
                print(f"✅ Session analysis completed")
                print(f"   Session ID: {analysis_result.get('session_id', 'N/A')}")
                print(f"   Analysis areas: {len(analysis_result.get('analysis_focus', []))}")
                print(f"   Improvements generated: {len(analysis_result.get('improvements', []))}")
                print(f"   Insights exported: {analysis_result.get('insights_exported', False)}")
                
                # Show some analysis results
                analysis_results = analysis_result.get('analysis_results', {})
                for focus_area, data in analysis_results.items():
                    if isinstance(data, dict) and 'score' in str(data):
                        print(f"   {focus_area.title()}: {data}")
            else:
                print(f"❌ Session analysis failed: {analysis_result.get('error', 'Unknown error')}")
        
        # Get live feedback
        feedback_result = await self.claude_interface.handle_claude_tool_call("claude_live_feedback", {
            "feedback_type": "visual_state",
            "monitoring_duration": 2.0,
            "include_suggestions": True
        })
        
        if feedback_result["success"]:
            print(f"✅ Live feedback collected")
            print(f"   Feedback type: {feedback_result.get('feedback_type', 'N/A')}")
            print(f"   Monitoring duration: {feedback_result.get('monitoring_duration', 0)}s")
            
            suggestions = feedback_result.get('suggestions', [])
            if suggestions:
                print(f"   Suggestions ({len(suggestions)}):")
                for suggestion in suggestions[:3]:
                    print(f"     - {suggestion}")
        else:
            print(f"❌ Live feedback failed: {feedback_result.get('error', 'Unknown error')}")

async def run_basic_examples():
    """Run basic MCP server examples"""
    runner = MCPExampleRunner()
    
    # Run selected examples
    await runner.example_01_session_management()
    await runner.example_02_application_launching()
    await runner.example_03_element_interaction()
    await runner.example_08_screenshot_recording()

async def run_claude_examples():
    """Run Claude Code specific examples"""
    runner = MCPExampleRunner()
    
    # Create session first
    await runner.example_01_session_management()
    
    # Run Claude-specific examples
    await runner.example_09_natural_language_interaction()
    await runner.example_10_workflow_automation()
    await runner.example_11_visual_understanding()

async def run_workflow_examples():
    """Run real-world workflow examples"""
    runner = MCPExampleRunner()
    
    # Create session
    await runner.example_01_session_management()
    
    # Run workflows
    await runner.example_13_calculator_workflow()
    await runner.example_14_document_creation()

def main():
    """Main example runner"""
    import argparse
    
    parser = argparse.ArgumentParser(description="KVirtualStage MCP Server Examples")
    parser.add_argument("--examples", choices=["all", "basic", "claude", "workflow"], 
                       default="basic", help="Which examples to run")
    parser.add_argument("--log-level", choices=["DEBUG", "INFO", "WARNING", "ERROR"],
                       default="INFO", help="Logging level")
    
    args = parser.parse_args()
    
    # Configure logging
    logging.getLogger().setLevel(getattr(logging, args.log_level))
    
    # Run examples
    try:
        if args.examples == "all":
            runner = MCPExampleRunner()
            asyncio.run(runner.run_all_examples())
        elif args.examples == "basic":
            asyncio.run(run_basic_examples())
        elif args.examples == "claude":
            asyncio.run(run_claude_examples())
        elif args.examples == "workflow":
            asyncio.run(run_workflow_examples())
            
        print("\n🎉 Examples completed successfully!")
        
    except KeyboardInterrupt:
        print("\n❌ Examples interrupted by user")
    except Exception as e:
        print(f"\n❌ Examples failed: {e}")
        raise

if __name__ == "__main__":
    main()