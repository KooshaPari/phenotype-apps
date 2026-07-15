# Claude Desktop XML-Based MCP Tool Integration

This package provides a comprehensive XML-based MCP tool integration for Claude Desktop, focusing on precise tool usage, structured reasoning, and sophisticated error handling.

## Overview

The Claude Desktop XML-based MCP tool integration system includes:

1. **XML Templates**: Standardized XML formats for all MCP tools
2. **Structured Reasoning Framework**: Decision processes for tool selection
3. **Error Handling Protocols**: Comprehensive error management
4. **Sequential Tool Usage Patterns**: Workflows for complex tasks
5. **Claude-Specific Integration**: Best practices for Claude Desktop
6. **Specialized Prompts**: Code review and development templates

## Installation

### Prerequisites

- Node.js (v16+)
- Claude Desktop application
- MCP Server setup

### Setup Steps

1. Install dependencies:
   ```
   npm install
   ```

2. Build the MCP server:
   ```
   cd mcp-server
   npm run build
   ```

3. Start the Claude Desktop MCP server:
   - Windows: `start-claude-mcp.bat`
   - Unix/Mac: `./start-claude-mcp.sh`

## Documentation

Detailed documentation for each component of the system is available in the `docs/claude_desktop` directory:

- [XML Templates](./xml_templates_complete.md): Standardized XML formats for all MCP tools
- [Reasoning Framework](./reasoning_framework_complete.md): Decision processes for tool selection
- [Error Handling](./error_handling_complete.md): Comprehensive error management
- [Sequential Patterns](./sequential_patterns_complete.md): Workflows for complex tasks
- [Specialized Prompts](./specialized_prompts_complete.md): Code review and development templates

## Tools

The integration includes specialized tools designed for Claude Desktop:

- **structured_reasoning**: Provides a structured reasoning framework for complex problem-solving
- **error_handling_demo**: Demonstrates different error handling patterns

## Usage Examples

### Basic Tool Invocation

```xml
<function_calls>
<invoke name="structured_reasoning">
<parameter name="problem_statement">How to improve MCP tool integration