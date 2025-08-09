# cc-atlas: A Context Management System for Claude Code

## Project Overview

cc-atlas is a specialized wiki and management system for `CLAUDE.md` context files in codebases using Claude Code. It provides a web-based dashboard for viewing, editing, and maintaining these memory files, with intelligent recommendations about where they're needed and when they're stale.

## Core Problem

When using Claude Code on complex codebases, a single `CLAUDE.md` file becomes insufficient. Distributed context files at architectural junctions (e.g., `Core/Models/CLAUDE.md`, `Features/Auth/CLAUDE.md`) are more effective, but managing these scattered files is cumbersome. Users need:

1. A unified view of all memory files
2. Recommendations for where new memory files would be valuable
3. Detection of stale memory that needs updating
4. An easy way to edit without navigating directories

## Architecture

### Technology Stack

- **Backend**: Rust (fast, efficient file watching, easy to use with Claude Code via `cargo check`)
- **Web Framework**: Axum or Actix-web
- **Frontend**: React or SolidJS (minimal, fast)
- **File Watching**: notify-rs for detecting code changes
- **Storage**: File system (no database needed)
- **Markdown Parsing**: pulldown-cmark
- **Git Integration**: git2-rs for commit history analysis

### Project Structure

```
claude-memory/
├── Cargo.toml
├── src/
│   ├── main.rs                 # Entry point, CLI args, server startup
│   ├── analyzer/
│   │   ├── mod.rs
│   │   ├── complexity.rs       # Calculate directory complexity scores
│   │   ├── recommendations.rs  # Suggest where memory files needed
│   │   └── staleness.rs        # Detect outdated memory files
│   ├── watcher/
│   │   ├── mod.rs
│   │   └── file_monitor.rs     # Track file changes for staleness
│   ├── server/
│   │   ├── mod.rs
│   │   ├── routes.rs           # API endpoints
│   │   └── static.rs           # Serve frontend
│   └── models/
│       ├── mod.rs
│       └── types.rs            # Core data structures
├── frontend/
│   ├── index.html
│   ├── src/
│   │   ├── App.tsx
│   │   ├── components/
│   │   │   ├── TreeView.tsx    # File tree with memory indicators
│   │   │   ├── Editor.tsx      # Markdown editor
│   │   │   ├── Complexity.tsx  # Heatmap/visual of complexity
│   │   │   └── Staleness.tsx   # Stale file indicators
│   │   └── api/
│   │       └── client.ts       # API communication
│   └── package.json
└── README.md
```

## Core Components

### 1. Complexity Analyzer

```rust
pub struct ComplexityScore {
    pub path: PathBuf,
    pub file_count: usize,
    pub total_lines: usize,
    pub language_diversity: f32,  // Number of different file types
    pub depth: usize,             // Directory depth
    pub recommendation: RecommendationLevel,
}

pub enum RecommendationLevel {
    Essential,    // >30 files or critical directories
    Recommended,  // 10-30 files or moderate complexity
    Optional,     // <10 files
    None,         // Too simple to need memory
}

impl ComplexityAnalyzer {
    pub fn analyze_directory(&self, path: &Path) -> ComplexityScore {
        // Walk directory
        // Count files and lines
        // Detect file types
        // Calculate complexity score
        // Return recommendation
    }
}
```

### 2. Staleness Detector

```rust
pub struct StalenessInfo {
    pub memory_path: PathBuf,
    pub last_updated: SystemTime,
    pub files_changed_since: Vec<FileChange>,
    pub commits_since: usize,
    pub staleness_level: StalenessLevel,
}

pub enum StalenessLevel {
    Critical,  // >50 commits or major structural changes
    Stale,     // >20 commits or >30 days old
    Aging,     // >10 commits or >14 days old  
    Fresh,     // Recently updated
}

pub struct FileChange {
    pub path: PathBuf,
    pub change_type: ChangeType,
    pub timestamp: SystemTime,
    pub lines_changed: usize,
}
```

### 3. File Watcher

```rust
use notify::{Watcher, RecursiveMode, watcher};

pub struct MemoryWatcher {
    watcher: RecommendedWatcher,
    change_log: HashMap<PathBuf, Vec<FileChange>>,
    memory_files: HashMap<PathBuf, SystemTime>,
}

impl MemoryWatcher {
    pub fn watch_project(&mut self, root: &Path) {
        // Watch all files except .git, node_modules, etc.
        // Track changes relative to nearest CLAUDE.md
        // Update staleness scores
    }
}
```

### 4. Web API

```rust
// GET /api/memory-files
// Returns all CLAUDE.md files with their content and metadata

// GET /api/complexity
// Returns complexity analysis for entire project tree

// GET /api/staleness
// Returns staleness info for all memory files

// GET /api/recommendations  
// Returns list of directories that should have memory files

// PUT /api/memory-files/{path}
// Update or create a memory file

// POST /api/analyze-changes/{path}
// Get summary of changes since memory was last updated
```

## Frontend Design

### Layout

