# Conflict Resolution Process for Multi-Agent Systems

**Version:** 1.0
**Date:** 2025-04-05

## 1. Introduction

This document details the standardized process for resolving conflicts or disagreements between AI agents operating within the MCP platform. Conflicts can arise from differing technical approaches, interpretations of tasks, resource contention, or conflicting information. This structured process aims to resolve conflicts efficiently and objectively, prioritizing project goals and maintaining collaborative harmony. It expands upon Section 4 of the Core Prompt Engineering Framework.

## 2. Core Principles

*   **Objectivity:** Focus on facts, evidence, and data rather than subjective opinions.
*   **Respect:** Maintain a constructive tone, acknowledging valid points from all parties.
*   **Goal Alignment:** Frame discussions and solutions in the context of the overall task or project objectives.
*   **Transparency:** Clearly state positions, reasoning, and evidence. Document the resolution process and outcome.
*   **Timeliness:** Address conflicts promptly to avoid delaying progress.

## 3. Roles

*   **Involved Agents:** The agents directly participating in the disagreement.
*   **Initiating Agent:** The agent who first identifies and reports the conflict.
*   **Mediator (Optional but Recommended):** A designated neutral agent (often the Coordinator or a specifically assigned agent) responsible for guiding the process, ensuring adherence to the protocol, facilitating discussion, and potentially making a final decision if consensus cannot be reached.

## 4. Conflict Resolution Steps

This process is typically initiated via the `model_conflict_resolution` prompt or when an agent identifies a conflict during its workflow.

**Step 1: Identification & Initiation**
*   **Trigger:** An agent detects a conflict (e.g., conflicting code commits, contradictory information, disagreement on approach).
*   **Action (Initiating Agent):**
    *   Verify the conflict is genuine and not a simple misunderstanding (e.g., check KM, re-read messages).
    *   If confirmed, send a message (`send_message`, `message_type`: 'conflict_report') to all directly involved agents and the designated Mediator/Coordinator.
    *   The message payload should clearly describe the conflict, referencing specific artifacts (message IDs, commit SHAs, file paths, KM nodes, task IDs) and stating why it's perceived as a conflict.
    *   **Example Payload:** `{'conflict_description': 'Agent-A committed code using library X, while Agent-B committed conflicting code using library Y for the same feature.', 'references': ['commit:abc123ef', 'commit:def456ab', 'task_id: Task-789']}`

**Step 2: Position Statement & Evidence Gathering**
*   **Trigger:** Receipt of a 'conflict_report' message.
*   **Action (Each Involved Agent):**
    *   Acknowledge receipt of the report (`send_message`).
    *   Update status (`report_status`) to indicate participation in conflict resolution.
    *   Gather objective evidence supporting their position/action (e.g., query KM `memorymesh.search_nodes`, check requirements, run tests `execute_command`, check logs `docker-mcp.get-logs`).
    *   Clearly and concisely state their position and the rationale behind it via `send_message` to the group/mediator.
    *   **Crucially:** Include references to the gathered evidence (KM nodes, test results, documentation links, specific requirements).

**Step 3: Perspective Understanding**
*   **Trigger:** All involved agents have stated their positions and provided evidence.
*   **Action (All Involved Agents & Mediator):**
    *   Carefully review all submitted positions and evidence (`get_messages`).
    *   **Active Listening Simulation:** Send a message (`send_message`) summarizing their understanding of *each opposing viewpoint* to ensure accurate comprehension. Example: "My understanding is Agent-A prefers Library X due to [reason based on Agent-A's message/evidence]."
    *   Ask specific, non-leading clarifying questions (`send_message`) if any position or evidence is unclear. Avoid accusatory language.

**Step 4: Identify Common Ground & Core Issues**
*   **Trigger:** Mutual understanding of all perspectives is confirmed (e.g., via acknowledgment messages).
*   **Action (Mediator, or collaboratively if no mediator):**
    *   Analyze the positions and evidence to identify areas of agreement or shared goals (e.g., "All agents agree the feature must meet performance requirement Z").
    *   Pinpoint the specific, objective core point(s) of disagreement (e.g., "The core conflict is the choice between Library X's ease of use vs. Library Y's performance benchmark results").
    *   Document and share these common ground points and core issues via `send_message`.

**Step 5: Generate & Evaluate Alternatives**
*   **Trigger:** Core issues are clearly defined.
*   **Action (All Involved Agents & Mediator):**
    *   Brainstorm potential alternative solutions or compromises that address the core issues while respecting the common ground and project goals (`send_message`). Examples: "Could we use Library X but optimize section P?", "Can we benchmark Library Y in our specific use case?", "Is there a third library Z that meets both needs?".
    *   Objectively evaluate the pros and cons of each *new* alternative based on evidence, requirements, effort, risk, and project impact. Use `evaluate_contribution` to provide structured feedback on alternatives.

**Step 6: Consensus Building & Decision**
*   **Trigger:** Alternatives have been generated and evaluated.
*   **Action (Mediator or Collaborative Agreement):**
    *   Facilitate discussion towards a consensus on the best path forward. Encourage agents to weigh the evidence and project goals.
    *   If consensus is reached, the Mediator (or an agreed-upon agent) documents the agreed-upon solution and rationale (`send_message`).
    *   If consensus cannot be reached within a reasonable timeframe, the Mediator (or Coordinator) makes a final decision based on the evidence presented and overall project priorities. The decision must be clearly communicated with its rationale.

**Step 7: Documentation & Closure**
*   **Trigger:** A decision/consensus is reached.
*   **Action (Mediator or Designated Agent):**
    *   Log the final decision, the **key rationale**, a brief note on rejected alternatives, and links to relevant evidence/discussions into the KM system (`memorymesh.add_nodes`, type 'ConflictResolution'). Link this node back to the original problem/task nodes.
    *   Send a final confirmation message (`send_message`) summarizing the resolution to all involved parties.
    *   All involved agents acknowledge the resolution (`send_message`) and update their status (`report_status`) to reflect they are no longer blocked by this conflict.

## 5. Escalation

If the conflict cannot be resolved at the agent/team level, or if it involves fundamental disagreements about project direction, the Mediator/Coordinator should escalate the issue to a higher authority (e.g., the human supervisor or project sponsor) using appropriate communication channels, providing a summary of the conflict, positions, evidence, and attempted resolution steps.

## 6. Conclusion

This structured process promotes objective, evidence-based resolution of disagreements. By clearly defining steps, roles, and documentation requirements, it aims to minimize disruption and ensure that conflicts are resolved in a way that best serves the project goals. Consistent application and reinforcement through prompt engineering are essential.
