use std::time::SystemTime;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{self, Serialize, Deserialize};

use super::employee_dbo::{ EmployeeDbo };

#[derive(Serialize, Deserialize)]
#[serde(crate="self::serde")]
pub struct SkillDbo {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub employee: Vec<EmployeeDbo>,
    pub created_at: SystemTime,
    pub updated_at: SystemTime
}
