#[macro_use] 
extern crate diesel;
extern crate dotenv;
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
}

#[macro_use] extern crate rocket;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

use models::skill::{ Skill };
use models::employee::{ Employee };
use dbo::employee_dbo::{mk_employee};
// use chrono::{DateTime, Duration, Utc};

// use schema::skill;
use dotenv::dotenv;
use std::env;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[launch]
fn rocket() -> _ {
    
    dotenv().ok();
    let connection = establish_connection();
    
    let new_employee = Skill{id:1, name: String::from("asdasda"), category: String::from("category__1"), created_at: SystemTime::now(), updated_at: SystemTime::now()};
    diesel::insert_into(schema::skill::table).values(&new_employee).execute(&connection);
    rocket::build().mount("/", routes![index])

    // rocket::build()
    //     .mount("/employee", routes![services::employee::get_employee])
    //     .mount("/employee", routes![services::employee::post_employee])
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}