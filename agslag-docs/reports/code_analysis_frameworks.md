# Code Analysis Frameworks for Large Context Windows

This document outlines frameworks and strategies for analyzing code across large codebases, specifically designed to leverage the capabilities of large context window models (e.g., 2M tokens).

## Leveraging Large Context Windows for Code Analysis

Large context windows offer significant advantages for understanding complex codebases:

1.  **Holistic Understanding:** Load entire modules, libraries, or even significant portions of the application into the context window to understand interdependencies, data flow, and overall architecture without constant context switching.
2.  **Deep Dependency Tracing:** Trace function calls, variable usage, and class inheritance across multiple files simultaneously within the context.
3.  **Cross-Cutting Concern Analysis:** Analyze how features or concerns (like logging, authentication, error handling) are implemented across the entire codebase loaded into context.
4.  **Refactoring Impact Assessment:** Load all potentially affected files into the context to accurately predict the impact of refactoring efforts and identify necessary changes.
5.  **Bug Triage and Root Cause Analysis:** Provide extensive context, including relevant code sections, logs (if available via tools), and issue descriptions, to pinpoint the root cause of bugs more effectively.

## Frameworks and Strategies

### 1. Full Project Context Loading (Selective)

*   **Goal:** Load the most relevant parts of a project for a specific analysis task.
*   **Strategy:**
    *   Use `mcp-server-everything-search` or `search_files` with specific queries (e.g., function names, class names, specific API usage) to identify key files related to the task.
    *   Prioritize loading core modules, entry points, configuration files, and files identified by the search.
    *   Use `read_file` to load the content of these selected files into the prompt context.
    *   Structure the prompt to guide the model's analysis based on the loaded context.
*   **Example Prompt Snippet:**
    ```
    Analyze the following code files loaded into context ([file1.py, file2.js, config.json]) to trace the data flow for the 'user_authentication' process. Identify all functions involved, data structures passed, and potential security vulnerabilities.
    ```

### 2. Hierarchical Summarization and Analysis

*   **Goal:** Analyze codebases larger than the context window by breaking them down.
*   **Strategy:**
    *   Divide the codebase into logical modules or directories.
    *   **Step 1 (Summarization):** For each module, load its files into the context and ask the model to generate a concise summary, including key functionalities, public APIs, dependencies, and core algorithms. Use `list_code_definition_names` to help identify key elements.
    *   **Step 2 (Integration):** Load the summaries of all modules into the context.
    *   **Step 3 (Analysis):** Ask the model to perform analysis based on the integrated summaries (e.g., identify architectural patterns, find dependencies between modules, suggest high-level refactoring).
*   **Example Prompt (Step 1):**
    ```
    Summarize the purpose, key functions/classes, external dependencies, and overall role of the 'payment_processing' module based on the following files: [payment_service.ts, stripe_integration.ts, models/transaction.ts].
    ```
*   **Example Prompt (Step 3):**
    ```
    Based on the summaries of the 'user_management', 'payment_processing', and 'order_fulfillment' modules provided in context, analyze the overall architecture for handling a customer order. Identify potential bottlenecks or areas for improvement in inter-module communication.
    ```

### 3. Definition-Focused Analysis

*   **Goal:** Understand the structure and relationships within the codebase by focusing on definitions.
*   **Strategy:**
    *   Use `list_code_definition_names` recursively (or iteratively on subdirectories) to gather a comprehensive list of classes, functions, methods, etc., across the project or relevant sections.
    *   Load this list of definitions into the context.
    *   Ask the model to analyze relationships, identify potential duplicate definitions, find unused code, or map out inheritance hierarchies based *only* on the definition names and their file locations.
    *   If deeper analysis is needed on specific relationships, load the relevant files identified in the previous step using `read_file`.
*   **Example Prompt:**
    ```
    Analyze the provided list of function and class definitions from the 'core_engine' directory. Identify potential areas of high coupling based on naming conventions and file organization. List classes that seem tightly related.
    ```

### 4. Targeted Search and Snippet Analysis

*   **Goal:** Investigate specific patterns, function usages, or potential issues without loading entire files.
*   **Strategy:**
    *   Use `search_files` with precise regex patterns to find specific code snippets (e.g., usage of a deprecated function, specific error handling patterns, database query construction).
    *   Load the search results (including context lines) into the prompt.
    *   Ask the model to analyze these specific snippets, identify patterns, suggest fixes, or categorize the findings.
*   **Example Prompt:**
    ```
    Analyze the following search results for the usage of 'unsafe_query_builder'. Identify instances where user input might be directly incorporated into the query and suggest safer alternatives based on the surrounding code context.
    [Search results from search_files tool]
    ```

## Tool Integration

*   **`mcp-server-everything-search` / `search_files`:** Essential for identifying relevant files and code snippets quickly across large codebases.
*   **`read_file`:** Crucial for loading the content of identified files into the context window. Use start/end lines for very large files if only specific sections are needed initially.
*   **`list_code_definition_names`:** Provides a high-level overview of code structure, useful for hierarchical analysis and understanding relationships without loading full file contents.
*   **`memorymesh`:** Store summaries, analysis results, and key findings persistently for later retrieval or integration into subsequent analysis prompts.

By combining these frameworks and leveraging the large context window alongside targeted tool usage, complex codebases can be analyzed efficiently and comprehensively.
