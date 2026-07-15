/**
 * KVirtualStage Node.js Bindings
 * 
 * Playwright-equivalent desktop automation platform for AI agents.
 * Provides high-level JavaScript/TypeScript API for:
 * - Desktop automation and control  
 * - Session management
 * - Recording and playbook
 * - Natural human-like interactions
 * - Cross-platform virtualization
 * 
 * @example
 * ```javascript
 * const kvs = require('kvirtualstage');
 * 
 * // Create automation instance
 * const automation = new kvs.KVirtualStage();
 * 
 * // Create a new desktop session
 * const session = await automation.createSession({
 *   userId: 'demo_user',
 *   sessionName: 'my_session', 
 *   desktopType: 'ubuntu'
 * });
 * 
 * // Perform natural automation
 * await session.moveCursor(400, 300);
 * await session.click();
 * await session.typeText('Hello from KVirtualStage!');
 * 
 * // Start recording
 * const recording = await session.startRecording('demo.mp4');
 * 
 * // Execute workflow
 * const workflow = new kvs.Workflow('Calculator Demo');
 * workflow.addStep('moveCursor', { x: 100, y: 100 });
 * workflow.addStep('click');
 * workflow.addStep('type', { text: '2 + 2 =' });
 * await session.executeWorkflow(workflow);
 * 
 * // Stop recording
 * await recording.stop();
 * ```
 */

const axios = require('axios');
const WebSocket = require('ws');
const EventEmitter = require('events');

/**
 * Mouse button types for click operations
 */
const MouseButton = {
  LEFT: 'left',
  RIGHT: 'right', 
  MIDDLE: 'middle'
};

/**
 * Supported desktop environments
 */
const DesktopType = {
  UBUNTU: 'ubuntu',
  UBUNTU_XFCE: 'ubuntu-xfce',
  UBUNTU_KDE: 'ubuntu-kde',
  CENTOS: 'centos',
  FEDORA: 'fedora',
  ARCH: 'arch',
  DEBIAN: 'debian'
};

/**
 * Recording quality presets
 */
const RecordingQuality = {
  LOW: 'low',
  MEDIUM: 'medium',
  HIGH: 'high', 
  STREAMING: 'streaming'
};

/**
 * 2D coordinate point
 */
class Point {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }
}

/**
 * Information about an active session
 */
class SessionInfo {
  constructor(data) {
    this.sessionId = data.session_id;
    this.userId = data.user_id;
    this.desktopType = data.desktop_type;
    this.status = data.status;
    this.createdAt = data.created_at;
    this.lastActivity = data.last_activity;
    this.recordingActive = data.recording_active;
  }
}

/**
 * Result of workflow execution
 */
class WorkflowResult {
  constructor(data) {
    this.workflowName = data.workflow_name;
    this.success = data.success;
    this.totalSteps = data.total_steps;
    this.successfulSteps = data.successful_steps;
    this.executionTimeMs = data.execution_time_ms;
    this.errors = data.errors || [];
  }
}

/**
 * Individual step in an automation workflow
 */
class WorkflowStep {
  constructor(name, actionType, parameters = {}) {
    this.name = name;
    this.actionType = actionType;
    this.parameters = parameters;
    this.timeoutSeconds = parameters.timeout || 30;
  }
}

/**
 * Automation workflow definition
 */
class Workflow {
  constructor(name, description = '', continueOnError = false) {
    this.name = name;
    this.description = description;
    this.continueOnError = continueOnError;
    this.steps = [];
  }

  /**
   * Add a step to the workflow
   */
  addStep(actionType, parameters = {}, name = null) {
    if (!name) {
      name = `Step ${this.steps.length + 1}: ${actionType}`;
    }
    
    const step = new WorkflowStep(name, actionType, parameters);
    this.steps.push(step);
    return this;
  }

