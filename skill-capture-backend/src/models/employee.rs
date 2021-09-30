
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

mod rated_skill;
use rated_skill::{ RatedSkill };

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Employee {
    firstName: String,
    lastName: String,
    title: String,
    assignedSkills: Vec<RatedSkill>
}

pub fn mk_employee(firstName: String, lastName: String, title: String, assignedSkills: Vec<RatedSkill>) -> Employee {
    Employee {
        firstName,
        lastName,
        title,
        assignedSkills
    }
}