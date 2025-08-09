use anyhow::Result;
use axum::{
    extract::{Path as AxumPath, State},
    http::StatusCode,
    response::Json,
    routing::{get, put},
    Router,
};
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

use crate::analyzer;
use crate::models::{DirectoryInfo, MemoryFile};

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
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/memory-files", get(get_memory_files))
        .route("/api/tree", get(get_directory_tree))
        .route("/api/memory-files/*path", put(update_memory_file))
        .route("/api/recommendations", get(get_recommendations))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn get_memory_files(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<MemoryFile>>, StatusCode> {
    let root = Path::new(&state.project_root);
    
    match analyzer::find_memory_files(root) {
        Ok(files) => Ok(Json(files)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_directory_tree(
    State(state): State<Arc<AppState>>,
) -> Result<Json<DirectoryInfo>, StatusCode> {
    let root = Path::new(&state.project_root);
    
    match analyzer::build_directory_tree(root) {
        Ok(tree) => Ok(Json(tree)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_memory_file(
    State(state): State<Arc<AppState>>,
    AxumPath(path): AxumPath<String>,
    Json(body): Json<Value>,
) -> StatusCode {
    let content = match body.get("content").and_then(|v| v.as_str()) {
        Some(c) => c,
        None => return StatusCode::BAD_REQUEST,
    };
    
    let file_path = Path::new(&state.project_root).join(&path);
    
    match std::fs::write(&file_path, content) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn get_recommendations(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let root = Path::new(&state.project_root);
    
    match analyzer::build_directory_tree(root) {
        Ok(tree) => {
            let recommendations = get_recommendation_paths(&tree);
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