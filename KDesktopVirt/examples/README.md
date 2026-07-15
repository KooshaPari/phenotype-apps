# KVirtualStage Examples

This directory contains comprehensive examples demonstrating the capabilities of the KVirtualStage automation platform. Each example showcases different aspects of the system, from basic automation to advanced enterprise workflows.

## 📁 Available Examples

### 1. Basic Automation (`basic_automation.rs`)
**Perfect for getting started with KVirtualStage**

Demonstrates fundamental automation capabilities:
- ✅ Session creation and management
- ✅ Natural cursor movement with WindMouse 2.0
- ✅ Human-like clicking simulation
- ✅ Natural typing with realistic timing
- ✅ Session recording and playback
- ✅ Secure credential storage
- ✅ Error handling and cleanup

**Run with:**
```bash
cargo run --example basic_automation
```

### 2. Advanced Workflow (`advanced_workflow.rs`)
**For complex automation scenarios**

Showcases enterprise-grade automation workflows:
- 🌐 **Web Application Testing**: Form automation, navigation, validation
- 📄 **Document Processing**: Content creation, editing, formatting
- 🔧 **System Administration**: Terminal operations, script execution
- 📊 **Data Entry**: Structured data input, calculations, validation
- 🛡️ **Error Recovery**: Resilient workflows with continuation logic

**Run with:**
```bash
cargo run --example advanced_workflow
```

## 🚀 Running Examples

### Prerequisites
1. **Rust Environment**: Ensure you have Rust 1.70+ installed
2. **Dependencies**: All required dependencies are included in `Cargo.toml`
3. **Desktop Environment**: Examples work best with Linux desktop environments (Ubuntu, GNOME, KDE)

### Quick Start
```bash
# Clone and navigate to the project
cd kvirtualstage

# Run basic automation example
cargo run --example basic_automation

# Run advanced workflow example
cargo run --example advanced_workflow

# Run with debug logging
RUST_LOG=debug cargo run --example basic_automation
```

### Build Examples Only
```bash
# Build all examples
cargo build --examples

# Build specific example
cargo build --example basic_automation
```

## 📊 Example Output

### Basic Automation Example
```
🚀 KVirtualStage Basic Automation Example
=========================================
📡 Initializing KVirtualStage API...
✅ API initialized successfully!

📦 Creating new session...
✅ Session created: basic_automation_demo

🎥 Starting session recording...
✅ Recording started: rec_12345

🖱️  Demonstrating natural cursor movement...
  📍 Moving to Top-left area: (100, 100)
  🖱️  Left click at current position
  📍 Moving to Center-right: (500, 200)
  🖱️  Right click at current position
  ...

⌨️  Demonstrating natural typing...
  📝 Typing example 1: "Hello, World!"
  📝 Typing example 2: "This is a demonstration..."
  ...

🔄 Executing a complete automation workflow...
  🎯 Executing workflow: 'Basic Demo Workflow'
  📊 Workflow execution results:
    - Total steps: 7
    - Successful steps: 7
    - Execution time: 15847 ms
    - Success: true

🛑 Stopping session recording...
✅ Recording saved to: /path/to/basic_automation_demo.mp4

🎉 Basic automation demo completed successfully!
```

### Advanced Workflow Example
```
🚀 KVirtualStage Advanced Workflow Example
==========================================
✅ API initialized
✅ Session created: advanced_workflow_demo
✅ Recording started: rec_67890

🌐 === Web Application Testing Workflow ===
🔄 Executing web testing workflow...
  📊 Workflow Results:
    - Name: Web Application Testing
    - Success: true
    - Steps: 22/22
    - Execution Time: 45623 ms
    - Success Rate: 100.0%

📄 === Document Processing Workflow ===
🔄 Executing document processing workflow...
  📊 Workflow Results:
    - Name: Document Processing
    - Success: true
    - Steps: 18/18
    - Execution Time: 32156 ms
    - Success Rate: 100.0%

🔧 === System Administration Workflow ===
🔄 Executing system administration workflow...
  📊 Workflow Results:
    - Name: System Administration
    - Success: true
    - Steps: 24/24
    - Execution Time: 28934 ms
    - Success Rate: 100.0%

📊 === Data Entry Workflow ===
🔄 Executing data entry workflow...
  📊 Workflow Results:
    - Name: Data Entry Automation
    - Success: true
    - Steps: 16/16
    - Execution Time: 22478 ms
    - Success Rate: 100.0%

🎬 Recording completed: /path/to/advanced_workflow_demo.mp4
🧹 Cleanup completed

🎉 Advanced workflow demonstration completed!
```

