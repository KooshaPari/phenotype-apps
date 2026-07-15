# Claude-Specific MCP Integration Guide

This document provides comprehensive guidance for integrating Claude Desktop with MCP tools using XML-based patterns.

## Understanding Claude's XML Format

Claude Desktop uses a specialized XML format for tool invocation and execution. This format ensures structured, predictable interactions with MCP tools.

### Basic Structure

```
<function_calls>
<invoke name="tool_name">
<parameter name="param1">value1