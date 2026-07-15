# Sequential Tool Usage Patterns for Claude Desktop

This document outlines standardized patterns for using multiple MCP tools in sequence to accomplish complex tasks with Claude Desktop.

## Core Sequential Patterns

### 1. Linear Chain Pattern

The Linear Chain Pattern involves using tools in a direct sequence, where each tool depends on the output of the previous tool.

**Structure:**
```
Tool A → Output A → Tool B → Output B → Tool C → Output C
```

**Implementation Example:**
```
Step 1: Use Tool A to generate initial data
Step 2: Use Tool B with the output from Tool A
Step 3: Use Tool C with the output from Tool B
```

This pattern is most effective when each step in the process builds directly on the previous step's output.

### 2. Branch and Merge Pattern

The Branch and Merge Pattern involves using multiple tools in parallel branches and then combining their outputs.

**Structure:**
```
           → Tool B1 → Output B1 →
Tool A →                           → Tool D → Output D
           → Tool B2 → Output B2 →
```

**Implementation Example:**
```
Step 1: Use Tool A to generate initial data
Step 2a: Use Tool B1 with Output A for analysis path 1
Step 2b: Use Tool B2 with Output A for analysis path 2
Step 3: Use Tool D with the combined outputs from B1 and B2
```

This pattern is effective when parallel processing of the same input data is required.

### 3. Conditional Branching Pattern

The Conditional Branching Pattern involves choosing different tool paths based on conditions or previous results.

**Structure:**
```
           → Condition Met → Tool B → Output B →
Tool A →                                        → Final Result
           → Condition Not Met → Tool C → Output C →
```

**Implementation Example:**
```
Step 1: Use Tool A to gather information
Step 2: Evaluate the results from Tool A
Step 3a: If certain conditions are met, use Tool B
Step 3b: If conditions are not met, use Tool C
Step 4: Combine or select appropriate output as final result
```

This pattern is useful when different situations require different processing approaches.

### 4. Iterative Refinement Pattern

The Iterative Refinement Pattern involves repeatedly using the same tool or set of tools to progressively improve results.

**Structure:**
```
Initial Input → Tool A → Output 1 → Tool A → Output 2 → Tool A → Final Output
```

**Implementation Example:**
```
Step 1: Use Tool A with initial input
Step 2: Evaluate Output 1 against desired criteria
Step 3: Use Tool A again with adjustments based on evaluation
Step 4: Repeat until satisfactory results are achieved
```

This pattern is effective for optimization problems or when progressive refinement is needed.

## Real-World Application Patterns

### Research and Communication Workflow

This workflow combines research tools with communication tools to share findings:

1. Use search or research tools to gather information
2. Use analytical tools to process and evaluate the information
3. Use communication tools to share findings with teammates

**Implementation Example:**
```
Step 1: Use brave_search to find information
Step 2: Use structured_reasoning to analyze the information
Step 3: Use create_thread_with_notification to share analysis with team
```

### Diagnostic and Recovery Workflow

This workflow diagnoses issues and implements solutions:

1. Use diagnostic tools to identify problems
2. Use analytical tools to determine solutions
3. Use implementation tools to apply fixes
4. Use testing tools to verify solutions

**Implementation Example:**
```
Step 1: Use diagnose_mcp_issues to identify server problems
Step 2: Use structured_reasoning to analyze possible solutions
Step 3: Use create_mcp_brave_search to implement a fixed version
Step 4: Use error_handling_demo to verify proper error handling
```

### Content Generation and Review Workflow

This workflow creates content and facilitates team review:

1. Use generation tools to create initial content
2. Use evaluation tools to assess quality
3. Use communication tools to gather feedback
4. Use refinement tools to improve content

**Implementation Example:**
```
Step 1: Use gemini_generate to create initial content
Step 2: Use structured_reasoning to evaluate content quality
Step 3: Use create_thread_with_notification to gather team feedback
Step 4: Use gemini_generate again to refine based on feedback
```

## Error Handling in Sequential Patterns

Each sequential pattern should include specific error handling strategies:

### Fallback Paths

Define alternative paths to take when a tool in the sequence fails:

```
Primary Path: Tool A → Tool B → Tool C
Fallback Path: Tool A → Tool D → Tool E (if Tool B fails)
```

### Error Recovery Loops

Implement retry mechanisms for transient errors:

```
Tool A → Tool B → [Error] → Wait/Adjust → Tool B (retry) → Proceed
```

### Graceful Degradation

When advanced tools are unavailable, fall back to simpler alternatives:

```
Try: Advanced Tool → Complex Processing
Fallback: Basic Tool → Simplified Processing
```

### Progressive Implementation

Start with essential steps and add optional ones if previous steps succeed:

```
Essential: Tool A → Tool B
If successful, add: Tool C → Tool D (enhancement)
```

## Implementation Guidelines

When implementing sequential tool patterns, follow these best practices:

1. **Plan the entire sequence** before beginning execution
2. **Validate inputs and outputs** at each step of the sequence
3. **Save intermediate results** for potential recovery or debugging
4. **Include error handling** for each transition in the sequence
5. **Document the expected flow** and potential alternative paths
6. **Consider resource limitations** that might affect long sequences
7. **Test the complete sequence** with various inputs and error conditions
