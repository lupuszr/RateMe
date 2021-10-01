use rocket::serde::{self, Serialize, Deserialize};
use std::time::SystemTime;

use super::skill::{ Skill };

use crate::schema::employee;

#[derive(Insertable, Identifiable)]
#[table_name = "employee"]
#[primary_key("id")]
pub struct AddEmployee {
    // id: i32,
    pub first_name: String,
    pub last_name: String,
    pub title: String
    // created_at: SystemTime,
    // updated_at: SystemTime,
}
#[derive(Queryable, Identifiable,Debug, Serialize, Deserialize)]
#[serde(crate="self::serde")]
#[table_name = "employee"]
#[primary_key("id")]
pub struct Employee {
    id: i32,
    pub first_name: String,
    pub last_name: String,
    pub title: String,
    created_at: SystemTime,
    updated_at: SystemTime
}