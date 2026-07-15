# Network Visualization Implementation - Part 3

This document continues the implementation plan for the network visualization component, focusing on the network page implementation and controls.

## Network Page Implementation

Create a comprehensive network visualization page:

```typescript
// src/app/network/page.tsx
"use client";

import React, { useState, useEffect } from "react";
import { useApp } from "@/contexts/AppContext";
import EnhancedNetworkGraph from "@/components/network/EnhancedNetworkGraph";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Label } from "@/components/ui/label";
import {
  Activity,
  Filter,
  RefreshCw,
  Network,
  Users,
  MessageSquare,
  Zap,
  Info,
} from "lucide-react";

// Define types for our data
interface Connection {
  source: string;
  target: string;
  strength: number;
  lastActive: number;
  type?: string;
}

interface ActivityItem {
  id: string;
  timestamp: string;
  type: string;
  source: string;
  target?: string;
  content: string;
}

// Mock data for connections and activities
// In a real implementation, this would come from the API
const mockConnections: Connection[] = [
  {
    source: "agent-1",
    target: "agent-2",
    strength: 0.8,
    lastActive: Date.now() - 5 * 60 * 1000, // 5 minutes ago
    type: "command",
  },
  {
    source: "agent-1",
    target: "agent-3",
    strength: 0.6,
    lastActive: Date.now() - 15 * 60 * 1000, // 15 minutes ago
    type: "data",
  },
  {
    source: "agent-2",
    target: "agent-4",
    strength: 0.9,
    lastActive: Date.now() - 2 * 60 * 1000, // 2 minutes ago
    type: "command",
  },
  {
    source: "agent-3",
    target: "agent-4",
    strength: 0.4,
    lastActive: Date.now() - 60 * 60 * 1000, // 1 hour ago
    type: "data",
  },
  {
    source: "agent-4",
    target: "agent-5",
    strength: 0.7,
    lastActive: Date.now() - 30 * 60 * 1000, // 30 minutes ago
  },
  {
    source: "agent-5",
    target: "agent-1",
    strength: 0.5,
    lastActive: Date.now() - 2 * 60 * 60 * 1000, // 2 hours ago
  },
];

const mockActivities: ActivityItem[] = [
  {
    id: "activity-1",
    timestamp: new Date(Date.now() - 5 * 60 * 1000).toISOString(),
    type: "message",
    source: "agent-1",
    target: "agent-2",
    content: "Requesting task status update",
  },
  {
    id: "activity-2",
    timestamp: new Date(Date.now() - 4 * 60 * 1000).toISOString(),
    type: "message",
    source: "agent-2",
    target: "agent-1",
    content: "Task 75% complete, estimated completion in 10 minutes",
  },
  {
    id: "activity-3",
    timestamp: new Date(Date.now() - 3 * 60 * 1000).toISOString(),
    type: "status",
    source: "agent-3",
    content: "Started processing new data batch",
  },
  {
    id: "activity-4",
    timestamp: new Date(Date.now() - 2 * 60 * 1000).toISOString(),
    type: "message",
    source: "agent-2",
    target: "agent-4",
    content: "Forwarding processed results for analysis",
  },
  {
    id: "activity-5",
    timestamp: new Date(Date.now() - 1 * 60 * 1000).toISOString(),
    type: "status",
    source: "agent-4",
    content: "Analysis complete, generating report",
  },
];

const NetworkPage: React.FC = () => {
  // Get agents from app context
  const { state, refreshAgents } = useApp();
  const { agents } = state;
  
  // State for connections and activities
  const [connections, setConnections] = useState<Connection[]>(mockConnections);
  const [activities, setActivities] = useState<ActivityItem[]>(mockActivities);
  
  // State for loading and error
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  
  // State for selected agent
  const [selectedAgent, setSelectedAgent] = useState<string | null>(null);
  
  // State for filters
  const [agentTypeFilter, setAgentTypeFilter] = useState<string>("all");
  const [agentStatusFilter, setAgentStatusFilter] = useState<string>("all");
  const [connectionStrengthFilter, setConnectionStrengthFilter] = useState<number>(0);
  const [showInactiveConnections, setShowInactiveConnections] = useState(true);
  const [searchQuery, setSearchQuery] = useState("");
  
  // Apply filters to agents
  const filteredAgents = agents.filter((agent) => {
    // Apply type filter
    if (agentTypeFilter !== "all" && agent.type !== agentTypeFilter) {
      return false;
    }
    
    // Apply status filter
    if (agentStatusFilter !== "all" && agent.status !== agentStatusFilter) {
      return false;
    }
    
    // Apply search query
    if (
      searchQuery &&
      !agent.name.toLowerCase().includes(searchQuery.toLowerCase()) &&
      !agent.id.toLowerCase().includes(searchQuery.toLowerCase())
    ) {
      return false;
    }
    
    return true;
  });
  
  // Apply filters to connections
  const filteredConnections = connections.filter((connection) => {
    // Filter by connection strength
    if (connection.strength < connectionStrengthFilter) {
      return false;
    }
    
    // Filter inactive connections if needed
    if (
      !showInactiveConnections &&
      Date.now() - connection.lastActive > 3600000 // 1 hour
    ) {
      return false;
    }
    
    // Only show connections between filtered agents
    const sourceAgent = filteredAgents.find((a) => a.id === connection.source);
    const targetAgent = filteredAgents.find((a) => a.id === connection.target);
    if (!sourceAgent || !targetAgent) {
      return false;
    }
    
    return true;
  });
  
  // Get unique agent types for filter dropdown
  const agentTypes = ["all", ...new Set(agents.map((agent) => agent.type))];
  
  // Get unique agent statuses for filter dropdown
  const agentStatuses = ["all", ...new Set(agents.map((agent) => agent.status))];
  
  // Handle agent selection
  const handleAgentSelect = (agentId: string | null) => {
    setSelectedAgent(agentId);
  };
  
  // Handle refresh
  const handleRefresh = async () => {
    setLoading(true);
    try {
      await refreshAgents();
      setError(null);
    } catch (err) {
      console.error("Error refreshing data:", err);
      setError("Failed to refresh data. Please try again.");
    } finally {
      setLoading(false);
    }
  };
  
  // Simulate real-time updates
  useEffect(() => {
    const interval = setInterval(() => {
      // Update a random connection's lastActive time
      if (connections.length > 0) {
        const randomIndex = Math.floor(Math.random() * connections.length);
        const updatedConnections = [...connections];
        updatedConnections[randomIndex] = {
          ...updatedConnections[randomIndex],
          lastActive: Date.now(),
        };
        setConnections(updatedConnections);
        
        // Add a new activity
        const newActivity: ActivityItem = {
          id: `activity-${Date.now()}`,
          timestamp: new Date().toISOString(),
          type: Math.random() > 0.5 ? "message" : "status",
          source: updatedConnections[randomIndex].source,
          target: updatedConnections[randomIndex].target,
          content: `Activity at ${new Date().toLocaleTimeString()}`,
        };
        setActivities((prev) => [newActivity, ...prev.slice(0, 19)]);
      }
    }, 10000); // Every 10 seconds
    
    return () => clearInterval(interval);
  }, [connections]);
  
  return (
    <div className="h-full w-full flex flex-col p-4 md:p-6 bg-gradient-to-br from-background to-background/80">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-primary">
          Network Visualization
        </h1>
        <Button
          variant="outline"
          size="sm"
          onClick={handleRefresh}
          disabled={loading}
        >
          <RefreshCw className="mr-2 h-4 w-4" />
          Refresh
        </Button>
      </div>
      
      <div className="grid grid-cols-1 lg:grid-cols-4 gap-6 h-[calc(100vh-12rem)]">
        {/* Left sidebar with filters */}
        <Card className="lg:col-span-1 overflow-auto border border-primary/10 bg-card/80 backdrop-blur-sm shadow-neoglass">
          <CardHeader>
            <CardTitle className="flex items-center">
              <Filter className="mr-2 h-5 w-5" />
              Filters
            </CardTitle>
          </CardHeader>
          <CardContent className="space-y-4">
            <div className="space-y-2">
              <Label htmlFor="search">Search Agents</Label>
              <Input
                id="search"
                placeholder="Search by name or ID..."
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
              />
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="agent-type">Agent Type</Label>
              <Select
                value={agentTypeFilter}
                onValueChange={setAgentTypeFilter}
              >
                <SelectTrigger id="agent-type">
                  <SelectValue placeholder="Select type" />
                </SelectTrigger>
                <SelectContent>
                  {agentTypes.map((type) => (
                    <SelectItem key={type} value={type}>
                      {type === "all"
                        ? "All Types"
                        : type.charAt(0).toUpperCase() + type.slice(1)}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            
            <div className="space-y-2">
              <Label htmlFor="agent-status">Agent Status</Label>
              <Select
                value={agentStatusFilter}
                onValueChange={setAgentStatusFilter}
              >
                <SelectTrigger id="agent-status">
                  <SelectValue placeholder="Select status" />
                </SelectTrigger>
                <SelectContent>
                  {agentStatuses.map((status) => (
                    <SelectItem key={status} value={status}>
                      {status === "all"
                        ? "All Statuses"
                        : status.charAt(0).toUpperCase() + status.slice(1)}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            
            <div className="my-4 gradient-separator" />
            
            <div className="space-y-2">
              <Label htmlFor="connection-strength">
                Connection Strength: {connectionStrengthFilter.toFixed(1)}
              </Label>
              <input
                id="connection-strength"
                type="range"
                min={0}
                max={1}
                step={0.1}
                value={connectionStrengthFilter}
                onChange={(e) =>
                  setConnectionStrengthFilter(parseFloat(e.target.value))
                }
                className="w-full h-2 appearance-none rounded-full bg-secondary/50 accent-primary"
                title="Connection strength filter"
              />
            </div>
            
            <div className="flex items-center space-x-2">
              <input
                id="show-inactive"
                type="checkbox"
                checked={showInactiveConnections}
                onChange={(e) => setShowInactiveConnections(e.target.checked)}
                className="h-4 w-4 rounded border-primary/30 bg-secondary/50 text-primary focus:ring-primary/30"
                title="Show inactive connections"
              />
              <Label htmlFor="show-inactive">Show Inactive Connections</Label>
            </div>
            
            <div className="my-4 gradient-separator" />
            
            <div className="pt-2">
              <h3 className="text-sm font-medium mb-2">Network Stats</h3>
              <div className="grid grid-cols-2 gap-2 text-sm">
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Agents:</span>
                  <span>{filteredAgents.length}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Connections:</span>
                  <span>{filteredConnections.length}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Online:</span>
                  <span>
                    {agents.filter((a) => a.status === "online").length}
                  </span>
                </div>
                <div className="flex justify-between">
                  <span className="text-muted-foreground">Offline:</span>
                  <span>
                    {agents.filter((a) => a.status === "offline").length}
                  </span>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
        
        {/* Main visualization area */}
        <div className="lg:col-span-2 h-full">
          <Card className="h-full border border-primary/10 bg-card/80 backdrop-blur-sm shadow-neoglass">
            <CardContent className="p-0 h-full">
              {(() => {
                if (loading) {
                  return (
                    <div className="flex items-center justify-center h-full">
                      <div className="text-center">
                        <div className="relative mx-auto mb-6 h-16 w-16">
                          <div className="absolute inset-0 rounded-full border-2 border-primary/20"></div>
                          <div className="absolute inset-0 rounded-full border-t-2 border-primary animate-spin"></div>
                          <div className="absolute inset-2 rounded-full border-r-2 border-primary/40 animate-spin animation-delay-150"></div>
                          <div className="absolute inset-4 rounded-full border-b-2 border-primary/60 animate-spin animation-delay-300"></div>
                        </div>
                        <p className="text-lg font-medium text-primary/80">
                          Loading network data...
                        </p>
                        <p className="text-sm text-muted-foreground mt-2">
                          Preparing visualization
                        </p>
                      </div>
                    </div>
                  );
                } else if (error) {
                  return (
                    <div className="flex items-center justify-center h-full">
                      <div className="text-center">
                        <div className="relative mx-auto mb-6 h-16 w-16 text-destructive/80">
                          <div className="absolute inset-0 flex items-center justify-center">
                            <Info className="h-10 w-10 animate-pulse" />
                          </div>
                          <div className="absolute inset-0 rounded-full border-2 border-destructive/20"></div>
                        </div>
                        <p className="text-lg font-medium text-destructive">
                          {error}
                        </p>
                        <p className="text-sm text-muted-foreground mt-2 mb-4">
                          Please try again or check your connection
                        </p>
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={handleRefresh}
                          className="bg-card/50 hover:bg-card border-destructive/20 hover:border-destructive/40 text-destructive hover:text-destructive"
                        >
                          <RefreshCw className="mr-2 h-4 w-4" />
                          Try Again
                        </Button>
                      </div>
                    </div>
                  );
                } else {
                  return (
                    <EnhancedNetworkGraph
                      agents={filteredAgents}
                      connections={filteredConnections}
                      height="100%"
                      showControls={true}
                      showGizmo={true}
                      onAgentSelect={handleAgentSelect}
                    />
                  );
                }
              })()}
            </CardContent>
          </Card>
        </div>
        
        {/* Right sidebar with details and activity */}
        <Card className="lg:col-span-1 overflow-auto border border-primary/10 bg-card/80 backdrop-blur-sm shadow-neoglass">
          <Tabs defaultValue="details">
            <CardHeader className="pb-0">
              <TabsList className="grid w-full grid-cols-2">
                <TabsTrigger value="details" className="flex items-center">
                  <Users className="mr-2 h-4 w-4" />
                  Details
                </TabsTrigger>
                <TabsTrigger value="activity" className="flex items-center">
                  <Activity className="mr-2 h-4 w-4" />
                  Activity
                </TabsTrigger>
              </TabsList>
            </CardHeader>
            
            <CardContent>
              <TabsContent value="details" className="space-y-4 mt-4">
                {selectedAgent ? (
                  <AgentDetails
                    agent={agents.find((a) => a.id === selectedAgent)}
                    connections={connections.filter(
                      (c) =>
                        c.source === selectedAgent ||
                        c.target === selectedAgent
                    )}
                    agents={agents}
                  />
                ) : (
                  <div className="flex flex-col items-center justify-center h-[300px] text-center">
                    <Network className="h-12 w-12 text-muted-foreground mb-4" />
                    <h3 className="text-lg font-medium">No Agent Selected</h3>
                    <p className="text-sm text-muted-foreground mt-2">
                      Click on an agent in the visualization to view details
                    </p>
                  </div>
                )}
              </TabsContent>
              
              <TabsContent value="activity" className="mt-4">
                <ActivityFeed activities={activities} agents={agents} />
              </TabsContent>
            </CardContent>
          </Tabs>
        </Card>
      </div>
    </div>
  );
};

export default NetworkPage;
```

