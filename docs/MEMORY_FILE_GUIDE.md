# CLAUDE.md Memory File Guide

## Purpose

Memory files provide context for AI assistants working on codebases. They capture decision rationale, non-obvious patterns, and essential context that isn't evident from reading code alone.

## Key Principles

1. **Non-obvious over obvious** - Don't document what's clear from types, names, or standard idioms
2. **Rationale over implementation** - Explain WHY, not WHAT
3. **Constraints over permissions** - "Don't use X because Y" prevents backsliding
4. **Current over historical** - Focus on NOW, minimal history
5. **Concise over comprehensive** - Hard limit: <70 lines for small projects, <150 for large modules

## When to Create Memory Files

- At key architectural junctions (root, major modules)
- When patterns aren't self-evident from code structure
- After significant refactors to document new patterns
- When specific constraints must be maintained

## When NOT to Create Memory Files

- For every directory (over-documentation reduces signal)
- To explain standard language/framework patterns
- For implementation details that change frequently
- When code structure makes intent obvious

## Template (Flexible)

```markdown
# [Module/Directory Name]

## Purpose
[1-2 lines: why this exists, what problem it solves]

## [Pick 2-4 sections that add most value:]

### Structure
[Only if non-obvious - quick map of where things are]

### Key Patterns
[Conventions THIS codebase follows, not general best practices]

### Services
[List of main services with one-line descriptions]

### Constraints
[What NOT to do, with rationale]

### Dependencies
[Specific technology choices or avoidances]

### Recent Changes
[If recently changed or in flux - establishes current state]

### Gotchas
[Non-obvious behaviors, edge cases]
```

## What to Include

**Good examples:**
- "We tried Redux, switched to Context because state is simple"
- "Error handling refactored Jan 2025: use ServerError enum"
- "PathBuf conversion must happen in utils/paths, not handlers"
- "Services never import from server/ - one-way dependency"

**Bad examples:**
- "This is the server module" (obvious)
- "Use camelCase for JavaScript" (standard convention)
- Detailed API documentation (belongs in code comments)
- Implementation that changes with each feature

## Hierarchy Guidance

**Flat repositories (2-3 levels):**
- Root: Architecture overview, main modules
- Key junctions only: 1-2 memory files for critical areas

**Hierarchical repositories (4+ levels):**
- Root: High-level architecture, core decisions
- Major modules: Module-specific patterns, constraints
- Deep modules: Only if unique patterns exist

## Maintenance

- Update when patterns change significantly
- Delete outdated sections rather than accumulating history
- If it becomes >100 lines, split or prune aggressively
- Review quarterly: is this still true? Still needed?

## Example (cc-atlas Backend)

```markdown
# Backend (src/)

## Purpose
Rust backend for cc-atlas - analyzes CLAUDE.md files, serves API.

## Structure
src/
├── server/     HTTP layer (routes, handlers, errors)
├── services/   Business logic (file analysis)
├── utils/      Shared utilities (markdown, paths)

## Services
- **analyzer** - File discovery, tree building, complexity analysis
- **chat_exporter** (planned) - Export chat transcripts to markdown

## Key Patterns
- Handlers are thin - delegate to services
- Path conversion happens in utils/paths
- Errors use ServerError enum
- Imports: services::*, utils::*, models::*

## Recent Changes
Reorganized Jan 2025: server/handlers/routes separated,
services layer established, structured errors.

## Constraints
- No database - filesystem only
- Services never import from server/
```

## Anti-patterns to Avoid

❌ **Too prescriptive** - Limits growth, makes refactoring feel like breaking rules
❌ **Too detailed** - Becomes stale, requires constant maintenance
❌ **Duplicating code comments** - If it's in docstrings, don't repeat
❌ **Historical narrative** - Not a changelog, focus on current state
❌ **Implementation tutorials** - "How to add a route" belongs in docs/

## Questions to Ask

Before adding content, ask:
1. Is this obvious from reading the code for 2 minutes?
2. Would someone make a mistake without knowing this?
3. Is this a temporary state or permanent pattern?
4. Does this explain WHY rather than WHAT?

If "no" to most: don't include it.
