# AGSLAG Documentation Index

This directory contains detailed documentation for the AGSLAG project and its integration with the MCP framework.

## Core Concepts & Overviews
- [AGSLAG Platform Whitepaper](./AGSLAG_Platform_Whitepaper.md): Comprehensive overview generated from documentation.
- [Executive Summary](./00_executive_summary.md): High-level summary of the project.
- [Project Overview](./project_overview.md): Vision, capabilities, and goals of the AI platform.
- [System Architecture](./system_architecture.md): Technical architecture details of the platform. (See also: [Architecture Enhancement Plan](./architecture_enhancement_plan.md))
- [Architecture Enhancement Plan](./architecture_enhancement_plan.md): Details on planned architectural improvements and future considerations.
- [Tech Company Processes](./tech_company_processes.md): Replication of real-world tech company operations.
## Subdirectories
- [Agent Comparison](./agent_comparison/): Analysis and reports comparing different agents (Jarvis, Augment SWE-Bench). Includes `jarvis_enhancement_plan.md`.
- [Architecture](./architecture/): Detailed architectural documents and diagrams.
- [Docs](./docs/): General documentation files.
- [Improvement](./improvement/): Documents related to project improvements and refactoring.
- [LLM Context](./llm_context/): Information regarding LLM context management and optimization.
- [Research](./research/): Research notes and findings related to the project.
- [Testing](./testing/): Documentation related to testing strategies and results (Note: See also Quality Assurance section).
- [Tools](./tools/): Documentation related to specific tools or tool categories.
- [Integrations](./integrations/): Examples and guides for integrating with external systems.
    - [Slack Corporate Agent Integration](./integrations/slack_corporate_agent_integration.md): Example architecture for Slack integration.

## MCP Framework and Integration
- [MCP Frameworks](./mcp_frameworks.md): Overview of the MCP framework components used (LiteMCP, ws-mcp).
- [MCP Agent Communication](./mcp_agent_communication.md): How agents communicate via MCP.
- [MCP Orchestration Framework](./mcp_orchestration_framework.md): Details on how workflows are managed.
- [MCP Integration Guide](./mcp_integration_guide.md): Guide for integrating with the MCP framework.
- [MCP Security and Authentication](./mcp_security_and_authentication.md): Security aspects of the MCP integration.
- [MCP Codebase Analysis](./mcp_codebase_analysis.md): Analysis of the MCP codebase.
- [MCP Servers Overview](./mcp-servers-overview.md): Summary of available MCP servers.
- [Implementing Custom MCP Servers](./implementing-custom-mcp-servers.md): Guide on creating new MCP servers.
- [MCP CLI](./mcp-cli.md): Information about the MCP command-line interface.
- [Structured Communication Protocol](./structured_communication_protocol.md): Details on the communication format.
- [Thread Notifications](./thread_notifications.md): How notifications work within threads.
- [MCPLB](./mcplb.md): Information about the MCP Load Balancer.
- [CentralMind Gateway](./centralmind-gateway.md): Details on the CentralMind gateway component.

## Agent and Workflow Management
- [Agent Roles and Workflows](./agent_roles_and_workflows.md): Definition of agent roles and their processes.
- [Agent Lifecycle Diagram](./agent_lifecycle_diagram.md): Visual representation of the agent lifecycle.
- [Standard Task Execution Workflow](./standard_task_execution_workflow.md): The default workflow for tasks.
- [Collaborative Problem Solving Framework](./collaborative_problem_solving_framework.md): Framework for agents solving problems together.
- [Team Coordination Protocols](./team_coordination_protocols.md): How agent teams coordinate.
- [Conflict Resolution Process](./conflict_resolution_process.md): Process for handling conflicts between agents.
- [Dual Agent Architecture Plan](./dual_agent_architecture_plan.md): Plan for a dual-agent setup.
- [VSCode UI Automation Agent Scaffold](./vscode_ui_automation_agent_scaffold.md): Scaffolding for UI automation agent.
## AI and LLM Integration
- [Gemini API Integration](./gemini_api_integration.md): Details on integrating with Google's Gemini API via VertexAI.
- [Gemini Integration Patterns](./gemini_integration_patterns.md): Patterns for using Gemini within the system.
- [Gemini Integration Diagram](./gemini_integration_diagram.md): Visual diagram of the Gemini integration.
- [Large Context Window Optimization Guide](./large_context_window_optimization_guide.md): Strategies for optimizing LLM context usage.
- [Core Prompt Engineering Framework](./core_prompt_engineering_framework.md): Framework for designing effective prompts.
- [Complex Reasoning Frameworks](./complex_reasoning_frameworks.md): Frameworks for enabling complex reasoning in agents.

## Knowledge Management
- [Knowledge Management Strategies](./knowledge_management_strategies.md): How knowledge is captured and utilized.
- [MCP Knowledge Graph](./mcp_knowledge_graph.md): Details on the knowledge graph implementation.

