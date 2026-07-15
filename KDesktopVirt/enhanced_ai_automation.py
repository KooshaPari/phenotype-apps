#!/usr/bin/env python3
"""
Enhanced AI Automation for KVirtualStage
Combines UI-TARS vision capabilities with Open-Interface LLM control
and KVirtualStage pixel-perfect execution
"""

import asyncio
import base64
import json
import logging
from typing import Dict, List, Optional, Tuple, Any
from dataclasses import dataclass
from accurate_automation import AccurateAutomation
import openai
from PIL import Image
import io

logger = logging.getLogger(__name__)

@dataclass
class UIElement:
    """Enhanced UI element with AI understanding"""
    name: str
    element_type: str
    coordinates: Tuple[int, int]
    confidence: float
    description: str
    ai_context: Optional[Dict[str, Any]] = None

@dataclass
class AutomationAction:
    """Action with AI reasoning"""
    action_type: str  # click, type, scroll, wait
    target: Optional[UIElement] = None
    text: Optional[str] = None
    coordinates: Optional[Tuple[int, int]] = None
    reasoning: Optional[str] = None
    metadata: Optional[Dict[str, Any]] = None

class VisionLanguageModel:
    """UI-TARS inspired vision model for UI understanding"""
    
    def __init__(self, model_name: str = "gpt-4-vision-preview"):
        self.model_name = model_name
        self.client = openai.AsyncOpenAI()
    
    async def analyze_screenshot(self, screenshot_bytes: bytes, task_description: str) -> Dict[str, Any]:
        """Analyze screenshot for UI elements and automation opportunities"""
        
        # Convert screenshot to base64
        image_b64 = base64.b64encode(screenshot_bytes).decode('utf-8')
        
        system_prompt = """You are a desktop automation expert. Analyze this screenshot and identify:
1. All interactive UI elements (buttons, text fields, menus, etc.)
2. Their approximate locations (describe position relative to screen)
3. Current state of applications
4. Suggested actions to accomplish the given task
5. Any potential automation challenges

Respond in JSON format with:
{
    "ui_elements": [
        {
            "type": "button|textfield|menu|window",
            "text": "visible text or label",
            "position": "top-left|center|bottom-right|etc",
            "description": "detailed description",
            "interactable": true/false
        }
    ],
    "current_state": "description of what's currently visible",
    "suggested_actions": [
        {
            "action": "click|type|scroll|wait",
            "target": "element description",
            "reasoning": "why this action is needed"
        }
    ],
    "automation_challenges": ["potential issues"]
}"""

        try:
            response = await self.client.chat.completions.create(
                model=self.model_name,
                messages=[
                    {
                        "role": "system",
                        "content": system_prompt
                    },
                    {
                        "role": "user",
                        "content": [
                            {
                                "type": "text",
                                "text": f"Task: {task_description}\n\nAnalyze this screenshot and provide automation guidance:"
                            },
                            {
                                "type": "image_url",
                                "image_url": {
                                    "url": f"data:image/png;base64,{image_b64}"
                                }
                            }
                        ]
                    }
                ],
                max_tokens=2000
            )
            
            content = response.choices[0].message.content
            
            # Try to parse JSON response
            try:
                return json.loads(content)
            except json.JSONDecodeError:
                # If JSON parsing fails, return structured fallback
                return {
                    "ui_elements": [],
                    "current_state": content,
                    "suggested_actions": [],
                    "automation_challenges": ["Failed to parse structured response"]
                }
                
        except Exception as e:
            logger.error(f"Vision model analysis failed: {e}")
            return {
                "ui_elements": [],
                "current_state": "Analysis failed",
                "suggested_actions": [],
                "automation_challenges": [str(e)]
            }

