# Jarvis SWE Agent Multi-Agent Coordination - Technical Implementation (Part 2)

## 4. Role Manager Implementation

### 4.1 Role Manager Service

```typescript
class RoleManager {
  private db: MongoDB.Db;
  private collection: MongoDB.Collection;
  private agentRegistry: AgentRegistry;
  
  constructor(config: RoleManagerConfig) {
    const client = new MongoDB.MongoClient(config.mongoUri);
    this.db = client.db(config.database || 'jarvis');
    this.collection = this.db.collection(config.collection || 'roles');
    this.agentRegistry = config.agentRegistry;
    
    // Create indexes
    this.collection.createIndex({ id: 1 }, { unique: true });
    this.collection.createIndex({ name: 1 });
  }
  
  async createRole(role: Partial<Role>): Promise<string> {
    const id = role.id || uuidv4();
    
    const newRole: Role = {
      id,
      name: role.name || 'Unnamed Role',
      description: role.description || '',
      capabilities: role.capabilities || {
        required: [],
        preferred: []
      },
      responsibilities: role.responsibilities || [],
      authority: role.authority || {
        decisions: [],
        resources: []
      },
      relationships: role.relationships || {
        reports_to: [],
        manages: [],
        collaborates_with: []
      }
    };
    
    // Store in MongoDB
    await this.collection.insertOne(newRole);
    
    return id;
  }
  
  async getRole(id: string): Promise<Role | null> {
    const role = await this.collection.findOne({ id });
    return role as Role;
  }
  
  async getRoleByName(name: string): Promise<Role | null> {
    const role = await this.collection.findOne({ name });
    return role as Role;
  }
  
  async updateRole(id: string, updates: Partial<Role>): Promise<Role | null> {
    const updateDoc: any = { $set: {} };
    
    if (updates.name !== undefined) {
      updateDoc.$set.name = updates.name;
    }
    
    if (updates.description !== undefined) {
      updateDoc.$set.description = updates.description;
    }
    
    if (updates.capabilities !== undefined) {
      updateDoc.$set.capabilities = updates.capabilities;
    }
    
    if (updates.responsibilities !== undefined) {
      updateDoc.$set.responsibilities = updates.responsibilities;
    }
    
    if (updates.authority !== undefined) {
      updateDoc.$set.authority = updates.authority;
    }
    
    if (updates.relationships !== undefined) {
      updateDoc.$set.relationships = updates.relationships;
    }
    
    const result = await this.collection.updateOne({ id }, updateDoc);
    
    if (result.modifiedCount === 0) return null;
    
    return this.getRole(id);
  }
  
  async deleteRole(id: string): Promise<boolean> {
    // Check if any agents have this role
    const agents = await this.agentRegistry.findAgents({ roles: id });
    
    if (agents.length > 0) {
      // Remove role from agents
      for (const agent of agents) {
        const updatedRoles = agent.roles.filter(r => r !== id);
        await this.agentRegistry.updateAgent(agent.id, { roles: updatedRoles });
      }
    }
    
    // Remove role
    const result = await this.collection.deleteOne({ id });
    
    return result.deletedCount > 0;
  }
  
  async listRoles(): Promise<Role[]> {
    const roles = await this.collection.find().toArray();
    return roles as Role[];
  }
  
  async assignRole(agentId: string, roleId: string): Promise<boolean> {
    // Get agent and role
    const agent = await this.agentRegistry.getAgent(agentId);
    const role = await this.getRole(roleId);
    
    if (!agent || !role) return false;
    
    // Check if agent has required capabilities
    const hasRequiredCapabilities = role.capabilities.required.every(
      capability => agent.capabilities.includes(capability)
    );
    
    if (!hasRequiredCapabilities) {
      throw new Error(`Agent ${agentId} does not have all required capabilities for role ${roleId}`);
    }
    
    // Add role to agent
    const updatedRoles = [...agent.roles];
    if (!updatedRoles.includes(roleId)) {
      updatedRoles.push(roleId);
    }
    
    // Update agent
    const updated = await this.agentRegistry.updateAgent(agentId, { roles: updatedRoles });
    
    return !!updated;
  }
  
  async unassignRole(agentId: string, roleId: string): Promise<boolean> {
    // Get agent
    const agent = await this.agentRegistry.getAgent(agentId);
    
    if (!agent) return false;
    
    // Remove role from agent
    const updatedRoles = agent.roles.filter(r => r !== roleId);
    
    // Update agent
    const updated = await this.agentRegistry.updateAgent(agentId, { roles: updatedRoles });
    
    return !!updated;
  }
  
  async findAgentsWithRole(roleId: string): Promise<Agent[]> {
    return this.agentRegistry.findAgents({ roles: roleId });
  }
  
  async findSuitableAgentsForRole(roleId: string): Promise<Agent[]> {
    const role = await this.getRole(roleId);
    
    if (!role) return [];
    
    // Find agents with required capabilities
    return this.agentRegistry.findAgentsByCapabilities(role.capabilities.required, true);
  }
  
  async createDefaultRoles(): Promise<void> {
    // Create standard roles if they don't exist
    const defaultRoles = [
      {
        name: 'team_lead',
        description: 'Coordinates team activities and makes strategic decisions',
        capabilities: {
          required: ['coordination', 'decision_making'],
          preferred: ['planning']
        },
        responsibilities: [
          'Coordinate team activities',
          'Make strategic decisions',
          'Assign tasks to team members',
          'Monitor overall progress',
          'Resolve conflicts'
        ],
        authority: {
          decisions: ['task_assignment', 'priority_setting', 'conflict_resolution'],
          resources: ['team_resources']
        },
        relationships: {
          reports_to: [],
          manages: ['developer', 'researcher', 'planner', 'reviewer'],
          collaborates_with: []
        }
      },
      {
        name: 'developer',
        description: 'Implements code and features',
        capabilities: {
          required: ['code_generation'],
          preferred: ['debugging', 'refactoring']
        },
        responsibilities: [
          'Implement features',
          'Write clean, maintainable code',
          'Fix bugs',
          'Write tests',
          'Document code'
        ],
        authority: {
          decisions: ['implementation_details'],
          resources: ['code_repository']
        },
        relationships: {
          reports_to: ['team_lead'],
          manages: [],
          collaborates_with: ['reviewer', 'planner', 'researcher']
        }
      },
      {
        name: 'researcher',
        description: 'Gathers and analyzes information',
        capabilities: {
          required: ['information_gathering'],
          preferred: ['analysis', 'summarization']
        },
        responsibilities: [
          'Gather information',
          'Analyze data',
          'Provide insights',
          'Create reports',
          'Answer questions'
        ],
        authority: {
          decisions: ['information_sources', 'research_methods'],
          resources: ['information_access']
        },
        relationships: {
          reports_to: ['team_lead'],
          manages: [],
          collaborates_with: ['developer', 'planner']
        }
      },
      {
        name: 'planner',
        description: 'Plans and organizes tasks',
        capabilities: {
          required: ['task_decomposition'],
          preferred: ['estimation', 'dependency_analysis']
        },
        responsibilities: [
          'Break down objectives into tasks',
          'Estimate effort',
          'Define dependencies',
          'Create schedules',
          'Track progress'
        ],
        authority: {
          decisions: ['task_breakdown', 'scheduling'],
          resources: ['planning_tools']
        },
        relationships: {
          reports_to: ['team_lead'],
          manages: [],
          collaborates_with: ['developer', 'researcher', 'reviewer']
        }
      },
      {
        name: 'reviewer',
        description: 'Reviews and ensures quality',
        capabilities: {
          required: ['code_review'],
          preferred: ['testing', 'quality_assurance']
        },
        responsibilities: [
          'Review code',
          'Ensure quality standards',
          'Identify issues',
          'Provide feedback',
          'Approve deliverables'
        ],
        authority: {
          decisions: ['quality_standards', 'approval'],
          resources: ['testing_tools']
        },
        relationships: {
          reports_to: ['team_lead'],
          manages: [],
          collaborates_with: ['developer', 'planner']
        }
      }
    ];
    
    for (const roleData of defaultRoles) {
      const existingRole = await this.getRoleByName(roleData.name);
      
      if (!existingRole) {
        await this.createRole(roleData);
      }
    }
  }
}
```

