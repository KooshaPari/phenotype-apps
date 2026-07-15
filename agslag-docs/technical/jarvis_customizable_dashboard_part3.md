# Jarvis SWE Agent Customizable Dashboard - Technical Implementation (Part 3)

## 6. Dashboard Backend Implementation

### 6.1 Dashboard Service

The Dashboard Service handles dashboard configuration storage and retrieval.

```typescript
import axios from 'axios';
import { API_BASE_URL } from '../config';

interface Dashboard {
  id: string;
  userId: string;
  name: string;
  layouts: any;
  widgets: any[];
  isDefault?: boolean;
  createdAt?: string;
  updatedAt?: string;
}

class DashboardService {
  /**
   * Get all dashboards for a user
   */
  public async getDashboards(userId: string): Promise<Dashboard[]> {
    try {
      const response = await axios.get(`${API_BASE_URL}/dashboards`, {
        params: { userId }
      });
      
      return response.data;
    } catch (error) {
      console.error('Failed to get dashboards:', error);
      throw error;
    }
  }
  
  /**
   * Get a dashboard by ID
   */
  public async getDashboard(dashboardId: string): Promise<Dashboard> {
    try {
      const response = await axios.get(`${API_BASE_URL}/dashboards/${dashboardId}`);
      
      return response.data;
    } catch (error) {
      console.error(`Failed to get dashboard ${dashboardId}:`, error);
      throw error;
    }
  }
  
  /**
   * Create a new dashboard
   */
  public async createDashboard(dashboard: Partial<Dashboard>): Promise<Dashboard> {
    try {
      const response = await axios.post(`${API_BASE_URL}/dashboards`, dashboard);
      
      return response.data;
    } catch (error) {
      console.error('Failed to create dashboard:', error);
      throw error;
    }
  }
  
  /**
   * Update a dashboard
   */
  public async updateDashboard(dashboardId: string, updates: Partial<Dashboard>): Promise<Dashboard> {
    try {
      const response = await axios.put(`${API_BASE_URL}/dashboards/${dashboardId}`, updates);
      
      return response.data;
    } catch (error) {
      console.error(`Failed to update dashboard ${dashboardId}:`, error);
      throw error;
    }
  }
  
  /**
   * Save a dashboard (create or update)
   */
  public async saveDashboard(dashboardId: string, dashboard: Dashboard): Promise<Dashboard> {
    try {
      // Check if dashboard exists
      try {
        await this.getDashboard(dashboardId);
        
        // Dashboard exists, update it
        return this.updateDashboard(dashboardId, dashboard);
      } catch (error) {
        // Dashboard doesn't exist, create it
        return this.createDashboard(dashboard);
      }
    } catch (error) {
      console.error(`Failed to save dashboard ${dashboardId}:`, error);
      throw error;
    }
  }
  
  /**
   * Delete a dashboard
   */
  public async deleteDashboard(dashboardId: string): Promise<boolean> {
    try {
      await axios.delete(`${API_BASE_URL}/dashboards/${dashboardId}`);
      
      return true;
    } catch (error) {
      console.error(`Failed to delete dashboard ${dashboardId}:`, error);
      throw error;
    }
  }
  
  /**
   * Get the default dashboard for a user
   */
  public async getDefaultDashboard(userId: string): Promise<Dashboard | null> {
    try {
      const dashboards = await this.getDashboards(userId);
      
      // Find default dashboard
      const defaultDashboard = dashboards.find(dashboard => dashboard.isDefault);
      
      if (defaultDashboard) {
        return defaultDashboard;
      }
      
      // If no default dashboard, return the first one
      if (dashboards.length > 0) {
        return dashboards[0];
      }
      
      // No dashboards found
      return null;
    } catch (error) {
      console.error(`Failed to get default dashboard for user ${userId}:`, error);
      throw error;
    }
  }
  
  /**
   * Set a dashboard as the default for a user
   */
  public async setDefaultDashboard(userId: string, dashboardId: string): Promise<boolean> {
    try {
      // Get all dashboards for user
      const dashboards = await this.getDashboards(userId);
      
      // Update each dashboard
      for (const dashboard of dashboards) {
        await this.updateDashboard(dashboard.id, {
          isDefault: dashboard.id === dashboardId
        });
      }
      
      return true;
    } catch (error) {
      console.error(`Failed to set default dashboard ${dashboardId} for user ${userId}:`, error);
      throw error;
    }
  }
  
  /**
   * Create a default dashboard for a new user
   */
  public async createDefaultDashboard(userId: string): Promise<Dashboard> {
    try {
      // Create default dashboard
      const defaultDashboard: Partial<Dashboard> = {
        userId,
        name: 'Default Dashboard',
        isDefault: true,
        layouts: {
          lg: [
            { i: 'agent-status', x: 0, y: 0, w: 4, h: 4 },
            { i: 'system-metrics', x: 4, y: 0, w: 8, h: 4 },
            { i: 'task-list', x: 0, y: 4, w: 6, h: 6 }
          ],
          md: [
            { i: 'agent-status', x: 0, y: 0, w: 4, h: 4 },
            { i: 'system-metrics', x: 4, y: 0, w: 6, h: 4 },
            { i: 'task-list', x: 0, y: 4, w: 6, h: 6 }
          ],
          sm: [
            { i: 'agent-status', x: 0, y: 0, w: 6, h: 4 },
            { i: 'system-metrics', x: 0, y: 4, w: 6, h: 4 },
            { i: 'task-list', x: 0, y: 8, w: 6, h: 6 }
          ],
          xs: [
            { i: 'agent-status', x: 0, y: 0, w: 4, h: 4 },
            { i: 'system-metrics', x: 0, y: 4, w: 4, h: 4 },
            { i: 'task-list', x: 0, y: 8, w: 4, h: 6 }
          ]
        },
        widgets: [
          {
            id: 'agent-status',
            type: 'agent-status',
            title: 'Agent Status',
            config: {
              showOfflineAgents: true,
              refreshInterval: 30
            }
          },
          {
            id: 'system-metrics',
            type: 'system-metrics',
            title: 'System Metrics',
            config: {
              metrics: ['cpu', 'memory', 'network'],
              refreshInterval: 10
            }
          },
          {
            id: 'task-list',
            type: 'task-list',
            title: 'Task List',
            config: {
              filter: 'all',
              sortBy: 'priority',
              limit: 10
            }
          }
        ]
      };
      
      return this.createDashboard(defaultDashboard);
    } catch (error) {
      console.error(`Failed to create default dashboard for user ${userId}:`, error);
      throw error;
    }
  }
}

export default new DashboardService();
```

