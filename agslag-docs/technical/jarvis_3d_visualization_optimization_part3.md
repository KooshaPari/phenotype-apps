# Jarvis SWE Agent 3D Visualization Optimization - Technical Implementation (Part 3)

## 7. Asset Management Optimization

Efficient asset management is crucial for 3D visualization performance, especially when dealing with complex scenes and large agent networks.

### 7.1 Asset Manager Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Asset Manager                          │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐       │
│  │  Loader     │   │  Cache      │   │  Texture    │       │
│  │  Manager    │   │  Manager    │   │  Atlas      │       │
│  └─────────────┘   └─────────────┘   └─────────────┘       │
│                                                             │
│  ┌─────────────┐   ┌─────────────┐   ┌─────────────┐       │
│  │  Model      │   │  Progressive│   │  Memory     │       │
│  │  Pool       │   │  Loader     │   │  Monitor    │       │
│  └─────────────┘   └─────────────┘   └─────────────┘       │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

### 7.2 Asset Manager Implementation

```typescript
class AssetManager {
  private loaderManager: LoaderManager;
  private cacheManager: CacheManager;
  private textureAtlas: TextureAtlas;
  private modelPool: ModelPool;
  private progressiveLoader: ProgressiveLoader;
  private memoryMonitor: MemoryMonitor;
  
  constructor() {
    this.loaderManager = new LoaderManager();
    this.cacheManager = new CacheManager();
    this.textureAtlas = new TextureAtlas();
    this.modelPool = new ModelPool();
    this.progressiveLoader = new ProgressiveLoader();
    this.memoryMonitor = new MemoryMonitor();
    
    // Initialize components
    this.initialize();
  }
  
  private initialize(): void {
    // Set up memory monitoring
    this.memoryMonitor.onMemoryPressure.add(this.handleMemoryPressure.bind(this));
    
    // Start monitoring
    this.memoryMonitor.startMonitoring();
  }
  
  public async loadModel(url: string, options: ModelLoadOptions = {}): Promise<THREE.Object3D> {
    // Check cache first
    const cachedModel = this.cacheManager.getModel(url);
    if (cachedModel) {
      return this.modelPool.getInstance(cachedModel);
    }
    
    // Set priority based on options
    const priority = options.priority || 'medium';
    
    // Load model
    const model = await this.progressiveLoader.loadModel(url, {
      ...options,
      priority
    });
    
    // Add to cache
    this.cacheManager.addModel(url, model);
    
    // Return instance
    return this.modelPool.getInstance(model);
  }
  
  public async loadTexture(url: string, options: TextureLoadOptions = {}): Promise<THREE.Texture> {
    // Check if texture is in atlas
    if (options.useAtlas !== false) {
      const atlasTexture = this.textureAtlas.getTexture(url);
      if (atlasTexture) {
        return atlasTexture;
      }
    }
    
    // Check cache
    const cachedTexture = this.cacheManager.getTexture(url);
    if (cachedTexture) {
      return cachedTexture;
    }
    
    // Set priority based on options
    const priority = options.priority || 'medium';
    
    // Load texture
    const texture = await this.progressiveLoader.loadTexture(url, {
      ...options,
      priority
    });
    
    // Add to cache
    this.cacheManager.addTexture(url, texture);
    
    // Add to atlas if requested
    if (options.useAtlas !== false) {
      this.textureAtlas.addTexture(url, texture);
    }
    
    return texture;
  }
  
  public releaseModel(model: THREE.Object3D): void {
    this.modelPool.releaseInstance(model);
  }
  
  public preloadAssets(assets: PreloadAsset[]): void {
    // Queue assets for preloading
    assets.forEach(asset => {
      if (asset.type === 'model') {
        this.progressiveLoader.queueModel(asset.url, {
          priority: asset.priority || 'low',
          lod: asset.lod || 0
        });
      } else if (asset.type === 'texture') {
        this.progressiveLoader.queueTexture(asset.url, {
          priority: asset.priority || 'low'
        });
      }
    });
  }
  
  public optimizeMemory(): void {
    // Trim caches based on usage
    this.cacheManager.trimCache();
    
    // Release unused model instances
    this.modelPool.releaseUnused();
    
    // Optimize texture atlas
    this.textureAtlas.optimize();
  }
  
  private handleMemoryPressure(level: 'low' | 'medium' | 'high'): void {
    console.log(`Memory pressure detected: ${level}`);
    
    // Take action based on pressure level
    if (level === 'low') {
      // Light optimization
      this.cacheManager.trimCache(0.9); // Keep 90% of cache
    } else if (level === 'medium') {
      // Moderate optimization
      this.cacheManager.trimCache(0.7); // Keep 70% of cache
      this.modelPool.releaseUnused();
    } else if (level === 'high') {
      // Aggressive optimization
      this.cacheManager.trimCache(0.5); // Keep 50% of cache
      this.modelPool.releaseUnused();
      this.textureAtlas.optimize();
      
      // Force garbage collection if possible
      if (window.gc) {
        window.gc();
      }
    }
  }
  
  public dispose(): void {
    // Dispose all resources
    this.cacheManager.clear();
    this.modelPool.clear();
    this.textureAtlas.dispose();
    this.memoryMonitor.stopMonitoring();
  }
}

interface ModelLoadOptions {
  priority?: 'high' | 'medium' | 'low';
  lod?: number;
  onProgress?: (progress: number) => void;
}

interface TextureLoadOptions {
  priority?: 'high' | 'medium' | 'low';
  useAtlas?: boolean;
  onProgress?: (progress: number) => void;
}

interface PreloadAsset {
  type: 'model' | 'texture';
  url: string;
  priority?: 'high' | 'medium' | 'low';
  lod?: number;
}
```

