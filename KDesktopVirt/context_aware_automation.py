#!/usr/bin/env python3
"""
Context-Aware Automation Engine for KVirtualStage
Integrates all natural automation components with intelligent workflow generation
Provides intent-based automation with adaptive learning
"""

import asyncio
import json
import time
import logging
from typing import Dict, List, Optional, Any, Tuple
from dataclasses import dataclass, field, asdict
from enum import Enum
import cv2
import numpy as np
import subprocess
from pathlib import Path

# Import our automation components
from natural_ui_automation import (
    NaturalUIAutomation, InteractionGesture, WindMouse2, 
    NaturalTyping, MultiModalDetection, GestureCoordination
)
from advanced_vision_detection import (
    AdvancedTemplateMatching, SemanticUIAnalyzer, 
    SelfHealingAutomation, LayoutAnalyzer, VisualElement
)

logger = logging.getLogger(__name__)

class AutomationIntent(Enum):
    """High-level automation intents"""
    NAVIGATE = "navigate"
    INPUT_DATA = "input_data"
    EXTRACT_INFO = "extract_info"
    VALIDATE = "validate"
    CONFIGURE = "configure"
    EXECUTE_TASK = "execute_task"
    MONITOR = "monitor"

@dataclass
class ContextualAction:
    """Action with context awareness"""
    intent: AutomationIntent
    target_description: str
    action_type: str
    parameters: Dict[str, Any] = field(default_factory=dict)
    preconditions: List[str] = field(default_factory=list)
    postconditions: List[str] = field(default_factory=list)
    confidence_requirement: float = 0.7
    retry_policy: Dict[str, Any] = field(default_factory=dict)
    context_metadata: Dict[str, Any] = field(default_factory=dict)

@dataclass
class AutomationContext:
    """Current automation context state"""
    current_application: Optional[str] = None
    active_window: Optional[str] = None
    ui_state: Dict[str, Any] = field(default_factory=dict)
    user_intent: Optional[AutomationIntent] = None
    workflow_progress: Dict[str, Any] = field(default_factory=dict)
    learned_patterns: Dict[str, Any] = field(default_factory=dict)
    error_history: List[Dict[str, Any]] = field(default_factory=list)
    performance_metrics: Dict[str, Any] = field(default_factory=dict)

@dataclass
class WorkflowStep:
    """Individual workflow step with context"""
    step_id: str
    intent: AutomationIntent
    description: str
    actions: List[ContextualAction]
    success_criteria: List[str]
    failure_recovery: List[ContextualAction] = field(default_factory=list)
    estimated_duration: float = 5.0
    dependencies: List[str] = field(default_factory=list)

@dataclass
class IntelligentWorkflow:
    """Complete intelligent workflow"""
    workflow_id: str
    name: str
    description: str
    target_goal: str
    steps: List[WorkflowStep]
    context_requirements: Dict[str, Any] = field(default_factory=dict)
    adaptation_rules: Dict[str, Any] = field(default_factory=dict)
    success_metrics: Dict[str, Any] = field(default_factory=dict)

class IntentRecognizer:
    """Recognize user intent from natural language descriptions"""
    
    def __init__(self):
        self.intent_patterns = self._load_intent_patterns()
        self.context_clues = self._load_context_clues()
    
    def _load_intent_patterns(self) -> Dict[AutomationIntent, List[str]]:
        """Load patterns for intent recognition"""
        return {
            AutomationIntent.NAVIGATE: [
                "open", "launch", "go to", "navigate", "switch to", "find", "locate"
            ],
            AutomationIntent.INPUT_DATA: [
                "type", "enter", "input", "fill", "write", "paste", "set"
            ],
            AutomationIntent.EXTRACT_INFO: [
                "get", "read", "extract", "copy", "capture", "retrieve", "find"
            ],
            AutomationIntent.VALIDATE: [
                "check", "verify", "confirm", "validate", "ensure", "test"
            ],
            AutomationIntent.CONFIGURE: [
                "configure", "setup", "adjust", "modify", "change", "customize"
            ],
            AutomationIntent.EXECUTE_TASK: [
                "run", "execute", "perform", "do", "process", "calculate", "compute"
            ],
            AutomationIntent.MONITOR: [
                "watch", "monitor", "observe", "track", "wait for", "listen"
            ]
        }
    
    def _load_context_clues(self) -> Dict[str, List[str]]:
        """Load context clues for better intent recognition"""
        return {
            "applications": [
                "calculator", "text editor", "browser", "terminal", "file manager"
            ],
            "ui_elements": [
                "button", "field", "menu", "dialog", "window", "tab", "link"
            ],
            "data_types": [
                "text", "number", "file", "image", "document", "email"
            ],
            "actions": [
                "click", "double-click", "right-click", "drag", "scroll", "hover"
            ]
        }
    
    def recognize_intent(self, description: str, 
                        current_context: AutomationContext = None) -> AutomationIntent:
        """Recognize intent from natural language description"""
        
        description_lower = description.lower()
        intent_scores = {}
        
        # Score each intent based on pattern matching
        for intent, patterns in self.intent_patterns.items():
            score = 0
            for pattern in patterns:
                if pattern in description_lower:
                    score += 1
            
            # Boost score based on context
            if current_context:
                score = self._apply_context_boost(score, intent, description_lower, current_context)
            
            intent_scores[intent] = score
        
        # Return intent with highest score
        if intent_scores:
            best_intent = max(intent_scores.items(), key=lambda x: x[1])
            if best_intent[1] > 0:
                return best_intent[0]
        
        # Default intent
        return AutomationIntent.EXECUTE_TASK
    
    def _apply_context_boost(self, base_score: float, intent: AutomationIntent,
                           description: str, context: AutomationContext) -> float:
        """Apply context-based scoring boost"""
        
        boost = 0
        
        # Application context boost
        if context.current_application:
            if context.current_application.lower() in description:
                boost += 0.5
        
        # UI state context boost
        if context.ui_state:
            # If we're in a form, boost INPUT_DATA intent
            if intent == AutomationIntent.INPUT_DATA and "form" in str(context.ui_state):
                boost += 0.3
            
            # If there are errors, boost VALIDATE intent
            if intent == AutomationIntent.VALIDATE and "error" in str(context.ui_state):
                boost += 0.3
        
        return base_score + boost

