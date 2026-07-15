/*!
KVirtualStage Demo Runner
Executes verified automation demonstrations as requested
*/

use anyhow::Result;
use kvirtualstage::automation::ComprehensiveAutomationPlatform;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing with detailed logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!("🚀 KVirtualStage Demo Runner Started");
    info!("🎯 Mission: Generate verified automation demos with real screenshot validation");

    // Create the comprehensive automation platform
    let platform = match ComprehensiveAutomationPlatform::new() {
        Ok(p) => {
            info!("✅ Automation platform initialized successfully");
            p
        }
        Err(e) => {
            error!("❌ Failed to initialize automation platform: {}", e);
            return Err(e);
        }
    };

    // Generate the verified demo as requested
    info!("🎬 Starting verified demo generation...");
    
    match platform.generate_verified_demo("Comprehensive_Automation_Demo").await {
        Ok(verification_results) => {
            info!("🏆 Demo generation completed successfully!");
            info!("📊 Verification Results Summary:");
            
            let total = verification_results.len();
            let passed = verification_results.iter().filter(|r| r.matches).count();
            let failed = total - passed;
            
            info!("   📈 Total Steps: {}", total);
            info!("   ✅ Passed: {} ({:.1}%)", passed, (passed as f64 / total as f64) * 100.0);
            info!("   ❌ Failed: {} ({:.1}%)", failed, (failed as f64 / total as f64) * 100.0);
            
            // Show detailed results
            for result in &verification_results {
                if result.matches {
                    info!("   ✅ Step {}: {} (confidence: {:.2})", 
                        result.step_number, result.expected, result.confidence);
                } else {
                    info!("   ❌ Step {}: Expected '{}', got '{}' (confidence: {:.2})", 
                        result.step_number, result.expected, result.actual, result.confidence);
                }
                info!("      📸 Screenshot: {}", result.screenshot_path);
            }
            
            // Generate verification report
            info!("📝 Generating verification report...");
            let verification_engine = &platform.verification_engine;
            let report = verification_engine.generate_report(&verification_results).await?;
            
            let report_path = "/tmp/automation_verification_report.md";
            tokio::fs::write(report_path, report).await?;
            info!("✅ Verification report saved: {}", report_path);
            
            // Success message
            info!("🎉 SUCCESS: Verified automation demo completed!");
            info!("🔗 Key outputs:");
            info!("   📹 Video: Comprehensive_Automation_Demo_recording.mp4");
            info!("   📊 Report: {}", report_path);
            info!("   📸 Screenshots: /tmp/demo_*.png");
            
            if passed == total {
                info!("🏆 PERFECT SCORE: All user story steps verified successfully!");
                info!("✨ The automation performed exactly as planned");
            } else {
                info!("⚠️  Some verification steps failed - see report for details");
            }
            
            Ok(())
        }
        Err(e) => {
            error!("❌ Demo generation failed: {}", e);
            error!("🔧 Check your desktop environment setup:");
            error!("   - Ensure X11 is running");
            error!("   - Check that applications can be launched");
            error!("   - Verify screenshot tools are available");
            Err(e)
        }
    }
}