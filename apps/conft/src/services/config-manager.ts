/**
 * ConfigManager — layered-precedence configuration facade.
 *
 * Combines multiple `ConfigSource` adapters into a single read API and
 * routes writes to the highest-priority writable source.
 *
 * Precedence model:
 * - `sources[0]` is the HIGHEST priority (overrides everything below).
 * - `sources[sources.length - 1]` is the LOWEST priority (defaults).
 * - On `get(key)`, the first source in priority order that returns a
 *   value wins.
 * - On `load()`, entries are merged with later sources overridden by
 *   earlier ones — equivalent to `Object.assign(lowest, ..., highest)`.
 *
 * This is the implementation referenced by the previous
 * "use ConfigManager" comment in `file-adapter.ts`.
 */

import {
  ConfigEntryLike,
  ConfigSourceNotFoundError,
  ConfigValueLike,
  ImmutableConfig,
} from '../domain/config';
import { ConfigSource } from '../ports/config-source';

export interface ConfigManagerOptions {
  /** Semantic version string attached to the snapshot. */
  version?: string;
  /** Keys whose values should be redacted in snapshots. */
  secretKeys?: Iterable<string>;
}

export class ConfigManager {
  readonly version: string;
  readonly secretKeys: ReadonlySet<string>;

  constructor(
    private readonly sources: ReadonlyArray<ConfigSource>,
    options: ConfigManagerOptions = {}
  ) {
    if (sources.length === 0) {
      throw new ConfigSourceNotFoundError(
        'ConfigManager requires at least one source',
        'manager'
      );
    }
    this.version = options.version ?? '0.0.0';
    this.secretKeys = options.secretKeys ? new Set(options.secretKeys) : new Set();
  }

  /** Read-only view of the underlying sources in priority order (highest first). */
  get sourceList(): ReadonlyArray<ConfigSource> {
    return this.sources;
  }

  /**
   * Load and merge entries from all sources. Earlier (higher-priority)
   * sources override later ones.
   *
   * Errors from individual sources are propagated — callers can wrap
   * the call in a Promise.allSettled if they want partial tolerance.
   */
  async load(): Promise<ImmutableConfig> {
    const merged = new Map<string, ConfigValueLike>();
    // Start from the lowest priority and let higher priorities overwrite.
    for (let i = this.sources.length - 1; i >= 0; i--) {
      const source = this.sources[i];
      const entries = await source.load();
      for (const entry of entries) {
        merged.set(entry.key, entry.value);
      }
    }
    return new ImmutableConfig(
      merged,
      this.sources.map((s) => s.name),
      this.version,
      this.secretKeys
    );
  }

  /**
   * Resolve a single key by walking the sources in priority order.
   * Returns `undefined` only when no source has the key.
   */
  async get(key: string): Promise<ConfigValueLike | undefined> {
    for (const source of this.sources) {
      const value = await source.get(key);
      if (value !== undefined) return value;
    }
    return undefined;
  }

  /**
   * Resolve multiple keys in one pass.
   */
  async getMany(keys: ReadonlyArray<string>): Promise<ReadonlyMap<string, ConfigValueLike>> {
    const out = new Map<string, ConfigValueLike>();
    for (const key of keys) {
      out.set(key, (await this.get(key)) as ConfigValueLike);
    }
    return out;
  }

  /**
   * Whether any source has the given key.
   */
  async has(key: string): Promise<boolean> {
    for (const source of this.sources) {
      const value = await source.get(key);
      if (value !== undefined) return true;
    }
    return false;
  }

  /**
   * Write a value to the highest-priority writable source.
   *
   * Fails closed: throws `ConfigSourceNotFoundError` when no source
   * accepts writes (e.g. all sources are read-only environments).
   *
   * Note: Secret<T> wrappers cannot be written through this path — they
   * are an in-memory redaction mechanism. Unwrap with `secret.reveal()`
   * before calling `set()`.
   */
  async set(key: string, value: ConfigValueLike): Promise<void> {
    for (const source of this.sources) {
      if (source.isWritable()) {
        // Source.set() takes ConfigValue (Zod-validatable); if a Secret
        // is passed we reveal() it as a fail-fast signal — Secrets should
        // not be persisted, only held in memory for redaction.
        if (typeof (value as { reveal?: () => unknown }).reveal === 'function') {
          throw new Error(
            `Refusing to persist Secret<unknown> through ConfigManager.set("${key}"); ` +
              `call secret.reveal() first.`
          );
        }
        await source.set(key, value as never);
        return;
      }
    }
    throw new ConfigSourceNotFoundError(
      `No writable source available for key "${key}"`,
      'manager',
      { key, sources: this.sources.map((s) => s.name) }
    );
  }

  /**
   * Build an `ImmutableConfig` snapshot from current source state and
   * emit a flat list of `ConfigEntryLike` records (one per merged key).
   */
  async entries(): Promise<ConfigEntryLike[]> {
    const config = await this.load();
    const now = Date.now();
    return Array.from(config.entries, ([key, value]) => ({
      key,
      value,
      source: this.sources[0].name,
      timestamp: now,
    }));
  }
}