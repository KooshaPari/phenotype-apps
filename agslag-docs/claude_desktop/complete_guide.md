# Claude Desktop XML-Based MCP Tool Integration Guide

## Overview

This comprehensive guide outlines the XML-based MCP tool integration for Claude Desktop, providing developers and users with detailed instructions on how to effectively implement and use the specialized prompt engineering system.

## Installation

### Prerequisites

- Node.js (v16+)
- Claude Desktop application
- MCP Server setup

### Setup Steps

1. Clone or download the MCP repository
2. Install dependencies:
   ```
   cd agslag
   npm install
   ```
3. Build the tools:
   ```
   cd mcp-server
   npm run build
   ```
4. Start the Claude Desktop MCP server:
   ```
   cd ..
   ./start-claude-mcp.bat   # Windows
   ./start-claude-mcp.sh    # Linux/Mac
   ```

## XML Syntax for Claude Desktop

Claude Desktop uses a specialized XML format for tool invocation:

```xml
<function_calls>
<invoke name="tool_name">
<parameter name="param1">value1