class WorkflowGenerator:
    """Generate intelligent workflows from high-level goals"""
    
    def __init__(self):
        self.intent_recognizer = IntentRecognizer()
        self.workflow_templates = self._load_workflow_templates()
        self.action_library = self._build_action_library()
    
    def _load_workflow_templates(self) -> Dict[str, Any]:
        """Load pre-defined workflow templates"""
        return {
            "calculator_operation": {
                "description": "Perform calculation using calculator",
                "steps": [
                    {"intent": "NAVIGATE", "action": "launch_calculator"},
                    {"intent": "INPUT_DATA", "action": "enter_calculation"},
                    {"intent": "EXECUTE_TASK", "action": "compute_result"},
                    {"intent": "EXTRACT_INFO", "action": "capture_result"}
                ]
            },
            "text_document_creation": {
                "description": "Create and edit text document",
                "steps": [
                    {"intent": "NAVIGATE", "action": "launch_text_editor"},
                    {"intent": "INPUT_DATA", "action": "type_content"},
                    {"intent": "CONFIGURE", "action": "format_document"},
                    {"intent": "EXECUTE_TASK", "action": "save_document"}
                ]
            },
            "data_extraction": {
                "description": "Extract information from UI",
                "steps": [
                    {"intent": "NAVIGATE", "action": "locate_source"},
                    {"intent": "EXTRACT_INFO", "action": "capture_data"},
                    {"intent": "VALIDATE", "action": "verify_extraction"},
                    {"intent": "EXECUTE_TASK", "action": "process_data"}
                ]
            }
        }
    
    def _build_action_library(self) -> Dict[str, Dict[str, Any]]:
        """Build library of contextual actions"""
        return {
            "launch_calculator": {
                "type": "application_launch",
                "target": "calculator",
                "gesture": "hover_click",
                "preconditions": [],
                "postconditions": ["calculator_visible"]
            },
            "enter_calculation": {
                "type": "input_sequence",
                "target": "calculator_buttons",
                "gesture": "precise_click",
                "preconditions": ["calculator_visible"],
                "postconditions": ["calculation_entered"]
            },
            "compute_result": {
                "type": "trigger_computation",
                "target": "equals_button",
                "gesture": "hover_click",
                "preconditions": ["calculation_entered"],
                "postconditions": ["result_displayed"]
            },
            "capture_result": {
                "type": "data_extraction",
                "target": "result_display",
                "method": "ocr",
                "preconditions": ["result_displayed"],
                "postconditions": ["result_captured"]
            },
            "launch_text_editor": {
                "type": "application_launch",
                "target": "text_editor",
                "gesture": "hover_click",
                "preconditions": [],
                "postconditions": ["text_editor_visible"]
            },
            "type_content": {
                "type": "text_input",
                "target": "text_area",
                "method": "natural_typing",
                "preconditions": ["text_editor_visible"],
                "postconditions": ["content_typed"]
            }
        }
    
    async def generate_workflow(self, goal_description: str,
                              context: AutomationContext = None) -> IntelligentWorkflow:
        """Generate intelligent workflow from goal description"""
        
        try:
            # Recognize primary intent
            primary_intent = self.intent_recognizer.recognize_intent(goal_description, context)
            
            # Find matching template or create custom workflow
            workflow = await self._create_workflow_from_intent(
                goal_description, primary_intent, context
            )
            
            # Optimize workflow based on context
            optimized_workflow = await self._optimize_workflow(workflow, context)
            
            return optimized_workflow
            
        except Exception as e:
            logger.error(f"Workflow generation failed: {e}")
            # Return minimal fallback workflow
            return self._create_fallback_workflow(goal_description)
    
    async def _create_workflow_from_intent(self, description: str,
                                         intent: AutomationIntent,
                                         context: AutomationContext) -> IntelligentWorkflow:
        """Create workflow based on recognized intent"""
        
        workflow_id = f"workflow_{int(time.time())}"
        
        # Analyze description for specific elements
        elements = self._extract_elements_from_description(description)
        
        # Generate steps based on intent and elements
        steps = await self._generate_steps_for_intent(intent, elements, context)
        
        workflow = IntelligentWorkflow(
            workflow_id=workflow_id,
            name=f"Generated Workflow: {description}",
            description=description,
            target_goal=description,
            steps=steps,
            context_requirements={"intent": intent.value},
            adaptation_rules=self._create_adaptation_rules(intent),
            success_metrics={"completion_rate": 0.9, "error_threshold": 0.1}
        )
        
        return workflow
    
    def _extract_elements_from_description(self, description: str) -> Dict[str, List[str]]:
        """Extract specific elements from description"""
        
        elements = {
            "applications": [],
            "targets": [],
            "data": [],
            "actions": []
        }
        
        description_lower = description.lower()
        
        # Extract applications
        for app in self.intent_recognizer.context_clues["applications"]:
            if app in description_lower:
                elements["applications"].append(app)
        
        # Extract UI elements
        for element in self.intent_recognizer.context_clues["ui_elements"]:
            if element in description_lower:
                elements["targets"].append(element)
        
        # Extract data types
        for data_type in self.intent_recognizer.context_clues["data_types"]:
            if data_type in description_lower:
                elements["data"].append(data_type)
        
        # Extract actions
        for action in self.intent_recognizer.context_clues["actions"]:
            if action in description_lower:
                elements["actions"].append(action)
        
        return elements
    
    async def _generate_steps_for_intent(self, intent: AutomationIntent,
                                       elements: Dict[str, List[str]],
                                       context: AutomationContext) -> List[WorkflowStep]:
        """Generate workflow steps for specific intent"""
        
        steps = []
        
        if intent == AutomationIntent.NAVIGATE:
            steps.extend(await self._create_navigation_steps(elements))
        elif intent == AutomationIntent.INPUT_DATA:
            steps.extend(await self._create_input_steps(elements))
        elif intent == AutomationIntent.EXTRACT_INFO:
            steps.extend(await self._create_extraction_steps(elements))
        elif intent == AutomationIntent.EXECUTE_TASK:
            steps.extend(await self._create_execution_steps(elements))
        else:
            steps.extend(await self._create_generic_steps(intent, elements))
        
        return steps
    
    async def _create_navigation_steps(self, elements: Dict[str, List[str]]) -> List[WorkflowStep]:
        """Create navigation-focused steps"""
        
        steps = []
        
        # If application specified, launch it
        if elements["applications"]:
            app = elements["applications"][0]
            
            launch_action = ContextualAction(
                intent=AutomationIntent.NAVIGATE,
                target_description=app,
                action_type="application_launch",
                parameters={"application": app},
                postconditions=[f"{app}_visible"]
            )
            
            step = WorkflowStep(
                step_id="nav_01",
                intent=AutomationIntent.NAVIGATE,
                description=f"Launch {app}",
                actions=[launch_action],
                success_criteria=[f"{app}_visible"]
            )
            steps.append(step)
        
        # If specific target, navigate to it
        if elements["targets"]:
            target = elements["targets"][0]
            
            locate_action = ContextualAction(
                intent=AutomationIntent.NAVIGATE,
                target_description=target,
                action_type="locate_element",
                parameters={"target": target},
                postconditions=[f"{target}_located"]
            )
            
            step = WorkflowStep(
                step_id="nav_02",
                intent=AutomationIntent.NAVIGATE,
                description=f"Locate {target}",
                actions=[locate_action],
                success_criteria=[f"{target}_located"]
            )
            steps.append(step)
        
        return steps
    
    async def _create_input_steps(self, elements: Dict[str, List[str]]) -> List[WorkflowStep]:
        """Create input-focused steps"""
        
        steps = []
        
        # Preparation step
        prep_action = ContextualAction(
            intent=AutomationIntent.NAVIGATE,
            target_description="input area",
            action_type="locate_input",
            parameters={"target_type": "input_field"},
            postconditions=["input_ready"]
        )
        
        prep_step = WorkflowStep(
            step_id="input_01",
            intent=AutomationIntent.NAVIGATE,
            description="Locate input area",
            actions=[prep_action],
            success_criteria=["input_ready"]
        )
        steps.append(prep_step)
        
        # Input step
        input_action = ContextualAction(
            intent=AutomationIntent.INPUT_DATA,
            target_description="input field",
            action_type="natural_typing",
            parameters={"text": "{{user_input}}", "allow_mistakes": True},
            preconditions=["input_ready"],
            postconditions=["data_entered"]
        )
        
        input_step = WorkflowStep(
            step_id="input_02",
            intent=AutomationIntent.INPUT_DATA,
            description="Enter data naturally",
            actions=[input_action],
            success_criteria=["data_entered"]
        )
        steps.append(input_step)
        
        return steps
    
    async def _create_extraction_steps(self, elements: Dict[str, List[str]]) -> List[WorkflowStep]:
        """Create information extraction steps"""
        
        steps = []
        
        # Locate source step
        locate_action = ContextualAction(
            intent=AutomationIntent.NAVIGATE,
            target_description="data source",
            action_type="locate_element",
            parameters={"target_type": "text"},
            postconditions=["source_located"]
        )
        
        locate_step = WorkflowStep(
            step_id="extract_01",
            intent=AutomationIntent.NAVIGATE,
            description="Locate data source",
            actions=[locate_action],
            success_criteria=["source_located"]
        )
        steps.append(locate_step)
        
        # Extract data step
        extract_action = ContextualAction(
            intent=AutomationIntent.EXTRACT_INFO,
            target_description="text content",
            action_type="data_extraction",
            parameters={"method": "ocr", "validation": True},
            preconditions=["source_located"],
            postconditions=["data_extracted"]
        )
        
        extract_step = WorkflowStep(
            step_id="extract_02",
            intent=AutomationIntent.EXTRACT_INFO,
            description="Extract information",
            actions=[extract_action],
            success_criteria=["data_extracted"]
        )
        steps.append(extract_step)
        
        return steps
    
    async def _create_execution_steps(self, elements: Dict[str, List[str]]) -> List[WorkflowStep]:
        """Create task execution steps"""
        
        steps = []
        
        # If calculator mentioned, create calculation workflow
        if "calculator" in elements.get("applications", []):
            calc_steps = await self._create_calculator_workflow()
            steps.extend(calc_steps)
        else:
            # Generic execution step
            execute_action = ContextualAction(
                intent=AutomationIntent.EXECUTE_TASK,
                target_description="execute button",
                action_type="trigger_action",
                parameters={"action": "click"},
                postconditions=["task_executed"]
            )
            
            execute_step = WorkflowStep(
                step_id="exec_01",
                intent=AutomationIntent.EXECUTE_TASK,
                description="Execute task",
                actions=[execute_action],
                success_criteria=["task_executed"]
            )
            steps.append(execute_step)
        
        return steps
    
    async def _create_calculator_workflow(self) -> List[WorkflowStep]:
        """Create specific calculator workflow"""
        
        return [
            WorkflowStep(
                step_id="calc_01",
                intent=AutomationIntent.NAVIGATE,
                description="Launch calculator",
                actions=[ContextualAction(
                    intent=AutomationIntent.NAVIGATE,
                    target_description="calculator",
                    action_type="application_launch",
                    parameters={"application": "galculator"},
                    postconditions=["calculator_visible"]
                )],
                success_criteria=["calculator_visible"]
            ),
            WorkflowStep(
                step_id="calc_02",
                intent=AutomationIntent.INPUT_DATA,
                description="Enter calculation",
                actions=[ContextualAction(
                    intent=AutomationIntent.INPUT_DATA,
                    target_description="calculator buttons",
                    action_type="sequence_input",
                    parameters={"sequence": "{{calculation_sequence}}"},
                    preconditions=["calculator_visible"],
                    postconditions=["calculation_entered"]
                )],
                success_criteria=["calculation_entered"]
            ),
            WorkflowStep(
                step_id="calc_03",
                intent=AutomationIntent.EXECUTE_TASK,
                description="Compute result",
                actions=[ContextualAction(
                    intent=AutomationIntent.EXECUTE_TASK,
                    target_description="equals button",
                    action_type="click",
                    parameters={"gesture": "hover_click"},
                    preconditions=["calculation_entered"],
                    postconditions=["result_displayed"]
                )],
                success_criteria=["result_displayed"]
            )
        ]
    
    async def _create_generic_steps(self, intent: AutomationIntent,
                                  elements: Dict[str, List[str]]) -> List[WorkflowStep]:
        """Create generic steps for other intents"""
        
        action = ContextualAction(
            intent=intent,
            target_description="target element",
            action_type="generic_action",
            parameters={"elements": elements},
            postconditions=["action_completed"]
        )
        
        step = WorkflowStep(
            step_id="generic_01",
            intent=intent,
            description=f"Perform {intent.value} action",
            actions=[action],
            success_criteria=["action_completed"]
        )
        
        return [step]
    
    async def _optimize_workflow(self, workflow: IntelligentWorkflow,
                               context: AutomationContext) -> IntelligentWorkflow:
        """Optimize workflow based on context"""
        
        # Add error handling
        for step in workflow.steps:
            if not step.failure_recovery:
                step.failure_recovery = [
                    ContextualAction(
                        intent=AutomationIntent.VALIDATE,
                        target_description="error recovery",
                        action_type="screenshot",
                        parameters={"path": f"/tmp/error_{step.step_id}.png"}
                    )
                ]
        
        # Add timing optimization
        for i, step in enumerate(workflow.steps):
            if i > 0:
                # Add dependency on previous step
                prev_step = workflow.steps[i-1]
                if prev_step.step_id not in step.dependencies:
                    step.dependencies.append(prev_step.step_id)
        
        return workflow
    
    def _create_adaptation_rules(self, intent: AutomationIntent) -> Dict[str, Any]:
        """Create adaptation rules for the workflow"""
        
        return {
            "retry_count": 3,
            "timeout_multiplier": 1.5,
            "fallback_enabled": True,
            "learning_enabled": True,
            "confidence_threshold": 0.7
        }
    
    def _create_fallback_workflow(self, description: str) -> IntelligentWorkflow:
        """Create minimal fallback workflow"""
        
        fallback_action = ContextualAction(
            intent=AutomationIntent.EXECUTE_TASK,
            target_description=description,
            action_type="screenshot",
            parameters={"path": "/tmp/fallback_workflow.png"}
        )
        
        fallback_step = WorkflowStep(
            step_id="fallback_01",
            intent=AutomationIntent.EXECUTE_TASK,
            description="Fallback action",
            actions=[fallback_action],
            success_criteria=["screenshot_taken"]
        )
        
        return IntelligentWorkflow(
            workflow_id="fallback",
            name="Fallback Workflow",
            description=description,
            target_goal=description,
            steps=[fallback_step]
        )