## Agent Details Component

Create a component to display agent details:

```typescript
// src/components/network/AgentDetails.tsx
import React from "react";
import { Badge } from "@/components/ui/badge";
import { Agent } from "@/contexts/AppContext";
import { MessageSquare, Zap } from "lucide-react";

interface Connection {
  source: string;
  target: string;
  strength: number;
  lastActive: number;
  type?: string;
}

interface AgentDetailsProps {
  agent?: Agent;
  connections: Connection[];
  agents: Agent[];
}

const AgentDetails: React.FC<AgentDetailsProps> = ({
  agent,
  connections,
  agents,
}) => {
  if (!agent) {
    return <div>No agent selected</div>;
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-3">
        <div className="flex-shrink-0">
          <div className="relative h-10 w-10 rounded-full flex items-center justify-center">
            <div className="absolute inset-0 rounded-full bg-primary/10"></div>
            <div className="text-lg font-bold text-primary">
              {agent.name.charAt(0)}
            </div>
          </div>
        </div>
        <div>
          <h3 className="text-lg font-semibold gradient-text">
            {agent.name}
          </h3>
          <p className="text-sm text-muted-foreground">
            {agent.id}
          </p>
        </div>
      </div>

      <div className="grid grid-cols-2 gap-2">
        <div>
          <p className="text-sm text-muted-foreground">Type</p>
          <p className="font-medium">
            {agent.type}
          </p>
        </div>
        <div>
          <p className="text-sm text-muted-foreground">
            Status
          </p>
          <Badge
            variant={
              agent.status === "online"
                ? "default"
                : "secondary"
            }
          >
            {agent.status}
          </Badge>
        </div>
      </div>

      {agent.currentTask && (
        <div>
          <p className="text-sm text-muted-foreground">
            Current Task
          </p>
          <p>
            {agent.currentTask}
          </p>
        </div>
      )}

      <div>
        <p className="text-sm text-muted-foreground">
          Last Active
        </p>
        <p>
          {agent.lastSeen
            ? new Date(agent.lastSeen).toLocaleString()
            : "Unknown"}
        </p>
      </div>

      <div className="my-4 gradient-separator" />

      <div>
        <h4 className="text-sm font-medium mb-2">
          Connections
        </h4>
        <div className="space-y-2">
          {connections.length > 0 ? (
            connections.map((connection) => {
              const otherAgentId =
                connection.source === agent.id
                  ? connection.target
                  : connection.source;
              const otherAgent = agents.find(
                (a) => a.id === otherAgentId
              );
              const direction =
                connection.source === agent.id
                  ? "outgoing"
                  : "incoming";

              return (
                <div
                  key={`${connection.source}-${connection.target}-${connection.lastActive}`}
                  className="flex items-center justify-between p-2 rounded-md bg-secondary/20"
                >
                  <div className="flex items-center">
                    <div className="mr-2">
                      {direction === "outgoing" ? (
                        <Zap className="h-4 w-4 text-primary" />
                      ) : (
                        <MessageSquare className="h-4 w-4 text-primary" />
                      )}
                    </div>
                    <div>
                      <p className="text-sm font-medium">
                        {otherAgent?.name || otherAgentId}
                      </p>
                      <p className="text-xs text-muted-foreground">
                        Strength: {connection.strength.toFixed(1)}
                      </p>
                    </div>
                  </div>
                  <Badge variant="outline" className="text-xs">
                    {new Date(connection.lastActive).toLocaleTimeString()}
                  </Badge>
                </div>
              );
            })
          ) : (
            <p className="text-sm text-muted-foreground">
              No connections found
            </p>
          )}
        </div>
      </div>
    </div>
  );
};

export default AgentDetails;
```