## 🔧 Customization

### Environment Variables
Configure example behavior with environment variables:

```bash
# Enable debug logging
export RUST_LOG=debug

# Set custom recording quality
export RECORDING_QUALITY=high

# Set custom desktop type
export DESKTOP_TYPE=ubuntu-gnome

# Disable cleanup for debugging
export NO_CLEANUP=1
```

### Modifying Examples
Examples are designed to be educational and easily modifiable:

1. **Change Desktop Environment**: Modify the desktop type in session creation
2. **Adjust Timing**: Update delays and timeouts for your system
3. **Add Custom Steps**: Insert additional workflow steps
4. **Modify Coordinates**: Adjust click positions for your screen resolution

### Creating Custom Examples
Use the existing examples as templates:

```rust
use anyhow::Result;
use kvirtualstage::*;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize API
    let api = KVirtualStageAPI::new().await?;
    
    // Create session
    let session_id = api.create_session(
        "your_user".to_string(),
        "your_session".to_string(),
        "ubuntu-xfce".to_string(),
    ).await?;
    
    // Your automation logic here...
    
    // Cleanup
    api.cleanup_expired_sessions(0).await?;
    Ok(())
}
```

## 📝 Example Features Demonstrated

### Core Automation
- [x] **Session Management**: Create, monitor, and cleanup sessions
- [x] **Cursor Movement**: Natural movement with WindMouse 2.0 algorithm
- [x] **Clicking**: Left, right, and middle clicks with natural timing
- [x] **Typing**: Human-like typing with character-level timing variation
- [x] **Recording**: High-quality video recording with multiple formats

### Advanced Features
- [x] **Workflow Execution**: Complex multi-step automation sequences
- [x] **Error Handling**: Graceful error recovery and continuation
- [x] **Performance Monitoring**: Timing and throughput measurements
- [x] **Security**: Encrypted credential storage and session isolation
- [x] **Integration**: Cross-application automation and data transfer

### Real-World Scenarios
- [x] **Web Testing**: Form filling, navigation, validation
- [x] **Document Processing**: Content creation, editing, formatting
- [x] **System Administration**: Command execution, script automation
- [x] **Data Entry**: Structured input, calculations, validation
- [x] **Quality Assurance**: Automated testing and verification

## 🛠️ Troubleshooting

### Common Issues

**1. Permission Errors**
```bash
# Ensure proper permissions for desktop automation
# May require additional setup on some systems
```

**2. Display Issues**
```bash
# Set display for headless environments
export DISPLAY=:0
```

**3. Recording Failures**
```bash
# Check FFmpeg installation
ffmpeg -version

# Install if missing (Ubuntu/Debian)
sudo apt update && sudo apt install ffmpeg
```

**4. Coordinate Issues**
```bash
# Adjust coordinates for your screen resolution
# Examples assume 1920x1080 display
```

### Performance Optimization

**For Better Performance:**
- Use release builds: `cargo run --release --example basic_automation`
- Adjust timing delays based on your system performance
- Monitor resource usage during automation

**For Debugging:**
- Enable debug logging: `RUST_LOG=debug`
- Reduce automation speed with longer delays
- Use smaller test workflows first

## 📚 Learning Path

### Beginner
1. Start with `basic_automation.rs`
2. Understand session lifecycle
3. Practice cursor movement and clicking
4. Learn typing automation basics

### Intermediate
1. Study workflow structure in `advanced_workflow.rs`
2. Create custom workflows
3. Implement error handling
4. Explore recording capabilities

### Advanced
1. Build enterprise automation solutions
2. Integrate with external systems
3. Optimize performance and reliability
4. Implement custom security patterns

## 🤝 Contributing

We welcome contributions to improve and expand the examples:

1. **Bug Fixes**: Report or fix issues in existing examples
2. **New Examples**: Add examples for specific use cases
3. **Documentation**: Improve comments and documentation
4. **Performance**: Optimize example performance and reliability

### Contribution Guidelines
- Follow Rust coding conventions
- Include comprehensive comments
- Add error handling and cleanup
- Test on multiple environments
- Update documentation

## 📄 License

Examples are provided under the same license as the main KVirtualStage project. See the main project LICENSE file for details.

## 🔗 Resources

- **Main Documentation**: `../README.md`
- **API Reference**: `../docs/api.md`
- **Architecture Guide**: `../docs/architecture.md`
- **Performance Guide**: `../docs/performance.md`
- **Security Guide**: `../docs/security.md`

---

*Ready to automate? Start with the basic example and build your way up to enterprise-grade automation workflows!*