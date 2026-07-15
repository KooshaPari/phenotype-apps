# Jarvis SWE Agent 3D Visualization Optimization - Technical Implementation (Part 5)

## 7. Asset Management Optimization (Continued)

### 7.8 Memory Monitor

The Memory Monitor tracks memory usage and provides notifications when memory pressure is detected.

```typescript
class MemoryMonitor {
  private memoryThresholds: {
    low: number;
    medium: number;
    high: number;
  };
  private monitoringInterval: number | null = null;
  private checkIntervalMs: number = 5000;
  public onMemoryPressure: EventDispatcher<'low' | 'medium' | 'high'> = new EventDispatcher();
  
  constructor(thresholds?: Partial<{ low: number; medium: number; high: number }>) {
    // Set default thresholds (in MB)
    this.memoryThresholds = {
      low: thresholds?.low || 200,    // 200 MB
      medium: thresholds?.medium || 500,  // 500 MB
      high: thresholds?.high || 800    // 800 MB
    };
  }
  
  public startMonitoring(): void {
    // Stop existing monitoring
    this.stopMonitoring();
    
    // Start new monitoring interval
    this.monitoringInterval = window.setInterval(() => {
      this.checkMemory();
    }, this.checkIntervalMs);
  }
  
  public stopMonitoring(): void {
    if (this.monitoringInterval !== null) {
      window.clearInterval(this.monitoringInterval);
      this.monitoringInterval = null;
    }
  }
  
  private checkMemory(): void {
    // Get current memory usage
    const memoryUsage = this.getMemoryUsage();
    
    // Check thresholds
    if (memoryUsage >= this.memoryThresholds.high) {
      this.onMemoryPressure.dispatch('high');
    } else if (memoryUsage >= this.memoryThresholds.medium) {
      this.onMemoryPressure.dispatch('medium');
    } else if (memoryUsage >= this.memoryThresholds.low) {
      this.onMemoryPressure.dispatch('low');
    }
  }
  
  private getMemoryUsage(): number {
    // Check if performance.memory is available (Chrome only)
    if (window.performance && (performance as any).memory) {
      return (performance as any).memory.usedJSHeapSize / (1024 * 1024);
    }
    
    // Fallback: estimate memory usage based on object count
    // This is a very rough estimate and not reliable
    return 0;
  }
  
  public getMemoryInfo(): {
    usage: number;
    limit: number | null;
    available: number | null;
  } {
    // Get memory usage
    const usage = this.getMemoryUsage();
    
    // Get memory limit (if available)
    let limit = null;
    let available = null;
    
    if (window.performance && (performance as any).memory) {
      limit = (performance as any).memory.jsHeapSizeLimit / (1024 * 1024);
      available = limit - usage;
    }
    
    return {
      usage,
      limit,
      available
    };
  }
}

class EventDispatcher<T> {
  private listeners: ((data: T) => void)[] = [];
  
  public add(listener: (data: T) => void): void {
    if (!this.listeners.includes(listener)) {
      this.listeners.push(listener);
    }
  }
  
  public remove(listener: (data: T) => void): void {
    const index = this.listeners.indexOf(listener);
    if (index !== -1) {
      this.listeners.splice(index, 1);
    }
  }
  
  public dispatch(data: T): void {
    // Create a copy of listeners to avoid issues if listeners are added/removed during dispatch
    const listeners = [...this.listeners];
    
    for (const listener of listeners) {
      try {
        listener(data);
      } catch (error) {
        console.error('Error in event listener:', error);
      }
    }
  }
  
  public clear(): void {
    this.listeners = [];
  }
}
```

## 8. Data Processing Optimization

Efficient data processing is crucial for handling large datasets in 3D visualizations.

### 8.1 Web Worker Implementation

Using Web Workers allows heavy data processing to be offloaded from the main thread.

