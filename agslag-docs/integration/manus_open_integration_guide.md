# Manus-Open Integration Guide

## Overview

This guide details the process of integrating manus-open, a containerized sandbox environment for AI agents, into the AGSLAG MCP Integration system. Manus-Open provides terminal and browser interaction capabilities that can enhance AGSLAG's ability to interact with external systems.

## Prerequisites

- AGSLAG MCP Integration system set up and running
- manus-open repository cloned and accessible
- Docker installed
- Node.js and npm installed
- TypeScript development environment configured

## Integration Steps

### 1. Create Terminal and Browser Interaction Interface

First, we need to define an interface that represents the terminal and browser interaction capabilities we want to expose from manus-open:

```typescript
// interfaces/terminal_browser_interaction.ts
export interface TerminalAndBrowserInteraction {
  /**
   * Execute a terminal command
   */
  executeTerminalCommand(params: {
    command: string;
    workingDirectory?: string;
    timeout?: number;
  }): Promise<{
    stdout: string;
    stderr: string;
    exitCode: number;
  }>;
  
  /**
   * Create a terminal session
   */
  createTerminalSession(): Promise<{
    sessionId: string;
  }>;
  
  /**
   * Write to a terminal session
   */
  writeToTerminalSession(params: {
    sessionId: string;
    input: string;
  }): Promise<{
    success: boolean;
  }>;
  
  /**
   * Read from a terminal session
   */
  readFromTerminalSession(params: {
    sessionId: string;
  }): Promise<{
    output: string;
  }>;
  
  /**
   * Close a terminal session
   */
  closeTerminalSession(params: {
    sessionId: string;
  }): Promise<{
    success: boolean;
  }>;
  
  /**
   * Execute a browser action
   */
  executeBrowserAction(params: {
    action: string;
    url?: string;
    selector?: string;
    value?: string;
    timeout?: number;
  }): Promise<{
    success: boolean;
    result?: any;
    screenshot?: string;
  }>;
  
  /**
   * Create a browser session
   */
  createBrowserSession(): Promise<{
    sessionId: string;
  }>;
  
  /**
   * Navigate to a URL in a browser session
   */
  navigateToBrowserUrl(params: {
    sessionId: string;
    url: string;
  }): Promise<{
    success: boolean;
    title?: string;
  }>;
  
  /**
   * Take a screenshot in a browser session
   */
  takeBrowserScreenshot(params: {
    sessionId: string;
    selector?: string;
  }): Promise<{
    screenshot: string;
  }>;
  
  /**
   * Close a browser session
   */
  closeBrowserSession(params: {
    sessionId: string;
  }): Promise<{
    success: boolean;
  }>;
  
  /**
   * Upload a file
   */
  uploadFile(params: {
    filePath: string;
    content: string | Buffer;
  }): Promise<{
    success: boolean;
    url?: string;
  }>;
  
  /**
   * Download a file
   */
  downloadFile(params: {
    url: string;
    filePath?: string;
  }): Promise<{
    success: boolean;
    filePath?: string;
    content?: string | Buffer;
  }>;
}
```

### 2. Create Manus-Open Adapter

Next, implement the adapter that connects to manus-open's API:

