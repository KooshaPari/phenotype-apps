# Jarvis SWE Agent Terminal Management Enhancement

## 1. Introduction

This document outlines the strategy for enhancing the terminal management capabilities of the Jarvis SWE Agent. Analysis of leading SWE agents like manus-open and ANUS reveals that advanced terminal management is a critical capability for effective software engineering tasks. The current Jarvis implementation offers only basic shell command execution without persistent sessions or interactive capabilities.

## 2. Current Implementation Analysis

### 2.1 Existing Terminal Capabilities

The current Jarvis SWE Agent implements basic shell command execution through:

```typescript
// Simplified representation of current implementation
async function execShellCommand(command: string): Promise<string> {
  return new Promise((resolve, reject) => {
    exec(command, (error, stdout, stderr) => {
      if (error) {
        reject(`Error: ${error.message}`);
        return;
      }
      resolve(stdout);
    });
  });
}
```

### 2.2 Limitations

- **Non-Interactive**: Cannot interact with processes that require input after starting
- **No Session Persistence**: Each command runs in a new process with no shared context
- **Limited Process Control**: No ability to terminate long-running processes
- **No Real-Time Output**: Output is only available after command completion
- **No Environment Customization**: Limited ability to customize the execution environment
- **No Multi-Session Support**: Cannot manage multiple terminal sessions simultaneously

## 3. Enhanced Terminal Management Architecture

