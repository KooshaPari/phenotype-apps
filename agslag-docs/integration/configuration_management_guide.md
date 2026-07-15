# Configuration Management Guide

## Overview

This guide details the configuration management approach for integrating Fluujo, OpenManus, and manus-open into the AGSLAG MCP Integration system. A unified configuration system is essential for managing settings across all integrated components.

## Configuration Structure

We'll create a unified configuration system that can manage settings for all integrated components:

```typescript
// config/unified_config.ts
export interface UnifiedConfig {
  // AGSLAG configuration
  agslag: {
    apiKey?: string;
    baseUrl?: string;
    timeout?: number;
    debug?: boolean;
  };
  
  // Fluujo configuration
  fluujo: {
    baseUrl: string;
    serverDir: string;
    transport: 'stdio' | 'websocket';
    websocketUrl?: string;
    env: Record<string, string>;
  };
  
  // OpenManus configuration
  openManus: {
    baseUrl: string;
    teamMembers: string[];
    reasoningModel: string;
    basicModel: string;
    visionModel: string;
    chromeHeadless: boolean;
  };
  
  // manus-open configuration
  manusOpen: {
    baseUrl: string;
    runtimeApiHost: string;
    chromeInstancePath: string;
  };
}
```

## Configuration Implementation

### 1. Create Configuration Manager

```typescript
// config/config_manager.ts
import { UnifiedConfig } from './unified_config';
import * as fs from 'fs';
import * as path from 'path';

export class ConfigManager {
  private static instance: ConfigManager;
  private config: UnifiedConfig;
  private configPath: string;
  
  private constructor(configPath: string) {
    this.configPath = configPath;
    this.loadConfig();
  }
  
  public static getInstance(configPath?: string): ConfigManager {
    if (!ConfigManager.instance) {
      if (!configPath) {
        configPath = path.join(process.cwd(), 'config', 'config.json');
      }
      ConfigManager.instance = new ConfigManager(configPath);
    }
    return ConfigManager.instance;
  }
  
  private loadConfig(): void {
    try {
      if (fs.existsSync(this.configPath)) {
        const configData = fs.readFileSync(this.configPath, 'utf8');
        this.config = JSON.parse(configData);
      } else {
        this.config = this.getDefaultConfig();
        this.saveConfig();
      }
    } catch (error) {
      console.error('Error loading configuration:', error);
      this.config = this.getDefaultConfig();
    }
  }
  
  private saveConfig(): void {
    try {
      const configDir = path.dirname(this.configPath);
      if (!fs.existsSync(configDir)) {
        fs.mkdirSync(configDir, { recursive: true });
      }
      fs.writeFileSync(this.configPath, JSON.stringify(this.config, null, 2));
    } catch (error) {
      console.error('Error saving configuration:', error);
    }
  }
  
  private getDefaultConfig(): UnifiedConfig {
    return {
      agslag: {
        timeout: 30000,
        debug: false
      },
      fluujo: {
        baseUrl: 'http://localhost:3000',
        serverDir: 'mcp-servers',
        transport: 'stdio',
        env: {}
      },
      openManus: {
        baseUrl: 'http://localhost:8000',
        teamMembers: ['researcher', 'coder', 'browser', 'reporter'],
        reasoningModel: 'gpt-4',
        basicModel: 'gpt-3.5-turbo',
        visionModel: 'gpt-4-vision-preview',
        chromeHeadless: true
      },
      manusOpen: {
        baseUrl: 'http://localhost:8330',
        runtimeApiHost: 'http://localhost',
        chromeInstancePath: '/usr/bin/chromium'
      }
    };
  }
  
  public getConfig(): UnifiedConfig {
    return this.config;
  }
  
  public updateConfig(newConfig: Partial<UnifiedConfig>): void {
    this.config = {
      ...this.config,
      ...newConfig
    };
    this.saveConfig();
  }
  
  public getAgslagConfig() {
    return this.config.agslag;
  }
  
  public getFluujoConfig() {
    return this.config.fluujo;
  }
  
  public getOpenManusConfig() {
    return this.config.openManus;
  }
  
  public getManusOpenConfig() {
    return this.config.manusOpen;
  }
}
```

### 2. Update AGSLAG MCP Integration to Use Configuration Manager

