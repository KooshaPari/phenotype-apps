# Project Management Interface Implementation - Part 4

This document continues the implementation plan for the project management interface, focusing on the project tasks component.

## Implementation Details

### 1. Project Tasks Component

Create a component to manage project tasks:

```typescript
// src/components/projects/ProjectTasks.tsx
"use client";

import React, { useState } from "react";
import { Project, Task, TaskPriority, TaskStatus } from "@/types/project";
import { useProject } from "@/contexts/ProjectContext";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Badge } from "@/components/ui/badge";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  PlusCircle,
  Search,
  Filter,
  MoreHorizontal,
  ArrowUpDown,
  Calendar,
  Clock,
  AlertCircle,
  CheckCircle,
  Circle,
  PauseCircle,
} from "lucide-react";
import { isTaskOverdue } from "@/types/project";
import TaskCreateDialog from "./TaskCreateDialog";
import TaskDetailDialog from "./TaskDetailDialog";

interface ProjectTasksProps {
  project: Project;
  tasks: Task[];
}

const ProjectTasks: React.FC<ProjectTasksProps> = ({ project, tasks }) => {
  // State for search and filters
  const [searchQuery, setSearchQuery] = useState("");
  const [statusFilter, setStatusFilter] = useState<string>("all");
  const [priorityFilter, setPriorityFilter] = useState<string>("all");
  const [sortField, setSortField] = useState<keyof Task>("updatedAt");
  const [sortDirection, setSortDirection] = useState<"asc" | "desc">("desc");
  
  // State for dialogs
  const [showCreateDialog, setShowCreateDialog] = useState(false);
  const [selectedTask, setSelectedTask] = useState<Task | null>(null);
  
  // Apply filters to tasks
  const filteredTasks = tasks
    .filter((task) => task.projectId === project.id)
    .filter((task) => {
      // Apply search filter
      if (
        searchQuery &&
        !task.title.toLowerCase().includes(searchQuery.toLowerCase()) &&
        !task.description.toLowerCase().includes(searchQuery.toLowerCase())
      ) {
        return false;
      }
      
      // Apply status filter
      if (statusFilter !== "all" && task.status !== statusFilter) {
        return false;
      }
      
      // Apply priority filter
      if (priorityFilter !== "all" && task.priority !== priorityFilter) {
        return false;
      }
      
      return true;
    });
  
  // Sort tasks
  const sortedTasks = [...filteredTasks].sort((a, b) => {
    let aValue = a[sortField];
    let bValue = b[sortField];
    
    // Handle null values
    if (aValue === null) return sortDirection === "asc" ? -1 : 1;
    if (bValue === null) return sortDirection === "asc" ? 1 : -1;
    
    // Handle date strings
    if (typeof aValue === "string" && typeof bValue === "string") {
      if (
        sortField === "startDate" ||
        sortField === "dueDate" ||
        sortField === "completedAt" ||
        sortField === "createdAt" ||
        sortField === "updatedAt"
      ) {
        aValue = aValue ? new Date(aValue).getTime() : 0;
        bValue = bValue ? new Date(bValue).getTime() : 0;
      }
    }
    
    // Compare values
    if (aValue < bValue) return sortDirection === "asc" ? -1 : 1;
    if (aValue > bValue) return sortDirection === "asc" ? 1 : -1;
    return 0;
  });
  
  // Handle sort
  const handleSort = (field: keyof Task) => {
    if (field === sortField) {
      setSortDirection(sortDirection === "asc" ? "desc" : "asc");
    } else {
      setSortField(field);
      setSortDirection("asc");
    }
  };
  
  // Handle task selection
  const handleTaskSelect = (task: Task) => {
    setSelectedTask(task);
  };
  
  // Get status icon
  const getStatusIcon = (status: TaskStatus) => {
    switch (status) {
      case "done":
        return <CheckCircle className="h-4 w-4 text-success" />;
      case "in_progress":
        return <Circle className="h-4 w-4 text-primary" />;
      case "review":
        return <PauseCircle className="h-4 w-4 text-accent" />;
      case "blocked":
        return <AlertCircle className="h-4 w-4 text-destructive" />;
      case "todo":
      default:
        return <Circle className="h-4 w-4 text-muted-foreground" />;
    }
  };
  
  // Get priority badge
  const getPriorityBadge = (priority: TaskPriority) => {
    switch (priority) {
      case "urgent":
        return (
          <Badge variant="destructive" className="text-xs">
            Urgent
          </Badge>
        );
      case "high":
        return (
          <Badge variant="default" className="text-xs">
            High
          </Badge>
        );
      case "medium":
        return (
          <Badge variant="secondary" className="text-xs">
            Medium
          </Badge>
        );
      case "low":
      default:
        return (
          <Badge variant="outline" className="text-xs">
            Low
          </Badge>
        );
    }
  };
  
  return (
    <div className="space-y-6">
      <div className="flex flex-col md:flex-row justify-between gap-4">
        <div className="flex flex-col md:flex-row gap-4 flex-grow">
          <div className="relative flex-grow">
            <Search className="absolute left-2.5 top-2.5 h-4 w-4 text-muted-foreground" />
            <Input
              placeholder="Search tasks..."
              className="pl-8"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
            />
          </div>
          
          <div className="flex gap-2">
            <Select value={statusFilter} onValueChange={setStatusFilter}>
              <SelectTrigger className="w-[130px]">
                <div className="flex items-center">
                  <Filter className="mr-2 h-4 w-4" />
                  <span>Status</span>
                </div>
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All Statuses</SelectItem>
                <SelectItem value="todo">To Do</SelectItem>
                <SelectItem value="in_progress">In Progress</SelectItem>
                <SelectItem value="review">Review</SelectItem>
                <SelectItem value="done">Done</SelectItem>
                <SelectItem value="blocked">Blocked</SelectItem>
              </SelectContent>
            </Select>
            
            <Select value={priorityFilter} onValueChange={setPriorityFilter}>
              <SelectTrigger className="w-[130px]">
                <div className="flex items-center">
                  <Filter className="mr-2 h-4 w-4" />
                  <span>Priority</span>
                </div>
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">All Priorities</SelectItem>
                <SelectItem value="urgent">Urgent</SelectItem>
                <SelectItem value="high">High</SelectItem>
                <SelectItem value="medium">Medium</SelectItem>
                <SelectItem value="low">Low</SelectItem>
              </SelectContent>
            </Select>
          </div>
        </div>
        
        <Button onClick={() => setShowCreateDialog(true)}>
          <PlusCircle className="mr-2 h-4 w-4" />
          New Task
        </Button>
      </div>
      
      <Card>
        <CardHeader>
          <CardTitle>Tasks</CardTitle>
        </CardHeader>
        <CardContent>
          {sortedTasks.length > 0 ? (
            <div className="rounded-md border">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead className="w-[50px]">Status</TableHead>
                    <TableHead>
                      <div
                        className="flex items-center cursor-pointer"
                        onClick={() => handleSort("title")}
                      >
                        Title
                        <ArrowUpDown className="ml-2 h-4 w-4" />
                      </div>
                    </TableHead>
                    <TableHead className="w-[100px]">Priority</TableHead>
                    <TableHead className="w-[150px]">
                      <div
                        className="flex items-center cursor-pointer"
                        onClick={() => handleSort("dueDate")}
                      >
                        Due Date
                        <ArrowUpDown className="ml-2 h-4 w-4" />
                      </div>
                    </TableHead>
                    <TableHead className="w-[100px]">Assignee</TableHead>
                    <TableHead className="w-[80px]">Actions</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {sortedTasks.map((task) => (
                    <TableRow
                      key={task.id}
                      className="cursor-pointer hover:bg-secondary/10"
                      onClick={() => handleTaskSelect(task)}
                    >
                      <TableCell>
                        {getStatusIcon(task.status)}
                      </TableCell>
                      <TableCell>
                        <div className="font-medium">{task.title}</div>
                        <div className="text-sm text-muted-foreground line-clamp-1">
                          {task.description}
                        </div>
                      </TableCell>
                      <TableCell>{getPriorityBadge(task.priority)}</TableCell>
                      <TableCell>
                        {task.dueDate ? (
                          <div
                            className={`flex items-center ${
                              isTaskOverdue(task)
                                ? "text-destructive"
                                : ""
                            }`}
                          >
                            <Calendar className="mr-2 h-4 w-4" />
                            <span>
                              {new Date(task.dueDate).toLocaleDateString()}
                            </span>
                          </div>
                        ) : (
                          <span className="text-muted-foreground">
                            No due date
                          </span>
                        )}
                      </TableCell>
                      <TableCell>
                        {task.assigneeId ? (
                          <div className="flex items-center">
                            <div className="h-6 w-6 rounded-full bg-primary/10 flex items-center justify-center mr-2">
                              <span className="text-xs text-primary">
                                {task.assigneeId.substring(0, 2)}
                              </span>
                            </div>
                            <span className="text-xs">
                              {task.assigneeId.substring(0, 6)}...
                            </span>
                          </div>
                        ) : (
                          <span className="text-muted-foreground">
                            Unassigned
                          </span>
                        )}
                      </TableCell>
                      <TableCell>
                        <DropdownMenu>
                          <DropdownMenuTrigger asChild>
                            <Button
                              variant="ghost"
                              className="h-8 w-8 p-0"
                              onClick={(e) => e.stopPropagation()}
                            >
                              <MoreHorizontal className="h-4 w-4" />
                            </Button>
                          </DropdownMenuTrigger>
                          <DropdownMenuContent align="end">
                            <DropdownMenuItem
                              onClick={(e) => {
                                e.stopPropagation();
                                handleTaskSelect(task);
                              }}
                            >
                              View Details
                            </DropdownMenuItem>
                            <DropdownMenuItem
                              onClick={(e) => {
                                e.stopPropagation();
                                // Handle edit task
                              }}
                            >
                              Edit Task
                            </DropdownMenuItem>
                            <DropdownMenuItem
                              onClick={(e) => {
                                e.stopPropagation();
                                // Handle delete task
                              }}
                              className="text-destructive"
                            >
                              Delete Task
                            </DropdownMenuItem>
                          </DropdownMenuContent>
                        </DropdownMenu>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>
          ) : (
            <div className="text-center py-12">
              <div className="mx-auto w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center mb-4">
                <CheckCircle className="h-6 w-6 text-primary" />
              </div>
              <h3 className="text-lg font-medium">No tasks found</h3>
              <p className="text-muted-foreground mt-1 mb-4">
                {tasks.length > 0
                  ? "Try adjusting your filters"
                  : "Create your first task to get started"}
              </p>
              {tasks.length === 0 && (
                <Button onClick={() => setShowCreateDialog(true)}>
                  <PlusCircle className="mr-2 h-4 w-4" />
                  New Task
                </Button>
              )}
            </div>
          )}
        </CardContent>
      </Card>
      
      {/* Create Task Dialog */}
      {showCreateDialog && (
        <TaskCreateDialog
          open={showCreateDialog}
          onOpenChange={setShowCreateDialog}
          projectId={project.id}
        />
      )}
      
      {/* Task Detail Dialog */}
      {selectedTask && (
        <TaskDetailDialog
          open={!!selectedTask}
          onOpenChange={() => setSelectedTask(null)}
          task={selectedTask}
        />
      )}
    </div>
  );
};

export default ProjectTasks;
```

This component provides a comprehensive task management interface for a project, including:

1. Search and filtering capabilities for tasks
2. Sorting functionality for different task fields
3. A table view of tasks with status, title, priority, due date, and assignee
4. Actions for viewing, editing, and deleting tasks
5. Integration with task creation and detail dialogs

The component handles cases where no tasks are found, either due to filtering or because no tasks have been created yet, showing appropriate messages and actions.

In the next part, we'll implement the task creation and detail dialogs to complete the task management functionality.
