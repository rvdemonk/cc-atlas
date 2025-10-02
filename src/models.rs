use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFile {
    pub path: PathBuf,
    pub content: String,
    pub content_html: Option<String>,  // Cached HTML version
    pub relative_path: String,
    pub stats: FileStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFileResponse {
    pub path: String,
    pub content: String,
    pub content_html: String,
    pub exists: bool,
    pub parent_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStats {
    pub file_count: usize,
    pub total_lines: usize,
    pub depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryInfo {
    pub path: PathBuf,
    pub name: String,
    pub has_memory: bool,
    pub children: Vec<DirectoryInfo>,
    pub stats: FileStats,
}

// ===== Chat Export Models =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMetadata {
    pub session_id: String,
    pub file_path: PathBuf,
    pub title: String,
    pub message_count: usize,
    pub last_modified: String,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportOptions {
    pub include_tools: bool,
    pub include_timestamps: bool,
    pub include_thinking: bool,
    pub max_tool_files: usize,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            include_tools: true,
            include_timestamps: false,
            include_thinking: false,
            max_tool_files: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub output_path: PathBuf,
    pub message_count: usize,
    pub export_size: u64,
    pub title: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatEntry {
    #[serde(rename = "type")]
    pub entry_type: String,
    pub message: Option<MessageContent>,
    #[serde(rename = "isMeta")]
    pub is_meta: Option<bool>,
    pub timestamp: Option<String>,
    #[allow(dead_code)]
    pub uuid: Option<String>,
    pub cwd: Option<String>,
    #[serde(rename = "gitBranch")]
    pub git_branch: Option<String>,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageContent {
    pub role: String,
    pub content: Value,  // Can be string or array
    pub model: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
    pub model: Option<String>,
    pub timestamp: String,
    pub tools_used: Vec<ToolCallSummary>,
}

#[derive(Debug, Clone)]
pub struct ToolCallSummary {
    pub tool_name: String,
    pub files: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SessionMetadata {
    pub session_id: String,
    pub cwd: String,
    pub git_branch: Option<String>,
    pub message_count: usize,
    pub models_used: Vec<String>,
    pub date_range: (String, String),
}