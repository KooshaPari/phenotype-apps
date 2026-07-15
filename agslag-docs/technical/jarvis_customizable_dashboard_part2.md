# Jarvis SWE Agent Customizable Dashboard - Technical Implementation (Part 2)

## 4. Widget Components

### 4.1 Widget Base Class

The Widget Base Class provides common functionality for all widgets.

```typescript
import React, { useState, useEffect } from 'react';
import DataProvider from '../services/DataProvider';

interface WidgetProps {
  id: string;
  config: any;
  width: number;
  height: number;
}

interface WidgetState {
  loading: boolean;
  error: string | null;
  data: any;
  lastUpdated: Date | null;
}

abstract class Widget<P extends WidgetProps, S extends WidgetState> extends React.Component<P, S> {
  private refreshInterval: number | null = null;
  
  constructor(props: P) {
    super(props);
    
    // Initialize state
    this.state = {
      loading: true,
      error: null,
      data: null,
      lastUpdated: null,
      ...this.getInitialState()
    } as S;
  }
  
  // Abstract methods to be implemented by subclasses
  protected abstract getInitialState(): Partial<S>;
  protected abstract renderContent(): React.ReactNode;
  protected abstract fetchData(): Promise<any>;
  
  componentDidMount() {
    // Fetch initial data
    this.loadData();
    
    // Set up refresh interval if configured
    if (this.props.config.refreshInterval) {
      this.startRefreshInterval();
    }
  }
  
  componentDidUpdate(prevProps: P) {
    // Check if config has changed
    if (JSON.stringify(prevProps.config) !== JSON.stringify(this.props.config)) {
      // Reload data
      this.loadData();
      
      // Update refresh interval
      if (prevProps.config.refreshInterval !== this.props.config.refreshInterval) {
        this.stopRefreshInterval();
        
        if (this.props.config.refreshInterval) {
          this.startRefreshInterval();
        }
      }
    }
  }
  
  componentWillUnmount() {
    // Clean up refresh interval
    this.stopRefreshInterval();
  }
  
  private startRefreshInterval() {
    this.stopRefreshInterval();
    
    this.refreshInterval = window.setInterval(
      () => this.loadData(),
      this.props.config.refreshInterval * 1000
    );
  }
  
  private stopRefreshInterval() {
    if (this.refreshInterval !== null) {
      clearInterval(this.refreshInterval);
      this.refreshInterval = null;
    }
  }
  
  private async loadData() {
    try {
      this.setState({ loading: true, error: null });
      
      const data = await this.fetchData();
      
      this.setState({
        loading: false,
        data,
        lastUpdated: new Date(),
        error: null
      });
    } catch (error) {
      console.error(`Error loading data for widget ${this.props.id}:`, error);
      
      this.setState({
        loading: false,
        error: error instanceof Error ? error.message : 'An error occurred'
      });
    }
  }
  
  render() {
    const { loading, error, lastUpdated } = this.state;
    
    return (
      <div className="widget">
        {loading && (
          <div className="widget-loading">
            <div className="spinner"></div>
          </div>
        )}
        
        {error && (
          <div className="widget-error">
            <i className="fas fa-exclamation-triangle"></i>
            <p>{error}</p>
            <button onClick={() => this.loadData()}>Retry</button>
          </div>
        )}
        
        <div className="widget-content">
          {this.renderContent()}
        </div>
        
        {lastUpdated && (
          <div className="widget-footer">
            <span className="last-updated">
              Updated: {lastUpdated.toLocaleTimeString()}
            </span>
            <button
              className="refresh-button"
              onClick={() => this.loadData()}
              aria-label="Refresh"
            >
              <i className="fas fa-sync-alt"></i>
            </button>
          </div>
        )}
      </div>
    );
  }
}

export default Widget;
```

### 4.2 Agent Status Widget

The Agent Status Widget displays the status of agents in the system.

