# Specialized Prompts for Code Review and Development

This document provides specialized prompts for Claude Desktop to perform code review and development tasks using MCP tools.

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
<parameter name="log_content">[CODE_TO_REVIEW]