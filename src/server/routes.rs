use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use super::AppState;
use super::handlers;

pub fn create_router(state: Arc<AppState>) -> Router {
    let api_routes = Router::new()
        .route("/memory-files", get(handlers::get_memory_files))
        .route("/tree", get(handlers::get_directory_tree))
        .route("/memory-files/*path", put(handlers::update_memory_file))
        .route("/memory-files/*path", post(handlers::create_memory_file))
        .route("/memory-files/*path", delete(handlers::delete_memory_file))
        .route("/recommendations", get(handlers::get_recommendations))
        .route("/chats", get(handlers::get_chats))
        .route("/chats/:session_id/export", post(handlers::export_chat))
        .route("/docs/tree", get(handlers::get_docs_tree))
        .route("/docs/files/*path", get(handlers::get_doc_file))
        .route("/docs/files/*path", put(handlers::update_doc_file))
        .with_state(state);

    let frontend_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("frontend")
        .join("dist");

    Router::new()
        .nest("/api", api_routes)
        .fallback_service(ServeDir::new(frontend_dir).append_index_html_on_directories(true))
        .layer(CorsLayer::permissive())
}
