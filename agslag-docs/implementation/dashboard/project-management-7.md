# Project Management Interface Implementation - Part 7

This document continues the implementation plan for the project management interface, focusing on the project timeline component.

## Implementation Details

### 1. Project Timeline Component

Create a component to visualize project timeline with tasks and milestones:

```typescript
// src/components/projects/ProjectTimeline.tsx
"use client";

import React, { useState } from "react";
import { Project, Task, Milestone, getTaskDurationDays } from "@/types/project";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";
import {
  Calendar,
  ChevronLeft,
  ChevronRight,
  Flag,
  Filter,
  PlusCircle,
} from "lucide-react";
import { format, addMonths, subMonths, startOfMonth, endOfMonth, eachDayOfInterval, isSameDay, isWeekend, addDays } from "date-fns";
import MilestoneCreateDialog from "./MilestoneCreateDialog";

interface ProjectTimelineProps {
  project: Project;
  tasks: Task[];
  milestones: Milestone[];
}

const ProjectTimeline: React.FC<ProjectTimelineProps> = ({
  project,
  tasks,
  milestones,
}) => {
  // State for current view date
  const [currentDate, setCurrentDate] = useState(new Date());
  const [viewMode, setViewMode] = useState<"month" | "quarter">("month");
  const [showMilestoneDialog, setShowMilestoneDialog] = useState(false);
  
  // Filter state
  const [statusFilter, setStatusFilter] = useState<string>("all");
  
  // Calculate start and end dates for the current view
  const startDate = startOfMonth(currentDate);
  const endDate = viewMode === "month"
    ? endOfMonth(currentDate)
    : endOfMonth(addMonths(currentDate, 2));
  
  // Generate days for the timeline
  const days = eachDayOfInterval({ start: startDate, end: endDate });
  
  // Filter tasks based on date range and status
  const filteredTasks = tasks.filter((task) => {
    // Skip tasks without start or due dates
    if (!task.startDate || !task.dueDate) return false;
    
    const taskStart = new Date(task.startDate);
    const taskEnd = new Date(task.dueDate);
    
    // Check if task overlaps with the current view
    const isInRange =
      (taskStart <= endDate && taskEnd >= startDate) ||
      (taskStart >= startDate && taskStart <= endDate) ||
      (taskEnd >= startDate && taskEnd <= endDate);
    
    // Apply status filter
    const matchesStatus = statusFilter === "all" || task.status === statusFilter;
    
    return isInRange && matchesStatus;
  });
  
  // Filter milestones based on date range
  const filteredMilestones = milestones.filter((milestone) => {
    const milestoneDate = new Date(milestone.date);
    return milestoneDate >= startDate && milestoneDate <= endDate;
  });
  
  // Navigate to previous period
  const goToPrevious = () => {
    setCurrentDate(viewMode === "month"
      ? subMonths(currentDate, 1)
      : subMonths(currentDate, 3)
    );
  };
  
  // Navigate to next period
  const goToNext = () => {
    setCurrentDate(viewMode === "month"
      ? addMonths(currentDate, 1)
      : addMonths(currentDate, 3)
    );
  };
  
  // Get task position and width for the timeline
  const getTaskStyle = (task: Task) => {
    const taskStart = new Date(task.startDate!);
    const taskEnd = new Date(task.dueDate!);
    
    // Calculate position
    const totalDays = days.length;
    const startDiff = Math.max(0, Math.floor((taskStart.getTime() - startDate.getTime()) / (1000 * 60 * 60 * 24)));
    const duration = Math.max(1, getTaskDurationDays(task));
    
    // Ensure task doesn't extend beyond the timeline
    const adjustedDuration = Math.min(duration, totalDays - startDiff);
    
    // Calculate left position and width as percentages
    const left = (startDiff / totalDays) * 100;
    const width = (adjustedDuration / totalDays) * 100;
    
    return {
      left: `${left}%`,
      width: `${width}%`,
    };
  };
  
  // Get milestone position for the timeline
  const getMilestonePosition = (milestone: Milestone) => {
    const milestoneDate = new Date(milestone.date);
    const totalDays = days.length;
    const dayDiff = Math.floor((milestoneDate.getTime() - startDate.getTime()) / (1000 * 60 * 60 * 24));
    
    // Calculate left position as percentage
    const left = (dayDiff / totalDays) * 100;
    
    return {
      left: `${left}%`,
    };
  };
  
  // Get color based on task status
  const getTaskColor = (task: Task) => {
    switch (task.status) {
      case "done":
        return "bg-success text-success-foreground";
      case "in_progress":
        return "bg-primary text-primary-foreground";
      case "review":
        return "bg-accent text-accent-foreground";
      case "blocked":
        return "bg-destructive text-destructive-foreground";
      case "todo":
      default:
        return "bg-secondary text-secondary-foreground";
    }
  };
  
  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div className="flex items-center space-x-2">
          <Button variant="outline" size="icon" onClick={goToPrevious}>
            <ChevronLeft className="h-4 w-4" />
          </Button>
          <h2 className="text-xl font-semibold">
            {viewMode === "month"
              ? format(currentDate, "MMMM yyyy")
              : `${format(currentDate, "MMM")} - ${format(
                  addMonths(currentDate, 2),
                  "MMM yyyy"
                )}`}
          </h2>
          <Button variant="outline" size="icon" onClick={goToNext}>
            <ChevronRight className="h-4 w-4" />
          </Button>
        </div>
        
        <div className="flex items-center space-x-2">
          <Tabs
            value={viewMode}
            onValueChange={(value) => setViewMode(value as "month" | "quarter")}
            className="mr-4"
          >
            <TabsList>
              <TabsTrigger value="month">Month</TabsTrigger>
              <TabsTrigger value="quarter">Quarter</TabsTrigger>
            </TabsList>
          </Tabs>
          
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
          
          <Button onClick={() => setShowMilestoneDialog(true)}>
            <PlusCircle className="mr-2 h-4 w-4" />
            Add Milestone
          </Button>
        </div>
      </div>
      
      <Card>
        <CardHeader>
          <CardTitle>Project Timeline</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="overflow-x-auto">
            <div className="min-w-[800px]">
              {/* Timeline header */}
              <div className="flex border-b">
                <div className="w-1/6 p-2 font-medium">Task</div>
                <div className="w-5/6 flex">
                  {days.map((day, index) => (
                    <div
                      key={day.toISOString()}
                      className={`flex-1 text-center text-xs p-1 ${
                        isWeekend(day) ? "bg-secondary/20" : ""
                      }`}
                    >
                      {index % 2 === 0 && format(day, "d")}
                    </div>
                  ))}
                </div>
              </div>
              
              {/* Week row */}
              <div className="flex border-b">
                <div className="w-1/6 p-2"></div>
                <div className="w-5/6 flex">
                  {days.map((day) => (
                    <div
                      key={`week-${day.toISOString()}`}
                      className={`flex-1 text-center text-xs p-1 ${
                        isWeekend(day) ? "bg-secondary/20" : ""
                      }`}
                    >
                      {format(day, "EEE").charAt(0)}
                    </div>
                  ))}
                </div>
              </div>
              
              {/* Milestones row */}
              {filteredMilestones.length > 0 && (
                <div className="flex border-b relative h-10">
                  <div className="w-1/6 p-2 font-medium">Milestones</div>
                  <div className="w-5/6 relative">
                    {filteredMilestones.map((milestone) => (
                      <TooltipProvider key={milestone.id}>
                        <Tooltip>
                          <TooltipTrigger asChild>
                            <div
                              className="absolute top-1 h-8 flex items-center"
                              style={getMilestonePosition(milestone)}
                            >
                              <div
                                className={`h-6 w-6 rounded-full flex items-center justify-center ${
                                  milestone.completed
                                    ? "bg-success text-success-foreground"
                                    : "bg-accent text-accent-foreground"
                                }`}
                              >
                                <Flag className="h-3 w-3" />
                              </div>
                            </div>
                          </TooltipTrigger>
                          <TooltipContent>
                            <div className="space-y-1">
                              <p className="font-medium">{milestone.title}</p>
                              <p className="text-xs">
                                {format(new Date(milestone.date), "PPP")}
                              </p>
                              <p className="text-xs text-muted-foreground">
                                {milestone.description}
                              </p>
                            </div>
                          </TooltipContent>
                        </Tooltip>
                      </TooltipProvider>
                    ))}
                  </div>
                </div>
              )}
              
              {/* Tasks rows */}
              {filteredTasks.length > 0 ? (
                filteredTasks.map((task) => (
                  <div key={task.id} className="flex border-b relative h-10">
                    <div className="w-1/6 p-2 truncate" title={task.title}>
                      {task.title}
                    </div>
                    <div className="w-5/6 relative">
                      <TooltipProvider>
                        <Tooltip>
                          <TooltipTrigger asChild>
                            <div
                              className={`absolute top-1 h-8 rounded-md ${getTaskColor(
                                task
                              )} flex items-center px-2`}
                              style={getTaskStyle(task)}
                            >
                              <span className="truncate text-xs">
                                {task.title}
                              </span>
                            </div>
                          </TooltipTrigger>
                          <TooltipContent>
                            <div className="space-y-1">
                              <p className="font-medium">{task.title}</p>
                              <div className="flex items-center space-x-2">
                                <Badge variant="outline">
                                  {task.status.replace("_", " ")}
                                </Badge>
                                <Badge variant="outline">
                                  {task.priority}
                                </Badge>
                              </div>
                              <p className="text-xs">
                                {format(new Date(task.startDate!), "PPP")} -{" "}
                                {format(new Date(task.dueDate!), "PPP")}
                              </p>
                              <p className="text-xs text-muted-foreground">
                                {task.description.substring(0, 100)}
                                {task.description.length > 100 ? "..." : ""}
                              </p>
                            </div>
                          </TooltipContent>
                        </Tooltip>
                      </TooltipProvider>
                    </div>
                  </div>
                ))
              ) : (
                <div className="text-center py-12">
                  <div className="mx-auto w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center mb-4">
                    <Calendar className="h-6 w-6 text-primary" />
                  </div>
                  <h3 className="text-lg font-medium">No tasks in this period</h3>
                  <p className="text-muted-foreground mt-1">
                    {statusFilter !== "all"
                      ? "Try changing your status filter"
                      : "Add tasks with start and due dates to see them here"}
                  </p>
                </div>
              )}
            </div>
          </div>
        </CardContent>
      </Card>
      
      {/* Milestone Create Dialog */}
      {showMilestoneDialog && (
        <MilestoneCreateDialog
          open={showMilestoneDialog}
          onOpenChange={setShowMilestoneDialog}
          projectId={project.id}
        />
      )}
    </div>
  );
};

export default ProjectTimeline;
```

