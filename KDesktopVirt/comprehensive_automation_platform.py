#!/usr/bin/env python3
"""
Comprehensive Automation Platform for KVirtualStage
Supports multiple automation approaches:
1. MCP Live Scripting (for real-time tool calls)
2. Normal Python Scripting (for traditional automation)
3. ACI - Agent-Computer Interface (for AI agents)
4. Desktop Usage Recording (for capturing real interactions)
"""

import asyncio
import subprocess
import time
import os
import json
import logging
from typing import Dict, List, Optional, Any, Union, Callable
from dataclasses import dataclass, asdict
from enum import Enum
from accurate_automation import AccurateAutomation
from live_scripting_automation import LiveScriptingMCPServer

logger = logging.getLogger(__name__)

class AutomationMode(Enum):
    """Different automation modes supported"""
    NORMAL_SCRIPT = "normal_script"
    MCP_LIVE = "mcp_live"
    ACI_AGENT = "aci_agent"
    DESKTOP_RECORDING = "desktop_recording"

@dataclass
class AutomationAction:
    """Universal automation action"""
    action_type: str  # click, type, wait, launch, etc.
    target: Optional[str] = None
    coordinates: Optional[tuple] = None
    text: Optional[str] = None
    delay: Optional[float] = None
    verify: bool = True
    metadata: Optional[Dict[str, Any]] = None

@dataclass
class AutomationScript:
    """Automation script definition"""
    name: str
    description: str
    actions: List[AutomationAction]
    mode: AutomationMode = AutomationMode.NORMAL_SCRIPT
    settings: Optional[Dict[str, Any]] = None

@dataclass
class DesktopRecording:
    """Desktop recording session"""
    recording_id: str
    output_file: str
    duration: Optional[int] = None
    quality: str = "high"
    include_audio: bool = False
    status: str = "pending"

class NormalScriptingEngine:
    """Traditional Python scripting automation"""
    
    def __init__(self):
        self.automation = AccurateAutomation()
        
    async def execute_script(self, script: AutomationScript) -> Dict[str, Any]:
        """Execute a normal automation script"""
        
        logger.info(f"🐍 Executing normal script: {script.name}")
        
        results = {
            "script_name": script.name,
            "mode": "normal_script",
            "actions_completed": 0,
            "total_actions": len(script.actions),
            "success": False,
            "errors": [],
            "execution_log": []
        }
        
        try:
            for i, action in enumerate(script.actions):
                logger.info(f"Step {i+1}/{len(script.actions)}: {action.action_type}")
                
                success = await self._execute_normal_action(action)
                
                if success:
                    results["actions_completed"] += 1
                    results["execution_log"].append(f"✅ {action.action_type}: Success")
                    
                    # Verification if requested
                    if action.verify:
                        screenshot_path = f"/tmp/verify_step_{i+1}.png"
                        self.automation.take_screenshot(screenshot_path)
                        results["execution_log"].append(f"📸 Verification: {screenshot_path}")
                else:
                    results["errors"].append(f"❌ Step {i+1} failed: {action.action_type}")
                
                # Natural delay between actions
                if action.delay:
                    await asyncio.sleep(action.delay)
                else:
                    await asyncio.sleep(0.5)
            
            results["success"] = results["actions_completed"] == results["total_actions"]
            
        except Exception as e:
            results["errors"].append(f"Script execution failed: {str(e)}")
        
        return results
    
    async def _execute_normal_action(self, action: AutomationAction) -> bool:
        """Execute a single normal automation action"""
        
        try:
            if action.action_type == "launch":
                subprocess.Popen([action.target])
                await asyncio.sleep(3)
                return self.automation.wait_for_application(action.target, 10)
                
            elif action.action_type == "click":
                if action.coordinates:
                    self.automation.precise_click(*action.coordinates, action.target or "Normal click")
                    return True
                else:
                    logger.error("Click action requires coordinates")
                    return False
                    
            elif action.action_type == "type":
                if action.text:
                    # Type with natural timing
                    import subprocess
                    for char in action.text:
                        if char == '\n':
                            subprocess.run(['xdotool', 'key', 'Return'])
                            await asyncio.sleep(0.2)
                        else:
                            subprocess.run(['xdotool', 'type', '--delay', '50', char])
                            await asyncio.sleep(0.02)
                    return True
                else:
                    return False
                    
            elif action.action_type == "wait":
                await asyncio.sleep(action.delay or 1.0)
                return True
                
            elif action.action_type == "screenshot":
                output_path = action.target or f"/tmp/screenshot_{int(time.time())}.png"
                self.automation.take_screenshot(output_path)
                return True
                
            else:
                logger.warning(f"Unknown action type: {action.action_type}")
                return False
                
        except Exception as e:
            logger.error(f"Action execution failed: {e}")
            return False

