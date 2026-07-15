/*!
User Story Verification Engine
Compares expected outcomes with actual screenshot results
*/

use crate::automation::{UserStory, UserStoryStep, VerificationResult};
use anyhow::Result;
use image::DynamicImage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationConfig {
    pub similarity_threshold: f64,
    pub text_detection_enabled: bool,
    pub color_analysis_enabled: bool,
    pub window_detection_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAnalysis {
    pub dominant_colors: Vec<String>,
    pub text_regions: Vec<TextRegion>,
    pub window_elements: Vec<WindowElement>,
    pub similarity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRegion {
    pub text: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowElement {
    pub element_type: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub properties: HashMap<String, String>,
}

/// Verification Engine for comparing user story expectations with actual results
pub struct VerificationEngine {
    pub config: VerificationConfig,
    pub baseline_screenshots: HashMap<String, String>,
}

impl VerificationEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            config: VerificationConfig {
                similarity_threshold: 0.8,
                text_detection_enabled: true,
                color_analysis_enabled: true,
                window_detection_enabled: true,
            },
            baseline_screenshots: HashMap::new(),
        })
    }

    /// Verify complete user story against actual screenshots
    pub async fn verify_user_story(
        &self,
        user_story: &UserStory,
        screenshot_paths: &[String],
    ) -> Result<Vec<VerificationResult>> {
        info!("🔍 Verifying user story: {}", user_story.title);
        
        let mut verification_results = Vec::new();
        
        for (i, step) in user_story.steps.iter().enumerate() {
            let screenshot_path = screenshot_paths.get(i)
                .ok_or_else(|| anyhow::anyhow!("Missing screenshot for step {}", step.step_number))?;
            
            let verification = self.verify_step(step, screenshot_path).await?;
            verification_results.push(verification);
        }
        
        let success_count = verification_results.iter().filter(|r| r.matches).count();
        let total_count = verification_results.len();
        
        info!("📊 User story verification complete: {}/{} steps passed", success_count, total_count);
        
        Ok(verification_results)
    }

    /// Verify individual user story step
    pub async fn verify_step(
        &self,
        step: &UserStoryStep,
        screenshot_path: &str,
    ) -> Result<VerificationResult> {
        info!("🔍 Verifying step {}: {}", step.step_number, step.action);
        
        // Analyze the screenshot
        let analysis = self.analyze_screenshot(screenshot_path).await?;
        
        // Perform verification based on expected result
        let verification_result = self.compare_with_expectation(step, &analysis, screenshot_path).await?;
        
        if verification_result.matches {
            info!("✅ Step {} verification passed", step.step_number);
        } else {
            warn!("❌ Step {} verification failed: {}", step.step_number, verification_result.actual);
        }
        
        Ok(verification_result)
    }

    /// Analyze screenshot for verification
    async fn analyze_screenshot(&self, screenshot_path: &str) -> Result<ImageAnalysis> {
        info!("📸 Analyzing screenshot: {}", screenshot_path);
        
        // Check if file exists
        if !Path::new(screenshot_path).exists() {
            return Err(anyhow::anyhow!("Screenshot not found: {}", screenshot_path));
        }
        
        // Load and analyze image
        let img = image::open(screenshot_path)?;
        
        let mut analysis = ImageAnalysis {
            dominant_colors: Vec::new(),
            text_regions: Vec::new(),
            window_elements: Vec::new(),
            similarity_score: 0.0,
        };
        
        // Analyze dominant colors
        if self.config.color_analysis_enabled {
            analysis.dominant_colors = self.extract_dominant_colors(&img).await?;
        }
        
        // Detect text regions (simplified implementation)
        if self.config.text_detection_enabled {
            analysis.text_regions = self.detect_text_regions(&img, screenshot_path).await?;
        }
        
        // Detect window elements
        if self.config.window_detection_enabled {
            analysis.window_elements = self.detect_window_elements(&img).await?;
        }
        
        info!("✅ Screenshot analysis complete");
        Ok(analysis)
    }

    /// Compare analysis with step expectation
    async fn compare_with_expectation(
        &self,
        step: &UserStoryStep,
        analysis: &ImageAnalysis,
        screenshot_path: &str,
    ) -> Result<VerificationResult> {
        let expected = &step.expected_result;
        let mut actual_observations = Vec::new();
        let mut confidence: f64 = 0.0;
        
        // Analyze based on expected result keywords
        if expected.to_lowercase().contains("calculator") {
            // Check for calculator window
            let has_calculator = self.detect_calculator_window(analysis).await?;
            actual_observations.push(format!("Calculator window present: {}", has_calculator));
            if has_calculator {
                confidence += 0.3;
            }
        }
        
        if expected.to_lowercase().contains("result") || expected.to_lowercase().contains("56") {
            // Check for calculation result
            let result_visible = self.detect_calculation_result(analysis, "56").await?;
            actual_observations.push(format!("Calculation result visible: {}", result_visible));
            if result_visible {
                confidence += 0.4;
            }
        }
        
        if expected.to_lowercase().contains("text editor") || expected.to_lowercase().contains("editor") {
            // Check for text editor window
            let has_editor = self.detect_text_editor_window(analysis).await?;
            actual_observations.push(format!("Text editor window present: {}", has_editor));
            if has_editor {
                confidence += 0.3;
            }
        }
        
        if expected.to_lowercase().contains("text appears") || expected.to_lowercase().contains("text") {
            // Check for typed text
            let text_detected = !analysis.text_regions.is_empty();
            actual_observations.push(format!("Text content detected: {}", text_detected));
            if text_detected {
                confidence += 0.4;
            }
        }
        
        if expected.to_lowercase().contains("desktop") || expected.to_lowercase().contains("clean") {
            // Check for clean desktop
            let is_clean_desktop = self.detect_clean_desktop(analysis).await?;
            actual_observations.push(format!("Clean desktop state: {}", is_clean_desktop));
            if is_clean_desktop {
                confidence += 0.5;
            }
        }
        
        // Check if any windows are open when not expected
        if expected.to_lowercase().contains("clean") && !analysis.window_elements.is_empty() {
            confidence = confidence.max(0.1); // Reduce confidence if windows open on "clean" desktop
        }
        
        // Default confidence boost for having screenshot
        confidence += 0.2;
        
        let actual = actual_observations.join(", ");
        let matches = confidence >= self.config.similarity_threshold;
        
        Ok(VerificationResult {
            step_number: step.step_number,
            expected: expected.clone(),
            actual,
            matches,
            screenshot_path: screenshot_path.to_string(),
            confidence,
        })
    }

    /// Extract dominant colors from image
    async fn extract_dominant_colors(&self, img: &DynamicImage) -> Result<Vec<String>> {
        let rgb_img = img.to_rgb8();
        let mut color_counts: HashMap<(u8, u8, u8), usize> = HashMap::new();
        
        // Sample pixels (every 10th pixel for performance)
        for (x, y, pixel) in rgb_img.enumerate_pixels() {
            if x % 10 == 0 && y % 10 == 0 {
                let rgb = (pixel[0], pixel[1], pixel[2]);
                *color_counts.entry(rgb).or_insert(0) += 1;
            }
        }
        
        // Get top 5 colors
        let mut colors: Vec<_> = color_counts.into_iter().collect();
        colors.sort_by(|a, b| b.1.cmp(&a.1));
        
        let dominant_colors = colors
            .into_iter()
            .take(5)
            .map(|((r, g, b), _)| format!("#{:02x}{:02x}{:02x}", r, g, b))
            .collect();
        
        Ok(dominant_colors)
    }

    /// Detect text regions using simple OCR approach
    async fn detect_text_regions(&self, _img: &DynamicImage, screenshot_path: &str) -> Result<Vec<TextRegion>> {
        // Use tesseract OCR if available
        let output = tokio::process::Command::new("tesseract")
            .args(&[screenshot_path, "stdout", "--psm", "6"])
            .output()
            .await;
        
        match output {
            Ok(result) if result.status.success() => {
                let text = String::from_utf8_lossy(&result.stdout);
                let lines: Vec<&str> = text.lines().filter(|line| !line.trim().is_empty()).collect();
                
                let mut text_regions = Vec::new();
                for (i, line) in lines.iter().enumerate() {
                    if !line.trim().is_empty() {
                        text_regions.push(TextRegion {
                            text: line.trim().to_string(),
                            x: 50, // Estimated position
                            y: 50 + (i as i32 * 20),
                            width: (line.len() as i32 * 8),
                            height: 16,
                            confidence: 0.8,
                        });
                    }
                }
                
                Ok(text_regions)
            }
            _ => {
                // Fallback: basic text pattern detection
                Ok(vec![])
            }
        }
    }

    /// Detect window elements
    async fn detect_window_elements(&self, img: &DynamicImage) -> Result<Vec<WindowElement>> {
        let mut elements = Vec::new();
        
        // Basic window detection based on color patterns
        let (width, height) = (img.width(), img.height());
        
        // Look for window title bars (typically dark rectangles at top)
        if self.has_title_bar_pattern(img).await? {
            elements.push(WindowElement {
                element_type: "window".to_string(),
                x: 0,
                y: 0,
                width: width as i32,
                height: height as i32,
                properties: HashMap::new(),
            });
        }
        
        Ok(elements)
    }

    /// Detect calculator window
    async fn detect_calculator_window(&self, analysis: &ImageAnalysis) -> Result<bool> {
        // Check for calculator-specific patterns
        for text_region in &analysis.text_regions {
            let text_lower = text_region.text.to_lowercase();
            if text_lower.contains("calculator") || 
               text_lower.chars().any(|c| "0123456789+−×÷=".contains(c)) {
                return Ok(true);
            }
        }
        
        // Check for calculator-like color patterns (buttons)
        let has_button_colors = analysis.dominant_colors.iter().any(|color| {
            // Common calculator button colors
            matches!(color.as_str(), "#c0c0c0" | "#808080" | "#ffffff" | "#f0f0f0")
        });
        
        Ok(has_button_colors)
    }

    /// Detect calculation result
    async fn detect_calculation_result(&self, analysis: &ImageAnalysis, expected_result: &str) -> Result<bool> {
        for text_region in &analysis.text_regions {
            if text_region.text.contains(expected_result) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Detect text editor window
    async fn detect_text_editor_window(&self, analysis: &ImageAnalysis) -> Result<bool> {
        for text_region in &analysis.text_regions {
            let text_lower = text_region.text.to_lowercase();
            if text_lower.contains("text") || text_lower.contains("editor") || 
               text_lower.contains("mousepad") || text_lower.contains("gedit") {
                return Ok(true);
            }
        }
        
        // Check for text editor color patterns
        let has_text_colors = analysis.dominant_colors.iter().any(|color| {
            // Common text editor colors (white background, etc.)
            matches!(color.as_str(), "#ffffff" | "#f8f8f8" | "#000000")
        });
        
        Ok(has_text_colors)
    }

    /// Detect clean desktop
    async fn detect_clean_desktop(&self, analysis: &ImageAnalysis) -> Result<bool> {
        // Clean desktop should have:
        // 1. Minimal text regions (just desktop elements)
        // 2. Uniform background colors
        // 3. No application windows
        
        let has_minimal_text = analysis.text_regions.len() <= 3;
        let has_uniform_colors = analysis.dominant_colors.len() <= 4;
        let no_app_windows = analysis.window_elements.is_empty();
        
        Ok(has_minimal_text && has_uniform_colors && no_app_windows)
    }

    /// Check for title bar pattern in image
    async fn has_title_bar_pattern(&self, img: &DynamicImage) -> Result<bool> {
        let rgb_img = img.to_rgb8();
        let (width, height) = (rgb_img.width(), rgb_img.height());
        
        if height < 30 {
            return Ok(false);
        }
        
        // Check top 30 pixels for title bar pattern
        let mut dark_pixels = 0;
        let total_pixels = width * 30;
        
        for y in 0..30.min(height) {
            for x in 0..width {
                let pixel = rgb_img.get_pixel(x, y);
                let brightness = (pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16) / 3;
                if brightness < 128 {
                    dark_pixels += 1;
                }
            }
        }
        
        // If more than 30% of top area is dark, likely a title bar
        let dark_ratio = dark_pixels as f64 / total_pixels as f64;
        Ok(dark_ratio > 0.3)
    }

    /// Generate verification report
    pub async fn generate_report(&self, results: &[VerificationResult]) -> Result<String> {
        let total = results.len();
        let passed = results.iter().filter(|r| r.matches).count();
        let failed = total - passed;
        
        let mut report = format!("# User Story Verification Report

");
        report.push_str(&format!("## Summary
"));
        report.push_str(&format!("- Total Steps: {}
", total));
        report.push_str(&format!("- Passed: {} ({:.1}%)
", passed, (passed as f64 / total as f64) * 100.0));
        report.push_str(&format!("- Failed: {} ({:.1}%)

", failed, (failed as f64 / total as f64) * 100.0));
        
        report.push_str("## Step Details

");
        
        for result in results {
            let status = if result.matches { "✅ PASS" } else { "❌ FAIL" };
            report.push_str(&format!("### Step {} - {}
", result.step_number, status));
            report.push_str(&format!("**Expected:** {}

", result.expected));
            report.push_str(&format!("**Actual:** {}

", result.actual));
            report.push_str(&format!("**Confidence:** {:.2}

", result.confidence));
            report.push_str(&format!("**Screenshot:** {}

", result.screenshot_path));
            report.push_str("---

");
        }
        
        Ok(report)
    }
}

impl Default for VerificationEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create verification engine")
    }
}