#[macro_use] 
extern crate diesel;
extern crate dotenv;
use diesel::pg::PgConnection;
use diesel::Insertable;
mod models {
    pub mod base_types;
    pub mod employee;
    pub mod rated_skill;
    pub mod skill;
}

mod services {
    pub mod employee;
}

#[macro_use] extern crate rocket;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

use models::employee::{ Employee, mk_employee };
// use chrono::{DateTime, Duration, Utc};

pub mod schema;
use schema::employee;
use dotenv::dotenv;
use std::env;


#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let connection = establish_connection();
    let new_employee = Employee{firstname:String::from("aa"), lastname:String::from("zzz"), title: String::from("xxxzzx")};
    diesel::insert_into(employee::table).values(&new_employee).execute(&connection);
    
    rocket::build()
        .mount("/employee", routes![services::employee::get_employee])
        .mount("/employee", routes![services::employee::post_employee])
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection.establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}