use axum::{
    extract::{Path as AxumPath, State},
    response::Json,
};
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;

use crate::services::{analyzer, chat_exporter};
use crate::utils::{markdown, paths};
use crate::models::{ChatMetadata, DirectoryInfo, ExportOptions, ExportResult, MemoryFileResponse};

use super::{AppState, error::ServerError};

pub async fn get_memory_files(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<MemoryFileResponse>>, ServerError> {
    let root = Path::new(&state.project_root);

    let files = analyzer::find_memory_files(root)
        .map_err(|e| ServerError::Internal(format!("Failed to find memory files: {}", e)))?;

    let responses = paths::convert_to_responses(files, &state.project_root);
    Ok(Json(responses))
}

pub async fn get_directory_tree(
    State(state): State<Arc<AppState>>,
) -> Result<Json<DirectoryInfo>, ServerError> {
    let root = Path::new(&state.project_root);

    let tree = analyzer::build_directory_tree(root)
        .map_err(|e| ServerError::Internal(format!("Failed to build directory tree: {}", e)))?;

    // Convert paths to relative paths for frontend
    let converted_tree = paths::convert_tree_paths(tree, &state.project_root);
    Ok(Json(converted_tree))
}

pub async fn update_memory_file(
    State(state): State<Arc<AppState>>,
    AxumPath(path): AxumPath<String>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, ServerError> {
    // Check if we're receiving HTML or markdown
    let content_html = body.get("content_html").and_then(|v| v.as_str());
    let content_md = body.get("content").and_then(|v| v.as_str());

    let final_content = if let Some(html) = content_html {
        // Convert HTML to markdown for saving
        markdown::html_to_markdown(html)
            .map_err(|e| ServerError::Internal(format!("Failed to convert HTML to markdown: {}", e)))?
    } else if let Some(md) = content_md {
        // Use markdown directly (for source mode saves)
        md.to_string()
    } else {
        return Err(ServerError::BadRequest("Missing content or content_html".to_string()));
    };

    let file_path = Path::new(&state.project_root).join(&path);

    std::fs::write(&file_path, &final_content)
        .map_err(|e| ServerError::Internal(format!("Failed to write file: {}", e)))?;

    Ok(Json(serde_json::json!({
        "content": final_content
    })))
}

pub async fn create_memory_file(
    State(state): State<Arc<AppState>>,
    AxumPath(path): AxumPath<String>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, ServerError> {
    // Check if we're receiving HTML or markdown
    let content_html = body.get("content_html").and_then(|v| v.as_str());
    let content_md = body.get("content").and_then(|v| v.as_str());

    let final_content = if let Some(html) = content_html {
        // Convert HTML to markdown for saving
        markdown::html_to_markdown(html)
            .map_err(|e| ServerError::Internal(format!("Failed to convert HTML to markdown: {}", e)))?
    } else if let Some(md) = content_md {
        // Use markdown directly
        md.to_string()
    } else {
        return Err(ServerError::BadRequest("Missing content or content_html".to_string()));
    };

    let file_path = Path::new(&state.project_root).join(&path);

    // Create parent directories if they don't exist
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| ServerError::Internal(format!("Failed to create parent directories: {}", e)))?;
    }

    // Check if file already exists
    if file_path.exists() {
        return Err(ServerError::Conflict(format!("File already exists: {}", path)));
    }

    std::fs::write(&file_path, &final_content)
        .map_err(|e| ServerError::Internal(format!("Failed to create file: {}", e)))?;

    Ok(Json(serde_json::json!({
        "path": path,
        "content": final_content,
        "created": true
    })))
}

pub async fn delete_memory_file(
    State(state): State<Arc<AppState>>,
    AxumPath(path): AxumPath<String>,
) -> Result<Json<Value>, ServerError> {
    let file_path = Path::new(&state.project_root).join(&path);

    if !file_path.exists() {
        return Err(ServerError::NotFound(format!("File not found: {}", path)));
    }

    std::fs::remove_file(&file_path)
        .map_err(|e| ServerError::Internal(format!("Failed to delete file: {}", e)))?;

    Ok(Json(serde_json::json!({
        "path": path,
        "deleted": true
    })))
}

pub async fn get_recommendations(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, ServerError> {
    let root = Path::new(&state.project_root);

    let tree = analyzer::build_directory_tree(root)
        .map_err(|e| ServerError::Internal(format!("Failed to build directory tree: {}", e)))?;

    // Get recommendations from analyzer
    let recommendations = analyzer::get_recommendations(&tree);

    // Convert PathBuf to relative String paths for frontend
    let relative_paths = paths::to_relative_paths(recommendations, &state.project_root);

    Ok(Json(relative_paths))
}

pub async fn get_chats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<ChatMetadata>>, ServerError> {
    let root = Path::new(&state.project_root);

    let chats = chat_exporter::find_project_chats(root)
        .map_err(|e| ServerError::Internal(format!("Failed to find chats: {}", e)))?;

    Ok(Json(chats))
}

pub async fn export_chat(
    State(state): State<Arc<AppState>>,
    AxumPath(session_id): AxumPath<String>,
    Json(body): Json<Value>,
) -> Result<Json<ExportResult>, ServerError> {
    let root = Path::new(&state.project_root);

    // Parse options from request body (use defaults if not provided)
    let options = if let Some(opts) = body.as_object() {
        ExportOptions {
            include_tools: opts.get("include_tools")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            include_timestamps: opts.get("include_timestamps")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            include_thinking: opts.get("include_thinking")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            max_tool_files: opts.get("max_tool_files")
                .and_then(|v| v.as_u64())
                .map(|v| v as usize)
                .unwrap_or(5),
        }
    } else {
        ExportOptions::default()
    };

    let result = chat_exporter::export_chat(&session_id, root, &options, None)
        .map_err(|e| ServerError::Internal(format!("Failed to export chat: {}", e)))?;

    Ok(Json(result))
}
