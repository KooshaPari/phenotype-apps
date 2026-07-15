#!/usr/bin/env python3
"""
KVirtualStage MCP Protocol Handler

Implements the full MCP (Model Context Protocol) server specification
for KVirtualStage desktop automation, enabling seamless integration
with Claude Code and other AI agents.

This handler provides:
- Full MCP 2024-11-05 protocol compliance
- JSON-RPC 2.0 message handling
- Tool and resource management
- Session state persistence
- Real-time communication with AI agents
- Error handling and recovery
"""

import asyncio
import json
import logging
import sys
import time
from typing import Dict, List, Optional, Any, Union
from dataclasses import dataclass, asdict
import traceback

from kvirtualstage_mcp_server import KVirtualStageMCPServer
from mcp_tools_claude_integration import ClaudeCodeMCPInterface

logger = logging.getLogger(__name__)

@dataclass
class MCPMessage:
    """MCP protocol message structure"""
    jsonrpc: str = "2.0"
    id: Optional[Union[str, int]] = None
    method: Optional[str] = None
    params: Optional[Dict[str, Any]] = None
    result: Optional[Any] = None
    error: Optional[Dict[str, Any]] = None

@dataclass
class MCPError:
    """MCP error structure"""
    code: int
    message: str
    data: Optional[Any] = None

