#!/bin/bash

# Get the cc-atlas root directory (parent of scripts dir)
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
CC_ATLAS_ROOT="$(dirname "$SCRIPT_DIR")"

# Development mode - uses cargo run instead of release binary
# and runs frontend in dev mode with hot reload

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

PROJECT_PATH="${1:-.}"
PORT="${2:-3999}"

# Convert to absolute path BEFORE changing directory
if [ -d "$PROJECT_PATH" ]; then
    PROJECT_PATH=$(cd "$PROJECT_PATH" && pwd)
fi

echo -e "${BLUE}Starting cc-atlas in development mode${NC}"
echo -e "Project: $PROJECT_PATH"
echo -e "Port: $PORT"
echo ""

cleanup() {
    echo -e "\n${YELLOW}Stopping development servers...${NC}"
    kill $BACKEND_PID 2>/dev/null
    kill $FRONTEND_PID 2>/dev/null
    exit
}

trap cleanup INT TERM

# Change to cc-atlas root directory
cd "$CC_ATLAS_ROOT"

# Install frontend dependencies if needed
if [ ! -d "frontend/node_modules" ]; then
    echo -e "${BLUE}Installing frontend dependencies...${NC}"
    (cd frontend && npm install)
fi

# Start backend in development mode
echo -e "${GREEN}Starting Rust backend (dev)...${NC}"
cargo run -- serve --port $PORT --project "$PROJECT_PATH" &
BACKEND_PID=$!

sleep 2

# Start frontend in development mode
echo -e "${GREEN}Starting React frontend (dev)...${NC}"
(cd frontend && npm run dev) &
FRONTEND_PID=$!

echo ""
echo -e "${GREEN}Development servers running!${NC}"
echo -e "Frontend: ${BLUE}http://localhost:3000${NC} (with hot reload)"
echo -e "Backend:  ${BLUE}http://localhost:$PORT${NC}"
echo ""
echo -e "${YELLOW}Press Ctrl+C to stop${NC}"

wait $BACKEND_PID $FRONTEND_PID