# Project Management Interface Implementation - Part 1

This document outlines the implementation plan for the project management interface in the MCP dashboard.

## Overview

The project management interface will provide:
- Project creation and management
- Team assignment and management
- Task tracking and visualization
- Timeline visualization with Gantt charts
- Resource allocation and monitoring
- Project analytics and reporting

## Implementation Details

### 1. Project Types and Interfaces

Create type definitions for project-related data:

```typescript
// src/types/project.ts
import { Agent } from "@/contexts/AppContext";

// Project status types
export type ProjectStatus = "active" | "completed" | "archived" | "draft";

// Task status types
export type TaskStatus = "todo" | "in_progress" | "review" | "done" | "blocked";

// Task priority types
export type TaskPriority = "low" | "medium" | "high" | "urgent";

// Team role types
export type TeamRole = "lead" | "member" | "reviewer" | "observer";

// Project interface
export interface Project {
  id: string;
  name: string;
  description: string;
  status: ProjectStatus;
  createdAt: string;
  updatedAt: string;
  startDate: string;
  endDate: string | null;
  teamIds: string[];
  metadata?: Record<string, any>;
}

// Team interface
export interface Team {
  id: string;
  name: string;
  description: string;
  members: TeamMember[];
  createdAt: string;
  updatedAt: string;
}

// Team member interface
export interface TeamMember {
  agentId: string;
  role: TeamRole;
  joinedAt: string;
}

// Task interface
export interface Task {
  id: string;
  projectId: string;
  title: string;
  description: string;
  status: TaskStatus;
  priority: TaskPriority;
  assigneeId: string | null;
  createdAt: string;
  updatedAt: string;
  startDate: string | null;
  dueDate: string | null;
  completedAt: string | null;
  estimatedHours: number | null;
  actualHours: number | null;
  dependencies: string[]; // IDs of tasks this task depends on
  tags: string[];
  metadata?: Record<string, any>;
}

// Milestone interface
export interface Milestone {
  id: string;
  projectId: string;
  title: string;
  description: string;
  date: string;
  completed: boolean;
  tasks: string[]; // IDs of tasks associated with this milestone
}

// Project statistics interface
export interface ProjectStats {
  totalTasks: number;
  completedTasks: number;
  inProgressTasks: number;
  blockedTasks: number;
  todoTasks: number;
  reviewTasks: number;
  completionPercentage: number;
  onTrack: boolean;
  daysRemaining: number | null;
  estimatedTotalHours: number;
  actualTotalHours: number;
}

// Project with related data
export interface ProjectWithRelations extends Project {
  teams: Team[];
  tasks: Task[];
  milestones: Milestone[];
  stats: ProjectStats;
}

// Helper function to calculate project statistics
export function calculateProjectStats(
  project: Project,
  tasks: Task[]
): ProjectStats {
  const projectTasks = tasks.filter((task) => task.projectId === project.id);
  
  const totalTasks = projectTasks.length;
  const completedTasks = projectTasks.filter((task) => task.status === "done").length;
  const inProgressTasks = projectTasks.filter((task) => task.status === "in_progress").length;
  const blockedTasks = projectTasks.filter((task) => task.status === "blocked").length;
  const todoTasks = projectTasks.filter((task) => task.status === "todo").length;
  const reviewTasks = projectTasks.filter((task) => task.status === "review").length;
  
  const completionPercentage = totalTasks > 0 ? (completedTasks / totalTasks) * 100 : 0;
  
  const estimatedTotalHours = projectTasks.reduce(
    (total, task) => total + (task.estimatedHours || 0),
    0
  );
  
  const actualTotalHours = projectTasks.reduce(
    (total, task) => total + (task.actualHours || 0),
    0
  );
  
  // Calculate days remaining
  let daysRemaining = null;
  if (project.endDate) {
    const endDate = new Date(project.endDate);
    const today = new Date();
    const diffTime = endDate.getTime() - today.getTime();
    daysRemaining = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
  }
  
  // Determine if project is on track
  const onTrack = completionPercentage >= 
    (daysRemaining !== null ? 
      (100 - (daysRemaining / getDurationDays(project) * 100)) : 0);
  
  return {
    totalTasks,
    completedTasks,
    inProgressTasks,
    blockedTasks,
    todoTasks,
    reviewTasks,
    completionPercentage,
    onTrack,
    daysRemaining,
    estimatedTotalHours,
    actualTotalHours,
  };
}

// Helper function to get project duration in days
export function getDurationDays(project: Project): number {
  const startDate = new Date(project.startDate);
  const endDate = project.endDate ? new Date(project.endDate) : new Date();
  const diffTime = endDate.getTime() - startDate.getTime();
  return Math.ceil(diffTime / (1000 * 60 * 60 * 24));
}

// Helper function to get task duration in days
export function getTaskDurationDays(task: Task): number {
  if (!task.startDate || !task.dueDate) return 0;
  
  const startDate = new Date(task.startDate);
  const dueDate = new Date(task.dueDate);
  const diffTime = dueDate.getTime() - startDate.getTime();
  return Math.ceil(diffTime / (1000 * 60 * 60 * 24));
}

// Helper function to check if a task is overdue
export function isTaskOverdue(task: Task): boolean {
  if (!task.dueDate || task.status === "done") return false;
  
  const dueDate = new Date(task.dueDate);
  const today = new Date();
  return dueDate < today;
}

// Helper function to get task completion percentage
export function getTaskCompletionPercentage(task: Task): number {
  switch (task.status) {
    case "todo":
      return 0;
    case "in_progress":
      return 50;
    case "review":
      return 80;
    case "done":
      return 100;
    case "blocked":
      return task.actualHours && task.estimatedHours
        ? (task.actualHours / task.estimatedHours) * 100
        : 25;
    default:
      return 0;
  }
}
```