  /**
   * Add cursor movement step
   */
  moveCursor(x, y, name = null) {
    return this.addStep('move_cursor', { x, y }, name);
  }

  /**
   * Add click step
   */
  click(x = null, y = null, button = MouseButton.LEFT, name = null) {
    const params = { button };
    if (x !== null && y !== null) {
      params.x = x;
      params.y = y;
    }
    return this.addStep('click', params, name);
  }

  /**
   * Add text typing step
   */
  typeText(text, name = null) {
    return this.addStep('type', { text }, name);
  }

  /**
   * Add wait/delay step
   */
  wait(seconds, name = null) {
    return this.addStep('wait', { duration: seconds }, name);
  }

  /**
   * Convert workflow to object for API
   */
  toObject() {
    return {
      name: this.name,
      description: this.description,
      continue_on_error: this.continueOnError,
      steps: this.steps.map(step => ({
        name: step.name,
        action_type: step.actionType,
        parameters: step.parameters,
        timeout_seconds: step.timeoutSeconds
      }))
    };
  }
}

/**
 * Recording session control
 */
class Recording extends EventEmitter {
  constructor(session, recordingId) {
    super();
    this.session = session;
    this.recordingId = recordingId;
    this.active = true;
    this.startTime = Date.now();
  }

  /**
   * Stop the recording and return output path
   */
  async stop() {
    if (!this.active) {
      throw new Error('Recording is not active');
    }

    const result = await this.session._apiCall(
      `sessions/${this.session.sessionId}/recording/stop`,
      'POST'
    );

    this.active = false;
    this.emit('stopped', result);
    console.log(`Recording stopped: ${result}`);
    return result;
  }

  /**
   * Get recording duration in seconds
   */
  get duration() {
    return (Date.now() - this.startTime) / 1000;
  }
}

/**
 * Desktop automation session
 */
class Session extends EventEmitter {
  constructor(kvsInstance, sessionId, userId, desktopType) {
    super();
    this.kvs = kvsInstance;
    this.sessionId = sessionId;
    this.userId = userId;
    this.desktopType = desktopType;
    this.currentRecording = null;
    this.websocket = null;
  }

  /**
   * Make API call to KVirtualStage server
   */
  async _apiCall(endpoint, method = 'GET', data = null) {
    return await this.kvs._apiCall(endpoint, method, data);
  }

  /**
   * Move cursor to specified coordinates with natural movement
   */
  async moveCursor(x, y) {
    await this._apiCall(
      `sessions/${this.sessionId}/cursor/move`,
      'POST',
      { target_x: x, target_y: y }
    );
    console.log(`Cursor moved to (${x}, ${y})`);
    this.emit('cursorMoved', { x, y });
  }

  /**
   * Click at current cursor position or specified coordinates
   */
  async click(x = null, y = null, button = MouseButton.LEFT) {
    if (x !== null && y !== null) {
      await this.moveCursor(x, y);
    }

    await this._apiCall(
      `sessions/${this.sessionId}/mouse/click`,
      'POST',
      { button }
    );
    console.log(`Clicked with ${button} button`);
    this.emit('clicked', { x, y, button });
  }

  /**
   * Double-click at current cursor position or specified coordinates
   */
  async doubleClick(x = null, y = null) {
    if (x !== null && y !== null) {
      await this.moveCursor(x, y);
    }

    await this.click();
    await new Promise(resolve => setTimeout(resolve, 100)); // Small delay
    await this.click();
    console.log('Double-clicked');
    this.emit('doubleClicked', { x, y });
  }

  /**
   * Right-click at current cursor position or specified coordinates
   */
  async rightClick(x = null, y = null) {
    await this.click(x, y, MouseButton.RIGHT);
  }

  /**
   * Type text with natural human-like timing
   */
  async typeText(text, wpm = 65) {
    await this._apiCall(
      `sessions/${this.sessionId}/keyboard/type`,
      'POST',
      { text, wpm }
    );
    const displayText = text.length > 50 ? text.substring(0, 50) + '...' : text;
    console.log(`Typed: ${displayText}`);
    this.emit('textTyped', { text });
  }

