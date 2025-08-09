# cc-atlas

A context management system for Claude Code `CLAUDE.md` files. Provides a web-based dashboard for viewing, editing, and maintaining memory files across your codebase.

## Quick Start

### Easy Way (Using Scripts)

```bash
# Run both backend and frontend
./cc-atlas

# Or from the scripts directory
./scripts/start.sh --project /path/to/project

# Development mode with hot reload
./scripts/dev.sh
```

### Manual Way

```bash
# Backend (Rust)
cargo run -- serve --port 3999 --project .

# Frontend (React) - in another terminal
cd frontend
npm install
npm run dev
```

Then open http://localhost:3000

## Features

- 📝 **Find all CLAUDE.md files** - Automatically discovers all memory files in your project
- 🌳 **Tree view** - Visual hierarchy of your project structure
- 💡 **Recommendations** - Suggests where new memory files would be valuable (>10 files or >500 lines)
- ✏️ **Live editing** - Edit memory files directly in the browser
- 📊 **Stats** - Shows file count, line count, and depth for each directory

## Architecture

- **Backend**: Rust with Axum web framework
- **Frontend**: React with TypeScript
- **No database**: Everything operates on the file system directly

## API Endpoints

- `GET /api/memory-files` - List all CLAUDE.md files
- `GET /api/tree` - Get directory tree structure
- `PUT /api/memory-files/{path}` - Update a memory file
- `GET /api/recommendations` - Get recommended locations for new memory files

## Installation (Global Usage)

```bash
# Install cc-atlas globally
./scripts/install.sh

# Then use from any project
cd /any/project
cc-atlas
```

## Scripts

All scripts are in the `scripts/` directory:

- `scripts/start.sh` - Production mode with built binaries
- `scripts/dev.sh` - Development mode with hot reload
- `scripts/install.sh` - Install globally to `/usr/local/bin`

## Next Steps

- Add staleness detection based on file modification times
- Git integration for commit-based staleness
- Better markdown editing experience
- Export/import functionality