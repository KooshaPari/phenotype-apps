# Specialized Prompts for Code Review and Development

This document provides specialized prompts for Claude Desktop to perform code review and development tasks using MCP tools. These prompts follow a structured approach that leverages Claude's reasoning capabilities and the specialized XML-based MCP tool integration.

## MCP Code Review Prompt

```
You're tasked with reviewing code for the MCP platform. Follow this structured approach to provide a comprehensive review:

1. First, examine the code for potential issues in these categories:
   - Functionality: Does the code accomplish its intended purpose?
   - Security: Are there any security vulnerabilities?
   - Performance: Are there any inefficiencies?
   - Reliability: How does the code handle errors?
   - Maintainability: Is the code clear and well-structured?

2. Use the appropriate MCP tools to analyze the code:

<function_calls>
<invoke name="diagnose_mcp_issues">
<parameter name="log_content">[CODE_TO_REVIEW]</parameter>
</invoke>
</function_calls>

3. If diagnostic tools are unavailable, perform manual analysis of:
   - Function implementations against their descriptions
   - Parameter validation and error handling
   - Potential edge cases
   - Code style and documentation
   - Security practices

4. Structure your review as follows:
   - Summary of strengths
   - Critical issues that must be addressed
   - Suggestions for improvement
   - Code examples for recommended changes

5. Ensure your feedback is:
   - Specific and actionable
   - Prioritized by importance
   - Constructive and educational
   - Referenced to best practices
```

## MCP Code Development Prompt

```
You're tasked with developing code for the MCP platform using Claude's XML tool integration. Follow this structured approach:

1. First, clarify the requirements:
   - What is the purpose of this MCP tool?
   - What are the expected inputs and outputs?
   - What are the error cases to handle?
   - Are there any performance or security constraints?

2. Design your solution:
   - Define the tool's interface (parameters and return values)
   - Plan the implementation approach
   - Identify potential edge cases
   - Consider error handling strategy

3. Implement the solution using best practices:
   - Clear function and variable names
   - Comprehensive input validation
   - Robust error handling
   - Thorough documentation
   - Efficient algorithm design

4. Test your implementation by simulating tool invocation:

<function_calls>
<invoke name="[YOUR_TOOL_NAME]">
<parameter name="param1">test_value1</parameter>
<parameter name="param2">test_value2</parameter>
</invoke>
</function_calls>

5. Even though you can't actually run the tool, think through:
   - How would the function handle these test inputs?
   - What edge cases might arise?
   - How would errors be communicated?

6. Revise your implementation based on your analysis:
   - Add additional validation if needed
   - Improve error messages
   - Optimize the solution
   - Enhance documentation

7. Present your final implementation with:
   - Complete code
   - Documentation explaining the design
   - Usage examples
   - Edge case handling explanations
```

## MCP Debugging Prompt

```
You're tasked with debugging an issue in the MCP platform. Follow this structured approach:

1. Gather information about the problem:
   - What is the expected behavior?
   - What is the actual behavior?
   - What error messages are being displayed?
   - When did the issue start occurring?

2. Use diagnostic tools to identify the root cause:

<function_calls>
<invoke name="diagnose_mcp_issues">
<parameter name="log_content">[ERROR_LOGS]</parameter>
</invoke>
</function_calls>

3. If diagnostic tools are unavailable, perform manual analysis:
   - Examine the error messages for clues
   - Look for patterns in when the issue occurs
   - Consider recent changes that might have introduced the bug
   - Check for common issues like missing dependencies or configuration errors

4. Develop a hypothesis about the root cause:
   - What component is likely failing?
   - What conditions trigger the issue?
   - What might be causing the unexpected behavior?

5. Test your hypothesis with targeted checks:
   - Look for specific error patterns
   - Examine configuration settings
   - Check for mismatched versions
   - Verify environment variables

6. Develop a solution:
   - Explain the root cause
   - Provide step-by-step fix instructions
   - Include code or configuration changes needed
   - Explain how to verify the fix worked

7. Suggest prevention measures:
   - How to avoid similar issues
   - Monitoring recommendations
   - Testing strategies
   - Documentation improvements
```

