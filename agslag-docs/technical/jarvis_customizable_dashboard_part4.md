# Jarvis SWE Agent Customizable Dashboard - Technical Implementation (Part 4)

## 8. Dashboard Editor and Widget Selector Components

### 8.1 Dashboard Editor

The Dashboard Editor allows users to create and edit dashboard metadata.

```typescript
import React, { useState } from 'react';

interface DashboardEditorProps {
  mode: 'create' | 'edit';
  dashboard?: any;
  onSave: (dashboard: any) => void;
  onCancel: () => void;
}

const DashboardEditor: React.FC<DashboardEditorProps> = ({
  mode,
  dashboard,
  onSave,
  onCancel
}) => {
  const [name, setName] = useState<string>(dashboard?.name || '');
  const [description, setDescription] = useState<string>(dashboard?.description || '');
  const [isDefault, setIsDefault] = useState<boolean>(dashboard?.isDefault || false);
  const [error, setError] = useState<string | null>(null);
  
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    
    // Validate form
    if (!name.trim()) {
      setError('Dashboard name is required');
      return;
    }
    
    // Create dashboard object
    const dashboardData = {
      ...(dashboard || {}),
      name,
      description,
      isDefault
    };
    
    // Save dashboard
    onSave(dashboardData);
  };
  
  return (
    <div className="dashboard-editor">
      <div className="dashboard-editor-header">
        <h2>{mode === 'create' ? 'Create Dashboard' : 'Edit Dashboard'}</h2>
      </div>
      
      <form onSubmit={handleSubmit}>
        {error && (
          <div className="error-message">{error}</div>
        )}
        
        <div className="form-group">
          <label htmlFor="dashboard-name">Name</label>
          <input
            id="dashboard-name"
            type="text"
            value={name}
            onChange={(e) => setName(e.target.value)}
            required
          />
        </div>
        
        <div className="form-group">
          <label htmlFor="dashboard-description">Description</label>
          <textarea
            id="dashboard-description"
            value={description}
            onChange={(e) => setDescription(e.target.value)}
            rows={3}
          />
        </div>
        
        <div className="form-group">
          <label htmlFor="dashboard-default">
            <input
              id="dashboard-default"
              type="checkbox"
              checked={isDefault}
              onChange={(e) => setIsDefault(e.target.checked)}
            />
            Set as default dashboard
          </label>
        </div>
        
        <div className="form-actions">
          <button
            type="button"
            className="button secondary"
            onClick={onCancel}
          >
            Cancel
          </button>
          <button
            type="submit"
            className="button primary"
          >
            {mode === 'create' ? 'Create' : 'Save'}
          </button>
        </div>
      </form>
    </div>
  );
};

export default DashboardEditor;
```

### 8.2 Widget Selector

The Widget Selector allows users to browse and select widgets to add to their dashboard.

