use diesel::prelude::*;
use serde::Serialize;
use crate::schema::posts;

#[derive(Queryable, Serialize)]
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