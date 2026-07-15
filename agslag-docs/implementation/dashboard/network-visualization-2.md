# Network Visualization Implementation - Part 2

This document continues the implementation plan for the network visualization component, focusing on enhanced visual effects and interactions.

## Enhanced Network Graph Component

Extend the base network graph with advanced visual effects:

```typescript
// src/components/network/EnhancedNetworkGraph.tsx
"use client";

import React, { useRef, useEffect, useState } from "react";
import * as THREE from "three";
import { OrbitControls } from "three/examples/jsm/controls/OrbitControls";
import { CSS2DRenderer, CSS2DObject } from "three/examples/jsm/renderers/CSS2DRenderer";
import { EffectComposer } from "three/examples/jsm/postprocessing/EffectComposer";
import { RenderPass } from "three/examples/jsm/postprocessing/RenderPass";
import { UnrealBloomPass } from "three/examples/jsm/postprocessing/UnrealBloomPass";
import { OutlinePass } from "three/examples/jsm/postprocessing/OutlinePass";
import { Agent } from "@/contexts/AppContext";
import NetworkGraph, { Connection } from "./NetworkGraph";

// Props for the EnhancedNetworkGraph component
interface EnhancedNetworkGraphProps {
  agents: Agent[];
  connections: Connection[];
  height?: string;
  width?: string;
  showControls?: boolean;
  showGizmo?: boolean;
  onAgentSelect?: (agentId: string | null) => void;
}

const EnhancedNetworkGraph: React.FC<EnhancedNetworkGraphProps> = (props) => {
  // Container ref
  const containerRef = useRef<HTMLDivElement>(null);
  
  // Refs for Three.js objects
  const rendererRef = useRef<THREE.WebGLRenderer | null>(null);
  const composerRef = useRef<EffectComposer | null>(null);
  const sceneRef = useRef<THREE.Scene | null>(null);
  const cameraRef = useRef<THREE.PerspectiveCamera | null>(null);
  const controlsRef = useRef<OrbitControls | null>(null);
  const nodesRef = useRef<Map<string, THREE.Mesh>>(new Map());
  const edgesRef = useRef<Map<string, THREE.Line>>(new Map());
  const particlesRef = useRef<Map<string, THREE.Points>>(new Map());
  const outlinePassRef = useRef<OutlinePass | null>(null);
  
  // State
  const [selectedAgent, setSelectedAgent] = useState<string | null>(null);
  const [isInitialized, setIsInitialized] = useState(false);
  const [activeConnections, setActiveConnections] = useState<Set<string>>(new Set());
  
  // Initialize Three.js scene with enhanced effects
  useEffect(() => {
    if (!containerRef.current) return;
    
    // Create scene
    const scene = new THREE.Scene();
    scene.background = new THREE.Color("#1f2022");
    scene.fog = new THREE.FogExp2("#1f2022", 0.002);
    sceneRef.current = scene;
    
    // Create camera
    const camera = new THREE.PerspectiveCamera(
      60,
      containerRef.current.clientWidth / containerRef.current.clientHeight,
      0.1,
      1000
    );
    camera.position.z = 20;
    cameraRef.current = camera;
    
    // Create renderer with advanced settings
    const renderer = new THREE.WebGLRenderer({
      antialias: true,
      alpha: true,
      powerPreference: "high-performance",
    });
    renderer.setSize(containerRef.current.clientWidth, containerRef.current.clientHeight);
    renderer.setPixelRatio(window.devicePixelRatio);
    renderer.shadowMap.enabled = true;
    renderer.shadowMap.type = THREE.PCFSoftShadowMap;
    renderer.outputEncoding = THREE.sRGBEncoding;
    renderer.toneMapping = THREE.ACESFilmicToneMapping;
    renderer.toneMappingExposure = 1.0;
    containerRef.current.appendChild(renderer.domElement);
    rendererRef.current = renderer;
    
    // Create post-processing composer
    const composer = new EffectComposer(renderer);
    const renderPass = new RenderPass(scene, camera);
    composer.addPass(renderPass);
    
    // Add bloom effect
    const bloomPass = new UnrealBloomPass(
      new THREE.Vector2(
        containerRef.current.clientWidth,
        containerRef.current.clientHeight
      ),
      0.2, // strength
      0.2, // radius
      0.9  // threshold
    );
    composer.addPass(bloomPass);
    
    // Add outline effect
    const outlinePass = new OutlinePass(
      new THREE.Vector2(
        containerRef.current.clientWidth,
        containerRef.current.clientHeight
      ),
      scene,
      camera
    );
    outlinePass.edgeStrength = 3;
    outlinePass.edgeGlow = 0.5;
    outlinePass.edgeThickness = 1;
    outlinePass.pulsePeriod = 2;
    outlinePass.visibleEdgeColor.set("#7ebab5");
    outlinePass.hiddenEdgeColor.set("#7ebab5");
    composer.addPass(outlinePass);
    outlinePassRef.current = outlinePass;
    
    composerRef.current = composer;
    
    // Create controls
    if (props.showControls) {
      const controls = new OrbitControls(camera, renderer.domElement);
      controls.enableDamping = true;
      controls.dampingFactor = 0.05;
      controls.rotateSpeed = 0.5;
      controls.zoomSpeed = 0.7;
      controls.minDistance = 5;
      controls.maxDistance = 50;
      controlsRef.current = controls;
    }
    
    // Add ambient light
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.3);
    scene.add(ambientLight);
    
    // Add directional light
    const directionalLight = new THREE.DirectionalLight(0xffffff, 0.8);
    directionalLight.position.set(0, 10, 10);
    directionalLight.castShadow = true;
    directionalLight.shadow.mapSize.width = 1024;
    directionalLight.shadow.mapSize.height = 1024;
    scene.add(directionalLight);
    
    // Add point lights
    const pointLight1 = new THREE.PointLight(0x7ebab5, 0.5, 20);
    pointLight1.position.set(5, 5, 5);
    scene.add(pointLight1);
    
    const pointLight2 = new THREE.PointLight(0x805ad5, 0.5, 20);
    pointLight2.position.set(-5, -5, 5);
    scene.add(pointLight2);
    
    // Add coordinate axes helper
    if (props.showGizmo) {
      const axesHelper = new THREE.AxesHelper(5);
      scene.add(axesHelper);
    }
    
    // Add background particles
    const particleGeometry = new THREE.BufferGeometry();
    const particleCount = 1000;
    const particlePositions = new Float32Array(particleCount * 3);
    const particleSizes = new Float32Array(particleCount);
    
    for (let i = 0; i < particleCount; i++) {
      const i3 = i * 3;
      particlePositions[i3] = (Math.random() - 0.5) * 100;
      particlePositions[i3 + 1] = (Math.random() - 0.5) * 100;
      particlePositions[i3 + 2] = (Math.random() - 0.5) * 100;
      particleSizes[i] = Math.random() * 0.5 + 0.1;
    }
    
    particleGeometry.setAttribute(
      "position",
      new THREE.BufferAttribute(particlePositions, 3)
    );
    particleGeometry.setAttribute(
      "size",
      new THREE.BufferAttribute(particleSizes, 1)
    );
    
    const particleMaterial = new THREE.PointsMaterial({
      color: 0x7ebab5,
      size: 0.1,
      transparent: true,
      opacity: 0.2,
      blending: THREE.AdditiveBlending,
      sizeAttenuation: true,
    });
    
    const particles = new THREE.Points(particleGeometry, particleMaterial);
    scene.add(particles);
    
    // Animation loop
    const clock = new THREE.Clock();
    
    const animate = () => {
      requestAnimationFrame(animate);
      
      const delta = clock.getDelta();
      
      // Rotate background particles
      particles.rotation.x += 0.0001;
      particles.rotation.y += 0.0002;
      
      // Update active connection animations
      activeConnections.forEach((connectionId) => {
        const particle = particlesRef.current.get(connectionId);
        if (particle) {
          // Move particles along the connection
          const positions = particle.geometry.attributes.position.array as Float32Array;
          for (let i = 0; i < positions.length; i += 3) {
            positions[i + 2] += 0.05; // Move in z-direction
            
            // Reset position if it goes beyond the end
            if (positions[i + 2] > 1) {
              positions[i + 2] = 0;
            }
          }
          particle.geometry.attributes.position.needsUpdate = true;
        }
      });
      
      if (controlsRef.current) {
        controlsRef.current.update();
      }
      
      if (composerRef.current) {
        composerRef.current.render();
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
        !composerRef.current
      )
        return;
      
      const width = containerRef.current.clientWidth;
      const height = containerRef.current.clientHeight;
      
      cameraRef.current.aspect = width / height;
      cameraRef.current.updateProjectionMatrix();
      
      rendererRef.current.setSize(width, height);
      composerRef.current.setSize(width, height);
    };
    
    window.addEventListener("resize", handleResize);
    
    // Cleanup
    return () => {
      window.removeEventListener("resize", handleResize);
      
      if (rendererRef.current && containerRef.current) {
        containerRef.current.removeChild(rendererRef.current.domElement);
      }
      
      if (controlsRef.current) {
        controlsRef.current.dispose();
      }
    };
  }, [props.showControls, props.showGizmo, activeConnections]);
  
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
    
    particlesRef.current.forEach((particle) => {
      sceneRef.current?.remove(particle);
    });
    
    nodesRef.current.clear();
    edgesRef.current.clear();
    particlesRef.current.clear();
    
    // Create nodes for agents
    const nodePositions = new Map<string, THREE.Vector3>();
    
    // Calculate positions using force-directed layout
    const positions = calculateNodePositions(props.agents, props.connections);
    
    // Create nodes
    props.agents.forEach((agent) => {
      const position = positions.get(agent.id) || new THREE.Vector3(
        (Math.random() - 0.5) * 20,
        (Math.random() - 0.5) * 20,
        (Math.random() - 0.5) * 20
      );
      
      nodePositions.set(agent.id, position);
      
      // Create node geometry
      const geometry = new THREE.SphereGeometry(0.7, 32, 32);
      
      // Create node material based on agent status
      const color = getAgentStatusColor(agent.status);
      const material = new THREE.MeshStandardMaterial({
        color,
        metalness: 0.5,
        roughness: 0.2,
        emissive: color,
        emissiveIntensity: 0.3,
      });
      
      // Create node mesh
      const mesh = new THREE.Mesh(geometry, material);
      mesh.position.copy(position);
      mesh.castShadow = true;
      mesh.receiveShadow = true;
      mesh.userData = { id: agent.id, type: "agent", agent };
      
      // Add to scene
      sceneRef.current?.add(mesh);
      nodesRef.current.set(agent.id, mesh);
      
      // Add glow effect
      const glowGeometry = new THREE.SphereGeometry(1, 32, 32);
      const glowMaterial = new THREE.MeshBasicMaterial({
        color,
        transparent: true,
        opacity: 0.15,
        side: THREE.BackSide,
      });
      
      const glowMesh = new THREE.Mesh(glowGeometry, glowMaterial);
      glowMesh.position.copy(position);
      glowMesh.scale.set(1.5, 1.5, 1.5);
      sceneRef.current?.add(glowMesh);
    });
    
    // Create edges for connections
    props.connections.forEach((connection) => {
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
      
      // Add particle effect for active connections
      const timeSinceActive = Date.now() - connection.lastActive;
      if (timeSinceActive < 3600000) { // 1 hour
        // Create particles along the edge
        const particleCount = Math.ceil(connection.strength * 10) + 5;
        const particleGeometry = new THREE.BufferGeometry();
        const particlePositions = new Float32Array(particleCount * 3);
        const particleSizes = new Float32Array(particleCount);
        
        const direction = new THREE.Vector3().subVectors(
          targetPosition,
          sourcePosition
        );
        const length = direction.length();
        direction.normalize();
        
        for (let i = 0; i < particleCount; i++) {
          const t = Math.random(); // Position along the line (0-1)
          const i3 = i * 3;
          
          // Calculate position along the line
          const x = sourcePosition.x + direction.x * length * t;
          const y = sourcePosition.y + direction.y * length * t;
          const z = sourcePosition.z + direction.z * length * t;
          
          particlePositions[i3] = x;
          particlePositions[i3 + 1] = y;
          particlePositions[i3 + 2] = z;
          
          // Vary particle sizes
          particleSizes[i] = Math.random() * 0.2 + 0.1;
        }
        
        particleGeometry.setAttribute(
          "position",
          new THREE.BufferAttribute(particlePositions, 3)
        );
        particleGeometry.setAttribute(
          "size",
          new THREE.BufferAttribute(particleSizes, 1)
        );
        
        // Create particle material
        const particleMaterial = new THREE.PointsMaterial({
          color,
          size: 0.2,
          transparent: true,
          opacity: 0.7,
          blending: THREE.AdditiveBlending,
          sizeAttenuation: true,
        });
        
        // Create particle system
        const particles = new THREE.Points(particleGeometry, particleMaterial);
        particles.userData = {
          id: `${connection.source}-${connection.target}`,
          type: "particle",
          connection,
        };
        
        // Add to scene
        sceneRef.current?.add(particles);
        particlesRef.current.set(
          `${connection.source}-${connection.target}`,
          particles
        );
        
        // Add to active connections
        setActiveConnections((prev) => {
          const next = new Set(prev);
          next.add(`${connection.source}-${connection.target}`);
          return next;
        });
      }
    });
    
    // Update selected agent
    if (selectedAgent) {
      const node = nodesRef.current.get(selectedAgent);
      if (node) {
        const material = node.material as THREE.MeshStandardMaterial;
        material.emissiveIntensity = 0.7;
        node.scale.set(1.3, 1.3, 1.3);
        
        // Update outline pass
        if (outlinePassRef.current) {
          outlinePassRef.current.selectedObjects = [node];
        }
      }
    }
  }, [props.agents, props.connections, isInitialized, selectedAgent]);
  
  // Handle agent selection
  const handleAgentSelect = (agentId: string | null) => {
    setSelectedAgent(agentId);
    if (props.onAgentSelect) {
      props.onAgentSelect(agentId);
    }
  };
  
  // Render the base NetworkGraph component
  return (
    <div ref={containerRef} style={{ height: props.height, width: props.width }}>
      <NetworkGraph
        agents={props.agents}
        connections={props.connections}
        height={props.height}
        width={props.width}
        showControls={props.showControls}
        showGizmo={props.showGizmo}
        onAgentSelect={handleAgentSelect}
      />
    </div>
  );
};

// Helper functions (same as in NetworkGraph)
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

const getConnectionColor = (connection: Connection): number => {
  if (connection.type === "command") return 0x805ad5; // Accent color
  if (connection.type === "data") return 0x7ebab5; // Primary color
  
  if (connection.strength > 0.7) return 0x7ebab5; // Primary color
  if (connection.strength > 0.3) return 0xa3d0cc; // Primary light
  return 0x4a5568; // Muted color
};

const getConnectionOpacity = (connection: Connection): number => {
  const timeSinceActive = Date.now() - connection.lastActive;
  const hourInMs = 3600000;
  
  if (timeSinceActive < hourInMs) {
    return Math.max(0.2, 1 - timeSinceActive / hourInMs);
  }
  
  return 0.2;
};

// Force-directed layout calculation (same as in NetworkGraph)
const calculateNodePositions = (
  agents: Agent[],
  connections: Connection[]
): Map<string, THREE.Vector3> => {
  // Implementation same as in NetworkGraph
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

export default EnhancedNetworkGraph;
```

This enhanced implementation adds several visual improvements to the base network graph:

1. Post-processing effects with EffectComposer:
   - Bloom effect for a subtle glow around bright objects
   - Outline effect for highlighting selected agents

2. Improved lighting:
   - Multiple light sources with different colors
   - Shadows for better depth perception

3. Particle effects:
   - Background particles for a more dynamic scene
   - Animated particles along active connections to visualize data flow

4. Enhanced materials:
   - More realistic materials with metalness and roughness
   - Glow effect around agent nodes

5. Animation:
   - Rotating background particles
   - Moving particles along active connections

These enhancements create a more visually appealing and informative network visualization that better represents the dynamic nature of agent interactions.