### 6.2 Widget Service

The Widget Service provides widget metadata and configuration options.

```typescript
import axios from 'axios';
import { API_BASE_URL } from '../config';

interface WidgetDefinition {
  type: string;
  name: string;
  description: string;
  category: string;
  defaultConfig: any;
  defaultSize: {
    w: number;
    h: number;
    minW?: number;
    maxW?: number;
    minH?: number;
    maxH?: number;
  };
}

class WidgetService {
  /**
   * Get all available widget types
   */
  public async getWidgetTypes(): Promise<WidgetDefinition[]> {
    try {
      const response = await axios.get(`${API_BASE_URL}/widgets/types`);
      
      return response.data;
    } catch (error) {
      console.error('Failed to get widget types:', error);
      throw error;
    }
  }
  
  /**
   * Get widget type definition
   */
  public async getWidgetType(type: string): Promise<WidgetDefinition> {
    try {
      const response = await axios.get(`${API_BASE_URL}/widgets/types/${type}`);
      
      return response.data;
    } catch (error) {
      console.error(`Failed to get widget type ${type}:`, error);
      throw error;
    }
  }
  
  /**
   * Get widget types by category
   */
  public async getWidgetTypesByCategory(): Promise<{ [category: string]: WidgetDefinition[] }> {
    try {
      const widgetTypes = await this.getWidgetTypes();
      
      // Group by category
      const categories: { [category: string]: WidgetDefinition[] } = {};
      
      widgetTypes.forEach(widgetType => {
        if (!categories[widgetType.category]) {
          categories[widgetType.category] = [];
        }
        
        categories[widgetType.category].push(widgetType);
      });
      
      return categories;
    } catch (error) {
      console.error('Failed to get widget types by category:', error);
      throw error;
    }
  }
  
  /**
   * Validate widget configuration
   */
  public async validateWidgetConfig(type: string, config: any): Promise<{ valid: boolean; errors?: string[] }> {
    try {
      const response = await axios.post(`${API_BASE_URL}/widgets/validate`, {
        type,
        config
      });
      
      return response.data;
    } catch (error) {
      console.error(`Failed to validate widget config for type ${type}:`, error);
      throw error;
    }
  }
}

export default new WidgetService();
```

