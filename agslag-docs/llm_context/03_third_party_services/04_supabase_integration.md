# Supabase Integration Guide

## Overview
Supabase is an open-source Firebase alternative providing:
- PostgreSQL database
- Authentication
- Storage
- Realtime subscriptions
- Edge Functions

## Setup and Authentication
```javascript
import { createClient } from '@supabase/supabase-js'

const supabase = createClient(
  'https://your-project-url.supabase.co',
  'your-anon-key'
)
```

## Core Features

### Database
- PostgreSQL with extensions
- Row-level security (RLS)
- Foreign key relationships
- Full-text search
- Stored procedures

```javascript
// Query data
const { data, error } = await supabase
  .from('agents')
  .select('id, name, role, status')
  .eq('status', 'available')
  .order('created_at', { ascending: false })
```

### Authentication
- Email/password authentication
- Social providers (Google, GitHub, etc.)
- Phone authentication
- JWT tokens
- User management

```javascript
// Sign up a new user
const { data, error } = await supabase.auth.signUp({
  email: 'user@example.com',
  password: 'secure-password',
  options: {
    data: {
      role: 'agent-developer'
    }
  }
})
```

### Storage
- File upload/download
- Public/private buckets
- Image transformations
- Access control

```javascript
// Upload file
const { data, error } = await supabase.storage
  .from('agent-artifacts')
  .upload('path/to/file.jpg', file)
```

### Realtime
- Database changes
- Broadcast
- Presence
- Custom channels

```javascript
// Subscribe to changes
const channel = supabase
  .channel('agent-status-changes')
  .on('postgres_changes', 
    { event: 'UPDATE', schema: 'public', table: 'agents' },
    (payload) => {
      console.log('Agent status changed:', payload)
    }
  )
  .subscribe()
```

## Schema Design for MCP

### Agents Table
```sql
CREATE TABLE agents (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  role TEXT NOT NULL,
  model_type TEXT NOT NULL,
  model_version TEXT NOT NULL,
  capabilities JSONB,
  status TEXT NOT NULL,
  current_task TEXT,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  last_heartbeat TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Row-level security
ALTER TABLE agents ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Agents are viewable by authenticated users"
  ON agents FOR SELECT
  USING (auth.role() = 'authenticated');
```

### Messages Table
```sql
CREATE TABLE messages (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  sender_id UUID REFERENCES agents(id),
  recipient_id UUID REFERENCES agents(id),
  thread_id UUID,
  content TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  is_read BOOLEAN DEFAULT FALSE
);
```

### Workflows Table
```sql
CREATE TABLE workflows (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name TEXT NOT NULL,
  description TEXT,
  steps JSONB NOT NULL,
  creator_id UUID REFERENCES agents(id),
  status TEXT NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

## API Example with RLS

```javascript
// Database functions with RLS
async function getAgentsByRole(role) {
  const { data, error } = await supabase
    .from('agents')
    .select('*')
    .eq('role', role)
  
  if (error) throw error
  return data
}

async function updateAgentStatus(agentId, status) {
  const { data, error } = await supabase
    .from('agents')
    .update({ status, last_heartbeat: new Date() })
    .eq('id', agentId)
    .select()
  
  if (error) throw error
  return data
}
```

## Supabase Functions (Edge Functions)

```typescript
// supabase/functions/agent-heartbeat/index.ts
import { serve } from 'https://deno.land/std@0.177.0/http/server.ts'
import { createClient } from 'https://esm.sh/@supabase/supabase-js@2.7.1'

serve(async (req) => {
  const { agent_id } = await req.json()
  
  const supabaseClient = createClient(
    Deno.env.get('SUPABASE_URL') ?? '',
    Deno.env.get('SUPABASE_ANON_KEY') ?? '',
    { global: { headers: { Authorization: req.headers.get('Authorization')! } } }
  )
  
  const { data, error } = await supabaseClient
    .from('agents')
    .update({ last_heartbeat: new Date() })
    .eq('id', agent_id)
    .select()
  
  if (error) {
    return new Response(JSON.stringify({ error: error.message }), {
      headers: { 'Content-Type': 'application/json' },
      status: 400,
    })
  }
  
  return new Response(JSON.stringify({ data }), {
    headers: { 'Content-Type': 'application/json' },
    status: 200,
  })
})
```