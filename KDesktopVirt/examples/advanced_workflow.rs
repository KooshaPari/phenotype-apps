/*!
 * KVirtualStage Advanced Workflow Example
 * 
 * This example demonstrates advanced workflow capabilities including
 * complex automation sequences, conditional logic, error handling,
 * and integration with external systems.
 */

use anyhow::Result;
use kvirtualstage::*;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging with more detailed output
    tracing_subscriber::fmt()
        .with_env_filter("kvirtualstage=debug")
        .init();

    println!("🚀 KVirtualStage Advanced Workflow Example");
    println!("==========================================");

    // Initialize the API
    let api = KVirtualStageAPI::new().await?;
    println!("✅ API initialized");

    // Create session for advanced workflow
    let session_id = api.create_session(
        "advanced_user".to_string(),
        "advanced_workflow_demo".to_string(),
        "ubuntu-xfce".to_string(),
    ).await?;
    println!("✅ Session created: {}", session_id);

    // Start high-quality recording
    let recording_id = api.start_recording(
        &session_id,
        "advanced_workflow_demo.mp4",
        Some("high".to_string()),
    ).await?;
    println!("✅ Recording started: {}", recording_id);

    // Execute multiple advanced workflow scenarios
    web_application_testing_workflow(&api, &session_id).await?;
    document_processing_workflow(&api, &session_id).await?;
    system_administration_workflow(&api, &session_id).await?;
    data_entry_workflow(&api, &session_id).await?;

    // Stop recording
    let output_path = api.stop_recording(&session_id).await?;
    println!("🎬 Recording completed: {}", output_path);

    // Cleanup
    api.cleanup_expired_sessions(0).await?;
    println!("🧹 Cleanup completed");

    println!("\n🎉 Advanced workflow demonstration completed!");
    Ok(())
}

