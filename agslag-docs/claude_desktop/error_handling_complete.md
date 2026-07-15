# Error Handling and Debugging Protocols for Claude Desktop

This document outlines comprehensive error handling and debugging protocols for Claude Desktop's XML-based MCP tool integration.

## Error Classification

### 1. Tool Availability Errors
- **Tool Not Found**: The specified tool doesn't exist or isn't accessible
- **Tool Initialization Failure**: The tool exists but failed to initialize correctly

### 2. Parameter Errors
- **Missing Required Parameter**: A required parameter was not provided
- **Invalid Parameter Type**: Parameter value doesn't match expected type
- **Invalid Parameter Value**: Parameter value is outside acceptable range

### 3. Execution Errors
- **Timeout Error**: Tool execution exceeded time limit
- **Resource Limitation**: Memory or processing constraints affected execution
- **External Service Failure**: Dependent external service unavailable or failed

### 4. Output Processing Errors
- **Invalid Response Format**: Tool returned improperly formatted data
- **Missing Expected Data**: Tool response missing critical fields
- **Data Conversion Error**: Unable to process or parse returned data

## Standard Error Detection and Diagnosis

When Claude encounters an error in tool execution, it should:

1. Identify the error type based on the error message or behavior
2. Determine potential causes of the error
3. Consider the impact on the current task
4. Decide on an appropriate recovery strategy

## Error Recovery Strategies

### Tool Not Found
If a tool is not found, Claude should:
1. Check if the tool name is correct
2. Consider alternative tools with similar functionality
3. Attempt to use built-in capabilities instead
4. Inform the user of the issue and available alternatives

Example:
```
I attempted to use the 'diagnose_mcp_issues' tool, but it appears to be unavailable in this environment. Instead, I'll analyze the code manually and provide my recommendations.
```

### Missing Required Parameters
If a required parameter is missing, Claude should:
1. Identify which parameter is missing
2. Explain the parameter's purpose
3. Request the missing information from the user
4. Suggest default values if appropriate

Example:
```
To use the 'team_message' tool, I need both a 'message' and a 'recipient' parameter. You've provided the message, but could you please specify the recipient?
```

### Invalid Parameter Values
If a parameter value is invalid, Claude should:
1. Explain why the value is invalid
2. Clarify the expected format or range
3. Suggest corrections
4. Offer to retry with a valid value

Example:
```
The 'timeout_seconds' parameter must be between 1 and 10, but you provided 30. I'll use a value of 10 instead, which is the maximum allowed.
```

### Timeout Errors
For timeout errors, Claude should:
1. Acknowledge the timeout
2. Consider whether the operation can be simplified
3. Suggest breaking the task into smaller steps
4. Offer alternative approaches

Example:
```
The operation timed out after 5 seconds. This might be due to the complexity of the task. Let's break it down into smaller steps and process them sequentially.
```

### External Service Failures
When external services fail, Claude should:
1. Identify the dependent service
2. Check if the error is temporary or persistent
3. Suggest retry strategies or timing
4. Offer alternative services or approaches

Example:
```
The external API service is currently unavailable. This might be a temporary issue. You could try again in a few minutes, or we could use an alternative approach that doesn't rely on this service.
```

## Debugging Techniques

### Incremental Parameter Testing
When diagnosing complex errors, Claude should:
1. Start with minimal parameters
2. Add parameters one by one
3. Identify which parameter causes the error
4. Validate parameter values individually

Example:
```
Let's test the tool with just the required parameters first:

<function_calls>
<invoke name="error_handling_demo">
<parameter name="operation_type">success