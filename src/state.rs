use crate::models::config::Config;
use axum::extract::FromRef; // Add this import for the FromRef trait
use bb8::Pool;
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    pub config: Config,
}

impl FromRef<AppState> for Pool<AsyncDieselConnectionManager<AsyncPgConnection>> {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}