  /**
   * Press a specific key (e.g., 'Enter', 'Tab', 'Escape')
   */
  async keyPress(key) {
    await this._apiCall(
      `sessions/${this.sessionId}/keyboard/key`,
      'POST',
      { key }
    );
    console.log(`Key pressed: ${key}`);
    this.emit('keyPressed', { key });
  }

  /**
   * Press a combination of keys (e.g., 'Ctrl', 'C')
   */
  async keyCombination(...keys) {
    await this._apiCall(
      `sessions/${this.sessionId}/keyboard/combo`,
      'POST',
      { keys }
    );
    console.log(`Key combination: ${keys.join('+')}`);
    this.emit('keyCombination', { keys });
  }

  /**
   * Scroll in the specified direction
   */
  async scroll(direction, amount = 3) {
    await this._apiCall(
      `sessions/${this.sessionId}/mouse/scroll`,
      'POST',
      { direction, amount }
    );
    console.log(`Scrolled ${direction} by ${amount}`);
    this.emit('scrolled', { direction, amount });
  }

  /**
   * Take a screenshot of the current desktop
   */
  async screenshot(filename = null) {
    const result = await this._apiCall(
      `sessions/${this.sessionId}/screenshot`,
      'POST',
      { filename }
    );
    console.log(`Screenshot taken: ${result}`);
    this.emit('screenshot', { filename: result });
    return result;
  }

  /**
   * Start recording the session
   */
  async startRecording(filename = null, quality = RecordingQuality.MEDIUM) {
    if (this.currentRecording && this.currentRecording.active) {
      throw new Error('Recording is already active');
    }

    if (!filename) {
      filename = `kvs_recording_${Date.now()}.mp4`;
    }

    const result = await this._apiCall(
      `sessions/${this.sessionId}/recording/start`,
      'POST',
      {
        output_filename: filename,
        quality
      }
    );

    const recording = new Recording(this, result.recording_id);
    this.currentRecording = recording;
    console.log(`Recording started: ${filename}`);
    this.emit('recordingStarted', { filename, recordingId: result.recording_id });
    return recording;
  }

  /**
   * Execute an automation workflow
   */
  async executeWorkflow(workflow) {
    const result = await this._apiCall(
      `sessions/${this.sessionId}/workflow`,
      'POST',
      workflow.toObject()
    );

    const workflowResult = new WorkflowResult(result);
    console.log(`Workflow '${workflow.name}' completed: ${workflowResult.success}`);
    this.emit('workflowCompleted', workflowResult);
    return workflowResult;
  }

  /**
   * Connect to WebSocket for live streaming
   */
  connectStream() {
    if (this.websocket) {
      this.websocket.close();
    }

    const wsUrl = `ws://${this.kvs.serverUrl.replace('http://', '').replace('https://', '')}/api/v1/sessions/${this.sessionId}/stream`;
    this.websocket = new WebSocket(wsUrl);

    this.websocket.on('open', () => {
      console.log('WebSocket stream connected');
      this.emit('streamConnected');
    });

    this.websocket.on('message', (data) => {
      try {
        const message = JSON.parse(data);
        this.emit('streamMessage', message);
      } catch (e) {
        this.emit('streamData', data);
      }
    });

    this.websocket.on('close', () => {
      console.log('WebSocket stream disconnected');
      this.emit('streamDisconnected');
    });

    this.websocket.on('error', (error) => {
      console.error('WebSocket error:', error);
      this.emit('streamError', error);
    });

    return this.websocket;
  }

  /**
   * Get detailed session information
   */
  async getInfo() {
    const result = await this._apiCall(`sessions/${this.sessionId}`);
    return new SessionInfo(result);
  }

