use crate::schema::robot_manager;
use diesel::prelude::*;
use diesel::Queryable;
use chrono::{NaiveDateTime};
use serde::{Serialize, Deserialize};

#[derive(Queryable ,Serialize, Deserialize)]
#[diesel(table_name = robot_manager)]
pub struct Robot {
    pub id: i32,
    pub robot_id: String,
    pub device_id: String,
    pub electricity: i32,
    pub activate: bool,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable,Serialize, Deserialize)]
#[diesel(table_name = robot_manager)]
pub struct NewRobot {
    pub robot_id: String,
    pub device_id: String,
    pub electricity: i32,
    pub activate: bool,
}

#[derive(AsChangeset)]
#[diesel(table_name = robot_manager)]
pub struct UpdateRobot {
    pub electricity: i32,
    pub activate: bool,
}
