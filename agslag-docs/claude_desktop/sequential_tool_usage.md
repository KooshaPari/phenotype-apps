# Patterns for Sequential Tool Usage

This document outlines standardized patterns for using multiple MCP tools in sequence to accomplish complex tasks.

## Basic Sequential Pattern

The basic pattern involves using tools in a linear sequence, where each step may depend on the output of previous steps:

```
Step 1: Tool A → Result A
Step 2: Tool B (using Result A) → Result B
Step 3: Tool C (using Result B) → Result C
```

### Implementation Example

```
<!-- Step 1: Generate data -->
<function_calls>
<invoke name="calculate_fibonacci">
<parameter name="n">10