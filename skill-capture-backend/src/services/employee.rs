use rocket::serde::json::{Json, Value, json};
use rocket::form::Form;
use crate::models::employee::{Employee,AddEmployee};
use diesel::prelude::*;

#[derive(FromForm)]
pub struct EmployeePostData {
    first_name: String,
    last_name: String,
    title: String
}

#[get("/")]
pub fn get_all_employees() -> Json<Vec<Employee>> {
    use crate::schema::employee::dsl::*;
    let connection = crate::establish_connection();
    let employees_res = employee.load::<Employee>(&connection);
    let unpacked_employees = employees_res.unwrap();
    return Json(unpacked_employees);
}

#[post("/", format = "application/x-www-form-urlencoded", data = "<employee>")]
pub fn post_employee(employee: Form<EmployeePostData>) -> &'static str {
    let connection = crate::establish_connection();

    let new_employee = AddEmployee{
        first_name: employee.first_name.clone(),
        last_name: employee.last_name.clone(),
        title: employee.title.clone()
    };

    diesel::insert_into(crate::schema::employee::table).values(&new_employee).execute(&connection);
    
    let response = format!("Insert of {} {} employee successful", new_employee.first_name, new_employee.last_name).to_owned();

    Box::leak(response.into_boxed_str())
}