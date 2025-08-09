# Scripts Context

## Purpose
Shell scripts for running, installing, and developing cc-atlas.

## Scripts Overview
- `start.sh` - Production mode (builds release binary)
- `dev.sh` - Development mode (cargo run, hot reload)
- `install.sh` - Global installation to /usr/local/bin

## Common Patterns

### Path Resolution
```bash
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
CC_ATLAS_ROOT="$(dirname "$SCRIPT_DIR")"
```

### Color Output
```bash
GREEN='\033[0;32m'
BLUE='\033[0;34m'
echo -e "${GREEN}Success!${NC}"
```

### Process Management
```bash
# Start background process
./app &
PID=$!

# Cleanup on exit
trap cleanup INT TERM
cleanup() {
    kill $PID 2>/dev/null
}
```

### Dependency Checks
```bash
if [ ! -d "frontend/node_modules" ]; then
    npm install
fi
```

## Usage Patterns
- Always cd to CC_ATLAS_ROOT before operations
- Check for cargo/npm before running
- Provide clear colored output
- Handle Ctrl+C gracefully

## Future
- Add update script for pulling latest changes
- Consider systemd service for production