#!/usr/bin/env python3
"""
Visual Intent Engine for KVirtualStage

Provides sophisticated visual feedback and intent capture capabilities
for AI-driven desktop automation. This engine enables:

- Visible cursor movement with path indication
- Real-time interaction feedback for AI agents
- Intent capture and replay for learning
- Visual annotation of automation actions
- Human-like animation patterns

Key Features:
- Cursor trail visualization with smooth animations
- Click feedback with ripple effects
- Typing visualization with character highlighting
- Element highlighting and selection indicators
- Recording intent patterns for ML training
- Visual debugging for automation failures
"""

import asyncio
import cv2
import numpy as np
import time
import math
import logging
from typing import Tuple, List, Dict, Any, Optional
from dataclasses import dataclass
from pathlib import Path
import subprocess
import json

logger = logging.getLogger(__name__)

@dataclass
class CursorPathPoint:
    """Single point in cursor movement path"""
    x: float
    y: float
    timestamp: float
    velocity: float = 0.0
    pressure: float = 1.0  # For future tablet support

@dataclass
class VisualIntent:
    """Captures visual intent for AI learning"""
    intent_id: str
    intent_type: str  # click, type, drag, hover
    target_element: str
    visual_cues: List[str]
    path_points: List[CursorPathPoint]
    success: bool
    timestamp: float
    annotations: Dict[str, Any]

@dataclass
class AnimationFrame:
    """Single frame of visual animation"""
    frame_id: int
    timestamp: float
    elements: List[Dict[str, Any]]  # Visual elements to draw
    duration_ms: int = 50