```typescript
import React, { useState, useEffect } from 'react';
import WidgetService from '../services/WidgetService';
import Modal from '../components/Modal';

interface WidgetSelectorProps {
  onSelect: (widgetType: string) => void;
  onClose: () => void;
}

const WidgetSelector: React.FC<WidgetSelectorProps> = ({
  onSelect,
  onClose
}) => {
  const [widgetsByCategory, setWidgetsByCategory] = useState<any>({});
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null);
  const [searchQuery, setSearchQuery] = useState<string>('');
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  
  // Load widget types
  useEffect(() => {
    const loadWidgetTypes = async () => {
      try {
        setIsLoading(true);
        setError(null);
        
        // Get widget types by category
        const categories = await WidgetService.getWidgetTypesByCategory();
        
        setWidgetsByCategory(categories);
        
        // Set selected category to first category
        if (Object.keys(categories).length > 0) {
          setSelectedCategory(Object.keys(categories)[0]);
        }
      } catch (error) {
        console.error('Failed to load widget types:', error);
        setError('Failed to load widget types. Please try again later.');
      } finally {
        setIsLoading(false);
      }
    };
    
    loadWidgetTypes();
  }, []);
  
  // Filter widgets by search query
  const filteredWidgets = React.useMemo(() => {
    if (!searchQuery.trim()) {
      return widgetsByCategory;
    }
    
    const query = searchQuery.toLowerCase();
    const filtered: any = {};
    
    Object.entries(widgetsByCategory).forEach(([category, widgets]: [string, any]) => {
      const matchingWidgets = widgets.filter((widget: any) =>
        widget.name.toLowerCase().includes(query) ||
        widget.description.toLowerCase().includes(query)
      );
      
      if (matchingWidgets.length > 0) {
        filtered[category] = matchingWidgets;
      }
    });
    
    return filtered;
  }, [widgetsByCategory, searchQuery]);
  
  // Get categories
  const categories = Object.keys(filteredWidgets);
  
  // Handle category selection
  const handleCategorySelect = (category: string) => {
    setSelectedCategory(category);
  };
  
  // Handle widget selection
  const handleWidgetSelect = (widgetType: string) => {
    onSelect(widgetType);
  };
  
  return (
    <Modal
      title="Add Widget"
      onClose={onClose}
    >
      <div className="widget-selector">
        <div className="widget-selector-search">
          <input
            type="text"
            placeholder="Search widgets..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
        </div>
        
        {isLoading ? (
          <div className="loading">Loading widgets...</div>
        ) : error ? (
          <div className="error">{error}</div>
        ) : categories.length === 0 ? (
          <div className="no-widgets">No widgets found</div>
        ) : (
          <div className="widget-selector-content">
            <div className="widget-categories">
              {categories.map(category => (
                <div
                  key={category}
                  className={`widget-category ${category === selectedCategory ? 'selected' : ''}`}
                  onClick={() => handleCategorySelect(category)}
                >
                  {category}
                </div>
              ))}
            </div>
            
            <div className="widget-list">
              {selectedCategory && filteredWidgets[selectedCategory]?.map((widget: any) => (
                <div
                  key={widget.type}
                  className="widget-item"
                  onClick={() => handleWidgetSelect(widget.type)}
                >
                  <div className="widget-icon">
                    <i className={`fas fa-${getWidgetIcon(widget.type)}`}></i>
                  </div>
                  <div className="widget-info">
                    <div className="widget-name">{widget.name}</div>
                    <div className="widget-description">{widget.description}</div>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>
    </Modal>
  );
};

// Helper function to get widget icon
const getWidgetIcon = (widgetType: string): string => {
  switch (widgetType) {
    case 'agent-status':
      return 'robot';
    case 'system-metrics':
      return 'chart-line';
    case 'task-list':
      return 'tasks';
    case 'agent-activity':
      return 'history';
    case 'model-usage':
      return 'brain';
    case 'workflow-status':
      return 'diagram-project';
    case 'resource-usage':
      return 'server';
    case 'code-metrics':
      return 'code';
    case 'terminal':
      return 'terminal';
    case 'notes':
      return 'sticky-note';
    default:
      return 'puzzle-piece';
  }
};

export default WidgetSelector;
```

## 9. Additional Widget Components

### 9.1 Agent Activity Widget

The Agent Activity Widget displays recent activities of agents in the system.

