# Analytics and Reporting Implementation - Part 3

This document continues the implementation plan for the analytics and reporting components, focusing on the analytics dashboard page.

## Implementation Details

### 1. Analytics Dashboard Page

Create a dashboard page for analytics overview:

```typescript
// src/app/analytics/page.tsx
"use client";

import React, { useEffect } from "react";
import { useAnalytics } from "@/contexts/AnalyticsContext";
import { useApp } from "@/contexts/AppContext";
import { useProject } from "@/contexts/ProjectContext";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Badge } from "@/components/ui/badge";
import { Progress } from "@/components/ui/progress";
import {
  BarChart,
  Bar,
  LineChart,
  Line,
  PieChart,
  Pie,
  Cell,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
} from "recharts";
import {
  Activity,
  BarChart3,
  Calendar,
  Clock,
  DollarSign,
  Download,
  FileText,
  Filter,
  PieChart as PieChartIcon,
  RefreshCw,
  Server,
  Users,
} from "lucide-react";
import { TimePeriod } from "@/types/analytics";
import { format } from "date-fns";
import * as analyticsApi from "@/lib/api/analytics-api";

const AnalyticsDashboardPage: React.FC = () => {
  const { state, refreshDashboardSummary, fetchChartData, setTimePeriod } = useAnalytics();
  const { state: appState } = useApp();
  const { state: projectState } = useProject();
  
  const { dashboardSummary, selectedTimePeriod, chartData, isLoading } = state;
  const { agents } = appState;
  const { projects } = projectState;
  
  // Fetch dashboard data
  useEffect(() => {
    refreshDashboardSummary();
    
    // Fetch agent activity chart data
    fetchChartData("agentActivity", () =>
      analyticsApi.fetchAgentActivityTimeline("all", selectedTimePeriod)
    );
    
    // Fetch project progress chart data
    fetchChartData("projectProgress", () =>
      analyticsApi.fetchProjectProgressTimeline("all", selectedTimePeriod)
    );
    
    // Fetch system resource chart data
    fetchChartData("systemResource", () =>
      analyticsApi.fetchSystemResourceTimeline("cpu", selectedTimePeriod)
    );
    
    // Fetch cost timeline chart data
    fetchChartData("costTimeline", () =>
      analyticsApi.fetchCostTimeline(selectedTimePeriod, "day")
    );
  }, [selectedTimePeriod]);
  
  // Handle time period change
  const handleTimePeriodChange = (period: TimePeriod) => {
    setTimePeriod(period);
  };
  
  // Handle refresh
  const handleRefresh = () => {
    refreshDashboardSummary();
    
    // Refresh all charts
    fetchChartData("agentActivity", () =>
      analyticsApi.fetchAgentActivityTimeline("all", selectedTimePeriod)
    );
    
    fetchChartData("projectProgress", () =>
      analyticsApi.fetchProjectProgressTimeline("all", selectedTimePeriod)
    );
    
    fetchChartData("systemResource", () =>
      analyticsApi.fetchSystemResourceTimeline("cpu", selectedTimePeriod)
    );
    
    fetchChartData("costTimeline", () =>
      analyticsApi.fetchCostTimeline(selectedTimePeriod, "day")
    );
  };
  
  // Format currency
  const formatCurrency = (value: number) => {
    return new Intl.NumberFormat("en-US", {
      style: "currency",
      currency: "USD",
      minimumFractionDigits: 2,
    }).format(value);
  };
  
  // Get system health color
  const getSystemHealthColor = (health: string) => {
    switch (health) {
      case "healthy":
        return "bg-success text-success-foreground";
      case "warning":
        return "bg-warning text-warning-foreground";
      case "critical":
        return "bg-destructive text-destructive-foreground";
      default:
        return "bg-muted text-muted-foreground";
    }
  };
  
  return (
    <div className="container mx-auto p-6">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-3xl font-bold text-primary">Analytics Dashboard</h1>
        <div className="flex gap-2">
          <Select
            value={selectedTimePeriod}
            onValueChange={(value) => handleTimePeriodChange(value as TimePeriod)}
          >
            <SelectTrigger className="w-[180px]">
              <div className="flex items-center">
                <Calendar className="mr-2 h-4 w-4" />
                <span>Time Period</span>
              </div>
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="day">Today</SelectItem>
              <SelectItem value="week">This Week</SelectItem>
              <SelectItem value="month">This Month</SelectItem>
              <SelectItem value="quarter">This Quarter</SelectItem>
              <SelectItem value="year">This Year</SelectItem>
              <SelectItem value="custom">Custom Range</SelectItem>
            </SelectContent>
          </Select>
          
          <Button
            variant="outline"
            size="sm"
            onClick={handleRefresh}
            disabled={isLoading.dashboardSummary}
          >
            <RefreshCw className="mr-2 h-4 w-4" />
            Refresh
          </Button>
          
          <Button>
            <Download className="mr-2 h-4 w-4" />
            Export
          </Button>
        </div>
      </div>
      
      {/* Summary Cards */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-6">
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm text-muted-foreground">
              Active Agents
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {dashboardSummary?.activeAgents || 0}
            </div>
            <p className="text-xs text-muted-foreground mt-1">
              of {agents.length} total agents
            </p>
            <Progress
              value={
                agents.length > 0
                  ? ((dashboardSummary?.activeAgents || 0) / agents.length) * 100
                  : 0
              }
              className="h-1 mt-2"
            />
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm text-muted-foreground">
              Active Projects
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {dashboardSummary?.activeProjects || 0}
            </div>
            <p className="text-xs text-muted-foreground mt-1">
              of {projects.length} total projects
            </p>
            <Progress
              value={
                projects.length > 0
                  ? ((dashboardSummary?.activeProjects || 0) / projects.length) * 100
                  : 0
              }
              className="h-1 mt-2"
            />
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm text-muted-foreground">
              Tasks Completed
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {dashboardSummary?.tasksCompleted || 0}
            </div>
            <p className="text-xs text-muted-foreground mt-1">
              of {(dashboardSummary?.tasksCompleted || 0) + (dashboardSummary?.tasksInProgress || 0)} total tasks
            </p>
            <Progress
              value={
                (dashboardSummary?.tasksCompleted || 0) +
                  (dashboardSummary?.tasksInProgress || 0) >
                0
                  ? ((dashboardSummary?.tasksCompleted || 0) /
                      ((dashboardSummary?.tasksCompleted || 0) +
                        (dashboardSummary?.tasksInProgress || 0))) *
                    100
                  : 0
              }
              className="h-1 mt-2"
            />
          </CardContent>
        </Card>
        
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm text-muted-foreground">
              Total Cost
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">
              {formatCurrency(dashboardSummary?.totalCost || 0)}
            </div>
            <div className="flex items-center justify-between mt-1">
              <p className="text-xs text-muted-foreground">System Health</p>
              <Badge
                className={getSystemHealthColor(dashboardSummary?.systemHealth || "")}
              >
                {dashboardSummary?.systemHealth || "Unknown"}
              </Badge>
            </div>
          </CardContent>
        </Card>
      </div>
      
      <Tabs defaultValue="overview" className="w-full">
        <TabsList className="grid grid-cols-4 mb-6">
          <TabsTrigger value="overview">Overview</TabsTrigger>
          <TabsTrigger value="agents">Agents</TabsTrigger>
          <TabsTrigger value="projects">Projects</TabsTrigger>
          <TabsTrigger value="costs">Costs</TabsTrigger>
        </TabsList>
        
        <TabsContent value="overview" className="mt-0 space-y-6">
          {/* Agent Activity Chart */}
          <Card>
            <CardHeader>
              <CardTitle>Agent Activity</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="h-[300px]">
                {chartData.agentActivity ? (
                  <ResponsiveContainer width="100%" height="100%">
                    <LineChart
                      data={chartData.agentActivity.series[0].data}
                      margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                    >
                      <CartesianGrid strokeDasharray="3 3" />
                      <XAxis
                        dataKey="timestamp"
                        tickFormatter={(timestamp) =>
                          format(new Date(timestamp), "MMM dd")
                        }
                      />
                      <YAxis />
                      <Tooltip
                        labelFormatter={(timestamp) =>
                          format(new Date(timestamp), "PPP")
                        }
                      />
                      <Legend />
                      <Line
                        type="monotone"
                        dataKey="value"
                        name="Active Agents"
                        stroke="#7ebab5"
                        activeDot={{ r: 8 }}
                      />
                    </LineChart>
                  </ResponsiveContainer>
                ) : (
                  <div className="flex items-center justify-center h-full">
                    <p className="text-muted-foreground">Loading chart data...</p>
                  </div>
                )}
              </div>
            </CardContent>
          </Card>
          
          {/* Project Progress and Cost Timeline */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {/* Project Progress */}
            <Card>
              <CardHeader>
                <CardTitle>Project Progress</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="h-[300px]">
                  {chartData.projectProgress ? (
                    <ResponsiveContainer width="100%" height="100%">
                      <BarChart
                        data={chartData.projectProgress.series[0].data}
                        margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                      >
                        <CartesianGrid strokeDasharray="3 3" />
                        <XAxis
                          dataKey="label"
                          tickFormatter={(label) =>
                            label.length > 10
                              ? `${label.substring(0, 10)}...`
                              : label
                          }
                        />
                        <YAxis />
                        <Tooltip />
                        <Legend />
                        <Bar
                          dataKey="value"
                          name="Completion %"
                          fill="#7ebab5"
                        />
                      </BarChart>
                    </ResponsiveContainer>
                  ) : (
                    <div className="flex items-center justify-center h-full">
                      <p className="text-muted-foreground">Loading chart data...</p>
                    </div>
                  )}
                </div>
              </CardContent>
            </Card>
            
            {/* Cost Timeline */}
            <Card>
              <CardHeader>
                <CardTitle>Cost Timeline</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="h-[300px]">
                  {chartData.costTimeline ? (
                    <ResponsiveContainer width="100%" height="100%">
                      <LineChart
                        data={chartData.costTimeline.series[0].data}
                        margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                      >
                        <CartesianGrid strokeDasharray="3 3" />
                        <XAxis
                          dataKey="timestamp"
                          tickFormatter={(timestamp) =>
                            format(new Date(timestamp), "MMM dd")
                          }
                        />
                        <YAxis
                          tickFormatter={(value) =>
                            `$${value.toFixed(2)}`
                          }
                        />
                        <Tooltip
                          labelFormatter={(timestamp) =>
                            format(new Date(timestamp), "PPP")
                          }
                          formatter={(value) =>
                            [`$${(value as number).toFixed(2)}`, "Cost"]
                          }
                        />
                        <Legend />
                        <Line
                          type="monotone"
                          dataKey="value"
                          name="Daily Cost"
                          stroke="#805ad5"
                          activeDot={{ r: 8 }}
                        />
                      </LineChart>
                    </ResponsiveContainer>
                  ) : (
                    <div className="flex items-center justify-center h-full">
                      <p className="text-muted-foreground">Loading chart data...</p>
                    </div>
                  )}
                </div>
              </CardContent>
            </Card>
          </div>
          
          {/* System Resources and Recent Activity */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            {/* System Resources */}
            <Card>
              <CardHeader>
                <CardTitle>System Resources</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="h-[300px]">
                  {chartData.systemResource ? (
                    <ResponsiveContainer width="100%" height="100%">
                      <LineChart
                        data={chartData.systemResource.series[0].data}
                        margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
                      >
                        <CartesianGrid strokeDasharray="3 3" />
                        <XAxis
                          dataKey="timestamp"
                          tickFormatter={(timestamp) =>
                            format(new Date(timestamp), "HH:mm")
                          }
                        />
                        <YAxis
                          tickFormatter={(value) => `${value}%`}
                        />
                        <Tooltip
                          labelFormatter={(timestamp) =>
                            format(new Date(timestamp), "PPP HH:mm:ss")
                          }
                          formatter={(value) =>
                            [`${value}%`, "CPU Usage"]
                          }
                        />
                        <Legend />
                        <Line
                          type="monotone"
                          dataKey="value"
                          name="CPU Usage"
                          stroke="#e53e3e"
                          activeDot={{ r: 8 }}
                        />
                      </LineChart>
                    </ResponsiveContainer>
                  ) : (
                    <div className="flex items-center justify-center h-full">
                      <p className="text-muted-foreground">Loading chart data...</p>
                    </div>
                  )}
                </div>
              </CardContent>
            </Card>
            
            {/* Recent Activity */}
            <Card>
              <CardHeader>
                <CardTitle>Recent Activity</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="space-y-4 h-[300px] overflow-auto">
                  {dashboardSummary?.recentActivity ? (
                    dashboardSummary.recentActivity.map((activity) => (
                      <div
                        key={activity.id}
                        className="flex items-start p-2 rounded-md bg-secondary/10"
                      >
                        <div className="mr-3 mt-0.5">
                          {activity.type === "agent" ? (
                            <Users className="h-5 w-5 text-primary" />
                          ) : activity.type === "project" ? (
                            <FileText className="h-5 w-5 text-accent" />
                          ) : activity.type === "task" ? (
                            <Activity className="h-5 w-5 text-secondary-foreground" />
                          ) : (
                            <Server className="h-5 w-5 text-destructive" />
                          )}
                        </div>
                        <div>
                          <p className="font-medium">{activity.action}</p>
                          <p className="text-sm text-muted-foreground">
                            {activity.subject}
                          </p>
                          {activity.details && (
                            <p className="text-sm text-muted-foreground mt-1">
                              {activity.details}
                            </p>
                          )}
                          <p className="text-xs text-muted-foreground mt-1">
                            {format(
                              new Date(activity.timestamp),
                              "MMM dd, yyyy HH:mm"
                            )}
                          </p>
                        </div>
                      </div>
                    ))
                  ) : (
                    <div className="flex items-center justify-center h-full">
                      <p className="text-muted-foreground">No recent activity</p>
                    </div>
                  )}
                </div>
              </CardContent>
            </Card>
          </div>
        </TabsContent>
        
        <TabsContent value="agents" className="mt-0">
          <div className="text-center py-12">
            <div className="mx-auto w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center mb-4">
              <Users className="h-6 w-6 text-primary" />
            </div>
            <h3 className="text-lg font-medium">Agent Analytics</h3>
            <p className="text-muted-foreground mt-1 mb-4">
              Detailed agent performance analytics will be shown here
            </p>
            <Button>
              <Users className="mr-2 h-4 w-4" />
              View Agent Analytics
            </Button>
          </div>
        </TabsContent>
        
        <TabsContent value="projects" className="mt-0">
          <div className="text-center py-12">
            <div className="mx-auto w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center mb-4">
              <FileText className="h-6 w-6 text-primary" />
            </div>
            <h3 className="text-lg font-medium">Project Analytics</h3>
            <p className="text-muted-foreground mt-1 mb-4">
              Detailed project analytics will be shown here
            </p>
            <Button>
              <FileText className="mr-2 h-4 w-4" />
              View Project Analytics
            </Button>
          </div>
        </TabsContent>
        
        <TabsContent value="costs" className="mt-0">
          <div className="text-center py-12">
            <div className="mx-auto w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center mb-4">
              <DollarSign className="h-6 w-6 text-primary" />
            </div>
            <h3 className="text-lg font-medium">Cost Analytics</h3>
            <p className="text-muted-foreground mt-1 mb-4">
              Detailed cost analytics and budget management will be shown here
            </p>
            <Button>
              <DollarSign className="mr-2 h-4 w-4" />
              View Cost Analytics
            </Button>
          </div>
        </TabsContent>
      </Tabs>
    </div>
  );
};

export default AnalyticsDashboardPage;
```

This implementation provides a comprehensive analytics dashboard with:

1. Summary cards showing key metrics (active agents, active projects, tasks completed, total cost)
2. Time period selection for filtering analytics data
3. Multiple tabs for different analytics categories (overview, agents, projects, costs)
4. Various charts for visualizing data:
   - Line chart for agent activity over time
   - Bar chart for project progress
   - Line chart for cost timeline
   - Line chart for system resource usage
5. Recent activity list showing the latest events in the system

The dashboard is designed to give users a quick overview of the system's performance and status, with options to drill down into more detailed analytics in the specific tabs. In the next part, we'll implement the cost management and budget tracking components.