class VisualFeedbackEngine:
    """
    Core engine for providing visual feedback during automation
    """
    
    def __init__(self, display: str = ":1"):
        self.display = display
        self.animation_enabled = True
        self.cursor_trail_enabled = True
        self.click_feedback_enabled = True
        self.typing_feedback_enabled = True
        
        # Animation state
        self.current_animations: List[AnimationFrame] = []
        self.cursor_path: List[CursorPathPoint] = []
        self.active_overlays: Dict[str, Any] = {}
        
        # Visual styling
        self.cursor_trail_color = (0, 150, 255, 180)  # Blue with transparency
        self.click_ripple_color = (255, 100, 100, 200)  # Red ripple
        self.typing_highlight_color = (100, 255, 100, 150)  # Green highlight
        self.element_highlight_color = (255, 255, 0, 180)  # Yellow outline
        
        # Animation parameters
        self.trail_fade_time = 2.0  # seconds
        self.ripple_max_radius = 50
        self.ripple_duration = 0.8  # seconds
        self.typing_highlight_duration = 0.3  # seconds
        
        logger.info("Visual Feedback Engine initialized")
    
    async def show_cursor_path(self, start_x: float, start_y: float, 
                             end_x: float, end_y: float, 
                             duration: float = 1.0, style: str = "smooth"):
        """
        Show visual cursor movement path
        
        Args:
            start_x, start_y: Starting coordinates
            end_x, end_y: Ending coordinates  
            duration: Animation duration in seconds
            style: Movement style (smooth, curved, stepped)
        """
        if not self.cursor_trail_enabled:
            return
        
        try:
            # Generate path points based on style
            if style == "smooth":
                path_points = self._generate_smooth_path(start_x, start_y, end_x, end_y, duration)
            elif style == "curved":
                path_points = self._generate_curved_path(start_x, start_y, end_x, end_y, duration)
            elif style == "stepped":
                path_points = self._generate_stepped_path(start_x, start_y, end_x, end_y, duration)
            else:
                path_points = self._generate_smooth_path(start_x, start_y, end_x, end_y, duration)
            
            # Add to cursor path
            self.cursor_path.extend(path_points)
            
            # Create overlay visualization
            overlay_id = f"cursor_path_{time.time()}"
            await self._create_path_overlay(overlay_id, path_points)
            
            # Animate the path
            await self._animate_cursor_path(path_points, duration)
            
            logger.debug(f"Displayed cursor path from ({start_x},{start_y}) to ({end_x},{end_y})")
            
        except Exception as e:
            logger.error(f"Failed to show cursor path: {e}")
    
    async def show_click_feedback(self, x: float, y: float, click_type: str = "left"):
        """
        Show visual feedback for click actions
        
        Args:
            x, y: Click coordinates
            click_type: Type of click (left, right, middle, double)
        """
        if not self.click_feedback_enabled:
            return
        
        try:
            # Create ripple effect
            ripple_id = f"click_ripple_{time.time()}"
            await self._create_click_ripple(ripple_id, x, y, click_type)
            
            # Add click marker
            marker_id = f"click_marker_{time.time()}"
            await self._create_click_marker(marker_id, x, y, click_type)
            
            logger.debug(f"Displayed click feedback at ({x},{y}) for {click_type} click")
            
        except Exception as e:
            logger.error(f"Failed to show click feedback: {e}")
    
    async def show_typing_feedback(self, text: str, position: Tuple[float, float], 
                                 char_index: int, total_chars: int):
        """
        Show visual feedback for typing actions
        
        Args:
            text: Text being typed
            position: Approximate text position
            char_index: Current character index
            total_chars: Total characters to type
        """
        if not self.typing_feedback_enabled:
            return
        
        try:
            # Show character highlight
            char = text[char_index] if char_index < len(text) else ""
            progress = char_index / max(total_chars, 1)
            
            highlight_id = f"typing_highlight_{time.time()}"
            await self._create_typing_highlight(highlight_id, position, char, progress)
            
            # Show typing progress indicator
            if char_index % 10 == 0:  # Update every 10 characters
                progress_id = f"typing_progress_{time.time()}"
                await self._create_typing_progress(progress_id, position, progress)
            
            logger.debug(f"Displayed typing feedback for character '{char}' ({char_index+1}/{total_chars})")
            
        except Exception as e:
            logger.error(f"Failed to show typing feedback: {e}")
    
    async def highlight_element(self, element_bounds: Tuple[float, float, float, float], 
                              highlight_type: str = "selection"):
        """
        Highlight UI element with visual indicator
        
        Args:
            element_bounds: (x, y, width, height) of element
            highlight_type: Type of highlight (selection, hover, error, success)
        """
        try:
            x, y, width, height = element_bounds
            
            # Choose highlight style based on type
            if highlight_type == "selection":
                color = self.element_highlight_color
                style = "outline"
            elif highlight_type == "hover":
                color = (100, 200, 255, 150)  # Light blue
                style = "glow"
            elif highlight_type == "error":
                color = (255, 0, 0, 200)  # Red
                style = "outline_thick"
            elif highlight_type == "success":
                color = (0, 255, 0, 180)  # Green
                style = "outline"
            else:
                color = self.element_highlight_color
                style = "outline"
            
            # Create highlight overlay
            highlight_id = f"element_highlight_{time.time()}"
            await self._create_element_highlight(highlight_id, element_bounds, color, style)
            
            logger.debug(f"Highlighted element at ({x},{y},{width},{height}) with {highlight_type}")
            
        except Exception as e:
            logger.error(f"Failed to highlight element: {e}")
    
    async def show_intent_visualization(self, intent: VisualIntent):
        """
        Show comprehensive visualization for captured intent
        
        Args:
            intent: Visual intent data to visualize
        """
        try:
            # Show path if available
            if intent.path_points:
                await self._visualize_intent_path(intent)
            
            # Show target highlighting
            await self._visualize_intent_target(intent)
            
            # Show intent annotations
            await self._visualize_intent_annotations(intent)
            
            logger.info(f"Visualized intent {intent.intent_id} of type {intent.intent_type}")
            
        except Exception as e:
            logger.error(f"Failed to visualize intent: {e}")
    
    # Path generation methods
    
    def _generate_smooth_path(self, start_x: float, start_y: float, 
                            end_x: float, end_y: float, duration: float) -> List[CursorPathPoint]:
        """Generate smooth linear path between points"""
        points = []
        steps = max(int(duration * 60), 10)  # 60 FPS target
        
        for i in range(steps + 1):
            progress = i / steps
            
            # Ease-in-out interpolation
            if progress < 0.5:
                eased = 2 * progress * progress
            else:
                eased = 1 - 2 * (1 - progress) * (1 - progress)
            
            x = start_x + (end_x - start_x) * eased
            y = start_y + (end_y - start_y) * eased
            timestamp = time.time() + (progress * duration)
            
            # Calculate velocity
            if i > 0:
                prev_point = points[-1]
                dx = x - prev_point.x
                dy = y - prev_point.y
                dt = timestamp - prev_point.timestamp
                velocity = math.sqrt(dx*dx + dy*dy) / max(dt, 0.001)
            else:
                velocity = 0.0
            
            points.append(CursorPathPoint(x, y, timestamp, velocity))
        
        return points
    
    def _generate_curved_path(self, start_x: float, start_y: float, 
                            end_x: float, end_y: float, duration: float) -> List[CursorPathPoint]:
        """Generate curved path with natural human-like movement"""
        points = []
        steps = max(int(duration * 60), 10)
        
        # Calculate curve control point (slight arc)
        mid_x = (start_x + end_x) / 2
        mid_y = (start_y + end_y) / 2
        
        # Add perpendicular offset for curve
        dx = end_x - start_x
        dy = end_y - start_y
        distance = math.sqrt(dx*dx + dy*dy)
        
        if distance > 0:
            # Normalize and rotate 90 degrees
            offset_x = -dy / distance * min(distance * 0.1, 50)  # Max 50px curve
            offset_y = dx / distance * min(distance * 0.1, 50)
            
            control_x = mid_x + offset_x
            control_y = mid_y + offset_y
        else:
            control_x = mid_x
            control_y = mid_y
        
        # Generate quadratic Bezier curve
        for i in range(steps + 1):
            t = i / steps
            
            # Quadratic Bezier formula
            x = (1-t)*(1-t)*start_x + 2*(1-t)*t*control_x + t*t*end_x
            y = (1-t)*(1-t)*start_y + 2*(1-t)*t*control_y + t*t*end_y
            
            timestamp = time.time() + (t * duration)
            
            # Calculate velocity
            if i > 0:
                prev_point = points[-1]
                dx = x - prev_point.x
                dy = y - prev_point.y
                dt = timestamp - prev_point.timestamp
                velocity = math.sqrt(dx*dx + dy*dy) / max(dt, 0.001)
            else:
                velocity = 0.0
            
            points.append(CursorPathPoint(x, y, timestamp, velocity))
        
        return points
    
    def _generate_stepped_path(self, start_x: float, start_y: float, 
                             end_x: float, end_y: float, duration: float) -> List[CursorPathPoint]:
        """Generate stepped path with brief pauses"""
        points = []
        steps = 5  # Fixed number of steps
        step_duration = duration / steps
        
        for i in range(steps + 1):
            progress = i / steps
            
            x = start_x + (end_x - start_x) * progress
            y = start_y + (end_y - start_y) * progress
            timestamp = time.time() + (progress * duration)
            
            points.append(CursorPathPoint(x, y, timestamp, 0.0))
            
            # Add pause point (except for last step)
            if i < steps:
                pause_timestamp = timestamp + step_duration * 0.2
                points.append(CursorPathPoint(x, y, pause_timestamp, 0.0))
        
        return points
    
    # Overlay creation methods
    
    async def _create_path_overlay(self, overlay_id: str, path_points: List[CursorPathPoint]):
        """Create visual overlay for cursor path"""
        # This would create an actual visual overlay
        # For now, we'll simulate with a simple implementation
        
        overlay_data = {
            "type": "cursor_path",
            "points": [(p.x, p.y) for p in path_points],
            "color": self.cursor_trail_color,
            "fade_time": self.trail_fade_time
        }
        
        self.active_overlays[overlay_id] = overlay_data
        
        # Schedule cleanup
        asyncio.create_task(self._cleanup_overlay(overlay_id, self.trail_fade_time))
    
    async def _create_click_ripple(self, ripple_id: str, x: float, y: float, click_type: str):
        """Create ripple effect for click"""
        ripple_data = {
            "type": "click_ripple",
            "center": (x, y),
            "max_radius": self.ripple_max_radius,
            "color": self.click_ripple_color,
            "duration": self.ripple_duration,
            "click_type": click_type
        }
        
        self.active_overlays[ripple_id] = ripple_data
        
        # Animate ripple expansion
        await self._animate_ripple(ripple_id, ripple_data)
    
    async def _create_click_marker(self, marker_id: str, x: float, y: float, click_type: str):
        """Create click position marker"""
        marker_data = {
            "type": "click_marker",
            "position": (x, y),
            "click_type": click_type,
            "duration": 1.0
        }
        
        self.active_overlays[marker_id] = marker_data
        
        # Schedule cleanup
        asyncio.create_task(self._cleanup_overlay(marker_id, 1.0))
    
    async def _create_typing_highlight(self, highlight_id: str, position: Tuple[float, float], 
                                     char: str, progress: float):
        """Create typing character highlight"""
        highlight_data = {
            "type": "typing_highlight",
            "position": position,
            "character": char,
            "progress": progress,
            "color": self.typing_highlight_color,
            "duration": self.typing_highlight_duration
        }
        
        self.active_overlays[highlight_id] = highlight_data
        
        # Schedule cleanup
        asyncio.create_task(self._cleanup_overlay(highlight_id, self.typing_highlight_duration))
    
    async def _create_typing_progress(self, progress_id: str, position: Tuple[float, float], 
                                    progress: float):
        """Create typing progress indicator"""
        progress_data = {
            "type": "typing_progress",
            "position": position,
            "progress": progress,
            "duration": 2.0
        }
        
        self.active_overlays[progress_id] = progress_data
        
        # Schedule cleanup
        asyncio.create_task(self._cleanup_overlay(progress_id, 2.0))
    
    async def _create_element_highlight(self, highlight_id: str, bounds: Tuple[float, float, float, float], 
                                      color: Tuple[int, int, int, int], style: str):
        """Create element highlight overlay"""
        highlight_data = {
            "type": "element_highlight",
            "bounds": bounds,
            "color": color,
            "style": style,
            "duration": 3.0
        }
        
        self.active_overlays[highlight_id] = highlight_data
        
        # Schedule cleanup
        asyncio.create_task(self._cleanup_overlay(highlight_id, 3.0))
    
    # Animation methods
    
    async def _animate_cursor_path(self, path_points: List[CursorPathPoint], duration: float):
        """Animate cursor movement along path"""
        if not path_points:
            return
        
        start_time = time.time()
        
        for point in path_points:
            # Wait until it's time for this point
            target_time = start_time + (point.timestamp - path_points[0].timestamp)
            current_time = time.time()
            
            if target_time > current_time:
                await asyncio.sleep(target_time - current_time)
            
            # Move cursor to this point (simulated)
            # In real implementation, this would move the actual cursor
            logger.debug(f"Cursor moved to ({point.x:.1f}, {point.y:.1f})")
    
    async def _animate_ripple(self, ripple_id: str, ripple_data: Dict[str, Any]):
        """Animate ripple effect"""
        duration = ripple_data["duration"]
        max_radius = ripple_data["max_radius"]
        
        steps = int(duration * 30)  # 30 FPS for smooth animation
        
        for i in range(steps):
            progress = i / steps
            current_radius = max_radius * progress
            alpha = int(255 * (1 - progress))  # Fade out
            
            # Update ripple data
            if ripple_id in self.active_overlays:
                self.active_overlays[ripple_id]["current_radius"] = current_radius
                self.active_overlays[ripple_id]["alpha"] = alpha
            
            await asyncio.sleep(duration / steps)
        
        # Remove ripple
        if ripple_id in self.active_overlays:
            del self.active_overlays[ripple_id]
    
    async def _visualize_intent_path(self, intent: VisualIntent):
        """Visualize path from captured intent"""
        if not intent.path_points:
            return
        
        # Create path visualization
        path_id = f"intent_path_{intent.intent_id}"
        await self._create_path_overlay(path_id, intent.path_points)
    
    async def _visualize_intent_target(self, intent: VisualIntent):
        """Visualize target element from intent"""
        # This would highlight the target element
        # For now, simulate with logged information
        logger.info(f"Intent target: {intent.target_element}")
    
    async def _visualize_intent_annotations(self, intent: VisualIntent):
        """Visualize intent annotations"""
        # This would show visual annotations
        # For now, log the annotations
        logger.info(f"Intent annotations: {intent.annotations}")
    
    async def _cleanup_overlay(self, overlay_id: str, delay: float):
        """Clean up overlay after delay"""
        await asyncio.sleep(delay)
        if overlay_id in self.active_overlays:
            del self.active_overlays[overlay_id]
            logger.debug(f"Cleaned up overlay: {overlay_id}")

