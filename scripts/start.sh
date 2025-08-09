#!/bin/bash

# Get the cc-atlas root directory (parent of scripts dir)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
CC_ATLAS_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
PORT=3999
PROJECT_PATH="."

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -p|--project)
            PROJECT_PATH="$2"
            shift 2
            ;;
        --port)
            PORT="$2"
            shift 2
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  -p, --project PATH   Path to project to analyze (default: current directory)"
            echo "  --port PORT          Port for the server (default: 3999)"
            echo "  -h, --help           Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use -h for help"
            exit 1
            ;;
    esac
done

# Convert to absolute path
PROJECT_PATH=$(cd "$PROJECT_PATH" 2>/dev/null && pwd || echo "$PROJECT_PATH")

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║         Starting cc-atlas              ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}Project:${NC} $PROJECT_PATH"
echo -e "${GREEN}Port:${NC} $PORT"
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${YELLOW}Warning: Cargo not found. Please install Rust.${NC}"
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo -e "${YELLOW}Warning: npm not found. Please install Node.js.${NC}"
    exit 1
fi

# Function to cleanup on exit
cleanup() {
    echo -e "\n${YELLOW}Shutting down cc-atlas...${NC}"
    kill $BACKEND_PID 2>/dev/null
    kill $FRONTEND_PID 2>/dev/null
    exit
}

# Set up trap to cleanup on Ctrl+C
trap cleanup INT TERM

# Change to cc-atlas root directory
cd "$CC_ATLAS_ROOT"

# Build backend if needed
echo -e "${BLUE}Building Rust backend...${NC}"
cargo build --release

# Install frontend dependencies if needed
if [ ! -d "frontend/node_modules" ]; then
    echo -e "${BLUE}Installing frontend dependencies...${NC}"
    (cd frontend && npm install)
fi

# Start backend
echo -e "${GREEN}Starting backend server on port $PORT...${NC}"
./target/release/cc-atlas serve --port $PORT --project "$PROJECT_PATH" &
BACKEND_PID=$!

# Wait a moment for backend to start
sleep 2

# Start frontend
echo -e "${GREEN}Starting frontend development server...${NC}"
(cd frontend && npm run dev -- --host) &
FRONTEND_PID=$!

echo ""
echo -e "${GREEN}═══════════════════════════════════════════${NC}"
echo -e "${GREEN}cc-atlas is running!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════${NC}"
echo -e "Frontend: ${BLUE}http://localhost:3000${NC}"
echo -e "Backend:  ${BLUE}http://localhost:$PORT${NC}"
echo -e "Project:  ${BLUE}$PROJECT_PATH${NC}"
echo ""
echo -e "${YELLOW}Press Ctrl+C to stop${NC}"
echo ""

# Wait for both processes
wait $BACKEND_PID $FRONTEND_PID