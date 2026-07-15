#!/usr/bin/env python3
"""
KVirtualStage Automation Stack - Modern Computer Use Implementation
Integrates multiple automation approaches for accurate UI interaction
"""

import os
import time
import logging
import cv2
import numpy as np
import pyautogui
from typing import Optional, Tuple, List, Dict, Any
from dataclasses import dataclass
from PIL import Image
import subprocess
import json

# Accessibility imports
try:
    from dogtail.tree import root
    from dogtail.utils import isA11yEnabled, enableA11y
    from dogtail.predicate import GenericPredicate
    DOGTAIL_AVAILABLE = True
except ImportError:
    DOGTAIL_AVAILABLE = False

# X11 imports
try:
    from Xlib import display, X
    from Xlib.ext.xtest import fake_input
    X11_AVAILABLE = True
except ImportError:
    X11_AVAILABLE = False

# Set up logging
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

@dataclass
class UIElement:
    """Represents a UI element with multiple detection methods"""
    name: str
    element_type: str
    coordinates: Optional[Tuple[int, int]] = None
    accessibility_path: Optional[str] = None
    template_path: Optional[str] = None
    confidence: float = 0.8

@dataclass
class AutomationResult:
    """Result of an automation action"""
    success: bool
    method_used: str
    coordinates: Optional[Tuple[int, int]] = None
    error_message: Optional[str] = None