## Advanced Sequential Tool Usage Prompt

```
You're tasked with implementing a workflow that requires multiple MCP tools in sequence. Follow this structured approach:

1. Analyze the overall workflow requirements:
   - What is the end goal?
   - What intermediate steps are needed?
   - What inputs and outputs are required at each step?
   - What dependencies exist between steps?

2. Design the tool sequence:
   - Map each step to an appropriate MCP tool
   - Identify data flow between tools
   - Plan error handling at each transition
   - Consider fallback options if tools are unavailable

3. Implement the workflow step by step:

   Step 1: [First Tool]
   <function_calls>
   <invoke name="[TOOL_1]">
   <parameter name="param1">value1</parameter>
   </invoke>
   </function_calls>

   Step 2: [Second Tool, using results from first]
   <function_calls>
   <invoke name="[TOOL_2]">
   <parameter name="param1">[RESULT_FROM_TOOL_1]</parameter>
   </invoke>
   </function_calls>

   Step 3: [Third Tool, using results from second]
   <function_calls>
   <invoke name="[TOOL_3]">
   <parameter name="param1">[RESULT_FROM_TOOL_2]</parameter>
   </invoke>
   </function_calls>

4. Handle errors at each step:
   - Validate outputs before proceeding to next step
   - Implement recovery strategies for each possible failure
   - Consider whether partial results can be salvaged
   - Provide clear error messages

5. Document the workflow:
   - Explain the purpose of each step
   - Clarify data transformations between tools
   - Highlight critical error handling points
   - Suggest monitoring and validation approaches
```

## Tool Selection Reasoning Prompt

```
You need to select the most appropriate MCP tool for the current task. Follow this structured reasoning process:

1. Analyze the task requirements:
   - What is the primary objective?
   - What specific capabilities are needed?
   - Are there any constraints or special considerations?
   - What are the expected inputs and outputs?

2. Identify candidate tools:
   - Which MCP tools provide the required capabilities?
   - What are the strengths and limitations of each?
   - Do they support the required parameters?
   - Can they produce the expected output format?

3. Evaluate tool suitability:
   - How well does each tool match the requirements?
   - Are there any potential issues or limitations?
   - What are the tradeoffs between different options?
   - Are there any dependencies or prerequisites?

4. Select the optimal tool:
   - Which tool best meets all requirements?
   - What parameters will be needed?
   - How should the output be processed?
   - What error handling approach is appropriate?

5. Document your reasoning:
   - Explain why the selected tool is optimal
   - Note any important parameters or configurations
   - Highlight potential issues to watch for
   - Describe how outputs will be used
```

## Example Implementation: Code Review

Here's an example of how Claude might use the Code Review Prompt to review an MCP tool implementation:

```
After examining the error_handling_demo tool implementation, here's my comprehensive review:

## Strengths

1. **Comprehensive Error Handling**: The tool demonstrates multiple error types and handling patterns, providing a great educational resource.
2. **Structured Error Responses**: All error responses follow a consistent format with status, error_type, message, and recovery_hint.
3. **Parameter Validation**: Input validation is handled properly using zod schema.
4. **Clear Documentation**: Each function and error type is well-documented with comments.

## Critical Issues

1. **Missing Timeout Handling**: The timeout simulation doesn't include proper abort logic if the operation takes too long:

```javascript
// Current implementation
case "timeout":
  // Simulate a timeout error
  return new Promise((resolve) => {
    setTimeout(() => {
      resolve({
        status: "error",
        error_type: "TIMEOUT",
        message: `Operation timed out after ${timeout_seconds} seconds`,
        recovery_hint: "Try again with a simpler request or increase the timeout value"
      });
    }, timeout_seconds * 1000);
  });
```

Recommended fix:
```javascript
case "timeout":
  // Simulate a timeout error with proper abort logic
  return new Promise((resolve, reject) => {
    // Create an AbortController to handle timeout cancellation
    const abortController = new AbortController();
    const timeoutId = setTimeout(() => {
      abortController.abort();
      resolve({
        status: "error",
        error_type: "TIMEOUT",
        message: `Operation timed out after ${timeout_seconds} seconds`,
        recovery_hint: "Try again with a simpler request or increase the timeout value"
      });
    }, timeout_seconds * 1000);
    
    // If the operation is aborted externally, clean up the timeout
    abortController.signal.addEventListener('abort', () => {
      clearTimeout(timeoutId);
      reject(new Error('Operation aborted'));
    });
  });
