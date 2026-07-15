# Frameworks for Complex Reasoning in Large Context Windows

Large context windows (e.g., 2M tokens) enable models to perform more complex reasoning tasks by holding vast amounts of related information simultaneously. However, guiding the model effectively through multi-step reasoning processes is crucial. This document outlines frameworks for structuring prompts and interactions to facilitate complex reasoning.

## Core Principles

1.  **Decomposition:** Break down complex problems into smaller, manageable sub-problems or reasoning steps.
2.  **Explicit Steps:** Clearly instruct the model on the sequence of steps to follow for analysis or problem-solving.
3.  **Information Synthesis:** Guide the model to explicitly synthesize information from different parts of the context to draw conclusions or make decisions.
4.  **Hypothesis Generation and Testing:** Encourage the model to generate hypotheses based on the context and then verify them against other information within the same context.
5.  **Chain-of-Thought (CoT) / Step-by-Step Reasoning:** Explicitly ask the model to "think step-by-step" or outline its reasoning process, making its internal state more transparent and allowing for easier debugging or correction.

## Framework Examples

### 1. Sequential Step-by-Step Analysis

*   **Goal:** Guide the model through a predefined sequence of analytical steps.
*   **Strategy:** Provide the full context and then explicitly list the steps the model should take in order. Each step should build upon the previous one.
*   **Prompt Template:**

```
**Task:** Analyze the performance impact of adding a new caching layer (`src/cache/redis_cache.py`) on the product recommendation engine (`src/recommendations/engine.py`).

**Context:** [Load `redis_cache.py`, `engine.py`, relevant data models, configuration files, and potentially some sample usage code or test cases]

**Reasoning Steps:**

1.  **Identify Integration Points:** Analyze `engine.py` to find where the caching layer (`redis_cache.py`) would be integrated (e.g., before database calls, after computation).
2.  **Analyze Cache Logic:** Examine `redis_cache.py`. What data structures does it use? What are the cache key strategies? What are the potential cache hit/miss scenarios?
3.  **Estimate Performance Gains (Cache Hits):** Based on the integration points and cache logic, estimate the potential time saved during cache hits (e.g., avoiding database queries or complex computations identified in `engine.py`).
4.  **Estimate Performance Overhead (Cache Misses):** Estimate the overhead introduced by the cache layer during cache misses (e.g., cache lookup time, data serialization/deserialization).
5.  **Analyze Potential Issues:** Consider potential issues like cache invalidation, data staleness, memory usage of the cache, and increased complexity based on the provided code.
6.  **Synthesize Findings:** Summarize the overall expected performance impact (latency reduction, throughput changes) and list key trade-offs or potential risks.

Please perform this analysis step-by-step, clearly labeling the output for each step.
```

### 2. Hypothesis-Driven Debugging

*   **Goal:** Diagnose a bug or unexpected behavior using a structured hypothesis testing approach.
*   **Strategy:** Provide code context, bug description, and relevant logs. Ask the model to generate potential hypotheses, then devise ways to test them against the provided context.
*   **Prompt Template:**

```
**Bug Description:** Users are reporting intermittent '500 Internal Server Error' when updating their profile information. The error seems to occur most often during peak hours.

**Context:** [Load `user_profile_controller.py`, `user_service.py`, `database_connector.py`, relevant deployment configuration (e.g., number of replicas), and error log snippets showing the 500 errors and surrounding logs]

**Reasoning Framework:**

1.  **Initial Analysis:** Briefly analyze the error logs and the code flow for profile updates (`user_profile_controller.py`, `user_service.py`).
2.  **Generate Hypotheses:** Based on the initial analysis and the intermittent/peak-hour nature of the bug, list 3-5 plausible hypotheses for the root cause (e.g., database connection pool exhaustion, race condition in `user_service.py`, downstream service timeout, resource limits in deployment).
3.  **Test Hypotheses against Context:** For each hypothesis, explain how you would test it *using only the information provided in the context*.
    *   Example (for DB pool exhaustion): "Check `database_connector.py` for pool size settings. Correlate error timestamps in logs with potential peak load indicators (if any). Look for specific database connection errors in the logs."
    *   Example (for race condition): "Examine `user_service.py` for shared state modifications without proper locking mechanisms during the profile update process."
4.  **Evaluate Hypotheses:** Based on the tests in step 3, evaluate the likelihood of each hypothesis. Which one(s) are most strongly supported by the context?
5.  **Suggest Next Steps:** Recommend specific code sections to investigate further or additional logging/monitoring to add to confirm the most likely hypothesis.

Follow this framework step-by-step in your response.
```