## Activity Feed Component

Create a component to display the activity feed:

```typescript
// src/components/network/ActivityFeed.tsx
import React from "react";
import { Badge } from "@/components/ui/badge";
import { Agent } from "@/contexts/AppContext";

interface ActivityItem {
  id: string;
  timestamp: string;
  type: string;
  source: string;
  target?: string;
  content: string;
}

interface ActivityFeedProps {
  activities: ActivityItem[];
  agents: Agent[];
}

const ActivityFeed: React.FC<ActivityFeedProps> = ({ activities, agents }) => {
  return (
    <div className="space-y-4">
      <h3 className="text-sm font-medium">Recent Activity</h3>

      {activities.length > 0 ? (
        <div className="space-y-2">
          {activities.map((activity) => {
            const sourceAgent = agents.find(
              (a) => a.id === activity.source
            );
            const targetAgent = activity.target
              ? agents.find((a) => a.id === activity.target)
              : null;

            return (
              <div
                key={activity.id}
                className="p-2 rounded-md bg-secondary/20"
              >
                <div className="flex justify-between items-start mb-1">
                  <span className="text-xs font-medium">
                    {sourceAgent?.name || activity.source}
                    {targetAgent && ` → ${targetAgent.name}`}
                  </span>
                  <Badge variant="outline" className="text-xs">
                    {new Date(activity.timestamp).toLocaleTimeString()}
                  </Badge>
                </div>
                <p className="text-sm">{activity.content}</p>
              </div>
            );
          })}
        </div>
      ) : (
        <p className="text-sm text-muted-foreground">
          No recent activity
        </p>
      )}
    </div>
  );
};

export default ActivityFeed;
```

