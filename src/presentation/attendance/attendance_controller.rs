
#![allow(unused_attributes)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

use rocket_contrib::Json;
use rocket::request::FromRequest;
use super::AttendanceResource;
use data::{ AttendanceRepository, AttendanceEntity };
use database::DatabaseConnection;

#[post("/attendances", format="application/json", data="<resource>")]
pub fn post_attendance(connection: DatabaseConnection, resource: Json<AttendanceResource>) -> Json<AttendanceResource> {
    resource
}

#[get("/attendances")]
pub fn get_attendances(connection: DatabaseConnection) -> Json<Vec<AttendanceResource>> {

    let repository = AttendanceRepository::new(connection);
    let attendances = repository.getAttendances().into_iter().map(|entity|
        AttendanceResource::new(&entity.user, &entity.check_at, entity.attendance_type)
    ).collect();

    Json(attendances)
}

#[get("/attendances/<user>")]
pub fn get_attendances_filterd_user(connection: DatabaseConnection, user: String) -> Json<Vec<AttendanceResource>> {

    let repository = AttendanceRepository::new(connection);
    let attendances = repository.getAttendancesByUser(&user).into_iter().map(|entity|
        AttendanceResource::new(&entity.user, &entity.check_at, entity.attendance_type)
    ).collect();

    Json(attendances)
}