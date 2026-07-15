# Agent Comparison Report: Jarvis vs. Augment SWE-Bench

## 1. Introduction

This report compares two distinct AI agents found within the AGSLAG repository:

1.  **Jarvis SWE Agent:** A general-purpose software engineering agent service. ([Details](./jarvis_swe_agent_report.md))
2.  **Augment SWE-Bench Agent:** A specialized agent focused on SWE-Bench benchmark tasks. ([Details](./augment_swebench_agent_report.md))

The comparison focuses on their purpose, architecture, capabilities, and integration within the broader system.

## 2. Core Purpose & Design Philosophy

*   **Jarvis:** Designed as a **persistent backend service** providing flexible, general SWE capabilities. It manages multiple agent instances concurrently and integrates with the AGSLAG system via API/WebSockets and MCP. Its architecture prioritizes flexibility, multi-tasking, and system integration.
*   **Augment SWE-Bench:** Designed as a **standalone tool/script** optimized for a specific task domain (SWE-Bench evaluation). It operates in single, focused runs, likely invoked via CLI. Its architecture prioritizes controlled execution environments (Docker) and specific code manipulation tools for benchmarking.

## 3. Technical Implementation

| Feature          | Jarvis SWE Agent         | Augment SWE-Bench Agent |
| :--------------- | :----------------------- | :---------------------- |
| **Language**     | TypeScript (Node.js)     | Python                  |
| **Architecture** | Service-Oriented         | Class-based Script      |
| **Concurrency**  | Multi-Instance           | Single Run              |
| **State**        | Persistent Agent History | Run-specific History    |
| **Execution**    | API / WebSocket Server   | CLI / Script Invocation |

## 4. LLM Integration

Both agents demonstrate multi-LLM capabilities, supporting OpenAI, Anthropic, and Google Gemini models.

*   **Jarvis:** Allows model selection **per agent instance** and **per request** via a `ModelConfig` object, offering fine-grained control. Also supports OpenRouter.
*   **Augment SWE-Bench:** Likely configures the model **per run** (via CLI args or config), suitable for consistent benchmark evaluations.

## 5. Tooling & Capabilities

The toolsets reflect their different purposes:

*   **Jarvis:** Possesses a **broad, general-purpose toolset** including shell, web search, file I/O, code generation/analysis, testing, and visualization (diagrams/charts). Suitable for diverse SWE tasks.
*   **Augment SWE-Bench:** Features a **focused toolset** optimized for code manipulation within controlled environments: bash (optional Docker), string replacement/patching (`StrReplaceEditorTool`), and structured thinking (`SequentialThinkingTool`).

## 6. System Integration (AGSLAG)

*   **Jarvis:** Explicitly designed for integration. It includes **MCP client configuration** and runs as a service, making it a functional node within the AGSLAG ecosystem, capable of receiving tasks and interacting with other components.
*   **Augment SWE-Bench:** Shows **no direct evidence of MCP integration**. It functions primarily as a standalone evaluation tool.

## 7. Summary Table

| Aspect                  | Jarvis SWE Agent                                  | Augment SWE-Bench Agent                          |
| :---------------------- | :------------------------------------------------ | :----------------------------------------------- |
| **Primary Role**        | General SWE Service (AGSLAG Node)                 | Specialized SWE-Bench Evaluator                  |
| **Architecture**        | Service (Node.js), Multi-Instance, Event-Driven   | Script (Python), Single-Run, Turn-Based          |
| **LLMs**                | OpenAI, Anthropic, Gemini, OpenRouter             | OpenAI, Anthropic, Gemini                        |
| **Model Selection**     | Per Instance / Per Request                        | Per Run                                          |
| **Core Tools**          | Broad: Shell, Search, Files, Code, Test, Vis      | Focused: Bash (Docker), StrReplace, SeqThinking  |
| **Execution Env**       | Direct System                                     | Direct or Docker                                 |
| **Integration**         | API, Sockets, MCP Client                          | Standalone CLI Tool                              |

## 8. Conclusion

Jarvis SWE Agent serves as a flexible, integrated backend service providing core SWE capabilities within the AGSLAG system. Augment SWE-Bench Agent is a specialized tool optimized for running and evaluating performance on the SWE-Bench benchmark. While both leverage similar LLMs, their architecture, toolsets, and integration approaches differ significantly based on their intended roles.