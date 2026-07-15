/*!
 * KVirtualStage Core Automation Engine
 * 
 * Implements natural, human-like automation with:
 * - WindMouse 2.0 physics-based cursor movement
 * - Natural typing simulation with character-by-character timing
 * - Context-aware intent simulation
 * - Frame-by-frame animation system
 * 
 * Based on the architectural specifications for professional-grade automation.
 */

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, info, warn};
use uuid::Uuid;

// ============================================================================
// Core Data Structures
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

impl std::ops::Add<Vector2> for Point {
    type Output = Point;
    fn add(self, rhs: Vector2) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Point {
    type Output = Vector2;
    fn sub(self, rhs: Point) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self { x: self.x / mag, y: self.y / mag }
        } else {
            *self
        }
    }
}

impl std::ops::Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::Mul<f64> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

// ============================================================================
// WindMouse 2.0 Implementation
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindMouseEngine {
    // Physics parameters
    pub gravity: f64,
    pub wind: f64,
    pub friction: f64,
    pub target_awareness: f64,
    
    // User profile for natural variation
    pub user_profile: UserMovementProfile,
    
    // Performance optimization
    trajectory_cache: HashMap<String, Vec<MovementFrame>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMovementProfile {
    pub movement_speed: f64,      // 0.5-2.0 (slower to faster)
    pub precision_level: f64,     // 0.0-1.0 (lower to higher precision)
    pub jitter_amount: f64,       // 0.0-1.0 (no jitter to high jitter)
    pub hesitation_factor: f64,   // 0.0-1.0 (decisive to hesitant)
    pub fatigue_level: f64,       // 0.0-1.0 (fresh to tired)
    pub path_curvature: f64,      // 0.0-1.0 (straight to curved paths)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementFrame {
    pub position: Point,
    pub velocity: Vector2,
    pub timestamp: f64,
    pub smoothing_factor: f64,
    pub meta: MovementMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementMeta {
    pub distance_remaining: f64,
    pub force_components: ForceBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceBreakdown {
    pub gravity: Vector2,
    pub wind: Vector2,
    pub tremor: Vector2,
    pub context: Vector2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementContext {
    pub base_gravity: f64,
    pub wind_strength: f64,
    pub precision_threshold: f64,
    pub precision_slowdown_factor: f64,
    pub obstacle_avoidance: bool,
    pub obstacles: Vec<Point>,
}

impl Default for UserMovementProfile {
    fn default() -> Self {
        Self {
            movement_speed: 1.0,
            precision_level: 0.8,
            jitter_amount: 0.1,
            hesitation_factor: 0.2,
            fatigue_level: 0.0,
            path_curvature: 0.3,
        }
    }
}

impl Default for MovementContext {
    fn default() -> Self {
        Self {
            base_gravity: 12.0,
            wind_strength: 4.0,
            precision_threshold: 50.0,
            precision_slowdown_factor: 0.3,
            obstacle_avoidance: false,
            obstacles: Vec::new(),
        }
    }
}

impl WindMouseEngine {
    pub fn new() -> Self {
        Self {
            gravity: 12.0,
            wind: 4.0,
            friction: 0.95,
            target_awareness: 15.0,
            user_profile: UserMovementProfile::default(),
            trajectory_cache: HashMap::new(),
        }
    }

    pub fn with_profile(mut self, profile: UserMovementProfile) -> Self {
        self.user_profile = profile;
        self
    }

    /// Generate natural cursor movement trajectory using WindMouse 2.0 algorithm
    pub fn generate_movement_trajectory(
        &mut self,
        start: Point,
        target: Point,
        context: MovementContext,
    ) -> Vec<MovementFrame> {
        // Check cache first
        let cache_key = format!("{:.0},{:.0}-{:.0},{:.0}", start.x, start.y, target.x, target.y);
        if let Some(cached) = self.trajectory_cache.get(&cache_key) {
            debug!("Using cached trajectory for movement");
            return cached.clone();
        }

        let mut frames = Vec::new();
        let mut current_pos = start;
        let mut velocity = Vector2::ZERO;
        let mut wind_force = Vector2::ZERO;
        let mut micro_tremor = TremorState::new();
        
        let total_distance = start.distance_to(target);
        let mut distance_remaining = total_distance;
        
        const DELTA_TIME: f64 = 1.0 / 60.0; // 60 FPS
        let mut frame_time = 0.0;

        // Physics parameters adapted to context and user profile
        let gravity = context.base_gravity * self.user_profile.precision_level;
        let wind_strength = context.wind_strength * self.user_profile.jitter_amount;

        while distance_remaining > 1.0 {
            // === FORCE CALCULATION ===
            
            // 1. Gravitational force (primary targeting force)
            let direction_to_target = (target - current_pos).normalized();
            let gravity_strength = self.adaptive_gravity_strength(distance_remaining, total_distance);
            let gravity_force = direction_to_target * gravity * gravity_strength;
            
            // 2. Wind force (controlled randomness)
            wind_force = self.update_wind_force(wind_force, wind_strength);
            
            // 3. Micro-tremor (human hand instability)
            let tremor_force = micro_tremor.calculate_tremor_force(
                self.user_profile.fatigue_level,
                distance_remaining,
            );
            
            // 4. Context-specific forces
            let context_force = self.calculate_context_forces(current_pos, &context);
            
            // === FORCE INTEGRATION ===
            let total_force = gravity_force + wind_force + tremor_force + context_force;
            
            // Apply force to velocity
            velocity = velocity + total_force * DELTA_TIME;
            
            // Apply friction
            velocity = velocity * self.friction;
            
            // === ADAPTIVE BEHAVIOR ===
            
            // Speed modulation based on distance to target
            let distance_modifier = self.calculate_distance_modifier(distance_remaining, total_distance);
            velocity = velocity * distance_modifier;
            
            // Precision mode near target
            if distance_remaining < context.precision_threshold {
                velocity = velocity * context.precision_slowdown_factor;
            }
            
            // === POSITION UPDATE ===
            current_pos = current_pos + velocity * DELTA_TIME;
            distance_remaining = current_pos.distance_to(target);
            
            // === NATURAL VARIATION ===
            if self.user_profile.path_curvature > 0.0 {
                let curvature_offset = self.calculate_natural_curvature(
                    current_pos,
                    start,
                    target,
                    self.user_profile.path_curvature,
                );
                current_pos = current_pos + curvature_offset;
            }
            
            // Create movement frame
            frames.push(MovementFrame {
                position: current_pos,
                velocity,
                timestamp: frame_time,
                smoothing_factor: self.calculate_smoothing_factor(velocity.magnitude()),
                meta: MovementMeta {
                    distance_remaining,
                    force_components: ForceBreakdown {
                        gravity: gravity_force,
                        wind: wind_force,
                        tremor: tremor_force,
                        context: context_force,
                    },
                },
            });
            
            // Update state
            micro_tremor.update(DELTA_TIME);
            frame_time += DELTA_TIME;
        }
        
        // Final precision adjustment
        frames.push(MovementFrame {
            position: target,
            velocity: Vector2::ZERO,
            timestamp: frame_time,
            smoothing_factor: 1.0,
            meta: MovementMeta {
                distance_remaining: 0.0,
                force_components: ForceBreakdown {
                    gravity: Vector2::ZERO,
                    wind: Vector2::ZERO,
                    tremor: Vector2::ZERO,
                    context: Vector2::ZERO,
                },
            },
        });

        // Cache the trajectory
        if frames.len() < 1000 { // Only cache reasonable-sized trajectories
            self.trajectory_cache.insert(cache_key, frames.clone());
        }

        frames
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

    fn update_wind_force(&self, current_wind: Vector2, wind_strength: f64) -> Vector2 {
        let wind_decay = 0.95;
        let wind_noise = Vector2::new(
            (fastrand::f64() * 2.0 - 1.0) * wind_strength,
            (fastrand::f64() * 2.0 - 1.0) * wind_strength,
        );
        
        current_wind * wind_decay + wind_noise
    }

    fn calculate_context_forces(&self, _current_pos: Point, _context: &MovementContext) -> Vector2 {
        // Placeholder for context-specific forces (obstacle avoidance, etc.)
        Vector2::ZERO
    }

    fn calculate_distance_modifier(&self, distance_remaining: f64, total_distance: f64) -> f64 {
        let progress = 1.0 - (distance_remaining / total_distance);
        
        // Acceleration curve: slow start, fast middle, slow end
        if progress < 0.2 {
            0.5 + progress * 2.5 // Accelerate from 50% to 100%
        } else if progress > 0.8 {
            1.0 - (progress - 0.8) * 2.5 // Decelerate from 100% to 50%
        } else {
            1.0 // Full speed in middle
        }
    }

    fn calculate_natural_curvature(
        &self,
        current_pos: Point,
        start: Point,
        target: Point,
        curvature_factor: f64,
    ) -> Vector2 {
        let total_vector = target - start;
        let current_vector = current_pos - start;
        let progress = current_vector.magnitude() / total_vector.magnitude();
        
        // Create gentle S-curve for natural movement
        let curve_offset = (progress * PI).sin() * curvature_factor;
        let perpendicular = Vector2::new(-total_vector.y, total_vector.x).normalized();
        
        perpendicular * curve_offset * 0.1
    }

    fn calculate_smoothing_factor(&self, velocity_magnitude: f64) -> f64 {
        // Higher smoothing for faster movements
        (velocity_magnitude / 10.0).clamp(0.1, 1.0)
    }
}

#[derive(Debug, Clone)]
struct TremorState {
    frequency: f64,
    amplitude: f64,
    phase: f64,
}

impl TremorState {
    fn new() -> Self {
        Self {
            frequency: 8.0 + fastrand::f64() * 4.0, // 8-12 Hz natural tremor
            amplitude: 0.1,
            phase: 0.0,
        }
    }

    fn calculate_tremor_force(&self, fatigue_level: f64, distance_to_target: f64) -> Vector2 {
        let fatigue_multiplier = 1.0 + fatigue_level * 2.0;
        let distance_multiplier = if distance_to_target < 50.0 {
            // More tremor when close to target (precision pressure)
            1.5
        } else {
            1.0
        };
        
        let amplitude = self.amplitude * fatigue_multiplier * distance_multiplier;
        
        Vector2::new(
            (self.phase * self.frequency).sin() * amplitude,
            (self.phase * self.frequency * 0.7).cos() * amplitude,
        )
    }

    fn update(&mut self, delta_time: f64) {
        self.phase += delta_time;
    }
}

// ============================================================================
// Natural Typing Engine
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NaturalTypingEngine {
    pub base_wpm: f64,
    pub keystroke_variance: f64,
    pub error_probability: f64,
    pub fatigue_model: TypingFatigue,
    pub correction_behavior: CorrectionStyle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingFatigue {
    pub current_fatigue: f64,
    pub fatigue_rate: f64,
    pub recovery_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CorrectionStyle {
    Immediate,     // Fix errors immediately
    Delayed,       // Fix errors after a few characters
    EndOfWord,     // Fix errors at word boundaries
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypingAction {
    pub character: char,
    pub timing: f64,
    pub is_correction: bool,
    pub micro_movements: Vec<Point>,
}

impl Default for NaturalTypingEngine {
    fn default() -> Self {
        Self {
            base_wpm: 65.0, // Average typing speed
            keystroke_variance: 0.3,
            error_probability: 0.02, // 2% error rate
            fatigue_model: TypingFatigue {
                current_fatigue: 0.0,
                fatigue_rate: 0.001,
                recovery_rate: 0.005,
            },
            correction_behavior: CorrectionStyle::Immediate,
        }
    }
}

impl NaturalTypingEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn generate_typing_sequence(&mut self, text: &str) -> Vec<TypingAction> {
        let mut sequence = Vec::new();
        let mut fatigue = self.fatigue_model.current_fatigue;
        
        // Base timing from WPM (characters per second)
        let base_time = 60.0 / (self.base_wpm * 5.0); // 5 chars per word average
        
        for (i, character) in text.char_indices() {
            // Calculate character-specific timing
            let char_modifier = self.calculate_character_timing_modifier(character);
            let fatigue_modifier = 1.0 + fatigue * 2.0; // Fatigue slows typing
            let variance = fastrand::f64() * self.keystroke_variance * 2.0 - self.keystroke_variance;
            
            let final_timing = base_time * char_modifier * fatigue_modifier * (1.0 + variance);
            
            // Check for typing errors
            if fastrand::f64() < self.error_probability * (1.0 + fatigue) {
                // Add error sequence
                let error_char = self.generate_adjacent_key_error(character);
                sequence.push(TypingAction {
                    character: error_char,
                    timing: final_timing,
                    is_correction: false,
                    micro_movements: self.generate_typing_micro_movements(),
                });
                
                // Add correction sequence
                let correction_delay = 0.2 + fastrand::f64() * 0.3; // 200-500ms delay
                sequence.push(TypingAction {
                    character: '\u{0008}', // Backspace
                    timing: correction_delay,
                    is_correction: true,
                    micro_movements: Vec::new(),
                });
            }
            
            // Add the correct character
            sequence.push(TypingAction {
                character,
                timing: final_timing,
                is_correction: false,
                micro_movements: self.generate_typing_micro_movements(),
            });
            
            // Update fatigue
            fatigue += self.fatigue_model.fatigue_rate;
            fatigue = fatigue.min(1.0);
            
            // Natural pauses at word boundaries
            if character == ' ' && i > 0 {
                let pause_chance = 0.1 + fatigue * 0.2;
                if fastrand::f64() < pause_chance {
                    let pause_duration = 0.1 + fastrand::f64() * 0.5;
                    sequence.push(TypingAction {
                        character: '\0', // Null character represents pause
                        timing: pause_duration,
                        is_correction: false,
                        micro_movements: Vec::new(),
                    });
                }
            }
        }
        
        // Update fatigue state
        self.fatigue_model.current_fatigue = fatigue;
        
        sequence
    }

    fn calculate_character_timing_modifier(&self, character: char) -> f64 {
        match character {
            'a'..='z' | 'A'..='Z' => 1.0,
            '0'..='9' => 1.2,
            ' ' => 0.7,
            '.' | ',' | ';' | ':' => 1.4,
            '!' | '?' => 1.8,
            '\n' => 1.5,
            '\t' => 1.6,
            _ => 1.3,
        }
    }

    fn generate_adjacent_key_error(&self, intended: char) -> char {
        // Simplified QWERTY adjacent key mapping
        match intended {
            'a' => ['s', 'q', 'w'][fastrand::usize(0..3)],
            'e' => ['w', 'r', 'd'][fastrand::usize(0..3)],
            'i' => ['u', 'o', 'k'][fastrand::usize(0..3)],
            'o' => ['i', 'p', 'l'][fastrand::usize(0..3)],
            't' => ['r', 'y', 'g'][fastrand::usize(0..3)],
            _ => intended, // No adjacent key mapping
        }
    }

    fn generate_typing_micro_movements(&self) -> Vec<Point> {
        // Generate subtle hand movements during typing
        let mut movements = Vec::new();
        let movement_count = fastrand::usize(1..4);
        
        for _ in 0..movement_count {
            movements.push(Point::new(
                fastrand::f64() * 2.0 - 1.0, // -1 to 1 pixel movement
                fastrand::f64() * 2.0 - 1.0,
            ));
        }
        
        movements
    }
}

// ============================================================================
// Main Automation Engine
// ============================================================================

#[derive(Debug)]
pub struct AutomationEngine {
    pub windmouse: WindMouseEngine,
    pub typing_engine: NaturalTypingEngine,
    pub session_id: Option<String>,
    pub is_recording: bool,
    performance_metrics: PerformanceMetrics,
}

#[derive(Debug, Clone, Default)]
struct PerformanceMetrics {
    movements_executed: u64,
    typing_actions_executed: u64,
    total_execution_time: Duration,
    average_movement_time: Duration,
}

impl AutomationEngine {
    pub fn new() -> Result<Self> {
        info!("Initializing AutomationEngine with WindMouse 2.0");
        
        Ok(Self {
            windmouse: WindMouseEngine::new(),
            typing_engine: NaturalTypingEngine::new(),
            session_id: None,
            is_recording: false,
            performance_metrics: PerformanceMetrics::default(),
        })
    }

    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }

    pub fn set_user_profile(&mut self, profile: UserMovementProfile) {
        self.windmouse = self.windmouse.clone().with_profile(profile);
    }

    /// Execute natural cursor movement from current position to target
    pub async fn move_cursor_naturally(
        &mut self,
        from: Point,
        to: Point,
        context: Option<MovementContext>,
    ) -> Result<()> {
        let start_time = Instant::now();
        
        let context = context.unwrap_or_default();
        let trajectory = self.windmouse.generate_movement_trajectory(from, to, context);
        
        info!("Executing natural cursor movement: ({:.0},{:.0}) -> ({:.0},{:.0}) with {} frames",
              from.x, from.y, to.x, to.y, trajectory.len());
        
        for frame in trajectory {
            // Execute cursor position update (platform-specific implementation needed)
            self.execute_cursor_position(frame.position).await?;
            
            // Natural frame timing (60 FPS)
            sleep(Duration::from_secs_f64(1.0 / 60.0)).await;
        }
        
        // Update performance metrics
        self.performance_metrics.movements_executed += 1;
        self.performance_metrics.total_execution_time += start_time.elapsed();
        
        debug!("Natural cursor movement completed in {:?}", start_time.elapsed());
        Ok(())
    }

    /// Execute natural typing with character-by-character timing
    pub async fn type_text_naturally(&mut self, text: &str) -> Result<()> {
        let start_time = Instant::now();
        
        let typing_sequence = self.typing_engine.generate_typing_sequence(text);
        let sequence_len = typing_sequence.len();
        
        info!("Executing natural typing: '{}' with {} actions", 
              text.chars().take(50).collect::<String>(), sequence_len);
        
        for action in typing_sequence {
            if action.character == '\0' {
                // Pause action
                sleep(Duration::from_secs_f64(action.timing)).await;
                continue;
            }
            
            if action.character == '\u{0008}' {
                // Backspace
                self.execute_key_press("BackSpace").await?;
            } else {
                // Normal character
                self.execute_character_input(action.character).await?;
            }
            
            // Execute micro-movements during typing
            for movement in action.micro_movements {
                self.execute_micro_movement(movement).await?;
            }
            
            // Natural keystroke timing
            sleep(Duration::from_secs_f64(action.timing)).await;
        }
        
        // Update performance metrics
        self.performance_metrics.typing_actions_executed += sequence_len as u64;
        
        debug!("Natural typing completed in {:?}", start_time.elapsed());
        Ok(())
    }

    /// Execute a natural click with WindMouse movement
    pub async fn click_naturally(
        &mut self,
        current_pos: Point,
        target: Point,
        button: MouseButton,
    ) -> Result<()> {
        info!("Executing natural click at ({:.0},{:.0})", target.x, target.y);
        
        // Move cursor naturally to target
        self.move_cursor_naturally(current_pos, target, None).await?;
        
        // Natural pre-click pause
        let pre_click_delay = 0.05 + fastrand::f64() * 0.1; // 50-150ms
        sleep(Duration::from_secs_f64(pre_click_delay)).await;
        
        // Execute click
        self.execute_mouse_click(button).await?;
        
        // Natural post-click pause
        let post_click_delay = 0.03 + fastrand::f64() * 0.07; // 30-100ms
        sleep(Duration::from_secs_f64(post_click_delay)).await;
        
        Ok(())
    }

    /// Get current performance metrics
    pub fn get_performance_metrics(&self) -> &PerformanceMetrics {
        &self.performance_metrics
    }

    // ========================================================================
    // Platform-specific implementations (to be implemented per platform)
    // ========================================================================

    async fn execute_cursor_position(&self, position: Point) -> Result<()> {
        // Platform-specific cursor positioning
        // This would use X11, Windows API, or macOS API depending on target
        debug!("Moving cursor to ({:.1}, {:.1})", position.x, position.y);
        Ok(())
    }

    async fn execute_character_input(&self, character: char) -> Result<()> {
        // Platform-specific character input
        debug!("Typing character: '{}'", character);
        Ok(())
    }

    async fn execute_key_press(&self, key: &str) -> Result<()> {
        // Platform-specific key press
        debug!("Pressing key: {}", key);
        Ok(())
    }

    async fn execute_mouse_click(&self, button: MouseButton) -> Result<()> {
        // Platform-specific mouse click
        debug!("Clicking mouse button: {:?}", button);
        Ok(())
    }

    async fn execute_micro_movement(&self, _movement: Point) -> Result<()> {
        // Platform-specific micro-movement
        // Very subtle cursor adjustments during typing
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

impl Default for AutomationEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create AutomationEngine")
    }
}

// ============================================================================
// Tests: Automation Engine
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-AUTOMATION-001 (Point geometry)
    #[test]
    fn test_point_creation_and_distance() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);

        assert_eq!(p1.x, 0.0);
        assert_eq!(p1.y, 0.0);
        assert!((p1.distance_to(p2) - 5.0).abs() < 0.001);
    }

    // Traces to: FR-AUTOMATION-001 (Point addition)
    #[test]
    fn test_point_vector_addition() {
        let p = Point::new(5.0, 3.0);
        let v = Vector2::new(2.0, 1.0);
        let result = p + v;

        assert_eq!(result.x, 7.0);
        assert_eq!(result.y, 4.0);
    }

    // Traces to: FR-AUTOMATION-001 (Point subtraction)
    #[test]
    fn test_point_subtraction_to_vector() {
        let p1 = Point::new(5.0, 3.0);
        let p2 = Point::new(2.0, 1.0);
        let v = p1 - p2;

        assert_eq!(v.x, 3.0);
        assert_eq!(v.y, 2.0);
    }

    // Traces to: FR-AUTOMATION-002 (Vector magnitude)
    #[test]
    fn test_vector_magnitude() {
        let v = Vector2::new(3.0, 4.0);
        assert!((v.magnitude() - 5.0).abs() < 0.001);
    }

    // Traces to: FR-AUTOMATION-002 (Vector normalization)
    #[test]
    fn test_vector_normalization() {
        let v = Vector2::new(3.0, 4.0);
        let normalized = v.normalized();
        let mag = normalized.magnitude();

        assert!((mag - 1.0).abs() < 0.001);
    }

    // Traces to: FR-AUTOMATION-003 (WindMouse initialization)
    #[test]
    fn test_windmouse_initialization() {
        let engine = WindMouseEngine::new();

        assert_eq!(engine.gravity, 12.0);
        assert_eq!(engine.wind, 4.0);
        assert_eq!(engine.friction, 0.95);
        assert_eq!(engine.user_profile.movement_speed, 1.0);
    }

    // Traces to: FR-AUTOMATION-004 (AutomationEngine creation)
    #[test]
    fn test_automation_engine_creation() {
        let engine = AutomationEngine::new();
        assert!(engine.is_ok());

        let engine = engine.unwrap();
        assert!(!engine.is_recording);
        assert!(engine.session_id.is_none());
    }

    // Traces to: FR-AUTOMATION-005 (Session assignment)
    #[test]
    fn test_automation_engine_with_session() {
        let engine = AutomationEngine::new()
            .unwrap()
            .with_session("test-session-123".to_string());

        assert_eq!(engine.session_id, Some("test-session-123".to_string()));
    }

    // Traces to: FR-AUTOMATION-006 (User profile setting)
    #[test]
    fn test_set_user_profile() {
        let mut engine = AutomationEngine::new().unwrap();
        let profile = UserMovementProfile {
            movement_speed: 0.5,
            precision_level: 0.9,
            jitter_amount: 0.05,
            hesitation_factor: 0.3,
            fatigue_level: 0.1,
            path_curvature: 0.5,
        };

        engine.set_user_profile(profile);
        assert_eq!(engine.windmouse.user_profile.movement_speed, 0.5);
    }

    // Traces to: FR-AUTOMATION-007 (Natural typing sequence generation)
    #[test]
    fn test_natural_typing_sequence_generation() {
        let mut engine = NaturalTypingEngine::new();
        let sequence = engine.generate_typing_sequence("hello");

        assert!(!sequence.is_empty());
        // Should have actions for each character plus possible errors/corrections
        assert!(sequence.len() >= 5);
    }

    // Traces to: FR-AUTOMATION-008 (Workflow execution state)
    #[tokio::test]
    async fn test_automation_engine_default() {
        let engine = AutomationEngine::default();
        assert!(!engine.is_recording);
        assert!(engine.session_id.is_none());
    }

    // Traces to: FR-AUTOMATION-009 (MovementContext defaults)
    #[test]
    fn test_movement_context_defaults() {
        let ctx = MovementContext::default();

        assert_eq!(ctx.base_gravity, 12.0);
        assert_eq!(ctx.wind_strength, 4.0);
        assert_eq!(ctx.precision_threshold, 50.0);
        assert!(!ctx.obstacle_avoidance);
    }

    // Traces to: FR-AUTOMATION-010 (Performance metrics initialization)
    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();

        assert_eq!(metrics.movements_executed, 0);
        assert_eq!(metrics.typing_actions_executed, 0);
        assert_eq!(metrics.total_execution_time, Duration::from_secs(0));
    }
}

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum AutomationError {
    #[error("Platform operation failed: {0}")]
    PlatformError(String),
    
    #[error("Invalid coordinates: ({0}, {1})")]
    InvalidCoordinates(f64, f64),
    
    #[error("Session not active: {0}")]
    SessionNotActive(String),
    
    #[error("Automation timeout after {0:?}")]
    Timeout(Duration),
}

// ============================================================================
// Public API
// ============================================================================

/// High-level automation API for common operations
impl AutomationEngine {
    /// Execute a complete automation workflow
    pub async fn execute_workflow(&mut self, workflow: AutomationWorkflow) -> Result<WorkflowResult> {
        let start_time = Instant::now();
        let mut results = Vec::new();
        
        info!("Executing automation workflow: '{}'", workflow.name);
        
        for (i, step) in workflow.steps.iter().enumerate() {
            let step_start = Instant::now();
            
            match self.execute_step(step).await {
                Ok(result) => {
                    results.push(StepResult {
                        step_number: i + 1,
                        success: true,
                        execution_time: step_start.elapsed(),
                        error: None,
                        metadata: result,
                    });
                }
                Err(e) => {
                    warn!("Workflow step {} failed: {}", i + 1, e);
                    results.push(StepResult {
                        step_number: i + 1,
                        success: false,
                        execution_time: step_start.elapsed(),
                        error: Some(e.to_string()),
                        metadata: HashMap::new(),
                    });
                    
                    if !workflow.continue_on_error {
                        break;
                    }
                }
            }
        }
        
        let workflow_result = WorkflowResult {
            workflow_name: workflow.name,
            total_steps: workflow.steps.len(),
            successful_steps: results.iter().filter(|r| r.success).count(),
            total_execution_time: start_time.elapsed(),
            step_results: results,
        };
        
        info!("Workflow completed: {}/{} steps successful in {:?}",
              workflow_result.successful_steps, workflow_result.total_steps,
              workflow_result.total_execution_time);
        
        Ok(workflow_result)
    }

    async fn execute_step(&mut self, step: &WorkflowStep) -> Result<HashMap<String, serde_json::Value>> {
        match &step.action {
            StepAction::MoveCursor { to } => {
                let from = Point::new(0.0, 0.0); // Get current cursor position
                self.move_cursor_naturally(from, *to, None).await?;
                Ok(HashMap::new())
            }
            StepAction::Click { position, button } => {
                let from = Point::new(0.0, 0.0); // Get current cursor position
                self.click_naturally(from, *position, *button).await?;
                Ok(HashMap::new())
            }
            StepAction::Type { text } => {
                self.type_text_naturally(text).await?;
                Ok(HashMap::new())
            }
            StepAction::Wait { duration } => {
                sleep(*duration).await;
                Ok(HashMap::new())
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationWorkflow {
    pub name: String,
    pub description: String,
    pub steps: Vec<WorkflowStep>,
    pub continue_on_error: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub name: String,
    pub action: StepAction,
    pub timeout: Option<Duration>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepAction {
    MoveCursor { to: Point },
    Click { position: Point, button: MouseButton },
    Type { text: String },
    Wait { duration: Duration },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    pub workflow_name: String,
    pub total_steps: usize,
    pub successful_steps: usize,
    pub total_execution_time: Duration,
    pub step_results: Vec<StepResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StepResult {
    pub step_number: usize,
    pub success: bool,
    pub execution_time: Duration,
    pub error: Option<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}