```typescript
import React from 'react';
import Widget from './Widget';
import DataProvider from '../services/DataProvider';
import { formatDistanceToNow } from 'date-fns';

interface AgentActivityWidgetProps {
  id: string;
  config: {
    agentId?: string;
    limit: number;
    refreshInterval: number;
  };
  width: number;
  height: number;
}

interface AgentActivityWidgetState {
  loading: boolean;
  error: string | null;
  data: {
    activities: Activity[];
  } | null;
  lastUpdated: Date | null;
}

interface Activity {
  id: string;
  agentId: string;
  agentName: string;
  type: string;
  description: string;
  timestamp: string;
  metadata?: any;
}

class AgentActivityWidget extends Widget<AgentActivityWidgetProps, AgentActivityWidgetState> {
  protected getInitialState(): Partial<AgentActivityWidgetState> {
    return {};
  }
  
  protected async fetchData(): Promise<any> {
    // Fetch agent activities from data provider
    const { agentId, limit } = this.props.config;
    const activities = await DataProvider.getAgentActivities(agentId, limit);
    
    return {
      activities
    };
  }
  
  protected renderContent(): React.ReactNode {
    const { data } = this.state;
    
    if (!data) {
      return null;
    }
    
    return (
      <div className="agent-activity-widget">
        <div className="activity-list">
          {data.activities.length === 0 ? (
            <div className="no-activities">No activities found</div>
          ) : (
            data.activities.map(activity => (
              <div key={activity.id} className={`activity-item activity-${activity.type}`}>
                <div className="activity-icon">
                  <i className={`fas fa-${getActivityIcon(activity.type)}`}></i>
                </div>
                <div className="activity-info">
                  <div className="activity-description">
                    <span className="agent-name">{activity.agentName}</span>
                    {' '}
                    {activity.description}
                  </div>
                  <div className="activity-time">
                    {formatDistanceToNow(new Date(activity.timestamp), { addSuffix: true })}
                  </div>
                </div>
              </div>
            ))
          )}
        </div>
      </div>
    );
  }
}

// Helper function to get activity icon
const getActivityIcon = (activityType: string): string => {
  switch (activityType) {
    case 'task-started':
      return 'play';
    case 'task-completed':
      return 'check';
    case 'task-failed':
      return 'times';
    case 'message':
      return 'comment';
    case 'status-change':
      return 'exchange-alt';
    case 'error':
      return 'exclamation-triangle';
    case 'connection':
      return 'plug';
    default:
      return 'circle';
  }
};

export default AgentActivityWidget;
```

### 9.2 Model Usage Widget

The Model Usage Widget displays usage statistics for AI models.

```typescript
import React from 'react';
import Widget from './Widget';
import DataProvider from '../services/DataProvider';
import { Doughnut } from 'react-chartjs-2';
import 'chart.js/auto';

interface ModelUsageWidgetProps {
  id: string;
  config: {
    period: 'day' | 'week' | 'month';
    refreshInterval: number;
  };
  width: number;
  height: number;
}

interface ModelUsageWidgetState {
  loading: boolean;
  error: string | null;
  data: {
    usage: ModelUsage[];
    totalTokens: number;
    totalCost: number;
  } | null;
  lastUpdated: Date | null;
}

interface ModelUsage {
  model: string;
  provider: string;
  tokens: number;
  cost: number;
  percentage: number;
}

class ModelUsageWidget extends Widget<ModelUsageWidgetProps, ModelUsageWidgetState> {
  protected getInitialState(): Partial<ModelUsageWidgetState> {
    return {};
  }
  
  protected async fetchData(): Promise<any> {
    // Fetch model usage from data provider
    const { period } = this.props.config;
    const usageData = await DataProvider.getModelUsage(period);
    
    // Calculate totals
    const totalTokens = usageData.reduce((sum: number, item: any) => sum + item.tokens, 0);
    const totalCost = usageData.reduce((sum: number, item: any) => sum + item.cost, 0);
    
    // Calculate percentages
    const usage = usageData.map((item: any) => ({
      ...item,
      percentage: totalTokens > 0 ? (item.tokens / totalTokens) * 100 : 0
    }));
    
    return {
      usage,
      totalTokens,
      totalCost
    };
  }
  
  protected renderContent(): React.ReactNode {
    const { data } = this.state;
    const { period } = this.props.config;
    
    if (!data) {
      return null;
    }
    
    // Prepare chart data
    const chartData = {
      labels: data.usage.map(item => `${item.model} (${item.provider})`),
      datasets: [
        {
          data: data.usage.map(item => item.tokens),
          backgroundColor: [
            '#FF6384',
            '#36A2EB',
            '#FFCE56',
            '#4BC0C0',
            '#9966FF',
            '#FF9F40',
            '#C9CBCF'
          ],
          hoverBackgroundColor: [
            '#FF6384',
            '#36A2EB',
            '#FFCE56',
            '#4BC0C0',
            '#9966FF',
            '#FF9F40',
            '#C9CBCF'
          ]
        }
      ]
    };
    
    // Chart options
    const chartOptions = {
      responsive: true,
      maintainAspectRatio: false,
      plugins: {
        legend: {
          position: 'right' as const,
          labels: {
            boxWidth: 12
          }
        },
        tooltip: {
          callbacks: {
            label: (context: any) => {
              const label = context.label || '';
              const value = context.raw;
              const percentage = (value / data.totalTokens) * 100;
              return `${label}: ${value.toLocaleString()} tokens (${percentage.toFixed(1)}%)`;
            }
          }
        }
      }
    };
    
    return (
      <div className="model-usage-widget">
        <div className="usage-summary">
          <div className="summary-item">
            <div className="summary-value">{data.totalTokens.toLocaleString()}</div>
            <div className="summary-label">Total Tokens</div>
          </div>
          <div className="summary-item">
            <div className="summary-value">${data.totalCost.toFixed(2)}</div>
            <div className="summary-label">Total Cost</div>
          </div>
          <div className="summary-item">
            <div className="summary-value">{getPeriodLabel(period)}</div>
            <div className="summary-label">Period</div>
          </div>
        </div>
        
        <div className="chart-container">
          <Doughnut data={chartData} options={chartOptions} />
        </div>
        
        <div className="usage-details">
          <table className="usage-table">
            <thead>
              <tr>
                <th>Model</th>
                <th>Provider</th>
                <th>Tokens</th>
                <th>Cost</th>
              </tr>
            </thead>
            <tbody>
              {data.usage.map(item => (
                <tr key={`${item.provider}-${item.model}`}>
                  <td>{item.model}</td>
                  <td>{item.provider}</td>
                  <td>{item.tokens.toLocaleString()}</td>
                  <td>${item.cost.toFixed(2)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    );
  }
}

// Helper function to get period label
const getPeriodLabel = (period: string): string => {
  switch (period) {
    case 'day':
      return 'Last 24 Hours';
    case 'week':
      return 'Last 7 Days';
    case 'month':
      return 'Last 30 Days';
    default:
      return 'Custom Period';
  }
};

export default ModelUsageWidget;
```