### 7.3 Loader Manager

```typescript
class LoaderManager {
  private loaders: Map<string, THREE.Loader> = new Map();
  private loadingManager: THREE.LoadingManager;
  
  constructor() {
    // Create loading manager
    this.loadingManager = new THREE.LoadingManager();
    
    // Set up default loaders
    this.setupDefaultLoaders();
  }
  
  private setupDefaultLoaders(): void {
    // GLTF loader
    const gltfLoader = new THREE.GLTFLoader(this.loadingManager);
    this.loaders.set('gltf', gltfLoader);
    this.loaders.set('glb', gltfLoader);
    
    // OBJ loader
    const objLoader = new THREE.OBJLoader(this.loadingManager);
    this.loaders.set('obj', objLoader);
    
    // FBX loader
    const fbxLoader = new THREE.FBXLoader(this.loadingManager);
    this.loaders.set('fbx', fbxLoader);
    
    // Texture loader
    const textureLoader = new THREE.TextureLoader(this.loadingManager);
    this.loaders.set('jpg', textureLoader);
    this.loaders.set('jpeg', textureLoader);
    this.loaders.set('png', textureLoader);
    this.loaders.set('webp', textureLoader);
  }
  
  public getLoader(extension: string): THREE.Loader | null {
    return this.loaders.get(extension.toLowerCase()) || null;
  }
  
  public registerLoader(extension: string, loader: THREE.Loader): void {
    this.loaders.set(extension.toLowerCase(), loader);
  }
  
  public async loadModel(url: string): Promise<THREE.Object3D> {
    // Get file extension
    const extension = this.getFileExtension(url);
    
    // Get loader for extension
    const loader = this.getLoader(extension);
    
    if (!loader) {
      throw new Error(`No loader found for extension: ${extension}`);
    }
    
    // Load model
    return new Promise((resolve, reject) => {
      loader.load(
        url,
        (result) => {
          // Handle different loader result types
          if (result.scene) {
            // GLTF result
            resolve(result.scene);
          } else {
            // Other loaders
            resolve(result);
          }
        },
        undefined,
        (error) => {
          reject(error);
        }
      );
    });
  }
  
  public async loadTexture(url: string): Promise<THREE.Texture> {
    // Get file extension
    const extension = this.getFileExtension(url);
    
    // Get loader for extension
    const loader = this.getLoader(extension);
    
    if (!loader) {
      throw new Error(`No loader found for extension: ${extension}`);
    }
    
    // Load texture
    return new Promise((resolve, reject) => {
      loader.load(
        url,
        (texture) => {
          resolve(texture);
        },
        undefined,
        (error) => {
          reject(error);
        }
      );
    });
  }
  
  private getFileExtension(url: string): string {
    return url.split('.').pop() || '';
  }
}
```

### 7.4 Cache Manager

