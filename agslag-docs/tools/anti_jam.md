# AGSLAG Custom Tools: Anti-Jamming & Responsiveness Checks

This document describes the MCP tools provided within AGSLAG designed to monitor agent responsiveness and communication patterns, helping to detect potential issues like unresponsive agents or message flooding ("jamming"). These tools are typically implemented in `src/tools/anti_jam.ts`.

## Tools Overview

*   **`check_agent_responsiveness`**: Checks if a specific agent has sent a heartbeat within a defined time threshold.
*   **`check_message_recipient_status`**: Checks the responsiveness of agents who recently received messages from a specific sender.
*   **`check_message_frequency`**: Checks if an agent is sending messages at an unusually high rate, potentially indicating spamming or a loop.

## Tool Details

### 1. `check_agent_responsiveness`

Verifies if a specific agent is considered responsive based on the timestamp of its last recorded heartbeat.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "agent_id": {
      "type": "string",
      "description": "The unique ID of the agent to check."
    },
    "threshold_seconds": {
      "type": "number",
      "format": "integer",
      "positive": true,
      "default": 300,
      "description": "The maximum time in seconds since the last heartbeat to consider the agent responsive (default: 300s / 5 minutes)."
    }
  },
  "required": ["agent_id"]
}
```

**Output:** An object indicating whether the agent `is_responsive`, the timestamp of the `last_heartbeat`, `seconds_since_heartbeat`, the `threshold_seconds` used, and a `reason` ("Within threshold", "Exceeded threshold", or "Agent not registered").

### 2. `check_message_recipient_status`

Checks the responsiveness of all unique agents who received messages from a specified sender within a given time window.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "sender_id": {
      "type": "string",
      "description": "The unique ID of the agent whose sent messages' recipients will be checked."
    },
    "time_window_seconds": {
      "type": "number",
      "format": "integer",
      "positive": true,
      "default": 600,
      "description": "How far back in seconds to look for messages sent by the sender (default: 600s / 10 minutes)."
    },
    "responsiveness_threshold_seconds": {
      "type": "number",
      "format": "integer",
      "positive": true,
      "default": 300,
      "description": "The maximum time in seconds since a recipient's last heartbeat to consider them responsive (default: 300s / 5 minutes)."
    }
  },
  "required": ["sender_id"]
}
```

**Output:** An object summarizing the check, including the number of `recipients_checked`, a list of `unresponsive_recipients` (with details like their last heartbeat and reason), and optionally a list of `all_recipient_statuses`.

### 3. `check_message_frequency`

Counts the number of messages sent by a specific agent within a recent time window and compares it against a threshold to detect potential message flooding or spamming.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "agent_id": {
      "type": "string",
      "description": "The unique ID of the agent whose message frequency will be checked."
    },
    "time_window_seconds": {
      "type": "number",
      "format": "integer",
      "positive": true,
      "default": 60,
      "description": "How far back in seconds to look for messages sent by the agent (default: 60s / 1 minute)."
    },
    "frequency_threshold": {
      "type": "number",
      "format": "integer",
      "positive": true,
      "default": 10,
      "description": "The maximum number of messages allowed within the time window before flagging as potential spam (default: 10)."
    }
  },
  "required": ["agent_id"]
}
```

**Output:** An object containing the `agent_id`, the actual `message_count` within the window, the `time_window_seconds` and `frequency_threshold` used, and a boolean flag `is_potentially_spamming`.
