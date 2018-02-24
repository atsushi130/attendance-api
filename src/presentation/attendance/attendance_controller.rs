
#![allow(unused_attributes)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

use rocket_contrib::Json;
use super::AttendanceResource;

#[post("/attendances", format="application/json", data="<resource>")]
pub fn post_attendance(resource: Json<AttendanceResource>) -> Json<AttendanceResource> {
    resource
}

#[get("/attendances")]
pub fn get_attendances() -> Json<Vec<AttendanceResource>> {

    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let attendances = numbers.iter().map(|number| {
        AttendanceResource::new("atsushi", "2018/02/24 12:00:00", 0)
    }).collect();

    Json(attendances)
}

#[get("/attendances/<user>")]
pub fn get_attendances_filterd_user(user: String) -> Json</*Vec<*/AttendanceResource/*>*/> {
    let attendance = AttendanceResource::new("atsushi", "2018/02/24 12:00:00:00", 0);
    Json(attendance)
}