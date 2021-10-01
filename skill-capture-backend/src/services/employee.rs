use rocket::serde::json::{Json, Value, json};
use rocket::form::Form;
use crate::models::employee::{Employee};
use diesel::prelude::*;

#[derive(FromForm)]
pub struct EmployeePostData {
    first_name: String,
    last_name: String,
    title: String
}


// #[get("/")]
// pub fn get_employee() -> Option<Json<EmployeeDbo>> {
//     // read from DB!
//     Some(Json(mk_employee(
//         String::from("test"),
//         String::from("k"),
//         String::from("hell"),
//         vec![]
//     )))
// }

#[post("/", format = "application/x-www-form-urlencoded", data = "<employee>")]
pub fn post_employee(employee: Form<EmployeePostData>) -> &'static str {
    let connection = crate::establish_connection();

    let new_employee = Employee{
        first_name: employee.first_name.clone(),
        last_name: employee.last_name.clone(),
        title: employee.title.clone()
    };

    diesel::insert_into(crate::schema::employee::table).values(&new_employee).execute(&connection);
    
    let response = format!("Insert of {} {} employee successful", new_employee.first_name, new_employee.last_name).to_owned();

    Box::leak(response.into_boxed_str())
}