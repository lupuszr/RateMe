use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

use super::base_types::{ Score, DateTime };
use super::skill::{ Skill };
use super::employee::{ Employee };

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RatedSkill {
    score: Score,
    // who_rated: Employee,
    created_at: DateTime,
    // skill: Skill
}
