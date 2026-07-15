# MCP Platform Visualization Guide

This guide provides information on the visualization capabilities of the MCP Platform, focusing on the 3D agent network visualization.

## Overview

The MCP Platform includes rich visualization features for agent networks, with both 2D and 3D representations. These visualizations help users understand the relationships and interactions between agents in the system.

## 3D Network Visualization

The 3D network visualization uses Three.js to create an interactive, immersive representation of the agent network.

### Key Features

- **Interactive 3D Graph**: Navigate through a 3D space to explore agent relationships
- **Animated Edges**: Edges between nodes animate to show data transmission
- **Visual Indicators**: Different colors and shapes represent different agent types and states
- **Zoom and Pan**: Interact with the visualization to focus on specific areas
- **Filtering**: Filter the visualization by agent type, status, or relationship

### Implementation

The 3D visualization is implemented in the `NetworkVisualization` component:

```typescript
import React, { useRef, useEffect } from 'react';
import * as THREE from 'three';
import { OrbitControls } from 'three/examples/jsm/controls/OrbitControls';
import { CSS2DRenderer, CSS2DObject } from 'three/examples/jsm/renderers/CSS2DRenderer';

interface NetworkVisualizationProps {
  agents: Agent[];
  connections: Connection[];
  onAgentClick?: (agentId: string) => void;
}

export function NetworkVisualization({ agents, connections, onAgentClick }: NetworkVisualizationProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  
  useEffect(() => {
    if (!containerRef.current) return;
    
    // Initialize Three.js scene
    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x1f2022); // Dark background
    
    // Initialize camera
    const camera = new THREE.PerspectiveCamera(
      75,
      containerRef.current.clientWidth / containerRef.current.clientHeight,
      0.1,
      1000
    );
    camera.position.z = 15;
    
    // Initialize renderer
    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(containerRef.current.clientWidth, containerRef.current.clientHeight);
    containerRef.current.appendChild(renderer.domElement);
    
    // Initialize CSS2D renderer for labels
    const labelRenderer = new CSS2DRenderer();
    labelRenderer.setSize(containerRef.current.clientWidth, containerRef.current.clientHeight);
    labelRenderer.domElement.style.position = 'absolute';
    labelRenderer.domElement.style.top = '0';
    containerRef.current.appendChild(labelRenderer.domElement);
    
    // Initialize controls
    const controls = new OrbitControls(camera, labelRenderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.05;
    
    // Add lighting
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
    scene.add(ambientLight);
    
    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.5);
    directionalLight.position.set(0, 1, 1);
    scene.add(directionalLight);
    
    // Create agent nodes
    const nodes: { [id: string]: THREE.Mesh } = {};
    
    agents.forEach((agent) => {
      // Create sphere for agent
      const geometry = new THREE.SphereGeometry(1, 32, 32);
      const material = new THREE.MeshStandardMaterial({
        color: getAgentColor(agent.type),
        metalness: 0.3,
        roughness: 0.4,
      });
      
      const mesh = new THREE.Mesh(geometry, material);
      
      // Position randomly in 3D space
      mesh.position.x = (Math.random() - 0.5) * 20;
      mesh.position.y = (Math.random() - 0.5) * 20;
      mesh.position.z = (Math.random() - 0.5) * 20;
      
      scene.add(mesh);
      nodes[agent.id] = mesh;
      
      // Add label
      const div = document.createElement('div');
      div.className = 'agent-label';
      div.textContent = agent.name || agent.id.substring(0, 8);
      div.style.backgroundColor = 'rgba(0, 0, 0, 0.6)';
      div.style.color = '#ffffff';
      div.style.padding = '2px 5px';
      div.style.borderRadius = '3px';
      div.style.fontSize = '12px';
      div.style.pointerEvents = 'none';
      
      const label = new CSS2DObject(div);
      label.position.set(0, 1.2, 0);
      mesh.add(label);
      
      // Add click handler
      if (onAgentClick) {
        const clickableMesh = new THREE.Mesh(
          new THREE.SphereGeometry(1.2, 32, 32),
          new THREE.MeshBasicMaterial({ transparent: true, opacity: 0 })
        );
        clickableMesh.position.copy(mesh.position);
        clickableMesh.userData = { agentId: agent.id };
        scene.add(clickableMesh);
      }
    });
    
    // Create connections between agents
    const lines: THREE.Line[] = [];
    
    connections.forEach((connection) => {
      const sourceNode = nodes[connection.source];
      const targetNode = nodes[connection.target];
      
      if (sourceNode && targetNode) {
        const material = new THREE.LineBasicMaterial({
          color: 0x7ebab5,
          transparent: true,
          opacity: 0.6,
        });
        
        const points = [sourceNode.position, targetNode.position];
        const geometry = new THREE.BufferGeometry().setFromPoints(points);
        
        const line = new THREE.Line(geometry, material);
        scene.add(line);
        lines.push(line);
      }
    });
    
    // Handle click events
    if (onAgentClick) {
      const raycaster = new THREE.Raycaster();
      const mouse = new THREE.Vector2();
      
      const onClick = (event: MouseEvent) => {
        const rect = containerRef.current!.getBoundingClientRect();
        mouse.x = ((event.clientX - rect.left) / containerRef.current!.clientWidth) * 2 - 1;
        mouse.y = -((event.clientY - rect.top) / containerRef.current!.clientHeight) * 2 + 1;
        
        raycaster.setFromCamera(mouse, camera);
        const intersects = raycaster.intersectObjects(scene.children);
        
        if (intersects.length > 0) {
          const object = intersects[0].object;
          if (object.userData && object.userData.agentId) {
            onAgentClick(object.userData.agentId);
          }
        }
      };
      
      containerRef.current.addEventListener('click', onClick);
    }
    
    // Animation loop
    const animate = () => {
      requestAnimationFrame(animate);
      
      // Update controls
      controls.update();
      
      // Render scene
      renderer.render(scene, camera);
      labelRenderer.render(scene, camera);
    };
    
    animate();
    
    // Handle window resize
    const handleResize = () => {
      if (!containerRef.current) return;
      
      camera.aspect = containerRef.current.clientWidth / containerRef.current.clientHeight;
      camera.updateProjectionMatrix();
      
      renderer.setSize(containerRef.current.clientWidth, containerRef.current.clientHeight);
      labelRenderer.setSize(containerRef.current.clientWidth, containerRef.current.clientHeight);
    };
    
    window.addEventListener('resize', handleResize);
    
    // Cleanup
    return () => {
      if (containerRef.current) {
        containerRef.current.removeChild(renderer.domElement);
        containerRef.current.removeChild(labelRenderer.domElement);
        
        if (onAgentClick) {
          containerRef.current.removeEventListener('click', onClick);
        }
      }
      
      window.removeEventListener('resize', handleResize);
    };
  }, [agents, connections, onAgentClick]);
  
  return <div ref={containerRef} style={{ width: '100%', height: '100%' }} />;
}

// Helper function to get color based on agent type
function getAgentColor(type: string): number {
  switch (type) {
    case 'jarvis':
      return 0x7ebab5; // Primary teal color
    case 'research':
      return 0x6a8caf; // Blue
    case 'writing':
      return 0xaf6a8c; // Purple
    case 'qa':
      return 0x8caf6a; // Green
    default:
      return 0xcccccc; // Gray
  }
}

interface Agent {
  id: string;
  name?: string;
  type: string;
}

interface Connection {
  source: string;
  target: string;
  type?: string;
}
```

