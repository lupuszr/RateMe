#[macro_use] 
extern crate rocket;
extern crate diesel;
extern crate dotenv;
use diesel::pg::PgConnection;
use diesel::Insertable;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};
// use chrono::{DateTime, Duration, Utc};

pub mod schema;
use schema::employee;
use dotenv::dotenv;
use std::env;

// todo use chrono
type DateTime = String;
type Score = i8;
type Id = usize;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Skill {
    id: Id,
    name: String,
    category: String
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct RatedSkill {
    score: Score,
    whoRated: Employee,
    createdAt: DateTime,
    skill: Skill
}


#[derive(Serialize, Deserialize, Insertable)]
#[serde(crate = "rocket::serde")]
#[table_name = "employee"]
struct Employee {
    firstname: String,
    lastname: String,
    title: String
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}


#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let connection = establish_connection();
    let new_employee = Employee{firstname:String::from("aa"), lastname:String::from("zzz"), title: String::from("xxxzzx")};
    diesel::insert_into(employee::table).values(&new_employee).execute(&connection);
    rocket::build().mount("/", routes![index])
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection.establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}