class IntentCaptureEngine:
    """
    Engine for capturing and learning from user intents
    """
    
    def __init__(self):
        self.captured_intents: List[VisualIntent] = []
        self.learning_enabled = True
        self.intent_patterns: Dict[str, Any] = {}
        
        logger.info("Intent Capture Engine initialized")
    
    async def capture_intent(self, intent_type: str, target_element: str, 
                           path_points: List[CursorPathPoint] = None,
                           visual_cues: List[str] = None,
                           annotations: Dict[str, Any] = None) -> VisualIntent:
        """
        Capture a user intent for learning
        
        Args:
            intent_type: Type of intent (click, type, drag, etc.)
            target_element: Description of target element
            path_points: Cursor movement path
            visual_cues: Visual indicators that led to this intent
            annotations: Additional metadata
            
        Returns:
            Captured VisualIntent object
        """
        intent_id = f"intent_{int(time.time() * 1000)}"
        
        intent = VisualIntent(
            intent_id=intent_id,
            intent_type=intent_type,
            target_element=target_element,
            visual_cues=visual_cues or [],
            path_points=path_points or [],
            success=True,  # Assume success for now
            timestamp=time.time(),
            annotations=annotations or {}
        )
        
        if self.learning_enabled:
            self.captured_intents.append(intent)
            await self._analyze_intent_pattern(intent)
        
        logger.info(f"Captured intent {intent_id}: {intent_type} on {target_element}")
        
        return intent
    
    async def _analyze_intent_pattern(self, intent: VisualIntent):
        """Analyze intent for patterns and learning"""
        intent_type = intent.intent_type
        
        if intent_type not in self.intent_patterns:
            self.intent_patterns[intent_type] = {
                "count": 0,
                "success_rate": 0.0,
                "common_targets": {},
                "average_path_length": 0.0,
                "common_cues": {}
            }
        
        pattern = self.intent_patterns[intent_type]
        pattern["count"] += 1
        
        # Update success rate
        if intent.success:
            pattern["success_rate"] = (pattern["success_rate"] * (pattern["count"] - 1) + 1.0) / pattern["count"]
        else:
            pattern["success_rate"] = (pattern["success_rate"] * (pattern["count"] - 1)) / pattern["count"]
        
        # Track common targets
        target = intent.target_element
        if target in pattern["common_targets"]:
            pattern["common_targets"][target] += 1
        else:
            pattern["common_targets"][target] = 1
        
        # Track path characteristics
        if intent.path_points:
            path_length = len(intent.path_points)
            pattern["average_path_length"] = (
                (pattern["average_path_length"] * (pattern["count"] - 1) + path_length) / pattern["count"]
            )
        
        # Track visual cues
        for cue in intent.visual_cues:
            if cue in pattern["common_cues"]:
                pattern["common_cues"][cue] += 1
            else:
                pattern["common_cues"][cue] = 1
        
        logger.debug(f"Updated pattern analysis for {intent_type}")
    
    def get_intent_insights(self) -> Dict[str, Any]:
        """Get insights from captured intents"""
        return {
            "total_intents": len(self.captured_intents),
            "patterns": self.intent_patterns,
            "recent_intents": [
                {
                    "id": intent.intent_id,
                    "type": intent.intent_type,
                    "target": intent.target_element,
                    "success": intent.success,
                    "timestamp": intent.timestamp
                }
                for intent in self.captured_intents[-10:]  # Last 10
            ]
        }
    
    async def export_learning_data(self, filepath: str):
        """Export captured intents for ML training"""
        export_data = {
            "metadata": {
                "export_timestamp": time.time(),
                "total_intents": len(self.captured_intents),
                "patterns": self.intent_patterns
            },
            "intents": [
                {
                    "intent_id": intent.intent_id,
                    "intent_type": intent.intent_type,
                    "target_element": intent.target_element,
                    "visual_cues": intent.visual_cues,
                    "path_points": [
                        {
                            "x": p.x,
                            "y": p.y,
                            "timestamp": p.timestamp,
                            "velocity": p.velocity
                        }
                        for p in intent.path_points
                    ],
                    "success": intent.success,
                    "timestamp": intent.timestamp,
                    "annotations": intent.annotations
                }
                for intent in self.captured_intents
            ]
        }
        
        with open(filepath, 'w') as f:
            json.dump(export_data, f, indent=2)
        
        logger.info(f"Exported {len(self.captured_intents)} intents to {filepath}")