class KDEComputerUseAutomation:
    """
    Advanced KDE automation system using multiple detection and interaction methods.
    Implements computer use patterns similar to UI-TARS and Playwright.
    """
    
    def __init__(self, display_id: str = ":1"):
        """Initialize the automation system"""
        self.display_id = display_id
        os.environ['DISPLAY'] = display_id
        
        # Configure PyAutoGUI
        pyautogui.FAILSAFE = True
        pyautogui.PAUSE = 0.1
        
        # Initialize accessibility if available
        self.accessibility_enabled = False
        if DOGTAIL_AVAILABLE:
            try:
                if not isA11yEnabled():
                    enableA11y(True)
                self.accessibility_enabled = True
                logger.info("Accessibility support enabled")
            except Exception as e:
                logger.warning(f"Could not enable accessibility: {e}")
        
        # Initialize X11 if available
        self.x11_display = None
        if X11_AVAILABLE:
            try:
                self.x11_display = display.Display(display_id)
                logger.info("X11 support enabled")
            except Exception as e:
                logger.warning(f"Could not connect to X11: {e}")
        
        # Template cache for faster matching
        self.template_cache: Dict[str, np.ndarray] = {}
        
        logger.info("KDE Computer Use Automation initialized")
    
    def take_screenshot(self, save_path: Optional[str] = None) -> np.ndarray:
        """Take a screenshot and optionally save it"""
        try:
            screenshot = pyautogui.screenshot()
            screenshot_np = np.array(screenshot)
            
            if save_path:
                screenshot.save(save_path)
                logger.info(f"Screenshot saved to {save_path}")
            
            return screenshot_np
        except Exception as e:
            logger.error(f"Failed to take screenshot: {e}")
            raise
    
    def find_element_by_accessibility(self, name: str, role: Optional[str] = None) -> Optional[Any]:
        """Find UI element using KDE accessibility APIs"""
        if not self.accessibility_enabled:
            return None
        
        try:
            if role:
                element = root.findChild(GenericPredicate(name=name, roleName=role))
            else:
                element = root.findChild(GenericPredicate(name=name))
            
            logger.info(f"Found element '{name}' via accessibility")
            return element
        except Exception as e:
            logger.debug(f"Accessibility search failed for '{name}': {e}")
            return None
    
    def find_element_by_template(self, template_path: str, confidence: float = 0.8) -> Optional[Tuple[int, int]]:
        """Find UI element using template matching"""
        try:
            # Load template from cache or file
            if template_path not in self.template_cache:
                if not os.path.exists(template_path):
                    logger.warning(f"Template file not found: {template_path}")
                    return None
                self.template_cache[template_path] = cv2.imread(template_path)
            
            template = self.template_cache[template_path]
            screenshot = self.take_screenshot()
            
            # Convert to grayscale for better matching
            screenshot_gray = cv2.cvtColor(screenshot, cv2.COLOR_RGB2GRAY)
            template_gray = cv2.cvtColor(template, cv2.COLOR_BGR2GRAY)
            
            # Perform template matching
            result = cv2.matchTemplate(screenshot_gray, template_gray, cv2.TM_CCOEFF_NORMED)
            locations = np.where(result >= confidence)
            
            if len(locations[0]) > 0:
                # Return center of first match
                y, x = locations[0][0], locations[1][0]
                h, w = template_gray.shape
                center_x = x + w // 2
                center_y = y + h // 2
                
                logger.info(f"Found element at ({center_x}, {center_y}) via template matching")
                return (center_x, center_y)
            
        except Exception as e:
            logger.debug(f"Template matching failed for '{template_path}': {e}")
        
        return None
    
    def find_element_by_text_ocr(self, text: str, confidence: float = 0.8) -> Optional[Tuple[int, int]]:
        """Find UI element by text using OCR"""
        try:
            import easyocr
            reader = easyocr.Reader(['en'])
            
            screenshot = self.take_screenshot()
            results = reader.readtext(screenshot)
            
            for (bbox, detected_text, conf) in results:
                if conf >= confidence and text.lower() in detected_text.lower():
                    # Calculate center of bounding box
                    x_coords = [point[0] for point in bbox]
                    y_coords = [point[1] for point in bbox]
                    center_x = int(sum(x_coords) / len(x_coords))
                    center_y = int(sum(y_coords) / len(y_coords))
                    
                    logger.info(f"Found text '{text}' at ({center_x}, {center_y}) via OCR")
                    return (center_x, center_y)
            
        except ImportError:
            logger.warning("EasyOCR not available for text detection")
        except Exception as e:
            logger.debug(f"OCR search failed for '{text}': {e}")
        
        return None
    
    def smooth_move_cursor(self, start_x: int, start_y: int, end_x: int, end_y: int, steps: int = 25, delay: float = 0.02):
        """Move cursor smoothly between two points"""
        for i in range(steps + 1):
            progress = i / steps
            current_x = int(start_x + (end_x - start_x) * progress)
            current_y = int(start_y + (end_y - start_y) * progress)
            
            pyautogui.moveTo(current_x, current_y)
            time.sleep(delay)
    
    def click_element(self, element: UIElement, method_priority: List[str] = None) -> AutomationResult:
        """
        Click on a UI element using multiple detection methods with fallback
        """
        if method_priority is None:
            method_priority = ['accessibility', 'template', 'coordinates', 'ocr']
        
        current_pos = pyautogui.position()
        
        for method in method_priority:
            try:
                if method == 'accessibility' and self.accessibility_enabled:
                    acc_element = self.find_element_by_accessibility(element.name, element.element_type)
                    if acc_element:
                        acc_element.click()
                        return AutomationResult(True, 'accessibility')
                
                elif method == 'template' and element.template_path:
                    coords = self.find_element_by_template(element.template_path, element.confidence)
                    if coords:
                        self.smooth_move_cursor(current_pos.x, current_pos.y, coords[0], coords[1])
                        pyautogui.click(coords[0], coords[1])
                        return AutomationResult(True, 'template', coords)
                
                elif method == 'coordinates' and element.coordinates:
                    self.smooth_move_cursor(current_pos.x, current_pos.y, element.coordinates[0], element.coordinates[1])
                    pyautogui.click(element.coordinates[0], element.coordinates[1])
                    return AutomationResult(True, 'coordinates', element.coordinates)
                
                elif method == 'ocr':
                    coords = self.find_element_by_text_ocr(element.name)
                    if coords:
                        self.smooth_move_cursor(current_pos.x, current_pos.y, coords[0], coords[1])
                        pyautogui.click(coords[0], coords[1])
                        return AutomationResult(True, 'ocr', coords)
                
            except Exception as e:
                logger.debug(f"Method '{method}' failed for element '{element.name}': {e}")
                continue
        
        return AutomationResult(False, 'none', error_message=f"Could not find element '{element.name}'")
    
    def type_text(self, text: str, delay: float = 0.05) -> AutomationResult:
        """Type text with natural timing"""
        try:
            for char in text:
                if char == '\n':
                    pyautogui.press('enter')
                    time.sleep(0.2)
                else:
                    pyautogui.write(char)
                    time.sleep(delay)
            
            return AutomationResult(True, 'keyboard')
        except Exception as e:
            return AutomationResult(False, 'keyboard', error_message=str(e))
    
    def wait_for_element(self, element: UIElement, timeout: int = 10) -> bool:
        """Wait for an element to appear on screen"""
        start_time = time.time()
        
        while time.time() - start_time < timeout:
            result = self.click_element(element, method_priority=['accessibility', 'template', 'ocr'])
            if result.success:
                return True
            time.sleep(0.5)
        
        return False
    
    def get_window_list(self) -> List[Dict[str, Any]]:
        """Get list of open windows"""
        try:
            result = subprocess.run(['wmctrl', '-l'], capture_output=True, text=True)
            windows = []
            
            for line in result.stdout.strip().split('\n'):
                if line:
                    parts = line.split(None, 3)
                    if len(parts) >= 4:
                        windows.append({
                            'id': parts[0],
                            'desktop': parts[1],
                            'pid': parts[2],
                            'title': parts[3]
                        })
            
            return windows
        except Exception as e:
            logger.error(f"Failed to get window list: {e}")
            return []
    
    def focus_window(self, window_title: str) -> bool:
        """Focus a window by title"""
        try:
            subprocess.run(['wmctrl', '-a', window_title], check=True)
            time.sleep(0.5)  # Wait for focus change
            return True
        except Exception as e:
            logger.error(f"Failed to focus window '{window_title}': {e}")
            return False

