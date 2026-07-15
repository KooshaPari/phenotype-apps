# Project Management Interface Implementation - Part 3

This document continues the implementation plan for the project management interface, focusing on the project overview component.

## Implementation Details

### 1. Project Overview Component

Create a component to display project overview:

```typescript
// src/components/projects/ProjectOverview.tsx
"use client";

import React from "react";
import { Project, ProjectStats, ProjectWithRelations } from "@/types/project";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Badge } from "@/components/ui/badge";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
} from "recharts";
import {
  Calendar,
  Clock,
  FileText,
  Users,
  CheckCircle,
  AlertCircle,
  Circle,
  PauseCircle,
} from "lucide-react";

interface ProjectOverviewProps {
  project: Project;
  projectWithRelations: ProjectWithRelations | null;
  stats: ProjectStats | null;
}

const ProjectOverview: React.FC<ProjectOverviewProps> = ({
  project,
  projectWithRelations,
  stats,
}) => {
  if (!stats) {
    return <div>Loading stats...</div>;
  }

  // Prepare task status data for pie chart
  const taskStatusData = [
    {
      name: "Completed",
      value: stats.completedTasks,
      color: "#38a169", // success
    },
    {
      name: "In Progress",
      value: stats.inProgressTasks,
      color: "#7ebab5", // primary
    },
    {
      name: "Review",
      value: stats.reviewTasks,
      color: "#805ad5", // accent
    },
    {
      name: "Blocked",
      value: stats.blockedTasks,
      color: "#e53e3e", // destructive
    },
    {
      name: "To Do",
      value: stats.todoTasks,
      color: "#4a5568", // muted
    },
  ].filter((item) => item.value > 0);

  // Prepare task progress data for bar chart
  const taskProgressData = projectWithRelations?.tasks
    .filter((task) => task.estimatedHours && task.estimatedHours > 0)
    .map((task) => ({
      name: task.title.length > 20 ? task.title.substring(0, 20) + "..." : task.title,
      estimated: task.estimatedHours || 0,
      actual: task.actualHours || 0,
    }))
    .slice(0, 5) || [];

  return (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {/* Project Information */}
        <Card className="md:col-span-2">
          <CardHeader>
            <CardTitle>Project Information</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <div>
                <h3 className="text-sm font-medium text-muted-foreground mb-1">
                  Description
                </h3>
                <p>{project.description}</p>
              </div>

              <div className="grid grid-cols-2 gap-4">
                <div>
                  <h3 className="text-sm font-medium text-muted-foreground mb-1">
                    Start Date
                  </h3>
                  <div className="flex items-center">
                    <Calendar className="h-4 w-4 mr-2 text-muted-foreground" />
                    <span>
                      {new Date(project.startDate).toLocaleDateString()}
                    </span>
                  </div>
                </div>

                <div>
                  <h3 className="text-sm font-medium text-muted-foreground mb-1">
                    End Date
                  </h3>
                  <div className="flex items-center">
                    <Calendar className="h-4 w-4 mr-2 text-muted-foreground" />
                    <span>
                      {project.endDate
                        ? new Date(project.endDate).toLocaleDateString()
                        : "Not set"}
                    </span>
                  </div>
                </div>

                <div>
                  <h3 className="text-sm font-medium text-muted-foreground mb-1">
                    Status
                  </h3>
                  <Badge
                    variant={
                      project.status === "active"
                        ? "default"
                        : project.status === "completed"
                        ? "success"
                        : project.status === "archived"
                        ? "secondary"
                        : "outline"
                    }
                  >
                    {project.status.charAt(0).toUpperCase() +
                      project.status.slice(1)}
                  </Badge>
                </div>

                <div>
                  <h3 className="text-sm font-medium text-muted-foreground mb-1">
                    Progress
                  </h3>
                  <Badge
                    variant={
                      stats.onTrack ? "outline" : "destructive"
                    }
                    className={
                      stats.onTrack
                        ? "bg-success/10 text-success border-success/20"
                        : ""
                    }
                  >
                    {stats.onTrack ? "On Track" : "At Risk"}
                  </Badge>
                </div>

                <div>
                  <h3 className="text-sm font-medium text-muted-foreground mb-1">
                    Created
                  </h3>
                  <div className="flex items-center">
                    <Clock className="h-4 w-4 mr-2 text-muted-foreground" />
                    <span>
                      {new Date(project.createdAt).toLocaleDateString()}
                    </span>
                  </div>
                </div>

                <div>
                  <h3 className="text-sm font-medium text-muted-foreground mb-1">
                    Last Updated
                  </h3>
                  <div className="flex items-center">
                    <Clock className="h-4 w-4 mr-2 text-muted-foreground" />
                    <span>
                      {new Date(project.updatedAt).toLocaleDateString()}
                    </span>
                  </div>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Task Status */}
        <Card>
          <CardHeader>
            <CardTitle>Task Status</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="h-[200px]">
              {taskStatusData.length > 0 ? (
                <ResponsiveContainer width="100%" height="100%">
                  <PieChart>
                    <Pie
                      data={taskStatusData}
                      cx="50%"
                      cy="50%"
                      innerRadius={60}
                      outerRadius={80}
                      paddingAngle={2}
                      dataKey="value"
                      label={({ name, percent }) =>
                        `${name} ${(percent * 100).toFixed(0)}%`
                      }
                      labelLine={false}
                    >
                      {taskStatusData.map((entry, index) => (
                        <Cell key={`cell-${index}`} fill={entry.color} />
                      ))}
                    </Pie>
                    <Tooltip
                      formatter={(value) => [`${value} tasks`, "Count"]}
                    />
                  </PieChart>
                </ResponsiveContainer>
              ) : (
                <div className="flex items-center justify-center h-full">
                  <p className="text-muted-foreground">No tasks yet</p>
                </div>
              )}
            </div>

            <div className="mt-4 space-y-2">
              <div className="flex items-center">
                <CheckCircle className="h-4 w-4 text-success mr-2" />
                <span className="text-sm">
                  Completed: {stats.completedTasks}
                </span>
              </div>
              <div className="flex items-center">
                <Circle className="h-4 w-4 text-primary mr-2" />
                <span className="text-sm">
                  In Progress: {stats.inProgressTasks}
                </span>
              </div>
              <div className="flex items-center">
                <PauseCircle className="h-4 w-4 text-accent mr-2" />
                <span className="text-sm">Review: {stats.reviewTasks}</span>
              </div>
              <div className="flex items-center">
                <AlertCircle className="h-4 w-4 text-destructive mr-2" />
                <span className="text-sm">Blocked: {stats.blockedTasks}</span>
              </div>
              <div className="flex items-center">
                <Circle className="h-4 w-4 text-muted-foreground mr-2" />
                <span className="text-sm">To Do: {stats.todoTasks}</span>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>

      {/* Task Progress */}
      <Card>
        <CardHeader>
          <CardTitle>Task Progress (Estimated vs. Actual Hours)</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="h-[300px]">
            {taskProgressData.length > 0 ? (
              <ResponsiveContainer width="100%" height="100%">
                <BarChart
                  data={taskProgressData}
                  margin={{ top: 20, right: 30, left: 20, bottom: 60 }}
                >
                  <CartesianGrid strokeDasharray="3 3" />
                  <XAxis
                    dataKey="name"
                    angle={-45}
                    textAnchor="end"
                    height={70}
                  />
                  <YAxis />
                  <Tooltip />
                  <Bar
                    dataKey="estimated"
                    name="Estimated Hours"
                    fill="#7ebab5"
                  />
                  <Bar
                    dataKey="actual"
                    name="Actual Hours"
                    fill="#805ad5"
                  />
                </BarChart>
              </ResponsiveContainer>
            ) : (
              <div className="flex items-center justify-center h-full">
                <p className="text-muted-foreground">
                  No tasks with estimated hours yet
                </p>
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {/* Recent Activity and Team Members */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Recent Activity */}
        <Card>
          <CardHeader>
            <CardTitle>Recent Activity</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {projectWithRelations?.tasks
                .sort(
                  (a, b) =>
                    new Date(b.updatedAt).getTime() -
                    new Date(a.updatedAt).getTime()
                )
                .slice(0, 5)
                .map((task) => (
                  <div
                    key={task.id}
                    className="flex items-start p-2 rounded-md bg-secondary/10"
                  >
                    <div className="mr-3 mt-0.5">
                      <FileText className="h-5 w-5 text-muted-foreground" />
                    </div>
                    <div>
                      <p className="font-medium">{task.title}</p>
                      <p className="text-sm text-muted-foreground">
                        Status changed to{" "}
                        <Badge variant="outline">
                          {task.status.charAt(0).toUpperCase() +
                            task.status.slice(1)}
                        </Badge>
                      </p>
                      <p className="text-xs text-muted-foreground mt-1">
                        {new Date(task.updatedAt).toLocaleString()}
                      </p>
                    </div>
                  </div>
                )) || (
                <p className="text-muted-foreground">No recent activity</p>
              )}
            </div>
          </CardContent>
        </Card>

        {/* Team Members */}
        <Card>
          <CardHeader>
            <CardTitle>Team Members</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              {projectWithRelations?.teams.flatMap((team) =>
                team.members.map((member) => (
                  <div
                    key={`${team.id}-${member.agentId}`}
                    className="flex items-center p-2 rounded-md bg-secondary/10"
                  >
                    <div className="mr-3">
                      <div className="h-8 w-8 rounded-full bg-primary/10 flex items-center justify-center">
                        <Users className="h-4 w-4 text-primary" />
                      </div>
                    </div>
                    <div>
                      <p className="font-medium">
                        Agent {member.agentId.substring(0, 8)}...
                      </p>
                      <p className="text-sm text-muted-foreground">
                        {team.name} -{" "}
                        <Badge variant="outline">
                          {member.role.charAt(0).toUpperCase() +
                            member.role.slice(1)}
                        </Badge>
                      </p>
                    </div>
                  </div>
                ))
              ).slice(0, 5) || (
                <p className="text-muted-foreground">No team members yet</p>
              )}
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  );
};

export default ProjectOverview;
```

This component provides a comprehensive overview of a project, including:

1. Project information (description, dates, status)
2. Task status visualization with a pie chart
3. Task progress visualization with a bar chart comparing estimated vs. actual hours
4. Recent activity list showing the latest task updates
5. Team members list showing agents assigned to the project

The component uses Recharts for data visualization and displays the information in a clean, organized layout with cards. It handles cases where data might not be available yet, showing appropriate placeholder messages.

In the next part, we'll implement the project tasks component for managing tasks within a project.
