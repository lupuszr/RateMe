use std::time::SystemTime;

use super::skill::{ Skill };

use crate::schema::employee;

#[derive(Queryable, Insertable, Identifiable)]
#[table_name = "employee"]
#[primary_key("id")]
pub struct Employee {
    id: i32,
    first_name: String,
    last_name: String,
    title: String,
    skill: Vec<Skill>,
    created_at: SystemTime,
    updated_at: SystemTime,
}