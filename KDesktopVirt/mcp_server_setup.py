#!/usr/bin/env python3
"""
MCP Server Setup and Configuration for KVirtualStage

This script provides easy setup and configuration for the KVirtualStage MCP server,
enabling AI agents like Claude Code to seamlessly integrate with desktop automation.

Features:
- Automated server installation and configuration
- Claude Code integration setup
- Service management (start/stop/restart)
- Configuration validation
- Health monitoring
- Example workflows and demonstrations

Usage:
    python mcp_server_setup.py --install
    python mcp_server_setup.py --start
    python mcp_server_setup.py --demo
"""

import asyncio
import json
import logging
import os
import subprocess
import sys
import time
from pathlib import Path
from typing import Dict, List, Any, Optional
import argparse
import yaml

# Setup logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

class MCPServerSetup:
    """
    Handles setup and configuration of KVirtualStage MCP Server
    """
    
    def __init__(self, config_dir: str = None):
        self.config_dir = Path(config_dir or os.path.expanduser("~/.config/kvirtualstage"))
        self.config_file = self.config_dir / "mcp_config.yaml"
        self.log_dir = self.config_dir / "logs"
        
        # Default configuration
        self.default_config = {
            "server": {
                "name": "KVirtualStage MCP Server",
                "version": "1.0.0",
                "transport": "stdio",  # stdio or tcp
                "host": "localhost",
                "port": 8000,
                "log_level": "INFO"
            },
            "automation": {
                "visual_feedback": True,
                "cursor_path_indication": True,
                "recording_enabled": True,
                "intent_capture": True,
                "claude_integration": True
            },
            "features": {
                "element_detection_methods": ["accessibility", "ocr", "template", "coordinates"],
                "typing_simulation": "human_like",
                "cursor_movement": "natural",
                "error_recovery": "adaptive",
                "session_persistence": True
            },
            "claude_code": {
                "natural_language_parsing": True,
                "context_awareness": True,
                "workflow_generation": True,
                "test_automation": True,
                "intent_learning": True
            },
            "paths": {
                "screenshots_dir": "/tmp/kvs_screenshots",
                "recordings_dir": "/tmp/kvs_recordings", 
                "logs_dir": str(self.log_dir),
                "templates_dir": "/tmp/kvs_templates"
            }
        }
        
        # Ensure directories exist
        self.config_dir.mkdir(parents=True, exist_ok=True)
        self.log_dir.mkdir(parents=True, exist_ok=True)
    
    def install(self) -> bool:
        """Install and configure the MCP server"""
        try:
            logger.info("🚀 Installing KVirtualStage MCP Server...")
            
            # Create configuration
            if not self._create_config():
                return False
            
            # Create required directories
            if not self._create_directories():
                return False
            
            # Install dependencies
            if not self._install_dependencies():
                return False
            
            # Create startup scripts
            if not self._create_startup_scripts():
                return False
            
            # Validate installation
            if not self._validate_installation():
                return False
            
            logger.info("✅ KVirtualStage MCP Server installed successfully!")
            logger.info(f"Configuration saved to: {self.config_file}")
            logger.info("Use 'python mcp_server_setup.py --start' to launch the server")
            
            return True
            
        except Exception as e:
            logger.error(f"Installation failed: {e}")
            return False
    
    def _create_config(self) -> bool:
        """Create configuration file"""
        try:
            config = self.default_config.copy()
            
            # Update paths in config
            for path_key, path_value in config["paths"].items():
                if not os.path.isabs(path_value):
                    config["paths"][path_key] = str(self.config_dir / path_value)
                
                # Create directory
                Path(config["paths"][path_key]).mkdir(parents=True, exist_ok=True)
            
            # Save configuration
            with open(self.config_file, 'w') as f:
                yaml.dump(config, f, default_flow_style=False, indent=2)
            
            logger.info(f"Configuration created: {self.config_file}")
            return True
            
        except Exception as e:
            logger.error(f"Failed to create configuration: {e}")
            return False
    
    def _create_directories(self) -> bool:
        """Create required directories"""
        try:
            config = self.load_config()
            
            for dir_path in config["paths"].values():
                Path(dir_path).mkdir(parents=True, exist_ok=True)
                logger.debug(f"Created directory: {dir_path}")
            
            return True
            
        except Exception as e:
            logger.error(f"Failed to create directories: {e}")
            return False
    
    def _install_dependencies(self) -> bool:
        """Install Python dependencies"""
        try:
            requirements = [
                "opencv-python",
                "numpy",
                "pillow", 
                "pyautogui",
                "pyyaml",
                "asyncio",
                "dataclasses"
            ]
            
            # Try to import optional dependencies
            optional_requirements = [
                "easyocr",  # For OCR support
                "dogtail",  # For accessibility
                "Xlib"      # For X11 support
            ]
            
            logger.info("Installing core dependencies...")
            for req in requirements:
                try:
                    __import__(req.replace('-', '_'))
                    logger.debug(f"✓ {req} already installed")
                except ImportError:
                    logger.info(f"Installing {req}...")
                    subprocess.run([sys.executable, "-m", "pip", "install", req], 
                                 check=True, capture_output=True)
            
            logger.info("Installing optional dependencies...")
            for req in optional_requirements:
                try:
                    __import__(req.replace('-', '_'))
                    logger.debug(f"✓ {req} already installed")
                except ImportError:
                    try:
                        logger.info(f"Installing optional dependency {req}...")
                        subprocess.run([sys.executable, "-m", "pip", "install", req], 
                                     check=True, capture_output=True)
                    except subprocess.CalledProcessError:
                        logger.warning(f"Failed to install optional dependency {req}")
            
            return True
            
        except Exception as e:
            logger.error(f"Failed to install dependencies: {e}")
            return False
    
    def _create_startup_scripts(self) -> bool:
        """Create startup scripts for the MCP server"""
        try:
            # Create stdio launcher
            stdio_script = self.config_dir / "start_mcp_stdio.sh"
            with open(stdio_script, 'w') as f:
                f.write(f"""#!/bin/bash
# KVirtualStage MCP Server - stdio launcher
cd {Path(__file__).parent}
python mcp_protocol_handler.py --transport stdio --log-level INFO
""")
            stdio_script.chmod(0o755)
            
            # Create TCP launcher
            tcp_script = self.config_dir / "start_mcp_tcp.sh"
            with open(tcp_script, 'w') as f:
                f.write(f"""#!/bin/bash
# KVirtualStage MCP Server - TCP launcher
cd {Path(__file__).parent}
python mcp_protocol_handler.py --transport tcp --host localhost --port 8000 --log-level INFO
""")
            tcp_script.chmod(0o755)
            
            # Create systemd service file (optional)
            try:
                service_file = self.config_dir / "kvirtualstage-mcp.service"
                with open(service_file, 'w') as f:
                    f.write(f"""[Unit]
Description=KVirtualStage MCP Server
After=network.target

[Service]
Type=simple
User={os.getenv('USER')}
WorkingDirectory={Path(__file__).parent}
ExecStart=/usr/bin/python3 mcp_protocol_handler.py --transport tcp
Restart=always
RestartSec=5
Environment=DISPLAY=:1

[Install]
WantedBy=multi-user.target
""")
                logger.info(f"Systemd service file created: {service_file}")
                logger.info("To install as system service: sudo cp {} /etc/systemd/system/".format(service_file))
                
            except Exception as e:
                logger.warning(f"Could not create systemd service file: {e}")
            
            logger.info("Startup scripts created")
            return True
            
        except Exception as e:
            logger.error(f"Failed to create startup scripts: {e}")
            return False
    
    def _validate_installation(self) -> bool:
        """Validate the installation"""
        try:
            # Check configuration file
            if not self.config_file.exists():
                logger.error("Configuration file not found")
                return False
            
            # Try to load configuration
            config = self.load_config()
            if not config:
                logger.error("Invalid configuration")
                return False
            
            # Check required directories
            for dir_path in config["paths"].values():
                if not Path(dir_path).exists():
                    logger.error(f"Required directory missing: {dir_path}")
                    return False
            
            # Try to import core modules
            try:
                from kvirtualstage_mcp_server import KVirtualStageMCPServer
                from mcp_tools_claude_integration import ClaudeCodeMCPInterface
                from mcp_protocol_handler import MCPProtocolHandler
                logger.info("✓ Core modules importable")
            except ImportError as e:
                logger.error(f"Failed to import core modules: {e}")
                return False
            
            logger.info("✓ Installation validation passed")
            return True
            
        except Exception as e:
            logger.error(f"Validation failed: {e}")
            return False
    
    def load_config(self) -> Optional[Dict[str, Any]]:
        """Load configuration from file"""
        try:
            if not self.config_file.exists():
                return None
            
            with open(self.config_file, 'r') as f:
                return yaml.safe_load(f)
                
        except Exception as e:
            logger.error(f"Failed to load configuration: {e}")
            return None
    
    def start_server(self, transport: str = None) -> bool:
        """Start the MCP server"""
        try:
            config = self.load_config()
            if not config:
                logger.error("No configuration found. Run --install first.")
                return False
            
            transport = transport or config["server"]["transport"]
            
            logger.info(f"🚀 Starting KVirtualStage MCP Server ({transport})...")
            
            # Import and start server
            from mcp_protocol_handler import MCPServerRunner
            
            server = MCPServerRunner()
            
            if transport == "stdio":
                asyncio.run(server.run_stdio())
            elif transport == "tcp":
                host = config["server"]["host"]
                port = config["server"]["port"]
                asyncio.run(server.run_tcp(host, port))
            else:
                logger.error(f"Unknown transport: {transport}")
                return False
            
            return True
            
        except KeyboardInterrupt:
            logger.info("Server stopped by user")
            return True
        except Exception as e:
            logger.error(f"Failed to start server: {e}")
            return False
    
    def test_server(self) -> bool:
        """Test the MCP server functionality"""
        try:
            logger.info("🧪 Testing KVirtualStage MCP Server...")
            
            # Import test modules
            from kvirtualstage_mcp_server import KVirtualStageMCPServer
            from mcp_tools_claude_integration import ClaudeCodeMCPInterface
            
            # Test basic server creation
            server = KVirtualStageMCPServer()
            logger.info("✓ Base server created")
            
            # Test Claude integration
            claude_interface = ClaudeCodeMCPInterface()
            logger.info("✓ Claude integration created")
            
            # Test tool listing
            tools = claude_interface.get_all_tools()
            logger.info(f"✓ {len(tools)} tools available")
            
            # Test configuration loading
            config = self.load_config()
            if config:
                logger.info("✓ Configuration loaded")
            else:
                logger.warning("⚠ Configuration not found")
            
            logger.info("✅ Server test completed successfully!")
            return True
            
        except Exception as e:
            logger.error(f"Server test failed: {e}")
            return False
    
    def demo_functionality(self) -> bool:
        """Demonstrate MCP server functionality"""
        try:
            logger.info("🎯 Running KVirtualStage MCP Server Demo...")
            
            # Run demo
            from kvirtualstage_mcp_server import demo_mcp_server
            from mcp_tools_claude_integration import demo_claude_integration
            
            logger.info("Running base server demo...")
            asyncio.run(demo_mcp_server())
            
            logger.info("Running Claude integration demo...")
            asyncio.run(demo_claude_integration())
            
            logger.info("✅ Demo completed successfully!")
            return True
            
        except Exception as e:
            logger.error(f"Demo failed: {e}")
            return False
    
    def show_status(self) -> Dict[str, Any]:
        """Show server status and configuration"""
        try:
            config = self.load_config()
            
            status = {
                "installation": {
                    "config_exists": self.config_file.exists(),
                    "config_valid": config is not None,
                    "directories_created": True
                },
                "configuration": config,
                "paths": {
                    "config_dir": str(self.config_dir),
                    "config_file": str(self.config_file),
                    "log_dir": str(self.log_dir)
                }
            }
            
            # Check directories
            if config:
                for name, path in config["paths"].items():
                    exists = Path(path).exists()
                    status["installation"][f"{name}_exists"] = exists
                    if not exists:
                        status["installation"]["directories_created"] = False
            
            return status
            
        except Exception as e:
            logger.error(f"Failed to get status: {e}")
            return {"error": str(e)}
    
    def create_claude_config(self) -> bool:
        """Create Claude Code MCP configuration"""
        try:
            logger.info("📝 Creating Claude Code MCP configuration...")
            
            # Claude Code MCP configuration
            claude_mcp_config = {
                "mcpServers": {
                    "kvirtualstage": {
                        "command": "python",
                        "args": [
                            str(Path(__file__).parent / "mcp_protocol_handler.py"),
                            "--transport", "stdio",
                            "--log-level", "INFO"
                        ],
                        "env": {
                            "DISPLAY": ":1"
                        }
                    }
                }
            }
            
            # Save to Claude config location
            claude_config_dir = Path.home() / ".config" / "claude"
            claude_config_dir.mkdir(parents=True, exist_ok=True)
            
            claude_config_file = claude_config_dir / "mcp_servers.json"
            
            # If file exists, merge configurations
            if claude_config_file.exists():
                with open(claude_config_file, 'r') as f:
                    existing_config = json.load(f)
                existing_config["mcpServers"]["kvirtualstage"] = claude_mcp_config["mcpServers"]["kvirtualstage"]
                claude_mcp_config = existing_config
            
            # Save configuration
            with open(claude_config_file, 'w') as f:
                json.dump(claude_mcp_config, f, indent=2)
            
            logger.info(f"✅ Claude Code configuration saved to: {claude_config_file}")
            logger.info("Restart Claude Code to use KVirtualStage MCP tools")
            
            return True
            
        except Exception as e:
            logger.error(f"Failed to create Claude configuration: {e}")
            return False

