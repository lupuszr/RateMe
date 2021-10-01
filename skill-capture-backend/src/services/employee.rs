use rocket::serde::json::{Json, Value, json};
use rocket::form::Form;
use crate::dbo::employee_dbo::{EmployeeDbo, mk_employee};

#[get("/")]
pub fn get_employee() -> Option<Json<EmployeeDbo>> {
    // read from DB!
    Some(Json(mk_employee(
        String::from("test"),
        String::from("k"),
        String::from("hell"),
        vec![]
    )))
}

#[post("/", format = "application/x-www-form-urlencoded", data = "<employee>")]
pub fn post_employee(employee: Form<EmployeeDbo>) {
    println!("Employee {}", employee.first_name);
    // insert employee in DB
}