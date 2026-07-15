# Dashboard State Management Implementation

This document outlines the implementation plan for state management in the MCP dashboard.

## Overview

The state management implementation will provide:
- React Context for global state management
- WebSocket integration for real-time updates
- Data fetching and caching utilities
- Optimistic UI updates
- Type-safe state management

## Implementation Details

### 1. WebSocket Context

Create a WebSocket context for real-time communication:

```typescript
// src/contexts/WebSocketContext.tsx
"use client";

import React, { createContext, useContext, useEffect, useRef, useState } from "react";
import { toast } from "@/components/ui/use-toast";

// WebSocket message types
export interface WebSocketMessage {
  type: string;
  [key: string]: any;
}

// WebSocket context interface
interface WebSocketContextType {
  isConnected: boolean;
  lastMessage: MessageEvent<any> | null;
  sendMessage: (message: WebSocketMessage) => void;
  reconnect: () => void;
}

// Create context with default values
const WebSocketContext = createContext<WebSocketContextType>({
  isConnected: false,
  lastMessage: null,
  sendMessage: () => {},
  reconnect: () => {},
});

// WebSocket provider props
interface WebSocketProviderProps {
  children: React.ReactNode;
  url?: string;
  reconnectInterval?: number;
  maxReconnectAttempts?: number;
}

export const WebSocketProvider: React.FC<WebSocketProviderProps> = ({
  children,
  url = process.env.NEXT_PUBLIC_WS_URL || "ws://localhost:8081/ws",
  reconnectInterval = 3000,
  maxReconnectAttempts = 5,
}) => {
  // State
  const [isConnected, setIsConnected] = useState(false);
  const [lastMessage, setLastMessage] = useState<MessageEvent<any> | null>(null);
  
  // Refs
  const socketRef = useRef<WebSocket | null>(null);
  const reconnectAttemptsRef = useRef(0);
  const reconnectTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  
  // Connect to WebSocket
  const connect = () => {
    try {
      // Close existing connection if any
      if (socketRef.current) {
        socketRef.current.close();
      }
      
      // Create new WebSocket connection
      const socket = new WebSocket(url);
      
      // Connection opened
      socket.onopen = () => {
        console.log("WebSocket connected");
        setIsConnected(true);
        reconnectAttemptsRef.current = 0;
      };
      
      // Connection closed
      socket.onclose = (event) => {
        console.log("WebSocket disconnected", event);
        setIsConnected(false);
        
        // Attempt to reconnect
        if (reconnectAttemptsRef.current < maxReconnectAttempts) {
          reconnectTimeoutRef.current = setTimeout(() => {
            reconnectAttemptsRef.current++;
            connect();
          }, reconnectInterval);
        } else {
          toast({
            title: "Connection lost",
            description: "Unable to reconnect to the server. Please refresh the page.",
            variant: "destructive",
          });
        }
      };
      
      // Connection error
      socket.onerror = (error) => {
        console.error("WebSocket error:", error);
        toast({
          title: "Connection error",
          description: "There was an error with the WebSocket connection.",
          variant: "destructive",
        });
      };
      
      // Message received
      socket.onmessage = (event) => {
        setLastMessage(event);
      };
      
      // Store socket reference
      socketRef.current = socket;
    } catch (error) {
      console.error("WebSocket connection error:", error);
      toast({
        title: "Connection error",
        description: "Failed to establish WebSocket connection.",
        variant: "destructive",
      });
    }
  };
  
  // Send message
  const sendMessage = (message: WebSocketMessage) => {
    if (socketRef.current && socketRef.current.readyState === WebSocket.OPEN) {
      socketRef.current.send(JSON.stringify(message));
    } else {
      toast({
        title: "Connection error",
        description: "Not connected to the server. Trying to reconnect...",
        variant: "destructive",
      });
      
      // Try to reconnect
      reconnect();
    }
  };
  
  // Reconnect
  const reconnect = () => {
    reconnectAttemptsRef.current = 0;
    connect();
  };
  
  // Connect on mount
  useEffect(() => {
    connect();
    
    // Cleanup on unmount
    return () => {
      if (socketRef.current) {
        socketRef.current.close();
      }
      
      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current);
      }
    };
  }, [url]);
  
  return (
    <WebSocketContext.Provider
      value={{
        isConnected,
        lastMessage,
        sendMessage,
        reconnect,
      }}
    >
      {children}
    </WebSocketContext.Provider>
  );
};

// Custom hook for using WebSocket context
export const useWebSocket = () => useContext(WebSocketContext);
```

