# MCP Platform Development Guide

This guide provides information for developers working on the MCP Platform.

## Development Principles

1. **Modular Architecture**: Keep components loosely coupled
2. **Type Safety**: Use TypeScript for all code
3. **Testing**: Write tests for critical functionality
4. **Documentation**: Document code and APIs
5. **Error Handling**: Implement proper error handling and logging

## Project Structure

### MCP Server

```
agslag/
├── src/
│   ├── index.ts           # Entry point
│   ├── types.ts           # Type definitions
│   ├── services/          # Service implementations
│   ├── utils/             # Utility functions
│   └── routes/            # API routes
├── build/                 # Compiled output
├── tsconfig.json          # TypeScript configuration
└── package.json           # Dependencies and scripts
```

### Jarvis SWE Agent

```
jarvis-swe-agent/
├── src/
│   ├── index.ts           # Entry point
│   ├── types.ts           # Type definitions
│   ├── services/          # Service implementations
│   ├── tools/             # Tool implementations
│   └── utils/             # Utility functions
├── dist/                  # Compiled output
├── tsconfig.json          # TypeScript configuration
└── package.json           # Dependencies and scripts
```

### Frontend Dashboard

```
old-frontend/
├── src/
│   ├── app/               # Next.js app router
│   ├── components/        # React components
│   ├── contexts/          # React contexts
│   ├── lib/               # Utility functions
│   └── types/             # Type definitions
├── public/                # Static assets
├── next.config.js         # Next.js configuration
└── package.json           # Dependencies and scripts
```

## Adding New Features

### Adding a New Tool

1. Define the tool interface in `types.ts`
2. Implement the tool in the `tools` directory
3. Register the tool in the appropriate service
4. Update the frontend to support the new tool

### Adding a New Agent Type

1. Define the agent type in `types.ts`
2. Implement the agent in the `services` directory
3. Register the agent type in the agent manager
4. Update the frontend to support the new agent type

### Adding a New UI Component

1. Create the component in the `components` directory
2. Add any necessary styles
3. Add the component to the appropriate page
4. Test the component with different data

## Debugging

### Backend Debugging

1. Use the `debug` package for logging
2. Set the `DEBUG` environment variable to enable specific logs
3. Use the Node.js debugger for step-by-step debugging

### Frontend Debugging

1. Use the React Developer Tools browser extension
2. Use the Network tab in browser DevTools to inspect API calls
3. Use `console.log` statements for quick debugging

## Testing

### Backend Testing

1. Use Jest for unit and integration tests
2. Mock external dependencies
3. Test error handling and edge cases

### Frontend Testing

1. Use React Testing Library for component tests
2. Use Cypress for end-to-end tests
3. Test different screen sizes and browsers

## Deployment

### Local Deployment

1. Build all components
2. Start the MCP Server
3. Start the Jarvis SWE Agent
4. Start the Frontend Dashboard

### Production Deployment

1. Set up a production environment
2. Configure environment variables for production
3. Build all components
4. Deploy the components to the appropriate servers
5. Set up monitoring and logging

## Common Issues and Solutions

### WebSocket Connection Issues

- Ensure the WebSocket server is running
- Check for firewall or network issues
- Verify the WebSocket URL is correct

### API Connection Issues

- Check that the API server is running
- Verify the API URL is correct
- Check for CORS issues

### Build Errors

- Check for TypeScript errors
- Ensure all dependencies are installed
- Verify that the correct Node.js version is being used

### React Errors

- Check for hook rule violations
- Verify that components are properly mounted
- Check for state management issues