class MCPProtocolHandler:
    """
    Handles MCP protocol communication for KVirtualStage
    
    Provides full protocol compliance for AI agent integration
    with sophisticated desktop automation capabilities.
    """
    
    # MCP Error Codes
    PARSE_ERROR = -32700
    INVALID_REQUEST = -32600
    METHOD_NOT_FOUND = -32601
    INVALID_PARAMS = -32602
    INTERNAL_ERROR = -32603
    SERVER_ERROR_BASE = -32000
    
    def __init__(self):
        self.server = KVirtualStageMCPServer()
        self.claude_interface = ClaudeCodeMCPInterface()
        
        # Protocol state
        self.initialized = False
        self.client_info = {}
        self.server_capabilities = {
            "tools": True,
            "resources": True,
            "prompts": False,  # Not implemented yet
            "logging": True
        }
        
        # Session management
        self.active_connections = {}
        
        logger.info("MCP Protocol Handler initialized")
    
    async def handle_message(self, message_data: str, connection_id: str = "default") -> str:
        """
        Handle incoming MCP message and return response
        
        Args:
            message_data: JSON string containing MCP message
            connection_id: Unique identifier for the connection
            
        Returns:
            JSON string containing MCP response
        """
        try:
            # Parse message
            message_dict = json.loads(message_data)
            message = MCPMessage(**message_dict)
            
            # Validate message structure
            if not self._validate_message(message):
                return self._create_error_response(
                    message.id, 
                    self.INVALID_REQUEST, 
                    "Invalid message structure"
                )
            
            # Route message to appropriate handler
            if message.method:
                # Request message
                response = await self._handle_request(message, connection_id)
            else:
                # Response or notification (not typical for server)
                response = self._create_error_response(
                    message.id,
                    self.METHOD_NOT_FOUND,
                    "Server does not handle response messages"
                )
            
            return json.dumps(asdict(response), separators=(',', ':'))
            
        except json.JSONDecodeError as e:
            logger.error(f"JSON parse error: {e}")
            return self._create_error_response(
                None,
                self.PARSE_ERROR,
                f"Parse error: {str(e)}"
            )
        except Exception as e:
            logger.error(f"Message handling error: {e}")
            logger.error(traceback.format_exc())
            return self._create_error_response(
                None,
                self.INTERNAL_ERROR,
                f"Internal error: {str(e)}"
            )
    
    def _validate_message(self, message: MCPMessage) -> bool:
        """Validate MCP message structure"""
        if message.jsonrpc != "2.0":
            return False
        
        # Must have either method (request) or result/error (response)
        if message.method is None and message.result is None and message.error is None:
            return False
            
        return True
    
    async def _handle_request(self, message: MCPMessage, connection_id: str) -> MCPMessage:
        """Handle MCP request message"""
        method = message.method
        params = message.params or {}
        
        try:
            # Route to appropriate handler based on method
            if method == "initialize":
                result = await self._handle_initialize(params, connection_id)
            elif method == "tools/list":
                result = await self._handle_tools_list(params)
            elif method == "tools/call":
                result = await self._handle_tools_call(params)
            elif method == "resources/list":
                result = await self._handle_resources_list(params)
            elif method == "resources/read":
                result = await self._handle_resources_read(params)
            elif method == "logging/setLevel":
                result = await self._handle_logging_set_level(params)
            elif method == "notifications/initialized":
                result = await self._handle_notifications_initialized(params)
            else:
                return MCPMessage(
                    id=message.id,
                    error={
                        "code": self.METHOD_NOT_FOUND,
                        "message": f"Method not found: {method}"
                    }
                )
            
            return MCPMessage(id=message.id, result=result)
            
        except Exception as e:
            logger.error(f"Request handling error for {method}: {e}")
            logger.error(traceback.format_exc())
            
            return MCPMessage(
                id=message.id,
                error={
                    "code": self.INTERNAL_ERROR,
                    "message": f"Internal error: {str(e)}",
                    "data": {"method": method}
                }
            )
    
    async def _handle_initialize(self, params: Dict[str, Any], connection_id: str) -> Dict[str, Any]:
        """Handle initialize request"""
        client_info = params.get("clientInfo", {})
        protocol_version = params.get("protocolVersion", "unknown")
        
        # Store client information
        self.active_connections[connection_id] = {
            "client_info": client_info,
            "protocol_version": protocol_version,
            "initialized_at": time.time()
        }
        
        self.client_info = client_info
        self.initialized = True
        
        logger.info(f"Initialized connection {connection_id} with client: {client_info.get('name', 'unknown')}")
        
        return {
            "protocolVersion": "2024-11-05",
            "capabilities": self.server_capabilities,
            "serverInfo": {
                "name": "KVirtualStage MCP Server",
                "version": "1.0.0",
                "description": "Comprehensive desktop automation server for AI agents"
            },
            "instructions": "KVirtualStage provides sophisticated desktop automation capabilities. Use tools to interact with applications, capture screenshots, manage windows, and execute complex workflows."
        }
    
    async def _handle_tools_list(self, params: Dict[str, Any]) -> Dict[str, Any]:
        """Handle tools/list request"""
        # Get tools from both base server and Claude interface
        base_tools = [
            {
                "name": tool.name,
                "description": tool.description,
                "inputSchema": tool.input_schema
            }
            for tool in self.server.tools
        ]
        
        claude_tools = [
            {
                "name": tool.name,
                "description": tool.description,
                "inputSchema": tool.input_schema
            }
            for tool in self.claude_interface.claude_tools
        ]
        
        all_tools = base_tools + claude_tools
        
        logger.info(f"Listing {len(all_tools)} available tools")
        
        return {"tools": all_tools}
    
    async def _handle_tools_call(self, params: Dict[str, Any]) -> Dict[str, Any]:
        """Handle tools/call request"""
        tool_name = params.get("name")
        arguments = params.get("arguments", {})
        
        if not tool_name:
            raise ValueError("Tool name is required")
        
        logger.info(f"Calling tool: {tool_name}")
        
        # Route to appropriate handler
        try:
            # Check if it's a Claude-specific tool
            claude_tool_names = [tool.name for tool in self.claude_interface.claude_tools]
            
            if tool_name in claude_tool_names:
                result = await self.claude_interface.handle_claude_tool_call(tool_name, arguments)
            else:
                result = await self.server.handle_tool_call(tool_name, arguments)
            
            # Format result for MCP response
            return {
                "content": [
                    {
                        "type": "text",
                        "text": json.dumps(result, indent=2)
                    }
                ],
                "isError": not result.get("success", False)
            }
            
        except Exception as e:
            logger.error(f"Tool call error for {tool_name}: {e}")
            return {
                "content": [
                    {
                        "type": "text", 
                        "text": f"Tool execution failed: {str(e)}"
                    }
                ],
                "isError": True
            }
    
    async def _handle_resources_list(self, params: Dict[str, Any]) -> Dict[str, Any]:
        """Handle resources/list request"""
        resources = [
            {
                "uri": resource.uri,
                "name": resource.name,
                "description": resource.description,
                "mimeType": resource.mime_type
            }
            for resource in self.server.resources
        ]
        
        logger.info(f"Listing {len(resources)} available resources")
        
        return {"resources": resources}
    
    async def _handle_resources_read(self, params: Dict[str, Any]) -> Dict[str, Any]:
        """Handle resources/read request"""
        uri = params.get("uri")
        
        if not uri:
            raise ValueError("Resource URI is required")
        
        logger.info(f"Reading resource: {uri}")
        
        # Route to appropriate resource handler
        try:
            if uri == "kvs://sessions":
                content = await self._get_sessions_resource()
            elif uri == "kvs://capabilities":
                content = await self._get_capabilities_resource()
            elif uri == "kvs://applications":
                content = await self._get_applications_resource()
            elif uri == "kvs://recordings":
                content = await self._get_recordings_resource()
            else:
                raise ValueError(f"Unknown resource URI: {uri}")
            
            return {
                "contents": [
                    {
                        "uri": uri,
                        "mimeType": "application/json",
                        "text": json.dumps(content, indent=2)
                    }
                ]
            }
            
        except Exception as e:
            logger.error(f"Resource read error for {uri}: {e}")
            raise ValueError(f"Failed to read resource {uri}: {str(e)}")
    
    async def _handle_logging_set_level(self, params: Dict[str, Any]) -> Dict[str, Any]:
        """Handle logging/setLevel request"""
        level = params.get("level", "info").upper()
        
        # Set logging level
        numeric_level = getattr(logging, level, logging.INFO)
        logging.getLogger().setLevel(numeric_level)
        
        logger.info(f"Logging level set to: {level}")
        
        return {"success": True, "level": level}
    
    async def _handle_notifications_initialized(self, params: Dict[str, Any]) -> Dict[str, Any]:
        """Handle notifications/initialized notification"""
        logger.info("Client initialization completed")
        return {"acknowledged": True}
    
    # Resource handlers
    
    async def _get_sessions_resource(self) -> Dict[str, Any]:
        """Get active sessions resource"""
        sessions_result = await self.server.handle_tool_call("kvs_session_list", {
            "include_details": True
        })
        
        return {
            "sessions": sessions_result.get("sessions", []),
            "total_sessions": sessions_result.get("session_count", 0),
            "timestamp": time.time()
        }
    
    async def _get_capabilities_resource(self) -> Dict[str, Any]:
        """Get capabilities resource"""
        return {
            "server_info": {
                "name": "KVirtualStage MCP Server",
                "version": "1.0.0",
                "protocol_version": "2024-11-05"
            },
            "automation_capabilities": [
                "desktop_interaction",
                "application_launching", 
                "window_management",
                "form_filling",
                "menu_navigation",
                "text_input",
                "visual_feedback",
                "session_recording",
                "element_detection",
                "workflow_automation"
            ],
            "detection_methods": [
                "accessibility_api",
                "optical_character_recognition", 
                "template_matching",
                "coordinate_based",
                "computer_vision"
            ],
            "supported_applications": [
                "calculator",
                "text_editor",
                "web_browser",
                "file_manager",
                "office_applications"
            ],
            "visual_features": [
                "cursor_path_indication",
                "click_animation",
                "typing_visualization",
                "element_highlighting",
                "real_time_feedback"
            ],
            "claude_integration": {
                "natural_language_processing": True,
                "intent_capture": True,
                "context_awareness": True,
                "adaptive_execution": True,
                "test_generation": True
            }
        }
    
    async def _get_applications_resource(self) -> Dict[str, Any]:
        """Get available applications resource"""
        # Get window list to see available applications
        window_result = await self.server.handle_tool_call("kvs_window_manage", {
            "action": "list"
        })
        
        return {
            "available_applications": [
                {
                    "name": "Calculator",
                    "command": "galculator",
                    "category": "utility",
                    "automation_support": "full"
                },
                {
                    "name": "Text Editor",
                    "command": "mousepad",
                    "category": "productivity",
                    "automation_support": "full"
                },
                {
                    "name": "File Manager",
                    "command": "thunar",
                    "category": "system",
                    "automation_support": "partial"
                },
                {
                    "name": "Web Browser",
                    "command": "firefox",
                    "category": "internet",
                    "automation_support": "partial"
                }
            ],
            "running_applications": window_result.get("windows", []),
            "timestamp": time.time()
        }
    
    async def _get_recordings_resource(self) -> Dict[str, Any]:
        """Get available recordings resource"""
        # This would list available session recordings
        return {
            "recordings": [
                {
                    "id": "sample_recording",
                    "filename": "sample_automation.mp4",
                    "duration": 120,
                    "created_at": time.time() - 3600,
                    "session_id": "sample_session"
                }
            ],
            "recording_formats": ["mp4", "webm", "gif"],
            "timestamp": time.time()
        }
    
    def _create_error_response(self, message_id: Optional[Union[str, int]], 
                             code: int, message: str, data: Any = None) -> str:
        """Create error response JSON string"""
        error_response = MCPMessage(
            id=message_id,
            error={
                "code": code,
                "message": message,
                "data": data
            }
        )
        return json.dumps(asdict(error_response), separators=(',', ':'))

