use rocket::serde::{self, Serialize, Deserialize};
use rocket::serde::json::{Json, Value, json};

use super::skill_dbo::{ SkillDbo };

#[derive(Serialize, Deserialize)]
#[serde(crate="self::serde")]
pub struct EmployeeDbo {
    first_name: String,
    last_name: String,
    title: String,
    assigned_skills: Vec<SkillDbo>
}

pub fn mk_employee(first_name: String, last_name: String, title: String, assigned_skills: Vec<RatedSkill>) -> Employee {
  EmployeeDbo {
        first_name,
        last_name,
        title,
        assigned_skills
    }
}
#[derive(FromForm)]
pub struct EmployeePostData {
    pub first_name: String,
    last_name: String,
    title: String
}