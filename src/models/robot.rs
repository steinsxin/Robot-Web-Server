use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::robot_manager;

#[derive(Queryable, Debug, Serialize)]
pub struct RobotManager {
    pub id: i32,
    pub robot_id: String,
    pub electricity: i32,
    pub activate: bool,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = robot_manager)]
pub struct UpdateRobot {
    pub electricity: Option<i32>,
    pub activate: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize)]
pub struct RobotPayload {
    pub robot_id: String,
    pub electricity: String,
    pub activate: String,
}

#[derive(Debug, Deserialize)]
pub struct RobotIdentify {
    pub robot_id: String,
}

#[derive(Debug, Deserialize)]
pub struct RobotIpRequest {
    pub robot_id: String,
}