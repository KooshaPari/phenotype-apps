# AGSLAG Platform: Complete Architecture Overview

This document provides a comprehensive overview of the AGSLAG platform's architecture, covering the frontend, backend, data storage, and integrations.

## 1. Frontend Architecture

The frontend of the AGSLAG platform is built using **React** and **Next.js**, following the Geist design language for a clean and modern user interface. Key components include:

- **Jarvis Next-Gen Rich Chat Interface**: 
  - Supports composable message types (text, code, images, files, charts, diagrams, interactive widgets).
  - Provides real-time dynamic visuals and sci-fi inspired effects.
  - Implements a message model that delineates between user messages, AI responses, and platform events.

### Key Files:
- `src/app/jarvis/chat/page.tsx`: Implements the chat page.
- `src/lib/api-client.ts`: Handles API communication.
- `src/components/jarvis/MessageRenderer.tsx`: Renders messages based on type.

## 2. Backend Architecture

The backend is built using **Node.js** and **Express**, with a focus on modularity and scalability. Key components include:

- **MCP Server**: Manages API requests and integrates with various services.
- **Routing**: Defined in the `src/routes/` directory, handling requests for agents, AI, budgets, projects, and more.
- **Controllers**: Implement business logic for handling requests, such as `agentController.ts` and `budgetController.ts`.
- **Services**: Encapsulate business logic and data access, such as `BudgetService.ts` and `NotificationService.ts`.

### Key Files:
- `src/index.ts`: Entry point for the server, sets up middleware and routes.
- `src/routes/agentRoutes.ts`: Manages agent-related API endpoints.
- `src/services/BudgetService.ts`: Handles budget management.

## 3. Data Storage

The AGSLAG platform uses **MongoDB** for data storage, with Mongoose as the ODM. Key models include:

- **Agent Model**: Represents agents with properties like `name`, `type`, `status`, `capabilities`, and `last_heartbeat`.
- **Budget Model**: Represents budgets with properties like `name`, `amount`, `spent`, `status`, and `alertThresholds`.

### Key Files:
- `src/models/Agent.ts`: Defines the agent schema and model.
- `src/models/Budget.ts`: Defines the budget schema and model.

## 4. Integrations

The AGSLAG platform integrates with various external services and protocols:

- **NATS**: For messaging and communication between services.
- **Redis**: For caching and performance optimization.
- **External APIs**: For AI model interactions and other functionalities.

## 5. Conclusion

The AGSLAG platform is designed to be modular, scalable, and efficient, leveraging modern technologies and best practices. This architecture overview provides a foundation for future enhancements and optimizations, ensuring the platform can adapt to evolving needs and technologies.
