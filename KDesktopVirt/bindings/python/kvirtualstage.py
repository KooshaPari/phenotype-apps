"""
KVirtualStage Python Bindings

Playwright-equivalent desktop automation platform for AI agents.
Provides high-level Python API for:
- Desktop automation and control
- Session management
- Recording and playback
- Natural human-like interactions
- Cross-platform virtualization

Example Usage:
    >>> import kvirtualstage as kvs
    >>> 
    >>> # Create automation instance
    >>> automation = kvs.KVirtualStage()
    >>> 
    >>> # Create a new desktop session
    >>> session = automation.create_session(
    ...     user_id="demo_user",
    ...     session_name="my_session",
    ...     desktop_type="ubuntu"
    ... )
    >>> 
    >>> # Perform natural automation
    >>> session.move_cursor(400, 300)
    >>> session.click()
    >>> session.type_text("Hello from KVirtualStage!")
    >>> 
    >>> # Start recording
    >>> recording = session.start_recording("demo.mp4")
    >>> 
    >>> # Execute workflow
    >>> workflow = kvs.Workflow("Calculator Demo")
    >>> workflow.add_step("move_cursor", x=100, y=100)
    >>> workflow.add_step("click")
    >>> workflow.add_step("type", text="2 + 2 =")
    >>> session.execute_workflow(workflow)
    >>> 
    >>> # Stop recording
    >>> recording.stop()
"""

import asyncio
import json
from typing import Dict, List, Optional, Any, Union
from dataclasses import dataclass
from enum import Enum
import time
import logging

# Configure logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

class MouseButton(Enum):
    """Mouse button types for click operations."""
    LEFT = "left"
    RIGHT = "right" 
    MIDDLE = "middle"

class DesktopType(Enum):
    """Supported desktop environments."""
    UBUNTU = "ubuntu"
    UBUNTU_XFCE = "ubuntu-xfce"
    UBUNTU_KDE = "ubuntu-kde"
    CENTOS = "centos"
    FEDORA = "fedora"
    ARCH = "arch"
    DEBIAN = "debian"

class RecordingQuality(Enum):
    """Recording quality presets."""
    LOW = "low"
    MEDIUM = "medium" 
    HIGH = "high"
    STREAMING = "streaming"

@dataclass
class Point:
    """2D coordinate point."""
    x: float
    y: float

@dataclass
class SessionInfo:
    """Information about an active session."""
    session_id: str
    user_id: str
    desktop_type: str
    status: str
    created_at: float
    last_activity: float
    recording_active: bool

@dataclass
class WorkflowResult:
    """Result of workflow execution."""
    workflow_name: str
    success: bool
    total_steps: int
    successful_steps: int
    execution_time_ms: int
    errors: List[str]

class WorkflowStep:
    """Individual step in an automation workflow."""
    
    def __init__(self, name: str, action_type: str, **kwargs):
        self.name = name
        self.action_type = action_type
        self.parameters = kwargs
        self.timeout_seconds = kwargs.get('timeout', 30)