## 10. Implementation Plan

### 10.1 Phase 1: Core Components (Weeks 1-2)

Focus on implementing the core components of the customizable dashboard.

#### 10.1.1 Dashboard UI Components

- **Week 1, Days 1-2**: Implement Layout Manager
- **Week 1, Days 3-4**: Implement Widget Registry
- **Week 1, Day 5**: Implement Dashboard Controller

#### 10.1.2 Basic Widget Components

- **Week 2, Days 1-2**: Implement Widget Base Class
- **Week 2, Days 3-5**: Implement initial set of widgets (Agent Status, System Metrics, Task List)

### 10.2 Phase 2: Backend Services (Weeks 3-4)

Build the backend services to support the dashboard functionality.

#### 10.2.1 Dashboard Service

- **Week 3, Days 1-2**: Implement dashboard CRUD operations
- **Week 3, Days 3-4**: Implement dashboard configuration storage
- **Week 3, Day 5**: Implement dashboard sharing functionality

#### 10.2.2 Data Provider

- **Week 4, Days 1-3**: Implement data provider for various data sources
- **Week 4, Days 4-5**: Implement caching and optimization

### 10.3 Phase 3: Dashboard Management (Weeks 5-6)

Implement the dashboard management functionality.

#### 10.3.1 Dashboard Manager

- **Week 5, Days 1-2**: Implement Dashboard Manager
- **Week 5, Days 3-4**: Implement Dashboard Selector
- **Week 5, Day 5**: Implement Dashboard Editor

#### 10.3.2 Widget Management

- **Week 6, Days 1-2**: Implement Widget Selector
- **Week 6, Days 3-5**: Implement Widget Settings components

### 10.4 Phase 4: Additional Widgets (Weeks 7-8)

Expand the widget library with additional widgets.

#### 10.4.1 Monitoring Widgets

- **Week 7, Days 1-3**: Implement Agent Activity Widget
- **Week 7, Days 4-5**: Implement Model Usage Widget

