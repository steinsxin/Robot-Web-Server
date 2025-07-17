use axum::{Router};
use std::sync::Arc;
use crate::state::app_state::AppState;

mod robot;
mod user;
mod root;

pub fn routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/", root::routes())
        .nest("/api", user::routes())
        .nest("/robot", robot::routes())
        .with_state(app_state)
}