## Project and Development Process
- [Digital Product Development Lifecycle](./digital_product_lifecycle.md): Overview of the end-to-end process.
- [Requirements](./requirements.md): Project requirements.
- [User Stories](./user_stories.md): User stories for the platform.
- [Use Cases](./use_cases.md): Specific use cases for the platform.
- [Project Initialization Plan](./project_initialization_plan.md): Plan for starting new projects.
- [Implementation Roadmap](./implementation_roadmap.md): Roadmap for platform development.
- [Project Management Framework](./project_management_framework.md): How projects are managed within the platform.
- [Work Package 1: Backend](./work_package_1_backend.md): Details on the backend work package.
- [Work Package 2: Frontend & Prompts](./work_package_2_frontend_prompts.md): Details on the frontend and prompt engineering work package.
- [End-to-End Process Connections](./end_to_end_process_connections.md): How different processes connect.
- [Progress Evaluation Report](./progress_evaluation_report.md): Report on project progress evaluation.
- [Cloned Repo Review](./cloned_repo_review.md): Review process for cloned repositories.
- [Code Analysis Frameworks](./code_analysis_frameworks.md): Frameworks used for code analysis.
- [Software Engineering Process Diagram](./software_engineering_process_diagram.md): Diagram of the development process.
- [Specification & Atomic Decomposition Process](./specification_process.md): Process for detailed scope definition.
- [Specification Process Report](./specification_process_report.md): Analysis of the specification process integration.
- [Digital Product Development Lifecycle](./digital_product_lifecycle.md): Overview of the end-to-end process.
- [Product Management Workflow](./product_management_workflow.md): Role and process for Product Managers.
- [UI/UX Design Workflow](./ui_ux_design_workflow.md): Process for the UI/UX Design team.
- [Marketing Product Workflow](./marketing_product_workflow.md): Role and process for the Marketing team.
- [Sales Product Workflow](./sales_product_workflow.md): Role and process for the Sales team.
- [Customer Support Workflow](./customer_support_workflow.md): Process for the Customer Support team.
- [DevOps Workflow](./devops_workflow.md): Role and process for the DevOps team.

## Quality Assurance
- [Quality Assurance Protocols](./quality_assurance_protocols.md): Protocols for functional, accessibility, and security testing.
- [Testing Strategy](./testing/testing_strategy.md): Overall testing strategy document (from testing/ subdirectory).
## Specific MCP Server Documentation (Alphabetical)
- [AlibabaCloud TableStore MCP Server](./alibabacloud-tablestore-mcp-server.md)
- [CentralMind Gateway](./centralmind-gateway.md)
- [DBHub](./dbhub.md)
- [DuckDB MCP Server](./duckdb-mcp-server.md)
- [Elasticsearch MCP Server](./elasticsearch-mcp-server.md)
- [Fetch MCP Server](./fetch-mcp-server.md)
- [JDBC MCP](./jdbc-mcp.md)
- [MCP BigQuery Server](./mcp-bigquery-server.md)
- [MCP BigQuery Server Guide](./mcp-bigquery-server-guide.md)
- [MCP ClickHouse](./mcp-clickhouse.md)
- [MCP CLI](./mcp-cli.md)
- [MCP Mongo Server](./mcp-mongo-server.md)
- [MCP Server MySQL](./mcp-server-mysql.md)
- [MCP Server Trino](./mcp-server-trino.md)
- [MCPLB](./mcplb.md)
- [MySQL MCP Server](./mysql_mcp_server.md)
- [Supabase MCP Server](./supabase-mcp-server.md)
- [Wren Engine](./wren-engine.md)
## Analysis and Reports
- [Awesome MCP Servers Analysis](./awesome_mcp_servers_analysis.md)
- [Awesome MCP Servers Analysis Part 2](./awesome_mcp_servers_analysis_part2.md)
- [External Integrations Report](./external_integrations_report.md)
- [External Integrations Report 2](./external_integrations_report_2.md)
- [External Integrations Report 3](./external_integrations_report_3.md)
- [Manus Report](./Manus_report.md)
- [Replicate Manus](./replicate_manus.md)

## Diagrams (Source Files)
- [Agent Lifecycle Diagram](./agent_lifecycle_diagram.md)
- [Gemini Integration Diagram](./gemini_integration_diagram.md)
- [Product Packaging Diagram](./product_packaging_diagram.md)
- [Service Integration Diagram](./service_integration_diagram.md)
- [Software Engineering Process Diagram](./software_engineering_process_diagram.md)
- [Team Structure Diagram](./team_structure_diagram.md)
*Note: These links point to the source Markdown files used to generate diagrams. Corresponding PDF versions may also exist in the directory.*

## Other
- [Documentation Generation Templates](./documentation_generation_templates.md)
- [Documentation Work Plan](./documentation_work_plan.md)
- [Third Party Integration](./third_party_integration.md)

<!-- Note about PDF files moved to the Diagrams section -->