### 3.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                Terminal Management Service                  │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐       │
│  │  Session    │   │  Process    │   │ Environment │       │
│  │  Manager    │   │  Manager    │   │  Manager    │       │
│  └─────────────┘   └─────────────┘   └─────────────┘       │
│                                                             │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐       │
│  │  Input/     │   │  Output     │   │  Signal     │       │
│  │  Output     │   │  Buffer     │   │  Handler    │       │
│  └─────────────┘   └─────────────┘   └─────────────┘       │
│                                                             │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                     WebSocket Server                        │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                      REST API Layer                         │
└─────────────────────────────────────────────────────────────┘
```

### 3.2 Core Components

#### 3.2.1 Session Manager

- **Purpose**: Create, track, and manage terminal sessions
- **Responsibilities**:
  - Generate unique session IDs
  - Track active sessions and their state
  - Handle session timeouts and cleanup
  - Manage session metadata (creation time, last activity, owner)
  - Enforce session limits and resource constraints

#### 3.2.2 Process Manager

- **Purpose**: Control processes within terminal sessions
- **Responsibilities**:
  - Start processes with appropriate options
  - Monitor process status and resource usage
  - Terminate processes when requested or on timeout
  - Handle process exit events
  - Manage process hierarchies (parent/child relationships)

#### 3.2.3 Environment Manager

- **Purpose**: Configure and customize terminal environments
- **Responsibilities**:
  - Set environment variables
  - Configure working directories
  - Manage shell configuration
  - Handle path and user context
  - Support different shells (bash, zsh, PowerShell)

#### 3.2.4 Input/Output Handler

- **Purpose**: Manage bidirectional communication with processes
- **Responsibilities**:
  - Send input to process stdin
  - Capture output from stdout and stderr
  - Handle special characters and control sequences
  - Support binary data transmission
  - Implement input validation and sanitization

#### 3.2.5 Output Buffer

- **Purpose**: Store and manage process output
- **Responsibilities**:
  - Buffer output for efficient retrieval
  - Implement scrollback functionality
  - Handle large output volumes
  - Support output filtering and searching
  - Manage output retention policies

#### 3.2.6 Signal Handler

- **Purpose**: Send signals to processes
- **Responsibilities**:
  - Support standard signals (SIGTERM, SIGKILL, etc.)
  - Handle custom signals for specific applications
  - Implement graceful termination sequences
  - Track signal delivery and results

### 3.3 API Endpoints

#### 3.3.1 REST API

- **`POST /terminals`**: Create a new terminal session
  - Parameters: `shell` (optional), `cwd` (optional), `env` (optional)
  - Returns: Session ID and initial connection details

- **`GET /terminals/{sessionId}`**: Get terminal session information
  - Returns: Session metadata, status, resource usage

- **`DELETE /terminals/{sessionId}`**: Terminate a terminal session
  - Parameters: `force` (optional, boolean)
  - Returns: Termination status

- **`POST /terminals/{sessionId}/resize`**: Resize terminal dimensions
  - Parameters: `cols` (number), `rows` (number)
  - Returns: Success status

- **`POST /terminals/{sessionId}/execute`**: Execute a command in the session
  - Parameters: `command` (string), `timeout` (optional, number)
  - Returns: Command ID for tracking

- **`GET /terminals/{sessionId}/commands/{commandId}`**: Get command execution status
  - Returns: Status, exit code, output (if completed)

#### 3.3.2 WebSocket API

- **`/terminals/{sessionId}/socket`**: WebSocket connection for real-time interaction
  - Supports bidirectional communication
  - Client sends input data
  - Server sends output data and status updates
  - Implements heartbeat mechanism for connection health

### 3.4 Data Models

#### 3.4.1 Terminal Session

```typescript
interface TerminalSession {
  id: string;                 // Unique session identifier
  status: SessionStatus;      // 'creating', 'running', 'terminated'
  shell: string;              // Shell type (bash, zsh, powershell)
  cwd: string;                // Current working directory
  env: Record<string, string>; // Environment variables
  createdAt: Date;            // Creation timestamp
  lastActivity: Date;         // Last activity timestamp
  owner: string;              // Session owner (user or agent ID)
  dimensions: {               // Terminal dimensions
    cols: number;
    rows: number;
  };
  resourceUsage: {            // Resource consumption metrics
    cpu: number;              // CPU usage percentage
    memory: number;           // Memory usage in MB
    processes: number;        // Number of processes in session
  };
}
```

#### 3.4.2 Command Execution

```typescript
interface CommandExecution {
  id: string;                 // Unique command identifier
  sessionId: string;          // Parent session identifier
  command: string;            // Command string
  status: CommandStatus;      // 'running', 'completed', 'failed', 'terminated'
  startedAt: Date;            // Start timestamp
  completedAt?: Date;         // Completion timestamp (if applicable)
  exitCode?: number;          // Exit code (if completed)
  outputSize: number;         // Size of output in bytes
  error?: string;             // Error message (if failed)
}
```

## 4. Implementation Strategy

### 4.1 Core Technology Selection

- **Pseudo-Terminal Library**: `node-pty` for creating and managing pseudo-terminals
- **WebSocket Implementation**: `socket.io` for real-time bidirectional communication
- **Process Management**: Node.js child process API with enhancements
- **Session Storage**: Redis for session state persistence
- **Containerization**: Docker for process isolation and security

### 4.2 Terminal Session Lifecycle

1. **Session Creation**:
   - Client requests new terminal session
   - Server creates pseudo-terminal with specified parameters
   - Server generates session ID and initializes session state
   - Server returns session ID and connection details

2. **Interactive Communication**:
   - Client establishes WebSocket connection
   - Client sends input data (keystrokes, paste events)
   - Server forwards input to pseudo-terminal
   - Server captures output and sends to client
   - Server tracks session activity and updates timestamps

3. **Command Execution**:
   - Client sends command execution request
   - Server writes command to pseudo-terminal
   - Server tracks command execution status
   - Server captures and buffers output
   - Server detects command completion and updates status

4. **Session Termination**:
   - Client requests session termination
   - Server sends termination signal to processes
   - Server waits for graceful shutdown or forces termination
   - Server cleans up resources and updates session state
   - Server notifies client of termination completion

### 4.3 Security Considerations

- **Input Validation**: Sanitize and validate all input to prevent command injection
- **Resource Limits**: Implement CPU, memory, and time limits for terminal sessions
- **Access Control**: Enforce strict access controls based on user/agent permissions
- **Isolation**: Run terminal sessions in isolated containers
- **Audit Logging**: Log all terminal activity for security monitoring
- **Timeout Policies**: Automatically terminate inactive sessions
- **Restricted Commands**: Implement blocklist/allowlist for commands

### 4.4 Performance Optimization

- **Connection Pooling**: Reuse connections to improve performance
- **Output Buffering**: Efficiently buffer and transmit terminal output
- **Lazy Loading**: Load session data on demand
- **Caching**: Cache frequently accessed session information
- **Batch Processing**: Batch multiple operations for efficiency
- **Resource Monitoring**: Track and optimize resource usage

## 5. Tool Implementation

### 5.1 Terminal Management Tools

The enhanced terminal capabilities will be exposed through the following tools:

#### 5.1.1 `start_terminal_session`

- **Purpose**: Create a new terminal session
- **Parameters**:
  - `shell`: Shell type (bash, zsh, powershell) - optional
  - `cwd`: Working directory - optional
  - `env`: Environment variables - optional
  - `dimensions`: Terminal dimensions (cols, rows) - optional
- **Returns**: Session ID and connection details

#### 5.1.2 `execute_in_terminal`

- **Purpose**: Execute a command in a terminal session
- **Parameters**:
  - `session_id`: Terminal session identifier
  - `command`: Command to execute
  - `timeout`: Maximum execution time (seconds) - optional
  - `wait_for_completion`: Whether to wait for command completion - optional
- **Returns**: Command ID, initial output, and status

#### 5.1.3 `send_input_to_terminal`

- **Purpose**: Send input to a running process
- **Parameters**:
  - `session_id`: Terminal session identifier
  - `input`: Input text or data
  - `end_with_newline`: Whether to append newline - optional
- **Returns**: Success status

#### 5.1.4 `get_terminal_output`

- **Purpose**: Retrieve output from a terminal session
- **Parameters**:
  - `session_id`: Terminal session identifier
  - `since`: Timestamp or marker for incremental retrieval - optional
  - `max_lines`: Maximum number of lines to retrieve - optional
- **Returns**: Terminal output and continuation marker

#### 5.1.5 `resize_terminal`

- **Purpose**: Resize terminal dimensions
- **Parameters**:
  - `session_id`: Terminal session identifier
  - `cols`: Number of columns
  - `rows`: Number of rows
- **Returns**: Success status

#### 5.1.6 `terminate_terminal_session`

- **Purpose**: Terminate a terminal session
- **Parameters**:
  - `session_id`: Terminal session identifier
  - `force`: Whether to force termination - optional
  - `signal`: Signal to send (SIGTERM, SIGKILL) - optional
- **Returns**: Termination status

#### 5.1.7 `list_terminal_sessions`

- **Purpose**: List active terminal sessions
- **Parameters**:
  - `filter`: Filter criteria (owner, status) - optional
  - `limit`: Maximum number of sessions to return - optional
  - `offset`: Pagination offset - optional
- **Returns**: List of session summaries

### 5.2 Integration with Existing Tools

- **Shell Tool**: Enhance existing shell tool to leverage terminal sessions
- **File System Tools**: Integrate with terminal sessions for file operations
- **Git Tools**: Use terminal sessions for Git operations
- **Build Tools**: Execute build commands in terminal sessions

### 5.3 Usage Examples

#### Example 1: Interactive Package Installation

```typescript
// Create a new terminal session
const { session_id } = await tools.start_terminal_session({
  shell: "bash",
  cwd: "/project/directory"
});

