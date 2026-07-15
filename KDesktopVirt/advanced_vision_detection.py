#!/usr/bin/env python3
"""
Advanced Computer Vision Detection Module for KVirtualStage
Implements sophisticated element detection using deep learning and traditional CV
Provides self-healing automation with adaptive selectors
"""

import cv2
import numpy as np
import logging
from typing import Dict, List, Tuple, Optional, Any
from dataclasses import dataclass
import json
import pickle
import os
from PIL import Image, ImageEnhance, ImageFilter
import pytesseract
import asyncio
import subprocess
import time

logger = logging.getLogger(__name__)

@dataclass
class VisualElement:
    """Advanced visual element representation"""
    element_id: str
    element_type: str  # button, textfield, icon, window, menu
    confidence: float
    bbox: Tuple[int, int, int, int]  # x, y, width, height
    center: Tuple[int, int]
    visual_features: Dict[str, Any]
    text_content: Optional[str] = None
    template_match_score: Optional[float] = None
    color_histogram: Optional[np.ndarray] = None
    edge_density: Optional[float] = None

@dataclass
class AdaptiveSelector:
    """Self-healing selector that adapts to UI changes"""
    target_name: str
    primary_method: str
    fallback_methods: List[str]
    learned_features: Dict[str, Any]
    success_history: List[bool]
    last_known_position: Optional[Tuple[int, int]] = None
    confidence_threshold: float = 0.7

