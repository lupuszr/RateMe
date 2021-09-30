use rocket::serde::json::{Json, Value, json};
use rocket::form::Form;
use crate::models::employee::{Employee, mk_employee, EmployeePostData};

#[get("/")]
pub fn get_employee() -> Option<Json<Employee>> {
    Some(Json(mk_employee(
        String::from("test"),
        String::from("k"),
        String::from("hell"),
        vec![]
    )))
}

#[post("/", format = "application/x-www-form-urlencoded", data = "<employee>")]
pub fn post_employee(employee: Form<EmployeePostData>) {
    println!("Employee {}", employee.first_name);
}