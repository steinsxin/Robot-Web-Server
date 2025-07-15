mod tcp_server;
use tcp_server::{TcpServer, ActiveIps, RobotIpMap, RobotConnMap};

mod db;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::AsyncWriteExt;
use axum::{
    routing::get,
    Router,
    response::Html, 
    extract::{Path, State},
    Json, 
};

use serde::Serialize;
use std::net::SocketAddr;
// db Test
mod schema;
mod models;

use models::Post;
use schema::posts::dsl::*;
use diesel::prelude::*;
use diesel::PgConnection;
use std::collections::HashMap;
use std::net::IpAddr;
use serde::Deserialize;

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://postgres:1@localhost/postgres";
    PgConnection::establish(database_url).expect("Error connecting to database")
}

pub fn get_posts(conn: &mut PgConnection) -> Vec<Post> {
    posts
        .filter(published.eq(true))
        .load::<Post>(conn)
        .expect("Error loading posts")
}

// 定义数据结构
#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Clone)]
struct AppState {
    robot_ip_map: RobotIpMap,
    robot_conn_map: RobotConnMap,
}

#[derive(Debug, Deserialize)]
struct RobotData {
    robot_id: String,
    electricity: String,
    activate: String,
}

#[derive(Debug, Deserialize)]
struct RobotIdentify {
    robot_id: String,
}

#[derive(Debug, Deserialize)]
struct RobotIpRequest {
    robot_id: String,
}

#[tokio::main]
async fn main() {
    // db Test
    let mut connection = establish_connection();

    let db_posts = get_posts(&mut connection);
    for date in db_posts {
        println!("{}: {}", date.title, date.context);
    }
    
    let conn_arc = Arc::new(Mutex::new(connection));
    
    // 初始化活跃IP记录表
    let active_ips: ActiveIps = Arc::new(Mutex::new(HashMap::new()));
    let robot_ip_map: RobotIpMap = Arc::new(Mutex::new(HashMap::new()));
    let robot_conn_map: RobotConnMap = Arc::new(Mutex::new(HashMap::new()));
    
    let state = AppState {
        robot_ip_map: robot_ip_map.clone(),
        robot_conn_map: robot_conn_map.clone(),
    };

    // 启动 TCP Server（例如端口4000，类型为 Data）
    let tcp_server = TcpServer::new(1034, conn_arc.clone(), active_ips.clone(), robot_ip_map.clone(), robot_conn_map.clone());
    tcp_server.start();

    // 启动 Axum HTTP Server
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/hello/:name", get(hello_handler))
        .route("/user", get(user_handler))
        .route("/robot_manage", axum::routing::post(robot_manage_handler))
        .route("/get_robot_ip", axum::routing::post(get_robot_ip))
        .with_state(Arc::new(state))
        .nest("/api", api_routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

// 机器人调度
async fn robot_manage_handler(
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
        return format!("✅ Data sent to robot {}", payload.robot_id);
    }else {
        return format!("❌ robot_id {} not connected", payload.robot_id);
    }
}

async fn get_robot_ip(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RobotIpRequest>,
) -> String {
    let map = state.robot_ip_map.lock().await;
    if let Some(ip) = map.get(&payload.robot_id) {
        ip.to_string()
    } else {
        "Not found".to_string()
    }
}
// API子路由
fn api_routes() -> Router {
    Router::new()
        .route("/users", get(get_users))
        .route("/status", get(|| async { "API is healthy" }))
}

// 处理器函数
async fn root_handler() -> Html<&'static str> {
    Html("<h1>Welcome to Rust Web Server!</h1>")
}

async fn hello_handler(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

async fn user_handler() -> Json<User> {
    Json(User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    })
}

async fn get_users() -> Json<Vec<User>> {
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

