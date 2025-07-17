use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::post::Post;
use crate::models::robot::{RobotManager, UpdateRobot, RobotPayload};
use crate::schema::{posts, robot_manager};
use diesel::prelude::*;
use chrono::Utc;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool(database_url: &str) -> DbPool {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}

pub fn get_posts(conn: &mut PgConnection) -> Vec<Post> {
    posts::table
        .filter(posts::published.eq(true))
        .load::<Post>(conn)
        .expect("Error loading posts")
}

pub fn update_robot_status(
    conn: &mut PgConnection,
    target_robot_id: &str,
    elec: i32,
    is_active: bool,
) -> Result<usize, diesel::result::Error> {
    let updated = UpdateRobot {
        electricity: Some(elec),
        activate: Some(is_active),
        updated_at: Some(Utc::now().naive_utc()),
    };

    diesel::update(robot_manager::table.filter(robot_manager::robot_id.eq(target_robot_id)))
        .set(&updated)
        .execute(conn)
}

pub fn parse_robot_payload(payload: &str) -> Option<(String, i32, bool)> {
    let parsed: RobotPayload = serde_json::from_str(payload.trim()).ok()?;

    let electricity_val = parsed.electricity.parse::<i32>().ok()?;

    let activate_val = match parsed.activate.to_lowercase().as_str() {
        "true" | "1" => true,
        "false" | "0" => false,
        _ => return None,
    };

    Some((parsed.robot_id, electricity_val, activate_val))
}