### 2. App Context

Create an app context for global state:

```typescript
// src/contexts/AppContext.tsx
"use client";

import React, { createContext, useContext, useReducer, useEffect } from "react";
import { useWebSocket } from "./WebSocketContext";
import { toast } from "@/components/ui/use-toast";
import { fetchAgents, fetchProjects, fetchWorkflows } from "@/lib/api";

// Define state types
export interface Agent {
  id: string;
  name: string;
  type: string;
  status: "online" | "offline" | "error";
  lastSeen: string;
  currentTask?: string;
  metadata?: Record<string, any>;
}

export interface Project {
  id: string;
  name: string;
  description: string;
  status: "active" | "completed" | "archived";
  createdAt: string;
  updatedAt: string;
  teamIds: string[];
  metadata?: Record<string, any>;
}

export interface Workflow {
  id: string;
  name: string;
  description: string;
  status: "active" | "paused" | "completed" | "failed";
  projectId: string;
  createdAt: string;
  updatedAt: string;
  progress: number;
  metadata?: Record<string, any>;
}

export interface AppState {
  agents: Agent[];
  projects: Project[];
  workflows: Workflow[];
  isLoading: {
    agents: boolean;
    projects: boolean;
    workflows: boolean;
  };
  error: {
    agents: string | null;
    projects: string | null;
    workflows: string | null;
  };
}

// Define action types
type AppAction =
  | { type: "SET_AGENTS"; payload: Agent[] }
  | { type: "ADD_AGENT"; payload: Agent }
  | { type: "UPDATE_AGENT"; payload: Agent }
  | { type: "REMOVE_AGENT"; payload: string }
  | { type: "SET_PROJECTS"; payload: Project[] }
  | { type: "ADD_PROJECT"; payload: Project }
  | { type: "UPDATE_PROJECT"; payload: Project }
  | { type: "REMOVE_PROJECT"; payload: string }
  | { type: "SET_WORKFLOWS"; payload: Workflow[] }
  | { type: "ADD_WORKFLOW"; payload: Workflow }
  | { type: "UPDATE_WORKFLOW"; payload: Workflow }
  | { type: "REMOVE_WORKFLOW"; payload: string }
  | { type: "SET_LOADING"; resource: "agents" | "projects" | "workflows"; payload: boolean }
  | { type: "SET_ERROR"; resource: "agents" | "projects" | "workflows"; payload: string | null };

// Initial state
const initialState: AppState = {
  agents: [],
  projects: [],
  workflows: [],
  isLoading: {
    agents: false,
    projects: false,
    workflows: false,
  },
  error: {
    agents: null,
    projects: null,
    workflows: null,
  },
};

// Create context
const AppContext = createContext<{
  state: AppState;
  dispatch: React.Dispatch<AppAction>;
  refreshAgents: () => Promise<void>;
  refreshProjects: () => Promise<void>;
  refreshWorkflows: () => Promise<void>;
}>({
  state: initialState,
  dispatch: () => null,
  refreshAgents: async () => {},
  refreshProjects: async () => {},
  refreshWorkflows: async () => {},
});

// Reducer function
const appReducer = (state: AppState, action: AppAction): AppState => {
  switch (action.type) {
    case "SET_AGENTS":
      return { ...state, agents: action.payload };
    case "ADD_AGENT":
      return { ...state, agents: [...state.agents, action.payload] };
    case "UPDATE_AGENT":
      return {
        ...state,
        agents: state.agents.map((agent) =>
          agent.id === action.payload.id ? action.payload : agent
        ),
      };
    case "REMOVE_AGENT":
      return {
        ...state,
        agents: state.agents.filter((agent) => agent.id !== action.payload),
      };
    case "SET_PROJECTS":
      return { ...state, projects: action.payload };
    case "ADD_PROJECT":
      return { ...state, projects: [...state.projects, action.payload] };
    case "UPDATE_PROJECT":
      return {
        ...state,
        projects: state.projects.map((project) =>
          project.id === action.payload.id ? action.payload : project
        ),
      };
    case "REMOVE_PROJECT":
      return {
        ...state,
        projects: state.projects.filter((project) => project.id !== action.payload),
      };
    case "SET_WORKFLOWS":
      return { ...state, workflows: action.payload };
    case "ADD_WORKFLOW":
      return { ...state, workflows: [...state.workflows, action.payload] };
    case "UPDATE_WORKFLOW":
      return {
        ...state,
        workflows: state.workflows.map((workflow) =>
          workflow.id === action.payload.id ? action.payload : workflow
        ),
      };
    case "REMOVE_WORKFLOW":
      return {
        ...state,
        workflows: state.workflows.filter((workflow) => workflow.id !== action.payload),
      };
    case "SET_LOADING":
      return {
        ...state,
        isLoading: {
          ...state.isLoading,
          [action.resource]: action.payload,
        },
      };
    case "SET_ERROR":
      return {
        ...state,
        error: {
          ...state.error,
          [action.resource]: action.payload,
        },
      };
    default:
      return state;
  }
};

// Provider component
export const AppProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [state, dispatch] = useReducer(appReducer, initialState);
  const { isConnected, lastMessage } = useWebSocket();

  // Fetch agents
  const refreshAgents = async () => {
    dispatch({ type: "SET_LOADING", resource: "agents", payload: true });
    try {
      const agents = await fetchAgents();
      dispatch({ type: "SET_AGENTS", payload: agents });
      dispatch({ type: "SET_ERROR", resource: "agents", payload: null });
    } catch (error) {
      console.error("Failed to fetch agents:", error);
      dispatch({
        type: "SET_ERROR",
        resource: "agents",
        payload: "Failed to load agents. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load agents. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "agents", payload: false });
    }
  };

  // Fetch projects
  const refreshProjects = async () => {
    dispatch({ type: "SET_LOADING", resource: "projects", payload: true });
    try {
      const projects = await fetchProjects();
      dispatch({ type: "SET_PROJECTS", payload: projects });
      dispatch({ type: "SET_ERROR", resource: "projects", payload: null });
    } catch (error) {
      console.error("Failed to fetch projects:", error);
      dispatch({
        type: "SET_ERROR",
        resource: "projects",
        payload: "Failed to load projects. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load projects. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "projects", payload: false });
    }
  };

  // Fetch workflows
  const refreshWorkflows = async () => {
    dispatch({ type: "SET_LOADING", resource: "workflows", payload: true });
    try {
      const workflows = await fetchWorkflows();
      dispatch({ type: "SET_WORKFLOWS", payload: workflows });
      dispatch({ type: "SET_ERROR", resource: "workflows", payload: null });
    } catch (error) {
      console.error("Failed to fetch workflows:", error);
      dispatch({
        type: "SET_ERROR",
        resource: "workflows",
        payload: "Failed to load workflows. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load workflows. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "workflows", payload: false });
    }
  };

  // Fetch initial data
  useEffect(() => {
    refreshAgents();
    refreshProjects();
    refreshWorkflows();
  }, []);

  // Handle WebSocket messages
  useEffect(() => {
    if (lastMessage) {
      try {
        const data = JSON.parse(lastMessage.data);
        
        switch (data.type) {
          case "agent_update":
            dispatch({ type: "UPDATE_AGENT", payload: data.agent });
            break;
          case "agent_added":
            dispatch({ type: "ADD_AGENT", payload: data.agent });
            toast({
              title: "New Agent Connected",
              description: `${data.agent.name} (${data.agent.type}) is now online`,
              variant: "default",
            });
            break;
          case "agent_removed":
            dispatch({ type: "REMOVE_AGENT", payload: data.agentId });
            break;
          case "project_update":
            dispatch({ type: "UPDATE_PROJECT", payload: data.project });
            break;
          case "project_added":
            dispatch({ type: "ADD_PROJECT", payload: data.project });
            toast({
              title: "New Project Created",
              description: `Project "${data.project.name}" has been created`,
              variant: "default",
            });
            break;
          case "project_removed":
            dispatch({ type: "REMOVE_PROJECT", payload: data.projectId });
            break;
          case "workflow_update":
            dispatch({ type: "UPDATE_WORKFLOW", payload: data.workflow });
            break;
          case "workflow_added":
            dispatch({ type: "ADD_WORKFLOW", payload: data.workflow });
            toast({
              title: "New Workflow Created",
              description: `Workflow "${data.workflow.name}" has been created`,
              variant: "default",
            });
            break;
          case "workflow_removed":
            dispatch({ type: "REMOVE_WORKFLOW", payload: data.workflowId });
            break;
          default:
            // Ignore unknown message types
            break;
        }
      } catch (error) {
        console.error("Error parsing WebSocket message:", error);
      }
    }
  }, [lastMessage]);

  // Show connection status changes
  useEffect(() => {
    if (isConnected) {
      toast({
        title: "Connected to server",
        description: "Real-time updates are now active",
        variant: "default",
      });
    } else {
      toast({
        title: "Disconnected from server",
        description: "Attempting to reconnect...",
        variant: "destructive",
      });
    }
  }, [isConnected]);

  return (
    <AppContext.Provider
      value={{
        state,
        dispatch,
        refreshAgents,
        refreshProjects,
        refreshWorkflows,
      }}
    >
      {children}
    </AppContext.Provider>
  );
};

// Custom hook for using the context
export const useApp = () => useContext(AppContext);
```

