# Network Visualization Implementation - Part 1

This document outlines the implementation plan for the network visualization component in the MCP dashboard.

## Overview

The network visualization component will provide:
- 3D visualization of agent networks using Three.js
- Interactive graph with agent nodes and connection edges
- Visual indicators for data transmission
- Animated edges between interacting agents
- Filtering and search capabilities
- Detailed agent information display

## Implementation Details

### 1. Base Network Graph Component

Create a base network graph component using Three.js:

```typescript
// src/components/network/NetworkGraph.tsx
"use client";

import React, { useRef, useEffect, useState } from "react";
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
import { CSS2DRenderer, CSS2DObject } from "three/examples/jsm/renderers/CSS2DRenderer";
import { Agent } from "@/contexts/AppContext";

// Define types for our data
export interface Connection {
  source: string;
  target: string;
  strength: number;
  lastActive: number;
  type?: string;
}

// Props for the NetworkGraph component
interface NetworkGraphProps {
  agents: Agent[];
  connections: Connection[];
  height?: string;
  width?: string;
  showControls?: boolean;
  showGizmo?: boolean;
  onAgentSelect?: (agentId: string | null) => void;
}

const NetworkGraph: React.FC<NetworkGraphProps> = ({
  agents,
  connections,
  height = "500px",
  width = "100%",
  showControls = true,
  showGizmo = true,
  onAgentSelect,
}) => {
  // Refs
  const containerRef = useRef<HTMLDivElement>(null);
  const rendererRef = useRef<THREE.WebGLRenderer | null>(null);
  const labelRendererRef = useRef<CSS2DRenderer | null>(null);
  const sceneRef = useRef<THREE.Scene | null>(null);
  const cameraRef = useRef<THREE.PerspectiveCamera | null>(null);
  const controlsRef = useRef<OrbitControls | null>(null);
  const nodesRef = useRef<Map<string, THREE.Mesh>>(new Map());
  const edgesRef = useRef<Map<string, THREE.Line>>(new Map());
  const labelsRef = useRef<Map<string, CSS2DObject>>(new Map());
  
  // State
  const [selectedAgent, setSelectedAgent] = useState<string | null>(null);
  const [isInitialized, setIsInitialized] = useState(false);
  
  // Initialize Three.js scene
  useEffect(() => {
    if (!containerRef.current) return;
    
    // Create scene
    const scene = new THREE.Scene();
    scene.background = new THREE.Color("#1f2022");
    sceneRef.current = scene;
    
    // Create camera
    const camera = new THREE.PerspectiveCamera(
      75,
      containerRef.current.clientWidth / containerRef.current.clientHeight,
      0.1,
      1000
    );
    camera.position.z = 15;
    cameraRef.current = camera;
    
    // Create renderer
    const renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(containerRef.current.clientWidth, containerRef.current.clientHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    containerRef.current.appendChild(renderer.domElement);
    rendererRef.current = renderer;
    
    // Create label renderer
    const labelRenderer = new CSS2DRenderer();
    labelRenderer.setSize(containerRef.current.clientWidth, containerRef.current.clientHeight);
    labelRenderer.domElement.style.position = "absolute";
    labelRenderer.domElement.style.top = "0";
    labelRenderer.domElement.style.pointerEvents = "none";
    containerRef.current.appendChild(labelRenderer.domElement);
    labelRendererRef.current = labelRenderer;
    
    // Create controls
    if (showControls) {
      const controls = new OrbitControls(camera, renderer.domElement);
      controls.enableDamping = true;
      controls.dampingFactor = 0.05;
      controls.rotateSpeed = 0.5;
      controlsRef.current = controls;
    }
    
    // Add ambient light
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.5);
    scene.add(ambientLight);
    
    // Add directional light
    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(0, 10, 10);
    scene.add(directionalLight);
    
    // Add coordinate axes helper
    if (showGizmo) {
      const axesHelper = new THREE.AxesHelper(5);
      scene.add(axesHelper);
    }
    
    // Animation loop
    const animate = () => {
      requestAnimationFrame(animate);
      
      if (controlsRef.current) {
        controlsRef.current.update();
      }
      
      if (rendererRef.current && sceneRef.current && cameraRef.current) {
        rendererRef.current.render(sceneRef.current, cameraRef.current);
      }
      
      if (labelRendererRef.current && sceneRef.current && cameraRef.current) {
        labelRendererRef.current.render(sceneRef.current, cameraRef.current);
      }
    };
    
    animate();
    setIsInitialized(true);
    
    // Handle window resize
    const handleResize = () => {
      if (
        !containerRef.current ||
        !cameraRef.current ||
        !rendererRef.current ||
        !labelRendererRef.current
      )
        return;
      
      const width = containerRef.current.clientWidth;
      const height = containerRef.current.clientHeight;
      
      cameraRef.current.aspect = width / height;
      cameraRef.current.updateProjectionMatrix();
      
      rendererRef.current.setSize(width, height);
      labelRendererRef.current.setSize(width, height);
    };
    
    window.addEventListener("resize", handleResize);
    
    // Cleanup
    return () => {
      window.removeEventListener("resize", handleResize);
      
      if (rendererRef.current && containerRef.current) {
        containerRef.current.removeChild(rendererRef.current.domElement);
      }
      
      if (labelRendererRef.current && containerRef.current) {
        containerRef.current.removeChild(labelRendererRef.current.domElement);
      }
      
      if (controlsRef.current) {
        controlsRef.current.dispose();
      }
    };
  }, [showControls, showGizmo]);
  
  // Update nodes and edges when agents or connections change
  useEffect(() => {
    if (!isInitialized || !sceneRef.current) return;
    
    // Clear existing nodes and edges
    nodesRef.current.forEach((node) => {
      sceneRef.current?.remove(node);
    });
    
    edgesRef.current.forEach((edge) => {
      sceneRef.current?.remove(edge);
    });
    
    labelsRef.current.forEach((label) => {
      sceneRef.current?.remove(label);
    });
    
    nodesRef.current.clear();
    edgesRef.current.clear();
    labelsRef.current.clear();
    
    // Create nodes for agents
    const nodePositions = new Map<string, THREE.Vector3>();
    
    // Calculate positions using force-directed layout
    const positions = calculateNodePositions(agents, connections);
    
    // Create nodes
    agents.forEach((agent, index) => {
      const position = positions.get(agent.id) || new THREE.Vector3(
        (Math.random() - 0.5) * 20,
        (Math.random() - 0.5) * 20,
        (Math.random() - 0.5) * 20
      );
      
      nodePositions.set(agent.id, position);
      
      // Create node geometry
      const geometry = new THREE.SphereGeometry(0.5, 32, 32);
      
      // Create node material based on agent status
      const color = getAgentStatusColor(agent.status);
      const material = new THREE.MeshStandardMaterial({
        color,
        metalness: 0.3,
        roughness: 0.4,
        emissive: color,
        emissiveIntensity: 0.2,
      });
      
      // Create node mesh
      const mesh = new THREE.Mesh(geometry, material);
      mesh.position.copy(position);
      mesh.userData = { id: agent.id, type: "agent" };
      
      // Add to scene
      sceneRef.current?.add(mesh);
      nodesRef.current.set(agent.id, mesh);
      
      // Create label
      const labelDiv = document.createElement("div");
      labelDiv.className = "agent-label";
      labelDiv.textContent = agent.name;
      labelDiv.style.color = "#f6f5f5";
      labelDiv.style.fontSize = "12px";
      labelDiv.style.padding = "2px 5px";
      labelDiv.style.backgroundColor = "rgba(31, 32, 34, 0.7)";
      labelDiv.style.borderRadius = "4px";
      labelDiv.style.pointerEvents = "none";
      
      const label = new CSS2DObject(labelDiv);
      label.position.copy(position);
      label.position.y += 0.7;
      
      sceneRef.current?.add(label);
      labelsRef.current.set(agent.id, label);
    });
    
    // Create edges for connections
    connections.forEach((connection) => {
      const sourcePosition = nodePositions.get(connection.source);
      const targetPosition = nodePositions.get(connection.target);
      
      if (!sourcePosition || !targetPosition) return;
      
      // Create edge geometry
      const points = [sourcePosition, targetPosition];
      const geometry = new THREE.BufferGeometry().setFromPoints(points);
      
      // Create edge material based on connection strength and activity
      const color = getConnectionColor(connection);
      const material = new THREE.LineBasicMaterial({
        color,
        transparent: true,
        opacity: getConnectionOpacity(connection),
        linewidth: 1,
      });
      
      // Create edge line
      const line = new THREE.Line(geometry, material);
      line.userData = {
        id: `${connection.source}-${connection.target}`,
        type: "connection",
        connection,
      };
      
      // Add to scene
      sceneRef.current?.add(line);
      edgesRef.current.set(
        `${connection.source}-${connection.target}`,
        line
      );
    });
    
    // Update selected agent
    if (selectedAgent) {
      const node = nodesRef.current.get(selectedAgent);
      if (node) {
        const material = node.material as THREE.MeshStandardMaterial;
        material.emissiveIntensity = 0.5;
        node.scale.set(1.2, 1.2, 1.2);
      }
    }
  }, [agents, connections, isInitialized, selectedAgent]);
  
  // Handle click events
  useEffect(() => {
    if (!isInitialized || !containerRef.current) return;
    
    const raycaster = new THREE.Raycaster();
    const mouse = new THREE.Vector2();
    
    const handleClick = (event: MouseEvent) => {
      if (
        !containerRef.current ||
        !cameraRef.current ||
        !sceneRef.current
      )
        return;
      
      // Calculate mouse position in normalized device coordinates
      const rect = containerRef.current.getBoundingClientRect();
      mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
      mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;
      
      // Update the picking ray with the camera and mouse position
      raycaster.setFromCamera(mouse, cameraRef.current);
      
      // Calculate objects intersecting the picking ray
      const intersects = raycaster.intersectObjects(
        Array.from(nodesRef.current.values())
      );
      
      if (intersects.length > 0) {
        const object = intersects[0].object;
        const agentId = object.userData.id as string;
        
        // Reset previous selection
        if (selectedAgent && selectedAgent !== agentId) {
          const prevNode = nodesRef.current.get(selectedAgent);
          if (prevNode) {
            const material = prevNode.material as THREE.MeshStandardMaterial;
            material.emissiveIntensity = 0.2;
            prevNode.scale.set(1, 1, 1);
          }
        }
        
        // Update selection
        if (selectedAgent !== agentId) {
          setSelectedAgent(agentId);
          if (onAgentSelect) onAgentSelect(agentId);
          
          const node = nodesRef.current.get(agentId);
          if (node) {
            const material = node.material as THREE.MeshStandardMaterial;
            material.emissiveIntensity = 0.5;
            node.scale.set(1.2, 1.2, 1.2);
          }
        } else {
          setSelectedAgent(null);
          if (onAgentSelect) onAgentSelect(null);
          
          const node = nodesRef.current.get(agentId);
          if (node) {
            const material = node.material as THREE.MeshStandardMaterial;
            material.emissiveIntensity = 0.2;
            node.scale.set(1, 1, 1);
          }
        }
      } else if (selectedAgent) {
        // Deselect if clicking empty space
        const node = nodesRef.current.get(selectedAgent);
        if (node) {
          const material = node.material as THREE.MeshStandardMaterial;
          material.emissiveIntensity = 0.2;
          node.scale.set(1, 1, 1);
        }
        
        setSelectedAgent(null);
        if (onAgentSelect) onAgentSelect(null);
      }
    };
    
    containerRef.current.addEventListener("click", handleClick);
    
    return () => {
      containerRef.current?.removeEventListener("click", handleClick);
    };
  }, [isInitialized, onAgentSelect, selectedAgent]);
  
  return (
    <div
      ref={containerRef}
      style={{
        height,
        width,
        position: "relative",
        overflow: "hidden",
      }}
    />
  );
};

// Helper function to get color based on agent status
const getAgentStatusColor = (status: string): number => {
  switch (status) {
    case "online":
      return 0x7ebab5; // Primary color
    case "offline":
      return 0x4a5568; // Muted color
    case "error":
      return 0xe53e3e; // Destructive color
    default:
      return 0x7ebab5; // Default to primary
  }
};

// Helper function to get color based on connection
const getConnectionColor = (connection: Connection): number => {
  // Base color on connection type if available
  if (connection.type === "command") return 0x805ad5; // Accent color
  if (connection.type === "data") return 0x7ebab5; // Primary color
  
  // Otherwise base on strength
  if (connection.strength > 0.7) return 0x7ebab5; // Primary color
  if (connection.strength > 0.3) return 0xa3d0cc; // Primary light
  return 0x4a5568; // Muted color
};

// Helper function to get opacity based on connection activity
const getConnectionOpacity = (connection: Connection): number => {
  const timeSinceActive = Date.now() - connection.lastActive;
  const hourInMs = 3600000;
  
  if (timeSinceActive < hourInMs) {
    return Math.max(0.2, 1 - timeSinceActive / hourInMs);
  }
  
  return 0.2;
};

// Helper function to calculate node positions using force-directed layout
const calculateNodePositions = (
  agents: Agent[],
  connections: Connection[]
): Map<string, THREE.Vector3> => {
  const positions = new Map<string, THREE.Vector3>();
  
  // Initialize random positions
  agents.forEach((agent) => {
    positions.set(
      agent.id,
      new THREE.Vector3(
        (Math.random() - 0.5) * 20,
        (Math.random() - 0.5) * 20,
        (Math.random() - 0.5) * 20
      )
    );
  });
  
  // Simple force-directed layout
  const iterations = 50;
  const repulsionForce = 1;
  const attractionForce = 0.05;
  
  for (let i = 0; i < iterations; i++) {
    // Calculate repulsion forces between all nodes
    agents.forEach((agent1) => {
      const pos1 = positions.get(agent1.id)!;
      const force = new THREE.Vector3();
      
      agents.forEach((agent2) => {
        if (agent1.id === agent2.id) return;
        
        const pos2 = positions.get(agent2.id)!;
        const diff = new THREE.Vector3().subVectors(pos1, pos2);
        const distance = diff.length();
        
        if (distance < 0.1) return; // Avoid division by zero
        
        const repulsion = repulsionForce / (distance * distance);
        force.add(diff.normalize().multiplyScalar(repulsion));
      });
      
      pos1.add(force.multiplyScalar(0.1));
    });
    
    // Calculate attraction forces along edges
    connections.forEach((connection) => {
      const pos1 = positions.get(connection.source);
      const pos2 = positions.get(connection.target);
      
      if (!pos1 || !pos2) return;
      
      const diff = new THREE.Vector3().subVectors(pos2, pos1);
      const distance = diff.length();
      const attraction = distance * attractionForce * connection.strength;
      
      const force = diff.normalize().multiplyScalar(attraction);
      pos1.add(force.clone().multiplyScalar(0.5));
      pos2.sub(force.clone().multiplyScalar(0.5));
    });
  }
  
  return positions;
};

export default NetworkGraph;
```

This implementation provides a basic 3D network graph visualization using Three.js. The component renders agent nodes as spheres and connections as lines, with colors and visual properties based on agent status and connection strength.

In the next part, we'll enhance this component with additional features like animated edges, particle effects, and improved interaction.