```typescript
// Main thread code
class DataProcessor {
  private worker: Worker | null = null;
  private taskId: number = 0;
  private tasks: Map<number, {
    resolve: (result: any) => void;
    reject: (error: any) => void;
  }> = new Map();
  
  constructor() {
    // Create worker
    this.initWorker();
  }
  
  private initWorker(): void {
    try {
      // Create worker from blob URL
      const workerCode = `
        // Worker code
        self.onmessage = function(e) {
          const { taskId, action, data } = e.data;
          
          try {
            let result;
            
            switch (action) {
              case 'processGraphData':
                result = processGraphData(data);
                break;
              case 'calculateNodePositions':
                result = calculateNodePositions(data);
                break;
              case 'optimizeGeometry':
                result = optimizeGeometry(data);
                break;
              default:
                throw new Error('Unknown action: ' + action);
            }
            
            // Send result back to main thread
            self.postMessage({
              taskId,
              result,
              error: null
            });
          } catch (error) {
            // Send error back to main thread
            self.postMessage({
              taskId,
              result: null,
              error: error.message
            });
          }
        };
        
        // Data processing functions
        function processGraphData(data) {
          // Process graph data
          const { nodes, edges } = data;
          
          // Create node map for quick lookup
          const nodeMap = new Map();
          
          nodes.forEach(node => {
            nodeMap.set(node.id, {
              ...node,
              connections: []
            });
          });
          
          // Process edges
          edges.forEach(edge => {
            const sourceNode = nodeMap.get(edge.source);
            const targetNode = nodeMap.get(edge.target);
            
            if (sourceNode && targetNode) {
              sourceNode.connections.push({
                target: edge.target,
                weight: edge.weight || 1
              });
              
              targetNode.connections.push({
                target: edge.source,
                weight: edge.weight || 1
              });
            }
          });
          
          // Calculate node metrics
          const processedNodes = Array.from(nodeMap.values()).map(node => {
            // Calculate degree (number of connections)
            const degree = node.connections.length;
            
            // Calculate weighted degree
            const weightedDegree = node.connections.reduce(
              (sum, connection) => sum + connection.weight,
              0
            );
            
            return {
              ...node,
              degree,
              weightedDegree
            };
          });
          
          return {
            nodes: processedNodes,
            edges
          };
        }
        
        function calculateNodePositions(data) {
          // Calculate node positions using force-directed algorithm
          const { nodes, edges, iterations, width, height, depth } = data;
          
          // Create node objects with initial random positions
          const nodeObjects = nodes.map(node => ({
            id: node.id,
            x: Math.random() * width - width / 2,
            y: Math.random() * height - height / 2,
            z: Math.random() * depth - depth / 2,
            vx: 0,
            vy: 0,
            vz: 0,
            mass: node.mass || 1
          }));
          
          // Create node map for quick lookup
          const nodeMap = new Map();
          
          nodeObjects.forEach(node => {
            nodeMap.set(node.id, node);
          });
          
          // Create edge objects
          const edgeObjects = edges.map(edge => {
            const source = nodeMap.get(edge.source);
            const target = nodeMap.get(edge.target);
            
            return {
              source,
              target,
              strength: edge.strength || 1
            };
          });
          
          // Run force-directed algorithm
          const repulsionForce = 50;
          const attractionForce = 0.1;
          const maxVelocity = 2;
          
          for (let i = 0; i < iterations; i++) {
            // Apply repulsion forces between all nodes
            for (let j = 0; j < nodeObjects.length; j++) {
              const nodeA = nodeObjects[j];
              
              for (let k = j + 1; k < nodeObjects.length; k++) {
                const nodeB = nodeObjects[k];
                
                // Calculate distance
                const dx = nodeB.x - nodeA.x;
                const dy = nodeB.y - nodeA.y;
                const dz = nodeB.z - nodeA.z;
                
                const distance = Math.sqrt(dx * dx + dy * dy + dz * dz) || 0.1;
                
                // Calculate repulsion force
                const force = repulsionForce / (distance * distance);
                
                // Apply force to velocity
                const fx = dx / distance * force;
                const fy = dy / distance * force;
                const fz = dz / distance * force;
                
                nodeA.vx += fx;
                nodeA.vy += fy;
                nodeA.vz += fz;
                
                nodeB.vx -= fx;
                nodeB.vy -= fy;
                nodeB.vz -= fz;
              }
            }
            
            // Apply attraction forces along edges
            for (const edge of edgeObjects) {
              const source = edge.source;
              const target = edge.target;
              
              // Calculate distance
              const dx = target.x - source.x;
              const dy = target.y - source.y;
              const dz = target.z - source.z;
              
              const distance = Math.sqrt(dx * dx + dy * dy + dz * dz) || 0.1;
              
              // Calculate attraction force
              const force = distance * attractionForce * edge.strength;
              
              // Apply force to velocity
              const fx = dx / distance * force;
              const fy = dy / distance * force;
              const fz = dz / distance * force;
              
              source.vx += fx;
              source.vy += fy;
              source.vz += fz;
              
              target.vx -= fx;
              target.vy -= fy;
              target.vz -= fz;
            }
            
            // Update positions
            for (const node of nodeObjects) {
              // Limit velocity
              const velocity = Math.sqrt(node.vx * node.vx + node.vy * node.vy + node.vz * node.vz);
              
              if (velocity > maxVelocity) {
                const scale = maxVelocity / velocity;
                node.vx *= scale;
                node.vy *= scale;
                node.vz *= scale;
              }
              
              // Update position
              node.x += node.vx;
              node.y += node.vy;
              node.z += node.vz;
              
              // Apply damping
              node.vx *= 0.9;
              node.vy *= 0.9;
              node.vz *= 0.9;
              
              // Keep within bounds
              node.x = Math.max(-width / 2, Math.min(width / 2, node.x));
              node.y = Math.max(-height / 2, Math.min(height / 2, node.y));
              node.z = Math.max(-depth / 2, Math.min(depth / 2, node.z));
            }
          }
          
          // Return final node positions
          return nodeObjects.map(node => ({
            id: node.id,
            position: [node.x, node.y, node.z]
          }));
        }
        
        function optimizeGeometry(data) {
          // Optimize geometry data
          const { vertices, indices, normals, uvs } = data;
          
          // Deduplicate vertices
          const uniqueVertices = new Map();
          const newIndices = [];
          const newVertices = [];
          const newNormals = normals ? [] : null;
          const newUvs = uvs ? [] : null;
          
          // Process each vertex
          for (let i = 0; i < indices.length; i++) {
            const index = indices[i];
            
            // Get vertex components
            const vx = vertices[index * 3];
            const vy = vertices[index * 3 + 1];
            const vz = vertices[index * 3 + 2];
            
            // Get normal components (if available)
            const nx = normals ? normals[index * 3] : 0;
            const ny = normals ? normals[index * 3 + 1] : 0;
            const nz = normals ? normals[index * 3 + 2] : 0;
            
            // Get UV components (if available)
            const u = uvs ? uvs[index * 2] : 0;
            const v = uvs ? uvs[index * 2 + 1] : 0;
            
            // Create key for vertex
            const key = \`\${vx},\${vy},\${vz},\${nx},\${ny},\${nz},\${u},\${v}\`;
            
            // Check if vertex already exists
            if (uniqueVertices.has(key)) {
              // Reuse existing vertex
              newIndices.push(uniqueVertices.get(key));
            } else {
              // Add new vertex
              const newIndex = newVertices.length / 3;
              
              newVertices.push(vx, vy, vz);
              
              if (newNormals) {
                newNormals.push(nx, ny, nz);
              }
              
              if (newUvs) {
                newUvs.push(u, v);
              }
              
              uniqueVertices.set(key, newIndex);
              newIndices.push(newIndex);
            }
          }
          
          return {
            vertices: newVertices,
            indices: newIndices,
            normals: newNormals,
            uvs: newUvs
          };
        }
      `;
      
      const blob = new Blob([workerCode], { type: 'application/javascript' });
      const url = URL.createObjectURL(blob);
      
      this.worker = new Worker(url);
      
      // Set up message handler
      this.worker.onmessage = this.handleWorkerMessage.bind(this);
      
      // Clean up URL
      URL.revokeObjectURL(url);
    } catch (error) {
      console.error('Failed to create worker:', error);
      this.worker = null;
    }
  }
  
  private handleWorkerMessage(event: MessageEvent): void {
    const { taskId, result, error } = event.data;
    
    // Get task
    const task = this.tasks.get(taskId);
    
    if (!task) {
      console.warn('Received response for unknown task:', taskId);
      return;
    }
    
    // Remove task
    this.tasks.delete(taskId);
    
    // Handle result or error
    if (error) {
      task.reject(new Error(error));
    } else {
      task.resolve(result);
    }
  }
  
  public async processGraphData(data: any): Promise<any> {
    return this.executeTask('processGraphData', data);
  }
  
  public async calculateNodePositions(data: any): Promise<any> {
    return this.executeTask('calculateNodePositions', data);
  }
  
  public async optimizeGeometry(data: any): Promise<any> {
    return this.executeTask('optimizeGeometry', data);
  }
  
  private async executeTask(action: string, data: any): Promise<any> {
    // Check if worker is available
    if (!this.worker) {
      // Try to reinitialize worker
      this.initWorker();
      
      if (!this.worker) {
        // Fall back to synchronous processing
        console.warn(`Worker not available, falling back to synchronous processing for ${action}`);
        return this.processSynchronously(action, data);
      }
    }
    
    // Generate task ID
    const taskId = this.taskId++;
    
    // Create promise
    return new Promise((resolve, reject) => {
      // Store task
      this.tasks.set(taskId, { resolve, reject });
      
      // Send message to worker
      this.worker!.postMessage({
        taskId,
        action,
        data
      });
    });
  }
  
  private processSynchronously(action: string, data: any): any {
    // Fallback synchronous processing
    // This is a simplified implementation
    
    switch (action) {
      case 'processGraphData':
        // Process graph data
        return data;
      
      case 'calculateNodePositions':
        // Calculate node positions
        return data.nodes.map(node => ({
          id: node.id,
          position: [
            Math.random() * data.width - data.width / 2,
            Math.random() * data.height - data.height / 2,
            Math.random() * data.depth - data.depth / 2
          ]
        }));
      
      case 'optimizeGeometry':
        // Optimize geometry
        return data;
      
      default:
        throw new Error(`Unknown action: ${action}`);
    }
  }
  
  public dispose(): void {
    // Terminate worker
    if (this.worker) {
      this.worker.terminate();
      this.worker = null;
    }
    
    // Reject all pending tasks
    for (const task of this.tasks.values()) {
      task.reject(new Error('DataProcessor disposed'));
    }
    
    this.tasks.clear();
  }
}
```

### 8.2 Incremental Processing

Incremental processing allows handling large datasets without blocking the main thread.

```typescript
class IncrementalProcessor {
  private processing: boolean = false;
  private queue: ProcessingTask[] = [];
  private currentTask: ProcessingTask | null = null;
  private chunkSize: number = 1000;
  private timeBudgetMs: number = 16; // ~60 FPS
  