class ACIAgentInterface:
    """Agent-Computer Interface for AI agents"""
    
    def __init__(self):
        self.automation = AccurateAutomation()
        self.mcp_server = LiveScriptingMCPServer()
        self.agent_sessions: Dict[str, Dict] = {}
        
    async def create_agent_session(self, agent_id: str, capabilities: List[str]) -> Dict[str, Any]:
        """Create a new ACI session for an AI agent"""
        
        session = {
            "agent_id": agent_id,
            "capabilities": capabilities,
            "created_at": time.time(),
            "active": True,
            "actions_performed": 0,
            "current_state": "initialized"
        }
        
        self.agent_sessions[agent_id] = session
        
        logger.info(f"🤖 Created ACI session for agent: {agent_id}")
        return {
            "session_created": True,
            "agent_id": agent_id,
            "available_capabilities": self._get_available_capabilities(),
            "session_info": session
        }
    
    def _get_available_capabilities(self) -> List[str]:
        """Get list of available ACI capabilities"""
        return [
            "desktop_control",
            "application_launching",
            "window_management",
            "precise_clicking",
            "text_input",
            "screenshot_capture",
            "state_monitoring",
            "workflow_orchestration"
        ]
    
    async def execute_agent_command(self, agent_id: str, command: Dict[str, Any]) -> Dict[str, Any]:
        """Execute a command from an AI agent"""
        
        if agent_id not in self.agent_sessions:
            return {"error": "Agent session not found", "success": False}
        
        session = self.agent_sessions[agent_id]
        command_type = command.get("type", "unknown")
        
        logger.info(f"🤖 Agent {agent_id} executing: {command_type}")
        
        try:
            if command_type == "observe_desktop":
                return await self._aci_observe_desktop(agent_id)
                
            elif command_type == "interact_with_element":
                return await self._aci_interact_element(agent_id, command)
                
            elif command_type == "launch_application":
                return await self._aci_launch_application(agent_id, command)
                
            elif command_type == "perform_workflow":
                return await self._aci_perform_workflow(agent_id, command)
                
            elif command_type == "get_session_state":
                return {"session_state": session, "success": True}
                
            else:
                return {"error": f"Unknown command type: {command_type}", "success": False}
                
        except Exception as e:
            logger.error(f"ACI command execution failed: {e}")
            return {"error": str(e), "success": False}
    
    async def _aci_observe_desktop(self, agent_id: str) -> Dict[str, Any]:
        """Let agent observe current desktop state"""
        
        screenshot_path = f"/tmp/aci_observation_{agent_id}_{int(time.time())}.png"
        self.automation.take_screenshot(screenshot_path)
        
        # Get window list
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
            "observation_complete": True,
            "screenshot_path": screenshot_path,
            "open_windows": windows,
            "window_count": len(windows),
            "timestamp": time.time(),
            "success": True
        }
    
    async def _aci_interact_element(self, agent_id: str, command: Dict[str, Any]) -> Dict[str, Any]:
        """Agent interaction with UI element"""
        
        interaction_type = command.get("interaction_type", "click")
        target = command.get("target", {})
        
        if interaction_type == "click" and "coordinates" in target:
            x, y = target["coordinates"]
            description = target.get("description", f"ACI click by {agent_id}")
            
            self.automation.precise_click(x, y, description)
            
            # Update session
            self.agent_sessions[agent_id]["actions_performed"] += 1
            
            return {
                "interaction_complete": True,
                "interaction_type": interaction_type,
                "coordinates": [x, y],
                "success": True
            }
        
        elif interaction_type == "type" and "text" in target:
            text = target["text"]
            
            # Type with natural timing
            for char in text:
                if char == '\n':
                    subprocess.run(['xdotool', 'key', 'Return'])
                    await asyncio.sleep(0.2)
                else:
                    subprocess.run(['xdotool', 'type', '--delay', '30', char])
                    await asyncio.sleep(0.03)
            
            self.agent_sessions[agent_id]["actions_performed"] += 1
            
            return {
                "interaction_complete": True,
                "interaction_type": interaction_type,
                "text_length": len(text),
                "success": True
            }
        
        else:
            return {"error": "Invalid interaction parameters", "success": False}
    
    async def _aci_launch_application(self, agent_id: str, command: Dict[str, Any]) -> Dict[str, Any]:
        """Agent launches an application"""
        
        app_command = command.get("application", "")
        if not app_command:
            return {"error": "Application command required", "success": False}
        
        subprocess.Popen([app_command])
        await asyncio.sleep(3)
        
        # Verify launch
        success = self.automation.wait_for_application(app_command, 10)
        
        if success:
            self.agent_sessions[agent_id]["actions_performed"] += 1
        
        return {
            "application_launched": success,
            "application": app_command,
            "success": success
        }
    
    async def _aci_perform_workflow(self, agent_id: str, command: Dict[str, Any]) -> Dict[str, Any]:
        """Agent performs a complex workflow"""
        
        workflow_steps = command.get("steps", [])
        results = []
        
        for i, step in enumerate(workflow_steps):
            step_result = await self.execute_agent_command(agent_id, step)
            results.append({
                "step": i + 1,
                "command": step,
                "result": step_result
            })
            
            if not step_result.get("success", False):
                break
            
            await asyncio.sleep(0.5)  # Brief pause between steps
        
        return {
            "workflow_complete": True,
            "steps_executed": len(results),
            "total_steps": len(workflow_steps),
            "results": results,
            "success": all(r["result"].get("success", False) for r in results)
        }

