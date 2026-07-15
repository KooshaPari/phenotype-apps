# Error Handling and Debugging Protocols

This document outlines standard procedures for handling errors when using MCP tools with Claude Desktop.

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

## Standard Error Handling Pattern

### Step 1: Detection and Diagnosis
```xml
<!-- Attempt tool invocation -->
<function_calls>
<invoke name="tool_name">
<parameter name="param1">value1