  constructor(chunkSize: number = 1000, timeBudgetMs: number = 16) {
    this.chunkSize = chunkSize;
    this.timeBudgetMs = timeBudgetMs;
  }
  
  public processData<T, R>(
    data: T[],
    processor: (item: T, index: number) => R,
    onProgress?: (processed: number, total: number) => void,
    onComplete?: (result: R[]) => void
  ): void {
    // Create task
    const task: ProcessingTask = {
      data,
      processor,
      result: [],
      processed: 0,
      onProgress,
      onComplete: onComplete as (result: any[]) => void
    };
    
    // Add to queue
    this.queue.push(task);
    
    // Start processing if not already
    if (!this.processing) {
      this.startProcessing();
    }
  }
  
  private startProcessing(): void {
    // Mark as processing
    this.processing = true;
    
    // Schedule first chunk
    requestAnimationFrame(this.processChunk.bind(this));
  }
  
  private processChunk(): void {
    // Get start time
    const startTime = performance.now();
    
    // Get current task
    if (!this.currentTask && this.queue.length > 0) {
      this.currentTask = this.queue.shift()!;
    }
    
    // If no task, stop processing
    if (!this.currentTask) {
      this.processing = false;
      return;
    }
    
    // Process chunk
    const task = this.currentTask;
    const endIndex = Math.min(task.processed + this.chunkSize, task.data.length);
    
    for (let i = task.processed; i < endIndex; i++) {
      task.result.push(task.processor(task.data[i], i));
      task.processed++;
      
      // Check time budget
      if (performance.now() - startTime > this.timeBudgetMs) {
        break;
      }
    }
    
    // Call progress callback
    if (task.onProgress) {
      task.onProgress(task.processed, task.data.length);
    }
    
    // Check if task is complete
    if (task.processed >= task.data.length) {
      // Call complete callback
      if (task.onComplete) {
        task.onComplete(task.result);
      }
      
      // Clear current task
      this.currentTask = null;
    }
    
    // Schedule next chunk
    requestAnimationFrame(this.processChunk.bind(this));
  }
  