### 4.2 Role-Based Prompting

```typescript
class RolePromptGenerator {
  private roleManager: RoleManager;
  
  constructor(config: RolePromptGeneratorConfig) {
    this.roleManager = config.roleManager;
  }
  
  async generateRolePrompt(agentId: string, roleId: string): Promise<string> {
    const role = await this.roleManager.getRole(roleId);
    
    if (!role) {
      throw new Error(`Role ${roleId} not found`);
    }
    
    // Generate role-specific prompt
    return `
      # Role: ${role.name}
      
      ## Description
      ${role.description}
      
      ## Responsibilities
      ${role.responsibilities.map(r => `- ${r}`).join('\n')}
      
      ## Authority
      You can make decisions about: ${role.authority.decisions.join(', ')}
      You have access to: ${role.authority.resources.join(', ')}
      
      ## Relationships
      ${role.relationships.reports_to.length > 0 ? `You report to: ${role.relationships.reports_to.join(', ')}` : 'You do not report to anyone.'}
      ${role.relationships.manages.length > 0 ? `You manage: ${role.relationships.manages.join(', ')}` : 'You do not manage anyone.'}
      ${role.relationships.collaborates_with.length > 0 ? `You collaborate with: ${role.relationships.collaborates_with.join(', ')}` : 'You do not have specific collaboration relationships.'}
      
      ## Guidelines
      - Always act within your responsibilities and authority
      - Consult with those you report to for decisions outside your authority
      - Coordinate with your collaborators when working on shared tasks
      - Provide clear direction to those you manage
      - Focus on your area of expertise
    `;
  }
  
  async enhancePromptWithRole(basePrompt: string, agentId: string): Promise<string> {
    const agent = await this.roleManager.agentRegistry.getAgent(agentId);
    
    if (!agent || agent.roles.length === 0) {
      return basePrompt;
    }
    
    // Get primary role
    const primaryRoleId = agent.roles[0];
    const rolePrompt = await this.generateRolePrompt(agentId, primaryRoleId);
    
    // Combine with base prompt
    return `
      ${rolePrompt}
      
      ${basePrompt}
    `;
  }
}
```