## CSS Styles

Create CSS styles for the network visualization:

```css
/* src/app/network/network.css */
/* Network visualization specific styles */

/* Agent label styles */
.agent-label {
  color: #f6f5f5;
  font-size: 12px;
  padding: 2px 5px;
  background-color: rgba(31, 32, 34, 0.7);
  border-radius: 4px;
  pointer-events: none;
  white-space: nowrap;
  text-align: center;
}

/* Gradient separator */
.gradient-separator {
  height: 1px;
  width: 100%;
  background: linear-gradient(to right, transparent, rgba(126, 186, 181, 0.2), transparent);
}

/* Network card */
.network-card {
  transition: all 0.3s ease;
}

.network-card:hover {
  box-shadow: 0 8px 32px 0 rgba(126, 186, 181, 0.1);
}

/* Animation delays */
.animation-delay-150 {
  animation-delay: 150ms;
}

.animation-delay-300 {
  animation-delay: 300ms;
}

/* Network visualization page */
.network-visualization-page {
  background-image: 
    radial-gradient(circle at 10% 20%, rgba(126, 186, 181, 0.03) 0%, transparent 20%),
    radial-gradient(circle at 90% 80%, rgba(126, 186, 181, 0.03) 0%, transparent 20%);
}

/* Tooltip styles */
.tooltip {
  position: absolute;
  transform: translate(-50%, -100%);
  background-color: rgba(31, 32, 34, 0.9);
  color: #f6f5f5;
  padding: 4px 8px;
  border-radius: 4px;
  font-size: 12px;
  pointer-events: none;
  opacity: 0;
  transition: opacity 0.2s ease;
  z-index: 1000;
}

.tooltip-visible {
  opacity: 1;
}

/* Particle styles */
.particle {
  position: absolute;
  border-radius: 50%;
  background-color: rgba(126, 186, 181, 0.1);
  pointer-events: none;
  animation: float 10s infinite ease-in-out;
}

@keyframes float {
  0%, 100% { transform: translateY(0) translateX(0); }
  25% { transform: translateY(-30px) translateX(15px); }
  50% { transform: translateY(-15px) translateX(-15px); }
  75% { transform: translateY(-30px) translateX(5px); }
}
```

These implementations provide a comprehensive network visualization page with filtering, detailed agent information, and an activity feed. The components are designed to work together to create an interactive and visually appealing experience for monitoring agent networks.
