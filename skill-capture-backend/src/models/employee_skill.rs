use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

use super::base_types::{ Score };
use std::time::SystemTime;
use super::skill::{ Skill };
use super::employee::{ Employee };

use crate::schema::employee_skill;

struct RatedBy {
  employee_id: i32,
  score: i8,
  created_at: SystemTime
}

#[derive(Insertable, Queryable, Identifiable)]
#[belongs_to(Skill, foreign_key="skill_id")]
#[belongs_to(Employee, foreign_key="employee_id")]
#[table_name = "employee_skill"]
#[primary_key("skill_id", "employee_id")]
pub struct EmployeeSkill {
    skill_id: i32,
    employee_id: i32,
    history: RatedBy,
}
