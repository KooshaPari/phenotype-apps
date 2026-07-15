# Project Management Interface Implementation - Part 5

This document continues the implementation plan for the project management interface, focusing on the task creation and detail dialogs.

## Implementation Details

### 1. Task Create Dialog

Create a dialog for creating new tasks:

```typescript
// src/components/projects/TaskCreateDialog.tsx
"use client";

import React, { useState } from "react";
import { useProject } from "@/contexts/ProjectContext";
import { useApp } from "@/contexts/AppContext";
import { TaskPriority, TaskStatus } from "@/types/project";
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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Calendar } from "@/components/ui/calendar";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
import { format } from "date-fns";
import { CalendarIcon, Loader2 } from "lucide-react";

interface TaskCreateDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  projectId: string;
}

const TaskCreateDialog: React.FC<TaskCreateDialogProps> = ({
  open,
  onOpenChange,
  projectId,
}) => {
  const { createNewTask } = useProject();
  const { state: appState } = useApp();
  const { agents } = appState;
  
  // Form state
  const [title, setTitle] = useState("");
  const [description, setDescription] = useState("");
  const [status, setStatus] = useState<TaskStatus>("todo");
  const [priority, setPriority] = useState<TaskPriority>("medium");
  const [assigneeId, setAssigneeId] = useState<string | null>(null);
  const [startDate, setStartDate] = useState<Date | undefined>(new Date());
  const [dueDate, setDueDate] = useState<Date | undefined>(undefined);
  const [estimatedHours, setEstimatedHours] = useState<number | null>(null);
  const [tags, setTags] = useState<string[]>([]);
  const [tagInput, setTagInput] = useState("");
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [errors, setErrors] = useState<Record<string, string>>({});
  
  // Validate form
  const validateForm = () => {
    const newErrors: Record<string, string> = {};
    
    if (!title.trim()) {
      newErrors.title = "Task title is required";
    }
    
    if (!description.trim()) {
      newErrors.description = "Task description is required";
    }
    
    if (startDate && dueDate && startDate > dueDate) {
      newErrors.dueDate = "Due date must be after start date";
    }
    
    if (estimatedHours !== null && estimatedHours < 0) {
      newErrors.estimatedHours = "Estimated hours cannot be negative";
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
      await createNewTask({
        projectId,
        title,
        description,
        status,
        priority,
        assigneeId,
        startDate: startDate?.toISOString() || null,
        dueDate: dueDate?.toISOString() || null,
        completedAt: status === "done" ? new Date().toISOString() : null,
        estimatedHours,
        actualHours: 0,
        dependencies: [],
        tags,
      });
      
      // Reset form
      setTitle("");
      setDescription("");
      setStatus("todo");
      setPriority("medium");
      setAssigneeId(null);
      setStartDate(new Date());
      setDueDate(undefined);
      setEstimatedHours(null);
      setTags([]);
      setTagInput("");
      
      // Close dialog
      onOpenChange(false);
    } catch (error) {
      console.error("Failed to create task:", error);
    } finally {
      setIsSubmitting(false);
    }
  };
  
  // Handle tag input
  const handleTagInputKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" && tagInput.trim()) {
      e.preventDefault();
      if (!tags.includes(tagInput.trim())) {
        setTags([...tags, tagInput.trim()]);
      }
      setTagInput("");
    }
  };
  
  // Remove tag
  const removeTag = (tagToRemove: string) => {
    setTags(tags.filter((tag) => tag !== tagToRemove));
  };
  
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[600px]">
        <form onSubmit={handleSubmit}>
          <DialogHeader>
            <DialogTitle>Create New Task</DialogTitle>
            <DialogDescription>
              Create a new task for this project.
            </DialogDescription>
          </DialogHeader>
          
          <div className="grid gap-4 py-4">
            <div className="grid gap-2">
              <Label htmlFor="title" className="required">
                Task Title
              </Label>
              <Input
                id="title"
                value={title}
                onChange={(e) => setTitle(e.target.value)}
                placeholder="Enter task title"
                className={errors.title ? "border-destructive" : ""}
              />
              {errors.title && (
                <p className="text-sm text-destructive">{errors.title}</p>
              )}
            </div>
            
            <div className="grid gap-2">
              <Label htmlFor="description" className="required">
                Description
              </Label>
              <Textarea
                id="description"
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="Enter task description"
                rows={3}
                className={errors.description ? "border-destructive" : ""}
              />
              {errors.description && (
                <p className="text-sm text-destructive">{errors.description}</p>
              )}
            </div>
            
            <div className="grid grid-cols-2 gap-4">
              <div className="grid gap-2">
                <Label htmlFor="status">Status</Label>
                <Select
                  value={status}
                  onValueChange={(value) => setStatus(value as TaskStatus)}
                >
                  <SelectTrigger id="status">
                    <SelectValue placeholder="Select status" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="todo">To Do</SelectItem>
                    <SelectItem value="in_progress">In Progress</SelectItem>
                    <SelectItem value="review">Review</SelectItem>
                    <SelectItem value="done">Done</SelectItem>
                    <SelectItem value="blocked">Blocked</SelectItem>
                  </SelectContent>
                </Select>
              </div>
              
              <div className="grid gap-2">
                <Label htmlFor="priority">Priority</Label>
                <Select
                  value={priority}
                  onValueChange={(value) => setPriority(value as TaskPriority)}
                >
                  <SelectTrigger id="priority">
                    <SelectValue placeholder="Select priority" />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="low">Low</SelectItem>
                    <SelectItem value="medium">Medium</SelectItem>
                    <SelectItem value="high">High</SelectItem>
                    <SelectItem value="urgent">Urgent</SelectItem>
                  </SelectContent>
                </Select>
              </div>
            </div>
            
            <div className="grid gap-2">
              <Label htmlFor="assignee">Assignee</Label>
              <Select
                value={assigneeId || ""}
                onValueChange={(value) => setAssigneeId(value || null)}
              >
                <SelectTrigger id="assignee">
                  <SelectValue placeholder="Select assignee" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="">Unassigned</SelectItem>
                  {agents.map((agent) => (
                    <SelectItem key={agent.id} value={agent.id}>
                      {agent.name}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            
            <div className="grid grid-cols-2 gap-4">
              <div className="grid gap-2">
                <Label htmlFor="startDate">Start Date</Label>
                <Popover>
                  <PopoverTrigger asChild>
                    <Button
                      variant="outline"
                      className="w-full justify-start text-left font-normal"
                    >
                      <CalendarIcon className="mr-2 h-4 w-4" />
                      {startDate ? (
                        format(startDate, "PPP")
                      ) : (
                        <span>Pick a date</span>
                      )}
                    </Button>
                  </PopoverTrigger>
                  <PopoverContent className="w-auto p-0">
                    <Calendar
                      mode="single"
                      selected={startDate}
                      onSelect={setStartDate}
                      initialFocus
                    />
                  </PopoverContent>
                </Popover>
              </div>
              
              <div className="grid gap-2">
                <Label htmlFor="dueDate">Due Date</Label>
                <Popover>
                  <PopoverTrigger asChild>
                    <Button
                      variant="outline"
                      className={`w-full justify-start text-left font-normal ${
                        errors.dueDate ? "border-destructive" : ""
                      }`}
                    >
                      <CalendarIcon className="mr-2 h-4 w-4" />
                      {dueDate ? (
                        format(dueDate, "PPP")
                      ) : (
                        <span>Pick a date</span>
                      )}
                    </Button>
                  </PopoverTrigger>
                  <PopoverContent className="w-auto p-0">
                    <Calendar
                      mode="single"
                      selected={dueDate}
                      onSelect={setDueDate}
                      initialFocus
                      disabled={(date) =>
                        startDate ? date < startDate : false
                      }
                    />
                  </PopoverContent>
                </Popover>
                {errors.dueDate && (
                  <p className="text-sm text-destructive">{errors.dueDate}</p>
                )}
              </div>
            </div>
            
            <div className="grid gap-2">
              <Label htmlFor="estimatedHours">Estimated Hours</Label>
              <Input
                id="estimatedHours"
                type="number"
                min="0"
                step="0.5"
                value={estimatedHours === null ? "" : estimatedHours}
                onChange={(e) =>
                  setEstimatedHours(
                    e.target.value ? parseFloat(e.target.value) : null
                  )
                }
                placeholder="Enter estimated hours"
                className={errors.estimatedHours ? "border-destructive" : ""}
              />
              {errors.estimatedHours && (
                <p className="text-sm text-destructive">
                  {errors.estimatedHours}
                </p>
              )}
            </div>
            
            <div className="grid gap-2">
              <Label htmlFor="tags">Tags</Label>
              <div className="flex flex-wrap gap-2 mb-2">
                {tags.map((tag) => (
                  <div
                    key={tag}
                    className="bg-secondary text-secondary-foreground px-2 py-1 rounded-md text-sm flex items-center"
                  >
                    {tag}
                    <button
                      type="button"
                      className="ml-2 text-secondary-foreground/70 hover:text-secondary-foreground"
                      onClick={() => removeTag(tag)}
                    >
                      &times;
                    </button>
                  </div>
                ))}
              </div>
              <Input
                id="tags"
                value={tagInput}
                onChange={(e) => setTagInput(e.target.value)}
                onKeyDown={handleTagInputKeyDown}
                placeholder="Type a tag and press Enter"
              />
              <p className="text-xs text-muted-foreground">
                Press Enter to add a tag
              </p>
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
              Create Task
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
};

export default TaskCreateDialog;
```