### 3. Theme Context

Create a theme context for managing theme preferences:

```typescript
// src/contexts/ThemeContext.tsx
"use client";

import React, { createContext, useContext, useEffect, useState } from "react";

type Theme = "light" | "dark" | "system";

interface ThemeContextType {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

const ThemeContext = createContext<ThemeContextType>({
  theme: "system",
  setTheme: () => null,
});

export const ThemeProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [theme, setTheme] = useState<Theme>("system");

  // Initialize theme from localStorage or default to system
  useEffect(() => {
    const storedTheme = localStorage.getItem("theme") as Theme | null;
    if (storedTheme) {
      setTheme(storedTheme);
    }
  }, []);

  // Update theme when it changes
  useEffect(() => {
    const root = window.document.documentElement;
    
    // Remove all theme classes
    root.classList.remove("light", "dark");
    
    // Add appropriate theme class
    if (theme === "system") {
      const systemTheme = window.matchMedia("(prefers-color-scheme: dark)")
        .matches
        ? "dark"
        : "light";
      root.classList.add(systemTheme);
    } else {
      root.classList.add(theme);
    }
    
    // Store theme preference
    localStorage.setItem("theme", theme);
  }, [theme]);

  // Listen for system theme changes
  useEffect(() => {
    const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
    
    const handleChange = () => {
      if (theme === "system") {
        const root = window.document.documentElement;
        root.classList.remove("light", "dark");
        root.classList.add(mediaQuery.matches ? "dark" : "light");
      }
    };
    
    mediaQuery.addEventListener("change", handleChange);
    return () => mediaQuery.removeEventListener("change", handleChange);
  }, [theme]);

  return (
    <ThemeContext.Provider value={{ theme, setTheme }}>
      {children}
    </ThemeContext.Provider>
  );
};

export const useTheme = () => useContext(ThemeContext);
```