### Usage

To use the 3D network visualization in a page:

```tsx
import { NetworkVisualization } from '@/components/visualization/NetworkVisualization';
import { useApp } from '@/contexts/AppContext';

export function NetworkPage() {
  const { agents } = useApp();
  
  // Generate connections based on agent interactions
  const connections = generateConnections(agents);
  
  const handleAgentClick = (agentId: string) => {
    console.log(`Agent clicked: ${agentId}`);
    // Navigate to agent details or open agent chat
  };
  
  return (
    <div className="h-screen">
      <NetworkVisualization
        agents={agents}
        connections={connections}
        onAgentClick={handleAgentClick}
      />
    </div>
  );
}

// Helper function to generate connections
function generateConnections(agents) {
  // In a real application, this would use actual interaction data
  const connections = [];
  
  // For demo purposes, connect some agents randomly
  for (let i = 0; i < agents.length; i++) {
    for (let j = i + 1; j < agents.length; j++) {
      if (Math.random() > 0.7) {
        connections.push({
          source: agents[i].id,
          target: agents[j].id,
        });
      }
    }
  }
  
  return connections;
}
```

## 2D Network Visualization

For simpler representations or lower-performance environments, the platform also includes a 2D network visualization using D3.js.

### Key Features

- **Force-Directed Graph**: Automatically arranges nodes based on connections
- **Interactive Elements**: Drag nodes, hover for details, click for actions
- **Responsive Design**: Adapts to different screen sizes
- **Customizable Appearance**: Configure colors, sizes, and styles

