use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

use super::base_types::{ Id };

#[derive(Serialize, Deserialize, FromForm)]
#[serde(crate = "rocket::serde")]
pub struct Skill {
    id: Id,
    name: String,
    category: String
}