#### 10.4.2 Specialized Widgets

- **Week 8, Days 1-2**: Implement Workflow Status Widget
- **Week 8, Days 3-4**: Implement Resource Usage Widget
- **Week 8, Day 5**: Implement Notes Widget

## 11. Integration with Jarvis UI

### 11.1 Dashboard Page

The Dashboard Page will be the main entry point for the customizable dashboard.

```typescript
import React from 'react';
import { useAuth } from '../contexts/AuthContext';
import DashboardManager from '../components/dashboard/DashboardManager';

const DashboardPage: React.FC = () => {
  const { user } = useAuth();
  
  if (!user) {
    return <div>Please log in to access your dashboard</div>;
  }
  
  return (
    <div className="dashboard-page">
      <h1>Dashboard</h1>
      <DashboardManager userId={user.id} />
    </div>
  );
};

export default DashboardPage;
```

### 11.2 Navigation Integration

The Dashboard will be integrated into the Jarvis navigation menu.

```typescript
import React from 'react';
import { Link, useLocation } from 'react-router-dom';

const Navigation: React.FC = () => {
  const location = useLocation();
  
  const isActive = (path: string) => {
    return location.pathname === path;
  };
  
  return (
    <nav className="main-navigation">
      <ul>
        <li className={isActive('/') ? 'active' : ''}>
          <Link to="/">
            <i className="fas fa-home"></i>
            <span>Home</span>
          </Link>
        </li>
        <li className={isActive('/dashboard') ? 'active' : ''}>
          <Link to="/dashboard">
            <i className="fas fa-tachometer-alt"></i>
            <span>Dashboard</span>
          </Link>
        </li>
        <li className={isActive('/agents') ? 'active' : ''}>
          <Link to="/agents">
            <i className="fas fa-robot"></i>
            <span>Agents</span>
          </Link>
        </li>
        <li className={isActive('/tasks') ? 'active' : ''}>
          <Link to="/tasks">
            <i className="fas fa-tasks"></i>
            <span>Tasks</span>
          </Link>
        </li>
        <li className={isActive('/workflows') ? 'active' : ''}>
          <Link to="/workflows">
            <i className="fas fa-diagram-project"></i>
            <span>Workflows</span>
          </Link>
        </li>
        <li className={isActive('/settings') ? 'active' : ''}>
          <Link to="/settings">
            <i className="fas fa-cog"></i>
            <span>Settings</span>
          </Link>
        </li>
      </ul>
    </nav>
  );
};

export default Navigation;
```

## 12. Conclusion

The customizable dashboard implementation outlined in this document provides a comprehensive approach to creating a flexible and user-friendly dashboard system for the Jarvis SWE Agent. By implementing a widget-based architecture with drag-and-drop capabilities, users can create personalized views of agent activities, system metrics, and other relevant information.

The key components of the implementation include:

1. **Layout Manager**: Handles the arrangement and resizing of widgets
2. **Widget Registry**: Maintains a catalog of available widgets
3. **Dashboard Controller**: Manages dashboard state and user interactions
4. **Widget Components**: Reusable components for displaying different types of data
5. **Dashboard Backend**: Services for managing dashboards and providing data

The phased implementation plan ensures a systematic approach to developing these components, with a focus on delivering a functional dashboard system that can be extended with additional widgets over time.

When integrated with the Jarvis UI, the customizable dashboard will provide users with a powerful tool for monitoring and managing their agent activities, system performance, and workflow status. The ability to create multiple dashboards with different layouts and widget configurations will allow users to tailor their experience to their specific needs and preferences.

Future enhancements could include:

1. **Dashboard Templates**: Pre-configured dashboards for common use cases
2. **Dashboard Sharing**: Ability to share dashboards with other users
3. **Widget Marketplace**: A marketplace for third-party widgets
4. **Advanced Filtering**: More sophisticated filtering options for widgets
5. **Real-Time Collaboration**: Collaborative editing of dashboards

These enhancements would further improve the flexibility and utility of the customizable dashboard system, making it an even more valuable tool for Jarvis SWE Agent users.
