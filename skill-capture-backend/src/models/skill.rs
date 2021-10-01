use std::time::SystemTime;

use super::employee::{ Employee };

use crate::schema::skill;

#[derive(Queryable, Insertable, Identifiable)]
#[table_name = "skill"]
#[primary_key("id")]
pub struct Skill {
    // pub id: i32,
    pub name: String,
    pub category: String,
    // pub created_at: SystemTime,
    // pub updated_at: SystemTime
}

#[derive(FromForm)]
pub struct SkillPostData {
    pub name: String,
    pub category: String
}