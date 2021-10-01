use std::time::SystemTime;

use super::employee::{ Employee };

use crate::schema::skill;

#[derive(Insertable, Identifiable)]
#[table_name = "skill"]
#[primary_key("id")]
pub struct AddSkill {
    // pub id: Option<i32>,
    pub name: String,
    pub category: String,
    // pub created_at:  Option<SystemTime>,
    // pub updated_at:  Option<SystemTime>
}
#[derive(Queryable, Identifiable,Debug)]
#[table_name = "skill"]
#[primary_key("id")]
pub struct Skill {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime
}

#[derive(FromForm)]
pub struct SkillPostData {
    pub name: String,
    pub category: String
}