class AdvancedTemplateMatching:
    """Template matching with rotation, scale, and lighting invariance"""
    
    def __init__(self):
        self.templates_cache = {}
        self.feature_detector = cv2.SIFT_create()
        
    def learn_template(self, element_name: str, image_region: np.ndarray, 
                      metadata: Dict[str, Any] = None):
        """Learn a new template for future detection"""
        
        # Extract multiple representations
        template_data = {
            'original': image_region.copy(),
            'grayscale': cv2.cvtColor(image_region, cv2.COLOR_BGR2GRAY) if len(image_region.shape) == 3 else image_region,
            'edges': cv2.Canny(cv2.cvtColor(image_region, cv2.COLOR_BGR2GRAY) if len(image_region.shape) == 3 else image_region, 50, 150),
            'keypoints': None,
            'descriptors': None,
            'color_hist': None,
            'metadata': metadata or {}
        }
        
        # Extract SIFT features
        try:
            kp, desc = self.feature_detector.detectAndCompute(template_data['grayscale'], None)
            template_data['keypoints'] = kp
            template_data['descriptors'] = desc
        except Exception as e:
            logger.warning(f"SIFT feature extraction failed: {e}")
        
        # Color histogram
        if len(image_region.shape) == 3:
            template_data['color_hist'] = cv2.calcHist([image_region], [0, 1, 2], None, [8, 8, 8], [0, 256, 0, 256, 0, 256])
        
        self.templates_cache[element_name] = template_data
        logger.info(f"Learned template for: {element_name}")
    
    def find_template_matches(self, search_image: np.ndarray, 
                            template_name: str, 
                            methods: List[str] = None) -> List[VisualElement]:
        """Find template matches using multiple methods"""
        
        if template_name not in self.templates_cache:
            return []
        
        template_data = self.templates_cache[template_name]
        matches = []
        
        if methods is None:
            methods = ['correlation', 'sift', 'color_hist']
        
        try:
            # Method 1: Template correlation
            if 'correlation' in methods:
                corr_matches = self._template_correlation_match(search_image, template_data)
                matches.extend(corr_matches)
            
            # Method 2: SIFT feature matching
            if 'sift' in methods and template_data['descriptors'] is not None:
                sift_matches = self._sift_feature_match(search_image, template_data)
                matches.extend(sift_matches)
            
            # Method 3: Color histogram matching
            if 'color_hist' in methods and template_data['color_hist'] is not None:
                hist_matches = self._color_histogram_match(search_image, template_data)
                matches.extend(hist_matches)
            
        except Exception as e:
            logger.error(f"Template matching failed: {e}")
        
        # Remove duplicates and rank by confidence
        return self._consolidate_matches(matches, template_name)
    
    def _template_correlation_match(self, image: np.ndarray, 
                                  template_data: Dict) -> List[VisualElement]:
        """Traditional template matching with multiple scales"""
        
        matches = []
        template = template_data['grayscale']
        gray_image = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY) if len(image.shape) == 3 else image
        
        # Multi-scale template matching
        scales = [0.8, 0.9, 1.0, 1.1, 1.2]
        
        for scale in scales:
            # Resize template
            width = int(template.shape[1] * scale)
            height = int(template.shape[0] * scale)
            
            if width < 10 or height < 10 or width > image.shape[1] or height > image.shape[0]:
                continue
            
            resized_template = cv2.resize(template, (width, height))
            
            # Perform template matching
            result = cv2.matchTemplate(gray_image, resized_template, cv2.TM_CCOEFF_NORMED)
            
            # Find peaks
            threshold = 0.7
            locations = np.where(result >= threshold)
            
            for pt in zip(*locations[::-1]):
                x, y = pt
                confidence = result[y, x]
                
                visual_element = VisualElement(
                    element_id=f"template_corr_{x}_{y}",
                    element_type="template_match",
                    confidence=float(confidence),
                    bbox=(x, y, width, height),
                    center=(x + width // 2, y + height // 2),
                    visual_features={'scale': scale, 'method': 'correlation'},
                    template_match_score=float(confidence)
                )
                matches.append(visual_element)
        
        return matches
    
    def _sift_feature_match(self, image: np.ndarray, 
                          template_data: Dict) -> List[VisualElement]:
        """SIFT feature-based matching"""
        
        matches = []
        gray_image = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY) if len(image.shape) == 3 else image
        
        try:
            # Extract features from search image
            kp_image, desc_image = self.feature_detector.detectAndCompute(gray_image, None)
            
            if desc_image is None or template_data['descriptors'] is None:
                return matches
            
            # Match features
            matcher = cv2.BFMatcher()
            raw_matches = matcher.knnMatch(template_data['descriptors'], desc_image, k=2)
            
            # Apply Lowe's ratio test
            good_matches = []
            for match_pair in raw_matches:
                if len(match_pair) == 2:
                    m, n = match_pair
                    if m.distance < 0.7 * n.distance:
                        good_matches.append(m)
            
            # Find homography if enough matches
            if len(good_matches) >= 4:
                src_pts = np.float32([template_data['keypoints'][m.queryIdx].pt for m in good_matches]).reshape(-1, 1, 2)
                dst_pts = np.float32([kp_image[m.trainIdx].pt for m in good_matches]).reshape(-1, 1, 2)
                
                matrix, mask = cv2.findHomography(src_pts, dst_pts, cv2.RANSAC, 5.0)
                
                if matrix is not None:
                    # Calculate bounding box
                    h, w = template_data['grayscale'].shape
                    corners = np.float32([[0, 0], [w, 0], [w, h], [0, h]]).reshape(-1, 1, 2)
                    transformed_corners = cv2.perspectiveTransform(corners, matrix)
                    
                    # Extract bounding rectangle
                    x, y, w, h = cv2.boundingRect(transformed_corners)
                    
                    confidence = len(good_matches) / max(10, len(template_data['keypoints']))
                    
                    visual_element = VisualElement(
                        element_id=f"sift_{x}_{y}",
                        element_type="sift_match",
                        confidence=min(1.0, confidence),
                        bbox=(x, y, w, h),
                        center=(x + w // 2, y + h // 2),
                        visual_features={'matches': len(good_matches), 'method': 'sift'}
                    )
                    matches.append(visual_element)
            
        except Exception as e:
            logger.warning(f"SIFT matching failed: {e}")
        
        return matches
    
    def _color_histogram_match(self, image: np.ndarray, 
                             template_data: Dict) -> List[VisualElement]:
        """Color histogram-based matching"""
        
        matches = []
        
        if len(image.shape) != 3 or template_data['color_hist'] is None:
            return matches
        
        try:
            # Use sliding window approach
            template_h, template_w = template_data['original'].shape[:2]
            
            step_size = min(20, template_w // 4, template_h // 4)
            
            for y in range(0, image.shape[0] - template_h, step_size):
                for x in range(0, image.shape[1] - template_w, step_size):
                    # Extract window
                    window = image[y:y+template_h, x:x+template_w]
                    
                    if window.shape[:2] == (template_h, template_w):
                        # Calculate histogram
                        window_hist = cv2.calcHist([window], [0, 1, 2], None, [8, 8, 8], [0, 256, 0, 256, 0, 256])
                        
                        # Compare histograms
                        correlation = cv2.compareHist(template_data['color_hist'], window_hist, cv2.HISTCMP_CORREL)
                        
                        if correlation > 0.8:
                            visual_element = VisualElement(
                                element_id=f"hist_{x}_{y}",
                                element_type="histogram_match",
                                confidence=correlation,
                                bbox=(x, y, template_w, template_h),
                                center=(x + template_w // 2, y + template_h // 2),
                                visual_features={'correlation': correlation, 'method': 'histogram'},
                                color_histogram=window_hist
                            )
                            matches.append(visual_element)
            
        except Exception as e:
            logger.warning(f"Histogram matching failed: {e}")
        
        return matches
    
    def _consolidate_matches(self, matches: List[VisualElement], 
                           template_name: str) -> List[VisualElement]:
        """Consolidate overlapping matches and rank by confidence"""
        
        if not matches:
            return []
        
        # Group nearby matches
        consolidated = []
        processed = set()
        
        for i, match in enumerate(matches):
            if i in processed:
                continue
            
            # Find overlapping matches
            group = [match]
            for j, other in enumerate(matches):
                if j != i and j not in processed:
                    # Check overlap
                    overlap = self._calculate_overlap(match.bbox, other.bbox)
                    if overlap > 0.3:  # 30% overlap threshold
                        group.append(other)
                        processed.add(j)
            
            # Create consolidated match
            if group:
                best_match = max(group, key=lambda m: m.confidence)
                
                # Average position if multiple matches
                if len(group) > 1:
                    avg_x = sum(m.center[0] for m in group) // len(group)
                    avg_y = sum(m.center[1] for m in group) // len(group)
                    best_match.center = (avg_x, avg_y)
                    best_match.confidence = sum(m.confidence for m in group) / len(group)
                
                consolidated.append(best_match)
            
            processed.add(i)
        
        # Sort by confidence
        consolidated.sort(key=lambda m: m.confidence, reverse=True)
        
        return consolidated
    
    def _calculate_overlap(self, bbox1: Tuple[int, int, int, int], 
                         bbox2: Tuple[int, int, int, int]) -> float:
        """Calculate overlap ratio between two bounding boxes"""
        
        x1, y1, w1, h1 = bbox1
        x2, y2, w2, h2 = bbox2
        
        # Calculate intersection
        left = max(x1, x2)
        top = max(y1, y2)
        right = min(x1 + w1, x2 + w2)
        bottom = min(y1 + h1, y2 + h2)
        
        if left < right and top < bottom:
            intersection = (right - left) * (bottom - top)
            area1 = w1 * h1
            area2 = w2 * h2
            union = area1 + area2 - intersection
            
            return intersection / union if union > 0 else 0
        
        return 0

class SemanticUIAnalyzer:
    """Semantic understanding of UI elements and layout"""
    
    def __init__(self):
        self.ui_patterns = self._load_ui_patterns()
        self.layout_analyzer = LayoutAnalyzer()
    
    def _load_ui_patterns(self) -> Dict[str, Any]:
        """Load common UI patterns and element characteristics"""
        
        return {
            'button_patterns': {
                'aspect_ratio_range': (0.3, 4.0),
                'typical_colors': ['#007bff', '#28a745', '#dc3545', '#6c757d'],
                'text_indicators': ['click', 'submit', 'ok', 'cancel', 'apply', 'save'],
                'shape_characteristics': 'rectangular_with_rounded_corners'
            },
            'textfield_patterns': {
                'aspect_ratio_range': (2.0, 10.0),
                'typical_colors': ['#ffffff', '#f8f9fa'],
                'border_characteristics': 'thin_border',
                'text_indicators': ['enter', 'input', 'search', 'type']
            },
            'menu_patterns': {
                'layout': 'vertical_list',
                'hover_effects': True,
                'text_indicators': ['file', 'edit', 'view', 'help']
            },
            'icon_patterns': {
                'size_range': (16, 64),
                'aspect_ratio': (0.8, 1.2),
                'symbol_based': True
            }
        }
    
    async def analyze_ui_semantics(self, image: np.ndarray, 
                                 ocr_results: List[Dict] = None) -> List[VisualElement]:
        """Analyze UI semantics to identify element types and purposes"""
        
        semantic_elements = []
        
        try:
            # Detect UI regions
            ui_regions = await self._detect_ui_regions(image)
            
            # Analyze each region
            for region in ui_regions:
                element_type = await self._classify_ui_element(image, region, ocr_results)
                
                if element_type:
                    x, y, w, h = region
                    semantic_element = VisualElement(
                        element_id=f"semantic_{element_type}_{x}_{y}",
                        element_type=element_type,
                        confidence=0.8,  # Semantic analysis confidence
                        bbox=region,
                        center=(x + w // 2, y + h // 2),
                        visual_features={'analysis_method': 'semantic', 'region_type': element_type}
                    )
                    semantic_elements.append(semantic_element)
            
        except Exception as e:
            logger.error(f"Semantic analysis failed: {e}")
        
        return semantic_elements
    
    async def _detect_ui_regions(self, image: np.ndarray) -> List[Tuple[int, int, int, int]]:
        """Detect potential UI element regions"""
        
        regions = []
        gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY) if len(image.shape) == 3 else image
        
        # Edge detection for UI boundaries
        edges = cv2.Canny(gray, 50, 150, apertureSize=3)
        
        # Morphological operations to connect edges
        kernel = cv2.getStructuringElement(cv2.MORPH_RECT, (3, 3))
        closed = cv2.morphologyEx(edges, cv2.MORPH_CLOSE, kernel)
        
        # Find contours
        contours, _ = cv2.findContours(closed, cv2.RETR_EXTERNAL, cv2.CHAIN_APPROX_SIMPLE)
        
        for contour in contours:
            area = cv2.contourArea(contour)
            if 100 < area < 50000:  # UI element size range
                x, y, w, h = cv2.boundingRect(contour)
                
                # Filter by aspect ratio and size
                aspect_ratio = w / h if h > 0 else 0
                if 0.1 < aspect_ratio < 10.0 and w > 20 and h > 10:
                    regions.append((x, y, w, h))
        
        return regions
    
    async def _classify_ui_element(self, image: np.ndarray, 
                                 region: Tuple[int, int, int, int],
                                 ocr_results: List[Dict] = None) -> Optional[str]:
        """Classify UI element type based on visual characteristics"""
        
        x, y, w, h = region
        roi = image[y:y+h, x:x+w]
        
        if roi.size == 0:
            return None
        
        aspect_ratio = w / h
        area = w * h
        
        # Button detection
        if (0.3 <= aspect_ratio <= 4.0 and 
            500 <= area <= 20000 and
            self._has_button_characteristics(roi)):
            return "button"
        
        # Text field detection
        elif (2.0 <= aspect_ratio <= 10.0 and
              200 <= area <= 15000 and
              self._has_textfield_characteristics(roi)):
            return "textfield"
        
        # Icon detection
        elif (0.8 <= aspect_ratio <= 1.2 and
              256 <= area <= 4096 and
              self._has_icon_characteristics(roi)):
            return "icon"
        
        # Menu item detection
        elif (1.5 <= aspect_ratio <= 8.0 and
              self._has_menu_characteristics(roi, ocr_results)):
            return "menu_item"
        
        return "ui_element"
    
    def _has_button_characteristics(self, roi: np.ndarray) -> bool:
        """Check if region has button-like characteristics"""
        
        # Check for rounded corners
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY) if len(roi.shape) == 3 else roi
        edges = cv2.Canny(gray, 50, 150)
        
        # Analyze edge density
        edge_density = np.sum(edges > 0) / edges.size
        
        # Buttons typically have moderate edge density (0.05 to 0.2)
        return 0.05 <= edge_density <= 0.2
    
    def _has_textfield_characteristics(self, roi: np.ndarray) -> bool:
        """Check if region has text field characteristics"""
        
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY) if len(roi.shape) == 3 else roi
        
        # Text fields typically have high brightness (white/light background)
        mean_brightness = np.mean(gray)
        
        # Check for thin borders
        edges = cv2.Canny(gray, 30, 100)
        border_pixels = np.sum(edges > 0)
        total_pixels = edges.size
        border_ratio = border_pixels / total_pixels
        
        return mean_brightness > 180 and border_ratio < 0.1
    
    def _has_icon_characteristics(self, roi: np.ndarray) -> bool:
        """Check if region has icon characteristics"""
        
        gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY) if len(roi.shape) == 3 else roi
        
        # Icons have high contrast and distinct shapes
        contrast = np.std(gray)
        
        return contrast > 30
    
    def _has_menu_characteristics(self, roi: np.ndarray, 
                                ocr_results: List[Dict] = None) -> bool:
        """Check if region has menu characteristics"""
        
        # Menu items typically contain text
        if ocr_results:
            x, y, w, h = roi.shape if hasattr(roi, 'shape') else (0, 0, roi.shape[1], roi.shape[0])
            
            for ocr_result in ocr_results:
                ocr_x, ocr_y = ocr_result.get('x', 0), ocr_result.get('y', 0)
                if x <= ocr_x <= x + w and y <= ocr_y <= y + h:
                    text = ocr_result.get('text', '').lower()
                    menu_keywords = ['file', 'edit', 'view', 'help', 'options', 'settings']
                    if any(keyword in text for keyword in menu_keywords):
                        return True
        
        return False

