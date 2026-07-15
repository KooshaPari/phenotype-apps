# Analytics and Reporting Implementation - Part 1

This document outlines the implementation plan for the analytics and reporting components in the MCP dashboard.

## Overview

The analytics and reporting components will provide:
- Agent performance metrics and visualization
- Project progress tracking and reporting
- Cost management and budget tracking
- System resource utilization monitoring
- Custom report generation and export

## Implementation Details

### 1. Analytics Types and Interfaces

Create type definitions for analytics-related data:

```typescript
// src/types/analytics.ts
import { Agent } from "@/contexts/AppContext";
import { Project, Task } from "@/types/project";

// Metric types
export type MetricType = 
  | "TASK_COMPLETION" 
  | "AGENT_ACTIVITY" 
  | "PROJECT_PROGRESS" 
  | "SYSTEM_RESOURCE" 
  | "LLM_REQUEST" 
  | "COST";

// Time period types
export type TimePeriod = "day" | "week" | "month" | "quarter" | "year" | "custom";

// Agent performance metrics
export interface AgentPerformanceMetrics {
  agentId: string;
  taskCount: number;
  taskCompletionRate: number;
  averageTaskDuration: number; // in hours
  activeTime: number; // in hours
  idleTime: number; // in hours
  errorRate: number;
  costIncurred: number;
  tokensConsumed: number;
}

// Project metrics
export interface ProjectMetrics {
  projectId: string;
  completionPercentage: number;
  tasksCompleted: number;
  totalTasks: number;
  onTrackPercentage: number;
  delayedTasks: number;
  costIncurred: number;
  estimatedRemainingCost: number;
  timeSpent: number; // in hours
  estimatedRemainingTime: number; // in hours
}

// System metrics
export interface SystemMetrics {
  cpuUsage: number; // percentage
  memoryUsage: number; // percentage
  diskUsage: number; // percentage
  networkUsage: number; // bytes
  activeAgents: number;
  activeProjects: number;
  activeTasks: number;
  errorCount: number;
  warningCount: number;
}

// Cost metrics
export interface CostMetrics {
  totalCost: number;
  costByAgent: Record<string, number>;
  costByProject: Record<string, number>;
  costByMetricType: Record<MetricType, number>;
  costTrend: DataPoint[];
  budgetUtilization: number; // percentage
  projectedCostToCompletion: number;
}

// Data point for time series
export interface DataPoint {
  timestamp: string;
  value: number;
  label?: string;
}

// Time series data
export interface TimeSeries {
  id: string;
  name: string;
  data: DataPoint[];
  color?: string;
}

// Chart data
export interface ChartData {
  series: TimeSeries[];
  xAxisType: "time" | "category";
  yAxisLabel?: string;
  xAxisLabel?: string;
}

// Dashboard summary
export interface DashboardSummary {
  activeAgents: number;
  activeProjects: number;
  tasksInProgress: number;
  tasksCompleted: number;
  totalCost: number;
  systemHealth: "healthy" | "warning" | "critical";
  recentActivity: ActivityItem[];
}

// Activity item
export interface ActivityItem {
  id: string;
  timestamp: string;
  type: "agent" | "project" | "task" | "system";
  action: string;
  subject: string;
  details?: string;
}

// Budget
export interface Budget {
  id: string;
  name: string;
  amount: number;
  spent: number;
  remaining: number;
  type: "project" | "agent" | "task" | "system";
  targetId?: string;
  startDate: string;
  endDate?: string;
  alerts: BudgetAlert[];
}

// Budget alert
export interface BudgetAlert {
  id: string;
  threshold: number; // percentage
  triggered: boolean;
  triggerDate?: string;
  notificationType: "email" | "dashboard" | "both";
}

// Report
export interface Report {
  id: string;
  name: string;
  description: string;
  type: "agent" | "project" | "system" | "cost" | "custom";
  createdAt: string;
  updatedAt: string;
  schedule?: ReportSchedule;
  parameters: Record<string, any>;
  lastGenerated?: string;
  format: "pdf" | "csv" | "json" | "html";
}

// Report schedule
export interface ReportSchedule {
  frequency: "daily" | "weekly" | "monthly" | "quarterly";
  dayOfWeek?: number; // 0-6, Sunday to Saturday
  dayOfMonth?: number; // 1-31
  time?: string; // HH:MM format
  recipients?: string[]; // email addresses
}

// Helper function to calculate agent performance metrics
export function calculateAgentPerformanceMetrics(
  agent: Agent,
  tasks: Task[],
  startDate: Date,
  endDate: Date
): AgentPerformanceMetrics {
  // Filter tasks assigned to this agent within the date range
  const agentTasks = tasks.filter(
    (task) =>
      task.assigneeId === agent.id &&
      new Date(task.updatedAt) >= startDate &&
      new Date(task.updatedAt) <= endDate
  );
  
  // Calculate metrics
  const taskCount = agentTasks.length;
  const completedTasks = agentTasks.filter((task) => task.status === "done");
  const taskCompletionRate = taskCount > 0 ? completedTasks.length / taskCount : 0;
  
  // Calculate average task duration for completed tasks
  let totalDuration = 0;
  let tasksWithDuration = 0;
  
  completedTasks.forEach((task) => {
    if (task.startDate && task.completedAt) {
      const start = new Date(task.startDate);
      const end = new Date(task.completedAt);
      const durationHours = (end.getTime() - start.getTime()) / (1000 * 60 * 60);
      totalDuration += durationHours;
      tasksWithDuration++;
    }
  });
  
  const averageTaskDuration = tasksWithDuration > 0 ? totalDuration / tasksWithDuration : 0;
  
  // For this example, we'll use placeholder values for some metrics
  // In a real implementation, these would be calculated from actual data
  const activeTime = 0; // Placeholder
  const idleTime = 0; // Placeholder
  const errorRate = 0; // Placeholder
  const costIncurred = 0; // Placeholder
  const tokensConsumed = 0; // Placeholder
  
  return {
    agentId: agent.id,
    taskCount,
    taskCompletionRate,
    averageTaskDuration,
    activeTime,
    idleTime,
    errorRate,
    costIncurred,
    tokensConsumed,
  };
}

// Helper function to calculate project metrics
export function calculateProjectMetrics(
  project: Project,
  tasks: Task[]
): ProjectMetrics {
  // Filter tasks for this project
  const projectTasks = tasks.filter((task) => task.projectId === project.id);
  
  // Calculate metrics
  const totalTasks = projectTasks.length;
  const completedTasks = projectTasks.filter((task) => task.status === "done").length;
  const completionPercentage = totalTasks > 0 ? (completedTasks / totalTasks) * 100 : 0;
  
  // Calculate on-track percentage
  const tasksWithDueDate = projectTasks.filter((task) => task.dueDate);
  const delayedTasks = tasksWithDueDate.filter((task) => {
    if (task.status === "done" && task.completedAt && task.dueDate) {
      return new Date(task.completedAt) > new Date(task.dueDate);
    }
    if (task.status !== "done" && task.dueDate) {
      return new Date() > new Date(task.dueDate);
    }
    return false;
  }).length;
  
  const onTrackPercentage = tasksWithDueDate.length > 0
    ? ((tasksWithDueDate.length - delayedTasks) / tasksWithDueDate.length) * 100
    : 100;
  
  // Calculate time spent
  let timeSpent = 0;
  projectTasks.forEach((task) => {
    if (task.actualHours) {
      timeSpent += task.actualHours;
    }
  });
  
  // Calculate estimated remaining time
  let estimatedRemainingTime = 0;
  projectTasks
    .filter((task) => task.status !== "done")
    .forEach((task) => {
      if (task.estimatedHours) {
        estimatedRemainingTime += task.estimatedHours;
      }
    });
  
  // For this example, we'll use placeholder values for cost metrics
  // In a real implementation, these would be calculated from actual data
  const costIncurred = 0; // Placeholder
  const estimatedRemainingCost = 0; // Placeholder
  
  return {
    projectId: project.id,
    completionPercentage,
    tasksCompleted: completedTasks,
    totalTasks,
    onTrackPercentage,
    delayedTasks,
    costIncurred,
    estimatedRemainingCost,
    timeSpent,
    estimatedRemainingTime,
  };
}

// Helper function to generate time series data
export function generateTimeSeries(
  data: DataPoint[],
  name: string,
  id: string,
  color?: string
): TimeSeries {
  return {
    id,
    name,
    data,
    color,
  };
}

// Helper function to format date for charts
export function formatDateForChart(date: Date): string {
  return date.toISOString();
}
```