```typescript
// integration/agslag_mcp_integration.ts
import { ConfigManager } from '../config/config_manager';
import { AgslagFluujoAdapter } from '../adapters/fluujo_adapter';
import { AgslagOpenManusAdapter } from '../adapters/openmanus_adapter';
import { AgslagManusOpenAdapter } from '../adapters/manus_open_adapter';

export class AgslagMcpIntegration {
  // Existing adapters...
  
  private fluujo: AgslagFluujoAdapter;
  private openManus: AgslagOpenManusAdapter;
  private manusOpen: AgslagManusOpenAdapter;
  private configManager: ConfigManager;
  
  constructor(configPath?: string) {
    // Initialize configuration manager
    this.configManager = ConfigManager.getInstance(configPath);
    
    // Initialize existing adapters...
    
    // Initialize new adapters based on configuration
    const fluujoConfig = this.configManager.getFluujoConfig();
    if (fluujoConfig) {
      this.fluujo = new AgslagFluujoAdapter({
        baseUrl: fluujoConfig.baseUrl
      });
    }
    
    const openManusConfig = this.configManager.getOpenManusConfig();
    if (openManusConfig) {
      this.openManus = new AgslagOpenManusAdapter({
        baseUrl: openManusConfig.baseUrl
      });
    }
    
    const manusOpenConfig = this.configManager.getManusOpenConfig();
    if (manusOpenConfig) {
      this.manusOpen = new AgslagManusOpenAdapter({
        baseUrl: manusOpenConfig.baseUrl
      });
    }
  }
  
  // Existing methods...
}
```

### 3. Create Configuration CLI

```typescript
// cli/config_cli.ts
import { ConfigManager } from '../config/config_manager';
import { Command } from 'commander';
import * as inquirer from 'inquirer';

const program = new Command();
const configManager = ConfigManager.getInstance();

program
  .name('agslag-config')
  .description('AGSLAG MCP Integration Configuration CLI')
  .version('1.0.0');

program
  .command('show')
  .description('Show current configuration')
  .action(() => {
    const config = configManager.getConfig();
    console.log(JSON.stringify(config, null, 2));
  });

program
  .command('set-fluujo')
  .description('Configure Fluujo integration')
  .action(async () => {
    const answers = await inquirer.prompt([
      {
        type: 'input',
        name: 'baseUrl',
        message: 'Fluujo base URL:',
        default: configManager.getFluujoConfig().baseUrl
      },
      {
        type: 'input',
        name: 'serverDir',
        message: 'MCP server directory:',
        default: configManager.getFluujoConfig().serverDir
      },
      {
        type: 'list',
        name: 'transport',
        message: 'Transport type:',
        choices: ['stdio', 'websocket'],
        default: configManager.getFluujoConfig().transport
      }
    ]);
    
    if (answers.transport === 'websocket') {
      const websocketAnswer = await inquirer.prompt([
        {
          type: 'input',
          name: 'websocketUrl',
          message: 'WebSocket URL:',
          default: configManager.getFluujoConfig().websocketUrl || 'ws://localhost:3001'
        }
      ]);
      answers.websocketUrl = websocketAnswer.websocketUrl;
    }
    
    configManager.updateConfig({
      fluujo: {
        ...configManager.getFluujoConfig(),
        ...answers
      }
    });
    
    console.log('Fluujo configuration updated successfully');
  });

program
  .command('set-openmanus')
  .description('Configure OpenManus integration')
  .action(async () => {
    const answers = await inquirer.prompt([
      {
        type: 'input',
        name: 'baseUrl',
        message: 'OpenManus base URL:',
        default: configManager.getOpenManusConfig().baseUrl
      },
      {
        type: 'input',
        name: 'reasoningModel',
        message: 'Reasoning model:',
        default: configManager.getOpenManusConfig().reasoningModel
      },
      {
        type: 'input',
        name: 'basicModel',
        message: 'Basic model:',
        default: configManager.getOpenManusConfig().basicModel
      },
      {
        type: 'input',
        name: 'visionModel',
        message: 'Vision model:',
        default: configManager.getOpenManusConfig().visionModel
      },
      {
        type: 'confirm',
        name: 'chromeHeadless',
        message: 'Run Chrome in headless mode:',
        default: configManager.getOpenManusConfig().chromeHeadless
      }
    ]);
    
    configManager.updateConfig({
      openManus: {
        ...configManager.getOpenManusConfig(),
        ...answers
      }
    });
    
    console.log('OpenManus configuration updated successfully');
  });

program
  .command('set-manusopen')
  .description('Configure manus-open integration')
  .action(async () => {
    const answers = await inquirer.prompt([
      {
        type: 'input',
        name: 'baseUrl',
        message: 'manus-open base URL:',
        default: configManager.getManusOpenConfig().baseUrl
      },
      {
        type: 'input',
        name: 'runtimeApiHost',
        message: 'Runtime API host:',
        default: configManager.getManusOpenConfig().runtimeApiHost
      },
      {
        type: 'input',
        name: 'chromeInstancePath',
        message: 'Chrome instance path:',
        default: configManager.getManusOpenConfig().chromeInstancePath
      }
    ]);
    
    configManager.updateConfig({
      manusOpen: {
        ...configManager.getManusOpenConfig(),
        ...answers
      }
    });
    
    console.log('manus-open configuration updated successfully');
  });

program.parse(process.argv);
```