```typescript
class CacheManager {
  private modelCache: Map<string, CacheEntry<THREE.Object3D>> = new Map();
  private textureCache: Map<string, CacheEntry<THREE.Texture>> = new Map();
  private maxCacheSize: number = 100; // Maximum number of entries
  
  constructor(maxCacheSize: number = 100) {
    this.maxCacheSize = maxCacheSize;
  }
  
  public addModel(url: string, model: THREE.Object3D): void {
    // Check if cache is full
    if (this.modelCache.size >= this.maxCacheSize) {
      this.trimModelCache();
    }
    
    // Add to cache
    this.modelCache.set(url, {
      asset: model,
      lastAccessed: Date.now(),
      accessCount: 0
    });
  }
  
  public getModel(url: string): THREE.Object3D | null {
    // Get from cache
    const entry = this.modelCache.get(url);
    
    if (!entry) return null;
    
    // Update access time and count
    entry.lastAccessed = Date.now();
    entry.accessCount++;
    
    return entry.asset;
  }
  
  public addTexture(url: string, texture: THREE.Texture): void {
    // Check if cache is full
    if (this.textureCache.size >= this.maxCacheSize) {
      this.trimTextureCache();
    }
    
    // Add to cache
    this.textureCache.set(url, {
      asset: texture,
      lastAccessed: Date.now(),
      accessCount: 0
    });
  }
  
  public getTexture(url: string): THREE.Texture | null {
    // Get from cache
    const entry = this.textureCache.get(url);
    
    if (!entry) return null;
    
    // Update access time and count
    entry.lastAccessed = Date.now();
    entry.accessCount++;
    
    return entry.asset;
  }
  
  public trimCache(keepRatio: number = 0.7): void {
    this.trimModelCache(keepRatio);
    this.trimTextureCache(keepRatio);
  }
  
  private trimModelCache(keepRatio: number = 0.7): void {
    // Calculate number of entries to keep
    const keepCount = Math.floor(this.maxCacheSize * keepRatio);
    
    // If cache is smaller than keep count, do nothing
    if (this.modelCache.size <= keepCount) return;
    
    // Sort entries by last accessed time and access count
    const entries = Array.from(this.modelCache.entries()).map(([url, entry]) => ({
      url,
      lastAccessed: entry.lastAccessed,
      accessCount: entry.accessCount
    }));
    
    // Sort by score (combination of recency and frequency)
    entries.sort((a, b) => {
      const scoreA = a.lastAccessed + a.accessCount * 10000;
      const scoreB = b.lastAccessed + b.accessCount * 10000;
      return scoreA - scoreB;
    });
    
    // Remove oldest entries
    const removeCount = this.modelCache.size - keepCount;
    const removeEntries = entries.slice(0, removeCount);
    
    // Remove from cache
    removeEntries.forEach(entry => {
      this.modelCache.delete(entry.url);
    });
  }
  
  private trimTextureCache(keepRatio: number = 0.7): void {
    // Calculate number of entries to keep
    const keepCount = Math.floor(this.maxCacheSize * keepRatio);
    
    // If cache is smaller than keep count, do nothing
    if (this.textureCache.size <= keepCount) return;
    
    // Sort entries by last accessed time and access count
    const entries = Array.from(this.textureCache.entries()).map(([url, entry]) => ({
      url,
      lastAccessed: entry.lastAccessed,
      accessCount: entry.accessCount
    }));
    
    // Sort by score (combination of recency and frequency)
    entries.sort((a, b) => {
      const scoreA = a.lastAccessed + a.accessCount * 10000;
      const scoreB = b.lastAccessed + b.accessCount * 10000;
      return scoreA - scoreB;
    });
    
    // Remove oldest entries
    const removeCount = this.textureCache.size - keepCount;
    const removeEntries = entries.slice(0, removeCount);
    
    // Remove from cache
    removeEntries.forEach(entry => {
      const texture = this.textureCache.get(entry.url)?.asset;
      if (texture) {
        texture.dispose();
      }
      this.textureCache.delete(entry.url);
    });
  }
  
  public clear(): void {
    // Dispose all textures
    this.textureCache.forEach(entry => {
      entry.asset.dispose();
    });
    
    // Clear caches
    this.modelCache.clear();
    this.textureCache.clear();
  }
}

interface CacheEntry<T> {
  asset: T;
  lastAccessed: number;
  accessCount: number;
}
```
