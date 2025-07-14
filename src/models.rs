use diesel::prelude::*;
use diesel::Queryable;
use crate::schema::posts;
use crate::schema::robot_manager;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub context: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub context: String,
    pub published: bool,
}

#[derive(Queryable, Debug)]
pub struct RobotManager {
    pub id: i32,
    pub robot_id: String,
    pub electricity: i32,
    pub activate: bool,
    pub updated_at: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::robot_manager)]
pub struct UpdateRobot {
    pub electricity: Option<i32>,
    pub activate: Option<bool>,
    pub updated_at: Option<NaiveDateTime>,
}