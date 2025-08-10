#!/bin/bash

# Get the cc-atlas root directory (parent of scripts dir)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
CC_ATLAS_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Installing cc-atlas globally...${NC}"

# Check prerequisites
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo not found. Please install Rust first.${NC}"
    echo "Visit: https://rustup.rs/"
    exit 1
fi

if ! command -v npm &> /dev/null; then
    echo -e "${RED}Error: npm not found. Please install Node.js first.${NC}"
    echo "Visit: https://nodejs.org/"
    exit 1
fi

# Build the Rust binary in release mode
echo -e "${BLUE}Building cc-atlas...${NC}"
cd "$CC_ATLAS_ROOT"
cargo build --release

# Install frontend dependencies and build
echo -e "${BLUE}Building frontend...${NC}"
cd "$CC_ATLAS_ROOT/frontend"
npm install
npm run build

# Determine installation directory
# Try user's cargo bin first (no sudo needed), then /usr/local/bin
if [ -d "$HOME/.cargo/bin" ]; then
    INSTALL_DIR="$HOME/.cargo/bin"
    USE_SUDO=""
    echo -e "${GREEN}Installing to $INSTALL_DIR (no sudo required)${NC}"
else
    INSTALL_DIR="/usr/local/bin"
    USE_SUDO="sudo"
    echo -e "${YELLOW}Installing to $INSTALL_DIR (requires sudo)${NC}"
fi

WRAPPER_SCRIPT="$INSTALL_DIR/cc-atlas"

echo -e "${BLUE}Creating global command...${NC}"

# Create wrapper script content
WRAPPER_CONTENT="#!/bin/bash

# cc-atlas global wrapper
CC_ATLAS_HOME=\"$CC_ATLAS_ROOT\"

# Default to current directory if no path specified
PROJECT_PATH=\"\${1:-.}\"

# Convert to absolute path
PROJECT_PATH=\$(cd \"\$PROJECT_PATH\" 2>/dev/null && pwd || echo \"\$PROJECT_PATH\")

if [ ! -d \"\$PROJECT_PATH\" ]; then
    echo \"Error: Directory \$PROJECT_PATH does not exist\"
    exit 1
fi

echo \"Starting cc-atlas for: \$PROJECT_PATH\"

# Change to cc-atlas directory and run
cd \"\$CC_ATLAS_HOME\"
./scripts/start.sh --project \"\$PROJECT_PATH\"
"

# Write the wrapper script
if [ -n "$USE_SUDO" ]; then
    echo "$WRAPPER_CONTENT" | sudo tee "$WRAPPER_SCRIPT" > /dev/null
    sudo chmod +x "$WRAPPER_SCRIPT"
else
    echo "$WRAPPER_CONTENT" > "$WRAPPER_SCRIPT"
    chmod +x "$WRAPPER_SCRIPT"
fi

echo ""
echo -e "${GREEN}═══════════════════════════════════════════${NC}"
echo -e "${GREEN}cc-atlas installed successfully!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════${NC}"
echo ""
echo -e "${BLUE}Installed to:${NC} $WRAPPER_SCRIPT"
echo ""
echo -e "${BLUE}Usage:${NC}"
echo "  cc-atlas              # Analyze current directory"
echo "  cc-atlas /path/to/project  # Analyze specific project"
echo ""
echo -e "${YELLOW}Examples:${NC}"
echo "  cd ~/projects/my-app"
echo "  cc-atlas"
echo ""
echo "  # Or from anywhere:"
echo "  cc-atlas ~/projects/my-app"
echo ""
echo -e "${YELLOW}Note:${NC} Make sure $INSTALL_DIR is in your PATH"
echo ""