  /**
   * Close the session and clean up resources
   */
  async close() {
    if (this.currentRecording && this.currentRecording.active) {
      await this.currentRecording.stop();
    }

    if (this.websocket) {
      this.websocket.close();
    }

    await this._apiCall(
      `sessions/${this.sessionId}`,
      'DELETE'
    );
    console.log(`Session closed: ${this.sessionId}`);
    this.emit('closed');
  }
}

/**
 * Main KVirtualStage automation interface
 */
class KVirtualStage extends EventEmitter {
  constructor(serverUrl = 'http://localhost:8080') {
    super();
    this.serverUrl = serverUrl.replace(/\/$/, '');
    this.baseUrl = `${this.serverUrl}/api/v1`;
    this.sessions = new Map();
    this.requestTimeout = 30000; // 30 seconds
  }

  /**
   * Make HTTP API call to KVirtualStage server
   */
  async _apiCall(endpoint, method = 'GET', data = null) {
    const url = `${this.baseUrl}/${endpoint.replace(/^\//, '')}`;
    
    const config = {
      method,
      url,
      timeout: this.requestTimeout,
      headers: {
        'Content-Type': 'application/json'
      }
    };

    if (data) {
      config.data = data;
    }

    try {
      const response = await axios(config);
      
      if (response.data && !response.data.success && response.data.error) {
        throw new Error(`API error: ${response.data.error}`);
      }

      return response.data.data || response.data;
    } catch (error) {
      if (error.response) {
        throw new Error(`API call failed (${error.response.status}): ${error.response.data}`);
      } else if (error.request) {
        throw new Error(`Network error: ${error.message}`);
      } else {
        throw error;
      }
    }
  }

  /**
   * Create a new desktop automation session
   */
  async createSession(options = {}) {
    const {
      userId,
      sessionName = `session_${Date.now()}`,
      desktopType = DesktopType.UBUNTU
    } = options;

    if (!userId) {
      throw new Error('userId is required');
    }

    const result = await this._apiCall(
      'sessions',
      'POST',
      {
        user_id: userId,
        session_name: sessionName,
        desktop_type: desktopType
      }
    );

    const session = new Session(this, result.session_id, userId, desktopType);
    this.sessions.set(session.sessionId, session);

    console.log(`Session created: ${session.sessionId}`);
    this.emit('sessionCreated', session);
    return session;
  }

  /**
   * Get an existing session by ID
   */
  async getSession(sessionId) {
    if (this.sessions.has(sessionId)) {
      return this.sessions.get(sessionId);
    }

    // Try to fetch from server
    try {
      const sessionInfo = await this._apiCall(`sessions/${sessionId}`);
      const session = new Session(this, sessionId, sessionInfo.user_id, sessionInfo.desktop_type);
      this.sessions.set(sessionId, session);
      return session;
    } catch (error) {
      return null;
    }
  }

  /**
   * List all active sessions
   */
  async listSessions() {
    const result = await this._apiCall('sessions');
    return result.map(s => new SessionInfo(s));
  }

  /**
   * Check server health and status
   */
  async healthCheck() {
    return await this._apiCall('health');
  }

  /**
   * Get server performance metrics
   */
  async getMetrics() {
    return await this._apiCall('metrics');
  }

  /**
   * Close all active sessions
   */
  async closeAllSessions() {
    const sessions = Array.from(this.sessions.values());
    await Promise.all(sessions.map(session => session.close()));
    this.sessions.clear();
  }

  /**
   * Set request timeout
   */
  setTimeout(ms) {
    this.requestTimeout = ms;
  }
}

// Export classes and constants
module.exports = {
  KVirtualStage,
  Session,
  Workflow,
  WorkflowStep,
  Recording,
  Point,
  SessionInfo,
  WorkflowResult,
  MouseButton,
  DesktopType,
  RecordingQuality
};

// Also export as ES6 module for modern Node.js
module.exports.default = KVirtualStage;