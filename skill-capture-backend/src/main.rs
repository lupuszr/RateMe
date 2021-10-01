#[macro_use] 
extern crate diesel;
extern crate dotenv;
use rocket::Build;
use rocket::Rocket;
use diesel::prelude::*;

pub mod schema;

mod models {
    pub mod base_types;
    pub mod employee;
    pub mod skill;
}

mod dbo {
    pub mod employee_dbo;
    pub mod skill_dbo;
}
use std::time::SystemTime;

mod services {
    pub mod employee;
    pub mod skill;
}

#[macro_use] extern crate rocket;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

use models::skill::{ Skill, AddSkill };
use models::employee::{ Employee };
use dbo::employee_dbo::{mk_employee};

use dotenv::dotenv;
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();

    // rocket::build().mount("/", routes![index])
    rocket::build()
    .mount("/employee", routes![services::employee::post_employee])
    .mount("/skill", routes![services::skill::post_skill])
    .mount("/skills", routes![services::skill::get_all_skills])
    .mount("/employees", routes![services::employee::get_all_employees])

}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}