class AutomationRecorder:
    """Records automation sessions for debugging and analysis"""
    
    def __init__(self, output_dir: str = "/tmp/automation_recordings"):
        self.output_dir = output_dir
        os.makedirs(output_dir, exist_ok=True)
        self.session_id = int(time.time())
        self.actions: List[Dict[str, Any]] = []
    
    def record_action(self, action_type: str, element: UIElement, result: AutomationResult):
        """Record an automation action"""
        action = {
            'timestamp': time.time(),
            'action_type': action_type,
            'element_name': element.name,
            'element_type': element.element_type,
            'success': result.success,
            'method_used': result.method_used,
            'coordinates': result.coordinates,
            'error_message': result.error_message
        }
        self.actions.append(action)
    
    def save_session(self) -> str:
        """Save the recording session to file"""
        session_file = os.path.join(self.output_dir, f"session_{self.session_id}.json")
        
        with open(session_file, 'w') as f:
            json.dump({
                'session_id': self.session_id,
                'start_time': min(action['timestamp'] for action in self.actions) if self.actions else time.time(),
                'end_time': max(action['timestamp'] for action in self.actions) if self.actions else time.time(),
                'actions': self.actions
            }, f, indent=2)
        
        logger.info(f"Automation session saved to {session_file}")
        return session_file

# Example usage and testing
if __name__ == "__main__":
    # Initialize automation system
    automation = KDEComputerUseAutomation()
    recorder = AutomationRecorder()
    
    # Take initial screenshot
    automation.take_screenshot("/tmp/kde_initial.png")
    
    # Example elements for KDE applications
    calculator_button = UIElement(
        name="Calculator",
        element_type="application",
        coordinates=(100, 100)  # Fallback coordinates
    )
    
    # Test the automation system
    logger.info("KDE Computer Use Automation system ready for testing")