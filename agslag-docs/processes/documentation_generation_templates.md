# Documentation Generation Templates for Large Context Windows

This document provides templates and strategies for generating comprehensive documentation using large context window models (e.g., 2M tokens). The large context allows loading significant amounts of source code and related artifacts to produce accurate and context-aware documentation.

## Core Principles

1.  **Context is Key:** Provide as much relevant source code, existing documentation, and architectural context as possible within the window.
2.  **Specify Audience and Purpose:** Clearly define the target audience (e.g., end-users, developers, API consumers) and the purpose of the documentation (e.g., tutorial, reference, explanation).
3.  **Define Structure and Format:** Guide the model by providing a desired structure (e.g., sections, headings) and output format (e.g., Markdown, JSDoc, Python docstrings).
4.  **Iterative Refinement:** Generate documentation section by section or file by file, review, and refine the prompts or generated content as needed.

## Template Examples

### 1. Generating Module/Library Documentation (Markdown)

*   **Goal:** Create a README or module-level documentation explaining its purpose, usage, and key components.
*   **Strategy:** Load all source files for the module, plus any relevant interface definitions or dependent modules if space permits.
*   **Prompt Template:**

```markdown
Generate comprehensive Markdown documentation for the '[Module Name]' module based on the provided source code files in context: `[file1.ts, file2.ts, types.ts, ...]`.

**Target Audience:** Developers integrating with or maintaining this module.

**Purpose:** Provide a clear understanding of the module's functionality, how to use it, and its internal structure.

**Required Sections:**

1.  **Overview:** Briefly describe the module's purpose and core responsibilities.
2.  **Installation/Setup:** (If applicable) Explain how to install or set up the module.
3.  **Core Concepts:** Explain any key concepts, patterns, or architectural decisions relevant to the module.
4.  **API Reference:**
    *   Document all public functions, classes, and methods.
    *   For each item, include:
        *   A clear description of its purpose.
        *   Parameters (name, type, description, optional/required).
        *   Return value (type, description).
        *   Example usage code snippets (generate simple, illustrative examples).
5.  **Usage Examples:** Provide 1-2 more detailed examples demonstrating common use cases.
6.  **Error Handling:** Describe common errors and how they should be handled.
7.  **Contributing:** (Optional) Guidelines for contributing to the module.

**Format:** Standard Markdown. Ensure code examples are properly formatted in code blocks with language identifiers. Extract information directly from the source code comments (like JSDoc or Python docstrings) where available, but elaborate and ensure consistency.
```

### 2. Generating Function/Method Docstrings (e.g., JSDoc, Python Docstrings)

*   **Goal:** Add or update detailed docstrings for specific functions or methods.
*   **Strategy:** Load the file containing the function/method, and potentially files containing related functions or data structures it uses/returns.
*   **Prompt Template (JSDoc):**

```javascript
Generate a detailed JSDoc comment block for the function `[functionName]` defined in the file `[filename.js]` provided in context. Analyze the function's implementation, parameters, return values, and any potential side effects or errors.

**Context Files:** `[filename.js, related_type_definitions.ts, ...]`

**Required JSDoc Tags:**

*   `@description` A clear, concise summary of what the function does.
*   `@param` {type} name - Description for each parameter.
*   `@returns` {type} - Description of the return value.
*   `@throws` {ErrorType} - Description of any errors explicitly thrown.
*   `@example` - Provide a simple code snippet demonstrating usage.

Ensure the types are inferred correctly from the code or type definitions provided. If the function is asynchronous, reflect that in the description or return type.

**Function Signature (for reference):**
```javascript
// [Paste function signature here if helpful]
```

**Source Code (in context):** `[filename.js]`
```

*(Adapt `@param`, `@returns`, `@throws` etc. for Python docstring format)*

### 3. Generating API Endpoint Documentation (OpenAPI/Swagger Snippet)

*   **Goal:** Create documentation for a specific API endpoint.
*   **Strategy:** Load the controller/handler file for the endpoint, relevant request/response model definitions, and potentially the router configuration file.
*   **Prompt Template:**

```yaml
Generate an OpenAPI 3.0 specification snippet (in YAML format) for the API endpoint handled by the function `[handlerFunctionName]` in file `[filename.controller.ts]` provided in context.

**Context Files:** `[filename.controller.ts, request.dto.ts, response.dto.ts, routes.ts]`

**Endpoint Details:**

*   **HTTP Method:** (Infer from code, e.g., GET, POST)
*   **Path:** (Infer from router config or controller decorators, e.g., /users/{id})
*   **Summary:** A short summary of the endpoint's purpose.
*   **Description:** A more detailed explanation.
*   **Tags:** (Suggest relevant tags, e.g., "User Management")
*   **Parameters:** Document path parameters, query parameters (name, in, description, required, schema). Infer types from DTOs/code.
*   **RequestBody:** Document the request body (description, required, content type e.g., application/json, schema referencing the Request DTO).
*   **Responses:**
    *   Document the primary success response (e.g., 200 OK, 201 Created). Include description and content schema referencing the Response DTO.
    *   Document potential error responses (e.g., 400 Bad Request, 404 Not Found, 500 Internal Server Error). Include descriptions.

Analyze the handler function, DTOs, and routing information in context to accurately populate the specification.
```

### 4. Generating User Guide / Tutorial Section

*   **Goal:** Create a step-by-step guide for a specific feature or workflow.
*   **Strategy:** Load relevant UI component code (if applicable), API client code, core logic modules involved in the feature, and any existing high-level descriptions.
*   **Prompt Template:**

```markdown
Write a section for a user guide explaining how to perform the '[Feature Name]' task using our application.

**Target Audience:** End-users with basic familiarity with the application.

**Purpose:** Provide clear, step-by-step instructions to successfully complete the task.

**Context Files:** `[feature.component.jsx, api_client.js, core_logic.py, overview.md]`

**Key Steps to Cover (based on context):**

1.  Navigate to the correct section/page.
2.  Input required data (mention specific fields/forms seen in UI code).
3.  Trigger the action (e.g., button click).
4.  Explain expected outcomes or feedback (e.g., success message, updated display).
5.  Mention potential common issues or tips.

Use simple language. Refer to UI elements (buttons, fields) consistently if their names are available in the code context. Generate illustrative examples where appropriate. Structure as a numbered list.
```

## Utilizing Tools

*   **`read_file`:** Essential for loading source code into the context.
*   **`list_code_definition_names`:** Helps identify key functions/classes to document within a file or module.
*   **`search_files`:** Useful for finding all usages of a function/class to generate more comprehensive examples or understand its role better.
*   **`memorymesh`:** Store generated documentation snippets or summaries for later assembly into larger documents.
*   **`ragdocs` (if available):** Could potentially be used to retrieve existing documentation snippets or style guides to ensure consistency.