mod tcp_server;
use tcp_server::TcpServer;

mod db;
use std::sync::Arc;
use tokio::sync::Mutex;

use axum::{
    routing::get,
    Router,
    response::Html, 
    extract::Path,
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

pub type ActiveIps = Arc<Mutex<HashMap<IpAddr, std::time::Instant>>>;

// 定义数据结构
#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct RobotData {
    robot_id: String,
    electricity: String,
    activate: String,
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

    // 启动 TCP Server（例如端口4000，类型为 Data）
    let tcp_server = TcpServer::new(1034, conn_arc.clone(), active_ips.clone());
    tcp_server.start();

    // 启动 Axum HTTP Server
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/hello/:name", get(hello_handler))
        .route("/user", get(user_handler))
        .route("/robot_manage", axum::routing::post(robot_manage_handler))
        .nest("/api", api_routes());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

// 机器人调度
async fn robot_manage_handler(Json(payload): Json<RobotData>) -> String {
    println!("🤖 Received robot manage POST: {:?}", payload);
    format!("Received robot_id: {}, electricity: {}, activate: {}", payload.robot_id, payload.electricity, payload.activate)
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

