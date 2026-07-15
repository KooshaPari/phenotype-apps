# Analytics and Reporting Implementation - Part 2

This document continues the implementation plan for the analytics and reporting components, focusing on the analytics context.

## Implementation Details

### 1. Analytics Context

Create a context for analytics data management:

```typescript
// src/contexts/AnalyticsContext.tsx
"use client";

import React, { createContext, useContext, useReducer, useEffect } from "react";
import { useToast } from "@/components/ui/use-toast";
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
import * as analyticsApi from "@/lib/api/analytics-api";

// Define state types
interface AnalyticsState {
  dashboardSummary: DashboardSummary | null;
  agentPerformanceMetrics: Record<string, AgentPerformanceMetrics>;
  projectMetrics: Record<string, ProjectMetrics>;
  systemMetrics: SystemMetrics | null;
  costMetrics: CostMetrics | null;
  budgets: Budget[];
  reports: Report[];
  chartData: Record<string, ChartData>;
  selectedTimePeriod: TimePeriod;
  customDateRange: {
    startDate: string | null;
    endDate: string | null;
  };
  isLoading: {
    dashboardSummary: boolean;
    agentPerformanceMetrics: boolean;
    projectMetrics: boolean;
    systemMetrics: boolean;
    costMetrics: boolean;
    budgets: boolean;
    reports: boolean;
    chartData: boolean;
  };
  error: {
    dashboardSummary: string | null;
    agentPerformanceMetrics: string | null;
    projectMetrics: string | null;
    systemMetrics: string | null;
    costMetrics: string | null;
    budgets: string | null;
    reports: string | null;
    chartData: string | null;
  };
}

// Define action types
type AnalyticsAction =
  | { type: "SET_DASHBOARD_SUMMARY"; payload: DashboardSummary }
  | { type: "SET_AGENT_PERFORMANCE_METRICS"; agentId: string; payload: AgentPerformanceMetrics }
  | { type: "SET_PROJECT_METRICS"; projectId: string; payload: ProjectMetrics }
  | { type: "SET_SYSTEM_METRICS"; payload: SystemMetrics }
  | { type: "SET_COST_METRICS"; payload: CostMetrics }
  | { type: "SET_BUDGETS"; payload: Budget[] }
  | { type: "ADD_BUDGET"; payload: Budget }
  | { type: "UPDATE_BUDGET"; payload: Budget }
  | { type: "REMOVE_BUDGET"; payload: string }
  | { type: "SET_REPORTS"; payload: Report[] }
  | { type: "ADD_REPORT"; payload: Report }
  | { type: "UPDATE_REPORT"; payload: Report }
  | { type: "REMOVE_REPORT"; payload: string }
  | { type: "SET_CHART_DATA"; chartId: string; payload: ChartData }
  | { type: "SET_TIME_PERIOD"; payload: TimePeriod }
  | { type: "SET_CUSTOM_DATE_RANGE"; startDate: string | null; endDate: string | null }
  | { type: "SET_LOADING"; resource: keyof AnalyticsState["isLoading"]; payload: boolean }
  | { type: "SET_ERROR"; resource: keyof AnalyticsState["error"]; payload: string | null };

// Initial state
const initialState: AnalyticsState = {
  dashboardSummary: null,
  agentPerformanceMetrics: {},
  projectMetrics: {},
  systemMetrics: null,
  costMetrics: null,
  budgets: [],
  reports: [],
  chartData: {},
  selectedTimePeriod: "month",
  customDateRange: {
    startDate: null,
    endDate: null,
  },
  isLoading: {
    dashboardSummary: false,
    agentPerformanceMetrics: false,
    projectMetrics: false,
    systemMetrics: false,
    costMetrics: false,
    budgets: false,
    reports: false,
    chartData: false,
  },
  error: {
    dashboardSummary: null,
    agentPerformanceMetrics: null,
    projectMetrics: null,
    systemMetrics: null,
    costMetrics: null,
    budgets: null,
    reports: null,
    chartData: null,
  },
};

// Create context
const AnalyticsContext = createContext<{
  state: AnalyticsState;
  dispatch: React.Dispatch<AnalyticsAction>;
  refreshDashboardSummary: () => Promise<void>;
  refreshAgentPerformanceMetrics: (agentId: string) => Promise<void>;
  refreshProjectMetrics: (projectId: string) => Promise<void>;
  refreshSystemMetrics: () => Promise<void>;
  refreshCostMetrics: () => Promise<void>;
  refreshBudgets: () => Promise<void>;
  refreshReports: () => Promise<void>;
  fetchChartData: (chartId: string, endpoint: () => Promise<ChartData>) => Promise<void>;
  setTimePeriod: (period: TimePeriod, startDate?: string, endDate?: string) => void;
  createNewBudget: (budget: Omit<Budget, "id">) => Promise<Budget>;
  updateExistingBudget: (id: string, updates: Partial<Budget>) => Promise<Budget>;
  createNewReport: (report: Omit<Report, "id">) => Promise<Report>;
  updateExistingReport: (id: string, updates: Partial<Report>) => Promise<Report>;
  generateAndDownloadReport: (reportId: string, format: "pdf" | "csv" | "json" | "html") => Promise<void>;
}>({
  state: initialState,
  dispatch: () => null,
  refreshDashboardSummary: async () => {},
  refreshAgentPerformanceMetrics: async () => {},
  refreshProjectMetrics: async () => {},
  refreshSystemMetrics: async () => {},
  refreshCostMetrics: async () => {},
  refreshBudgets: async () => {},
  refreshReports: async () => {},
  fetchChartData: async () => {},
  setTimePeriod: () => {},
  createNewBudget: async () => ({} as Budget),
  updateExistingBudget: async () => ({} as Budget),
  createNewReport: async () => ({} as Report),
  updateExistingReport: async () => ({} as Report),
  generateAndDownloadReport: async () => {},
});

// Reducer function
const analyticsReducer = (state: AnalyticsState, action: AnalyticsAction): AnalyticsState => {
  switch (action.type) {
    case "SET_DASHBOARD_SUMMARY":
      return { ...state, dashboardSummary: action.payload };
    case "SET_AGENT_PERFORMANCE_METRICS":
      return {
        ...state,
        agentPerformanceMetrics: {
          ...state.agentPerformanceMetrics,
          [action.agentId]: action.payload,
        },
      };
    case "SET_PROJECT_METRICS":
      return {
        ...state,
        projectMetrics: {
          ...state.projectMetrics,
          [action.projectId]: action.payload,
        },
      };
    case "SET_SYSTEM_METRICS":
      return { ...state, systemMetrics: action.payload };
    case "SET_COST_METRICS":
      return { ...state, costMetrics: action.payload };
    case "SET_BUDGETS":
      return { ...state, budgets: action.payload };
    case "ADD_BUDGET":
      return { ...state, budgets: [...state.budgets, action.payload] };
    case "UPDATE_BUDGET":
      return {
        ...state,
        budgets: state.budgets.map((budget) =>
          budget.id === action.payload.id ? action.payload : budget
        ),
      };
    case "REMOVE_BUDGET":
      return {
        ...state,
        budgets: state.budgets.filter((budget) => budget.id !== action.payload),
      };
    case "SET_REPORTS":
      return { ...state, reports: action.payload };
    case "ADD_REPORT":
      return { ...state, reports: [...state.reports, action.payload] };
    case "UPDATE_REPORT":
      return {
        ...state,
        reports: state.reports.map((report) =>
          report.id === action.payload.id ? action.payload : report
        ),
      };
    case "REMOVE_REPORT":
      return {
        ...state,
        reports: state.reports.filter((report) => report.id !== action.payload),
      };
    case "SET_CHART_DATA":
      return {
        ...state,
        chartData: {
          ...state.chartData,
          [action.chartId]: action.payload,
        },
      };
    case "SET_TIME_PERIOD":
      return { ...state, selectedTimePeriod: action.payload };
    case "SET_CUSTOM_DATE_RANGE":
      return {
        ...state,
        customDateRange: {
          startDate: action.startDate,
          endDate: action.endDate,
        },
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
export const AnalyticsProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [state, dispatch] = useReducer(analyticsReducer, initialState);
  const { toast } = useToast();

  // Fetch dashboard summary
  const refreshDashboardSummary = async () => {
    dispatch({ type: "SET_LOADING", resource: "dashboardSummary", payload: true });
    try {
      const summary = await analyticsApi.fetchDashboardSummary();
      dispatch({ type: "SET_DASHBOARD_SUMMARY", payload: summary });
      dispatch({ type: "SET_ERROR", resource: "dashboardSummary", payload: null });
    } catch (error) {
      console.error("Failed to fetch dashboard summary:", error);
      dispatch({
        type: "SET_ERROR",
        resource: "dashboardSummary",
        payload: "Failed to load dashboard summary. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load dashboard summary. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "dashboardSummary", payload: false });
    }
  };

  // Fetch agent performance metrics
  const refreshAgentPerformanceMetrics = async (agentId: string) => {
    dispatch({ type: "SET_LOADING", resource: "agentPerformanceMetrics", payload: true });
    try {
      const { selectedTimePeriod, customDateRange } = state;
      const metrics = await analyticsApi.fetchAgentPerformanceMetrics(
        agentId,
        selectedTimePeriod,
        customDateRange.startDate || undefined,
        customDateRange.endDate || undefined
      );
      dispatch({
        type: "SET_AGENT_PERFORMANCE_METRICS",
        agentId,
        payload: metrics,
      });
      dispatch({ type: "SET_ERROR", resource: "agentPerformanceMetrics", payload: null });
    } catch (error) {
      console.error(`Failed to fetch agent performance metrics for ${agentId}:`, error);
      dispatch({
        type: "SET_ERROR",
        resource: "agentPerformanceMetrics",
        payload: "Failed to load agent performance metrics. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load agent performance metrics. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "agentPerformanceMetrics", payload: false });
    }
  };

  // Fetch project metrics
  const refreshProjectMetrics = async (projectId: string) => {
    dispatch({ type: "SET_LOADING", resource: "projectMetrics", payload: true });
    try {
      const { selectedTimePeriod, customDateRange } = state;
      const metrics = await analyticsApi.fetchProjectMetrics(
        projectId,
        selectedTimePeriod,
        customDateRange.startDate || undefined,
        customDateRange.endDate || undefined
      );
      dispatch({
        type: "SET_PROJECT_METRICS",
        projectId,
        payload: metrics,
      });
      dispatch({ type: "SET_ERROR", resource: "projectMetrics", payload: null });
    } catch (error) {
      console.error(`Failed to fetch project metrics for ${projectId}:`, error);
      dispatch({
        type: "SET_ERROR",
        resource: "projectMetrics",
        payload: "Failed to load project metrics. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load project metrics. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "projectMetrics", payload: false });
    }
  };

  // Fetch system metrics
  const refreshSystemMetrics = async () => {
    dispatch({ type: "SET_LOADING", resource: "systemMetrics", payload: true });
    try {
      const { selectedTimePeriod, customDateRange } = state;
      const metrics = await analyticsApi.fetchSystemMetrics(
        selectedTimePeriod,
        customDateRange.startDate || undefined,
        customDateRange.endDate || undefined
      );
      dispatch({ type: "SET_SYSTEM_METRICS", payload: metrics });
      dispatch({ type: "SET_ERROR", resource: "systemMetrics", payload: null });
    } catch (error) {
      console.error("Failed to fetch system metrics:", error);
      dispatch({
        type: "SET_ERROR",
        resource: "systemMetrics",
        payload: "Failed to load system metrics. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load system metrics. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "systemMetrics", payload: false });
    }
  };

  // Fetch cost metrics
  const refreshCostMetrics = async () => {
    dispatch({ type: "SET_LOADING", resource: "costMetrics", payload: true });
    try {
      const { selectedTimePeriod, customDateRange } = state;
      const metrics = await analyticsApi.fetchCostMetrics(
        selectedTimePeriod,
        customDateRange.startDate || undefined,
        customDateRange.endDate || undefined
      );
      dispatch({ type: "SET_COST_METRICS", payload: metrics });
      dispatch({ type: "SET_ERROR", resource: "costMetrics", payload: null });
    } catch (error) {
      console.error("Failed to fetch cost metrics:", error);
      dispatch({
        type: "SET_ERROR",
        resource: "costMetrics",
        payload: "Failed to load cost metrics. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load cost metrics. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "costMetrics", payload: false });
    }
  };

  // Fetch budgets
  const refreshBudgets = async () => {
    dispatch({ type: "SET_LOADING", resource: "budgets", payload: true });
    try {
      const budgets = await analyticsApi.fetchBudgets();
      dispatch({ type: "SET_BUDGETS", payload: budgets });
      dispatch({ type: "SET_ERROR", resource: "budgets", payload: null });
    } catch (error) {
      console.error("Failed to fetch budgets:", error);
      dispatch({
        type: "SET_ERROR",
        resource: "budgets",
        payload: "Failed to load budgets. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load budgets. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "budgets", payload: false });
    }
  };

  // Fetch reports
  const refreshReports = async () => {
    dispatch({ type: "SET_LOADING", resource: "reports", payload: true });
    try {
      const reports = await analyticsApi.fetchReports();
      dispatch({ type: "SET_REPORTS", payload: reports });
      dispatch({ type: "SET_ERROR", resource: "reports", payload: null });
    } catch (error) {
      console.error("Failed to fetch reports:", error);
      dispatch({
        type: "SET_ERROR",
        resource: "reports",
        payload: "Failed to load reports. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load reports. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "reports", payload: false });
    }
  };

  // Fetch chart data
  const fetchChartData = async (chartId: string, endpoint: () => Promise<ChartData>) => {
    dispatch({ type: "SET_LOADING", resource: "chartData", payload: true });
    try {
      const data = await endpoint();
      dispatch({ type: "SET_CHART_DATA", chartId, payload: data });
      dispatch({ type: "SET_ERROR", resource: "chartData", payload: null });
    } catch (error) {
      console.error(`Failed to fetch chart data for ${chartId}:`, error);
      dispatch({
        type: "SET_ERROR",
        resource: "chartData",
        payload: "Failed to load chart data. Please try again.",
      });
      toast({
        title: "Error",
        description: "Failed to load chart data. Please try again.",
        variant: "destructive",
      });
    } finally {
      dispatch({ type: "SET_LOADING", resource: "chartData", payload: false });
    }
  };

  // Set time period
  const setTimePeriod = (period: TimePeriod, startDate?: string, endDate?: string) => {
    dispatch({ type: "SET_TIME_PERIOD", payload: period });
    
    if (period === "custom" && startDate && endDate) {
      dispatch({
        type: "SET_CUSTOM_DATE_RANGE",
        startDate,
        endDate,
      });
    } else if (period !== "custom") {
      dispatch({
        type: "SET_CUSTOM_DATE_RANGE",
        startDate: null,
        endDate: null,
      });
    }
  };

  // Create new budget
  const createNewBudget = async (budget: Omit<Budget, "id">): Promise<Budget> => {
    try {
      const newBudget = await analyticsApi.createBudget(budget);
      dispatch({ type: "ADD_BUDGET", payload: newBudget });
      toast({
        title: "Budget Created",
        description: `Budget "${newBudget.name}" has been created successfully.`,
      });
      return newBudget;
    } catch (error) {
      console.error("Failed to create budget:", error);
      toast({
        title: "Error",
        description: "Failed to create budget. Please try again.",
        variant: "destructive",
      });
      throw error;
    }
  };

  // Update existing budget
  const updateExistingBudget = async (
    id: string,
    updates: Partial<Budget>
  ): Promise<Budget> => {
    try {
      const updatedBudget = await analyticsApi.updateBudget(id, updates);
      dispatch({ type: "UPDATE_BUDGET", payload: updatedBudget });
      toast({
        title: "Budget Updated",
        description: `Budget "${updatedBudget.name}" has been updated successfully.`,
      });
      return updatedBudget;
    } catch (error) {
      console.error("Failed to update budget:", error);
      toast({
        title: "Error",
        description: "Failed to update budget. Please try again.",
        variant: "destructive",
      });
      throw error;
    }
  };

  // Create new report
  const createNewReport = async (report: Omit<Report, "id">): Promise<Report> => {
    try {
      const newReport = await analyticsApi.createReport(report);
      dispatch({ type: "ADD_REPORT", payload: newReport });
      toast({
        title: "Report Created",
        description: `Report "${newReport.name}" has been created successfully.`,
      });
      return newReport;
    } catch (error) {
      console.error("Failed to create report:", error);
      toast({
        title: "Error",
        description: "Failed to create report. Please try again.",
        variant: "destructive",
      });
      throw error;
    }
  };

  // Update existing report
  const updateExistingReport = async (
    id: string,
    updates: Partial<Report>
  ): Promise<Report> => {
    try {
      const updatedReport = await analyticsApi.updateReport(id, updates);
      dispatch({ type: "UPDATE_REPORT", payload: updatedReport });
      toast({
        title: "Report Updated",
        description: `Report "${updatedReport.name}" has been updated successfully.`,
      });
      return updatedReport;
    } catch (error) {
      console.error("Failed to update report:", error);
      toast({
        title: "Error",
        description: "Failed to update report. Please try again.",
        variant: "destructive",
      });
      throw error;
    }
  };

  // Generate and download report
  const generateAndDownloadReport = async (
    reportId: string,
    format: "pdf" | "csv" | "json" | "html"
  ): Promise<void> => {
    try {
      // First generate the report
      await analyticsApi.generateReport(reportId);
      
      // Then download it
      const blob = await analyticsApi.downloadReport(reportId, format);
      
      // Create a download link
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.style.display = "none";
      a.href = url;
      a.download = `report-${reportId}.${format}`;
      document.body.appendChild(a);
      a.click();
      window.URL.revokeObjectURL(url);
      
      toast({
        title: "Report Downloaded",
        description: `Report has been downloaded successfully.`,
      });
    } catch (error) {
      console.error("Failed to generate and download report:", error);
      toast({
        title: "Error",
        description: "Failed to generate and download report. Please try again.",
        variant: "destructive",
      });
      throw error;
    }
  };

  // Fetch initial data
  useEffect(() => {
    refreshDashboardSummary();
    refreshBudgets();
    refreshReports();
  }, []);

  return (
    <AnalyticsContext.Provider
      value={{
        state,
        dispatch,
        refreshDashboardSummary,
        refreshAgentPerformanceMetrics,
        refreshProjectMetrics,
        refreshSystemMetrics,
        refreshCostMetrics,
        refreshBudgets,
        refreshReports,
        fetchChartData,
        setTimePeriod,
        createNewBudget,
        updateExistingBudget,
        createNewReport,
        updateExistingReport,
        generateAndDownloadReport,
      }}
    >
      {children}
    </AnalyticsContext.Provider>
  );
};

// Custom hook for using the context
export const useAnalytics = () => useContext(AnalyticsContext);
```

This implementation provides a comprehensive analytics context for managing analytics data, including:

1. State management for various analytics metrics and data
2. Functions for fetching and refreshing different types of analytics data
3. Time period selection and custom date range support
4. Budget and report management functions
5. Chart data fetching and caching
6. Error handling and loading state management

The context is designed to be used throughout the analytics components to provide a consistent data management approach. In the next part, we'll implement the analytics dashboard and visualization components.
