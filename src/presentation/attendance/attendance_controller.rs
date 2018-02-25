
#![allow(unused_attributes)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

use rocket_contrib::{ Json, Value };
use super::AttendanceResource;
use data::{ AttendanceRepository, AttendanceInsertableEntity };
use database::DatabaseConnection;

type AttendanceResources = Vec<AttendanceResource>;

#[post("/attendances", format="application/json", data="<resource>")]
pub fn post_attendance(connection: DatabaseConnection, resource: Json<AttendanceResource>) -> Json<Value> {

    let attendance = resource.0;
    let entity = AttendanceInsertableEntity::new(&attendance.user, &attendance.check_at, attendance.attendance_type);

    let repository = AttendanceRepository::new(connection);
    repository.register(&entity);

    Json(json!({"status": "ok"}))
}

#[get("/attendances")]
pub fn get_attendances(connection: DatabaseConnection) -> Json<AttendanceResources> {

    let repository = AttendanceRepository::new(connection);
    let attendances = repository.get_attendances().into_iter().map(|entity|
        AttendanceResource::new(&entity.user, &entity.check_at, entity.attendance_type)
    ).collect();

    Json(attendances)
}

#[get("/users/<user>/attendances")]
pub fn get_attendances_by_user(connection: DatabaseConnection, user: String) -> Json<AttendanceResources> {

    let repository = AttendanceRepository::new(connection);
    let attendances = repository.get_attendances_by_user(&user).into_iter().map(|entity|
        AttendanceResource::new(&entity.user, &entity.check_at, entity.attendance_type)
    ).collect();

    Json(attendances)
}

#[get("/users/<user>/month/<month>/attendances")]
pub fn get_attendances_by_month(connection: DatabaseConnection, user: String, month: u32) -> Json<AttendanceResources> {

    let repository = AttendanceRepository::new(connection);
    let attendances = repository.get_attendances_by_month(&user, month).into_iter().map(|entity|
        AttendanceResource::new(&entity.user, &entity.check_at, entity.attendance_type)
    ).collect();

    Json(attendances)
}

#[get("/users/<user>/month/<month>/working_time")]
pub fn get_working_time_by_month(connection: DatabaseConnection, user: String, month: u32) -> Json<AttendanceResources> {

    let repository = AttendanceRepository::new(connection);
    let attendances = repository.get_attendances_by_month(&user, month).into_iter().map(|entity|
        AttendanceResource::new(&entity.user, &entity.check_at, entity.attendance_type)
    ).collect();

    Json(attendances)
}

#[get("/users/<user>/month/<month>/working_overtime")]
pub fn get_working_overtime_by_month(connection: DatabaseConnection, user: String, month: u32) -> Json<AttendanceResources> {

    let repository = AttendanceRepository::new(connection);
    let attendances = repository.get_attendances_by_month(&user, month).into_iter().map(|entity|
        AttendanceResource::new(&entity.user, &entity.check_at, entity.attendance_type)
    ).collect();

    Json(attendances)
}
