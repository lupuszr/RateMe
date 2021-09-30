use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Skill {
    id: Id,
    name: String,
    category: String
}
