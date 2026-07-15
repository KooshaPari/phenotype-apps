# Jarvis SWE Agent 3D Visualization Optimization - Technical Implementation (Part 2)

## 5. Occlusion Culling Implementation

Occlusion culling is a technique to avoid rendering objects that are not visible to the camera, either because they are outside the view frustum or because they are occluded by other objects.

### 5.1 Frustum Culling

Frustum culling removes objects that are outside the camera's view frustum.

```typescript
class FrustumCuller {
  private camera: THREE.Camera;
  private frustum: THREE.Frustum;
  private frustumMatrix: THREE.Matrix4;
  
  constructor(camera: THREE.Camera) {
    this.camera = camera;
    this.frustum = new THREE.Frustum();
    this.frustumMatrix = new THREE.Matrix4();
    
    // Initial update
    this.update();
  }
  
  public update(): void {
    // Update frustum from camera
    this.frustumMatrix.multiplyMatrices(
      this.camera.projectionMatrix,
      this.camera.matrixWorldInverse
    );
    this.frustum.setFromProjectionMatrix(this.frustumMatrix);
  }
  
  public isVisible(object: THREE.Object3D): boolean {
    // Use bounding sphere for quick check
    if (!object.geometry) return true;
    
    // Get or compute bounding sphere
    let boundingSphere: THREE.Sphere;
    
    if (object.geometry.boundingSphere === null) {
      object.geometry.computeBoundingSphere();
    }
    
    boundingSphere = object.geometry.boundingSphere!.clone();
    
    // Transform bounding sphere to world space
    const center = boundingSphere.center.clone();
    center.applyMatrix4(object.matrixWorld);
    
    // Scale radius by largest scale component
    const scale = object.scale.clone();
    const maxScale = Math.max(scale.x, Math.max(scale.y, scale.z));
    const radius = boundingSphere.radius * maxScale;
    
    // Create transformed bounding sphere
    const worldBoundingSphere = new THREE.Sphere(center, radius);
    
    // Check if sphere intersects frustum
    return this.frustum.intersectsSphere(worldBoundingSphere);
  }
  
  public cullObjects(objects: THREE.Object3D[]): THREE.Object3D[] {
    return objects.filter(obj => this.isVisible(obj));
  }
}
```

### 5.2 Hierarchical Z-Buffer Occlusion Culling

Hierarchical Z-Buffer occlusion culling identifies objects that are occluded by other objects closer to the camera.

