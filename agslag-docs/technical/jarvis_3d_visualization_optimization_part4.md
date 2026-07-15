# Jarvis SWE Agent 3D Visualization Optimization - Technical Implementation (Part 4)

## 7. Asset Management Optimization (Continued)

### 7.5 Texture Atlas

The Texture Atlas combines multiple textures into a single larger texture to reduce draw calls and improve performance.

```typescript
class TextureAtlas {
  private atlas: THREE.Texture | null = null;
  private atlasSize: number = 2048;
  private textures: Map<string, AtlasEntry> = new Map();
  private dirty: boolean = false;
  private packer: RectanglePacker;
  
  constructor(atlasSize: number = 2048) {
    this.atlasSize = atlasSize;
    this.packer = new RectanglePacker(atlasSize, atlasSize);
    
    // Create empty atlas texture
    this.createAtlasTexture();
  }
  
  private createAtlasTexture(): void {
    // Create canvas
    const canvas = document.createElement('canvas');
    canvas.width = this.atlasSize;
    canvas.height = this.atlasSize;
    
    // Get context
    const context = canvas.getContext('2d');
    
    if (!context) {
      console.error('Failed to get 2D context for texture atlas');
      return;
    }
    
    // Clear canvas
    context.fillStyle = 'rgba(0, 0, 0, 0)';
    context.fillRect(0, 0, this.atlasSize, this.atlasSize);
    
    // Create texture from canvas
    this.atlas = new THREE.Texture(canvas);
    this.atlas.needsUpdate = true;
  }
  
  public addTexture(url: string, texture: THREE.Texture): boolean {
    // Skip if already in atlas
    if (this.textures.has(url)) {
      return true;
    }
    
    // Get texture image
    const image = texture.image;
    
    if (!image) {
      console.error('Texture has no image:', url);
      return false;
    }
    
    // Find space in atlas
    const width = image.width || 64;
    const height = image.height || 64;
    
    const rect = this.packer.pack(width, height);
    
    if (!rect) {
      console.warn('No space in texture atlas for:', url);
      return false;
    }
    
    // Add to atlas
    const canvas = this.atlas!.image as HTMLCanvasElement;
    const context = canvas.getContext('2d');
    
    if (!context) {
      console.error('Failed to get 2D context for texture atlas');
      return false;
    }
    
    // Draw texture to atlas
    context.drawImage(image, rect.x, rect.y, rect.width, rect.height);
    
    // Create UV coordinates
    const u0 = rect.x / this.atlasSize;
    const v0 = rect.y / this.atlasSize;
    const u1 = (rect.x + rect.width) / this.atlasSize;
    const v1 = (rect.y + rect.height) / this.atlasSize;
    
    // Store atlas entry
    this.textures.set(url, {
      rect,
      uv: new THREE.Vector4(u0, v0, u1, v1)
    });
    
    // Mark atlas as dirty
    this.dirty = true;
    
    return true;
  }
  
  public getTexture(url: string): THREE.Texture | null {
    // Check if texture is in atlas
    const entry = this.textures.get(url);
    
    if (!entry || !this.atlas) {
      return null;
    }
    
    // Update atlas if dirty
    if (this.dirty) {
      this.atlas.needsUpdate = true;
      this.dirty = false;
    }
    
    // Create a new texture that references the atlas
    const texture = this.atlas.clone();
    
    // Set UV transform
    texture.offset.set(entry.uv.x, entry.uv.y);
    texture.repeat.set(
      entry.uv.z - entry.uv.x,
      entry.uv.w - entry.uv.y
    );
    
    return texture;
  }
  
  public optimize(): void {
    // If no textures, nothing to optimize
    if (this.textures.size === 0) {
      return;
    }
    
    // Get usage statistics
    const entries = Array.from(this.textures.entries());
    
    // Sort by usage (placeholder - in a real implementation, would track usage)
    entries.sort((a, b) => {
      // Placeholder sorting logic
      return 0;
    });
    
    // If atlas is more than 25% empty, repack
    const usedArea = Array.from(this.textures.values()).reduce(
      (total, entry) => total + (entry.rect.width * entry.rect.height),
      0
    );
    
    const totalArea = this.atlasSize * this.atlasSize;
    const usageRatio = usedArea / totalArea;
    
    if (usageRatio < 0.75) {
      this.repackAtlas();
    }
  }
  
  private repackAtlas(): void {
    // Create new packer
    this.packer = new RectanglePacker(this.atlasSize, this.atlasSize);
    
    // Get all textures
    const entries = Array.from(this.textures.entries());
    
    // Clear texture map
    this.textures.clear();
    
    // Create new atlas texture
    this.createAtlasTexture();
    
    // Re-add textures
    for (const [url, entry] of entries) {
      // Try to add texture
      const rect = this.packer.pack(entry.rect.width, entry.rect.height);
      
      if (!rect) {
        console.warn('Failed to repack texture:', url);
        continue;
      }
      
      // Add to atlas
      const canvas = this.atlas!.image as HTMLCanvasElement;
      const context = canvas.getContext('2d');
      
      if (!context) {
        console.error('Failed to get 2D context for texture atlas');
        continue;
      }
      
      // Draw texture to atlas (placeholder - would need original image)
      // In a real implementation, would keep references to original images
      
      // Create UV coordinates
      const u0 = rect.x / this.atlasSize;
      const v0 = rect.y / this.atlasSize;
      const u1 = (rect.x + rect.width) / this.atlasSize;
      const v1 = (rect.y + rect.height) / this.atlasSize;
      
      // Store atlas entry
      this.textures.set(url, {
        rect,
        uv: new THREE.Vector4(u0, v0, u1, v1)
      });
    }
    
    // Mark atlas as dirty
    this.dirty = true;
  }
  
  public dispose(): void {
    // Dispose atlas texture
    if (this.atlas) {
      this.atlas.dispose();
      this.atlas = null;
    }
    
    // Clear textures
    this.textures.clear();
  }
}

interface AtlasEntry {
  rect: {
    x: number;
    y: number;
    width: number;
    height: number;
  };
  uv: THREE.Vector4;
}

class RectanglePacker {
  private width: number;
  private height: number;
  private root: PackerNode | null = null;
  
  constructor(width: number, height: number) {
    this.width = width;
    this.height = height;
  }
  
  public pack(width: number, height: number): { x: number; y: number; width: number; height: number } | null {
    // Ensure dimensions are valid
    if (width <= 0 || height <= 0 || width > this.width || height > this.height) {
      return null;
    }
    
    // If no root node, create one
    if (!this.root) {
      this.root = {
        x: 0,
        y: 0,
        width: this.width,
        height: this.height,
        used: false,
        right: null,
        down: null
      };
    }
    
    // Find node for rectangle
    const node = this.findNode(this.root, width, height);
    
    if (!node) {
      return null;
    }
    
    // Split node
    this.splitNode(node, width, height);
    
    return {
      x: node.x,
      y: node.y,
      width,
      height
    };
  }
  
  private findNode(node: PackerNode, width: number, height: number): PackerNode | null {
    // If node is used, try children
    if (node.used) {
      const rightNode = node.right ? this.findNode(node.right, width, height) : null;
      const downNode = !rightNode && node.down ? this.findNode(node.down, width, height) : null;
      
      return rightNode || downNode;
    }
    
    // If node is too small, return null
    if (node.width < width || node.height < height) {
      return null;
    }
    
    // Node fits, return it
    return node;
  }
  
  private splitNode(node: PackerNode, width: number, height: number): void {
    // Mark node as used
    node.used = true;
    
    // Create right child
    node.right = {
      x: node.x + width,
      y: node.y,
      width: node.width - width,
      height,
      used: false,
      right: null,
      down: null
    };
    
    // Create down child
    node.down = {
      x: node.x,
      y: node.y + height,
      width: node.width,
      height: node.height - height,
      used: false,
      right: null,
      down: null
    };
  }
}

interface PackerNode {
  x: number;
  y: number;
  width: number;
  height: number;
  used: boolean;
  right: PackerNode | null;
  down: PackerNode | null;
}
```

