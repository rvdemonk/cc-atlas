# Rust Backend Context

## Module Organization
- `main.rs` - CLI entry, command routing
- `analyzer.rs` - File analysis, complexity scoring, recommendations
- `server.rs` - Axum web server, API routes
- `models.rs` - Core data structures

## Key Patterns

### Small Functions
Every function does ONE thing. If it's >15 lines, split it.

### Error Handling
- Use `Result<T>` everywhere
- `anyhow::Result` for simplicity
- Let errors bubble up to handlers

### Analyzer Module
```rust
// Constants at top
const COMPLEXITY_FILE_THRESHOLD: usize = 10;

// Public API functions first
pub fn analyze_project(path: &str) -> Result<()>

// Private helpers below
fn calculate_stats(path: &Path) -> Result<FileStats>
```

### Server Module
- State wrapped in `Arc<AppState>`
- Simple REST routes: GET tree, GET/PUT memory files
- CORS enabled for frontend dev

## Dependencies
- `axum` - Web framework
- `walkdir` - Directory traversal
- `clap` - CLI parsing
- Keep minimal, add only when necessary

## Style
- Early returns over nested ifs
- Explicit types in function signatures
- Constants for magic numbers