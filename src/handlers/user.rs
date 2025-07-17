use axum::{Router, routing::get, Json};
use serde::Serialize;
use std::sync::Arc;
use crate::state::app_state::AppState;

#[derive(Serialize)]
pub struct User {
    id: u64,
    name: String,
    email: String,
}

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/users", get(get_users))
}

pub async fn get_users() -> Json<Vec<User>> {
    Json(vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
    ])
}