  public cancelAll(): void {
    // Clear queue
    this.queue = [];
    
    // Clear current task
    this.currentTask = null;
    
    // Stop processing
    this.processing = false;
  }
}

interface ProcessingTask {
  data: any[];
  processor: (item: any, index: number) => any;
  result: any[];
  processed: number;
  onProgress?: (processed: number, total: number) => void;
  onComplete?: (result: any[]) => void;
}
```

### 8.3 Efficient Data Structures

Using efficient data structures is crucial for handling large datasets.

```typescript
class SpatialHashGrid {
  private cellSize: number;
  private cells: Map<string, Set<string>> = new Map();
  private objectPositions: Map<string, THREE.Vector3> = new Map();
  
  constructor(cellSize: number = 10) {
    this.cellSize = cellSize;
  }
  
  public addObject(id: string, position: THREE.Vector3): void {
    // Remove from previous position
    this.removeObject(id);
    
    // Store position
    this.objectPositions.set(id, position.clone());
    
    // Get cell coordinates
    const cellX = Math.floor(position.x / this.cellSize);
    const cellY = Math.floor(position.y / this.cellSize);
    const cellZ = Math.floor(position.z / this.cellSize);
    
    // Get cell key
    const cellKey = `${cellX},${cellY},${cellZ}`;
    
    // Get or create cell
    let cell = this.cells.get(cellKey);
    
    if (!cell) {
      cell = new Set();
      this.cells.set(cellKey, cell);
    }
    
    // Add object to cell
    cell.add(id);
  }
  