class LLMAutomationPlanner:
    """Open-Interface inspired LLM for automation planning"""
    
    def __init__(self, model_name: str = "gpt-4"):
        self.model_name = model_name
        self.client = openai.AsyncOpenAI()
        self.conversation_history = []
    
    async def plan_automation(self, 
                            task: str, 
                            ui_analysis: Dict[str, Any], 
                            previous_actions: List[AutomationAction] = None) -> List[AutomationAction]:
        """Plan automation steps based on task and UI analysis"""
        
        system_prompt = """You are an expert desktop automation planner. Given a task and UI analysis, create a detailed step-by-step automation plan.

Consider:
1. The current state of the desktop
2. Available UI elements
3. Any previous actions taken
4. Potential errors or edge cases

Respond with a JSON array of actions:
[
    {
        "action_type": "click|type|scroll|wait|screenshot",
        "target_description": "element to interact with",
        "text": "text to type (if applicable)",
        "reasoning": "why this action is needed",
        "expected_outcome": "what should happen",
        "error_recovery": "what to do if this fails"
    }
]

Be specific about target elements and provide clear reasoning."""

        user_message = f"""
Task: {task}

Current UI Analysis:
{json.dumps(ui_analysis, indent=2)}

Previous Actions: {json.dumps([action.__dict__ for action in (previous_actions or [])], indent=2)}

Create an automation plan to accomplish this task."""

        try:
            response = await self.client.chat.completions.create(
                model=self.model_name,
                messages=[
                    {"role": "system", "content": system_prompt},
                    {"role": "user", "content": user_message}
                ],
                max_tokens=1500
            )
            
            content = response.choices[0].message.content
            
            # Parse JSON response
            try:
                action_data = json.loads(content)
                actions = []
                
                for item in action_data:
                    action = AutomationAction(
                        action_type=item.get("action_type", "wait"),
                        text=item.get("text"),
                        reasoning=item.get("reasoning", ""),
                        metadata={
                            "target_description": item.get("target_description", ""),
                            "expected_outcome": item.get("expected_outcome", ""),
                            "error_recovery": item.get("error_recovery", "")
                        }
                    )
                    actions.append(action)
                
                return actions
                
            except json.JSONDecodeError:
                logger.error(f"Failed to parse LLM response: {content}")
                return []
                
        except Exception as e:
            logger.error(f"LLM planning failed: {e}")
            return []