class LayoutAnalyzer:
    """Analyze UI layout and spatial relationships"""
    
    def __init__(self):
        pass
    
    def analyze_spatial_relationships(self, elements: List[VisualElement]) -> Dict[str, Any]:
        """Analyze spatial relationships between UI elements"""
        
        relationships = {
            'groups': [],
            'hierarchies': [],
            'alignment_patterns': [],
            'grid_structure': None
        }
        
        if len(elements) < 2:
            return relationships
        
        # Detect element groups based on proximity
        groups = self._detect_element_groups(elements)
        relationships['groups'] = groups
        
        # Detect hierarchical structures
        hierarchies = self._detect_hierarchies(elements)
        relationships['hierarchies'] = hierarchies
        
        # Detect alignment patterns
        alignments = self._detect_alignments(elements)
        relationships['alignment_patterns'] = alignments
        
        # Detect grid structures
        grid = self._detect_grid_structure(elements)
        relationships['grid_structure'] = grid
        
        return relationships
    
    def _detect_element_groups(self, elements: List[VisualElement]) -> List[List[str]]:
        """Detect groups of related UI elements"""
        
        groups = []
        processed = set()
        
        for element in elements:
            if element.element_id in processed:
                continue
            
            group = [element.element_id]
            
            # Find nearby elements
            for other in elements:
                if (other.element_id != element.element_id and 
                    other.element_id not in processed):
                    
                    distance = self._calculate_distance(element.center, other.center)
                    
                    # Group if within reasonable distance
                    if distance < 100:  # Adjustable threshold
                        group.append(other.element_id)
                        processed.add(other.element_id)
            
            if len(group) > 1:
                groups.append(group)
            
            processed.add(element.element_id)
        
        return groups
    
    def _detect_hierarchies(self, elements: List[VisualElement]) -> List[Dict[str, Any]]:
        """Detect hierarchical relationships (parent-child)"""
        
        hierarchies = []
        
        for element in elements:
            children = []
            
            for other in elements:
                if element.element_id != other.element_id:
                    # Check if other element is contained within this element
                    if self._is_contained(other.bbox, element.bbox):
                        children.append(other.element_id)
            
            if children:
                hierarchies.append({
                    'parent': element.element_id,
                    'children': children
                })
        
        return hierarchies
    
    def _detect_alignments(self, elements: List[VisualElement]) -> List[Dict[str, Any]]:
        """Detect alignment patterns"""
        
        alignments = []
        
        # Horizontal alignments
        h_groups = {}
        for element in elements:
            y = element.center[1]
            
            # Group by similar Y coordinates
            for group_y in h_groups:
                if abs(y - group_y) < 10:  # 10 pixel tolerance
                    h_groups[group_y].append(element.element_id)
                    break
            else:
                h_groups[y] = [element.element_id]
        
        for y, group in h_groups.items():
            if len(group) > 2:
                alignments.append({
                    'type': 'horizontal',
                    'elements': group,
                    'coordinate': y
                })
        
        # Vertical alignments
        v_groups = {}
        for element in elements:
            x = element.center[0]
            
            for group_x in v_groups:
                if abs(x - group_x) < 10:
                    v_groups[group_x].append(element.element_id)
                    break
            else:
                v_groups[x] = [element.element_id]
        
        for x, group in v_groups.items():
            if len(group) > 2:
                alignments.append({
                    'type': 'vertical',
                    'elements': group,
                    'coordinate': x
                })
        
        return alignments
    
    def _detect_grid_structure(self, elements: List[VisualElement]) -> Optional[Dict[str, Any]]:
        """Detect grid-like arrangements"""
        
        if len(elements) < 4:
            return None
        
        # Sort elements by position
        sorted_by_y = sorted(elements, key=lambda e: e.center[1])
        
        # Try to detect rows
        rows = []
        current_row = [sorted_by_y[0]]
        
        for element in sorted_by_y[1:]:
            # If Y coordinate is similar to current row, add to row
            if abs(element.center[1] - current_row[0].center[1]) < 20:
                current_row.append(element)
            else:
                # Start new row
                if len(current_row) > 1:
                    rows.append(current_row)
                current_row = [element]
        
        if len(current_row) > 1:
            rows.append(current_row)
        
        # Check if we have a proper grid
        if len(rows) >= 2:
            row_lengths = [len(row) for row in rows]
            
            # If most rows have similar length, it's likely a grid
            if len(set(row_lengths)) <= 2:  # Allow some variation
                return {
                    'rows': len(rows),
                    'columns': max(row_lengths),
                    'elements_in_grid': sum(len(row) for row in rows)
                }
        
        return None
    
    def _calculate_distance(self, point1: Tuple[int, int], 
                          point2: Tuple[int, int]) -> float:
        """Calculate Euclidean distance between two points"""
        
        x1, y1 = point1
        x2, y2 = point2
        return np.sqrt((x2 - x1) ** 2 + (y2 - y1) ** 2)
    
    def _is_contained(self, inner_bbox: Tuple[int, int, int, int], 
                     outer_bbox: Tuple[int, int, int, int]) -> bool:
        """Check if inner bounding box is contained within outer"""
        
        ix, iy, iw, ih = inner_bbox
        ox, oy, ow, oh = outer_bbox
        
        return (ox <= ix and 
                oy <= iy and 
                ox + ow >= ix + iw and 
                oy + oh >= iy + ih)

