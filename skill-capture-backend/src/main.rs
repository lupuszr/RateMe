#[macro_use] extern crate rocket;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};
// use chrono::{DateTime, Duration, Utc};

// todo use chrono
type DateTime = String;
type Score = i8;
type Id = usize;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Skill {
    id: Id,
    name: String,
    category: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RatedSkill {
    score: Score,
    whoRated: Employee,
    createdAt: DateTime,
    skill: Skill
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Employee {
    firstName: String,
    lastName: String,
    title: String,
    assignedSkills: Vec<RatedSkill>
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}