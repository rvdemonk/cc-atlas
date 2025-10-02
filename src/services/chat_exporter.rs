use anyhow::{Result, Context};
use std::fs::{File, self};
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
use serde_json::Value;

use crate::models::{
    ChatMetadata, ExportOptions, ExportResult, ChatEntry, ChatMessage,
    ToolCallSummary, SessionMetadata
};

/// Find all chat files for the current project
pub fn find_project_chats(project_root: &Path) -> Result<Vec<ChatMetadata>> {
    let home = std::env::var("HOME").context("Failed to get HOME directory")?;
    let claude_projects = PathBuf::from(home).join(".claude/projects");

    // Canonicalize the project path to get absolute path
    let absolute_project = project_root.canonicalize()
        .context("Failed to resolve project path")?;

    // Convert path to string, strip leading /, then replace remaining / with -
    let path_str = absolute_project.display().to_string();
    let path_str = path_str.strip_prefix('/').unwrap_or(&path_str);
    let project_dir = claude_projects.join(format!("-{}", path_str.replace("/", "-")));

    if !project_dir.exists() {
        return Ok(Vec::new());
    }

    let mut chats = Vec::new();

    for entry in fs::read_dir(&project_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("jsonl") {
            if let Ok(metadata) = get_chat_metadata(&path) {
                chats.push(metadata);
            }
        }
    }

    // Sort by last modified, newest first
    chats.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));

    Ok(chats)
}

/// Get metadata for a single chat file without parsing all messages
pub fn get_chat_metadata(path: &Path) -> Result<ChatMetadata> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut line_count = 0;
    let mut first_timestamp = None;
    let mut last_timestamp = None;
    let mut session_id = None;
    let mut cwd = None;

    // Scan file line by line for metadata
    for line in reader.lines() {
        let line = line?;
        line_count += 1;

        if let Ok(entry) = serde_json::from_str::<ChatEntry>(&line) {
            // Capture first/last timestamps
            if let Some(ts) = &entry.timestamp {
                if first_timestamp.is_none() {
                    first_timestamp = Some(ts.clone());
                }
                last_timestamp = Some(ts.clone());
            }

            // Capture session ID and cwd
            if session_id.is_none() {
                session_id = entry.session_id.clone();
            }
            if cwd.is_none() {
                cwd = entry.cwd.clone();
            }
        }
    }

    let metadata = fs::metadata(path)?;
    let session_id = session_id.unwrap_or_else(|| {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    });

    let project_name = cwd
        .as_ref()
        .and_then(|p| Path::new(p).file_name())
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    let date = last_timestamp
        .as_ref()
        .and_then(|ts| ts.split('T').next())
        .unwrap_or("unknown");

    let title = format!("{} · {} messages · {}", date, line_count, project_name);

    Ok(ChatMetadata {
        session_id,
        file_path: path.to_path_buf(),
        title,
        message_count: line_count,
        last_modified: last_timestamp.unwrap_or_default(),
        file_size: metadata.len(),
    })
}

/// Export a chat to markdown
pub fn export_chat(
    session_id: &str,
    project_root: &Path,
    options: &ExportOptions,
    custom_name: Option<&str>
) -> Result<ExportResult> {
    // Find the chat file
    let chats = find_project_chats(project_root)?;
    let chat = chats.iter()
        .find(|c| c.session_id == session_id)
        .context("Chat not found")?;

    // Stream and parse messages
    let (messages, metadata) = stream_and_parse(&chat.file_path)?;

    // Format as markdown
    let markdown = format_as_markdown(&messages, &metadata, options);

    // Generate export path
    let export_path = generate_export_path(session_id, custom_name);

    // Ensure parent directory exists
    if let Some(parent) = export_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write file
    fs::write(&export_path, &markdown)?;

    let file_size = fs::metadata(&export_path)?.len();

    Ok(ExportResult {
        output_path: export_path,
        message_count: messages.len(),
        export_size: file_size,
        title: metadata.session_id.clone(),
    })
}

