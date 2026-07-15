# Phase 1: Local Environment Setup (Weeks 1-4)

This document details the implementation of the local environment components for the MCP agentic architecture.

## 1.1 UI Automation Framework

The UI automation framework enables programmatic control of VS Code extensions through tools like Playwright or Puppeteer.

### Implementation Example

```javascript
// Example automation script for Cline
const { chromium } = require('playwright');

async function automateVSCode() {
  const browser = await chromium.launch({
    headless: false,  // Initially develop with UI visible
    slowMo: 50        // Slow down for debugging
  });
  
  const context = await browser.newContext();
  const page = await context.newPage();
  
  // Navigate to VS Code Web or connect to local VS Code instance
  await page.goto('https://vscode.dev/');
  
  // Wait for VS Code to load
  await page.waitForSelector('.monaco-editor', { timeout: 30000 });
  
  // Interact with Cline extension
  await page.click('.activity-bar-item[title*="Cline"]');
  
  // Configure and use Cline
  // ...
}
```

### Technical Considerations

- **Browser Selection**: Playwright supports Chromium, Firefox, and WebKit
- **Headless Mode**: Start with visible mode for debugging, transition to headless for production
- **Selectors**: Use stable selectors that won't break with VS Code updates
- **Error Handling**: Implement robust error handling for UI interactions
- **State Management**: Track application state during automation

## 1.2 VS Code Extension Container

Containerized VS Code with pre-installed extensions provides consistent environments for local testing and development.

### Dockerfile Implementation

```dockerfile
FROM codercom/code-server:latest

# Install VS Code extensions
RUN code-server --install-extension cline.cline
RUN code-server --install-extension RooVeterinaryInc.roo-cline

# Install UI automation tools
RUN apt-get update && apt-get install -y \
    xvfb \
    libgbm-dev \
    libnss3 \
    libatk1.0-0 \
    libatk-bridge2.0-0 \
    libcups2 \
    libdrm2 \
    libxkbcommon0 \
    libxcomposite1 \
    libxdamage1 \
    libxrandr2 \
    libgbm1 \
    libasound2

# Install Node.js and Playwright
RUN curl -sL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get install -y nodejs
RUN npm install -g playwright

# Copy automation scripts
COPY scripts/ /app/scripts/

WORKDIR /app
ENTRYPOINT ["node", "scripts/automate.js"]
```

### Container Configuration

- **Base Image**: Uses Code Server for web-accessible VS Code
- **Extensions**: Installs Cline and Roo Code extensions
- **Dependencies**: Includes all UI automation dependencies
- **Entrypoint**: Runs automation script on startup

## 1.3 Aider Integration

Aider is a CLI-based AI pair programming tool that can be containerized for consistent deployment.

### Dockerfile Implementation

```dockerfile
FROM python:3.12-slim

# Install Git
RUN apt-get update && apt-get install -y git

# Install Aider
RUN pip install aider-chat

# Create working directory
WORKDIR /workspace

# Copy agent scripts
COPY scripts/ /app/scripts/

# Environment variables
ENV ANTHROPIC_API_KEY=""
ENV AGENT_ROLE="developer"
ENV TEAM_BRANCH=""
ENV REPO_URL=""

# Start agent
ENTRYPOINT ["python", "/app/scripts/aider_agent.py"]
```

### Agent Script Implementation

```python
# /app/scripts/aider_agent.py
import os
import sys
import subprocess
import json
from pathlib import Path

def initialize_repository():
    """Clone the repository and checkout the team branch."""
    repo_url = os.environ.get('REPO_URL')
    team_branch = os.environ.get('TEAM_BRANCH')
    
    if not repo_url or not team_branch:
        print("ERROR: REPO_URL and TEAM_BRANCH environment variables must be set")
        sys.exit(1)
    
    subprocess.run(['git', 'clone', repo_url, '.'])
    subprocess.run(['git', 'checkout', '-b', team_branch])

def run_aider_with_task(task_description):
    """Run Aider with the given task."""
    anthropic_key = os.environ.get('ANTHROPIC_API_KEY')
    
    if not anthropic_key:
        print("ERROR: ANTHROPIC_API_KEY environment variable must be set")
        sys.exit(1)
    
    # Set up configuration
    config_dir = Path.home() / '.aider'
    config_dir.mkdir(exist_ok=True)
    
    with open(config_dir / 'config.yml', 'w') as f:
        f.write(f"""
model: claude-3-sonnet-20240229
anthropic_api_key: {anthropic_key}
auto_commits: true
git_autocommit_on_save: true
""")
    
    # Run Aider with the task
    process = subprocess.Popen(
        ['aider', '--message', task_description],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    
    # Monitor and interact with Aider process
    # ...

if __name__ == "__main__":
    # Initialize repository
    initialize_repository()
    
    # Get task from environment or config file
    task_file = Path('/app/task/task.json')
    if task_file.exists():
        with open(task_file, 'r') as f:
            task_data = json.load(f)
            task_description = task_data.get('description', '')
    else:
        task_description = os.environ.get('TASK_DESCRIPTION', '')
    
    if not task_description:
        print("ERROR: No task description provided")
        sys.exit(1)
    
    # Run Aider with the task
    run_aider_with_task(task_description)
```

### Integration Considerations

- **Environment Variables**: Control behavior through environment variables
- **Git Integration**: Automatic repository initialization and branch checkout
- **Configuration Management**: Dynamic configuration generation based on environment
- **Task Processing**: Accept tasks from environment or configuration file
- **Process Monitoring**: Monitor and interact with long-running Aider process

## Next Steps

After implementing Phase 1, the following components should be available:

1. UI automation scripts for Cline and Roo Code extensions
2. Containerized VS Code environment with required extensions
3. Containerized Aider agent with Git integration
4. Basic task assignment and processing capabilities

These components form the foundation for the local environment setup, which will be expanded in subsequent phases.