### Implementation

The 2D visualization is implemented in the `NetworkGraph2D` component:

```typescript
import React, { useRef, useEffect } from 'react';
import * as d3 from 'd3';

interface NetworkGraph2DProps {
  agents: Agent[];
  connections: Connection[];
  onAgentClick?: (agentId: string) => void;
}

export function NetworkGraph2D({ agents, connections, onAgentClick }: NetworkGraph2DProps) {
  const svgRef = useRef<SVGSVGElement>(null);
  
  useEffect(() => {
    if (!svgRef.current) return;
    
    // Clear previous graph
    d3.select(svgRef.current).selectAll('*').remove();
    
    // Set up SVG
    const svg = d3.select(svgRef.current);
    const width = svgRef.current.clientWidth;
    const height = svgRef.current.clientHeight;
    
    // Create simulation
    const simulation = d3.forceSimulation(agents)
      .force('link', d3.forceLink(connections).id((d: any) => d.id))
      .force('charge', d3.forceManyBody().strength(-300))
      .force('center', d3.forceCenter(width / 2, height / 2));
    
    // Create links
    const link = svg.append('g')
      .selectAll('line')
      .data(connections)
      .enter()
      .append('line')
      .attr('stroke', '#7ebab5')
      .attr('stroke-opacity', 0.6)
      .attr('stroke-width', 2);
    
    // Create nodes
    const node = svg.append('g')
      .selectAll('circle')
      .data(agents)
      .enter()
      .append('circle')
      .attr('r', 10)
      .attr('fill', (d) => getAgentColor(d.type))
      .call(d3.drag()
        .on('start', dragstarted)
        .on('drag', dragged)
        .on('end', dragended));
    
    // Add labels
    const label = svg.append('g')
      .selectAll('text')
      .data(agents)
      .enter()
      .append('text')
      .text((d) => d.name || d.id.substring(0, 8))
      .attr('font-size', 12)
      .attr('dx', 15)
      .attr('dy', 4)
      .attr('fill', '#ffffff');
    
    // Add click handler
    if (onAgentClick) {
      node.on('click', (event, d) => {
        onAgentClick(d.id);
      });
    }
    
    // Update positions on simulation tick
    simulation.on('tick', () => {
      link
        .attr('x1', (d: any) => d.source.x)
        .attr('y1', (d: any) => d.source.y)
        .attr('x2', (d: any) => d.target.x)
        .attr('y2', (d: any) => d.target.y);
      
      node
        .attr('cx', (d: any) => d.x)
        .attr('cy', (d: any) => d.y);
      
      label
        .attr('x', (d: any) => d.x)
        .attr('y', (d: any) => d.y);
    });
    
    // Drag functions
    function dragstarted(event: any) {
      if (!event.active) simulation.alphaTarget(0.3).restart();
      event.subject.fx = event.subject.x;
      event.subject.fy = event.subject.y;
    }
    
    function dragged(event: any) {
      event.subject.fx = event.x;
      event.subject.fy = event.y;
    }
    
    function dragended(event: any) {
      if (!event.active) simulation.alphaTarget(0);
      event.subject.fx = null;
      event.subject.fy = null;
    }
    
    // Cleanup
    return () => {
      simulation.stop();
    };
  }, [agents, connections, onAgentClick]);
  
  return <svg ref={svgRef} width="100%" height="100%" />;
}

// Helper function to get color based on agent type
function getAgentColor(type: string): string {
  switch (type) {
    case 'jarvis':
      return '#7ebab5'; // Primary teal color
    case 'research':
      return '#6a8caf'; // Blue
    case 'writing':
      return '#af6a8c'; // Purple
    case 'qa':
      return '#8caf6a'; // Green
    default:
      return '#cccccc'; // Gray
  }
}

interface Agent {
  id: string;
  name?: string;
  type: string;
}

interface Connection {
  source: string;
  target: string;
  type?: string;
}
```