class ContextAwareAutomationEngine:
    """Main context-aware automation engine"""
    
    def __init__(self):
        self.natural_automation = NaturalUIAutomation()
        self.self_healing = SelfHealingAutomation()
        self.workflow_generator = WorkflowGenerator()
        self.current_context = AutomationContext()
        self.execution_history = []
        
    async def execute_natural_language_automation(self, 
                                                description: str,
                                                parameters: Dict[str, Any] = None) -> Dict[str, Any]:
        """Execute automation from natural language description"""
        
        try:
            logger.info(f"🎯 Executing natural language automation: {description}")
            
            # Update context
            await self._update_context()
            
            # Generate workflow
            workflow = await self.workflow_generator.generate_workflow(
                description, self.current_context
            )
            
            # Execute workflow
            results = await self._execute_workflow(workflow, parameters or {})
            
            # Learn from execution
            await self._learn_from_execution(workflow, results)
            
            return results
            
        except Exception as e:
            logger.error(f"Natural language automation failed: {e}")
            return {
                "success": False,
                "error": str(e),
                "description": description
            }
    
    async def _update_context(self):
        """Update current automation context"""
        
        try:
            # Take screenshot for context analysis
            screenshot_path = await self.natural_automation.take_screenshot()
            
            # Detect current application
            current_app = await self._detect_current_application()
            self.current_context.current_application = current_app
            
            # Analyze UI state
            ui_state = await self._analyze_ui_state(screenshot_path)
            self.current_context.ui_state = ui_state
            
            logger.info(f"Context updated: app={current_app}, ui_elements={len(ui_state)}")
            
        except Exception as e:
            logger.warning(f"Context update failed: {e}")
    
    async def _detect_current_application(self) -> Optional[str]:
        """Detect currently active application"""
        
        try:
            # Get active window information
            result = subprocess.run(['xdotool', 'getactivewindow', 'getwindowname'], 
                                  capture_output=True, text=True)
            
            if result.returncode == 0:
                window_name = result.stdout.strip().lower()
                
                # Map window names to applications
                app_mappings = {
                    'galculator': 'calculator',
                    'mousepad': 'text_editor',
                    'firefox': 'browser',
                    'terminal': 'terminal',
                    'file manager': 'file_manager'
                }
                
                for window_keyword, app_name in app_mappings.items():
                    if window_keyword in window_name:
                        return app_name
                
                return "unknown"
            
        except Exception as e:
            logger.warning(f"Application detection failed: {e}")
        
        return None
    
    async def _analyze_ui_state(self, screenshot_path: str) -> Dict[str, Any]:
        """Analyze current UI state"""
        
        try:
            # Use multi-modal detection
            elements = await self.natural_automation.element_detection.detect_elements(
                screenshot_path
            )
            
            ui_state = {
                "element_count": len(elements),
                "element_types": {},
                "text_content": [],
                "interactive_elements": []
            }
            
            for element in elements:
                # Count element types
                element_type = element.method
                ui_state["element_types"][element_type] = ui_state["element_types"].get(element_type, 0) + 1
                
                # Collect text content
                if element.text_content:
                    ui_state["text_content"].append(element.text_content)
                
                # Identify interactive elements
                if element.method in ['cv', 'ocr'] and element.confidence > 0.7:
                    ui_state["interactive_elements"].append({
                        "type": element.method,
                        "position": element.coordinates,
                        "confidence": element.confidence
                    })
            
            return ui_state
            
        except Exception as e:
            logger.warning(f"UI state analysis failed: {e}")
            return {}
    
    async def _execute_workflow(self, workflow: IntelligentWorkflow,
                              parameters: Dict[str, Any]) -> Dict[str, Any]:
        """Execute generated workflow"""
        
        results = {
            "workflow_id": workflow.workflow_id,
            "workflow_name": workflow.name,
            "success": False,
            "steps_completed": 0,
            "total_steps": len(workflow.steps),
            "step_results": [],
            "errors": [],
            "execution_time": 0,
            "screenshots": []
        }
        
        start_time = time.time()
        
        try:
            logger.info(f"📋 Executing workflow: {workflow.name}")
            
            for i, step in enumerate(workflow.steps):
                logger.info(f"Step {i+1}/{len(workflow.steps)}: {step.description}")
                
                step_result = await self._execute_workflow_step(step, parameters)
                results["step_results"].append(step_result)
                
                if step_result["success"]:
                    results["steps_completed"] += 1
                else:
                    results["errors"].append(f"Step {i+1} failed: {step_result.get('error', 'Unknown')}")
                    
                    # Try recovery actions
                    if step.failure_recovery:
                        logger.info("🔄 Attempting step recovery...")
                        recovery_result = await self._execute_recovery_actions(step.failure_recovery)
                        step_result["recovery_attempted"] = True
                        step_result["recovery_success"] = recovery_result
                
                # Take progress screenshot
                screenshot_path = f"/tmp/workflow_step_{i+1}.png"
                await self.natural_automation.take_screenshot(screenshot_path)
                results["screenshots"].append(screenshot_path)
                
                # Natural pause between steps
                await asyncio.sleep(1.0)
            
            results["success"] = results["steps_completed"] == results["total_steps"]
            results["execution_time"] = time.time() - start_time
            
            if results["success"]:
                logger.info("🏆 Workflow completed successfully!")
            else:
                logger.warning(f"⚠️ Workflow partially completed: {results['steps_completed']}/{results['total_steps']}")
            
        except Exception as e:
            results["errors"].append(f"Workflow execution failed: {str(e)}")
            logger.error(f"Workflow execution error: {e}")
        
        return results
    
    async def _execute_workflow_step(self, step: WorkflowStep,
                                   parameters: Dict[str, Any]) -> Dict[str, Any]:
        """Execute individual workflow step"""
        
        step_result = {
            "step_id": step.step_id,
            "description": step.description,
            "success": False,
            "actions_completed": 0,
            "total_actions": len(step.actions),
            "action_results": [],
            "error": None
        }
        
        try:
            for action in step.actions:
                action_result = await self._execute_contextual_action(action, parameters)
                step_result["action_results"].append(action_result)
                
                if action_result["success"]:
                    step_result["actions_completed"] += 1
                else:
                    step_result["error"] = action_result.get("error", "Action failed")
                    break
            
            step_result["success"] = step_result["actions_completed"] == step_result["total_actions"]
            
        except Exception as e:
            step_result["error"] = str(e)
        
        return step_result
    
    async def _execute_contextual_action(self, action: ContextualAction,
                                       parameters: Dict[str, Any]) -> Dict[str, Any]:
        """Execute individual contextual action"""
        
        action_result = {
            "intent": action.intent.value,
            "target": action.target_description,
            "action_type": action.action_type,
            "success": False,
            "error": None,
            "metadata": {}
        }
        
        try:
            # Substitute parameters in action
            processed_action = self._substitute_parameters(action, parameters)
            
            # Execute based on action type
            if processed_action.action_type == "application_launch":
                success = await self._execute_application_launch(processed_action)
            elif processed_action.action_type == "locate_element":
                success = await self._execute_locate_element(processed_action)
            elif processed_action.action_type == "natural_typing":
                success = await self._execute_natural_typing(processed_action)
            elif processed_action.action_type == "click":
                success = await self._execute_click_action(processed_action)
            elif processed_action.action_type == "data_extraction":
                success = await self._execute_data_extraction(processed_action)
            elif processed_action.action_type == "screenshot":
                success = await self._execute_screenshot(processed_action)
            else:
                # Generic execution
                success = await self._execute_generic_action(processed_action)
            
            action_result["success"] = success
            
        except Exception as e:
            action_result["error"] = str(e)
            logger.error(f"Action execution failed: {e}")
        
        return action_result
    
    def _substitute_parameters(self, action: ContextualAction,
                             parameters: Dict[str, Any]) -> ContextualAction:
        """Substitute parameters in action"""
        
        # Create copy of action
        processed_action = ContextualAction(
            intent=action.intent,
            target_description=action.target_description,
            action_type=action.action_type,
            parameters=action.parameters.copy(),
            preconditions=action.preconditions.copy(),
            postconditions=action.postconditions.copy(),
            confidence_requirement=action.confidence_requirement,
            retry_policy=action.retry_policy.copy(),
            context_metadata=action.context_metadata.copy()
        )
        
        # Substitute parameters
        for key, value in processed_action.parameters.items():
            if isinstance(value, str) and "{{" in value and "}}" in value:
                # Extract parameter name
                param_name = value.replace("{{", "").replace("}}", "")
                if param_name in parameters:
                    processed_action.parameters[key] = parameters[param_name]
        
        return processed_action
    
    async def _execute_application_launch(self, action: ContextualAction) -> bool:
        """Execute application launch action"""
        
        try:
            app_name = action.parameters.get("application", action.target_description)
            
            # Launch application
            subprocess.Popen([app_name])
            await asyncio.sleep(3)
            
            # Wait for application to appear
            success = await self.natural_automation.wait_for_element(app_name, timeout=10)
            
            if success:
                logger.info(f"✅ Successfully launched: {app_name}")
            else:
                logger.warning(f"❌ Failed to launch: {app_name}")
            
            return success
            
        except Exception as e:
            logger.error(f"Application launch failed: {e}")
            return False
    
    async def _execute_locate_element(self, action: ContextualAction) -> bool:
        """Execute element location action"""
        
        try:
            target = action.target_description
            
            # Use self-healing automation to find element
            screenshot_path = await self.natural_automation.take_screenshot()
            element = await self.self_healing.find_element_adaptive(target, screenshot_path)
            
            if element:
                logger.info(f"✅ Located element: {target} at {element.center}")
                return True
            else:
                logger.warning(f"❌ Could not locate element: {target}")
                return False
            
        except Exception as e:
            logger.error(f"Element location failed: {e}")
            return False
    
    async def _execute_natural_typing(self, action: ContextualAction) -> bool:
        """Execute natural typing action"""
        
        try:
            text = action.parameters.get("text", "")
            allow_mistakes = action.parameters.get("allow_mistakes", True)
            
            if text:
                success = await self.natural_automation.type_naturally(text, allow_mistakes)
                
                if success:
                    logger.info(f"✅ Typed text naturally: {text[:50]}...")
                else:
                    logger.warning(f"❌ Natural typing failed")
                
                return success
            
            return False
            
        except Exception as e:
            logger.error(f"Natural typing failed: {e}")
            return False
    
    async def _execute_click_action(self, action: ContextualAction) -> bool:
        """Execute click action"""
        
        try:
            target = action.target_description
            gesture_type = action.parameters.get("gesture", "hover_click")
            
            gesture = InteractionGesture(gesture_type)
            success = await self.natural_automation.find_and_interact(target, gesture)
            
            if success:
                logger.info(f"✅ Clicked: {target}")
            else:
                logger.warning(f"❌ Click failed: {target}")
            
            return success
            
        except Exception as e:
            logger.error(f"Click action failed: {e}")
            return False
    
    async def _execute_data_extraction(self, action: ContextualAction) -> bool:
        """Execute data extraction action"""
        
        try:
            # Take screenshot for extraction
            screenshot_path = await self.natural_automation.take_screenshot()
            
            # Use OCR or other methods to extract data
            method = action.parameters.get("method", "ocr")
            
            if method == "ocr":
                # Simple OCR extraction
                import pytesseract
                from PIL import Image
                
                image = Image.open(screenshot_path)
                extracted_text = pytesseract.image_to_string(image)
                
                if extracted_text.strip():
                    logger.info(f"✅ Extracted data: {extracted_text[:100]}...")
                    return True
                else:
                    logger.warning("❌ No data extracted")
                    return False
            
            return True
            
        except Exception as e:
            logger.error(f"Data extraction failed: {e}")
            return False
    
    async def _execute_screenshot(self, action: ContextualAction) -> bool:
        """Execute screenshot action"""
        
        try:
            path = action.parameters.get("path", f"/tmp/context_screenshot_{int(time.time())}.png")
            screenshot_path = await self.natural_automation.take_screenshot(path)
            
            logger.info(f"✅ Screenshot saved: {screenshot_path}")
            return True
            
        except Exception as e:
            logger.error(f"Screenshot failed: {e}")
            return False
    
    async def _execute_generic_action(self, action: ContextualAction) -> bool:
        """Execute generic action"""
        
        try:
            # Default to finding and clicking the target
            target = action.target_description
            success = await self.natural_automation.find_and_interact(
                target, InteractionGesture.HOVER_CLICK
            )
            
            if success:
                logger.info(f"✅ Generic action completed: {target}")
            else:
                logger.warning(f"❌ Generic action failed: {target}")
            
            return success
            
        except Exception as e:
            logger.error(f"Generic action failed: {e}")
            return False
    
    async def _execute_recovery_actions(self, recovery_actions: List[ContextualAction]) -> bool:
        """Execute recovery actions"""
        
        try:
            for action in recovery_actions:
                result = await self._execute_contextual_action(action, {})
                if result["success"]:
                    return True
            
            return False
            
        except Exception as e:
            logger.error(f"Recovery actions failed: {e}")
            return False
    
    async def _learn_from_execution(self, workflow: IntelligentWorkflow,
                                  results: Dict[str, Any]):
        """Learn from workflow execution"""
        
        try:
            # Store execution in history
            execution_record = {
                "workflow_id": workflow.workflow_id,
                "timestamp": time.time(),
                "success": results["success"],
                "completion_rate": results["steps_completed"] / results["total_steps"],
                "execution_time": results["execution_time"],
                "errors": results["errors"]
            }
            
            self.execution_history.append(execution_record)
            
            # Update context learned patterns
            if results["success"]:
                pattern_key = f"{workflow.workflow_id}_success"
                self.current_context.learned_patterns[pattern_key] = {
                    "workflow": asdict(workflow),
                    "success_time": execution_record["execution_time"],
                    "context": asdict(self.current_context)
                }
            
            logger.info(f"📚 Learning completed for workflow: {workflow.name}")
            
        except Exception as e:
            logger.warning(f"Learning failed: {e}")

