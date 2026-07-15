# VSCode Extensions UI Automation Agent - Scaffold Design

This document outlines the scaffold for an orchestrated agent that combines API calls, Playwright UI automation, Omniparser Autogui MCP, and native OS automation to control RooCode, Cline, and Augment Code extensions in VSCode on Mac and Windows.

---

## 1. Architecture Overview

```mermaid
flowchart TD
    UserRequest["User / API Request"] --> Orchestrator["Automation Orchestrator Agent"]

    subgraph Orchestrator
        Orchestrator --> API_Module["API & Command Palette Module"]
        Orchestrator --> Playwright_Module["Playwright UI Automation Module"]
        Orchestrator --> Omniparser_Module["Omniparser Autogui Module"]
        Orchestrator --> Native_Module["Native OS Automation Module"]
    end

    API_Module --> VSCode["VSCode Instance"]
    Playwright_Module --> VSCode
    Omniparser_Module --> VSCode
    Native_Module --> VSCode
```

---

## 2. Core Components

### a. **Automation Orchestrator Agent**

- Receives high-level commands (e.g., "Run test suite", "Open RooCode panel", "Trigger Cline agent").
- Decides which module(s) to invoke based on task, environment, and fallback logic.
- Handles retries, error handling, and logging.
- Provides a unified API interface.

### b. **API & Command Palette Module**

- Invokes VSCode commands via CLI (`code --command`) or extension APIs.
- Fastest and most reliable when supported.

### c. **Playwright UI Automation Module**

- Launches or attaches to VSCode Electron app.
- Automates UI interactions: clicks, inputs, menu navigation.
- Used when API is insufficient.

### d. **Omniparser Autogui Module**

- Performs pixel/screen-based automation.
- Useful for complex UI flows or when DOM access is limited.

### e. **Native OS Automation Module**

- Uses AppleScript/Automator (Mac) or PowerShell/AutoHotkey (Windows).
- Manages window focus, resizing, and menu navigation.

---

## 3. Workflow Outline

1. **Receive Task Request**
2. **Attempt API/Command Palette Automation**
3. **If insufficient, invoke Playwright UI Automation**
4. **If still blocked, use Omniparser Autogui for pixel-based control**
5. **Use Native OS Automation for window/menu management as needed**
6. **Return success/failure and logs**

---

## 4. Implementation Plan

- **Phase 1:** Scaffold Orchestrator with stub modules and unified API.
- **Phase 2:** Implement API/Command Palette Module.
- **Phase 3:** Develop Playwright scripts for key UI flows.
- **Phase 4:** Integrate Omniparser Autogui workflows.
- **Phase 5:** Add native OS automation scripts.
- **Phase 6:** Combine, test, and optimize fallback logic.

---

*This scaffold enables a robust, multi-layered automation agent for controlling VSCode extensions across platforms.*
