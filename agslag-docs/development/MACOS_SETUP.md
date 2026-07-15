# macOS Homebrew TypeScript Setup Guide

This guide provides specific instructions for setting up and building the MCP Cross-Agent server on macOS with TypeScript installed via Homebrew.

## Prerequisites

- macOS with Homebrew installed
- Node.js and npm
- TypeScript installed via Homebrew (`brew install typescript`)

## Installation

1. Extract the zip file to a directory of your choice:
   ```bash
   unzip mcp_official_macos.zip -d /path/to/destination
   ```

2. Navigate to the extracted directory:
   ```bash
   cd /path/to/destination/mcp_official
   ```

3. Install dependencies:
   ```bash
   npm install
   ```

## Building the Project

The package.json includes several build scripts specifically for macOS with Homebrew:

### Option 1: Use the macOS-specific build script

```bash
npm run build:macos
```

This script directly uses the Homebrew TypeScript installation at `/opt/homebrew/bin/tsc`.

### Option 2: Use the default build script with PATH modification

```bash
npm run build
```

This script adds the Homebrew bin directory to the PATH before running TypeScript.

### Option 3: Use the global TypeScript installation directly

```bash
/opt/homebrew/bin/tsc
chmod +x build/index.js
```

## Running the Server

After building the project, you can run the server:

```bash
npm start
```

## Troubleshooting

If you encounter TypeScript-related issues:

1. **Verify your TypeScript installation**:
   ```bash
   which tsc
   ```
   This should show `/opt/homebrew/bin/tsc` if TypeScript is installed via Homebrew.

2. **Check TypeScript version**:
   ```bash
   /opt/homebrew/bin/tsc --version
   ```
   Make sure it's version 5.0.0 or higher.

3. **Try reinstalling TypeScript**:
   ```bash
   brew reinstall typescript
   ```

4. **Install TypeScript locally in the project**:
   ```bash
   npm install typescript@latest --save-dev
   ```

## Claude Desktop Integration

For Claude Desktop integration on macOS, locate your config file at:
`~/Library/Application Support/Claude/claude_desktop_config.json`

Add the Cross-Agent MCP configuration:

```json
{
  "mcpServers": {
    "cross-agent": {
      "command": "npm",
      "args": ["run", "start", "--prefix", "/path/to/mcp_official"],
      "env": {
        "AGENT_ID": "claude-desktop-1",
        "AGENT_TYPE": "claude"
      }
    }
  }
}
```

Replace `/path/to/mcp_official` with the actual path to your installation.

## Additional Resources

For more information, refer to:
- TROUBLESHOOTING.md for general troubleshooting
- README.md for general usage instructions