async def demo_context_aware_automation():
    """Comprehensive demonstration of context-aware automation"""
    
    print("🚀 Context-Aware Automation Engine Demo")
    print("Features: Intent Recognition, Workflow Generation, Natural Execution, Adaptive Learning")
    
    # Initialize engine
    automation_engine = ContextAwareAutomationEngine()
    
    # Test scenarios demonstrating different intents and contexts
    test_scenarios = [
        {
            "description": "Open calculator and compute 15 times 8",
            "parameters": {"calculation_sequence": ["1", "5", "*", "8", "="]}
        },
        {
            "description": "Launch text editor and write a summary of the calculation",
            "parameters": {"user_input": "Calculation Summary:\n15 × 8 = 120\n\nThis demonstrates context-aware automation with:\n• Natural language intent recognition\n• Intelligent workflow generation\n• Adaptive execution strategies\n• Self-healing element detection"}
        },
        {
            "description": "Take a screenshot to document the automation results",
            "parameters": {}
        }
    ]
    
    # Execute scenarios
    for i, scenario in enumerate(test_scenarios):
        print(f"\n{i+1}️⃣ Scenario: {scenario['description']}")
        
        results = await automation_engine.execute_natural_language_automation(
            scenario["description"],
            scenario["parameters"]
        )
        
        print(f"✅ Success: {results['success']}")
        if results.get("steps_completed") and results.get("total_steps"):
            print(f"📊 Progress: {results['steps_completed']}/{results['total_steps']} steps")
        
        if results.get("execution_time"):
            print(f"⏱️ Time: {results['execution_time']:.2f}s")
        
        if results.get("errors"):
            print(f"❌ Errors: {len(results['errors'])}")
            for error in results["errors"][:3]:  # Show first 3 errors
                print(f"   - {error}")
        
        # Show screenshots if available
        if results.get("screenshots"):
            print(f"📸 Screenshots: {len(results['screenshots'])}")
            for screenshot in results["screenshots"][-2:]:  # Show last 2
                print(f"   - {screenshot}")
        
        # Natural pause between scenarios
        await asyncio.sleep(2)
    
    # Display learning summary
    print(f"\n🧠 Learning Summary:")
    print(f"Execution history: {len(automation_engine.execution_history)} workflows")
    print(f"Learned patterns: {len(automation_engine.current_context.learned_patterns)}")
    print(f"Current application: {automation_engine.current_context.current_application}")
    
    ui_state = automation_engine.current_context.ui_state
    if ui_state:
        print(f"UI elements detected: {ui_state.get('element_count', 0)}")
        print(f"Interactive elements: {len(ui_state.get('interactive_elements', []))}")
    
    print(f"\n🏆 Context-Aware Automation Demo Complete!")
    print("Demonstrated: Intent Recognition, Workflow Generation, Natural Execution, Adaptive Learning")

if __name__ == "__main__":
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(levelname)s - %(message)s'
    )
    
    asyncio.run(demo_context_aware_automation())