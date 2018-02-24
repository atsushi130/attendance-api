
#![allow(unused_attributes)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

use rocket;

mod attendance;
use self::attendance::attendance_controller;

pub fn routes() {
    rocket::ignite().mount(
        "/",
        routes![
            attendance_controller::get_attendances,
            attendance_controller::get_attendances_filterd_user,
            attendance_controller::post_attendance
        ]
    ).launch();
}