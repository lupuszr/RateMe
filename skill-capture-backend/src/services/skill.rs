use rocket::serde::json::{Json, Value, json};
use rocket::form::Form;
use crate::dbo::skill_dbo::{ SkillDbo };
use crate::models::skill::{ Skill };
use std::time::SystemTime;
use diesel::prelude::*;

// #[get("/")]
// pub fn get_skill() -> Option<Json<Skill>> {
//     Some(Json(mk_employee(
//         String::from("test"),
//         String::from("k"),
//         String::from("hell"),
//         vec![]
//     )))
// }

#[post("/", format = "application/x-www-form-urlencoded", data = "<skill>")]
pub fn post_skill(skill: Form<SkillDbo>) -> &'static str {
    println!("Skill {}", skill.category);
    
    let connection = crate::establish_connection();
    let new_skill = Skill{
        name: String::from("testName"), 
        category: String::from("testCat")
    };

    diesel::insert_into(crate::schema::skill::table).values(&new_skill).execute(&connection);
    
    "Insert successful"
}