```typescript
import React from 'react';
import Widget from './Widget';
import DataProvider from '../services/DataProvider';

interface AgentStatusWidgetProps {
  id: string;
  config: {
    showOfflineAgents: boolean;
    refreshInterval: number;
  };
  width: number;
  height: number;
}

interface AgentStatusWidgetState {
  loading: boolean;
  error: string | null;
  data: {
    agents: Agent[];
    totalAgents: number;
    activeAgents: number;
    idleAgents: number;
    offlineAgents: number;
  } | null;
  lastUpdated: Date | null;
}

interface Agent {
  id: string;
  name: string;
  status: 'active' | 'idle' | 'offline';
  lastActivity: string;
  type: string;
}

class AgentStatusWidget extends Widget<AgentStatusWidgetProps, AgentStatusWidgetState> {
  protected getInitialState(): Partial<AgentStatusWidgetState> {
    return {};
  }
  
  protected async fetchData(): Promise<any> {
    // Fetch agent data from data provider
    const agents = await DataProvider.getAgents();
    
    // Filter agents if configured
    const filteredAgents = this.props.config.showOfflineAgents
      ? agents
      : agents.filter(agent => agent.status !== 'offline');
    
    // Calculate statistics
    const totalAgents = filteredAgents.length;
    const activeAgents = filteredAgents.filter(agent => agent.status === 'active').length;
    const idleAgents = filteredAgents.filter(agent => agent.status === 'idle').length;
    const offlineAgents = filteredAgents.filter(agent => agent.status === 'offline').length;
    
    return {
      agents: filteredAgents,
      totalAgents,
      activeAgents,
      idleAgents,
      offlineAgents
    };
  }
  
  protected renderContent(): React.ReactNode {
    const { data } = this.state;
    
    if (!data) {
      return null;
    }
    
    return (
      <div className="agent-status-widget">
        <div className="agent-stats">
          <div className="stat">
            <div className="stat-value">{data.totalAgents}</div>
            <div className="stat-label">Total</div>
          </div>
          <div className="stat">
            <div className="stat-value">{data.activeAgents}</div>
            <div className="stat-label">Active</div>
          </div>
          <div className="stat">
            <div className="stat-value">{data.idleAgents}</div>
            <div className="stat-label">Idle</div>
          </div>
          <div className="stat">
            <div className="stat-value">{data.offlineAgents}</div>
            <div className="stat-label">Offline</div>
          </div>
        </div>
        
        <div className="agent-list">
          {data.agents.map(agent => (
            <div key={agent.id} className={`agent-item agent-${agent.status}`}>
              <div className="agent-icon">
                <i className="fas fa-robot"></i>
              </div>
              <div className="agent-info">
                <div className="agent-name">{agent.name}</div>
                <div className="agent-type">{agent.type}</div>
              </div>
              <div className="agent-status">
                <span className={`status-indicator status-${agent.status}`}></span>
                <span className="status-text">{agent.status}</span>
              </div>
            </div>
          ))}
        </div>
      </div>
    );
  }
}

export default AgentStatusWidget;
```

### 4.3 System Metrics Widget

The System Metrics Widget displays system performance metrics.

```typescript
import React from 'react';
import Widget from './Widget';
import DataProvider from '../services/DataProvider';
import { Line } from 'react-chartjs-2';
import 'chart.js/auto';

interface SystemMetricsWidgetProps {
  id: string;
  config: {
    metrics: string[];
    refreshInterval: number;
  };
  width: number;
  height: number;
}

interface SystemMetricsWidgetState {
  loading: boolean;
  error: string | null;
  data: {
    timestamps: string[];
    metrics: {
      [key: string]: number[];
    };
  } | null;
  lastUpdated: Date | null;
}

class SystemMetricsWidget extends Widget<SystemMetricsWidgetProps, SystemMetricsWidgetState> {
  protected getInitialState(): Partial<SystemMetricsWidgetState> {
    return {};
  }
  
  protected async fetchData(): Promise<any> {
    // Fetch metrics data from data provider
    const { metrics } = this.props.config;
    const metricsData = await DataProvider.getSystemMetrics(metrics);
    
    return metricsData;
  }
  
  protected renderContent(): React.ReactNode {
    const { data } = this.state;
    const { metrics } = this.props.config;
    
    if (!data) {
      return null;
    }
    
    // Prepare chart data
    const chartData = {
      labels: data.timestamps,
      datasets: metrics.map((metric, index) => {
        const colors = [
          { bg: 'rgba(75, 192, 192, 0.2)', border: 'rgba(75, 192, 192, 1)' },
          { bg: 'rgba(255, 99, 132, 0.2)', border: 'rgba(255, 99, 132, 1)' },
          { bg: 'rgba(54, 162, 235, 0.2)', border: 'rgba(54, 162, 235, 1)' },
          { bg: 'rgba(255, 206, 86, 0.2)', border: 'rgba(255, 206, 86, 1)' }
        ];
        
        const colorIndex = index % colors.length;
        
        return {
          label: this.getMetricLabel(metric),
          data: data.metrics[metric] || [],
          fill: true,
          backgroundColor: colors[colorIndex].bg,
          borderColor: colors[colorIndex].border,
          tension: 0.4
        };
      })
    };
    
    // Chart options
    const chartOptions = {
      responsive: true,
      maintainAspectRatio: false,
      scales: {
        y: {
          beginAtZero: true,
          ticks: {
            callback: (value: number) => this.formatMetricValue(value)
          }
        }
      },
      plugins: {
        legend: {
          position: 'bottom' as const
        },
        tooltip: {
          callbacks: {
            label: (context: any) => {
              const label = context.dataset.label || '';
              const value = context.parsed.y;
              return `${label}: ${this.formatMetricValue(value)}`;
            }
          }
        }
      }
    };
    
    return (
      <div className="system-metrics-widget">
        <div className="chart-container" style={{ height: '100%' }}>
          <Line data={chartData} options={chartOptions} />
        </div>
      </div>
    );
  }
  
  private getMetricLabel(metric: string): string {
    switch (metric) {
      case 'cpu':
        return 'CPU Usage';
      case 'memory':
        return 'Memory Usage';
      case 'network':
        return 'Network Traffic';
      case 'disk':
        return 'Disk Usage';
      default:
        return metric;
    }
  }
  
  private formatMetricValue(value: number): string {
    const { metrics } = this.props.config;
    
    if (metrics.includes('cpu')) {
      return `${value.toFixed(1)}%`;
    }
    
    if (metrics.includes('memory')) {
      return `${value.toFixed(1)}%`;
    }
    
    if (metrics.includes('network')) {
      if (value < 1024) {
        return `${value.toFixed(1)} KB/s`;
      } else {
        return `${(value / 1024).toFixed(1)} MB/s`;
      }
    }
    
    if (metrics.includes('disk')) {
      return `${value.toFixed(1)}%`;
    }
    
    return value.toString();
  }
}

export default SystemMetricsWidget;
```