class DesktopRecordingEngine:
    """Desktop usage recording and capture"""
    
    def __init__(self, output_dir: str = "/tmp/desktop_recordings"):
        self.output_dir = output_dir
        self.active_recordings: Dict[str, subprocess.Popen] = {}
        os.makedirs(output_dir, exist_ok=True)
        
    async def start_desktop_recording(self, recording_config: DesktopRecording) -> Dict[str, Any]:
        """Start recording desktop usage"""
        
        output_path = os.path.join(self.output_dir, recording_config.output_file)
        
        # FFmpeg command for desktop recording
        cmd = [
            'ffmpeg', '-f', 'x11grab',
            '-framerate', '30',
            '-video_size', '1024x768',
            '-i', ':1.0',
            '-c:v', 'libx264',
            '-preset', 'ultrafast',
            '-crf', '18' if recording_config.quality == 'high' else '23',
            '-pix_fmt', 'yuv420p'
        ]
        
        # Add duration if specified
        if recording_config.duration:
            cmd.extend(['-t', str(recording_config.duration)])
        
        # Add audio if requested
        if recording_config.include_audio:
            cmd.extend(['-f', 'pulse', '-i', 'default'])
        
        cmd.extend(['-y', output_path])
        
        try:
            process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            self.active_recordings[recording_config.recording_id] = process
            
            logger.info(f"📹 Started desktop recording: {recording_config.recording_id}")
            
            return {
                "recording_started": True,
                "recording_id": recording_config.recording_id,
                "output_file": output_path,
                "process_id": process.pid,
                "success": True
            }
            
        except Exception as e:
            logger.error(f"Failed to start recording: {e}")
            return {
                "recording_started": False,
                "error": str(e),
                "success": False
            }
    
    async def stop_desktop_recording(self, recording_id: str) -> Dict[str, Any]:
        """Stop an active desktop recording"""
        
        if recording_id not in self.active_recordings:
            return {"error": "Recording not found", "success": False}
        
        process = self.active_recordings[recording_id]
        
        try:
            process.terminate()
            process.wait(timeout=10)
            
            del self.active_recordings[recording_id]
            
            logger.info(f"📹 Stopped desktop recording: {recording_id}")
            
            return {
                "recording_stopped": True,
                "recording_id": recording_id,
                "success": True
            }
            
        except subprocess.TimeoutExpired:
            process.kill()
            del self.active_recordings[recording_id]
            
            return {
                "recording_stopped": True,
                "recording_id": recording_id,
                "forced_stop": True,
                "success": True
            }
        except Exception as e:
            logger.error(f"Failed to stop recording: {e}")
            return {
                "recording_stopped": False,
                "error": str(e),
                "success": False
            }
    
    async def capture_desktop_usage_demo(self, demo_name: str, duration: int = 60) -> Dict[str, Any]:
        """Capture a desktop usage demonstration"""
        
        recording_id = f"demo_{demo_name}_{int(time.time())}"
        output_file = f"{demo_name}_desktop_usage.mp4"
        
        recording_config = DesktopRecording(
            recording_id=recording_id,
            output_file=output_file,
            duration=duration,
            quality="high"
        )
        
        logger.info(f"🎬 Starting desktop usage demo: {demo_name}")
        
        # Start recording
        start_result = await self.start_desktop_recording(recording_config)
        if not start_result["success"]:
            return start_result
        
        # Wait for recording duration or manual stop
        try:
            await asyncio.sleep(duration)
            stop_result = await self.stop_desktop_recording(recording_id)
            
            return {
                "demo_captured": True,
                "demo_name": demo_name,
                "output_file": os.path.join(self.output_dir, output_file),
                "duration": duration,
                "recording_id": recording_id,
                "success": True
            }
            
        except Exception as e:
            logger.error(f"Demo capture failed: {e}")
            await self.stop_desktop_recording(recording_id)
            return {
                "demo_captured": False,
                "error": str(e),
                "success": False
            }

