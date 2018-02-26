
#![allow(unused_attributes)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

use rocket_contrib::{ Json, Value };
use super::AttendanceResource;
use data::{ AttendanceRepository, AttendanceInsertableEntity };
use domain::AttendanceService;
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

    let attendances = AttendanceService.get_attendances(connection).into_iter().map(|entity|
        AttendanceResource::new(&entity.user, &entity.check_at, entity.attendance_type)
    ).collect();

    Json(attendances)
}

#[get("/users/<user>/attendances")]
pub fn get_attendances_by_user(connection: DatabaseConnection, user: String) -> Json<AttendanceResources> {

    let attendances = AttendanceService.get_attendances_by_user(&user, connection).into_iter().map(|entity|
        AttendanceResource::new(&entity.user, &entity.check_at, entity.attendance_type)
    ).collect();

    Json(attendances)
}

#[get("/users/<user>/month/<month>/attendances")]
pub fn get_attendances_by_month(connection: DatabaseConnection, user: String, month: u32) -> Json<AttendanceResources> {

    let attendances = AttendanceService.get_attendances_by_month(&user, month, connection).into_iter().map(|entity|
        AttendanceResource::new(&entity.user, &entity.check_at, entity.attendance_type)
    ).collect();

    Json(attendances)
}

#[get("/users/<user>/month/<month>/working_time")]
pub fn get_working_time_by_month(connection: DatabaseConnection, user: String, month: u32) -> Json<Value> {
    let working_time = AttendanceService.get_working_time(&user, month, connection);
    Json(json!({"working_time": working_time}))
}

#[get("/users/<user>/month/<month>/working_overtime")]
pub fn get_working_overtime_by_month(connection: DatabaseConnection, user: String, month: u32) -> Json<Value> {
    let working_overtime = AttendanceService.get_working_overtime(&user, month, connection);
    Json(json!({"working_overtime": working_overtime}))
}
