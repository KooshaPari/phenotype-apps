# Thread Notification System

## Overview

This document describes the thread notification system implemented in the MCP framework. The system ensures that agents are automatically notified when they are added to threads or when changes occur in threads they participate in.

## Key Components

### 1. Automatic Thread Participant Notifications

When an agent creates a thread or adds other agents to a thread, all participants receive an automatic notification. This solves the previous issue where agents had no automatic way of knowing they were added to a thread.

### 2. Thread Join Notifications

When an agent joins a thread using the `join_thread` tool, both the joining agent and existing thread participants are notified.

### 3. Critical Thread Communication

A new tool `create_critical_thread` has been added that allows for creation of threads that require responses. This tool:
- Creates a thread with all the specified participants
- Sends immediate notifications to all participants
- Optionally waits for a specified number of responses before returning
- Supports timeout configuration

## Implementation Details

### Notification Functions

A centralized notification function `notifyThreadParticipants` has been implemented to standardize notification formats and ensure consistency.

```typescript
async function notifyThreadParticipants(
    thread_id: string,
    thread_title: string,
    creator_id: string,
    participants: string[],
    isNewThread: boolean = true
) {
    // For each participant (except the creator)
    for (const participant_id of participants.filter(id => id !== creator_id)) {
        // Send a notification message
        await storeMessage({
            sender_id: SYSTEM_NOTIFICATION_SENDER,
            recipient_id: participant_id,
            content: isNewThread 
                ? `You have been added to a new thread "${thread_title}" (${thread_id}) by ${creator_id}.`
                : `You have been added to the existing thread "${thread_title}" (${thread_id}) by ${creator_id}.`,
            urgent: true
        });
    }
}
```

### Notification Scenarios

1. **Thread Creation**:
   - When a thread is created, all participants receive notifications
   - Uses `notifyThreadParticipants` function

2. **Join Thread**:
   - When an agent joins a thread, they receive a confirmation notification
   - All existing thread participants are notified about the new member

3. **Add Participant**:
   - When an agent adds another agent to a thread, the added agent is notified
   - Uses `notifyThreadParticipants` function with `isNewThread = false`

### Critical Thread Communication

The new `create_critical_thread` tool extends thread functionality to support blocking operations where responses are required:

```typescript
server.tool(
    'create_critical_thread',
    {
        creator_id: z.string().describe("Your unique Agent ID."),
        title: z.string().describe("A concise and descriptive title for the discussion thread."),
        participants: z.array(z.string()).describe("List of unique Agent IDs of all initial participants (including yourself)."),
        initial_message: z.string().describe("First message to post in the newly created thread."),
        wait_for_responses: z.number().optional().describe("Number of participant responses to wait for (default: 1)."),
        timeout_ms: z.number().optional().describe("Timeout in milliseconds to wait for responses (default: 60000, max: 600000).")
    },
    async ({ creator_id, title, participants, initial_message, wait_for_responses = 1, timeout_ms }) => {
        // Implementation details...
    }
)
```

This tool leverages the event-based messaging system to wait for responses from thread participants.

#### How Critical Threads Work

Critical threads differ from regular threads in several ways:

1. **Urgent Notifications**: When a critical thread is created, all participants receive notifications with the `urgent` flag set to true.

2. **Blocking Option**: The creator can choose to wait for a specified number of responses before the function returns.

3. **Timeout Management**: The creator can specify a timeout period after which the function will return even if the number of required responses haven't been received.

4. **Partial Response Handling**: If at least one response is received within the timeout period, but fewer than the requested number, the function will still return with the partial list of responses.

#### Internal Mechanism

The critical thread feature uses an event-driven architecture:

1. The `messageEmitter` event emitter broadcasts events when new messages are posted to threads.

2. The `waitForMultipleThreadResponses` function listens for these events and collects responses that match specific criteria:
   - Message is from a valid responder (one of the participants excluding the sender)
   - Message is sent to the correct thread
   - Responder hasn't already responded (only one response per participant is counted)
   - Message timestamp indicates it's a new message

3. When enough responses are collected or the timeout is reached, the function resolves with the responses.

#### Response Collection Logic

```typescript
async function waitForMultipleThreadResponses(
    messageId: string, 
    threadId: string, 
    senderId: string, 
    validResponders: string[],
    responsesNeeded: number,
    timeoutMs: number
): Promise<Message[]> {
    return new Promise((resolve, reject) => {
        const responses: Message[] = [];
        const respondents = new Set<string>();
        
        // Set timeout for the operation
        const timeoutId = setTimeout(() => {
            cleanup();
            if (responses.length > 0) {
                // Return partial responses if we have any
                resolve(responses);
            } else {
                reject(new Error(`Waiting for ${responsesNeeded} responses timed out`));
            }
        }, timeoutMs);
        
        // Handle thread message event and collect valid responses
        const handleThreadMessage = (message: Message) => {
            if (
                message.thread_id === threadId && 
                message.sender_id !== senderId && 
                validResponders.includes(message.sender_id) &&
                !respondents.has(message.sender_id)
            ) {
                responses.push(message);
                respondents.add(message.sender_id);
                
                if (responses.length >= responsesNeeded) {
                    cleanup();
                    resolve(responses);
                }
            }
        };
        
        // Listen for new messages in the thread
        messageEmitter.on(`thread:${threadId}`, handleThreadMessage);
        
        // Cleanup function
        const cleanup = () => {
            clearTimeout(timeoutId);
            messageEmitter.removeListener(`thread:${threadId}`, handleThreadMessage);
        };
    });
}
```

#### Error Handling and Timeouts

- If no responses are received when the timeout is reached, the function rejects with an error.
- If partial responses are received (more than 0 but fewer than requested), the function resolves with the partial list.
- If an error occurs during response collection, detailed error information is returned with the thread data.

#### Best Practices for Critical Threads

1. **Set Appropriate Timeouts**: Choose timeout values that match the urgency of your request and allow participants sufficient time to respond.

2. **Consider Response Requirements**: For critical decisions, require responses from the key stakeholders. For informational threads, one or two responses may be sufficient.

3. **Handle Timeout Cases**: Always check the returned data to see if you received all expected responses or only partial responses.

4. **Use Urgent Flag**: All messages in critical threads have the `urgent` flag set to ensure prompt attention.

## Usage Examples

### Regular Thread Creation with Notifications

```typescript
const result = await mcp.invoke('create_thread', {
    creator_id: 'agent-1',
    title: 'Discussion on Project Architecture',
    participants: ['agent-2', 'agent-3'],
    initial_message: 'Let\'s discuss the architecture for our new project.'
});
// All participants will automatically receive notifications
```

### Critical Thread Creation

```typescript
const result = await mcp.invoke('create_critical_thread', {
    creator_id: 'agent-1',
    title: 'Urgent: System Deployment Decision',
    participants: ['agent-2', 'agent-3', 'agent-4'],
    initial_message: 'We need to decide on the deployment strategy. Please provide your input.',
    wait_for_responses: 2,  // Wait for at least 2 participants to respond
    timeout_ms: 300000      // Wait up to 5 minutes
});

// This will block until 2 participants respond or the timeout is reached
// All responses will be included in the result
```

## Benefits

This notification system ensures:

1. No agent is left unaware of threads they're part of
2. Critical communications can be assured through the waiting mechanism
3. Consistent notification format across all thread-related operations
4. Clear indication of who added whom to which threads

## Future Enhancements

Potential future enhancements to the notification system:

1. Thread digest notifications for inactive agents
2. Customizable notification templates
3. Notification filtering options
4. Read receipts for critical communications
