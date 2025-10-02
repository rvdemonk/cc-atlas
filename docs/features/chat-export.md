# Chat Export Feature

## Overview

Export Claude Code conversation transcripts from `~/.claude/projects/[project-name]/` to readable markdown files for sharing with other AI models or archival purposes.

## Use Case

When conversations contain valuable context that shouldn't be compressed or summarized:

**Example scenario:**
1. User has raw, detailed discussion with Sonnet about a problem
2. Sonnet provides a well-reasoned plan
3. User switches to Opus, gets completely different perspective
4. User wants to export both responses verbatim to share with GPT-5 for third-party adjudication
5. Raw prompt + both responses are too nuanced to compress into a summary document

The lossy compression of creating a spin-off document would lose critical context. Full transcript preserves the conversation's texture.

## Technical Details

### Source Data
- Location: `~/.claude/projects/[project-name]/`
- Format: JSONL files (one JSON object per line)
- Each line represents a chat message with role, content, metadata

### Processing
1. Detect current project name from cc-atlas working directory
2. Locate corresponding `~/.claude/projects/[project-name]/` directory
3. List available chat files
4. Parse selected JSONL file(s)
5. Strip unnecessary metadata
6. Format as clean markdown transcript
7. Export to specified output location

### Output Format

```markdown
# Chat Transcript: [chat-title]
**Date:** [timestamp]
**Models:** [list of models used in conversation]

---

## User

[user message content]

## Assistant (Sonnet 4.5)

[assistant response content]

## User

[next user message]

## Assistant (Opus 4)

[next assistant response]

---

**Exported by cc-atlas**
```

## Implementation Plan

### Backend (Rust)

**New service:** `src/services/chat_exporter.rs`

Functions:
- `find_project_chats(project_root: &Path) -> Result<Vec<ChatFile>>`
  - Derive project name from current directory
  - Locate `~/.claude/projects/[name]/`
  - List available chat JSONL files with metadata (timestamp, message count)

- `parse_chat_file(path: &Path) -> Result<Chat>`
  - Parse JSONL format
  - Extract relevant fields (role, content, model)
  - Filter system/tool messages (configurable)

- `export_to_markdown(chat: &Chat, output_path: &Path) -> Result<()>`
  - Format as markdown transcript
  - Include header with metadata
  - Clean formatting for readability

**New models:** `src/models.rs`
```rust
pub struct ChatFile {
    pub path: PathBuf,
    pub title: String,
    pub timestamp: DateTime,
    pub message_count: usize,
}

pub struct Chat {
    pub title: String,
    pub messages: Vec<Message>,
    pub models_used: Vec<String>,
}

pub struct Message {
    pub role: String,        // "user" | "assistant"
    pub content: String,
    pub model: Option<String>,
}
```

**New API endpoints:**
- `GET /api/chats` - List available chat files
- `GET /api/chats/:filename` - Preview chat metadata
- `POST /api/chats/:filename/export` - Export to markdown
  - Body: `{ "output_path": "...", "include_system": false }`

### Frontend (React)

**New section in UI:**
- Chat browser panel (similar to CLAUDE.md tree view)
- List of available chats with metadata
- Preview pane showing first few messages
- Export button with configuration options:
  - Output location selector
  - Toggle: include system prompts
  - Toggle: include tool calls
  - Toggle: include timestamps per message

### UX Flow

1. User opens cc-atlas for a project
2. Navigates to "Chats" tab
3. Sees list of conversation files with timestamps
4. Selects a chat to preview
5. Clicks "Export to Markdown"
6. Configures export options (output location, filtering)
7. Confirms export
8. File written to specified location
9. Success notification with file path

## Configuration Options

**Filtering:**
- Include/exclude system prompts
- Include/exclude tool use/results
- Include/exclude thinking blocks
- Date range filtering (for bulk export)

**Output:**
- Single file per chat (default)
- Merged file for multiple chats (optional)
- Output directory (user-specified or project default)

**Format:**
- Simple (role headers only)
- Detailed (timestamps, model names per message)
- Minimal (strip all metadata, just conversation)

## Edge Cases

1. **Project name detection fails** - Fallback to manual directory selection
2. **Chat file corrupted** - Skip malformed lines, report which lines failed
3. **Large chat files** - Stream processing for >1000 messages
4. **Multiple models in one chat** - Annotate each assistant message with model name
5. **Output file exists** - Prompt for overwrite/append/rename

## Future Enhancements

- Search within chats
- Diff two chats (compare Sonnet vs Opus responses)
- Export to other formats (JSON, TXT, HTML)
- Share directly to Claude.ai or ChatGPT web interface
- Tag/organize exported transcripts
- Automatic export on chat completion

## Success Criteria

- User can export any chat from current project in <5 clicks
- Exported markdown is readable without technical knowledge
- No data loss from original conversation content
- Export completes in <2s for typical chat (<100 messages)