### 4.4 Task List Widget

The Task List Widget displays a list of tasks in the system.

```typescript
import React from 'react';
import Widget from './Widget';
import DataProvider from '../services/DataProvider';

interface TaskListWidgetProps {
  id: string;
  config: {
    filter: 'all' | 'pending' | 'in-progress' | 'completed';
    sortBy: 'priority' | 'deadline' | 'created';
    limit: number;
  };
  width: number;
  height: number;
}

interface TaskListWidgetState {
  loading: boolean;
  error: string | null;
  data: {
    tasks: Task[];
    totalTasks: number;
  } | null;
  lastUpdated: Date | null;
}

interface Task {
  id: string;
  title: string;
  status: 'pending' | 'in-progress' | 'completed';
  priority: 'low' | 'medium' | 'high';
  deadline: string | null;
  assignee: string | null;
  created: string;
}

class TaskListWidget extends Widget<TaskListWidgetProps, TaskListWidgetState> {
  protected getInitialState(): Partial<TaskListWidgetState> {
    return {};
  }
  
  protected async fetchData(): Promise<any> {
    // Fetch task data from data provider
    const { filter, sortBy, limit } = this.props.config;
    const tasks = await DataProvider.getTasks(filter, sortBy, limit);
    
    return {
      tasks,
      totalTasks: tasks.length
    };
  }
  
  protected renderContent(): React.ReactNode {
    const { data } = this.state;
    
    if (!data) {
      return null;
    }
    
    return (
      <div className="task-list-widget">
        <div className="task-list-header">
          <div className="task-count">
            Showing {data.tasks.length} of {data.totalTasks} tasks
          </div>
        </div>
        
        <div className="task-list">
          {data.tasks.length === 0 ? (
            <div className="no-tasks">No tasks found</div>
          ) : (
            data.tasks.map(task => (
              <div key={task.id} className={`task-item task-${task.status}`}>
                <div className="task-priority">
                  <span className={`priority-indicator priority-${task.priority}`}></span>
                </div>
                <div className="task-info">
                  <div className="task-title">{task.title}</div>
                  <div className="task-details">
                    <span className="task-status">{task.status}</span>
                    {task.assignee && (
                      <span className="task-assignee">
                        <i className="fas fa-user"></i> {task.assignee}
                      </span>
                    )}
                    {task.deadline && (
                      <span className="task-deadline">
                        <i className="fas fa-clock"></i> {new Date(task.deadline).toLocaleDateString()}
                      </span>
                    )}
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

export default TaskListWidget;
```

## 5. Widget Configuration Components

### 5.1 Widget Settings Modal

The Widget Settings Modal allows users to configure widget settings.

