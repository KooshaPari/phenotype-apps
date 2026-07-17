/**
 * File-based configuration source adapter.
 *
 * Implements ConfigSource port for JSON config files.
 *
 * Performance:
 * - The first `load()` call reads + parses + caches the file.
 * - Subsequent `load()` and `get()` calls reuse the cached entries
 *   without touching disk.
 * - Call `invalidate()` to force a re-read on the next call (e.g. after
 *   a `set()` writes the file).
 * - `reload()` performs an immediate refresh and returns the new entries.
 *
 * Concurrency:
 * - A single in-flight `load()` is de-duplicated via `inflightLoad` so
 *   concurrent callers share one disk read.
 */

import { readFile, writeFile } from 'fs/promises';
import { ConfigSource } from '../ports/config-source';
import {
  ConfigEntry,
  ConfigSourceNotFoundError,
  ConfigValidationError,
  ConfigValue,
} from '../domain/config';

// Re-export the typed errors so consumers of `FileConfigSource` can
// import them from the adapter barrel directly.
export { ConfigSourceNotFoundError, ConfigValidationError };

export class FileConfigSource implements ConfigSource {
  readonly name = 'file';
  private readonly path: string;
  private cache: Map<string, ConfigValue> | null = null;
  private inflightLoad: Promise<Map<string, ConfigValue>> | null = null;
  /**
   * Number of times the file has been read from disk since construction
   * (or the last `resetReadCount()`). Useful for cache-de-duplication
   * assertions in tests; not intended for production telemetry.
   */
  private readCount = 0;

  constructor(path: string) {
    this.path = path;
  }

  async load(): Promise<ConfigEntry[]> {
    const entries = await this.getEntries();
    const now = Date.now();
    return Array.from(entries, ([key, value]) => ({
      key,
      value,
      source: this.name,
      timestamp: now,
    }));
  }

  async get(key: string): Promise<ConfigValue | undefined> {
    const entries = await this.getEntries();
    return entries.get(key);
  }

  /**
   * Replace the cached entries with an in-memory map. Useful for tests
   * and for callers that already have parsed data.
   */
  hydrate(entries: Iterable<readonly [string, ConfigValue]>): void {
    this.cache = new Map(entries);
  }

  /**
   * Drop the cached entries so the next call re-reads the file.
   */
  invalidate(): void {
    this.cache = null;
  }

  /**
   * Number of completed disk reads. Increments after each `readFromDisk`
   * call (cache misses / explicit invalidations).
   */
  getReadCount(): number {
    return this.readCount;
  }

  /**
   * Reset the read counter back to zero (does not touch the cache).
   */
  resetReadCount(): void {
    this.readCount = 0;
  }

  /**
   * Force a synchronous refresh and return the new entries.
   */
  async reload(): Promise<ConfigEntry[]> {
    this.invalidate();
    return this.load();
  }

  async set(key: string, value: ConfigValue): Promise<void> {
    const entries = await this.getEntries();
    entries.set(key, value);
    await this.persist(entries);
    // Persist succeeded — refresh cache so we serve the new value.
    this.cache = entries;
  }

  isWritable(): boolean {
    return true;
  }

  private async getEntries(): Promise<Map<string, ConfigValue>> {
    if (this.cache !== null) return this.cache;
    if (this.inflightLoad) return this.inflightLoad;
    this.inflightLoad = this.readFromDisk().finally(() => {
      this.inflightLoad = null;
    });
    return this.inflightLoad;
  }

  private async readFromDisk(): Promise<Map<string, ConfigValue>> {
    let content: string;
    try {
      content = await readFile(this.path, 'utf-8');
    } catch (err) {
      throw new ConfigSourceNotFoundError(
        `Failed to read config file: ${this.path}`,
        this.name,
        { path: this.path, cause: (err as Error).message }
      );
    }
    let parsed: unknown;
    try {
      parsed = JSON.parse(content);
    } catch (err) {
      throw new ConfigValidationError(
        `Failed to parse config file as JSON: ${this.path}`,
        [this.path],
        content,
        { source: this.name, cause: (err as Error).message }
      );
    }
    if (parsed === null || typeof parsed !== 'object' || Array.isArray(parsed)) {
      throw new ConfigValidationError(
        `Config file must contain a JSON object at the root: ${this.path}`,
        [this.path],
        parsed,
        { source: this.name }
      );
    }
    const entries = new Map<string, ConfigValue>();
    for (const [key, value] of Object.entries(parsed)) {
      entries.set(key, value as ConfigValue);
    }
    this.cache = entries;
    this.readCount += 1;
    return entries;
  }

  private async persist(entries: Map<string, ConfigValue>): Promise<void> {
    const obj: Record<string, ConfigValue> = {};
    for (const [key, value] of entries) obj[key] = value;
    await writeFile(this.path, JSON.stringify(obj, null, 2), 'utf-8');
  }
}