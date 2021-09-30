#[macro_use] extern crate rocket;
use rocket::serde::json::{Json, Value, json};
use rocket::serde::{Serialize, Deserialize};

mod crate::models::employee;
use employee::{ Employee, mk_employee };
// use chrono::{DateTime, Duration, Utc};


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
    rocket::build().mount("/", routes![index])
}