/// Stream through JSONL and extract messages
fn stream_and_parse(path: &Path) -> Result<(Vec<ChatMessage>, SessionMetadata)> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut messages = Vec::new();
    let mut session_id = String::new();
    let mut cwd = String::new();
    let mut git_branch = None;
    let mut models_used = Vec::new();
    let mut first_ts = None;
    let mut last_ts = None;

    for line in reader.lines() {
        let line = line?;

        let entry: ChatEntry = match serde_json::from_str(&line) {
            Ok(e) => e,
            Err(_) => continue, // Skip malformed lines
        };

        // Skip non-message entries
        if entry.entry_type != "user" && entry.entry_type != "assistant" {
            continue;
        }

        // Skip meta messages
        if entry.is_meta == Some(true) {
            continue;
        }

        // Extract metadata
        if session_id.is_empty() {
            if let Some(sid) = &entry.session_id {
                session_id = sid.clone();
            }
        }
        if cwd.is_empty() {
            if let Some(c) = &entry.cwd {
                cwd = c.clone();
            }
        }
        if git_branch.is_none() {
            git_branch = entry.git_branch.clone();
        }

        // Track timestamps
        if let Some(ts) = &entry.timestamp {
            if first_ts.is_none() {
                first_ts = Some(ts.clone());
            }
            last_ts = Some(ts.clone());
        }

        // Parse message
        if let Some(msg_content) = entry.message {
            let content_str = extract_content_text(&msg_content.content);

            // Extract tools from assistant messages
            let tools_used = if entry.entry_type == "assistant" {
                extract_tool_summary(&msg_content.content)
            } else {
                Vec::new()
            };

            // Track models used
            if let Some(model) = &msg_content.model {
                if !models_used.contains(model) {
                    models_used.push(model.clone());
                }
            }

            // Only add messages with text content (skip tool-only messages)
            if !content_str.is_empty() {
                // Indent markdown headings by one level for proper nesting
                let indented_content = indent_markdown_headings(&content_str);

                messages.push(ChatMessage {
                    role: msg_content.role,
                    content: indented_content,
                    model: msg_content.model,
                    timestamp: entry.timestamp.unwrap_or_default(),
                    tools_used,
                });
            }
        }
    }

    let metadata = SessionMetadata {
        session_id,
        cwd,
        git_branch,
        message_count: messages.len(),
        models_used,
        date_range: (
            first_ts.unwrap_or_default(),
            last_ts.unwrap_or_default()
        ),
    };

    Ok((messages, metadata))
}

/// Indent markdown headings by one level (## -> ###, # -> ##, etc.)
fn indent_markdown_headings(text: &str) -> String {
    let mut result = String::new();

    for line in text.lines() {
        if line.starts_with('#') {
            // Find where the heading text starts (after the # characters and space)
            let hash_count = line.chars().take_while(|c| *c == '#').count();
            let rest = &line[hash_count..];

            // Add one more # to indent the heading
            result.push_str(&"#".repeat(hash_count + 1));
            result.push_str(rest);
        } else {
            result.push_str(line);
        }
        result.push('\n');
    }

    // Remove trailing newline that was added in the loop
    if result.ends_with('\n') {
        result.pop();
    }

    result
}

/// Extract text content from message content (can be string or array)
fn extract_content_text(content: &Value) -> String {
    match content {
        Value::String(s) => s.clone(),
        Value::Array(arr) => {
            let mut result = String::new();
            for item in arr {
                if let Some(obj) = item.as_object() {
                    if obj.get("type").and_then(|v| v.as_str()) == Some("text") {
                        if let Some(text) = obj.get("text").and_then(|v| v.as_str()) {
                            if !result.is_empty() {
                                result.push_str("\n\n");
                            }
                            result.push_str(text);
                        }
                    }
                }
            }
            result
        }
        _ => String::new(),
    }
}

/// Extract tool call summaries from message content
fn extract_tool_summary(content: &Value) -> Vec<ToolCallSummary> {
    let mut summaries = Vec::new();

    if let Value::Array(arr) = content {
        for item in arr {
            if let Some(obj) = item.as_object() {
                // Only process tool_use, not tool_result (which contains bloat)
                if obj.get("type").and_then(|v| v.as_str()) == Some("tool_use") {
                    let tool_name = obj.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let mut files = Vec::new();

                    // Extract file paths from input
                    if let Some(input) = obj.get("input").and_then(|v| v.as_object()) {
                        if let Some(file_path) = input.get("file_path").and_then(|v| v.as_str()) {
                            files.push(file_path.to_string());
                        }
                        if let Some(path) = input.get("path").and_then(|v| v.as_str()) {
                            files.push(path.to_string());
                        }
                    }

                    summaries.push(ToolCallSummary { tool_name, files });
                }
            }
        }
    }

    summaries
}