class Workflow:
    """Automation workflow definition."""
    
    def __init__(self, name: str, description: str = "", continue_on_error: bool = False):
        self.name = name
        self.description = description
        self.continue_on_error = continue_on_error
        self.steps: List[WorkflowStep] = []
    
    def add_step(self, action_type: str, name: str = None, **kwargs) -> 'Workflow':
        """Add a step to the workflow."""
        if name is None:
            name = f"Step {len(self.steps) + 1}: {action_type}"
        
        step = WorkflowStep(name, action_type, **kwargs)
        self.steps.append(step)
        return self
    
    def move_cursor(self, x: float, y: float, name: str = None) -> 'Workflow':
        """Add cursor movement step."""
        return self.add_step("move_cursor", name=name, x=x, y=y)
    
    def click(self, x: float = None, y: float = None, button: MouseButton = MouseButton.LEFT, name: str = None) -> 'Workflow':
        """Add click step."""
        params = {"button": button.value}
        if x is not None and y is not None:
            params.update({"x": x, "y": y})
        return self.add_step("click", name=name, **params)
    
    def type_text(self, text: str, name: str = None) -> 'Workflow':
        """Add text typing step."""
        return self.add_step("type", name=name, text=text)
    
    def wait(self, seconds: float, name: str = None) -> 'Workflow':
        """Add wait/delay step."""
        return self.add_step("wait", name=name, duration=seconds)
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert workflow to dictionary for API."""
        return {
            "name": self.name,
            "description": self.description,
            "continue_on_error": self.continue_on_error,
            "steps": [
                {
                    "name": step.name,
                    "action_type": step.action_type,
                    "parameters": step.parameters,
                    "timeout_seconds": step.timeout_seconds
                }
                for step in self.steps
            ]
        }

class Recording:
    """Recording session control."""
    
    def __init__(self, session: 'Session', recording_id: str):
        self.session = session
        self.recording_id = recording_id
        self.active = True
        self.start_time = time.time()
    
    async def stop(self) -> str:
        """Stop the recording and return output path."""
        if not self.active:
            raise RuntimeError("Recording is not active")
        
        output_path = await self.session._api_call(
            f"sessions/{self.session.session_id}/recording/stop",
            method="POST"
        )
        
        self.active = False
        logger.info(f"Recording stopped: {output_path}")
        return output_path
    
    @property
    def duration(self) -> float:
        """Get recording duration in seconds."""
        return time.time() - self.start_time

class Session:
    """Desktop automation session."""
    
    def __init__(self, kvs_instance: 'KVirtualStage', session_id: str, user_id: str, desktop_type: str):
        self.kvs = kvs_instance
        self.session_id = session_id
        self.user_id = user_id
        self.desktop_type = desktop_type
        self.current_recording: Optional[Recording] = None
    
    async def _api_call(self, endpoint: str, method: str = "GET", data: Dict = None) -> Any:
        """Make API call to KVirtualStage server."""
        return await self.kvs._api_call(endpoint, method, data)
    
    async def move_cursor(self, x: float, y: float) -> None:
        """Move cursor to specified coordinates with natural movement."""
        await self._api_call(
            f"sessions/{self.session_id}/cursor/move",
            method="POST",
            data={"target_x": x, "target_y": y}
        )
        logger.info(f"Cursor moved to ({x}, {y})")
    
    async def click(self, x: float = None, y: float = None, button: MouseButton = MouseButton.LEFT) -> None:
        """Click at current cursor position or specified coordinates."""
        if x is not None and y is not None:
            await self.move_cursor(x, y)
        
        await self._api_call(
            f"sessions/{self.session_id}/mouse/click",
            method="POST",
            data={"button": button.value}
        )
        logger.info(f"Clicked with {button.value} button")
    
    async def double_click(self, x: float = None, y: float = None) -> None:
        """Double-click at current cursor position or specified coordinates."""
        if x is not None and y is not None:
            await self.move_cursor(x, y)
        
        await self.click()
        await asyncio.sleep(0.1)  # Small delay between clicks
        await self.click()
        logger.info("Double-clicked")
    
    async def right_click(self, x: float = None, y: float = None) -> None:
        """Right-click at current cursor position or specified coordinates."""
        await self.click(x, y, MouseButton.RIGHT)
    
    async def type_text(self, text: str, wpm: float = 65.0) -> None:
        """Type text with natural human-like timing."""
        await self._api_call(
            f"sessions/{self.session_id}/keyboard/type",
            method="POST",
            data={"text": text, "wpm": wpm}
        )
        logger.info(f"Typed: {text[:50]}{'...' if len(text) > 50 else ''}")
    
    async def key_press(self, key: str) -> None:
        """Press a specific key (e.g., 'Enter', 'Tab', 'Escape')."""
        await self._api_call(
            f"sessions/{self.session_id}/keyboard/key",
            method="POST", 
            data={"key": key}
        )
        logger.info(f"Key pressed: {key}")
    
    async def key_combination(self, *keys: str) -> None:
        """Press a combination of keys (e.g., 'Ctrl', 'C')."""
        await self._api_call(
            f"sessions/{self.session_id}/keyboard/combo",
            method="POST",
            data={"keys": list(keys)}
        )
        logger.info(f"Key combination: {'+'.join(keys)}")
    
    async def scroll(self, direction: str, amount: int = 3) -> None:
        """Scroll in the specified direction."""
        await self._api_call(
            f"sessions/{self.session_id}/mouse/scroll",
            method="POST",
            data={"direction": direction, "amount": amount}
        )
        logger.info(f"Scrolled {direction} by {amount}")
    
    async def screenshot(self, filename: str = None) -> str:
        """Take a screenshot of the current desktop."""
        result = await self._api_call(
            f"sessions/{self.session_id}/screenshot",
            method="POST",
            data={"filename": filename}
        )
        logger.info(f"Screenshot taken: {result}")
        return result
    
    async def start_recording(self, filename: str = None, quality: RecordingQuality = RecordingQuality.MEDIUM) -> Recording:
        """Start recording the session."""
        if self.current_recording and self.current_recording.active:
            raise RuntimeError("Recording is already active")
        
        if filename is None:
            filename = f"kvs_recording_{int(time.time())}.mp4"
        
        result = await self._api_call(
            f"sessions/{self.session_id}/recording/start",
            method="POST",
            data={
                "output_filename": filename,
                "quality": quality.value
            }
        )
        
        recording = Recording(self, result["recording_id"])
        self.current_recording = recording
        logger.info(f"Recording started: {filename}")
        return recording
    
    async def execute_workflow(self, workflow: Workflow) -> WorkflowResult:
        """Execute an automation workflow."""
        result = await self._api_call(
            f"sessions/{self.session_id}/workflow",
            method="POST",
            data=workflow.to_dict()
        )
        
        workflow_result = WorkflowResult(
            workflow_name=result["workflow_name"],
            success=result["success"],
            total_steps=result["total_steps"],
            successful_steps=result["successful_steps"],
            execution_time_ms=result["execution_time_ms"],
            errors=result["errors"]
        )
        
        logger.info(f"Workflow '{workflow.name}' completed: {workflow_result.success}")
        return workflow_result
    
    async def get_info(self) -> SessionInfo:
        """Get detailed session information."""
        result = await self._api_call(f"sessions/{self.session_id}")
        
        return SessionInfo(
            session_id=result["session_id"],
            user_id=result["user_id"], 
            desktop_type=result["desktop_type"],
            status=result["status"],
            created_at=result.get("created_at", 0),
            last_activity=result.get("last_activity", 0),
            recording_active=result.get("recording_active", False)
        )
    
    async def close(self) -> None:
        """Close the session and clean up resources."""
        if self.current_recording and self.current_recording.active:
            await self.current_recording.stop()
        
        await self._api_call(
            f"sessions/{self.session_id}",
            method="DELETE"
        )
        logger.info(f"Session closed: {self.session_id}")

class KVirtualStage:
    """Main KVirtualStage automation interface."""
    
    def __init__(self, server_url: str = "http://localhost:8080"):
        self.server_url = server_url.rstrip('/')
        self.base_url = f"{self.server_url}/api/v1"
        self.sessions: Dict[str, Session] = {}
    
    async def _api_call(self, endpoint: str, method: str = "GET", data: Dict = None) -> Any:
        """Make HTTP API call to KVirtualStage server."""
        import aiohttp
        
        url = f"{self.base_url}/{endpoint.lstrip('/')}"
        
        async with aiohttp.ClientSession() as session:
            kwargs = {
                "headers": {"Content-Type": "application/json"}
            }
            
            if data:
                kwargs["json"] = data
            
            async with session.request(method, url, **kwargs) as response:
                if response.status >= 400:
                    text = await response.text()
                    raise RuntimeError(f"API call failed ({response.status}): {text}")
                
                result = await response.json()
                
                if not result.get("success", True):
                    error_msg = result.get("error", "Unknown error")
                    raise RuntimeError(f"API error: {error_msg}")
                
                return result.get("data", result)
    
    async def create_session(self, user_id: str, session_name: str = None, 
                           desktop_type: DesktopType = DesktopType.UBUNTU) -> Session:
        """Create a new desktop automation session."""
        if session_name is None:
            session_name = f"session_{int(time.time())}"
        
        result = await self._api_call(
            "sessions",
            method="POST",
            data={
                "user_id": user_id,
                "session_name": session_name,
                "desktop_type": desktop_type.value
            }
        )
        
        session = Session(self, result["session_id"], user_id, desktop_type.value)
        self.sessions[session.session_id] = session
        
        logger.info(f"Session created: {session.session_id}")
        return session
    
    async def get_session(self, session_id: str) -> Optional[Session]:
        """Get an existing session by ID."""
        if session_id in self.sessions:
            return self.sessions[session_id]
        
        # Try to fetch from server
        try:
            session_info = await self._api_call(f"sessions/{session_id}")
            session = Session(self, session_id, session_info["user_id"], session_info["desktop_type"])
            self.sessions[session_id] = session
            return session
        except RuntimeError:
            return None
    
    async def list_sessions(self) -> List[SessionInfo]:
        """List all active sessions."""
        result = await self._api_call("sessions")
        
        return [
            SessionInfo(
                session_id=s["session_id"],
                user_id=s["user_id"],
                desktop_type=s["desktop_type"],
                status=s["status"],
                created_at=s.get("created_at", 0),
                last_activity=s.get("last_activity", 0),
                recording_active=s.get("recording_active", False)
            )
            for s in result
        ]
    
    async def health_check(self) -> Dict[str, Any]:
        """Check server health and status."""
        return await self._api_call("health")
    
    async def get_metrics(self) -> Dict[str, Any]:
        """Get server performance metrics."""
        return await self._api_call("metrics")
    
    async def close_all_sessions(self) -> None:
        """Close all active sessions."""
        for session in list(self.sessions.values()):
            await session.close()
        self.sessions.clear()

# Convenience functions for synchronous usage
def run_async(coro):
    """Run an async function synchronously."""
    try:
        loop = asyncio.get_event_loop()
    except RuntimeError:
        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
    
    return loop.run_until_complete(coro)

class SyncSession:
    """Synchronous wrapper for Session class."""
    
    def __init__(self, async_session: Session):
        self._async_session = async_session
    
    def move_cursor(self, x: float, y: float) -> None:
        return run_async(self._async_session.move_cursor(x, y))
    
    def click(self, x: float = None, y: float = None, button: MouseButton = MouseButton.LEFT) -> None:
        return run_async(self._async_session.click(x, y, button))
    
    def type_text(self, text: str, wpm: float = 65.0) -> None:
        return run_async(self._async_session.type_text(text, wpm))
    
    def screenshot(self, filename: str = None) -> str:
        return run_async(self._async_session.screenshot(filename))
    
    def execute_workflow(self, workflow: Workflow) -> WorkflowResult:
        return run_async(self._async_session.execute_workflow(workflow))
    
    def close(self) -> None:
        return run_async(self._async_session.close())

class SyncKVirtualStage:
    """Synchronous wrapper for KVirtualStage class."""
    
    def __init__(self, server_url: str = "http://localhost:8080"):
        self._async_kvs = KVirtualStage(server_url)
    
    def create_session(self, user_id: str, session_name: str = None,
                      desktop_type: DesktopType = DesktopType.UBUNTU) -> SyncSession:
        async_session = run_async(self._async_kvs.create_session(user_id, session_name, desktop_type))
        return SyncSession(async_session)
    
    def list_sessions(self) -> List[SessionInfo]:
        return run_async(self._async_kvs.list_sessions())
    
    def health_check(self) -> Dict[str, Any]:
        return run_async(self._async_kvs.health_check())

# Export public API
__all__ = [
    'KVirtualStage', 'SyncKVirtualStage',
    'Session', 'SyncSession', 
    'Workflow', 'WorkflowStep', 'Recording',
    'MouseButton', 'DesktopType', 'RecordingQuality',
    'Point', 'SessionInfo', 'WorkflowResult',
    'run_async'
]

# Version info
__version__ = "0.1.0"
__author__ = "KVirtualStage Team"
__description__ = "Playwright-equivalent desktop automation platform for AI agents"