### 2. Task Detail Dialog

Create a dialog for viewing task details:

```typescript
// src/components/projects/TaskDetailDialog.tsx
"use client";

import React, { useState } from "react";
import { useProject } from "@/contexts/ProjectContext";
import { Task, TaskPriority, TaskStatus } from "@/types/project";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
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
import { Textarea } from "@/components/ui/textarea";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Separator } from "@/components/ui/separator";
import {
  Calendar,
  Clock,
  Edit,
  CheckCircle,
  AlertCircle,
  Circle,
  PauseCircle,
  Save,
  Loader2,
  Tag,
  User,
} from "lucide-react";

interface TaskDetailDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  task: Task;
}

const TaskDetailDialog: React.FC<TaskDetailDialogProps> = ({
  open,
  onOpenChange,
  task,
}) => {
  const { updateExistingTask } = useProject();
  
  // State for edit mode
  const [isEditing, setIsEditing] = useState(false);
  const [isSubmitting, setIsSubmitting] = useState(false);
  
  // Form state
  const [editedTask, setEditedTask] = useState<Task>(task);
  
  // Handle edit toggle
  const handleEditToggle = () => {
    if (isEditing) {
      // Cancel edit
      setEditedTask(task);
    }
    setIsEditing(!isEditing);
  };
  
  // Handle form submission
  const handleSubmit = async () => {
    setIsSubmitting(true);
    
    try {
      await updateExistingTask(task.id, {
        title: editedTask.title,
        description: editedTask.description,
        status: editedTask.status,
        priority: editedTask.priority,
        assigneeId: editedTask.assigneeId,
        startDate: editedTask.startDate,
        dueDate: editedTask.dueDate,
        estimatedHours: editedTask.estimatedHours,
        actualHours: editedTask.actualHours,
        tags: editedTask.tags,
      });
      
      setIsEditing(false);
    } catch (error) {
      console.error("Failed to update task:", error);
    } finally {
      setIsSubmitting(false);
    }
  };
  
  // Handle field change
  const handleChange = (
    field: keyof Task,
    value: string | number | string[] | null
  ) => {
    setEditedTask({
      ...editedTask,
      [field]: value,
    });
  };
  
  // Get status icon
  const getStatusIcon = (status: TaskStatus) => {
    switch (status) {
      case "done":
        return <CheckCircle className="h-5 w-5 text-success" />;
      case "in_progress":
        return <Circle className="h-5 w-5 text-primary" />;
      case "review":
        return <PauseCircle className="h-5 w-5 text-accent" />;
      case "blocked":
        return <AlertCircle className="h-5 w-5 text-destructive" />;
      case "todo":
      default:
        return <Circle className="h-5 w-5 text-muted-foreground" />;
    }
  };
  
  // Get priority badge
  const getPriorityBadge = (priority: TaskPriority) => {
    switch (priority) {
      case "urgent":
        return (
          <Badge variant="destructive">
            Urgent
          </Badge>
        );
      case "high":
        return (
          <Badge variant="default">
            High
          </Badge>
        );
      case "medium":
        return (
          <Badge variant="secondary">
            Medium
          </Badge>
        );
      case "low":
      default:
        return (
          <Badge variant="outline">
            Low
          </Badge>
        );
    }
  };
  
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[700px]">
        <DialogHeader>
          <div className="flex justify-between items-start">
            <div className="flex items-center gap-2">
              {getStatusIcon(task.status)}
              <DialogTitle className="text-xl">
                {task.title}
              </DialogTitle>
            </div>
            <Button
              variant="outline"
              size="sm"
              onClick={handleEditToggle}
              disabled={isSubmitting}
            >
              {isEditing ? "Cancel" : <Edit className="h-4 w-4 mr-2" />}
              {isEditing ? "Cancel" : "Edit"}
            </Button>
          </div>
        </DialogHeader>
        
        <Tabs defaultValue="details">
          <TabsList className="grid grid-cols-2 mb-4">
            <TabsTrigger value="details">Details</TabsTrigger>
            <TabsTrigger value="activity">Activity</TabsTrigger>
          </TabsList>
          
          <TabsContent value="details" className="space-y-4">
            {isEditing ? (
              // Edit mode
              <div className="space-y-4">
                <div className="grid gap-2">
                  <Label htmlFor="title">Title</Label>
                  <Input
                    id="title"
                    value={editedTask.title}
                    onChange={(e) => handleChange("title", e.target.value)}
                  />
                </div>
                
                <div className="grid gap-2">
                  <Label htmlFor="description">Description</Label>
                  <Textarea
                    id="description"
                    value={editedTask.description}
                    onChange={(e) => handleChange("description", e.target.value)}
                    rows={4}
                  />
                </div>
                
                <div className="grid grid-cols-2 gap-4">
                  <div className="grid gap-2">
                    <Label htmlFor="status">Status</Label>
                    <Select
                      value={editedTask.status}
                      onValueChange={(value) =>
                        handleChange("status", value as TaskStatus)
                      }
                    >
                      <SelectTrigger id="status">
                        <SelectValue placeholder="Select status" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="todo">To Do</SelectItem>
                        <SelectItem value="in_progress">In Progress</SelectItem>
                        <SelectItem value="review">Review</SelectItem>
                        <SelectItem value="done">Done</SelectItem>
                        <SelectItem value="blocked">Blocked</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                  
                  <div className="grid gap-2">
                    <Label htmlFor="priority">Priority</Label>
                    <Select
                      value={editedTask.priority}
                      onValueChange={(value) =>
                        handleChange("priority", value as TaskPriority)
                      }
                    >
                      <SelectTrigger id="priority">
                        <SelectValue placeholder="Select priority" />
                      </SelectTrigger>
                      <SelectContent>
                        <SelectItem value="low">Low</SelectItem>
                        <SelectItem value="medium">Medium</SelectItem>
                        <SelectItem value="high">High</SelectItem>
                        <SelectItem value="urgent">Urgent</SelectItem>
                      </SelectContent>
                    </Select>
                  </div>
                </div>
                
                <div className="grid grid-cols-2 gap-4">
                  <div className="grid gap-2">
                    <Label htmlFor="estimatedHours">Estimated Hours</Label>
                    <Input
                      id="estimatedHours"
                      type="number"
                      min="0"
                      step="0.5"
                      value={editedTask.estimatedHours === null ? "" : editedTask.estimatedHours}
                      onChange={(e) =>
                        handleChange(
                          "estimatedHours",
                          e.target.value ? parseFloat(e.target.value) : null
                        )
                      }
                    />
                  </div>
                  
                  <div className="grid gap-2">
                    <Label htmlFor="actualHours">Actual Hours</Label>
                    <Input
                      id="actualHours"
                      type="number"
                      min="0"
                      step="0.5"
                      value={editedTask.actualHours === null ? "" : editedTask.actualHours}
                      onChange={(e) =>
                        handleChange(
                          "actualHours",
                          e.target.value ? parseFloat(e.target.value) : null
                        )
                      }
                    />
                  </div>
                </div>
                
                <div className="grid gap-2">
                  <Label htmlFor="tags">Tags (comma separated)</Label>
                  <Input
                    id="tags"
                    value={editedTask.tags.join(", ")}
                    onChange={(e) =>
                      handleChange(
                        "tags",
                        e.target.value
                          .split(",")
                          .map((tag) => tag.trim())
                          .filter((tag) => tag)
                      )
                    }
                    placeholder="Enter tags separated by commas"
                  />
                </div>
              </div>
            ) : (
              // View mode
              <div className="space-y-4">
                <div>
                  <h3 className="text-sm font-medium text-muted-foreground mb-1">
                    Description
                  </h3>
                  <p className="whitespace-pre-line">{task.description}</p>
                </div>
                
                <Separator />
                
                <div className="grid grid-cols-2 gap-4">
                  <div>
                    <h3 className="text-sm font-medium text-muted-foreground mb-1">
                      Status
                    </h3>
                    <div className="flex items-center">
                      {getStatusIcon(task.status)}
                      <span className="ml-2 capitalize">
                        {task.status.replace("_", " ")}
                      </span>
                    </div>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-muted-foreground mb-1">
                      Priority
                    </h3>
                    <div>{getPriorityBadge(task.priority)}</div>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-muted-foreground mb-1">
                      Assignee
                    </h3>
                    <div className="flex items-center">
                      <User className="h-4 w-4 mr-2 text-muted-foreground" />
                      <span>
                        {task.assigneeId
                          ? `Agent ${task.assigneeId.substring(0, 8)}...`
                          : "Unassigned"}
                      </span>
                    </div>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-muted-foreground mb-1">
                      Created
                    </h3>
                    <div className="flex items-center">
                      <Clock className="h-4 w-4 mr-2 text-muted-foreground" />
                      <span>
                        {new Date(task.createdAt).toLocaleDateString()}
                      </span>
                    </div>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-muted-foreground mb-1">
                      Start Date
                    </h3>
                    <div className="flex items-center">
                      <Calendar className="h-4 w-4 mr-2 text-muted-foreground" />
                      <span>
                        {task.startDate
                          ? new Date(task.startDate).toLocaleDateString()
                          : "Not set"}
                      </span>
                    </div>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-muted-foreground mb-1">
                      Due Date
                    </h3>
                    <div className="flex items-center">
                      <Calendar className="h-4 w-4 mr-2 text-muted-foreground" />
                      <span>
                        {task.dueDate
                          ? new Date(task.dueDate).toLocaleDateString()
                          : "Not set"}
                      </span>
                    </div>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-muted-foreground mb-1">
                      Estimated Hours
                    </h3>
                    <span>
                      {task.estimatedHours !== null
                        ? `${task.estimatedHours} hours`
                        : "Not estimated"}
                    </span>
                  </div>
                  
                  <div>
                    <h3 className="text-sm font-medium text-muted-foreground mb-1">
                      Actual Hours
                    </h3>
                    <span>
                      {task.actualHours !== null
                        ? `${task.actualHours} hours`
                        : "Not tracked"}
                    </span>
                  </div>
                </div>
                
                <Separator />
                
                <div>
                  <h3 className="text-sm font-medium text-muted-foreground mb-2">
                    Tags
                  </h3>
                  {task.tags.length > 0 ? (
                    <div className="flex flex-wrap gap-2">
                      {task.tags.map((tag) => (
                        <Badge key={tag} variant="outline">
                          <Tag className="h-3 w-3 mr-1" />
                          {tag}
                        </Badge>
                      ))}
                    </div>
                  ) : (
                    <p className="text-muted-foreground text-sm">No tags</p>
                  )}
                </div>
              </div>
            )}
          </TabsContent>
          
          <TabsContent value="activity" className="space-y-4">
            <div className="text-center py-8">
              <div className="mx-auto w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center mb-4">
                <Clock className="h-6 w-6 text-primary" />
              </div>
              <h3 className="text-lg font-medium">Activity Log</h3>
              <p className="text-muted-foreground mt-1">
                Task activity will be shown here
              </p>
            </div>
          </TabsContent>
        </Tabs>
        
        {isEditing && (
          <DialogFooter>
            <Button
              variant="outline"
              onClick={handleEditToggle}
              disabled={isSubmitting}
            >
              Cancel
            </Button>
            <Button onClick={handleSubmit} disabled={isSubmitting}>
              {isSubmitting && (
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
              )}
              <Save className="mr-2 h-4 w-4" />
              Save Changes
            </Button>
          </DialogFooter>
        )}
      </DialogContent>
    </Dialog>
  );
};

export default TaskDetailDialog;
```

These components provide comprehensive task management functionality, including:

1. A task creation dialog with form validation and rich input options
2. A task detail dialog with both view and edit modes
3. Support for all task properties including title, description, status, priority, dates, hours, and tags
4. Visual indicators for task status and priority

The implementation follows a consistent design language and provides a smooth user experience for managing tasks within projects.