/// Format messages as markdown
fn format_as_markdown(
    messages: &[ChatMessage],
    metadata: &SessionMetadata,
    options: &ExportOptions
) -> String {
    let mut md = String::new();

    // Header
    md.push_str(&format_header(metadata));
    md.push_str("\n\n---\n\n");

    // Messages
    for msg in messages {
        md.push_str(&format_message_block(msg, options));
        md.push_str("\n\n");
    }

    // Footer
    md.push_str("---\n\n");
    md.push_str("*Exported by cc-atlas*\n");

    md
}

/// Format header with metadata
fn format_header(metadata: &SessionMetadata) -> String {
    let mut header = String::new();

    header.push_str("# Chat Transcript\n\n");
    header.push_str(&format!("**Session:** {}\n", metadata.session_id));
    header.push_str(&format!("**Project:** {}\n", metadata.cwd));

    if let Some(branch) = &metadata.git_branch {
        header.push_str(&format!("**Branch:** {}\n", branch));
    }

    let date = metadata.date_range.0.split('T').next().unwrap_or("unknown");
    header.push_str(&format!("**Date:** {}\n", date));
    header.push_str(&format!("**Messages:** {}\n", metadata.message_count));

    if !metadata.models_used.is_empty() {
        let models = metadata.models_used.join(", ");
        header.push_str(&format!("**Models:** {}\n", models));
    }

    header
}

/// Format a single message block
fn format_message_block(msg: &ChatMessage, options: &ExportOptions) -> String {
    let mut block = String::new();

    // Header
    let role_display = if msg.role == "user" {
        "User".to_string()
    } else if let Some(model) = &msg.model {
        format!("Assistant ({})", model)
    } else {
        "Assistant".to_string()
    };

    block.push_str(&format!("## {}\n", role_display));

    // Timestamp
    if options.include_timestamps {
        let ts = msg.timestamp.split('T')
            .nth(1)
            .and_then(|t| t.split('.').next())
            .unwrap_or(&msg.timestamp);
        block.push_str(&format!("*{}*\n", ts));
    }

    // Tools used
    if options.include_tools && !msg.tools_used.is_empty() {
        let tools_str = format_tools(&msg.tools_used, options.max_tool_files);
        block.push_str(&format!("*Tools: {}*\n", tools_str));
    }

    block.push_str("\n");
    block.push_str(&msg.content);

    block
}

/// Format tools as compact string
fn format_tools(tools: &[ToolCallSummary], max_files: usize) -> String {
    let mut result = Vec::new();

    for tool in tools {
        if tool.files.is_empty() {
            result.push(tool.tool_name.clone());
        } else {
            let files: Vec<_> = tool.files.iter()
                .take(max_files)
                .map(|f| {
                    // Extract just filename from path
                    Path::new(f)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or(f)
                })
                .collect();

            let files_str = files.join(", ");
            result.push(format!("{}({})", tool.tool_name, files_str));
        }
    }

    result.join(", ")
}

/// Generate export path
fn generate_export_path(session_id: &str, custom_name: Option<&str>) -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let base_dir = PathBuf::from(home).join("Desktop/cc-atlas-exports");

    // Use custom name if provided, otherwise session_id
    let base_name = custom_name.unwrap_or(session_id);

    // Ensure .md extension
    let filename = if base_name.ends_with(".md") {
        base_name.to_string()
    } else {
        format!("{}.md", base_name)
    };

    let mut path = base_dir.join(&filename);

    // Handle conflicts by appending counter
    if path.exists() {
        let mut counter = 1;
        let name_without_ext = filename.trim_end_matches(".md");
        loop {
            path = base_dir.join(format!("{}_{}.md", name_without_ext, counter));
            if !path.exists() {
                break;
            }
            counter += 1;
        }
    }

    path
}
