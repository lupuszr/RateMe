use rocket::serde::{self, Serialize, Deserialize};
use rocket::serde::json::{Json, Value, json};

use super::skill_dbo::{ SkillDbo };

#[derive(Serialize, Deserialize, FromForm)]
#[serde(crate="self::serde")]
pub struct EmployeeDbo {
    pub first_name: String,
    last_name: String,
    title: String,
    assigned_skills: Vec<SkillDbo>
}

pub fn mk_employee(first_name: String, last_name: String, title: String, assigned_skills: Vec<SkillDbo>) -> EmployeeDbo {
  EmployeeDbo {
        first_name,
        last_name,
        title,
        assigned_skills
    }
}