// Execute npm init
await tools.execute_in_terminal({
  session_id,
  command: "npm init"
});

// Send responses to prompts
await tools.send_input_to_terminal({
  session_id,
  input: "my-project\n"  // Package name
});

await tools.send_input_to_terminal({
  session_id,
  input: "1.0.0\n"  // Version
});

// Continue with other prompts...

// Install dependencies
await tools.execute_in_terminal({
  session_id,
  command: "npm install express --save"
});

// Get terminal output
const { output } = await tools.get_terminal_output({
  session_id
});

// Terminate the session when done
await tools.terminate_terminal_session({
  session_id
});
```

#### Example 2: Running and Monitoring a Development Server

```typescript
// Create a new terminal session
const { session_id } = await tools.start_terminal_session({
  shell: "bash",
  cwd: "/project/directory"
});

// Start development server
await tools.execute_in_terminal({
  session_id,
  command: "npm run dev",
  wait_for_completion: false  // Don't wait as this is a long-running process
});

// Wait for server to start by monitoring output
let serverStarted = false;
while (!serverStarted) {
  const { output } = await tools.get_terminal_output({
    session_id,
    since: "last"
  });
  
  if (output.includes("Server started on port")) {
    serverStarted = true;
  }
  
  // Avoid tight polling
  await new Promise(resolve => setTimeout(resolve, 500));
}

