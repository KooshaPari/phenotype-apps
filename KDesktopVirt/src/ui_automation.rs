use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{info, warn, error};
use tokio::process::Command;
use rand::Rng;
use std::f64::consts::PI;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiElement {
    pub id: String,
    pub element_type: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub text: Option<String>,
    pub attributes: HashMap<String, String>,
    pub confidence: f64,
    pub detection_method: String,
    pub accessibility_info: Option<AccessibilityInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityInfo {
    pub role: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub states: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindMouseConfig {
    pub gravity: f64,
    pub wind: f64,
    pub min_wait: f64,
    pub max_wait: f64,
    pub max_step: f64,
    pub target_area: f64,
    pub tremor_chance: f64,
    pub tremor_amount: f64,
}

impl Default for WindMouseConfig {
    fn default() -> Self {
        Self {
            gravity: 9.0,
            wind: 5.0,
            min_wait: 0.008,
            max_wait: 0.020,
            max_step: 5.0,
            target_area: 10.0,
            tremor_chance: 0.1,
            tremor_amount: 1.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingConfig {
    pub base_delay: f64,
    pub variance: f64,
    pub mistake_chance: f64,
    pub correction_delay: f64,
    pub burst_typing: bool,
    pub pause_chance: f64,
}

impl Default for TypingConfig {
    fn default() -> Self {
        Self {
            base_delay: 0.080,
            variance: 0.040,
            mistake_chance: 0.03,
            correction_delay: 0.5,
            burst_typing: true,
            pause_chance: 0.15,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    pub fn distance_to(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementFrame {
    pub position: Point,
    pub timestamp: f64,
    pub velocity: Point,
    pub meta: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    pub element: UiElement,
    pub method: DetectionMethod,
    pub confidence: f64,
    pub processing_time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetectionMethod {
    ComputerVision,
    OCR,
    Accessibility,
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionGesture {
    PreciseClick,
    HoverClick,
    DoubleClick,
    RightClick,
    DragDrop { target_x: i32, target_y: i32 },
    Scroll { direction: String, amount: i32 },
    NaturalType { text: String },
    KeySequence { keys: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiAction {
    pub action_type: String,
    pub target: Option<String>,
    pub coordinates: Option<(i32, i32)>,
    pub text: Option<String>,
    pub delay: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiScript {
    pub name: String,
    pub actions: Vec<UiAction>,
    pub variables: HashMap<String, String>,
}

pub struct UiAutomationEngine {
    display: Option<String>,
    sessions: HashMap<String, UiSession>,
    windmouse_config: WindMouseConfig,
    typing_config: TypingConfig,
    current_position: Point,
    movement_history: Vec<MovementFrame>,
    detection_cache: HashMap<String, DetectionResult>,
    performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone, Serialize)]
pub struct PerformanceMetrics {
    pub total_actions: u64,
    pub successful_actions: u64,
    pub average_action_time: f64,
    pub detection_accuracy: f64,
    #[serde(skip)]
    pub last_updated: Instant,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            total_actions: 0,
            successful_actions: 0,
            average_action_time: 0.0,
            detection_accuracy: 0.0,
            last_updated: Instant::now(),
        }
    }
}

struct UiSession {
    session_id: String,
    display: String,
    elements: Vec<UiElement>,
    last_screenshot: Option<String>,
}

impl UiAutomationEngine {
    pub async fn new() -> Result<Self> {
        info!("Initializing Enhanced UI Automation Engine with WindMouse 2.0");

        // Initialize X11 connection
        let display = std::env::var("DISPLAY").ok();
        
        // Get initial cursor position
        let current_position = Self::get_cursor_position(&display.as_deref().unwrap_or(":0")).await
            .unwrap_or(Point::new(0.0, 0.0));

        Ok(Self {
            display,
            sessions: HashMap::new(),
            windmouse_config: WindMouseConfig::default(),
            typing_config: TypingConfig::default(),
            current_position,
            movement_history: Vec::new(),
            detection_cache: HashMap::new(),
            performance_metrics: PerformanceMetrics::default(),
        })
    }
    
    async fn get_cursor_position(display: &str) -> Result<Point> {
        let output = Command::new("xdotool")
            .env("DISPLAY", display)
            .args(["getmouselocation", "--shell"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!("Failed to get cursor position"));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut x = 0.0;
        let mut y = 0.0;

        for line in output_str.lines() {
            if let Some(stripped) = line.strip_prefix("X=") {
                x = stripped.parse::<f64>().unwrap_or(0.0);
            } else if let Some(stripped) = line.strip_prefix("Y=") {
                y = stripped.parse::<f64>().unwrap_or(0.0);
            }
        }

        Ok(Point::new(x, y))
    }

    pub async fn execute_script(&mut self, script_content: String) -> Result<()> {
        info!("Executing UI automation script");

        let script: UiScript = serde_json::from_str(&script_content)?;

        for action in &script.actions {
            self.execute_action(action, None).await?;
        }

        Ok(())
    }

    pub async fn execute_script_in_session(
        &mut self,
        script_content: String,
        session_id: String,
    ) -> Result<()> {
        info!("Executing UI automation script in session: {}", session_id);

        let script: UiScript = serde_json::from_str(&script_content)?;

        for action in &script.actions {
            self.execute_action(action, Some(session_id.clone()))
                .await?;
        }

        Ok(())
    }

    pub async fn execute_action(
        &mut self,
        action: &UiAction,
        session_id: Option<String>,
    ) -> Result<()> {
        info!("Executing enhanced action: {}", action.action_type);

        match action.action_type.as_str() {
            "click" => self.click_action(action, session_id).await?,
            "type" => self.type_action(action, session_id).await?,
            "key" => self.key_action(action, session_id).await?,
            "wait" => self.wait_action(action).await?,
            "screenshot" => self.screenshot_action(action, session_id).await?,
            "find_element" => self.find_element_action(action, session_id).await?,
            "drag" => self.drag_action(action, session_id).await?,
            "scroll" => self.scroll_action(action, session_id).await?,
            "natural_gesture" => self.natural_gesture_action(action, session_id).await?,
            "hover_click" => self.hover_click_action(action, session_id).await?,
            "precise_click" => self.precise_click_action(action, session_id).await?,
            "double_click" => self.double_click_action(action, session_id).await?,
            "right_click" => self.right_click_action(action, session_id).await?,
            _ => return Err(anyhow!("Unknown action type: {}", action.action_type)),
        }

        // Apply delay if specified
        if let Some(delay) = action.delay {
            tokio::time::sleep(delay).await;
        }

        Ok(())
    }

    async fn click_action(&mut self, action: &UiAction, session_id: Option<String>) -> Result<()> {
        let (x, y) = action
            .coordinates
            .ok_or_else(|| anyhow!("Click action requires coordinates"))?;

        info!("Performing natural click at ({}, {})", x, y);
        let start_time = Instant::now();

        // Use WindMouse 2.0 for natural movement
        let target = Point::new(x as f64, y as f64);
        let success = self.windmouse_move_to(target, session_id.clone()).await?;
        
        if !success {
            return Err(anyhow!("Failed to move cursor naturally to target"));
        }

        // Add small delay before click (human reaction time)
        let delay = 0.02 + rand::thread_rng().gen::<f64>() * 0.03;
        tokio::time::sleep(Duration::from_secs_f64(delay)).await;

        // Perform click
        let display = self.get_display_for_session(session_id)?;
        let output = Command::new("xdotool")
            .env("DISPLAY", display)
            .args(["click", "1"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!(
                "Failed to click: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        
        // Update performance metrics
        self.performance_metrics.total_actions += 1;
        self.performance_metrics.successful_actions += 1;
        let action_time = start_time.elapsed().as_secs_f64();
        self.performance_metrics.average_action_time = 
            (self.performance_metrics.average_action_time * (self.performance_metrics.total_actions - 1) as f64 + action_time) 
            / self.performance_metrics.total_actions as f64;
        
        info!("Natural click completed in {:.3}s", action_time);
        Ok(())
    }

    async fn type_action(&mut self, action: &UiAction, session_id: Option<String>) -> Result<()> {
        let text = action
            .text
            .as_ref()
            .ok_or_else(|| anyhow!("Type action requires text"))?;

        info!("Performing natural typing: '{}'", text);
        let start_time = Instant::now();
        
        // Use natural typing algorithm
        let success = self.natural_type_text(text, session_id).await?;
        
        if !success {
            return Err(anyhow!("Failed to type text naturally"));
        }
        
        // Update performance metrics
        self.performance_metrics.total_actions += 1;
        self.performance_metrics.successful_actions += 1;
        let action_time = start_time.elapsed().as_secs_f64();
        self.performance_metrics.average_action_time = 
            (self.performance_metrics.average_action_time * (self.performance_metrics.total_actions - 1) as f64 + action_time) 
            / self.performance_metrics.total_actions as f64;
        
        info!("Natural typing completed in {:.3}s", action_time);
        Ok(())
    }

    async fn key_action(&self, action: &UiAction, session_id: Option<String>) -> Result<()> {
        let key = action
            .text
            .as_ref()
            .ok_or_else(|| anyhow!("Key action requires key name"))?;

        info!("Pressing key: {}", key);

        let display = self.get_display_for_session(session_id)?;

        let output = tokio::process::Command::new("xdotool")
            .env("DISPLAY", display)
            .args(["key", key])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!(
                "Failed to press key: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    async fn wait_action(&self, action: &UiAction) -> Result<()> {
        let duration = action.delay.unwrap_or(Duration::from_secs(1));

        info!("Waiting for {:?}", duration);
        tokio::time::sleep(duration).await;

        Ok(())
    }

    async fn screenshot_action(&self, action: &UiAction, session_id: Option<String>) -> Result<()> {
        let default_path = "screenshot.png".to_string();
        let output_path = action.text.as_ref().unwrap_or(&default_path);

        info!("Taking screenshot: {}", output_path);

        let display = self.get_display_for_session(session_id)?;

        let output = tokio::process::Command::new("import")
            .env("DISPLAY", display)
            .args(["-window", "root", output_path])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!(
                "Failed to take screenshot: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    async fn find_element_action(
        &self,
        action: &UiAction,
        _session_id: Option<String>,
    ) -> Result<()> {
        let selector = action
            .target
            .as_ref()
            .ok_or_else(|| anyhow!("Find element action requires target"))?;

        info!("Finding element: {}", selector);

        // This is a simplified implementation
        // In a real implementation, this would use AI/ML to find elements
        // or integrate with accessibility APIs

        Ok(())
    }

    async fn drag_action(&mut self, action: &UiAction, session_id: Option<String>) -> Result<()> {
        let (x, y) = action
            .coordinates
            .ok_or_else(|| anyhow!("Drag action requires coordinates"))?;

        info!("Performing natural drag to ({}, {})", x, y);
        let start_time = Instant::now();

        let display = self.get_display_for_session(session_id.clone())?.to_string();
        let target = Point::new(x as f64, y as f64);

        // Natural drag sequence
        // 1. Mouse down at current position
        let output = Command::new("xdotool")
            .env("DISPLAY", &display)
            .args(["mousedown", "1"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!("Failed to mouse down for drag"));
        }

        // 2. Small delay to simulate grip
        tokio::time::sleep(Duration::from_millis(50 + rand::thread_rng().gen_range(0..50))).await;

        // 3. Natural movement to target while holding
        self.windmouse_move_to(target, session_id).await?;

        // 4. Small delay before release
        tokio::time::sleep(Duration::from_millis(30 + rand::thread_rng().gen_range(0..30))).await;

        // 5. Mouse up
        let output = Command::new("xdotool")
            .env("DISPLAY", &display)
            .args(["mouseup", "1"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!("Failed to mouse up for drag"));
        }
        
        // Update performance metrics
        self.performance_metrics.total_actions += 1;
        self.performance_metrics.successful_actions += 1;
        let action_time = start_time.elapsed().as_secs_f64();
        self.performance_metrics.average_action_time = 
            (self.performance_metrics.average_action_time * (self.performance_metrics.total_actions - 1) as f64 + action_time) 
            / self.performance_metrics.total_actions as f64;

        info!("Natural drag completed in {:.3}s", action_time);
        Ok(())
    }

    async fn scroll_action(&self, action: &UiAction, session_id: Option<String>) -> Result<()> {
        let default_direction = "down".to_string();
        let direction = action.text.as_ref().unwrap_or(&default_direction);

        info!("Scrolling: {}", direction);

        let display = self.get_display_for_session(session_id)?;

        let button = match direction.as_str() {
            "up" => "4",
            "down" => "5",
            "left" => "6",
            "right" => "7",
            _ => "5", // default to down
        };

        let output = tokio::process::Command::new("xdotool")
            .env("DISPLAY", display)
            .args(["click", button])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!(
                "Failed to scroll: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        Ok(())
    }

    async fn get_mouse_position(&self, display: &str) -> Result<(i32, i32)> {
        let output = Command::new("xdotool")
            .env("DISPLAY", display)
            .args(["getmouselocation", "--shell"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow!("Failed to get mouse position"));
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut x = 0;
        let mut y = 0;

        for line in output_str.lines() {
            if let Some(stripped) = line.strip_prefix("X=") {
                x = stripped.parse::<i32>().unwrap_or(0);
            } else if let Some(stripped) = line.strip_prefix("Y=") {
                y = stripped.parse::<i32>().unwrap_or(0);
            }
        }

        Ok((x, y))
    }

    fn get_display_for_session(&self, session_id: Option<String>) -> Result<&str> {
        if let Some(_session_id) = session_id {
            // For containerized sessions, we would map to the container's display
            // For now, use the default display
            Ok(":0")
        } else {
            Ok(self.display.as_deref().unwrap_or(":0"))
        }
    }

    pub async fn create_session(&mut self, session_id: String, display: String) -> Result<()> {
        info!("Creating UI automation session: {}", session_id);

        let session = UiSession {
            session_id: session_id.clone(),
            display,
            elements: Vec::new(),
            last_screenshot: None,
        };

        self.sessions.insert(session_id, session);

        Ok(())
    }

    pub async fn remove_session(&mut self, session_id: String) -> Result<()> {
        info!("Removing UI automation session: {}", session_id);

        self.sessions.remove(&session_id);

        Ok(())
    }
    
    // === WINDMOUSE 2.0 ALGORITHM ===
    
    async fn windmouse_move_to(&mut self, target: Point, _session_id: Option<String>) -> Result<bool> {
        let start = self.current_position.clone();
        let distance = start.distance_to(&target);
        
        if distance < 3.0 {
            // Direct movement for very short distances
            self.current_position = target.clone();
            return Ok(true);
        }
        
        let frames = self.generate_windmouse_path(&start, &target).await?;
        
        for frame in frames {
            // Execute movement frame
            let move_result = Command::new("xdotool")
                .args(["mousemove", &(frame.position.x as i32).to_string(), &(frame.position.y as i32).to_string()])
                .output()
                .await;
                
            if move_result.is_err() {
                warn!("Failed to execute movement frame");
                continue;
            }
            
            // Wait based on frame timing
            let wait_time = Duration::from_secs_f64(frame.timestamp);
            tokio::time::sleep(wait_time).await;
            
            // Store in movement history
            self.movement_history.push(frame.clone());
            
            // Limit history size
            if self.movement_history.len() > 100 {
                self.movement_history.remove(0);
            }
        }
        
        self.current_position = target;
        Ok(true)
    }
    
    async fn generate_windmouse_path(&self, start: &Point, target: &Point) -> Result<Vec<MovementFrame>> {
        let mut frames = Vec::new();
        let mut current_pos = start.clone();
        let mut velocity = Point::new(0.0, 0.0);
        let mut current_time = 0.0;
        
        let total_distance = start.distance_to(target);
        let mut remaining_distance = total_distance;
        
        while remaining_distance > 1.0 {
            // Calculate forces
            let direction_x = target.x - current_pos.x;
            let direction_y = target.y - current_pos.y;
            let distance = (direction_x * direction_x + direction_y * direction_y).sqrt();
            
            // Normalize direction
            let norm_x = direction_x / distance;
            let norm_y = direction_y / distance;
            
            // Gravity force (attraction to target)
            let gravity_strength = self.windmouse_config.gravity * self.adaptive_gravity_strength(remaining_distance, total_distance);
            let gravity_x = norm_x * gravity_strength;
            let gravity_y = norm_y * gravity_strength;
            
            // Wind force (random variation)
            let mut rng = rand::thread_rng();
            let wind_x = (rng.gen::<f64>() - 0.5) * 2.0 * self.windmouse_config.wind;
            let wind_y = (rng.gen::<f64>() - 0.5) * 2.0 * self.windmouse_config.wind;
            
            // Tremor force (natural hand instability)
            let tremor_x = if rng.gen::<f64>() < self.windmouse_config.tremor_chance {
                (rng.gen::<f64>() - 0.5) * 2.0 * self.windmouse_config.tremor_amount
            } else {
                0.0
            };
            let tremor_y = if rng.gen::<f64>() < self.windmouse_config.tremor_chance {
                (rng.gen::<f64>() - 0.5) * 2.0 * self.windmouse_config.tremor_amount
            } else {
                0.0
            };
            
            // Update velocity
            velocity.x += gravity_x + wind_x + tremor_x;
            velocity.y += gravity_y + wind_y + tremor_y;
            
            // Limit velocity
            let velocity_magnitude = (velocity.x * velocity.x + velocity.y * velocity.y).sqrt();
            if velocity_magnitude > self.windmouse_config.max_step {
                velocity.x = (velocity.x / velocity_magnitude) * self.windmouse_config.max_step;
                velocity.y = (velocity.y / velocity_magnitude) * self.windmouse_config.max_step;
            }
            
            // Update position
            current_pos.x += velocity.x;
            current_pos.y += velocity.y;
            
            // Calculate remaining distance
            remaining_distance = current_pos.distance_to(target);
            
            // Calculate frame timing
            let wait_time = rng.gen_range(self.windmouse_config.min_wait..=self.windmouse_config.max_wait);
            if remaining_distance < 20.0 {
                // Slower near target
                current_time += wait_time * 1.5;
            } else {
                current_time += wait_time;
            }
            
            // Create frame
            frames.push(MovementFrame {
                position: current_pos.clone(),
                timestamp: current_time,
                velocity: velocity.clone(),
                meta: Some(format!("distance_remaining: {:.2}", remaining_distance)),
            });
        }
        
        // Final precise positioning
        frames.push(MovementFrame {
            position: target.clone(),
            timestamp: current_time + 0.01,
            velocity: Point::new(0.0, 0.0),
            meta: Some("final_position".to_string()),
        });
        
        Ok(frames)
    }
    
    fn adaptive_gravity_strength(&self, distance_remaining: f64, total_distance: f64) -> f64 {
        let progress = 1.0 - (distance_remaining / total_distance);
        
        if progress < 0.1 {
            // Strong initial pull
            1.2 + (0.1 - progress) * 2.0
        } else if progress > 0.9 {
            // Gentle final approach
            0.3 + (1.0 - progress) * 0.7
        } else {
            // Normal gravity in middle section
            1.0
        }
    }
    
    // === NATURAL TYPING ALGORITHM ===
    
    async fn natural_type_text(&mut self, text: &str, _session_id: Option<String>) -> Result<bool> {
        let mut rng = rand::thread_rng();
        
        for (char_index, character) in text.char_indices() {
            // Calculate natural typing delay
            let base_delay = self.typing_config.base_delay;
            let variance = rng.gen_range(-self.typing_config.variance..=self.typing_config.variance);
            let typing_delay = (base_delay + variance).max(0.01);
            
            // Burst typing for common words
            let adjusted_delay = if self.typing_config.burst_typing && char_index > 0 {
                if self.is_common_word_context(text, char_index) {
                    typing_delay * 0.6
                } else {
                    typing_delay
                }
            } else {
                typing_delay
            };
            
            // Simulate typing mistakes
            if rng.gen::<f64>() < self.typing_config.mistake_chance {
                // Type wrong character first
                if let Some(wrong_char) = self.get_adjacent_key(character) {
                    self.type_character(&wrong_char.to_string()).await?;
                    tokio::time::sleep(Duration::from_secs_f64(adjusted_delay)).await;
                    
                    // Realize mistake and correct
                    tokio::time::sleep(Duration::from_secs_f64(self.typing_config.correction_delay)).await;
                    
                    // Backspace
                    Command::new("xdotool").args(["key", "BackSpace"]).output().await?;
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
            
            // Type correct character
            self.type_character(&character.to_string()).await?;
            
            // Natural pauses (thinking)
            if rng.gen::<f64>() < self.typing_config.pause_chance {
                let pause_duration = rng.gen_range(0.2..=0.8);
                tokio::time::sleep(Duration::from_secs_f64(pause_duration)).await;
            } else {
                tokio::time::sleep(Duration::from_secs_f64(adjusted_delay)).await;
            }
        }
        
        Ok(true)
    }
    
    async fn type_character(&self, character: &str) -> Result<()> {
        match character {
            "\n" => {
                Command::new("xdotool").args(["key", "Return"]).output().await?;
            },
            "\t" => {
                Command::new("xdotool").args(["key", "Tab"]).output().await?;
            },
            " " => {
                Command::new("xdotool").args(["key", "space"]).output().await?;
            },
            _ => {
                Command::new("xdotool").args(["type", character]).output().await?;
            }
        }
        Ok(())
    }
    
    fn is_common_word_context(&self, text: &str, char_index: usize) -> bool {
        let common_words = ["the ", "and ", "for ", "you ", "with ", "this ", "that "];
        
        for word in &common_words {
            if char_index >= word.len() - 1 {
                let start_index = char_index - (word.len() - 1);
                if let Some(context) = text.get(start_index..=char_index) {
                    if context == *word {
                        return true;
                    }
                }
            }
        }
        false
    }
    
    fn get_adjacent_key(&self, character: char) -> Option<char> {
        // QWERTY keyboard layout for realistic typos
        match character.to_ascii_lowercase() {
            'q' => Some('w'),
            'w' => Some('e'),
            'e' => Some('r'),
            'r' => Some('t'),
            't' => Some('y'),
            'y' => Some('u'),
            'u' => Some('i'),
            'i' => Some('o'),
            'o' => Some('p'),
            'a' => Some('s'),
            's' => Some('d'),
            'd' => Some('f'),
            'f' => Some('g'),
            'g' => Some('h'),
            'h' => Some('j'),
            'j' => Some('k'),
            'k' => Some('l'),
            'z' => Some('x'),
            'x' => Some('c'),
            'c' => Some('v'),
            'v' => Some('b'),
            'b' => Some('n'),
            'n' => Some('m'),
            _ => None,
        }
    }

    pub async fn get_session_elements(&self, session_id: String) -> Result<Vec<UiElement>> {
        if let Some(session) = self.sessions.get(&session_id) {
            Ok(session.elements.clone())
        } else {
            Err(anyhow!("Session not found: {}", session_id))
        }
    }

    pub async fn find_elements(
        &self,
        _session_id: Option<String>,
        selector: String,
    ) -> Result<Vec<UiElement>> {
        info!("Finding elements with selector: {}", selector);

        // This is a placeholder implementation
        // In a real implementation, this would use AI/ML for element detection
        // or integrate with accessibility APIs

        Ok(vec![])
    }

    pub async fn get_element_text(
        &self,
        _session_id: Option<String>,
        element_id: String,
    ) -> Result<String> {
        info!("Getting text for element: {}", element_id);

        // Placeholder implementation
        Ok(String::new())
    }

    pub async fn set_element_text(
        &mut self,
        session_id: Option<String>,
        element_id: String,
        text: String,
    ) -> Result<()> {
        info!("Setting text for element {}: {}", element_id, text);
        
        // Use natural typing for setting text
        self.natural_type_text(&text, session_id).await?;
        Ok(())
    }
    
    // === ADVANCED GESTURE ACTIONS ===
    
    async fn natural_gesture_action(&mut self, action: &UiAction, session_id: Option<String>) -> Result<()> {
        let default_gesture = "hover_click".to_string();
        let gesture_type = action.text.as_ref().unwrap_or(&default_gesture);
        
        match gesture_type.as_str() {
            "hover_click" => self.hover_click_action(action, session_id).await,
            "precise_click" => self.precise_click_action(action, session_id).await,
            "double_click" => self.double_click_action(action, session_id).await,
            "right_click" => self.right_click_action(action, session_id).await,
            _ => self.hover_click_action(action, session_id).await,
        }
    }
    
    async fn hover_click_action(&mut self, action: &UiAction, session_id: Option<String>) -> Result<()> {
        let (x, y) = action.coordinates.ok_or_else(|| anyhow!("Hover click requires coordinates"))?;
        let target = Point::new(x as f64, y as f64);
        
        info!("Performing hover click at ({}, {})", x, y);
        
        // Move to position with slight offset for natural hover
        let mut rng = rand::thread_rng();
        let hover_offset = Point::new(
            x as f64 + rng.gen_range(-3.0..=3.0),
            y as f64 + rng.gen_range(-3.0..=3.0)
        );
        
        self.windmouse_move_to(hover_offset, session_id.clone()).await?;
        
        // Hover with micro-movements
        for _ in 0..3 {
            let micro_x = x as f64 + rng.gen_range(-2.0..=2.0);
            let micro_y = y as f64 + rng.gen_range(-2.0..=2.0);
            
            Command::new("xdotool")
                .args(["mousemove", &(micro_x as i32).to_string(), &(micro_y as i32).to_string()])
                .output()
                .await?;
                
            tokio::time::sleep(Duration::from_millis(150 + rng.gen_range(0..100))).await;
        }
        
        // Final precise positioning and click
        self.windmouse_move_to(target, session_id).await?;
        tokio::time::sleep(Duration::from_millis(50)).await;
        
        let display = self.get_display_for_session(None)?;
        Command::new("xdotool")
            .env("DISPLAY", display)
            .args(["click", "1"])
            .output()
            .await?;
            
        Ok(())
    }
    
    async fn precise_click_action(&mut self, action: &UiAction, session_id: Option<String>) -> Result<()> {
        let (x, y) = action.coordinates.ok_or_else(|| anyhow!("Precise click requires coordinates"))?;
        let target = Point::new(x as f64, y as f64);
        
        info!("Performing precise click at ({}, {})", x, y);
        
        // Direct precise movement
        self.windmouse_move_to(target, session_id).await?;
        tokio::time::sleep(Duration::from_millis(20)).await;
        
        let display = self.get_display_for_session(None)?;
        Command::new("xdotool")
            .env("DISPLAY", display)
            .args(["click", "1"])
            .output()
            .await?;
            
        Ok(())
    }
    
    async fn double_click_action(&mut self, action: &UiAction, session_id: Option<String>) -> Result<()> {
        let (x, y) = action.coordinates.ok_or_else(|| anyhow!("Double click requires coordinates"))?;
        let target = Point::new(x as f64, y as f64);
        
        info!("Performing double click at ({}, {})", x, y);
        
        self.windmouse_move_to(target, session_id).await?;
        
        let display = self.get_display_for_session(None)?;
        
        // First click
        Command::new("xdotool")
            .env("DISPLAY", display)
            .args(["click", "1"])
            .output()
            .await?;
            
        // Natural interval between clicks
        let mut rng = rand::thread_rng();
        let interval = Duration::from_millis(100 + rng.gen_range(0..100));
        tokio::time::sleep(interval).await;
        
        // Second click
        Command::new("xdotool")
            .env("DISPLAY", display)
            .args(["click", "1"])
            .output()
            .await?;
            
        Ok(())
    }
    
    async fn right_click_action(&mut self, action: &UiAction, session_id: Option<String>) -> Result<()> {
        let (x, y) = action.coordinates.ok_or_else(|| anyhow!("Right click requires coordinates"))?;
        let target = Point::new(x as f64, y as f64);
        
        info!("Performing right click at ({}, {})", x, y);
        
        self.windmouse_move_to(target, session_id).await?;
        tokio::time::sleep(Duration::from_millis(20)).await;
        
        let display = self.get_display_for_session(None)?;
        Command::new("xdotool")
            .env("DISPLAY", display)
            .args(["click", "3"])
            .output()
            .await?;
            
        Ok(())
    }
    
    // === PERFORMANCE AND MONITORING ===
    
    pub fn get_performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance_metrics
    }
    
    pub async fn optimize_performance(&mut self) {
        // Adjust WindMouse parameters based on performance
        if self.performance_metrics.average_action_time > 2.0 {
            self.windmouse_config.max_step *= 1.2;
            self.windmouse_config.min_wait *= 0.9;
            self.windmouse_config.max_wait *= 0.9;
        }
        
        // Adjust typing speed based on accuracy
        if (self.performance_metrics.successful_actions as f64) / (self.performance_metrics.total_actions as f64) < 0.95 {
            self.typing_config.base_delay *= 1.1;
            self.typing_config.mistake_chance *= 0.8;
        }
        
        info!("Performance optimized: avg_time={:.3}s, success_rate={:.2}%", 
              self.performance_metrics.average_action_time,
              ((self.performance_metrics.successful_actions as f64) / (self.performance_metrics.total_actions as f64)) * 100.0
        );
    }
    
    pub fn configure_windmouse(&mut self, config: WindMouseConfig) {
        self.windmouse_config = config;
        info!("WindMouse 2.0 configuration updated");
    }
    
    pub fn configure_typing(&mut self, config: TypingConfig) {
        self.typing_config = config;
        info!("Natural typing configuration updated");
    }
    
    pub fn clear_movement_history(&mut self) {
        self.movement_history.clear();
        info!("Movement history cleared");
    }
    
    pub fn get_movement_history(&self) -> &Vec<MovementFrame> {
        &self.movement_history
    }
    
    pub async fn take_performance_screenshot(&self, path: &str) -> Result<()> {
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("{}/natural_automation_performance_{}.png", path, timestamp);

        Command::new("import")
            .args(["-window", "root", &filename])
            .output()
            .await?;

        info!("Performance screenshot saved: {}", filename);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ========== UiElement Tests ==========

    #[test]
    fn test_ui_element_creation() {
        // Traces to: FR-KDESKTOPVIRT-005
        let element = UiElement {
            id: "btn-1".to_string(),
            element_type: "button".to_string(),
            x: 100,
            y: 200,
            width: 80,
            height: 40,
            text: Some("Click Me".to_string()),
            attributes: HashMap::new(),
            confidence: 0.95,
            detection_method: "visual".to_string(),
            accessibility_info: None,
        };
        assert_eq!(element.id, "btn-1");
        assert_eq!(element.element_type, "button");
        assert!(element.confidence > 0.9);
    }

    #[test]
    fn test_ui_element_bounds() {
        // Traces to: FR-KDESKTOPVIRT-005
        let element = UiElement {
            id: "input-1".to_string(),
            element_type: "input".to_string(),
            x: 0,
            y: 0,
            width: 200,
            height: 30,
            text: None,
            attributes: HashMap::new(),
            confidence: 0.88,
            detection_method: "ai".to_string(),
            accessibility_info: None,
        };
        assert!(element.width > 0);
        assert!(element.height > 0);
    }

    #[test]
    fn test_ui_element_with_accessibility() {
        // Traces to: FR-KDESKTOPVIRT-005
        let accessibility = AccessibilityInfo {
            role: "button".to_string(),
            name: Some("Submit".to_string()),
            description: Some("Submit form".to_string()),
            states: vec!["enabled".to_string(), "focused".to_string()],
        };
        let element = UiElement {
            id: "submit-btn".to_string(),
            element_type: "button".to_string(),
            x: 150,
            y: 300,
            width: 100,
            height: 50,
            text: Some("Submit".to_string()),
            attributes: HashMap::new(),
            confidence: 0.99,
            detection_method: "accessibility_tree".to_string(),
            accessibility_info: Some(accessibility),
        };
        assert!(element.accessibility_info.is_some());
        let info = element.accessibility_info.unwrap();
        assert_eq!(info.role, "button");
    }

    // ========== Point Tests ==========

    #[test]
    fn test_point_creation() {
        // Traces to: FR-KDESKTOPVIRT-005
        let point = Point::new(100.0, 200.0);
        assert_eq!(point.x, 100.0);
        assert_eq!(point.y, 200.0);
    }

    #[test]
    fn test_point_distance_to_self() {
        // Traces to: FR-KDESKTOPVIRT-005
        let point = Point::new(50.0, 50.0);
        let distance = point.distance_to(&point);
        assert!((distance - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_point_distance_calculation() {
        // Traces to: FR-KDESKTOPVIRT-005
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        let distance = p1.distance_to(&p2);
        assert!((distance - 5.0).abs() < 0.001); // 3-4-5 triangle
    }

    #[test]
    fn test_point_negative_coordinates() {
        // Traces to: FR-KDESKTOPVIRT-005
        let p1 = Point::new(-100.0, -50.0);
        let p2 = Point::new(100.0, 50.0);
        let distance = p1.distance_to(&p2);
        assert!(distance > 0.0);
    }

    // ========== WindMouseConfig Tests ==========

    #[test]
    fn test_windmouse_config_default() {
        // Traces to: FR-KDESKTOPVIRT-005
        let config = WindMouseConfig::default();
        assert_eq!(config.gravity, 9.0);
        assert_eq!(config.wind, 5.0);
        assert!(config.min_wait < config.max_wait);
    }

    #[test]
    fn test_windmouse_config_custom() {
        // Traces to: FR-KDESKTOPVIRT-005
        let config = WindMouseConfig {
            gravity: 15.0,
            wind: 8.0,
            min_wait: 0.01,
            max_wait: 0.03,
            max_step: 10.0,
            target_area: 20.0,
            tremor_chance: 0.2,
            tremor_amount: 2.0,
        };
        assert_eq!(config.gravity, 15.0);
        assert!(config.max_step > 0.0);
    }

    #[test]
    fn test_windmouse_tremor_params() {
        // Traces to: FR-KDESKTOPVIRT-005
        let config = WindMouseConfig::default();
        assert!(config.tremor_chance >= 0.0 && config.tremor_chance <= 1.0);
        assert!(config.tremor_amount > 0.0);
    }

    // ========== TypingConfig Tests ==========

    #[test]
    fn test_typing_config_default() {
        // Traces to: FR-KDESKTOPVIRT-005
        let config = TypingConfig::default();
        assert_eq!(config.base_delay, 0.080);
        assert!(config.variance > 0.0);
        assert!(config.mistake_chance < 0.1);
    }

    #[test]
    fn test_typing_config_custom() {
        // Traces to: FR-KDESKTOPVIRT-005
        let config = TypingConfig {
            base_delay: 0.1,
            variance: 0.05,
            mistake_chance: 0.02,
            correction_delay: 0.4,
            burst_typing: false,
            pause_chance: 0.1,
        };
        assert_eq!(config.base_delay, 0.1);
        assert!(!config.burst_typing);
    }

    #[test]
    fn test_typing_config_mistake_bounds() {
        // Traces to: FR-KDESKTOPVIRT-005
        let config = TypingConfig::default();
        assert!(config.mistake_chance >= 0.0 && config.mistake_chance <= 1.0);
        assert!(config.pause_chance >= 0.0 && config.pause_chance <= 1.0);
    }

    // ========== Movement Frame Tests ==========

    #[test]
    fn test_movement_frame_creation() {
        // Traces to: FR-KDESKTOPVIRT-005
        let frame = MovementFrame {
            timestamp: 1.0,
            position: Point::new(100.0, 200.0),
            velocity: Point::new(5.0, 3.0),
            meta: Some("test movement".to_string()),
        };
        assert_eq!(frame.position.x, 100.0);
        assert_eq!(frame.velocity.x, 5.0);
    }

    #[test]
    fn test_movement_sequence() {
        // Traces to: FR-KDESKTOPVIRT-005
        let mut frames = Vec::new();
        for i in 0..10 {
            frames.push(MovementFrame {
                timestamp: i as f64,
                position: Point::new((i * 10) as f64, (i * 10) as f64),
                velocity: Point::new(5.0 + (i as f64) * 0.1, 2.0),
                meta: None,
            });
        }
        assert_eq!(frames.len(), 10);
        assert!(frames[9].velocity.x > frames[0].velocity.x);
    }

    // ========== UI Automation Engine Tests ==========

    #[test]
    fn test_ui_automation_engine_configuration() {
        // Traces to: FR-KDESKTOPVIRT-005
        let config = WindMouseConfig::default();
        assert!(config.gravity > 0.0);
        let typing_config = TypingConfig::default();
        assert!(typing_config.base_delay > 0.0);
    }

    // ========== Automation Engine Configuration Tests ==========

    #[test]
    fn test_windmouse_config_bounds() {
        // Traces to: FR-KDESKTOPVIRT-005
        let config = WindMouseConfig {
            gravity: 12.0,
            wind: 6.0,
            min_wait: 0.01,
            max_wait: 0.025,
            max_step: 7.0,
            target_area: 15.0,
            tremor_chance: 0.15,
            tremor_amount: 1.8,
        };
        assert!(config.min_wait < config.max_wait);
        assert!(config.gravity > 0.0);
    }

    #[test]
    fn test_typing_config_variance() {
        // Traces to: FR-KDESKTOPVIRT-005
        let config = TypingConfig {
            base_delay: 0.09,
            variance: 0.045,
            mistake_chance: 0.025,
            correction_delay: 0.45,
            burst_typing: true,
            pause_chance: 0.12,
        };
        assert!(config.variance > 0.0);
        assert!(config.base_delay > config.variance);
    }

    #[test]
    fn test_point_ordering() {
        // Traces to: FR-KDESKTOPVIRT-005
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(100.0, 100.0);
        let p3 = Point::new(50.0, 50.0);
        assert!(p1.distance_to(&p3) < p1.distance_to(&p2));
    }

    // ========== Edge Cases ==========

    #[test]
    fn test_element_zero_dimensions() {
        // Traces to: FR-KDESKTOPVIRT-005
        let element = UiElement {
            id: "zero".to_string(),
            element_type: "element".to_string(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            text: None,
            attributes: HashMap::new(),
            confidence: 0.0,
            detection_method: "test".to_string(),
            accessibility_info: None,
        };
        assert_eq!(element.width, 0);
        assert_eq!(element.height, 0);
    }

    #[test]
    fn test_large_coordinate_values() {
        // Traces to: FR-KDESKTOPVIRT-005
        let point = Point::new(10000.0, 5000.0);
        let distance = point.distance_to(&Point::new(10001.0, 5000.0));
        assert!((distance - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_accessibility_info_optional() {
        // Traces to: FR-KDESKTOPVIRT-005
        let mut element = UiElement {
            id: "no-access".to_string(),
            element_type: "div".to_string(),
            x: 0,
            y: 0,
            width: 100,
            height: 100,
            text: None,
            attributes: HashMap::new(),
            confidence: 0.5,
            detection_method: "visual".to_string(),
            accessibility_info: None,
        };
        assert!(element.accessibility_info.is_none());
        element.accessibility_info = Some(AccessibilityInfo {
            role: "none".to_string(),
            name: None,
            description: None,
            states: vec![],
        });
        assert!(element.accessibility_info.is_some());
    }
}