class VisualIntentSystem:
    """
    Complete visual intent system combining feedback and capture
    """
    
    def __init__(self, display: str = ":1"):
        self.feedback_engine = VisualFeedbackEngine(display)
        self.capture_engine = IntentCaptureEngine()
        
        logger.info("Visual Intent System initialized")
    
    async def execute_with_visual_intent(self, intent_type: str, target_element: str,
                                       start_pos: Tuple[float, float],
                                       end_pos: Tuple[float, float],
                                       execution_func,
                                       visual_cues: List[str] = None):
        """
        Execute an action with full visual intent capture and feedback
        
        Args:
            intent_type: Type of intent being executed
            target_element: Target element description
            start_pos: Starting position
            end_pos: Ending position
            execution_func: Function to execute the actual action
            visual_cues: Visual cues that led to this action
        """
        try:
            # Show cursor path
            await self.feedback_engine.show_cursor_path(
                start_pos[0], start_pos[1], end_pos[0], end_pos[1]
            )
            
            # Capture path points
            path_points = self.feedback_engine.cursor_path.copy()
            
            # Execute the action
            result = await execution_func()
            
            # Show appropriate feedback
            if intent_type == "click":
                await self.feedback_engine.show_click_feedback(end_pos[0], end_pos[1])
            
            # Capture intent
            intent = await self.capture_engine.capture_intent(
                intent_type=intent_type,
                target_element=target_element,
                path_points=path_points,
                visual_cues=visual_cues,
                annotations={"execution_result": result}
            )
            
            # Visualize captured intent
            await self.feedback_engine.show_intent_visualization(intent)
            
            logger.info(f"Executed action with visual intent: {intent_type} on {target_element}")
            
            return result
            
        except Exception as e:
            logger.error(f"Failed to execute action with visual intent: {e}")
            raise

# Example usage and testing
async def demo_visual_intent():
    """Demonstrate visual intent capabilities"""
    system = VisualIntentSystem()
    
    print("🎨 Visual Intent System Demo")
    print("============================")
    
    # Demo cursor path
    await system.feedback_engine.show_cursor_path(100, 100, 500, 300, duration=2.0, style="curved")
    
    # Demo click feedback
    await system.feedback_engine.show_click_feedback(500, 300, "left")
    
    # Demo element highlighting
    await system.feedback_engine.highlight_element((450, 250, 100, 50), "selection")
    
    # Demo intent capture
    intent = await system.capture_engine.capture_intent(
        intent_type="click",
        target_element="submit button",
        visual_cues=["blue color", "raised appearance", "click affordance"]
    )
    
    # Show insights
    insights = system.capture_engine.get_intent_insights()
    print(f"Intent insights: {insights}")
    
    print("✅ Visual intent demo completed!")

if __name__ == "__main__":
    logging.basicConfig(level=logging.INFO)
    asyncio.run(demo_visual_intent())