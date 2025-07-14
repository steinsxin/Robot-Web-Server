use diesel::prelude::*;
use crate::schema::robot_manager::dsl::*;
use diesel::PgConnection;
use chrono::Local;
use chrono::Utc;
use crate::models::UpdateRobot;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RobotPayload {
    pub robot_id: String,
    pub electricity: String,
    pub activate: String,
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

    diesel::update(robot_manager.filter(robot_id.eq(target_robot_id)))
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