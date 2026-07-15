# Jarvis SWE Agent Customizable Dashboard - Technical Implementation (Part 1)

## 1. Overview

This document outlines the technical implementation plan for adding a customizable dashboard to the Jarvis SWE Agent. The customizable dashboard will allow users to arrange and configure widgets according to their preferences, providing a personalized view of agent activities, system metrics, and other relevant information.

## 2. Architecture

### 2.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Dashboard UI                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐       │
│  │  Layout     │   │  Widget     │   │  Dashboard  │       │
│  │  Manager    │   │  Registry   │   │  Controller │       │
│  └─────────────┘   └─────────────┘   └─────────────┘       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
                            ▲
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                      Dashboard Backend                      │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐       │
│  │  Dashboard  │   │  Widget     │   │  Data       │       │
│  │  Service    │   │  Service    │   │  Provider   │       │
│  └─────────────┘   └─────────────┘   └─────────────┘       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
                            ▲
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                      Data Sources                           │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐       │
│  │  Agent      │   │  System     │   │  External   │       │
│  │  Service    │   │  Metrics    │   │  Services   │       │
│  └─────────────┘   └─────────────┘   └─────────────┘       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Key Components

#### 2.2.1 Dashboard UI

The Dashboard UI is the frontend component that users interact with. It includes:

- **Layout Manager**: Handles the arrangement and resizing of widgets
- **Widget Registry**: Maintains a catalog of available widgets
- **Dashboard Controller**: Manages dashboard state and user interactions

#### 2.2.2 Dashboard Backend

The Dashboard Backend provides services for managing dashboards and widgets:

- **Dashboard Service**: Handles dashboard configuration storage and retrieval
- **Widget Service**: Provides widget metadata and configuration options
- **Data Provider**: Serves as a unified interface for accessing data from various sources

#### 2.2.3 Data Sources

Data Sources provide the information displayed in dashboard widgets:

- **Agent Service**: Information about agents and their activities
- **System Metrics**: Performance metrics and system status
- **External Services**: Data from external APIs and services

## 3. Dashboard UI Implementation

### 3.1 Layout Manager

The Layout Manager is responsible for arranging widgets on the dashboard and handling drag-and-drop interactions.

```typescript
import React, { useState, useCallback } from 'react';
import { Responsive, WidthProvider } from 'react-grid-layout';
import 'react-grid-layout/css/styles.css';
import 'react-resizable/css/styles.css';

const ResponsiveGridLayout = WidthProvider(Responsive);

interface LayoutManagerProps {
  layouts: Layouts;
  widgets: WidgetInstance[];
  onLayoutChange: (layout: Layout[], layouts: Layouts) => void;
  onWidgetRemove: (widgetId: string) => void;
  onWidgetSettings: (widgetId: string) => void;
  breakpoints?: { [key: string]: number };
  cols?: { [key: string]: number };
}

interface Layouts {
  [key: string]: Layout[];
}

interface Layout {
  i: string;
  x: number;
  y: number;
  w: number;
  h: number;
  minW?: number;
  maxW?: number;
  minH?: number;
  maxH?: number;
  static?: boolean;
}

interface WidgetInstance {
  id: string;
  type: string;
  title: string;
  config: any;
  component: React.ComponentType<any>;
}

const LayoutManager: React.FC<LayoutManagerProps> = ({
  layouts,
  widgets,
  onLayoutChange,
  onWidgetRemove,
  onWidgetSettings,
  breakpoints = { lg: 1200, md: 996, sm: 768, xs: 480, xxs: 0 },
  cols = { lg: 12, md: 10, sm: 6, xs: 4, xxs: 2 }
}) => {
  const [currentBreakpoint, setCurrentBreakpoint] = useState<string>('lg');
  
  const handleLayoutChange = useCallback((currentLayout: Layout[], allLayouts: Layouts) => {
    onLayoutChange(currentLayout, allLayouts);
  }, [onLayoutChange]);
  
  const handleBreakpointChange = useCallback((newBreakpoint: string) => {
    setCurrentBreakpoint(newBreakpoint);
  }, []);
  
  return (
    <div className="layout-manager">
      <ResponsiveGridLayout
        className="layout"
        layouts={layouts}
        breakpoints={breakpoints}
        cols={cols}
        rowHeight={100}
        margin={[16, 16]}
        containerPadding={[16, 16]}
        onLayoutChange={handleLayoutChange}
        onBreakpointChange={handleBreakpointChange}
        draggableHandle=".widget-header"
        resizeHandles={['se']}
      >
        {widgets.map(widget => (
          <div key={widget.id} className="widget-container">
            <div className="widget-header">
              <h3>{widget.title}</h3>
              <div className="widget-controls">
                <button
                  className="widget-settings-button"
                  onClick={() => onWidgetSettings(widget.id)}
                  aria-label="Widget settings"
                >
                  <i className="fas fa-cog"></i>
                </button>
                <button
                  className="widget-remove-button"
                  onClick={() => onWidgetRemove(widget.id)}
                  aria-label="Remove widget"
                >
                  <i className="fas fa-times"></i>
                </button>
              </div>
            </div>
            <div className="widget-content">
              <widget.component
                id={widget.id}
                config={widget.config}
                width={100}
                height={100}
              />
            </div>
          </div>
        ))}
      </ResponsiveGridLayout>
    </div>
  );
};

export default LayoutManager;
```

