use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

use super::rated_skill::{ RatedSkill };


#[derive(Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct Employee {
    first_name: String,
    last_name: String,
    title: String,
    // assigned_skills: Vec<RatedSkill>
}

#[derive(FromForm)]
pub struct EmployeePostData {
    pub first_name: String,
    last_name: String,
    title: String
}

pub fn mk_employee(first_name: String, last_name: String, title: String, assigned_skills: Vec<RatedSkill>) -> Employee {
    Employee {
        first_name,
        last_name,
        title,
        // assigned_skills
    }
}