### 4. Toast Context

Create a toast context for notifications:

```typescript
// src/contexts/ToastContext.tsx
"use client";

import React, { createContext, useContext, useState } from "react";
import {
  Toast,
  ToastClose,
  ToastDescription,
  ToastProvider,
  ToastTitle,
  ToastViewport,
} from "@/components/ui/toast";

export interface ToastProps {
  id: string;
  title?: string;
  description?: string;
  action?: React.ReactNode;
  variant?: "default" | "destructive" | "success" | "warning";
}

interface ToastContextType {
  toasts: ToastProps[];
  addToast: (toast: Omit<ToastProps, "id">) => void;
  dismissToast: (id: string) => void;
}

const ToastContext = createContext<ToastContextType>({
  toasts: [],
  addToast: () => null,
  dismissToast: () => null,
});

export const ToastProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [toasts, setToasts] = useState<ToastProps[]>([]);

  const addToast = (toast: Omit<ToastProps, "id">) => {
    const id = Math.random().toString(36).substring(2, 9);
    setToasts((prev) => [...prev, { id, ...toast }]);
    
    // Auto-dismiss after 5 seconds
    setTimeout(() => {
      dismissToast(id);
    }, 5000);
  };

  const dismissToast = (id: string) => {
    setToasts((prev) => prev.filter((toast) => toast.id !== id));
  };

  return (
    <ToastContext.Provider value={{ toasts, addToast, dismissToast }}>
      <ToastProvider>
        {children}
        {toasts.map(({ id, title, description, action, variant }) => (
          <Toast key={id} variant={variant}>
            {title && <ToastTitle>{title}</ToastTitle>}
            {description && <ToastDescription>{description}</ToastDescription>}
            {action}
            <ToastClose onClick={() => dismissToast(id)} />
          </Toast>
        ))}
        <ToastViewport />
      </ToastProvider>
    </ToastContext.Provider>
  );
};

export const useToast = () => {
  const { addToast, dismissToast } = useContext(ToastContext);
  
  return {
    toast: addToast,
    dismiss: dismissToast,
  };
};
```

