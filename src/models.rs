use serde::{Deserialize, Serialize};
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