class EnhancedAIAutomation:
    """Main class combining AI intelligence with pixel-perfect execution"""
    
    def __init__(self):
        self.vision_model = VisionLanguageModel()
        self.llm_planner = LLMAutomationPlanner()
        self.accurate_automation = AccurateAutomation()
        self.execution_history = []
        
    async def execute_natural_language_task(self, task: str) -> Dict[str, Any]:
        """Execute a task described in natural language"""
        
        logger.info(f"🎯 Starting AI-enhanced automation: {task}")
        
        results = {
            "task": task,
            "success": False,
            "steps_completed": 0,
            "total_steps": 0,
            "errors": [],
            "execution_log": []
        }
        
        try:
            # Step 1: Take initial screenshot for AI analysis
            logger.info("📸 Taking screenshot for AI analysis...")
            screenshot_path = "/tmp/ai_automation_analysis.png"
            self.accurate_automation.take_screenshot(screenshot_path)
            
            with open(screenshot_path, 'rb') as f:
                screenshot_bytes = f.read()
            
            # Step 2: AI vision analysis
            logger.info("🧠 Analyzing UI with vision model...")
            ui_analysis = await self.vision_model.analyze_screenshot(screenshot_bytes, task)
            results["execution_log"].append(f"UI Analysis: {ui_analysis.get('current_state', 'Unknown')}")
            
            # Step 3: LLM planning
            logger.info("📋 Planning automation steps with LLM...")
            planned_actions = await self.llm_planner.plan_automation(task, ui_analysis)
            results["total_steps"] = len(planned_actions)
            
            if not planned_actions:
                results["errors"].append("No automation plan generated")
                return results
            
            # Step 4: Execute actions with pixel-perfect precision
            logger.info(f"🚀 Executing {len(planned_actions)} planned actions...")
            
            for i, action in enumerate(planned_actions):
                try:
                    logger.info(f"Step {i+1}/{len(planned_actions)}: {action.reasoning}")
                    
                    success = await self._execute_action_with_precision(action)
                    
                    if success:
                        results["steps_completed"] += 1
                        results["execution_log"].append(f"✅ Step {i+1}: {action.reasoning}")
                    else:
                        results["errors"].append(f"❌ Step {i+1} failed: {action.reasoning}")
                        
                        # Try error recovery if available
                        if action.metadata and action.metadata.get("error_recovery"):
                            logger.info(f"🔄 Attempting error recovery: {action.metadata['error_recovery']}")
                            # Could implement recovery logic here
                    
                    # Take screenshot after each step for progress tracking
                    progress_screenshot = f"/tmp/ai_automation_step_{i+1}.png"
                    self.accurate_automation.take_screenshot(progress_screenshot)
                    
                    # Brief pause for natural timing
                    await asyncio.sleep(1)
                    
                except Exception as e:
                    error_msg = f"Exception in step {i+1}: {str(e)}"
                    logger.error(error_msg)
                    results["errors"].append(error_msg)
            
            # Final success determination
            results["success"] = results["steps_completed"] == results["total_steps"]
            
            if results["success"]:
                logger.info("🏆 AI-enhanced automation completed successfully!")
            else:
                logger.warning(f"⚠️ Automation partially completed: {results['steps_completed']}/{results['total_steps']} steps")
            
        except Exception as e:
            error_msg = f"AI automation failed: {str(e)}"
            logger.error(error_msg)
            results["errors"].append(error_msg)
        
        return results
    
    async def _execute_action_with_precision(self, action: AutomationAction) -> bool:
        """Execute an action using pixel-perfect automation"""
        
        try:
            if action.action_type == "click":
                return await self._execute_click_action(action)
            elif action.action_type == "type":
                return await self._execute_type_action(action)
            elif action.action_type == "wait":
                return await self._execute_wait_action(action)
            elif action.action_type == "screenshot":
                return await self._execute_screenshot_action(action)
            else:
                logger.warning(f"Unknown action type: {action.action_type}")
                return False
                
        except Exception as e:
            logger.error(f"Action execution failed: {e}")
            return False
    
    async def _execute_click_action(self, action: AutomationAction) -> bool:
        """Execute click with AI target finding + precise clicking"""
        
        target_desc = action.metadata.get("target_description", "") if action.metadata else ""
        
        if "calculator" in target_desc.lower():
            # Use our proven calculator automation
            if not self.accurate_automation.wait_for_application('galculator'):
                # Launch calculator if not running
                import subprocess
                subprocess.Popen(['galculator'])
                if not self.accurate_automation.wait_for_application('galculator'):
                    return False
            
            # Get window and calculate positions
            window_info = self.accurate_automation.find_window_info('galculator')
            if not window_info:
                return False
                
            buttons = self.accurate_automation.calculate_galculator_buttons(window_info)
            
            # Find target button from description
            for button_name, coords in buttons.items():
                if button_name in target_desc or target_desc in button_name:
                    self.accurate_automation.precise_click(*coords, f"Calculator button: {button_name}")
                    return True
        
        elif "text" in target_desc.lower() or "editor" in target_desc.lower():
            # Handle text editor clicking
            if not self.accurate_automation.wait_for_application('mousepad'):
                import subprocess
                subprocess.Popen(['mousepad'])
                if not self.accurate_automation.wait_for_application('mousepad'):
                    return False
            
            window_info = self.accurate_automation.find_window_info('mousepad')
            if window_info:
                # Click in center of text area
                text_x = window_info['x'] + window_info['width'] // 2
                text_y = window_info['y'] + window_info['height'] // 2 + 20
                self.accurate_automation.precise_click(text_x, text_y, "Text editor area")
                return True
        
        # Fallback: try to use coordinates if available
        if action.coordinates:
            self.accurate_automation.precise_click(*action.coordinates, target_desc)
            return True
        
        logger.warning(f"Could not execute click action: {target_desc}")
        return False
    
    async def _execute_type_action(self, action: AutomationAction) -> bool:
        """Execute typing with natural rhythm"""
        
        if not action.text:
            return False
        
        try:
            # Type with natural timing using xdotool
            import subprocess
            for char in action.text:
                if char == '\n':
                    subprocess.run(['xdotool', 'key', 'Return'])
                    await asyncio.sleep(0.2)
                else:
                    subprocess.run(['xdotool', 'type', '--delay', '50', char])
                    # Vary typing speed naturally
                    if char in '.,!?':
                        await asyncio.sleep(0.1)
                    elif char == ' ':
                        await asyncio.sleep(0.05)
            
            return True
            
        except Exception as e:
            logger.error(f"Typing failed: {e}")
            return False
    
    async def _execute_wait_action(self, action: AutomationAction) -> bool:
        """Execute wait with optional application verification"""
        
        wait_time = 2  # Default wait time
        if action.metadata and "wait_time" in action.metadata:
            wait_time = action.metadata["wait_time"]
        
        await asyncio.sleep(wait_time)
        return True
    
    async def _execute_screenshot_action(self, action: AutomationAction) -> bool:
        """Execute screenshot with specified path"""
        
        screenshot_path = "/tmp/ai_automation_screenshot.png"
        if action.metadata and "path" in action.metadata:
            screenshot_path = action.metadata["path"]
        
        self.accurate_automation.take_screenshot(screenshot_path)
        return True

async def demo_ai_enhanced_automation():
    """Demonstration of AI-enhanced automation capabilities"""
    
    automation = EnhancedAIAutomation()
    
    # Test tasks demonstrating AI + precision combination
    test_tasks = [
        "Open a calculator and compute 8 times 7",
        "Open a text editor and write a summary of the calculation",
        "Take a screenshot to document the completed automation"
    ]
    
    for task in test_tasks:
        print(f"\n🎯 Executing task: {task}")
        result = await automation.execute_natural_language_task(task)
        
        print(f"✅ Success: {result['success']}")
        print(f"📊 Steps: {result['steps_completed']}/{result['total_steps']}")
        
        if result['errors']:
            print(f"❌ Errors: {result['errors']}")
        
        for log_entry in result['execution_log']:
            print(f"📝 {log_entry}")

if __name__ == "__main__":
    # Configure logging
    logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
    
    # Run demonstration
    asyncio.run(demo_ai_enhanced_automation())