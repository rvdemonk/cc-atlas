use anyhow::Result;
use std::sync::Arc;

pub mod routes;
pub mod handlers;
pub mod error;

pub struct AppState {
    pub project_root: String,
}

pub async fn run(port: u16, project: String) -> Result<()> {
    let state = Arc::new(AppState {
        project_root: project.clone(),
    });

    let app = routes::create_router(state);
    let addr = format!("0.0.0.0:{}", port);

    println!("Server running at http://{}", addr);
    println!("Analyzing project: {}", project);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