### 2. Project API Client

Create API client functions for project management:

```typescript
// src/lib/api/project-api.ts
import { Project, Team, Task, Milestone, ProjectWithRelations } from "@/types/project";

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

// Projects API
export async function fetchProjects(): Promise<Project[]> {
  return fetchApi<Project[]>("/projects");
}

export async function fetchProject(id: string): Promise<Project> {
  return fetchApi<Project>(`/projects/${id}`);
}

export async function fetchProjectWithRelations(id: string): Promise<ProjectWithRelations> {
  return fetchApi<ProjectWithRelations>(`/projects/${id}/relations`);
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

// Teams API
export async function fetchTeams(): Promise<Team[]> {
  return fetchApi<Team[]>("/teams");
}

export async function fetchTeam(id: string): Promise<Team> {
  return fetchApi<Team>(`/teams/${id}`);
}

export async function fetchProjectTeams(projectId: string): Promise<Team[]> {
  return fetchApi<Team[]>(`/projects/${projectId}/teams`);
}

export async function createTeam(
  team: Omit<Team, "id" | "createdAt" | "updatedAt">
): Promise<Team> {
  return fetchApi<Team>("/teams", {
    method: "POST",
    body: JSON.stringify(team),
  });
}

export async function updateTeam(
  id: string,
  updates: Partial<Team>
): Promise<Team> {
  return fetchApi<Team>(`/teams/${id}`, {
    method: "PUT",
    body: JSON.stringify(updates),
  });
}

export async function deleteTeam(id: string): Promise<void> {
  return fetchApi<void>(`/teams/${id}`, {
    method: "DELETE",
  });
}

export async function addTeamMember(
  teamId: string,
  agentId: string,
  role: string
): Promise<Team> {
  return fetchApi<Team>(`/teams/${teamId}/members`, {
    method: "POST",
    body: JSON.stringify({ agentId, role }),
  });
}

export async function removeTeamMember(
  teamId: string,
  agentId: string
): Promise<Team> {
  return fetchApi<Team>(`/teams/${teamId}/members/${agentId}`, {
    method: "DELETE",
  });
}

// Tasks API
export async function fetchTasks(): Promise<Task[]> {
  return fetchApi<Task[]>("/tasks");
}

export async function fetchTask(id: string): Promise<Task> {
  return fetchApi<Task>(`/tasks/${id}`);
}

export async function fetchProjectTasks(projectId: string): Promise<Task[]> {
  return fetchApi<Task[]>(`/projects/${projectId}/tasks`);
}

export async function createTask(
  task: Omit<Task, "id" | "createdAt" | "updatedAt">
): Promise<Task> {
  return fetchApi<Task>("/tasks", {
    method: "POST",
    body: JSON.stringify(task),
  });
}

export async function updateTask(
  id: string,
  updates: Partial<Task>
): Promise<Task> {
  return fetchApi<Task>(`/tasks/${id}`, {
    method: "PUT",
    body: JSON.stringify(updates),
  });
}

export async function deleteTask(id: string): Promise<void> {
  return fetchApi<void>(`/tasks/${id}`, {
    method: "DELETE",
  });
}

// Milestones API
export async function fetchMilestones(): Promise<Milestone[]> {
  return fetchApi<Milestone[]>("/milestones");
}

export async function fetchMilestone(id: string): Promise<Milestone> {
  return fetchApi<Milestone>(`/milestones/${id}`);
}

export async function fetchProjectMilestones(projectId: string): Promise<Milestone[]> {
  return fetchApi<Milestone[]>(`/projects/${projectId}/milestones`);
}

export async function createMilestone(
  milestone: Omit<Milestone, "id">
): Promise<Milestone> {
  return fetchApi<Milestone>("/milestones", {
    method: "POST",
    body: JSON.stringify(milestone),
  });
}

export async function updateMilestone(
  id: string,
  updates: Partial<Milestone>
): Promise<Milestone> {
  return fetchApi<Milestone>(`/milestones/${id}`, {
    method: "PUT",
    body: JSON.stringify(updates),
  });
}

export async function deleteMilestone(id: string): Promise<void> {
  return fetchApi<void>(`/milestones/${id}`, {
    method: "DELETE",
  });
}
```

