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

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> Rocket<Build> {
    dotenv().ok();

    // rocket::build().mount("/", routes![index])
    rocket::build()
    .attach(CORS)
    .mount("/employees", routes![services::employee::post_employee])
    .mount("/skills", routes![services::skill::post_skill])
    .mount("/skills", routes![services::skill::get_all_skills])
    .mount("/employees", routes![services::employee::get_all_employees])

}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}