### 6.3 Data Provider

The Data Provider serves as a unified interface for accessing data from various sources.

```typescript
import axios from 'axios';
import { API_BASE_URL } from '../config';

class DataProvider {
  /**
   * Get agents
   */
  public async getAgents(): Promise<any[]> {
    try {
      const response = await axios.get(`${API_BASE_URL}/agents`);
      
      return response.data;
    } catch (error) {
      console.error('Failed to get agents:', error);
      throw error;
    }
  }
  
  /**
   * Get system metrics
   */
  public async getSystemMetrics(metrics: string[]): Promise<any> {
    try {
      const response = await axios.get(`${API_BASE_URL}/metrics`, {
        params: { metrics: metrics.join(',') }
      });
      
      return response.data;
    } catch (error) {
      console.error('Failed to get system metrics:', error);
      throw error;
    }
  }
  
  /**
   * Get tasks
   */
  public async getTasks(filter: string, sortBy: string, limit: number): Promise<any[]> {
    try {
      const response = await axios.get(`${API_BASE_URL}/tasks`, {
        params: { filter, sortBy, limit }
      });
      
      return response.data;
    } catch (error) {
      console.error('Failed to get tasks:', error);
      throw error;
    }
  }
  
  /**
   * Get agent activities
   */
  public async getAgentActivities(agentId?: string, limit: number = 10): Promise<any[]> {
    try {
      const params: any = { limit };
      
      if (agentId) {
        params.agentId = agentId;
      }
      
      const response = await axios.get(`${API_BASE_URL}/agents/activities`, {
        params
      });
      
      return response.data;
    } catch (error) {
      console.error('Failed to get agent activities:', error);
      throw error;
    }
  }
  
  /**
   * Get model usage
   */
  public async getModelUsage(period: string = 'day'): Promise<any> {
    try {
      const response = await axios.get(`${API_BASE_URL}/models/usage`, {
        params: { period }
      });
      
      return response.data;
    } catch (error) {
      console.error('Failed to get model usage:', error);
      throw error;
    }
  }
  
  /**
   * Get workflow status
   */
  public async getWorkflowStatus(workflowId: string): Promise<any> {
    try {
      const response = await axios.get(`${API_BASE_URL}/workflows/${workflowId}/status`);
      
      return response.data;
    } catch (error) {
      console.error(`Failed to get workflow status for ${workflowId}:`, error);
      throw error;
    }
  }
  
  /**
   * Get active workflows
   */
  public async getActiveWorkflows(): Promise<any[]> {
    try {
      const response = await axios.get(`${API_BASE_URL}/workflows/active`);
      
      return response.data;
    } catch (error) {
      console.error('Failed to get active workflows:', error);
      throw error;
    }
  }
  
  /**
   * Get resource usage
   */
  public async getResourceUsage(): Promise<any> {
    try {
      const response = await axios.get(`${API_BASE_URL}/system/resources`);
      
      return response.data;
    } catch (error) {
      console.error('Failed to get resource usage:', error);
      throw error;
    }
  }
  
  /**
   * Get custom data
   */
  public async getCustomData(endpoint: string, params: any = {}): Promise<any> {
    try {
      const response = await axios.get(`${API_BASE_URL}/${endpoint}`, {
        params
      });
      
      return response.data;
    } catch (error) {
      console.error(`Failed to get custom data from ${endpoint}:`, error);
      throw error;
    }
  }
}

export default new DataProvider();
```

## 7. Dashboard Management Components

### 7.1 Dashboard Manager

The Dashboard Manager allows users to create, edit, and delete dashboards.

