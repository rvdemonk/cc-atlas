# Backend (src/)

## Purpose
Rust backend for cc-atlas - analyzes CLAUDE.md files, serves API, manages file operations.

## Structure
```
src/
├── server/     HTTP layer (routes, handlers, errors)
├── services/   Business logic (file analysis, chat exports)
├── utils/      Shared utilities (markdown, paths)
├── models.rs   Data structures
└── main.rs     CLI entry
```

## Services
- **analyzer** - CLAUDE.md discovery, directory tree building, complexity analysis
- **chat_exporter** (planned) - Parse ~/.claude/projects/[project]/chats, export to markdown transcripts

## Key Patterns
- Handlers are thin - delegate to services
- Path conversion happens in utils/paths
- Errors use ServerError enum (NotFound, Internal, BadRequest, Conflict)
- All imports: services::*, utils::*, models::*

## Recent Changes
Backend reorganized Jan 2025: server/handlers/routes separated, services layer established, structured errors.

## Constraints
- No database - filesystem only
- Keep handlers stateless
- Services never import from server/