```typescript
class HierarchicalZBufferCuller {
  private renderer: THREE.WebGLRenderer;
  private camera: THREE.Camera;
  private scene: THREE.Scene;
  private depthTexture: THREE.DepthTexture;
  private depthTarget: THREE.WebGLRenderTarget;
  private hierarchicalZBuffer: HierarchicalZBuffer;
  private occluders: THREE.Object3D[] = [];
  
  constructor(renderer: THREE.WebGLRenderer, camera: THREE.Camera, scene: THREE.Scene) {
    this.renderer = renderer;
    this.camera = camera;
    this.scene = scene;
    
    // Create depth texture
    this.depthTexture = new THREE.DepthTexture(
      window.innerWidth,
      window.innerHeight
    );
    this.depthTexture.format = THREE.DepthFormat;
    this.depthTexture.type = THREE.UnsignedShortType;
    
    // Create render target with depth texture
    this.depthTarget = new THREE.WebGLRenderTarget(
      window.innerWidth,
      window.innerHeight,
      {
        depthTexture: this.depthTexture,
        depthBuffer: true
      }
    );
    
    // Create hierarchical Z-buffer
    this.hierarchicalZBuffer = new HierarchicalZBuffer(
      window.innerWidth,
      window.innerHeight
    );
  }
  
  public setOccluders(occluders: THREE.Object3D[]): void {
    this.occluders = occluders;
  }
  
  public update(): void {
    // Render occluders to depth buffer
    const originalRenderTarget = this.renderer.getRenderTarget();
    
    // Set render target to depth target
    this.renderer.setRenderTarget(this.depthTarget);
    
    // Clear depth buffer
    this.renderer.clear(false, true, false);
    
    // Render occluders only
    const originalVisibility: Map<THREE.Object3D, boolean> = new Map();
    
    // Hide non-occluders
    this.scene.traverse(object => {
      if (object.visible) {
        originalVisibility.set(object, true);
        object.visible = this.occluders.includes(object);
      }
    });
    
    // Render occluders to depth buffer
    this.renderer.render(this.scene, this.camera);
    
    // Restore original visibility
    originalVisibility.forEach((visible, object) => {
      object.visible = visible;
    });
    
    // Restore original render target
    this.renderer.setRenderTarget(originalRenderTarget);
    
    // Update hierarchical Z-buffer from depth texture
    this.hierarchicalZBuffer.update(this.depthTexture);
  }
  
  public isVisible(object: THREE.Object3D): boolean {
    // Skip check for objects without geometry
    if (!object.geometry) return true;
    
    // Get or compute bounding box
    let boundingBox: THREE.Box3;
    
    if (object.geometry.boundingBox === null) {
      object.geometry.computeBoundingBox();
    }
    
    boundingBox = object.geometry.boundingBox!.clone();
    
    // Transform bounding box to world space
    boundingBox.applyMatrix4(object.matrixWorld);
    
    // Transform bounding box to screen space
    const screenBounds = this.transformBoundsToScreen(boundingBox);
    
    // Check if bounding box is occluded
    return !this.hierarchicalZBuffer.isOccluded(
      screenBounds.min.x,
      screenBounds.min.y,
      screenBounds.max.x,
      screenBounds.max.y,
      screenBounds.minZ
    );
  }
  
  public cullObjects(objects: THREE.Object3D[]): THREE.Object3D[] {
    return objects.filter(obj => this.isVisible(obj));
  }
  
  private transformBoundsToScreen(bounds: THREE.Box3): {
    min: THREE.Vector2;
    max: THREE.Vector2;
    minZ: number;
  } {
    // Get corners of bounding box
    const corners = [
      new THREE.Vector3(bounds.min.x, bounds.min.y, bounds.min.z),
      new THREE.Vector3(bounds.min.x, bounds.min.y, bounds.max.z),
      new THREE.Vector3(bounds.min.x, bounds.max.y, bounds.min.z),
      new THREE.Vector3(bounds.min.x, bounds.max.y, bounds.max.z),
      new THREE.Vector3(bounds.max.x, bounds.min.y, bounds.min.z),
      new THREE.Vector3(bounds.max.x, bounds.min.y, bounds.max.z),
      new THREE.Vector3(bounds.max.x, bounds.max.y, bounds.min.z),
      new THREE.Vector3(bounds.max.x, bounds.max.y, bounds.max.z)
    ];
    
    // Project corners to screen space
    const screenCorners = corners.map(corner => {
      const projected = corner.clone().project(this.camera);
      return {
        x: (projected.x + 1) * window.innerWidth / 2,
        y: (-projected.y + 1) * window.innerHeight / 2,
        z: projected.z
      };
    });
    
    // Find min/max screen coordinates
    let minX = Infinity;
    let minY = Infinity;
    let maxX = -Infinity;
    let maxY = -Infinity;
    let minZ = Infinity;
    
    screenCorners.forEach(corner => {
      minX = Math.min(minX, corner.x);
      minY = Math.min(minY, corner.y);
      maxX = Math.max(maxX, corner.x);
      maxY = Math.max(maxY, corner.y);
      minZ = Math.min(minZ, corner.z);
    });
    
    // Clamp to screen bounds
    minX = Math.max(0, Math.min(window.innerWidth, minX));
    minY = Math.max(0, Math.min(window.innerHeight, minY));
    maxX = Math.max(0, Math.min(window.innerWidth, maxX));
    maxY = Math.max(0, Math.min(window.innerHeight, maxY));
    
    return {
      min: new THREE.Vector2(minX, minY),
      max: new THREE.Vector2(maxX, maxY),
      minZ
    };
  }
}

class HierarchicalZBuffer {
  private width: number;
  private height: number;
  private levels: Float32Array[] = [];
  private levelWidths: number[] = [];
  private levelHeights: number[] = [];
  
  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
    
    // Create mipmap levels
    let levelWidth = width;
    let levelHeight = height;
    
    while (levelWidth > 1 || levelHeight > 1) {
      // Create level buffer
      const buffer = new Float32Array(levelWidth * levelHeight);
      this.levels.push(buffer);
      this.levelWidths.push(levelWidth);
      this.levelHeights.push(levelHeight);
      
      // Next level dimensions
      levelWidth = Math.max(1, Math.floor(levelWidth / 2));
      levelHeight = Math.max(1, Math.floor(levelHeight / 2));
    }
    
    // Add final 1x1 level
    this.levels.push(new Float32Array(1));
    this.levelWidths.push(1);
    this.levelHeights.push(1);
  }
  
  public update(depthTexture: THREE.DepthTexture): void {
    // Read depth texture to level 0
    // Note: In a real implementation, this would use a shader to read the depth texture
    // This is a simplified version that assumes we can access the depth data
    const depthData = this.readDepthTexture(depthTexture);
    this.levels[0].set(depthData);
    
    // Build mipmap hierarchy
    for (let level = 1; level < this.levels.length; level++) {
      const srcLevel = level - 1;
      const srcWidth = this.levelWidths[srcLevel];
      const srcHeight = this.levelHeights[srcLevel];
      const dstWidth = this.levelWidths[level];
      const dstHeight = this.levelHeights[level];
      
      // For each pixel in destination level
      for (let y = 0; y < dstHeight; y++) {
        for (let x = 0; x < dstWidth; x++) {
          // Find maximum depth value in 2x2 source region
          let maxDepth = -Infinity;
          
          for (let dy = 0; dy < 2; dy++) {
            for (let dx = 0; dx < 2; dx++) {
              const srcX = x * 2 + dx;
              const srcY = y * 2 + dy;
              
              if (srcX < srcWidth && srcY < srcHeight) {
                const srcIndex = srcY * srcWidth + srcX;
                maxDepth = Math.max(maxDepth, this.levels[srcLevel][srcIndex]);
              }
            }
          }
          
          // Store maximum depth in destination level
          const dstIndex = y * dstWidth + x;
          this.levels[level][dstIndex] = maxDepth;
        }
      }
    }
  }
  
  public isOccluded(minX: number, minY: number, maxX: number, maxY: number, z: number): boolean {
    // Convert z to depth value (0-1)
    const depth = (z + 1) / 2;
    
    // Find appropriate mipmap level
    const width = maxX - minX;
    const height = maxY - minY;
    const size = Math.max(width, height);
    const level = Math.floor(Math.log2(size));
    const clampedLevel = Math.min(level, this.levels.length - 1);
    
    // Check if region is occluded at this level
    const levelWidth = this.levelWidths[clampedLevel];
    const levelHeight = this.levelHeights[clampedLevel];
    
    // Convert screen coordinates to level coordinates
    const levelMinX = Math.floor(minX * levelWidth / this.width);
    const levelMinY = Math.floor(minY * levelHeight / this.height);
    const levelMaxX = Math.ceil(maxX * levelWidth / this.width);
    const levelMaxY = Math.ceil(maxY * levelHeight / this.height);
    
    // Check if any pixel in region has depth less than z
    for (let y = levelMinY; y < levelMaxY; y++) {
      for (let x = levelMinX; x < levelMaxX; x++) {
        if (x >= 0 && x < levelWidth && y >= 0 && y < levelHeight) {
          const index = y * levelWidth + x;
          const pixelDepth = this.levels[clampedLevel][index];
          
          // If any pixel is closer than our object, it's not occluded
          if (pixelDepth < depth) {
            return false;
          }
        }
      }
    }
    
    // All pixels are further than our object, it's occluded
    return true;
  }
  
  private readDepthTexture(depthTexture: THREE.DepthTexture): Float32Array {
    // Note: In a real implementation, this would use a shader to read the depth texture
    // This is a simplified placeholder that returns a dummy depth buffer
    return new Float32Array(this.width * this.height).fill(1.0);
  }
}
```