class ComprehensiveAutomationPlatform:
    """Main platform supporting all automation modes"""
    
    def __init__(self):
        self.normal_scripting = NormalScriptingEngine()
        self.mcp_server = LiveScriptingMCPServer()
        self.aci_interface = ACIAgentInterface()
        self.recording_engine = DesktopRecordingEngine()
        
    async def execute_automation(self, 
                                automation_request: Dict[str, Any],
                                mode: AutomationMode = AutomationMode.NORMAL_SCRIPT) -> Dict[str, Any]:
        """Execute automation in the specified mode"""
        
        logger.info(f"🚀 Executing automation in mode: {mode.value}")
        
        try:
            if mode == AutomationMode.NORMAL_SCRIPT:
                script = AutomationScript(**automation_request)
                return await self.normal_scripting.execute_script(script)
                
            elif mode == AutomationMode.MCP_LIVE:
                session_id = automation_request.get("session_id", "default")
                tool_calls = automation_request.get("tool_calls", [])
                
                results = []
                for call in tool_calls:
                    result = await self.mcp_server.execute_live_tool_call(
                        session_id, call["tool"], call["params"]
                    )
                    results.append(result)
                
                return {
                    "mode": "mcp_live",
                    "tool_calls_executed": len(results),
                    "results": [asdict(r) for r in results],
                    "success": all(r.success for r in results)
                }
                
            elif mode == AutomationMode.ACI_AGENT:
                agent_id = automation_request.get("agent_id", "default_agent")
                commands = automation_request.get("commands", [])
                
                # Create agent session if needed
                if agent_id not in self.aci_interface.agent_sessions:
                    await self.aci_interface.create_agent_session(agent_id, ["desktop_control"])
                
                results = []
                for command in commands:
                    result = await self.aci_interface.execute_agent_command(agent_id, command)
                    results.append(result)
                
                return {
                    "mode": "aci_agent",
                    "agent_id": agent_id,
                    "commands_executed": len(results),
                    "results": results,
                    "success": all(r.get("success", False) for r in results)
                }
                
            elif mode == AutomationMode.DESKTOP_RECORDING:
                recording_config = DesktopRecording(**automation_request)
                return await self.recording_engine.start_desktop_recording(recording_config)
                
            else:
                return {"error": f"Unknown automation mode: {mode}", "success": False}
                
        except Exception as e:
            logger.error(f"Automation execution failed: {e}")
            return {"error": str(e), "success": False}

