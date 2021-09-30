#[macro_use] 
extern crate diesel;
extern crate dotenv;
use diesel::prelude::*;

pub mod schema;

mod models {
    pub mod base_types;
    pub mod employee;
    pub mod rated_skill;
    pub mod skill;
}
use std::time::SystemTime;
#[macro_use] extern crate rocket;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

use models::skill::{ Skill };
use models::employee::{ Employee, mk_employee };
// use chrono::{DateTime, Duration, Utc};

// use schema::skill;
use dotenv::dotenv;
use std::env;

#[get("/")]
fn index() -> Option<Json<Employee>> {
    Some(Json(mk_employee(
        String::from("test"),
        String::from("k"),
        String::from("hell"),
        vec![]
    )))
}

#[launch]
fn rocket() -> _ {
    
    dotenv().ok();
    let connection = establish_connection();
    
    let new_employee = Skill{id:1, name: String::from("asdasda"), category: String::from("category__1"), created_at: SystemTime::now(), updated_at: SystemTime::now()};
    diesel::insert_into(schema::skill::table).values(&new_employee).execute(&connection);
    rocket::build().mount("/", routes![index])
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}