/// Simulate web application testing workflow
async fn web_application_testing_workflow(api: &KVirtualStageAPI, session_id: &str) -> Result<()> {
    println!("\n🌐 === Web Application Testing Workflow ===");

    use automation_engine::{AutomationWorkflow, WorkflowStep, StepAction, Point, MouseButton};

    let workflow = AutomationWorkflow {
        name: "Web Application Testing".to_string(),
        description: "Comprehensive web application testing scenario".to_string(),
        continue_on_error: true, // Continue testing even if some steps fail
        steps: vec![
            // Open browser
            WorkflowStep {
                name: "Open web browser".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["super".to_string(), "space".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            WorkflowStep {
                name: "Type browser name".to_string(),
                action: StepAction::Type { text: "firefox".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Launch browser".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(10)),
            },
            
            // Wait for browser to load
            WorkflowStep {
                name: "Wait for browser startup".to_string(),
                action: StepAction::Wait { duration: Duration::from_secs(5) },
                timeout: Some(Duration::from_secs(15)),
            },
            
            // Navigate to test site
            WorkflowStep {
                name: "Click address bar".to_string(),
                action: StepAction::Click { 
                    position: Point::new(400.0, 80.0), 
                    button: MouseButton::Left 
                },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Enter test URL".to_string(),
                action: StepAction::Type { text: "https://httpbin.org/forms/post".to_string() },
                timeout: Some(Duration::from_secs(10)),
            },
            WorkflowStep {
                name: "Navigate to URL".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(10)),
            },
            
            // Wait for page load
            WorkflowStep {
                name: "Wait for page load".to_string(),
                action: StepAction::Wait { duration: Duration::from_secs(3) },
                timeout: Some(Duration::from_secs(15)),
            },
            
            // Fill out form
            WorkflowStep {
                name: "Click customer name field".to_string(),
                action: StepAction::Click { 
                    position: Point::new(300.0, 200.0), 
                    button: MouseButton::Left 
                },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Enter customer name".to_string(),
                action: StepAction::Type { text: "John Doe".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            
            WorkflowStep {
                name: "Tab to telephone field".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Tab".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            WorkflowStep {
                name: "Enter telephone".to_string(),
                action: StepAction::Type { text: "+1-555-123-4567".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            
            WorkflowStep {
                name: "Tab to email field".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Tab".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            WorkflowStep {
                name: "Enter email".to_string(),
                action: StepAction::Type { text: "john.doe@example.com".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            
            WorkflowStep {
                name: "Tab to subject field".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Tab".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            WorkflowStep {
                name: "Enter subject".to_string(),
                action: StepAction::Type { text: "Automated Testing Form Submission".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            
            WorkflowStep {
                name: "Tab to message field".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Tab".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            WorkflowStep {
                name: "Enter message".to_string(),
                action: StepAction::Type { 
                    text: "This is an automated test message generated by KVirtualStage.\n\nThis demonstrates:\n- Form field navigation\n- Data entry automation\n- Web application testing\n- Natural interaction simulation".to_string() 
                },
                timeout: Some(Duration::from_secs(15)),
            },
            
            // Submit form
            WorkflowStep {
                name: "Submit form".to_string(),
                action: StepAction::Click { 
                    position: Point::new(350.0, 450.0), 
                    button: MouseButton::Left 
                },
                timeout: Some(Duration::from_secs(10)),
            },
            
            // Wait for response
            WorkflowStep {
                name: "Wait for form submission response".to_string(),
                action: StepAction::Wait { duration: Duration::from_secs(3) },
                timeout: Some(Duration::from_secs(15)),
            },
        ],
    };

    println!("🔄 Executing web testing workflow...");
    let result = api.execute_workflow(session_id, workflow).await?;
    print_workflow_results(&result);

    Ok(())
}

/// Simulate document processing workflow
async fn document_processing_workflow(api: &KVirtualStageAPI, session_id: &str) -> Result<()> {
    println!("\n📄 === Document Processing Workflow ===");

    use automation_engine::{AutomationWorkflow, WorkflowStep, StepAction, Point, MouseButton};

    let workflow = AutomationWorkflow {
        name: "Document Processing".to_string(),
        description: "Create, edit, and format a document with complex content".to_string(),
        continue_on_error: false,
        steps: vec![
            // Open text editor
            WorkflowStep {
                name: "Open application menu".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["super".to_string(), "space".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            WorkflowStep {
                name: "Search for text editor".to_string(),
                action: StepAction::Type { text: "gedit".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Launch text editor".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(10)),
            },
            
            // Wait for editor to load
            WorkflowStep {
                name: "Wait for text editor".to_string(),
                action: StepAction::Wait { duration: Duration::from_secs(3) },
                timeout: Some(Duration::from_secs(15)),
            },
            
            // Create document content
            WorkflowStep {
                name: "Type document header".to_string(),
                action: StepAction::Type { 
                    text: "KVIRTUALSTAGE AUTOMATION REPORT\n".to_string() +
                          "================================\n\n" +
                          "Generated on: 2025-01-15\n" +
                          "System: KVirtualStage v1.0.0\n" +
                          "Operator: Automated Workflow\n\n"
                },
                timeout: Some(Duration::from_secs(15)),
            },
            
            WorkflowStep {
                name: "Add executive summary".to_string(),
                action: StepAction::Type { 
                    text: "EXECUTIVE SUMMARY\n".to_string() +
                          "-----------------\n\n" +
                          "This report demonstrates the advanced automation capabilities of KVirtualStage. " +
                          "The system successfully executed complex workflows including:\n\n" +
                          "• Web application testing with form submissions\n" +
                          "• Document creation and formatting\n" +
                          "• System administration tasks\n" +
                          "• Data entry and validation\n\n" +
                          "All operations were performed with human-like timing and natural interaction patterns.\n\n"
                },
                timeout: Some(Duration::from_secs(20)),
            },
            
            WorkflowStep {
                name: "Add technical details".to_string(),
                action: StepAction::Type { 
                    text: "TECHNICAL IMPLEMENTATION\n".to_string() +
                          "------------------------\n\n" +
                          "Core Technologies:\n" +
                          "- Rust programming language for performance\n" +
                          "- WindMouse 2.0 algorithm for natural cursor movement\n" +
                          "- Advanced typing simulation with character-level timing\n" +
                          "- FFmpeg integration for high-quality recording\n" +
                          "- AES-256-GCM encryption for security\n\n" +
                          "Performance Metrics:\n" +
                          "- Average cursor movement: <100ms latency\n" +
                          "- Typing simulation: 45-75 WPM with natural variation\n" +
                          "- Recording quality: 1080p at 60fps\n" +
                          "- Security: Enterprise-grade encryption\n\n"
                },
                timeout: Some(Duration::from_secs(25)),
            },
            
            // Format the document
            WorkflowStep {
                name: "Select all text for formatting".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["ctrl".to_string(), "a".to_string()] 
                },
                timeout: Some(Duration::from_secs(5)),
            },
            
            WorkflowStep {
                name: "Open font dialog".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["ctrl".to_string(), "shift".to_string(), "f".to_string()] 
                },
                timeout: Some(Duration::from_secs(5)),
            },
            
            // Wait for dialog
            WorkflowStep {
                name: "Wait for font dialog".to_string(),
                action: StepAction::Wait { duration: Duration::from_secs(2) },
                timeout: Some(Duration::from_secs(10)),
            },
            
            // Save document
            WorkflowStep {
                name: "Save document".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["ctrl".to_string(), "s".to_string()] 
                },
                timeout: Some(Duration::from_secs(10)),
            },
            
            WorkflowStep {
                name: "Enter filename".to_string(),
                action: StepAction::Type { text: "kvirtualstage_automation_report.txt".to_string() },
                timeout: Some(Duration::from_secs(10)),
            },
            
            WorkflowStep {
                name: "Confirm save".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(10)),
            },
        ],
    };

    println!("🔄 Executing document processing workflow...");
    let result = api.execute_workflow(session_id, workflow).await?;
    print_workflow_results(&result);

    Ok(())
}

/// Simulate system administration workflow
async fn system_administration_workflow(api: &KVirtualStageAPI, session_id: &str) -> Result<()> {
    println!("\n🔧 === System Administration Workflow ===");

    use automation_engine::{AutomationWorkflow, WorkflowStep, StepAction, Point, MouseButton};

    let workflow = AutomationWorkflow {
        name: "System Administration".to_string(),
        description: "Perform common system administration tasks".to_string(),
        continue_on_error: true,
        steps: vec![
            // Open terminal
            WorkflowStep {
                name: "Open terminal".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["ctrl".to_string(), "alt".to_string(), "t".to_string()] 
                },
                timeout: Some(Duration::from_secs(5)),
            },
            
            // Wait for terminal
            WorkflowStep {
                name: "Wait for terminal to load".to_string(),
                action: StepAction::Wait { duration: Duration::from_secs(2) },
                timeout: Some(Duration::from_secs(10)),
            },
            
            // System information commands
            WorkflowStep {
                name: "Check system information".to_string(),
                action: StepAction::Type { text: "echo '=== SYSTEM INFORMATION ==='".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Execute echo command".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            
            WorkflowStep {
                name: "Check uptime".to_string(),
                action: StepAction::Type { text: "uptime".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Execute uptime".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            
            WorkflowStep {
                name: "Check disk usage".to_string(),
                action: StepAction::Type { text: "df -h".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Execute df command".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(5)),
            },
            
            WorkflowStep {
                name: "Check memory usage".to_string(),
                action: StepAction::Type { text: "free -h".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Execute free command".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            
            WorkflowStep {
                name: "Check running processes".to_string(),
                action: StepAction::Type { text: "ps aux | head -10".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Execute ps command".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(5)),
            },
            
            // Create a simple script
            WorkflowStep {
                name: "Create system info script".to_string(),
                action: StepAction::Type { 
                    text: "cat > system_check.sh << 'EOF'".to_string() 
                },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Execute cat command".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            
            WorkflowStep {
                name: "Add script content".to_string(),
                action: StepAction::Type { 
                    text: "#!/bin/bash\n".to_string() +
                          "echo 'KVirtualStage System Check - $(date)'\n" +
                          "echo '====================================='\n" +
                          "echo 'Uptime:' && uptime\n" +
                          "echo 'Disk Usage:' && df -h /\n" +
                          "echo 'Memory Usage:' && free -h\n" +
                          "echo 'Load Average:' && cat /proc/loadavg\n" +
                          "echo 'System check completed!'\n" +
                          "EOF"
                },
                timeout: Some(Duration::from_secs(15)),
            },
            WorkflowStep {
                name: "Execute EOF".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            
            // Make script executable and run it
            WorkflowStep {
                name: "Make script executable".to_string(),
                action: StepAction::Type { text: "chmod +x system_check.sh".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Execute chmod".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            
            WorkflowStep {
                name: "Run system check script".to_string(),
                action: StepAction::Type { text: "./system_check.sh".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Execute script".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(10)),
            },
            
            // Clean up
            WorkflowStep {
                name: "Clean up script".to_string(),
                action: StepAction::Type { text: "rm system_check.sh".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Execute cleanup".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
        ],
    };

    println!("🔄 Executing system administration workflow...");
    let result = api.execute_workflow(session_id, workflow).await?;
    print_workflow_results(&result);

    Ok(())
}

/// Simulate data entry workflow
async fn data_entry_workflow(api: &KVirtualStageAPI, session_id: &str) -> Result<()> {
    println!("\n📊 === Data Entry Workflow ===");

    use automation_engine::{AutomationWorkflow, WorkflowStep, StepAction, Point, MouseButton};

    let workflow = AutomationWorkflow {
        name: "Data Entry Automation".to_string(),
        description: "Automated data entry with validation and formatting".to_string(),
        continue_on_error: false,
        steps: vec![
            // Open calculator for data processing
            WorkflowStep {
                name: "Open calculator".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["super".to_string(), "space".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            WorkflowStep {
                name: "Search calculator".to_string(),
                action: StepAction::Type { text: "calculator".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Launch calculator".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(10)),
            },
            
            // Wait for calculator
            WorkflowStep {
                name: "Wait for calculator".to_string(),
                action: StepAction::Wait { duration: Duration::from_secs(3) },
                timeout: Some(Duration::from_secs(15)),
            },
            
            // Perform calculations
            WorkflowStep {
                name: "Calculate first sum".to_string(),
                action: StepAction::Type { text: "1234 + 5678".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Execute calculation".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            
            WorkflowStep {
                name: "Clear calculator".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["ctrl".to_string(), "a".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            
            WorkflowStep {
                name: "Calculate percentage".to_string(),
                action: StepAction::Type { text: "85 * 1.15".to_string() },
                timeout: Some(Duration::from_secs(5)),
            },
            WorkflowStep {
                name: "Execute percentage calculation".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["Return".to_string()] 
                },
                timeout: Some(Duration::from_secs(3)),
            },
            
            // Switch to text editor for data entry
            WorkflowStep {
                name: "Switch to text editor".to_string(),
                action: StepAction::KeySequence { 
                    keys: vec!["alt".to_string(), "Tab".to_string()] 
                },
                timeout: Some(Duration::from_secs(5)),
            },
            
            // Enter structured data
            WorkflowStep {
                name: "Enter data table header".to_string(),
                action: StepAction::Type { 
                    text: "\n\nDATA ENTRY RESULTS\n".to_string() +
                          "==================\n\n" +
                          "Transaction ID | Amount | Tax Rate | Total | Status\n" +
                          "---------------|--------|----------|-------|-------\n"
                },
                timeout: Some(Duration::from_secs(10)),
            },
            
            // Simulate data entry for multiple records
            WorkflowStep {
                name: "Enter transaction record 1".to_string(),
                action: StepAction::Type { 
                    text: "TXN-001        | $1,234 | 15%      | $1,419| COMPLETE\n".to_string() 
                },
                timeout: Some(Duration::from_secs(8)),
            },
            
            WorkflowStep {
                name: "Enter transaction record 2".to_string(),
                action: StepAction::Type { 
                    text: "TXN-002        | $5,678 | 15%      | $6,530| COMPLETE\n".to_string() 
                },
                timeout: Some(Duration::from_secs(8)),
            },
            
            WorkflowStep {
                name: "Enter transaction record 3".to_string(),
                action: StepAction::Type { 
                    text: "TXN-003        | $2,500 | 15%      | $2,875| PENDING\n".to_string() 
                },
                timeout: Some(Duration::from_secs(8)),
            },
            
            WorkflowStep {
                name: "Enter transaction record 4".to_string(),
                action: StepAction::Type { 
                    text: "TXN-004        | $4,200 | 15%      | $4,830| COMPLETE\n".to_string() 
                },
                timeout: Some(Duration::from_secs(8)),
            },
            
            WorkflowStep {
                name: "Add data summary".to_string(),
                action: StepAction::Type { 
                    text: "\nSUMMARY:\n".to_string() +
                          "- Total Records: 4\n" +
                          "- Completed: 3\n" +
                          "- Pending: 1\n" +
                          "- Gross Amount: $13,612\n" +
                          "- Total Tax: $2,041\n" +
                          "- Net Amount: $15,653\n\n" +
                          "Data entry completed with automated validation.\n"
                },
                timeout: Some(Duration::from_secs(15)),
            },
        ],
    };

    println!("🔄 Executing data entry workflow...");
    let result = api.execute_workflow(session_id, workflow).await?;
    print_workflow_results(&result);

    Ok(())
}

/// Helper function to print workflow results
fn print_workflow_results(result: &api_surface::WorkflowExecutionResult) {
    println!("  📊 Workflow Results:");
    println!("    - Name: {}", result.workflow_name);
    println!("    - Success: {}", result.success);
    println!("    - Steps: {}/{}", result.successful_steps, result.total_steps);
    println!("    - Execution Time: {} ms", result.execution_time_ms);
    
    if !result.success && !result.errors.is_empty() {
        println!("    - Errors:");
        for (i, error) in result.errors.iter().enumerate() {
            println!("      {}. {}", i + 1, error);
        }
    }
    
    let success_rate = (result.successful_steps as f64 / result.total_steps as f64) * 100.0;
    println!("    - Success Rate: {:.1}%", success_rate);
}

/// Demonstrate error recovery and resilience
async fn demonstrate_error_recovery(api: &KVirtualStageAPI, session_id: &str) -> Result<()> {
    println!("\n🛡️ === Error Recovery Demonstration ===");

    use automation_engine::{AutomationWorkflow, WorkflowStep, StepAction, Point, MouseButton};

    let workflow = AutomationWorkflow {
        name: "Error Recovery Test".to_string(),
        description: "Test workflow resilience and error handling".to_string(),
        continue_on_error: true, // Continue despite errors
        steps: vec![
            // Valid step
            WorkflowStep {
                name: "Valid cursor movement".to_string(),
                action: StepAction::MoveCursor { to: Point::new(100.0, 100.0) },
                timeout: Some(Duration::from_secs(5)),
            },
            
            // This might fail if coordinates are invalid
            WorkflowStep {
                name: "Potentially invalid coordinates".to_string(),
                action: StepAction::MoveCursor { to: Point::new(99999.0, 99999.0) },
                timeout: Some(Duration::from_secs(5)),
            },
            
            // Valid step that should continue
            WorkflowStep {
                name: "Recovery step".to_string(),
                action: StepAction::MoveCursor { to: Point::new(200.0, 200.0) },
                timeout: Some(Duration::from_secs(5)),
            },
            
            // Test timeout handling
            WorkflowStep {
                name: "Quick timeout test".to_string(),
                action: StepAction::Wait { duration: Duration::from_secs(1) },
                timeout: Some(Duration::from_millis(500)), // Intentionally short timeout
            },
            
            // Final recovery
            WorkflowStep {
                name: "Final recovery click".to_string(),
                action: StepAction::Click { 
                    position: Point::new(300.0, 300.0), 
                    button: MouseButton::Left 
                },
                timeout: Some(Duration::from_secs(5)),
            },
        ],
    };

    println!("🔄 Executing error recovery workflow...");
    let result = api.execute_workflow(session_id, workflow).await?;
    print_workflow_results(&result);

    Ok(())
}