### 3. Project Context

Create a context for project management:

```typescript
// src/contexts/ProjectContext.tsx
"use client";

import React, { createContext, useContext, useReducer, useEffect } from "react";
import { useToast } from "@/components/ui/use-toast";
import { Project, Team, Task, Milestone } from "@/types/project";
import * as projectApi from "@/lib/api/project-api";

// Define state types
interface ProjectState {
  projects: Project[];
  teams: Team[];
  tasks: Task[];
  milestones: Milestone[];
  selectedProjectId: string | null;
  isLoading: {
    projects: boolean;
    teams: boolean;
    tasks: boolean;
    milestones: boolean;
  };
  error: {
    projects: string | null;
    teams: string | null;
    tasks: string | null;
    milestones: string | null;
  };
}

// Define action types
type ProjectAction =
  | { type: "SET_PROJECTS"; payload: Project[] }
  | { type: "ADD_PROJECT"; payload: Project }
  | { type: "UPDATE_PROJECT"; payload: Project }
  | { type: "REMOVE_PROJECT"; payload: string }
  | { type: "SET_TEAMS"; payload: Team[] }
  | { type: "ADD_TEAM"; payload: Team }
  | { type: "UPDATE_TEAM"; payload: Team }
  | { type: "REMOVE_TEAM"; payload: string }
  | { type: "SET_TASKS"; payload: Task[] }
  | { type: "ADD_TASK"; payload: Task }
  | { type: "UPDATE_TASK"; payload: Task }
  | { type: "REMOVE_TASK"; payload: string }
  | { type: "SET_MILESTONES"; payload: Milestone[] }
  | { type: "ADD_MILESTONE"; payload: Milestone }
  | { type: "UPDATE_MILESTONE"; payload: Milestone }
  | { type: "REMOVE_MILESTONE"; payload: string }
  | { type: "SET_SELECTED_PROJECT"; payload: string | null }
  | { type: "SET_LOADING"; resource: keyof ProjectState["isLoading"]; payload: boolean }
  | { type: "SET_ERROR"; resource: keyof ProjectState["error"]; payload: string | null };

// Initial state
const initialState: ProjectState = {
  projects: [],
  teams: [],
  tasks: [],
  milestones: [],
  selectedProjectId: null,
  isLoading: {
    projects: false,
    teams: false,
    tasks: false,
    milestones: false,
  },
  error: {
    projects: null,
    teams: null,
    tasks: null,
    milestones: null,
  },
};

// Create context
const ProjectContext = createContext<{
  state: ProjectState;
  dispatch: React.Dispatch<ProjectAction>;
  refreshProjects: () => Promise<void>;
  refreshTeams: () => Promise<void>;
  refreshTasks: () => Promise<void>;
  refreshMilestones: () => Promise<void>;
  selectProject: (projectId: string | null) => void;
  createNewProject: (project: Omit<Project, "id" | "createdAt" | "updatedAt">) => Promise<Project>;
  updateExistingProject: (id: string, updates: Partial<Project>) => Promise<Project>;
  createNewTask: (task: Omit<Task, "id" | "createdAt" | "updatedAt">) => Promise<Task>;
  updateExistingTask: (id: string, updates: Partial<Task>) => Promise<Task>;
}>({
  state: initialState,
  dispatch: () => null,
  refreshProjects: async () => {},
  refreshTeams: async () => {},
  refreshTasks: async () => {},
  refreshMilestones: async () => {},
  selectProject: () => {},
  createNewProject: async () => ({} as Project),
  updateExistingProject: async () => ({} as Project),
  createNewTask: async () => ({} as Task),
  updateExistingTask: async () => ({} as Task),
});

// Reducer function
const projectReducer = (state: ProjectState, action: ProjectAction): ProjectState => {
  switch (action.type) {
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
    case "SET_TEAMS":
      return { ...state, teams: action.payload };
    case "ADD_TEAM":
      return { ...state, teams: [...state.teams, action.payload] };
    case "UPDATE_TEAM":
      return {
        ...state,
        teams: state.teams.map((team) =>
          team.id === action.payload.id ? action.payload : team
        ),
      };
    case "REMOVE_TEAM":
      return {
        ...state,
        teams: state.teams.filter((team) => team.id !== action.payload),
      };
    case "SET_TASKS":
      return { ...state, tasks: action.payload };
    case "ADD_TASK":
      return { ...state, tasks: [...state.tasks, action.payload] };
    case "UPDATE_TASK":
      return {
        ...state,
        tasks: state.tasks.map((task) =>
          task.id === action.payload.id ? action.payload : task
        ),
      };
    case "REMOVE_TASK":
      return {
        ...state,
        tasks: state.tasks.filter((task) => task.id !== action.payload),
      };
    case "SET_MILESTONES":
      return { ...state, milestones: action.payload };
    case "ADD_MILESTONE":
      return { ...state, milestones: [...state.milestones, action.payload] };
    case "UPDATE_MILESTONE":
      return {
        ...state,
        milestones: state.milestones.map((milestone) =>
          milestone.id === action.payload.id ? action.payload : milestone
        ),
      };
    case "REMOVE_MILESTONE":
      return {
        ...state,
        milestones: state.milestones.filter((milestone) => milestone.id !== action.payload),
      };
    case "SET_SELECTED_PROJECT":
      return { ...state, selectedProjectId: action.payload };
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
export const ProjectProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [state, dispatch] = useReducer(projectReducer, initialState);
  const { toast } = useToast();

  // Fetch projects
  const refreshProjects = async () => {
    dispatch({ type: "SET_LOADING", resource: "projects", payload: true });
    try {
      const projects = await projectApi.fetchProjects();
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

  // Fetch teams
  const refreshTeams = async () => {
    dispatch({ type: "SET_LOADING", resource: "teams", payload: true });
    try {
      const teams = await projectApi.fetchTeams();
      dispatch({ type: "SET_TEAMS", payload: teams });
      dispatch({ type: "SET_ERROR", resource: "teams", payload: null });
    } catch (error) {
      console.error("Failed to fetch teams:", error);
      dispatch({
        type: "SET_ERROR",
        resource: "teams",
        payload: "Failed to load teams. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load teams. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "teams", payload: false });
    }
  };

  // Fetch tasks
  const refreshTasks = async () => {
    dispatch({ type: "SET_LOADING", resource: "tasks", payload: true });
    try {
      const tasks = await projectApi.fetchTasks();
      dispatch({ type: "SET_TASKS", payload: tasks });
      dispatch({ type: "SET_ERROR", resource: "tasks", payload: null });
    } catch (error) {
      console.error("Failed to fetch tasks:", error);
      dispatch({
        type: "SET_ERROR",
        resource: "tasks",
        payload: "Failed to load tasks. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load tasks. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "tasks", payload: false });
    }
  };

  // Fetch milestones
  const refreshMilestones = async () => {
    dispatch({ type: "SET_LOADING", resource: "milestones", payload: true });
    try {
      const milestones = await projectApi.fetchMilestones();
      dispatch({ type: "SET_MILESTONES", payload: milestones });
      dispatch({ type: "SET_ERROR", resource: "milestones", payload: null });
    } catch (error) {
      console.error("Failed to fetch milestones:", error);
      dispatch({
        type: "SET_ERROR",
        resource: "milestones",
        payload: "Failed to load milestones. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load milestones. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "milestones", payload: false });
    }
  };

  // Select project
  const selectProject = (projectId: string | null) => {
    dispatch({ type: "SET_SELECTED_PROJECT", payload: projectId });
  };

  // Create new project
  const createNewProject = async (
    project: Omit<Project, "id" | "createdAt" | "updatedAt">
  ): Promise<Project> => {
    try {
      const newProject = await projectApi.createProject(project);
      dispatch({ type: "ADD_PROJECT", payload: newProject });
      toast({
        title: "Project Created",
        description: `Project "${newProject.name}" has been created successfully.`,
      });
      return newProject;
    } catch (error) {
      console.error("Failed to create project:", error);
      toast({
        title: "Error",
        description: "Failed to create project. Please try again.",
        variant: "destructive",
      });
      throw error;
    }
  };

  // Update existing project
  const updateExistingProject = async (
    id: string,
    updates: Partial<Project>
  ): Promise<Project> => {
    try {
      const updatedProject = await projectApi.updateProject(id, updates);
      dispatch({ type: "UPDATE_PROJECT", payload: updatedProject });
      toast({
        title: "Project Updated",
        description: `Project "${updatedProject.name}" has been updated successfully.`,
      });
      return updatedProject;
    } catch (error) {
      console.error("Failed to update project:", error);
      toast({
        title: "Error",
        description: "Failed to update project. Please try again.",
        variant: "destructive",
      });
      throw error;
    }
  };

  // Create new task
  const createNewTask = async (
    task: Omit<Task, "id" | "createdAt" | "updatedAt">
  ): Promise<Task> => {
    try {
      const newTask = await projectApi.createTask(task);
      dispatch({ type: "ADD_TASK", payload: newTask });
      toast({
        title: "Task Created",
        description: `Task "${newTask.title}" has been created successfully.`,
      });
      return newTask;
    } catch (error) {
      console.error("Failed to create task:", error);
      toast({
        title: "Error",
        description: "Failed to create task. Please try again.",
        variant: "destructive",
      });
      throw error;
    }
  };

  // Update existing task
  const updateExistingTask = async (
    id: string,
    updates: Partial<Task>
  ): Promise<Task> => {
    try {
      const updatedTask = await projectApi.updateTask(id, updates);
      dispatch({ type: "UPDATE_TASK", payload: updatedTask });
      toast({
        title: "Task Updated",
        description: `Task "${updatedTask.title}" has been updated successfully.`,
      });
      return updatedTask;
    } catch (error) {
      console.error("Failed to update task:", error);
      toast({
        title: "Error",
        description: "Failed to update task. Please try again.",
        variant: "destructive",
      });
      throw error;
    }
  };

  // Fetch initial data
  useEffect(() => {
    refreshProjects();
    refreshTeams();
    refreshTasks();
    refreshMilestones();
  }, []);

  return (
    <ProjectContext.Provider
      value={{
        state,
        dispatch,
        refreshProjects,
        refreshTeams,
        refreshTasks,
        refreshMilestones,
        selectProject,
        createNewProject,
        updateExistingProject,
        createNewTask,
        updateExistingTask,
      }}
    >
      {children}
    </ProjectContext.Provider>
  );
};

// Custom hook for using the context
export const useProject = () => useContext(ProjectContext);
```

This implementation provides the foundation for the project management interface, including type definitions, API client functions, and a context for state management. In the next part, we'll implement the UI components for project management.
