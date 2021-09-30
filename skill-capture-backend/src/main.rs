#[macro_use] extern crate rocket;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

mod crate::models::employee;
use employee::{ Employee, mk_employee };
// use chrono::{DateTime, Duration, Utc};

extern crate dotenv;
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

    rocket::build().mount("/", routes![index])
}