async def demo_comprehensive_platform():
    """Demonstrate all automation modes"""
    
    platform = ComprehensiveAutomationPlatform()
    
    print("🚀 Comprehensive Automation Platform Demo")
    print("Supporting: Normal Scripting, MCP Live, ACI, Desktop Recording")
    
    # 1. Normal Scripting Demo
    print("\n1️⃣ Normal Scripting Demo")
    normal_script = {
        "name": "Calculator Demo",
        "description": "Traditional Python automation",
        "actions": [
            {"action_type": "launch", "target": "galculator", "delay": 3},
            {"action_type": "screenshot", "target": "/tmp/normal_script_demo.png"},
            {"action_type": "wait", "delay": 2}
        ]
    }
    
    normal_result = await platform.execute_automation(normal_script, AutomationMode.NORMAL_SCRIPT)
    print(f"Normal Script Result: {normal_result['success']}")
    
    # 2. MCP Live Scripting Demo
    print("\n2️⃣ MCP Live Scripting Demo")
    mcp_request = {
        "session_id": "demo_session",
        "tool_calls": [
            {"tool": "create_session", "params": {}},
            {"tool": "take_screenshot", "params": {"output_path": "/tmp/mcp_demo.png"}},
            {"tool": "get_session_state", "params": {}}
        ]
    }
    
    mcp_result = await platform.execute_automation(mcp_request, AutomationMode.MCP_LIVE)
    print(f"MCP Live Result: {mcp_result['success']}")
    
    # 3. ACI Agent Demo
    print("\n3️⃣ ACI Agent Demo")
    aci_request = {
        "agent_id": "demo_agent",
        "commands": [
            {"type": "observe_desktop"},
            {"type": "get_session_state"}
        ]
    }
    
    aci_result = await platform.execute_automation(aci_request, AutomationMode.ACI_AGENT)
    print(f"ACI Agent Result: {aci_result['success']}")
    
    # 4. Desktop Recording Demo
    print("\n4️⃣ Desktop Recording Demo")
    recording_request = {
        "recording_id": "demo_recording",
        "output_file": "comprehensive_demo.mp4",
        "duration": 10,
        "quality": "high"
    }
    
    recording_result = await platform.execute_automation(recording_request, AutomationMode.DESKTOP_RECORDING)
    print(f"Desktop Recording Result: {recording_result['success']}")
    
    # Wait for recording to complete
    if recording_result['success']:
        print("📹 Recording for 10 seconds...")
        await asyncio.sleep(12)
        
        stop_result = await platform.recording_engine.stop_desktop_recording("demo_recording")
        print(f"Recording Stopped: {stop_result['success']}")
    
    print("\n🏆 Comprehensive Platform Demo Complete!")
    print("All automation modes demonstrated successfully.")

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
    asyncio.run(demo_comprehensive_platform())