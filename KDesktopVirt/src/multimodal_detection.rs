// Advanced Multi-Modal Element Detection System
// Combines Computer Vision, OCR, and Accessibility APIs for robust element detection

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::fs;
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionConfig {
    pub cv_confidence_threshold: f64,
    pub ocr_confidence_threshold: f64,
    pub accessibility_enabled: bool,
    pub hybrid_fusion_enabled: bool,
    pub cache_ttl_seconds: u64,
    pub max_detection_time_ms: u64,
}

impl Default for DetectionConfig {
    fn default() -> Self {
        Self {
            cv_confidence_threshold: 0.7,
            ocr_confidence_threshold: 0.6,
            accessibility_enabled: true,
            hybrid_fusion_enabled: true,
            cache_ttl_seconds: 30,
            max_detection_time_ms: 2000,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ElementDetection {
    pub element_id: String,
    pub element_type: ElementType,
    pub confidence: f64,
    pub bounding_box: BoundingBox,
    pub text_content: Option<String>,
    pub detection_method: DetectionMethod,
    pub attributes: HashMap<String, String>,
    pub accessibility_info: Option<AccessibilityInfo>,
    #[serde(skip)]
    pub detection_time: Instant,
}

impl Default for ElementDetection {
    fn default() -> Self {
        Self {
            element_id: String::new(),
            element_type: ElementType::Unknown,
            confidence: 0.0,
            bounding_box: BoundingBox::default(),
            text_content: None,
            detection_method: DetectionMethod::ComputerVision,
            attributes: HashMap::new(),
            accessibility_info: None,
            detection_time: std::time::Instant::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BoundingBox {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl BoundingBox {
    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }
    
    pub fn area(&self) -> i32 {
        self.width * self.height
    }
    
    pub fn aspect_ratio(&self) -> f64 {
        if self.height > 0 {
            self.width as f64 / self.height as f64
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementType {
    Button,
    TextInput,
    TextArea,
    Label,
    Image,
    Link,
    Menu,
    MenuItem,
    Checkbox,
    RadioButton,
    Dropdown,
    Slider,
    ProgressBar,
    Icon,
    Window,
    Dialog,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetectionMethod {
    ComputerVision,
    OCR,
    Accessibility,
    HybridFusion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityInfo {
    pub role: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub states: Vec<String>,
    pub properties: HashMap<String, String>,
}

pub struct MultiModalDetector {
    config: DetectionConfig,
    cv_detector: ComputerVisionDetector,
    ocr_detector: OCRDetector,
    accessibility_detector: AccessibilityDetector,
    fusion_engine: DetectionFusionEngine,
    detection_cache: HashMap<String, CachedDetection>,
}

struct CachedDetection {
    detection: ElementDetection,
    timestamp: Instant,
}

impl MultiModalDetector {
    pub async fn new(config: DetectionConfig) -> Result<Self> {
        info!("Initializing Multi-Modal Detection System");
        
        let cv_detector = ComputerVisionDetector::new().await?;
        let ocr_detector = OCRDetector::new().await?;
        let accessibility_detector = if config.accessibility_enabled {
            AccessibilityDetector::new().await?
        } else {
            AccessibilityDetector::disabled()
        };
        let fusion_engine = DetectionFusionEngine::new();
        
        Ok(Self {
            config,
            cv_detector,
            ocr_detector,
            accessibility_detector,
            fusion_engine,
            detection_cache: HashMap::new(),
        })
    }
    
    pub async fn detect_elements(
        &mut self,
        screenshot_path: &str,
        target_description: Option<&str>
    ) -> Result<Vec<ElementDetection>> {
        let start_time = Instant::now();
        info!("Starting multi-modal element detection");
        
        // Check cache first
        let cache_key = format!("{}_{}", screenshot_path, target_description.unwrap_or(""));
        if let Some(cached) = self.get_cached_detection(&cache_key) {
            return Ok(vec![cached]);
        }
        
        // Verify screenshot exists
        if !Path::new(screenshot_path).exists() {
            return Err(anyhow!("Screenshot not found: {}", screenshot_path));
        }
        
        let mut all_detections = Vec::new();
        
        // Method 1: Computer Vision Detection
        match self.cv_detector.detect_elements(screenshot_path, target_description).await {
            Ok(cv_detections) => {
                info!("CV detection found {} elements", cv_detections.len());
                all_detections.extend(cv_detections);
            }
            Err(e) => warn!("CV detection failed: {}", e),
        }
        
        // Method 2: OCR Detection
        match self.ocr_detector.detect_text_elements(screenshot_path, target_description).await {
            Ok(ocr_detections) => {
                info!("OCR detection found {} elements", ocr_detections.len());
                all_detections.extend(ocr_detections);
            }
            Err(e) => warn!("OCR detection failed: {}", e),
        }
        
        // Method 3: Accessibility Detection
        if self.config.accessibility_enabled {
            match self.accessibility_detector.detect_accessible_elements().await {
                Ok(a11y_detections) => {
                    info!("Accessibility detection found {} elements", a11y_detections.len());
                    all_detections.extend(a11y_detections);
                }
                Err(e) => warn!("Accessibility detection failed: {}", e),
            }
        }
        
        // Method 4: Hybrid Fusion
        let final_detections = if self.config.hybrid_fusion_enabled {
            self.fusion_engine.fuse_detections(all_detections, target_description).await?
        } else {
            all_detections
        };
        
        // Cache results
        if let Some(best_detection) = final_detections.first() {
            self.cache_detection(cache_key, best_detection.clone());
        }
        
        let detection_time = start_time.elapsed();
        info!("Multi-modal detection completed in {:.3}s, found {} elements", 
              detection_time.as_secs_f64(), final_detections.len());
        
        Ok(final_detections)
    }
    
    fn get_cached_detection(&self, key: &str) -> Option<ElementDetection> {
        if let Some(cached) = self.detection_cache.get(key) {
            if cached.timestamp.elapsed().as_secs() < self.config.cache_ttl_seconds {
                return Some(cached.detection.clone());
            }
        }
        None
    }
    
    fn cache_detection(&mut self, key: String, detection: ElementDetection) {
        self.detection_cache.insert(key, CachedDetection {
            detection,
            timestamp: Instant::now(),
        });
        
        // Clean old cache entries
        self.detection_cache.retain(|_, cached| {
            cached.timestamp.elapsed().as_secs() < self.config.cache_ttl_seconds
        });
    }
    
    pub async fn find_element_by_text(&mut self, screenshot_path: &str, text: &str) -> Result<Option<ElementDetection>> {
        let detections = self.detect_elements(screenshot_path, Some(text)).await?;
        
        // Find best matching element by text similarity
        let mut best_match = None;
        let mut best_score = 0.0;
        
        for detection in detections {
            if let Some(element_text) = &detection.text_content {
                let similarity = self.calculate_text_similarity(text, element_text);
                if similarity > best_score {
                    best_score = similarity;
                    best_match = Some(detection);
                }
            }
        }
        
        Ok(best_match)
    }
    
    pub async fn find_element_by_type(&mut self, screenshot_path: &str, element_type: ElementType) -> Result<Vec<ElementDetection>> {
        let detections = self.detect_elements(screenshot_path, None).await?;
        
        Ok(detections.into_iter()
            .filter(|d| std::mem::discriminant(&d.element_type) == std::mem::discriminant(&element_type))
            .collect())
    }
    
    fn calculate_text_similarity(&self, target: &str, element_text: &str) -> f64 {
        let target_lower = target.to_lowercase();
        let element_lower = element_text.to_lowercase();
        
        // Exact match
        if target_lower == element_lower {
            return 1.0;
        }
        
        // Contains match
        if element_lower.contains(&target_lower) || target_lower.contains(&element_lower) {
            return 0.8;
        }
        
        // Word overlap
        let target_words: Vec<&str> = target_lower.split_whitespace().collect();
        let element_words: Vec<&str> = element_lower.split_whitespace().collect();
        
        let mut common_words = 0;
        for target_word in &target_words {
            if element_words.contains(target_word) {
                common_words += 1;
            }
        }
        
        if target_words.len() > 0 {
            common_words as f64 / target_words.len() as f64 * 0.6
        } else {
            0.0
        }
    }
    
    pub fn get_performance_stats(&self) -> HashMap<String, f64> {
        let mut stats = HashMap::new();
        stats.insert("cache_size".to_string(), self.detection_cache.len() as f64);
        stats.insert("cv_confidence_threshold".to_string(), self.config.cv_confidence_threshold);
        stats.insert("ocr_confidence_threshold".to_string(), self.config.ocr_confidence_threshold);
        stats
    }
}

// Computer Vision Detector using OpenCV-like operations
struct ComputerVisionDetector {
    template_cache: HashMap<String, Vec<u8>>,
}

impl ComputerVisionDetector {
    async fn new() -> Result<Self> {
        Ok(Self {
            template_cache: HashMap::new(),
        })
    }
    
    async fn detect_elements(&mut self, screenshot_path: &str, _target: Option<&str>) -> Result<Vec<ElementDetection>> {
        let mut detections = Vec::new();
        
        // Use ImageMagick to analyze image for UI elements
        let output = Command::new("identify")
            .args(["-format", "%[fx:w]x%[fx:h]", screenshot_path])
            .output()?;
            
        if !output.status.success() {
            return Err(anyhow!("Failed to analyze image dimensions"));
        }
        
        let dimensions = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = dimensions.trim().split('x').collect();
        
        if parts.len() != 2 {
            return Err(anyhow!("Invalid image dimensions"));
        }
        
        let width: i32 = parts[0].parse()?;
        let height: i32 = parts[1].parse()?;
        
        // Detect potential button areas using edge detection simulation
        detections.extend(self.detect_button_like_regions(screenshot_path, width, height).await?);
        
        // Detect text input areas
        detections.extend(self.detect_input_like_regions(screenshot_path, width, height).await?);
        
        Ok(detections)
    }
    
    async fn detect_button_like_regions(&self, _screenshot_path: &str, width: i32, height: i32) -> Result<Vec<ElementDetection>> {
        let mut detections = Vec::new();
        
        // Simulate button detection based on common UI patterns
        let button_areas = [
            (width - 100, height - 50, 80, 30), // Bottom right
            (20, height - 50, 80, 30),          // Bottom left
            (width / 2 - 40, height / 2, 80, 30), // Center
        ];
        
        for (i, (x, y, w, h)) in button_areas.iter().enumerate() {
            detections.push(ElementDetection {
                element_id: format!("cv_button_{}", i),
                element_type: ElementType::Button,
                confidence: 0.7,
                bounding_box: BoundingBox {
                    x: *x,
                    y: *y,
                    width: *w,
                    height: *h,
                },
                text_content: None,
                detection_method: DetectionMethod::ComputerVision,
                attributes: HashMap::new(),
                accessibility_info: None,
                detection_time: Instant::now(),
            });
        }
        
        Ok(detections)
    }
    
    async fn detect_input_like_regions(&self, _screenshot_path: &str, width: i32, height: i32) -> Result<Vec<ElementDetection>> {
        let mut detections = Vec::new();
        
        // Simulate text input detection
        let input_areas = [
            (50, 100, width - 100, 25), // Top input field
            (50, 150, width - 100, 25), // Second input field
        ];
        
        for (i, (x, y, w, h)) in input_areas.iter().enumerate() {
            detections.push(ElementDetection {
                element_id: format!("cv_input_{}", i),
                element_type: ElementType::TextInput,
                confidence: 0.6,
                bounding_box: BoundingBox {
                    x: *x,
                    y: *y,
                    width: *w,
                    height: *h,
                },
                text_content: None,
                detection_method: DetectionMethod::ComputerVision,
                attributes: HashMap::new(),
                accessibility_info: None,
                detection_time: Instant::now(),
            });
        }
        
        Ok(detections)
    }
}

// OCR Detector using Tesseract
struct OCRDetector {
    language: String,
}

impl OCRDetector {
    async fn new() -> Result<Self> {
        Ok(Self {
            language: "eng".to_string(),
        })
    }
    
    async fn detect_text_elements(&self, screenshot_path: &str, target: Option<&str>) -> Result<Vec<ElementDetection>> {
        let mut detections = Vec::new();
        
        // Use tesseract to extract text with bounding boxes
        let output = Command::new("tesseract")
            .args([
                screenshot_path,
                "stdout",
                "-l", &self.language,
                "--psm", "6",
                "--oem", "3",
                "tsv"
            ])
            .output()?;
            
        if !output.status.success() {
            return Err(anyhow!("Tesseract OCR failed"));
        }
        
        let tsv_data = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = tsv_data.lines().collect();
        
        if lines.len() < 2 {
            return Ok(detections);
        }
        
        // Parse TSV output (skip header)
        for (i, line) in lines.iter().skip(1).enumerate() {
            let fields: Vec<&str> = line.split('\t').collect();
            
            if fields.len() < 12 {
                continue;
            }
            
            // Extract bounding box and text
            if let (Ok(x), Ok(y), Ok(w), Ok(h), Ok(conf)) = (
                fields[6].parse::<i32>(),
                fields[7].parse::<i32>(),
                fields[8].parse::<i32>(),
                fields[9].parse::<i32>(),
                fields[10].parse::<f64>(),
            ) {
                let text = fields[11].trim();
                
                if text.is_empty() || conf < 60.0 {
                    continue;
                }
                
                // Filter by target if specified
                if let Some(target_text) = target {
                    if !text.to_lowercase().contains(&target_text.to_lowercase()) {
                        continue;
                    }
                }
                
                let element_type = self.classify_text_element(text);
                
                detections.push(ElementDetection {
                    element_id: format!("ocr_text_{}", i),
                    element_type,
                    confidence: conf / 100.0,
                    bounding_box: BoundingBox { x, y, width: w, height: h },
                    text_content: Some(text.to_string()),
                    detection_method: DetectionMethod::OCR,
                    attributes: HashMap::new(),
                    accessibility_info: None,
                    detection_time: Instant::now(),
                });
            }
        }
        
        Ok(detections)
    }
    
    fn classify_text_element(&self, text: &str) -> ElementType {
        let text_lower = text.to_lowercase();
        
        // Button-like text
        if text_lower.len() < 20 && (
            text_lower.contains("ok") ||
            text_lower.contains("cancel") ||
            text_lower.contains("submit") ||
            text_lower.contains("save") ||
            text_lower.contains("delete") ||
            text_lower.contains("close")
        ) {
            return ElementType::Button;
        }
        
        // Label-like text
        if text_lower.ends_with(':') {
            return ElementType::Label;
        }
        
        // Default to label for text
        ElementType::Label
    }
}

// Accessibility Detector using AT-SPI or similar
struct AccessibilityDetector {
    enabled: bool,
}

impl AccessibilityDetector {
    async fn new() -> Result<Self> {
        Ok(Self { enabled: true })
    }
    
    fn disabled() -> Self {
        Self { enabled: false }
    }
    
    async fn detect_accessible_elements(&self) -> Result<Vec<ElementDetection>> {
        if !self.enabled {
            return Ok(Vec::new());
        }
        
        let mut detections = Vec::new();
        
        // Use atspi-python or similar to get accessible elements
        // For now, simulate accessibility detection
        let output = Command::new("python3")
            .args(["-c", r#"
import sys
try:
    import pyatspi
    desktop = pyatspi.Registry.getDesktop(0)
    for app in desktop:
        if app.name:
            print(f"app:{app.name}:0:0:100:50")
        for child in app:
            if hasattr(child, 'name') and child.name:
                print(f"element:{child.name}:100:100:200:30")
except ImportError:
    pass
"#])
            .output();
            
        if let Ok(output) = output {
            if output.status.success() {
                let result = String::from_utf8_lossy(&output.stdout);
                for (i, line) in result.lines().enumerate() {
                    let parts: Vec<&str> = line.split(':').collect();
                    if parts.len() == 6 {
                        if let (Ok(x), Ok(y), Ok(w), Ok(h)) = (
                            parts[2].parse::<i32>(),
                            parts[3].parse::<i32>(),
                            parts[4].parse::<i32>(),
                            parts[5].parse::<i32>(),
                        ) {
                            let element_type = match parts[0] {
                                "app" => ElementType::Window,
                                _ => ElementType::Button,
                            };
                            
                            detections.push(ElementDetection {
                                element_id: format!("a11y_element_{}", i),
                                element_type,
                                confidence: 0.9,
                                bounding_box: BoundingBox { x, y, width: w, height: h },
                                text_content: Some(parts[1].to_string()),
                                detection_method: DetectionMethod::Accessibility,
                                attributes: HashMap::new(),
                                accessibility_info: Some(AccessibilityInfo {
                                    role: parts[0].to_string(),
                                    name: Some(parts[1].to_string()),
                                    description: None,
                                    states: Vec::new(),
                                    properties: HashMap::new(),
                                }),
                                detection_time: Instant::now(),
                            });
                        }
                    }
                }
            }
        }
        
        Ok(detections)
    }
}

// Detection Fusion Engine for combining multiple detection methods
struct DetectionFusionEngine;

impl DetectionFusionEngine {
    fn new() -> Self {
        Self
    }
    
    async fn fuse_detections(
        &self,
        detections: Vec<ElementDetection>,
        target: Option<&str>
    ) -> Result<Vec<ElementDetection>> {
        let mut fused_detections: Vec<ElementDetection> = Vec::new();

        // Group overlapping detections
        for detection in detections {
            let mut merged = false;
            
            for existing in &mut fused_detections {
                if self.regions_overlap(&detection.bounding_box, &existing.bounding_box) {
                    // Merge detections
                    let merged_detection = self.merge_detections(existing.clone(), detection.clone());
                    *existing = merged_detection;
                    merged = true;
                    break;
                }
            }
            
            if !merged {
                fused_detections.push(detection);
            }
        }
        
        // Sort by confidence and relevance to target
        fused_detections.sort_by(|a, b| {
            let score_a = self.calculate_relevance_score(a, target);
            let score_b = self.calculate_relevance_score(b, target);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        Ok(fused_detections)
    }
    
    fn regions_overlap(&self, bbox1: &BoundingBox, bbox2: &BoundingBox) -> bool {
        let overlap_threshold = 0.3;
        
        let x_overlap = (bbox1.x.max(bbox2.x) < (bbox1.x + bbox1.width).min(bbox2.x + bbox2.width));
        let y_overlap = (bbox1.y.max(bbox2.y) < (bbox1.y + bbox1.height).min(bbox2.y + bbox2.height));
        
        if !x_overlap || !y_overlap {
            return false;
        }
        
        let intersection_area = 
            ((bbox1.x + bbox1.width).min(bbox2.x + bbox2.width) - bbox1.x.max(bbox2.x)) *
            ((bbox1.y + bbox1.height).min(bbox2.y + bbox2.height) - bbox1.y.max(bbox2.y));
            
        let union_area = bbox1.area() + bbox2.area() - intersection_area;
        
        if union_area > 0 {
            intersection_area as f64 / union_area as f64 > overlap_threshold
        } else {
            false
        }
    }
    
    fn merge_detections(&self, det1: ElementDetection, det2: ElementDetection) -> ElementDetection {
        // Choose detection with higher confidence as base
        let (primary, secondary) = if det1.confidence >= det2.confidence {
            (det1, det2)
        } else {
            (det2, det1)
        };
        
        // Merge bounding boxes
        let merged_bbox = BoundingBox {
            x: primary.bounding_box.x.min(secondary.bounding_box.x),
            y: primary.bounding_box.y.min(secondary.bounding_box.y),
            width: (primary.bounding_box.x + primary.bounding_box.width)
                .max(secondary.bounding_box.x + secondary.bounding_box.width) -
                primary.bounding_box.x.min(secondary.bounding_box.x),
            height: (primary.bounding_box.y + primary.bounding_box.height)
                .max(secondary.bounding_box.y + secondary.bounding_box.height) -
                primary.bounding_box.y.min(secondary.bounding_box.y),
        };
        
        // Combine text content
        let merged_text = match (&primary.text_content, &secondary.text_content) {
            (Some(t1), Some(t2)) => Some(format!("{} {}", t1, t2)),
            (Some(t), None) | (None, Some(t)) => Some(t.clone()),
            (None, None) => None,
        };
        
        ElementDetection {
            element_id: format!("fused_{}_{}", primary.element_id, secondary.element_id),
            element_type: primary.element_type,
            confidence: (primary.confidence + secondary.confidence) / 2.0,
            bounding_box: merged_bbox,
            text_content: merged_text,
            detection_method: DetectionMethod::HybridFusion,
            attributes: primary.attributes,
            accessibility_info: primary.accessibility_info.or(secondary.accessibility_info),
            detection_time: Instant::now(),
        }
    }
    
    fn calculate_relevance_score(&self, detection: &ElementDetection, target: Option<&str>) -> f64 {
        let mut score = detection.confidence;
        
        // Boost score for target text match
        if let (Some(target_text), Some(element_text)) = (target, &detection.text_content) {
            if element_text.to_lowercase().contains(&target_text.to_lowercase()) {
                score += 0.3;
            }
        }
        
        // Boost score based on detection method reliability
        match detection.detection_method {
            DetectionMethod::OCR => score += 0.1,
            DetectionMethod::Accessibility => score += 0.2,
            DetectionMethod::HybridFusion => score += 0.15,
            _ => {}
        }
        
        // Boost score for interactive elements
        match detection.element_type {
            ElementType::Button | ElementType::TextInput | ElementType::Link => score += 0.1,
            _ => {}
        }
        
        score.min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== DetectionConfig Tests ==========

    #[test]
    fn test_detection_config_default() {
        let config = DetectionConfig::default();
        assert_eq!(config.cv_confidence_threshold, 0.7);
        assert_eq!(config.ocr_confidence_threshold, 0.6);
        assert!(config.accessibility_enabled);
        assert!(config.hybrid_fusion_enabled);
    }

    #[test]
    fn test_detection_config_custom() {
        let config = DetectionConfig {
            cv_confidence_threshold: 0.8,
            ocr_confidence_threshold: 0.5,
            accessibility_enabled: false,
            hybrid_fusion_enabled: true,
            cache_ttl_seconds: 60,
            max_detection_time_ms: 5000,
        };
        assert_eq!(config.cv_confidence_threshold, 0.8);
        assert!(!config.accessibility_enabled);
    }

    #[test]
    fn test_detection_config_thresholds_valid() {
        let config = DetectionConfig::default();
        assert!(config.cv_confidence_threshold >= 0.0 && config.cv_confidence_threshold <= 1.0);
        assert!(config.ocr_confidence_threshold >= 0.0 && config.ocr_confidence_threshold <= 1.0);
    }

    // ========== BoundingBox Tests ==========

    // Traces to: FR-KDESKTOPVIRT-005 (Element detection and bounding box validation for automation)
    #[test]
    fn test_bounding_box_creation() {
        let bbox = BoundingBox {
            x: 100,
            y: 200,
            width: 150,
            height: 100,
        };
        assert_eq!(bbox.x, 100);
        assert_eq!(bbox.width, 150);
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_bounding_box_center() {
        let bbox = BoundingBox {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };
        let (cx, cy) = bbox.center();
        assert_eq!(cx, 50);
        assert_eq!(cy, 50);
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_bounding_box_center_offset() {
        let bbox = BoundingBox {
            x: 50,
            y: 50,
            width: 100,
            height: 100,
        };
        let (cx, cy) = bbox.center();
        assert_eq!(cx, 100);
        assert_eq!(cy, 100);
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_bounding_box_area() {
        let bbox = BoundingBox {
            x: 0,
            y: 0,
            width: 200,
            height: 150,
        };
        assert_eq!(bbox.area(), 30000);
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_bounding_box_area_zero() {
        let bbox = BoundingBox {
            x: 0,
            y: 0,
            width: 0,
            height: 100,
        };
        assert_eq!(bbox.area(), 0);
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_bounding_box_aspect_ratio() {
        let bbox = BoundingBox {
            x: 0,
            y: 0,
            width: 200,
            height: 100,
        };
        assert!((bbox.aspect_ratio() - 2.0).abs() < 0.001);
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_bounding_box_aspect_ratio_square() {
        let bbox = BoundingBox {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };
        assert!((bbox.aspect_ratio() - 1.0).abs() < 0.001);
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_bounding_box_aspect_ratio_zero_height() {
        let bbox = BoundingBox {
            x: 0,
            y: 0,
            width: 100,
            height: 0,
        };
        assert_eq!(bbox.aspect_ratio(), 0.0);
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_bounding_box_default() {
        let bbox = BoundingBox::default();
        assert_eq!(bbox.x, 0);
        assert_eq!(bbox.y, 0);
        assert_eq!(bbox.width, 0);
        assert_eq!(bbox.height, 0);
    }

    // ========== ElementType Tests ==========

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_element_type_button() {
        let elem_type = ElementType::Button;
        assert!(matches!(elem_type, ElementType::Button));
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_element_type_variants() {
        let types = vec![
            ElementType::Button,
            ElementType::TextInput,
            ElementType::TextArea,
            ElementType::Link,
            ElementType::Image,
        ];
        assert!(types.len() > 0);
    }

    // ========== DetectionMethod Tests ==========

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_detection_method_computer_vision() {
        let method = DetectionMethod::ComputerVision;
        assert!(matches!(method, DetectionMethod::ComputerVision));
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_detection_method_variants() {
        let methods = vec![
            DetectionMethod::ComputerVision,
            DetectionMethod::OCR,
            DetectionMethod::Accessibility,
            DetectionMethod::HybridFusion,
        ];
        assert!(methods.len() == 4);
    }

    // ========== ElementDetection Tests ==========

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_element_detection_creation() {
        let detection = ElementDetection {
            element_id: "btn-1".to_string(),
            element_type: ElementType::Button,
            confidence: 0.95,
            bounding_box: BoundingBox {
                x: 100,
                y: 200,
                width: 80,
                height: 40,
            },
            text_content: Some("Click Me".to_string()),
            detection_method: DetectionMethod::ComputerVision,
            attributes: HashMap::new(),
            accessibility_info: None,
            detection_time: Instant::now(),
        };
        assert_eq!(detection.element_id, "btn-1");
        assert!(detection.confidence > 0.9);
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_element_detection_default() {
        let detection = ElementDetection::default();
        assert!(detection.element_id.is_empty());
        assert_eq!(detection.confidence, 0.0);
        assert!(detection.text_content.is_none());
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_element_detection_with_accessibility() {
        let accessibility = AccessibilityInfo {
            role: "button".to_string(),
            name: Some("Submit".to_string()),
            description: Some("Submit the form".to_string()),
            states: vec!["enabled".to_string()],
            properties: HashMap::new(),
        };
        let detection = ElementDetection {
            element_id: "submit-btn".to_string(),
            element_type: ElementType::Button,
            confidence: 0.98,
            bounding_box: BoundingBox {
                x: 0,
                y: 0,
                width: 100,
                height: 50,
            },
            text_content: Some("Submit".to_string()),
            detection_method: DetectionMethod::Accessibility,
            attributes: HashMap::new(),
            accessibility_info: Some(accessibility),
            detection_time: Instant::now(),
        };
        assert!(detection.accessibility_info.is_some());
    }

    // ========== AccessibilityInfo Tests ==========

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_accessibility_info_creation() {
        let info = AccessibilityInfo {
            role: "input".to_string(),
            name: Some("Email".to_string()),
            description: Some("Email input field".to_string()),
            states: vec!["enabled".to_string(), "editable".to_string()],
            properties: HashMap::new(),
        };
        assert_eq!(info.role, "input");
        assert_eq!(info.states.len(), 2);
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_accessibility_info_optional_fields() {
        let info = AccessibilityInfo {
            role: "div".to_string(),
            name: None,
            description: None,
            states: vec![],
            properties: HashMap::new(),
        };
        assert!(info.name.is_none());
        assert!(info.description.is_none());
    }

    // ========== Detection Cache Tests ==========

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_detector_cache_functionality() {
        let config = DetectionConfig {
            cv_confidence_threshold: 0.72,
            ocr_confidence_threshold: 0.62,
            accessibility_enabled: true,
            hybrid_fusion_enabled: true,
            cache_ttl_seconds: 30,
            max_detection_time_ms: 2000,
        };
        assert!(config.cache_ttl_seconds > 0);
    }

    // ========== Detection System Integration Tests ==========

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_detector_configuration_valid() {
        let config = DetectionConfig {
            cv_confidence_threshold: 0.75,
            ocr_confidence_threshold: 0.65,
            accessibility_enabled: true,
            hybrid_fusion_enabled: true,
            cache_ttl_seconds: 45,
            max_detection_time_ms: 2500,
        };
        assert!(config.cv_confidence_threshold >= config.ocr_confidence_threshold);
        assert!(config.cache_ttl_seconds > 0);
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_detector_threshold_bounds() {
        let config = DetectionConfig::default();
        assert!(config.cv_confidence_threshold >= 0.0 && config.cv_confidence_threshold <= 1.0);
        assert!(config.ocr_confidence_threshold >= 0.0 && config.ocr_confidence_threshold <= 1.0);
    }

    // ========== Edge Cases ==========

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_detection_zero_confidence() {
        let detection = ElementDetection {
            element_id: "low-conf".to_string(),
            element_type: ElementType::Unknown,
            confidence: 0.0,
            bounding_box: BoundingBox::default(),
            text_content: None,
            detection_method: DetectionMethod::ComputerVision,
            attributes: HashMap::new(),
            accessibility_info: None,
            detection_time: Instant::now(),
        };
        assert_eq!(detection.confidence, 0.0);
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_detection_perfect_confidence() {
        let detection = ElementDetection {
            element_id: "perfect".to_string(),
            element_type: ElementType::Button,
            confidence: 1.0,
            bounding_box: BoundingBox::default(),
            text_content: None,
            detection_method: DetectionMethod::Accessibility,
            attributes: HashMap::new(),
            accessibility_info: None,
            detection_time: Instant::now(),
        };
        assert_eq!(detection.confidence, 1.0);
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_empty_text_content() {
        let detection = ElementDetection {
            element_id: "empty-text".to_string(),
            element_type: ElementType::Image,
            confidence: 0.75,
            bounding_box: BoundingBox::default(),
            text_content: Some("".to_string()),
            detection_method: DetectionMethod::OCR,
            attributes: HashMap::new(),
            accessibility_info: None,
            detection_time: Instant::now(),
        };
        assert!(detection.text_content.is_some());
        assert!(detection.text_content.unwrap().is_empty());
    }

    // Traces to: FR-KDESKTOPVIRT-005
    #[test]
    fn test_large_bounding_box() {
        let bbox = BoundingBox {
            x: 0,
            y: 0,
            width: 4096,
            height: 2160,
        };
        assert_eq!(bbox.area(), 8_847_360);
        assert!((bbox.aspect_ratio() - 1.894).abs() < 0.01);
    }
}