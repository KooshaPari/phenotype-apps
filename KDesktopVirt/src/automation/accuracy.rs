/*!
Pixel-Perfect Accuracy Engine for Desktop Automation
Provides mathematical coordinate calculation and smooth cursor movement
*/

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::process::Command as AsyncCommand;
use tokio::time::{sleep, Duration};
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub id: String,
    pub title: String,
    pub class: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ButtonLayout {
    pub buttons: HashMap<String, (i32, i32)>,
    pub window_info: WindowInfo,
}

/// Accuracy Engine for pixel-perfect desktop automation
pub struct AccuracyEngine {
    pub animation_duration: f64,
    pub click_precision: i32,
}

impl AccuracyEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            animation_duration: 0.5,
            click_precision: 1,
        })
    }

    /// Take screenshot with verification
    pub async fn take_screenshot(&self, output_path: &str) -> Result<()> {
        info!("📸 Taking screenshot: {}", output_path);
        
        let output = AsyncCommand::new("scrot")
            .arg(output_path)
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Screenshot failed: {}", 
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Verify screenshot was created
        if !tokio::fs::metadata(output_path).await.is_ok() {
            return Err(anyhow::anyhow!("Screenshot file not created: {}", output_path));
        }

        info!("✅ Screenshot saved: {}", output_path);
        Ok(())
    }

    /// Launch application with verification
    pub async fn launch_application(&self, app_name: &str) -> Result<()> {
        info!("🚀 Launching application: {}", app_name);
        
        // Launch the application
        AsyncCommand::new(app_name)
            .spawn()?;

        // Wait for application to start
        sleep(Duration::from_secs(3)).await;

        // Verify application launched by checking window list
        let windows = self.list_windows().await?;
        let app_running = windows.iter().any(|w| 
            w.title.to_lowercase().contains(&app_name.to_lowercase()) ||
            w.class.to_lowercase().contains(&app_name.to_lowercase())
        );

        if !app_running {
            warn!("⚠️ Application may not have launched correctly: {}", app_name);
        } else {
            info!("✅ Application launched successfully: {}", app_name);
        }

        Ok(())
    }

    /// Precise click with smooth cursor movement
    pub async fn precise_click(&self, x: i32, y: i32, description: Option<&str>) -> Result<()> {
        let desc = description.unwrap_or("Precise click");
        info!("🎯 {}: ({}, {})", desc, x, y);

        // Smooth cursor movement with cubic easing
        self.smooth_cursor_movement(x, y).await?;

        // Execute the click
        let output = AsyncCommand::new("xdotool")
            .args(&["click", "1"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Click failed: {}", 
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        // Brief pause after click
        sleep(Duration::from_millis(200)).await;
        
        info!("✅ Click completed at ({}, {})", x, y);
        Ok(())
    }

    /// Type text with natural timing
    pub async fn type_text(&self, text: &str) -> Result<()> {
        info!("⌨️ Typing text: {}", if text.len() > 50 { 
            format!("{}...", &text[..50]) 
        } else { 
            text.to_string() 
        });

        for char in text.chars() {
            if char == '\n' {
                AsyncCommand::new("xdotool")
                    .args(&["key", "Return"])
                    .output()
                    .await?;
                sleep(Duration::from_millis(200)).await;
            } else {
                AsyncCommand::new("xdotool")
                    .args(&["type", "--delay", "50", &char.to_string()])
                    .output()
                    .await?;
                sleep(Duration::from_millis(50)).await;
            }
        }

        info!("✅ Text typed successfully");
        Ok(())
    }

    /// Find window information by class or title
    pub async fn find_window_info(&self, window_identifier: &str) -> Result<Option<WindowInfo>> {
        let windows = self.list_windows().await?;
        
        // Try to find by class first, then by title
        for window in windows {
            if window.class.to_lowercase().contains(&window_identifier.to_lowercase()) ||
               window.title.to_lowercase().contains(&window_identifier.to_lowercase()) {
                return Ok(Some(window));
            }
        }
        
        Ok(None)
    }

    /// List all open windows
    pub async fn list_windows(&self) -> Result<Vec<WindowInfo>> {
        let output = AsyncCommand::new("wmctrl")
            .args(&["-l", "-G"])
            .output()
            .await?;

        if !output.status.success() {
            return Err(anyhow::anyhow!(
                "Failed to list windows: {}", 
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut windows = Vec::new();

        for line in stdout.lines() {
            if let Some(window) = self.parse_window_line(line).await? {
                windows.push(window);
            }
        }

        Ok(windows)
    }

    /// Calculate button layout for calculator application
    pub async fn calculate_calculator_buttons(&self, window_info: &WindowInfo) -> Result<ButtonLayout> {
        info!("🔢 Calculating calculator button layout");

        // Standard calculator button layout (4x5 grid)
        let button_width = window_info.width / 4;
        let button_height = (window_info.height - 80) / 5; // Account for title bar and display
        
        let mut buttons = HashMap::new();
        
        // Number buttons (0-9)
        let number_positions = [
            ("7", 0, 1), ("8", 1, 1), ("9", 2, 1),
            ("4", 0, 2), ("5", 1, 2), ("6", 2, 2),
            ("1", 0, 3), ("2", 1, 3), ("3", 2, 3),
            ("0", 1, 4),
        ];

        for (number, col, row) in number_positions {
            let x = window_info.x + (col * button_width) + (button_width / 2);
            let y = window_info.y + 80 + (row * button_height) + (button_height / 2);
            buttons.insert(number.to_string(), (x, y));
        }

        // Operator buttons
        let operator_positions = [
            ("÷", 3, 1), ("×", 3, 2), ("-", 3, 3), ("+", 3, 4),
            ("=", 2, 4), (".", 0, 4),
        ];

        for (operator, col, row) in operator_positions {
            let x = window_info.x + (col * button_width) + (button_width / 2);
            let y = window_info.y + 80 + (row * button_height) + (button_height / 2);
            buttons.insert(operator.to_string(), (x, y));
        }

        // Clear button
        let clear_x = window_info.x + (3 * button_width) + (button_width / 2);
        let clear_y = window_info.y + 40 + (button_height / 2);
        buttons.insert("C".to_string(), (clear_x, clear_y));

        info!("✅ Calculator layout calculated: {} buttons", buttons.len());

        Ok(ButtonLayout {
            buttons,
            window_info: window_info.clone(),
        })
    }

    /// Smooth cursor movement with cubic easing
    async fn smooth_cursor_movement(&self, target_x: i32, target_y: i32) -> Result<()> {
        // Get current cursor position
        let output = AsyncCommand::new("xdotool")
            .args(&["getmouselocation", "--shell"])
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut current_x = 0;
        let mut current_y = 0;

        for line in stdout.lines() {
            if line.starts_with("X=") {
                current_x = line[2..].parse().unwrap_or(0);
            } else if line.starts_with("Y=") {
                current_y = line[2..].parse().unwrap_or(0);
            }
        }

        // Calculate movement steps
        let steps = 10;
        let dx = target_x - current_x;
        let dy = target_y - current_y;

        // Smooth movement using cubic easing
        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            let eased_t = self.cubic_ease_in_out(t);
            
            let x = current_x + (dx as f64 * eased_t) as i32;
            let y = current_y + (dy as f64 * eased_t) as i32;

            AsyncCommand::new("xdotool")
                .args(&["mousemove", &x.to_string(), &y.to_string()])
                .output()
                .await?;

            sleep(Duration::from_millis(20)).await;
        }

        Ok(())
    }

    /// Cubic ease-in-out function for smooth animation
    fn cubic_ease_in_out(&self, t: f64) -> f64 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }

    /// Parse window information from wmctrl output
    async fn parse_window_line(&self, line: &str) -> Result<Option<WindowInfo>> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        if parts.len() >= 7 {
            let id = parts[0].to_string();
            let x = parts[2].parse().unwrap_or(0);
            let y = parts[3].parse().unwrap_or(0);
            let width = parts[4].parse().unwrap_or(0);
            let height = parts[5].parse().unwrap_or(0);
            
            // Get window class
            let class_output = AsyncCommand::new("xprop")
                .args(&["-id", &id, "WM_CLASS"])
                .output()
                .await?;
            
            let class = if class_output.status.success() {
                let class_str = String::from_utf8_lossy(&class_output.stdout);
                class_str.split('"').nth(3).unwrap_or("unknown").to_string()
            } else {
                "unknown".to_string()
            };

            let title = if parts.len() > 7 {
                parts[7..].join(" ")
            } else {
                "".to_string()
            };

            return Ok(Some(WindowInfo {
                id, x, y, width, height, title, class
            }));
        }
        
        Ok(None)
    }

    /// Wait for application to be ready
    pub async fn wait_for_application(&self, app_name: &str, timeout_secs: u64) -> Result<bool> {
        info!("⏳ Waiting for application: {} (timeout: {}s)", app_name, timeout_secs);
        
        for _ in 0..timeout_secs {
            let windows = self.list_windows().await?;
            let app_ready = windows.iter().any(|w| 
                w.title.to_lowercase().contains(&app_name.to_lowercase()) ||
                w.class.to_lowercase().contains(&app_name.to_lowercase())
            );

            if app_ready {
                info!("✅ Application ready: {}", app_name);
                return Ok(true);
            }

            sleep(Duration::from_secs(1)).await;
        }

        warn!("⏰ Timeout waiting for application: {}", app_name);
        Ok(false)
    }
}

impl Default for AccuracyEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create accuracy engine")
    }
}