```typescript
// adapters/manus_open_adapter.ts
import { TerminalAndBrowserInteraction } from '../interfaces/terminal_browser_interaction';
import axios from 'axios';
import * as WebSocket from 'ws';

export class AgslagManusOpenAdapter implements TerminalAndBrowserInteraction {
  private baseUrl: string;
  private terminalSessions: Map<string, WebSocket> = new Map();
  private terminalOutputs: Map<string, string> = new Map();
  
  constructor(config: {
    baseUrl: string;
  }) {
    this.baseUrl = config.baseUrl;
  }
  
  /**
   * Execute a terminal command
   */
  async executeTerminalCommand(params: {
    command: string;
    workingDirectory?: string;
    timeout?: number;
  }): Promise<{
    stdout: string;
    stderr: string;
    exitCode: number;
  }> {
    try {
      const response = await axios.post(`${this.baseUrl}/terminal/execute`, {
        command: params.command,
        working_directory: params.workingDirectory,
        timeout: params.timeout || 30000
      });
      
      return {
        stdout: response.data.stdout,
        stderr: response.data.stderr,
        exitCode: response.data.exit_code
      };
    } catch (error) {
      console.error('[AgslagManusOpenAdapter] Error executing terminal command:', error);
      throw new Error(`Failed to execute terminal command: ${error.message}`);
    }
  }
  
  /**
   * Create a terminal session
   */
  async createTerminalSession(): Promise<{
    sessionId: string;
  }> {
    try {
      const response = await axios.post(`${this.baseUrl}/terminal/session`);
      const sessionId = response.data.session_id;
      
      // Create WebSocket connection for the terminal session
      const ws = new WebSocket(`ws://${new URL(this.baseUrl).host}/terminal/${sessionId}`);
      
      // Set up event handlers
      ws.on('open', () => {
        console.log(`[AgslagManusOpenAdapter] WebSocket connection opened for session ${sessionId}`);
      });
      
      ws.on('message', (data: WebSocket.Data) => {
        const output = this.terminalOutputs.get(sessionId) || '';
        this.terminalOutputs.set(sessionId, output + data.toString());
      });
      
      ws.on('error', (error) => {
        console.error(`[AgslagManusOpenAdapter] WebSocket error for session ${sessionId}:`, error);
      });
      
      ws.on('close', () => {
        console.log(`[AgslagManusOpenAdapter] WebSocket connection closed for session ${sessionId}`);
        this.terminalSessions.delete(sessionId);
      });
      
      // Store the WebSocket connection
      this.terminalSessions.set(sessionId, ws);
      
      return {
        sessionId
      };
    } catch (error) {
      console.error('[AgslagManusOpenAdapter] Error creating terminal session:', error);
      throw new Error(`Failed to create terminal session: ${error.message}`);
    }
  }
  
  /**
   * Write to a terminal session
   */
  async writeToTerminalSession(params: {
    sessionId: string;
    input: string;
  }): Promise<{
    success: boolean;
  }> {
    try {
      const ws = this.terminalSessions.get(params.sessionId);
      
      if (!ws) {
        throw new Error(`Terminal session ${params.sessionId} not found`);
      }
      
      ws.send(params.input);
      
      return {
        success: true
      };
    } catch (error) {
      console.error('[AgslagManusOpenAdapter] Error writing to terminal session:', error);
      throw new Error(`Failed to write to terminal session: ${error.message}`);
    }
  }
  
  /**
   * Read from a terminal session
   */
  async readFromTerminalSession(params: {
    sessionId: string;
  }): Promise<{
    output: string;
  }> {
    try {
      const output = this.terminalOutputs.get(params.sessionId) || '';
      
      // Clear the output after reading
      this.terminalOutputs.set(params.sessionId, '');
      
      return {
        output
      };
    } catch (error) {
      console.error('[AgslagManusOpenAdapter] Error reading from terminal session:', error);
      throw new Error(`Failed to read from terminal session: ${error.message}`);
    }
  }
  
  /**
   * Close a terminal session
   */
  async closeTerminalSession(params: {
    sessionId: string;
  }): Promise<{
    success: boolean;
  }> {
    try {
      const ws = this.terminalSessions.get(params.sessionId);
      
      if (ws) {
        ws.close();
        this.terminalSessions.delete(params.sessionId);
        this.terminalOutputs.delete(params.sessionId);
      }
      
      await axios.delete(`${this.baseUrl}/terminal/session/${params.sessionId}`);
      
      return {
        success: true
      };
    } catch (error) {
      console.error('[AgslagManusOpenAdapter] Error closing terminal session:', error);
      throw new Error(`Failed to close terminal session: ${error.message}`);
    }
  }
  
  /**
   * Execute a browser action
   */
  async executeBrowserAction(params: {
    action: string;
    url?: string;
    selector?: string;
    value?: string;
    timeout?: number;
  }): Promise<{
    success: boolean;
    result?: any;
    screenshot?: string;
  }> {
    try {
      const response = await axios.post(`${this.baseUrl}/browser/action`, {
        action: params.action,
        url: params.url,
        selector: params.selector,
        value: params.value,
        timeout: params.timeout || 30000
      });
      
      return {
        success: response.data.success,
        result: response.data.result,
        screenshot: response.data.screenshot
      };
    } catch (error) {
      console.error('[AgslagManusOpenAdapter] Error executing browser action:', error);
      throw new Error(`Failed to execute browser action: ${error.message}`);
    }
  }
  
  /**
   * Create a browser session
   */
  async createBrowserSession(): Promise<{
    sessionId: string;
  }> {
    try {
      const response = await axios.post(`${this.baseUrl}/browser/session`);
      
      return {
        sessionId: response.data.session_id
      };
    } catch (error) {
      console.error('[AgslagManusOpenAdapter] Error creating browser session:', error);
      throw new Error(`Failed to create browser session: ${error.message}`);
    }
  }
  
  /**
   * Navigate to a URL in a browser session
   */
  async navigateToBrowserUrl(params: {
    sessionId: string;
    url: string;
  }): Promise<{
    success: boolean;
    title?: string;
  }> {
    try {
      const response = await axios.post(`${this.baseUrl}/browser/session/${params.sessionId}/navigate`, {
        url: params.url
      });
      
      return {
        success: response.data.success,
        title: response.data.title
      };
    } catch (error) {
      console.error('[AgslagManusOpenAdapter] Error navigating to URL:', error);
      throw new Error(`Failed to navigate to URL: ${error.message}`);
    }
  }
  
  /**
   * Take a screenshot in a browser session
   */
  async takeBrowserScreenshot(params: {
    sessionId: string;
    selector?: string;
  }): Promise<{
    screenshot: string;
  }> {
    try {
      const response = await axios.post(`${this.baseUrl}/browser/session/${params.sessionId}/screenshot`, {
        selector: params.selector
      });
      
      return {
        screenshot: response.data.screenshot
      };
    } catch (error) {
      console.error('[AgslagManusOpenAdapter] Error taking screenshot:', error);
      throw new Error(`Failed to take screenshot: ${error.message}`);
    }
  }
  
  /**
   * Close a browser session
   */
  async closeBrowserSession(params: {
    sessionId: string;
  }): Promise<{
    success: boolean;
  }> {
    try {
      await axios.delete(`${this.baseUrl}/browser/session/${params.sessionId}`);
      
      return {
        success: true
      };
    } catch (error) {
      console.error('[AgslagManusOpenAdapter] Error closing browser session:', error);
      throw new Error(`Failed to close browser session: ${error.message}`);
    }
  }
  
  /**
   * Upload a file
   */
  async uploadFile(params: {
    filePath: string;
    content: string | Buffer;
  }): Promise<{
    success: boolean;
    url?: string;
  }> {
    try {
      const formData = new FormData();
      
      if (typeof params.content === 'string') {
        formData.append('file', new Blob([params.content]), params.filePath);
      } else {
        formData.append('file', new Blob([params.content]), params.filePath);
      }
      
      const response = await axios.post(`${this.baseUrl}/file/upload`, formData, {
        headers: {
          'Content-Type': 'multipart/form-data'
        }
      });
      
      return {
        success: response.data.success,
        url: response.data.url
      };
    } catch (error) {
      console.error('[AgslagManusOpenAdapter] Error uploading file:', error);
      throw new Error(`Failed to upload file: ${error.message}`);
    }
  }
  
  /**
   * Download a file
   */
  async downloadFile(params: {
    url: string;
    filePath?: string;
  }): Promise<{
    success: boolean;
    filePath?: string;
    content?: string | Buffer;
  }> {
    try {
      const response = await axios.post(`${this.baseUrl}/file/download`, {
        url: params.url,
        file_path: params.filePath
      });
      
      return {
        success: response.data.success,
        filePath: response.data.file_path,
        content: response.data.content
      };
    } catch (error) {
      console.error('[AgslagManusOpenAdapter] Error downloading file:', error);
      throw new Error(`Failed to download file: ${error.message}`);
    }
  }
}
```

### 3. Extend AGSLAG MCP Integration

Update the AGSLAG MCP Integration class to include the manus-open adapter:

```typescript
// integration/agslag_mcp_integration.ts
import { AgslagManusOpenAdapter } from '../adapters/manus_open_adapter';