### 3. Comparative Analysis

*   **Goal:** Compare and contrast different implementations, libraries, or architectural approaches found within the context.
*   **Strategy:** Load the code for the different approaches into the context. Provide clear criteria for comparison.
*   **Prompt Template:**

```
**Task:** Compare the two different approaches used for handling asynchronous tasks found in the codebase:
    A) The custom `AsyncTaskProcessor` class in `src/utils/async_processor.py`.
    B) The usage of the 'Celery' library in `src/workers/celery_tasks.py`.

**Context:** [Load `async_processor.py`, `celery_tasks.py`, examples of how each is used, and potentially configuration files for Celery]

**Comparison Criteria:**

1.  **Ease of Use:** How simple is it to define and dispatch tasks using each approach?
2.  **Scalability:** How well does each approach handle a large number of tasks or workers? (Infer from code structure and library features).
3.  **Error Handling:** How are errors managed in each system? Is there support for retries?
4.  **Dependencies:** What external dependencies does each approach introduce?
5.  **Monitoring/Introspection:** Does the code provide ways to monitor task status or progress?
6.  **Use Cases:** Based on the code, what types of tasks seem best suited for each approach?

**Output:** Provide a structured comparison based on these criteria, highlighting the pros and cons of each approach as implemented in this specific codebase. Conclude with a recommendation on whether to standardize on one approach or if maintaining both is justified.
```

### 4. System Design and Refactoring Proposals

*   **Goal:** Propose changes to system design or refactor existing code based on a holistic understanding.
*   **Strategy:** Load significant portions of the relevant system components. Ask the model to analyze the current state and propose specific, actionable changes, considering the broader context. Use CoT prompting.
*   **Prompt Template:**

```
**Task:** Propose a refactoring plan to decouple the `OrderProcessingService` (`src/services/order.py`) from the `NotificationService` (`src/services/notification.py`). Currently, `OrderProcessingService` directly calls methods on `NotificationService`.

**Context:** [Load `order.py`, `notification.py`, any shared data models, and examples of how `OrderProcessingService` is used]

**Request:**

Analyze the current interaction between `OrderProcessingService` and `NotificationService`. Then, thinking step-by-step, propose a refactoring strategy to decouple them using an event-driven approach (e.g., publishing an `OrderCreated` event).

Your proposal should include:

1.  **Event Definition:** Define the structure of the `OrderCreated` event.
2.  **Publisher Changes:** Specify the changes needed in `OrderProcessingService` to publish this event instead of directly calling `NotificationService`.
3.  **Subscriber/Handler:** Describe how `NotificationService` (or a new dedicated handler) would subscribe to and handle this event.
4.  **Event Bus/Mechanism:** Briefly mention the type of event bus or mechanism assumed (e.g., simple in-memory, Redis pub/sub, Kafka - acknowledge if the specific mechanism isn't defined in context but state the assumption).
5.  **Benefits:** List the benefits of this decoupled approach.
6.  **Risks/Challenges:** Identify potential risks or challenges with implementing this change.

Outline your reasoning clearly for each part of the proposal.
```

## Tool Integration

*   **`read_file`:** Essential for loading the necessary code and configuration context.
*   **`search_files`:** Useful for finding related code sections or usage examples to add to the context.
*   **`list_code_definition_names`:** Can help in identifying components and their relationships before loading full code.
*   **`memorymesh`:** Store intermediate reasoning steps, hypotheses, or design proposals for complex, multi-turn interactions.

By structuring complex reasoning tasks into explicit steps and leveraging the model's ability to process large amounts of interconnected information, these frameworks help ensure more accurate, thorough, and reliable results.