### 7.6 Model Pool

The Model Pool manages instances of 3D models to enable efficient reuse and reduce memory usage.

```typescript
class ModelPool {
  private templates: Map<string, THREE.Object3D> = new Map();
  private instances: Map<string, PooledInstance[]> = new Map();
  private instanceIdCounter: number = 0;
  
  constructor() {
    // Initialize maps
  }
  
  public addTemplate(id: string, model: THREE.Object3D): void {
    // Store template
    this.templates.set(id, model);
    
    // Initialize instance array
    if (!this.instances.has(id)) {
      this.instances.set(id, []);
    }
  }
  
  public getInstance(templateOrId: string | THREE.Object3D): THREE.Object3D {
    let templateId: string;
    let template: THREE.Object3D;
    
    // Get template ID and model
    if (typeof templateOrId === 'string') {
      templateId = templateOrId;
      template = this.templates.get(templateId)!;
      
      if (!template) {
        throw new Error(`Template not found: ${templateId}`);
      }
    } else {
      // Generate ID for new template
      templateId = `template_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      template = templateOrId;
      
      // Add as template
      this.addTemplate(templateId, template);
    }
    
    // Check for available instance
    const instanceArray = this.instances.get(templateId)!;
    const availableInstance = instanceArray.find(instance => !instance.inUse);
    
    if (availableInstance) {
      // Reuse existing instance
      availableInstance.inUse = true;
      availableInstance.lastUsed = Date.now();
      
      return availableInstance.instance;
    }
    
    // Create new instance
    const instance = this.createInstance(template);
    const instanceId = `instance_${this.instanceIdCounter++}`;
    
    // Add to instance array
    instanceArray.push({
      id: instanceId,
      instance,
      inUse: true,
      created: Date.now(),
      lastUsed: Date.now()
    });
    
    return instance;
  }
  
  public releaseInstance(instance: THREE.Object3D): void {
    // Find instance in pool
    for (const [templateId, instanceArray] of this.instances.entries()) {
      const pooledInstance = instanceArray.find(i => i.instance === instance);
      
      if (pooledInstance) {
        // Mark as not in use
        pooledInstance.inUse = false;
        pooledInstance.lastUsed = Date.now();
        return;
      }
    }
    
    // Instance not found in pool, just dispose it
    this.disposeObject(instance);
  }
  
  public releaseUnused(maxAge: number = 60000): void {
    const now = Date.now();
    
    // Check each template
    for (const [templateId, instanceArray] of this.instances.entries()) {
      // Find unused instances older than maxAge
      const oldInstances = instanceArray.filter(
        instance => !instance.inUse && (now - instance.lastUsed) > maxAge
      );
      
      // Remove old instances
      oldInstances.forEach(instance => {
        // Dispose instance
        this.disposeObject(instance.instance);
        
        // Remove from array
        const index = instanceArray.indexOf(instance);
        if (index !== -1) {
          instanceArray.splice(index, 1);
        }
      });
    }
  }
  
  public clear(): void {
    // Dispose all instances
    for (const instanceArray of this.instances.values()) {
      instanceArray.forEach(instance => {
        this.disposeObject(instance.instance);
      });
    }
    
    // Clear maps
    this.instances.clear();
    this.templates.clear();
  }
  
  private createInstance(template: THREE.Object3D): THREE.Object3D {
    // Clone template
    const instance = template.clone();
    
    // Reset transform
    instance.position.set(0, 0, 0);
    instance.rotation.set(0, 0, 0);
    instance.scale.set(1, 1, 1);
    
    return instance;
  }
  
  private disposeObject(object: THREE.Object3D): void {
    // Dispose geometries and materials
    object.traverse(child => {
      if (child instanceof THREE.Mesh) {
        if (child.geometry) {
          child.geometry.dispose();
        }
        
        if (child.material) {
          if (Array.isArray(child.material)) {
            child.material.forEach(material => material.dispose());
          } else {
            child.material.dispose();
          }
        }
      }
    });
  }
}