// Server is now running, perform other operations...

// When done, terminate the server
await tools.terminate_terminal_session({
  session_id
});
```

## 6. Implementation Plan

### 6.1 Phase 1: Core Infrastructure (Weeks 1-3)

1. **Setup Terminal Service**:
   - Implement Session Manager
   - Integrate node-pty
   - Create basic process management

2. **Implement API Layer**:
   - Develop REST API endpoints
   - Set up WebSocket server
   - Create data models and validation

3. **Develop Basic Tools**:
   - Implement start_terminal_session
   - Implement execute_in_terminal
   - Implement terminate_terminal_session

### 6.2 Phase 2: Interactive Capabilities (Weeks 4-6)

1. **Enhance Input/Output Handling**:
   - Implement bidirectional communication
   - Develop output buffering
   - Add support for special characters

2. **Add Process Control**:
   - Implement signal handling
   - Add process monitoring
   - Develop resource tracking

3. **Implement Additional Tools**:
   - Develop send_input_to_terminal
   - Implement get_terminal_output
   - Add resize_terminal

### 6.3 Phase 3: Advanced Features (Weeks 7-9)

1. **Environment Customization**:
   - Implement Environment Manager
   - Add support for different shells
   - Develop working directory management

2. **Security Enhancements**:
   - Implement input validation
   - Add resource limits
   - Develop audit logging

3. **Performance Optimization**:
   - Optimize connection handling
   - Implement efficient buffering
   - Add caching mechanisms

### 6.4 Phase 4: Integration and Testing (Weeks 10-12)

1. **Tool Integration**:
   - Integrate with existing tools
   - Develop usage examples
   - Create documentation

2. **Testing**:
   - Unit testing
   - Integration testing
   - Performance testing
   - Security testing

3. **Deployment**:
   - Containerization
   - CI/CD integration
   - Monitoring setup

## 7. Technical Requirements

### 7.1 Dependencies

- Node.js v16+
- node-pty
- socket.io
- Redis
- Docker

### 7.2 Development Environment

- TypeScript development tools
- Testing framework (Jest)
- Docker and Docker Compose
- Redis instance

### 7.3 Production Environment

- Containerized deployment
- Redis for session state
- Monitoring and logging infrastructure
- Load balancer for WebSocket connections

## 8. Risks and Mitigations

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Performance issues with many concurrent sessions | High | Medium | Implement resource limits, optimize I/O handling, use worker processes |
| Security vulnerabilities in command execution | High | Medium | Strict input validation, containerization, access controls |
| Memory leaks in long-running sessions | Medium | Medium | Implement session timeouts, monitor memory usage, regular testing |
| Compatibility issues across different environments | Medium | High | Containerization, comprehensive testing across platforms |
| WebSocket connection stability | High | Medium | Implement reconnection logic, heartbeat mechanism, fallback to polling |

## 9. Conclusion

Enhancing the terminal management capabilities of the Jarvis SWE Agent will significantly improve its ability to perform complex software engineering tasks. The proposed architecture provides interactive, persistent terminal sessions with comprehensive process control and real-time communication. This enhancement aligns with the capabilities observed in leading SWE agents and addresses key limitations in the current implementation.

The phased implementation plan ensures a systematic approach to developing these capabilities, with a focus on security, performance, and usability. When completed, this enhancement will enable Jarvis to perform a wide range of interactive terminal operations, from package installation to running development servers, significantly expanding its software engineering capabilities.