```typescript
import React, { useState } from 'react';
import Modal from '../components/Modal';

interface WidgetSettingsProps {
  widget: {
    id: string;
    type: string;
    title: string;
    config: any;
  };
  onUpdate: (updates: any) => void;
  onClose: () => void;
}

const WidgetSettings: React.FC<WidgetSettingsProps> = ({
  widget,
  onUpdate,
  onClose
}) => {
  const [title, setTitle] = useState<string>(widget.title);
  const [config, setConfig] = useState<any>(widget.config);
  
  // Get widget definition
  const widgetDefinition = WidgetRegistry.getWidget(widget.type);
  
  if (!widgetDefinition) {
    return null;
  }
  
  const handleSave = () => {
    onUpdate({
      title,
      config
    });
  };
  
  const handleConfigChange = (key: string, value: any) => {
    setConfig(prevConfig => ({
      ...prevConfig,
      [key]: value
    }));
  };
  
  return (
    <Modal
      title={`Edit Widget: ${widget.title}`}
      onClose={onClose}
      footer={
        <>
          <button className="button secondary" onClick={onClose}>Cancel</button>
          <button className="button primary" onClick={handleSave}>Save</button>
        </>
      }
    >
      <div className="widget-settings">
        <div className="form-group">
          <label htmlFor="widget-title">Title</label>
          <input
            id="widget-title"
            type="text"
            value={title}
            onChange={(e) => setTitle(e.target.value)}
          />
        </div>
        
        <h3>Widget Settings</h3>
        
        {widgetDefinition.configComponent ? (
          <widgetDefinition.configComponent
            config={config}
            onChange={handleConfigChange}
          />
        ) : (
          <div className="no-settings">
            This widget has no configurable settings.
          </div>
        )}
      </div>
    </Modal>
  );
};

export default WidgetSettings;
```

### 5.2 Agent Status Widget Config

The Agent Status Widget Config component allows users to configure the Agent Status Widget.

```typescript
import React from 'react';

interface AgentStatusWidgetConfigProps {
  config: {
    showOfflineAgents: boolean;
    refreshInterval: number;
  };
  onChange: (key: string, value: any) => void;
}

const AgentStatusWidgetConfig: React.FC<AgentStatusWidgetConfigProps> = ({
  config,
  onChange
}) => {
  return (
    <div className="widget-config">
      <div className="form-group">
        <label htmlFor="show-offline-agents">
          <input
            id="show-offline-agents"
            type="checkbox"
            checked={config.showOfflineAgents}
            onChange={(e) => onChange('showOfflineAgents', e.target.checked)}
          />
          Show offline agents
        </label>
      </div>
      
      <div className="form-group">
        <label htmlFor="refresh-interval">Refresh Interval (seconds)</label>
        <input
          id="refresh-interval"
          type="number"
          min="5"
          max="3600"
          value={config.refreshInterval}
          onChange={(e) => onChange('refreshInterval', parseInt(e.target.value))}
        />
      </div>
    </div>
  );
};

export default AgentStatusWidgetConfig;
```

### 5.3 System Metrics Widget Config

The System Metrics Widget Config component allows users to configure the System Metrics Widget.

```typescript
import React from 'react';

interface SystemMetricsWidgetConfigProps {
  config: {
    metrics: string[];
    refreshInterval: number;
  };
  onChange: (key: string, value: any) => void;
}

const SystemMetricsWidgetConfig: React.FC<SystemMetricsWidgetConfigProps> = ({
  config,
  onChange
}) => {
  const availableMetrics = [
    { id: 'cpu', label: 'CPU Usage' },
    { id: 'memory', label: 'Memory Usage' },
    { id: 'network', label: 'Network Traffic' },
    { id: 'disk', label: 'Disk Usage' }
  ];
  
  const handleMetricChange = (metricId: string, checked: boolean) => {
    let newMetrics: string[];
    
    if (checked) {
      // Add metric
      newMetrics = [...config.metrics, metricId];
    } else {
      // Remove metric
      newMetrics = config.metrics.filter(id => id !== metricId);
    }
    
    onChange('metrics', newMetrics);
  };
  
  return (
    <div className="widget-config">
      <div className="form-group">
        <label>Metrics to Display</label>
        <div className="checkbox-group">
          {availableMetrics.map(metric => (
            <label key={metric.id} htmlFor={`metric-${metric.id}`}>
              <input
                id={`metric-${metric.id}`}
                type="checkbox"
                checked={config.metrics.includes(metric.id)}
                onChange={(e) => handleMetricChange(metric.id, e.target.checked)}
              />
              {metric.label}
            </label>
          ))}
        </div>
      </div>
      
      <div className="form-group">
        <label htmlFor="refresh-interval">Refresh Interval (seconds)</label>
        <input
          id="refresh-interval"
          type="number"
          min="5"
          max="3600"
          value={config.refreshInterval}
          onChange={(e) => onChange('refreshInterval', parseInt(e.target.value))}
        />
      </div>
    </div>
  );
};

export default SystemMetricsWidgetConfig;
```