interface PooledInstance {
  id: string;
  instance: THREE.Object3D;
  inUse: boolean;
  created: number;
  lastUsed: number;
}
```

### 7.7 Progressive Loader

The Progressive Loader manages the loading of assets with priority-based queuing and progressive loading for large models.

```typescript
class ProgressiveLoader {
  private loaderManager: LoaderManager;
  private modelQueue: PriorityQueue<QueuedModel> = new PriorityQueue();
  private textureQueue: PriorityQueue<QueuedTexture> = new PriorityQueue();
  private loading: boolean = false;
  private maxConcurrentLoads: number = 3;
  private currentLoads: number = 0;
  
  constructor() {
    this.loaderManager = new LoaderManager();
    
    // Start processing queue
    this.processQueue();
  }
  
  public queueModel(url: string, options: ModelLoadOptions = {}): void {
    // Add to queue
    this.modelQueue.enqueue({
      url,
      options,
      priority: this.getPriorityValue(options.priority || 'medium')
    });
  }
  
  public queueTexture(url: string, options: TextureLoadOptions = {}): void {
    // Add to queue
    this.textureQueue.enqueue({
      url,
      options,
      priority: this.getPriorityValue(options.priority || 'medium')
    });
  }
  
  public async loadModel(url: string, options: ModelLoadOptions = {}): Promise<THREE.Object3D> {
    // If high priority, load immediately
    if (options.priority === 'high') {
      return this.loaderManager.loadModel(url);
    }
    
    // Otherwise, queue and wait for result
    return new Promise((resolve, reject) => {
      this.queueModel(url, {
        ...options,
        onLoad: (model) => {
          resolve(model);
        },
        onError: (error) => {
          reject(error);
        }
      });
    });
  }
  
