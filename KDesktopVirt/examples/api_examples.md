# KVirtualStage API Examples

This document provides comprehensive examples of using KVirtualStage through different interfaces.

## Table of Contents

1. [REST API Examples](#rest-api-examples)
2. [Python Bindings Examples](#python-bindings-examples)
3. [Node.js Bindings Examples](#nodejs-bindings-examples)
4. [C/C++ Examples](#cc-examples)
5. [MCP Server Examples](#mcp-server-examples)
6. [CLI Examples](#cli-examples)
7. [TUI Usage](#tui-usage)

## REST API Examples

### Creating a Session

```bash
# Create a new Ubuntu session
curl -X POST http://localhost:8080/api/v1/sessions \
  -H "Content-Type: application/json" \
  -d '{
    "user_id": "demo_user",
    "session_name": "my_automation_session",
    "desktop_type": "ubuntu"
  }'

# Response:
# {
#   "success": true,
#   "data": {
#     "session_id": "session_20241212_143022",
#     "status": "active",
#     "vnc_url": "ws://localhost:8080/api/v1/sessions/session_20241212_143022/stream"
#   },
#   "error": null,
#   "timestamp": "2024-12-12T14:30:22Z"
# }
```

### Basic Automation

```bash
SESSION_ID="session_20241212_143022"

# Move cursor to center of screen
curl -X POST "http://localhost:8080/api/v1/sessions/$SESSION_ID/cursor/move" \
  -H "Content-Type: application/json" \
  -d '{"target_x": 400, "target_y": 300}'

# Click at current position
curl -X POST "http://localhost:8080/api/v1/sessions/$SESSION_ID/mouse/click" \
  -H "Content-Type: application/json" \
  -d '{"button": "left"}'

# Type some text
curl -X POST "http://localhost:8080/api/v1/sessions/$SESSION_ID/keyboard/type" \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello from KVirtualStage!"}'
```

### Recording a Session

```bash
# Start recording
curl -X POST "http://localhost:8080/api/v1/sessions/$SESSION_ID/recording/start" \
  -H "Content-Type: application/json" \
  -d '{
    "output_filename": "demo_recording.mp4",
    "quality": "high"
  }'

# Perform some automation...
# (cursor movements, clicks, typing)

# Stop recording
curl -X POST "http://localhost:8080/api/v1/sessions/$SESSION_ID/recording/stop" \
  -H "Content-Type: application/json"
```

### Complex Workflow

```bash
curl -X POST "http://localhost:8080/api/v1/sessions/$SESSION_ID/workflow" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Calculator Demo",
    "description": "Demonstrate calculator usage",
    "continue_on_error": false,
    "steps": [
      {
        "name": "Move to calculator position",
        "action_type": "move_cursor",
        "parameters": {"x": 100, "y": 100},
        "timeout_seconds": 5
      },
      {
        "name": "Click calculator",
        "action_type": "click",
        "parameters": {"button": "left"},
        "timeout_seconds": 5
      },
      {
        "name": "Type calculation",
        "action_type": "type",
        "parameters": {"text": "2 + 2 ="},
        "timeout_seconds": 10
      }
    ]
  }'
```

## Python Bindings Examples

### Async API Usage

```python
import asyncio
import kvirtualstage as kvs

async def main():
    # Create automation instance
    automation = kvs.KVirtualStage()
    
    # Create a new desktop session
    session = await automation.create_session(
        user_id="demo_user",
        session_name="python_session",
        desktop_type=kvs.DesktopType.UBUNTU
    )
    
    print(f"Session created: {session.session_id}")
    
    # Start recording
    recording = await session.start_recording("python_demo.mp4", kvs.RecordingQuality.HIGH)
    
    # Perform natural automation
    await session.move_cursor(400, 300)
    await session.click()
    await session.type_text("Hello from Python!")
    
    # Move and right-click
    await session.right_click(500, 400)
    
    # Create and execute workflow
    workflow = kvs.Workflow("Text Editor Demo")
    workflow.move_cursor(200, 150)
    workflow.click()
    workflow.type_text("This is a workflow demonstration.")
    workflow.key_press("Enter")
    workflow.type_text("Multiple lines of text!")
    
    result = await session.execute_workflow(workflow)
    print(f"Workflow completed: {result.success}")
    
    # Stop recording
    output_path = await recording.stop()
    print(f"Recording saved: {output_path}")
    
    # Get session info
    info = await session.get_info()
    print(f"Session info: {info}")
    
    # Cleanup
    await session.close()

# Run the async function
asyncio.run(main())
```

### Synchronous API Usage

```python
import kvirtualstage as kvs

# Create synchronous automation instance
automation = kvs.SyncKVirtualStage()

# Create session
session = automation.create_session(
    user_id="sync_user",
    desktop_type=kvs.DesktopType.UBUNTU_XFCE
)

# Simple automation
session.move_cursor(300, 200)
session.click()
session.type_text("Synchronous automation example")

# Take screenshot
screenshot_path = session.screenshot("sync_demo.png")
print(f"Screenshot saved: {screenshot_path}")

# Execute workflow
workflow = kvs.Workflow("Calculator Demo")
workflow.add_step("move_cursor", x=100, y=100)
workflow.add_step("click")
workflow.add_step("type", text="5 * 5 =")

result = session.execute_workflow(workflow)
print(f"Workflow result: {result}")

session.close()
```

### Event Handling

```python
import asyncio
import kvirtualstage as kvs

async def automation_with_events():
    automation = kvs.KVirtualStage()
    session = await automation.create_session("event_user", "event_session")
    
    # Event handlers
    def on_cursor_moved(event):
        print(f"Cursor moved to: ({event['x']}, {event['y']})")
    
    def on_clicked(event):
        print(f"Clicked with {event['button']} button")
    
    def on_text_typed(event):
        print(f"Typed: {event['text']}")
    
    # Register event handlers
    session.on('cursorMoved', on_cursor_moved)
    session.on('clicked', on_clicked)
    session.on('textTyped', on_text_typed)
    
    # Perform automation with events
    await session.move_cursor(100, 100)  # Triggers cursorMoved event
    await session.click()                 # Triggers clicked event
    await session.type_text("Hello!")    # Triggers textTyped event
    
    await session.close()

asyncio.run(automation_with_events())
```

## Node.js Bindings Examples

### Basic Usage

```javascript
const kvs = require('kvirtualstage');

async function main() {
    // Create automation instance
    const automation = new kvs.KVirtualStage();
    
    // Create session
    const session = await automation.createSession({
        userId: 'nodejs_user',
        sessionName: 'nodejs_demo',
        desktopType: kvs.DesktopType.UBUNTU
    });
    
    console.log(`Session created: ${session.sessionId}`);
    
    // Start recording
    const recording = await session.startRecording('nodejs_demo.mp4', kvs.RecordingQuality.MEDIUM);
    
    // Automation sequence
    await session.moveCursor(400, 300);
    await session.click();
    await session.typeText('Hello from Node.js!');
    
    // Double-click somewhere else
    await session.doubleClick(200, 150);
    
    // Key combinations
    await session.keyCombination('Ctrl', 'A');  // Select all
    await session.typeText('Replaced text');
    
    // Stop recording
    const outputPath = await recording.stop();
    console.log(`Recording saved: ${outputPath}`);
    
    await session.close();
}

main().catch(console.error);
```

### Workflow Automation

```javascript
const kvs = require('kvirtualstage');

async function workflowDemo() {
    const automation = new kvs.KVirtualStage();
    const session = await automation.createSession({
        userId: 'workflow_user',
        desktopType: kvs.DesktopType.UBUNTU_KDE
    });
    
    // Create complex workflow
    const workflow = new kvs.Workflow('File Management Demo', 'Demonstrate file operations');
    
    workflow
        .moveCursor(50, 50)           // Move to file manager
        .click()                      // Open file manager
        .wait(2)                      // Wait for it to load
        .keyCombination('Ctrl', 'N')  // New folder
        .typeText('Demo Folder')      // Name the folder
        .keyPress('Enter')            // Confirm
        .doubleClick(100, 150)        // Enter the folder
        .typeText('Hello World.txt')  // Create a file
        .keyPress('Enter');
    
    // Execute workflow
    const result = await session.executeWorkflow(workflow);
    
    if (result.success) {
        console.log(`Workflow completed successfully in ${result.executionTimeMs}ms`);
    } else {
        console.log(`Workflow failed: ${result.errors.join(', ')}`);
    }
    
    await session.close();
}

workflowDemo().catch(console.error);
```

### Event-Driven Programming

```javascript
const kvs = require('kvirtualstage');

async function eventDemo() {
    const automation = new kvs.KVirtualStage();
    const session = await automation.createSession({userId: 'event_user'});
    
    // Event listeners
    session.on('cursorMoved', (data) => {
        console.log(`Cursor moved to (${data.x}, ${data.y})`);
    });
    
    session.on('clicked', (data) => {
        console.log(`${data.button} click at (${data.x}, ${data.y})`);
    });
    
    session.on('textTyped', (data) => {
        console.log(`Typed: "${data.text}"`);
    });
    
    session.on('workflowCompleted', (result) => {
        console.log(`Workflow "${result.workflowName}" completed: ${result.success}`);
    });
    
    // Live streaming
    const ws = session.connectStream();
    ws.on('streamConnected', () => {
        console.log('Live stream connected');
    });
    
    ws.on('streamMessage', (message) => {
        console.log('Stream message:', message);
    });
    
    // Perform automation
    await session.moveCursor(200, 200);
    await session.click(300, 300);
    await session.typeText('Event-driven automation');
    
    await session.close();
}

eventDemo().catch(console.error);
```

## C/C++ Examples

### Basic C Example

```c
#include "kvirtualstage.h"
#include <stdio.h>
#include <stdlib.h>

int main() {
    // Initialize KVirtualStage
    if (kvs_init() != KVS_SUCCESS) {
        fprintf(stderr, "Failed to initialize KVirtualStage\n");
        return 1;
    }
    
    printf("KVirtualStage initialized successfully\n");
    
    // Create session
    char session_id[256];
    if (kvs_create_session("c_user", "c_demo_session", "ubuntu", 
                          session_id, sizeof(session_id)) != KVS_SUCCESS) {
        fprintf(stderr, "Failed to create session\n");
        kvs_shutdown();
        return 1;
    }
    
    printf("Session created: %s\n", session_id);
    
    // Start recording
    char recording_id[256];
    if (kvs_start_recording(session_id, "c_demo.mp4", "medium", 
                           recording_id, sizeof(recording_id)) != KVS_SUCCESS) {
        fprintf(stderr, "Failed to start recording\n");
    } else {
        printf("Recording started: %s\n", recording_id);
    }
    
    // Perform automation
    kvs_move_cursor(session_id, 400.0, 300.0);
    kvs_click(session_id, "left");
    kvs_type_text(session_id, "Hello from C!");
    
    // Move and right-click
    kvs_move_cursor(session_id, 200.0, 150.0);
    kvs_click(session_id, "right");
    
    // Stop recording
    char output_path[512];
    if (kvs_stop_recording(session_id, output_path, sizeof(output_path)) == KVS_SUCCESS) {
        printf("Recording saved: %s\n", output_path);
    }
    
    // Get session info
    kvs_session_info_t session_info;
    if (kvs_get_session_info(session_id, &session_info) == KVS_SUCCESS) {
        printf("Session Info:\n");
        printf("  ID: %s\n", session_info.session_id);
        printf("  User: %s\n", session_info.user_id);
        printf("  Desktop: %s\n", session_info.desktop_type);
        printf("  Status: %s\n", session_info.status);
        printf("  Recording: %s\n", session_info.recording_active ? "Active" : "Inactive");
    }
    
    // Cleanup
    kvs_remove_session(session_id);
    kvs_shutdown();
    
    printf("Demo completed successfully\n");
    return 0;
}
```

### C++ Example with RAII

```cpp
#include "kvirtualstage.h"
#include <iostream>
#include <string>
#include <vector>
#include <memory>

class KVSSession {
private:
    std::string session_id_;
    bool valid_;

public:
    KVSSession(const std::string& user_id, const std::string& name, const std::string& desktop_type) 
        : valid_(false) {
        char buffer[256];
        if (kvs_create_session(user_id.c_str(), name.c_str(), desktop_type.c_str(), 
                              buffer, sizeof(buffer)) == KVS_SUCCESS) {
            session_id_ = buffer;
            valid_ = true;
        }
    }
    
    ~KVSSession() {
        if (valid_) {
            kvs_remove_session(session_id_.c_str());
        }
    }
    
    bool isValid() const { return valid_; }
    const std::string& getId() const { return session_id_; }
    
    bool moveCursor(double x, double y) {
        return kvs_move_cursor(session_id_.c_str(), x, y) == KVS_SUCCESS;
    }
    
    bool click(const std::string& button = "left") {
        return kvs_click(session_id_.c_str(), button.c_str()) == KVS_SUCCESS;
    }
    
    bool typeText(const std::string& text) {
        return kvs_type_text(session_id_.c_str(), text.c_str()) == KVS_SUCCESS;
    }
    
    std::string startRecording(const std::string& filename, const std::string& quality = "medium") {
        char buffer[256];
        if (kvs_start_recording(session_id_.c_str(), filename.c_str(), quality.c_str(),
                               buffer, sizeof(buffer)) == KVS_SUCCESS) {
            return std::string(buffer);
        }
        return "";
    }
    
    std::string stopRecording() {
        char buffer[512];
        if (kvs_stop_recording(session_id_.c_str(), buffer, sizeof(buffer)) == KVS_SUCCESS) {
            return std::string(buffer);
        }
        return "";
    }
};

class KVSManager {
private:
    bool initialized_;

public:
    KVSManager() : initialized_(false) {
        if (kvs_init() == KVS_SUCCESS) {
            initialized_ = true;
        }
    }
    
    ~KVSManager() {
        if (initialized_) {
            kvs_shutdown();
        }
    }
    
    bool isInitialized() const { return initialized_; }
    
    std::unique_ptr<KVSSession> createSession(const std::string& user_id, 
                                            const std::string& name,
                                            const std::string& desktop_type = "ubuntu") {
        if (!initialized_) return nullptr;
        
        auto session = std::make_unique<KVSSession>(user_id, name, desktop_type);
        return session->isValid() ? std::move(session) : nullptr;
    }
};

int main() {
    KVSManager manager;
    
    if (!manager.isInitialized()) {
        std::cerr << "Failed to initialize KVirtualStage" << std::endl;
        return 1;
    }
    
    std::cout << "KVirtualStage initialized successfully" << std::endl;
    
    // Create session
    auto session = manager.createSession("cpp_user", "cpp_demo", "ubuntu");
    if (!session) {
        std::cerr << "Failed to create session" << std::endl;
        return 1;
    }
    
    std::cout << "Session created: " << session->getId() << std::endl;
    
    // Start recording
    std::string recording_id = session->startRecording("cpp_demo.mp4", "high");
    if (!recording_id.empty()) {
        std::cout << "Recording started: " << recording_id << std::endl;
    }
    
    // Automation sequence
    session->moveCursor(400, 300);
    session->click();
    session->typeText("Hello from C++!");
    
    // Complex interaction
    session->moveCursor(100, 100);
    session->click("left");
    session->typeText("C++ automation with RAII");
    
    // Stop recording
    std::string output_path = session->stopRecording();
    if (!output_path.empty()) {
        std::cout << "Recording saved: " << output_path << std::endl;
    }
    
    std::cout << "C++ demo completed successfully" << std::endl;
    return 0;
}
```

## MCP Server Examples

### Claude/ChatGPT Integration

```
Human: I need to automate a desktop task. Can you help me create a session and demonstrate calculator usage?