### 3.2 Widget Registry

The Widget Registry maintains a catalog of available widgets and their metadata.

```typescript
interface WidgetDefinition {
  type: string;
  name: string;
  description: string;
  category: string;
  component: React.ComponentType<any>;
  defaultConfig: any;
  defaultSize: {
    w: number;
    h: number;
    minW?: number;
    maxW?: number;
    minH?: number;
    maxH?: number;
  };
  configComponent?: React.ComponentType<any>;
}

class WidgetRegistry {
  private widgets: Map<string, WidgetDefinition> = new Map();
  
  constructor() {
    // Register built-in widgets
    this.registerBuiltInWidgets();
  }
  
  private registerBuiltInWidgets(): void {
    // Agent Status Widget
    this.registerWidget({
      type: 'agent-status',
      name: 'Agent Status',
      description: 'Displays the status of agents in the system',
      category: 'Agents',
      component: AgentStatusWidget,
      defaultConfig: {
        showOfflineAgents: true,
        refreshInterval: 30
      },
      defaultSize: {
        w: 4,
        h: 4,
        minW: 2,
        minH: 2
      },
      configComponent: AgentStatusWidgetConfig
    });
    
    // System Metrics Widget
    this.registerWidget({
      type: 'system-metrics',
      name: 'System Metrics',
      description: 'Displays system performance metrics',
      category: 'System',
      component: SystemMetricsWidget,
      defaultConfig: {
        metrics: ['cpu', 'memory', 'network'],
        refreshInterval: 10
      },
      defaultSize: {
        w: 6,
        h: 4,
        minW: 3,
        minH: 2
      },
      configComponent: SystemMetricsWidgetConfig
    });
    
    // Task List Widget
    this.registerWidget({
      type: 'task-list',
      name: 'Task List',
      description: 'Displays a list of tasks in the system',
      category: 'Tasks',
      component: TaskListWidget,
      defaultConfig: {
        filter: 'all',
        sortBy: 'priority',
        limit: 10
      },
      defaultSize: {
        w: 4,
        h: 6,
        minW: 3,
        minH: 3
      },
      configComponent: TaskListWidgetConfig
    });
    
    // Add more built-in widgets...
  }
  
  public registerWidget(definition: WidgetDefinition): void {
    this.widgets.set(definition.type, definition);
  }
  
  public getWidget(type: string): WidgetDefinition | undefined {
    return this.widgets.get(type);
  }
  
  public getAllWidgets(): WidgetDefinition[] {
    return Array.from(this.widgets.values());
  }
  
  public getWidgetsByCategory(): { [category: string]: WidgetDefinition[] } {
    const categories: { [category: string]: WidgetDefinition[] } = {};
    
    this.widgets.forEach(widget => {
      if (!categories[widget.category]) {
        categories[widget.category] = [];
      }
      
      categories[widget.category].push(widget);
    });
    
    return categories;
  }
}

export default new WidgetRegistry();
```

### 3.3 Dashboard Controller

The Dashboard Controller manages the state of the dashboard and handles user interactions.