  public async loadTexture(url: string, options: TextureLoadOptions = {}): Promise<THREE.Texture> {
    // If high priority, load immediately
    if (options.priority === 'high') {
      return this.loaderManager.loadTexture(url);
    }
    
    // Otherwise, queue and wait for result
    return new Promise((resolve, reject) => {
      this.queueTexture(url, {
        ...options,
        onLoad: (texture) => {
          resolve(texture);
        },
        onError: (error) => {
          reject(error);
        }
      });
    });
  }
  
  private async processQueue(): Promise<void> {
    // If already processing, do nothing
    if (this.loading) {
      return;
    }
    
    // Mark as loading
    this.loading = true;
    
    // Process queue until empty
    while (!this.modelQueue.isEmpty() || !this.textureQueue.isEmpty()) {
      // If at max concurrent loads, wait
      if (this.currentLoads >= this.maxConcurrentLoads) {
        await new Promise(resolve => setTimeout(resolve, 100));
        continue;
      }
      
      // Determine which queue to process
      let item: QueuedModel | QueuedTexture | null = null;
      
      if (!this.modelQueue.isEmpty() && !this.textureQueue.isEmpty()) {
        // Compare priorities
        const modelPriority = this.modelQueue.peek()!.priority;
        const texturePriority = this.textureQueue.peek()!.priority;
        
        if (modelPriority >= texturePriority) {
          item = this.modelQueue.dequeue()!;
        } else {
          item = this.textureQueue.dequeue()!;
        }
      } else if (!this.modelQueue.isEmpty()) {
        item = this.modelQueue.dequeue()!;
      } else if (!this.textureQueue.isEmpty()) {
        item = this.textureQueue.dequeue()!;
      }
      
      // If no item, break
      if (!item) {
        break;
      }
      
      // Increment current loads
      this.currentLoads++;
      
      // Process item
      if ('lod' in item) {
        // Model
        this.processModelItem(item as QueuedModel);
      } else {
        // Texture
        this.processTextureItem(item as QueuedTexture);
      }
    }
    
    // Mark as not loading
    this.loading = false;
  }
  
  private async processModelItem(item: QueuedModel): Promise<void> {
    try {
      // Load model
      const model = await this.loaderManager.loadModel(item.url);
      
      // Call onLoad callback
      if (item.options.onLoad) {
        item.options.onLoad(model);
      }
    } catch (error) {
      // Call onError callback
      if (item.options.onError) {
        item.options.onError(error);
      } else {
        console.error('Error loading model:', item.url, error);
      }
    } finally {
      // Decrement current loads
      this.currentLoads--;
      
      // Continue processing queue
      this.processQueue();
    }
  }
  
  private async processTextureItem(item: QueuedTexture): Promise<void> {
    try {
      // Load texture
      const texture = await this.loaderManager.loadTexture(item.url);
      
      // Call onLoad callback
      if (item.options.onLoad) {
        item.options.onLoad(texture);
      }
    } catch (error) {
      // Call onError callback
      if (item.options.onError) {
        item.options.onError(error);
      } else {
        console.error('Error loading texture:', item.url, error);
      }
    } finally {
      // Decrement current loads
      this.currentLoads--;
      
      // Continue processing queue
      this.processQueue();
    }
  }
  
  private getPriorityValue(priority: 'high' | 'medium' | 'low'): number {
    switch (priority) {
      case 'high':
        return 100;
      case 'medium':
        return 50;
      case 'low':
        return 10;
      default:
        return 0;
    }
  }
}

interface QueuedModel {
  url: string;
  options: ModelLoadOptions & {
    onLoad?: (model: THREE.Object3D) => void;
    onError?: (error: any) => void;
  };
  priority: number;
}

interface QueuedTexture {
  url: string;
  options: TextureLoadOptions & {
    onLoad?: (texture: THREE.Texture) => void;
    onError?: (error: any) => void;
  };
  priority: number;
}

class PriorityQueue<T extends { priority: number }> {
  private items: T[] = [];
  
  public enqueue(item: T): void {
    // Add item to queue
    this.items.push(item);
    
    // Sort by priority (descending)
    this.items.sort((a, b) => b.priority - a.priority);
  }
  
  public dequeue(): T | null {
    if (this.isEmpty()) {
      return null;
    }
    
    return this.items.shift()!;
  }
  
  public peek(): T | null {
    if (this.isEmpty()) {
      return null;
    }
    
    return this.items[0];
  }
  
  public isEmpty(): boolean {
    return this.items.length === 0;
  }
  
  public size(): number {
    return this.items.length;
  }
}
```
