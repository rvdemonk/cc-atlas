cc-atlas Project Context
==========

Purpose
----------

Web-based management system for Claude Code CLAUDE.md files across codebases. The main goal is to provide a rich and intuitive editing experience for the CLAUDE.md memory files in a codebase, without having to scour and use the IDE. Memory files can be edited, created and deleted.

Architecture
----------

* **Backend**: Rust with Axum (port 3999)

* **Frontend**: React with TypeScript (port 3000)

* **Storage**: File system only, no database

Key Principles
----------

1. **Simplicity first** - MVP focused, avoid over-engineering

2. **Small functions** - Each function does one thing well

3. **Expandable** - Easy to add features without major refactoring

Project Structure
----------

```
cc-atlas           # Main entry point
src/               # Rust backend - analyzer, server, models
frontend/          # React UI - tree view, editor
scripts/           # Shell scripts for running/installing
```

Development Workflow
----------

```
./scripts/dev.sh   # Development with hot reload
./scripts/start.sh # Production build
cargo check        # Verify Rust compilation
```

* Don't start the server unless I ask -- I'm usually running it myself.

Core Features
----------

* Find all CLAUDE.md files in a project

* Tree view with visual indicators

* Edit memory files with live save

* Complexity analysis (\>10 files or \>500 lines triggers recommendation)

Next Phase
----------

* Staleness detection based on git commits and file changes

* Automatic creation algorithms and prompts for memory files