## Data Visualization

In addition to network visualizations, the MCP Platform includes various data visualizations for analytics:

### Agent Activity Timeline

```tsx
import React from 'react';
import { Line } from 'react-chartjs-2';

interface ActivityTimelineProps {
  data: {
    labels: string[];
    datasets: {
      label: string;
      data: number[];
      borderColor: string;
      backgroundColor: string;
    }[];
  };
}

export function ActivityTimeline({ data }: ActivityTimelineProps) {
  const options = {
    responsive: true,
    plugins: {
      legend: {
        position: 'top' as const,
      },
      title: {
        display: true,
        text: 'Agent Activity Timeline',
      },
    },
    scales: {
      y: {
        beginAtZero: true,
      },
    },
  };
  
  return <Line data={data} options={options} />;
}
```

### Resource Usage Gauge

```tsx
import React from 'react';
import { Doughnut } from 'react-chartjs-2';

interface ResourceGaugeProps {
  label: string;
  value: number;
  max: number;
  color: string;
}

export function ResourceGauge({ label, value, max, color }: ResourceGaugeProps) {
  const data = {
    labels: ['Used', 'Available'],
    datasets: [
      {
        data: [value, max - value],
        backgroundColor: [color, '#2a2a2a'],
        borderWidth: 0,
      },
    ],
  };
  
  const options = {
    responsive: true,
    cutout: '70%',
    plugins: {
      legend: {
        display: false,
      },
      title: {
        display: true,
        text: label,
      },
    },
  };
  
  return (
    <div className="relative">
      <Doughnut data={data} options={options} />
      <div className="absolute inset-0 flex items-center justify-center">
        <div className="text-2xl font-bold">{Math.round((value / max) * 100)}%</div>
      </div>
    </div>
  );
}
```

## Best Practices

When implementing visualizations in the MCP Platform, follow these best practices:

1. **Performance Optimization**
   - Use WebGL for 3D visualizations
   - Implement level-of-detail rendering for large networks
   - Use requestAnimationFrame for smooth animations

2. **Accessibility**
   - Provide alternative text representations of visualizations
   - Ensure sufficient color contrast
   - Support keyboard navigation

3. **Responsive Design**
   - Adapt visualizations to different screen sizes
   - Provide simplified views for mobile devices
   - Handle window resize events

4. **User Interaction**
   - Provide clear visual feedback for interactive elements
   - Implement intuitive navigation controls
   - Include tooltips and help information

5. **Data Updates**
   - Handle real-time data updates efficiently
   - Animate transitions between states
   - Provide loading indicators for data fetching