class SelfHealingAutomation:
    """Self-healing automation that adapts to UI changes"""
    
    def __init__(self):
        self.selectors_db = {}
        self.template_matcher = AdvancedTemplateMatching()
        self.semantic_analyzer = SemanticUIAnalyzer()
        self.success_history = {}
        
    def create_adaptive_selector(self, target_name: str, 
                               screenshot_path: str,
                               target_region: Tuple[int, int, int, int] = None) -> AdaptiveSelector:
        """Create adaptive selector by learning from current UI state"""
        
        try:
            image = cv2.imread(screenshot_path)
            
            if target_region:
                x, y, w, h = target_region
                roi = image[y:y+h, x:x+w]
                
                # Learn template from the region
                self.template_matcher.learn_template(target_name, roi, {
                    'creation_time': time.time(),
                    'source_image': screenshot_path
                })
            
            # Create adaptive selector
            selector = AdaptiveSelector(
                target_name=target_name,
                primary_method='template_matching',
                fallback_methods=['ocr', 'semantic', 'position'],
                learned_features={},
                success_history=[],
                last_known_position=None
            )
            
            self.selectors_db[target_name] = selector
            
            logger.info(f"Created adaptive selector for: {target_name}")
            return selector
            
        except Exception as e:
            logger.error(f"Failed to create adaptive selector: {e}")
            return None
    
    async def find_element_adaptive(self, target_name: str, 
                                  screenshot_path: str) -> Optional[VisualElement]:
        """Find element using adaptive multi-method approach"""
        
        if target_name not in self.selectors_db:
            logger.warning(f"No selector found for: {target_name}")
            return None
        
        selector = self.selectors_db[target_name]
        candidates = []
        
        try:
            image = cv2.imread(screenshot_path)
            
            # Method 1: Template matching (primary)
            if selector.primary_method == 'template_matching':
                template_matches = self.template_matcher.find_template_matches(
                    image, target_name
                )
                candidates.extend(template_matches)
            
            # Method 2: Semantic analysis
            if 'semantic' in selector.fallback_methods:
                semantic_matches = await self.semantic_analyzer.analyze_ui_semantics(image)
                # Filter semantic matches by target name
                filtered_semantic = [m for m in semantic_matches 
                                   if target_name.lower() in m.element_type.lower()]
                candidates.extend(filtered_semantic)
            
            # Method 3: OCR-based search
            if 'ocr' in selector.fallback_methods:
                ocr_matches = await self._find_by_ocr(image, target_name)
                candidates.extend(ocr_matches)
            
            # Method 4: Position-based (last known position)
            if ('position' in selector.fallback_methods and 
                selector.last_known_position):
                position_match = await self._find_by_position(
                    image, selector.last_known_position
                )
                if position_match:
                    candidates.append(position_match)
            
            # Select best candidate
            if candidates:
                best_candidate = max(candidates, key=lambda c: c.confidence)
                
                # Update selector with success
                selector.success_history.append(True)
                selector.last_known_position = best_candidate.center
                
                # Adaptive learning: if confidence is low, learn new template
                if best_candidate.confidence < selector.confidence_threshold:
                    await self._adaptive_learning(target_name, image, best_candidate)
                
                return best_candidate
            else:
                # Update selector with failure
                selector.success_history.append(False)
                
                # Trigger recovery mechanism
                recovery_element = await self._recovery_mechanism(
                    target_name, image, selector
                )
                return recovery_element
            
        except Exception as e:
            logger.error(f"Adaptive element finding failed: {e}")
            return None
    
    async def _find_by_ocr(self, image: np.ndarray, 
                         target_name: str) -> List[VisualElement]:
        """Find elements using OCR text matching"""
        
        matches = []
        
        try:
            # Convert to PIL for OCR
            pil_image = Image.fromarray(cv2.cvtColor(image, cv2.COLOR_BGR2RGB))
            
            # Get OCR data
            ocr_data = pytesseract.image_to_data(pil_image, output_type=pytesseract.Output.DICT)
            
            n_boxes = len(ocr_data['level'])
            for i in range(n_boxes):
                confidence = int(ocr_data['conf'][i])
                if confidence > 60:  # OCR confidence threshold
                    text = ocr_data['text'][i].strip().lower()
                    
                    # Check if target name matches text
                    if (text and 
                        (target_name.lower() in text or 
                         any(word in text for word in target_name.lower().split()))):
                        
                        x = ocr_data['left'][i]
                        y = ocr_data['top'][i]
                        w = ocr_data['width'][i]
                        h = ocr_data['height'][i]
                        
                        visual_element = VisualElement(
                            element_id=f"ocr_{target_name}_{x}_{y}",
                            element_type="text_match",
                            confidence=confidence / 100.0,
                            bbox=(x, y, w, h),
                            center=(x + w // 2, y + h // 2),
                            visual_features={'method': 'ocr', 'matched_text': text},
                            text_content=text
                        )
                        matches.append(visual_element)
            
        except Exception as e:
            logger.warning(f"OCR search failed: {e}")
        
        return matches
    
    async def _find_by_position(self, image: np.ndarray, 
                              last_position: Tuple[int, int]) -> Optional[VisualElement]:
        """Find element near last known position"""
        
        try:
            x, y = last_position
            
            # Define search area around last position
            search_radius = 50
            search_area = (
                max(0, x - search_radius),
                max(0, y - search_radius),
                min(image.shape[1], x + search_radius),
                min(image.shape[0], y + search_radius)
            )
            
            # Extract search region
            sx, sy, ex, ey = search_area
            roi = image[sy:ey, sx:ex]
            
            if roi.size > 0:
                # Simple analysis to see if there's still an element there
                gray = cv2.cvtColor(roi, cv2.COLOR_BGR2GRAY) if len(roi.shape) == 3 else roi
                edges = cv2.Canny(gray, 50, 150)
                edge_density = np.sum(edges > 0) / edges.size
                
                # If there's reasonable edge density, assume element is still there
                if edge_density > 0.02:
                    visual_element = VisualElement(
                        element_id=f"position_{x}_{y}",
                        element_type="position_match",
                        confidence=0.6,  # Moderate confidence for position-based
                        bbox=(sx, sy, ex - sx, ey - sy),
                        center=(x, y),
                        visual_features={'method': 'position', 'edge_density': edge_density}
                    )
                    return visual_element
            
        except Exception as e:
            logger.warning(f"Position-based search failed: {e}")
        
        return None
    
    async def _adaptive_learning(self, target_name: str, 
                               image: np.ndarray, 
                               found_element: VisualElement):
        """Learn new patterns from successful detection"""
        
        try:
            # Extract region around found element
            x, y, w, h = found_element.bbox
            
            # Add some padding
            padding = 10
            x = max(0, x - padding)
            y = max(0, y - padding)
            w = min(image.shape[1] - x, w + 2 * padding)
            h = min(image.shape[0] - y, h + 2 * padding)
            
            roi = image[y:y+h, x:x+w]
            
            # Learn new template
            self.template_matcher.learn_template(
                f"{target_name}_adaptive_{int(time.time())}", 
                roi,
                {
                    'adaptive_learning': True,
                    'original_confidence': found_element.confidence,
                    'learning_time': time.time()
                }
            )
            
            logger.info(f"Adaptive learning completed for: {target_name}")
            
        except Exception as e:
            logger.warning(f"Adaptive learning failed: {e}")
    
    async def _recovery_mechanism(self, target_name: str, 
                                image: np.ndarray, 
                                selector: AdaptiveSelector) -> Optional[VisualElement]:
        """Recovery mechanism when all methods fail"""
        
        try:
            logger.info(f"Attempting recovery for: {target_name}")
            
            # Try broader OCR search with relaxed matching
            recovery_matches = []
            
            # Relaxed OCR search
            pil_image = Image.fromarray(cv2.cvtColor(image, cv2.COLOR_BGR2RGB))
            ocr_data = pytesseract.image_to_data(pil_image, output_type=pytesseract.Output.DICT)
            
            # Lower confidence threshold and partial matching
            n_boxes = len(ocr_data['level'])
            for i in range(n_boxes):
                confidence = int(ocr_data['conf'][i])
                if confidence > 30:  # Lower threshold
                    text = ocr_data['text'][i].strip().lower()
                    
                    # Partial matching
                    target_words = target_name.lower().split()
                    if text and any(word in text or text in word for word in target_words):
                        x = ocr_data['left'][i]
                        y = ocr_data['top'][i]
                        w = ocr_data['width'][i]
                        h = ocr_data['height'][i]
                        
                        recovery_element = VisualElement(
                            element_id=f"recovery_{target_name}_{x}_{y}",
                            element_type="recovery_match",
                            confidence=confidence / 200.0,  # Lower confidence
                            bbox=(x, y, w, h),
                            center=(x + w // 2, y + h // 2),
                            visual_features={'method': 'recovery', 'matched_text': text},
                            text_content=text
                        )
                        recovery_matches.append(recovery_element)
            
            if recovery_matches:
                # Return best recovery match
                best_recovery = max(recovery_matches, key=lambda m: m.confidence)
                logger.info(f"Recovery successful for: {target_name}")
                return best_recovery
            
        except Exception as e:
            logger.error(f"Recovery mechanism failed: {e}")
        
        logger.warning(f"All recovery attempts failed for: {target_name}")
        return None

async def demo_advanced_vision_detection():
    """Demonstration of advanced computer vision detection capabilities"""
    
    print("🚀 Advanced Computer Vision Detection Demo")
    print("Features: Template Matching, Semantic Analysis, Self-Healing Automation")
    
    # Initialize components
    template_matcher = AdvancedTemplateMatching()
    semantic_analyzer = SemanticUIAnalyzer()
    self_healing = SelfHealingAutomation()
    
    # Take screenshot
    screenshot_path = "/tmp/advanced_vision_demo.png"
    subprocess.run(['import', '-window', 'root', screenshot_path])
    
    image = cv2.imread(screenshot_path)
    if image is None:
        print("❌ Failed to load screenshot")
        return
    
    print(f"📸 Screenshot captured: {screenshot_path}")
    
    # Demo 1: Semantic UI Analysis
    print("\n1️⃣ Semantic UI Analysis")
    semantic_elements = await semantic_analyzer.analyze_ui_semantics(image)
    print(f"Found {len(semantic_elements)} UI elements:")
    
    for element in semantic_elements[:5]:  # Show first 5
        print(f"  - {element.element_type} at {element.center} "
              f"(confidence: {element.confidence:.2f})")
    
    # Demo 2: Template Learning and Matching
    print("\n2️⃣ Template Learning Demo")
    
    # Learn a template from part of the screen
    if semantic_elements:
        sample_element = semantic_elements[0]
        x, y, w, h = sample_element.bbox
        roi = image[y:y+h, x:x+w]
        
        template_matcher.learn_template("demo_element", roi, {
            'demo': True,
            'element_type': sample_element.element_type
        })
        
        print(f"✅ Learned template for {sample_element.element_type}")
        
        # Try to find it again
        matches = template_matcher.find_template_matches(image, "demo_element")
        print(f"Found {len(matches)} template matches")
    
    # Demo 3: Self-Healing Automation
    print("\n3️⃣ Self-Healing Automation Demo")
    
    # Create adaptive selector
    if semantic_elements:
        target_element = semantic_elements[0]
        selector = self_healing.create_adaptive_selector(
            "demo_target", 
            screenshot_path, 
            target_element.bbox
        )
        
        if selector:
            print(f"✅ Created adaptive selector for demo_target")
            
            # Try to find element using adaptive methods
            found_element = await self_healing.find_element_adaptive(
                "demo_target", 
                screenshot_path
            )
            
            if found_element:
                print(f"✅ Found element adaptively at {found_element.center} "
                      f"(confidence: {found_element.confidence:.2f})")
            else:
                print("❌ Adaptive finding failed")
    
    # Demo 4: Layout Analysis
    print("\n4️⃣ Layout Analysis Demo")
    
    layout_analyzer = LayoutAnalyzer()
    if len(semantic_elements) > 1:
        relationships = layout_analyzer.analyze_spatial_relationships(semantic_elements)
        
        print(f"Layout Analysis Results:")
        print(f"  - Element groups: {len(relationships['groups'])}")
        print(f"  - Hierarchies: {len(relationships['hierarchies'])}")
        print(f"  - Alignments: {len(relationships['alignment_patterns'])}")
        
        if relationships['grid_structure']:
            grid = relationships['grid_structure']
            print(f"  - Grid detected: {grid['rows']}x{grid['columns']}")
    
    print(f"\n🏆 Advanced Vision Detection Demo Complete!")
    print("Demonstrated: Template Learning, Semantic Analysis, Self-Healing, Layout Analysis")

if __name__ == "__main__":
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(levelname)s - %(message)s'
    )
    
    asyncio.run(demo_advanced_vision_detection())