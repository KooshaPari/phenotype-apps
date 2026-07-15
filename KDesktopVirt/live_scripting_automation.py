#!/usr/bin/env python3
"""
Live Scripting Automation for KVirtualStage
MCP-powered real-time automation with live tool calls and feedback
Similar to Playwright MCP but for desktop automation
"""

import asyncio
import json
import logging
from typing import Dict, List, Optional, Any, Callable
from dataclasses import dataclass, asdict
from accurate_automation import AccurateAutomation
import time

logger = logging.getLogger(__name__)

@dataclass
class MCPToolCall:
    """MCP tool call with live feedback"""
    tool_name: str
    parameters: Dict[str, Any]
    call_id: str
    timestamp: float
    result: Optional[Dict[str, Any]] = None
    success: Optional[bool] = None
    error: Optional[str] = None

@dataclass
class LiveScriptingSession:
    """Live scripting session state"""
    session_id: str
    active: bool = True
    tool_calls: List[MCPToolCall] = None
    current_step: int = 0
    automation_state: Dict[str, Any] = None
    
    def __post_init__(self):
        if self.tool_calls is None:
            self.tool_calls = []
        if self.automation_state is None:
            self.automation_state = {}

class LiveScriptingMCPServer:
    """
    MCP Server for Live Scripting Desktop Automation
    Provides real-time tool calls similar to Playwright MCP
    """
    
    def __init__(self):
        self.automation = AccurateAutomation()
        self.active_sessions: Dict[str, LiveScriptingSession] = {}
        self.tool_registry = self._register_tools()
        
    def _register_tools(self) -> Dict[str, Callable]:
        """Register all available MCP tools for live scripting"""
        return {
            # Core automation tools
            "take_screenshot": self._tool_take_screenshot,
            "precise_click": self._tool_precise_click,
            "type_text": self._tool_type_text,
            "launch_application": self._tool_launch_application,
            "wait_for_application": self._tool_wait_for_application,
            
            # Window management tools
            "get_window_info": self._tool_get_window_info,
            "focus_window": self._tool_focus_window,
            "list_windows": self._tool_list_windows,
            
            # Calculator automation tools
            "click_calculator_button": self._tool_click_calculator_button,
            "get_calculator_layout": self._tool_get_calculator_layout,
            "perform_calculation": self._tool_perform_calculation,
            
            # Text editor automation tools
            "click_text_area": self._tool_click_text_area,
            
            # Session management tools
            "create_session": self._tool_create_session,
            "get_session_state": self._tool_get_session_state,
            
            # Live feedback tools
            "verify_action": self._tool_verify_action,
            "get_current_state": self._tool_get_current_state,
        }
    
    async def execute_live_tool_call(self, 
                                   session_id: str,
                                   tool_name: str, 
                                   parameters: Dict[str, Any]) -> MCPToolCall:
        """Execute a live MCP tool call with real-time feedback"""
        
        call_id = f"{session_id}_{int(time.time() * 1000)}"
        tool_call = MCPToolCall(
            tool_name=tool_name,
            parameters=parameters,
            call_id=call_id,
            timestamp=time.time()
        )
        
        # Ensure session exists
        if session_id not in self.active_sessions:
            self.active_sessions[session_id] = LiveScriptingSession(session_id=session_id)
        
        session = self.active_sessions[session_id]
        session.tool_calls.append(tool_call)
        
        try:
            logger.info(f"🔧 Executing live tool call: {tool_name}")
            
            # Execute the tool
            if tool_name in self.tool_registry:
                result = await self.tool_registry[tool_name](session_id, parameters)
                tool_call.result = result
                tool_call.success = True
                
                # Update session state
                session.current_step += 1
                session.automation_state.update(result.get('state_updates', {}))
                
                logger.info(f"✅ Tool call completed: {tool_name}")
                
            else:
                error_msg = f"Unknown tool: {tool_name}"
                tool_call.error = error_msg
                tool_call.success = False
                logger.error(f"❌ {error_msg}")
        
        except Exception as e:
            error_msg = f"Tool execution failed: {str(e)}"
            tool_call.error = error_msg
            tool_call.success = False
            logger.error(f"❌ {error_msg}")
        
        return tool_call
    
    # Core Automation Tools
    async def _tool_take_screenshot(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Take screenshot with live feedback"""
        output_path = params.get('output_path', f'/tmp/live_screenshot_{session_id}_{int(time.time())}.png')
        
        self.automation.take_screenshot(output_path)
        
        return {
            'screenshot_taken': True,
            'output_path': output_path,
            'timestamp': time.time(),
            'state_updates': {'last_screenshot': output_path}
        }
    
    async def _tool_precise_click(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Precise click with live feedback"""
        x = params.get('x')
        y = params.get('y')
        description = params.get('description', 'Live scripted click')
        
        if x is None or y is None:
            raise ValueError("Coordinates x and y are required")
        
        self.automation.precise_click(x, y, description)
        
        return {
            'clicked': True,
            'coordinates': [x, y],
            'description': description,
            'state_updates': {'last_click': [x, y]}
        }
    
    async def _tool_type_text(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Type text with live feedback"""
        text = params.get('text', '')
        delay = params.get('delay', 0.05)
        
        if not text:
            raise ValueError("Text parameter is required")
        
        # Type with natural timing
        import subprocess
        for char in text:
            if char == '\n':
                subprocess.run(['xdotool', 'key', 'Return'])
                await asyncio.sleep(0.2)
            else:
                subprocess.run(['xdotool', 'type', '--delay', str(int(delay * 1000)), char])
                await asyncio.sleep(delay)
        
        return {
            'text_typed': True,
            'text_length': len(text),
            'text_content': text[:50] + '...' if len(text) > 50 else text,
            'state_updates': {'last_typed_text': text}
        }
    
    async def _tool_launch_application(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Launch application with live feedback"""
        app_name = params.get('app_name', '')
        command = params.get('command', app_name)
        
        if not command:
            raise ValueError("Application command is required")
        
        # Launch application
        import subprocess
        subprocess.Popen([command])
        
        # Wait briefly for launch
        await asyncio.sleep(2)
        
        return {
            'application_launched': True,
            'app_name': app_name,
            'command': command,
            'state_updates': {'launched_apps': self.active_sessions[session_id].automation_state.get('launched_apps', []) + [app_name]}
        }
    
    async def _tool_wait_for_application(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Wait for application with live feedback"""
        app_name = params.get('app_name', '')
        timeout = params.get('timeout', 10)
        
        if not app_name:
            raise ValueError("Application name is required")
        
        success = self.automation.wait_for_application(app_name, timeout)
        
        return {
            'application_ready': success,
            'app_name': app_name,
            'timeout': timeout,
            'state_updates': {'ready_apps': self.active_sessions[session_id].automation_state.get('ready_apps', []) + ([app_name] if success else [])}
        }
    
    # Window Management Tools
    async def _tool_get_window_info(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Get window information with live feedback"""
        window_class = params.get('window_class', '')
        
        if not window_class:
            raise ValueError("Window class is required")
        
        window_info = self.automation.find_window_info(window_class)
        
        return {
            'window_found': window_info is not None,
            'window_info': window_info or {},
            'window_class': window_class,
            'state_updates': {'current_window': window_info}
        }
    
    # Calculator Automation Tools
    async def _tool_click_calculator_button(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Click calculator button with live feedback"""
        button = params.get('button', '')
        
        if not button:
            raise ValueError("Button parameter is required")
        
        # Get calculator window and button layout
        window_info = self.automation.find_window_info('galculator')
        if not window_info:
            raise RuntimeError("Calculator window not found")
        
        buttons = self.automation.calculate_galculator_buttons(window_info)
        
        if button not in buttons:
            raise ValueError(f"Button '{button}' not found in calculator layout")
        
        # Click the button
        x, y = buttons[button]
        self.automation.precise_click(x, y, f"Calculator button: {button}")
        
        return {
            'button_clicked': True,
            'button': button,
            'coordinates': [x, y],
            'state_updates': {'calculator_buttons_pressed': self.active_sessions[session_id].automation_state.get('calculator_buttons_pressed', []) + [button]}
        }
    
    async def _tool_get_calculator_layout(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Get calculator button layout"""
        window_info = self.automation.find_window_info('galculator')
        if not window_info:
            raise RuntimeError("Calculator window not found")
        
        buttons = self.automation.calculate_galculator_buttons(window_info)
        
        return {
            'layout_retrieved': True,
            'window_info': window_info,
            'button_layout': {button: list(coords) for button, coords in buttons.items()},
            'button_count': len(buttons),
            'state_updates': {'calculator_layout': buttons}
        }
    
    async def _tool_perform_calculation(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Perform complete calculation with live feedback"""
        expression = params.get('expression', '')  # e.g., "8*7"
        
        if not expression:
            raise ValueError("Expression parameter is required")
        
        # Parse and execute calculation
        buttons_to_press = []
        for char in expression:
            if char.isdigit():
                buttons_to_press.append(char)
            elif char == '+':
                buttons_to_press.append('+')
            elif char == '-':
                buttons_to_press.append('-')
            elif char == '*' or char == '×':
                buttons_to_press.append('×')
            elif char == '/' or char == '÷':
                buttons_to_press.append('÷')
            elif char == '=':
                buttons_to_press.append('=')
        
        # Add equals if not present
        if '=' not in buttons_to_press:
            buttons_to_press.append('=')
        
        # Get calculator layout
        window_info = self.automation.find_window_info('galculator')
        if not window_info:
            raise RuntimeError("Calculator window not found")
        
        buttons = self.automation.calculate_galculator_buttons(window_info)
        
        # Press each button
        pressed_buttons = []
        for button in buttons_to_press:
            if button in buttons:
                x, y = buttons[button]
                self.automation.precise_click(x, y, f"Calculator: {button}")
                pressed_buttons.append(button)
                await asyncio.sleep(0.5)  # Natural timing
        
        return {
            'calculation_performed': True,
            'expression': expression,
            'buttons_pressed': pressed_buttons,
            'state_updates': {'last_calculation': expression}
        }
    
    # Text Editor Tools
    async def _tool_click_text_area(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Click text editor area with live feedback"""
        editor_name = params.get('editor_name', 'mousepad')
        
        window_info = self.automation.find_window_info(editor_name)
        if not window_info:
            raise RuntimeError(f"Text editor '{editor_name}' window not found")
        
        # Click in center of text area
        text_x = window_info['x'] + window_info['width'] // 2
        text_y = window_info['y'] + window_info['height'] // 2 + 20
        
        self.automation.precise_click(text_x, text_y, f"Text area in {editor_name}")
        
        return {
            'text_area_clicked': True,
            'editor_name': editor_name,
            'coordinates': [text_x, text_y],
            'state_updates': {'active_editor': editor_name}
        }
    
    # Session Management Tools
    async def _tool_create_session(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Create live scripting session"""
        if session_id not in self.active_sessions:
            self.active_sessions[session_id] = LiveScriptingSession(session_id=session_id)
        
        return {
            'session_created': True,
            'session_id': session_id,
            'state_updates': {'session_active': True}
        }
    
    async def _tool_get_session_state(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Get current session state"""
        if session_id not in self.active_sessions:
            raise ValueError(f"Session {session_id} not found")
        
        session = self.active_sessions[session_id]
        
        return {
            'session_active': session.active,
            'current_step': session.current_step,
            'total_tool_calls': len(session.tool_calls),
            'automation_state': session.automation_state,
            'recent_calls': [asdict(call) for call in session.tool_calls[-5:]]  # Last 5 calls
        }
    
    async def _tool_focus_window(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Focus window tool implementation"""
        window_title = params.get('window_title', '')
        
        if not window_title:
            raise ValueError("Window title is required")
        
        try:
            # Use wmctrl to focus window
            result = subprocess.run(['wmctrl', '-a', window_title], capture_output=True, text=True)
            success = result.returncode == 0
            
            return {
                'window_focused': success,
                'window_title': window_title,
                'state_updates': {'focused_window': window_title if success else None}
            }
        except Exception as e:
            raise RuntimeError(f"Failed to focus window: {e}")
    
    async def _tool_list_windows(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """List windows tool implementation"""
        
        result = subprocess.run(['wmctrl', '-l'], capture_output=True, text=True)
        windows = []
        
        if result.returncode == 0:
            for line in result.stdout.split('\n'):
                if line.strip():
                    parts = line.split(None, 3)
                    if len(parts) >= 4:
                        windows.append({
                            'id': parts[0],
                            'desktop': parts[1], 
                            'host': parts[2],
                            'title': parts[3]
                        })
        
        return {
            'windows_listed': True,
            'window_count': len(windows),
            'windows': windows,
            'state_updates': {'window_list': windows}
        }
    
    # Live Feedback Tools
    async def _tool_verify_action(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Verify last action with live feedback"""
        action_type = params.get('action_type', 'screenshot')
        
        if action_type == 'screenshot':
            # Take verification screenshot
            verify_path = f'/tmp/verify_{session_id}_{int(time.time())}.png'
            self.automation.take_screenshot(verify_path)
            
            return {
                'verification_completed': True,
                'verification_type': 'screenshot',
                'verification_path': verify_path,
                'state_updates': {'last_verification': verify_path}
            }
        
        return {
            'verification_completed': False,
            'error': f"Unknown verification type: {action_type}"
        }
    
    async def _tool_get_current_state(self, session_id: str, params: Dict[str, Any]) -> Dict[str, Any]:
        """Get current desktop state for live feedback"""
        
        # Take current screenshot
        state_screenshot = f'/tmp/state_{session_id}_{int(time.time())}.png'
        self.automation.take_screenshot(state_screenshot)
        
        # Get window list
        import subprocess
        result = subprocess.run(['wmctrl', '-l'], capture_output=True, text=True)
        windows = []
        if result.returncode == 0:
            for line in result.stdout.split('\n'):
                if line.strip():
                    parts = line.split(None, 3)
                    if len(parts) >= 4:
                        windows.append({
                            'id': parts[0],
                            'desktop': parts[1],
                            'host': parts[2],
                            'title': parts[3]
                        })
        
        return {
            'current_state_captured': True,
            'screenshot_path': state_screenshot,
            'open_windows': windows,
            'window_count': len(windows),
            'state_updates': {'current_screenshot': state_screenshot, 'window_list': windows}
        }

async def demo_live_scripting():
    """Demonstration of live scripting MCP automation"""
    
    server = LiveScriptingMCPServer()
    session_id = "demo_session"
    
    print("🚀 Starting Live Scripting MCP Automation Demo")
    
    # Sequence of live tool calls
    tool_calls = [
        {"tool": "create_session", "params": {}},
        {"tool": "take_screenshot", "params": {"output_path": "/tmp/demo_start.png"}},
        {"tool": "launch_application", "params": {"app_name": "galculator", "command": "galculator"}},
        {"tool": "wait_for_application", "params": {"app_name": "galculator", "timeout": 10}},
        {"tool": "get_window_info", "params": {"window_class": "galculator"}},
        {"tool": "perform_calculation", "params": {"expression": "8*7"}},
        {"tool": "verify_action", "params": {"action_type": "screenshot"}},
        {"tool": "launch_application", "params": {"app_name": "mousepad", "command": "mousepad"}},
        {"tool": "wait_for_application", "params": {"app_name": "mousepad", "timeout": 10}},
        {"tool": "click_text_area", "params": {"editor_name": "mousepad"}},
        {"tool": "type_text", "params": {"text": "LIVE SCRIPTING DEMO\n\nCalculation result: 8 × 7 = 56\n\nThis demonstrates MCP live scripting with real-time tool calls!"}},
        {"tool": "get_session_state", "params": {}},
        {"tool": "take_screenshot", "params": {"output_path": "/tmp/demo_complete.png"}}
    ]
    
    for i, call in enumerate(tool_calls):
        print(f"\n📞 Tool Call {i+1}/{len(tool_calls)}: {call['tool']}")
        
        result = await server.execute_live_tool_call(
            session_id=session_id,
            tool_name=call['tool'],
            parameters=call['params']
        )
        
        if result.success:
            print(f"✅ Success: {result.tool_name}")
            if result.result:
                # Show key results
                for key, value in result.result.items():
                    if not key.startswith('state_updates'):
                        print(f"   {key}: {value}")
        else:
            print(f"❌ Failed: {result.error}")
        
        # Brief pause between calls
        await asyncio.sleep(1)
    
    print("\n🏆 Live Scripting Demo Completed!")
    print(f"📊 Session State: {len(server.active_sessions[session_id].tool_calls)} tool calls executed")

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
    asyncio.run(demo_live_scripting())