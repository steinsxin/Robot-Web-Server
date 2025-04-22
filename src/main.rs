mod schema;
mod models;
mod state;
mod controller;

use axum::{
    Router,
    routing::{get, post},
    response::{Html, Json},
    extract::{Path,State},
    http::StatusCode
};

use std::net::SocketAddr;

use crate::models::config::{Config};
use crate::state::AppState;
use crate::controller::robot_controller::{Robot, NewRobot, UpdateRobot};

// 数据库
use diesel::{QueryDsl, ExpressionMethods};
use diesel_async::{AsyncPgConnection, RunQueryDsl, pooled_connection::AsyncDieselConnectionManager};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = Config::from_file("config.yaml")?;
    println!("{}", config.database.url);

    let config_manager =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(config.database.url.clone());
    let pool = bb8::Pool::builder()
        .max_size(20)
        .min_idle(Some(5))
        .connection_timeout(std::time::Duration::from_secs(10))
        .idle_timeout(Some(std::time::Duration::from_secs(300)))
        .max_lifetime(Some(std::time::Duration::from_secs(1800)))
        .build(config_manager)
        .await
        .unwrap();

    // 异步任务
    start_tasks();

    let app_state = AppState {
        pool,
        config,
    };

    // 构建路由
    let app = Router::new()
        // 根路由
        .route("/", get(root_handler))

        // 基础路由
        .route("/hello/:name", get(hello_handler))

        // 机器人相关路由
        .route("/create/Robot", post(create_robot))
        .route("/query/Robot", get(get_robot_info))

        // 路由资源
        .with_state(app_state)

        // 嵌套路由
        .nest("/api", api_routes());


    // 启动服务器
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    ).await.unwrap();

    Ok(())
}

// API子路由
fn api_routes() -> Router {
    Router::new()
        .route("/status", get(|| async { "API is healthy" }))
}

// 处理器函数
async fn root_handler() -> Html<&'static str> {
    Html("<h1>Welcome to Rust Web Server!</h1>")
}

async fn hello_handler(Path(name): Path<String>) -> String {
    format!("Hello, {}!", name)
}

async fn create_robot(
    State(state): State<AppState>,
    Json(robot_t): Json<NewRobot>
) -> Result<StatusCode, (StatusCode, String)>{
    println!("Robot_id:{}, Device_id: {}, electricity:{} ,activate:{} ", &robot_t.robot_id, &robot_t.device_id, &robot_t.electricity, &robot_t.activate );

    let mut conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(e) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get database connection: {}", e)
        )),
    };

    // 连接到数据库
    let new_robot = NewRobot {
        robot_id: robot_t.robot_id,
        device_id: robot_t.device_id,
        electricity: robot_t.electricity,
        activate: robot_t.activate,
    };

    use crate::schema::robot_manager::dsl::*;

    diesel::insert_into(robot_manager)
        .values(&new_robot)
        .execute(&mut conn)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e)
        ))?;

    Ok(StatusCode::CREATED)
}

fn start_tasks(){
    use tokio::time;
    tokio::spawn(async move {
        let mut interval = time::interval(std::time::Duration::from_secs(3));
        loop {
            interval.tick().await;
            let now = std::time::Instant::now();
            println!("Hello NowTime:{:?}", now);
        }
    });
}


async fn get_robot_info(
    State(state): State<AppState>
) -> Result<Json<Vec<Robot>>, (StatusCode, String)> {

    let mut conn = match state.pool.get().await {
        Ok(conn) => conn,
        Err(e) => return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to get database connection: {}", e)
        )),
    };
    // 查询机器人数据
    use crate::schema::robot_manager::dsl::*;
    let robots = robot_manager
        .filter(activate.eq(true))
        .load::<Robot>(&mut conn)
        .await
        .expect("Error loading posts");

    // 返回数据
    Ok(Json(robots))
}