use std::time::SystemTime;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{self, Serialize, Deserialize};

use super::employee_dbo::{ EmployeeDbo };

#[derive(Serialize, Deserialize, FromForm)]
#[serde(crate="self::serde")]
pub struct SkillDbo {
    pub name: String,
    pub category: String
}
