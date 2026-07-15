# Structured Reasoning Framework for Tool Selection

This document provides a structured approach for selecting appropriate MCP tools based on task requirements.

## Decision Flow Process

1. **Analyze Task Requirements**
   - Identify the primary objective of the task
   - Determine required capabilities (communication, computation, research, etc.)
   - Note any constraints or special requirements

2. **Match Task to Tool Category**
   - Communication tasks → team_message, send_team_message, create_thread_with_notification
   - Computational tasks → calculate_fibonacci
   - External model integration → gemini_generate
   - Research tasks → brave_search
   - Infrastructure setup → create_brave_search_mcp_server, create_windows_mcp_config
   - Diagnostics → diagnose_mcp_issues

3. **Select Specific Tool**
   - Consider required parameters
   - Evaluate capability match
   - Check for potential errors or edge cases
   - Prepare fallback options

4. **Validate Selection**
   - Ensure all required parameters are available
   - Confirm tool handles expected output format
   - Prepare for potential error scenarios

## Decision Matrix

| Task Type | Primary Tool | Fallback Tool | Manual Alternative |
|-----------|--------------|---------------|-------------------|
| 1:1 Communication | team_message | send_team_message | Direct message template |
| Group Communication | create_thread_with_notification | broadcast_thread_creation | Thread creation instructions |
| External Generation | gemini_generate | N/A | Claude native capabilities |
| Web Search | brave_search | N/A | Manual search instructions |
| Infrastructure Setup | create_brave_search_mcp_server | create_mcp_brave_search | Manual setup guide |
| Diagnostics | diagnose_mcp_issues | N/A | Manual diagnostics guide |
| Computation | calculate_fibonacci | N/A | Manual algorithm implementation |

## Example Decision Process

**Task**: Share project status with development team

**Analysis**:
1. Primary objective: Team communication
2. Target: Multiple recipients
3. Purpose: Status update
4. Additional need: Discussion thread

**Tool Selection**:
- Category: Group Communication
- Primary tool: create_thread_with_notification
- Parameters needed: title, purpose, creator_id, participants, initial_message
- Fallback: broadcast_thread_creation if thread already exists

**Validation**:
- All required parameters are available
- Output format matches expectations
- Error handling plan in place