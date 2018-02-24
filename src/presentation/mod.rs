
#![allow(unused_attributes)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenv::dotenv;
use std::env;
use r2d2;
use r2d2_diesel::ConnectionManager;

use rocket;

mod attendance;
use self::attendance::attendance_controller;

pub fn routes() {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let connection = r2d2::Pool::builder().build(manager).unwrap();

    rocket::ignite().mount(
        "/",
        routes![
            attendance_controller::get_attendances,
            attendance_controller::get_attendances_filterd_user,
            attendance_controller::post_attendance
        ]
    )
    .manage(connection)
    .launch();
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}