### 5. API Client

Create an API client for data fetching:

```typescript
// src/lib/api/index.ts
import { Agent, Project, Workflow } from "@/contexts/AppContext";

// API base URL
const API_BASE_URL = process.env.NEXT_PUBLIC_API_URL || "http://localhost:8081/api";

// Fetch options with credentials
const fetchOptions: RequestInit = {
  credentials: "include",
  headers: {
    "Content-Type": "application/json",
  },
};

// Generic fetch function with error handling
async function fetchApi<T>(
  endpoint: string,
  options: RequestInit = {}
): Promise<T> {
  try {
    const response = await fetch(`${API_BASE_URL}${endpoint}`, {
      ...fetchOptions,
      ...options,
    });

    if (!response.ok) {
      const errorData = await response.json().catch(() => ({}));
      throw new Error(
        errorData.message || `API error: ${response.status} ${response.statusText}`
      );
    }

    return await response.json();
  } catch (error) {
    console.error(`API error for ${endpoint}:`, error);
    throw error;
  }
}

// Agents API
export async function fetchAgents(): Promise<Agent[]> {
  return fetchApi<Agent[]>("/agents");
}

export async function fetchAgent(id: string): Promise<Agent> {
  return fetchApi<Agent>(`/agents/${id}`);
}

export async function createAgent(agent: Omit<Agent, "id">): Promise<Agent> {
  return fetchApi<Agent>("/agents", {
    method: "POST",
    body: JSON.stringify(agent),
  });
}

export async function updateAgent(
  id: string,
  updates: Partial<Agent>
): Promise<Agent> {
  return fetchApi<Agent>(`/agents/${id}`, {
    method: "PUT",
    body: JSON.stringify(updates),
  });
}

export async function deleteAgent(id: string): Promise<void> {
  return fetchApi<void>(`/agents/${id}`, {
    method: "DELETE",
  });
}

// Projects API
export async function fetchProjects(): Promise<Project[]> {
  return fetchApi<Project[]>("/projects");
}

export async function fetchProject(id: string): Promise<Project> {
  return fetchApi<Project>(`/projects/${id}`);
}

export async function createProject(
  project: Omit<Project, "id" | "createdAt" | "updatedAt">
): Promise<Project> {
  return fetchApi<Project>("/projects", {
    method: "POST",
    body: JSON.stringify(project),
  });
}

export async function updateProject(
  id: string,
  updates: Partial<Project>
): Promise<Project> {
  return fetchApi<Project>(`/projects/${id}`, {
    method: "PUT",
    body: JSON.stringify(updates),
  });
}

export async function deleteProject(id: string): Promise<void> {
  return fetchApi<void>(`/projects/${id}`, {
    method: "DELETE",
  });
}

// Workflows API
export async function fetchWorkflows(): Promise<Workflow[]> {
  return fetchApi<Workflow[]>("/workflows");
}

export async function fetchWorkflow(id: string): Promise<Workflow> {
  return fetchApi<Workflow>(`/workflows/${id}`);
}

export async function createWorkflow(
  workflow: Omit<Workflow, "id" | "createdAt" | "updatedAt">
): Promise<Workflow> {
  return fetchApi<Workflow>("/workflows", {
    method: "POST",
    body: JSON.stringify(workflow),
  });
}

export async function updateWorkflow(
  id: string,
  updates: Partial<Workflow>
): Promise<Workflow> {
  return fetchApi<Workflow>(`/workflows/${id}`, {
    method: "PUT",
    body: JSON.stringify(updates),
  });
}

export async function deleteWorkflow(id: string): Promise<void> {
  return fetchApi<void>(`/workflows/${id}`, {
    method: "DELETE",
  });
}
```

## Implementation Steps

1. Create the WebSocket context
2. Implement the app context
3. Create the theme context
4. Implement the toast context
5. Create the API client
6. Document usage patterns for the team
