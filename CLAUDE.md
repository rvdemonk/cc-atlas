# cc-atlas Project Context

## Purpose
Web-based management system for Claude Code CLAUDE.md files across codebases.

## Architecture
- **Backend**: Rust with Axum (port 3999)
- **Frontend**: React with TypeScript (port 3000)
- **Storage**: File system only, no database

## Key Principles
1. **Simplicity first** - MVP focused, avoid over-engineering
2. **Small functions** - Each function does one thing well
3. **Expandable** - Easy to add features without major refactoring

## Project Structure
```
cc-atlas           # Main entry point
src/               # Rust backend - analyzer, server, models
frontend/          # React UI - tree view, editor
scripts/           # Shell scripts for running/installing
```

## Development Workflow
```bash
./scripts/dev.sh   # Development with hot reload
./scripts/start.sh # Production build
cargo check        # Verify Rust compilation
```

## Core Features
- Find all CLAUDE.md files in a project
- Tree view with visual indicators (📝 has memory, 💡 recommended)
- Edit memory files with live save
- Complexity analysis (>10 files or >500 lines triggers recommendation)

## Next Phase
- Staleness detection based on git commits
- In-browser file creation
- Better markdown editing experience