## 5. Team Manager Implementation

### 5.1 Team Manager Service

```typescript
class TeamManager {
  private db: MongoDB.Db;
  private collection: MongoDB.Collection;
  private agentRegistry: AgentRegistry;
  private roleManager: RoleManager;
  
  constructor(config: TeamManagerConfig) {
    const client = new MongoDB.MongoClient(config.mongoUri);
    this.db = client.db(config.database || 'jarvis');
    this.collection = this.db.collection(config.collection || 'teams');
    this.agentRegistry = config.agentRegistry;
    this.roleManager = config.roleManager;
    
    // Create indexes
    this.collection.createIndex({ id: 1 }, { unique: true });
    this.collection.createIndex({ name: 1 });
    this.collection.createIndex({ status: 1 });
    this.collection.createIndex({ 'members.agentId': 1 });
  }
  
  async createTeam(team: Partial<Team>): Promise<string> {
    const id = team.id || uuidv4();
    
    const newTeam: Team = {
      id,
      name: team.name || 'Unnamed Team',
      objective: team.objective || '',
      members: team.members || [],
      workflows: team.workflows || [],
      status: team.status || 'forming',
      performance: team.performance || {
        objectiveCompletion: 0,
        collaboration: 0,
        efficiency: 0
      },
      metadata: {
        created: new Date(),
        lastActive: new Date(),
        type: team.metadata?.type || 'fixed'
      }
    };
    
    // Store in MongoDB
    await this.collection.insertOne(newTeam);
    
    // Update agent team memberships
    for (const member of newTeam.members) {
      const agent = await this.agentRegistry.getAgent(member.agentId);
      
      if (agent) {
        const updatedTeams = [...agent.teams];
        if (!updatedTeams.includes(id)) {
          updatedTeams.push(id);
        }
        
        await this.agentRegistry.updateAgent(member.agentId, { teams: updatedTeams });
      }
    }
    
    return id;
  }
  
  async getTeam(id: string): Promise<Team | null> {
    const team = await this.collection.findOne({ id });
    return team as Team;
  }
  
  async updateTeam(id: string, updates: Partial<Team>): Promise<Team | null> {
    const updateDoc: any = { $set: {} };
    
    if (updates.name !== undefined) {
      updateDoc.$set.name = updates.name;
    }
    
    if (updates.objective !== undefined) {
      updateDoc.$set.objective = updates.objective;
    }
    
    if (updates.status !== undefined) {
      updateDoc.$set.status = updates.status;
    }
    
    if (updates.performance !== undefined) {
      updateDoc.$set.performance = updates.performance;
    }
    
    if (updates.workflows !== undefined) {
      updateDoc.$set.workflows = updates.workflows;
    }
    
    // Always update lastActive
    updateDoc.$set['metadata.lastActive'] = new Date();
    
    // Handle member updates separately to maintain agent team memberships
    if (updates.members !== undefined) {
      await this.updateTeamMembers(id, updates.members);
    }
    
    const result = await this.collection.updateOne({ id }, updateDoc);
    
    if (result.modifiedCount === 0 && !updates.members) return null;
    
    return this.getTeam(id);
  }
  
  private async updateTeamMembers(teamId: string, newMembers: Team['members']): Promise<void> {
    const team = await this.getTeam(teamId);
    
    if (!team) return;
    
    // Find members to remove
    const currentMemberIds = team.members.map(m => m.agentId);
    const newMemberIds = newMembers.map(m => m.agentId);
    
    const membersToRemove = currentMemberIds.filter(id => !newMemberIds.includes(id));
    const membersToAdd = newMemberIds.filter(id => !currentMemberIds.includes(id));
    
    // Remove team from agents no longer in the team
    for (const agentId of membersToRemove) {
      const agent = await this.agentRegistry.getAgent(agentId);
      
      if (agent) {
        const updatedTeams = agent.teams.filter(t => t !== teamId);
        await this.agentRegistry.updateAgent(agentId, { teams: updatedTeams });
      }
    }
    
    // Add team to new members
    for (const agentId of membersToAdd) {
      const agent = await this.agentRegistry.getAgent(agentId);
      
      if (agent) {
        const updatedTeams = [...agent.teams];
        if (!updatedTeams.includes(teamId)) {
          updatedTeams.push(teamId);
        }
        
        await this.agentRegistry.updateAgent(agentId, { teams: updatedTeams });
      }
    }
    
    // Update team members
    await this.collection.updateOne(
      { id: teamId },
      { $set: { members: newMembers } }
    );
  }
  
  async deleteTeam(id: string): Promise<boolean> {
    const team = await this.getTeam(id);
    
    if (!team) return false;
    
    // Remove team from all members
    for (const member of team.members) {
      const agent = await this.agentRegistry.getAgent(member.agentId);
      
      if (agent) {
        const updatedTeams = agent.teams.filter(t => t !== id);
        await this.agentRegistry.updateAgent(member.agentId, { teams: updatedTeams });
      }
    }
    
    // Remove team
    const result = await this.collection.deleteOne({ id });
    
    return result.deletedCount > 0;
  }
  
  async listTeams(filter: any = {}): Promise<Team[]> {
    const teams = await this.collection.find(filter).toArray();
    return teams as Team[];
  }
  
  async addMember(teamId: string, agentId: string, roleId: string): Promise<boolean> {
    const team = await this.getTeam(teamId);
    const agent = await this.agentRegistry.getAgent(agentId);
    const role = await this.roleManager.getRole(roleId);
    
    if (!team || !agent || !role) return false;
    
    // Check if agent is already a member
    if (team.members.some(m => m.agentId === agentId)) {
      // Update role if needed
      const updatedMembers = team.members.map(m => 
        m.agentId === agentId ? { ...m, roleId } : m
      );
      
      await this.collection.updateOne(
        { id: teamId },
        { $set: { members: updatedMembers } }
      );
    } else {
      // Add new member
      const newMember = {
        agentId,
        roleId,
        joinedAt: new Date()
      };
      
      await this.collection.updateOne(
        { id: teamId },
        { $push: { members: newMember } }
      );
      
      // Add team to agent
      const updatedTeams = [...agent.teams];
      if (!updatedTeams.includes(teamId)) {
        updatedTeams.push(teamId);
      }
      
      await this.agentRegistry.updateAgent(agentId, { teams: updatedTeams });
    }
    
    return true;
  }
  
  async removeMember(teamId: string, agentId: string): Promise<boolean> {
    const team = await this.getTeam(teamId);
    const agent = await this.agentRegistry.getAgent(agentId);
    
    if (!team || !agent) return false;
    
    // Remove member from team
    const updatedMembers = team.members.filter(m => m.agentId !== agentId);
    
    await this.collection.updateOne(
      { id: teamId },
      { $set: { members: updatedMembers } }
    );
    
    // Remove team from agent
    const updatedTeams = agent.teams.filter(t => t !== teamId);
    await this.agentRegistry.updateAgent(agentId, { teams: updatedTeams });
    
    return true;
  }
  
  async getTeamMembers(teamId: string): Promise<Agent[]> {
    const team = await this.getTeam(teamId);
    
    if (!team) return [];
    
    const members: Agent[] = [];
    
    for (const member of team.members) {
      const agent = await this.agentRegistry.getAgent(member.agentId);
      
      if (agent) {
        members.push(agent);
      }
    }
    
    return members;
  }
  
  async getAgentTeams(agentId: string): Promise<Team[]> {
    const agent = await this.agentRegistry.getAgent(agentId);
    
    if (!agent) return [];
    
    const teams: Team[] = [];
    
    for (const teamId of agent.teams) {
      const team = await this.getTeam(teamId);
      
      if (team) {
        teams.push(team);
      }
    }
    
    return teams;
  }
  
  async createTeamFromTemplate(template: TeamTemplate, objective: string): Promise<string> {
    // Create team with template configuration
    const teamId = await this.createTeam({
      name: template.name,
      objective,
      status: 'forming',
      metadata: {
        type: template.type
      }
    });
    
    // Find and assign agents for each role
    for (const roleConfig of template.roles) {
      const role = await this.roleManager.getRoleByName(roleConfig.role);
      
      if (!role) continue;
      
      // Find suitable agents
      const agents = await this.roleManager.findSuitableAgentsForRole(role.id);
      
      // Filter to available agents
      const availableAgents = agents.filter(a => a.status === 'available');
      
      if (availableAgents.length > 0) {
        // Select agent (could implement more sophisticated selection)
        const selectedAgent = availableAgents[0];
        
        // Add to team
        await this.addMember(teamId, selectedAgent.id, role.id);
        
        // Assign role if not already assigned
        if (!selectedAgent.roles.includes(role.id)) {
          await this.roleManager.assignRole(selectedAgent.id, role.id);
        }
      }
    }
    
    return teamId;
  }
}
```
