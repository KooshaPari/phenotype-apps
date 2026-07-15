# MCP Platform Troubleshooting Guide

This guide provides solutions for common issues encountered when working with the MCP Platform.

## WebSocket Connection Issues

### Symptoms

- "Failed to connect to WebSocket" errors
- WebSocket connection status shows as disconnected
- Real-time updates not working

### Solutions

1. **Check if the MCP Server is running**
   ```bash
   # Check if the process is running
   ps aux | grep mcp-server
   
   # Restart the server if needed
   cd agslag
   npm run start:server
   ```

2. **Verify WebSocket port is not in use**
   ```bash
   # Check if port 8081 is in use
   netstat -ano | findstr :8081
   
   # If it's in use, you can kill the process
   taskkill /PID <PID> /F
   ```

3. **Check WebSocket URL configuration**
   - Ensure the WebSocket URL in `config.json` is correct
   - For local development, it should be `ws://localhost:8081`

4. **Check for network issues**
   - Ensure there are no firewall rules blocking WebSocket connections
   - Try disabling any VPN or proxy services temporarily

## Missing Avatar Images

### Symptoms

- 404 errors for avatar images in the browser console
- Missing profile pictures in the UI

### Solutions

1. **Create the avatars directory**
   ```bash
   mkdir -p old-frontend/public/avatars
   ```

2. **Add placeholder avatar images**
   - Create or copy images for:
     - `agent-1.png`
     - `agent-2.png`
     - `research-agent.png`
     - `writing-agent.png`
     - `qa-agent.png`
     - `current-user.png`

3. **Check image paths in the code**
   - Ensure the image paths in the code match the actual file locations
   - Image paths should be relative to the `public` directory

## API Connection Errors

### Symptoms

- "Failed to fetch" errors in the browser console
- Empty data in the UI
- Actions like creating agents or sending messages fail

### Solutions

1. **Check if backend services are running**
   ```bash
   # Check MCP Server
   ps aux | grep mcp-server
   
   # Check Jarvis SWE Agent
   ps aux | grep jarvis-swe-agent
   ```

2. **Verify API URLs in configuration**
   - Check `config.json` for correct API URLs
   - For local development:
     - MCP API: `http://localhost:3002`
     - Jarvis API: `http://localhost:3100`

3. **Check for CORS issues**
   - Open browser DevTools and look for CORS errors
   - Ensure the backend services have CORS enabled for your frontend URL

4. **Test API endpoints directly**
   ```bash
   # Test MCP Server health endpoint
   curl http://localhost:8081/health
   
   # Test Jarvis SWE Agent endpoint
   curl http://localhost:3100/agents
   ```

## Build Errors

### Symptoms

- Build process fails with errors
- TypeScript compilation errors
- Missing dependencies errors

### Solutions

1. **Check for TypeScript errors**
   ```bash
   # In the project directory
   npx tsc --noEmit
   ```

2. **Ensure all dependencies are installed**
   ```bash
   # In the project directory
   npm install
   ```

3. **Clear node_modules and reinstall**
   ```bash
   # In the project directory
   rm -rf node_modules
   npm install
   ```

4. **Check Node.js version**
   ```bash
   node -v
   ```
   - Ensure you're using Node.js 18 or later

## React Errors

### Symptoms

- React error messages in the browser console
- UI components not rendering correctly
- Unexpected behavior in the UI

### Solutions

1. **Check for hook rule violations**
   - Ensure hooks are only called at the top level of components
   - Ensure hooks are not called conditionally

2. **Check for missing dependencies in useEffect**
   - Ensure all variables used in useEffect are included in the dependency array

3. **Check for state updates on unmounted components**
   - Use cleanup functions in useEffect to prevent updates after unmounting

4. **Use React DevTools for debugging**
   - Install the React DevTools browser extension
   - Inspect component props and state

## OpenAI API Errors

### Symptoms

- "Failed to generate OpenAI response" errors
- Tool calls not working correctly
- Messages not being processed

### Solutions

1. **Check OpenAI API key**
   - Ensure the API key is set in the environment variables
   - Verify the API key is valid and has sufficient credits

2. **Check for rate limiting**
   - OpenAI has rate limits that may be exceeded
   - Implement exponential backoff for retries

3. **Check message format**
   - Ensure messages follow the correct format for the OpenAI API
   - For tool responses, ensure `tool_call_id` is set correctly

4. **Check for token limit issues**
   - Long conversations may exceed token limits
   - Implement conversation summarization or truncation

## Performance Issues

### Symptoms

- Slow response times
- High CPU or memory usage
- UI lag or freezing

### Solutions

1. **Check server resource usage**
   ```bash
   # Check CPU and memory usage
   top
   ```

2. **Optimize database queries**
   - Add indexes for frequently queried fields
   - Limit the amount of data returned in queries

3. **Implement caching**
   - Use Redis for caching frequently accessed data
   - Implement browser caching for static assets

4. **Optimize frontend rendering**
   - Use React.memo for expensive components
   - Implement virtualization for long lists

## Deployment Issues

### Symptoms

- Application doesn't start after deployment
- Environment-specific errors
- Configuration issues

### Solutions

1. **Check environment variables**
   - Ensure all required environment variables are set
   - Check for environment-specific configuration

2. **Check file permissions**
   - Ensure the application has permission to read/write necessary files
   - Check log files for permission errors

3. **Check network configuration**
   - Ensure ports are open and accessible
   - Check for firewall or proxy issues

4. **Check for platform-specific issues**
   - Different operating systems may have different requirements
   - Check for dependencies that need to be installed system-wide