### 2. Milestone Create Dialog

Create a dialog for creating new milestones:

```typescript
// src/components/projects/MilestoneCreateDialog.tsx
"use client";

import React, { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { Calendar } from "@/components/ui/calendar";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
import { Checkbox } from "@/components/ui/checkbox";
import { format } from "date-fns";
import { CalendarIcon, Loader2 } from "lucide-react";
import * as projectApi from "@/lib/api/project-api";

interface MilestoneCreateDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  projectId: string;
}

const MilestoneCreateDialog: React.FC<MilestoneCreateDialogProps> = ({
  open,
  onOpenChange,
  projectId,
}) => {
  // Form state
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [date, setDate] = useState<Date | undefined>(new Date());
  const [completed, setCompleted] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errors, setErrors] = useState<Record<string, string>>({});
  
  // Validate form
  const validateForm = () => {
    const newErrors: Record<string, string> = {};
    
    if (!title.trim()) {
      newErrors.title = "Milestone title is required";
    }
    
    if (!date) {
      newErrors.date = "Date is required";
    }
    
    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };
  
  // Handle form submission
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (!validateForm()) {
      return;
    }
    
    setIsSubmitting(true);
    
    try {
      await projectApi.createMilestone({
        projectId,
        title,
        description,
        date: date!.toISOString(),
        completed,
        tasks: [],
      });
      
      // Reset form
      setTitle("");
      setDescription("");
      setDate(new Date());
      setCompleted(false);
      
      // Close dialog
      onOpenChange(false);
    } catch (error) {
      console.error("Failed to create milestone:", error);
    } finally {
      setIsSubmitting(false);
    }
  };
  
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[500px]">
        <form onSubmit={handleSubmit}>
          <DialogHeader>
            <DialogTitle>Create New Milestone</DialogTitle>
            <DialogDescription>
              Add a milestone to mark important project events.
            </DialogDescription>
          </DialogHeader>
          
          <div className="grid gap-4 py-4">
            <div className="grid gap-2">
              <Label htmlFor="title" className="required">
                Milestone Title
              </Label>
              <Input
                id="title"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                placeholder="Enter milestone title"
                className={errors.title ? "border-destructive" : ""}
              />
              {errors.title && (
                <p className="text-sm text-destructive">{errors.title}</p>
              )}
            </div>
            
            <div className="grid gap-2">
              <Label htmlFor="description">
                Description
              </Label>
              <Textarea
                id="description"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="Enter milestone description"
                rows={3}
              />
            </div>
            
            <div className="grid gap-2">
              <Label htmlFor="date" className="required">
                Date
              </Label>
              <Popover>
                <PopoverTrigger asChild>
                  <Button
                    variant="outline"
                    className={`w-full justify-start text-left font-normal ${
                      errors.date ? "border-destructive" : ""
                    }`}
                    id="date"
                  >
                    <CalendarIcon className="mr-2 h-4 w-4" />
                    {date ? (
                      format(date, "PPP")
                    ) : (
                      <span>Pick a date</span>
                    )}
                  </Button>
                </PopoverTrigger>
                <PopoverContent className="w-auto p-0">
                  <Calendar
                    mode="single"
                    selected={date}
                    onSelect={setDate}
                    initialFocus
                  />
                </PopoverContent>
              </Popover>
              {errors.date && (
                <p className="text-sm text-destructive">{errors.date}</p>
              )}
            </div>
            
            <div className="flex items-center space-x-2">
              <Checkbox
                id="completed"
                checked={completed}
                onCheckedChange={(checked) => setCompleted(checked as boolean)}
              />
              <Label htmlFor="completed">
                Mark as completed
              </Label>
            </div>
          </div>
          
          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
              disabled={isSubmitting}
            >
              Cancel
            </Button>
            <Button type="submit" disabled={isSubmitting}>
              {isSubmitting && (
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
              )}
              Create Milestone
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};

export default MilestoneCreateDialog;
```

These components provide a comprehensive timeline visualization for a project, including:

1. A Gantt chart-like timeline view showing tasks and milestones
2. Month and quarter view modes for different time scales
3. Navigation controls to move between time periods
4. Status filtering for tasks
5. Tooltips with detailed information when hovering over tasks and milestones
6. A milestone creation dialog for adding new project milestones

The timeline visualization helps project managers and team members understand the project schedule at a glance, see task dependencies, and track progress against milestones. The implementation handles cases where no tasks or milestones exist in the current time period, showing appropriate messages.

With these components, we've completed the core functionality of the project management interface, providing a comprehensive set of tools for managing projects, teams, tasks, and timelines.
