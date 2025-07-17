mod handlers;
mod models;
mod services;
mod state;
mod schema;

use std::sync::Arc;
use std::net::SocketAddr;
use crate::state::app_state::AppState;
use crate::services::tcp_server::TcpServer;
use axum::{Router, response::Html,routing::get};
use tokio::sync::Mutex;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_pool = services::db::create_db_pool("postgres://postgres:1@localhost/postgres");
    let state = Arc::new(AppState::new(db_pool));

    // TCP 服务异步运行
    let tcp_server = TcpServer::new(1034, state.clone());
    tokio::spawn(async move {
        if let Err(e) = tcp_server.run().await {
            eprintln!("TCP server failed: {}", e);
        }
    });

    // HTTP 路由注册并注入共享状态
    let app = handlers::routes(state.clone());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("HTTP server running on http://{}", addr);

    axum::serve(TcpListener::bind(addr).await?, app).await?;
    Ok(())
}
