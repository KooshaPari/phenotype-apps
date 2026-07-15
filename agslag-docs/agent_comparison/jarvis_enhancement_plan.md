# Jarvis SWE Agent Enhancement Plan

This document outlines a plan to enhance the `jarvis-swe-agent` by incorporating capabilities observed in other advanced agents like `OpenManus`, `manus-open`, `anus`, `claude-code`, and `augment-swebench-agent`.

## 1. Agent Comparison Summary

Analysis of the other agents reveals several capability areas where Jarvis could be enhanced:

*   **Advanced Terminal Management:** Other agents (`manus-open`, `augment-swebench-agent`) feature persistent, interactive terminal sessions with input injection, process control, and contextual execution (SSH/Docker). Jarvis currently has basic, non-interactive shell execution.
*   **Advanced Browser Automation:** `manus-open` demonstrates full browser control (navigation, clicks, input, JS execution, etc.), far exceeding Jarvis's simple API-based web search.
*   **Sophisticated File Editing:** Agents like `manus-open` and `augment-swebench-agent` offer indentation-aware string replacement, insertion, undo capabilities, and regex search within files. Jarvis has basic file I/O.
*   **Specialized Tools:** Other agents include tools like Python REPLs, web crawlers, safe calculators, text manipulation suites, direct Git integration, and specific code understanding/refactoring commands (`explain`, `refactor`, `fix`).
*   **Meta-Cognitive Tools:** `augment-swebench-agent` includes tools for sequential thinking/planning and using agents hierarchically.

## 2. Proposed Enhancements

Based on the comparison, the following enhancements are proposed for `jarvis-swe-agent`:

### Proposal 1: Enhance Shell Tool (`shell.ts`)
*   **Goal:** Upgrade `execShellCommand` to support persistent, interactive terminal sessions.
*   **Approach:** Use `node-pty`, implement session management, add functions for starting/closing sessions, executing commands within sessions, sending input, and killing processes.

### Proposal 2: Add Browser Automation Tool (`browser.ts` - New)
*   **Goal:** Implement comprehensive browser automation using Playwright.
*   **Approach:** Add `playwright` dependency, create a `BrowserManager` for session handling, define tool functions for navigation, clicking, input, screenshots, scrolling, JS execution, etc.

### Proposal 3: Enhance File System Tools (`file-system.ts`)
*   **Goal:** Add advanced editing capabilities like indentation-aware replacement and undo.
*   **Approach:** Add functions like `replaceInFile` (with indentation handling), `insertInFile`, `findInFile` (regex search), and potentially `undoFileChange`. Consider enhancing `writeFile` with a patch mode.

### Proposal 4: Add Sequential Thinking Tool (`sequential-thinking.ts` - New)
*   **Goal:** Implement a meta-cognitive tool for structured reasoning.
*   **Approach:** Define a tool `logStructuredThought` that accepts structured input (thought, reasoning, plan, critique) and logs it. Update system prompt to encourage its use.

### Proposal 5: Add Agent-as-a-Tool (`agent-delegation.ts` - New)
*   **Goal:** Enable hierarchical agent delegation.
*   **Approach:** Enhance `AgentManager` to create/manage sub-agents. Define a `delegateTaskToAgent` tool that creates a sub-agent, sends it a task, waits for the result, and cleans up.

### Proposal 6: Add Python REPL Tool (`python-repl.ts` - New)
*   **Goal:** Provide an interactive Python REPL.
*   **Approach:** Use `child_process.spawn` to run a persistent Python process, manage sessions, and define tools to execute code within the REPL session. Consider sandboxing for safety.

### Proposal 7: Add Code Understanding/Modification Tools (`code-enhancement.ts` - New)
*   **Goal:** Implement tools leveraging the LLM for specific code tasks.
*   **Approach:** Define tools like `explainCode`, `refactorCode`, `fixCode` that construct targeted prompts, call the `LLMService`, and return the results (explanations, suggested code, fixes).

### Proposal 8: Add Git Tool (`git.ts` - New) - *Lower Priority per User Feedback*
*   **Goal:** Provide structured access to Git operations.
*   **Approach:** Use `simple-git` library, define wrapper functions for common commands (`status`, `diff`, `add`, `commit`, `log`, `checkout`, `pull`, `push`). *Implementation deferred pending review of MCP options.*

## 3. Implementation Strategy

Implementation should proceed based on prioritized features. Leveraging existing MCP servers (e.g., for Git, Browser) should be investigated where applicable to promote modularity.