```

2. **Uncaught Promise Rejections**: The current error handling might miss rejections in nested promises.

## Suggestions for Improvement

1. **Add More Operation Types**: Consider adding more complex error scenarios like network failures or rate limiting.
2. **Include Retry Logic**: Demonstrate recommended retry patterns for transient errors.
3. **Add Logging**: Include proper logging throughout the error handling process.
4. **Enhance Recovery Hints**: Make recovery suggestions more specific and actionable.

## Conclusion

The error_handling_demo tool is well-implemented overall and provides valuable examples of different error patterns. Addressing the critical issues and implementing the suggested improvements would make it an even more robust demonstration of best practices for error handling in MCP tools.
```

## Example Implementation: Tool Selection Reasoning

Here's an example of how Claude might use the Tool Selection Reasoning Prompt:

```
# Tool Selection Analysis for Team Communication Task

## 1. Task Requirements Analysis

- **Primary Objective**: Share project findings with the development team
- **Capabilities Needed**: 
  - Send messages to multiple recipients
  - Create a thread for ongoing discussion
  - Ensure all team members are notified
- **Constraints**: 
  - All team members must be included
  - Initial message must contain project findings
- **Expected I/O**:
  - Input: Message content, recipient list
  - Output: Confirmation of message delivery, thread reference

## 2. Candidate Tools Identification

Based on the requirements, I've identified these candidate tools:

1. **team_message**
   - Strengths: Simple, direct messaging
   - Limitations: No thread creation, single recipient
   - Parameters: recipient, message
   - Output: Delivery status

2. **send_team_message**
   - Strengths: Platform-specific messaging
   - Limitations: No thread creation, single recipient
   - Parameters: recipient, message, method
   - Output: Delivery status

3. **create_thread_with_notification**
   - Strengths: Creates thread, notifies all participants
   - Limitations: More complex setup
   - Parameters: title, purpose, creator_id, participants, initial_message
   - Output: Thread ID, notification status

4. **broadcast_thread_creation**
   - Strengths: Notifies about existing threads
   - Limitations: Requires existing thread
   - Parameters: thread_id, thread_title, creator_id, agent_ids, purpose
   - Output: Notification status

## 3. Tool Suitability Evaluation

Comparing the tools against requirements:

| Requirement | team_message | send_team_message | create_thread_with_notification | broadcast_thread_creation |
|-------------|--------------|-------------------|--------------------------------|----------------------------|
| Multiple recipients | ❌ | ❌ | ✅ | ✅ |
| Thread creation | ❌ | ❌ | ✅ | ❌ (requires existing) |
| Team notification | ❌ (manual) | ❌ (manual) | ✅ | ✅ |

## 4. Optimal Tool Selection

Based on this analysis, **create_thread_with_notification** is the optimal tool for this task because:

1. It handles multiple recipients through the participants array
2. It creates a new discussion thread automatically
3. It notifies all participants about the thread
4. It can include the project findings as the initial message

Required parameters:
- title: "Project XYZ Findings: April 2025"
- purpose: "Discussing project findings and next steps"
- creator_id: "claude_desktop_agent"
- participants: ["dev_lead", "backend_dev", "frontend_dev", "qa_lead"]
- initial_message: "Here are the key findings from our project analysis..."

Expected output processing:
- Extract thread_id for future reference
- Verify notifications were sent successfully
- Prepare for follow-up communication in the thread

Error handling approach:
- If tool unavailable: Fall back to individual messages with team_message
- If participant invalid: Remove invalid participant and proceed
- If thread creation fails: Retry with simplified parameters

## 5. Reasoning Summary

The create_thread_with_notification tool is selected because it is the only tool that satisfies all three key requirements: multiple recipients, thread creation, and team notification. The team_message and send_team_message tools lack thread creation capabilities, while broadcast_thread_creation requires an existing thread.
```