```typescript
import React, { useState, useEffect } from 'react';
import DashboardService from '../services/DashboardService';
import DashboardController from './DashboardController';
import DashboardSelector from './DashboardSelector';
import DashboardEditor from './DashboardEditor';

interface DashboardManagerProps {
  userId: string;
}

const DashboardManager: React.FC<DashboardManagerProps> = ({ userId }) => {
  const [dashboards, setDashboards] = useState<any[]>([]);
  const [selectedDashboardId, setSelectedDashboardId] = useState<string | null>(null);
  const [isEditing, setIsEditing] = useState<boolean>(false);
  const [isCreating, setIsCreating] = useState<boolean>(false);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  
  // Load dashboards
  useEffect(() => {
    const loadDashboards = async () => {
      try {
        setIsLoading(true);
        setError(null);
        
        // Get all dashboards for user
        const userDashboards = await DashboardService.getDashboards(userId);
        
        setDashboards(userDashboards);
        
        // If no dashboards, create default
        if (userDashboards.length === 0) {
          const defaultDashboard = await DashboardService.createDefaultDashboard(userId);
          
          setDashboards([defaultDashboard]);
          setSelectedDashboardId(defaultDashboard.id);
        } else {
          // Get default dashboard
          const defaultDashboard = await DashboardService.getDefaultDashboard(userId);
          
          if (defaultDashboard) {
            setSelectedDashboardId(defaultDashboard.id);
          } else {
            setSelectedDashboardId(userDashboards[0].id);
          }
        }
      } catch (error) {
        console.error('Failed to load dashboards:', error);
        setError('Failed to load dashboards. Please try again later.');
      } finally {
        setIsLoading(false);
      }
    };
    
    loadDashboards();
  }, [userId]);
  
  // Create dashboard
  const handleCreateDashboard = async (dashboard: any) => {
    try {
      setIsLoading(true);
      setError(null);
      
      // Create dashboard
      const newDashboard = await DashboardService.createDashboard({
        ...dashboard,
        userId
      });
      
      // Update dashboards
      setDashboards(prevDashboards => [...prevDashboards, newDashboard]);
      
      // Select new dashboard
      setSelectedDashboardId(newDashboard.id);
      
      // Exit create mode
      setIsCreating(false);
    } catch (error) {
      console.error('Failed to create dashboard:', error);
      setError('Failed to create dashboard. Please try again later.');
    } finally {
      setIsLoading(false);
    }
  };
  
  // Update dashboard
  const handleUpdateDashboard = async (dashboardId: string, updates: any) => {
    try {
      setIsLoading(true);
      setError(null);
      
      // Update dashboard
      const updatedDashboard = await DashboardService.updateDashboard(dashboardId, updates);
      
      // Update dashboards
      setDashboards(prevDashboards =>
        prevDashboards.map(dashboard =>
          dashboard.id === dashboardId ? updatedDashboard : dashboard
        )
      );
      
      // Exit edit mode
      setIsEditing(false);
    } catch (error) {
      console.error(`Failed to update dashboard ${dashboardId}:`, error);
      setError('Failed to update dashboard. Please try again later.');
    } finally {
      setIsLoading(false);
    }
  };
  
  // Delete dashboard
  const handleDeleteDashboard = async (dashboardId: string) => {
    try {
      setIsLoading(true);
      setError(null);
      
      // Delete dashboard
      await DashboardService.deleteDashboard(dashboardId);
      
      // Update dashboards
      const updatedDashboards = dashboards.filter(dashboard => dashboard.id !== dashboardId);
      setDashboards(updatedDashboards);
      
      // Select another dashboard
      if (updatedDashboards.length > 0) {
        setSelectedDashboardId(updatedDashboards[0].id);
      } else {
        // Create default dashboard
        const defaultDashboard = await DashboardService.createDefaultDashboard(userId);
        
        setDashboards([defaultDashboard]);
        setSelectedDashboardId(defaultDashboard.id);
      }
    } catch (error) {
      console.error(`Failed to delete dashboard ${dashboardId}:`, error);
      setError('Failed to delete dashboard. Please try again later.');
    } finally {
      setIsLoading(false);
    }
  };
  
  // Set default dashboard
  const handleSetDefaultDashboard = async (dashboardId: string) => {
    try {
      setIsLoading(true);
      setError(null);
      
      // Set default dashboard
      await DashboardService.setDefaultDashboard(userId, dashboardId);
      
      // Update dashboards
      setDashboards(prevDashboards =>
        prevDashboards.map(dashboard => ({
          ...dashboard,
          isDefault: dashboard.id === dashboardId
        }))
      );
    } catch (error) {
      console.error(`Failed to set default dashboard ${dashboardId}:`, error);
      setError('Failed to set default dashboard. Please try again later.');
    } finally {
      setIsLoading(false);
    }
  };
  
  if (isLoading && dashboards.length === 0) {
    return <div>Loading dashboards...</div>;
  }
  
  if (error) {
    return <div className="error">{error}</div>;
  }
  
  if (isCreating) {
    return (
      <DashboardEditor
        mode="create"
        onSave={handleCreateDashboard}
        onCancel={() => setIsCreating(false)}
      />
    );
  }
  
  if (isEditing && selectedDashboardId) {
    const dashboard = dashboards.find(d => d.id === selectedDashboardId);
    
    if (!dashboard) {
      return <div className="error">Dashboard not found</div>;
    }
    
    return (
      <DashboardEditor
        mode="edit"
        dashboard={dashboard}
        onSave={(updates) => handleUpdateDashboard(selectedDashboardId, updates)}
        onCancel={() => setIsEditing(false)}
      />
    );
  }
  
  return (
    <div className="dashboard-manager">
      <DashboardSelector
        dashboards={dashboards}
        selectedDashboardId={selectedDashboardId}
        onSelect={setSelectedDashboardId}
        onCreate={() => setIsCreating(true)}
        onEdit={() => setIsEditing(true)}
        onDelete={handleDeleteDashboard}
        onSetDefault={handleSetDefaultDashboard}
      />
      
      {selectedDashboardId && (
        <DashboardController
          dashboardId={selectedDashboardId}
          userId={userId}
        />
      )}
    </div>
  );
};

export default DashboardManager;
```

