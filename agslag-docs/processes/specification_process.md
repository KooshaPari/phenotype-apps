# Specification & Atomic Decomposition Process

## 1. Introduction

This document outlines the **Specification & Atomic Decomposition Process**, a critical initiation stage within the AGSLAG [Digital Product Development Lifecycle](./digital_product_lifecycle.md). This process focuses on translating high-level project goals, use cases, and initial designs into granular, trackable units ("atomic items") that define the precise scope of work.

This stage is typically led by a dedicated **Specification Team** and occurs before or in parallel with detailed planning phases (like backlog creation and release planning). Its primary goal is to ensure clarity, completeness, and measurability of scope *before* significant development resources are committed.

## 2. Goals

*   **Scope Clarity:** Achieve a deep, shared understanding of exactly what needs to be built.
*   **Granularity:** Decompose high-level requirements into the smallest meaningful, independently trackable items.
*   **Traceability:** Establish clear links between high-level goals, features, use cases, and atomic scope items.
*   **Measurability:** Define clear acceptance criteria or completion definitions for each atomic item, enabling objective progress and quality tracking.
*   **Risk Reduction:** Identify ambiguities, gaps, or potential complexities early in the lifecycle.
*   **Foundation for Planning:** Provide well-defined inputs for subsequent planning activities (estimation, backlog creation, sprint planning).

## 3. The Specification Team

*   **Role:** This cross-functional team is responsible for executing the Specification & Atomic Decomposition Process.
*   **Composition (Typical):**
    *   Product Manager(s)
    *   Lead Engineer(s) / Architect(s)
    *   Lead UI/UX Designer(s)
    *   Business Analyst(s)
    *   QA Lead(s) (for testability input)
    *   Subject Matter Experts (SMEs) (as needed)
    *   *AI agents may assist in analysis, decomposition, and documentation.*
*   **Responsibilities:**
    *   Deeply analyze inputs (project briefs, use cases, initial designs).
    *   Facilitate workshops and discussions to clarify requirements.
    *   Identify and document detailed functional and non-functional requirements.
    *   Decompose features/epics into atomic user stories or tasks.
    *   Define clear acceptance criteria for each atomic item.
    *   Identify dependencies between items.
    *   Maintain the repository of atomic scope items (e.g., in a dedicated specification tool, wiki, or structured document).
    *   Ensure traceability back to original goals/use cases.

## 4. Process Workflow

```mermaid
graph TD
    A[Input: Project Goals, Use Cases, High-Level Design, Market Needs] --> B{1. Deep Analysis Workshop};
    B -- Clarifications --> A;
    B --> C[2. Identify Core Features & Epics];
    C --> D[3. Decompose Features into Atomic Items <br><i>(e.g., User Stories, Tasks)</i>];
    D --> E[4. Define Acceptance Criteria for Each Item];
    E --> F[5. Identify Dependencies & Relationships];
    F --> G[6. Review & Refine Specification];
    G -- Feedback --> C; % Loop back to refine decomposition/criteria
    G --> H[Output: Atomic Scope Specification <br><i>(Traceable, Measurable Items)</i>];
    H --> I(Input for Planning & Backlog Creation);

    subgraph "Specification Team Activities"
        direction TB
        B; C; D; E; F; G;
    end

    style H fill:#cfc,stroke:#333,stroke-width:2px;
    style I fill:#eee,stroke:#333,stroke-width:1px,stroke-dasharray: 5 5;
```

**Workflow Steps:**

1.  **Deep Analysis Workshop:** The Specification Team convenes to thoroughly review all input materials, ask clarifying questions, and establish a shared understanding.
2.  **Identify Core Features & Epics:** High-level functional areas are identified and documented.
3.  **Decompose Features:** Each feature/epic is broken down into the smallest possible units of work that deliver distinct value or functionality (atomic items). This might involve techniques like user story mapping.
4.  **Define Acceptance Criteria:** Clear, testable acceptance criteria are written for *each* atomic item. This defines "done" for that specific piece of scope.
5.  **Identify Dependencies:** Relationships and dependencies between atomic items are mapped out.
6.  **Review & Refine:** The complete set of atomic items and criteria is reviewed by the team and potentially key stakeholders for completeness, clarity, and accuracy. Iterations occur as needed.
7.  **Output:** The final output is a detailed **Atomic Scope Specification**, serving as the definitive source of truth for what needs to be built.

## 5. Atomic Items

*   **Definition:** The smallest unit of work that can be independently estimated, developed, tested, and tracked.
*   **Examples:** A specific user story ("As a user, I can reset my password via email"), a specific API endpoint implementation, a single UI component state, a specific backend task.
*   **Characteristics:** Testable, Independent (as much as possible), Value-delivering (even if small), Estimable.
*   **Tracking:** These items form the basis for detailed backlog creation, sprint planning, progress tracking (beyond just feature completion), and quality assessment (tracking completion against acceptance criteria).

## 6. Integration with Lifecycle

*   This process forms the core of the **Specification** phase, bridging the gap between high-level **Ideation/Strategy** and detailed **Planning/Backlog Creation**.
*   The Atomic Scope Specification is a primary input for the [Product Management Workflow](./product_management_workflow.md) (backlog creation) and the [Software Engineering Process](./software_engineering_process_diagram.md) (sprint planning).
*   It provides the granularity needed for detailed QA planning ([Quality Assurance Protocols](./quality_assurance_protocols.md)).

## 7. Benefits

*   Reduces ambiguity and scope creep later in the project.
*   Improves estimation accuracy.
*   Enables more precise progress tracking.
*   Facilitates better quality control by defining "done" at a granular level.
*   Enhances team alignment and shared understanding.