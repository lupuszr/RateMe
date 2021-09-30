use std::time::SystemTime;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};


use crate::schema::skill;

#[derive(Queryable, Insertable)]
#[table_name = "skill"]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime
}
