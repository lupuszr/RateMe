use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

use super::rated_skill::{ RatedSkill };


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EmployeeDbo {
    first_name: String,
    last_name: String,
    title: String,
    assigned_skills: Vec<RatedSkill>
}

pub fn mk_employee(first_name: String, last_name: String, title: String, assigned_skills: Vec<RatedSkill>) -> Employee {
  EmployeeDbo {
        first_name,
        last_name,
        title,
        assigned_skills
    }
}