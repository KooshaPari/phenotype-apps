/**
 * Tests for ConfigManager (layered-precedence facade).
 *
 * Closes the L8 dangling-reference gap: previously `file-adapter.ts:set`
 * said "use ConfigManager"; the manager is now implemented and exercised
 * here.
 */

import { afterEach, describe, expect, it } from 'vitest';
import {
  ConfigManager,
  ConfigValue,
  EnvConfigSource,
  FileConfigSource,
  ImmutableConfig,
  REDACTED_PLACEHOLDER,
} from '../index';
import { mkdtemp, writeFile } from 'fs/promises';
import { tmpdir } from 'os';
import { join } from 'path';
import { Secret } from '../domain/secret';

const ENV_KEYS = ['CM_ENABLED', 'CM_COUNT'];

afterEach(() => {
  for (const k of ENV_KEYS) delete process.env[k];
});

describe('ConfigManager — precedence', () => {
  it('highest-priority source overrides lower ones', async () => {
    const dir = await mkdtemp(join(tmpdir(), 'conft-mgr-'));
    const file = join(dir, 'config.json');
    await writeFile(file, JSON.stringify({ mode: 'file', retries: 3 }), 'utf-8');

    process.env.CM_ENABLED = 'true';
    process.env.CM_COUNT = '5';

    const manager = new ConfigManager([
      new EnvConfigSource('CM_'), // highest priority (overrides file)
      new FileConfigSource(file), // lowest priority (defaults)
    ]);

    // 'enabled' / 'count' come from env (higher priority).
    await expect(manager.get('enabled')).resolves.toBe(true);
    await expect(manager.get('count')).resolves.toBe(5);
    // 'mode' / 'retries' come from file (lower priority).
    await expect(manager.get('mode')).resolves.toBe('file');
    await expect(manager.get('retries')).resolves.toBe(3);
  });

  it('load() returns an ImmutableConfig with merged entries', async () => {
    const dir = await mkdtemp(join(tmpdir(), 'conft-mgr-'));
    const file = join(dir, 'config.json');
    await writeFile(file, JSON.stringify({ a: 'file-a', b: 'file-b' }), 'utf-8');
    process.env.CM_ENABLED = 'true';

    const manager = new ConfigManager([
      new EnvConfigSource('CM_'),
      new FileConfigSource(file),
    ]);

    const config = await manager.load();

    expect(config).toBeInstanceOf(ImmutableConfig);
    expect(config.get('a')).toBe('file-a');
    expect(config.get('enabled')).toBe(true);
    expect(config.sources).toEqual(['env', 'file']);
  });

  it('has() returns true only if some source has the key', async () => {
    const dir = await mkdtemp(join(tmpdir(), 'conft-mgr-'));
    const file = join(dir, 'config.json');
    await writeFile(file, JSON.stringify({ x: 1 }), 'utf-8');

    const manager = new ConfigManager([
      new EnvConfigSource('CM_'),
      new FileConfigSource(file),
    ]);

    await expect(manager.has('x')).resolves.toBe(true);
    await expect(manager.has('missing')).resolves.toBe(false);
  });

  it('getMany() resolves keys in one pass', async () => {
    process.env.CM_ENABLED = 'true';
    const dir = await mkdtemp(join(tmpdir(), 'conft-mgr-'));
    const file = join(dir, 'config.json');
    await writeFile(file, JSON.stringify({ mode: 'dev' }), 'utf-8');

    const manager = new ConfigManager([
      new EnvConfigSource('CM_'),
      new FileConfigSource(file),
    ]);

    const got = await manager.getMany(['enabled', 'mode', 'nope']);
    expect(got.get('enabled')).toBe(true);
    expect(got.get('mode')).toBe('dev');
    expect(got.get('nope')).toBeUndefined();
  });

  it('rejects construction with zero sources', () => {
    expect(() => new ConfigManager([])).toThrow(/at least one source/);
  });
});

describe('ConfigManager — secret redaction in snapshot', () => {
  it('redacts secret keys in the merged snapshot', async () => {
    const dir = await mkdtemp(join(tmpdir(), 'conft-mgr-'));
    const file = join(dir, 'config.json');
    await writeFile(
      file,
      JSON.stringify({ apiKey: 'sk-from-file', publicId: 'pub' }),
      'utf-8'
    );
    process.env.CM_ENABLED = 'true';

    const manager = new ConfigManager(
      [new EnvConfigSource('CM_'), new FileConfigSource(file)],
      { version: '1.2.3', secretKeys: ['apiKey', 'token'] }
    );

    const config = await manager.load();
    const snapshot = config.toSnapshot();

    expect(snapshot.version).toBe('1.2.3');
    expect(snapshot.entries['apiKey']).toBe(REDACTED_PLACEHOLDER);
    expect(snapshot.entries['publicId']).toBe('pub');
    expect(snapshot.entries['enabled']).toBe(true);

    // JSON.stringify of the snapshot must not leak the secret either.
    const serialised = JSON.stringify(snapshot);
    expect(serialised).not.toContain('sk-from-file');
    expect(serialised).toContain(REDACTED_PLACEHOLDER);
  });

  it('does not redact keys that are not registered as secret', async () => {
    const dir = await mkdtemp(join(tmpdir(), 'conft-mgr-'));
    const file = join(dir, 'config.json');
    await writeFile(file, JSON.stringify({ notSecret: 'visible' }), 'utf-8');

    const manager = new ConfigManager([new FileConfigSource(file)], {
      secretKeys: ['something-else'],
    });

    const config = await manager.load();
    expect(config.toSnapshot().entries['notSecret']).toBe('visible');
  });
});

describe('ConfigManager — write routing', () => {
  it('writes to the highest-priority writable source', async () => {
    const dir = await mkdtemp(join(tmpdir(), 'conft-mgr-'));
    const file = join(dir, 'config.json');
    await writeFile(file, JSON.stringify({}), 'utf-8');

    const manager = new ConfigManager([
      new EnvConfigSource('CM_'), // read-only
      new FileConfigSource(file), // writable, lower priority
    ]);

    // EnvConfigSource.set() throws — manager should skip it and write to file.
    await manager.set('newKey', 'newValue');

    // Re-reading via the file source confirms persistence.
    const src = new FileConfigSource(file);
    await expect(src.get('newKey')).resolves.toBe('newValue');
  });

  it('fails closed when no source is writable', async () => {
    process.env.CM_ENABLED = 'true';
    const manager = new ConfigManager([new EnvConfigSource('CM_')]);

    await expect(manager.set('x', 1)).rejects.toThrow(/No writable source/);
  });
});

describe('ConfigManager — Secret<unknown> entries in snapshot', () => {
  it('redacts a value wrapped in Secret even if the key is not in secretKeys', async () => {
    const secretValue = new Secret('hidden');
    const config = new ImmutableConfig(
      new Map<string, ConfigValue | Secret<string>>([
        ['apiKey', secretValue],
        ['name', 'visible' as ConfigValue],
      ]),
      ['memory'],
      '0.0.1'
    );

    // Secret-wrapped values are ALWAYS redacted in the snapshot, even
    // when the key isn't listed in secretKeys — defence in depth.
    const snapshot = config.toSnapshot();
    expect(snapshot.entries['apiKey']).toBe(REDACTED_PLACEHOLDER);
    expect(snapshot.entries['name']).toBe('visible');

    const serialised = JSON.stringify(snapshot);
    expect(serialised).toContain('[REDACTED]');
    expect(serialised).toContain('visible');
    expect(serialised).not.toContain('hidden');
  });
});