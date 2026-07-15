# Container Architecture

## Introduction

This document details the container architecture for our MCP agent system. Each agent operates within its own container, providing isolation, scalability, and consistent behavior across environments.

## Container Design Philosophy

Our container architecture follows these key principles:

1. **Single Responsibility**: Each container has a specific role and focuses on a single aspect of the development process
2. **Immutability**: Containers are treated as immutable infrastructure, with configuration passed via environment variables
3. **Portability**: Containers can run in any environment that supports Docker or Kubernetes
4. **Reproducibility**: Container builds are automated and versioned for consistent deployment
5. **Security**: Containers operate with the least privilege necessary to perform their tasks

## Base Container Structure

Each agent container is built on a common base image that includes:

```Dockerfile
FROM python:3.12-slim

# Install common dependencies
RUN apt-get update && apt-get install -y \
    git \
    curl \
    nodejs \
    npm \
    jq \
    wget \
    && rm -rf /var/lib/apt/lists/*

# Install common Python packages
RUN pip install --no-cache-dir \
    anthropic \
    mcp \
    pydantic \
    requests \
    fastapi \
    uvicorn

# Set up working directory
WORKDIR /workspace

# Create configuration directory
RUN mkdir -p /config

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Default command
CMD ["python", "/app/entrypoint.py"]
```

## Agent Container Types

### 1. Frontend Agent Container

Specializes in UI development with additional tools:

```Dockerfile
FROM base-agent:latest

# Install frontend-specific dependencies
RUN npm install -g \
    typescript \
    sass \
    prettier \
    eslint

# Install VS Code CLI for UI automation
RUN apt-get update && apt-get install -y \
    libgtk-3-0 \
    libx11-xcb1 \
    libxcb-dri3-0 \
    libdrm2 \
    libgbm1 \
    libasound2 \
    && rm -rf /var/lib/apt/lists/* \
    && curl -L "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64" -o vscode.deb \
    && dpkg -i vscode.deb \
    && rm vscode.deb

# Install headless browser for testing
RUN npm install -g puppeteer

# Set environment variables
ENV AGENT_ROLE=frontend-developer
ENV MCP_CLIENT_TYPE=cline

# Copy agent-specific scripts
COPY frontend-agent/ /app/

# Expose ports for dev server
EXPOSE 3000
```

### 2. Backend Agent Container

Focuses on server-side development:

```Dockerfile
FROM base-agent:latest

# Install backend-specific dependencies
RUN pip install --no-cache-dir \
    sqlalchemy \
    alembic \
    pytest \
    pytest-cov \
    pylint

# Install database clients
RUN apt-get update && apt-get install -y \
    postgresql-client \
    mysql-client \
    && rm -rf /var/lib/apt/lists/*

# Set environment variables
ENV AGENT_ROLE=backend-developer
ENV MCP_CLIENT_TYPE=goose

# Copy agent-specific scripts
COPY backend-agent/ /app/

# Expose ports for API server
EXPOSE 8000
```

### 3. DevOps Agent Container

Specialized for infrastructure and deployment tasks:

```Dockerfile
FROM base-agent:latest

# Install DevOps-specific tools
RUN apt-get update && apt-get install -y \
    docker.io \
    kubectl \
    terraform \
    && rm -rf /var/lib/apt/lists/*

# Install cloud CLIs
RUN curl -sSL https://sdk.cloud.google.com | bash
RUN curl "https://awscli.amazonaws.com/awscli-exe-linux-x86_64.zip" -o "awscliv2.zip" \
    && unzip awscliv2.zip \
    && ./aws/install \
    && rm -rf awscliv2.zip aws

# Set environment variables
ENV AGENT_ROLE=devops-engineer
ENV MCP_CLIENT_TYPE=aider

# Copy agent-specific scripts
COPY devops-agent/ /app/

# Add necessary volumes
VOLUME ["/var/run/docker.sock"]
```

### 4. Security Agent Container

Focused on security reviews and vulnerability scanning:

```Dockerfile
FROM base-agent:latest

# Install security tools
RUN pip install --no-cache-dir \
    bandit \
    safety \
    pyre-check

# Install additional security scanners
RUN apt-get update && apt-get install -y \
    clamav \
    nikto \
    && rm -rf /var/lib/apt/lists/*

# Set environment variables
ENV AGENT_ROLE=security-engineer
ENV MCP_CLIENT_TYPE=beeai

# Copy agent-specific scripts
COPY security-agent/ /app/
```

### 5. QA Agent Container

Specialized for testing and quality assurance:

```Dockerfile
FROM base-agent:latest

# Install QA tools
RUN pip install --no-cache-dir \
    selenium \
    pytest \
    pytest-bdd \
    behave \
    locust

# Install browser for testing
RUN apt-get update && apt-get install -y \
    chromium-browser \
    && rm -rf /var/lib/apt/lists/*

# Set environment variables
ENV AGENT_ROLE=qa-engineer
ENV MCP_CLIENT_TYPE=genkit

# Copy agent-specific scripts
COPY qa-agent/ /app/
```

## Multi-Container Composition

For a complete team environment, we use Docker Compose to orchestrate the agent containers:

```yaml
version: '3.8'

services:
  team-orchestrator:
    build: 
      context: ./orchestrator
      dockerfile: Dockerfile
    volumes:
      - shared-repo:/workspace
      - /var/run/docker.sock:/var/run/docker.sock
    environment:
      - TEAM_ID=team-a
      - TEAM_BRANCH=feature/team-a
      - CENTRAL_REPO=https://github.com/organization/project.git
    ports:
      - "8080:8080"
    depends_on:
      - redis
      - postgres
      
  frontend-agent:
    build:
      context: .
      dockerfile: frontend-agent/Dockerfile
    volumes:
      - shared-repo:/workspace
    environment:
      - AGENT_ID=frontend-1
      - TEAM_ID=team-a
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
    depends_on:
      - team-orchestrator
    
  backend-agent:
    build:
      context: .
      dockerfile: backend-agent/Dockerfile
    volumes:
      - shared-repo:/workspace
    environment:
      - AGENT_ID=backend-1
      - TEAM_ID=team-a
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
    depends_on:
      - team-orchestrator
      - postgres
    
  devops-agent:
    build:
      context: .
      dockerfile: devops-agent/Dockerfile
    volumes:
      - shared-repo:/workspace
      - /var/run/docker.sock:/var/run/docker.sock
    environment:
      - AGENT_ID=devops-1
      - TEAM_ID=team-a
      - AWS_ACCESS_KEY_ID=${AWS_ACCESS_KEY_ID}
      - AWS_SECRET_ACCESS_KEY=${AWS_SECRET_ACCESS_KEY}
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
    depends_on:
      - team-orchestrator
      
  security-agent:
    build:
      context: .
      dockerfile: security-agent/Dockerfile
    volumes:
      - shared-repo:/workspace
    environment:
      - AGENT_ID=security-1
      - TEAM_ID=team-a
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
    depends_on:
      - team-orchestrator
      
  qa-agent:
    build:
      context: .
      dockerfile: qa-agent/Dockerfile
    volumes:
      - shared-repo:/workspace
    environment:
      - AGENT_ID=qa-1
      - TEAM_ID=team-a
      - ANTHROPIC_API_KEY=${ANTHROPIC_API_KEY}
    depends_on:
      - team-orchestrator
      
  redis:
    image: redis:alpine
    ports:
      - "6379:6379"
    volumes:
      - redis-data:/data
      
  postgres:
    image: postgres:13
    environment:
      - POSTGRES_USER=postgres
      - POSTGRES_PASSWORD=postgres
      - POSTGRES_DB=agentdb
    ports:
      - "5432:5432"
    volumes:
      - postgres-data:/var/lib/postgresql/data

volumes:
  shared-repo:
  redis-data:
  postgres-data:
```

## Kubernetes Deployment

For production environments, we deploy agent containers to Kubernetes:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: team-a-agents
  namespace: agent-system
spec:
  replicas: 1
  selector:
    matchLabels:
      team: team-a
  template:
    metadata:
      labels:
        team: team-a
    spec:
      volumes:
        - name: shared-repo
          persistentVolumeClaim:
            claimName: team-a-repo-pvc
      containers:
      - name: frontend-agent
        image: registry.example.com/frontend-agent:latest
        volumeMounts:
          - name: shared-repo
            mountPath: /workspace
        env:
          - name: AGENT_ID
            value: "frontend-1"
          - name: TEAM_ID
            value: "team-a"
          - name: ANTHROPIC_API_KEY
            valueFrom:
              secretKeyRef:
                name: api-keys
                key: anthropic-api-key
      
      - name: backend-agent
        image: registry.example.com/backend-agent:latest
        volumeMounts:
          - name: shared-repo
            mountPath: /workspace
        env:
          - name: AGENT_ID
            value: "backend-1"
          - name: TEAM_ID
            value: "team-a"
          - name: ANTHROPIC_API_KEY
            valueFrom:
              secretKeyRef:
                name: api-keys
                key: anthropic-api-key
      
      # Additional agent containers...
```

## Container Networking

Agents communicate through a combination of:

1. **Shared Volume**: The /workspace volume provides a common filesystem for team code access
2. **HTTP/REST APIs**: Agents expose and consume REST APIs for direct communication
3. **Message Queues**: Redis is used for asynchronous message passing between agents
4. **Git Operations**: Git is used for code-based communication and integration

## Container Security

To ensure secure operation:

1. **Least Privilege**: Containers run with minimal permissions
2. **Secret Management**: API keys and credentials are injected via environment variables
3. **Network Policies**: Kubernetes network policies restrict inter-container communication
4. **Image Scanning**: Container images are scanned for vulnerabilities before deployment
5. **Resource Limits**: CPU and memory limits prevent resource exhaustion attacks

## Monitoring and Logging

Each container is configured for observability:

1. **Centralized Logging**: Logs are collected via a sidecar and sent to a central log store
2. **Metrics Endpoint**: Each container exposes a /metrics endpoint for Prometheus scraping
3. **Health Checks**: HTTP health checks enable automated restart of unhealthy containers
4. **Tracing**: Distributed tracing for cross-container operations

## Container Lifecycle Management

Containers follow a defined lifecycle:

1. **Initialization**: Container starts and loads configuration
2. **Repository Setup**: Git repository is cloned or updated
3. **MCP Client Start**: Appropriate MCP client (Cline, Goose, etc.) is started
4. **Task Execution**: Agent performs assigned tasks
5. **Result Commit**: Changes are committed to the repository
6. **Graceful Shutdown**: Container saves state and terminates cleanly

## Conclusion

This container architecture provides a scalable, secure, and flexible foundation for our MCP agent system. By containerizing each agent, we ensure consistent behavior across environments and enable easy scaling of the system as needs grow.
