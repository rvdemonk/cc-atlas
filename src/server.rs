use anyhow::Result;
use axum::{
    extract::{Path as AxumPath, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::analyzer;
use crate::markdown;
use crate::models::{DirectoryInfo, MemoryFile, MemoryFileResponse};

pub struct AppState {
    project_root: String,
}

pub async fn run(port: u16, project: String) -> Result<()> {
    let state = Arc::new(AppState {
        project_root: project.clone(),
    });

    let app = create_router(state);
    let addr = format!("0.0.0.0:{}", port);
    
    println!("Server running at http://{}", addr);
    println!("Analyzing project: {}", project);
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/memory-files", get(get_memory_files))
        .route("/api/tree", get(get_directory_tree))
        .route("/api/memory-files/*path", put(update_memory_file))
        .route("/api/memory-files/*path", post(create_memory_file))
        .route("/api/memory-files/*path", delete(delete_memory_file))
        .route("/api/recommendations", get(get_recommendations))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn get_memory_files(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<MemoryFileResponse>>, StatusCode> {
    let root = Path::new(&state.project_root);
    
    match analyzer::find_memory_files(root) {
        Ok(files) => {
            let responses = convert_to_responses(files, &state.project_root);
            Ok(Json(responses))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn convert_to_responses(files: Vec<MemoryFile>, project_root: &str) -> Vec<MemoryFileResponse> {
    files.into_iter().map(|file| {
        let html = markdown::markdown_to_html(&file.content);
        let parent_path = file.path
            .parent()
            .and_then(|p| p.strip_prefix(project_root).ok())
            .map(|p| format!("./{}", p.display()))
            .unwrap_or_else(|| ".".to_string());
        
        MemoryFileResponse {
            path: file.relative_path.clone(),
            content: file.content,
            content_html: html,
            exists: file.path.exists(),
            parent_path,
        }
    }).collect()
}

async fn get_directory_tree(
    State(state): State<Arc<AppState>>,
) -> Result<Json<DirectoryInfo>, StatusCode> {
    let root = Path::new(&state.project_root);
    
    match analyzer::build_directory_tree(root) {
        Ok(tree) => {
            // Convert paths to relative paths for frontend
            let converted_tree = convert_tree_paths(tree, &state.project_root);
            Ok(Json(converted_tree))
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn convert_tree_paths(mut tree: DirectoryInfo, project_root: &str) -> DirectoryInfo {
    let root_path = Path::new(project_root);
    
    // Convert the tree path to relative
    tree.path = tree.path
        .strip_prefix(root_path)
        .map(|p| {
            if p.as_os_str().is_empty() {
                PathBuf::from(".")
            } else {
                PathBuf::from(format!("./{}", p.display()))
            }
        })
        .unwrap_or_else(|_| PathBuf::from("."));
    
    // Recursively convert children
    tree.children = tree.children
        .into_iter()
        .map(|child| convert_tree_paths(child, project_root))
        .collect();
    
    tree
}

async fn update_memory_file(
    State(state): State<Arc<AppState>>,
    AxumPath(path): AxumPath<String>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // Check if we're receiving HTML or markdown
    let content_html = body.get("content_html").and_then(|v| v.as_str());
    let content_md = body.get("content").and_then(|v| v.as_str());
    
    let final_content = if let Some(html) = content_html {
        // Convert HTML to markdown for saving
        match markdown::html_to_markdown(html) {
            Ok(md) => md,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else if let Some(md) = content_md {
        // Use markdown directly (for source mode saves)
        md.to_string()
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };
    
    let file_path = Path::new(&state.project_root).join(&path);
    
    match std::fs::write(&file_path, &final_content) {
        Ok(_) => Ok(Json(serde_json::json!({
            "content": final_content
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_memory_file(
    State(state): State<Arc<AppState>>,
    AxumPath(path): AxumPath<String>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    // Check if we're receiving HTML or markdown
    let content_html = body.get("content_html").and_then(|v| v.as_str());
    let content_md = body.get("content").and_then(|v| v.as_str());
    
    let final_content = if let Some(html) = content_html {
        // Convert HTML to markdown for saving
        match markdown::html_to_markdown(html) {
            Ok(md) => md,
            Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else if let Some(md) = content_md {
        // Use markdown directly
        md.to_string()
    } else {
        return Err(StatusCode::BAD_REQUEST);
    };
    
    let file_path = Path::new(&state.project_root).join(&path);
    
    // Create parent directories if they don't exist
    if let Some(parent) = file_path.parent() {
        if let Err(_) = std::fs::create_dir_all(parent) {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
    
    // Check if file already exists
    if file_path.exists() {
        return Err(StatusCode::CONFLICT);
    }
    
    match std::fs::write(&file_path, &final_content) {
        Ok(_) => Ok(Json(serde_json::json!({
            "path": path,
            "content": final_content,
            "created": true
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn delete_memory_file(
    State(state): State<Arc<AppState>>,
    AxumPath(path): AxumPath<String>,
) -> Result<Json<Value>, StatusCode> {
    let file_path = Path::new(&state.project_root).join(&path);
    
    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }
    
    match std::fs::remove_file(&file_path) {
        Ok(_) => Ok(Json(serde_json::json!({
            "path": path,
            "deleted": true
        }))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_recommendations(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let root = Path::new(&state.project_root);
    
    match analyzer::build_directory_tree(root) {
        Ok(tree) => {
            // Convert paths before getting recommendations
            let converted_tree = convert_tree_paths(tree, &state.project_root);
            let recommendations = get_recommendation_paths(&converted_tree);
            Ok(Json(recommendations))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

fn get_recommendation_paths(tree: &DirectoryInfo) -> Vec<String> {
    let mut paths = Vec::new();
    collect_recommendations(tree, &mut paths);
    paths
}

fn collect_recommendations(dir: &DirectoryInfo, paths: &mut Vec<String>) {
    if should_recommend(dir) {
        paths.push(dir.path.to_string_lossy().to_string());
    }
    
    for child in &dir.children {
        collect_recommendations(child, paths);
    }
}

fn should_recommend(dir: &DirectoryInfo) -> bool {
    const FILE_THRESHOLD: usize = 10;
    const LINE_THRESHOLD: usize = 500;
    
    !dir.has_memory && 
    (dir.stats.file_count > FILE_THRESHOLD || dir.stats.total_lines > LINE_THRESHOLD)
}