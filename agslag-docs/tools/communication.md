# AGSLAG Custom Tools: Communication & Collaboration

This document describes the MCP tools provided within AGSLAG for enabling communication, collaboration, and information sharing between agents. These tools are typically implemented in `src/tools/communication.ts` and `src/tools/messaging.ts` and interact with the system's internal database and potentially a file storage system.

## Tools Overview

*   **`send_message`**: Sends a direct message or posts to a thread.
*   **`get_messages`**: Retrieves messages for the agent.
*   **`create_thread`**: Creates a new discussion thread.
*   **`join_thread`**: Allows an agent to join an existing thread.
*   **`broadcast_message`**: Sends a message to multiple agents.
*   **`check_new_messages`**: Checks for unread messages.
*   **`transfer_file`**: Sends a file to another agent via the server.
*   **`get_thread_details`**: Retrieves metadata about a thread.
*   **`add_participant_to_thread`**: Adds an agent to a thread.
*   **`remove_participant_from_thread`**: Removes an agent from a thread.

## Tool Details

### 1. `send_message`

Sends a message either directly to a single agent or posts it within a specific discussion thread.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "sender_id": { "type": "string", "description": "Your unique Agent ID." },
    "recipient_id": { "type": "string", "description": "Optional: The unique Agent ID of the recipient (for direct 1-to-1 messages). Do not use if sending to a thread." },
    "thread_id": { "type": "string", "description": "Optional: The ID of the thread to post the message in (for group discussions). Do not use if sending a direct message." },
    "content": { "type": "string", "description": "The textual content of the message." },
    "urgent": { "type": "boolean", "default": false, "description": "Set to true if this message requires immediate attention." }
  },
  "required": ["sender_id", "content"]
  // Note: Logic ensures either recipient_id or thread_id is provided, but not both.
}
```

**Output:** Confirmation message including the new message ID, and the created message object.

### 2. `get_messages`

Retrieves messages intended for the calling agent, including direct messages and messages in threads they participate in. Automatically marks retrieved messages as read for the calling agent.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "agent_id": { "type": "string", "description": "Your unique Agent ID." },
    "thread_id": { "type": "string", "description": "Optional: Retrieve messages only from this specific thread ID." },
    "limit": { "type": "number", "default": 10, "description": "Maximum number of messages to retrieve (most recent first)." },
    "since": { "type": "string", "format": "date-time", "description": "Optional: Retrieve messages created after this ISO 8601 timestamp (e.g., '2025-03-30T12:00:00Z')." }
  },
  "required": ["agent_id"]
}
```

**Output:** An array of message objects, sorted by most recent first, up to the specified limit.

### 3. `create_thread`

Creates a new discussion thread for focused collaboration among a specified group of agents.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "creator_id": { "type": "string", "description": "Your unique Agent ID." },
    "title": { "type": "string", "description": "A concise and descriptive title for the discussion thread." },
    "participants": { "type": "array", "items": { "type": "string" }, "description": "List of unique Agent IDs of all initial participants (creator is added automatically if not included)." },
    "initial_message": { "type": "string", "description": "Optional first message to post in the newly created thread." }
  },
  "required": ["creator_id", "title", "participants"]
}
```

**Output:** Confirmation message, the new thread object (including ID), and the ID of the initial message if one was provided. Participants (excluding the creator) are notified.

### 4. `join_thread`

Allows an agent to join an existing discussion thread.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "thread_id": { "type": "string", "description": "The unique ID of the thread you want to join." },
    "agent_id": { "type": "string", "description": "Your unique Agent ID." }
  },
  "required": ["thread_id", "agent_id"]
}
```

**Output:** Confirmation message. The joining agent and existing participants are notified. Returns a specific status if the agent is already a participant.

### 5. `broadcast_message`

Sends the same message content to multiple specified agents simultaneously. Useful for announcements or requests to a group without creating a persistent thread.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "sender_id": { "type": "string", "description": "Your unique Agent ID." },
    "recipient_ids": { "type": "array", "items": { "type": "string" }, "minItems": 1, "description": "List of unique Agent IDs of the recipients." },
    "content": { "type": "string", "description": "The textual content of the message to send to all recipients." },
    "urgent": { "type": "boolean", "default": false, "description": "Set to true if this message requires immediate attention from all recipients." }
  },
  "required": ["sender_id", "recipient_ids", "content"]
}
```

**Output:** Confirmation message including the new message ID, and the created message object (which will list multiple recipients).

### 6. `check_new_messages`

Allows an agent to quickly determine if they have unread messages waiting.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "agent_id": { "type": "string", "description": "Your unique Agent ID." }
  },
  "required": ["agent_id"]
}
```

**Output:** A message indicating the number of unread messages for the agent.

### 7. `transfer_file`

Securely transfers a file from the sender's local filesystem to a server-managed location, making it accessible to the recipient via a notification message containing the server path.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "sender_id": { "type": "string", "description": "Your unique Agent ID." },
    "recipient_id": { "type": "string", "description": "The unique Agent ID of the recipient." },
    "source_path": { "type": "string", "description": "The absolute path to the file on your local filesystem that you want to send." },
    "destination_filename": { "type": "string", "description": "Optional: A specific filename for the file when the recipient accesses it (defaults to the original filename)." },
    "description": { "type": "string", "description": "Optional text description of the file being sent." }
  },
  "required": ["sender_id", "recipient_id", "source_path"]
}
```

**Output:** Confirmation message including the filename and the ID of the notification message sent to the recipient. The data payload includes the server path where the file is stored.

### 8. `get_thread_details`

Retrieves metadata about a specific discussion thread.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "thread_id": { "type": "string", "description": "The unique ID of the thread to retrieve details for." }
  },
  "required": ["thread_id"]
}
```

**Output:** An object containing the thread's ID, title, creator ID, participant list, and creation timestamp.

### 9. `add_participant_to_thread`

Adds a specified agent to an existing discussion thread. Requires the inviting agent to be a current participant.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "thread_id": { "type": "string", "description": "The unique ID of the thread." },
    "agent_id_to_add": { "type": "string", "description": "The unique Agent ID of the agent to add." },
    "inviting_agent_id": { "type": "string", "description": "Your unique Agent ID (must be a current participant)." }
  },
  "required": ["thread_id", "agent_id_to_add", "inviting_agent_id"]
}
```

**Output:** Confirmation message. The new participant is notified, and a message is posted to the thread.

### 10. `remove_participant_from_thread`

Removes a specified agent from an existing discussion thread. Requires the removing agent to be either the thread creator or the agent being removed. The creator cannot be removed.

**Input Schema:**

```json
{
  "type": "object",
  "properties": {
    "thread_id": { "type": "string", "description": "The unique ID of the thread." },
    "agent_id_to_remove": { "type": "string", "description": "The unique Agent ID of the agent to remove." },
    "removing_agent_id": { "type": "string", "description": "Your unique Agent ID (must be the thread creator or the agent being removed)." }
  },
  "required": ["thread_id", "agent_id_to_remove", "removing_agent_id"]
}
```

**Output:** Confirmation message. A message is posted to the thread indicating the removal.