def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(description="KVirtualStage MCP Server Setup")
    
    parser.add_argument("--install", action="store_true",
                       help="Install and configure the MCP server")
    parser.add_argument("--start", action="store_true",
                       help="Start the MCP server")
    parser.add_argument("--test", action="store_true",
                       help="Test server functionality")
    parser.add_argument("--demo", action="store_true", 
                       help="Run functionality demonstration")
    parser.add_argument("--status", action="store_true",
                       help="Show server status and configuration")
    parser.add_argument("--claude-config", action="store_true",
                       help="Create Claude Code MCP configuration")
    parser.add_argument("--transport", choices=["stdio", "tcp"], default=None,
                       help="Transport method for server")
    parser.add_argument("--config-dir", default=None,
                       help="Configuration directory path")
    parser.add_argument("--log-level", choices=["DEBUG", "INFO", "WARNING", "ERROR"],
                       default="INFO", help="Logging level")
    
    args = parser.parse_args()
    
    # Configure logging
    logging.getLogger().setLevel(getattr(logging, args.log_level))
    
    # Create setup instance
    setup = MCPServerSetup(args.config_dir)
    
    try:
        if args.install:
            success = setup.install()
            sys.exit(0 if success else 1)
        
        elif args.start:
            success = setup.start_server(args.transport)
            sys.exit(0 if success else 1)
        
        elif args.test:
            success = setup.test_server()
            sys.exit(0 if success else 1)
        
        elif args.demo:
            success = setup.demo_functionality()
            sys.exit(0 if success else 1)
        
        elif args.status:
            status = setup.show_status()
            print(json.dumps(status, indent=2))
            sys.exit(0)
        
        elif args.claude_config:
            success = setup.create_claude_config()
            sys.exit(0 if success else 1)
        
        else:
            # Show help and status by default
            parser.print_help()
            print("\n" + "="*50)
            print("Current Status:")
            status = setup.show_status()
            print(json.dumps(status, indent=2))
    
    except KeyboardInterrupt:
        logger.info("Operation cancelled by user")
        sys.exit(1)
    except Exception as e:
        logger.error(f"Setup failed: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()