  public removeObject(id: string): void {
    // Get position
    const position = this.objectPositions.get(id);
    
    if (!position) {
      return;
    }
    
    // Get cell coordinates
    const cellX = Math.floor(position.x / this.cellSize);
    const cellY = Math.floor(position.y / this.cellSize);
    const cellZ = Math.floor(position.z / this.cellSize);
    
    // Get cell key
    const cellKey = `${cellX},${cellY},${cellZ}`;
    
    // Get cell
    const cell = this.cells.get(cellKey);
    
    if (cell) {
      // Remove object from cell
      cell.delete(id);
      
      // Remove cell if empty
      if (cell.size === 0) {
        this.cells.delete(cellKey);
      }
    }
    
    // Remove position
    this.objectPositions.delete(id);
  }
  
  public updateObject(id: string, position: THREE.Vector3): void {
    // Get old position
    const oldPosition = this.objectPositions.get(id);
    
    if (!oldPosition) {
      // Add as new object
      this.addObject(id, position);
      return;
    }
    
    // Get old cell coordinates
    const oldCellX = Math.floor(oldPosition.x / this.cellSize);
    const oldCellY = Math.floor(oldPosition.y / this.cellSize);
    const oldCellZ = Math.floor(oldPosition.z / this.cellSize);
    
    // Get new cell coordinates
    const newCellX = Math.floor(position.x / this.cellSize);
    const newCellY = Math.floor(position.y / this.cellSize);
    const newCellZ = Math.floor(position.z / this.cellSize);
    
    // Check if cell changed
    if (oldCellX !== newCellX || oldCellY !== newCellY || oldCellZ !== newCellZ) {
      // Remove from old cell
      this.removeObject(id);
      
      // Add to new cell
      this.addObject(id, position);
    } else {
      // Just update position
      this.objectPositions.set(id, position.clone());
    }
  }
  
  public findNearbyObjects(position: THREE.Vector3, radius: number): string[] {
    // Get cell range
    const minCellX = Math.floor((position.x - radius) / this.cellSize);
    const maxCellX = Math.floor((position.x + radius) / this.cellSize);
    const minCellY = Math.floor((position.y - radius) / this.cellSize);
    const maxCellY = Math.floor((position.y + radius) / this.cellSize);
    const minCellZ = Math.floor((position.z - radius) / this.cellSize);
    const maxCellZ = Math.floor((position.z + radius) / this.cellSize);
    
    // Get objects in range
    const nearbyObjects: string[] = [];
    const radiusSquared = radius * radius;
    
    // Check each cell in range
    for (let cellX = minCellX; cellX <= maxCellX; cellX++) {
      for (let cellY = minCellY; cellY <= maxCellY; cellY++) {
        for (let cellZ = minCellZ; cellZ <= maxCellZ; cellZ++) {
          // Get cell key
          const cellKey = `${cellX},${cellY},${cellZ}`;
          
          // Get cell
          const cell = this.cells.get(cellKey);
          
          if (!cell) {
            continue;
          }
          
          // Check each object in cell
          for (const objectId of cell) {
            // Get object position
            const objectPosition = this.objectPositions.get(objectId);
            
            if (!objectPosition) {
              continue;
            }
            
            // Check distance
            const distanceSquared = position.distanceToSquared(objectPosition);
            
            if (distanceSquared <= radiusSquared) {
              nearbyObjects.push(objectId);
            }
          }
        }
      }
    }
    
    return nearbyObjects;
  }
  
  public clear(): void {
    this.cells.clear();
    this.objectPositions.clear();
  }
}
```
