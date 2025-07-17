use axum::{Router, routing::get};
use axum::extract::Path;
use std::sync::Arc;
use crate::state::app_state::AppState;

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(root_handler))
        .route("/hello/:name", get(hello_handler))
}

async fn root_handler() -> &'static str {
    "Welcome to Rust Web Server!"
}

async fn hello_handler(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}