class MCPServerRunner:
    """
    Runs the MCP server with various transport options
    """
    
    def __init__(self):
        self.handler = MCPProtocolHandler()
        self.running = False
    
    async def run_stdio(self):
        """Run MCP server over stdio"""
        logger.info("Starting KVirtualStage MCP Server on stdio")
        
        self.running = True
        connection_id = "stdio"
        
        try:
            while self.running:
                # Read line from stdin
                line = await asyncio.get_event_loop().run_in_executor(
                    None, sys.stdin.readline
                )
                
                if not line:
                    break
                
                line = line.strip()
                if not line:
                    continue
                
                # Process message
                response = await self.handler.handle_message(line, connection_id)
                
                # Write response to stdout
                print(response, flush=True)
                
        except KeyboardInterrupt:
            logger.info("Shutting down MCP server")
        except Exception as e:
            logger.error(f"MCP server error: {e}")
            logger.error(traceback.format_exc())
        finally:
            self.running = False
    
    async def run_tcp(self, host: str = "localhost", port: int = 8000):
        """Run MCP server over TCP"""
        logger.info(f"Starting KVirtualStage MCP Server on {host}:{port}")
        
        async def handle_client(reader, writer):
            connection_id = f"tcp_{id(writer)}"
            addr = writer.get_extra_info('peername')
            logger.info(f"Client connected from {addr} as {connection_id}")
            
            try:
                while True:
                    # Read message
                    data = await reader.readline()
                    if not data:
                        break
                    
                    message = data.decode().strip()
                    if not message:
                        continue
                    
                    # Process message
                    response = await self.handler.handle_message(message, connection_id)
                    
                    # Send response
                    writer.write(f"{response}\n".encode())
                    await writer.drain()
                    
            except Exception as e:
                logger.error(f"Client error for {connection_id}: {e}")
            finally:
                writer.close()
                await writer.wait_closed()
                logger.info(f"Client {connection_id} disconnected")
        
        server = await asyncio.start_server(handle_client, host, port)
        
        self.running = True
        
        async with server:
            try:
                await server.serve_forever()
            except KeyboardInterrupt:
                logger.info("Shutting down MCP server")
            finally:
                self.running = False
    
    def stop(self):
        """Stop the MCP server"""
        self.running = False

# CLI Interface

async def main():
    """Main entry point for MCP server"""
    import argparse
    
    parser = argparse.ArgumentParser(description="KVirtualStage MCP Server")
    parser.add_argument("--transport", choices=["stdio", "tcp"], default="stdio",
                       help="Transport method (default: stdio)")
    parser.add_argument("--host", default="localhost",
                       help="Host for TCP transport (default: localhost)")
    parser.add_argument("--port", type=int, default=8000,
                       help="Port for TCP transport (default: 8000)")
    parser.add_argument("--log-level", choices=["DEBUG", "INFO", "WARNING", "ERROR"],
                       default="INFO", help="Logging level (default: INFO)")
    
    args = parser.parse_args()
    
    # Configure logging
    logging.basicConfig(
        level=getattr(logging, args.log_level),
        format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
        handlers=[
            logging.StreamHandler(sys.stderr)  # Log to stderr to avoid interfering with stdio
        ]
    )
    
    # Create and run server
    server = MCPServerRunner()
    
    try:
        if args.transport == "stdio":
            await server.run_stdio()
        elif args.transport == "tcp":
            await server.run_tcp(args.host, args.port)
    except Exception as e:
        logger.error(f"Server startup failed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    asyncio.run(main())