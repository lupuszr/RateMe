use rocket::serde::json::{Json, Value, json};
use rocket::form::Form;
use crate::dbo::skill_dbo::{ SkillDbo };
use crate::models::skill::{ Skill,AddSkill };
use std::time::SystemTime;
use diesel::prelude::*;

#[derive(FromForm)]
pub struct SkillPostData {
    pub name: String,
    pub category: String
}

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
pub fn post_skill(skill: Form<SkillPostData>) -> &'static str {
    let connection = crate::establish_connection();
    let new_skill = AddSkill{
        name: skill.name.clone(), 
        category: skill.category.clone()
    };

    diesel::insert_into(crate::schema::skill::table).values(&new_skill).execute(&connection);
    
    let response = format!("Insert of {} skill successful", new_skill.name).to_owned();

    Box::leak(response.into_boxed_str())
}