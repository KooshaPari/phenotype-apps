# Structured Reasoning Framework for MCP Tool Selection

This comprehensive guide outlines the structured reasoning framework for selecting appropriate MCP tools using Claude Desktop.

## Core Decision Process

The structured reasoning framework follows a systematic approach to tool selection:

1. **Task Analysis**
2. **Capability Matching**
3. **Tool Selection**
4. **Parameter Preparation**
5. **Execution Planning**
6. **Error Handling Strategy**
7. **Output Processing Plan**

## 1. Task Analysis

Begin by thoroughly analyzing the task to understand its requirements and constraints:

### Core Questions
- What is the primary objective of the task?
- What specific capabilities are needed?
- Are there any time, resource, or other constraints?
- What are the expected inputs and outputs?

### Example Analysis
```
Task: Share project findings with the development team
Primary Objective: Team communication
Required Capabilities: Message sending, thread creation
Constraints: Must notify all team members
Inputs: Message content, recipient list
Outputs: Confirmation of message delivery, thread creation
```

## 2. Capability Matching

Match the task requirements to general capability categories:

### Core Capability Categories
- **Communication**: Sharing information between agents
- **Computation**: Processing data or performing calculations
- **External Integration**: Accessing external services or models
- **Infrastructure**: Setting up or configuring environments
- **Diagnostics**: Analyzing issues or troubleshooting problems

### Example Capability Matching
```
Task: Share project findings with the development team
Primary Capability: Communication
Secondary Capability: None
```

## 3. Tool Selection

Select specific tools based on the identified capabilities:

### Decision Criteria
- **Functionality Match**: Does the tool provide the required capabilities?
- **Parameter Compatibility**: Can you provide all required parameters?
- **Output Suitability**: Does the tool's output match what you need?
- **Reliability**: Is the tool likely to be available and stable?
- **Efficiency**: Is this the most direct way to accomplish the task?

### Tool Categories and Options

| Category | Primary Tools | Alternative Tools |
|----------|---------------|-------------------|
| Communication | team_message, create_thread_with_notification | send_team_message, broadcast_thread_creation |
| Computation | calculate_fibonacci | Manual calculation |
| External Generation | gemini_generate | Claude's native capabilities |
| Search | brave_search | Manual search instructions |
| Infrastructure | create_brave_search_mcp_server | create_mcp_brave_search |
| Diagnostics | diagnose_mcp_issues | Manual diagnostics |

### Example Tool Selection
```
Task: Share project findings with the development team
Selected Tool: create_thread_with_notification
Rationale: 
- Multiple recipients needed (team-wide communication)
- Need to create a discussion thread for responses
- Need to ensure all team members are notified
```

## 4. Parameter Preparation

Prepare all required parameters for the selected tool:

### Parameter Guidelines
- Identify all required parameters from tool documentation
- Validate parameter values against expected formats and ranges
- Consider default values for optional parameters
- Sanitize user-provided inputs for security

### Example Parameter Preparation
```
Tool: create_thread_with_notification
Parameters:
- title: "Project XYZ Findings: April 2025"
- purpose: "Discussing project findings and next steps"
- creator_id: "claude_desktop_agent"
- participants: ["dev_lead", "backend_dev", "frontend_dev", "qa_lead"]
- initial_message: "Here are the key findings from our project analysis..."
```

## 5. Execution Planning

Plan the execution of the tool and prepare for possible outcomes:

### Execution Considerations
- Timing: When should the tool be executed?
- Prerequisites: Are there any preconditions to check?
- Dependencies: Does this tool depend on other tools or resources?
- Sequence: Is this part of a larger sequence of tools?

### Example Execution Plan
```
Tool: create_thread_with_notification
Execution Plan:
1. Verify all team members are in the participants list
2. Execute the tool to create the thread
3. Verify thread creation success
4. Prepare for follow-up communication in the thread
```

## 6. Error Handling Strategy

Develop strategies for handling potential errors:

### Common Error Types
- Tool not found
- Missing required parameters
- Invalid parameter values
- Execution failures
- External service errors

### Error Handling Approaches
- Verification: Check parameters before execution
- Fallbacks: Identify alternative tools or approaches
- Retries: Determine if retrying might resolve the issue
- Graceful Degradation: Plan how to proceed with limited functionality

### Example Error Strategy
```
Tool: create_thread_with_notification
Error Strategy:
- If tool not found: Fall back to broadcast_thread_creation or individual messages
- If participant invalid: Remove invalid participant and proceed
- If thread creation fails: Try again with simplified parameters
- If all communication fails: Provide manual instructions for the user
```

## 7. Output Processing Plan

Plan how to use the tool's output:

### Output Considerations
- Validation: How will you verify the output is correct?
- Transformation: Does the output need processing or reformatting?
- Integration: How will the output be used in subsequent steps?
- Presentation: How should the output be presented to the user?

### Example Output Plan
```
Tool: create_thread_with_notification
Output Processing:
- Extract thread_id from successful response
- Provide confirmation to user with thread details
- Save thread_id for future reference in the conversation
- Prepare for continued discussion in the thread
```

## Decision Flow Diagram

```
START
  ↓
ANALYZE TASK
  ↓
IDENTIFY PRIMARY CAPABILITY
  ↓
MATCH CAPABILITY TO TOOL CATEGORY
  ↓
SELECT SPECIFIC TOOL
  ↓
PREPARE PARAMETERS
  ↓
DEVELOP ERROR STRATEGY
  ↓
PLAN OUTPUT PROCESSING
  ↓
EXECUTE TOOL
  ↓
HANDLE RESULT OR ERROR
  ↓
END
```

## Case Studies

### Case Study 1: Team Communication

**Task:** Share project status with development team

**Analysis:**
1. Primary objective: Team communication
2. Target: Multiple recipients
3. Purpose: Status update
4. Additional need: Discussion thread

**Tool Selection:**
- Category: Group Communication
- Primary tool: create_thread_with_notification
- Parameters needed: title, purpose, creator_id, participants, initial_message
- Fallback: broadcast_thread_creation if thread already exists

**Validation:**
- All required parameters are available
- Output format matches expectations
- Error handling plan in place

### Case Study 2: External Model Integration

**Task:** Generate creative solutions to a technical problem

**Analysis:**
1. Primary objective: Idea generation
2. Capability needed: External AI model
3. Purpose: Creative thinking
4. Output needs: Multiple solution approaches

**Tool Selection:**
- Category: External Model Integration
- Primary tool: gemini_generate
- Parameters needed: prompt, potentially apiKey
- Fallback: Use Claude's native capabilities

**Validation:**
- Prompt parameter is well-formulated
- Output format is appropriate for the task
- Error handling strategy accounts for API limitations

### Case Study 3: Diagnostic Analysis

**Task:** Troubleshoot MCP server issues

**Analysis:**
1. Primary objective: Problem diagnosis
2. Capability needed: Log analysis
3. Purpose: Identify root cause
4. Output needs: Problem identification and solutions

**Tool Selection:**
- Category: Diagnostics
- Primary tool: diagnose_mcp_issues
- Parameters needed: log_content
- Fallback: Manual log analysis by Claude

**Validation:**
- Log content is properly formatted
- Output provides actionable insights
- Error handling accounts for complex log formats
