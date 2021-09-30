use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct RatedSkill {
    score: Score,
    whoRated: Employee,
    createdAt: DateTime,
    skill: Skill
}
