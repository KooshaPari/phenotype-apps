/**
 * Domain models for configuration management.
 *
 * Following hexagonal architecture:
 * - Pure domain logic with no external dependencies
 * - Zod schemas for runtime validation
 * - Immutable config objects
 *
 * xDD Principles:
 * - KISS: Simple data classes
 * - DRY: Shared validation schemas
 * - PoLA: Descriptive error messages
 */

import { z } from 'zod';
import { isSecret, Secret } from './secret';

// =============================================================================
// Schema Definitions
// =============================================================================

/**
 * Base config value types.
 */
export const ConfigValueSchema = z.union([
  z.string(),
  z.number(),
  z.boolean(),
  z.array(z.string()),
  z.record(z.string(), z.string()),
]);

export type ConfigValue = z.infer<typeof ConfigValueSchema>;

/**
 * Config entries may also hold `Secret<T>` wrappers, which are runtime
 * objects (not parseable by Zod). They are accepted as values here so
 * in-memory configs can carry redaction-safe payloads; Zod validation
 * applies only to the wire shape.
 */
export type ConfigValueLike = ConfigValue | Secret<unknown>;

/**
 * Type guard: `true` when a value is a Secret wrapper.
 * Re-exported from `domain/secret.ts` for convenience.
 */
export { isSecret };

/**
 * Config entry with metadata.
 */
export const ConfigEntrySchema = z.object({
  key: z.string(),
  value: ConfigValueSchema,
  source: z.string().optional(),
  timestamp: z.number().optional(),
});

/**
 * Runtime config entry: accepts both Zod-validatable config values and
 * `Secret<T>` wrappers. Use this when working with entries that may
 * carry redaction-safe payloads (adapters, manager, in-memory state).
 */
export interface ConfigEntryLike {
  key: string;
  value: ConfigValueLike;
  source?: string;
  timestamp?: number;
}

export type ConfigEntry = z.infer<typeof ConfigEntrySchema>;

/**
 * Config snapshot - immutable config state.
 *
 * Secret handling:
 * - When a key is registered via `ImmutableConfig.secretKeys`, its value is
 *   replaced with `'[REDACTED]'` in the snapshot regardless of the underlying
 *   type. This keeps snapshots safe to log, persist, or transmit.
 */
export const ConfigSnapshotSchema = z.object({
  entries: z.record(z.union([ConfigValueSchema, z.literal('[REDACTED]')])),
  sources: z.array(z.string()),
  version: z.string(),
  timestamp: z.number(),
});

export type ConfigSnapshot = z.infer<typeof ConfigSnapshotSchema>;

/**
 * Marker string used in snapshots to redact sensitive values.
 * Exported so callers can grep for accidental leaks.
 */
export const REDACTED_PLACEHOLDER = '[REDACTED]';

/**
 * Validation error with context.
 */
export const ConfigErrorSchema = z.object({
  message: z.string(),
  path: z.array(z.string()),
  value: z.unknown(),
  context: z.record(z.unknown()).optional(),
});

export type ConfigError = z.infer<typeof ConfigErrorSchema>;

// =============================================================================
// Domain Classes
// =============================================================================

/**
 * Configuration validation error.
 *
 * Following PoLA (Principle of Least Astonishment):
 * - Descriptive error messages
 * - Path to the invalid field
 * - Original value for debugging
 */
export class ConfigValidationError extends Error {
  constructor(
    message: string,
    public readonly path: string[],
    public readonly value: unknown,
    public readonly context?: Record<string, unknown>
  ) {
    super(message);
    this.name = 'ConfigValidationError';
  }

  toJSON(): ConfigError {
    return {
      message: this.message,
      path: this.path,
      value: this.value,
      context: this.context,
    };
  }
}

/**
 * Config source not found error.
 */
export class ConfigSourceNotFoundError extends Error {
  constructor(
    message: string,
    public readonly source: string,
    public readonly context?: Record<string, unknown>
  ) {
    super(message);
    this.name = 'ConfigSourceNotFoundError';
  }
}

/**
 * Immutable configuration snapshot.
 *
 * Secret-key redaction:
 * - Construct with `secretKeys` to declare which keys hold sensitive values.
 * - `toSnapshot()` replaces those entries with `REDACTED_PLACEHOLDER`
 *   so the snapshot can be safely logged or transmitted.
 * - Entries whose value is a `Secret<T>` wrapper are ALWAYS replaced
 *   with `REDACTED_PLACEHOLDER` in the snapshot, regardless of whether
 *   the key is in `secretKeys` — defence in depth.
 */
export class ImmutableConfig {
  readonly secretKeys: ReadonlySet<string>;

  constructor(
    public readonly entries: ReadonlyMap<string, ConfigValueLike>,
    public readonly sources: ReadonlyArray<string>,
    public readonly version: string,
    secretKeys?: Iterable<string>
  ) {
    this.secretKeys = secretKeys ? new Set(secretKeys) : new Set();
  }

  get(key: string): ConfigValueLike | undefined {
    return this.entries.get(key);
  }

  has(key: string): boolean {
    return this.entries.has(key);
  }

  /**
   * Whether the given key is registered as a secret.
   */
  isSecret(key: string): boolean {
    return this.secretKeys.has(key);
  }

  toSnapshot(): ConfigSnapshot {
    const entries: Record<string, ConfigValue | typeof REDACTED_PLACEHOLDER> = {};
    for (const [key, value] of this.entries) {
      if (this.secretKeys.has(key) || isSecret(value)) {
        entries[key] = REDACTED_PLACEHOLDER;
      } else {
        entries[key] = value;
      }
    }
    return {
      entries,
      sources: [...this.sources],
      version: this.version,
      timestamp: Date.now(),
    };
  }
}