### 7.2 Dashboard Selector

The Dashboard Selector allows users to select a dashboard from a list of available dashboards.

```typescript
import React from 'react';

interface DashboardSelectorProps {
  dashboards: any[];
  selectedDashboardId: string | null;
  onSelect: (dashboardId: string) => void;
  onCreate: () => void;
  onEdit: () => void;
  onDelete: (dashboardId: string) => void;
  onSetDefault: (dashboardId: string) => void;
}

const DashboardSelector: React.FC<DashboardSelectorProps> = ({
  dashboards,
  selectedDashboardId,
  onSelect,
  onCreate,
  onEdit,
  onDelete,
  onSetDefault
}) => {
  const handleDelete = (dashboardId: string) => {
    if (window.confirm('Are you sure you want to delete this dashboard?')) {
      onDelete(dashboardId);
    }
  };
  
  return (
    <div className="dashboard-selector">
      <div className="dashboard-selector-header">
        <h2>Dashboards</h2>
        <button
          className="create-dashboard-button"
          onClick={onCreate}
        >
          <i className="fas fa-plus"></i> New Dashboard
        </button>
      </div>
      
      <div className="dashboard-list">
        {dashboards.map(dashboard => (
          <div
            key={dashboard.id}
            className={`dashboard-item ${dashboard.id === selectedDashboardId ? 'selected' : ''}`}
            onClick={() => onSelect(dashboard.id)}
          >
            <div className="dashboard-info">
              <div className="dashboard-name">
                {dashboard.name}
                {dashboard.isDefault && (
                  <span className="default-badge">Default</span>
                )}
              </div>
              <div className="dashboard-meta">
                {dashboard.widgets.length} widgets
              </div>
            </div>
            
            <div className="dashboard-actions">
              {!dashboard.isDefault && (
                <button
                  className="set-default-button"
                  onClick={(e) => {
                    e.stopPropagation();
                    onSetDefault(dashboard.id);
                  }}
                  title="Set as default"
                >
                  <i className="fas fa-star"></i>
                </button>
              )}
              
              <button
                className="edit-button"
                onClick={(e) => {
                  e.stopPropagation();
                  onSelect(dashboard.id);
                  onEdit();
                }}
                title="Edit dashboard"
              >
                <i className="fas fa-edit"></i>
              </button>
              
              {!dashboard.isDefault && (
                <button
                  className="delete-button"
                  onClick={(e) => {
                    e.stopPropagation();
                    handleDelete(dashboard.id);
                  }}
                  title="Delete dashboard"
                >
                  <i className="fas fa-trash"></i>
                </button>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};

export default DashboardSelector;
```