### 5.3 Portal-Based Culling

Portal-based culling is useful for complex scenes with distinct areas connected by portals (doors, windows, etc.).

```typescript
class PortalCuller {
  private camera: THREE.Camera;
  private portals: Portal[] = [];
  private currentCell: Cell | null = null;
  
  constructor(camera: THREE.Camera) {
    this.camera = camera;
  }
  
  public setCells(cells: Cell[]): void {
    // Set up cells and portals
    this.portals = [];
    
    // Extract portals from cells
    cells.forEach(cell => {
      cell.portals.forEach(portal => {
        this.portals.push(portal);
      });
    });
  }
  
  public setCurrentCell(cell: Cell): void {
    this.currentCell = cell;
  }
  
  public update(): void {
    // Update current cell based on camera position
    if (!this.currentCell) return;
    
    const cameraPosition = this.camera.position.clone();
    
    // Check if camera is still in current cell
    if (!this.currentCell.contains(cameraPosition)) {
      // Find new cell
      this.currentCell = this.findCellContaining(cameraPosition);
    }
    
    // Update visible cells through portals
    if (this.currentCell) {
      this.currentCell.updateVisibility(this.camera);
    }
  }
  
  public isVisible(object: THREE.Object3D): boolean {
    // If no current cell, assume everything is visible
    if (!this.currentCell) return true;
    
    // Get object position
    const position = new THREE.Vector3();
    object.getWorldPosition(position);
    
    // Find cell containing object
    const cell = this.findCellContaining(position);
    
    // If object is not in any cell, assume it's visible
    if (!cell) return true;
    
    // Check if cell is visible
    return cell.isVisible;
  }
  
  public cullObjects(objects: THREE.Object3D[]): THREE.Object3D[] {
    return objects.filter(obj => this.isVisible(obj));
  }
  
  private findCellContaining(position: THREE.Vector3): Cell | null {
    // Find cell containing position
    // This is a simplified implementation
    // In a real implementation, this would use spatial partitioning
    
    if (this.currentCell && this.currentCell.contains(position)) {
      return this.currentCell;
    }
    
    // Check neighboring cells first
    if (this.currentCell) {
      for (const portal of this.currentCell.portals) {
        const neighborCell = portal.getOtherCell(this.currentCell);
        if (neighborCell && neighborCell.contains(position)) {
          return neighborCell;
        }
      }
    }
    
    // Check all cells
    // In a real implementation, this would use spatial partitioning
    const cells: Cell[] = [];
    this.portals.forEach(portal => {
      if (!cells.includes(portal.cell1)) cells.push(portal.cell1);
      if (!cells.includes(portal.cell2)) cells.push(portal.cell2);
    });
    
    for (const cell of cells) {
      if (cell.contains(position)) {
        return cell;
      }
    }
    
    return null;
  }
}

class Cell {
  public id: string;
  public bounds: THREE.Box3;
  public portals: Portal[] = [];
  public isVisible: boolean = false;
  public objects: THREE.Object3D[] = [];
  
  constructor(id: string, bounds: THREE.Box3) {
    this.id = id;
    this.bounds = bounds;
  }
  
  public contains(position: THREE.Vector3): boolean {
    return this.bounds.containsPoint(position);
  }
  
  public addPortal(portal: Portal): void {
    if (!this.portals.includes(portal)) {
      this.portals.push(portal);
    }
  }
  
  public addObject(object: THREE.Object3D): void {
    if (!this.objects.includes(object)) {
      this.objects.push(object);
    }
  }
  
  public updateVisibility(camera: THREE.Camera): void {
    // If this is the current cell, it's visible
    const cameraPosition = camera.position.clone();
    
    if (this.contains(cameraPosition)) {
      this.isVisible = true;
      
      // Update visibility of neighboring cells
      this.updateNeighborVisibility(camera, new Set<string>([this.id]));
    }
  }
  
  private updateNeighborVisibility(camera: THREE.Camera, visitedCells: Set<string>): void {
    // For each portal in this cell
    for (const portal of this.portals) {
      // Get the cell on the other side of the portal
      const otherCell = portal.getOtherCell(this);
      
      // Skip if already visited
      if (!otherCell || visitedCells.has(otherCell.id)) continue;
      
      // Check if portal is visible from camera
      if (portal.isVisibleFrom(camera)) {
        // Mark other cell as visible
        otherCell.isVisible = true;
        
        // Add to visited cells
        visitedCells.add(otherCell.id);
        
        // Recursively update visibility of cells visible through this one
        otherCell.updateNeighborVisibility(camera, visitedCells);
      }
    }
  }
}

class Portal {
  public id: string;
  public cell1: Cell;
  public cell2: Cell;
  public geometry: THREE.BufferGeometry;
  
  constructor(id: string, cell1: Cell, cell2: Cell, geometry: THREE.BufferGeometry) {
    this.id = id;
    this.cell1 = cell1;
    this.cell2 = cell2;
    this.geometry = geometry;
    
    // Add portal to cells
    cell1.addPortal(this);
    cell2.addPortal(this);
  }
  
  public getOtherCell(cell: Cell): Cell | null {
    if (cell === this.cell1) return this.cell2;
    if (cell === this.cell2) return this.cell1;
    return null;
  }
  
  public isVisibleFrom(camera: THREE.Camera): boolean {
    // Check if portal is visible from camera position
    // This is a simplified implementation
    // In a real implementation, this would use frustum culling and occlusion testing
    
    // Get portal center and normal
    const center = new THREE.Vector3();
    const normal = new THREE.Vector3();
    
    // Calculate center from geometry
    if (this.geometry.attributes.position) {
      const positions = this.geometry.attributes.position.array;
      const vertexCount = positions.length / 3;
      
      for (let i = 0; i < vertexCount; i++) {
        center.x += positions[i * 3];
        center.y += positions[i * 3 + 1];
        center.z += positions[i * 3 + 2];
      }
      
      center.divideScalar(vertexCount);
    }
    
    // Calculate normal (assuming portal is a quad)
    if (this.geometry.attributes.position && this.geometry.attributes.position.count >= 3) {
      const positions = this.geometry.attributes.position.array;
      
      const v0 = new THREE.Vector3(
        positions[0],
        positions[1],
        positions[2]
      );
      
      const v1 = new THREE.Vector3(
        positions[3],
        positions[4],
        positions[5]
      );
      
      const v2 = new THREE.Vector3(
        positions[6],
        positions[7],
        positions[8]
      );
      
      const edge1 = v1.clone().sub(v0);
      const edge2 = v2.clone().sub(v0);
      
      normal.crossVectors(edge1, edge2).normalize();
    }
    
    // Check if camera is in front of portal
    const cameraToPortal = center.clone().sub(camera.position);
    const dotProduct = cameraToPortal.dot(normal);
    
    // If dot product is negative, camera is in front of portal
    return dotProduct < 0;
  }
}
```

