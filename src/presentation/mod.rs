
#![allow(unused_attributes)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

use dotenv::dotenv;
use std::env;

use rocket;

mod attendance;
use self::attendance::attendance_controller;

use database::init_pool;

pub fn routes() {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    rocket::ignite().mount(
        "/",
        routes![
            attendance_controller::get_attendances,
            attendance_controller::get_attendances_filterd_user,
            attendance_controller::post_attendance
        ]
    )
    .manage(init_pool(&database_url))
    .launch();
}