## Environment Variables

In addition to the configuration file, we'll support environment variables for sensitive information and deployment-specific settings:

```typescript
// config/env_config.ts
export function getEnvConfig() {
  return {
    agslag: {
      apiKey: process.env.AGSLAG_API_KEY,
      baseUrl: process.env.AGSLAG_BASE_URL,
      timeout: process.env.AGSLAG_TIMEOUT ? parseInt(process.env.AGSLAG_TIMEOUT) : undefined,
      debug: process.env.AGSLAG_DEBUG === 'true'
    },
    fluujo: {
      baseUrl: process.env.FLUUJO_BASE_URL,
      serverDir: process.env.FLUUJO_SERVER_DIR,
      transport: process.env.FLUUJO_TRANSPORT as 'stdio' | 'websocket',
      websocketUrl: process.env.FLUUJO_WEBSOCKET_URL
    },
    openManus: {
      baseUrl: process.env.OPENMANUS_BASE_URL,
      reasoningModel: process.env.OPENMANUS_REASONING_MODEL,
      basicModel: process.env.OPENMANUS_BASIC_MODEL,
      visionModel: process.env.OPENMANUS_VISION_MODEL,
      chromeHeadless: process.env.OPENMANUS_CHROME_HEADLESS === 'true'
    },
    manusOpen: {
      baseUrl: process.env.MANUSOPEN_BASE_URL,
      runtimeApiHost: process.env.MANUSOPEN_RUNTIME_API_HOST,
      chromeInstancePath: process.env.MANUSOPEN_CHROME_INSTANCE_PATH
    }
  };
}
```

Update the ConfigManager to merge environment variables with the configuration file:

```typescript
// config/config_manager.ts
import { getEnvConfig } from './env_config';

// Inside the loadConfig method
private loadConfig(): void {
  try {
    if (fs.existsSync(this.configPath)) {
      const configData = fs.readFileSync(this.configPath, 'utf8');
      const fileConfig = JSON.parse(configData);
      const envConfig = getEnvConfig();
      
      // Merge file config with environment variables
      this.config = this.mergeConfigs(fileConfig, envConfig);
    } else {
      this.config = this.getDefaultConfig();
      this.saveConfig();
    }
  } catch (error) {
    console.error('Error loading configuration:', error);
    this.config = this.getDefaultConfig();
  }
}

private mergeConfigs(fileConfig: any, envConfig: any): UnifiedConfig {
  const result = { ...fileConfig };
  
  // Merge each section
  for (const section of ['agslag', 'fluujo', 'openManus', 'manusOpen']) {
    if (envConfig[section]) {
      result[section] = {
        ...result[section],
        ...Object.fromEntries(
          Object.entries(envConfig[section]).filter(([_, value]) => value !== undefined)
        )
      };
    }
  }
  
  return result;
}
```

## Docker Compose Configuration

Create a Docker Compose configuration that sets up all the integrated components:

```yaml
// docker-compose.yml
version: '3.8'

services:
  # AGSLAG MCP Integration
  agslag:
    build:
      context: .
      dockerfile: Dockerfile
    ports:
      - "3002:3002"
    environment:
      - AGSLAG_API_KEY=${AGSLAG_API_KEY}
      - AGSLAG_BASE_URL=${AGSLAG_BASE_URL}
      - AGSLAG_TIMEOUT=${AGSLAG_TIMEOUT}
      - AGSLAG_DEBUG=${AGSLAG_DEBUG}
      - FLUUJO_BASE_URL=http://fluujo:3000
      - FLUUJO_SERVER_DIR=${FLUUJO_SERVER_DIR}
      - FLUUJO_TRANSPORT=${FLUUJO_TRANSPORT}
      - FLUUJO_WEBSOCKET_URL=ws://fluujo:3001
      - OPENMANUS_BASE_URL=http://openmanus:8000
      - OPENMANUS_REASONING_MODEL=${OPENMANUS_REASONING_MODEL}
      - OPENMANUS_BASIC_MODEL=${OPENMANUS_BASIC_MODEL}
      - OPENMANUS_VISION_MODEL=${OPENMANUS_VISION_MODEL}
      - OPENMANUS_CHROME_HEADLESS=${OPENMANUS_CHROME_HEADLESS}
      - MANUSOPEN_BASE_URL=http://manusopen:8330
      - MANUSOPEN_RUNTIME_API_HOST=http://manusopen
      - MANUSOPEN_CHROME_INSTANCE_PATH=/usr/bin/chromium
    volumes:
      - ./config:/app/config
    depends_on:
      - fluujo
      - openmanus
      - manusopen

  # Fluujo
  fluujo:
    build:
      context: ./FLUJO
      dockerfile: Dockerfile
    ports:
      - "3000:3000"
      - "3001:3001"
    environment:
      - NODE_ENV=production
    volumes:
      - ./FLUJO:/app
      - fluujo_data:/app/data

  # OpenManus
  openmanus:
    build:
      context: ./OpenManus
      dockerfile: docker/unified/Dockerfile
    ports:
      - "8000:8000"
    environment:
      - PYTHONPATH=/app
      - FLASK_APP=src/server.py
      - FLASK_ENV=production
      - REASONING_MODEL=${OPENMANUS_REASONING_MODEL}
      - BASIC_MODEL=${OPENMANUS_BASIC_MODEL}
      - VL_MODEL=${OPENMANUS_VISION_MODEL}
      - CHROME_HEADLESS=${OPENMANUS_CHROME_HEADLESS}
    volumes:
      - ./OpenManus:/app
      - openmanus_data:/app/data

  # manus-open
  manusopen:
    build:
      context: ./manus-open
      dockerfile: Dockerfile
    ports:
      - "8330:8330"
    environment:
      - RUNTIME_API_HOST=http://localhost
      - CHROME_INSTANCE_PATH=/usr/bin/chromium
    volumes:
      - ./manus-open:/app
      - manusopen_data:/app/data

volumes:
  fluujo_data:
  openmanus_data:
  manusopen_data:
```

## Environment File

Create a `.env` file template for configuring the Docker Compose environment:

```
# AGSLAG Configuration
AGSLAG_API_KEY=your_api_key_here
AGSLAG_BASE_URL=http://localhost:3002
AGSLAG_TIMEOUT=30000
AGSLAG_DEBUG=false

# Fluujo Configuration
FLUUJO_SERVER_DIR=mcp-servers
FLUUJO_TRANSPORT=stdio

# OpenManus Configuration
OPENMANUS_REASONING_MODEL=gpt-4
OPENMANUS_BASIC_MODEL=gpt-3.5-turbo
OPENMANUS_VISION_MODEL=gpt-4-vision-preview
OPENMANUS_CHROME_HEADLESS=true
```

## Configuration Usage

Here's how to use the configuration in your code:

```typescript
// Example usage
import { ConfigManager } from './config/config_manager';

// Get configuration manager instance
const configManager = ConfigManager.getInstance();

// Get specific configuration
const fluujoConfig = configManager.getFluujoConfig();
console.log('Fluujo base URL:', fluujoConfig.baseUrl);

// Update configuration
configManager.updateConfig({
  fluujo: {
    ...fluujoConfig,
    baseUrl: 'http://new-fluujo-url:3000'
  }
});

// Get full configuration
const fullConfig = configManager.getConfig();
console.log('Full configuration:', fullConfig);
```

## Configuration Security

For sensitive configuration values:

1. **Environment Variables**: Use environment variables for API keys and other sensitive information
2. **Encryption**: Encrypt sensitive values in the configuration file
3. **Access Control**: Restrict access to configuration files and environment variables

## Next Steps

After setting up the configuration management system:

1. Create a configuration UI for easier management
2. Implement configuration validation to ensure all required values are provided
3. Add support for multiple configuration profiles (development, testing, production)
4. Implement automatic configuration backup and versioning