## 6. WebGL Optimization

### 6.1 Instanced Rendering

Instanced rendering allows rendering many similar objects with a single draw call.

```typescript
class InstancedRenderer {
  private scene: THREE.Scene;
  private instancedMeshes: Map<string, THREE.InstancedMesh> = new Map();
  private instanceData: Map<string, InstanceData[]> = new Map();
  
  constructor(scene: THREE.Scene) {
    this.scene = scene;
  }
  
  public addInstance(
    groupId: string,
    geometry: THREE.BufferGeometry,
    material: THREE.Material,
    position: THREE.Vector3,
    rotation: THREE.Euler,
    scale: THREE.Vector3,
    userData: any = {}
  ): string {
    // Generate instance ID
    const instanceId = uuidv4();
    
    // Create instance data
    const instanceData: InstanceData = {
      id: instanceId,
      position: position.clone(),
      rotation: rotation.clone(),
      scale: scale.clone(),
      matrix: new THREE.Matrix4(),
      userData
    };
    
    // Update instance matrix
    this.updateInstanceMatrix(instanceData);
    
    // Add to instance data map
    if (!this.instanceData.has(groupId)) {
      this.instanceData.set(groupId, []);
    }
    
    this.instanceData.get(groupId)!.push(instanceData);
    
    // Create or update instanced mesh
    this.updateInstancedMesh(groupId, geometry, material);
    
    return instanceId;
  }
  
  public removeInstance(groupId: string, instanceId: string): boolean {
    // Get instance data for group
    const instances = this.instanceData.get(groupId);
    
    if (!instances) return false;
    
    // Find instance index
    const index = instances.findIndex(instance => instance.id === instanceId);
    
    if (index === -1) return false;
    
    // Remove instance
    instances.splice(index, 1);
    
    // Update instanced mesh
    const mesh = this.instancedMeshes.get(groupId);
    
    if (mesh) {
      // If no instances left, remove mesh
      if (instances.length === 0) {
        this.scene.remove(mesh);
        mesh.dispose();
        this.instancedMeshes.delete(groupId);
        this.instanceData.delete(groupId);
      } else {
        // Otherwise, update mesh
        this.updateInstanceMatrices(groupId);
      }
    }
    
    return true;
  }
  
  public updateInstance(
    groupId: string,
    instanceId: string,
    position?: THREE.Vector3,
    rotation?: THREE.Euler,
    scale?: THREE.Vector3,
    userData?: any
  ): boolean {
    // Get instance data for group
    const instances = this.instanceData.get(groupId);
    
    if (!instances) return false;
    
    // Find instance
    const instance = instances.find(instance => instance.id === instanceId);
    
    if (!instance) return false;
    
    // Update instance data
    if (position) instance.position.copy(position);
    if (rotation) instance.rotation.copy(rotation);
    if (scale) instance.scale.copy(scale);
    if (userData) instance.userData = { ...instance.userData, ...userData };
    
    // Update instance matrix
    this.updateInstanceMatrix(instance);
    
    // Update instanced mesh
    this.updateInstanceMatrices(groupId);
    
    return true;
  }
  
  public getInstanceCount(groupId: string): number {
    const instances = this.instanceData.get(groupId);
    return instances ? instances.length : 0;
  }
  
  public clear(): void {
    // Remove all instanced meshes
    this.instancedMeshes.forEach(mesh => {
      this.scene.remove(mesh);
      mesh.dispose();
    });
    
    // Clear maps
    this.instancedMeshes.clear();
    this.instanceData.clear();
  }
  
  private updateInstancedMesh(
    groupId: string,
    geometry: THREE.BufferGeometry,
    material: THREE.Material
  ): void {
    // Get instances for group
    const instances = this.instanceData.get(groupId)!;
    
    // Get or create instanced mesh
    let mesh = this.instancedMeshes.get(groupId);
    
    if (!mesh || mesh.count !== instances.length) {
      // Remove old mesh if it exists
      if (mesh) {
        this.scene.remove(mesh);
        mesh.dispose();
      }
      
      // Create new instanced mesh
      mesh = new THREE.InstancedMesh(
        geometry,
        material,
        instances.length
      );
      
      mesh.frustumCulled = true;
      mesh.name = `instanced-${groupId}`;
      
      // Add to scene
      this.scene.add(mesh);
      
      // Store mesh
      this.instancedMeshes.set(groupId, mesh);
    }
    
    // Update instance matrices
    this.updateInstanceMatrices(groupId);
  }
  
  private updateInstanceMatrices(groupId: string): void {
    // Get instanced mesh
    const mesh = this.instancedMeshes.get(groupId);
    
    if (!mesh) return;
    
    // Get instances
    const instances = this.instanceData.get(groupId)!;
    
    // Update instance matrices
    instances.forEach((instance, index) => {
      mesh.setMatrixAt(index, instance.matrix);
    });
    
    // Mark instance matrix as needs update
    mesh.instanceMatrix.needsUpdate = true;
  }
  
  private updateInstanceMatrix(instance: InstanceData): void {
    // Create matrix from position, rotation, and scale
    instance.matrix.compose(
      instance.position,
      new THREE.Quaternion().setFromEuler(instance.rotation),
      instance.scale
    );
  }
}

interface InstanceData {
  id: string;
  position: THREE.Vector3;
  rotation: THREE.Euler;
  scale: THREE.Vector3;
  matrix: THREE.Matrix4;
  userData: any;
}
```
