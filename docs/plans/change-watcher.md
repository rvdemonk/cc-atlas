# Hash-Based Change Detection Implementation Plan

## Overview

Implement a hash-based change detection system that checks for external modifications to CLAUDE.md files before operations and prompts users to resolve conflicts when detected. This approach prioritizes data integrity and user control while keeping implementation complexity low.

## Core Concept

- Store SHA-256 hash of file content when loaded
- Before any save operation, re-read and hash the file
- If hashes differ, external changes occurred - prompt user to resolve
- Extend this to also check periodically and on navigation events

## Implementation Details

### 1. Data Model Changes

#### `src/models.rs`
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFile {
    pub path: PathBuf,
    pub content: String,
    pub content_hash: String,  // NEW: SHA-256 of content
    pub last_checked: SystemTime,  // NEW: When we last verified
    pub external_modified: bool,  // NEW: Flag for UI indicator
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileConflict {
    pub path: PathBuf,
    pub current_content: String,  // What's in editor
    pub disk_content: String,     // What's on disk
    pub base_content: String,     // Original content when loaded
}
```

### 2. Hash Calculation Module

#### `src/hash.rs` (new file)
```rust
use sha2::{Sha256, Digest};

pub fn calculate_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

pub fn file_hash(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(calculate_hash(&content))
}
```

### 3. Server Endpoints

#### Modified Endpoints

**GET `/api/memory/:path`**
- Calculate and store hash when loading file
- Return hash to frontend with content

**PUT `/api/memory/:path`**
- Check current file hash before saving
- If mismatch, return 409 Conflict with both versions
- If match, save and update hash

#### New Endpoints

**POST `/api/memory/:path/check`**
- Quick hash check without loading full content
- Returns: `{ "changed": bool, "current_hash": String }`

**POST `/api/memory/:path/resolve`**
- Accept conflict resolution choice
- Options: "keep_mine", "take_theirs", "merge"
- Update hash after resolution

**GET `/api/memory/check-all`**
- Batch check all tracked files
- Returns list of changed files
- Called periodically by frontend

### 4. Frontend Changes

#### State Management
```typescript
interface MemoryFileState {
  path: string;
  content: string;
  contentHash: string;
  hasExternalChanges: boolean;
  lastChecked: Date;
}

interface ConflictState {
  isOpen: boolean;
  conflict: FileConflict | null;
  resolution: 'mine' | 'theirs' | 'merge' | null;
}
```

#### Conflict Resolution UI

**New Component: `ConflictModal.tsx`**
- Side-by-side diff view (current vs external)
- Three options:
  1. Keep my version (discard external)
  2. Use external version (discard mine)
  3. Manual merge (future: provide merge editor)
- Show what changed with diff highlighting

#### Change Detection Triggers

1. **Before Save** (Critical)
   - Always check hash before PUT request
   - Block save if conflict, show modal

2. **On File Navigation** 
   - Check hash when user clicks different file
   - Show indicator if external changes detected

3. **Periodic Background Check** (Every 30s)
   - Call `/api/memory/check-all`
   - Update UI indicators for changed files
   - Show subtle badge on tree nodes

4. **On Window Focus**
   - When browser tab regains focus
   - Quick check of current file
   - Useful after IDE editing

### 5. Visual Indicators

#### Tree View
- Yellow dot badge = external changes detected
- Tooltip: "Modified externally"

#### Editor Header
- Warning banner when file has external changes
- "This file was modified externally. [Reload] [Ignore]"

#### Save Button
- Disabled state with tooltip when conflict exists
- "Resolve external changes before saving"

### 6. Error Handling

#### Race Conditions
- Use optimistic locking with hash as version
- Retry logic with exponential backoff
- Maximum 3 retries before showing error

#### File Permissions
- Gracefully handle read-only files
- Clear error messages for permission issues

#### Network Failures
- Queue hash checks when offline
- Reconcile when connection restored

### 7. Performance Optimizations

#### Caching
- Cache hashes for 5 seconds to avoid repeated reads
- Invalidate cache on known modifications

#### Batch Operations
- Group multiple hash checks in single request
- Debounce rapid navigation events

#### Large Files
- Skip hash checking for files > 1MB
- Use modification time as fallback

### 8. Configuration

#### `config.toml` (optional future feature)
```toml
[change_detection]
enabled = true
check_interval_seconds = 30
check_on_focus = true
check_on_navigation = true
max_file_size_bytes = 1048576
```

## Implementation Phases

### Phase 1: Core Hash Checking (MVP)
1. Add hash field to MemoryFile model
2. Implement hash calculation
3. Add conflict detection to save endpoint
4. Basic conflict modal UI
5. Update frontend to handle 409 responses

### Phase 2: Proactive Detection
1. Add check-all endpoint
2. Implement periodic checking
3. Add tree view indicators
4. Window focus detection

### Phase 3: Enhanced UX
1. Diff visualization in conflict modal
2. Merge option (if feasible)
3. Conflict history/undo
4. Keyboard shortcuts

### Phase 4: Optimization
1. Add caching layer
2. Batch operations
3. Performance monitoring
4. Configuration options

## Testing Strategy

### Unit Tests
- Hash calculation correctness
- Conflict detection logic
- Resolution strategies

### Integration Tests
- Concurrent modification scenarios
- Network failure handling
- Large file handling

### Manual Testing Scenarios
1. Edit in IDE while web UI open
2. Claude Code updates while UI open
3. Git operations changing files
4. Multiple browser tabs

## Success Metrics

- Zero data loss from conflicts
- < 100ms overhead on save operations
- < 500ms to detect all changes in typical project
- User can always see which version they're choosing

## Alternative Considerations

### Why Not File Watching?
- Complexity of cross-platform watching
- Resource usage for large codebases
- Dealing with watch descriptor limits
- This approach is simpler and sufficient

### Why Not Git Integration?
- Not all changes are committed immediately
- Adds git as a dependency
- Complex to handle staging area
- This approach is more direct

## Migration Path

1. Deploy with feature flag disabled
2. Store hashes for newly loaded files
3. Gradually enable for beta users
4. Full rollout after validation

## Dependencies

```toml
# Cargo.toml additions
sha2 = "0.10"
similar = "2.3"  # For diff display (optional)
```

## Timeline Estimate

- Phase 1: 2-3 days
- Phase 2: 2 days  
- Phase 3: 3-4 days
- Phase 4: 2 days

Total: ~2 weeks for full implementation

## Open Questions

1. Should we auto-reload if no local changes exist?
2. How to handle binary CLAUDE.md files (shouldn't exist but...)?
3. Should we track file moves/renames?
4. Integration with future collaboration features?