```typescript
import React, { useState, useEffect, useCallback } from 'react';
import LayoutManager from './LayoutManager';
import WidgetRegistry from './WidgetRegistry';
import WidgetSelector from './WidgetSelector';
import WidgetSettings from './WidgetSettings';
import DashboardService from '../services/DashboardService';
import { v4 as uuidv4 } from 'uuid';

interface DashboardControllerProps {
  dashboardId: string;
  userId: string;
}

const DashboardController: React.FC<DashboardControllerProps> = ({
  dashboardId,
  userId
}) => {
  const [layouts, setLayouts] = useState<any>({});
  const [widgets, setWidgets] = useState<any[]>([]);
  const [isWidgetSelectorOpen, setIsWidgetSelectorOpen] = useState<boolean>(false);
  const [selectedWidgetId, setSelectedWidgetId] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  
  // Load dashboard configuration
  useEffect(() => {
    const loadDashboard = async () => {
      try {
        setIsLoading(true);
        const dashboard = await DashboardService.getDashboard(dashboardId);
        
        if (dashboard) {
          setLayouts(dashboard.layouts || {});
          
          // Load widget instances
          const widgetInstances = dashboard.widgets.map((widget: any) => {
            const definition = WidgetRegistry.getWidget(widget.type);
            
            if (!definition) {
              console.warn(`Widget type ${widget.type} not found`);
              return null;
            }
            
            return {
              id: widget.id,
              type: widget.type,
              title: widget.title || definition.name,
              config: { ...definition.defaultConfig, ...widget.config },
              component: definition.component
            };
          }).filter(Boolean);
          
          setWidgets(widgetInstances);
        }
      } catch (err) {
        console.error('Failed to load dashboard:', err);
        setError('Failed to load dashboard. Please try again later.');
      } finally {
        setIsLoading(false);
      }
    };
    
    loadDashboard();
  }, [dashboardId]);
  
  // Save dashboard configuration
  const saveDashboard = useCallback(async () => {
    try {
      await DashboardService.saveDashboard(dashboardId, {
        id: dashboardId,
        userId,
        layouts,
        widgets: widgets.map(widget => ({
          id: widget.id,
          type: widget.type,
          title: widget.title,
          config: widget.config
        }))
      });
    } catch (err) {
      console.error('Failed to save dashboard:', err);
      setError('Failed to save dashboard. Please try again later.');
    }
  }, [dashboardId, userId, layouts, widgets]);
  
  // Auto-save when layouts or widgets change
  useEffect(() => {
    if (!isLoading) {
      const timeoutId = setTimeout(() => {
        saveDashboard();
      }, 1000);
      
      return () => clearTimeout(timeoutId);
    }
  }, [layouts, widgets, isLoading, saveDashboard]);
  
  // Handle layout change
  const handleLayoutChange = useCallback((currentLayout: any, allLayouts: any) => {
    setLayouts(allLayouts);
  }, []);
  
  // Add widget
  const handleAddWidget = useCallback((widgetType: string) => {
    const definition = WidgetRegistry.getWidget(widgetType);
    
    if (!definition) {
      console.error(`Widget type ${widgetType} not found`);
      return;
    }
    
    const widgetId = uuidv4();
    
    // Add widget to state
    setWidgets(prevWidgets => [
      ...prevWidgets,
      {
        id: widgetId,
        type: widgetType,
        title: definition.name,
        config: { ...definition.defaultConfig },
        component: definition.component
      }
    ]);
    
    // Add widget to layout
    const { w, h, minW, maxW, minH, maxH } = definition.defaultSize;
    
    setLayouts(prevLayouts => {
      const newLayouts = { ...prevLayouts };
      
      // Add widget to each breakpoint layout
      Object.keys(newLayouts).forEach(breakpoint => {
        const layout = newLayouts[breakpoint];
        
        // Find position for new widget
        let maxY = 0;
        layout.forEach((item: any) => {
          maxY = Math.max(maxY, item.y + item.h);
        });
        
        // Add widget to layout
        newLayouts[breakpoint] = [
          ...layout,
          {
            i: widgetId,
            x: 0,
            y: maxY,
            w,
            h,
            minW,
            maxW,
            minH,
            maxH
          }
        ];
      });
      
      return newLayouts;
    });
    
    // Close widget selector
    setIsWidgetSelectorOpen(false);
  }, []);
  
  // Remove widget
  const handleRemoveWidget = useCallback((widgetId: string) => {
    // Remove widget from state
    setWidgets(prevWidgets => prevWidgets.filter(widget => widget.id !== widgetId));
    
    // Remove widget from layout
    setLayouts(prevLayouts => {
      const newLayouts = { ...prevLayouts };
      
      // Remove widget from each breakpoint layout
      Object.keys(newLayouts).forEach(breakpoint => {
        newLayouts[breakpoint] = newLayouts[breakpoint].filter(
          (item: any) => item.i !== widgetId
        );
      });
      
      return newLayouts;
    });
  }, []);
  
  // Open widget settings
  const handleWidgetSettings = useCallback((widgetId: string) => {
    setSelectedWidgetId(widgetId);
  }, []);
  
  // Update widget settings
  const handleUpdateWidgetSettings = useCallback((widgetId: string, updates: any) => {
    setWidgets(prevWidgets => prevWidgets.map(widget => {
      if (widget.id === widgetId) {
        return {
          ...widget,
          ...updates
        };
      }
      
      return widget;
    }));
    
    setSelectedWidgetId(null);
  }, []);
  
  if (isLoading) {
    return <div>Loading dashboard...</div>;
  }
  
  if (error) {
    return <div className="error">{error}</div>;
  }
  
  return (
    <div className="dashboard-controller">
      <div className="dashboard-header">
        <h2>Dashboard</h2>
        <button
          className="add-widget-button"
          onClick={() => setIsWidgetSelectorOpen(true)}
        >
          Add Widget
        </button>
      </div>
      
      <LayoutManager
        layouts={layouts}
        widgets={widgets}
        onLayoutChange={handleLayoutChange}
        onWidgetRemove={handleRemoveWidget}
        onWidgetSettings={handleWidgetSettings}
      />
      
      {isWidgetSelectorOpen && (
        <WidgetSelector
          onSelect={handleAddWidget}
          onClose={() => setIsWidgetSelectorOpen(false)}
        />
      )}
      
      {selectedWidgetId && (
        <WidgetSettings
          widget={widgets.find(w => w.id === selectedWidgetId)}
          onUpdate={(updates) => handleUpdateWidgetSettings(selectedWidgetId, updates)}
          onClose={() => setSelectedWidgetId(null)}
        />
      )}
    </div>
  );
};

export default DashboardController;
```
