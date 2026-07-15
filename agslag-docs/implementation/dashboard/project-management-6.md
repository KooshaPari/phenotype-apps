# Project Management Interface Implementation - Part 6

This document continues the implementation plan for the project management interface, focusing on the project team management component.

## Implementation Details

### 1. Project Team Component

Create a component to manage project teams:

```typescript
// src/components/projects/ProjectTeam.tsx
"use client";

import React, { useState } from "react";
import { Project, Team, TeamRole } from "@/types/project";
import { useApp } from "@/contexts/AppContext";
import { useProject } from "@/contexts/ProjectContext";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
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
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  PlusCircle,
  Users,
  UserPlus,
  MoreHorizontal,
  Loader2,
  UserMinus,
  Shield,
  User,
  Eye,
  Pencil,
} from "lucide-react";
import * as projectApi from "@/lib/api/project-api";

interface ProjectTeamProps {
  project: Project;
  teams: Team[];
}

const ProjectTeam: React.FC<ProjectTeamProps> = ({ project, teams }) => {
  const { state: appState } = useApp();
  const { dispatch, refreshTeams } = useProject();
  const { agents } = appState;
  
  // State for dialogs
  const [showCreateTeamDialog, setShowCreateTeamDialog] = useState(false);
  const [showAddMemberDialog, setShowAddMemberDialog] = useState(false);
  const [selectedTeam, setSelectedTeam] = useState<Team | null>(null);
  const [isSubmitting, setIsSubmitting] = useState(false);
  
  // Form state for creating a team
  const [teamName, setTeamName] = useState("");
  const [teamDescription, setTeamDescription] = useState("");
  
  // Form state for adding a member
  const [selectedAgentId, setSelectedAgentId] = useState("");
  const [selectedRole, setSelectedRole] = useState<TeamRole>("member");
  
  // Handle create team
  const handleCreateTeam = async () => {
    if (!teamName.trim()) return;
    
    setIsSubmitting(true);
    
    try {
      const newTeam = await projectApi.createTeam({
        name: teamName,
        description: teamDescription,
        members: [],
      });
      
      // Update project with new team
      await projectApi.updateProject(project.id, {
        teamIds: [...project.teamIds, newTeam.id],
      });
      
      // Refresh teams
      await refreshTeams();
      
      // Reset form
      setTeamName("");
      setTeamDescription("");
      
      // Close dialog
      setShowCreateTeamDialog(false);
    } catch (error) {
      console.error("Failed to create team:", error);
    } finally {
      setIsSubmitting(false);
    }
  };
  
  // Handle add member
  const handleAddMember = async () => {
    if (!selectedTeam || !selectedAgentId || !selectedRole) return;
    
    setIsSubmitting(true);
    
    try {
      await projectApi.addTeamMember(
        selectedTeam.id,
        selectedAgentId,
        selectedRole
      );
      
      // Refresh teams
      await refreshTeams();
      
      // Reset form
      setSelectedAgentId("");
      setSelectedRole("member");
      
      // Close dialog
      setShowAddMemberDialog(false);
    } catch (error) {
      console.error("Failed to add team member:", error);
    } finally {
      setIsSubmitting(false);
    }
  };
  
  // Handle remove member
  const handleRemoveMember = async (teamId: string, agentId: string) => {
    try {
      await projectApi.removeTeamMember(teamId, agentId);
      
      // Refresh teams
      await refreshTeams();
    } catch (error) {
      console.error("Failed to remove team member:", error);
    }
  };
  
  // Get role icon
  const getRoleIcon = (role: TeamRole) => {
    switch (role) {
      case "lead":
        return <Shield className="h-4 w-4 text-primary" />;
      case "reviewer":
        return <Eye className="h-4 w-4 text-accent" />;
      case "observer":
        return <Eye className="h-4 w-4 text-muted-foreground" />;
      case "member":
      default:
        return <User className="h-4 w-4 text-secondary-foreground" />;
    }
  };
  
  // Get agent by ID
  const getAgentById = (agentId: string) => {
    return agents.find((agent) => agent.id === agentId);
  };
  
  // Filter out agents already in the team
  const getAvailableAgents = (team: Team) => {
    const teamMemberIds = team.members.map((member) => member.agentId);
    return agents.filter((agent) => !teamMemberIds.includes(agent.id));
  };
  
  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-semibold">Project Teams</h2>
        <Button onClick={() => setShowCreateTeamDialog(true)}>
          <PlusCircle className="mr-2 h-4 w-4" />
          New Team
        </Button>
      </div>
      
      {teams.length > 0 ? (
        <div className="grid grid-cols-1 gap-6">
          {teams.map((team) => (
            <Card key={team.id}>
              <CardHeader className="pb-2">
                <div className="flex justify-between items-start">
                  <CardTitle className="text-lg">{team.name}</CardTitle>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => {
                      setSelectedTeam(team);
                      setShowAddMemberDialog(true);
                    }}
                  >
                    <UserPlus className="mr-2 h-4 w-4" />
                    Add Member
                  </Button>
                </div>
                <p className="text-sm text-muted-foreground">
                  {team.description}
                </p>
              </CardHeader>
              <CardContent>
                {team.members.length > 0 ? (
                  <div className="rounded-md border">
                    <Table>
                      <TableHeader>
                        <TableRow>
                          <TableHead>Agent</TableHead>
                          <TableHead>Role</TableHead>
                          <TableHead>Joined</TableHead>
                          <TableHead className="w-[80px]">Actions</TableHead>
                        </TableRow>
                      </TableHeader>
                      <TableBody>
                        {team.members.map((member) => {
                          const agent = getAgentById(member.agentId);
                          return (
                            <TableRow key={member.agentId}>
                              <TableCell>
                                <div className="flex items-center">
                                  <div className="h-8 w-8 rounded-full bg-primary/10 flex items-center justify-center mr-2">
                                    <Users className="h-4 w-4 text-primary" />
                                  </div>
                                  <div>
                                    <div className="font-medium">
                                      {agent?.name || "Unknown Agent"}
                                    </div>
                                    <div className="text-sm text-muted-foreground">
                                      {member.agentId.substring(0, 8)}...
                                    </div>
                                  </div>
                                </div>
                              </TableCell>
                              <TableCell>
                                <div className="flex items-center">
                                  {getRoleIcon(member.role)}
                                  <span className="ml-2 capitalize">
                                    {member.role}
                                  </span>
                                </div>
                              </TableCell>
                              <TableCell>
                                {new Date(member.joinedAt).toLocaleDateString()}
                              </TableCell>
                              <TableCell>
                                <DropdownMenu>
                                  <DropdownMenuTrigger asChild>
                                    <Button
                                      variant="ghost"
                                      className="h-8 w-8 p-0"
                                    >
                                      <MoreHorizontal className="h-4 w-4" />
                                    </Button>
                                  </DropdownMenuTrigger>
                                  <DropdownMenuContent align="end">
                                    <DropdownMenuItem
                                      onClick={() => {
                                        // Handle change role
                                      }}
                                    >
                                      <Pencil className="mr-2 h-4 w-4" />
                                      Change Role
                                    </DropdownMenuItem>
                                    <DropdownMenuItem
                                      onClick={() =>
                                        handleRemoveMember(
                                          team.id,
                                          member.agentId
                                        )
                                      }
                                      className="text-destructive"
                                    >
                                      <UserMinus className="mr-2 h-4 w-4" />
                                      Remove
                                    </DropdownMenuItem>
                                  </DropdownMenuContent>
                                </DropdownMenu>
                              </TableCell>
                            </TableRow>
                          );
                        })}
                      </TableBody>
                    </Table>
                  </div>
                ) : (
                  <div className="text-center py-8">
                    <div className="mx-auto w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center mb-4">
                      <Users className="h-6 w-6 text-primary" />
                    </div>
                    <h3 className="text-lg font-medium">No team members</h3>
                    <p className="text-muted-foreground mt-1 mb-4">
                      Add members to this team to get started
                    </p>
                    <Button
                      onClick={() => {
                        setSelectedTeam(team);
                        setShowAddMemberDialog(true);
                      }}
                    >
                      <UserPlus className="mr-2 h-4 w-4" />
                      Add Member
                    </Button>
                  </div>
                )}
              </CardContent>
            </Card>
          ))}
        </div>
      ) : (
        <div className="text-center py-12">
          <div className="mx-auto w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center mb-4">
            <Users className="h-6 w-6 text-primary" />
          </div>
          <h3 className="text-lg font-medium">No teams found</h3>
          <p className="text-muted-foreground mt-1 mb-4">
            Create a team to start collaborating
          </p>
          <Button onClick={() => setShowCreateTeamDialog(true)}>
            <PlusCircle className="mr-2 h-4 w-4" />
            New Team
          </Button>
        </div>
      )}
      
      {/* Create Team Dialog */}
      <Dialog open={showCreateTeamDialog} onOpenChange={setShowCreateTeamDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Create New Team</DialogTitle>
            <DialogDescription>
              Create a new team for this project.
            </DialogDescription>
          </DialogHeader>
          
          <div className="grid gap-4 py-4">
            <div className="grid gap-2">
              <Label htmlFor="team-name">Team Name</Label>
              <Input
                id="team-name"
                value={teamName}
                onChange={(e) => setTeamName(e.target.value)}
                placeholder="Enter team name"
              />
            </div>
            
            <div className="grid gap-2">
              <Label htmlFor="team-description">Description</Label>
              <Textarea
                id="team-description"
                value={teamDescription}
                onChange={(e) => setTeamDescription(e.target.value)}
                placeholder="Enter team description"
                rows={3}
              />
            </div>
          </div>
          
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => setShowCreateTeamDialog(false)}
              disabled={isSubmitting}
            >
              Cancel
            </Button>
            <Button onClick={handleCreateTeam} disabled={isSubmitting}>
              {isSubmitting && (
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
              )}
              Create Team
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
      
      {/* Add Member Dialog */}
      <Dialog open={showAddMemberDialog} onOpenChange={setShowAddMemberDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Add Team Member</DialogTitle>
            <DialogDescription>
              Add a new member to {selectedTeam?.name}.
            </DialogDescription>
          </DialogHeader>
          
          <div className="grid gap-4 py-4">
            <div className="grid gap-2">
              <Label htmlFor="agent">Agent</Label>
              <Select
                value={selectedAgentId}
                onValueChange={setSelectedAgentId}
              >
                <SelectTrigger id="agent">
                  <SelectValue placeholder="Select an agent" />
                </SelectTrigger>
                <SelectContent>
                  {selectedTeam &&
                    getAvailableAgents(selectedTeam).map((agent) => (
                      <SelectItem key={agent.id} value={agent.id}>
                        {agent.name}
                      </SelectItem>
                    ))}
                </SelectContent>
              </Select>
            </div>
            
            <div className="grid gap-2">
              <Label htmlFor="role">Role</Label>
              <Select
                value={selectedRole}
                onValueChange={(value) => setSelectedRole(value as TeamRole)}
              >
                <SelectTrigger id="role">
                  <SelectValue placeholder="Select a role" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="lead">Lead</SelectItem>
                  <SelectItem value="member">Member</SelectItem>
                  <SelectItem value="reviewer">Reviewer</SelectItem>
                  <SelectItem value="observer">Observer</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
          
          <DialogFooter>
            <Button
              variant="outline"
              onClick={() => setShowAddMemberDialog(false)}
              disabled={isSubmitting}
            >
              Cancel
            </Button>
            <Button onClick={handleAddMember} disabled={isSubmitting}>
              {isSubmitting && (
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
              )}
              Add Member
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </div>
  );
};

export default ProjectTeam;
```

This component provides comprehensive team management functionality for a project, including:

1. Creating new teams with names and descriptions
2. Viewing team members with their roles and join dates
3. Adding new members to teams with specific roles
4. Removing members from teams
5. Visual indicators for different team roles

The component handles cases where no teams exist yet or when a team has no members, showing appropriate messages and actions. It integrates with the project API to perform team management operations and refreshes the data after changes.

In the next part, we'll implement the project timeline component for visualizing tasks and milestones on a timeline.