export class AgslagMcpIntegration {
  // Existing code...
  
  private manusOpen: AgslagManusOpenAdapter;
  
  constructor(config: {
    // Existing config...
    manusOpen?: {
      baseUrl: string;
    }
  }) {
    // Existing initialization...
    
    // Initialize manus-open adapter if config is provided
    if (config.manusOpen) {
      this.manusOpen = new AgslagManusOpenAdapter(config.manusOpen);
    }
  }
  
  // New methods for terminal and browser interaction
  
  /**
   * Execute a terminal command
   */
  async executeTerminalCommand(command: string, workingDirectory?: string, timeout?: number) {
    if (!this.manusOpen) {
      throw new Error('manus-open adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Executing terminal command: ${command}`);
    
    return await this.manusOpen.executeTerminalCommand({
      command,
      workingDirectory,
      timeout
    });
  }
  
  /**
   * Create a terminal session
   */
  async createTerminalSession() {
    if (!this.manusOpen) {
      throw new Error('manus-open adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Creating terminal session`);
    
    return await this.manusOpen.createTerminalSession();
  }
  
  /**
   * Write to a terminal session
   */
  async writeToTerminalSession(sessionId: string, input: string) {
    if (!this.manusOpen) {
      throw new Error('manus-open adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Writing to terminal session: ${sessionId}`);
    
    return await this.manusOpen.writeToTerminalSession({
      sessionId,
      input
    });
  }
  
  /**
   * Read from a terminal session
   */
  async readFromTerminalSession(sessionId: string) {
    if (!this.manusOpen) {
      throw new Error('manus-open adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Reading from terminal session: ${sessionId}`);
    
    return await this.manusOpen.readFromTerminalSession({
      sessionId
    });
  }
  
  /**
   * Close a terminal session
   */
  async closeTerminalSession(sessionId: string) {
    if (!this.manusOpen) {
      throw new Error('manus-open adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Closing terminal session: ${sessionId}`);
    
    return await this.manusOpen.closeTerminalSession({
      sessionId
    });
  }
  
  /**
   * Execute a browser action
   */
  async executeBrowserAction(action: string, url?: string, selector?: string, value?: string, timeout?: number) {
    if (!this.manusOpen) {
      throw new Error('manus-open adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Executing browser action: ${action}`);
    
    return await this.manusOpen.executeBrowserAction({
      action,
      url,
      selector,
      value,
      timeout
    });
  }
  
  /**
   * Create a browser session
   */
  async createBrowserSession() {
    if (!this.manusOpen) {
      throw new Error('manus-open adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Creating browser session`);
    
    return await this.manusOpen.createBrowserSession();
  }
  
  /**
   * Navigate to a URL in a browser session
   */
  async navigateToBrowserUrl(sessionId: string, url: string) {
    if (!this.manusOpen) {
      throw new Error('manus-open adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Navigating to URL: ${url}`);
    
    return await this.manusOpen.navigateToBrowserUrl({
      sessionId,
      url
    });
  }
  
  /**
   * Take a screenshot in a browser session
   */
  async takeBrowserScreenshot(sessionId: string, selector?: string) {
    if (!this.manusOpen) {
      throw new Error('manus-open adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Taking browser screenshot`);
    
    return await this.manusOpen.takeBrowserScreenshot({
      sessionId,
      selector
    });
  }
  
  /**
   * Close a browser session
   */
  async closeBrowserSession(sessionId: string) {
    if (!this.manusOpen) {
      throw new Error('manus-open adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Closing browser session: ${sessionId}`);
    
    return await this.manusOpen.closeBrowserSession({
      sessionId
    });
  }
  
  /**
   * Upload a file
   */
  async uploadFile(filePath: string, content: string | Buffer) {
    if (!this.manusOpen) {
      throw new Error('manus-open adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Uploading file: ${filePath}`);
    
    return await this.manusOpen.uploadFile({
      filePath,
      content
    });
  }
  
  /**
   * Download a file
   */
  async downloadFile(url: string, filePath?: string) {
    if (!this.manusOpen) {
      throw new Error('manus-open adapter not initialized');
    }
    
    console.log(`[AGSLAG-MCP] Downloading file from: ${url}`);
    
    return await this.manusOpen.downloadFile({
      url,
      filePath
    });
  }
}
```

### 4. Create Example Usage

Create an example file that demonstrates how to use the manus-open integration:

```typescript
// examples/manus_open_example.ts
import { AgslagMcpIntegration } from '../integration/agslag_mcp_integration';
import * as fs from 'fs';
import * as path from 'path';

async function manusOpenExample() {
  // Initialize AGSLAG MCP Integration with manus-open config
  const integration = new AgslagMcpIntegration({
    manusOpen: {
      baseUrl: 'http://localhost:8330' // Adjust based on your manus-open setup
    }
  });
  
  try {
    // Execute a simple terminal command
    console.log('Executing a terminal command...');
    const result = await integration.executeTerminalCommand('ls -la');
    console.log('Command output:', result.stdout);
    
    // Create a terminal session
    console.log('Creating a terminal session...');
    const session = await integration.createTerminalSession();
    console.log('Terminal session created:', session);
    
    // Write to the terminal session
    console.log('Writing to the terminal session...');
    await integration.writeToTerminalSession(session.sessionId, 'echo "Hello, World!"\n');
    
    // Wait for the command to execute
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // Read from the terminal session
    console.log('Reading from the terminal session...');
    const output = await integration.readFromTerminalSession(session.sessionId);
    console.log('Terminal output:', output);
    
    // Create a browser session
    console.log('Creating a browser session...');
    const browserSession = await integration.createBrowserSession();
    console.log('Browser session created:', browserSession);
    
    // Navigate to a URL
    console.log('Navigating to a URL...');
    const navigation = await integration.navigateToBrowserUrl(browserSession.sessionId, 'https://www.example.com');
    console.log('Navigation result:', navigation);
    
    // Take a screenshot
    console.log('Taking a screenshot...');
    const screenshot = await integration.takeBrowserScreenshot(browserSession.sessionId);
    console.log('Screenshot taken');
    
    // Save the screenshot to a file
    const screenshotPath = path.join(__dirname, 'example.png');
    fs.writeFileSync(screenshotPath, Buffer.from(screenshot.screenshot, 'base64'));
    console.log(`Screenshot saved to ${screenshotPath}`);
    
    // Execute a browser action
    console.log('Executing a browser action...');
    const action = await integration.executeBrowserAction(
      'click',
      undefined,
      'a',
      undefined,
      5000
    );
    console.log('Browser action result:', action);
    
    // Close the browser session
    console.log('Closing the browser session...');
    await integration.closeBrowserSession(browserSession.sessionId);
    
    // Close the terminal session
    console.log('Closing the terminal session...');
    await integration.closeTerminalSession(session.sessionId);
    
    console.log('Example completed successfully');
  } catch (error) {
    console.error('Error in manus-open example:', error);
  }
}

// Run the example
manusOpenExample().catch(console.error);
```

### 5. Set Up Docker Environment

Create a Docker configuration to run manus-open:

```dockerfile
// Dockerfile.manus-open
FROM mcr.microsoft.com/playwright/python:v1.50.0-noble

# Set environment variables
ENV RUNTIME_API_HOST=http://localhost \
    CHROME_INSTANCE_PATH=/usr/bin/chromium \
    HOME=/home/ubuntu

# Install system dependencies and Node.js 20.x in a single layer
RUN apt-get update && \
    apt-get install -y \
        bash \
        sudo \
        curl \
        bc \
        software-properties-common && \
    # Add NodeSource and Python PPA
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    add-apt-repository -y ppa:deadsnakes/ppa && \
    apt-get update && \
    # Install all packages
    apt-get install -y \
        nodejs \
        python3.11 \
        python3.11-venv && \
    # Clean up
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Copy the application code
COPY ./manus-open /app

# Set the working directory
WORKDIR /app

# Install Python dependencies
RUN python3.11 -m venv /venv && \
    . /venv/bin/activate && \
    pip install --no-cache-dir -r requirements.txt

# Expose the API port
EXPOSE 8330

# Start the application
CMD ["/venv/bin/python", "app/main.py"]
```

## Configuration

To configure the manus-open integration, you need to set up the following:

1. **Docker Environment**: Ensure Docker is installed and configured
2. **manus-open Server**: Build and run the manus-open Docker container
3. **Base URL**: Configure the base URL for the manus-open API
4. **WebSocket Support**: Ensure WebSocket support is available for terminal sessions

See the [Configuration Management Guide](./configuration_management_guide.md) for more details.

## Testing

To test the manus-open integration:

1. Build the manus-open Docker image: `docker build -f Dockerfile.manus-open -t manus-open .`
2. Run the manus-open container: `docker run -p 8330:8330 manus-open`
3. Execute the example script: `npx ts-node examples/manus_open_example.ts`
4. Verify that terminal commands are executed, browser sessions are created, and screenshots are taken successfully

## Troubleshooting

Common issues and solutions:

1. **Docker Errors**: Ensure Docker is running and the manus-open container is built correctly
2. **Connection Errors**: Verify that the manus-open server is running and the base URL is correct
3. **API Errors**: Check that the manus-open API endpoints match the ones used in the adapter
4. **WebSocket Errors**: Ensure that WebSocket support is properly configured

## Next Steps

After integrating manus-open, consider:

1. Creating more complex terminal and browser interactions
2. Developing a UI for monitoring terminal and browser sessions
3. Implementing error handling and recovery mechanisms for terminal and browser interactions
4. Creating templates for common interaction patterns
