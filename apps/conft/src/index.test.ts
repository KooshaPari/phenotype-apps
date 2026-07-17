import { afterEach, describe, expect, it } from 'vitest';
import { mkdtemp, writeFile } from 'fs/promises';
import { tmpdir } from 'os';
import { join } from 'path';
import {
  ConfigValidationError,
  type ConfigValue,
  ImmutableConfig,
  EnvConfigSource,
  FileConfigSource,
} from '.';

describe('config primitives', () => {
  afterEach(() => {
    delete process.env.APP_ENABLED;
    delete process.env.APP_COUNT;
    delete process.env.APP_FEATURES;
  });

  it('serializes validation errors with full context', () => {
    const error = new ConfigValidationError('Invalid config', ['server', 'port'], 0, {
      source: 'env',
    });

    expect(error.toJSON()).toEqual({
      message: 'Invalid config',
      path: ['server', 'port'],
      value: 0,
      context: { source: 'env' },
    });
  });

  it('reads environment config with typed value coercion', async () => {
    process.env.APP_ENABLED = 'true';
    process.env.APP_COUNT = '42';
    process.env.APP_FEATURES = '["a","b"]';

    const source = new EnvConfigSource();

    await expect(source.get('enabled')).resolves.toBe(true);
    await expect(source.get('count')).resolves.toBe(42);
    await expect(source.get('features')).resolves.toEqual(['a', 'b']);
  });

  it('reads file config entries', async () => {
    const dir = await mkdtemp(join(tmpdir(), 'conft-'));
    const file = join(dir, 'config.json');

    await writeFile(file, JSON.stringify({ apiUrl: 'https://example.test', retries: 3 }), 'utf-8');

    const source = new FileConfigSource(file);
    await expect(source.load()).resolves.toEqual([
      expect.objectContaining({
        key: 'apiUrl',
        value: 'https://example.test',
        source: 'file',
      }),
      expect.objectContaining({
        key: 'retries',
        value: 3,
        source: 'file',
      }),
    ]);
  });

  it('captures immutable config snapshots', () => {
    const config = new ImmutableConfig(
      new Map<string, ConfigValue>([
        ['feature', true],
        ['name', 'conft'],
      ]),
      ['env', 'file'],
      '1.0.0'
    );

    const snapshot = config.toSnapshot();

    expect(snapshot).toEqual({
      entries: { feature: true, name: 'conft' },
      sources: ['env', 'file'],
      version: '1.0.0',
      timestamp: expect.any(Number),
    });
  });
});