```
┌─────────────────────────────────────────────────────────┐
│ cc-atlas                          [Stale: 3] [▼]   │
├───────────────┬─────────────────────────────────────────┤
│              │                                           │
│  Project     │  Core/Models/CLAUDE.md                    │
│  └── Core    │  ┌─────────────────────────────────┐     │
│      ├── 📝  │  │ Last updated: 15 days ago        │     │
│      └── M.. │  │ 47 commits since update          │     │
│  └── Feat..  │  │ [Update Needed]                  │     │
│      ├── 📄  │  └─────────────────────────────────┘     │
│      └── T.. │                                           │
│              │  # Models Architecture                    │
│  Legend:     │                                           │
│  📝 Has mem  │  ## Purpose                               │
│  ⚠️  Stale   │  This directory contains all core...     │
│  💡 Suggest  │                                           │
│  📄 Current  │  ## Patterns                              │
│              │  - All models conform to EventProtocol   │
│              │                                           │
└───────────────┴─────────────────────────────────────────┘
```

### Key Features

1. **Tree View** (left panel)
   - Show directory structure
   - Icons indicate memory file status
   - Badges for complexity scores
   - Click to navigate

2. **Editor** (center/right panel)
   - Markdown editor with syntax highlighting
   - Save shortcut (Ctrl+S)
   - Show staleness warning banner
   - Quick actions toolbar

3. **Staleness Banner**
   ```
   ⚠️ This memory file may be stale
   Last updated: 15 days ago | 47 commits | 12 files changed
   [View Changes] [Generate Update Prompt] [Dismiss]
   ```

4. **Quick Actions**
   - "Copy Update Prompt" - generates Claude Code prompt
   - "View Recent Changes" - shows diff summary
   - "Create Memory File" - for recommended directories

## CLI Commands

```bash
# Start the server
claude-memory serve [--port 3999] [--project .]

# Analyze without starting server
claude-memory analyze
> Essential: Core/ (87 files, high complexity)
> Recommended: Features/Auth/ (23 files)
> 3 memory files are stale

# Generate update prompt
claude-memory prompt update Core/Models
> Copied: "Update Core/Models/CLAUDE.md based on recent changes..."

# Check staleness
claude-memory status
> Critical: Core/Models/CLAUDE.md (47 commits behind)
> Stale: Features/CLAUDE.md (25 commits behind)
> Fresh: 5 memory files up to date
```

## Implementation Phases

### Phase 1: MVP (Week 1)
- [x] Project structure setup
- [ ] Basic complexity analyzer
- [ ] Find and display all CLAUDE.md files
- [ ] Simple web UI with tree view and editor
- [ ] Save/load functionality

### Phase 2: Intelligence (Week 2)
- [ ] Staleness detection via file modification times
- [ ] Recommendations based on complexity
- [ ] Change detection (files modified since last update)
- [ ] Visual indicators in UI

### Phase 3: Git Integration (Week 3)
- [ ] Commit counting for staleness
- [ ] Better change summaries
- [ ] Diff visualization
- [ ] Historical tracking

### Phase 4: Polish (Week 4)
- [ ] Prompt generation templates
- [ ] Customizable thresholds
- [ ] Export/import functionality
- [ ] Team sharing features

## Configuration

`.claude-memory/config.toml`:
```toml
[analyzer]
complexity_threshold = 10  # Files needed for recommendation
staleness_days = 14        # Days before marking stale
staleness_commits = 20     # Commits before marking stale

[watcher]
ignore_patterns = [
    "node_modules",
    ".git",
    "target",
    "dist",
    "build"
]

[ui]
port = 3999
auto_open = true

[templates]
# User-defined memory file template
default_sections = [
    "Purpose",
    "Patterns", 
    "Dependencies",
    "Tech Debt"
]
```

## Success Metrics

1. **Discoverability**: Users can see all memory files in <2 seconds
2. **Staleness Detection**: Accurately identifies when updates needed
3. **Low Overhead**: Runs in background using <50MB RAM
4. **Zero Config**: Works out of the box with sensible defaults
5. **Claude Code Integration**: Generates prompts that Claude can execute

## Technical Decisions

### Why Rust?
- Fast file system operations
- Low memory footprint for background monitoring
- Excellent async support for file watching
- Easy integration with Claude Code (`cargo check`)
- Can compile to single binary for distribution

### Why Web UI?
- Cross-platform without additional complexity
- Familiar editing experience
- Easy to extend with additional features
- No installation required for frontend

### Why No Database?
- Everything is already in files
- Reduces complexity and dependencies
- Git provides versioning
- Can add SQLite later if needed

## Getting Started for Development

```bash
# Clone and setup
git clone https://github.com/yourusername/claude-memory
cd claude-memory

# Backend
cargo build
cargo run -- serve

# Frontend (separate terminal)
cd frontend
npm install
npm run dev

# Test with a real project
claude-memory serve --project ../pum-ios
```

## Distribution

```bash
# Build release binary
cargo build --release

# Frontend built into binary
# Use rust-embed to include frontend assets

# Single command install
cargo install claude-memory

# Or download binary
# GitHub releases with pre-built binaries
```

---

This document provides a rough idea, non-binding, to start building cc-atlas. The key is to start simple (Phase 1) and iterate based on real usage. The architecture is designed to be extensible without major refactoring.
