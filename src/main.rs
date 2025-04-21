mod schema;
mod models;
mod robot_controller;

use axum::{
    routing::{get, post},
    Router,
    response::{Html, Json},
    extract::Path,
    http::StatusCode
};
use serde::Serialize;
use std::net::SocketAddr;
use models::config::{Config};

use robot_controller::{Robot, NewRobot, UpdateRobot};
use diesel::{PgConnection,prelude::*};
use diesel::{insert_into,update};

pub fn establish_connection() -> PgConnection {
    let database_url = "postgres://postgres:1@localhost/postgres";
    PgConnection::establish(database_url).expect("Error connecting to database")
}

// 获取数据
pub fn get_robot(conn: &mut PgConnection) -> Vec<Robot> {
    use crate::schema::robot_manager::dsl::*;
    robot_manager
        .filter(activate.eq(true))
        .load::<Robot>(conn)
        .expect("Error loading posts")
}

// 添加数据
pub fn insert_robot(conn: &mut PgConnection, new_robot: NewRobot) {
    use crate::schema::robot_manager::dsl::*;
    insert_into(robot_manager)
        .values(&new_robot)
        .execute(conn)
        .expect("Error inserting new robot");
}

// 更新数据
pub fn update_robot(conn: &mut PgConnection, find_robot_id: &str, update_robot: UpdateRobot) {
    use crate::schema::robot_manager::dsl::*;
    update(robot_manager.filter(robot_id.eq(find_robot_id)))
        .set(&update_robot)
        .execute(conn)
        .expect("Error updating robot");
}

// 定义数据结构
#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 更新机器人数据
    // let update_robot_t = UpdateRobot {
    //     electricity: 90,
    //     activate: true,
    //     updated_at: chrono::Utc::now().naive_utc(),
    // };
    // update_robot(&mut connection, "Robot-002", update_robot_t);

    let config = Config::from_file("config.yaml")?;
    println!("{}", config.database.url);


    // 构建路由
    let app = Router::new()
        // 根路由
        .route("/", get(root_handler))

        // 基础路由
        .route("/hello/:name", get(hello_handler))
        .route("/user", get(user_handler))

        // 机器人相关路由
        .route("/create/Robot", post(create_robot))
        .route("/query/Robot", get(get_robot_info))

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

async fn create_robot(
    Json(robot_t): Json<NewRobot>
) -> Result<StatusCode, (StatusCode, String)>{
    println!("Robot_id:{}, Device_id: {}, electricity:{} ,activate:{} ",&robot_t.robot_id, &robot_t.device_id, &robot_t.electricity, &robot_t.activate );

    // 连接到数据库
    // let mut conn = establish_connection();
    // let new_robot = NewRobot {
    //     robot_id: "Robot-002".to_string(),
    //     device_id: "MacAddress or Other".to_string(),
    //     electricity: 80,
    //     activate: false,
    //     updated_at: chrono::Utc::now().naive_utc(),
    // };
    // insert_robot(&mut conn, new_robot);
    Ok(StatusCode::CREATED)
}


async fn get_robot_info() -> Result<Json<Vec<Robot>>, StatusCode> {
    // 连接到数据库
    let mut conn = establish_connection();
    // 查询机器人数据
    let robots : Vec<Robot> = get_robot(&mut conn);

    // 返回数据
    Ok(Json(robots))
}