#!/usr/bin/env python3
"""
KVirtualStage MCP Tools Demonstration Script

This script demonstrates how to interact with KVirtualStage's MCP server
to perform desktop automation tasks programmatically.
"""

import requests
import json
import time

# MCP Server Configuration
MCP_SERVER_URL = "http://localhost:3001"

def call_mcp_tool(tool_name, parameters=None):
    """Call an MCP tool via HTTP API"""
    if parameters is None:
        parameters = {}
    
    payload = {
        "method": f"tools/{tool_name}",
        "params": parameters,
        "jsonrpc": "2.0",
        "id": int(time.time())
    }
    
    try:
        response = requests.post(f"{MCP_SERVER_URL}/mcp", json=payload)
        return response.json()
    except Exception as e:
        return {"error": str(e)}

def demo_individual_tools():
    """Demonstrate individual MCP tool calls"""
    
    print("🎯 KVirtualStage MCP Tools Demonstration")
    print("=" * 50)
    
    # 1. List available tools
    print("\n1. Getting list of available tools...")
    tools_result = call_mcp_tool("list_tools")
    print(f"Available tools: {json.dumps(tools_result, indent=2)}")
    
    # 2. Take a screenshot
    print("\n2. Taking a screenshot...")
    screenshot_result = call_mcp_tool("take_screenshot", {
        "output": "/tmp/mcp_demo_screenshot.png"
    })
    print(f"Screenshot result: {json.dumps(screenshot_result, indent=2)}")
    
    # 3. Get system sessions
    print("\n3. Getting active sessions...")
    sessions_result = call_mcp_tool("get_sessions")
    print(f"Sessions: {json.dumps(sessions_result, indent=2)}")
    
    # 4. Run automation script
    print("\n4. Running automation to open Calculator...")
    automation_result = call_mcp_tool("run_automation", {
        "script": "open -a Calculator"
    })
    print(f"Automation result: {json.dumps(automation_result, indent=2)}")
    
    time.sleep(2)  # Wait for app to open
    
    # 5. Take another screenshot with Calculator open
    print("\n5. Taking screenshot with Calculator open...")
    calc_screenshot_result = call_mcp_tool("take_screenshot", {
        "output": "/tmp/mcp_calculator_demo.png"
    })
    print(f"Calculator screenshot: {json.dumps(calc_screenshot_result, indent=2)}")
    
    # 6. Find UI elements (simulated)
    print("\n6. Finding UI elements...")
    find_result = call_mcp_tool("find_element", {
        "selector": "button",
        "text": "5"
    })
    print(f"Find element result: {json.dumps(find_result, indent=2)}")
    
    # 7. Type text (simulated for demonstration)
    print("\n7. Typing text...")
    type_result = call_mcp_tool("type_text", {
        "text": "Hello from MCP!"
    })
    print(f"Type text result: {json.dumps(type_result, indent=2)}")
    
    # 8. Click element (simulated coordinates)
    print("\n8. Clicking UI element...")
    click_result = call_mcp_tool("click_element", {
        "x": 150,
        "y": 200
    })
    print(f"Click result: {json.dumps(click_result, indent=2)}")
    
    # 9. Text-to-speech demonstration
    print("\n9. Text-to-speech...")
    tts_result = call_mcp_tool("text_to_speech", {
        "text": "KVirtualStage MCP demonstration complete!"
    })
    print(f"TTS result: {json.dumps(tts_result, indent=2)}")
    
    # 10. Get credentials (example)
    print("\n10. Getting stored credentials...")
    creds_result = call_mcp_tool("get_credentials", {
        "service": "demo_service"
    })
    print(f"Credentials result: {json.dumps(creds_result, indent=2)}")
    
    print("\n✅ MCP Tools Demonstration Complete!")
    print("=" * 50)

def demo_batch_automation():
    """Demonstrate batch automation workflow"""
    
    print("\n🚀 Batch Automation Workflow Demo")
    print("=" * 40)
    
    workflow_steps = [
        {
            "name": "Initial Screenshot",
            "tool": "take_screenshot",
            "params": {"output": "/tmp/batch_demo_start.png"}
        },
        {
            "name": "Open TextEdit",
            "tool": "run_automation", 
            "params": {"script": "open -a TextEdit"}
        },
        {
            "name": "Wait for TextEdit",
            "delay": 2
        },
        {
            "name": "Type Demo Content",
            "tool": "type_text",
            "params": {"text": "Automated document creation via KVirtualStage MCP\\n\\nThis text was typed programmatically!"}
        },
        {
            "name": "Screenshot with Content",
            "tool": "take_screenshot",
            "params": {"output": "/tmp/batch_demo_textedit.png"}
        },
        {
            "name": "Open Calculator",
            "tool": "run_automation",
            "params": {"script": "open -a Calculator"}
        },
        {
            "name": "Wait for Calculator",
            "delay": 2
        },
        {
            "name": "Final Screenshot",
            "tool": "take_screenshot", 
            "params": {"output": "/tmp/batch_demo_final.png"}
        }
    ]
    
    for i, step in enumerate(workflow_steps, 1):
        print(f"\nStep {i}: {step['name']}")
        
        if 'delay' in step:
            print(f"  Waiting {step['delay']} seconds...")
            time.sleep(step['delay'])
        else:
            result = call_mcp_tool(step['tool'], step['params'])
            print(f"  Result: {json.dumps(result, indent=4)}")
    
    print("\n✅ Batch Automation Workflow Complete!")
    print("=" * 40)

if __name__ == "__main__":
    print("🎮 Starting KVirtualStage MCP Demonstration")
    print("🔗 Connecting to MCP server at", MCP_SERVER_URL)
    
    # Test connection
    try:
        response = requests.get(f"{MCP_SERVER_URL}/health")
        if response.status_code == 200:
            print("✅ MCP Server connection successful!")
        else:
            print("❌ MCP Server connection failed")
            exit(1)
    except:
        print("❌ Cannot connect to MCP server. Is it running?")
        exit(1)
    
    # Run demonstrations
    demo_individual_tools()
    time.sleep(1)
    demo_batch_automation()
    
    print("\n🎉 All demonstrations complete!")
    print("Check /tmp/ directory for generated screenshots")