### 2. Analytics API Client

Create API client functions for analytics data:

```typescript
// src/lib/api/analytics-api.ts
import {
  AgentPerformanceMetrics,
  ProjectMetrics,
  SystemMetrics,
  CostMetrics,
  DashboardSummary,
  Budget,
  Report,
  TimePeriod,
  ChartData,
} from "@/types/analytics";

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

// Dashboard summary
export async function fetchDashboardSummary(): Promise<DashboardSummary> {
  return fetchApi<DashboardSummary>("/analytics/dashboard");
}

// Agent performance metrics
export async function fetchAgentPerformanceMetrics(
  agentId: string,
  period: TimePeriod,
  startDate?: string,
  endDate?: string
): Promise<AgentPerformanceMetrics> {
  let endpoint = `/analytics/agents/${agentId}/performance?period=${period}`;
  
  if (period === "custom" && startDate && endDate) {
    endpoint += `&startDate=${startDate}&endDate=${endDate}`;
  }
  
  return fetchApi<AgentPerformanceMetrics>(endpoint);
}

// Agent performance comparison
export async function fetchAgentPerformanceComparison(
  agentIds: string[],
  period: TimePeriod,
  startDate?: string,
  endDate?: string
): Promise<Record<string, AgentPerformanceMetrics>> {
  let endpoint = `/analytics/agents/compare?period=${period}`;
  
  if (period === "custom" && startDate && endDate) {
    endpoint += `&startDate=${startDate}&endDate=${endDate}`;
  }
  
  return fetchApi<Record<string, AgentPerformanceMetrics>>(endpoint, {
    method: "POST",
    body: JSON.stringify({ agentIds }),
  });
}

// Agent activity timeline
export async function fetchAgentActivityTimeline(
  agentId: string,
  period: TimePeriod,
  startDate?: string,
  endDate?: string
): Promise<ChartData> {
  let endpoint = `/analytics/agents/${agentId}/activity?period=${period}`;
  
  if (period === "custom" && startDate && endDate) {
    endpoint += `&startDate=${startDate}&endDate=${endDate}`;
  }
  
  return fetchApi<ChartData>(endpoint);
}

// Project metrics
export async function fetchProjectMetrics(
  projectId: string,
  period: TimePeriod,
  startDate?: string,
  endDate?: string
): Promise<ProjectMetrics> {
  let endpoint = `/analytics/projects/${projectId}/metrics?period=${period}`;
  
  if (period === "custom" && startDate && endDate) {
    endpoint += `&startDate=${startDate}&endDate=${endDate}`;
  }
  
  return fetchApi<ProjectMetrics>(endpoint);
}

// Project progress timeline
export async function fetchProjectProgressTimeline(
  projectId: string,
  period: TimePeriod,
  startDate?: string,
  endDate?: string
): Promise<ChartData> {
  let endpoint = `/analytics/projects/${projectId}/progress?period=${period}`;
  
  if (period === "custom" && startDate && endDate) {
    endpoint += `&startDate=${startDate}&endDate=${endDate}`;
  }
  
  return fetchApi<ChartData>(endpoint);
}

// System metrics
export async function fetchSystemMetrics(
  period: TimePeriod,
  startDate?: string,
  endDate?: string
): Promise<SystemMetrics> {
  let endpoint = `/analytics/system/metrics?period=${period}`;
  
  if (period === "custom" && startDate && endDate) {
    endpoint += `&startDate=${startDate}&endDate=${endDate}`;
  }
  
  return fetchApi<SystemMetrics>(endpoint);
}

// System resource timeline
export async function fetchSystemResourceTimeline(
  resource: "cpu" | "memory" | "disk" | "network",
  period: TimePeriod,
  startDate?: string,
  endDate?: string
): Promise<ChartData> {
  let endpoint = `/analytics/system/${resource}?period=${period}`;
  
  if (period === "custom" && startDate && endDate) {
    endpoint += `&startDate=${startDate}&endDate=${endDate}`;
  }
  
  return fetchApi<ChartData>(endpoint);
}

// Cost metrics
export async function fetchCostMetrics(
  period: TimePeriod,
  startDate?: string,
  endDate?: string
): Promise<CostMetrics> {
  let endpoint = `/analytics/cost/metrics?period=${period}`;
  
  if (period === "custom" && startDate && endDate) {
    endpoint += `&startDate=${startDate}&endDate=${endDate}`;
  }
  
  return fetchApi<CostMetrics>(endpoint);
}

// Cost timeline
export async function fetchCostTimeline(
  period: TimePeriod,
  groupBy: "agent" | "project" | "metricType" | "day" | "week" | "month",
  startDate?: string,
  endDate?: string
): Promise<ChartData> {
  let endpoint = `/analytics/cost/timeline?period=${period}&groupBy=${groupBy}`;
  
  if (period === "custom" && startDate && endDate) {
    endpoint += `&startDate=${startDate}&endDate=${endDate}`;
  }
  
  return fetchApi<ChartData>(endpoint);
}

// Budgets
export async function fetchBudgets(): Promise<Budget[]> {
  return fetchApi<Budget[]>("/analytics/budgets");
}

export async function fetchBudget(budgetId: string): Promise<Budget> {
  return fetchApi<Budget>(`/analytics/budgets/${budgetId}`);
}

export async function createBudget(budget: Omit<Budget, "id">): Promise<Budget> {
  return fetchApi<Budget>("/analytics/budgets", {
    method: "POST",
    body: JSON.stringify(budget),
  });
}

export async function updateBudget(
  budgetId: string,
  updates: Partial<Budget>
): Promise<Budget> {
  return fetchApi<Budget>(`/analytics/budgets/${budgetId}`, {
    method: "PUT",
    body: JSON.stringify(updates),
  });
}

export async function deleteBudget(budgetId: string): Promise<void> {
  return fetchApi<void>(`/analytics/budgets/${budgetId}`, {
    method: "DELETE",
  });
}

// Reports
export async function fetchReports(): Promise<Report[]> {
  return fetchApi<Report[]>("/analytics/reports");
}

export async function fetchReport(reportId: string): Promise<Report> {
  return fetchApi<Report>(`/analytics/reports/${reportId}`);
}

export async function createReport(report: Omit<Report, "id">): Promise<Report> {
  return fetchApi<Report>("/analytics/reports", {
    method: "POST",
    body: JSON.stringify(report),
  });
}

export async function updateReport(
  reportId: string,
  updates: Partial<Report>
): Promise<Report> {
  return fetchApi<Report>(`/analytics/reports/${reportId}`, {
    method: "PUT",
    body: JSON.stringify(updates),
  });
}

export async function deleteReport(reportId: string): Promise<void> {
  return fetchApi<void>(`/analytics/reports/${reportId}`, {
    method: "DELETE",
  });
}

export async function generateReport(reportId: string): Promise<string> {
  return fetchApi<string>(`/analytics/reports/${reportId}/generate`, {
    method: "POST",
  });
}

export async function downloadReport(reportId: string, format: "pdf" | "csv" | "json" | "html"): Promise<Blob> {
  const response = await fetch(
    `${API_BASE_URL}/analytics/reports/${reportId}/download?format=${format}`,
    {
      ...fetchOptions,
      method: "GET",
    }
  );
  
  if (!response.ok) {
    const errorData = await response.json().catch(() => ({}));
    throw new Error(
      errorData.message || `API error: ${response.status} ${response.statusText}`
    );
  }
  
  return await response.blob();
}
```

This implementation provides the foundation for the analytics and reporting components, including type definitions and API client functions. In the next part, we'll implement the analytics context and UI components.
