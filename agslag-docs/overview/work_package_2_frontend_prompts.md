# Work Package 2: Frontend UI/UX & Prompt Engineering

**Target Agent/Developer:** Agent 2 (e.g., Frontend Specialist / Prompt Engineer)

**Objective:** Overhaul the frontend UI with a neoglassmorphic design, implement new pages and interactive components for workflows, project creation, analytics, and graphs, fix existing UI issues, and develop the core prompt engineering framework.

**Key Areas:** `agslag/mcp-dashboard-next/src/`, `src/prompts/`, `Documentation/`

**Detailed Instructions:**

1.  **Neoglassmorphic UI Overhaul (`agslag/mcp-dashboard-next/src/`):**
    *   Develop a new CSS framework/theme incorporating neoglassmorphism, 3D effects, glassmorphism, and Vercel/web3-inspired styles. Apply this theme across the entire dashboard. Use modern CSS techniques (e.g., CSS variables, potentially Tailwind CSS if appropriate for the project).
    *   Refactor existing components (`agslag/mcp-dashboard-next/src/components/`) for the new design system, ensuring consistency and reusability.
    *   Implement interactive JavaScript/TypeScript elements for enhanced UX (e.g., smooth transitions, micro-interactions, loading states). Leverage React/Next.js capabilities effectively.
    *   Ensure responsiveness across different screen sizes using media queries and flexible layouts (e.g., Flexbox, Grid).
    *   Add more cosmetic elements and "eye candy" as requested, focusing on subtle animations and visual hierarchy.
    *   **Fix Broken Pages:** Specifically address and fix the Network page (`src/app/network/page.tsx`?) and rework the main Dashboard to be more comprehensive yet customizable (implement modular widgets using a library or custom logic).

2.  **New Feature Pages & Components (`agslag/mcp-dashboard-next/src/app/`, `agslag/mcp-dashboard-next/src/components/`):**
    *   **Workflow Designer:** Create a new page/section (`/workflows`?) with a visual workflow builder interface (consider libraries like React Flow) allowing drag-and-drop creation of tasks, dependencies, and conditions. Implement components for viewing/selecting workflow templates fetched from the backend.
    *   **Project Creation/Orchestration:** Build a new page (`/create-project`?) for creating projects/orchestrations. Include forms for enumerating project details, uploading documentation (interfacing with backend), viewing/editing suggested team structures, and initiating agent orchestrations (calling the backend API/MCP tool from Package 1).
    *   **Cost/Usage Analytics:** Create a new analytics page (`agslag/mcp-dashboard-next/src/app/analytics/page.tsx`) to display metrics fetched from the backend API. Implement charts and visualizations (e.g., using Chart.js, Recharts) for cost/time/usage exploration, predictions, and recommendations. Show live and projected costs during creation/runtime.
    *   **Interactive Graphs:** Develop components (potentially using libraries like Cytoscape.js, Vis.js, or D3) to visualize:
        *   Org/Team/Project structure (implement node/edge editing capabilities that call backend APIs).
        *   Project knowledge maps.
        *   Agent/Team knowledge maps.
        *   Resource graphs and other PM visualizations.
        *   Integrate these graphs into relevant dashboard pages (e.g., project overview, team pages).
    *   **Modular Dashboard:** Implement a system (e.g., using local storage or user preferences stored in the DB) for users to add/remove/rearrange widgets on the main dashboard.

3.  **Richer Comms UI (`agslag/mcp-dashboard-next/src/app/communications/`):**
    *   Enhance the communication interface (`Message.tsx`, etc.) to support new features like reactions, threading indicators, agent statuses, and progress bars (mimicking Discord integrations where appropriate). Fetch progress data from backend/MCP tools via WebSocket or API polling.

4.  **Core Prompt Engineering Framework (`src/prompts/`, `Documentation/`):**
    *   Develop the foundational prompt engineering framework based on the previous 5-part plan (focusing on Package 1: Core PE Framework from the initial plan).
    *   Create structured communication protocols (`src/prompts/collaboration.ts`?), collaborative problem-solving frameworks, conflict resolution processes, and knowledge transfer protocols within the prompt structures. Define clear roles and responsibilities for agents within prompts.
    *   Develop templates and best practices for guiding agents in using the `team-communications` MCP and other core tools (`src/tools/`). Ensure prompts clearly specify expected inputs and outputs for tool calls.
    *   Write comprehensive documentation (`Documentation/`) for the prompt framework, including examples and usage guidelines for different agent roles.

5.  **Agent Spawning UI (`agslag/mcp-dashboard-next/src/app/` - new page):**
    *   Design the UI elements within the new Project Creation/Orchestration page to configure agent parameters (model, system prompt, assigned MCPs, initial task) and trigger the programmatic agent spawning MCP tool developed in Package 1 via the backend API.
