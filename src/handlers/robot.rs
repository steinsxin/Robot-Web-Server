use axum::{Router, routing::post};
use axum::extract::State;
use axum::Json; 
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use crate::state::app_state::AppState;
use crate::models::robot::{RobotIdentify, RobotIpRequest};

pub fn routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/manage", post(robot_manage_handler))
        .route("/ip", post(get_robot_ip))
}

pub async fn robot_manage_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RobotIdentify>,
) -> String {
    println!("🤖 Received robot_id: {:?}", payload.robot_id);

    let mut map = state.robot_conn_map.lock().await;
    if let Some(conn) = map.get(&payload.robot_id) {
        let mut stream = conn.lock().await;
        let message = b"hello robot";
        if let Err(e) = stream.write_all(message).await {
            return format!("❌ Failed to send data: {}", e);
        }
        format!("✅ Data sent to robot {}", payload.robot_id)
    } else {
        format!("❌ robot_id {} not connected", payload.robot_id)
    }
}

pub async fn get_robot_ip(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RobotIpRequest>,
) -> String {
    let map = state.robot_ip_map.lock().await;
    map.get(&payload.robot_id)
        .map(|ip| ip.to_string())
        .unwrap_or_else(|| "Not found".to_string())
}