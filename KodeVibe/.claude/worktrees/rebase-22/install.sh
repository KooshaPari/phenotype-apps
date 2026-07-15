#!/bin/bash

# KodeVibe Installer
# Version: 1.0.0
# Author: KooshaPari

set -e

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}🌊 Installing KodeVibe - The Ultimate Code Quality Guardian${NC}"

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   echo -e "${RED}❌ This script should not be run as root${NC}" 
   exit 1
fi

# Default installation directory
INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="$HOME/.kodevibe"

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --install-dir)
            INSTALL_DIR="$2"
            shift 2
            ;;
        --config-dir)
            CONFIG_DIR="$2"
            shift 2
            ;;
        --local)
            INSTALL_DIR="$HOME/.local/bin"
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  --install-dir DIR    Installation directory (default: /usr/local/bin)"
            echo "  --config-dir DIR     Configuration directory (default: ~/.kodevibe)"
            echo "  --local              Install to ~/.local/bin"
            echo "  --help               Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}❌ Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

echo -e "${YELLOW}📁 Installation directory: $INSTALL_DIR${NC}"
echo -e "${YELLOW}⚙️  Configuration directory: $CONFIG_DIR${NC}"

# Create directories
mkdir -p "$INSTALL_DIR" "$CONFIG_DIR"

# Check if kodevibe already exists
if [[ -f "$INSTALL_DIR/kodevibe" ]]; then
    echo -e "${YELLOW}⚠️  KodeVibe already exists. Backing up...${NC}"
    cp "$INSTALL_DIR/kodevibe" "$INSTALL_DIR/kodevibe.backup.$(date +%Y%m%d_%H%M%S)"
fi

# Download or copy kodevibe script
if [[ -f "./kodevibe" ]]; then
    echo -e "${GREEN}📦 Installing from local file...${NC}"
    cp "./kodevibe" "$INSTALL_DIR/kodevibe"
else
    echo -e "${GREEN}📡 Downloading from GitHub...${NC}"
    REPO_URL="https://raw.githubusercontent.com/KooshaPari/KodeVibe/main"
    
    # Check if curl is available
    if command -v curl &> /dev/null; then
        curl -sSL "$REPO_URL/kodevibe" -o "$INSTALL_DIR/kodevibe"
    elif command -v wget &> /dev/null; then
        wget -q "$REPO_URL/kodevibe" -O "$INSTALL_DIR/kodevibe"
    else
        echo -e "${RED}❌ Neither curl nor wget found. Please install one of them.${NC}"
        exit 1
    fi
fi

# Make executable
chmod +x "$INSTALL_DIR/kodevibe"

# Create default configuration if it doesn't exist
if [[ ! -f "$CONFIG_DIR/config.yaml" ]]; then
    echo -e "${GREEN}⚙️  Creating default configuration...${NC}"
    cat > "$CONFIG_DIR/config.yaml" << 'EOF'
# KodeVibe Configuration
vibes:
  security:
    enabled: true
    level: strict
    
  code:
    enabled: true
    level: moderate
    max_function_length: 50
    max_nesting_depth: 4
    
  performance:
    enabled: true
    level: moderate
    
  file:
    enabled: true
    level: strict
    
  git:
    enabled: true
    min_commit_message_length: 10
    
  dependency:
    enabled: true
    check_vulnerabilities: true
    
  documentation:
    enabled: false

# Project settings
project:
  type: auto-detect
  
# File exclusions
exclude:
  files:
    - "node_modules/**/*"
    - ".git/**/*"
    - "coverage/**/*"
    - "*.min.js"
    - "*.min.css"
    - "vendor/**/*"
    - "build/**/*"
    - "dist/**/*"
EOF
fi

# Add to PATH if not already there
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}⚠️  $INSTALL_DIR is not in your PATH${NC}"
    echo -e "${YELLOW}💡 Add this line to your ~/.bashrc or ~/.zshrc:${NC}"
    echo -e "${YELLOW}   export PATH=\"$INSTALL_DIR:\$PATH\"${NC}"
fi

# Test installation
if "$INSTALL_DIR/kodevibe" version &> /dev/null; then
    echo -e "${GREEN}✅ KodeVibe installed successfully!${NC}"
    echo -e "${GREEN}📚 Run 'kodevibe help' to get started${NC}"
    echo -e "${GREEN}⚙️  Configuration: $CONFIG_DIR/config.yaml${NC}"
else
    echo -e "${RED}❌ Installation failed. Please check permissions.${NC}"
    exit 1
fi

# Offer to install git hooks
echo ""
read -p "🔗 Would you like to install git hooks in the current directory? (y/n): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    if [[ -d ".git" ]]; then
        "$INSTALL_DIR/kodevibe" hooks install
        echo -e "${GREEN}✅ Git hooks installed${NC}"
    else
        echo -e "${YELLOW}⚠️  Not a git repository. Skipping git hooks installation.${NC}"
    fi
fi

echo -e "${GREEN}🎉 KodeVibe installation complete!${NC}"
echo -e "${GREEN}🚀 Get started with: kodevibe scan${NC}"